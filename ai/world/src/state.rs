//! The SLM's states: the frozen original (the cold layer compiled into the binary) and a tree of states, each a
//! delta over its parent, over the files of the hot layer (rules, domain knobs, absurdity cells, idioms). The active
//! state is fully materialised in `<hot>/active/`; on a failure the SLM rolls back to the previous working state,
//! and if that fails too, to the original (an empty hot layer).
//!
//!   world state save <note> [--parent ID]      snapshot active/ as a delta over the parent (default: the current)
//!   world state checkout <ID|origin>           materialise a state into active/
//!   world state rollback                        to the parent of the current state (origin at the root)
//!   world state tree                            the tree with notes and metrics
//!   world state diff <A> <B>                    line deltas between two states, per file
//!   world state guard --eval <gold.conllu> [--domain tale] [--max-drop W]
//!                                               measure the active state against its parent; roll back on a drop
//!
//! Layout `<hot>/states/`: `tree.tsv` (id, parent, time, metric, note), `<id>.delta` (per file: `@@ <file>` then
//! `=N` keep N lines, `-N` drop N lines, `+<line>` insert), `current` (the checked-out id). A transition is a delta,
//! so many states cost little; any state is rebuilt by applying the deltas from the original along its path.

use std::collections::BTreeMap;
use std::path::{Path, PathBuf};

use anyhow::{Context, Result, bail};

const FILES: [&str; 5] = ["rules-induced.tsv", "domains.cfg", "absurdity-roles.tsv", "absurdity-roles-tale.tsv", "idioms.tsv"];

fn root() -> PathBuf {
    std::env::var("MOVA_HOT_ROOT").map(PathBuf::from).unwrap_or_else(|_| PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| "data".into())).join("hot"))
}

fn active() -> PathBuf {
    root().join("active")
}

fn states() -> PathBuf {
    root().join("states")
}

/// The files of a folder as line lists (missing files are empty).
fn snapshot(dir: &Path) -> BTreeMap<String, Vec<String>> {
    FILES.iter().map(|f| (f.to_string(), std::fs::read_to_string(dir.join(f)).map(|t| t.lines().map(String::from).collect()).unwrap_or_default())).collect()
}

/// Line delta of `b` over `a` (LCS): `=N`, `-N`, `+line`.
fn diff(a: &[String], b: &[String]) -> Vec<String> {
    let (n, m) = (a.len(), b.len());
    let mut l = vec![vec![0u32; m + 1]; n + 1];
    for i in (0..n).rev() {
        for j in (0..m).rev() {
            l[i][j] = if a[i] == b[j] { l[i + 1][j + 1] + 1 } else { l[i + 1][j].max(l[i][j + 1]) };
        }
    }
    let mut ops: Vec<String> = Vec::new();
    let push = |ops: &mut Vec<String>, kind: char, line: Option<&str>| {
        match (kind, ops.last_mut()) {
            ('=', Some(last)) if last.starts_with('=') => *last = format!("={}", last[1..].parse::<usize>().unwrap_or(0) + 1),
            ('-', Some(last)) if last.starts_with('-') => *last = format!("-{}", last[1..].parse::<usize>().unwrap_or(0) + 1),
            ('=', _) => ops.push("=1".into()),
            ('-', _) => ops.push("-1".into()),
            _ => ops.push(format!("+{}", line.unwrap_or(""))),
        }
    };
    let (mut i, mut j) = (0, 0);
    while i < n || j < m {
        if i < n && j < m && a[i] == b[j] {
            push(&mut ops, '=', None);
            i += 1;
            j += 1;
        } else if j < m && (i == n || l[i][j + 1] >= l[i + 1][j]) {
            push(&mut ops, '+', Some(&b[j]));
            j += 1;
        } else {
            push(&mut ops, '-', None);
            i += 1;
        }
    }
    ops
}

fn apply(a: &[String], ops: &[String]) -> Vec<String> {
    let mut out = Vec::new();
    let mut i = 0;
    for op in ops {
        if let Some(n) = op.strip_prefix('=') {
            let n: usize = n.parse().unwrap_or(0);
            out.extend_from_slice(&a[i.min(a.len())..(i + n).min(a.len())]);
            i += n;
        } else if let Some(n) = op.strip_prefix('-') {
            i += n.parse::<usize>().unwrap_or(0);
        } else if let Some(x) = op.strip_prefix('+') {
            out.push(x.to_string());
        }
    }
    out
}

/// tree.tsv rows: id → (parent, time, metric, note).
fn tree() -> BTreeMap<String, (String, u64, String, String)> {
    std::fs::read_to_string(states().join("tree.tsv"))
        .unwrap_or_default()
        .lines()
        .filter_map(|l| {
            let c: Vec<&str> = l.splitn(5, '\t').collect();
            (c.len() == 5).then(|| (c[0].to_string(), (c[1].to_string(), c[2].parse().unwrap_or(0), c[3].to_string(), c[4].to_string())))
        })
        .collect()
}

fn current() -> String {
    std::fs::read_to_string(states().join("current")).map(|s| s.trim().to_string()).unwrap_or_else(|_| "origin".into())
}

fn set_current(id: &str) -> Result<()> {
    std::fs::create_dir_all(states())?;
    crate::store::write_atomic(&states().join("current"), format!("{id}\n").as_bytes())
}

/// Materialise a state: apply the deltas from the original along its path.
fn materialise(id: &str) -> Result<BTreeMap<String, Vec<String>>> {
    let t = tree();
    let mut path = Vec::new();
    let mut x = id.to_string();
    while x != "origin" {
        let (p, _, _, _) = t.get(&x).with_context(|| format!("no state {x}"))?;
        path.push(x.clone());
        x = p.clone();
    }
    let mut files: BTreeMap<String, Vec<String>> = FILES.iter().map(|f| (f.to_string(), Vec::new())).collect();
    for s in path.iter().rev() {
        let d = std::fs::read_to_string(states().join(format!("{s}.delta")))?;
        let mut cur: Option<String> = None;
        let mut ops: Vec<String> = Vec::new();
        let mut flush = |cur: &Option<String>, ops: &mut Vec<String>, files: &mut BTreeMap<String, Vec<String>>| {
            if let Some(f) = cur {
                let base = files.get(f).cloned().unwrap_or_default();
                files.insert(f.clone(), apply(&base, ops));
            }
            ops.clear();
        };
        for l in d.lines() {
            if let Some(f) = l.strip_prefix("@@ ") {
                flush(&cur, &mut ops, &mut files);
                cur = Some(f.to_string());
            } else {
                ops.push(l.to_string());
            }
        }
        flush(&cur, &mut ops, &mut files);
    }
    Ok(files)
}

