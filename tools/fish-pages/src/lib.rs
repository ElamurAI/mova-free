//! Bounded immutable annotation pages with durable source-ID accounting.
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    os::{fd::AsRawFd, unix::fs::OpenOptionsExt},
    path::{Path, PathBuf},
    sync::{mpsc, Arc},
    thread,
    time::Duration,
};
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub const KIND: &str = "mova_fish_pages";
pub fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn digest(s: &str) -> bool {
    s.len() == 64
        && s.bytes()
            .all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b))
}
fn field<'a>(v: &'a Value, k: &str) -> Result<&'a str> {
    v[k].as_str()
        .ok_or_else(|| format!("missing string {k}").into())
}
fn number(v: &Value, k: &str) -> Result<u64> {
    v[k].as_u64()
        .ok_or_else(|| format!("missing integer {k}").into())
}
fn keys(v: &Value, allowed: &[&str]) -> Result<()> {
    if v.as_object()
        .ok_or("object required")?
        .keys()
        .any(|k| !allowed.contains(&k.as_str()))
    {
        return Err("unknown field".into());
    }
    Ok(())
}
fn absolute(s: &str) -> Result<PathBuf> {
    let p = PathBuf::from(s);
    if !p.is_absolute()
        || p.components().any(|c| {
            matches!(
                c,
                std::path::Component::ParentDir | std::path::Component::CurDir
            )
        })
    {
        return Err("absolute canonical path required".into());
    }
    Ok(p)
}
fn regular(p: &Path) -> Result<Vec<u8>> {
    if fs::canonicalize(p)? != p {
        return Err("symlink or noncanonical input".into());
    }
    let mut f = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW)
        .open(p)?;
    if !f.metadata()?.is_file() || f.metadata()?.len() > 64 * 1024 * 1024 {
        return Err("bounded regular file required".into());
    }
    let mut b = vec![];
    Read::by_ref(&mut f)
        .take(64 * 1024 * 1024 + 1)
        .read_to_end(&mut b)?;
    if b.len() > 64 * 1024 * 1024 {
        return Err("input grew".into());
    }
    Ok(b)
}
fn read_json(p: &Path) -> Result<Value> {
    Ok(serde_json::from_slice(&regular(p)?)?)
}
fn fresh(p: &Path, b: &[u8]) -> Result<()> {
    let mut f = OpenOptions::new()
        .write(true)
        .create_new(true)
        .custom_flags(libc::O_NOFOLLOW)
        .open(p)?;
    f.write_all(b)?;
    f.sync_all()?;
    File::open(p.parent().ok_or("parent")?)?.sync_all()?;
    Ok(())
}
fn fresh_json(p: &Path, v: &Value) -> Result<()> {
    fresh(p, &serde_json::to_vec_pretty(v)?)
}
fn atomic(p: &Path, v: &Value) -> Result<()> {
    let t = p.with_extension("next");
    if t.exists() {
        fs::remove_file(&t)?
    }
    fresh_json(&t, v)?;
    fs::rename(&t, p)?;
    File::open(p.parent().ok_or("parent")?)?.sync_all()?;
    Ok(())
}
#[derive(Clone, Debug)]
pub struct Pin {
    pub path: PathBuf,
    pub sha256: String,
}
impl Pin {
    fn parse(v: &Value) -> Result<Self> {
        keys(v, &["path", "sha256"])?;
        let h = field(v, "sha256")?;
        if !digest(h) {
            return Err("invalid SHA256".into());
        }
        Ok(Self {
            path: absolute(field(v, "path")?)?,
            sha256: h.into(),
        })
    }
    fn read(&self) -> Result<Vec<u8>> {
        let b = regular(&self.path)?;
        if hash(&b) != self.sha256 {
            return Err(format!("pin mismatch {}", self.path.display()).into());
        }
        Ok(b)
    }
    fn value(&self) -> Value {
        json!({"path":self.path,"sha256":self.sha256})
    }
}
#[cfg(test)]
fn pin(p: &Path) -> Result<Pin> {
    Ok(Pin {
        path: p.into(),
        sha256: hash(&regular(p)?),
    })
}

