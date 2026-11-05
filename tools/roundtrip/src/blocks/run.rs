//! `roundtrip blocks`: the block-level training gate. Every block of every corpus of the training mix goes up through
//! the layers with one discourse and one world, and must come back byte for byte with its residual (exact mode);
//! the block is decoded from each start (normal mode). The residual bits per block, layer and class are the training
//! signal (`residual-blocks.tsv`); the corpus total is the number to drive down.
//!
//!   roundtrip blocks --out <dir> [--threads 8] [--max-words 2500] [--only <corpus,…>] [--stride N] [--controls N]
//!                    [--school <dir>, default $MOVA_DATA/school/courses] [--tales <dir>] [--justso <txt>] [--alice <txt>] [--judaism <dir>]
//!                    [--shakespeare <tsv>] [--corpora <dir>] [--hbo <dir> --langseg <model> --gparse <bin>]
//!                    [--hbo-books GEN,EXO,LEV,NUM,DEU]
//!
//! Runs: `block` (the gate: one discourse and one world a block), `sentence` (the same blocks, the discourse, the
//! realizer and the world reset at every sentence), and on every N-th block of each corpus (`--controls`, 10) the
//! negative controls: `flat` (the block's sentences joined by a space, in order) against `shuffle` (the same,
//! shuffled), and `damage-<layer>` (the exact decode loses one residual item of the layer: the gate must show red).

use std::collections::BTreeMap;
use std::fmt::Write as _;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result};
use en::annotate::Annotator;
use langseg::{Rng, fnv};

use super::{Block, Chapter, MAX_WORDS};
use crate::class::{CLASSES, Class, LAYERS, Layer};
use crate::pipeline::{self, Opts, Unit};
use crate::report;
use crate::tally::{STARTS, Start, Tally};
use crate::vol::{self, Block as VBlock, Piece, Volume};

/// One block's numbers, without examples.
#[derive(Clone, Debug)]
pub struct Row {
    pub block: String,
    pub chapters: usize,
    pub words: usize,
    pub sents: u64,
    pub tokens: u64,
    pub exact: bool,
    pub damaged: bool,
    pub cells: Vec<(Layer, Class, u64, f64)>,
}

/// A run over a corpus: the total (with examples) and the rows.
#[derive(Clone, Debug, Default)]
pub struct Out {
    pub total: Tally,
    pub rows: Vec<Row>,
}

fn row(b: &Block, t: &Tally) -> Row {
    Row {
        block: b.id.clone(),
        chapters: b.chapters.len(),
        words: b.words,
        sents: t.sents,
        tokens: t.tokens,
        exact: t.exact_ok == 1,
        damaged: t.damaged == 1,
        cells: t.residual.cells.iter().map(|((l, c), v)| (*l, *c, v.items, v.bits)).collect(),
    }
}

/// The block's sentences as one volume: each sentence a whole block, joined by a space; shuffled if asked.
pub fn flat(a: &Annotator, v: &Volume, shuffle: bool) -> Result<Volume> {
    let mut ss = Vec::new();
    for (b, blk) in v.blocks.iter().enumerate() {
        for s in vol::segment(a, &blk.text, b, blk.whole, None)?.sents {
            ss.push(s.original());
        }
    }
    if shuffle {
        Rng(0x5b10c ^ fnv(&v.name)).shuffle(&mut ss);
    }
    let mut f = Volume { name: format!("{}{}", v.name, if shuffle { " (shuffled)" } else { " (flat)" }), pieces: Vec::new(), blocks: Vec::new(), canon_sep: " ".into() };
    for (k, s) in ss.into_iter().enumerate() {
        if k > 0 {
            f.pieces.push(Piece::Lit(" ".into(), Class::Layout));
        }
        f.blocks.push(VBlock { text: s, gold: None, whole: true, marks: Vec::new() });
        f.pieces.push(Piece::Text(f.blocks.len() - 1));
    }
    Ok(f)
}

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum Shape {
    AsIs,
    Flat,
    Shuffled,
}

/// One run over the blocks, in chunks merged in order (the same numbers and examples on every run).
fn run_blocks(a: &Annotator, blocks: &[&Block], o: &Opts, shape: Shape, threads: usize) -> Result<Out> {
    let mut out = Out::default();
    for chunk in blocks.chunks(512) {
        let ts = super::heb::par_each(chunk, threads, |_, b| {
            let v = match shape {
                Shape::AsIs => b.vol.clone(),
                Shape::Flat => flat(a, &b.vol, false)?,
                Shape::Shuffled => flat(a, &b.vol, true)?,
            };
            pipeline::run(a, &v, o)
        });
        for (b, t) in chunk.iter().zip(ts) {
            // a block that cannot go up or come down is red, with its reason
            let t = t.unwrap_or_else(|e| Tally { volumes: 1, damaged: o.damage.is_some() as u64, exact_fail: vec![format!("{}: {e:#}", b.vol.name)], ..Default::default() });
            out.rows.push(row(b, &t));
            out.total.merge(&t);
        }
    }
    Ok(out)
}