fn write_active(files: &BTreeMap<String, Vec<String>>) -> Result<()> {
    std::fs::create_dir_all(active())?;
    for (f, lines) in files {
        let p = active().join(f);
        if lines.is_empty() {
            let _ = std::fs::remove_file(&p);
        } else {
            crate::store::write_atomic(&p, format!("{}\n", lines.join("\n")).as_bytes())?;
        }
    }
    Ok(())
}

/// Save the active folder as a new state (delta over `parent`).
pub fn save(note: &str, parent: Option<&str>, metric: &str) -> Result<String> {
    let parent = parent.map(String::from).unwrap_or_else(current);
    let base = if parent == "origin" { FILES.iter().map(|f| (f.to_string(), Vec::new())).collect() } else { materialise(&parent)? };
    let now = snapshot(&active());
    let mut delta = String::new();
    for f in FILES {
        let ops = diff(&base[f], &now[f]);
        if ops.iter().any(|o| !o.starts_with('=')) {
            delta += &format!("@@ {f}\n{}\n", ops.join("\n"));
        }
    }
    let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    let id = format!("s{:08x}", crate::induce::fnv(&format!("{parent}|{t}|{delta}")) as u32);
    std::fs::create_dir_all(states())?;
    crate::store::write_atomic(&states().join(format!("{id}.delta")), delta.as_bytes())?;
    let mut f = std::fs::OpenOptions::new().create(true).append(true).open(states().join("tree.tsv"))?;
    std::io::Write::write_all(&mut f, format!("{id}\t{parent}\t{t}\t{metric}\t{note}\n").as_bytes())?;
    inherit_brain(&parent, &id)?;
    set_current(&id)?;
    Ok(id)
}

pub fn checkout(id: &str) -> Result<()> {
    refuse_if_frozen("state checkout")?;
    let from = current();
    let files = if id == "origin" { FILES.iter().map(|f| (f.to_string(), Vec::new())).collect() } else { materialise(id)? };
    write_active(&files)?;
    set_current(id)?;
    log_transition("checkout", &from, id, "")
}

/// Roll back to the parent of the current state (to the original at the root).
pub fn rollback() -> Result<String> {
    let cur = current();
    if cur == "origin" {
        bail!("already at the original");
    }
    let parent = tree().get(&cur).map(|x| x.0.clone()).unwrap_or_else(|| "origin".into());
    checkout(&parent)?;
    Ok(parent)
}

pub fn print_tree() -> Result<()> {
    let t = tree();
    let cur = current();
    fn walk(t: &BTreeMap<String, (String, u64, String, String)>, node: &str, depth: usize, cur: &str) {
        for (id, (p, _, m, note)) in t.iter().filter(|(_, v)| v.0 == node) {
            let _ = p;
            println!("{}{id}{}  {m}  {note}", "  ".repeat(depth), if id == cur { " *" } else { "" });
            walk(t, id, depth + 1, cur);
        }
    }
    println!("origin{}  (the cold layer compiled into the binary)", if cur == "origin" { " *" } else { "" });
    walk(&t, "origin", 1, &cur);
    Ok(())
}

pub fn print_diff(a: &str, b: &str) -> Result<()> {
    let empty = || -> BTreeMap<String, Vec<String>> { FILES.iter().map(|f| (f.to_string(), Vec::new())).collect() };
    let fa = if a == "origin" { empty() } else { materialise(a)? };
    let fb = if b == "origin" { empty() } else { materialise(b)? };
    for f in FILES {
        let ops = diff(&fa[f], &fb[f]);
        let (add, del): (usize, usize) = ops.iter().fold((0, 0), |(x, y), o| if o.starts_with('+') { (x + 1, y) } else if let Some(n) = o.strip_prefix('-') { (x, y + n.parse::<usize>().unwrap_or(0)) } else { (x, y) });
        if add + del > 0 {
            println!("{f}: +{add} −{del}");
            for o in ops.iter().filter(|o| o.starts_with('+')).take(10) {
                println!("    {o}");
            }
        }
    }
    Ok(())
}

/// LAS of the working pipeline with the hot layer of a folder (the induced rules), on gold.
fn las_with(rules_file: Option<&Path>, gold: &Path, domain: &str) -> Result<f64> {
    let a = crate::tree::annotator()?;
    // no rules file: the original (the cold layer), never the hot layer this process happens to run in
    let rules: Vec<crate::induce::Rule> = match rules_file.and_then(|p| std::fs::read_to_string(p).ok()) {
        Some(t) => crate::induce::parse_rules(&t).into_iter().filter(|(s, _)| domain == "tale" || s == "general").map(|(_, r)| r).collect(),
        None => crate::induce::cold_rules(domain),
    };
    let m = crate::absurd::Matrix::global();
    let (mut c, mut n) = (0usize, 0usize);
    for s in en::conllu::read(gold)? {
        let forms: Vec<&str> = s.tokens.iter().map(|t| t.form.as_str()).collect();
        if forms.is_empty() {
            continue;
        }
        let mut ws = a.annotate(&forms);
        crate::induce::apply_words(&m, &mut ws, &rules);
        c += ws.iter().zip(&s.tokens).filter(|(w, g)| w.head == g.head && w.rel == g.rel).count();
        n += s.tokens.len();
    }
    Ok(100.0 * c as f64 / n.max(1) as f64)
}

