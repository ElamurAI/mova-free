//! cycle — the MMM teaching cycle, one round after another (design note, 15.10: "not just sentences to Haiku, together with
//! the MMM parse — cyclic learning").
//!
//!   cycle round <k>          run round k (steps already done are skipped: a broken round resumes)
//!   CYCLE_FLOW=pre (default): MMM + Stanza/Trankit/spaCy trf → treeagree → Haiku patches the agreed tree;
//!   CYCLE_FLOW=post: MMM → Haiku → treeagree (rounds 1–3, 15.10)
//!   cycle run <from> <to>    rounds from..=to, stops early when the pool is used up
//!   cycle daemon <from> <to> the asynchronous cycle (`daemon.rs`): our hardware feeds rounds while Haiku works;
//!                            results improve the parse database (`db.rs`, $MOVA_BIG/parsedb) and fine-tune the GPU MMM
//!   cycle db-init <seed>     a fresh parse database: the seed corpus, then every finished round
//!   cycle db-agree           agree every sentence's versions (history.conllu): weighted vote per word → current
//!   cycle trainer            the GPU MMM learns on its own: whenever the database changed, a ~30 s fine-tune
//!                            from the newest model → models/live-<n>; never waits for the teachers
//!   cycle report             the per-round table (`cycle.tsv`)
//!
//! A round:
//! 1. `annot batch-prepare`: the MMM (`en`, retrained on the base data + everything the cycle agreed so far)
//!    drafts N random pool sentences not yet in the cycle corpus; the drafts go to Haiku as patch requests;
//! 2. `annot batch-run` (Message Batches, 50% price) and `annot batch-apply` (patches, UD gates, `school-cycle.conllu`);
//! 3. Stanza, Trankit and spaCy trf parse the round's trees (`neural.py`, GPU);
//! 4. `treeagree agree` turns them into agreed trees, appended to `school-cycle-agreed.conllu` — the next round's
//!    MMM trains on it;
//! 5. a row in `cycle.tsv`: how much Haiku changed the MMM draft (the number that must fall), how much treeagree
//!    changed after Haiku, the cost.
//!
//! Settings (env, with defaults for s2): CYCLE_ROOT, CYCLE_POOL, CYCLE_N (200), CYCLE_SEED (1015), CYCLE_BASE (train
//! files, space-separated), CYCLE_CORPUS, CYCLE_AGREED, CYCLE_GPARSE_MODEL (GPU parser re-parses the draft; after
//! each round it is fine-tuned in ~30 s into `models/round<k>`, the next round's drafter), CYCLE_EN_MODEL (a saved
//! `en` model instead of retraining it), CYCLE_LABEL (e.g. `recheck`: rounds over sentences already in
//! the MMM's training data are marked `pre/recheck` in `cycle.tsv` — their numbers are flattered). Mechanics only: no network except through `annot`.

mod daemon;
mod db;

use std::fmt::Write as _;
use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

fn env(k: &str, d: &str) -> String {
    std::env::var(k).unwrap_or_else(|_| d.to_string())
}

struct Cfg {
    root: PathBuf,
    pool: PathBuf,
    n: usize,
    seed: u64,
    base: Vec<String>,
    corpus: PathBuf,
    agreed: PathBuf,
    annot: String,
    treeagree: String,
    jobs: String,
    neural: String,
    venv: String,
    venv310: String,
    /// "pre" (MMM + open parsers → treeagree → Haiku, 15.10) or "post" (MMM → Haiku → treeagree)
    flow: String,
    /// corpus the retag FEATS and lemmas are learned from in the pre flow
    lex: String,
    /// GPU parser model dir: in the pre flow it re-parses the MMM draft (tags, heads, relations; GP_LEX refills
    /// LEMMA/FEATS) before the open parsers and treeagree; empty — the perceptron draft goes on as is
    gparse_model: String,
    gparse: String,
    /// a saved `en` model for the drafts (tokens, lemmas, features); empty — `annot` trains one each round
    en_model: String,
    /// EWT dev for the GPU fine-tune's model choice
    dev: String,
    /// suffix of the flow column, e.g. "/recheck" for rounds over sentences the MMM has already trained on
    label: String,
    /// sentences per round for Sonnet (0.2% of the 47 k primary-school sentences) and Opus (0.05%)
    sonnet_n: usize,
    opus_n: usize,
    /// held-out test sentences (never drafted, never trained on)
    test: String,
    /// training files of the GPU fine-tune besides the parse database (EWT, ESLSpok)
    finetune_base: Vec<String>,
    /// the pragmatic reader (`prag world`, `prag label`, `prag agree`); empty — none
    prag: String,
    /// the pragmatics SLM's silver run dir and en model, the witnesses' script
    prag_silver: String,
    prag_en: String,
    witness: String,
}

