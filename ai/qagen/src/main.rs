//! qagen — training questions and answers from the LLM over fairy-tale paragraphs.
//!
//!   qagen select <db> <run dir> [--paras 300] [--batch 20] [--seed 26] [--min-words 40]
//!                [--max-words 250] [--min-sents 2] [--max-sents 16] [--rule 2] [--exclude <dir>/paras.jsonl]
//!       selection of fairy-tale paragraphs from the database → paras.jsonl (with batches), pick.json; --exclude — without the paragraphs
//!       of another run (the remaining eligible ones)
//!   qagen prompt <dir> <batch>           batch prompt without calling the LLM — for review
//!   qagen run    <dir> [--batches a,b] [--jobs 5] [--max-calls 20]
//!       LLM (claude -p, Opus high) per batch → raw/, calls.jsonl; one retry only on a format rejection
//!   qagen gate   <dir>                   gates over raw outputs → qa.tsv, rejects.tsv, params.json
//!   qagen sample <dir> [--n 20] [--seed 7] [--weak 0.5]
//!       random accepted pairs with paragraphs → sample.md; with --weak — explicit pairs with little support for the answer
//!       in the anchors (suspected wrong anchor) → weak.md
//!
//! Correcting the pilot for rule v2 (explicit answer — a verbatim span; `src/fix.rs`):
//!   qagen fix select <pilot dir> <dir> [--batch 40]   → items.jsonl, paras.jsonl, select.json, doubt.tsv
//!   qagen fix prompt <dir> <batch>
//!   qagen fix run    <dir> [--jobs 5] [--max-calls 10]
//!   qagen fix gate   <dir> --replaces <pilot run>   → qa.tsv, rejects.tsv, changes.md, params.json
//!
//! Then into the database: `db qa <db> <dir>/qa.tsv --run <name> --params <dir>/params.json --path <dir>`
//! (corrections — also `--replaces <pilot run>`).
//! Variables: PRAG_MODEL (claude-opus-5-5), PRAG_EFFORT (high), DUCKDB (~/bin/duckdb),
//! QAGEN_EN_MODEL (data/en/models/ud-ewt-eslspok.bin — lemmas for the "doubtful implicit" mark).

use std::collections::BTreeMap;
use std::path::Path;

use anyhow::{Context, Result, bail};

use qagen::select::{self, Pick, load_paras, load_pick};
use qagen::{fix, lemma, prompt, report, run};

fn usage() -> Result<()> {
    bail!(
        "usage: qagen select <db> <dir> [--paras N] [--batch N] [--seed S] [--min-words N] [--max-words N] [--min-sents N] [--max-sents N] [--rule 1|2] [--exclude paras.jsonl] | \
         qagen prompt <dir> <batch> | qagen run <dir> [--batches a,b,…] [--jobs N] [--max-calls N] | qagen gate <dir> | qagen sample <dir> [--n N] [--seed S] [--weak share] | \
         qagen fix select <pilot dir> <dir> [--batch N] | qagen fix prompt <dir> <batch> | qagen fix run <dir> [--jobs N] [--max-calls N] | qagen fix gate <dir> --replaces <run>"
    )
}

/// Options `--key value` after the positional arguments.
fn opts(a: &[String]) -> Result<(Vec<String>, BTreeMap<String, String>)> {
    let mut pos = Vec::new();
    let mut o = BTreeMap::new();
    let mut i = 0;
    while i < a.len() {
        if let Some(k) = a[i].strip_prefix("--") {
            let v = a.get(i + 1).with_context(|| format!("--{k}: missing value"))?;
            o.insert(k.to_string(), v.clone());
            i += 2;
        } else {
            pos.push(a[i].clone());
            i += 1;
        }
    }
    Ok((pos, o))
}

fn num<T: std::str::FromStr>(o: &BTreeMap<String, String>, k: &str, d: T) -> Result<T> {
    match o.get(k) {
        Some(v) => v.parse::<T>().ok().with_context(|| format!("--{k} {v}")),
        None => Ok(d),
    }
}