fn flag(args: &[String], name: &str) -> Option<String> {
    args.iter().position(|a| a == name).and_then(|i| args.get(i + 1)).cloned()
}

fn per(x: f64, n: u64) -> f64 {
    if n == 0 { 0.0 } else { x / n as f64 }
}

/// Bits of a layer, ids and lesson keys (`meta`) aside.
fn lbits(t: &Tally, l: Layer) -> f64 {
    t.residual.cells.iter().filter(|((x, c), _)| *x == l && *c != Class::Meta).map(|(_, v)| v.bits).sum::<f64>() + 0.0
}

fn cbits(t: &Tally, l: Layer, c: Class) -> f64 {
    t.residual.cells.get(&(l, c)).map_or(0.0, |v| v.bits)
}

fn total_bits(t: &Tally) -> f64 {
    LAYERS.iter().map(|&l| lbits(t, l)).sum()
}

fn rbits(r: &Row, l: Option<Layer>, meta: bool) -> f64 {
    r.cells.iter().filter(|(x, c, _, _)| l.is_none_or(|l| *x == l) && ((*c == Class::Meta) == meta)).map(|x| x.3).sum::<f64>() + 0.0
}

/// The corpora of the training mix as blocks.
fn corpora(args: &[String], max: usize) -> Result<Vec<(String, Vec<Block>)>> {
    let md = PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| format!("{}/mova-data", std::env::var("HOME").unwrap_or_default())));
    let p = |name: &str, def: PathBuf| flag(args, name).map(PathBuf::from).unwrap_or(def);
    let only: Option<Vec<String>> = flag(args, "--only").map(|x| x.split(',').map(String::from).collect());
    // with the site's Markdown alone (`--only site`), no other corpus
    let want = |c: &str| only.as_ref().is_none_or(|o| o.iter().any(|x| x == c));
    let stride: usize = flag(args, "--stride").map(|x| x.parse()).transpose()?.unwrap_or(1);
    let judaism = p("--judaism", md.join("corpora/judaism"));
    let c04 = md.join("runs/corpus-0.4");
    let mut out: Vec<(String, Vec<Block>)> = Vec::new();
    let t0 = std::time::Instant::now();
    let add = |name: &str, chs: Vec<Chapter>, out: &mut Vec<(String, Vec<Block>)>| {
        let bs = super::blocks(name, &chs, max);
        eprintln!("{name}: {} chapters, {} blocks ({:.1}s)", chs.len(), bs.len(), t0.elapsed().as_secs_f64());
        out.push((name.to_string(), bs));
    };
    if want("school") {
        add("school", super::school(&p("--school", md.join("school/courses")), stride)?, &mut out);
    }
    if want("tales") {
        let mut chs = super::books_dir(&p("--tales", c04.join("corpus-0.3/tales")))?;
        // Alice is a corpus of its own
        chs.retain(|c| c.group != "pg11");
        chs.extend(super::book(&p("--justso", c04.join("corpus-0.4/books/pg2781.txt")), "pg2781", true)?);
        add("tales", chs, &mut out);
    }
    if want("alice") {
        add("alice", super::book(&p("--alice", c04.join("corpus-0.4/books/pg11.txt")), "pg11", true)?, &mut out);
    }
    if want("tanakh") || want("hbo") {
        add("tanakh", super::jps(&judaism, &[])?, &mut out);
    }
    if want("talmud") {
        add("talmud", super::talmud_en(&judaism)?, &mut out);
    }
    if want("shakespeare") {
        add("shakespeare", super::shakespeare(&p("--shakespeare", md.join("corpora/emode/text/shakespeare.tsv")))?, &mut out);
    }
    if want("grammars") {
        add("grammars", super::grammars(&p("--corpora", md.join("corpora")))?, &mut out);
    }
    // the site's Markdown: a corpus per section, sampled and split (`site_md`)
    if let Some(root) = flag(args, "--site-md") {
        let mut s = super::site_md::Sample { max, ..Default::default() };
        if let Some(n) = flag(args, "--site-target") {
            s.target = n.parse()?;
        }
        if let Some(sp) = flag(args, "--site-split") {
            s.split = (sp != "all").then_some(sp);
        }
        s.only = flag(args, "--site-sections").map(|x| x.split(',').map(String::from).collect());
        let threads: usize = flag(args, "--threads").map(|x| x.parse()).transpose()?.unwrap_or(8);
        for (name, bs) in super::site_md::blocks(Path::new(&root), &s, threads)? {
            eprintln!("{name}: {} blocks, {} pages ({:.1}s)", bs.len(), bs.iter().map(|b| b.chapters.len()).sum::<usize>(), t0.elapsed().as_secs_f64());
            out.push((name, bs));
        }
    }
    Ok(out)
}