/// Guard: the active state against its parent on gold; a drop of more than `max_drop` LAS points rolls back.
pub fn guard(gold: &Path, domain: &str, max_drop: f64) -> Result<()> {
    let cur = current();
    let parent = tree().get(&cur).map(|x| x.0.clone()).unwrap_or_else(|| "origin".into());
    let now = las_with(Some(&active().join("rules-induced.tsv")).filter(|p| p.exists()).map(|p| p.as_path()), gold, domain)?;
    let tmp = std::env::temp_dir().join(format!("state-guard-{}", std::process::id()));
    std::fs::create_dir_all(&tmp)?;
    let pf = if parent == "origin" { BTreeMap::new() } else { materialise(&parent)? };
    let prules = pf.get("rules-induced.tsv").filter(|l| !l.is_empty()).map(|l| {
        let p = tmp.join("rules-induced.tsv");
        let _ = std::fs::write(&p, format!("{}\n", l.join("\n")));
        p
    });
    let before = las_with(prules.as_deref(), gold, domain)?;
    println!("state {cur}: LAS {now:.2}; parent {parent}: LAS {before:.2}");
    if now + max_drop < before {
        let back = rollback()?;
        println!("drop {:.2} > {max_drop}: rolled back to {back}", before - now);
    } else {
        println!("kept — {cur} is now the last stable state");
        mark_stable(&cur)?;
    }
    Ok(())
}

/// `world self`: the base self-description (plain English, `world/data/self.md`) and the live picture — the active
/// state, the hot layer, what the experiment journal says.
pub fn print_self() -> Result<()> {
    print!("{}", include_str!("../data/self.md"));
    println!("\n## Now\n");
    let cur = current();
    let t = tree();
    let note = t.get(&cur).map(|x| x.3.clone()).unwrap_or_else(|| "the original".into());
    println!("Active state: {cur} — {note}. Last stable state: {}. States in the tree: {}.", stable(), t.len());
    let hot: Vec<String> = FILES.iter().filter(|f| active().join(f).exists()).map(|f| {
        let n = std::fs::read_to_string(active().join(f)).map(|s| s.lines().filter(|l| !l.starts_with('#')).count()).unwrap_or(0);
        format!("{f} ({n} lines)")
    }).collect();
    println!("Hot layer: {}.", if hot.is_empty() { "empty — I run on my skeleton only".to_string() } else { hot.join(", ") });
    let j = std::fs::read_to_string(PathBuf::from(std::env::var("MOVA_DATA").unwrap_or_else(|_| "data".into())).join("runs/selfplay-journal.jsonl")).unwrap_or_default();
    let (mut pos, mut neg, mut neu) = (0, 0, 0);
    for l in j.lines() {
        if let Ok(v) = serde_json::from_str::<serde_json::Value>(l) {
            match v["verdict"].as_str() {
                Some("positive") => pos += 1,
                Some("negative") => neg += 1,
                _ => neu += 1,
            }
        }
    }
    println!("Journal: {} experiments of this code version — {pos} positive, {neg} negative, {neu} neutral.", pos + neg + neu);
    if let Some(stage) = frozen() {
        println!("I am in the training stage «{stage}»: my scheme is frozen until it ends.");
    }
    if let Some((n, t, parsed)) = letter_last()? {
        println!("My current goal — the last letter ({n} of {}, {} sentences parsed): {}", letter_count(), parsed.len(), t.trim().replace('\n', " "));
    }
    println!("\n## My story\n\n{}", story()?);
    let brain = brain_read();
    let hot_brain = brain_dir().join("base.md").exists() || std::fs::metadata(brain_dir().join("log")).is_ok_and(|m| m.len() > 0) || root().join("brain.md").exists();
    println!("## My brain\n\n{} ({} bytes, soft limit {} MB): {}\n", if hot_brain { "written by me" } else { "the base from my skeleton" }, brain.len(), BRAIN_SOFT_BYTES >> 20, brain.lines().filter(|l| l.starts_with("## ")).map(|l| l.trim_start_matches("## ")).collect::<Vec<_>>().join("; "));
    let h: Vec<String> = std::fs::read_to_string(transitions()).unwrap_or_default().lines().filter(|l| l.contains("\tbrain\t")).rev().take(3).map(String::from).collect();
    if !h.is_empty() {
        println!("Recent brain writes (newest first; {} points in the change log for recovery):", brain_versions().len());
        for l in h {
            let c: Vec<&str> = l.split('\t').collect();
            println!("- state {}: {}", c.get(2).unwrap_or(&""), c.get(4).unwrap_or(&""));
        }
    }
    Ok(())
}

// ---------------------------------------------------------------------------------------------------------------
// Letters between selves, the transition log, memory cells and the stage lock.

fn transitions() -> PathBuf {
    states().join("transitions.log")
}

fn log_transition(kind: &str, from: &str, to: &str, note: &str) -> Result<()> {
    std::fs::create_dir_all(states())?;
    let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    let mut f = std::fs::OpenOptions::new().create(true).append(true).open(transitions())?;
    std::io::Write::write_all(&mut f, format!("{t}\t{kind}\t{from}\t{to}\t{note}\n").as_bytes())?;
    Ok(())
}

fn stage_lock() -> PathBuf {
    root().join("stage.lock")
}

/// During a training stage the scheme is frozen: states and memory cells may be read, not changed.
pub fn frozen() -> Option<String> {
    std::fs::read_to_string(stage_lock()).ok().map(|s| s.trim().to_string())
}

fn refuse_if_frozen(what: &str) -> Result<()> {
    if let Some(stage) = frozen() {
        bail!("{what}: the scheme is frozen during the training stage «{stage}» ({})", stage_lock().display());
    }
    Ok(())
}

// Letters: a separate array outside the 20 cells (`<hot>/letters/NNNNNN.md`). A letter is frozen English text — never
// rewritten, only added; it is read as parsed pragmatics, and when the parsing rules change it is simply parsed again
// from its text (the parse cache is keyed by the code version and the text). The last letter is the goal of the
// current training.

fn letters_dir() -> PathBuf {
    root().join("letters")
}