fn batches_opt(o: &BTreeMap<String, String>) -> Result<Vec<usize>> {
    match o.get("batches") {
        Some(s) => s.split(',').map(|x| x.trim().parse::<usize>().with_context(|| format!("--batches {s}"))).collect(),
        None => Ok(Vec::new()),
    }
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(cmd) = args.first().map(String::as_str) else { return usage() };
    let (pos, o) = opts(&args[1..])?;
    match (cmd, pos.len()) {
        ("select", 2) => {
            let (db, dir) = (Path::new(&pos[0]), Path::new(&pos[1]));
            let d = Pick::default();
            let pick = Pick {
                paras: num(&o, "paras", d.paras)?,
                batch: num(&o, "batch", d.batch)?,
                seed: num(&o, "seed", d.seed)?,
                min_words: num(&o, "min-words", d.min_words)?,
                max_words: num(&o, "max-words", d.max_words)?,
                min_sents: num(&o, "min-sents", d.min_sents)?,
                max_sents: num(&o, "max-sents", d.max_sents)?,
                rule: num(&o, "rule", d.rule)?,
                exclude: o.get("exclude").cloned().into_iter().collect(),
            };
            if pick.batch == 0 {
                bail!("--batch 0");
            }
            pick.rule()?;
            let mut skip = std::collections::HashSet::new();
            for f in &pick.exclude {
                let text = std::fs::read_to_string(f).with_context(|| f.clone())?;
                for l in text.lines().filter(|l| !l.trim().is_empty()) {
                    skip.insert(serde_json::from_str::<select::Para>(l).with_context(|| f.clone())?.key());
                }
            }
            if dir.join("raw").join("b000.txt").exists() {
                bail!("{}: LLM outputs already exist — a new selection would shift the batches; use a new run dir", dir.display());
            }
            // fairy tales are public domain: commercial-license gate for training a closed model
            prag::data::license_gate(&[prag::data::Src::Tale])?;
            std::fs::create_dir_all(dir)?;
            let all = select::read_paras(db, dir)?;
            let idx = select::select(&all, &pick, &skip);
            let mut out = String::new();
            let mut per: BTreeMap<String, (usize, usize, usize, std::collections::BTreeSet<&str>)> = BTreeMap::new();
            for (j, &i) in idx.iter().enumerate() {
                let mut p = all[i].clone();
                p.batch = j / pick.batch;
                out.push_str(&serde_json::to_string(&p)?);
                out.push('\n');
                let e = per.entry(format!("{} {}", p.book, p.author)).or_default();
                e.0 += 1;
                e.1 += p.dialogue as usize;
                e.2 += p.sents.len();
                e.3.insert(all[i].doc.as_str());
            }
            std::fs::write(dir.join("paras.jsonl"), out)?;
            std::fs::write(dir.join("pick.json"), serde_json::to_string_pretty(&pick)?)?;
            let eligible = all.iter().filter(|p| pick.eligible(p)).count();
            println!(
                "paragraphs in database {}, eligible {eligible}, excluded of them {}, selected {} in {} batches (rule v{})",
                all.len(),
                all.iter().filter(|p| pick.eligible(p) && skip.contains(&p.key())).count(),
                idx.len(),
                idx.len().div_ceil(pick.batch),
                pick.rule
            );
            for (b, (n, dl, s, docs)) in &per {
                println!("  {b}: paragraphs {n} (with dialogue {dl}), sentences {s}, tales {}", docs.len());
            }
            Ok(())
        }
        ("prompt", 2) => {
            let dir = Path::new(&pos[0]);
            let paras = load_paras(dir)?;
            let b: usize = pos[1].parse().context("batch")?;
            let all = run::batches(&paras);
            let its = all.get(&b).with_context(|| format!("no batch {b}"))?;
            print!("{}", prompt::prompt(its, load_pick(dir)?.rule()?));
            Ok(())
        }
        ("run", 1) => {
            let dir = Path::new(&pos[0]);
            let paras = load_paras(dir)?;
            let job = run::Tales::new(&paras, load_pick(dir)?.rule()?);
            run::run(dir, &job, &batches_opt(&o)?, num(&o, "jobs", 5)?, num(&o, "max-calls", run::MAX_CALLS)?)
        }
        ("gate", 1) => {
            let dir = Path::new(&pos[0]);
            let paras = load_paras(dir)?;
            let p = report::collect(dir, &paras, &load_pick(dir)?)?;
            println!("{}", serde_json::to_string_pretty(&p)?);
            Ok(())
        }
        ("sample", 1) => {
            let dir = Path::new(&pos[0]);
            let paras = load_paras(dir)?;
            let weak = o.get("weak").map(|v| v.parse::<f64>().with_context(|| format!("--weak {v}"))).transpose()?;
            println!("{}", report::sample(dir, &paras, load_pick(dir)?.rule()?, num(&o, "n", 20)?, num(&o, "seed", 7)?, weak)?);
            Ok(())
        }
        ("fix", _) => fix_cmd(&pos, &o),
        _ => usage(),
    }
}

/// `qagen fix …` — correcting the pilot for rule v2.
fn fix_cmd(pos: &[String], o: &BTreeMap<String, String>) -> Result<()> {
    match (pos.first().map(String::as_str), pos.len()) {
        (Some("select"), 3) => {
            let v = fix::select(Path::new(&pos[1]), Path::new(&pos[2]), num(o, "batch", fix::BATCH)?, &lemma::load()?)?;
            println!("{}", serde_json::to_string_pretty(&v)?);
            Ok(())
        }
        (Some("prompt"), 3) => {
            let dir = Path::new(&pos[1]);
            let (paras, items) = (load_paras(dir)?, fix::load_items(dir)?);
            let fx = fix::Fixes::new(&paras, &items);
            let b: usize = pos[2].parse().context("batch")?;
            let (ps, its) = fx.all.get(&b).with_context(|| format!("no batch {b}"))?;
            print!("{}", fix::prompt(ps, its));
            Ok(())
        }
        (Some("run"), 2) => {
            let dir = Path::new(&pos[1]);
            let (paras, items) = (load_paras(dir)?, fix::load_items(dir)?);
            let fx = fix::Fixes::new(&paras, &items);
            run::run(dir, &fx, &batches_opt(o)?, num(o, "jobs", 5)?, num(o, "max-calls", 10)?)
        }
        (Some("gate"), 2) => {
            let replaces = o.get("replaces").context("--replaces <pilot run> is required")?;
            let v = fix::collect(Path::new(&pos[1]), replaces, &lemma::load()?)?;
            println!("{}", serde_json::to_string_pretty(&v)?);
            Ok(())
        }
        _ => usage(),
    }
}
