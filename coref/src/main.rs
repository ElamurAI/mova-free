//! coref — SLM (small language model) coreference (closed part).
//!
//!   coref resolve <in.conllu> [--trees <trees.conllu>]           CoNLL-U with Entity= (CorefUD) to stdout
//!   coref explain <in.conllu> [<doc-id>] [--trees <trees>]       document chains with reasons (sieve id + evidence)
//!   coref score <gold.conllu> <output.conllu> [--exact]           MUC, B³, CEAF-e, CoNLL F1 (with and without singletons)
//!   coref eval <gold.conllu> --out <dir> [--trees <trees>] [--label <name>]   baselines, sieves, pronouns
//!   coref mentions <gold.conllu> [--trees <trees>]                mention detection against gold (for debugging)
//!   coref lexicon count <source> <in.conllu|->...                 noun–pronoun path counts (part file) to stdout
//!   coref lexicon build <out.tsv> <part>... [--label <note>]...   merged table with provenance (`lexicon`)
//!   coref lexicon paths <gold.conllu> [--trees <trees>]           precision of each path on gold coreference
//!   coref lexicon rotate <table.tsv> <out.tsv> <k>              negative control: counts moved k lemmas on
//!   coref lexicon show <table.tsv> <lemma>...                     features a table gives
//!   coref lexicon compare <a.tsv> <b.tsv>                         agreement of two tables' decisions
//!
//! `--trees` — output of `en annotate` on the same tokens: trees are taken from it, comments and gold from the first file.

use std::collections::{BTreeMap, HashMap};
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};
use coref::corefud;
use coref::doc::{self, Document};
use coref::eval::{self, Input};
use coref::score::{self, MSpan, Matching};
use coref::sieve::{Config, Resolver};
use en::conllu::Doc as UdDoc;

struct Args {
    pos: Vec<String>,
    trees: Option<PathBuf>,
    out: Option<PathBuf>,
    label: Option<String>,
    exact: bool,
}

fn args() -> Result<(String, Args)> {
    let mut it = std::env::args().skip(1);
    let cmd = it.next().unwrap_or_default();
    let mut a = Args { pos: Vec::new(), trees: None, out: None, label: None, exact: false };
    while let Some(x) = it.next() {
        match x.as_str() {
            "--trees" => a.trees = Some(PathBuf::from(it.next().context("--trees <file>")?)),
            "--out" => a.out = Some(PathBuf::from(it.next().context("--out <dir>")?)),
            "--label" => a.label = Some(it.next().context("--label <name>")?),
            "--exact" => a.exact = true,
            _ => a.pos.push(x),
        }
    }
    Ok((cmd, a))
}

/// Input: a file and, optionally, trees from another file.
fn load(path: &Path, trees: Option<&Path>) -> Result<(UdDoc, Vec<Document>)> {
    let gold = UdDoc::read(path)?;
    let ud = match trees {
        Some(t) => doc::retree(&gold, &UdDoc::read(t)?).with_context(|| format!("trees {}", t.display()))?,
        None => gold,
    };
    let docs = doc::documents(&ud)?;
    Ok((ud, docs))
}