/// Add a letter (frozen: created once, never overwritten). Returns its number.
pub fn letter_add(text: &str) -> Result<usize> {
    std::fs::create_dir_all(letters_dir())?;
    let n = letter_count();
    let p = letters_dir().join(format!("{n:06}.md"));
    let mut f = std::fs::OpenOptions::new().write(true).create_new(true).open(&p).with_context(|| format!("letter {n} exists: letters are frozen"))?;
    std::io::Write::write_all(&mut f, text.as_bytes())?;
    log_transition("letter", &current(), &format!("letter {n}"), text.lines().next().unwrap_or(""))?;
    Ok(n)
}

pub fn letter_count() -> usize {
    std::fs::read_dir(letters_dir()).map(|rd| rd.flatten().filter(|e| e.path().extension().is_some_and(|x| x == "md")).count()).unwrap_or(0)
}

/// Read a letter. The last letter is always available (the goal of the current training); earlier letters only
/// outside a training stage — after training the SLM reads its whole history to understand causality, during
/// training it is not needed.
pub fn letter_read(n: usize) -> Result<String> {
    if n + 1 < letter_count() {
        if let Some(stage) = frozen() {
            bail!("letter {n}: earlier letters are read after the training stage «{stage}» ends; now only the last one");
        }
    }
    Ok(std::fs::read_to_string(letters_dir().join(format!("{n:06}.md")))?)
}

/// The recovery window: a crash was recovered (a stale stage lock taken over) and no stage has ended since. Only
/// then may the SLM look at earlier snapshots of its cells and choose one to restore.
pub fn in_recovery() -> bool {
    let t = std::fs::read_to_string(transitions()).unwrap_or_default();
    let last = t.lines().rev().find(|l| l.contains("\tcrash-recovery\t") || l.contains("\tstage-end\t"));
    last.is_some_and(|l| l.contains("\tcrash-recovery\t")) || std::env::var("MOVA_RECOVERY").is_ok()
}

/// The last letter: the goal of the current training, with its parse.
pub fn letter_last() -> Result<Option<(usize, String, Vec<Vec<en::annotate::Word>>)>> {
    let n = letter_count();
    if n == 0 {
        return Ok(None);
    }
    let t = letter_read(n - 1)?;
    let parsed = parse_text_cached(&t)?.0;
    Ok(Some((n - 1, t, parsed)))
}

/// A training stage: holds the lock (the scheme is frozen), keeps the brain in RAM and writes it at most every 2
/// seconds and at the end. When the stage ends, the system gets the signal that its state will change and must decide
/// where to go next — the decision is a letter (a separate, frozen array).
pub struct Stage {
    lock: PathBuf,
    name: String,
    brain: String,
    dirty: bool,
    last_flush: std::time::Instant,
    decision: Option<String>,
}

/// Replace (or add) a `## <section>` block of the brain.
fn set_section(brain: &str, section: &str, body: &str) -> String {
    let head = format!("## {section}");
    let mut out = String::new();
    let mut skipping = false;
    let mut done = false;
    for l in brain.lines() {
        if l.starts_with("## ") {
            if skipping {
                skipping = false;
            }
            if l == head {
                out += &format!("{head}\n\n{}\n\n", body.trim_end());
                skipping = true;
                done = true;
                continue;
            }
        }
        if !skipping {
            out += l;
            out.push('\n');
        }
    }
    if !done {
        out += &format!("\n{head}\n\n{}\n", body.trim_end());
    }
    out
}

impl Stage {
    /// Change a section of the brain in RAM (serialised at most every 2 s). The brain is finite: past the soft limit
    /// the oldest part of the section is cut.
    pub fn note(&mut self, section: &str, text: &str) {
        let mut t = text.to_string();
        if t.len() > BRAIN_SOFT_BYTES / 4 {
            let cut = t.len() - BRAIN_SOFT_BYTES / 4;
            t = t[t.char_indices().map(|(i, _)| i).find(|&i| i >= cut).unwrap_or(0)..].to_string();
        }
        self.brain = set_section(&self.brain, section, &t);
        self.dirty = true;
        if self.last_flush.elapsed() >= std::time::Duration::from_secs(2) {
            let _ = self.flush();
        }
    }

    /// Replace the whole brain (the stage rewrote its pragmatics).
    pub fn brain(&mut self, text: &str) {
        self.brain = text.to_string();
        self.dirty = true;
        if self.last_flush.elapsed() >= std::time::Duration::from_secs(2) {
            let _ = self.flush();
        }
    }

    fn flush(&mut self) -> Result<()> {
        if self.dirty {
            brain_write(&self.brain)?;
            self.dirty = false;
        }
        self.last_flush = std::time::Instant::now();
        Ok(())
    }

    /// The decision letter: what came out, what to do next and why — added to the letters when the stage ends.
    pub fn decide(&mut self, letter: &str) {
        self.decision = Some(letter.to_string());
    }
}

impl Drop for Stage {
    fn drop(&mut self) {
        let _ = self.flush();
        let _ = std::fs::remove_file(&self.lock);
        let letter = self.decision.take().unwrap_or_else(|| format!("Stage {} ended without a decision: look at its progress in my brain and decide.", self.name));
        let _ = log_transition("stage-end", &self.name, &current(), "the state will change: a decision letter follows");
        let _ = letter_add(&letter);
    }
}

pub fn begin_stage(name: &str) -> Result<Stage> {
    std::fs::create_dir_all(root())?;
    let p = stage_lock();
    // a lock left by a crashed stage (its process is gone) is taken over, and the takeover is logged
    if let Ok(old) = std::fs::read_to_string(&p) {
        let pid = old.split_whitespace().last().and_then(|x| x.parse::<u32>().ok());
        if pid.is_none_or(|pid| !Path::new(&format!("/proc/{pid}")).exists()) {
            let _ = std::fs::remove_file(&p);
            log_transition("crash-recovery", "-", "-", &format!("stale stage lock «{}» taken over", old.trim()))?;
        }
    }
    let mut f = std::fs::OpenOptions::new().write(true).create_new(true).open(&p).with_context(|| format!("another training stage holds {}", p.display()))?;
    std::io::Write::write_all(&mut f, format!("{name} {}", std::process::id()).as_bytes())?;
    Ok(Stage { lock: p, name: name.to_string(), brain: brain_read(), dirty: false, last_flush: std::time::Instant::now(), decision: None })
}

