//! Disjoint immutable annotation shards and fail-closed child supervision.
use crate::Result;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Write},
    os::{
        fd::AsRawFd,
        unix::{fs::symlink, process::CommandExt},
    },
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};

fn err(s: impl Into<String>) -> Box<dyn std::error::Error> {
    s.into().into()
}
fn field<'a>(v: &'a Value, k: &str) -> Result<&'a str> {
    v[k].as_str().ok_or_else(|| err(format!("missing {k}")))
}
fn path(v: &Value, k: &str) -> Result<PathBuf> {
    let p = PathBuf::from(field(v, k)?);
    if !p.is_absolute() {
        return Err(err(format!("{k} must be absolute")));
    }
    Ok(p)
}
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn load(p: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&fs::read(p)?)?)
}
fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
fn atomic(p: &Path, b: &[u8]) -> Result<()> {
    let tmp = p.with_extension("tmp");
    let mut f = File::create(&tmp)?;
    f.write_all(b)?;
    f.sync_all()?;
    fs::rename(tmp, p)?;
    File::open(p.parent().ok_or("no parent")?)?.sync_all()?;
    Ok(())
}
fn save(p: &Path, v: &Value) -> Result<()> {
    atomic(p, &serde_json::to_vec_pretty(v)?)
}
fn exclusive(p: &Path) -> Result<File> {
    let f = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .open(p)?;
    if unsafe { libc::flock(f.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } != 0 {
        return Err(err(format!("lock held: {}", p.display())));
    }
    Ok(f)
}
// Preparation leases are never intentionally inherited. Explicit unlock prevents
// a concurrent fork's transient duplicate from extending their scope.
struct PreparationLease(File);
impl Drop for PreparationLease {
    fn drop(&mut self) {
        unsafe {
            libc::flock(self.0.as_raw_fd(), libc::LOCK_UN);
        }
    }
}
fn annotation_root(c: &Value) -> Result<PathBuf> {
    if c.get("annotation_root").is_some() {
        path(c, "annotation_root")
    } else {
        Ok(path(c, "run_root")?.join("annotation"))
    }
}
fn idset(v: &Value) -> Result<BTreeSet<String>> {
    v.as_array()
        .ok_or("expected ID array")?
        .iter()
        .map(|x| {
            x.as_str()
                .map(String::from)
                .ok_or_else(|| err("invalid ID"))
        })
        .collect()
}
fn files(dir: &Path) -> Result<Vec<PathBuf>> {
    let mut v = Vec::new();
    for e in fs::read_dir(dir)? {
        v.push(e?.path())
    }
    v.sort();
    Ok(v)
}

// The file lock is held through preparation, so a stopped legacy controller cannot race us.
fn legacy_quiescent(root: &Path) -> Result<(File, BTreeSet<String>, Value)> {
    if !root.join("STOP").is_file() {
        return Err(err(
            "legacy STOP is required; let its active cycle finish first",
        ));
    }
    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .open(root.join("controller.lock"))?;
    if unsafe { libc::flock(f.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } != 0 {
        return Err(err("legacy controller still active"));
    }
    if load(&root.join("status.json"))?["state"] != "stopped" {
        return Err(err(
            "legacy status is not stopped; inspect uncertainty before migration",
        ));
    }
    let mut covered = BTreeSet::new();
    let mut receipts = BTreeMap::new();
    let mut classifications = BTreeMap::new();
    for p in files(&root.join("ledger"))? {
        if p.extension().and_then(|x| x.to_str()) != Some("json") {
            continue;
        }
        let r = load(&p)?;
        let job = r["job"].as_u64().ok_or("bad receipt job")?;
        if p.file_name().and_then(|x| x.to_str()) != Some(format!("{job:08}.json").as_str()) {
            return Err(err("legacy receipt filename mismatch"));
        }
        let ids = idset(&r["ids"])?;
        let mut lists = BTreeMap::new();
        for key in [
            "accepted",
            "missing_final",
            "rejected_missing_prag",
            "rejected_invalid_prag",
        ] {
            let values = if r["details"][key].is_null() {
                BTreeSet::new()
            } else {
                idset(&r["details"][key])?
            };
            if !values.is_subset(&ids) {
                return Err(err("legacy outcome ID outside receipt"));
            }
            lists.insert(key, values);
        }
        for id in &ids {
            let class = if lists["accepted"].contains(id) {
                "accepted"
            } else if r["outcome"] == "quarantined" || lists["rejected_invalid_prag"].contains(id) {
                "quarantined"
            } else if lists["missing_final"].contains(id)
                || lists["rejected_missing_prag"].contains(id)
            {
                "missing_or_rejected"
            } else {
                "other_excluded"
            };
            if classifications.insert(id.clone(), class).is_some() {
                return Err(err("duplicate legacy coverage ID"));
            }
        }
        covered.extend(ids.clone());
        if receipts.insert(job, ids).is_some() {
            return Err(err("duplicate legacy receipt"));
        }
    }
    for j in files(&root.join("jobs"))? {
        if !j.is_dir() {
            continue;
        }
        let rp = j.join("reserved.json");
        if !rp.exists() {
            continue;
        }
        let r = load(&rp)?;
        let job = r["job"].as_u64().ok_or("bad reservation job")?;
        let ids = idset(&r["ids"])?;
        if receipts.get(&job) != Some(&ids) {
            return Err(err(format!(
                "uncertain legacy reservation {job}; never automatically resubmit"
            )));
        }
        covered.extend(ids);
    }
    let mut counts = BTreeMap::from([
        ("accepted", 0usize),
        ("quarantined", 0),
        ("missing_or_rejected", 0),
        ("other_excluded", 0),
    ]);
    for class in classifications.values() {
        *counts.get_mut(class).expect("known class") += 1;
    }
    Ok((
        f,
        covered,
        json!({"receipt_classes":counts,"automatic_requeue":false,"classification":"per-ID receipt outcome; annotation correctness is unproved"}),
    ))
}
fn coverage_hash(ids: &BTreeSet<String>, outcomes: &Value) -> Result<String> {
    Ok(hash(&serde_json::to_vec(
        &json!({"ids":ids,"outcomes":outcomes}),
    )?))
}
fn inherit_lease(cmd: &mut Command, f: &File) {
    let fd = f.as_raw_fd();
    // Change descriptor flags only in the intended child after fork: changing
    // them in the parent leaks the lease into unrelated concurrent execs.
    unsafe {
        cmd.pre_exec(move || {
            let flags = libc::fcntl(fd, libc::F_GETFD);
            if flags < 0 || libc::fcntl(fd, libc::F_SETFD, flags & !libc::FD_CLOEXEC) < 0 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
}
fn held_metadata(p: &Path) -> Result<(BTreeSet<String>, BTreeSet<String>, String)> {
    let mut ids = BTreeSet::new();
    let mut texts = BTreeSet::new();
    let mut h = Sha256::new();
    for l in BufReader::new(File::open(p)?).split(b'\n') {
        let l = l?;
        h.update(&l);
        h.update(b"\n");
        let s = std::str::from_utf8(&l)?;
        if let Some(id) = s.strip_prefix("# sent_id = ") {
            ids.insert(id.into());
        }
        if let Some(t) = s.strip_prefix("# text = ") {
            texts.insert(hash(t.as_bytes()));
        }
    }
    // Exact hash includes whether the final newline exists; metadata iteration above is not used for identity.
    Ok((ids, texts, hash(&fs::read(p)?)))
}
fn source_exclusions(p: &Path, held: &BTreeSet<String>) -> Result<(BTreeSet<String>, String)> {
    let mut h = Sha256::new();
    let mut ids = Vec::new();
    let mut current = None;
    let mut blocked = BTreeSet::new();
    let mut line = Vec::new();
    let mut r = BufReader::new(File::open(p)?);
    loop {
        line.clear();
        if r.read_until(b'\n', &mut line)? == 0 {
            break;
        }
        h.update(&line);
        let l = std::str::from_utf8(&line)?.trim_end_matches(['\r', '\n']);
        if l.is_empty() {
            if ids.len() > 1 {
                blocked.extend(ids.drain(..));
            } else {
                ids.clear();
            }
            current = None;
        } else if let Some(id) = l.strip_prefix("# sent_id = ") {
            ids.push(id.to_string());
            current = Some(id.to_string());
        } else if let Some(t) = l.strip_prefix("# text = ") {
            if held.contains(&hash(t.as_bytes())) {
                if let Some(id) = &current {
                    blocked.insert(id.clone());
                }
            }
        }
    }
    if ids.len() > 1 {
        blocked.extend(ids);
    }
    Ok((blocked, format!("{:x}", h.finalize())))
}

fn context_union(
    held: &Path,
    source: &Path,
    blocked: &BTreeSet<String>,
    held_ids: &BTreeSet<String>,
    held_sha: &str,
    source_sha: &str,
) -> Result<(Vec<u8>, usize)> {
    let mut bytes = fs::read(held)?;
    if hash(&bytes) != held_sha {
        return Err(err("heldout changed while deriving context exclusion"));
    }
    let mut metadata = BTreeMap::new();
    let mut current = None;
    let mut line = Vec::new();
    let mut r = BufReader::new(File::open(source)?);
    let mut h = Sha256::new();
    loop {
        line.clear();
        if r.read_until(b'\n', &mut line)? == 0 {
            break;
        }
        h.update(&line);
        let s = std::str::from_utf8(&line)?.trim_end_matches(['\r', '\n']);
        if let Some(id) = s.strip_prefix("# sent_id = ") {
            current = Some(id.to_string());
        } else if let Some(text) = s.strip_prefix("# text = ") {
            if let Some(id) = &current {
                if blocked.contains(id) && !held_ids.contains(id) {
                    if let Some(previous) = metadata.insert(id.clone(), text.to_string()) {
                        if previous != text {
                            return Err(err("conflicting context-exclusion text"));
                        }
                    }
                }
            }
        }
    }
    if format!("{:x}", h.finalize()) != source_sha {
        return Err(err("source changed while deriving context exclusion"));
    }
    for id in blocked.difference(held_ids) {
        if !metadata.contains_key(id) {
            return Err(err("context-exclusion ID lacks source text"));
        }
    }
    if !bytes.ends_with(b"\n") {
        bytes.push(b'\n');
    }
    if !bytes.ends_with(b"\n\n") {
        bytes.push(b'\n');
    }
    for (id, text) in &metadata {
        bytes.extend_from_slice(format!("# sent_id = {id}\n# text = {text}\n\n").as_bytes());
    }
    Ok((bytes, metadata.len()))
}

/// Prepare a new campaign family; never edit or replay a legacy reservation.
pub fn prepare(c: &Value) -> Result<Value> {
    let root = annotation_root(c)?;
    let parent = root.parent().ok_or("no annotation parent")?;
    fs::create_dir_all(parent)?;
    let _prepare = PreparationLease(exclusive(&parent.join("peta-prepare.lock"))?);
    let legacy = path(c, "legacy_campaign")?;
    let (legacy_guard, covered, legacy_outcomes) = legacy_quiescent(&legacy)?;
    let _legacy = PreparationLease(legacy_guard);
    let legacy_coverage_sha = coverage_hash(&covered, &legacy_outcomes)?;
    let config_path = path(c, "legacy_config")?;
    let old_bytes = fs::read(&config_path)?;
    let old: Value = serde_json::from_slice(&old_bytes)?;
    if path(&old, "root")? != legacy {
        return Err(err("legacy config/root mismatch"));
    }
    let campaign = load(&legacy.join("campaign.json"))?;
    if campaign["config_sha256"] != hash(&old_bytes) {
        return Err(err("legacy config identity mismatch"));
    }
    let n = c["workers"].as_u64().unwrap_or(crate::DEFAULT_FISH_WORKERS);
    if !(1..=8).contains(&n) {
        return Err(err("workers must be in 1..=8"));
    }
    let n = n as usize;
    let review = path(c, "review_worker")?;
    let exe = path(c, "peta_executable")?;
    let gpu = path(c, "gpu_lock")?;
    let identity = json!({"schema":1,"legacy_config_sha256":hash(&old_bytes),"legacy_campaign":legacy,"legacy_campaign_sha256":hash(&fs::read(legacy.join("campaign.json"))?),"legacy_coverage_sha256":legacy_coverage_sha,"review_worker_sha256":hash(&fs::read(&review)?),"peta_executable_sha256":hash(&fs::read(&exe)?),"gpu_lock":gpu,"workers":n,"annotation_root":root,"annotation_env":c.get("annotation_env").cloned().unwrap_or(json!({}))});
    let identity_sha = hash(&serde_json::to_vec(&identity)?);
    let held_path = path(&old, "heldout")?;
    let (held_ids, held_texts, held_sha) = held_metadata(&held_path)?;
    if campaign["heldout_sha256"] != held_sha {
        return Err(err("heldout metadata identity mismatch"));
    }
    let (source_blocked, source_sha) = source_exclusions(&path(&old, "source")?, &held_texts)?;
    if old["source_sha256"] != source_sha || campaign["source_sha256"] != source_sha {
        return Err(err("legacy source identity mismatch"));
    }
    let queue_bytes = fs::read(path(&old, "queue")?)?;
    if old["queue_sha256"] != hash(&queue_bytes) || campaign["queue_sha256"] != hash(&queue_bytes) {
        return Err(err("legacy queue identity mismatch"));
    }
    if root.exists() {
        let m = load(&root.join("manifest.json"))?;
        if m["identity_sha256"] != identity_sha || m["identity"] != identity {
            return Err(err("existing annotation preparation has another identity"));
        }
        verify_manifest(&m)?;
        return Ok(m);
    }
    let mut shards = vec![String::new(); n];
    let mut counts = vec![0usize; n];
    let mut seen = BTreeSet::new();
    let mut excluded = BTreeMap::<String, usize>::new();
    let mut pending = 0usize;
    for l in std::str::from_utf8(&queue_bytes)?
        .lines()
        .filter(|x| !x.trim().is_empty())
    {
        let q: Value = serde_json::from_str(l)?;
        let id = field(&q, "id")?;
        if !seen.insert(id.to_string()) {
            return Err(err("duplicate legacy queue ID"));
        }
        let why = if covered.contains(id) {
            Some("legacy_receipt_or_reservation")
        } else if held_ids.contains(id) {
            Some("heldout_id")
        } else if source_blocked.contains(id) {
            Some("heldout_text_or_joined_source")
        } else if q["state"] != "pending_annotation_review" {
            Some("nonpending_state")
        } else {
            None
        };
        if let Some(why) = why {
            *excluded.entry(why.into()).or_default() += 1;
            continue;
        }
        let slot = pending % n;
        shards[slot].push_str(l);
        shards[slot].push('\n');
        counts[slot] += 1;
        pending += 1;
    }
    if pending == 0 {
        return Err(err("no pending annotation IDs after exclusions"));
    }
    let (context_bytes, context_extra_ids) = context_union(
        &held_path,
        &path(&old, "source")?,
        &source_blocked,
        &held_ids,
        &held_sha,
        &source_sha,
    )?;
    let context_sha = hash(&context_bytes);
    let stage = root.with_extension(format!("prepare-{}", std::process::id()));
    fs::create_dir(&stage)?;
    atomic(&stage.join("context-exclusions.conllu"), &context_bytes)?;
    for name in ["peta-gparse", "peta-python", "peta-python310"] {
        symlink(&exe, stage.join(name))?;
    }
    let mut configs = Vec::new();
    let mut campaigns = Vec::new();
    for i in 0..n {
        let worker = format!("worker-{i:02}");
        let target = root.join(&worker);
        let staging = stage.join(&worker);
        fs::create_dir(&staging)?;
        let queue = target.join("queue.jsonl");
        atomic(&staging.join("queue.jsonl"), shards[i].as_bytes())?;
        let mut cfg = old.clone();
        cfg["root"] = json!(target.join("campaign"));
        cfg["queue"] = json!(queue);
        cfg["queue_sha256"] = json!(hash(shards[i].as_bytes()));
        cfg["heldout"] = json!(root.join("context-exclusions.conllu"));
        if let Some(overrides) = c.get("annotation_env") {
            for (k, v) in overrides
                .as_object()
                .ok_or("annotation_env must be object")?
            {
                if !k.starts_with("CYCLE_") && !k.starts_with("ANNOT_") {
                    return Err(err(format!("unsupported annotation env {k}")));
                }
                if !v.is_string() {
                    return Err(err("annotation env values must be strings"));
                }
                cfg["env"][k] = v.clone();
            }
        }
        for (key, real, wrapper) in [
            ("CYCLE_GPARSE", "CYCLE_PETA_GPARSE_REAL", "peta-gparse"),
            ("CYCLE_VENV", "CYCLE_PETA_VENV_REAL", "peta-python"),
            ("CYCLE_VENV310", "CYCLE_PETA_VENV310_REAL", "peta-python310"),
        ] {
            let original = field(&cfg["env"], key)?.to_string();
            if !Path::new(&original).is_absolute() {
                return Err(err("GPU executable must be absolute"));
            }
            cfg["env"][real] = json!(original);
            cfg["env"][key] = json!(root.join(wrapper));
        }
        cfg["env"]["CYCLE_PETA_GPU_LOCK"] = json!(gpu);
        let bytes = serde_json::to_vec_pretty(&cfg)?;
        atomic(&staging.join("config.json"), &bytes)?;
        configs.push(json!({"slot":i,"path":target.join("config.json"),"sha256":hash(&bytes),"queue_sha256":cfg["queue_sha256"],"ids":counts[i],"campaign":target.join("campaign")}));
        let staged_campaign = staging.join("campaign");
        fs::create_dir(&staged_campaign)?;
        fs::create_dir(staged_campaign.join("ledger"))?;
        fs::create_dir(staged_campaign.join("jobs"))?;
        save(
            &staged_campaign.join("campaign.json"),
            &json!({"config_sha256":hash(&bytes),"source_sha256":cfg["source_sha256"],"queue_sha256":cfg["queue_sha256"],"heldout_sha256":context_sha}),
        )?;
        File::open(&staged_campaign)?.sync_all()?;
        campaigns.push(json!({"root":target.join("campaign"),"config":target.join("config.json"),"config_sha256":hash(&bytes)}));
        File::open(&staging)?.sync_all()?;
    }
    let manifest = json!({"schema":1,"identity":identity,"identity_sha256":identity_sha,"configs":configs,"campaigns":campaigns,"pending_ids":pending,"excluded":excluded,"legacy_covered_ids":covered.len(),"legacy_outcomes":legacy_outcomes,"heldout_metadata_only":true,"context_exclusions_sha256":context_sha,"context_extra_ids":context_extra_ids,"context_file_is_assessment":false,"source_sha256":source_sha,"created_unix":now()});
    save(&stage.join("manifest.json"), &manifest)?;
    File::open(&stage)?.sync_all()?;
    fs::rename(&stage, &root)?;
    File::open(parent)?.sync_all()?;
    Ok(manifest)
}
fn verify_manifest(m: &Value) -> Result<()> {
    for cfg in m["configs"].as_array().ok_or("missing prepared configs")? {
        let b = fs::read(path(cfg, "path")?)?;
        if cfg["sha256"] != hash(&b) {
            return Err(err("prepared config changed"));
        }
        let v: Value = serde_json::from_slice(&b)?;
        if hash(&fs::read(path(&v, "queue")?)?) != field(&v, "queue_sha256")? {
            return Err(err("prepared queue changed"));
        }
        let campaign = load(&path(&v, "root")?.join("campaign.json"))?;
        let held_sha = hash(&fs::read(path(&v, "heldout")?)?);
        if campaign["config_sha256"] != hash(&b)
            || campaign["source_sha256"] != v["source_sha256"]
            || campaign["queue_sha256"] != v["queue_sha256"]
            || campaign["heldout_sha256"] != held_sha
            || m["context_exclusions_sha256"] != held_sha
        {
            return Err(err("prepared campaign or context identity changed"));
        }
    }
    Ok(())
}
fn pid_stamp(pid: u32) -> Option<String> {
    let s = fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
    Some(s.rsplit_once(") ")?.1.split_whitespace().nth(19)?.into())
}
fn marker(p: &Path, text: &str) -> Result<()> {
    if !p.exists() {
        let mut f = OpenOptions::new().create_new(true).write(true).open(p)?;
        f.write_all(text.as_bytes())?;
        f.sync_all()?;
    }
    Ok(())
}
fn remove_own(p: &Path, owner: &str) -> Result<()> {
    if p.exists() && fs::read_to_string(p)? == owner {
        fs::remove_file(p)?;
    }
    Ok(())
}
fn pool_status(root: &Path, state: &str, detail: &str) -> Result<()> {
    save(
        &root.join("status.json"),
        &json!({"state":state,"detail":detail,"heartbeat_unix":now(),"workers_parallel":true,"gpu_shared_lock":true}),
    )
}

/// Start each prepared controller once; uncertain exits are never automatically restarted.
pub fn run(c: &Value, once: bool) -> Result<()> {
    let m = prepare(c)?;
    // A separate lifetime lease closes the prepare/run gap. Descendants retain it after parent death.
    let (legacy_lifetime, covered, outcomes) = legacy_quiescent(&path(c, "legacy_campaign")?)?;
    if m["identity"]["legacy_coverage_sha256"] != coverage_hash(&covered, &outcomes)? {
        return Err(err("legacy coverage changed after preparation"));
    }
    let _legacy_lifetime = legacy_lifetime;
    let root = annotation_root(c)?;
    let _lock = exclusive(&root.join("supervisor.lock"))?;
    verify_manifest(&m)?;
    let common = path(c, "run_root")?;
    let journal = root.join("children.json");
    let owner = format!("peta-supervisor:{}\n", field(&m, "identity_sha256")?);
    if root.join("STOP").exists() || common.join("STOP").exists() {
        for item in m["configs"].as_array().ok_or("missing configs")? {
            let child_root = path(item, "campaign")?;
            marker(&child_root.join("STOP"), &owner)?;
        }
        if journal.exists()
            && load(&journal)?
                .as_array()
                .ok_or("invalid child journal")?
                .iter()
                .any(|x| x["state"] != "finished")
        {
            pool_status(
                &root,
                "stopping_existing",
                "STOP propagated; unresolved children require inspection",
            )?;
            return Err(err(
                "stop requested for unresolved existing children; no new launch",
            ));
        }
        pool_status(&root, "stopped", "pool/common STOP present")?;
        return Ok(());
    }
    if journal.exists() {
        let old = load(&journal)?;
        for x in old.as_array().ok_or("invalid child journal")? {
            if x["state"] == "reserved" {
                return Err(err("uncertain child launch; inspect before restart"));
            }
            if x["state"] == "running" {
                return Err(err(format!(
                    "recorded worker PID {}; current stamp {:?}; inspect orphan or unresolved exit",
                    x["pid"],
                    x["pid"].as_u64().and_then(|p| pid_stamp(p as u32))
                )));
            }
            if x["state"] != "finished" {
                return Err(err("prior child failed/uncertain; manual review required"));
            }
        }
    }
    let review = path(c, "review_worker")?;
    let mut children: Vec<(Child, PathBuf, usize)> = Vec::new();
    let mut records = Vec::new();
    for item in m["configs"].as_array().ok_or("missing configs")? {
        let cfg = path(item, "path")?;
        let child_root = path(item, "campaign")?;
        fs::create_dir_all(&child_root)?;
        remove_own(&child_root.join("STOP"), &owner)?;
        if root.join("PAUSE").exists() || common.join("PAUSE").exists() {
            marker(&child_root.join("PAUSE"), &owner)?;
        } else {
            remove_own(&child_root.join("PAUSE"), &owner)?;
        }
        let log = OpenOptions::new()
            .create(true)
            .append(true)
            .open(child_root.join("controller.log"))?;
        let index = records.len();
        records.push(
            json!({"config":cfg,"campaign":child_root,"state":"reserved","created_unix":now()}),
        );
        save(&journal, &json!(records))?;
        let mut cmd = Command::new(&review);
        inherit_lease(&mut cmd, &_legacy_lifetime);
        cmd.arg("run").arg(&cfg);
        if once {
            cmd.arg("--once");
        }
        cmd.stdout(Stdio::from(log.try_clone()?))
            .stderr(Stdio::from(log));
        let child = match cmd.spawn() {
            Ok(ch) => ch,
            Err(e) => {
                pool_status(
                    &root,
                    "paused_uncertain",
                    &format!("child spawn error: {e}"),
                )?;
                for (_, p, _) in &children {
                    marker(&p.join("STOP"), &owner)?;
                }
                return Err(e.into());
            }
        };
        let pid = child.id();
        records[index]["state"] = json!("running");
        records[index]["pid"] = json!(pid);
        records[index]["start_stamp"] = json!(pid_stamp(pid));
        save(&journal, &json!(records))?;
        children.push((child, child_root, index));
    }
    let mut failed = false;
    let mut finished = BTreeSet::new();
    let sleep = c["supervisor_poll_seconds"]
        .as_u64()
        .unwrap_or(5)
        .clamp(1, 30);
    while finished.len() < children.len() {
        let stop = root.join("STOP").exists() || common.join("STOP").exists() || failed;
        let pause = root.join("PAUSE").exists() || common.join("PAUSE").exists();
        for (_, p, i) in &children {
            if finished.contains(i) {
                continue;
            }
            if stop {
                marker(&p.join("STOP"), &owner)?;
            } else if pause {
                marker(&p.join("PAUSE"), &owner)?;
            } else {
                remove_own(&p.join("PAUSE"), &owner)?;
            }
        }
        for (child, p, i) in &mut children {
            if finished.contains(i) {
                continue;
            }
            if let Some(exit) = child.try_wait()? {
                let state = load(&p.join("status.json"))
                    .ok()
                    .and_then(|x| x["state"].as_str().map(String::from))
                    .unwrap_or_else(|| "unknown".into());
                let clean = exit.success() && matches!(state.as_str(), "idle" | "stopped");
                records[*i]["state"] = json!(if clean {
                    "finished"
                } else {
                    "failed_or_uncertain"
                });
                records[*i]["exit_code"] = json!(exit.code());
                records[*i]["controller_state"] = json!(state);
                save(&journal, &json!(records))?;
                finished.insert(*i);
                failed |= !clean;
            }
        }
        pool_status(
            &root,
            if failed {
                "stopping_after_failure"
            } else if stop {
                "stopping"
            } else if pause {
                "paused"
            } else {
                "running"
            },
            "controllers keep independent reservations; no automatic retry",
        )?;
        if finished.len() < children.len() {
            thread::sleep(Duration::from_secs(sleep));
        }
    }
    pool_status(
        &root,
        if failed {
            "paused_uncertain"
        } else if root.join("STOP").exists() || common.join("STOP").exists() {
            "stopped"
        } else {
            "idle"
        },
        "all supervised children exited",
    )?;
    if failed {
        return Err(err(
            "annotation child failed or uncertain; inspect per-campaign status/ledger",
        ));
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::os::unix::fs::PermissionsExt;
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(0);
    struct Fixture {
        dir: PathBuf,
        c: Value,
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.dir);
        }
    }
    fn fixture() -> Fixture {
        let dir = std::env::temp_dir().join(format!(
            "peta-supervisor-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&dir).unwrap();
        let legacy = dir.join("legacy");
        fs::create_dir(&legacy).unwrap();
        fs::create_dir(legacy.join("ledger")).unwrap();
        fs::create_dir(legacy.join("jobs")).unwrap();
        fs::write(legacy.join("controller.lock"), "").unwrap();
        fs::write(legacy.join("STOP"), "operator\n").unwrap();
        save(&legacy.join("status.json"), &json!({"state":"stopped"})).unwrap();
        let sent = |id: &str, t: &str| {
            format!("# sent_id = {id}\n# text = {t}\n1\t{t}\t_\tNOUN\tNN\t_\t0\troot\t_\t_\n\n")
        };
        let held = sent("held:1", "Protected");
        fs::write(dir.join("held.conllu"), &held).unwrap();
        let mut source = String::new();
        let mut queue = String::new();
        for (id, t) in [
            ("held:1", "Protected"),
            ("alias:1", "Protected"),
            ("done:1", "Done"),
            ("remaining:1", "One"),
            ("remaining:2", "Two"),
            ("remaining:3", "Three"),
        ] {
            let b = sent(id, t);
            source.push_str(&b);
            queue.push_str(&format!("{}\n",json!({"id":id,"state":"pending_annotation_review","tree_sha256":hash(b.as_bytes())})));
        }
        fs::write(dir.join("source.conllu"), &source).unwrap();
        fs::write(dir.join("queue.jsonl"), &queue).unwrap();
        save(
            &legacy.join("ledger/00000000.json"),
            &json!({"job":0,"ids":["done:1"]}),
        )
        .unwrap();
        let old = json!({"source":dir.join("source.conllu"),"source_sha256":hash(source.as_bytes()),"queue":dir.join("queue.jsonl"),"queue_sha256":hash(queue.as_bytes()),"heldout":dir.join("held.conllu"),"root":legacy,"cycle":"/bin/true","codex":"/bin/true","env":{"CYCLE_GPARSE":"/bin/true","CYCLE_VENV":"/bin/true","CYCLE_VENV310":"/bin/true"}});
        let bytes = serde_json::to_vec_pretty(&old).unwrap();
        fs::write(dir.join("legacy-config.json"), &bytes).unwrap();
        save(&legacy.join("campaign.json"),&json!({"config_sha256":hash(&bytes),"source_sha256":old["source_sha256"],"queue_sha256":old["queue_sha256"],"heldout_sha256":hash(held.as_bytes())})).unwrap();
        let c = json!({"run_root":dir.join("new"),"legacy_config":dir.join("legacy-config.json"),"legacy_campaign":legacy,"workers":3,"review_worker":"/bin/true","peta_executable":"/bin/true","gpu_lock":dir.join("gpu.lock"),"supervisor_poll_seconds":1});
        Fixture { dir, c }
    }
    #[test]
    fn shards_are_disjoint_pinned_idempotent_and_exclude_heldout_aliases() {
        let f = fixture();
        let m = prepare(&f.c).unwrap();
        assert_eq!(m["pending_ids"], 3);
        let mut all = BTreeSet::new();
        for x in m["configs"].as_array().unwrap() {
            let cfg = load(&path(x, "path").unwrap()).unwrap();
            let rows = fs::read_to_string(path(&cfg, "queue").unwrap()).unwrap();
            assert_eq!(x["ids"], 1);
            for l in rows.lines() {
                let v: Value = serde_json::from_str(l).unwrap();
                assert!(all.insert(field(&v, "id").unwrap().to_string()));
            }
            assert!(field(&cfg["env"], "CYCLE_GPARSE")
                .unwrap()
                .ends_with("peta-gparse"));
            assert!(field(&cfg["env"], "CYCLE_VENV")
                .unwrap()
                .ends_with("peta-python"));
            assert_eq!(cfg["env"]["CYCLE_PETA_VENV_REAL"], "/bin/true");
        }
        assert_eq!(
            all,
            BTreeSet::from([
                "remaining:1".into(),
                "remaining:2".into(),
                "remaining:3".into()
            ])
        );
        assert_eq!(prepare(&f.c).unwrap(), m);
        let first = &m["configs"][0];
        fs::write(path(first, "path").unwrap(), "{}").unwrap();
        assert!(prepare(&f.c).is_err());
    }
    #[test]
    fn prepared_campaigns_are_ready_for_empty_store_ingestion() {
        let f = fixture();
        let m = prepare(&f.c).unwrap();
        for x in m["campaigns"].as_array().unwrap() {
            let cfg = load(&path(x, "config").unwrap()).unwrap();
            let root = path(x, "root").unwrap();
            let campaign = load(&root.join("campaign.json")).unwrap();
            assert_eq!(campaign["config_sha256"], x["config_sha256"]);
            assert_eq!(campaign["queue_sha256"], cfg["queue_sha256"]);
            assert_eq!(campaign["source_sha256"], cfg["source_sha256"]);
            assert!(root.join("ledger").is_dir());
            assert!(root.join("jobs").is_dir());
        }
        fs::write(f.dir.join("held.conllu"), "changed").unwrap();
        assert!(prepare(&f.c)
            .unwrap_err()
            .to_string()
            .contains("heldout metadata identity mismatch"));
    }
    #[test]
    fn active_or_unresolved_legacy_is_not_migrated() {
        let f = fixture();
        let lock = exclusive(&f.dir.join("legacy/controller.lock")).unwrap();
        assert!(prepare(&f.c)
            .unwrap_err()
            .to_string()
            .contains("still active"));
        drop(lock);
        let j = f.dir.join("legacy/jobs/00000001");
        fs::create_dir(&j).unwrap();
        save(
            &j.join("reserved.json"),
            &json!({"job":1,"ids":["remaining:1"]}),
        )
        .unwrap();
        assert!(prepare(&f.c)
            .unwrap_err()
            .to_string()
            .contains("uncertain legacy reservation"));
        assert!(!annotation_root(&f.c).unwrap().exists());
    }
    #[test]
    fn stopped_marker_and_status_both_required() {
        let f = fixture();
        fs::remove_file(f.dir.join("legacy/STOP")).unwrap();
        assert!(prepare(&f.c).is_err());
        fs::write(f.dir.join("legacy/STOP"), "").unwrap();
        save(
            &f.dir.join("legacy/status.json"),
            &json!({"state":"paused_uncertain"}),
        )
        .unwrap();
        assert!(prepare(&f.c).is_err());
    }
    #[test]
    fn joined_source_and_exact_text_aliases_are_excluded() {
        let f = fixture();
        let p = f.dir.join("source.conllu");
        let old = fs::read_to_string(&p).unwrap();
        let joined = old.replace("\n\n# sent_id = remaining:2", "\n# sent_id = remaining:2");
        fs::write(&p, &joined).unwrap();
        let held = held_metadata(&f.dir.join("held.conllu")).unwrap();
        let (blocked, sha) = source_exclusions(&p, &held.1).unwrap();
        assert_eq!(sha, hash(joined.as_bytes()));
        assert!(blocked.contains("remaining:1"));
        assert!(blocked.contains("remaining:2"));
        assert!(blocked.contains("alias:1"));
        assert!(!blocked.contains("remaining:3"));
    }
    #[test]
    fn pool_lock_and_unresolved_child_journal_prevent_duplicate_launch() {
        let f = fixture();
        prepare(&f.c).unwrap();
        let root = annotation_root(&f.c).unwrap();
        let lock = exclusive(&root.join("supervisor.lock")).unwrap();
        assert!(run(&f.c, true)
            .unwrap_err()
            .to_string()
            .contains("lock held"));
        drop(lock);
        save(&root.join("children.json"), &json!([{"state":"reserved"}])).unwrap();
        assert!(run(&f.c, true)
            .unwrap_err()
            .to_string()
            .contains("uncertain child launch"));
        save(
            &root.join("children.json"),
            &json!([{"state":"running","pid":std::process::id()}]),
        )
        .unwrap();
        assert!(run(&f.c, true)
            .unwrap_err()
            .to_string()
            .contains("recorded worker PID"));
    }
    #[test]
    fn once_supervises_all_children_without_hidden_restarts() {
        let mut f = fixture();
        let fake = f.dir.join("fake-worker");
        fs::write(&fake,"#!/bin/sh\nbase=$(dirname \"$2\")\nmkdir -p \"$base/campaign\"\nprintf '{\"state\":\"idle\"}' > \"$base/campaign/status.json\"\nprintf 'once\\n' >> \"$base/campaign/calls\"\n").unwrap();
        fs::set_permissions(&fake, fs::Permissions::from_mode(0o700)).unwrap();
        f.c["review_worker"] = json!(fake);
        run(&f.c, true).unwrap();
        let root = annotation_root(&f.c).unwrap();
        let journal = load(&root.join("children.json")).unwrap();
        assert_eq!(journal.as_array().unwrap().len(), 3);
        for x in journal.as_array().unwrap() {
            assert_eq!(x["state"], "finished");
            assert_eq!(
                fs::read_to_string(path(x, "campaign").unwrap().join("calls")).unwrap(),
                "once\n"
            );
        }
        assert_eq!(load(&root.join("status.json")).unwrap()["state"], "idle");
    }
    #[test]
    fn failing_child_is_not_restarted_and_orphan_stop_propagates() {
        let mut f = fixture();
        let fake = f.dir.join("fake-failure");
        fs::write(&fake,"#!/bin/sh\nbase=$(dirname \"$2\")\nprintf 'once\\n' >> \"$base/campaign/calls\"\nexit 1\n").unwrap();
        fs::set_permissions(&fake, fs::Permissions::from_mode(0o700)).unwrap();
        f.c["review_worker"] = json!(fake);
        assert!(run(&f.c, true).is_err());
        let refusal = run(&f.c, true).unwrap_err().to_string();
        assert!(
            refusal.contains("manual review"),
            "unexpected refusal: {refusal}"
        );
        let root = annotation_root(&f.c).unwrap();
        let journal = load(&root.join("children.json")).unwrap();
        for x in journal.as_array().unwrap() {
            assert_eq!(
                fs::read_to_string(path(x, "campaign").unwrap().join("calls")).unwrap(),
                "once\n"
            );
        }
        fs::write(path(&f.c, "run_root").unwrap().join("STOP"), "operator\n").unwrap();
        assert!(run(&f.c, false)
            .unwrap_err()
            .to_string()
            .contains("unresolved existing children"));
        for x in journal.as_array().unwrap() {
            assert!(path(x, "campaign").unwrap().join("STOP").exists());
        }
        assert_eq!(
            load(&root.join("status.json")).unwrap()["state"],
            "stopping_existing"
        );
    }
    #[test]
    fn legacy_receipt_classes_are_separate_and_coverage_is_pinned() {
        let f = fixture();
        let legacy = f.dir.join("legacy");
        save(&legacy.join("ledger/00000000.json"),&json!({"job":0,"ids":["done:1"],"outcome":"checked","details":{"accepted":["done:1"]}})).unwrap();
        save(
            &legacy.join("ledger/00000001.json"),
            &json!({"job":1,"ids":["remaining:1"],"outcome":"quarantined"}),
        )
        .unwrap();
        save(&legacy.join("ledger/00000002.json"),&json!({"job":2,"ids":["remaining:2"],"outcome":"checked","details":{"accepted":[],"missing_final":["remaining:2"]}})).unwrap();
        let m = prepare(&f.c).unwrap();
        let classes = &m["legacy_outcomes"]["receipt_classes"];
        assert_eq!(classes["accepted"], 1);
        assert_eq!(classes["quarantined"], 1);
        assert_eq!(classes["missing_or_rejected"], 1);
        assert_eq!(classes["other_excluded"], 0);
        assert_eq!(m["pending_ids"], 1);
        save(&legacy.join("ledger/00000003.json"),&json!({"job":3,"ids":["remaining:3"],"outcome":"checked","details":{"accepted":["remaining:3"]}})).unwrap();
        assert!(prepare(&f.c)
            .unwrap_err()
            .to_string()
            .contains("another identity"));
    }
    #[test]
    fn intended_child_inheritance_keeps_parent_and_unrelated_exec_close_on_exec() {
        let f = fixture();
        let p = f.dir.join("legacy/controller.lock");
        let preparation = PreparationLease(exclusive(&p).unwrap());
        let transient_copy = preparation.0.try_clone().unwrap();
        assert!(exclusive(&p).is_err());
        drop(preparation);
        let independent = exclusive(&p).unwrap();
        drop(independent);
        drop(transient_copy);
        let owner = exclusive(&p).unwrap();
        let mut intended = Command::new("/bin/sh");
        intended.args(["-c", "sleep 0.15"]);
        inherit_lease(&mut intended, &owner);
        let flags = unsafe { libc::fcntl(owner.as_raw_fd(), libc::F_GETFD) };
        assert_ne!(flags & libc::FD_CLOEXEC, 0);
        let mut unrelated = Command::new("/bin/sh")
            .args(["-c", "sleep 0.5"])
            .spawn()
            .unwrap();
        let mut child = intended.spawn().unwrap();
        drop(owner);
        assert!(exclusive(&p).is_err());
        assert!(child.wait().unwrap().success());
        let released = exclusive(&p).unwrap();
        drop(released);
        assert!(unrelated.wait().unwrap().success());
    }
    #[test]
    fn legacy_lease_is_held_and_inherited_until_children_finish() {
        let mut f = fixture();
        let fake = f.dir.join("fake-wait");
        let permit = f.dir.join("permit");
        let started = f.dir.join("started");
        fs::write(&fake,format!("#!/bin/sh\nbase=$(dirname \"$2\")\nprintf 'started' > '{}'\ni=0\nwhile [ ! -e '{}' ] && [ \"$i\" -lt 50 ]; do sleep 0.1; i=$((i+1)); done\nprintf '{{\"state\":\"idle\"}}' > \"$base/campaign/status.json\"\n",started.display(),permit.display())).unwrap();
        fs::set_permissions(&fake, fs::Permissions::from_mode(0o700)).unwrap();
        f.c["review_worker"] = json!(fake);
        let cfg = f.c.clone();
        let handle = thread::spawn(move || run(&cfg, true).map_err(|e| e.to_string()));
        for _ in 0..100 {
            if started.exists() {
                break;
            }
            thread::sleep(Duration::from_millis(20));
        }
        let excluded = exclusive(&f.dir.join("legacy/controller.lock")).is_err();
        let root = annotation_root(&f.c).unwrap();
        let records = load(&root.join("children.json")).unwrap();
        let pid = records[0]["pid"].as_u64().unwrap();
        let inherited = fs::read_dir(format!("/proc/{pid}/fd"))
            .unwrap()
            .filter_map(|e| e.ok())
            .any(|e| fs::read_link(e.path()).ok() == Some(f.dir.join("legacy/controller.lock")));
        fs::write(&permit, "finish\n").unwrap();
        assert!(handle.join().unwrap().is_ok());
        assert!(
            excluded,
            "negative control: stopped legacy cannot restart while shard children run"
        );
        assert!(inherited, "lease survives executable handoff");
        let _released = exclusive(&f.dir.join("legacy/controller.lock")).unwrap();
    }
    #[test]
    fn context_exclusion_union_blocks_alias_and_joined_prefix_ids() {
        let f = fixture();
        let held = f.dir.join("held.conllu");
        let held_bytes = fs::read(&held).unwrap();
        let source = f.dir.join("source.conllu");
        let joined = fs::read_to_string(&source)
            .unwrap()
            .replace("\n\n# sent_id = remaining:2", "\n# sent_id = remaining:2");
        fs::write(&source, &joined).unwrap();
        let (ids, texts, held_sha) = held_metadata(&held).unwrap();
        let (blocked, source_sha) = source_exclusions(&source, &texts).unwrap();
        let (union, n) =
            context_union(&held, &source, &blocked, &ids, &held_sha, &source_sha).unwrap();
        assert_eq!(n, 3);
        let union_path = f.dir.join("context.conllu");
        fs::write(&union_path, &union).unwrap();
        let (excluded, _, _) = held_metadata(&union_path).unwrap();
        for id in ["held:1", "alias:1", "remaining:1", "remaining:2"] {
            assert!(excluded.contains(id));
        }
        assert!(!excluded.contains("remaining:3"));
        assert_eq!(fs::read(&held).unwrap(), held_bytes);
        assert!(union.starts_with(&held_bytes));
        assert!(std::str::from_utf8(&union)
            .unwrap()
            .contains("# sent_id = alias:1\n# text = Protected\n\n"));
        assert!(context_union(&held, &source, &blocked, &ids, &held_sha, "wrong").is_err());
    }
    #[test]
    fn common_stop_prevents_start() {
        let f = fixture();
        fs::create_dir_all(path(&f.c, "run_root").unwrap()).unwrap();
        fs::write(path(&f.c, "run_root").unwrap().join("STOP"), "operator\n").unwrap();
        run(&f.c, false).unwrap();
        let root = annotation_root(&f.c).unwrap();
        assert!(!root.join("children.json").exists());
        assert_eq!(load(&root.join("status.json")).unwrap()["state"], "stopped");
    }
}