fn main() -> Result<()> {
    let (cmd, a) = args()?;
    match cmd.as_str() {
        "resolve" if a.pos.len() == 1 => {
            let (mut ud, docs) = load(Path::new(&a.pos[0]), a.trees.as_deref())?;
            let cfg = Config::default();
            let outs: Vec<_> = docs.iter().map(|d| eval::out_mentions(&eval::resolve(d, &cfg))).collect();
            corefud::write(&mut ud, &docs, &outs)?;
            print!("{}", ud.text());
        }
        "explain" if !a.pos.is_empty() => {
            let (_, docs) = load(Path::new(&a.pos[0]), a.trees.as_deref())?;
            let want = a.pos.get(1);
            let mut found = false;
            for d in &docs {
                if want.is_some_and(|w| *w != d.id) {
                    continue;
                }
                found = true;
                explain(d);
            }
            if !found {
                bail!("document \"{}\" not found", want.cloned().unwrap_or_default());
            }
        }
        "score" if a.pos.len() == 2 => {
            let (_, gd) = load(Path::new(&a.pos[0]), None)?;
            let (_, pd) = load(Path::new(&a.pos[1]), None)?;
            if gd.len() != pd.len() {
                bail!("documents: gold {}, output {}", gd.len(), pd.len());
            }
            let gold: Vec<eval::Gold> = gd.iter().map(eval::gold_of).collect::<Result<_>>()?;
            eval::self_check(&gold)?;
            let pred: Vec<Vec<Vec<MSpan>>> = pd.iter().map(|d| eval::gold_of(d).map(|g| g.entities)).collect::<Result<_>>()?;
            let how = if a.exact { Matching::Exact } else { Matching::Head };
            println!("{}", eval::HEADER);
            println!("{}", eval::row("with singletons", &eval::score_all(&gold, &pred, how, true)));
            println!("{}", eval::row("without singletons", &eval::score_all(&gold, &pred, how, false)));
        }
        "eval" if a.pos.len() == 1 && a.out.is_some() => {
            let out = a.out.clone().unwrap();
            std::fs::create_dir_all(&out)?;
            let gold_path = Path::new(&a.pos[0]);
            let (_, gold_docs) = load(gold_path, None)?;
            let (ud, docs) = load(gold_path, a.trees.as_deref())?;
            let label = a.label.clone().unwrap_or_else(|| if a.trees.is_some() { "SLM trees".into() } else { "gold trees".into() });
            let stem = gold_path.file_stem().unwrap_or_default().to_string_lossy().into_owned();
            let tag = if a.trees.is_some() { "slm" } else { "gold" };
            let mut report = format!("# SLM coreference: {stem}, {label}\n");
            let t0 = std::time::Instant::now();
            eval::run(&label, &gold_docs, &Input { label: label.clone(), ud, docs }, &mut report, Some(&out.join(format!("{stem}.{tag}-trees.coref.conllu"))), Some(&out.join(format!("{stem}.{tag}-trees.errors.tsv"))))?;
            report.push_str(&format!("\nMeasurement time: {:.1} s.\n", t0.elapsed().as_secs_f64()));
            let rp = out.join(format!("{stem}.{tag}-trees.md"));
            std::fs::write(&rp, &report)?;
            print!("{report}");
            eprintln!("report: {}", rp.display());
        }
        "mentions" if a.pos.len() == 1 => {
            let gold_path = Path::new(&a.pos[0]);
            let (_, gold_docs) = load(gold_path, None)?;
            let (_, docs) = load(gold_path, a.trees.as_deref())?;
            mention_stats(&gold_docs, &docs)?;
        }
        "lexicon" if !a.pos.is_empty() => lexicon_cmd(&a)?,
        _ => bail!("usage: coref resolve <in.conllu> [--trees f] | explain <in.conllu> [doc-id] [--trees f] | score <gold> <pred> [--exact] | eval <gold> --out <dir> [--trees f] [--label s] | mentions <gold> [--trees f] | lexicon count|build|paths|show|compare …"),
    }
    Ok(())
}

/// Document chains with reasons.
fn explain(d: &Document) {
    let mut r = Resolver::new(d, Config::default());
    r.run();
    let by = r.link_of();
    println!("# {} — {} sentences, {} mentions", d.id, d.sents.len(), r.ms.len());
    let mut stats: BTreeMap<&str, usize> = BTreeMap::new();
    for l in &r.links {
        *stats.entry(l.sieve.id()).or_default() += 1;
    }
    println!("links by sieve: {}", stats.iter().map(|(k, v)| format!("{k} {v}")).collect::<Vec<_>>().join(", "));
    for (k, c) in r.clusters().iter().enumerate() {
        if c.len() < 2 {
            continue;
        }
        let m0 = c[0];
        println!("\n## chain {} ({} mentions): {}", k + 1, c.len(), r.cluster_feats(m0));
        for &m in c {
            let x = &r.ms[m];
            let s = &d.sents[x.span.sent];
            let tag = if x.internal { " [memory only]" } else { "" };
            match by.get(&m) {
                Some(l) => println!("  - {}:{}-{} «{}»{tag} ← [{}] {}", s.id, x.span.start + 1, x.span.end + 1, s.text(x.span.start, x.span.end), l.sieve.id(), l.why),
                None => println!("  - {}:{}-{} «{}»{tag} — first mention", s.id, x.span.start + 1, x.span.end + 1, s.text(x.span.start, x.span.end)),
            }
        }
    }
    println!();
}