#[derive(Clone, Debug)]
pub struct Config {
    pub root: PathBuf,
    pub plan: Pin,
    pub ready_root: PathBuf,
    pub fish_executable: Pin,
    pub not_after_unix: u64,
    pub timeout_seconds: u64,
    pub poll_seconds: u64,
    pub identity: String,
    bytes: Vec<u8>,
}
impl Config {
    pub fn load(path: &Path) -> Result<Self> {
        let bytes = regular(path)?;
        let v: Value = serde_json::from_slice(&bytes)?;
        keys(
            &v,
            &[
                "schema",
                "root",
                "plan",
                "ready_root",
                "fish_executable",
                "not_after_unix",
                "timeout_seconds",
                "poll_seconds",
            ],
        )?;
        if number(&v, "schema")? != 1 {
            return Err("config schema".into());
        }
        let timeout = v["timeout_seconds"].as_u64().unwrap_or(300);
        let poll = v["poll_seconds"].as_u64().unwrap_or(5);
        if !(1..=900).contains(&timeout) || !(1..=60).contains(&poll) {
            return Err("timeout/poll bounds".into());
        }
        Ok(Self {
            root: absolute(field(&v, "root")?)?,
            plan: Pin::parse(&v["plan"])?,
            ready_root: absolute(field(&v, "ready_root")?)?,
            fish_executable: Pin::parse(&v["fish_executable"])?,
            not_after_unix: number(&v, "not_after_unix")?,
            timeout_seconds: timeout,
            poll_seconds: poll,
            identity: hash(&bytes),
            bytes,
        })
    }
}
#[derive(Clone, Debug)]
struct Intent {
    pin: Pin,
    id: String,
    sources: BTreeSet<String>,
}
fn plans(c: &Config) -> Result<Vec<Intent>> {
    let v: Value = serde_json::from_slice(&c.plan.read()?)?;
    keys(&v, &["schema", "pages"])?;
    if v["schema"] != 1 {
        return Err("plan schema".into());
    }
    let pages = v["pages"].as_array().ok_or("plan pages")?;
    if pages.len() > 100000 {
        return Err("plan page cap".into());
    }
    let mut out = vec![];
    let mut ids = BTreeSet::new();
    let mut all = BTreeSet::new();
    for p in pages {
        let pin = Pin::parse(p)?;
        let i: Value = serde_json::from_slice(&pin.read()?)?;
        keys(&i, &["schema", "id", "sources"])?;
        let id = field(&i, "id")?;
        if i["schema"] != 1
            || id.is_empty()
            || id.len() > 64
            || !id
                .bytes()
                .all(|b| b.is_ascii_alphanumeric() || b"_-".contains(&b))
            || !ids.insert(id.to_owned())
        {
            return Err("unsafe/duplicate intent id".into());
        }
        let s = i["sources"].as_array().ok_or("intent sources")?;
        if s.is_empty() || s.len() > 256 {
            return Err("intent1..256sources".into());
        }
        let mut sources = BTreeSet::new();
        for x in s {
            let x = x.as_str().ok_or("source id string")?;
            if x.is_empty()
                || x.len() > 1024
                || !sources.insert(x.to_owned())
                || !all.insert(x.to_owned())
            {
                return Err("duplicate/invalid source ownership".into());
            }
        }
        out.push(Intent {
            pin,
            id: id.into(),
            sources,
        });
    }
    Ok(out)
}
fn child_config(c: &Config, i: &Intent, ready: &Value) -> Result<PathBuf> {
    let dir = c.root.join("pages").join(&i.id);
    fs::create_dir_all(&dir)?;
    let queue = Pin::parse(&ready["queue"])?;
    let v = json!({"schema":1,"root":dir.join("run"),"queue":queue.path,"queue_sha256":queue.sha256,
 "fish_executable":c.fish_executable.path,"fish_executable_sha256":c.fish_executable.sha256,
 "not_after_unix":c.not_after_unix,"timeout_seconds":c.timeout_seconds,"poll_seconds":c.poll_seconds.min(5)});
    let path = dir.join("swarm.json");
    let b = serde_json::to_vec_pretty(&v)?;
    if path.exists() {
        if regular(&path)? != b {
            return Err("existing page config differs".into());
        }
    } else {
        fresh(&path, &b)?
    }
    Ok(path)
}
#[derive(Clone, Debug)]
struct Admission {
    pin: Pin,
    ready: Value,
    deferred: usize,
    rejected: usize,
    queued: usize,
}
fn admit(c: &Config, i: &Intent) -> Result<Admission> {
    let path = c.ready_root.join(format!("{}.json", i.id));
    let bytes = regular(&path)?;
    let r: Value = serde_json::from_slice(&bytes)?;
    keys(&r, &["schema", "intent", "queue", "deferred", "rejected"])?;
    if r["schema"] != 1 || r["intent"] != i.pin.value() {
        return Err("ready intent mismatch".into());
    }
    let queue = Pin::parse(&r["queue"])?;
    let qb = queue.read()?;
    let mut accounted = BTreeSet::new();
    let mut queued = 0;
    for line in std::str::from_utf8(&qb)?.lines() {
        if line.is_empty() {
            return Err("blank queue line".into());
        }
        let job: Value = serde_json::from_str(line)?;
        let tp = Pin::parse(&job["task"])?;
        let t: Value = serde_json::from_slice(&tp.read()?)?;
        let targets = t["target_ids"].as_array().ok_or("task target IDs")?;
        if targets.is_empty() || targets.len() > 8 {
            return Err("target count".into());
        }
        for target in targets {
            let id = target.as_str().ok_or("target id string")?;
            if !i.sources.contains(id) || !accounted.insert(id.to_owned()) {
                return Err("foreign/duplicate queued source".into());
            }
            queued += 1;
        }
    }
    let mut gap_counts = [0, 0];
    for (n, k) in ["deferred", "rejected"].iter().enumerate() {
        let gaps = r[*k].as_array().ok_or("explicit gap array required")?;
        for g in gaps {
            keys(g, &["source_id", "reason"])?;
            let id = field(g, "source_id")?;
            let why = field(g, "reason")?;
            if why.trim().is_empty()
                || why.len() > 2048
                || !i.sources.contains(id)
                || !accounted.insert(id.into())
            {
                return Err("foreign/duplicate/unexplained gap".into());
            }
            gap_counts[n] += 1;
        }
    }
    if accounted != i.sources {
        return Err("unaccounted intent source".into());
    }
    Ok(Admission {
        pin: Pin {
            path,
            sha256: hash(&bytes),
        },
        ready: r,
        deferred: gap_counts[0],
        rejected: gap_counts[1],
        queued,
    })
}

