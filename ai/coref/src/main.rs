//! coref — SLM (small language model) coreference (closed part).
//!
//!   coref resolve <in.conllu> [--trees <trees.conllu>]           CoNLL-U with Entity= (CorefUD) to stdout
//!   coref explain <in.conllu> [<doc-id>] [--trees <trees>]       document chains with reasons (sieve id + evidence)
//!   coref score <gold.conllu> <output.conllu> [--exact]           MUC, B³, CEAF-e, CoNLL F1 (with and without singletons)
//!   coref eval <gold.conllu> --out <dir> [--trees <trees>] [--label <name>]   baselines, sieves, pronouns
//!   coref mentions <gold.conllu> [--trees <trees>]                mention detection against gold (for debugging)
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
        _ => bail!("usage: coref resolve <in.conllu> [--trees f] | explain <in.conllu> [doc-id] [--trees f] | score <gold> <pred> [--exact] | eval <gold> --out <dir> [--trees f] [--label s] | mentions <gold> [--trees f]"),
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
