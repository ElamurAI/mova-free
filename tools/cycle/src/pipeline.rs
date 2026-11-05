//! The cycle without waiting (design note, 15.10: "Haiku in parallel: the GPU and CPUs on their own, the queue does not
//! block them"). One loop with two independent sides:
//! - **feed**: while fewer than CYCLE_INFLIGHT batches are with Haiku, prepare the next round on our hardware
//!   (perceptron draft → GPU parser → Stanza/Trankit/spaCy trf → treeagree → patch requests) and submit it;
//! - **collect**: look at every batch in flight once; a finished one is applied (patches, gates), its trees improve
//!   the parse database, and the GPU SLM is fine-tuned on it (~30 s) — the next fed round drafts with that model.
//! The loop sleeps only when there is nothing to feed and nothing has come back. Restartable: rounds with a batch id
//! and no results are in flight, rounds with a row in `cycle.tsv` are done.

use std::collections::{BTreeMap, BTreeSet};
use std::fs;
use std::path::Path;
use std::process::Command;

use crate::{append_new, apply_metrics, db, gparse_for, ids_of, num_after, parsers, row, sh, Cfg, R};

/// Sentence ids no new round may take: the held-out test lessons and the sentences of rounds still in flight.
fn exclude(c: &Cfg) -> R<(std::path::PathBuf, BTreeSet<String>)> {
    let mut test: BTreeSet<String> = BTreeSet::new();
    if let Ok(t) = fs::read_to_string(&c.test) {
        test.extend(ids_of(&t));
    }
    let mut all = test.clone();
    // the rounds still with Haiku (drafted, not yet back): a new round must not take their sentences; finished rounds
    // are fair game again — a recheck takes any sentence of the pool
    for e in fs::read_dir(&c.root).map_err(|e| e.to_string())?.flatten() {
        if e.path().join("final.conllu").exists() {
            continue;
        }
        if let Ok(t) = fs::read_to_string(e.path().join("mmm.conllu")) {
            all.extend(ids_of(&t));
        }
    }
    let f = c.root.join("exclude.txt");
    fs::write(&f, all.iter().map(|x| format!("{x}\n")).collect::<String>()).map_err(|e| e.to_string())?;
    Ok((f, test))
}

/// Codex is the teacher for new runs. Legacy tiers remain for reading old runs; the CLI rejects new Claude training.
#[derive(Clone, Copy, PartialEq, Eq)]
enum Base {
    Codex,
    Haiku,
    Sonnet,
    Opus,
}

fn base() -> Base {
    match std::env::var("CYCLE_TEACHER").as_deref() {
        Ok("sonnet") => Base::Sonnet,
        Ok("opus") => Base::Opus,
        _ => Base::Codex,
    }
}

impl Base {
    fn name(self) -> &'static str {
        match self {
            Base::Codex => "Codex",
            Base::Haiku => "Haiku",
            Base::Sonnet => "Sonnet",
            Base::Opus => "Opus",
        }
    }
    fn model(self) -> String {
        match self {
            Base::Codex => mova_teacher::model("CYCLE_MODEL"),
            Base::Haiku => "claude-haiku-5-5".into(),
            Base::Sonnet => "claude-sonnet-5-5".into(),
            Base::Opus => "claude-opus-5-5".into(),
        }
    }
    /// the first tier that reviews the base, if any
    fn above(self) -> Option<Tier> {
        match self {
            Base::Codex => None,
            Base::Haiku => Some(Tier::Sonnet),
            Base::Sonnet => Some(Tier::Opus),
            Base::Opus => None,
        }
    }
}

fn annot_env(c: &Cfg, cmd: &mut Command) {
    cmd.env("ANNOT_FLOW", "patch").env("ANNOT_BATCH", "8").env("ANNOT_MODEL", base().model()).env("ANNOT_EFFORT", "medium").env("ANNOT_CORPUS", &c.corpus);
}