impl Cfg {
    fn load() -> Cfg {
        // the data directory ($MOVA_DATA, else ~/mova-data) and the heavy-data directory ($MOVA_BIG, else /data/mova)
        let d = std::env::var("MOVA_DATA").unwrap_or_else(|_| format!("{}/mova-data", std::env::var("HOME").unwrap_or_default()));
        let b = std::env::var("MOVA_BIG").unwrap_or_else(|_| "/data/mova".to_string());
        let repo = concat!(env!("CARGO_MANIFEST_DIR"), "/../..");
        let root = PathBuf::from(env("CYCLE_ROOT", &format!("{b}/runs/cycle-2026-10-15")));
        let u = format!("{d}/en/ud/ud-treebanks-v2.18/UD_English-EWT/en_ewt-ud-train.conllu");
        Cfg {
            pool: PathBuf::from(env("CYCLE_POOL", &root.join("new.tsv").display().to_string())),
            n: env("CYCLE_N", "200").parse().unwrap_or(200),
            seed: env("CYCLE_SEED", "1015").parse().unwrap_or(1015),
            base: env("CYCLE_BASE", &format!("{u} {d}/en/ud-mova/eslspok-train.conllu {d}/corpus/school/school-agreed.conllu"))
                .split_whitespace()
                .map(String::from)
                .collect(),
            corpus: PathBuf::from(env("CYCLE_CORPUS", &format!("{d}/corpus/school/school-cycle.conllu"))),
            agreed: PathBuf::from(env("CYCLE_AGREED", &format!("{d}/corpus/school/school-cycle-agreed.conllu"))),
            annot: env("CYCLE_ANNOT", &format!("{b}/target/annot/release/annot")),
            treeagree: env("CYCLE_TREEAGREE", &format!("{b}/target/tools-treeagree/release/treeagree")),
            jobs: env("CYCLE_JOBS", &format!("{b}/target/tools-jobs/release/jobs")),
            neural: env("CYCLE_NEURAL", &format!("{b}/runs/parse-school-2026-10-15/bin/neural.py")),
            venv: env("CYCLE_VENV", &format!("{b}/parsers/venv/bin/python")),
            venv310: env("CYCLE_VENV310", &format!("{b}/parsers/venv310/bin/python")),
            flow: env("CYCLE_FLOW", "pre"),
            lex: env("CYCLE_LEX", &format!("{d}/corpus/school/school-agreed.conllu")),
            gparse_model: env("CYCLE_GPARSE_MODEL", ""),
            gparse: env("CYCLE_GPARSE", &format!("{b}/target/gparse/release/gparse")),
            en_model: env("CYCLE_EN_MODEL", ""),
            dev: env("CYCLE_DEV", &format!("{d}/en/ud/ud-treebanks-v2.18/UD_English-EWT/en_ewt-ud-dev.conllu")),
            label: std::env::var("CYCLE_LABEL").map(|l| format!("/{l}")).unwrap_or_default(),
            sonnet_n: env("CYCLE_SONNET_N", "94").parse().unwrap_or(94),
            opus_n: env("CYCLE_OPUS_N", "24").parse().unwrap_or(24),
            test: env("CYCLE_TEST", &format!("{b}/runs/mmm-parse-2026-10-15/data/school-agreed-test.conllu")),
            prag: env("CYCLE_PRAG", &format!("{b}/target/prag/release/prag")),
            prag_silver: env("CYCLE_PRAG_SILVER", &format!("{b}/runs/prag-2026-09-26")),
            prag_en: env("CYCLE_PRAG_EN", &format!("{d}/en/models/en.bin")),
            witness: env("CYCLE_WITNESS", &format!("{repo}/prag/witness/witness.py")),
            finetune_base: vec![u.clone(), format!("{d}/en/ud-mova/eslspok-train.conllu")],
            root,
        }
    }
}