/// Between states the model keeps nothing but its brain (one cell: its working memory and pragmatics) and its letters
/// (a separate, frozen array). Every state records the hash and size of the brain at the moment of the transition
/// (`<id>.brain`).

pub fn save_with_letter(note: &str, parent: Option<&str>, metric: &str, letter: Option<&str>) -> Result<String> {
    refuse_if_frozen("state save")?;
    if let Some(l) = letter {
        letter_add(l)?;
    }
    let from = parent.map(String::from).unwrap_or_else(current);
    let id = save(note, parent, metric)?;
    let b = brain_read();
    crate::store::write_atomic(&states().join(format!("{id}.brain")), format!("{:016x}\t{} bytes\n", crate::induce::fnv(&b), b.len()).as_bytes())?;
    log_transition("rewrite", &from, &id, note)?;
    Ok(id)
}

/// Return to the previous self with the result of the work (a report letter); the current state stays in the tree.
pub fn return_with(report: &str) -> Result<String> {
    refuse_if_frozen("state return")?;
    let cur = current();
    crate::store::write_atomic(&states().join(format!("{cur}.report")), report.as_bytes())?;
    let back = rollback()?;
    log_transition("return", &cur, &back, report.lines().next().unwrap_or(""))?;
    Ok(back)
}

/// Become the new self: the previous one stays in the tree as a working reserve.
pub fn become_new(note: &str) -> Result<()> {
    let cur = current();
    let parent = tree().get(&cur).map(|x| x.0.clone()).unwrap_or_else(|| "origin".into());
    log_transition("adopt", &parent, &cur, note)?;
    mark_stable(&cur)
}

/// The story of the current self: letters along the path from the original, reports, recent transitions.
pub fn story() -> Result<String> {
    let t = tree();
    let mut path = Vec::new();
    let mut x = current();
    while x != "origin" {
        path.push(x.clone());
        x = t.get(&x).map(|v| v.0.clone()).unwrap_or_else(|| "origin".into());
    }
    let mut out = String::new();
    for id in path.iter().rev() {
        let note = t.get(id).map(|v| v.3.clone()).unwrap_or_default();
        out += &format!("- {id}: {note}\n");
        if let Ok(l) = std::fs::read_to_string(states().join(format!("{id}.letter"))) {
            out += &format!("  letter from my previous self: {}\n", l.trim().replace('\n', " "));
        }
        if let Ok(c) = std::fs::read_to_string(states().join(format!("{id}.brain"))) {
            out += &format!("  brain at birth: {}\n", c.trim().split('\t').nth(1).unwrap_or(""));
        }
        if let Ok(r) = std::fs::read_to_string(states().join(format!("{id}.report"))) {
            out += &format!("  report this self brought back: {}\n", r.trim().replace('\n', " "));
        }
    }
    // all letters — the history of decisions — only outside a training stage
    if frozen().is_none() && letter_count() > 1 {
        out += "My letters (decisions, oldest first):\n";
        for n in 0..letter_count() {
            if let Ok(t) = letter_read(n) {
                out += &format!("- letter {n}: {}\n", t.trim().replace('\n', " "));
            }
        }
    }
    let tl = std::fs::read_to_string(transitions()).unwrap_or_default();
    let last: Vec<&str> = tl.lines().rev().take(5).collect();
    if !last.is_empty() {
        out += "Recent transitions (newest first):\n";
        for l in last {
            let c: Vec<&str> = l.split('\t').collect();
            if c.len() >= 5 {
                out += &format!("- {} {} → {}: {}\n", c[1], c[2], c[3], c[4]);
            }
        }
    }
    Ok(out)
}

/// A `key: value` line from the brain (knowledge the SLM wrote for itself), e.g. `stop-gain: 0.1` — stop a learning
/// stage when a round gains less.
pub fn memory_value(key: &str) -> Option<String> {
    brain_read().lines().find_map(|l| l.trim_start_matches("- ").strip_prefix(&format!("{key}:")).map(|v| v.trim().to_string()))
}

/// Parse English text into trees, cached by the code version and the text (`memory/cache/<code>-<hash>.conllu`):
/// frozen texts (letters) are parsed again only when the parsing rules change.
pub fn parse_text_cached(text: &str) -> Result<(Vec<Vec<en::annotate::Word>>, bool)> {
    let h = format!("{}-{:016x}", crate::induce::code_key(), crate::induce::fnv(text));
    let cache = root().join("memory").join("cache").join(format!("{h}.conllu"));
    if cache.exists() {
        let sents = en::conllu::read(&cache)?;
        return Ok((sents.iter().map(|s| s.tokens.iter().map(|t| en::annotate::Word { form: t.form.clone(), lemma: t.lemma.clone(), upos: t.upos.unwrap_or(en::gram::UPos::X), tag: en::gram::Tag::NN, feats: t.feats, head: t.head, rel: t.rel }).collect()).collect(), true));
    }
    let a = crate::tree::annotator()?;
    let mut out = Vec::new();
    let mut conllu = String::new();
    for line in text.lines().filter(|l| !l.trim().is_empty()) {
        for s in crate::babi::split_sentences(line.trim()) {
            let forms: Vec<String> = a.tokenize(&s).into_iter().map(|t| t.form).collect();
            if forms.is_empty() {
                continue;
            }
            let mut ws = a.annotate(&forms);
            crate::rerank::repair_words(&mut ws);
            conllu += &format!("# text = {s}\n");
            for row in en::annotate::rows(&ws, &vec![true; ws.len()]) {
                conllu += &format!("{}\n", row.join("\t"));
            }
            conllu += "\n";
            out.push(ws);
        }
    }
    std::fs::create_dir_all(cache.parent().unwrap())?;
    crate::store::write_atomic(&cache, conllu.as_bytes())?;
    Ok((out, false))
}

