//! kg — the Mova knowledge graph: simple sentences → facts (a fact vertex + edges to the sentence members).
//!
//!   kg tatoeba-uk <tatoeba-dir> <dict_corp_vis.txt.bz2> <store>   parse Tatoeba uk sentences into the graph
//!   kg show <store> <tatoeba-id>...                                 a sentence's fact in human-readable form
//!   kg stats <store>                                                size, relations, flags, compression
//!   kg sub <store> <comma-separated flags> <out>                    subgraph by mask, compressed
//!   kg parse <vesum.bz2> <store> <in.tsv> <out.tsv>                 parse arbitrary `id\ttext` lines
//!                                                                   (names from `<store>/names.tsv`)
//!
//! Store: `heaps.tsv`, `keys/<heap>.tsv` (stable ids), `edges/` (columns, delta+zstd),
//! `sentences.tsv`, reports `report.txt`, `samples.tsv`, `render-fail.tsv`, `unknown.tsv`.

mod cols;
mod textgen;
mod gram;
mod id;
mod morph;
mod uk;

use std::collections::{BTreeMap, HashMap, HashSet};
use std::fs::{self, File};
use std::io::{BufRead, BufReader, BufWriter, Write};
use std::path::Path;
use std::time::Instant;

use anyhow::{Context, Result, bail};
use bzip2::read::MultiBzDecoder;

use cols::{Edges, flag};
use gram::Gram;
use id::{Id, Kind, Registry};
use morph::Morph;

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let a: Vec<&str> = args.iter().map(String::as_str).collect();
    match a.as_slice() {
        ["tatoeba-uk", tatoeba, vesum, store] => tatoeba_uk(Path::new(tatoeba), Path::new(vesum), Path::new(store)),
        ["show", store, ids @ ..] if !ids.is_empty() => show(Path::new(store), ids),
        ["stats", store] => stats(Path::new(store)),
        ["sub", store, mask, out] => sub(Path::new(store), mask, Path::new(out)),
        ["parse", vesum, store, input, out] => parse_lines(Path::new(vesum), Path::new(store), Path::new(input), Path::new(out)),
        _ => bail!("usage: kg tatoeba-uk <tatoeba-dir> <vesum.bz2> <store> | show <store> <id>... | stats <store> | sub <store> <mask> <out>"),
    }
}

fn read_tatoeba(path: &Path) -> Result<Vec<(u64, String)>> {
    let mut out = Vec::new();
    for line in BufReader::new(MultiBzDecoder::new(File::open(path)?)).lines() {
        let line = line?;
        let mut p = line.splitn(3, '\t');
        let (Some(id), Some(_), Some(text)) = (p.next(), p.next(), p.next()) else { bail!("malformed Tatoeba line: {line}") };
        out.push((id.parse()?, text.to_string()));
    }
    Ok(out)
}