type R<T> = Result<T, String>;

/// Run a command, its stdout+stderr into `log`; fail-fast on a non-zero exit.
fn sh(cmd: &mut Command, log: &Path) -> R<()> {
    let out = cmd.output().map_err(|e| format!("{cmd:?}: {e}"))?;
    let mut text = out.stdout;
    text.extend(out.stderr);
    fs::write(log, &text).map_err(|e| e.to_string())?;
    if !out.status.success() {
        let tail: String = String::from_utf8_lossy(&text).lines().rev().take(5).collect::<Vec<_>>().join(" | ");
        return Err(format!("{cmd:?} failed ({}); log {}: {tail}", out.status, log.display()));
    }
    Ok(())
}

fn ids_of(conllu: &str) -> Vec<String> {
    conllu.lines().filter_map(|l| l.strip_prefix("# sent_id = ").map(String::from)).collect()
}

/// `in/ids.txt` and `in/tokens.txt` for the parser runner, from a CoNLL-U file.
fn parser_input(conllu: &str, dir: &Path) -> R<usize> {
    fs::create_dir_all(dir.join("in")).map_err(|e| e.to_string())?;
    fs::create_dir_all(dir.join("out")).map_err(|e| e.to_string())?;
    fs::create_dir_all(dir.join("log")).map_err(|e| e.to_string())?;
    let (mut ids, mut toks) = (String::new(), String::new());
    let mut cur: Vec<&str> = Vec::new();
    let mut n = 0;
    for l in conllu.lines().chain(std::iter::once("")) {
        if let Some(id) = l.strip_prefix("# sent_id = ") {
            ids.push_str(id);
            ids.push('\n');
        } else if l.is_empty() {
            if !cur.is_empty() {
                toks.push_str(&cur.join(" "));
                toks.push('\n');
                cur.clear();
                n += 1;
            }
        } else if !l.starts_with('#') {
            let c: Vec<&str> = l.split('\t').collect();
            if c.len() == 10 && !c[0].contains('-') && !c[0].contains('.') {
                cur.push(c[1]);
            }
        }
    }
    if ids.lines().count() != n {
        return Err(format!("{} sentence ids for {n} sentences", ids.lines().count()));
    }
    fs::write(dir.join("in/ids.txt"), ids).map_err(|e| e.to_string())?;
    fs::write(dir.join("in/tokens.txt"), toks).map_err(|e| e.to_string())?;
    Ok(n)
}

/// A number after `key` in `text` ("Changed: 277 words (4.91%)" → key "words (" → 4.91).
fn num_after(text: &str, key: &str) -> Option<f64> {
    let i = text.find(key)? + key.len();
    let s: String = text[i..].chars().take_while(|c| c.is_ascii_digit() || *c == '.').collect();
    s.parse().ok()
}

/// From `apply.log`: (UAS, LAS) of Haiku's final trees against the MMM draft, cost, sentences into the corpus.
fn apply_metrics(log: &str) -> (f64, f64, f64, usize) {
    let row = log.lines().find(|l| l.starts_with("| final | Rust |")).unwrap_or("");
    let c: Vec<f64> = row.split('|').filter_map(|x| x.trim().parse().ok()).collect();
    let (uas, las) = (c.get(6).copied().unwrap_or(f64::NAN), c.get(7).copied().unwrap_or(f64::NAN));
    let cost = log.lines().find_map(|l| l.split("API equivalent at batch price $").nth(1).and_then(|x| x.trim().parse().ok())).unwrap_or(f64::NAN);
    let added = log.lines().find_map(|l| l.split(": +").nth(1).and_then(|x| x.split_whitespace().next()?.parse().ok())).unwrap_or(0);
    (uas, las, cost, added)
}

