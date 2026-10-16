//! The cycle without waiting (design note, 15.10: "Haiku in parallel: the GPU and CPUs on their own, the queue does not
//! block them"). One loop with two independent sides:
//! - **feed**: while fewer than CYCLE_INFLIGHT batches are with Haiku, prepare the next round on our hardware
//!   (perceptron draft → GPU parser → Stanza/Trankit/spaCy trf → treeagree → patch requests) and submit it;
//! - **collect**: look at every batch in flight once; a finished one is applied (patches, gates), its trees improve
//!   the parse database, and the GPU MMM is fine-tuned on it (~30 s) — the next fed round drafts with that model.
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

fn annot_env(c: &Cfg, cmd: &mut Command) {
    cmd.env("ANNOT_FLOW", "patch").env("ANNOT_BATCH", "8").env("ANNOT_MODEL", "claude-haiku-5-5").env("ANNOT_EFFORT", "medium").env("ANNOT_CORPUS", &c.corpus);
}

/// Our side of round k, up to the submitted batch. false — the pool is used up.
fn feed(c: &Cfg, k: usize) -> R<bool> {
    let dir = c.root.join(format!("round{k}"));
    let pre = dir.join("pre");
    fs::create_dir_all(&pre).map_err(|e| e.to_string())?;
    let mmm = dir.join("mmm.conllu");
    if !mmm.exists() {
        let (ex, _) = exclude(c)?;
        let mut cmd = Command::new(&c.jobs);
        cmd.args(["run", "--name", &format!("cycle{k}-drafts"), "--mem-gb", "12", "--secs", "3600", "--", &c.annot, "drafts"]);
        cmd.args(&c.base).arg("--").arg(&c.pool).arg(c.n.to_string()).arg((c.seed + k as u64).to_string()).arg(&mmm);
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
        eprintln!("round {k}: GPU MMM over the draft ({model})");
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
        let world = dir.join("world.tsv");
        if !c.prag.is_empty() && !world.exists() {
            sh(Command::new(&c.prag).arg("world").arg(db::dir().join("current.conllu")).arg(&world), &dir.join("world.log"))?;
        }
        // the pragmatic reading of the agreed draft: our SLM, the open witnesses on the GPU, the agreer
        let prag = prag_reading(c, k, &pre)?;
        let mut cmd = Command::new(&c.annot);
        cmd.arg("batch-prepare").args(&c.base).arg("--").arg(&c.pool).arg("0").arg("0").arg(&dir);
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
        eprintln!("round {k}: submitted to Haiku");
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
        cmd.args(["-I", &c.witness]).arg(&trees).arg(&wit).env("PYTHONIOENCODING", "utf-8");
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

/// Haiku's answer is back: apply, improve the database, fine-tune the GPU MMM, write the row.
fn collect(c: &Cfg, k: usize, test: &BTreeSet<String>) -> R<()> {
    let dir = c.root.join(format!("round{k}"));
    if !dir.join("final.conllu").exists() {
        let mut cmd = Command::new(&c.annot);
        cmd.arg("batch-apply").arg(&dir).arg(dir.join("batch-results.jsonl"));
        annot_env(c, &mut cmd);
        cmd.env("ANNOT_DRAFT_LABEL", "MMM (gparse tree, en lemmas) + Stanza, Trankit, spaCy trf (treeagree)");
        let pa = dir.join("pre/prag-agreed.tsv");
        if pa.exists() {
            cmd.env("ANNOT_PRAG", &pa);
        }
        sh(&mut cmd, &dir.join("apply.log"))?;
    }
    let fin = dir.join("final.conllu");
    let (added, replaced) = db::update(&db::dir(), &fin, &format!("round{k}"))?;
    if dir.join("prag-final.tsv").exists() {
        db::update_prag(&db::dir(), &dir.join("prag-final.tsv"))?;
    }
    append_new(&fs::read_to_string(&fin).map_err(|e| e.to_string())?, &c.agreed)?;
    sh(Command::new(&c.treeagree).arg("check").arg(&fin), &dir.join("check.log"))?;
    let out = c.root.join(format!("models/round{k}"));
    if !c.gparse_model.is_empty() && !out.join("model.safetensors").exists() {
        let init = gparse_for(c, k);
        let train = db::training_file(&db::dir(), test)?;
        let mut cmd = Command::new(&c.gparse);
        cmd.arg("train").arg(&out).args(&c.finetune_base).arg(&train).arg("--").arg(&c.dev);
        cmd.env("GP_INIT", &init).env("GP_ALWAYS", &fin).env("GP_SAMPLE", "0.15").env("GP_EPOCHS", "2").env("GP_LR", "0.0005").env("GP_TOKENS", "1000");
        sh(&mut cmd, &dir.join("finetune.log"))?;
    }
    let (uas, las, cost, n) = apply_metrics(&fs::read_to_string(dir.join("apply.log")).unwrap_or_default());
    let changed = num_after(&fs::read_to_string(dir.join("pre/agree-rep/agree.md")).unwrap_or_default(), "words (").unwrap_or(f64::NAN);
    row(c, k, &format!("async{}", c.label), n, uas, las, changed, cost)?;
    eprintln!("round {k}: back from Haiku — {n} sentences, db +{added} new / {replaced} improved; Haiku kept the agreed draft UAS {uas:.2} LAS {las:.2}; ${cost:.3}; GPU MMM fine-tuned");
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
            Tier::Opus => round.join("sonnet/drafts.conllu"),
        };
        let n = match t {
            Tier::Sonnet => c.sonnet_n,
            Tier::Opus => c.opus_n,
        };
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
        let mut cmd = Command::new(&c.annot);
        cmd.arg("batch-prepare").args(&c.base).arg("--").arg(&c.pool).arg("0").arg("0").arg(&dir).env("ANNOT_DRAFTS", &drafts);
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

/// The GPU MMM learning on its own, apart from the teachers (design note, 15.10: «the MMM learns calmly on the GPUs, no
/// waiting, asynchronously; the batches come into the database»): every minute it looks whether the database grew;
/// then it agrees the versions, and fine-tunes from the newest model on everything that changed plus a random 15%.
pub fn trainer(c: &Cfg) -> R<()> {
    let db = db::dir();
    let (_, test) = exclude(c)?;
    let models = c.root.join("models");
    fs::create_dir_all(&models).map_err(|e| e.to_string())?;
    let mut seen = 0u64;
    loop {
        let size = fs::metadata(db.join("log.tsv")).map(|m| m.len()).unwrap_or(0);
        if size > seen {
            let (m, w) = db::agree(&db)?;
            // what changed since the last pass: the ids logged after `seen`
            let log = fs::read(db.join("log.tsv")).map_err(|e| e.to_string())?;
            let fresh: std::collections::BTreeSet<String> = String::from_utf8_lossy(&log[seen as usize..]).lines().filter_map(|l| l.split('\t').nth(1).map(String::from)).collect();
            let cur = fs::read_to_string(db.join("current.conllu")).map_err(|e| e.to_string())?;
            let fresh_file = db.join("fresh.conllu");
            fs::write(&fresh_file, cur.split("\n\n").filter(|b| ids_of(b).first().is_some_and(|id| fresh.contains(id) && !test.contains(id))).map(|b| format!("{}\n\n", b.trim_end())).collect::<String>()).map_err(|e| e.to_string())?;
            let train = db::training_file(&db, &test)?;
            let init = gparse_for(c, usize::MAX);
            let n = fs::read_dir(&models).map(|d| d.count()).unwrap_or(0);
            let out = models.join(format!("live-{n:04}"));
            let mut cmd = Command::new(&c.gparse);
            cmd.arg("train").arg(&out).args(&c.finetune_base).arg(&train).arg("--").arg(&c.dev);
            cmd.env("GP_INIT", &init).env("GP_ALWAYS", &fresh_file).env("GP_SAMPLE", "0.15").env("GP_EPOCHS", "2").env("GP_LR", "0.0005").env("GP_TOKENS", "1000");
            sh(&mut cmd, &models.join(format!("live-{n:04}.log")))?;
            eprintln!("trainer: db agreed ({m} multi-version sentences, {w} words moved); {} fresh sentences → {}", fresh.len(), out.display());
            seen = size;
        }
        std::thread::sleep(std::time::Duration::from_secs(60));
    }
}

pub fn run(c: &Cfg, from: usize, to: usize) -> R<()> {
    let max: usize = std::env::var("CYCLE_INFLIGHT").ok().and_then(|x| x.parse().ok()).unwrap_or(3);
    let (_, test) = exclude(c)?;
    let mut flight: Vec<usize> = (from..=to).filter(|&k| c.root.join(format!("round{k}/batch-id.txt")).exists() && !done(c, k)).collect();
    let mut next = (from..=to).find(|&k| !done(c, k) && !flight.contains(&k)).unwrap_or(to + 1);
    let mut pool_empty = false;
    let tiers = std::env::var("CYCLE_TIERS").as_deref() != Ok("0");
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
        // collect whatever Haiku has finished; its round goes on to Sonnet
        for k in flight.clone() {
            if ready(c, k)? {
                collect(c, k, &test)?;
                flight.retain(|&x| x != k);
                if tiers && review_feed(c, k, Tier::Sonnet)? {
                    reviews.push((k, Tier::Sonnet));
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
                if t == Tier::Sonnet && review_feed(c, k, Tier::Opus)? {
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