fn tatoeba_uk(tatoeba: &Path, vesum: &Path, store: &Path) -> Result<()> {
    let t0 = Instant::now();
    let sents = read_tatoeba(&tatoeba.join("ukr_sentences.tsv.bz2"))?;
    let vocab: HashSet<String> =
        sents.iter().flat_map(|(_, t)| uk::tokenize(t).into_iter().filter(|t| t.word).map(|t| t.norm)).collect();
    let morph = Morph::load(vesum, &vocab)?;
    let t_morph = t0.elapsed();
    // names: forms that mid-sentence are capitalized more often than not ("Том" yes, "Люди" no)
    let mut case_mid: HashMap<String, (u32, u32)> = HashMap::new();
    for (_, t) in &sents {
        for tok in uk::tokenize(t).into_iter().filter(|t| t.word).skip(1) {
            let e = case_mid.entry(tok.norm.clone()).or_default();
            if tok.text.chars().next().is_some_and(char::is_uppercase) {
                e.0 += 1;
            } else {
                e.1 += 1;
            }
        }
    }
    let names: HashSet<String> = case_mid.into_iter().filter(|(_, (up, low))| up > low).map(|(w, _)| w).collect();
    let mut nf = BufWriter::new(File::create(store.join("names.tsv"))?);
    let mut sorted: Vec<&String> = names.iter().collect();
    sorted.sort();
    for n in sorted {
        writeln!(nf, "{n}")?;
    }
    drop(nf);

    let mut reg = Registry::open(store)?;
    let h_rel = reg.heap(Kind::Service, "rel")?;
    let h_word = reg.heap(Kind::Global, "word:uk")?;
    let h_sent = reg.heap(Kind::Global, "sentence:tatoeba")?;
    let h_fact = reg.heap(Kind::Global, "fact:tatoeba:uk")?;
    let rel_prov = reg.id(h_rel, "from_sentence")?.local();

    let mut edges = Edges::default();
    let mut rejects: BTreeMap<String, usize> = BTreeMap::new();
    let mut unknown: HashMap<String, usize> = HashMap::new();
    let mut fails: BTreeMap<String, usize> = BTreeMap::new();
    let (mut parsed, mut checked) = (0usize, 0usize);
    let mut samples = BufWriter::new(File::create(store.join("samples.tsv"))?);
    let mut render_fail = BufWriter::new(File::create(store.join("render-fail.tsv"))?);
    let mut texts = BufWriter::new(File::create(store.join("sentences.tsv"))?);
    let mut rejected = BufWriter::new(File::create(store.join("rejects.tsv"))?);
    let mut kept: Vec<(u64, &str, uk::Fact)> = Vec::new();
    for (sid, text) in &sents {
        let f = match uk::parse(text, &morph, &names) {
            Ok(f) => f,
            Err(r) => {
                if let uk::Reject::Unknown(w) = &r {
                    *unknown.entry(w.clone()).or_default() += 1;
                }
                *rejects.entry(r.code()).or_default() += 1;
                writeln!(rejected, "{sid}\t{}\t{text}", r.code())?;
                continue;
            }
        };
        parsed += 1;
        let mut fl = flag::SRC_TATOEBA | flag::LANG_UK;
        match uk::render_check(text, &f, &morph) {
            Ok(()) => {
                fl |= flag::CHECKED;
                checked += 1;
            }
            Err(e) => {
                *fails.entry(e.split(' ').next().unwrap_or("").to_string()).or_default() += 1;
                writeln!(render_fail, "{sid}\t{text}\t{e}")?;
            }
        }
        let s = reg.id(h_sent, &sid.to_string())?;
        let fid = reg.id(h_fact, &sid.to_string())?;
        edges.push(fid.0, rel_prov, s.0, f.gram.0, 0, fl | flag::LAYER_PROV);
        let mut shown = Vec::new();
        for m in &f.members {
            let r = reg.id(h_rel, &m.rel)?.local();
            let w = reg.id(h_word, &m.lemma)?;
            edges.push(fid.0, r, w.0, f.gram.0, m.gram.0, fl | flag::LAYER_FACT);
            shown.push(format!("{}→{} [{}]", m.rel, m.lemma, m.gram));
        }
        writeln!(texts, "{sid}\t{text}")?;
        kept.push((*sid, text.as_str(), f.clone()));
        if parsed % 97 == 1 {
            writeln!(samples, "{sid}\t{text}\t{}\t{}", fl & flag::CHECKED != 0, shown.join(" · "))?;
        }
    }
    edges.sort();
    let (raw, packed) = edges.write(&store.join("edges"))?;
    reg.save()?;

    // round trip: learn order on 90% of parsed sentences, generate and compare on the held-out 10% (sid % 10 == 0)
    let mut om = textgen::OrderModel::default();
    for (sid, _, f) in &kept {
        if sid % 10 != 0 {
            om.observe(f);
        }
    }
    // form frequencies come only from training sentences (sid % 10 != 0), so as not to peek at the test set
    let mut freq = textgen::Freq::new();
    for (sid, t) in &sents {
        if sid % 10 != 0 {
            for tok in uk::tokenize(t).into_iter().filter(|t| t.word) {
                *freq.word.entry(tok.norm).or_default() += 1;
            }
        }
    }
    for (sid, t, f) in &kept {
        if sid % 10 != 0 {
            let words: Vec<String> = uk::tokenize(t).into_iter().filter(|t| t.word).map(|t| t.norm).collect();
            freq.observe(f, &words);
        }
    }
    let words_of = |s: &str| -> Vec<String> { uk::tokenize(s).into_iter().filter(|t| t.word).map(|t| t.norm).collect() };
    let mut rt: BTreeMap<&str, usize> = BTreeMap::new();
    let mut rt_out = BufWriter::new(File::create(store.join("roundtrip.tsv"))?);
    for (sid, text, f) in kept.iter().filter(|(sid, _, _)| sid % 10 == 0) {
        let kind = match textgen::generate(f, &morph, &freq, &om) {
            None => "not generated",
            Some(g) => {
                let (a, b) = (words_of(text), words_of(&g));
                let kind = if a == b {
                    "verbatim"
                } else {
                    let (mut sa, mut sb) = (a.clone(), b.clone());
                    sa.sort();
                    sb.sort();
                    if sa == sb { "same words, different order" } else { "different words" }
                };
                writeln!(rt_out, "{kind}\t{sid}\t{text}\t{g}")?;
                kind
            }
        };
        *rt.entry(kind).or_default() += 1;
    }
    let rt_total: usize = rt.values().sum();

    let mut unk: Vec<(&String, &usize)> = unknown.iter().collect();
    unk.sort_by(|a, b| b.1.cmp(a.1).then(a.0.cmp(b.0)));
    let mut u = BufWriter::new(File::create(store.join("unknown.tsv"))?);
    for (w, c) in &unk {
        writeln!(u, "{c}\t{w}")?;
    }
    let mut rep = String::new();
    use std::fmt::Write as _;
    writeln!(rep, "Tatoeba uk sentences: {}", sents.len())?;
    writeln!(rep, "sentence vocabulary: {} forms; found in VESUM: {} forms, synthesis: {} (lemma, grammar) pairs; names in corpus: {}", vocab.len(), morph.forms.len(), morph.synth.len(), names.len())?;
    writeln!(rep, "parsed as a simple sentence: {parsed} ({:.1}%)", 100.0 * parsed as f64 / sents.len() as f64)?;
    writeln!(rep, "  of these passed the completeness check (graph + grammar reproduce all words; this is not role correctness): {checked} ({:.1}% of parsed)", 100.0 * checked as f64 / parsed.max(1) as f64)?;
    for (k, v) in &fails {
        writeln!(rep, "  check failure «{k}…»: {v}")?;
    }
    writeln!(rep, "round trip (graph → sentence) on the held-out 10% of parsed ({rt_total}):")?;
    for (k, v) in &rt {
        writeln!(rep, "  {k}: {v} ({:.1}%)", 100.0 * *v as f64 / rt_total.max(1) as f64)?;
    }
    writeln!(rep, "rejections:")?;
    for (k, v) in &rejects {
        writeln!(rep, "  {k}: {v}")?;
    }
    writeln!(rep, "edges: {}; raw columns {} B ({:.1} B/edge), compressed {} B ({:.2} B/edge)", edges.len(), raw, raw as f64 / edges.len().max(1) as f64, packed, packed as f64 / edges.len().max(1) as f64)?;
    writeln!(rep, "time: VESUM {:.1} s, total {:.1} s", t_morph.as_secs_f64(), t0.elapsed().as_secs_f64())?;
    fs::write(store.join("report.txt"), &rep)?;
    print!("{rep}");
    Ok(())
}