pub fn main(args: &[String], model: &Path) -> Result<()> {
    let out = PathBuf::from(flag(args, "--out").context("--out <dir>")?);
    // the markup dictionary learned from the site's training pages (`site-learn`)
    if let Some(d) = flag(args, "--site-dict") {
        super::site_learn::load_dict(Path::new(&d))?;
    }
    // the punctuation table learned from them, for the frames' realizer
    if let Some(p) = flag(args, "--site-punct") {
        en::lin::set_punct_table(en::lin::PunctTable::read(&std::fs::read_to_string(&p).with_context(|| p.clone())?));
    }
    // the Hebrew trees' lexicon learned from them
    if let Some(h) = flag(args, "--site-heb") {
        let _ = super::site_heb::HEB_LEX.set(super::site_heb::read(&std::fs::read_to_string(&h).with_context(|| h.clone())?));
    }
    let threads: usize = flag(args, "--threads").map(|x| x.parse()).transpose()?.unwrap_or(8);
    let max: usize = flag(args, "--max-words").map(|x| x.parse()).transpose()?.unwrap_or(MAX_WORDS);
    let ctrl: usize = flag(args, "--controls").map(|x| x.parse()).transpose()?.unwrap_or(10);
    let only: Option<Vec<String>> = flag(args, "--only").map(|x| x.split(',').map(String::from).collect());
    std::fs::create_dir_all(&out)?;
    let t0 = std::time::Instant::now();
    let mut sets = corpora(args, max)?;
    let a = Annotator::load(model)?;
    eprintln!("model {} loaded in {:.1}s", model.display(), t0.elapsed().as_secs_f64());
    // the Hebrew Bible, in the blocks of the JPS
    let mut heb: Option<(Vec<Block>, super::heb::Runs)> = None;
    if let Some(hbo) = flag(args, "--hbo") {
        let md = PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_default());
        let judaism = flag(args, "--judaism").map(PathBuf::from).unwrap_or(md.join("corpora/judaism"));
        let books: Vec<String> = flag(args, "--hbo-books").unwrap_or_else(|| "GEN,EXO,LEV,NUM,DEU".into()).split(',').map(String::from).collect();
        let br: Vec<&str> = books.iter().map(String::as_str).collect();
        let chs = super::wlc(&judaism, &br)?;
        let jps = sets.iter().find(|s| s.0 == "tanakh").map(|s| s.1.clone()).unwrap_or_default();
        let hb = super::like("hbo", &chs, &jps);
        eprintln!("hbo: {} chapters, {} blocks", chs.len(), hb.len());
        let paths = super::heb::Paths {
            judaism,
            hbo: PathBuf::from(hbo),
            langseg: PathBuf::from(flag(args, "--langseg").context("--langseg <model>")?),
            gparse: flag(args, "--gparse").unwrap_or_else(|| "gparse".into()),
            out: out.join("hbo"),
        };
        let t1 = std::time::Instant::now();
        let runs = super::heb::run(&a, &hb, &paths, threads, &["block", "sentence", "shuffle", "damage"])?;
        eprintln!("hbo in {:.1}s", t1.elapsed().as_secs_f64());
        heb = Some((hb, runs));
    }
    if only.as_ref().is_some_and(|o| !o.iter().any(|x| x == "tanakh")) {
        sets.retain(|s| s.0 != "tanakh");
    }
    // the runs
    let full = Opts { exact: true, normal: true, ..Default::default() };
    let plan: Vec<(String, Opts, Shape, bool)> = {
        let mut p = vec![("block".to_string(), full, Shape::AsIs, false), ("sentence".to_string(), Opts { unit: Unit::Sentence, ..full }, Shape::AsIs, false), ("world".to_string(), Opts { unit: Unit::World, ..full }, Shape::AsIs, false)];
        p.push(("flat".into(), Opts { exact: true, normal: false, ..Default::default() }, Shape::Flat, true));
        p.push(("shuffle".into(), Opts { exact: true, normal: false, ..Default::default() }, Shape::Shuffled, true));
        for l in LAYERS {
            p.push((format!("damage-{}", l.name()), Opts { exact: true, normal: false, damage: Some(l), ..Default::default() }, Shape::AsIs, true));
        }
        p
    };
    // `--units block,sentence,world,controls`: the runs to make (all by default)
    let units: Option<Vec<String>> = flag(args, "--units").map(|x| x.split(',').map(String::from).collect());
    let plan: Vec<_> = plan.into_iter().filter(|(name, _, _, sub)| units.as_ref().is_none_or(|u| u.iter().any(|x| x == name || (*sub && x == "controls")))).collect();
    let mut runs: BTreeMap<(String, String), Out> = BTreeMap::new();
    for (name, o, shape, sub) in &plan {
        for (corpus, bs) in &sets {
            let t1 = std::time::Instant::now();
            let pick: Vec<&Block> = bs.iter().enumerate().filter(|(i, _)| !sub || i % ctrl.max(1) == 0).map(|x| x.1).collect();
            let r = run_blocks(&a, &pick, o, *shape, threads)?;
            eprintln!("{name} / {corpus}: {} blocks, {} sentences, exact {}/{} in {:.1}s", pick.len(), r.total.sents, r.total.exact_ok, r.total.volumes, t1.elapsed().as_secs_f64());
            runs.insert((name.clone(), corpus.clone()), r);
        }
    }
    if let Some((hb, hr)) = &heb {
        let names = [("block", "block"), ("sentence", "sentence"), ("shuffle", "shuffle"), ("damage", "damage-tree")];
        for (hn, rn) in names {
            let mut o = Out::default();
            for (b, t) in hb.iter().zip(&hr[hn]) {
                o.rows.push(row(b, t));
                o.total.merge(t);
            }
            runs.insert((rn.to_string(), "hbo".into()), o);
        }
        // the Hebrew shuffle's counterpart is its block run
        let b = runs[&("block".to_string(), "hbo".to_string())].clone();
        runs.insert(("flat".into(), "hbo".into()), b);
        sets.push(("hbo".into(), hb.clone()));
    }
    write(&out, &sets, &runs, max, ctrl)?;
    eprintln!("done in {:.1}s", t0.elapsed().as_secs_f64());
    Ok(())
}