#[derive(Clone, Default, Debug)]
struct Counts {
    valid: u64,
    quarantine: u64,
    deferred: u64,
    rejected: u64,
}
impl Counts {
    fn add(&mut self, a: &Admission, audit: &Value) {
        self.valid += audit["validated_targets"].as_u64().unwrap_or(0);
        self.quarantine += audit["quarantined_targets"].as_u64().unwrap_or(0);
        self.deferred += a.deferred as u64;
        self.rejected += a.rejected as u64;
    }
    fn value(&self) -> Value {
        json!({"validated_targets":self.valid,"quarantined_targets":self.quarantine,"deferred_targets":self.deferred,"rejected_targets":self.rejected})
    }
}
fn existing_child(c: &Config, i: &Intent, a: &Admission) -> Result<mova_fish_swarm::Config> {
    let p = c.root.join("pages").join(&i.id).join("swarm.json");
    let v = read_json(&p)?;
    let q = Pin::parse(&a.ready["queue"])?;
    if v["schema"] != 1
        || v["root"]
            != c.root
                .join("pages")
                .join(&i.id)
                .join("run")
                .to_string_lossy()
                .as_ref()
        || v["queue"] != q.path.to_string_lossy().as_ref()
        || v["queue_sha256"] != q.sha256
        || v["fish_executable"] != c.fish_executable.path.to_string_lossy().as_ref()
        || v["fish_executable_sha256"] != c.fish_executable.sha256
        || v["not_after_unix"] != c.not_after_unix
        || v["timeout_seconds"] != c.timeout_seconds
        || v["poll_seconds"] != c.poll_seconds.min(5)
    {
        return Err("child config differs from pinned page".into());
    }
    Ok(mova_fish_swarm::Config::load(&p)?)
}
fn sealed_prefix(c: &Config, intents: &[Intent]) -> Result<(usize, Counts, String)> {
    let mut n = 0;
    let mut counts = Counts::default();
    let mut previous = "0".repeat(64);
    for i in intents {
        let p = c.root.join("seals").join(format!("{}.json", i.id));
        if !p.exists() {
            break;
        }
        let b = regular(&p)?;
        let seal: Value = serde_json::from_slice(&b)?;
        let a = admit(c, i)?;
        let sc = existing_child(c, i, &a)?;
        let audit = mova_fish_swarm::audit_history(&sc)?;
        if seal["schema"] != 1
            || seal["config_sha256"] != c.identity
            || seal["plan_sha256"] != c.plan.sha256
            || seal["intent"] != i.pin.value()
            || seal["admission"] != a.pin.value()
            || seal["previous_seal_sha256"] != previous
            || seal["audit"] != audit
            || !audit_fits(&audit, &a)
            || audit["jobs_terminal"] != audit["jobs_expected"]
            || audit["not_started_jobs"] != 0
            || audit["pending_targets"] != 0
        {
            return Err("completed page seal/history mismatch".into());
        }
        counts.add(&a, &audit);
        previous = hash(&b);
        n += 1;
    }
    if c.root.join("checkpoint.json").exists() {
        let v = read_json(&c.root.join("checkpoint.json"))?;
        if v["schema"] != 1
            || v["config_sha256"] != c.identity
            || v["plan_sha256"] != c.plan.sha256
            || v["cursor"].as_u64().ok_or("checkpoint cursor")? > n as u64
        {
            return Err("checkpoint exceeds audited seals".into());
        }
        if v["cursor"] == n
            && (v["counts"] != counts.value() || v["previous_seal_sha256"] != previous)
        {
            return Err("checkpoint counters or seal hash differ".into());
        }
    }
    let order: BTreeMap<_, _> = intents
        .iter()
        .enumerate()
        .map(|(n, i)| (i.id.as_str(), n))
        .collect();
    for entry in fs::read_dir(c.root.join("pages"))? {
        let p = entry?.path();
        let id = p
            .file_name()
            .and_then(|x| x.to_str())
            .ok_or("page directory name")?;
        let Some(index) = order.get(id) else {
            return Err("foreign retained page directory".into());
        };
        if *index > n
            && p.join("run/jobs").exists()
            && fs::read_dir(p.join("run/jobs"))?.next().is_some()
        {
            return Err("future unsealed page has history".into());
        }
    }
    Ok((n, counts, previous))
}
fn lease_unheld(p: &Path) -> Result<bool> {
    let f = OpenOptions::new()
        .read(true)
        .write(true)
        .custom_flags(libc::O_NOFOLLOW)
        .open(p)?;
    let yes = unsafe { libc::flock(f.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } == 0;
    if yes {
        unsafe { libc::flock(f.as_raw_fd(), libc::LOCK_UN) };
    }
    Ok(yes)
}
fn audit_fits(audit: &Value, a: &Admission) -> bool {
    audit["no_unresolved_reservations"] == true
        && audit["validated_targets"]
            .as_u64()
            .unwrap_or(u64::MAX)
            .checked_add(audit["quarantined_targets"].as_u64().unwrap_or(u64::MAX))
            .and_then(|n| n.checked_add(audit["pending_targets"].as_u64().unwrap_or(u64::MAX)))
            == Some(a.queued as u64)
}
pub struct Pager {
    pub config: Config,
    intents: Vec<Intent>,
    cursor: usize,
    counts: Counts,
    previous: String,
    _lease: File,
    active: Option<Arc<mova_fish_swarm::Supervisor>>,
}
impl Pager {
    pub fn open(config: Config) -> Result<Self> {
        config.fish_executable.read()?;
        let intents = plans(&config)?;
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
            return Err("pager lease held".into());
        }
        let cp = config.root.join("config.json");
        if cp.exists() {
            if regular(&cp)? != config.bytes {
                return Err("immutable pager config differs".into());
            }
        } else {
            fresh(&cp, &config.bytes)?
        }
        let identity = json!({"schema":1,"kind":KIND,"config_sha256":config.identity});
        let ip = config.root.join("config-identity.json");
        if ip.exists() {
            if read_json(&ip)? != identity {
                return Err("pager identity mismatch".into());
            }
        } else {
            fresh_json(&ip, &identity)?
        }
        fs::create_dir_all(config.root.join("pages"))?;
        fs::create_dir_all(config.root.join("seals"))?;
        let (cursor, counts, previous) = sealed_prefix(&config, &intents)?;
        let s = Self {
            config,
            intents,
            cursor,
            counts,
            previous,
            _lease: lease,
            active: None,
        };
        s.checkpoint()?;
        Ok(s)
    }
    fn checkpoint(&self) -> Result<()> {
        atomic(
            &self.config.root.join("checkpoint.json"),
            &json!({"schema":1,"config_sha256":self.config.identity,"plan_sha256":self.config.plan.sha256,"cursor":self.cursor,"previous_seal_sha256":self.previous,"counts":self.counts.value()}),
        )
    }
    fn status(
        &self,
        state: &str,
        now: u64,
        call: Value,
        safe: bool,
        extra: Value,
    ) -> Result<Value> {
        let total: usize = self.intents.iter().map(|i| i.sources.len()).sum();
        let mut progress = self.counts.clone();
        progress.valid += extra["current_audit"]["validated_targets"]
            .as_u64()
            .unwrap_or(0);
        progress.quarantine += extra["current_audit"]["quarantined_targets"]
            .as_u64()
            .unwrap_or(0);
        progress.deferred += extra["admission_deferred"].as_u64().unwrap_or(0);
        progress.rejected += extra["admission_rejected"].as_u64().unwrap_or(0);
        let v = json!({"schema":1,"kind":KIND,"state":state,"config_sha256":self.config.identity,"plan_sha256":self.config.plan.sha256,
 "pid":std::process::id(),"heartbeat_unix":now,"observed_unix":now,"not_after_unix":self.config.not_after_unix,
 "active_model_call":call,"no_unresolved_reservations":safe,"review_only":true,"training_authorized":false,
 "cursor":self.cursor,"pages_total":self.intents.len(),"source_targets_total":total,"sealed_counts":self.counts.value(),"counts":progress.value(),"review_pending_targets":(total as u64).saturating_sub(progress.valid),
 "current_page":self.intents.get(self.cursor).map(|i|i.id.clone()),"details":extra});
        atomic(&self.config.root.join("status.json"), &v)?;
        Ok(v)
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
    /// Performs at most one existing swarm step; the monitor writes fresh heartbeats while it drains.
    pub fn step<F>(&mut self, clock: F) -> Result<Value>
    where
        F: Fn() -> Result<u64> + Send + Sync + Copy + 'static,
    {
        let time = clock()?;
        let Some(i) = self.intents.get(self.cursor).cloned() else {
            if let Some(s) = self.boundary(time) {
                return self.status(s, time, json!(false), true, json!({}));
            }
            return self.status(
                "plan_exhausted",
                time,
                json!(false),
                true,
                json!({"automatic_repeat":false}),
            );
        };
        let ready_path = self.config.ready_root.join(format!("{}.json", i.id));
        let page_dir = self.config.root.join("pages").join(&i.id);
        if !ready_path.exists() {
            if page_dir.exists() {
                return self.status(
                    "uncertain",
                    time,
                    Value::Null,
                    false,
                    json!({"reason":"retained page without immutable admission"}),
                );
            }
            if let Some(s) = self.boundary(time) {
                return self.status(s, time, json!(false), true, json!({}));
            }
            return self.status(
                "waiting_page",
                time,
                json!(false),
                true,
                json!({"next_intent":i.id}),
            );
        }
        let a = admit(&self.config, &i)?;
        let sc_path = child_config(&self.config, &i, &a.ready)?;
        let sc = mova_fish_swarm::Config::load(&sc_path)?;
        if self.active.is_none() {
            self.active = Some(Arc::new(mova_fish_swarm::Supervisor::open(sc.clone())?));
        }
        let before = mova_fish_swarm::audit_history(&sc)?;
        if !audit_fits(&before, &a) {
            return self.status(
                "uncertain",
                time,
                Value::Null,
                false,
                json!({"current_audit":before,"reason":"unresolved inner history"}),
            );
        }
        let extra = json!({"current_audit":before,"admission_deferred":a.deferred,"admission_rejected":a.rejected});
        if let Some(s) = self.boundary(time) {
            return self.status(s, time, json!(false), true, extra);
        }
        let marker = sc.root.join("PAUSE");
        let owned = format!("mova-fish-pages:{}\n", self.config.identity);
        if marker.exists() && fs::read(&marker)? == owned.as_bytes() {
            fs::remove_file(&marker)?;
        }
        let worker = self.active.as_ref().unwrap().clone();
        let (tx, rx) = mpsc::channel();
        self.status("active", time, Value::Null, false, extra.clone())?;
        let handle = thread::spawn(move || {
            let result = worker.step(clock).map_err(|e| e.to_string());
            let _ = tx.send(result);
        });
        let result = loop {
            match rx.recv_timeout(Duration::from_millis(500)) {
                Ok(r) => break r,
                Err(mpsc::RecvTimeoutError::Disconnected) => {
                    break Err("inner step thread disconnected".into())
                }
                Err(mpsc::RecvTimeoutError::Timeout) => {
                    if self.config.root.join("PAUSE").exists() && !marker.exists() {
                        fresh(&marker, owned.as_bytes())?;
                    }
                    let child = mova_fish_swarm::status(&sc).unwrap_or(Value::Null);
                    let call = if child["state"] == "active" {
                        json!(true)
                    } else {
                        Value::Null
                    };
                    self.status("active",clock()?,call,false,json!({"current_audit":before,"admission_deferred":a.deferred,"admission_rejected":a.rejected,"inner_state":child["state"]}))?;
                }
            }
        };
        if handle.join().is_err() {
            return self.status(
                "uncertain",
                clock()?,
                Value::Null,
                false,
                json!({"reason":"inner step panic"}),
            );
        }
        let value = match result {
            Ok(v) => v,
            Err(e) => {
                return self.status(
                    "uncertain",
                    clock()?,
                    Value::Null,
                    false,
                    json!({"reason":e}),
                )
            }
        };
        let audit = mova_fish_swarm::audit_history(&sc)?;
        if !audit_fits(&audit, &a)
            || matches!(value["state"].as_str(), Some("uncertain" | "failed"))
        {
            return self.status(
                "uncertain",
                clock()?,
                Value::Null,
                false,
                json!({"current_audit":audit,"inner_state":value["state"]}),
            );
        }
        let extra = json!({"current_audit":audit,"admission_deferred":a.deferred,"admission_rejected":a.rejected});
        if let Some(s) = self.boundary(clock()?) {
            return self.status(s, clock()?, json!(false), true, extra);
        }
        // Provider busy before any work: the job is back in the inner queue (bounded backoff) or
        // held for review. Neither is an uncertain submission nor a usage limit.
        if matches!(
            value["state"].as_str(),
            Some("provider_busy_backoff" | "provider_busy_held")
        ) {
            let mut extra = extra;
            extra["provider_busy"] = value["details"].clone();
            return self.status(
                value["state"].as_str().unwrap_or_default(),
                clock()?,
                json!(false),
                true,
                extra,
            );
        }
        if value["state"] == "queue_exhausted" {
            if audit["pending_targets"] != 0 || audit["not_started_jobs"] != 0 {
                return self.status("uncertain",clock()?,Value::Null,false,json!({"reason":"exhausted page retains unattempted targets","current_audit":audit}));
            }
            let seal = json!({"schema":1,"config_sha256":self.config.identity,"plan_sha256":self.config.plan.sha256,
 "intent":i.pin.value(),"admission":a.pin.value(),"previous_seal_sha256":self.previous,"audit":audit,
 "review_only":true,"training_authorized":false});
            let path = self
                .config
                .root
                .join("seals")
                .join(format!("{}.json", i.id));
            fresh_json(&path, &seal)?;
            self.previous = hash(&regular(&path)?);
            self.counts.add(&a, &audit);
            self.cursor += 1;
            self.active = None;
            self.checkpoint()?;
            return self.status(
                "page_completed",
                clock()?,
                json!(false),
                true,
                json!({"completed_page":i.id}),
            );
        }
        self.status(
            if value["state"] == "paused" {
                "child_paused"
            } else {
                "job_completed"
            },
            clock()?,
            json!(false),
            true,
            extra,
        )
    }
}
/// Read-only safe-boundary check; no reservation, subprocess or training side effects.
pub fn quiescent(root: &Path, s: &Value, time: u64) -> Result<bool> {
    let check = (|| -> Result<bool> {
        let c = Config::load(&root.join("config.json"))?;
        let identity = read_json(&root.join("config-identity.json"))?;
        if c.root != root
            || s["schema"] != 1
            || s["kind"] != KIND
            || identity["kind"] != KIND
            || identity["config_sha256"] != c.identity
            || s["config_sha256"] != c.identity
            || s["plan_sha256"] != c.plan.sha256
            || s["review_only"] != true
            || s["training_authorized"] != false
            || s["active_model_call"] != false
            || s["no_unresolved_reservations"] != true
        {
            return Ok(false);
        }
        let intents = plans(&c)?;
        let (cursor, sealed_counts, _) = sealed_prefix(&c, &intents)?;
        let mut observed_counts = sealed_counts.clone();
        if s["sealed_counts"] != sealed_counts.value()
            || s["source_targets_total"] != intents.iter().map(|i| i.sources.len()).sum::<usize>()
            || s["cursor"] != cursor
        {
            return Ok(false);
        }
        let paused = s["state"] == "paused"
            && root.join("PAUSE").is_file()
            && s["heartbeat_unix"]
                .as_u64()
                .is_some_and(|t| t <= time && time - t <= 30);
        let terminal = (s["state"] == "stopped" && root.join("STOP").is_file())
            || (s["state"] == "deadline_reached"
                && time >= c.not_after_unix
                && s["not_after_unix"] == c.not_after_unix);
        if !paused && !terminal {
            return Ok(false);
        }
        if terminal && !lease_unheld(&root.join("lease"))? {
            return Ok(false);
        }
        if let Some(i) = intents.get(cursor) {
            let dir = root.join("pages").join(&i.id);
            if dir.exists() {
                let a = admit(&c, i)?;
                let sc = existing_child(&c, i, &a)?;
                let audit = mova_fish_swarm::audit_history(&sc)?;
                if !audit_fits(&audit, &a) {
                    return Ok(false);
                }
                if terminal && !lease_unheld(&sc.root.join("lease"))? {
                    return Ok(false);
                }
                observed_counts.add(&a, &audit);
            }
        }
        if s["counts"] != observed_counts.value() {
            return Ok(false);
        }
        Ok(true)
    })();
    Ok(check.unwrap_or(false))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        process::Command,
        sync::atomic::{AtomicUsize, Ordering},
    };
    static SEQ: AtomicUsize = AtomicUsize::new(0);
    struct Fixture {
        base: PathBuf,
        config: PathBuf,
    }
    impl Fixture {
        fn new(page_sources: Vec<Vec<&str>>, statuses: Vec<&str>, delay_ms: u64) -> Result<Self> {
            let base = std::env::temp_dir().join(format!(
                "fish-pages-test-{}-{}",
                std::process::id(),
                SEQ.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&base)?;
            fs::create_dir(base.join("ready"))?;
            fs::create_dir(base.join("templates"))?;
            let fake_source = base.join("fake.rs");
            let fake = base.join("fake");
            let code = format!(
                r#"use std::{{fs,env,time::Duration,io::Write}};
 fn main(){{let a:Vec<_>=env::args().collect();let task=fs::read_to_string(&a[2]).unwrap();
 let id=task.split("\"id\": \"").nth(1).unwrap().split('"').next().unwrap();
 std::thread::sleep(Duration::from_millis({delay_ms}));
 let b=fs::read(format!("{{}}/templates/{{}}.json",{base:?},id)).unwrap();
 fs::create_dir(&a[4]).unwrap();fs::write(format!("{{}}/receipt.json",&a[4]),&b).unwrap();
 let mut f=fs::OpenOptions::new().create(true).append(true).open(format!("{{}}/calls",{base:?})).unwrap();
 writeln!(f,"{{}}",id).unwrap();let t=String::from_utf8(b).unwrap();
 if t.contains("provider_busy"){{fs::copy(format!("{{}}/busy.stdout",{base:?}),format!("{{}}/raw.stdout",&a[4])).unwrap();std::process::exit(1)}}
 if t.contains("invalid_response"){{std::process::exit(1)}}
 }}"#,
                base = base.to_string_lossy()
            );
            fs::write(&fake_source, code)?;
            let result = Command::new("rustc")
                .arg("--edition=2021")
                .arg(&fake_source)
                .arg("-o")
                .arg(&fake)
                .output()?;
            if !result.status.success() {
                return Err(format!(
                    "fake rust compile: {}",
                    String::from_utf8_lossy(&result.stderr)
                )
                .into());
            }
            let profile = base.join("profile.json");
            fresh_json(&profile, &json!({"family":"fake"}))?;
            let busy = "{\"type\":\"turn.failed\",\"error\":{\"message\":\"Selected model is at capacity. Please try a different model.\"}}\n";
            fs::write(base.join("busy.stdout"), busy)?;
            let mut page_pins = vec![];
            for (index, sources) in page_sources.iter().enumerate() {
                let id = format!("p{index}");
                let ip = base.join(format!("intent-{index}.json"));
                fresh_json(&ip, &json!({"schema":1,"id":id,"sources":sources}))?;
                page_pins.push(pin(&ip)?.value());
                let task = base.join(format!("task-{index}.json"));
                fresh_json(
                    &task,
                    &json!({"id":format!("j{index}"),"target_ids":sources}),
                )?;
                let pp = pin(&profile)?.value();
                let tp = pin(&task)?.value();
                let input = base.join(format!("source-{index}.json"));
                fresh_json(&input, &json!({"source":index}))?;
                let queue = base.join(format!("queue-{index}.jsonl"));
                let q = json!({"schema":1,"id":format!("j{index}"),"task":tp,"profile":pp,"source_pins":[pin(&input)?.value()],"provenance":{}});
                fresh(
                    &queue,
                    format!("{}\n", serde_json::to_string(&q)?).as_bytes(),
                )?;
                fresh_json(
                    &base.join("ready").join(format!("{id}.json")),
                    &json!({"schema":1,"intent":pin(&ip)?.value(),"queue":pin(&queue)?.value(),"deferred":[],"rejected":[]}),
                )?;
                let rb = regular(&task)?;
                let pb = regular(&profile)?;
                let status = statuses
                    .get(index)
                    .copied()
                    .unwrap_or("validated_review_only");
                let mut template = json!({"schema":1,"task_id":format!("j{index}"),
 "task_file_sha256":hash(&rb),"task_sha256":hash(format!("typed-{index}").as_bytes()),"profile_file_sha256":hash(&pb),
 "status":status,"exit_code":0,
 "model_process_started":true,"model_process_reaped":true,"model_process_group_absent":true,
 "replay_authorized":false,"review_only":true,"training_authorized":false});
                if status == "provider_busy" {
                    template["exit_code"] = json!(1);
                    template["provider_busy_message"] =
                        json!("Selected model is at capacity. Please try a different model.");
                    template["raw_output_hashes"] = json!({"stdout_sha256":hash(busy.as_bytes())});
                }
                fresh_json(
                    &base.join("templates").join(format!("j{index}.json")),
                    &template,
                )?;
            }
            let plan = base.join("plan.json");
            fresh_json(&plan, &json!({"schema":1,"pages":page_pins}))?;
            let config = base.join("input-config.json");
            fresh_json(
                &config,
                &json!({"schema":1,"root":base.join("run"),"plan":pin(&plan)?.value(),"ready_root":base.join("ready"),
 "fish_executable":pin(&fake)?.value(),"not_after_unix":100,"timeout_seconds":3,"poll_seconds":1}),
            )?;
            Ok(Self { base, config })
        }
        fn pager(&self) -> Result<Pager> {
            Pager::open(Config::load(&self.config)?)
        }
        fn root(&self) -> PathBuf {
            self.base.join("run")
        }
        fn calls(&self) -> usize {
            fs::read_to_string(self.base.join("calls"))
                .unwrap_or_default()
                .lines()
                .count()
        }
    }

    #[test]
    fn pages_count_sources_and_never_replay_on_restart() -> Result<()> {
        let f = Fixture::new(vec![vec!["d:1", "d:2"], vec!["d:3"]], vec![], 0)?;
        let mut p = f.pager()?;
        assert_eq!(p.step(|| Ok(1))?["counts"]["validated_targets"], 2);
        assert_eq!(f.calls(), 1);
        drop(p);
        let mut p = f.pager()?;
        assert_eq!(p.step(|| Ok(1))?["state"], "page_completed");
        assert_eq!(p.step(|| Ok(1))?["counts"]["validated_targets"], 3);
        assert_eq!(f.calls(), 2);
        assert_eq!(p.step(|| Ok(1))?["state"], "page_completed");
        assert_eq!(p.step(|| Ok(1))?["state"], "plan_exhausted");
        drop(p);
        let mut p = f.pager()?;
        assert_eq!(p.step(|| Ok(1))?["state"], "plan_exhausted");
        assert_eq!(f.calls(), 2);
        Ok(())
    }
    #[test]
    fn provider_busy_is_a_safe_backoff_not_uncertain_and_the_retry_completes() -> Result<()> {
        let f = Fixture::new(vec![vec!["d:1"]], vec!["provider_busy"], 0)?;
        let mut p = f.pager()?;
        let s = p.step(|| Ok(1))?;
        assert_eq!(s["state"], "provider_busy_backoff");
        assert_eq!(s["no_unresolved_reservations"], true);
        assert_eq!(s["active_model_call"], false);
        assert_eq!(s["details"]["provider_busy"]["retry_after_unix"], 61);
        assert_eq!(p.step(|| Ok(30))?["state"], "provider_busy_backoff");
        assert_eq!(f.calls(), 1);
        // A restart keeps the retained busy attempt resolved; no uncertain state, no early call.
        drop(p);
        let mut p = f.pager()?;
        assert_eq!(p.step(|| Ok(31))?["state"], "provider_busy_backoff");
        let t = f.base.join("templates/j0.json");
        let mut v = read_json(&t)?;
        v["status"] = json!("validated_review_only");
        v["exit_code"] = json!(0);
        let o = v.as_object_mut().unwrap();
        o.remove("provider_busy_message");
        o.remove("raw_output_hashes");
        fs::write(&t, serde_json::to_vec(&v)?)?;
        assert_eq!(p.step(|| Ok(61))?["state"], "job_completed");
        let s = p.step(|| Ok(62))?;
        assert_eq!(s["state"], "page_completed");
        let seal = read_json(&f.root().join("seals/p0.json"))?;
        assert_eq!(seal["audit"]["provider_busy_attempts"], 1);
        assert_eq!(f.calls(), 2);
        Ok(())
    }
    #[test]
    fn waiting_page_pause_and_terminal_lease_are_distinct() -> Result<()> {
        let f = Fixture::new(vec![vec!["d:1"]], vec![], 0)?;
        let ready = f.base.join("ready/p0.json");
        let saved = f.base.join("held-ready.json");
        fs::rename(&ready, &saved)?;
        let mut p = f.pager()?;
        assert_eq!(p.step(|| Ok(1))?["state"], "waiting_page");
        assert_eq!(f.calls(), 0);
        fs::write(f.root().join("PAUSE"), "test")?;
        let s = p.step(|| Ok(1))?;
        assert!(quiescent(&f.root(), &s, 1)?);
        assert!(!quiescent(&f.root(), &s, 32)?);
        fs::remove_file(f.root().join("PAUSE"))?;
        fs::rename(saved, ready)?;
        assert_eq!(p.step(|| Ok(1))?["state"], "job_completed");
        assert_eq!(f.calls(), 1);
        let s = p.step(|| Ok(100))?;
        assert_eq!(s["state"], "deadline_reached");
        assert!(!quiescent(&f.root(), &s, 100)?);
        drop(p);
        assert!(quiescent(&f.root(), &s, 100)?);
        Ok(())
    }
    #[test]
    fn known_closed_quarantine_advances_but_remains_unreviewed() -> Result<()> {
        let f = Fixture::new(
            vec![vec!["d:1"], vec!["d:2", "d:3"]],
            vec!["invalid_response", "validated_review_only"],
            0,
        )?;
        let mut p = f.pager()?;
        let s = p.step(|| Ok(1))?;
        assert_eq!(s["counts"]["quarantined_targets"], 1);
        assert_eq!(s["review_pending_targets"], 3);
        assert_eq!(p.step(|| Ok(1))?["state"], "page_completed");
        let s = p.step(|| Ok(1))?;
        assert_eq!(s["counts"]["validated_targets"], 2);
        assert_eq!(s["counts"]["quarantined_targets"], 1);
        assert_eq!(s["review_pending_targets"], 1);
        assert_eq!(f.calls(), 2);
        Ok(())
    }
    #[test]
    fn orphan_precedes_pause_stop_and_deadline() -> Result<()> {
        let f = Fixture::new(vec![vec!["d:1"]], vec![], 0)?;
        let mut p = f.pager()?;
        fs::write(f.root().join("PAUSE"), "test")?;
        assert_eq!(p.step(|| Ok(1))?["state"], "paused");
        let dir = f.root().join("pages/p0/run/jobs/orphan");
        fs::create_dir(&dir)?;
        fresh_json(&dir.join("reservation.json"), &json!({"schema":1}))?;
        fs::write(f.root().join("STOP"), "test")?;
        let s = p.step(|| Ok(100))?;
        assert_eq!(s["state"], "uncertain");
        assert_eq!(s["active_model_call"], Value::Null);
        assert!(!quiescent(&f.root(), &s, 100)?);
        assert_eq!(f.calls(), 0);
        Ok(())
    }
    #[test]
    fn duplicate_source_ownership_and_unaccounted_pages_fail_before_child() -> Result<()> {
        let f = Fixture::new(vec![vec!["d:1"], vec!["d:1"]], vec![], 0)?;
        assert!(f.pager().is_err());
        assert_eq!(f.calls(), 0);
        let f = Fixture::new(vec![vec!["d:1"]], vec![], 0)?;
        let mut r = read_json(&f.base.join("ready/p0.json"))?;
        r["deferred"] = json!([{"source_id":"d:1","reason":"duplicate"}]);
        fs::write(f.base.join("ready/p0.json"), serde_json::to_vec_pretty(&r)?)?;
        let mut p = f.pager()?;
        assert!(p.step(|| Ok(1)).is_err());
        assert_eq!(f.calls(), 0);
        Ok(())
    }
    #[test]
    fn explicit_gaps_account_for_sources_without_model_claims() -> Result<()> {
        let f = Fixture::new(vec![vec!["d:1", "d:2"]], vec![], 0)?;
        let queue = f.base.join("gaps.jsonl");
        fresh(&queue, b"")?;
        let mut r = read_json(&f.base.join("ready/p0.json"))?;
        r["queue"] = pin(&queue)?.value();
        r["deferred"] = json!([{"source_id":"d:1","reason":"cached native bundle missing"}]);
        r["rejected"] = json!([{"source_id":"d:2","reason":"native identity domain"}]);
        fs::write(f.base.join("ready/p0.json"), serde_json::to_vec_pretty(&r)?)?;
        let mut p = f.pager()?;
        let s = p.step(|| Ok(1))?;
        assert_eq!(s["state"], "page_completed");
        assert_eq!(s["counts"]["deferred_targets"], 1);
        assert_eq!(s["counts"]["rejected_targets"], 1);
        assert_eq!(s["counts"]["validated_targets"], 0);
        assert_eq!(s["review_pending_targets"], 2);
        assert_eq!(f.calls(), 0);
        Ok(())
    }
    #[test]
    fn active_step_drains_and_publishes_pause_only_after_closed_proof() -> Result<()> {
        let f = Fixture::new(vec![vec!["d:1"]], vec![], 1200)?;
        let mut p = f.pager()?;
        let root = f.root();
        let watcher = thread::spawn(move || {
            thread::sleep(Duration::from_millis(100));
            fs::write(root.join("PAUSE"), "test").unwrap();
            thread::sleep(Duration::from_millis(650));
            let s = read_json(&root.join("status.json")).unwrap();
            assert_eq!(s["state"], "active");
            assert!(!quiescent(&root, &s, 1).unwrap());
        });
        let s = p.step(|| Ok(1))?;
        watcher.join().unwrap();
        assert_eq!(s["state"], "paused");
        assert_eq!(s["counts"]["validated_targets"], 1);
        assert!(quiescent(&f.root(), &s, 1)?);
        assert_eq!(f.calls(), 1);
        Ok(())
    }
    #[test]
    fn altered_closed_proof_and_retained_checkpoint_are_not_silently_repaired() -> Result<()> {
        let f = Fixture::new(vec![vec!["d:1"]], vec![], 0)?;
        let mut p = f.pager()?;
        p.step(|| Ok(1))?;
        p.step(|| Ok(1))?;
        drop(p);
        let receipt = f.root().join("pages/p0/run/jobs/j0/output/receipt.json");
        let mut r = read_json(&receipt)?;
        r["model_process_group_absent"] = json!(false);
        fs::write(receipt, serde_json::to_vec_pretty(&r)?)?;
        assert!(f.pager().is_err());
        assert_eq!(f.calls(), 1);
        let f = Fixture::new(vec![], vec![], 0)?;
        let p = f.pager()?;
        drop(p);
        fs::write(f.root().join("checkpoint.json"), b"{broken")?;
        assert!(f.pager().is_err());
        Ok(())
    }
}