fn show(store: &Path, ids: &[&str]) -> Result<()> {
    let mut reg = Registry::open(store)?;
    let edges = Edges::read(&store.join("edges"))?;
    let h_fact = reg.heap_by_name("fact:tatoeba:uk").context("no fact heap")?;
    let h_rel = reg.heap_by_name("rel").context("no relation heap")?;
    let texts: HashMap<String, String> = BufReader::new(File::open(store.join("sentences.tsv"))?)
        .lines()
        .map_while(Result::ok)
        .filter_map(|l| l.split_once('\t').map(|(a, b)| (a.to_string(), b.to_string())))
        .collect();
    for sid in ids {
        println!("«{}» (Tatoeba {sid})", texts.get(*sid).map_or("?", String::as_str));
        let Some(fid) = reg.find(h_fact, sid)? else {
            println!("  no fact (sentence not parsed)");
            continue;
        };
        println!("  fact {fid}");
        for i in edges.out(fid.0) {
            let rel = reg.key(Id::new(h_rel, edges.rel[i]))?;
            let dst = Id(edges.dst[i]);
            println!("  ─{rel}→ {} [{}] ({dst}; flags {:#x})", reg.key(dst)?, Gram(edges.gd[i]), edges.flags[i]);
        }
    }
    Ok(())
}