/// Our side of round k, up to the submitted batch. false — the pool is used up.
fn feed(c: &Cfg, k: usize) -> R<bool> {
    let dir = c.root.join(format!("round{k}"));
    let pre = dir.join("pre");
    fs::create_dir_all(&pre).map_err(|e| e.to_string())?;
    let mmm = dir.join("mmm.conllu");
    if !mmm.exists() {
        let (ex, _) = exclude(c)?;
        let base = crate::annot_train(c, &[], &dir)?;
        let mut cmd = Command::new(&c.jobs);
        cmd.args(["run", "--name", &format!("cycle{k}-drafts"), "--mem-gb", "12", "--secs", "3600", "--", &c.annot, "drafts"]);
        cmd.args(&base).arg("--").arg(&c.pool).arg(c.n.to_string()).arg((c.seed + k as u64).to_string()).arg(&mmm);
        annot_env(c, &mut cmd);
        cmd.env("ANNOT_EXCLUDE", &ex);
        // CYCLE_REPEAT=1: a second pass over the school — sentences Haiku has already seen may come again (only the
        // test lessons and the rounds in flight stay out)
        if std::env::var("CYCLE_REPEAT").as_deref() == Ok("1") {
            let empty = c.root.join("empty.conllu");
            fs::write(&empty, "").map_err(|e| e.to_string())?;
            cmd.env("ANNOT_CORPUS", &empty);
        }
        if !c.en_model.is_empty() {
            cmd.env("ANNOT_EN_MODEL", &c.en_model);
        }
        sh(&mut cmd, &dir.join("drafts.log"))?;
    }
    if ids_of(&fs::read_to_string(&mmm).map_err(|e| e.to_string())?).is_empty() {
        eprintln!("round {k}: no sentences left in the pool");
        return Ok(false);
    }
    let gp = dir.join("mmm-gparse.conllu");
    if !gp.exists() {
        let model = gparse_for(c, k);
        eprintln!("round {k}: GPU SLM over the draft ({model})");
        let mut cmd = Command::new(&c.gparse);
        cmd.arg("annotate").arg(&model).arg(&mmm).arg(&gp).env("GP_LEX", &c.lex);
        sh(&mut cmd, &dir.join("gparse.log"))?;
    }
    let drafts = fs::read_to_string(&gp).map_err(|e| e.to_string())?;
    parsers(c, &drafts, &pre, k)?;
    if !pre.join("agreed.conllu").exists() {
        let mut cmd = Command::new(&c.treeagree);
        cmd.arg("agree").arg(&gp).arg(&pre).arg(pre.join("agreed.conllu")).arg(pre.join("agree-rep")).env("TREEAGREE_LEX", &c.lex);
        sh(&mut cmd, &pre.join("log/treeagree.log"))?;
    }
    if !dir.join("batch-state.json").exists() {
        // the temporary world before each sentence, read over the whole parse database (lessons and books whole,
        // in order) — a page summary for Haiku instead of the pages
        // (with the round's agreed drafts of sentences the database does not have yet: a book or a problem read for
        // the first time still gets the context of what came before it)
        let world = dir.join("world.tsv");
        if !c.prag.is_empty() && !world.exists() {
            let inp = dir.join("world-in.conllu");
            let cur = fs::read_to_string(db::dir().join("current.conllu")).unwrap_or_default();
            let agreed = fs::read_to_string(pre.join("agreed.conllu")).map_err(|e| e.to_string())?;
            fs::write(&inp, db::with_drafts(&cur, &agreed)).map_err(|e| e.to_string())?;
            // only the documents of this round's sentences are read (a sentence's world is its own document's): the
            // same lines, without reading the whole database each round (56 of a round's 60 minutes, 2026-10-24)
            let ids = dir.join("world-ids.txt");
            fs::write(&ids, ids_of(&agreed).into_iter().collect::<Vec<_>>().join("\n")).map_err(|e| e.to_string())?;
            sh(Command::new(&c.prag).arg("world").arg(&inp).arg(&world).arg("--only").arg(&ids), &dir.join("world.log"))?;
            let _ = fs::remove_file(&inp);
        }
        // the pragmatic reading of the agreed draft: our SLM, the open witnesses on the GPU, the agreer
        let prag = prag_reading(c, k, &pre)?;
        let base = crate::annot_train(c, &[], &dir)?;
        let mut cmd = Command::new(&c.annot);
        cmd.arg("batch-prepare").args(&base).arg("--").arg(&c.pool).arg("0").arg("0").arg(&dir);
        annot_env(c, &mut cmd);
        cmd.env("ANNOT_DRAFTS", pre.join("agreed.conllu"));
        if world.exists() {
            cmd.env("ANNOT_CONTEXT", &world);
        }
        if let Some(p) = &prag {
            cmd.env("ANNOT_PRAG", p);
        }
        sh(&mut cmd, &dir.join("prepare.log"))?;
    }
    if !dir.join("batch-id.txt").exists() {
        let mut cmd = Command::new(&c.annot);
        cmd.arg("batch-run").arg(&dir).env("ANNOT_NO_WAIT", "1");
        sh(&mut cmd, &dir.join("submit.log"))?;
        eprintln!("round {k}: submitted to {}", base().name());
    }
    Ok(true)
}

/// Our pragmatic reading of the round's agreed draft trees: `prag label` (the SLM over whole documents), the open
/// witnesses (`witness.py` on the GPU), `prag agree` → `pre/prag-agreed.tsv`. None when no prag is configured.
fn prag_reading(c: &Cfg, k: usize, pre: &Path) -> R<Option<std::path::PathBuf>> {
    if c.prag.is_empty() {
        return Ok(None);
    }
    let agreed = pre.join("prag-agreed.tsv");
    if agreed.exists() {
        return Ok(Some(agreed));
    }
    let trees = pre.join("agreed.conllu");
    let labels = pre.join("prag-label.tsv");
    if !labels.exists() {
        sh(Command::new(&c.prag).arg("label").arg(&c.prag_silver).arg(&c.prag_en).arg(&trees).arg(&labels), &pre.join("log/prag-label.log"))?;
    }
    let wit = pre.join("prag-witness.tsv");
    if !wit.exists() {
        eprintln!("round {k}: pragmatic witnesses on the GPU");
        let mut cmd = Command::new(&c.venv);
        cmd.args(["-I", &c.witness]).arg(&trees).arg(&wit).env("PYTHONIOENCODING", "utf-8").env("MOVA_BIG", crate::mova_big());
        sh(&mut cmd, &pre.join("log/prag-witness.log"))?;
    }
    sh(Command::new(&c.prag).arg("agree").arg(&labels).arg(&wit).arg(&trees).arg(&agreed).arg(pre.join("prag-agree.md")), &pre.join("log/prag-agree.log"))?;
    Ok(Some(agreed))
}