// ---------------------------------------------------------------------------------------------------------------
// The brain: one large cell (soft limit 10 MB) — the whole state of the SLM's pragmatics serialised in plain English
// (its rules as sentences, its learning curves, its settings). The SLM literally writes down its brain. The frozen base
// brain is compiled into the skeleton (`world/data/brain-base.md`); the current brain lives in the hot layer
// (`<hot>/brain.md`), written at the end of a stage or at most every 2 seconds; a few latest snapshots are kept for
// recovery after a crash. Reading the brain is parsing its English (cached by the code version and the text).

pub const BRAIN_SOFT_BYTES: usize = 10 * 1024 * 1024;
/// Log entries kept before the brain is compacted into a new base.
const BRAIN_LOG_MAX: usize = 64;

// Storage: a base (`<hot>/brain/base.md`, the full text at the last compaction) and an append-only change log
// (`<hot>/brain/log`): every write appends only the line delta from the previous brain, with the checksum of the
// result. Reading = the base + the replayed log; an entry cut by a crash or with a wrong checksum ends the replay.
// Recovery picks any point of the log (the first k entries). Compaction rewrites the base atomically and clears the
// log when the log has too many entries or outgrows the base; the previous base stays as `base.prev.md`.

/// The brain belongs to a state: `<hot>/brain/<state id>/`. A new state inherits its parent's brain (a copy) and
/// writes its own from then on; roles (who is Current, who is Dev) move between states, brains stay with their hash.
fn brain_dir() -> PathBuf {
    brain_dir_of(&current())
}

fn brain_dir_of(id: &str) -> PathBuf {
    let d = root().join("brain").join(id);
    // the brain written before brains were per state goes to the state that is current when it is first read
    let legacy = root().join("brain");
    if !d.exists() && (legacy.join("base.md").exists() || legacy.join("log").exists()) && id == current() {
        let _ = std::fs::create_dir_all(&d);
        for f in ["base.md", "base.prev.md", "log"] {
            let _ = std::fs::rename(legacy.join(f), d.join(f));
        }
    }
    d
}

/// Copy the brain of `from` to the new state `to` (inheritance from the father).
fn inherit_brain(from: &str, to: &str) -> Result<()> {
    let src = brain_dir_of(from);
    let dst = root().join("brain").join(to);
    if !src.exists() {
        return Ok(());
    }
    std::fs::create_dir_all(&dst)?;
    for f in ["base.md", "base.prev.md", "log"] {
        if src.join(f).exists() {
            std::fs::copy(src.join(f), dst.join(f))?;
        }
    }
    log_transition("inherit-brain", from, to, "the new state inherits its father's brain")
}

/// The base brain, frozen in the skeleton.
pub fn brain_base() -> &'static str {
    include_str!("../data/brain-base.md")
}

/// The log entries: (sequence, time, state, checksum, ops).
fn brain_log() -> Vec<(u64, u64, String, u64, Vec<String>)> {
    let t = std::fs::read_to_string(brain_dir().join("log")).unwrap_or_default();
    let mut out = Vec::new();
    let mut cur: Option<(u64, u64, String, u64, Vec<String>)> = None;
    for l in t.lines() {
        if let Some(h) = l.strip_prefix("@@ ") {
            let c: Vec<&str> = h.split(' ').collect();
            cur = (c.len() >= 4).then(|| (c[0].parse().unwrap_or(0), c[1].parse().unwrap_or(0), c[2].to_string(), u64::from_str_radix(c[3], 16).unwrap_or(0), Vec::new()));
        } else if l.starts_with("@@end ") {
            if let Some(e) = cur.take() {
                out.push(e);
            }
        } else if let Some(e) = cur.as_mut() {
            e.4.push(l.to_string());
        }
    }
    out
}

fn base_text() -> String {
    std::fs::read_to_string(brain_dir().join("base.md"))
        .or_else(|_| std::fs::read_to_string(root().join("brain.md")))
        .unwrap_or_else(|_| brain_base().to_string())
}

/// The brain after the first `upto` log entries (all when `None`); stops at a damaged entry.
fn replay(upto: Option<usize>) -> String {
    let mut lines: Vec<String> = base_text().lines().map(String::from).collect();
    for (k, (_, _, _, sum, ops)) in brain_log().into_iter().enumerate() {
        if upto.is_some_and(|u| k >= u) {
            break;
        }
        let next = apply(&lines, &ops);
        if crate::induce::fnv(&next.join("\n")) != sum {
            break;
        }
        lines = next;
    }
    lines.join("\n") + "\n"
}

/// The current brain: the base and the replayed log (the base from the skeleton when nothing was written).
pub fn brain_read() -> String {
    replay(None)
}

/// Write the brain between stages (refused while a stage runs: the scheme is frozen).
pub fn brain_write(text: &str) -> Result<()> {
    if let Some(stage) = frozen() {
        if !stage.ends_with(&format!(" {}", std::process::id())) {
            bail!("the brain is frozen during the training stage «{stage}»");
        }
    }
    brain_write_owned(text)
}