/// Mention detection against gold: recall by head UPOS and relation, spurious ones by form, spans.
fn mention_stats(gold_docs: &[Document], docs: &[Document]) -> Result<()> {
    let (mut ng, mut np, mut hit, mut exact) = (0usize, 0usize, 0usize, 0usize);
    let mut miss: HashMap<String, usize> = HashMap::new();
    let mut miss_ex: HashMap<String, Vec<String>> = HashMap::new();
    let mut extra: HashMap<String, usize> = HashMap::new();
    let mut extra_form: HashMap<String, usize> = HashMap::new();
    let mut extra_ex: HashMap<String, Vec<String>> = HashMap::new();
    let mut span_diff: HashMap<String, usize> = HashMap::new();
    let mut span_ex: HashMap<String, Vec<String>> = HashMap::new();
    for (gd, d) in gold_docs.iter().zip(docs) {
        let g = eval::gold_of(gd)?;
        let r = Resolver::new(d, Config { sieves: vec![], ..Config::default() });
        let vis: Vec<usize> = (0..r.ms.len()).filter(|&m| !r.ms[m].internal).collect();
        let pm: Vec<MSpan> = vis.iter().map(|&m| MSpan { span: r.ms[m].span, head: corefud::span_head(&d.sents[r.ms[m].span.sent], r.ms[m].span) }).collect();
        let mt = score::match_mentions(&g.mentions, &pm, Matching::Head);
        ng += g.mentions.len();
        np += pm.len();
        let mut got = vec![false; g.mentions.len()];
        for (j, x) in mt.iter().enumerate() {
            let s = &d.sents[pm[j].span.sent];
            match x {
                Some(i) => {
                    got[*i] = true;
                    hit += 1;
                    let gs = g.mentions[*i].span;
                    if gs == pm[j].span {
                        exact += 1;
                    } else {
                        let k = format!(
                            "{}{}",
                            if pm[j].span.start < gs.start { "left+" } else if pm[j].span.start > gs.start { "left−" } else { "" },
                            if pm[j].span.end > gs.end { "right+" } else if pm[j].span.end < gs.end { "right−" } else { "" }
                        );
                        *span_diff.entry(k.clone()).or_default() += 1;
                        let e = span_ex.entry(k).or_default();
                        if e.len() < 6 {
                            e.push(format!("{}: ours \"{}\" / gold \"{}\"", s.id, s.text(pm[j].span.start, pm[j].span.end), s.text(gs.start, gs.end)));
                        }
                    }
                }
                None => {
                    let h = pm[j].head;
                    let k = format!("{} {}", s.upos(h).map_or("_".into(), |u| u.to_string()), s.rel(h));
                    *extra.entry(k.clone()).or_default() += 1;
                    let e = extra_ex.entry(k).or_default();
                    if e.len() < 4 {
                        e.push(format!("{}: «{}»", s.id, s.text(pm[j].span.start, pm[j].span.end)));
                    }
                    *extra_form.entry(s.low(h)).or_default() += 1;
                }
            }
        }
        for (i, ok) in got.iter().enumerate() {
            if *ok {
                continue;
            }
            let m = g.mentions[i];
            let s = &gd.sents[m.span.sent];
            let k = format!("{} {}", s.upos(m.head).map_or("_".into(), |u| u.to_string()), s.rel(m.head));
            *miss.entry(k.clone()).or_default() += 1;
            let e = miss_ex.entry(k).or_default();
            if e.len() < 4 {
                e.push(format!("{}: \"{}\" (head \"{}\")", s.id, s.text(m.span.start, m.span.end), s.toks[m.head].form));
            }
        }
    }
    println!("gold {ng}, ours {np}, head matches {hit}: recall {:.1}%, precision {:.1}%; exact span among matches {:.1}%", 100.0 * hit as f64 / ng as f64, 100.0 * hit as f64 / np as f64, 100.0 * exact as f64 / hit.max(1) as f64);
    let top = |m: &HashMap<String, usize>, n: usize| {
        let mut v: Vec<(&String, &usize)> = m.iter().collect();
        v.sort_by_key(|(k, c)| (std::cmp::Reverse(**c), (*k).clone()));
        v.into_iter().take(n).map(|(k, c)| (k.clone(), *c)).collect::<Vec<_>>()
    };
    println!("\nmissed gold (head UPOS relation):");
    for (k, c) in top(&miss, 25) {
        println!("  {c:5} {k}   {}", miss_ex[&k].join(" | "));
    }
    println!("\nspurious ours (UPOS relation):");
    for (k, c) in top(&extra, 20) {
        println!("  {c:5} {k}   {}", extra_ex[&k].join(" | "));
    }
    println!("\nspurious ours (head form):");
    for (k, c) in top(&extra_form, 40) {
        print!("{k} {c}; ");
    }
    println!("\n\nmismatched spans:");
    for (k, c) in top(&span_diff, 10) {
        println!("  {c:5} {k}   {}", span_ex[&k].join(" | "));
    }
    Ok(())
}