/// Look at round k's batch once; true when its results are here.
fn ready(c: &Cfg, k: usize) -> R<bool> {
    let dir = c.root.join(format!("round{k}"));
    if !dir.join("batch-results.jsonl").exists() {
        let mut cmd = Command::new(&c.annot);
        cmd.arg("batch-run").arg(&dir).env("ANNOT_NO_WAIT", "1");
        sh(&mut cmd, &dir.join("poll.log"))?;
    }
    Ok(dir.join("batch-results.jsonl").exists())
}

/// The base teacher's answer is back: apply, improve the database, optionally fine-tune the SLM, write the row.
fn collect(c: &Cfg, k: usize, held: &db::Heldout) -> R<()> {
    let dir = c.root.join(format!("round{k}"));
    if !dir.join("final.conllu").exists() {
        let mut cmd = Command::new(&c.annot);
        cmd.arg("batch-apply").arg(&dir).arg(dir.join("batch-results.jsonl"));
        annot_env(c, &mut cmd);
        cmd.env("ANNOT_DRAFT_LABEL", "SLM (gparse tree, en lemmas) + Stanza, Trankit, spaCy trf (treeagree)");
        let pa = dir.join("pre/prag-agreed.tsv");
        if pa.exists() {
            cmd.env("ANNOT_PRAG", &pa);
        }
        sh(&mut cmd, &dir.join("apply.log"))?;
    }
    let fin = dir.join("final.conllu");
    let source = if fs::read_to_string(dir.join("batch-id.txt")).is_ok_and(|id| id.trim() == "codex-local") {
        format!("round{k}-codex")
    } else {
        format!("round{k}") // historical results retain their original teacher provenance
    };
    let (added, replaced) = db::update(&db::dir(), &fin, &source)?;
    if dir.join("prag-final.tsv").exists() {
        db::update_prag(&db::dir(), &dir.join("prag-final.tsv"))?;
    }
    append_new(&fs::read_to_string(&fin).map_err(|e| e.to_string())?, &c.agreed)?;
    sh(Command::new(&c.treeagree).arg("check").arg(&fin), &dir.join("check.log"))?;
    if enrichment_enabled()? { enrich(c, k)?; }
    let out = crate::models_dir(c).join(format!("round{k}"));
    let training_status = if !crate::finetune_rounds() {
        "SLM fine-tuning disabled"
    } else if c.gparse_model.is_empty() {
        "SLM fine-tuning skipped (no configured gparse model)"
    } else if out.join("model.safetensors").exists() {
        "SLM candidate already exists (no fine-tuning in this invocation)"
    } else {
        let init = gparse_for(c, k);
        let train = db::training_file(&db::dir(), held)?;
        let inputs = dir.join("train-inputs");
        let base = db::filtered_inputs(held, &c.finetune_base, &inputs)?;
        let always = db::filtered_inputs(held, &[fin.display().to_string()], &inputs)?;
        let mut cmd = Command::new(&c.gparse);
        let part = crate::part_dir(&out);
        cmd.arg("train").arg(&part).args(&base).arg(&train).arg("--").arg(&c.dev);
        cmd.env("GP_INIT", &init).env("GP_ALWAYS", &always[0]).env("GP_SAMPLE", "0.15").env("GP_EPOCHS", "2").env("GP_LR", "0.0005").env("GP_TOKENS", "1000");
        sh(&mut cmd, &dir.join("finetune.log"))?;
        fs::rename(&part, &out).map_err(|e| e.to_string())?;
        "SLM fine-tuning completed in this invocation"
    };
    let (uas, las, cost, n) = apply_metrics(&fs::read_to_string(dir.join("apply.log")).unwrap_or_default());
    let changed = num_after(&fs::read_to_string(dir.join("pre/agree-rep/agree.md")).unwrap_or_default(), "words (").unwrap_or(f64::NAN);
    row(c, k, &format!("async{}", c.label), n, uas, las, changed, cost)?;
    let b = base().name();
    eprintln!("round {k}: back from {b} — {n} sentences, db +{added} new / {replaced} improved; {b} kept the agreed draft UAS {uas:.2} LAS {las:.2}; ${cost:.3}; {training_status}");
    Ok(())
}

/// The teachers above Haiku (design note, 15.10: «5% of the corpus on Haiku, 0.2% on Sonnet, 0.05% on Opus», as a cascade):
/// each reviews a share of the tier below (`cascade`) — the trees and the pragmatic reading as that tier left them —
/// and the parse database takes its word over the lower one. How much it changes estimates the lower tier's errors (`tiers.tsv`).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tier {
    Sonnet,
    Opus,
}

