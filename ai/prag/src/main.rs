//! prag — Mova pragmatics (closed part).
//!
//!   prag select <db> <tatoeba.tsv> <en model> <run dir> [--tales 2000] [--tatoeba 1000] [--seed 26]
//!       first-wave selection: whole tale paragraphs from the database and Tatoeba CC0 sentences → items.jsonl, batches;
//!       UD bronze for the Tatoeba sentences (tatoeba-ud.conllu) for `db load`, so the sentences are in the database
//!   prag silver <run dir> [--batches 0,38] [--jobs 4]
//!       LLM silver (claude -p, Opus medium) per batch, gates → silver.tsv, rejects.tsv, calls.jsonl
//!   prag prompt <run dir> <batch>
//!       batch prompt without calling the LLM — for review
//!   prag gate <run dir>
//!       only the gates over already received raw outputs
//!   prag eval <run dir> <en model> [--gum en_gum-ud-test.conllu] [--seed 26]
//!       features, training and measurement of MMM models (perceptron, IGTree, k-NN) against held-out silver,
//!       baseline, negative control, model agreement, speed; form vs GUM s_type (measurement only);
//!       MMM predictions (cross-validated by document) → rust-<model>.tsv for `db prag`
//!
//! Variables: PRAG_MODEL (claude-opus-5-5), PRAG_EFFORT (medium), DUCKDB (~/bin/duckdb).

use std::fmt::Write as _;
use std::path::Path;

use anyhow::{Context, Result, bail};

use prag::data::{self, Src};
use prag::silver;

fn usage() -> Result<()> {
    bail!(
        "usage: prag select <db> <tatoeba.tsv> <en model> <dir> [--tales N] [--tatoeba N] [--seed S] | \
         prag silver <dir> [--batches a,b,…] [--jobs N] | prag prompt <dir> <batch> | prag gate <dir> | prag eval <dir> <en model> [--gum test.conllu] [--seed S]"
    )
}