fn write(out: &Path, sets: &[(String, Vec<Block>)], runs: &BTreeMap<(String, String), Out>, max: usize, ctrl: usize) -> Result<()> {
    let get = |r: &str, c: &str| runs.get(&(r.to_string(), c.to_string()));
    // the training signal: every block's residual by layer and class (the gate's run)
    let mut long = String::from("corpus\tblock\tlayer\tclass\titems\tbits\n");
    let mut wide = String::from("run\tcorpus\tblock\tchapters\twords\tsentences\ttokens\texact\tbits_tokens\tbits_tree\tbits_frames\tbits_world\tbits_meta\tbits\n");
    for ((run, corpus), o) in runs {
        if run != "block" && run != "sentence" && run != "world" {
            continue;
        }
        for r in &o.rows {
            if run == "block" {
                for (l, c, n, b) in &r.cells {
                    let _ = writeln!(long, "{corpus}\t{}\t{}\t{}\t{n}\t{b:.1}", r.block, l.name(), c.name());
                }
            }
            let _ = writeln!(
                wide,
                "{run}\t{corpus}\t{}\t{}\t{}\t{}\t{}\t{}\t{:.1}\t{:.1}\t{:.1}\t{:.1}\t{:.1}\t{:.1}",
                r.block,
                r.chapters,
                r.words,
                r.sents,
                r.tokens,
                r.exact as u8,
                rbits(r, Some(Layer::Tokens), false),
                rbits(r, Some(Layer::Tree), false),
                rbits(r, Some(Layer::Frames), false),
                rbits(r, Some(Layer::World), false),
                rbits(r, None, true),
                rbits(r, None, false)
            );
        }
    }
    std::fs::write(out.join("residual-blocks.tsv"), long)?;
    std::fs::write(out.join("blocks.tsv"), wide)?;
    let mut tot = String::from("run\tcorpus\tblocks\tsentences\ttokens\texact\tbits\tbits_per_block\tbits_per_sentence\tbits_per_token\n");
    for ((run, corpus), o) in runs {
        let t = &o.total;
        let b = total_bits(t);
        let _ = writeln!(tot, "{run}\t{corpus}\t{}\t{}\t{}\t{}\t{b:.1}\t{:.1}\t{:.2}\t{:.3}", t.volumes, t.sents, t.tokens, t.exact_ok, per(b, t.volumes), per(b, t.sents), per(b, t.tokens));
    }
    std::fs::write(out.join("total.tsv"), tot)?;
    // per run, the formats of `roundtrip run` (for `roundtrip compare`)
    for run in ["block", "sentence", "world"] {
        let s: Vec<(String, Tally)> = sets.iter().filter_map(|(c, _)| get(run, c).map(|o| (c.clone(), o.total.clone()))).collect();
        let d = out.join(run);
        std::fs::create_dir_all(&d)?;
        let (r, n) = report::tsv(&s);
        std::fs::write(d.join("residual.tsv"), r)?;
        std::fs::write(d.join("normal.tsv"), n)?;
        let ex: String = s.iter().map(|(n, t)| report::examples(n, t)).collect();
        std::fs::write(d.join("examples.md"), format!("# Round-trip examples, unit {run}\n\n{ex}"))?;
    }
    std::fs::write(out.join("report.md"), report_md(sets, runs, max, ctrl))?;
    Ok(())
}