impl Tier {
    fn name(self) -> &'static str {
        match self {
            Tier::Sonnet => "sonnet",
            Tier::Opus => "opus",
        }
    }
    fn model(self) -> &'static str {
        match self {
            Tier::Sonnet => "claude-sonnet-5-5",
            Tier::Opus => "claude-opus-5-5",
        }
    }
    /// the tier whose output it reviews: (dir under the round, its final trees, its pragmatic reading)
    fn below(self, round: &Path) -> (std::path::PathBuf, std::path::PathBuf) {
        match self {
            Tier::Sonnet => (round.join("final.conllu"), round.join("prag-final.tsv")),
            Tier::Opus if base() == Base::Sonnet => (round.join("final.conllu"), round.join("prag-final.tsv")),
            Tier::Opus => (round.join("sonnet/final.conllu"), round.join("sonnet/prag-final.tsv")),
        }
    }
}

/// The cascade (design note, 15.10: «Sonnet 0.2% … or a cascade, as you say»): a tier gets `n` sentences of the tier below —
/// half the hardest (where the tier below changed its draft the most), half drawn at random (a fair estimate of the
/// tier below's errors). `below` and `draft` are that tier's output and its input.
fn cascade(below: &str, draft: &str, k: usize, n: usize) -> String {
    let h = |s: &str| s.bytes().fold(0xcbf29ce484222325u64 ^ k as u64, |a, b| (a ^ b as u64).wrapping_mul(0x100000001b3));
    let heads = |b: &str| -> Vec<(String, String)> { b.lines().filter(|l| !l.starts_with('#') && !l.is_empty()).filter_map(|l| { let c: Vec<&str> = l.split('\t').collect(); (c.len() == 10).then(|| (c[6].to_string(), c[7].to_string())) }).collect() };
    let before: BTreeMap<String, Vec<(String, String)>> = draft.split("\n\n").filter_map(|b| ids_of(b).into_iter().next().map(|id| (id, heads(b)))).collect();
    let mut blocks: Vec<(String, String, usize)> = below
        .split("\n\n")
        .filter_map(|b| {
            let id = ids_of(b).into_iter().next()?;
            let now = heads(b);
            let changed = before.get(&id).map_or(0, |o| o.iter().zip(&now).filter(|(a, b)| a != b).count());
            Some((id, format!("{}\n\n", b.trim_end()), changed))
        })
        .collect();
    let mut out: Vec<usize> = Vec::new();
    let mut by_change: Vec<usize> = (0..blocks.len()).filter(|&i| blocks[i].2 > 0).collect();
    by_change.sort_by_key(|&i| (std::cmp::Reverse(blocks[i].2), h(&blocks[i].0)));
    out.extend(by_change.into_iter().take(n / 2));
    let mut rest: Vec<usize> = (0..blocks.len()).filter(|i| !out.contains(i)).collect();
    rest.sort_by_key(|&i| h(&blocks[i].0));
    out.extend(rest.into_iter().take(n - out.len()));
    out.sort();
    out.iter().map(|&i| std::mem::take(&mut blocks[i].1)).collect()
}

/// How many sentences a tier reviews so that the money lands on the budget shares (operator, 16.10: Haiku 70%,
/// Sonnet 20%, Opus 10%; `CYCLE_SHARES=h,s,o`, `off` keeps the fixed counts). The tier gets
/// Haiku's cost of this round × its share / Haiku's share, divided by the tier's measured cost per sentence (the
/// mean of its last three rows in `tiers.tsv`); with no measurement yet, or no Haiku cost, the fixed count. At least 2 (one hard, one
/// random), Opus never more than Sonnet reviewed.
fn budget_n(c: &Cfg, k: usize, t: Tier, fixed: usize) -> usize {
    let shares = std::env::var("CYCLE_SHARES").unwrap_or_else(|_| "70,20,10".into());
    if shares == "off" {
        return fixed;
    }
    let s: Vec<f64> = shares.split(',').filter_map(|x| x.trim().parse().ok()).collect();
    // the base tier's share: Haiku's, or Sonnet's when Sonnet reads the whole round
    let hi = if base() == Base::Sonnet { 1 } else { 0 };
    let (Some(&h), Some(&share)) = (s.get(hi), s.get(if matches!(t, Tier::Sonnet) { 1 } else { 2 })) else { return fixed };
    let round = c.root.join(format!("round{k}"));
    let (_, _, haiku_cost, _) = crate::apply_metrics(&fs::read_to_string(round.join("apply.log")).unwrap_or_default());
    let table = fs::read_to_string(c.root.join("tiers.tsv")).unwrap_or_default();
    let (mut cost, mut sents) = (0.0, 0.0);
    // the tier's last three reviews: the price per sentence moves with the prompt (context lines made it about
    // twice as dear on 16.10), so the old history would overspend
    let rows: Vec<Vec<&str>> = table.lines().skip(1).map(|l| l.split('\t').collect::<Vec<_>>()).filter(|r| r.len() >= 7 && r[1] == t.name()).collect();
    for r in rows.iter().skip(rows.len().saturating_sub(3)) {
        if let (Ok(n), Ok(x)) = (r[2].parse::<f64>(), r[6].parse::<f64>()) {
            sents += n;
            cost += x;
        }
    }
    if !(haiku_cost > 0.0) || sents == 0.0 || cost <= 0.0 || h <= 0.0 {
        return fixed;
    }
    let n = (haiku_cost * share / h / (cost / sents)).round().max(2.0) as usize;
    match t {
        Tier::Opus if base() == Base::Sonnet => n.min(ids_of(&fs::read_to_string(round.join("final.conllu")).unwrap_or_default()).len().max(2)),
        Tier::Opus => n.min(ids_of(&fs::read_to_string(round.join("sonnet/drafts.conllu")).unwrap_or_default()).len().max(2)),
        Tier::Sonnet => n,
    }
}