/// Stanza, spaCy trf and Trankit on the sentences of `conllu`, into `<v>/out/` (skipped when already there).
fn parsers(c: &Cfg, conllu: &str, v: &Path, k: usize) -> R<usize> {
    let n = parser_input(conllu, v)?;
    for (p, py) in [("stanza", &c.venv), ("spacy-trf", &c.venv), ("trankit", &c.venv310)] {
        if !v.join(format!("out/{p}.conllu")).exists() {
            eprintln!("round {k}: {p} on {n} sentences");
            let mut cmd = Command::new(py);
            cmd.args(["-I", &c.neural, p]).arg(v).env("PYTHONIOENCODING", "utf-8").env("LANG", "C.UTF-8");
            sh(&mut cmd, &v.join(format!("log/{p}.log")))?;
        }
    }
    Ok(n)
}

/// Append the blocks of `src` whose ids are not yet in `dst`.
fn append_new(src: &str, dst: &Path) -> R<usize> {
    let have: std::collections::BTreeSet<String> = ids_of(&fs::read_to_string(dst).unwrap_or_default()).into_iter().collect();
    let mut add = String::new();
    let mut n = 0;
    for block in src.split("\n\n").filter(|b| !b.trim().is_empty()) {
        if ids_of(block).first().is_some_and(|id| !have.contains(id)) {
            add.push_str(block.trim_end());
            add.push_str("\n\n");
            n += 1;
        }
    }
    if !add.is_empty() {
        use std::io::Write as _;
        let mut f = fs::OpenOptions::new().create(true).append(true).open(dst).map_err(|e| e.to_string())?;
        f.write_all(add.as_bytes()).map_err(|e| e.to_string())?;
    }
    Ok(n)
}

/// The GPU model to draft with: the newest model under `models/` (fine-tuned after a round, or by the trainer),
/// else CYCLE_GPARSE_MODEL. `k` is kept for the old per-round rule (rounds below k only) when no trainer runs.
fn gparse_for(c: &Cfg, k: usize) -> String {
    let dirs: Vec<(std::time::SystemTime, PathBuf)> = fs::read_dir(c.root.join("models"))
        .map(|d| d.flatten().map(|e| e.path()).filter(|p| p.join("model.safetensors").exists()).filter(|p| {
            p.file_name().and_then(|n| n.to_str()).and_then(|n| n.strip_prefix("round")).and_then(|x| x.parse::<usize>().ok()).is_none_or(|j| j < k)
        }).filter_map(|p| fs::metadata(p.join("model.safetensors")).and_then(|m| m.modified()).ok().map(|t| (t, p))).collect())
        .unwrap_or_default();
    dirs.into_iter().max().map(|(_, p)| p.display().to_string()).unwrap_or_else(|| c.gparse_model.clone())
}

