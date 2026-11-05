//! Durable, audit-only cycle controller. One reserved job is never submitted twice.
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{HashMap, HashSet},
    ffi::{CString, OsString},
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Write},
    os::fd::AsRawFd,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

type R<T> = Result<T, Box<dyn std::error::Error>>;
#[derive(Deserialize)]
struct Config {
    source: PathBuf,
    source_sha256: String,
    queue: PathBuf,
    queue_sha256: String,
    heldout: PathBuf,
    root: PathBuf,
    cycle: PathBuf,
    codex: PathBuf,
    /// Codex teacher model; Terra is the recommended Fish default, Sol selectable.
    #[serde(default = "default_model")]
    model: String,
    env: HashMap<String, String>,
}
fn default_model() -> String {
    "gpt-6.1-terra".into()
}
#[derive(Deserialize)]
struct Queue {
    id: String,
    state: String,
    tree_sha256: String,
}
#[derive(Clone)]
struct Tree {
    id: String,
    text: String,
    block: String,
    doc: String,
}
#[derive(Serialize)]
struct Receipt {
    job: usize,
    ids: Vec<String>,
    outcome: String,
    source_sha256: String,
    queue_sha256: String,
    details: Value,
}
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
fn atomic(path: &Path, bytes: &[u8]) -> R<()> {
    let tmp = path.with_extension("tmp");
    let mut f = File::create(&tmp)?;
    f.write_all(bytes)?;
    f.sync_all()?;
    fs::rename(tmp, path)?;
    File::open(path.parent().unwrap())?.sync_all()?;
    Ok(())
}
fn write_json(path: &Path, v: &impl Serialize) -> R<()> {
    atomic(path, &serde_json::to_vec_pretty(v)?)
}
fn status(c: &Config, state: &str, detail: &str, job: Option<usize>) -> R<()> {
    write_json(
        &c.root.join("status.json"),
        &json!({"state":state,"detail":detail,"job":job,"heartbeat_unix":now(),"audit_only":true}),
    )
}
fn parse_trees(bytes: &[u8]) -> R<Vec<Tree>> {
    let s = std::str::from_utf8(bytes)?;
    let mut out = Vec::new();
    let mut block = String::new();
    for line in s.split_inclusive('\n') {
        block.push_str(line);
        if line.trim().is_empty() {
            push_tree(&mut out, &block)?;
            block.clear();
        }
    }
    if !block.is_empty() {
        push_tree(&mut out, &block)?;
    }
    Ok(out)
}
fn push_tree(out: &mut Vec<Tree>, b: &str) -> R<()> {
    if b.trim().is_empty() {
        return Ok(());
    }
    let id = b
        .lines()
        .find_map(|l| l.strip_prefix("# sent_id = "))
        .ok_or("missing sent_id")?
        .to_string();
    let text = b
        .lines()
        .find_map(|l| l.strip_prefix("# text = "))
        .ok_or("missing text")?
        .to_string();
    if id.contains(['\t', '\n', '\r']) || text.contains(['\t', '\n', '\r']) {
        return Err("invalid pool fields".into());
    }
    let doc = id.rsplit_once(':').map(|x| x.0).unwrap_or(&id).to_string();
    out.push(Tree {
        id,
        text,
        block: b.to_string(),
        doc,
    });
    Ok(())
}
fn read_snapshot(c: &Config) -> R<(Vec<Tree>, Vec<Queue>, HashSet<String>)> {
    let source = fs::read(&c.source)?;
    let queue = fs::read(&c.queue)?;
    if hash(&source) != c.source_sha256 || hash(&queue) != c.queue_sha256 {
        return Err("immutable source or queue hash mismatch".into());
    }
    let trees = parse_trees(&source)?;
    let held: HashSet<_> = parse_trees(&fs::read(&c.heldout)?)?
        .into_iter()
        .map(|t| t.id)
        .collect();
    let mut seen = HashSet::new();
    for t in &trees {
        if !seen.insert(t.id.clone()) {
            return Err("duplicate source id".into());
        }
    }
    let mut qs = Vec::new();
    let mut qseen = HashSet::new();
    for l in BufReader::new(queue.as_slice()).lines() {
        let q: Queue = serde_json::from_str(&l?)?;
        if !qseen.insert(q.id.clone()) {
            return Err("duplicate queue id".into());
        }
        qs.push(q);
    }
    Ok((trees, qs, held))
}
fn disk_ok(root: &Path) -> R<bool> {
    let p = CString::new(root.as_os_str().as_encoded_bytes())?;
    let mut v: libc::statvfs = unsafe { std::mem::zeroed() };
    if unsafe { libc::statvfs(p.as_ptr(), &mut v) } != 0 {
        return Err(std::io::Error::last_os_error().into());
    }
    fn size(p: &Path) -> R<u64> {
        let mut n = 0;
        for e in fs::read_dir(p)? {
            let e = e?;
            let m = e.file_type()?;
            if m.is_dir() {
                n += size(&e.path())?
            } else if m.is_file() {
                n += e.metadata()?.len()
            }
        }
        Ok(n)
    }
    Ok(
        (v.f_bavail as u64).saturating_mul(v.f_frsize as u64) >= 10_000_000_000
            && size(root)? < 50_000_000_000,
    )
}
fn config(args: &[OsString]) -> R<Config> {
    let p = args
        .get(1)
        .ok_or("usage: run CONFIG [--once] | status CONFIG")?;
    let c: Config = serde_json::from_slice(&fs::read(p)?)?;
    teacher::validate_model(&c.model).map_err(|e| format!("{e}; no provider fallback"))?;
    Ok(c)
}
fn receipt(c: &Config, r: &Receipt) -> R<()> {
    let p = c.root.join("ledger").join(format!("{:08}.json", r.job));
    write_json(&p, r)
}
fn scope_exclusion(id: &str) -> Option<&'static str> {
    if id == "question:books:2038-pg902-part8:e1" {
        Some("ambiguous_phase_pending")
    } else if id.starts_with("math:svamp-") {
        Some("svamp_scope_pending")
    } else {
        None
    }
}
fn tokens(t: &Tree) -> Vec<Vec<&str>> {
    t.block
        .lines()
        .filter(|l| !l.starts_with('#'))
        .filter_map(|l| {
            let f: Vec<_> = l.split('\t').collect();
            if f.len() == 10 && f[0].parse::<u64>().is_ok() {
                Some(f)
            } else {
                None
            }
        })
        .collect()
}
fn comparison(old: &Tree, new: &Tree) -> Value {
    let a = tokens(old);
    let b = tokens(new);
    let aligned = !a.is_empty()
        && a.len() == b.len()
        && a.iter().zip(&b).all(|(x, y)| x[0] == y[0] && x[1] == y[1]);
    if !aligned {
        return json!({"id":old.id,"tokenization_aligned":false,"uas":null,"las":null});
    }
    let n = a.len();
    let heads = a.iter().zip(&b).filter(|(x, y)| x[6] == y[6]).count();
    let labels = a
        .iter()
        .zip(&b)
        .filter(|(x, y)| x[6] == y[6] && x[7] == y[7])
        .count();
    json!({"id":old.id,"tokenization_aligned":true,"tokens":n,"matching_heads":heads,"matching_labeled_heads":labels,"uas":heads as f64/n as f64,"las":labels as f64/n as f64})
}
fn prag_valid(f: &[&str]) -> bool {
    f.len() == 8
        && ["decl", "q", "wh", "imp", "excl", "frag", "intj", "other"].contains(&f[1])
        && [
            "assert",
            "narrate",
            "ask",
            "request",
            "offer",
            "suggest",
            "promise",
            "thank",
            "apologize",
            "greet",
            "agree",
            "disagree",
            "evaluate",
            "express",
            "warn",
            "other",
        ]
        .contains(&f[2])
        && ["0", "1"].contains(&f[3])
        && ["0", "1"].contains(&f[4])
        && ["pos", "neg", "none"].contains(&f[5])
        && ["narrator", "character", "none"].contains(&f[6])
        && !f[7].is_empty()
}
fn output_check(dir: &Path, selected: &[Tree]) -> R<Value> {
    let final_path = dir.join("round1/final.conllu");
    let got = parse_trees(&fs::read(final_path)?)?;
    let expected: HashMap<_, _> = selected
        .iter()
        .map(|t| (t.id.as_str(), t.text.as_str()))
        .collect();
    let mut seen = HashSet::new();
    for t in &got {
        if !seen.insert(t.id.clone()) || expected.get(t.id.as_str()) != Some(&t.text.as_str()) {
            return Err(format!(
                "unexpected, duplicate, or text-mismatched final id {}",
                t.id
            )
            .into());
        }
    }
    let prag = fs::read_to_string(dir.join("round1/prag-final.tsv"))?;
    let mut lines = prag.lines();
    if lines.next() != Some("sent_id\tform\tact\tindirect\thedge\tpolarity\tvoice\tteacher") {
        return Err("invalid pragmatic header".into());
    }
    let mut pids = HashSet::new();
    let mut invalid_prag = Vec::new();
    for l in lines.filter(|l| !l.trim().is_empty()) {
        let fields: Vec<_> = l.split('\t').collect();
        let id = fields[0];
        if !expected.contains_key(id) || !pids.insert(id.to_owned()) {
            return Err(format!("unexpected or duplicate pragmatic id {id}").into());
        }
        if !prag_valid(&fields) {
            invalid_prag.push(id.to_owned());
        }
    }
    let accepted: Vec<_> = selected
        .iter()
        .filter(|t| seen.contains(&t.id) && pids.contains(&t.id) && !invalid_prag.contains(&t.id))
        .map(|t| t.id.clone())
        .collect();
    let rejected: Vec<_> = selected
        .iter()
        .filter(|t| seen.contains(&t.id) && !pids.contains(&t.id))
        .map(|t| t.id.clone())
        .collect();
    let missing: Vec<_> = selected
        .iter()
        .filter(|t| !seen.contains(&t.id))
        .map(|t| t.id.clone())
        .collect();
    let comparisons: Vec<_> = got
        .iter()
        .filter_map(|t| {
            selected
                .iter()
                .find(|o| o.id == t.id)
                .map(|o| comparison(o, t))
        })
        .collect();
    Ok(
        json!({"scope":"fresh reannotation with source comparison", "accepted":accepted,"rejected_missing_prag":rejected,"rejected_invalid_prag":invalid_prag,"missing_final":missing,"tree_comparisons":comparisons,"semantic_correctness":"not established"}),
    )
}
pub fn dispatch(args: &[OsString]) -> R<()> {
    let c = config(args)?;
    fs::create_dir_all(&c.root)?;
    fs::create_dir_all(c.root.join("ledger"))?;
    fs::create_dir_all(c.root.join("jobs"))?;
    if args[0] == "status" {
        println!("{}", fs::read_to_string(c.root.join("status.json"))?);
        return Ok(());
    }
    if args[0] != "run" {
        return Err("use run CONFIG [--once] or status CONFIG".into());
    }
    let lock = OpenOptions::new()
        .create(true)
        .write(true)
        .open(c.root.join("controller.lock"))?;
    if unsafe { libc::flock(lock.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } != 0 {
        return Err("another controller holds the lock".into());
    }
    status(&c, "starting", "verifying immutable inputs", None)?;
    let identity = json!({"config_sha256":hash(&fs::read(args.get(1).unwrap())?),"source_sha256":c.source_sha256,"queue_sha256":c.queue_sha256,"heldout_sha256":hash(&fs::read(&c.heldout)?)});
    let identity_path = c.root.join("campaign.json");
    if identity_path.exists() {
        let previous: Value = serde_json::from_slice(&fs::read(&identity_path)?)?;
        if previous != identity {
            status(&c, "paused", "campaign identity mismatch", None)?;
            return Ok(());
        }
    } else if fs::read_dir(c.root.join("ledger"))?.next().is_some()
        || fs::read_dir(c.root.join("jobs"))?.next().is_some()
    {
        status(&c, "paused", "existing work has no campaign identity", None)?;
        return Ok(());
    } else {
        write_json(&identity_path, &identity)?;
    }
    let (trees, queue, held) = match read_snapshot(&c) {
        Ok(v) => v,
        Err(e) => {
            status(&c, "paused", &e.to_string(), None)?;
            return Err(e);
        }
    };
    let index: HashMap<_, _> = trees
        .iter()
        .enumerate()
        .map(|(i, t)| (t.id.as_str(), i))
        .collect();
    let mut documents: HashMap<&str, Vec<(u64, usize)>> = HashMap::new();
    for (i, t) in trees.iter().enumerate() {
        if let Some(n) = t.id.rsplit(':').next().and_then(|n| n.parse::<u64>().ok()) {
            documents.entry(&t.doc).or_default().push((n, i));
        }
    }
    for v in documents.values_mut() {
        v.sort_unstable();
    }
    let once = args.iter().any(|a| a == "--once");
    let mut pos = 0;
    let mut job = 0;
    'campaign: loop {
        if c.root.join("STOP").exists() {
            status(&c, "stopped", "STOP file present", None)?;
            return Ok(());
        }
        if c.root.join("PAUSE").exists() {
            status(&c, "paused", "PAUSE file present", None)?;
            if once {
                return Ok(());
            }
            thread::sleep(Duration::from_secs(10));
            continue;
        }
        if !disk_ok(&c.root)? {
            status(
                &c,
                "paused",
                "disk floor or campaign size limit reached",
                None,
            )?;
            return Ok(());
        }
        if pos >= queue.len() {
            status(
                &c,
                "awaiting_new_snapshot",
                "immutable queue exhausted",
                None,
            )?;
            if once {
                return Ok(());
            }
            thread::sleep(Duration::from_secs(10));
            continue;
        }
        let end = (pos + 8).min(queue.len());
        let slice = &queue[pos..end];
        let ids: Vec<_> = slice.iter().map(|q| q.id.clone()).collect();
        let ledger = c.root.join("ledger").join(format!("{job:08}.json"));
        if ledger.exists() {
            let prior: Value = serde_json::from_slice(&fs::read(&ledger)?)?;
            if prior["job"] != job || prior["ids"] != json!(ids) {
                status(
                    &c,
                    "paused",
                    "ledger does not match immutable queue",
                    Some(job),
                )?;
                return Ok(());
            }
            pos = end;
            job += 1;
            continue;
        }
        let j = c.root.join("jobs").join(format!("{job:08}"));
        let reservation = j.join("reserved.json");
        if reservation.exists() {
            status(
                &c,
                "paused_uncertain",
                "reserved job has no durable receipt; inspect job before manual resolution",
                Some(job),
            )?;
            return Ok(());
        }
        let mut selected = Vec::new();
        let mut excluded = Vec::new();
        for q in slice {
            if q.state == "excluded_heldout" && held.contains(&q.id) {
                excluded.push(json!({"id":q.id,"reason":"heldout"}));
                continue;
            }
            if let Some(reason) = scope_exclusion(&q.id) {
                excluded.push(json!({"id":q.id,"reason":reason}));
                continue;
            }
            match index.get(q.id.as_str()).map(|i|&trees[*i]) {
                Some(t) if q.state=="pending_annotation_review" && !held.contains(&q.id) && hash(t.block.as_bytes())==q.tree_sha256 => selected.push(t.clone()),
                _ => excluded.push(json!({"id":q.id,"reason":"invalid_state_missing_source_heldout_or_hash_mismatch"})),
            }
        }
        if selected.is_empty() {
            receipt(
                &c,
                &Receipt {
                    job,
                    ids,
                    outcome: "excluded_or_quarantined".into(),
                    source_sha256: c.source_sha256.clone(),
                    queue_sha256: c.queue_sha256.clone(),
                    details: json!({"excluded":excluded}),
                },
            )?;
            pos = end;
            job += 1;
            if once {
                return Ok(());
            }
            continue;
        }
        fs::create_dir_all(j.join("cycle"))?;
        fs::create_dir_all(j.join("db"))?;
        fs::create_dir_all(j.join("models"))?;
        let selected_ids: HashSet<_> = selected.iter().map(|t| t.id.as_str()).collect();
        let mut prefix = String::new();
        let mut included = HashSet::new();
        for t in &selected {
            if t.id.starts_with("question:") {
                continue;
            }
            let Some(n) = t.id.rsplit(':').next().and_then(|n| n.parse::<u64>().ok()) else {
                continue;
            };
            let Some(doc) = documents.get(t.doc.as_str()) else {
                continue;
            };
            let prev: Vec<_> = doc
                .iter()
                .rev()
                .filter(|(k, i)| {
                    *k < n
                        && !held.contains(&trees[*i].id)
                        && scope_exclusion(&trees[*i].id).is_none()
                        && !selected_ids.contains(trees[*i].id.as_str())
                })
                .take(32)
                .collect();
            for (_, i) in prev.into_iter().rev() {
                let p = &trees[*i];
                if included.insert(p.id.clone()) {
                    prefix.push_str(&p.block);
                }
            }
        }
        fs::write(j.join("db/current.conllu"), &prefix)?;
        fs::write(
            j.join("original.conllu"),
            selected
                .iter()
                .map(|t| t.block.as_str())
                .collect::<String>(),
        )?;
        fs::write(
            j.join("pool.tsv"),
            selected
                .iter()
                .map(|t| format!("{}\t{}\n", t.id, t.text))
                .collect::<String>(),
        )?;
        for p in ["corpus.conllu", "agreed.conllu"] {
            fs::write(j.join(p), "")?;
        }
        write_json(
            &reservation,
            &json!({"job":job,"queue_start":pos,"ids":ids,"created_unix":now()}),
        )?;
        let log = File::create(j.join("cycle.log"))?;
        let mut cmd = Command::new(&c.cycle);
        cmd.args(["pipeline", "1", "1"])
            .stdout(Stdio::from(log.try_clone()?))
            .stderr(Stdio::from(log));
        for (k, _) in std::env::vars_os() {
            if k.to_string_lossy().starts_with("CYCLE_")
                || k.to_string_lossy().starts_with("ANNOT_")
                || k.to_string_lossy().starts_with("MOVA_TEACHER_")
                || k.to_string_lossy().starts_with("ENRICH_")
            {
                cmd.env_remove(k);
            }
        }
        for (k, v) in &c.env {
            if !k.starts_with("CYCLE_") && !k.starts_with("ANNOT_") {
                return Err(format!("unsupported config env {k}").into());
            }
            cmd.env(k, v);
        }
        cmd.env("CYCLE_ROOT", j.join("cycle"))
            .env("CYCLE_DB", j.join("db"))
            .env("CYCLE_POOL", j.join("pool.tsv"))
            .env("CYCLE_CORPUS", j.join("corpus.conllu"))
            .env("CYCLE_AGREED", j.join("agreed.conllu"))
            .env("CYCLE_MODELS", j.join("models"))
            .env("CYCLE_TEST", &c.heldout)
            .env("CYCLE_N", "8")
            .env("CYCLE_INFLIGHT", "1")
            .env("CYCLE_REPEAT", "1")
            .env("CYCLE_TEACHER", "codex")
            .env("CYCLE_MODEL", &c.model)
            .env("CYCLE_TIERS", "0")
            .env("CYCLE_FINETUNE", "0")
            .env("CYCLE_ENRICH", "0")
            .env("CYCLE_BASE", "")
            .env("MOVA_TEACHER_MODEL", &c.model)
            .env("MOVA_TEACHER_TIMEOUT_SECS", "1800")
            .env("ANNOT_EFFORT", "medium")
            .env("CARGO_NET_OFFLINE", "true")
            .env("HF_HUB_OFFLINE", "1")
            .env("TRANSFORMERS_OFFLINE", "1")
            .env("MOVA_CODEX_BIN", std::env::current_exe()?)
            .env("MOVA_REVIEW_CODEX_REAL", &c.codex);
        status(&c, "running", "cycle launched", Some(job))?;
        let mut child = match cmd.spawn() {
            Ok(x) => x,
            Err(e) => {
                status(
                    &c,
                    "paused_uncertain",
                    &format!("cycle launch failed: {e}"),
                    Some(job),
                )?;
                return Ok(());
            }
        };
        let started = std::time::Instant::now();
        let result = loop {
            if let Ok(error) = fs::read_to_string(j.join("cycle/round1/codex-error.txt")) {
                let _ = child.kill();
                let _ = child.wait();
                if error.contains("prompt exceeds 25000 UTF-8 bytes") {
                    receipt(
                        &c,
                        &Receipt {
                            job,
                            ids: ids.clone(),
                            outcome: "quarantined".into(),
                            source_sha256: c.source_sha256.clone(),
                            queue_sha256: c.queue_sha256.clone(),
                            details: json!({"teacher_error":error,"excluded":excluded}),
                        },
                    )?;
                    pos = end;
                    job += 1;
                    if once {
                        status(&c, "idle", "oversized prompt quarantined", Some(job - 1))?;
                        return Ok(());
                    }
                    continue 'campaign;
                }
                status(
                    &c,
                    "paused_uncertain",
                    &format!("teacher stopped: {error}"),
                    Some(job),
                )?;
                return Ok(());
            }
            if let Some(s) = child.try_wait()? {
                break s;
            }
            if started.elapsed() >= Duration::from_secs(7200) {
                let _ = child.kill();
                let _ = child.wait();
                status(&c,"paused_uncertain","cycle exceeded two hours; detached worker may remain; inspect before manual resolution",Some(job))?;
                return Ok(());
            }
            status(
                &c,
                "running",
                if c.root.join("STOP").exists() || c.root.join("PAUSE").exists() {
                    "stop requested; waiting for active cycle"
                } else {
                    "cycle active"
                },
                Some(job),
            )?;
            thread::sleep(Duration::from_secs(10));
        };
        if !result.success() {
            if let Ok(error) = fs::read_to_string(j.join("cycle/round1/codex-error.txt")) {
                if error.contains("prompt exceeds 25000 UTF-8 bytes") {
                    receipt(
                        &c,
                        &Receipt {
                            job,
                            ids: ids.clone(),
                            outcome: "quarantined".into(),
                            source_sha256: c.source_sha256.clone(),
                            queue_sha256: c.queue_sha256.clone(),
                            details: json!({"teacher_error":error,"excluded":excluded}),
                        },
                    )?;
                    pos = end;
                    job += 1;
                    if once {
                        status(&c, "idle", "oversized prompt quarantined", Some(job - 1))?;
                        return Ok(());
                    }
                    continue 'campaign;
                }
                status(
                    &c,
                    "paused_uncertain",
                    &format!("teacher stopped: {error}"),
                    Some(job),
                )?;
                return Ok(());
            }
            status(
                &c,
                "paused_uncertain",
                &format!("cycle exit {result}; inspect job logs and detached worker"),
                Some(job),
            )?;
            return Ok(());
        }
        let mut details = match output_check(&j.join("cycle"), &selected) {
            Ok(d) => d,
            Err(e) => json!({"validation_error":e.to_string()}),
        };
        details["excluded"] = json!(excluded);
        details["teacher_calls"] =
            match fs::read_to_string(j.join("cycle/round1/codex-teacher/codex-calls.jsonl")) {
                Ok(v) => json!(v
                    .lines()
                    .filter_map(|l| serde_json::from_str::<Value>(l).ok())
                    .collect::<Vec<_>>()),
                Err(_) => json!([]),
            };
        let outcome = if details.get("validation_error").is_some() {
            "quarantined"
        } else {
            "checked"
        };
        receipt(
            &c,
            &Receipt {
                job,
                ids,
                outcome: outcome.into(),
                source_sha256: c.source_sha256.clone(),
                queue_sha256: c.queue_sha256.clone(),
                details,
            },
        )?;
        pos = end;
        job += 1;
        if once {
            status(&c, "idle", "one job completed", Some(job - 1))?;
            return Ok(());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn config_model_defaults_to_terra_and_refuses_claude_ids() {
        let root = std::env::temp_dir().join(format!("fish-controller-model-{}", std::process::id()));
        fs::create_dir_all(&root).unwrap();
        let path = root.join("config.json");
        let mut v = json!({"source":"/s","source_sha256":"0","queue":"/q","queue_sha256":"0",
            "heldout":"/h","root":"/r","cycle":"/c","codex":"/x","env":{}});
        let load = |v: &Value| {
            fs::write(&path, serde_json::to_vec(v).unwrap()).unwrap();
            config(&["run".into(), path.clone().into_os_string()])
        };
        assert_eq!(load(&v).unwrap().model, "gpt-6.1-terra");
        v["model"] = json!("gpt-6.1-sol");
        assert_eq!(load(&v).unwrap().model, "gpt-6.1-sol");
        for bad in ["sonnet", "opus", "claude-sonnet-5", "gpt- x"] {
            v["model"] = json!(bad);
            assert!(load(&v).is_err(), "{bad}");
        }
        let _ = fs::remove_dir_all(root);
    }
}