/// Submit round k's review by a higher tier (skipped when already submitted). false — nothing to review.
fn review_feed(c: &Cfg, k: usize, t: Tier) -> R<bool> {
    let round = c.root.join(format!("round{k}"));
    let dir = round.join(t.name());
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let (trees, prag) = t.below(&round);
    let drafts = dir.join("drafts.conllu");
    if !drafts.exists() {
        // the tier below's own input, to see where it changed most: Haiku's = the agreed draft, Sonnet's = its drafts
        let input = match t {
            Tier::Sonnet => round.join("pre/agreed.conllu"),
            Tier::Opus if base() == Base::Sonnet => round.join("pre/agreed.conllu"),
            Tier::Opus => round.join("sonnet/drafts.conllu"),
        };
        let fixed = match t {
            Tier::Sonnet => c.sonnet_n,
            Tier::Opus => c.opus_n,
        };
        let n = budget_n(c, k, t, fixed);
        eprintln!("round {k}: {} gets {n} sentences (budget shares {})", t.name(), std::env::var("CYCLE_SHARES").unwrap_or_else(|_| "70,20,10".into()));
        let sample = cascade(&fs::read_to_string(&trees).map_err(|e| format!("{}: {e}", trees.display()))?, &fs::read_to_string(&input).unwrap_or_default(), k, n);
        fs::write(&drafts, &sample).map_err(|e| e.to_string())?;
    }
    if ids_of(&fs::read_to_string(&drafts).map_err(|e| e.to_string())?).is_empty() {
        return Ok(false);
    }
    let env_for = |cmd: &mut Command| {
        cmd.env("ANNOT_FLOW", "patch").env("ANNOT_BATCH", "8").env("ANNOT_MODEL", t.model()).env("ANNOT_EFFORT", "medium").env("ANNOT_CORPUS", c.root.join(format!("tier-{}.conllu", t.name())));
        if prag.exists() {
            cmd.env("ANNOT_PRAG", &prag);
        }
        let w = round.join("world.tsv");
        if w.exists() {
            cmd.env("ANNOT_CONTEXT", &w);
        }
    };
    if !dir.join("batch-state.json").exists() {
        let base = crate::annot_train(c, &[], &dir)?;
        let mut cmd = Command::new(&c.annot);
        cmd.arg("batch-prepare").args(&base).arg("--").arg(&c.pool).arg("0").arg("0").arg(&dir).env("ANNOT_DRAFTS", &drafts);
        env_for(&mut cmd);
        sh(&mut cmd, &dir.join("prepare.log"))?;
    }
    if !dir.join("batch-id.txt").exists() {
        sh(Command::new(&c.annot).arg("batch-run").arg(&dir).env("ANNOT_NO_WAIT", "1"), &dir.join("submit.log"))?;
        eprintln!("round {k}: {} reviews a tenth of the tier below", t.name());
    }
    Ok(true)
}

/// A finished review: apply, let it overrule the lower tier in the database, record how much it changed.
fn review_collect(c: &Cfg, k: usize, t: Tier) -> R<()> {
    let round = c.root.join(format!("round{k}"));
    let dir = round.join(t.name());
    if !dir.join("final.conllu").exists() {
        let mut cmd = Command::new(&c.annot);
        cmd.arg("batch-apply").arg(&dir).arg(dir.join("batch-results.jsonl"));
        // whose trees it reviews: Sonnet always reviews Haiku's; Opus reviews Sonnet's (Sonnet's own review, or Sonnet as the base)
        cmd.env("ANNOT_DRAFT_LABEL", if t == Tier::Sonnet { "Haiku's trees" } else { "Sonnet's trees" });
        let (_, prag) = t.below(&round);
        if prag.exists() {
            cmd.env("ANNOT_PRAG", &prag);
        }
        sh(&mut cmd, &dir.join("apply.log"))?;
    }
    let fin = dir.join("final.conllu");
    db::update(&db::dir(), &fin, &format!("round{k}-{}", t.name()))?;
    if dir.join("prag-final.tsv").exists() {
        db::update_prag(&db::dir(), &dir.join("prag-final.tsv"))?;
    }
    let (uas, las, cost, n) = apply_metrics(&fs::read_to_string(dir.join("apply.log")).unwrap_or_default());
    let prag_changed = fs::read_to_string(dir.join("prag-final.tsv")).unwrap_or_default().lines().skip(1).filter(|l| !l.ends_with("\t-")).count();
    let table = c.root.join("tiers.tsv");
    let mut tt = fs::read_to_string(&table).unwrap_or_else(|_| "round\ttier\tsentences\tkept_uas\tkept_las\tprag_changed\tcost_usd\n".into());
    std::fmt::Write::write_fmt(&mut tt, format_args!("{k}\t{}\t{n}\t{uas:.2}\t{las:.2}\t{prag_changed}\t{cost:.3}\n", t.name())).map_err(|e| e.to_string())?;
    fs::write(&table, tt).map_err(|e| e.to_string())?;
    eprintln!("round {k}: back from {} — {n} sentences; it kept the tier below at UAS {uas:.2} LAS {las:.2}, changed the pragmatics of {prag_changed}; ${cost:.3}", t.name());
    Ok(())
}