/// The pre flow (15.10): the MMM drafts, Stanza/Trankit/spaCy trf parse the same sentences, treeagree agrees
/// them, and only the agreed tree goes to Haiku. Haiku's corrected trees are the round's result.
fn round_pre(c: &Cfg, k: usize) -> R<bool> {
    let dir = c.root.join(format!("round{k}"));
    let pre = dir.join("pre");
    fs::create_dir_all(&pre).map_err(|e| e.to_string())?;
    let mut train = c.base.clone();
    if c.agreed.exists() {
        train.push(c.agreed.display().to_string());
    }
    let annot_env = |cmd: &mut Command| {
        cmd.env("ANNOT_FLOW", "patch").env("ANNOT_BATCH", "8").env("ANNOT_MODEL", "claude-haiku-5-5").env("ANNOT_EFFORT", "medium").env("ANNOT_CORPUS", &c.corpus);
    };
    // 1. MMM drafts of N new pool sentences
    let mmm = dir.join("mmm.conllu");
    if !mmm.exists() {
        eprintln!("round {k}: MMM drafts ({} training files)", train.len());
        let mut cmd = Command::new(&c.jobs);
        cmd.args(["run", "--name", &format!("cycle{k}-drafts"), "--mem-gb", "12", "--secs", "3600", "--", &c.annot, "drafts"]);
        cmd.args(&train).arg("--").arg(&c.pool).arg(c.n.to_string()).arg((c.seed + k as u64).to_string()).arg(&mmm);
        annot_env(&mut cmd);
        if !c.en_model.is_empty() {
            cmd.env("ANNOT_EN_MODEL", &c.en_model);
        }
        sh(&mut cmd, &dir.join("drafts.log"))?;
    }
    // 1b. the GPU parser re-parses the draft (hybrid: its tags and tree, the perceptron's lemmas and features)
    let mmm = if c.gparse_model.is_empty() {
        mmm
    } else {
        let gp = dir.join("mmm-gparse.conllu");
        if !gp.exists() {
            let model = gparse_for(c, k);
            eprintln!("round {k}: GPU parser over the draft ({model})");
            let mut cmd = Command::new(&c.gparse);
            cmd.arg("annotate").arg(&model).arg(&mmm).arg(&gp).env("GP_LEX", &c.lex);
            sh(&mut cmd, &dir.join("gparse.log"))?;
        }
        gp
    };
    let drafts = fs::read_to_string(&mmm).map_err(|e| e.to_string())?;
    if ids_of(&drafts).is_empty() {
        eprintln!("round {k}: no sentences left in the pool");
        return Ok(false);
    }
    // 2–3. open parsers, treeagree over the MMM draft
    parsers(c, &drafts, &pre, k)?;
    if !pre.join("agreed.conllu").exists() {
        let mut cmd = Command::new(&c.treeagree);
        cmd.arg("agree").arg(&mmm).arg(&pre).arg(pre.join("agreed.conllu")).arg(pre.join("agree-rep")).env("TREEAGREE_LEX", &c.lex);
        sh(&mut cmd, &pre.join("log/treeagree.log"))?;
    }
    // 4–5. the agreed trees to Haiku as drafts; patches, gates, corpus
    if !dir.join("batch-state.json").exists() {
        let mut cmd = Command::new(&c.annot);
        cmd.arg("batch-prepare").args(&c.base).arg("--").arg(&c.pool).arg("0").arg("0").arg(&dir);
        annot_env(&mut cmd);
        cmd.env("ANNOT_DRAFTS", pre.join("agreed.conllu"));
        sh(&mut cmd, &dir.join("prepare.log"))?;
    }
    if !dir.join("batch-results.jsonl").exists() {
        eprintln!("round {k}: batch to Haiku (polls every 60 s)");
        sh(Command::new(&c.annot).arg("batch-run").arg(&dir), &dir.join("run.log"))?;
    }
    if !dir.join("final.conllu").exists() {
        let mut cmd = Command::new(&c.annot);
        cmd.arg("batch-apply").arg(&dir).arg(dir.join("batch-results.jsonl"));
        annot_env(&mut cmd);
        cmd.env("ANNOT_DRAFT_LABEL", if c.gparse_model.is_empty() { "MMM (en) + Stanza, Trankit, spaCy trf (treeagree)" } else { "MMM (gparse tree, en lemmas) + Stanza, Trankit, spaCy trf (treeagree)" });
        sh(&mut cmd, &dir.join("apply.log"))?;
    }
    // 6. Haiku's trees train the next MMM; the verifier only reports
    let fin = fs::read_to_string(dir.join("final.conllu")).map_err(|e| e.to_string())?;
    append_new(&fin, &c.agreed)?;
    sh(Command::new(&c.treeagree).arg("check").arg(dir.join("final.conllu")), &dir.join("check.log"))?;
    // 7. the GPU MMM learns the round right away: a warm start from the current model, two short epochs over all of
    //    the round's trees plus a random 15% of the rest (design note: no full convergence, subsets) — about half a minute
    if !c.gparse_model.is_empty() {
        let out = c.root.join(format!("models/round{k}"));
        if !out.join("model.safetensors").exists() {
            let init = gparse_for(c, k);
            eprintln!("round {k}: GPU MMM fine-tune from {init}");
            let mut cmd = Command::new(&c.gparse);
            cmd.arg("train").arg(&out).args(&train).arg("--").arg(&c.dev);
            cmd.env("GP_INIT", &init).env("GP_ALWAYS", dir.join("final.conllu")).env("GP_SAMPLE", "0.15").env("GP_EPOCHS", "2").env("GP_LR", "0.0005").env("GP_TOKENS", "1000");
            sh(&mut cmd, &dir.join("finetune.log"))?;
        }
    }
    let (uas, las, cost, added) = apply_metrics(&fs::read_to_string(dir.join("apply.log")).unwrap_or_default());
    let changed = num_after(&fs::read_to_string(pre.join("agree-rep/agree.md")).unwrap_or_default(), "words (").unwrap_or(f64::NAN);
    row(c, k, &format!("pre{}", c.label), added, uas, las, changed, cost)?;
    eprintln!("round {k} (pre): {added} sentences; treeagree changed the MMM draft {changed:.2}%; Haiku kept the agreed draft UAS {uas:.2} LAS {las:.2}; ${cost:.3}");
    Ok(true)
}