/// `--key value` options after the positional arguments.
fn opts(a: &[String]) -> Result<(Vec<String>, std::collections::BTreeMap<String, String>)> {
    let mut pos = Vec::new();
    let mut o = std::collections::BTreeMap::new();
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

fn num(o: &std::collections::BTreeMap<String, String>, k: &str, d: usize) -> Result<usize> {
    o.get(k).map(|v| v.parse::<usize>().with_context(|| format!("--{k} {v}"))).transpose().map(|x| x.unwrap_or(d))
}

fn main() -> Result<()> {
    let args: Vec<String> = std::env::args().skip(1).collect();
    let Some(cmd) = args.first().map(String::as_str) else { return usage() };
    let (pos, o) = opts(&args[1..])?;
    match (cmd, pos.len()) {
        ("select", 4) => select(Path::new(&pos[0]), Path::new(&pos[1]), Path::new(&pos[2]), Path::new(&pos[3]), num(&o, "tales", 2000)?, num(&o, "tatoeba", 1000)?, num(&o, "seed", 26)? as u64),
        ("silver", 1) => {
            let dir = Path::new(&pos[0]);
            let items = data::read_items(&dir.join("items.jsonl"))?;
            let which: Vec<usize> = match o.get("batches") {
                Some(v) => v.split(',').map(|x| x.trim().parse::<usize>().with_context(|| format!("--batches {v}"))).collect::<Result<_>>()?,
                None => Vec::new(),
            };
            let r = silver::run(dir, &items, &which, num(&o, "jobs", 4)?);
            gate_report(dir, &items)?;
            r
        }
        ("prompt", 2) => {
            // batch prompt without a call — for review
            let items = data::read_items(&Path::new(&pos[0]).join("items.jsonl"))?;
            let b = silver::batches(&items);
            let its = b.get(&pos[1].parse::<usize>()?).context("no such batch")?;
            print!("{}", silver::prompt(its));
            Ok(())
        }
        ("gate", 1) => {
            let dir = Path::new(&pos[0]);
            let items = data::read_items(&dir.join("items.jsonl"))?;
            gate_report(dir, &items)
        }
        ("eval", 2) => prag::eval::run(Path::new(&pos[0]), Path::new(&pos[1]), o.get("gum").map(std::path::PathBuf::from).as_deref(), num(&o, "seed", 26)? as u64),
        _ => usage(),
    }
}

fn select(db: &Path, tatoeba: &Path, model: &Path, dir: &Path, n_tales: usize, n_tat: usize, seed: u64) -> Result<()> {
    if n_tales + n_tat > silver::WAVE_MAX {
        bail!("{} sentences > {} — wave ceiling", n_tales + n_tat, silver::WAVE_MAX);
    }
    data::license_gate(&[Src::Tale, Src::Tatoeba])?;
    std::fs::create_dir_all(dir)?;
    let tales = data::read_tales(db, &std::env::temp_dir().join("prag"))?;
    let wins = data::select_tales(&tales, n_tales, seed, (10, 18), 14);
    let tat = data::select_tatoeba(tatoeba, n_tat, seed)?;
    let items = data::make_items(&tales, &wins, &tat, 60, 50);
    data::write_items(&dir.join("items.jsonl"), &items)?;
    // UD bronze for the Tatoeba sentences, so they go into the database (`db load --docs idprefix --match auto`)
    let a = en::annotate::Annotator::load(model)?;
    let mut conllu = String::new();
    for (id, t) in &tat {
        let toks = a.tokenize(t);
        let forms: Vec<&str> = toks.iter().map(|x| x.form.as_str()).collect();
        let space: Vec<bool> = toks.iter().map(|x| x.space_after).collect();
        let _ = writeln!(conllu, "# source = Tatoeba #{id} (CC0 1.0)");
        let _ = writeln!(conllu, "# annotation = Mova prag: Rust (en, {}) — UD bronze for pragmatics features; {}", model.file_name().unwrap_or_default().to_string_lossy(), dir.display());
        let _ = writeln!(conllu, "# text = {t}\n# sent_id = tatoeba:{id}");
        for r in en::annotate::rows(&a.annotate(&forms), &space) {
            let _ = writeln!(conllu, "{}", r.join("\t"));
        }
        conllu.push('\n');
    }
    std::fs::write(dir.join("tatoeba-ud.conllu"), conllu)?;
    // selection overview
    let b = silver::batches(&items);
    let docs: std::collections::BTreeSet<&str> = items.iter().filter(|x| x.src == Src::Tale).map(|x| x.doc.as_str()).collect();
    let n_t = items.iter().filter(|x| x.src == Src::Tale).count();
    let quoted = wins.iter().flatten().flatten().filter(|&&i| data::has_quote(&tales[i].text)).count();
    let n_paras: usize = wins.iter().map(Vec::len).sum();
    let mut by_src: std::collections::BTreeMap<String, usize> = std::collections::BTreeMap::new();
    for it in items.iter().filter(|x| x.src == Src::Tale) {
        let book = it.doc.split(':').take(2).collect::<Vec<_>>().join(":");
        *by_src.entry(book).or_default() += 1;
    }
    let sizes: Vec<usize> = b.values().map(Vec::len).collect();
    let mut s = String::new();
    let _ = writeln!(s, "# prag selection (seed {seed})\n");
    let _ = writeln!(s, "- tales: {n_t} sentences in {} windows ({n_paras} paragraphs) from {} tales; sentences with quotes {quoted}", wins.len(), docs.len());
    let _ = writeln!(s, "- by book: {}", by_src.iter().map(|(k, v)| format!("{k} {v}")).collect::<Vec<_>>().join(", "));
    let _ = writeln!(s, "- Tatoeba: {} sentences", tat.len());
    let _ = writeln!(s, "- {} batches: size from {} to {}", b.len(), sizes.iter().min().unwrap_or(&0), sizes.iter().max().unwrap_or(&0));
    std::fs::write(dir.join("select.md"), &s)?;
    print!("{s}");
    Ok(())
}

fn gate_report(dir: &Path, items: &[data::Item]) -> Result<()> {
    let (ok, bad, missing) = silver::collect(dir, items)?;
    let calls = prag::opus::read_calls(&dir.join("calls.jsonl"))?;
    let secs: f64 = calls.iter().map(|c| c.secs).sum();
    let (i, out, cr, cw): (u64, u64, u64, u64) = calls.iter().fold((0, 0, 0, 0), |a, c| (a.0 + c.input_tokens, a.1 + c.output_tokens, a.2 + c.cache_read_tokens, a.3 + c.cache_write_tokens));
    println!(
        "silver: accepted {ok}, rejected {bad}, batches without output {missing}; {} calls, {:.0} s total; tokens: input {i}, output {out}, cache read {cr}, cache write {cw}",
        calls.len(),
        secs
    );
    // run provenance for the database (`db prag --params`): model, effort, call accounting
    let opus = prag::opus::Opus::from_env(dir.join("cwd"));
    let params = serde_json::json!({
        "model": opus.model, "effort": opus.effort, "via": "claude -p (--settings env CLAUDE_CODE_EFFORT_LEVEL)",
        "scheme": "docs/context.md — Pragmatics, first cut (26.09.2026)", "prompt": "prag/src/silver.rs RULES",
        "sentences": items.len(), "accepted": ok, "rejected": bad, "batches": silver::batches(items).len(),
        "calls": calls.len(), "call_secs_sum": secs, "input_tokens": i, "output_tokens": out, "cache_read_tokens": cr, "cache_write_tokens": cw,
        "cost_usd_estimate": calls.iter().map(|c| c.cost_usd).sum::<f64>(),
        "texts": {"tales": "gutenberg-tales (public domain)", "tatoeba": "tatoeba-eng-cc0 (CC0 1.0)"},
    });
    std::fs::write(dir.join("silver-params.json"), serde_json::to_string_pretty(&params)?)?;
    Ok(())
}