fn done(c: &Cfg, k: usize) -> bool {
    fs::read_to_string(c.root.join("cycle.tsv")).unwrap_or_default().lines().any(|l| l.split('\t').next() == Some(&k.to_string()))
}

/// The GPU SLM learning on its own, apart from the teachers (design note, 15.10: «the SLM learns calmly on the GPUs, no
/// waiting, asynchronously; the batches come into the database»): every minute it looks whether the database grew;
/// then it agrees the versions, and fine-tunes from the newest model on everything that changed plus a random 15%.
pub fn trainer(c: &Cfg) -> R<()> {
    // every corpus's database (operator, 17.10: «all corpora train at once — the weights and the skeleton are one and
    // open»): one model, one training set; a pass starts when any of them grew
    let dbs = db::train_dbs();
    let db = db::dir();
    let (_, test) = exclude(c)?;
    let held = crate::heldout(c)?;
    let models = crate::models_dir(c);
    fs::create_dir_all(&models).map_err(|e| e.to_string())?;
    // the pipeline's own database was learnt by the trainer before this start; the others are new to it
    let mut seen: BTreeMap<std::path::PathBuf, u64> = BTreeMap::new();
    seen.insert(db.clone(), fs::metadata(db.join("log.tsv")).map(|m| m.len()).unwrap_or(0));
    eprintln!("trainer: {} databases ({}) → {}", dbs.len(), dbs.iter().map(|d| d.display().to_string()).collect::<Vec<_>>().join(", "), models.display());
    loop {
        let sizes: Vec<(std::path::PathBuf, u64)> = dbs.iter().map(|d| (d.clone(), fs::metadata(d.join("log.tsv")).map(|m| m.len()).unwrap_or(0))).collect();
        if sizes.iter().any(|(d, s)| *s > seen.get(d).copied().unwrap_or(0)) {
            // what changed since the last pass: the ids each database logged after what was seen of it
            let mut fresh_blocks = String::new();
            let (mut moved, mut multi, mut nfresh) = (0, 0, 0);
            for (d, size) in &sizes {
                let was = seen.get(d).copied().unwrap_or(0);
                if *size <= was {
                    continue;
                }
                let (m, w) = db::agree(d)?;
                multi += m;
                moved += w;
                let log = fs::read(d.join("log.tsv")).map_err(|e| e.to_string())?;
                let fresh: std::collections::BTreeSet<String> = String::from_utf8_lossy(&log[was as usize..]).lines().filter_map(|l| l.split('\t').nth(1).map(String::from)).collect();
                nfresh += fresh.len();
                let cur = fs::read_to_string(d.join("current.conllu")).map_err(|e| e.to_string())?;
                let allow = crate::reviewed::training_allow(d)?;
                fresh_blocks.extend(db::blocks(&cur).into_iter().filter(|(id, b)| fresh.contains(id) && !test.contains(id) && !held.hits(id, b) && allow.as_ref().is_none_or(|a| a.contains(id))).map(|(_, b)| b));
            }
            let fresh_file = db.join("fresh.conllu");
            fs::write(&fresh_file, fresh_blocks).map_err(|e| e.to_string())?;
            let train = db::training_file(&db, &held)?;
            let base = db::filtered_inputs(&held, &c.finetune_base, &db.join("train-inputs"))?;
            let init = gparse_for(c, usize::MAX);
            let n = fs::read_dir(&models).map(|d| d.count()).unwrap_or(0);
            let out = models.join(format!("live-{n:04}"));
            let mut cmd = Command::new(&c.gparse);
            let part = crate::part_dir(&out);
            cmd.arg("train").arg(&part).args(&base).arg(&train).arg("--").arg(&c.dev);
            cmd.env("GP_INIT", &init).env("GP_ALWAYS", &fresh_file).env("GP_SAMPLE", "0.15").env("GP_EPOCHS", "2").env("GP_LR", "0.0005").env("GP_TOKENS", "1000");
            // one GPU is shared with the other pipelines: when it is out of memory, wait and try the same pass again
            // (local, costs nothing) instead of dying; any other failure still stops the trainer
            if let Err(e) = sh(&mut cmd, &models.join(format!("live-{n:04}.log"))) {
                let log = fs::read_to_string(models.join(format!("live-{n:04}.log"))).unwrap_or_default();
                if log.contains("CUDA_ERROR_OUT_OF_MEMORY") {
                    eprintln!("trainer: the GPU is out of memory; trying again in 10 minutes");
                    let _ = fs::remove_dir_all(&part);
                    std::thread::sleep(std::time::Duration::from_secs(600));
                    continue;
                }
                return Err(e);
            }
            fs::rename(&part, &out).map_err(|e| e.to_string())?;
            eprintln!("trainer: dbs agreed ({multi} multi-version sentences, {moved} words moved); {nfresh} fresh sentences from {} databases → {}", dbs.len(), out.display());
            for (d, s) in sizes {
                seen.insert(d, s);
            }
        }
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}

pub fn run(c: &Cfg, from: usize, to: usize) -> R<()> {
    let max: usize = std::env::var("CYCLE_INFLIGHT").ok().and_then(|x| x.parse().ok()).unwrap_or(3);
    exclude(c)?;
    let held = crate::heldout(c)?;
    let mut flight: Vec<usize> = (from..=to).filter(|&k| c.root.join(format!("round{k}/batch-id.txt")).exists() && !done(c, k)).collect();
    let mut next = (from..=to).find(|&k| !done(c, k) && !flight.contains(&k)).unwrap_or(to + 1);
    let mut pool_empty = false;
    let tiers = std::env::var("CYCLE_TIERS").as_deref() == Ok("1");
    // reviews in flight: (round, tier); restartable — submitted and not yet applied
    let mut reviews: Vec<(usize, Tier)> = Vec::new();
    for k in from..=to {
        for t in [Tier::Sonnet, Tier::Opus] {
            let d = c.root.join(format!("round{k}/{}", t.name()));
            if d.join("batch-id.txt").exists() && !d.join("final.conllu").exists() {
                reviews.push((k, t));
            }
        }
    }
    loop {
        let mut progressed = false;
        // collect whatever the base teacher has finished; its round goes on to the tier above it
        for k in flight.clone() {
            if ready(c, k)? {
                collect(c, k, &held)?;
                flight.retain(|&x| x != k);
                if let Some(t) = base().above().filter(|_| tiers) {
                    if review_feed(c, k, t)? {
                        reviews.push((k, t));
                    }
                }
                progressed = true;
            }
        }
        // the reviews: Sonnet's goes on to Opus
        for (k, t) in reviews.clone() {
            let dir = c.root.join(format!("round{k}/{}", t.name()));
            if !dir.join("batch-results.jsonl").exists() {
                sh(Command::new(&c.annot).arg("batch-run").arg(&dir).env("ANNOT_NO_WAIT", "1"), &dir.join("poll.log"))?;
            }
            if dir.join("batch-results.jsonl").exists() {
                review_collect(c, k, t)?;
                reviews.retain(|x| *x != (k, t));
                if t == Tier::Sonnet && base() == Base::Haiku && review_feed(c, k, Tier::Opus)? {
                    reviews.push((k, Tier::Opus));
                }
                progressed = true;
            }
        }
        // feed the next round while the queue has room
        if !pool_empty && next <= to && flight.len() < max {
            if feed(c, next)? {
                flight.push(next);
            } else {
                pool_empty = true;
            }
            next += 1;
            progressed = true;
        }
        if flight.is_empty() && reviews.is_empty() && (pool_empty || next > to) {
            eprintln!("cycle: all rounds done");
            return Ok(());
        }
        if !progressed {
            std::thread::sleep(std::time::Duration::from_secs(60));
        }
    }
}


/// Optional corpus-enrichment mode. Normal annotation/training keeps its old path.
fn enrichment_enabled() -> R<bool> {
    match std::env::var("CYCLE_ENRICH").unwrap_or_else(|_|"0".into()).as_str() {
        "0"=>Ok(false),"1"=>Ok(true),_=>Err("CYCLE_ENRICH must be 0 or 1".into()),
    }
}
struct EnrichmentLock(std::path::PathBuf);
impl Drop for EnrichmentLock {fn drop(&mut self){let _=fs::remove_dir_all(&self.0);}}
/// One bounded enrichment job after a round, also callable to backfill old rounds.
/// The attempted-call ledger is durable and reserved BEFORE invoking the teacher.
pub fn enrich(c:&Cfg,k:usize)->R<()> {
    let required=|name:&str|std::env::var(name).map_err(|_|format!("{name} is required in enrichment mode"));
    let binary=required("CYCLE_ENRICH_BIN")?;
    let dictionary=required("CYCLE_ENRICH_DICTIONARY")?;
    let school=required("CYCLE_ENRICH_SCHOOL")?;
    let budget:usize=std::env::var("CYCLE_ENRICH_BUDGET").unwrap_or_else(|_|"1".into()).parse().map_err(|_|"invalid enrichment budget")?;
    let n:usize=std::env::var("CYCLE_ENRICH_SENTENCES").unwrap_or_else(|_|"32".into()).parse().map_err(|_|"invalid enrichment source budget")?;
    if !(8..=64).contains(&n) {return Err("enrichment source budget must be 8..64".into());}
    let round=c.root.join(format!("round{k}"));
    let job=round.join("enrichment");
    let lock=c.root.join("enrichment-queue-lock");
    fs::create_dir(&lock).map_err(|_|format!("enrichment queue active or stale lock {}; check its PID",lock.display()))?;
    let _lock=EnrichmentLock(lock.clone());
    fs::write(lock.join("pid"),std::process::id().to_string()).map_err(|e|e.to_string())?;
    if !job.join("packet.json").exists() {
        sh(Command::new(&binary).arg("prepare").arg(&round).arg(&dictionary).arg(&school).arg(&job).arg(n.to_string()),
            &round.join("enrichment-prepare.log"))?;
    }
    sh(Command::new(&binary).arg("check-source").arg(&job).arg(&round).arg(&dictionary).arg(&school),
        &round.join("enrichment-source-check.log"))?;
    let repaired=round.join("enrichment-refined");
    if repaired.exists() {
        sh(Command::new(&binary).arg("check-source").arg(&repaired).arg(&round).arg(&dictionary).arg(&school),
            &round.join("enrichment-repair-source-check.log"))?;
        return sh(Command::new(&binary).arg("require-staged").arg(&repaired),&round.join("enrichment-status.log"));
    }
    if job.join("teacher-failure.json").exists() {
        return Err(format!("teacher failure retained at {}; no automatic transport retry",job.display()));
    }
    if !job.join("teacher-reply.json").exists() {
        reserve_enrichment_attempt(&c.root,k,&job,budget)?;
        sh(Command::new(&binary).arg("review").arg(&job),&round.join("enrichment-review.log"))?;
    }
    let mut needs_repair=job.join("validation-rejected.json").exists();
    if !needs_repair&&!job.join("stage-receipt.json").exists() {
        if let Err(e)=sh(Command::new(&binary).arg("stage").arg(&job),&round.join("enrichment-stage.log")) {
            if !job.join("validation-rejected.json").exists(){return Err(e);}
            needs_repair=true;
        }
    }
    if needs_repair {
        reserve_enrichment_attempt(&c.root,k,&repaired,budget)?;
        sh(Command::new(&binary).arg("refine").arg(&job).arg(&repaired),&round.join("enrichment-refine.log"))?;
        sh(Command::new(&binary).arg("require-staged").arg(&repaired),&round.join("enrichment-status.log"))?;
    } else {
        sh(Command::new(&binary).arg("require-staged").arg(&job),&round.join("enrichment-status.log"))?;
    }
    eprintln!("round {k}: enrichment drafts staged; publication and factual review remain separate");
    Ok(())
}

fn reserve_enrichment_attempt(root:&Path,k:usize,job:&Path,budget:usize)->R<()> {
    use std::io::Write;
    let ledger=root.join("enrichment-attempts.tsv");
    let used=match fs::read_to_string(&ledger) {
        Ok(t)=>t.lines().count(),Err(e) if e.kind()==std::io::ErrorKind::NotFound=>0,Err(e)=>return Err(e.to_string()),
    };
    if used>=budget {return Err(format!("enrichment call budget exhausted ({used}/{budget}); no teacher invoked"));}
    let mut file=fs::OpenOptions::new().create(true).append(true).open(&ledger).map_err(|e|e.to_string())?;
    writeln!(file,"round{k}\t{}",job.display()).map_err(|e|e.to_string())?;
    file.sync_all().map_err(|e|e.to_string())
}
#[cfg(test)]
mod wave33_tests {
    use super::*;
    #[test]
    fn cascade_count_order_and_zero() {
        let block =
            |id: &str, rel: &str| format!("# sent_id = {id}\n1\tA\ta\tX\t_\t_\t0\t{rel}\t_\t_\n\n");
        let old = (0..6)
            .map(|i| block(&format!("s{i}"), "root"))
            .collect::<String>();
        let new = (0..6)
            .map(|i| block(&format!("s{i}"), if i == 4 { "dep" } else { "root" }))
            .collect::<String>();
        assert_eq!(cascade(&new, &old, 7, 0), "");
        let selected = cascade(&new, &old, 7, 2);
        let ids = ids_of(&selected);
        assert_eq!(ids.len(), 2);
        assert!(ids.contains(&"s4".into()));
        assert!(ids.windows(2).all(|p| p[0] < p[1]));
        assert_eq!(cascade(&new, &old, 7, 99), new);
        assert_eq!(cascade(&new, &old, 7, 2), selected);
        assert_eq!(cascade("", "", 7, 2), "");
    }
    #[test]
    fn attempt_budget_is_reserved_before_retry() {
        let root = std::env::temp_dir().join(format!("wave33-attempt-{}", std::process::id()));
        fs::create_dir(&root).unwrap();
        let job = root.join("job");
        assert!(reserve_enrichment_attempt(&root, 2, &job, 0).is_err());
        assert!(!root.join("enrichment-attempts.tsv").exists());
        reserve_enrichment_attempt(&root, 2, &job, 1).unwrap();
        let before = fs::read(root.join("enrichment-attempts.tsv")).unwrap();
        assert_eq!(String::from_utf8_lossy(&before).lines().count(), 1);
        assert!(reserve_enrichment_attempt(&root, 3, &job, 1).is_err());
        assert_eq!(
            fs::read(root.join("enrichment-attempts.tsv")).unwrap(),
            before
        );
        fs::remove_dir_all(root).unwrap();
    }
}