/// One row of `cycle.tsv` (a round's old row is replaced).
#[allow(clippy::too_many_arguments)]
fn row(c: &Cfg, k: usize, flow: &str, added: usize, uas: f64, las: f64, changed: f64, cost: f64) -> R<()> {
    let table = c.root.join("cycle.tsv");
    let head = "round\tflow\tsentences\tdraft_kept_uas\tdraft_kept_las\ttreeagree_changed_pct\tcost_usd\n";
    let old = fs::read_to_string(&table).unwrap_or_default();
    let mut t = String::from(head);
    for l in old.lines().skip(1) {
        let c: Vec<&str> = l.split('\t').collect();
        if c.first() == Some(&k.to_string().as_str()) {
            continue;
        }
        // rows written before the flow column: post
        if c.len() == 6 {
            writeln!(t, "{}\tpost\t{}", c[0], c[1..].join("\t")).unwrap();
        } else {
            writeln!(t, "{l}").unwrap();
        }
    }
    writeln!(t, "{k}\t{flow}\t{added}\t{uas:.2}\t{las:.2}\t{changed:.2}\t{cost:.3}").unwrap();
    fs::write(&table, t).map_err(|e| e.to_string())
}

fn round(c: &Cfg, k: usize) -> R<bool> {
    if c.flow == "pre" {
        return round_pre(c, k);
    }
    let dir = c.root.join(format!("round{k}"));
    fs::create_dir_all(&dir).map_err(|e| e.to_string())?;
    let annot_env = |cmd: &mut Command| {
        cmd.env("ANNOT_FLOW", "patch").env("ANNOT_BATCH", "8").env("ANNOT_MODEL", "claude-haiku-5-5").env("ANNOT_EFFORT", "medium").env("ANNOT_CORPUS", &c.corpus);
    };
    // 1. drafts by the MMM retrained on base + cycle-agreed
    if !dir.join("batch-state.json").exists() {
        let mut train = c.base.clone();
        if c.agreed.exists() {
            train.push(c.agreed.display().to_string());
        }
        eprintln!("round {k}: MMM drafts ({} training files) → Haiku patch requests", train.len());
        let mut cmd = Command::new(&c.jobs);
        cmd.args(["run", "--name", &format!("cycle{k}-prepare"), "--mem-gb", "12", "--secs", "3600", "--", &c.annot, "batch-prepare"]);
        cmd.args(&train).arg("--").arg(&c.pool).arg(c.n.to_string()).arg((c.seed + k as u64).to_string()).arg(&dir);
        annot_env(&mut cmd);
        sh(&mut cmd, &dir.join("prepare.log"))?;
        if fs::metadata(dir.join("batch-requests.jsonl")).map_or(0, |m| m.len()) == 0 {
            eprintln!("round {k}: no sentences left in the pool");
            return Ok(false);
        }
    }
    // 2. Haiku through Message Batches; patches, gates, corpus
    if !dir.join("batch-results.jsonl").exists() {
        eprintln!("round {k}: batch to Haiku (polls every 60 s)");
        sh(Command::new(&c.annot).arg("batch-run").arg(&dir), &dir.join("run.log"))?;
    }
    if !dir.join("final.conllu").exists() {
        let mut cmd = Command::new(&c.annot);
        cmd.arg("batch-apply").arg(&dir).arg(dir.join("batch-results.jsonl"));
        annot_env(&mut cmd);
        sh(&mut cmd, &dir.join("apply.log"))?;
    }
    // 3. the three open parsers on the round's trees
    let v = dir.join("verify");
    let fin = fs::read_to_string(dir.join("final.conllu")).map_err(|e| e.to_string())?;
    let n = parsers(c, &fin, &v, k)?;
    // 4. agreed trees → the cycle corpus the next MMM learns from
    if !v.join("agreed.conllu").exists() {
        sh(Command::new(&c.treeagree).arg("agree").arg(dir.join("final.conllu")).arg(&v).arg(v.join("agreed.conllu")).arg(v.join("agree-rep")), &v.join("log/treeagree.log"))?;
    }
    append_new(&fs::read_to_string(v.join("agreed.conllu")).map_err(|e| e.to_string())?, &c.agreed)?;
    // 5. the round's row
    let (uas, las, cost, added) = apply_metrics(&fs::read_to_string(dir.join("apply.log")).unwrap_or_default());
    let changed = num_after(&fs::read_to_string(v.join("agree-rep/agree.md")).unwrap_or_default(), "words (").unwrap_or(f64::NAN);
    row(c, k, &format!("post{}", c.label), added, uas, las, changed, cost)?;
    eprintln!("round {k}: {n} sentences; Haiku kept the MMM draft UAS {uas:.2} LAS {las:.2}; treeagree changed {changed:.2}%; ${cost:.3}");
    Ok(true)
}

