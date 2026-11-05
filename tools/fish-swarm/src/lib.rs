//! Immutable prepared annotation queues; transport eligibility is never semantic approval.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    os::{
        fd::AsRawFd,
        unix::{fs::OpenOptionsExt, process::CommandExt},
    },
    path::{Path, PathBuf},
    process::{Command, Stdio},
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
const MAX_BYTES: u64 = 64 * 1024 * 1024;
pub fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn digest(s: &str) -> bool {
    s.len() == 64
        && s.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
pub fn now() -> Result<u64> {
    Ok(SystemTime::now().duration_since(UNIX_EPOCH)?.as_secs())
}
fn field<'a>(v: &'a Value, k: &str) -> Result<&'a str> {
    v[k].as_str()
        .ok_or_else(|| format!("missing string {k}").into())
}
fn integer(v: &Value, k: &str) -> Result<u64> {
    v[k].as_u64()
        .ok_or_else(|| format!("missing unsigned integer {k}").into())
}
fn keys(v: &Value, allowed: &[&str]) -> Result<()> {
    let o = v.as_object().ok_or("object required")?;
    if o.keys().any(|k| !allowed.contains(&k.as_str())) {
        return Err("unknown field".into());
    }
    Ok(())
}
fn absolute(s: &str) -> Result<PathBuf> {
    let p = PathBuf::from(s);
    if !p.is_absolute()
        || p.components().any(|x| {
            matches!(
                x,
                std::path::Component::ParentDir | std::path::Component::CurDir
            )
        })
    {
        return Err("canonical absolute path required".into());
    }
    Ok(p)
}
fn regular(p: &Path) -> Result<Vec<u8>> {
    if fs::canonicalize(p)? != p {
        return Err("symlink or noncanonical pinned path".into());
    }
    let mut f = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW)
        .open(p)?;
    if !f.metadata()?.is_file() || f.metadata()?.len() > MAX_BYTES {
        return Err("bounded regular file required".into());
    }
    let mut b = Vec::new();
    Read::by_ref(&mut f)
        .take(MAX_BYTES + 1)
        .read_to_end(&mut b)?;
    if b.len() as u64 > MAX_BYTES {
        return Err("file exceeds bound".into());
    }
    Ok(b)
}
fn fresh(p: &Path, b: &[u8]) -> Result<()> {
    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .custom_flags(libc::O_NOFOLLOW)
        .open(p)?;
    f.write_all(b)?;
    f.sync_all()?;
    File::open(p.parent().ok_or("no parent")?)?.sync_all()?;
    Ok(())
}
fn fresh_json(p: &Path, v: &Value) -> Result<()> {
    fresh(p, &serde_json::to_vec_pretty(v)?)
}
#[derive(Clone, Debug)]
pub struct Pin {
    pub path: PathBuf,
    pub sha256: String,
}
impl Pin {
    fn parse(v: &Value) -> Result<Self> {
        keys(v, &["path", "sha256"])?;
        let sha256 = field(v, "sha256")?.to_string();
        if !digest(&sha256) {
            return Err("invalid SHA256".into());
        }
        Ok(Self {
            path: absolute(field(v, "path")?)?,
            sha256,
        })
    }
    fn read(&self) -> Result<Vec<u8>> {
        let b = regular(&self.path)?;
        if hash(&b) != self.sha256 {
            return Err(format!("pin mismatch: {}", self.path.display()).into());
        }
        Ok(b)
    }
}
#[derive(Clone, Debug)]
pub struct Job {
    pub id: String,
    pub task: Pin,
    pub profile: Pin,
    pub source_pins: Vec<Pin>,
    pub provenance: Value,
    pub identity: String,
}
impl Job {
    fn parse(v: Value) -> Result<Self> {
        keys(
            &v,
            &[
                "schema",
                "id",
                "task",
                "profile",
                "source_pins",
                "provenance",
            ],
        )?;
        if integer(&v, "schema")? != 1 {
            return Err("unsupported queue schema".into());
        }
        let id = field(&v, "id")?.to_string();
        if id.is_empty()
            || id.len() > 64
            || !id
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b == b'_' || b == b'-')
        {
            return Err("unsafe job id".into());
        }
        let source_pins = v["source_pins"]
            .as_array()
            .ok_or("source_pins required")?
            .iter()
            .map(Pin::parse)
            .collect::<Result<Vec<_>>>()?;
        if source_pins.is_empty() || source_pins.len() > 64 || !v["provenance"].is_object() {
            return Err("source pins and object provenance required".into());
        }
        Ok(Self {
            id,
            task: Pin::parse(&v["task"])?,
            profile: Pin::parse(&v["profile"])?,
            source_pins,
            provenance: v["provenance"].clone(),
            identity: hash(&serde_json::to_vec(&v)?),
        })
    }
}
#[derive(Clone, Debug)]
pub struct Config {
    pub root: PathBuf,
    pub queue: Pin,
    pub executable: Pin,
    pub not_after_unix: u64,
    pub timeout_seconds: u64,
    pub poll_seconds: u64,
    pub identity: String,
}
impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        let b = regular(path)?;
        let v: Value = serde_json::from_slice(&b)?;
        keys(
            &v,
            &[
                "schema",
                "root",
                "queue",
                "queue_sha256",
                "fish_executable",
                "fish_executable_sha256",
                "not_after_unix",
                "timeout_seconds",
                "poll_seconds",
            ],
        )?;
        if integer(&v, "schema")? != 1 {
            return Err("unsupported config schema".into());
        }
        let timeout_seconds = match v.get("timeout_seconds") {
            Some(_) => integer(&v, "timeout_seconds")?,
            None => 300,
        };
        let poll_seconds = match v.get("poll_seconds") {
            Some(_) => integer(&v, "poll_seconds")?,
            None => 5,
        };
        if !(1..=900).contains(&timeout_seconds) || !(1..=60).contains(&poll_seconds) {
            return Err("timeout 1..900, poll 1..60 required".into());
        }
        let queue =
            Pin::parse(&json!({"path":field(&v,"queue")?,"sha256":field(&v,"queue_sha256")?}))?;
        let executable = Pin::parse(
            &json!({"path":field(&v,"fish_executable")?,"sha256":field(&v,"fish_executable_sha256")?}),
        )?;
        Ok(Self {
            root: absolute(field(&v, "root")?)?,
            queue,
            executable,
            not_after_unix: integer(&v, "not_after_unix")?,
            timeout_seconds,
            poll_seconds,
            identity: hash(&b),
        })
    }
    pub fn jobs(&self) -> Result<Vec<Job>> {
        let b = self.queue.read()?;
        let s = std::str::from_utf8(&b)?;
        let mut jobs = Vec::new();
        let mut ids = BTreeSet::new();
        for line in s.lines() {
            if line.trim().is_empty() {
                return Err("blank queue line".into());
            }
            let j = Job::parse(serde_json::from_str(line)?)?;
            if !ids.insert(j.id.clone()) {
                return Err("duplicate job id".into());
            }
            jobs.push(j);
        }
        Ok(jobs)
    }
}
fn closed_bound(inner: &Value, task: &[u8], profile: &[u8]) -> bool {
    let task_json: Value = match serde_json::from_slice(task) {
        Ok(v) => v,
        Err(_) => return false,
    };
    inner["schema"] == 1
        && task_json["id"].is_string()
        && inner["task_id"] == task_json["id"]
        && inner["task_file_sha256"] == hash(task)
        && inner["profile_file_sha256"] == hash(profile)
        && inner["task_sha256"].as_str().is_some_and(digest)
        && inner["review_only"] == true
        && inner["training_authorized"] == false
        && inner["replay_authorized"] == false
        && inner["exit_code"] == 0
        && inner["model_process_started"] == true
        && inner["model_process_reaped"] == true
        && inner["model_process_group_absent"] == true
}
fn closed_history(
    dir: &Path,
    reserved: &Value,
    outer: &Value,
    expected_status: &str,
) -> Result<bool> {
    let task = regular(&dir.join("task.json"))?;
    let profile = regular(&dir.join("profile.json"))?;
    let bytes = regular(&dir.join("output/receipt.json"))?;
    let inner: Value = serde_json::from_slice(&bytes)?;
    Ok(outer["schema"] == 1
        && outer["review_only"] == true
        && outer["annotation_gold"] == false
        && outer["timed_out"] == false
        && outer["task_file_sha256"] == reserved["task_file_sha256"]
        && outer["profile_file_sha256"] == reserved["profile_file_sha256"]
        && outer["task_file_sha256"] == hash(&task)
        && outer["profile_file_sha256"] == hash(&profile)
        && outer["task_id"] == inner["task_id"]
        && outer["task_sha256"] == inner["task_sha256"]
        && outer["executor_receipt_sha256"] == hash(&bytes)
        && inner["status"] == expected_status
        && closed_bound(&inner, &task, &profile))
}
/// Backoff after the 1st, 2nd and 3rd provider-busy refusal of one job; a 4th refusal holds the
/// job (and its pager) for operator review. Busy attempts are retained under `busy/<job>/<k>`.
pub const PROVIDER_BUSY_BACKOFF_SECONDS: [u64; 3] = [60, 120, 240];
/// The executor proved "provider busy, nothing done": the Codex child exited nonzero after an exact
/// capacity refusal and before any item, so no reply exists. Never eligible for training/transport.
fn busy_bound(inner: &Value, task: &[u8], profile: &[u8]) -> bool {
    let task_json: Value = match serde_json::from_slice(task) {
        Ok(v) => v,
        Err(_) => return false,
    };
    inner["schema"] == 1
        && task_json["id"].is_string()
        && inner["task_id"] == task_json["id"]
        && inner["task_file_sha256"] == hash(task)
        && inner["profile_file_sha256"] == hash(profile)
        && inner["status"] == "provider_busy"
        && inner["provider_busy_message"]
            .as_str()
            .is_some_and(|m| !m.is_empty())
        && inner["exit_code"].as_i64().is_some_and(|c| c != 0)
        && inner["review_only"] == true
        && inner["training_authorized"] == false
        && inner["replay_authorized"] == false
        && inner["model_process_started"] == true
        && inner["model_process_reaped"] == true
        && inner["model_process_group_absent"] == true
}
/// A retained provider-busy attempt: outer and inner receipts bind the same task/profile, the raw
/// Codex stdout still matches the executor's hash, and no reply was ever written.
fn busy_history(dir: &Path, reserved: &Value, outer: &Value) -> Result<bool> {
    let task = regular(&dir.join("task.json"))?;
    let profile = regular(&dir.join("profile.json"))?;
    let bytes = regular(&dir.join("output/receipt.json"))?;
    let inner: Value = serde_json::from_slice(&bytes)?;
    let raw = regular(&dir.join("output/raw.stdout"))?;
    Ok(outer["schema"] == 1
        && outer["status"] == "provider_busy"
        && outer["review_only"] == true
        && outer["annotation_gold"] == false
        && outer["transport_eligible"] == false
        && outer["training_authorized"] == false
        && outer["timed_out"] == false
        && outer["exit_code"].as_i64().is_some_and(|c| c != 0)
        && outer["finished_unix"].as_u64().is_some()
        && outer["task_file_sha256"] == reserved["task_file_sha256"]
        && outer["profile_file_sha256"] == reserved["profile_file_sha256"]
        && outer["task_file_sha256"] == hash(&task)
        && outer["profile_file_sha256"] == hash(&profile)
        && outer["task_id"] == inner["task_id"]
        && outer["executor_receipt_sha256"] == hash(&bytes)
        && outer["provider_busy_message"] == inner["provider_busy_message"]
        && inner["raw_output_hashes"]["stdout_sha256"] == hash(&raw)
        && !dir.join("output/reply.json").exists()
        && busy_bound(&inner, &task, &profile))
}
/// Retained busy attempts of one job, in order, each checked against this config and job identity.
/// Returns the finish time of each attempt; an unverifiable attempt is an error (uncertain).
fn busy_attempts(root: &Path, identity: &str, job: &Job) -> Result<Vec<u64>> {
    let dir = root.join("busy").join(&job.id);
    if !dir.exists() {
        return Ok(Vec::new());
    }
    let mut names = fs::read_dir(&dir)?
        .map(|e| Ok(e?.file_name().to_string_lossy().into_owned()))
        .collect::<Result<Vec<_>>>()?;
    names.sort();
    let mut finished = Vec::new();
    for (k, name) in names.iter().enumerate() {
        if *name != format!("{:02}", k + 1) {
            return Err("noncontiguous provider-busy attempts".into());
        }
        let p = dir.join(name);
        let reserved: Value = serde_json::from_slice(&regular(&p.join("reservation.json"))?)?;
        let outer: Value = serde_json::from_slice(&regular(&p.join("receipt.json"))?)?;
        if reserved["config_sha256"] != identity
            || outer["config_sha256"] != identity
            || reserved["job_sha256"] != job.identity.as_str()
            || outer["job_sha256"] != job.identity.as_str()
            || !busy_history(&p, &reserved, &outer)?
        {
            return Err("unverifiable provider-busy attempt".into());
        }
        finished.push(integer(&outer, "finished_unix")?);
    }
    Ok(finished)
}
pub struct Supervisor {
    pub config: Config,
    _lease: File,
}
impl Supervisor {
    pub fn open(config: Config) -> Result<Self> {
        fs::create_dir_all(&config.root)?;
        if fs::canonicalize(&config.root)? != config.root {
            return Err("canonical root required".into());
        }
        let lease = OpenOptions::new()
            .write(true)
            .create(true)
            .truncate(false)
            .custom_flags(libc::O_NOFOLLOW)
            .open(config.root.join("lease"))?;
        if unsafe { libc::flock(lease.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } != 0 {
            return Err("supervisor lease already held".into());
        }
        let identity = config.root.join("config-identity.json");
        if identity.exists() {
            let v: Value = serde_json::from_slice(&regular(&identity)?)?;
            if v["config_sha256"] != config.identity {
                return Err("root belongs to a different immutable config".into());
            }
        } else {
            fresh_json(
                &identity,
                &json!({"schema":1,"config_sha256":config.identity}),
            )?;
        }
        fs::create_dir_all(config.root.join("jobs"))?;
        Ok(Self {
            config,
            _lease: lease,
        })
    }
    fn publish(&self, v: &Value) -> Result<Value> {
        let tmp = self.config.root.join("status.next");
        // A crash can retain this temporary file; the lease owns its replacement.
        if tmp.exists() {
            fs::remove_file(&tmp)?;
        }
        fresh_json(&tmp, v)?;
        fs::rename(tmp, self.config.root.join("status.json"))?;
        File::open(&self.config.root)?.sync_all()?;
        Ok(v.clone())
    }
    /// Moves a closed provider-busy attempt out of `jobs/` so the job returns to the queue; the
    /// retained attempt directory is the durable record, the journal line is its readable index.
    fn retire_busy(&self, id: &str, time: u64) -> Result<Value> {
        let from = self.config.root.join("jobs").join(id);
        let outer: Value = serde_json::from_slice(&regular(&from.join("receipt.json"))?)?;
        let parent = self.config.root.join("busy").join(id);
        fs::create_dir_all(&parent)?;
        let k = fs::read_dir(&parent)?.count() + 1;
        let to = parent.join(format!("{k:02}"));
        if to.exists() {
            return Err("provider-busy attempt slot already used".into());
        }
        fs::rename(&from, &to)?;
        File::open(self.config.root.join("jobs"))?.sync_all()?;
        File::open(&parent)?.sync_all()?;
        let finished = integer(&outer, "finished_unix")?;
        let retry_after = PROVIDER_BUSY_BACKOFF_SECONDS
            .get(k - 1)
            .map(|b| finished + b);
        let line = json!({"schema":1,"config_sha256":self.config.identity,"job_id":id,"attempt":k,
            "attempt_directory":to,"finished_unix":finished,"journaled_unix":time,
            "provider_busy_message":outer["provider_busy_message"],"retry_after_unix":retry_after,
            "held_for_review":retry_after.is_none(),"uncertain":false,"usage_limit":false,
            "training_authorized":false});
        let mut journal = OpenOptions::new()
            .append(true)
            .create(true)
            .custom_flags(libc::O_NOFOLLOW)
            .open(self.config.root.join("provider-busy.jsonl"))?;
        journal.write_all(format!("{line}\n").as_bytes())?;
        journal.sync_all()?;
        Ok(line)
    }
    fn report(&self, state: &str, time: u64, extra: Value) -> Result<Value> {
        self.publish(&json!({"schema":1,"state":state,"observed_unix":time,"heartbeat_unix":time,"active_model_call":if state=="uncertain" {Value::Null} else {json!(state=="active")},"no_unresolved_reservations":!matches!(state,"uncertain"|"active"|"reserved"),"pid":std::process::id(),"config_sha256":self.config.identity,"not_after_unix":self.config.not_after_unix,"review_only":true,"training_authorized":false,"details":extra}))
    }
    fn boundary(&self, time: u64) -> Option<&'static str> {
        if self.config.root.join("STOP").exists() {
            Some("stopped")
        } else if time >= self.config.not_after_unix {
            Some("deadline_reached")
        } else if self.config.root.join("PAUSE").exists() {
            Some("paused")
        } else {
            None
        }
    }
    /// One job at most. Supplied clock permits deterministic boundary checks.
    pub fn step<F: Fn() -> Result<u64>>(&self, clock: F) -> Result<Value> {
        let time = clock()?;
        // Crash recovery: a closed provider-busy attempt still under jobs/ is retired first.
        let mut busy = Vec::new();
        for ent in fs::read_dir(self.config.root.join("jobs"))? {
            let p = ent?.path();
            let closed = (|| -> Result<bool> {
                let reserved: Value =
                    serde_json::from_slice(&regular(&p.join("reservation.json"))?)?;
                let receipt: Value = serde_json::from_slice(&regular(&p.join("receipt.json"))?)?;
                Ok(reserved["config_sha256"] == self.config.identity
                    && receipt["config_sha256"] == self.config.identity
                    && receipt["job_sha256"] == reserved["job_sha256"]
                    && receipt["status"] == "provider_busy"
                    && busy_history(&p, &reserved, &receipt)?)
            })()
            .unwrap_or(false);
            if closed {
                busy.push(
                    p.file_name()
                        .ok_or("job name")?
                        .to_string_lossy()
                        .into_owned(),
                );
            }
        }
        for id in busy {
            self.retire_busy(&id, time)?;
        }
        for ent in fs::read_dir(self.config.root.join("jobs"))? {
            let p = ent?.path();
            let terminal = (|| -> Result<bool> {
                let reserved: Value =
                    serde_json::from_slice(&regular(&p.join("reservation.json"))?)?;
                let receipt: Value = serde_json::from_slice(&regular(&p.join("receipt.json"))?)?;
                Ok(reserved["config_sha256"] == self.config.identity
                    && receipt["config_sha256"] == self.config.identity
                    && receipt["job_sha256"] == reserved["job_sha256"]
                    && receipt["training_authorized"] == false
                    && match receipt["status"].as_str() {
                        Some("not_started_boundary" | "spawn_failed") => true,
                        Some("validated_review_only") => {
                            receipt["exit_code"] == 0
                                && closed_history(&p, &reserved, &receipt, "validated_review_only")?
                        }
                        Some("quarantined_review_only") => {
                            closed_history(&p, &reserved, &receipt, "invalid_response")?
                        }
                        _ => false,
                    })
            })()
            .unwrap_or(false);
            if !terminal {
                return self.report(
                    "uncertain",
                    time,
                    json!({"job_directory":p,"automatic_retry":false}),
                );
            }
        }
        if let Some(state) = self.boundary(time) {
            return self.report(state, time, json!({}));
        }
        let jobs = self.config.jobs()?;
        let mut chosen = None;
        for j in jobs {
            let p = self.config.root.join("jobs").join(&j.id);
            if p.join("receipt.json").exists() {
                let r: Value = serde_json::from_slice(&regular(&p.join("receipt.json"))?)?;
                if r["config_sha256"] != self.config.identity || r["job_sha256"] != j.identity {
                    return Err("retained job binding mismatch".into());
                }
                if r["status"] != "validated_review_only"
                    && r["status"] != "quarantined_review_only"
                    && r["status"] != "not_started_boundary"
                {
                    return self.report(
                        "failed",
                        time,
                        json!({"job_id":j.id,"automatic_retry":false}),
                    );
                }
            } else if p.exists() {
                return self.report(
                    "uncertain",
                    time,
                    json!({"job_id":j.id,"automatic_retry":false}),
                );
            } else if chosen.is_none() {
                chosen = Some(j);
            }
        }
        if let Some(j) = &chosen {
            let finished = match busy_attempts(&self.config.root, &self.config.identity, j) {
                Ok(f) => f,
                Err(e) => {
                    return self.report(
                        "uncertain",
                        time,
                        json!({"job_id":j.id,"automatic_retry":false,"reason":e.to_string()}),
                    )
                }
            };
            if let Some(&last) = finished.last() {
                let Some(backoff) = PROVIDER_BUSY_BACKOFF_SECONDS.get(finished.len() - 1) else {
                    return self.report(
                        "provider_busy_held",
                        time,
                        json!({"job_id":j.id,"provider_busy_attempts":finished.len(),"automatic_retry":false,"review":"required"}),
                    );
                };
                if time < last + backoff {
                    return self.report(
                        "provider_busy_backoff",
                        time,
                        json!({"job_id":j.id,"provider_busy_attempts":finished.len(),"retry_after_unix":last + backoff}),
                    );
                }
            }
        }
        let Some(j) = chosen else {
            return self.report("queue_exhausted", time, json!({"automatic_repeat":false}));
        };
        let task = j.task.read()?;
        let profile = j.profile.read()?;
        for pin in &j.source_pins {
            pin.read()?;
        }
        self.config.executable.read()?;
        let task_json: Value = serde_json::from_slice(&task)?;
        let task_id = field(&task_json, "id")?;
        let time = clock()?;
        if let Some(state) = self.boundary(time) {
            return self.report(state, time, json!({}));
        }
        let dir = self.config.root.join("jobs").join(&j.id);
        fs::create_dir(&dir)?;
        File::open(dir.parent().ok_or("no job parent")?)?.sync_all()?;
        fresh_json(
            &dir.join("reservation.json"),
            &json!({"schema":1,"config_sha256":self.config.identity,"job_sha256":j.identity,"job_id":j.id,"task_file_sha256":j.task.sha256,"profile_file_sha256":j.profile.sha256,"reserved_unix":time,"source_pins":j.source_pins.iter().map(|p|json!({"path":p.path,"sha256":p.sha256})).collect::<Vec<_>>(),"provenance":j.provenance}),
        )?;
        fresh(&dir.join("task.json"), &task)?;
        fresh(&dir.join("profile.json"), &profile)?;
        let time = clock()?;
        if let Some(state) = self.boundary(time) {
            fresh_json(
                &dir.join("receipt.json"),
                &json!({"schema":1,"config_sha256":self.config.identity,"job_sha256":j.identity,"status":"not_started_boundary","boundary":state,"training_authorized":false}),
            )?;
            return self.report(state, time, json!({"job_id":j.id}));
        }
        let stdout = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(dir.join("executor.stdout"))?;
        let stderr = OpenOptions::new()
            .write(true)
            .create_new(true)
            .open(dir.join("executor.stderr"))?;
        self.report("reserved", time, json!({"job_id":j.id}))?;
        let child = Command::new(&self.config.executable.path)
            .arg("annotation-execute")
            .arg(dir.join("task.json"))
            .arg(dir.join("profile.json"))
            .arg(dir.join("output"))
            .arg(self.config.not_after_unix.to_string())
            .arg(self.config.timeout_seconds.to_string())
            .stdin(Stdio::null())
            .stdout(stdout)
            .stderr(stderr)
            .process_group(0)
            .spawn();
        let mut child = match child {
            Ok(c) => c,
            Err(e) => {
                fresh_json(
                    &dir.join("receipt.json"),
                    &json!({"schema":1,"config_sha256":self.config.identity,"job_sha256":j.identity,"status":"spawn_failed","error":e.to_string(),"training_authorized":false}),
                )?;
                return self.report(
                    "failed",
                    clock()?,
                    json!({"job_id":j.id,"automatic_retry":false}),
                );
            }
        };
        let pid = child.id();
        if let Err(e) = fresh_json(
            &dir.join("accepted.json"),
            &json!({"schema":1,"pid":pid,"config_sha256":self.config.identity,"job_sha256":j.identity,"accepted_unix":clock()?}),
        ) {
            unsafe {
                libc::kill(-(pid as i32), libc::SIGKILL);
            }
            let _ = child.wait();
            return Err(e);
        }
        self.report(
            "active",
            clock()?,
            json!({"job_id":j.id,"accepted_pid":pid}),
        )?;
        let started = Instant::now();
        let mut heartbeat = Instant::now();
        let mut timed_out = false;
        let exit = loop {
            if let Some(s) = child.try_wait()? {
                break s;
            }
            if started.elapsed() > Duration::from_secs(self.config.timeout_seconds + 10) {
                unsafe {
                    libc::kill(-(pid as i32), libc::SIGTERM);
                }
                std::thread::sleep(Duration::from_millis(200));
                unsafe {
                    libc::kill(-(pid as i32), libc::SIGKILL);
                }
                timed_out = true;
                break child.wait()?;
            }
            if heartbeat.elapsed() >= Duration::from_secs(5) {
                self.report(
                    "active",
                    clock()?,
                    json!({"job_id":j.id,"accepted_pid":pid}),
                )?;
                heartbeat = Instant::now();
            }
            std::thread::sleep(Duration::from_millis(25));
        };
        let mut transport = false;
        let mut quarantined = false;
        let mut busy = false;
        let mut busy_message = Value::Null;
        let mut typed_task_sha256 = None;
        let mut receipt_error = None;
        let rpath = dir.join("output/receipt.json");
        match regular(&rpath).and_then(|b| Ok(serde_json::from_slice::<Value>(&b)?)) {
            Ok(r) => {
                typed_task_sha256 = r["task_sha256"]
                    .as_str()
                    .filter(|s| digest(s))
                    .map(str::to_owned);
                let closed = closed_bound(&r, &task, &profile);
                transport = exit.success()
                    && !timed_out
                    && closed
                    && r["status"] == "validated_review_only";
                quarantined = !timed_out && closed && r["status"] == "invalid_response";
                busy = !timed_out && !exit.success() && busy_bound(&r, &task, &profile);
                if busy {
                    busy_message = r["provider_busy_message"].clone();
                }
                if !transport && !quarantined && !busy {
                    receipt_error = Some("executor receipt/exit failed eligibility");
                }
            }
            Err(_) => {
                receipt_error = Some("executor receipt missing or malformed");
            }
        }
        let status = if transport {
            "validated_review_only"
        } else if quarantined {
            "quarantined_review_only"
        } else if busy {
            "provider_busy"
        } else if timed_out {
            "executor_timeout"
        } else {
            "executor_failed"
        };
        fresh_json(
            &dir.join("receipt.json"),
            &json!({"schema":1,"config_sha256":self.config.identity,"job_sha256":j.identity,"job_id":j.id,"task_id":task_id,"task_file_sha256":j.task.sha256,"task_sha256":typed_task_sha256,"profile_file_sha256":j.profile.sha256,"annotation_gold":false,"transport_eligible":transport,"status":status,"exit_code":exit.code(),"timed_out":timed_out,"receipt_error":receipt_error,"executor_receipt_sha256":regular(&rpath).ok().map(|b|hash(&b)),"stdout_sha256":hash(&regular(&dir.join("executor.stdout"))?),"stderr_sha256":hash(&regular(&dir.join("executor.stderr"))?),"finished_unix":clock()?,"provider_busy_message":busy_message,"review_only":true,"training_authorized":false,"semantic_correctness":"not established"}),
        )?;
        if busy {
            let time = clock()?;
            let line = match busy_history(&dir, &serde_json::from_slice(&regular(&dir.join("reservation.json"))?)?, &serde_json::from_slice(&regular(&dir.join("receipt.json"))?)?) {
                Ok(true) => self.retire_busy(&j.id, time)?,
                _ => {
                    return self.report(
                        "uncertain",
                        time,
                        json!({"job_id":j.id,"automatic_retry":false,"reason":"unverifiable provider-busy attempt"}),
                    )
                }
            };
            return self.report(
                if line["held_for_review"] == true {
                    "provider_busy_held"
                } else {
                    "provider_busy_backoff"
                },
                time,
                json!({"job_id":j.id,"provider_busy_attempts":line["attempt"],"retry_after_unix":line["retry_after_unix"],"provider_busy_message":line["provider_busy_message"],"automatic_retry":line["held_for_review"] != true}),
            );
        }
        self.report(
            if transport {
                "completed_review_only"
            } else if quarantined {
                "completed_quarantined"
            } else {
                "uncertain"
            },
            clock()?,
            json!({"job_id":j.id,"automatic_retry":false,"semantic_review":if quarantined {"quarantined_not_eligible"}else{"pending"},"executor_terminal_status":status}),
        )
    }
}
pub fn status(config: &Config) -> Result<Value> {
    let p = config.root.join("status.json");
    if !p.exists() {
        return Ok(json!({"schema":1,"state":"not_started","config_sha256":config.identity}));
    }
    let mut v: Value = serde_json::from_slice(&regular(&p)?)?;
    if v["config_sha256"] != config.identity {
        return Err("status config mismatch".into());
    }
    let f = OpenOptions::new()
        .write(true)
        .custom_flags(libc::O_NOFOLLOW)
        .open(config.root.join("lease"))?;
    let available = unsafe { libc::flock(f.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } == 0;
    v["lease_held"] = json!(!available);
    v["status_is_snapshot"] = json!(true);
    Ok(v)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        cell::Cell,
        sync::{
            atomic::{AtomicU64, Ordering},
            OnceLock,
        },
    };
    static NEXT: AtomicU64 = AtomicU64::new(0);
    static FAKE: OnceLock<PathBuf> = OnceLock::new();
    fn fake() -> PathBuf {
        FAKE.get_or_init(||{
            let p=std::env::temp_dir().join(format!("fish-swarm-fake-{}",std::process::id()));
            fs::create_dir(&p).unwrap();
            let source=p.join("fake.rs");
            fs::write(&source,r#"
use std::{fs,path::Path,io::Write,os::unix::process::CommandExt,process::Command};
fn main(){
 let a:Vec<String>=std::env::args().collect(); assert_eq!(a[1],"annotation-execute");
 let base=fs::read_to_string(&a[3]).unwrap(); let base=Path::new(base.trim());
 fs::OpenOptions::new().append(true).create(true).open(base.join("calls")).unwrap().write_all(b"call\n").unwrap();
 fs::write(base.join("argv"),a.join("\n")).unwrap();
 let mode=fs::read_to_string(base.join("mode")).unwrap();
 if mode=="sleep"{std::thread::sleep(std::time::Duration::from_millis(300));}
 if mode=="pause"{fs::write(base.join("runtime/PAUSE"),"fixture").unwrap();}
 if mode=="hang"{let c=Command::new("sleep").arg("30").process_group(0).spawn().unwrap();fs::write(base.join("escaped-pid"),c.id().to_string()).unwrap();std::thread::sleep(std::time::Duration::from_secs(30));}
 if mode=="fail"{eprintln!("retained fake failure");std::process::exit(7);}
 if mode.starts_with("busy"){let o=Path::new(&a[4]);fs::create_dir(o).unwrap();fs::copy(base.join("busy.stdout"),o.join("raw.stdout")).unwrap();fs::copy(base.join("template-busy"),o.join("receipt.json")).unwrap();if mode=="busy-reply"{fs::write(o.join("reply.json"),"{}").unwrap();}std::process::exit(1);}
 fs::create_dir(&a[4]).unwrap();
 if mode=="malformed"{fs::write(Path::new(&a[4]).join("receipt.json"),"{}").unwrap();}
 else{fs::copy(base.join("template"),Path::new(&a[4]).join("receipt.json")).unwrap();}
 if mode=="invalid"{std::process::exit(1);}
}
"#).unwrap();
            let exe=p.join("executor");
            assert!(Command::new("rustc").args(["--edition","2021"]).arg(source).arg("-o").arg(&exe).status().unwrap().success());
            exe
        }).clone()
    }
    struct Fixture {
        base: PathBuf,
        config: Config,
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.base);
        }
    }
    impl Fixture {
        fn new(mode: &str) -> Self {
            Self::with_deadline(mode, 100, json!({"id":"example"}))
        }
        fn with_deadline(mode: &str, deadline: u64, task_json: Value) -> Self {
            let base = std::env::temp_dir().join(format!(
                "fish-swarm-test-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::SeqCst)
            ));
            fs::create_dir(&base).unwrap();
            let task = base.join("task");
            fs::write(&task, serde_json::to_vec_pretty(&task_json).unwrap()).unwrap();
            let profile = base.join("profile");
            fs::write(&profile, base.to_str().unwrap()).unwrap();
            let source = base.join("source");
            fs::write(&source, b"source").unwrap();
            let task_hash = hash(&fs::read(&task).unwrap());
            fs::write(base.join("template"),serde_json::to_vec(&json!({"schema":1,"task_id":"example","task_file_sha256":task_hash,"task_sha256":hash(br#"{"id":"example"}"#),"profile_file_sha256":hash(&fs::read(&profile).unwrap()),"model_process_started":true,"model_process_reaped":true,"model_process_group_absent":true,"exit_code":0,"replay_authorized":false,"review_only":true,"training_authorized":false,"status":"validated_review_only","semantic_correctness":"not established"})).unwrap()).unwrap();
            let busy = "{\"type\":\"thread.started\"}\n{\"type\":\"turn.started\"}\n{\"type\":\"error\",\"message\":\"Selected model is at capacity. Please try a different model.\"}\n{\"type\":\"turn.failed\",\"error\":{\"message\":\"Selected model is at capacity. Please try a different model.\"}}\n";
            fs::write(base.join("busy.stdout"), busy).unwrap();
            let mut t: Value =
                serde_json::from_slice(&fs::read(base.join("template")).unwrap()).unwrap();
            t["status"] = json!("provider_busy");
            t["exit_code"] = json!(1);
            t["provider_busy_message"] =
                json!("Selected model is at capacity. Please try a different model.");
            t["raw_output_hashes"] = json!({"stdout_sha256":hash(busy.as_bytes())});
            fs::write(base.join("template-busy"), serde_json::to_vec(&t).unwrap()).unwrap();
            fs::write(base.join("mode"), mode).unwrap();
            let pin = |p: &Path| json!({"path":p,"sha256":hash(&fs::read(p).unwrap())});
            let job = json!({"schema":1,"id":"one","task":pin(&task),"profile":pin(&profile),"source_pins":[pin(&source)],"provenance":{"purpose":"fake transport check"}});
            let queue = base.join("queue");
            fs::write(&queue, format!("{}\n", job)).unwrap();
            let exe = fake();
            let c = json!({"schema":1,"root":base.join("runtime"),"queue":queue,"queue_sha256":hash(&fs::read(&queue).unwrap()),"fish_executable":exe,"fish_executable_sha256":hash(&fs::read(&exe).unwrap()),"not_after_unix":deadline,"timeout_seconds":1,"poll_seconds":1});
            let cp = base.join("config");
            fs::write(&cp, serde_json::to_vec(&c).unwrap()).unwrap();
            Self {
                base,
                config: Config::load(&cp).unwrap(),
            }
        }
        fn calls(&self) -> usize {
            fs::read_to_string(self.base.join("calls"))
                .map(|s| s.lines().count())
                .unwrap_or(0)
        }
    }
    #[test]
    fn provider_busy_requeues_with_bounded_backoff_then_holds() -> Result<()> {
        let f = Fixture::with_deadline("busy", 10_000, json!({"id":"example","target_ids":["s1"]}));
        let s = Supervisor::open(f.config.clone())?;
        let v = s.step(|| Ok(1))?;
        assert_eq!(v["state"], "provider_busy_backoff");
        assert_eq!(v["no_unresolved_reservations"], true);
        assert_eq!(v["details"]["retry_after_unix"], 61);
        assert!(!s.config.root.join("jobs/one").exists());
        assert!(s
            .config
            .root
            .join("busy/one/01/output/raw.stdout")
            .is_file());
        // Backoff: no call before the retry time; the audit stays resolved with the job pending.
        assert_eq!(s.step(|| Ok(60))?["state"], "provider_busy_backoff");
        assert_eq!(f.calls(), 1);
        let a = audit_history(&s.config)?;
        assert_eq!(
            (
                a["no_unresolved_reservations"].clone(),
                a["provider_busy_attempts"].clone(),
                a["jobs_terminal"].clone()
            ),
            (json!(true), json!(1), json!(0))
        );
        assert_eq!(s.step(|| Ok(61))?["details"]["retry_after_unix"], 181);
        assert_eq!(s.step(|| Ok(181))?["details"]["retry_after_unix"], 421);
        assert_eq!(s.step(|| Ok(420))?["state"], "provider_busy_backoff");
        let v = s.step(|| Ok(421))?;
        assert_eq!(v["state"], "provider_busy_held");
        assert_eq!(v["details"]["automatic_retry"], false);
        assert_eq!(s.step(|| Ok(5000))?["state"], "provider_busy_held");
        assert_eq!(f.calls(), 4);
        let journal = fs::read_to_string(s.config.root.join("provider-busy.jsonl"))?;
        let lines: Vec<Value> = journal
            .lines()
            .map(|l| serde_json::from_str(l).unwrap())
            .collect();
        assert_eq!(lines.len(), 4);
        assert!(lines
            .iter()
            .all(|l| l["uncertain"] == false && l["usage_limit"] == false));
        assert_eq!(lines[3]["held_for_review"], true);
        // A successful retry after one busy refusal completes the job normally.
        let f = Fixture::with_deadline("busy", 10_000, json!({"id":"example","target_ids":["s1"]}));
        let s = Supervisor::open(f.config.clone())?;
        assert_eq!(s.step(|| Ok(1))?["state"], "provider_busy_backoff");
        fs::write(f.base.join("mode"), "ok")?;
        assert_eq!(s.step(|| Ok(61))?["state"], "completed_review_only");
        assert_eq!(s.step(|| Ok(62))?["state"], "queue_exhausted");
        let a = audit_history(&s.config)?;
        assert_eq!(
            (
                a["validated_jobs"].clone(),
                a["provider_busy_attempts"].clone()
            ),
            (json!(1), json!(1))
        );
        Ok(())
    }
    #[test]
    fn unprovable_busy_and_tampered_busy_history_stay_uncertain() -> Result<()> {
        // Red control: a busy-looking attempt that wrote a reply is not "nothing done".
        let f = Fixture::with_deadline(
            "busy-reply",
            10_000,
            json!({"id":"example","target_ids":["s1"]}),
        );
        let s = Supervisor::open(f.config.clone())?;
        assert_eq!(s.step(|| Ok(1))?["state"], "uncertain");
        assert_eq!(s.step(|| Ok(100))?["state"], "uncertain");
        assert_eq!(f.calls(), 1);
        // Red control: an ordinary failed child (e.g. a usage limit) stays uncertain, never requeued.
        let f = Fixture::with_deadline("fail", 10_000, json!({"id":"example","target_ids":["s1"]}));
        let s = Supervisor::open(f.config.clone())?;
        assert_eq!(s.step(|| Ok(1))?["state"], "uncertain");
        assert!(!s.config.root.join("busy").exists());
        // Red control: tampering with a retained busy attempt makes the history unresolved.
        let f = Fixture::with_deadline("busy", 10_000, json!({"id":"example","target_ids":["s1"]}));
        let s = Supervisor::open(f.config.clone())?;
        assert_eq!(s.step(|| Ok(1))?["state"], "provider_busy_backoff");
        fs::write(s.config.root.join("busy/one/01/output/raw.stdout"), "{}\n")?;
        assert_eq!(s.step(|| Ok(100))?["state"], "uncertain");
        assert_eq!(
            audit_history(&s.config)?["no_unresolved_reservations"],
            false
        );
        assert_eq!(f.calls(), 1);
        Ok(())
    }
    #[test]
    fn expiration_pause_stop_make_zero_calls() -> Result<()> {
        let f = Fixture::new("ok");
        let s = Supervisor::open(f.config.clone())?;
        assert_eq!(s.step(|| Ok(100))?["state"], "deadline_reached");
        fresh(&s.config.root.join("PAUSE"), b"")?;
        assert_eq!(s.step(|| Ok(1))?["state"], "paused");
        fresh(&s.config.root.join("STOP"), b"")?;
        assert_eq!(s.step(|| Ok(1))?["state"], "stopped");
        assert_eq!(f.calls(), 0);
        Ok(())
    }
    #[test]
    fn active_completion_may_cross_deadline_but_never_repeat() -> Result<()> {
        let f = Fixture::new("sleep");
        let s = Supervisor::open(f.config.clone())?;
        let n = Cell::new(0);
        let v = s.step(|| {
            let x = n.get();
            n.set(x + 1);
            Ok(if x < 5 { 99 } else { 101 })
        })?;
        assert_eq!(v["state"], "completed_review_only");
        assert_eq!(f.calls(), 1);
        assert_eq!(s.step(|| Ok(101))?["state"], "deadline_reached");
        assert_eq!(s.step(|| Ok(99))?["state"], "queue_exhausted");
        assert_eq!(f.calls(), 1);
        let args = fs::read_to_string(f.base.join("argv"))?;
        assert!(args.ends_with("100\n1"));
        let r: Value =
            serde_json::from_slice(&fs::read(s.config.root.join("jobs/one/receipt.json"))?)?;
        assert_eq!(r["training_authorized"], false);
        assert_eq!(r["semantic_correctness"], "not established");
        Ok(())
    }
    #[test]
    fn boundary_rechecked_after_pins_and_after_reservation() -> Result<()> {
        for cross in [1, 2] {
            let f = Fixture::new("ok");
            let s = Supervisor::open(f.config.clone())?;
            let n = Cell::new(0);
            assert_eq!(
                s.step(|| {
                    let x = n.get();
                    n.set(x + 1);
                    Ok(if x < cross { 99 } else { 100 })
                })?["state"],
                "deadline_reached"
            );
            assert_eq!(f.calls(), 0);
            assert_eq!(
                s.config.root.join("jobs/one/reservation.json").exists(),
                cross == 2
            );
            if cross == 2 {
                assert_eq!(s.step(|| Ok(99))?["state"], "queue_exhausted");
            }
        }
        Ok(())
    }
    #[test]
    fn lease_and_config_identity_exclude_second_owner() -> Result<()> {
        let f = Fixture::new("ok");
        let s = Supervisor::open(f.config.clone())?;
        assert!(Supervisor::open(f.config.clone()).is_err());
        let mut c = f.config.clone();
        c.identity = hash(b"different config");
        drop(s);
        assert!(Supervisor::open(c).is_err());
        Ok(())
    }
    #[test]
    fn uncertain_reservation_is_never_replayed() -> Result<()> {
        let f = Fixture::new("ok");
        let s = Supervisor::open(f.config.clone())?;
        let orphan = s.config.root.join("jobs/removed-from-queue");
        fs::create_dir(&orphan)?;
        fresh_json(
            &orphan.join("reservation.json"),
            &json!({"note":"possible accepted child"}),
        )?;
        assert_eq!(s.step(|| Ok(1))?["state"], "uncertain");
        fresh(&s.config.root.join("PAUSE"), b"")?;
        assert_eq!(s.step(|| Ok(1))?["state"], "uncertain");
        fresh(&s.config.root.join("STOP"), b"")?;
        let v = s.step(|| Ok(100))?;
        assert_eq!(v["state"], "uncertain");
        assert_eq!(v["active_model_call"], Value::Null);
        assert_eq!(v["no_unresolved_reservations"], false);
        assert_eq!(f.calls(), 0);
        Ok(())
    }
    #[test]
    fn failed_or_malformed_output_blocks_next_attempt() -> Result<()> {
        for mode in ["fail", "malformed"] {
            let f = Fixture::new(mode);
            let s = Supervisor::open(f.config.clone())?;
            assert_eq!(s.step(|| Ok(1))?["state"], "uncertain");
            assert_eq!(s.step(|| Ok(2))?["state"], "uncertain");
            assert_eq!(f.calls(), 1);
            if mode == "fail" {
                assert!(
                    fs::read_to_string(s.config.root.join("jobs/one/executor.stderr"))?
                        .contains("retained fake failure")
                );
            }
        }
        Ok(())
    }
    #[test]
    fn pause_waits_for_accepted_completion_and_hung_executor_is_bounded() -> Result<()> {
        let f = Fixture::new("pause");
        let s = Supervisor::open(f.config.clone())?;
        assert_eq!(s.step(|| Ok(1))?["state"], "completed_review_only");
        let v = s.step(|| Ok(2))?;
        assert_eq!(v["state"], "paused");
        assert_eq!(v["active_model_call"], false);
        assert_eq!(v["no_unresolved_reservations"], true);
        assert_eq!(f.calls(), 1);
        let f = Fixture::new("hang");
        let s = Supervisor::open(f.config.clone())?;
        let start = Instant::now();
        let v = s.step(|| Ok(1))?;
        assert_eq!(v["state"], "uncertain");
        assert_eq!(v["active_model_call"], Value::Null);
        assert_eq!(v["no_unresolved_reservations"], false);
        assert!(start.elapsed() < Duration::from_secs(15));
        let r: Value =
            serde_json::from_slice(&fs::read(s.config.root.join("jobs/one/receipt.json"))?)?;
        assert_eq!(r["status"], "executor_timeout");
        assert_eq!(s.step(|| Ok(2))?["state"], "uncertain");
        fresh(&s.config.root.join("STOP"), b"")?;
        assert_eq!(s.step(|| Ok(100))?["state"], "uncertain");
        drop(s);
        let restarted = Supervisor::open(f.config.clone())?;
        let v = restarted.step(|| Ok(100))?;
        assert_eq!(v["state"], "uncertain");
        assert_eq!(v["active_model_call"], Value::Null);
        let escaped_pid = fs::read_to_string(f.base.join("escaped-pid"))?.parse::<i32>()?;
        let escaped_exists = unsafe { libc::kill(escaped_pid, 0) } == 0;
        let proc_stat = fs::read_to_string(format!("/proc/{escaped_pid}/stat"))?;
        let state = proc_stat
            .rsplit_once(") ")
            .ok_or("missing process state")?
            .1
            .chars()
            .next()
            .ok_or("empty process state")?;
        let escaped_still_live = escaped_exists && !matches!(state, 'Z' | 'X' | 'x');
        unsafe {
            libc::kill(-escaped_pid, libc::SIGKILL);
        }
        assert!(
            escaped_still_live,
            "outer group termination must not imply separately grouped child death"
        );
        assert_eq!(f.calls(), 1);
        Ok(())
    }
    #[test]
    fn mutated_source_queue_and_executable_are_rejected_before_reserve() -> Result<()> {
        for name in ["source", "queue"] {
            let f = Fixture::new("ok");
            let s = Supervisor::open(f.config.clone())?;
            fs::write(f.base.join(name), b"changed")?;
            assert!(s.step(|| Ok(1)).is_err());
            assert_eq!(f.calls(), 0);
            assert!(!s.config.root.join("jobs/one").exists());
        }
        let f = Fixture::new("ok");
        let mut c = f.config.clone();
        c.executable.sha256 = hash(b"wrong");
        let s = Supervisor::open(c)?;
        assert!(s.step(|| Ok(1)).is_err());
        assert_eq!(f.calls(), 0);
        Ok(())
    }
    #[test]
    fn duplicate_ids_and_wrong_receipt_binding_are_rejected() -> Result<()> {
        let f = Fixture::new("ok");
        let raw = fs::read(&f.config.queue.path)?;
        let mut twice = raw.clone();
        twice.extend_from_slice(&raw);
        fs::write(&f.config.queue.path, &twice)?;
        let mut c = f.config.clone();
        c.queue.sha256 = hash(&twice);
        assert!(c.jobs().is_err());
        let f = Fixture::new("ok");
        let s = Supervisor::open(f.config.clone())?;
        let mut r: Value = serde_json::from_slice(&fs::read(f.base.join("template"))?)?;
        r["task_file_sha256"] = json!(hash(b"wrong"));
        fs::write(f.base.join("template"), serde_json::to_vec(&r)?)?;
        assert_eq!(s.step(|| Ok(1))?["state"], "uncertain");
        assert_eq!(f.calls(), 1);
        Ok(())
    }
    #[test]
    fn known_closed_invalid_response_quarantines_then_advances_without_replay() -> Result<()> {
        let mut f = Fixture::new("invalid");
        let task2 = f.base.join("task-two");
        fs::write(&task2, serde_json::to_vec_pretty(&json!({"id":"second"}))?)?;
        let mut job2: Value = serde_json::from_slice(&fs::read(&f.config.queue.path)?)?;
        job2["id"] = json!("two");
        job2["task"] = json!({"path":task2,"sha256":hash(&fs::read(&task2)?)});
        let mut queue = fs::read(&f.config.queue.path)?;
        queue.extend_from_slice(format!("{}\n", job2).as_bytes());
        fs::write(&f.config.queue.path, &queue)?;
        let cp = f.base.join("config");
        let mut c: Value = serde_json::from_slice(&fs::read(&cp)?)?;
        c["queue_sha256"] = json!(hash(&queue));
        fs::write(&cp, serde_json::to_vec(&c)?)?;
        f.config = Config::load(&cp)?;
        let mut inner: Value = serde_json::from_slice(&fs::read(f.base.join("template"))?)?;
        inner["status"] = json!("invalid_response");
        fs::write(f.base.join("template"), serde_json::to_vec(&inner)?)?;
        let s = Supervisor::open(f.config.clone())?;
        let v = s.step(|| Ok(1))?;
        assert_eq!(v["state"], "completed_quarantined");
        assert_eq!(v["active_model_call"], false);
        assert_eq!(v["no_unresolved_reservations"], true);
        let outer: Value =
            serde_json::from_slice(&fs::read(s.config.root.join("jobs/one/receipt.json"))?)?;
        assert_eq!(outer["status"], "quarantined_review_only");
        assert_eq!(outer["exit_code"], 1);
        assert_eq!(outer["annotation_gold"], false);
        assert_eq!(outer["transport_eligible"], false);
        fs::write(f.base.join("mode"), "ok")?;
        inner["status"] = json!("validated_review_only");
        inner["task_id"] = json!("second");
        inner["task_file_sha256"] = json!(hash(&fs::read(task2)?));
        inner["task_sha256"] = json!(hash(br#"{"id":"second"}"#));
        fs::write(f.base.join("template"), serde_json::to_vec(&inner)?)?;
        assert_eq!(s.step(|| Ok(2))?["state"], "completed_review_only");
        assert_eq!(s.step(|| Ok(3))?["state"], "queue_exhausted");
        assert_eq!(f.calls(), 2);
        drop(s);
        let s = Supervisor::open(f.config.clone())?;
        fresh(&s.config.root.join("STOP"), b"")?;
        assert_eq!(s.step(|| Ok(100))?["state"], "stopped");
        assert_eq!(f.calls(), 2);
        Ok(())
    }
    #[test]
    fn incomplete_or_unbound_closure_proof_never_quarantines() -> Result<()> {
        for (key, value) in [
            ("model_process_reaped", json!(false)),
            ("model_process_reaped", Value::Null),
            ("model_process_group_absent", json!(false)),
            ("model_process_group_absent", Value::Null),
            ("model_process_started", Value::Null),
            ("profile_file_sha256", json!(hash(b"wrong profile"))),
            ("task_file_sha256", json!(hash(b"wrong task"))),
            ("exit_code", json!(7)),
            ("replay_authorized", json!(true)),
            ("status", json!("timed_out")),
        ] {
            let f = Fixture::new("invalid");
            let mut inner: Value = serde_json::from_slice(&fs::read(f.base.join("template"))?)?;
            inner["status"] = json!("invalid_response");
            inner[key] = value;
            fs::write(f.base.join("template"), serde_json::to_vec(&inner)?)?;
            let s = Supervisor::open(f.config.clone())?;
            let v = s.step(|| Ok(1))?;
            assert_eq!(v["state"], "uncertain", "proof field {key}");
            assert_eq!(v["active_model_call"], Value::Null);
            fresh(&s.config.root.join("STOP"), b"")?;
            assert_eq!(s.step(|| Ok(100))?["state"], "uncertain");
            assert_eq!(f.calls(), 1);
        }
        Ok(())
    }
    #[test]
    fn retained_quarantine_rechecks_underlying_proof_and_copied_profile() -> Result<()> {
        for drift_profile in [false, true] {
            let f = Fixture::new("invalid");
            let mut inner: Value = serde_json::from_slice(&fs::read(f.base.join("template"))?)?;
            inner["status"] = json!("invalid_response");
            fs::write(f.base.join("template"), serde_json::to_vec(&inner)?)?;
            let s = Supervisor::open(f.config.clone())?;
            assert_eq!(s.step(|| Ok(1))?["state"], "completed_quarantined");
            let dir = s.config.root.join("jobs/one");
            if drift_profile {
                fs::write(dir.join("profile.json"), "changed profile")?;
            } else {
                inner["model_process_group_absent"] = json!(false);
                let b = serde_json::to_vec(&inner)?;
                fs::write(dir.join("output/receipt.json"), &b)?;
                let mut outer: Value =
                    serde_json::from_slice(&fs::read(dir.join("receipt.json"))?)?;
                outer["executor_receipt_sha256"] = json!(hash(&b));
                fs::write(dir.join("receipt.json"), serde_json::to_vec(&outer)?)?;
            }
            fresh(&s.config.root.join("STOP"), b"")?;
            assert_eq!(s.step(|| Ok(100))?["state"], "uncertain");
            assert_eq!(f.calls(), 1);
        }
        Ok(())
    }
    #[test]
    fn pretty_raw_task_hash_and_typed_commitment_are_distinct() -> Result<()> {
        let f = Fixture::new("ok");
        let r: Value = serde_json::from_slice(&fs::read(f.base.join("template"))?)?;
        assert_ne!(r["task_file_sha256"], r["task_sha256"]);
        assert_eq!(r["task_file_sha256"], f.config.jobs()?[0].task.sha256);
        let s = Supervisor::open(f.config.clone())?;
        assert_eq!(s.step(|| Ok(1))?["state"], "completed_review_only");
        let outer: Value =
            serde_json::from_slice(&fs::read(s.config.root.join("jobs/one/receipt.json"))?)?;
        assert_eq!(outer["task_file_sha256"], r["task_file_sha256"]);
        assert_eq!(outer["task_sha256"], r["task_sha256"]);
        let f = Fixture::new("ok");
        let mut r: Value = serde_json::from_slice(&fs::read(f.base.join("template"))?)?;
        r["task_sha256"] = json!("not-a-typed-sha256");
        fs::write(f.base.join("template"), serde_json::to_vec(&r)?)?;
        let s = Supervisor::open(f.config.clone())?;
        assert_eq!(s.step(|| Ok(1))?["state"], "uncertain");
        assert_eq!(f.calls(), 1);
        Ok(())
    }
}

/// Read-only bounded history audit; never reserves work or starts a child.
pub fn audit_history(config: &Config) -> Result<Value> {
    let identity: Value =
        serde_json::from_slice(&regular(&config.root.join("config-identity.json"))?)?;
    if identity["config_sha256"] != config.identity {
        return Err("audit root identity mismatch".into());
    }
    let jobs = config.jobs()?;
    let mut expected = std::collections::BTreeMap::new();
    for job in &jobs {
        expected.insert(job.id.clone(), job);
    }
    let mut pins = Vec::new();
    let mut finished = BTreeSet::new();
    let (mut valid_jobs, mut quarantine_jobs, mut not_started_jobs) = (0usize, 0usize, 0usize);
    let (mut valid_targets, mut quarantine_targets, mut pending_targets) = (0usize, 0usize, 0usize);
    for entry in fs::read_dir(config.root.join("jobs"))? {
        let p = entry?.path();
        let id = p
            .file_name()
            .and_then(|s| s.to_str())
            .ok_or("invalid history directory")?;
        let Some(job) = expected.get(id) else {
            return Ok(
                json!({"no_unresolved_reservations":false,"uncertain_job":id,"reason":"foreign retained job"}),
            );
        };
        let checked = (|| -> Result<(Value, Value)> {
            let rb = regular(&p.join("reservation.json"))?;
            let cb = regular(&p.join("receipt.json"))?;
            let r: Value = serde_json::from_slice(&rb)?;
            let c: Value = serde_json::from_slice(&cb)?;
            if r["config_sha256"] != config.identity
                || c["config_sha256"] != config.identity
                || r["job_sha256"] != job.identity
                || c["job_sha256"] != job.identity
                || c["training_authorized"] != false
            {
                return Err("retained audit binding mismatch".into());
            }
            let closed = match c["status"].as_str() {
                Some("not_started_boundary" | "spawn_failed") => true,
                Some("validated_review_only") => {
                    c["exit_code"] == 0 && closed_history(&p, &r, &c, "validated_review_only")?
                }
                Some("quarantined_review_only") => closed_history(&p, &r, &c, "invalid_response")?,
                _ => false,
            };
            if !closed {
                return Err("unclosed retained attempt".into());
            }
            Ok((c, json!({"path":p.join("receipt.json"),"sha256":hash(&cb)})))
        })();
        let Ok((receipt, pin)) = checked else {
            return Ok(
                json!({"no_unresolved_reservations":false,"uncertain_job":id,"reason":"unclosed or malformed retained attempt"}),
            );
        };
        finished.insert(id.to_owned());
        pins.push(pin);
        let t: Value = serde_json::from_slice(&job.task.read()?)?;
        let n = t["target_ids"]
            .as_array()
            .ok_or("target IDs missing")?
            .len();
        match receipt["status"].as_str() {
            Some("validated_review_only") => {
                valid_jobs += 1;
                valid_targets += n;
            }
            Some("quarantined_review_only") => {
                quarantine_jobs += 1;
                quarantine_targets += n;
            }
            _ => {
                not_started_jobs += 1;
                pending_targets += n;
            }
        }
    }
    let mut busy_total = 0usize;
    let busy_root = config.root.join("busy");
    if busy_root.exists() {
        for entry in fs::read_dir(&busy_root)? {
            let id = entry?.file_name().to_string_lossy().into_owned();
            let Some(job) = expected.get(id.as_str()) else {
                return Ok(
                    json!({"no_unresolved_reservations":false,"uncertain_job":id,"reason":"foreign provider-busy attempt"}),
                );
            };
            match busy_attempts(&config.root, &config.identity, job) {
                Ok(f) => busy_total += f.len(),
                Err(_) => {
                    return Ok(
                        json!({"no_unresolved_reservations":false,"uncertain_job":id,"reason":"unverifiable provider-busy attempt"}),
                    )
                }
            }
        }
    }
    for job in &jobs {
        if !finished.contains(&job.id) {
            let t: Value = serde_json::from_slice(&job.task.read()?)?;
            pending_targets += t["target_ids"]
                .as_array()
                .ok_or("target IDs missing")?
                .len();
        }
    }
    pins.sort_by(|a, b| a["path"].as_str().cmp(&b["path"].as_str()));
    Ok(
        json!({"schema":1,"config_sha256":config.identity,"no_unresolved_reservations":true,
        "jobs_expected":jobs.len(),"jobs_terminal":finished.len(),"validated_jobs":valid_jobs,
        "quarantined_jobs":quarantine_jobs,"not_started_jobs":not_started_jobs,
        "validated_targets":valid_targets,"quarantined_targets":quarantine_targets,
        "pending_targets":pending_targets,"provider_busy_attempts":busy_total,"receipt_pins":pins,"review_only":true,"training_authorized":false}),
    )
}