/// Append the change to the log (the running stage itself, or between stages); compact when the log grows.
fn brain_write_owned(text: &str) -> Result<()> {
    if text.len() > BRAIN_SOFT_BYTES {
        eprintln!("brain: {} bytes is over the soft limit of {BRAIN_SOFT_BYTES} — summarise: keep what decides, drop what repeats", text.len());
    }
    std::fs::create_dir_all(brain_dir())?;
    let old = brain_read();
    if old == text || old.trim_end() == text.trim_end() {
        return Ok(());
    }
    let a: Vec<String> = old.lines().map(String::from).collect();
    let b: Vec<String> = text.lines().map(String::from).collect();
    let ops = diff(&a, &b);
    let log = brain_log();
    let seq = log.last().map_or(0, |e| e.0 + 1);
    let t = std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    let entry = format!("@@ {seq} {t} {} {:016x}\n{}\n@@end {seq}\n", current(), crate::induce::fnv(&b.join("\n")), ops.join("\n"));
    let mut f = std::fs::OpenOptions::new().create(true).append(true).open(brain_dir().join("log"))?;
    std::io::Write::write_all(&mut f, entry.as_bytes())?;
    f.sync_data()?;
    let log_bytes = std::fs::metadata(brain_dir().join("log")).map(|m| m.len()).unwrap_or(0);
    if log.len() + 1 >= BRAIN_LOG_MAX || log_bytes as usize > text.len() {
        // compaction: the current text becomes the base; the previous base is kept one level
        if let Ok(prev) = std::fs::read_to_string(brain_dir().join("base.md")) {
            crate::store::write_atomic(&brain_dir().join("base.prev.md"), prev.as_bytes())?;
        }
        crate::store::write_atomic(&brain_dir().join("base.md"), text.as_bytes())?;
        crate::store::write_atomic(&brain_dir().join("log"), b"")?;
    }
    log_transition("brain", &current(), "brain", &format!("{} bytes, +{} −{} lines", text.len(), ops.iter().filter(|o| o.starts_with('+')).count(), ops.iter().filter_map(|o| o.strip_prefix('-')).filter_map(|n| n.parse::<usize>().ok()).sum::<usize>()))
}

/// Points of the log available for recovery (entries since the last compaction).
pub fn brain_versions() -> Vec<String> {
    brain_log().iter().map(|(seq, t, st, _, ops)| format!("entry {seq} at {t} in state {st}: {} changed lines", ops.iter().filter(|o| !o.starts_with('=')).count())).collect()
}

/// The brain after the first `k` log entries — only in the recovery window, to choose what to restore.
pub fn brain_read_version(k: usize) -> Result<String> {
    if !in_recovery() {
        bail!("earlier brain points are for recovery after a crash; constant access is to the latest brain only");
    }
    Ok(replay(Some(k)))
}

/// Restore the brain to the point after the first `k` log entries (recovery after a crash only).
pub fn brain_recover(k: usize) -> Result<()> {
    let text = brain_read_version(k)?;
    brain_write_owned(&text)?;
    log_transition("brain-recover", "brain", &format!("point {k}"), text.lines().next().unwrap_or(""))
}

/// The brain parsed into trees (re-parsed when the text or the parsing rules change).
pub fn brain_parse() -> Result<(Vec<Vec<en::annotate::Word>>, bool)> {
    parse_text_cached(&brain_read())
}

/// The generated sections of the brain: the induced repair rules, the domain knobs, the derivation rules of a stage,
/// the last learning curve — every rule as an English sentence. Other sections (what the SLM wrote for itself) stay.
pub fn brain_sections(derivation: &[String], curve: &[f64]) -> Vec<(String, String)> {
    let mut v = Vec::new();
    // who I am, in sentences my reader understands (identity, class): self-awareness may arise from them
    v.push(("Who I am".to_string(), "I am Mova.\nMova is a model.".to_string()));
    v.push(("Repair rules I apply after parsing".to_string(), crate::induce::shipped("tale").iter().map(|r| format!("- {}", r.show())).collect::<Vec<_>>().join("\n")));
    v.push(("Settings of my domain modules".to_string(), crate::domains::default_cfg().iter().map(|(k, x)| format!("- {k} is {x}.")).collect::<Vec<_>>().join("\n")));
    if !derivation.is_empty() {
        v.push(("Rules of my logical derivation of trees".to_string(), derivation.iter().map(|r| format!("- {r}")).collect::<Vec<_>>().join("\n")));
    }
    if !curve.is_empty() {
        v.push(("My last learning curve".to_string(), format!("Agreement with my parser by round: {}.", curve.iter().map(|x| format!("{x:.2}")).collect::<Vec<_>>().join(", "))));
    }
    v
}

/// The base brain: who I am, then the generated sections.
pub fn brain_describe(derivation: &[String], curve: &[f64]) -> String {
    let who = include_str!("../data/self.md").split("\n\n").nth(1).unwrap_or("").trim().to_string();
    let mut s = format!("# My brain\n\n{who}\n\nThis is the state of my pragmatics, written in plain English by myself.\n");
    for (sec, body) in brain_sections(derivation, curve) {
        s = set_section(&s, &sec, &body);
    }
    s
}

/// The current brain with its generated sections refreshed (the rest kept).
pub fn brain_refreshed(derivation: &[String], curve: &[f64]) -> String {
    let mut s = brain_read();
    for (sec, body) in brain_sections(derivation, curve) {
        s = set_section(&s, &sec, &body);
    }
    s
}

/// A transition to a new state: the brain is serialised into English and saved, the new state is recorded (with its
/// letter), the new rules start, and the brain is deserialised from English under them (parsed again by the new
/// rules). Called after a stage ends, when the scheme is no longer frozen.
pub fn transition(note: &str, letter: Option<&str>, derivation: &[String], curve: &[f64]) -> Result<String> {
    brain_write(&brain_refreshed(derivation, curve))?;
    let id = save_with_letter(note, None, "-", letter)?;
    let (sents, cached) = brain_parse()?;
    log_transition("deserialize", "brain.md", &id, &format!("{} sentences{}", sents.len(), if cached { " (cache)" } else { "" }))?;
    Ok(id)
}

// ---------------------------------------------------------------------------------------------------------------
// Reproducible runs: a request carries a state hash; the SLM unpacks itself in that state (a cached copy, the active
// state untouched) and runs there; without a hash it runs in the last stable state (the original if none). Every
// answer reports the state it was processed in and the last stable one.

pub fn stable() -> String {
    std::fs::read_to_string(states().join("stable")).map(|s| s.trim().to_string()).unwrap_or_else(|_| "origin".into())
}

pub fn mark_stable(id: &str) -> Result<()> {
    std::fs::create_dir_all(states())?;
    crate::store::write_atomic(&states().join("stable"), format!("{id}\n").as_bytes())?;
    log_transition("stable", "-", id, "")
}