#[cfg(test)]
mod wave27_tests {
    use super::*;
    #[test]
    fn synthetic_load_preserves_gold_and_rejects_mismatched_trees() {
        let dir = std::env::temp_dir().join(format!("mova-coref-wave27-{}", std::process::id()));
        std::fs::create_dir(&dir).unwrap();
        let gold = dir.join("gold.conllu");
        let trees = dir.join("trees.conllu");
        let text = "# newdoc id = synthetic\n# sent_id = one\n1\tBox\tbox\tNOUN\tNN\tNumber=Sing\t0\troot\t_\tEntity=(1)\n\n";
        std::fs::write(&gold, text).unwrap();
        std::fs::write(&trees, text.replace("Number=Sing", "Number=Plur")).unwrap();
        let (ud, docs) = load(&gold, Some(&trees)).unwrap();
        assert!(ud.text().contains("Number=Plur"));
        assert_eq!(eval::gold_of(&docs[0]).unwrap().mentions.len(), 1);
        std::fs::write(&trees, text.replace("Box", "Crate")).unwrap();
        assert!(
            load(&gold, Some(&trees))
                .unwrap_err()
                .to_string()
                .contains("trees")
        );
        std::fs::remove_dir_all(dir).unwrap();
    }
}

/// `coref lexicon …`: the noun lexicon learned from Mova's parses (`coref::lexicon`).
fn lexicon_cmd(a: &Args) -> Result<()> {
    use coref::lexicon::{self, Counts};
    let sub = a.pos[0].as_str();
    let rest = &a.pos[1..];
    match sub {
        "count" if rest.len() >= 2 => {
            let mut c = Counts::default();
            for input in &rest[1..] {
                lexicon::count_stream(&rest[0], input, &mut c)?;
            }
            print!("{}", c.write_part());
        }
        "build" if rest.len() >= 2 => {
            let mut c = Counts::default();
            for p in &rest[1..] {
                c.read_part(&std::fs::read_to_string(p).with_context(|| p.clone())?)?;
            }
            let header: Vec<String> = a.label.iter().cloned().collect();
            std::fs::write(&rest[0], lexicon::build(&c, &header))?;
            let t = lexicon::load(Path::new(&rest[0])).or_else(|_| {
                // a CC BY-SA table is written for the comparison; read it back as internal
                lexicon::parse_table(&std::fs::read_to_string(&rest[0])?)
            })?;
            let decided = t.rows.keys().filter(|l| t.feats(l).is_some()).count();
            eprintln!("{} lemmas, {} with features → {}", t.rows.len(), decided, rest[0]);
        }
        "paths" if rest.len() == 1 => {
            let gold_path = Path::new(&rest[0]);
            let (_, gold_docs) = load(gold_path, None)?;
            let (_, docs) = load(gold_path, a.trees.as_deref())?;
            // precision of each path: the noun and the pronoun are heads of mentions of one gold entity
            let mut st: BTreeMap<(&str, &str), (usize, usize)> = BTreeMap::new();
            let mut wrong: BTreeMap<&str, Vec<String>> = BTreeMap::new();
            for (gd, d) in gold_docs.iter().zip(&docs) {
                let g = eval::gold_of(gd)?;
                let mut ent: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
                for (i, m) in g.mentions.iter().enumerate() {
                    ent.entry((m.span.sent, m.head)).or_default().push(g.entity_of[i]);
                }
                for (si, s) in d.sents.iter().enumerate() {
                    for h in lexicon::hits(s) {
                        let (en_, ep) = (ent.get(&(si, h.noun)), ent.get(&(si, h.pron)));
                        let ok = matches!((en_, ep), (Some(x), Some(y)) if x.iter().any(|e| y.contains(e)));
                        let e = st.entry((h.path, lexicon::CLASSES[h.class])).or_default();
                        e.0 += 1;
                        if ok {
                            e.1 += 1;
                        } else {
                            let w = wrong.entry(h.path).or_default();
                            if w.len() < 5 {
                                w.push(format!("{}: «{}» ~ «{}»", s.id, s.toks[h.noun].form, s.toks[h.pron].form));
                            }
                        }
                    }
                }
            }
            println!("path\tclass\tpairs\tcoreferent\tprecision");
            let mut tot: BTreeMap<&str, (usize, usize)> = BTreeMap::new();
            for ((p, c), (n, ok)) in &st {
                println!("{p}\t{c}\t{n}\t{ok}\t{:.3}", *ok as f64 / (*n).max(1) as f64);
                let t = tot.entry(p).or_default();
                t.0 += n;
                t.1 += ok;
            }
            for (p, (n, ok)) in &tot {
                println!("{p}\tall\t{n}\t{ok}\t{:.3}", *ok as f64 / (*n).max(1) as f64);
            }
            for (p, w) in &wrong {
                println!("# not coreferent in gold, {p}: {}", w.join(" | "));
            }
        }
        "rotate" if rest.len() == 3 => {
            // the negative control: the counts moved k lemmas on (`lexicon::rotate`)
            let k: usize = rest[2].parse()?;
            let t = lexicon::parse_table(&std::fs::read_to_string(&rest[0])?)?;
            let r = lexicon::rotate(&t, k);
            std::fs::write(&rest[1], lexicon::write_counts(&r, &[format!("negative control: the counts of {} moved {k} lemmas on", rest[0])]))?;
        }
        "show" if rest.len() >= 2 => {
            let t = lexicon::parse_table(&std::fs::read_to_string(&rest[0])?)?;
            for l in &rest[1..] {
                match t.feats(l) {
                    Some(f) => println!("{l}\tgender {}\tanimacy {}\t{}", f.gender, f.animacy, f.why),
                    None => println!("{l}\tunknown\t{:?}", t.rows.get(l.as_str()).map(|r| r.counts)),
                }
            }
        }
        "compare" if rest.len() == 2 => {
            let ta = lexicon::parse_table(&std::fs::read_to_string(&rest[0])?)?;
            let tb = lexicon::parse_table(&std::fs::read_to_string(&rest[1])?)?;
            let (mut both, mut same_a, mut same_g, mut only_a, mut only_b) = (0usize, 0usize, 0usize, 0usize, 0usize);
            let mut diff: Vec<String> = Vec::new();
            let keys: std::collections::BTreeSet<&String> = ta.rows.keys().chain(tb.rows.keys()).collect();
            for l in keys {
                match (ta.feats(l), tb.feats(l)) {
                    (Some(x), Some(y)) => {
                        both += 1;
                        same_a += (x.animacy == y.animacy) as usize;
                        same_g += (x.gender == y.gender) as usize;
                        if (x.animacy != y.animacy || x.gender != y.gender) && diff.len() < 40 {
                            diff.push(format!("{l} a:{}/{} g:{}/{}", x.animacy, y.animacy, x.gender, y.gender));
                        }
                    }
                    (Some(_), None) => only_a += 1,
                    (None, Some(_)) => only_b += 1,
                    _ => {}
                }
            }
            println!("decided in both {both}: same animacy {same_a} ({:.1}%), same gender {same_g} ({:.1}%); only in A {only_a}, only in B {only_b}", 100.0 * same_a as f64 / both.max(1) as f64, 100.0 * same_g as f64 / both.max(1) as f64);
            println!("differences (animacy bits 1 anim 2 inan; gender 1 masc 2 fem 4 neut): {}", diff.join("; "));
        }
        _ => bail!("usage: coref lexicon count <source> <in.conllu|->... | build <out.tsv> <part>... [--label note] | paths <gold> [--trees f] | rotate <table> <out> <k> | show <table> <lemma>... | compare <a> <b>"),
    }
    Ok(())
}