fn stats(store: &Path) -> Result<()> {
    let mut reg = Registry::open(store)?;
    let edges = Edges::read(&store.join("edges"))?;
    let h_rel = reg.heap_by_name("rel").context("no relation heap")?;
    println!("edges: {}", edges.len());
    let mut by_rel: HashMap<u32, usize> = HashMap::new();
    for &r in &edges.rel {
        *by_rel.entry(r).or_default() += 1;
    }
    let mut v: Vec<_> = by_rel.into_iter().collect();
    v.sort_by(|a, b| b.1.cmp(&a.1));
    for (r, c) in v.iter().take(25) {
        println!("  {c:>8}  {}", reg.key(Id::new(h_rel, *r))?);
    }
    let mut by_flags: BTreeMap<u32, usize> = BTreeMap::new();
    for &f in &edges.flags {
        *by_flags.entry(f).or_default() += 1;
    }
    for (f, c) in by_flags {
        let names: Vec<&str> = flag::NAMES.iter().filter(|(b, _)| f & b != 0).map(|(_, n)| *n).collect();
        println!("  flags {:<40} {c}", names.join(","));
    }
    let size: u64 = fs::read_dir(store.join("edges"))?.filter_map(|e| e.ok()?.metadata().ok()).map(|m| m.len()).sum();
    println!("on disk: {size} B ({:.2} B/edge)", size as f64 / edges.len().max(1) as f64);
    Ok(())
}

fn sub(store: &Path, mask: &str, out: &Path) -> Result<()> {
    let m = flag::parse(mask).with_context(|| format!("unknown flags in «{mask}»"))?;
    let t0 = Instant::now();
    let edges = Edges::read(&store.join("edges"))?;
    let t_read = t0.elapsed();
    let t1 = Instant::now();
    let s = edges.filter(m);
    let t_filter = t1.elapsed();
    let (raw, packed) = s.write(out)?;
    println!(
        "subgraph {mask}: {} of {} edges; filter {:.2} ms (read {:.1} ms); raw {raw} B, compressed {packed} B ({:.2} B/edge)",
        s.len(),
        edges.len(),
        t_filter.as_secs_f64() * 1e3,
        t_read.as_secs_f64() * 1e3,
        packed as f64 / s.len().max(1) as f64
    );
    Ok(())
}

/// Parse arbitrary `id\ttext` lines with the same parser (names from a previous build).
/// Output: `id\tstatus\ttext\tmembers`, status `ok` or the rejection reason.
fn parse_lines(vesum: &Path, store: &Path, input: &Path, out: &Path) -> Result<()> {
    let rows: Vec<(String, String)> = BufReader::new(File::open(input)?)
        .lines()
        .map_while(Result::ok)
        .filter_map(|l| l.split_once('\t').map(|(a, b)| (a.to_string(), b.to_string())))
        .collect();
    let names: HashSet<String> = BufReader::new(File::open(store.join("names.tsv"))?).lines().map_while(Result::ok).collect();
    let vocab: HashSet<String> =
        rows.iter().flat_map(|(_, t)| uk::tokenize(t).into_iter().filter(|t| t.word).map(|t| t.norm)).collect();
    let morph = Morph::load(vesum, &vocab)?;
    let mut w = BufWriter::new(File::create(out)?);
    let (mut ok, mut total) = (0, 0);
    let mut why: BTreeMap<String, usize> = BTreeMap::new();
    for (id, text) in &rows {
        total += 1;
        match uk::parse(text, &morph, &names) {
            Ok(f) => {
                ok += 1;
                let shown: Vec<String> = f.members.iter().map(|m| format!("{}→{} [{}]", m.rel, m.lemma, m.gram)).collect();
                writeln!(w, "{id}\tok\t{text}\t{}", shown.join(" · "))?;
            }
            Err(r) => {
                *why.entry(r.code()).or_default() += 1;
                writeln!(w, "{id}\t{}\t{text}\t", r.code())?;
            }
        }
    }
    println!("parsed {ok} of {total} ({:.1}%)", 100.0 * ok as f64 / total.max(1) as f64);
    for (k, v) in why {
        println!("  {k}: {v}");
    }
    Ok(())
}