/// The (layer, class) cells that move most from run `a` to run `b`: the mean Δ over the corpora in bits per sentence.
fn delta_table<'a>(names: &[&str], get: &dyn Fn(&str, &str) -> Option<&'a Out>, a: &str, b: &str) -> String {
    let mut delta: BTreeMap<(Layer, Class), (f64, Vec<String>)> = BTreeMap::new();
    let k = names.iter().filter(|c| get(a, c).is_some() && get(b, c).is_some()).count().max(1) as f64;
    for c in names {
        let (Some(x), Some(y)) = (get(a, c), get(b, c)) else { continue };
        for l in LAYERS {
            for cl in CLASSES {
                let (p, q) = (per(cbits(&x.total, l, cl), x.total.sents), per(cbits(&y.total, l, cl), y.total.sents));
                if p == 0.0 && q == 0.0 {
                    continue;
                }
                let e = delta.entry((l, cl)).or_default();
                e.0 += (q - p) / k;
                e.1.push(format!("{c} {p:.1}→{q:.1}"));
            }
        }
    }
    let mut dv: Vec<_> = delta.into_iter().collect();
    dv.sort_by(|x, y| y.1.0.abs().total_cmp(&x.1.0.abs()).then(x.0.cmp(&y.0)));
    let mut s = format!("| layer | class | mean Δ | per corpus ({a} → {b}) |\n|---|---|---:|---|\n");
    for ((l, c), (d, per_c)) in dv.iter().take(12) {
        let _ = writeln!(s, "| {} | {} | {:+.2} | {} |", l.boundary(), c.name(), d, per_c.join(", "));
    }
    s
}

fn median(mut v: Vec<usize>) -> usize {
    v.sort();
    v.get(v.len() / 2).copied().unwrap_or(0)
}