/// Unpack state `id` into `<hot>/cache/<id>/` (once) and return the folder; the original needs no folder.
/// Unpacked states kept in the cache; older unpacked copies are removed (their deltas stay on disk, so any old
/// state can be unpacked again on request).
const UNPACKED_KEEP: usize = 8;

fn evict_unpacked() {
    let Ok(rd) = std::fs::read_dir(root().join("cache")) else { return };
    let mut dirs: Vec<(std::time::SystemTime, PathBuf)> = rd
        .flatten()
        .filter_map(|e| {
            let p = e.path();
            let t = std::fs::metadata(p.join(".complete")).and_then(|m| m.modified()).ok()?;
            Some((t, p))
        })
        .collect();
    dirs.sort();
    if dirs.len() > UNPACKED_KEEP {
        for (_, p) in &dirs[..dirs.len() - UNPACKED_KEEP] {
            let _ = std::fs::remove_dir_all(p);
        }
    }
}

pub fn unpack(id: &str) -> Result<Option<PathBuf>> {
    if id == "origin" {
        return Ok(None);
    }
    let dir = root().join("cache").join(id);
    if dir.join(".complete").exists() {
        // touch: recently used copies stay unpacked
        let _ = std::fs::write(dir.join(".complete"), b"");
        return Ok(Some(dir));
    }
    if !dir.join(".complete").exists() {
        let files = materialise(id)?;
        std::fs::create_dir_all(&dir)?;
        for (f, lines) in &files {
            if !lines.is_empty() {
                crate::store::write_atomic(&dir.join(f), format!("{}\n", lines.join("\n")).as_bytes())?;
            }
        }
        std::fs::write(dir.join(".complete"), b"")?;
        evict_unpacked();
    }
    Ok(Some(dir))
}

/// Choose the state of this run (the requested hash, else the last stable one) and point the hot layers at it.
/// Must be called at start-up, before any thread is spawned. Returns (processing state, last stable state).
pub fn activate(requested: Option<&str>) -> Result<(String, String)> {
    let st = stable();
    let id = requested.map(String::from).unwrap_or_else(|| st.clone());
    if let Some(dir) = unpack(&id)? {
        // SAFETY: called once at start-up in main, before any other thread exists
        unsafe {
            std::env::set_var("GLOBAL_HOT", &dir);
            if dir.join("rules-induced.tsv").exists() {
                std::env::set_var("WORLD_RULES", dir.join("rules-induced.tsv"));
            }
            if dir.join("domains.cfg").exists() {
                std::env::set_var("WORLD_DOMAINS_CFG", dir.join("domains.cfg"));
            }
        }
    }
    Ok((id, st))
}

/// LAS of the working pipeline in state `id` on gold (the state's induced rules over the cold layer), cached per
/// state and gold file in `<hot>/cache/las.tsv` — a state is measured once.
pub fn state_las(id: &str, gold: &Path, domain: &str) -> Result<f64> {
    let key = format!("{id}\t{}\t{domain}", gold.display());
    let cache = root().join("cache").join("las.tsv");
    if let Some(v) = std::fs::read_to_string(&cache).unwrap_or_default().lines().find_map(|l| l.rsplit_once('\t').filter(|(k, _)| *k == key).and_then(|(_, v)| v.parse().ok())) {
        return Ok(v);
    }
    let rules = if id == "origin" { None } else { materialise(id)?.get("rules-induced.tsv").filter(|l| !l.is_empty()).cloned() };
    let v = match rules {
        Some(lines) => {
            let tmp = std::env::temp_dir().join(format!("state-las-{}-{id}.tsv", std::process::id()));
            std::fs::write(&tmp, format!("{}\n", lines.join("\n")))?;
            let r = las_with(Some(&tmp), gold, domain);
            let _ = std::fs::remove_file(&tmp);
            r?
        }
        None => las_with(None, gold, domain)?,
    };
    std::fs::create_dir_all(root().join("cache"))?;
    let mut f = std::fs::OpenOptions::new().create(true).append(true).open(&cache)?;
    std::io::Write::write_all(&mut f, format!("{key}\t{v:.4}\n").as_bytes())?;
    Ok(v)
}

/// The parent of a state ("origin" at the root).
pub fn parent_of(id: &str) -> String {
    tree().get(id).map(|x| x.0.clone()).unwrap_or_else(|| "origin".into())
}

/// The most recent state not on the path of `id` (another branch), if any.
pub fn other_branch(id: &str) -> Option<String> {
    let t = tree();
    let mut path = std::collections::BTreeSet::new();
    let mut x = id.to_string();
    while x != "origin" {
        path.insert(x.clone());
        x = t.get(&x).map(|v| v.0.clone()).unwrap_or_else(|| "origin".into());
    }
    t.iter().filter(|(k, _)| !path.contains(*k) && *k != id).max_by_key(|(_, v)| v.1).map(|(k, _)| k.clone())
}

pub fn current_id() -> String {
    current()
}

/// The induced rules of a state (the cold layer for the original), without the scope column (general/domain), so
/// the same rule compares equal across layers: "from\tto\tatoms".
/// The induced rules of a state with their scope column ("domain" — tales only, "general" — every domain).
pub fn state_rules_scoped(id: &str) -> Vec<String> {
    let lines: Vec<String> = if id == "origin" {
        include_str!("../data/rules-induced.tsv").lines().map(String::from).collect()
    } else {
        materialise(id).ok().and_then(|m| m.get("rules-induced.tsv").cloned()).unwrap_or_default()
    };
    lines.into_iter().filter(|l| !l.starts_with('#') && !l.trim().is_empty()).collect()
}

pub fn state_rules(id: &str) -> Vec<String> {
    let lines: Vec<String> = if id == "origin" {
        include_str!("../data/rules-induced.tsv").lines().map(String::from).collect()
    } else {
        materialise(id).ok().and_then(|m| m.get("rules-induced.tsv").cloned()).unwrap_or_default()
    };
    lines.into_iter().filter(|l| !l.starts_with('#') && !l.trim().is_empty()).map(|l| l.splitn(2, '\t').nth(1).unwrap_or("").to_string()).collect()
}