fn main() {
    let a: Vec<String> = std::env::args().collect();
    let c = Cfg::load();
    let r = match a.get(1).map(String::as_str) {
        Some("round") if a.len() == 3 => a[2].parse().map_err(|_| "round number".to_string()).and_then(|k| round(&c, k).map(|_| ())),
        Some("run") if a.len() == 4 => (|| {
            let (from, to): (usize, usize) = (a[2].parse().map_err(|_| "from")?, a[3].parse().map_err(|_| "to")?);
            for k in from..=to {
                if !round(&c, k)? {
                    break;
                }
            }
            Ok(())
        })(),
        Some("daemon") if a.len() == 4 => (|| daemon::run(&c, a[2].parse().map_err(|_| "from")?, a[3].parse().map_err(|_| "to")?))(),
        Some("db-agree") => db::agree(&db::dir()).map(|(m, w)| println!("db: {m} sentences with several versions; the vote moved {w} words off the strongest version")),
        Some("trainer") => daemon::trainer(&c),
        Some("db-init") if a.len() == 3 => db::init(&db::dir(), Path::new(&a[2]), &c.root),
        Some("report") => fs::read_to_string(c.root.join("cycle.tsv")).map(|t| print!("{t}")).map_err(|e| e.to_string()),
        _ => Err("usage: cycle round <k> | cycle run <from> <to> | cycle daemon <from> <to> | cycle db-init <seed.conllu> | cycle report".into()),
    };
    if let Err(e) = r {
        eprintln!("cycle: {e}");
        std::process::exit(1);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn metrics_from_logs() {
        let log = "| final | Rust | 98.76 | 98.88 | 98.65 | 98.65 | 97.89 | 99.75 | 96.31 | 94.08 | 93.92 |\nbatch: 25 requests, failed 0; tokens: input 1; API equivalent at batch price $0.082\ninto corpus /x.conllu: +188 sentences (was 0)\n";
        assert_eq!(apply_metrics(log), (96.31, 94.08, 0.082, 188));
        assert_eq!(num_after("- Changed: 277 words (4.91%) in 124", "words ("), Some(4.91));
        // negative control: no table row → NaN, not a fake number
        assert!(apply_metrics("nothing").0.is_nan());
    }

    #[test]
    fn parser_input_matches_ids() {
        let d = std::env::temp_dir().join(format!("cycle-pi-{}", std::process::id()));
        let src = "# sent_id = a\n1\tHi\thi\tINTJ\tUH\t_\t0\troot\t_\t_\n2\t!\t!\tPUNCT\t.\t_\t1\tpunct\t_\t_\n\n# sent_id = b\n1\tYes\tyes\tINTJ\tUH\t_\t0\troot\t_\t_\n\n";
        assert_eq!(parser_input(src, &d).unwrap(), 2);
        assert_eq!(fs::read_to_string(d.join("in/tokens.txt")).unwrap(), "Hi !\nYes\n");
        // negative control: an id without words is refused
        assert!(parser_input("# sent_id = x\n\n# sent_id = y\n1\tA\ta\tX\tX\t_\t0\troot\t_\t_\n\n", &d).is_err());
        fs::remove_dir_all(&d).ok();
    }
}