fn report_md(sets: &[(String, Vec<Block>)], runs: &BTreeMap<(String, String), Out>, max: usize, ctrl: usize) -> String {
    let get = |r: &str, c: &str| runs.get(&(r.to_string(), c.to_string()));
    let names: Vec<&str> = sets.iter().map(|s| s.0.as_str()).collect();
    let mut s = format!("# Round trip by blocks\n\nA block: a whole lesson, a chapter, or a run of whole chapters up to {max} words, cut only at chapter boundaries.\n\n");
    s += "## Blocks\n\n| corpus | chapters | blocks | words | words / block (median / max) | blocks over the size (one chapter) | blocks cut at paragraphs | sentences | tokens |\n|---|---:|---:|---:|---|---:|---:|---:|---:|\n";
    for (c, bs) in sets {
        let t = get("block", c).map(|o| o.total.clone()).unwrap_or_default();
        let w: Vec<usize> = bs.iter().map(|b| b.words).collect();
        let _ = writeln!(
            s,
            "| {c} | {} | {} | {} | {} / {} | {} | {} | {} | {} |",
            bs.iter().map(|b| b.chapters.len()).sum::<usize>(),
            bs.len(),
            w.iter().sum::<usize>(),
            median(w.clone()),
            w.iter().max().copied().unwrap_or(0),
            bs.iter().filter(|b| b.words > max).count(),
            bs.iter().filter(|b| b.para_cut).count(),
            t.sents,
            t.tokens
        );
    }
    s += "\n## The gate (exact mode: every block byte for byte with its residual)\n\n| corpus | block unit | sentence unit | world per sentence | first failure |\n|---|---:|---:|---:|---|\n";
    for c in &names {
        let (b, se, wo) = (get("block", c), get("sentence", c), get("world", c));
        let f = |o: Option<&Out>| o.map(|o| format!("{}/{}", o.total.exact_ok, o.total.volumes)).unwrap_or("—".into());
        let fail = b.and_then(|o| o.total.exact_fail.first().cloned()).unwrap_or_default();
        let _ = writeln!(s, "| {c} | {} | {} | {} | {} |", f(b), f(se), f(wo), fail.chars().take(160).collect::<String>());
    }
    // residual
    s += "\n## Residual of the block unit (bits; ids and headings, `meta`, aside)\n\n| corpus | text ↔ tokens | tokens ↔ tree | tree ↔ frames | frames ↔ world | all, per block | per sentence | per token | corpus total | meta (aside) |\n|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n";
    let mut grand = 0.0;
    for c in &names {
        let Some(o) = get("block", c) else { continue };
        let t = &o.total;
        let b = total_bits(t);
        grand += b;
        let ls: Vec<String> = LAYERS.iter().map(|&l| format!("{:.1}", per(lbits(t, l), t.sents))).collect();
        let _ = writeln!(s, "| {c} | {} | {:.0} | {:.1} | {:.2} | {:.0} | {:.0} |", ls.join(" | "), per(b, t.volumes), per(b, t.sents), per(b, t.tokens), b, t.residual.cells.iter().filter(|(k, _)| k.1 == Class::Meta).map(|(_, v)| v.bits).sum::<f64>() + 0.0);
    }
    let _ = writeln!(s, "\nLayers in bits per sentence. All corpora together: **{:.0} bits** (the number to drive down).\n", grand);
    // the site's pages: per block and per 1 000 words, the markup, and the pieces written from the layers
    if names.iter().any(|c| c.starts_with("site/")) {
        s += "## The site's pages (bits per block and per 1 000 words of prose; markup, CoNLL-U and frames tables included)\n\n| corpus | blocks | pages | exact | words | per block | per 1 000 words | markup | conllu | frames-table | prose layers |\n|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|---:|\n";
        for c in names.iter().filter(|c| c.starts_with("site/")) {
            let Some(o) = get("block", c) else { continue };
            let t = &o.total;
            let b = total_bits(t);
            let words: usize = sets.iter().find(|x| x.0 == *c).map(|x| x.1.iter().map(|b| b.words).sum()).unwrap_or(0);
            let pages: usize = sets.iter().find(|x| x.0 == *c).map(|x| x.1.iter().map(|b| b.chapters.len()).sum()).unwrap_or(0);
            let cl = |cl: Class| t.residual.cells.iter().filter(|(k, _)| k.1 == cl).map(|(_, v)| v.bits).sum::<f64>() + 0.0;
            let (mk, cn, ft) = (cl(Class::Markup), cl(Class::Conllu), cl(Class::FrTable));
            let _ = writeln!(s, "| {c} | {} | {pages} | {} | {words} | {:.0} | {:.0} | {:.0} | {:.0} | {:.0} | {:.0} |", t.volumes, t.exact_ok, per(b, t.volumes), 1000.0 * b / words.max(1) as f64, per(mk, t.volumes), per(cn, t.volumes), per(ft, t.volumes), per(b - mk - cn - ft, t.volumes));
        }
        s += "\nMarkup, conllu, frames-table and the prose layers in bits per block.\n\n**Pieces written from the layers** (`blocks::site_code`): how many, how many need nothing stored but their fixed bits, the bits of storing them whole against the bits of their code:\n\n| corpus | piece | pieces | nothing stored | stored whole, bits | code, bits | saved |\n|---|---|---:|---:|---:|---:|---:|\n";
        for c in names.iter().filter(|c| c.starts_with("site/")) {
            let Some(o) = get("block", c) else { continue };
            for (k, d) in &o.total.derived {
                let _ = writeln!(s, "| {c} | {k} | {} | {} | {:.0} | {:.0} | {:.1}% |", d.pieces, d.free, d.literal_bits, d.code_bits, 100.0 * (1.0 - d.code_bits / d.literal_bits.max(1e-9)));
            }
        }
        s += "\nWhat the CoNLL-U and frames-table codes store, by kind (all site corpora, bits):\n\n| layer | class | kind | items | bits |\n|---|---|---|---:|---:|\n";
        let mut kinds: BTreeMap<(Layer, Class, String), (u64, f64)> = BTreeMap::new();
        for c in names.iter().filter(|c| c.starts_with("site/")) {
            let Some(o) = get("block", c) else { continue };
            for ((l, cl), cell) in &o.total.residual.cells {
                if !matches!(cl, Class::Conllu | Class::FrTable) {
                    continue;
                }
                for (k, n) in &cell.kinds {
                    let e = kinds.entry((*l, *cl, k.clone())).or_default();
                    e.0 += n;
                    e.1 += cell.kind_bits.get(k).copied().unwrap_or(0.0);
                }
            }
        }
        let mut kv: Vec<_> = kinds.into_iter().collect();
        kv.sort_by(|a, b| b.1.1.total_cmp(&a.1.1).then(a.0.cmp(&b.0)));
        for ((l, cl, k), (n, b)) in kv.iter().take(30) {
            let _ = writeln!(s, "| {} | {} | {k} | {n} | {b:.0} |", l.boundary(), cl.name());
        }
        s += "\n";
    }
    // normal
    s += "## Normal mode: the whole block decoded from each start (exact % / chrF)\n\n| corpus | tokens | tree | frames | world | world, sentence unit |\n|---|---|---|---|---|---|\n";
    for c in &names {
        let Some(o) = get("block", c) else { continue };
        if o.total.whole.is_empty() {
            let _ = writeln!(s, "| {c} | — | — | — | — | — |");
            continue;
        }
        let f = |t: &Tally, st: Start| t.whole.get(&st).map(|d| format!("{:.1} / {:.1}", 100.0 * per(d.exact as f64, d.sents), 100.0 * per(d.chrf, d.sents))).unwrap_or("—".into());
        let ws = get("sentence", c).map(|x| f(&x.total, Start::World)).unwrap_or("—".into());
        let _ = writeln!(s, "| {c} | {} | {} | {} | {} | {ws} |", f(&o.total, Start::Tokens), f(&o.total, Start::Tree), f(&o.total, Start::Frames), f(&o.total, Start::World));
    }
    s += "\nSentence by sentence (exact % / chrF, block unit):\n\n| corpus | tree | frames | world |\n|---|---|---|---|\n";
    for c in &names {
        let Some(o) = get("block", c) else { continue };
        let f = |st: Start| o.total.normal.get(&st).map(|d| format!("{:.1} / {:.1}", 100.0 * per(d.exact as f64, d.sents), 100.0 * per(d.chrf, d.sents))).unwrap_or("—".into());
        let _ = writeln!(s, "| {c} | {} | {} | {} |", f(Start::Tree), f(Start::Frames), f(Start::World));
    }
    let _ = STARTS;
    // the unit grows: sentence → block
    s += "\n## From sentence to block: what changes (bits per sentence: sentence unit / world per sentence / block unit)\n\n`sentence`: the reader's discourse, the realizer and the world reset at every sentence (the frames change: a pronoun the reader cannot resolve stays a pronoun, and nothing pays for the identity it loses); `world`: the frames read with the block's discourse, the realizer and the world reset at every sentence (the same frames as the block); `block`: one discourse and one world.\n\n| corpus | tree ↔ frames | frames ↔ world | all | unresolved pronouns / 100 sentences (sentence → block) |\n|---|---|---|---|---|\n";
    for c in &names {
        let (Some(b), Some(se)) = (get("block", c), get("sentence", c)) else { continue };
        let wo = get("world", c);
        let mid = |x: &dyn Fn(&Tally) -> f64| wo.map_or("—".to_string(), |w| format!("{:.1}", x(&w.total)));
        let f = |l: Layer| format!("{:.1} / {} / {:.1}", per(lbits(&se.total, l), se.total.sents), mid(&|t: &Tally| per(lbits(t, l), t.sents)), per(lbits(&b.total, l), b.total.sents));
        let all = format!("{:.1} / {} / {:.1}", per(total_bits(&se.total), se.total.sents), mid(&|t: &Tally| per(total_bits(t), t.sents)), per(total_bits(&b.total), b.total.sents));
        // the Hebrew reader keeps no pronoun entities: nothing to count
        let un = if *c == "hbo" { "—".to_string() } else { format!("{:.1} → {:.1}", 100.0 * per(se.total.unresolved as f64, se.total.sents), 100.0 * per(b.total.unresolved as f64, b.total.sents)) };
        let _ = writeln!(s, "| {c} | {} | {} | {all} | {un} |", f(Layer::Frames), f(Layer::World));
    }
    s += "\nOne world for the block against a world per sentence, the same frames (mean Δ over the corpora, block − world per sentence, bits per sentence; the classes that move most):\n\n";
    s += &delta_table(&names, &get, "world", "block");
    s += "\nThe block against the sentence read alone (block − sentence):\n\n";
    s += &delta_table(&names, &get, "sentence", "block");
    // controls
    let _ = write!(s, "\n## Negative controls (every {ctrl}th block of each corpus)\n\n**Shuffle.** The block's sentences in order (`flat`, a space between them) against the same sentences shuffled (bits per sentence; Hebrew: the verses of the block in order and shuffled):\n\n| corpus | tree ↔ frames | frames ↔ world | world pron/name | world sentence | world links | exact, shuffled |\n|---|---|---|---|---|---|---|\n");
    for c in &names {
        let (Some(fl), Some(sh)) = (get("flat", c), get("shuffle", c)) else { continue };
        let f = |l: Layer| format!("{:.1} → {:.1}", per(lbits(&fl.total, l), fl.total.sents), per(lbits(&sh.total, l), sh.total.sents));
        let g = |cl: Class| format!("{:.1} → {:.1}", per(cbits(&fl.total, Layer::World, cl), fl.total.sents), per(cbits(&sh.total, Layer::World, cl), sh.total.sents));
        let _ = writeln!(s, "| {c} | {} | {} | {} | {} | {} | {}/{} |", f(Layer::Frames), f(Layer::World), g(Class::PronName), g(Class::Sentence), g(Class::Links), sh.total.exact_ok, sh.total.volumes);
    }
    s += "\n**Damage.** The exact decode loses the first residual item of one layer (Hebrew: the whole tree residual): red / blocks that had an item to lose / blocks.\n\n| corpus | text ↔ tokens | tokens ↔ tree | tree ↔ frames | frames ↔ world |\n|---|---|---|---|---|\n";
    for c in &names {
        let cells: Vec<String> = LAYERS
            .iter()
            .map(|l| match get(&format!("damage-{}", l.name()), c) {
                Some(o) => format!("{} / {} / {}", o.total.volumes - o.total.exact_ok, o.rows.iter().filter(|r| r.damaged).count(), o.total.volumes),
                None => "—".into(),
            })
            .collect();
        let _ = writeln!(s, "| {c} | {} |", cells.join(" | "));
    }
    // ranked
    s += "\n## What loses most at block level\n\nEvery (layer, class) of the block unit, by its share of all bits of all corpora (meta aside):\n\n| rank | layer | class | bits | share | bits / sentence, per corpus |\n|---:|---|---|---:|---:|---|\n";
    let mut cells: BTreeMap<(Layer, Class), (f64, Vec<String>)> = BTreeMap::new();
    for c in &names {
        let Some(o) = get("block", c) else { continue };
        for ((l, cl), v) in &o.total.residual.cells {
            if *cl == Class::Meta {
                continue;
            }
            let e = cells.entry((*l, *cl)).or_default();
            e.0 += v.bits;
            e.1.push(format!("{c} {:.1}", per(v.bits, o.total.sents)));
        }
    }
    let all: f64 = cells.values().map(|v| v.0).sum();
    let mut cv: Vec<_> = cells.into_iter().collect();
    cv.sort_by(|a, b| b.1.0.total_cmp(&a.1.0).then(a.0.cmp(&b.0)));
    for (i, ((l, c), (b, pc))) in cv.iter().take(20).enumerate() {
        let _ = writeln!(s, "| {} | {} | {} | {:.0} | {:.1}% | {} |", i + 1, l.boundary(), c.name(), b, 100.0 * b / all.max(1e-9), pc.join(", "));
    }
    s += "\nThe same ranked by bits per sentence, the mean over the corpora (each corpus weighing the same), with the change each asks for:\n\n";
    let st: Vec<(String, Tally)> = names.iter().filter_map(|c| get("block", c).map(|o| (c.to_string(), o.total.clone()))).collect();
    s += &report::ranked(&st);
    s
}

#[cfg(test)]
mod wave32 {
    use super::*;
    #[test]
    fn row_numeric_partitions() {
        let b = super::super::blocks(
            "synthetic",
            &super::super::book_text("Alpha beta.", "x", false),
            10,
        )
        .remove(0);
        let mut t = Tally {
            volumes: 1,
            exact_ok: 2,
            damaged: 2,
            ..Default::default()
        };
        t.residual.add(Layer::Tokens, Class::Meta, 8.0, String::new);
        t.residual
            .add(Layer::Tokens, Class::Spacing, 3.0, String::new);
        let r = row(&b, &t);
        assert!(!r.exact && !r.damaged);
        assert_eq!(rbits(&r, None, true), 8.0);
        assert_eq!(rbits(&r, None, false), 3.0);
        assert_eq!(total_bits(&t), 3.0);
        assert_eq!(per(8.0, 0), 0.0);
        assert_eq!(per(8.0, 2), 4.0);
        assert_eq!(median(vec![8, 2]), 8);
        assert_eq!(median(vec![]), 0);
    }
}
