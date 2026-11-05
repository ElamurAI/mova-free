//! The Whales as one of Mova's animals: a persistent, singleton service on a private socket that
//! takes ordinary English through Mova's native deputy (`mova_octopus::Deputy`: Mova's annotator,
//! world reader, configured capabilities and typed arguments bound through the parse) and answers
//! in English realized by `answer.rs`, with the structured result alongside.
//!
//! What is described, by which prompt and against which contract stays configuration: the
//! service config names a catalog (a pinned JSONL file of sources), the survey run skeleton (spec,
//! teacher, store, originality, metadata export) and the frozen `mova-whale` executable. The public
//! code knows no concrete task domain.
//!
//! Work. A describe request is appended to the request journal `requests.jsonl` (one fsynced line
//! per event: queued, started, finished) and answered at once. The service's workers admit waiting
//! requests urgent first, then by arrival; a prompt never interrupts running work, priority only
//! orders the waiting ones. Each admitted request becomes its own new one-item pinned Whale run
//! (`runs/<job>/survey`, prepared by `whale::prepare`, then `mova-whale whale CONFIG --once` as a
//! child of the pinned executable): no existing queue, manifest or bundle is ever rewritten, and the
//! run makes at most one teacher call. `started` is journaled before preparation; a started request
//! without a finished line after a restart is resolved from its run's receipt if one exists, else
//! recorded `uncertain` and never relaunched. A queued request that never started survives a restart
//! and waits again (nothing was done for it). Questions are answered from the journal, the shared
//! state and the bundle store without waiting for any work. `PAUSE` in the run root stops admission;
//! running work may finish.
use crate::{
    answer, whale,
    spec::Spec,
    util::{error, file_hash, hash, lease, load, now, private_dir, valid_hash},
    Result,
};
use mova_octopus::{Action, CapabilityHandler, Deputy, Report};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet, VecDeque},
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Read, Seek, SeekFrom, Write},
    os::unix::fs::OpenOptionsExt,
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::{Arc, Condvar, Mutex},
    time::Duration,
};

/// The capability operations this animal installs, and the slot each reads.
pub const OPERATIONS: [(&str, Option<&str>); 6] =
    [("describe", Some("sources")), ("count", None), ("status", Some("source")), ("show", Some("source")), ("pause", None), ("resume", None)];
/// Operations that change state (queue, pause or resume work); never declared `observes` in the deputy configuration.
pub const ACTING: [&str; 3] = ["describe", "pause", "resume"];
/// Finished requests kept in memory for status answers (the journal keeps all of them).
const KEEP_FINISHED: usize = 64;
const CATALOG_ROW_MAX: usize = 1 << 20;
const DESCRIPTION_MAX: usize = 1 << 20;
/// Survey keys the service sets per request; a skeleton naming them is refused.
const SERVICE_KEYS: [&str; 5] = ["run_root", "items", "workers", "max_items", "worker_index"];

fn d_id() -> String {
    "id".into()
}
fn d_workers() -> usize {
    1
}
fn d_waiting() -> usize {
    64
}
fn d_noun() -> String {
    "source".into()
}

/// A URL that names a catalog row: `prefix` + the row's `field` value.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct UrlRule {
    pub prefix: String,
    pub field: String,
}

/// The pinned catalog of describable sources: one JSON object per line.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct CatalogConfig {
    pub path: PathBuf,
    pub sha256: String,
    /// The survey kind (a key of the specification's `kinds`) of every catalog source.
    pub kind: String,
    #[serde(default = "d_id")]
    pub id_field: String,
    /// The row field holding the absolute source file path.
    pub path_field: String,
    #[serde(default)]
    pub title_field: Option<String>,
    #[serde(default)]
    pub licence_field: Option<String>,
    /// File-name suffix -> item format (a key of the kind's `formats`), longest suffix first.
    pub formats: BTreeMap<String, String>,
    /// Row fields that hold a full URL of the source.
    #[serde(default)]
    pub url_fields: Vec<String>,
    #[serde(default)]
    pub url_prefixes: Vec<UrlRule>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct PinConfig {
    pub path: PathBuf,
    pub sha256: String,
}

/// `mova-whale serve` configuration (JSON, unknown fields refused).
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct AnimalConfig {
    /// Private 0700 directory: journal, per-request runs, PAUSE marker, service lease.
    pub run_root: PathBuf,
    /// The frozen `mova-whale` binary that runs each request's one-item survey.
    pub executable: PinConfig,
    /// Optional Codex CLI path handed to the children as `MOVA_CODEX_BIN`.
    #[serde(default)]
    pub codex: Option<PathBuf>,
    /// The survey run skeleton (`spec`, `teacher`, `store`, optional `originality`,
    /// `metadata_export`, `storage_floor_bytes`); run root and items are set per request.
    pub survey: Value,
    pub catalog: CatalogConfig,
    /// Concurrent requests (1..4).
    #[serde(default = "d_workers")]
    pub workers: usize,
    /// Most requests waiting at once (1..256).
    #[serde(default = "d_waiting")]
    pub max_waiting: usize,
    /// The noun answers use for one source ("paper").
    #[serde(default = "d_noun")]
    pub noun: String,
    /// Sea Spider's private socket (absolute). Unset (the default): the Whales do not talk to Sea
    /// Spider. Set: `seaspider::ask_sea_spider` may send it English requests as caller "whale".
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub seaspider_socket: Option<PathBuf>,
}

impl AnimalConfig {
    pub fn load(path: &Path) -> Result<Self> {
        let c: AnimalConfig = serde_json::from_value(load(path, 2 << 20)?)?;
        c.validate()?;
        Ok(c)
    }
    pub fn validate(&self) -> Result<()> {
        if !self.run_root.is_absolute() || !self.executable.path.is_absolute() || !self.catalog.path.is_absolute() {
            return Err(error("run_root, executable.path and catalog.path must be absolute"));
        }
        if self.codex.as_ref().is_some_and(|p| !p.is_absolute()) {
            return Err(error("codex must be absolute"));
        }
        if self.seaspider_socket.as_ref().is_some_and(|p| !p.is_absolute()) {
            return Err(error("seaspider_socket must be absolute"));
        }
        if !valid_hash(&self.executable.sha256) || !valid_hash(&self.catalog.sha256) {
            return Err(error("executable and catalog pins need lowercase SHA-256"));
        }
        if !(1..=4).contains(&self.workers) || !(1..=256).contains(&self.max_waiting) {
            return Err(error("workers must be 1..4 and max_waiting 1..256"));
        }
        if self.noun.is_empty() || self.noun.len() > 32 || !self.noun.bytes().all(|b| b.is_ascii_lowercase()) {
            return Err(error("noun must be one lowercase word"));
        }
        let o = self.survey.as_object().ok_or("survey must be an object")?;
        if let Some(k) = SERVICE_KEYS.iter().find(|k| o.contains_key(**k)) {
            return Err(error(format!("survey must not set {k}; the service sets it per request")));
        }
        let spec = Spec::load(&self.survey)?;
        whale::teacher(&self.survey)?;
        crate::util::absolute(&self.survey, "store")?;
        let kind = spec.kind(&self.catalog.kind)?;
        if self.catalog.formats.is_empty() || self.catalog.formats.values().any(|f| !kind.formats.contains_key(f)) {
            return Err(error("catalog formats must name formats admitted for the catalog kind"));
        }
        Ok(())
    }
}

/// The catalog index: id -> (byte offset, length) of its row, and URL -> id.
pub struct Catalog {
    cfg: CatalogConfig,
    rows: BTreeMap<String, (u64, usize)>,
    urls: BTreeMap<String, String>,
}

fn url_key(u: &str) -> String {
    u.trim_end_matches('/').to_ascii_lowercase()
}

impl Catalog {
    /// Read the whole catalog once, checking its pin; duplicate ids are refused.
    pub fn load(cfg: &CatalogConfig) -> Result<Self> {
        use sha2::Digest;
        let mut h = sha2::Sha256::new();
        let mut r = BufReader::new(File::open(&cfg.path)?);
        let (mut rows, mut urls) = (BTreeMap::new(), BTreeMap::new());
        let (mut offset, mut line) = (0u64, Vec::new());
        loop {
            line.clear();
            let n = r.read_until(b'\n', &mut line)?;
            if n == 0 {
                break;
            }
            h.update(&line);
            if n > CATALOG_ROW_MAX {
                return Err(error("oversized catalog row"));
            }
            if !line.iter().all(u8::is_ascii_whitespace) {
                let v: Value = serde_json::from_slice(&line)?;
                if let Some(id) = v[&cfg.id_field].as_str() {
                    if rows.insert(id.to_string(), (offset, n)).is_some() {
                        return Err(error(format!("duplicate catalog id {id}")));
                    }
                    for f in &cfg.url_fields {
                        if let Some(u) = v[f].as_str() {
                            urls.entry(url_key(u)).or_insert_with(|| id.to_string());
                        }
                    }
                    for rule in &cfg.url_prefixes {
                        if let Some(x) = v[&rule.field].as_str().filter(|x| !x.is_empty()) {
                            urls.entry(url_key(&format!("{}{x}", rule.prefix))).or_insert_with(|| id.to_string());
                        }
                    }
                }
            }
            offset += n as u64;
        }
        if format!("{:x}", h.finalize()) != cfg.sha256 {
            return Err(error("catalog hash mismatch"));
        }
        Ok(Self { cfg: cfg.clone(), rows, urls })
    }
    pub fn len(&self) -> usize {
        self.rows.len()
    }
    pub fn is_empty(&self) -> bool {
        self.rows.is_empty()
    }
    /// The catalog id an argument names: a URL through the URL index, otherwise an id.
    pub fn resolve(&self, arg: &str) -> Option<String> {
        if arg.contains("://") {
            self.urls.get(&url_key(arg)).cloned()
        } else {
            self.rows.contains_key(arg).then(|| arg.to_string())
        }
    }
    pub fn row(&self, id: &str) -> Result<Value> {
        let (off, len) = *self.rows.get(id).ok_or("not in the catalog")?;
        let mut f = File::open(&self.cfg.path)?;
        f.seek(SeekFrom::Start(off))?;
        let mut b = vec![0; len];
        f.read_exact(&mut b)?;
        let v: Value = serde_json::from_slice(&b)?;
        if v[&self.cfg.id_field] != id {
            return Err(error("the catalog changed since the service started"));
        }
        Ok(v)
    }
    pub fn title(&self, row: &Value) -> Option<String> {
        self.cfg.title_field.as_ref().and_then(|f| row[f].as_str()).map(str::to_string)
    }
    /// The survey item for a catalog row; hashes the source file now.
    pub fn item(&self, id: &str, row: &Value) -> Result<Value> {
        let path = PathBuf::from(row[&self.cfg.path_field].as_str().ok_or("the catalog row has no source path")?);
        if !path.is_absolute() {
            return Err(error("the catalog source path is not absolute"));
        }
        let name = path.to_string_lossy();
        let format = self
            .cfg
            .formats
            .iter()
            .filter(|(suffix, _)| name.ends_with(suffix.as_str()))
            .max_by_key(|(suffix, _)| suffix.len())
            .map(|(_, f)| f.clone())
            .ok_or("its source file has no configured format")?;
        let sha = file_hash(&path).map_err(|e| error(format!("its source file cannot be read: {e}")))?;
        let mut item = json!({"kind":self.cfg.kind,"id":id,"path":path,"sha256":sha,"format":format});
        if let Some(t) = self.title(row) {
            item["title"] = json!(t);
        }
        if let Some(l) = self.cfg.licence_field.as_ref().and_then(|f| row[f].as_str()) {
            item["licence"] = json!(l);
        }
        Ok(item)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Job {
    pub job: String,
    pub id: String,
    #[serde(default)]
    pub title: Option<String>,
    pub item: Value,
    pub urgent: bool,
    pub seq: u64,
    pub queued_unix: u64,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Finished {
    pub job: String,
    pub id: String,
    #[serde(default)]
    pub title: Option<String>,
    /// completed, quarantined, uncertain, local_only, preparation_failed, launch_failed
    pub state: String,
    pub started_unix: u64,
    pub finished_unix: u64,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub reason: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub manifest: Option<String>,
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub words: Option<u64>,
    pub teacher_called: Option<bool>,
}

#[derive(Clone)]
struct Running {
    job: Job,
    started_unix: u64,
}

#[derive(Default)]
struct State {
    waiting: Vec<Job>,
    running: Vec<Running>,
    finished: VecDeque<Finished>,
    /// The latest finished attempt per source id.
    last: BTreeMap<String, Finished>,
    totals: BTreeMap<String, u64>,
    seq: u64,
    stop: bool,
}

impl State {
    /// The next request: urgent first, then by arrival.
    fn next(&mut self) -> Option<Job> {
        let i = (0..self.waiting.len()).min_by_key(|&i| (!self.waiting[i].urgent, self.waiting[i].seq))?;
        Some(self.waiting.remove(i))
    }
    fn ordered_waiting(&self) -> Vec<&Job> {
        let mut w: Vec<&Job> = self.waiting.iter().collect();
        w.sort_by_key(|j| (!j.urgent, j.seq));
        w
    }
    fn finish(&mut self, f: Finished) {
        *self.totals.entry(f.state.clone()).or_default() += 1;
        self.last.insert(f.id.clone(), f.clone());
        self.finished.push_back(f);
        while self.finished.len() > KEEP_FINISHED {
            self.finished.pop_front();
        }
    }
}

struct Shared {
    cfg: AnimalConfig,
    spec: Spec,
    catalog: Catalog,
    state: Mutex<State>,
    cond: Condvar,
    journal: Mutex<File>,
}

impl Shared {
    fn lock(&self) -> std::sync::MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(|p| p.into_inner())
    }
    fn log(&self, v: Value) {
        let mut f = self.journal.lock().unwrap_or_else(|p| p.into_inner());
        let mut line = v.to_string();
        line.push('\n');
        if let Err(e) = f.write_all(line.as_bytes()).and_then(|_| f.sync_data()) {
            eprintln!("mova-whale: request journal: {e}");
        }
    }
    fn paused(&self) -> bool {
        self.cfg.run_root.join("PAUSE").exists()
    }
    fn store(&self) -> Result<PathBuf> {
        crate::util::absolute(&self.cfg.survey, "store")
    }
    /// The bucket of one source's bundles in the store.
    fn bucket(&self, id: &str) -> Result<PathBuf> {
        Ok(self.store()?.join(&self.spec.bundle.store_prefix).join(&self.cfg.catalog.kind).join(hash(id)))
    }
}

/// Complete bundle versions of one bucket, latest last: (manifest path, manifest, mtime).
fn versions(bucket: &Path) -> Vec<(PathBuf, Value, std::time::SystemTime)> {
    let mut out = vec![];
    let Ok(rd) = fs::read_dir(bucket) else { return out };
    for e in rd.flatten() {
        let name = e.file_name().to_string_lossy().to_string();
        let p = e.path().join("manifest.json");
        if name.starts_with('.') || !p.is_file() {
            continue;
        }
        if let (Ok(m), Ok(meta)) = (load(&p, 4 << 20), fs::metadata(&p)) {
            out.push((p, m, meta.modified().unwrap_or(std::time::UNIX_EPOCH)));
        }
    }
    out.sort_by(|a, b| (a.2, &a.0).cmp(&(b.2, &b.0)));
    out
}

/// Read a terminal Whale receipt of a one-item run, if there is one.
fn receipt(survey_root: &Path) -> Option<Value> {
    let m = load(&survey_root.join("manifest.json"), 64 << 20).ok()?;
    let key = m["tasks"][0]["key"].as_str()?;
    let r = load(&survey_root.join("jobs").join(key).join("receipt.json"), 2 << 20).ok()?;
    (r["task_sha256"] == key).then_some(r)
}

fn from_receipt(job: &Job, started: u64, r: &Value) -> Finished {
    let mut f = Finished {
        job: job.job.clone(),
        id: job.id.clone(),
        title: job.title.clone(),
        state: "uncertain".into(),
        started_unix: started,
        finished_unix: now(),
        reason: r["reason"].as_str().map(str::to_string),
        manifest: None,
        words: None,
        teacher_called: Some(true),
    };
    match r["outcome"].as_str() {
        Some("completed") => {
            f.state = "completed".into();
            f.manifest = r["output"]["manifest"].as_str().map(str::to_string);
            f.words = r["output"]["length_measure"]["count"].as_u64();
        }
        Some("quarantined") => f.state = "quarantined".into(),
        Some("local_only_no_remote_teacher") => {
            f.state = "local_only".into();
            f.teacher_called = Some(false);
        }
        _ => f.reason = Some(f.reason.unwrap_or_else(|| "the receipt names no terminal outcome".into())),
    }
    f
}

/// Run one admitted request as a new one-item pinned Whale run in a child process.
fn run_job(sh: &Shared, job: &Job, started: u64) -> Finished {
    let fail = |state: &str, reason: String, called: Option<bool>| Finished {
        job: job.job.clone(),
        id: job.id.clone(),
        title: job.title.clone(),
        state: state.into(),
        started_unix: started,
        finished_unix: now(),
        reason: Some(reason),
        manifest: None,
        words: None,
        teacher_called: called,
    };
    let dir = sh.cfg.run_root.join("runs").join(&job.job);
    if let Err(e) = fs::create_dir(&dir) {
        return fail("preparation_failed", format!("the run directory could not be created: {e}"), Some(false));
    }
    let survey_root = dir.join("survey");
    let mut c = sh.cfg.survey.clone();
    c["run_root"] = json!(survey_root);
    c["workers"] = json!(1);
    c["max_items"] = json!(1);
    c["items"] = json!([job.item]);
    let prepared = whale::prepare(&c).and_then(|m| {
        m["worker_configs"][0]["config"].as_str().map(PathBuf::from).ok_or_else(|| error("the prepared run has no worker config"))
    });
    let config = match prepared {
        Ok(p) => p,
        Err(e) => return fail("preparation_failed", e.to_string(), Some(false)),
    };
    match file_hash(&sh.cfg.executable.path) {
        Ok(h) if h == sh.cfg.executable.sha256 => {}
        Ok(_) => return fail("launch_failed", "the pinned mova-whale executable changed".into(), Some(false)),
        Err(e) => return fail("launch_failed", format!("the executable cannot be read: {e}"), Some(false)),
    }
    let log = match OpenOptions::new().create_new(true).write(true).mode(0o600).open(dir.join("process.log")) {
        Ok(f) => f,
        Err(e) => return fail("launch_failed", e.to_string(), Some(false)),
    };
    let mut cmd = Command::new(&sh.cfg.executable.path);
    cmd.arg("whale").arg(&config).arg("--once").stdin(Stdio::null());
    if let Ok(l2) = log.try_clone() {
        cmd.stdout(l2);
    }
    cmd.stderr(log);
    if let Some(codex) = &sh.cfg.codex {
        cmd.env("MOVA_CODEX_BIN", codex);
    }
    let status = match cmd.spawn() {
        Ok(mut child) => {
            sh.log(json!({"event":"launched","job":job.job,"pid":child.id(),"unix":now()}));
            child.wait()
        }
        Err(e) => return fail("launch_failed", format!("the child could not start: {e}"), Some(false)),
    };
    match receipt(&survey_root) {
        Some(r) => from_receipt(job, started, &r),
        None => fail(
            "uncertain",
            format!("the child ended ({}) without a terminal receipt; it is not retried", status.map(|s| s.to_string()).unwrap_or_else(|e| e.to_string())),
            None,
        ),
    }
}

fn worker(sh: Arc<Shared>) {
    loop {
        let job = {
            let mut s = sh.lock();
            loop {
                if s.stop {
                    break None;
                }
                if !sh.paused() {
                    if let Some(j) = s.next() {
                        let started = now();
                        // The reservation: journaled before anything is prepared or launched.
                        sh.log(json!({"event":"started","job":j.job,"id":j.id,"unix":started}));
                        s.running.push(Running { job: j.clone(), started_unix: started });
                        break Some((j, started));
                    }
                }
                s = sh.cond.wait_timeout(s, Duration::from_millis(500)).unwrap_or_else(|p| p.into_inner()).0;
            }
        };
        let Some((job, started)) = job else { break };
        let f = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| run_job(&sh, &job, started))).unwrap_or_else(|_| Finished {
            job: job.job.clone(),
            id: job.id.clone(),
            title: job.title.clone(),
            state: "uncertain".into(),
            started_unix: started,
            finished_unix: now(),
            reason: Some("the request panicked in the service; it is not retried".into()),
            manifest: None,
            words: None,
            teacher_called: None,
        });
        sh.log(json!({"event":"finished","job":f.job,"result":f}));
        let mut s = sh.lock();
        s.running.retain(|r| r.job.job != job.job);
        s.finish(f);
        sh.cond.notify_all();
    }
}

/// Read the request journal: waiting requests (queued, never started), finished ones, and those
/// started but never finished (resolved from their run receipt, else uncertain; never relaunched).
fn recover(sh: &Shared, path: &Path) -> Result<()> {
    let text = match fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(e) => return Err(e.into()),
    };
    let mut queued: BTreeMap<String, Job> = BTreeMap::new();
    let mut started: BTreeMap<String, u64> = BTreeMap::new();
    let mut finished = vec![];
    let mut seq = 0;
    for line in text.lines() {
        let Ok(v) = serde_json::from_str::<Value>(line) else { continue };
        let job = v["job"].as_str().unwrap_or("").to_string();
        match v["event"].as_str() {
            Some("queued") => {
                if let Ok(j) = serde_json::from_value::<Job>(v["request"].clone()) {
                    seq = seq.max(j.seq + 1);
                    queued.insert(job, j);
                }
            }
            Some("started") => {
                started.insert(job, v["unix"].as_u64().unwrap_or(0));
            }
            Some("finished") => {
                queued.remove(&job);
                started.remove(&job);
                if let Ok(f) = serde_json::from_value::<Finished>(v["result"].clone()) {
                    finished.push(f);
                }
            }
            _ => {}
        }
    }
    let mut s = sh.lock();
    s.seq = seq;
    for f in finished {
        s.finish(f);
    }
    for (job, t) in started {
        let Some(j) = queued.remove(&job) else { continue };
        let f = match receipt(&sh.cfg.run_root.join("runs").join(&job).join("survey")) {
            Some(r) => {
                let mut f = from_receipt(&j, t, &r);
                f.reason = Some(format!("observed after a restart{}", f.reason.map(|r| format!(": {r}")).unwrap_or_default()));
                f
            }
            None => Finished {
                job: job.clone(),
                id: j.id.clone(),
                title: j.title.clone(),
                state: "uncertain".into(),
                started_unix: t,
                finished_unix: now(),
                reason: Some("the service restarted during the attempt; it is not replayed".into()),
                manifest: None,
                words: None,
                teacher_called: None,
            },
        };
        sh.log(json!({"event":"finished","job":job,"result":f}));
        s.finish(f);
    }
    s.waiting = queued.into_values().collect();
    Ok(())
}

pub struct Service {
    shared: Arc<Shared>,
    workers: Vec<std::thread::JoinHandle<()>>,
    _lease: File,
}

/// What one describe argument came to.
#[derive(Clone, Debug, Serialize)]
pub struct Outcome {
    pub source: String,
    pub id: Option<String>,
    pub title: Option<String>,
    /// queued, waiting, running, described, failed_before, unknown, unreadable, full
    pub state: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub job: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub detail: Option<Value>,
}

impl Service {
    /// Take the service lease, check the pins, index the catalog, recover the journal and start
    /// the workers.
    pub fn start(cfg: AnimalConfig) -> Result<Service> {
        cfg.validate()?;
        private_dir(&cfg.run_root)?;
        private_dir(&cfg.run_root.join("runs"))?;
        let lease = lease(&cfg.run_root.join("service.lock")).map_err(|e| error(format!("another Whale service holds this run root ({e})")))?;
        if file_hash(&cfg.executable.path)? != cfg.executable.sha256 {
            return Err(error("executable hash mismatch"));
        }
        let spec = Spec::load(&cfg.survey)?;
        let catalog = Catalog::load(&cfg.catalog)?;
        let path = cfg.run_root.join("requests.jsonl");
        let journal = OpenOptions::new().create(true).append(true).mode(0o600).open(&path)?;
        let shared = Arc::new(Shared { cfg, spec, catalog, state: Mutex::new(State::default()), cond: Condvar::new(), journal: Mutex::new(journal) });
        recover(&shared, &path)?;
        let workers = (0..shared.cfg.workers)
            .map(|_| {
                let sh = shared.clone();
                std::thread::spawn(move || worker(sh))
            })
            .collect();
        Ok(Service { shared, workers, _lease: lease })
    }

    /// Stop admitting; running children are waited for (they may finish their one call).
    pub fn stop(&mut self) {
        self.shared.lock().stop = true;
        self.shared.cond.notify_all();
        for w in self.workers.drain(..) {
            let _ = w.join();
        }
    }

    pub fn noun(&self) -> &str {
        &self.shared.cfg.noun
    }

    /// Queue one request per argument that names a catalog source not already described,
    /// waiting, running or failed before.
    pub fn describe(&self, sources: &[String], job: &str, urgent: bool) -> Vec<Outcome> {
        let sh = &self.shared;
        let mut out = vec![];
        let mut seen = BTreeSet::new();
        for (n, src) in sources.iter().enumerate() {
            let mut o = Outcome { source: src.clone(), id: None, title: None, state: "unknown".into(), job: None, detail: None };
            let Some(id) = sh.catalog.resolve(src) else {
                out.push(o);
                continue;
            };
            o.id = Some(id.clone());
            if !seen.insert(id.clone()) {
                continue;
            }
            let row = match sh.catalog.row(&id) {
                Ok(r) => r,
                Err(e) => {
                    o.state = "unreadable".into();
                    o.detail = Some(json!(e.to_string()));
                    out.push(o);
                    continue;
                }
            };
            o.title = sh.catalog.title(&row);
            {
                let s = sh.lock();
                if let Some(r) = s.running.iter().find(|r| r.job.id == id) {
                    o.state = "running".into();
                    o.job = Some(r.job.job.clone());
                } else if let Some(w) = s.waiting.iter().find(|w| w.id == id) {
                    o.state = "waiting".into();
                    o.job = Some(w.job.clone());
                } else if let Some(f) = s.last.get(&id).filter(|f| f.state != "completed") {
                    o.state = "failed_before".into();
                    o.job = Some(f.job.clone());
                    o.detail = Some(json!(f));
                }
            }
            if o.state == "unknown" {
                if let Some((p, m, _)) = sh.bucket(&id).ok().and_then(|b| versions(&b).pop()) {
                    o.state = "described".into();
                    o.detail = Some(json!({"manifest":p,"words":m["length_measure"]["count"]}));
                }
            }
            if o.state != "unknown" {
                out.push(o);
                continue;
            }
            let item = match sh.catalog.item(&id, &row) {
                Ok(i) => i,
                Err(e) => {
                    o.state = "unreadable".into();
                    o.detail = Some(json!(e.to_string()));
                    out.push(o);
                    continue;
                }
            };
            let name = if sources.len() == 1 { job.to_string() } else { format!("{job}-{n}") };
            let mut s = sh.lock();
            if s.stop {
                o.state = "stopping".into();
            } else if s.waiting.len() >= sh.cfg.max_waiting {
                o.state = "full".into();
                o.detail = Some(json!(sh.cfg.max_waiting));
            } else {
                let j = Job { job: name.clone(), id: id.clone(), title: o.title.clone(), item, urgent, seq: s.seq, queued_unix: now() };
                s.seq += 1;
                // Ahead of it: every waiting urgent request, and every waiting one if it is normal.
                let ahead = s.waiting.iter().filter(|w| w.urgent || !urgent).count();
                sh.log(json!({"event":"queued","job":j.job,"request":j}));
                s.waiting.push(j);
                sh.cond.notify_all();
                o.state = "queued".into();
                o.job = Some(name);
                o.detail = Some(json!({"ahead":ahead,"running":s.running.len(),"paused":sh.paused(),"urgent":urgent}));
            }
            out.push(o);
        }
        out
    }

    /// Counts: sources with a complete bundle in the store, and this service's requests.
    pub fn count(&self) -> Result<Value> {
        let sh = &self.shared;
        let dir = sh.store()?.join(&sh.spec.bundle.store_prefix).join(&sh.cfg.catalog.kind);
        let described = match fs::read_dir(&dir) {
            Ok(rd) => rd.flatten().filter(|e| !e.file_name().to_string_lossy().starts_with('.') && !versions(&e.path()).is_empty()).count(),
            Err(_) => 0,
        };
        let s = sh.lock();
        Ok(json!({"described":described,"waiting":s.waiting.len(),"running":s.running.len(),"totals":s.totals,"paused":sh.paused(),"noun":sh.cfg.noun}))
    }

    /// The state of one source, or of the whole service.
    pub fn status(&self, source: Option<&str>) -> Result<Value> {
        let sh = &self.shared;
        let Some(src) = source else {
            let s = sh.lock();
            let running: Vec<Value> = s.running.iter().map(|r| json!({"job":r.job.job,"id":r.job.id,"title":r.job.title,"started_unix":r.started_unix,"urgent":r.job.urgent})).collect();
            let waiting: Vec<Value> = s.ordered_waiting().iter().map(|j| json!({"job":j.job,"id":j.id,"urgent":j.urgent})).collect();
            return Ok(json!({"running":running,"waiting":waiting,"paused":sh.paused(),"last":s.finished.back(),"noun":sh.cfg.noun}));
        };
        let Some(id) = sh.catalog.resolve(src) else {
            return Ok(json!({"source":src,"state":"unknown"}));
        };
        let title = sh.catalog.row(&id).ok().and_then(|r| sh.catalog.title(&r));
        let base = json!({"source":src,"id":id,"title":title});
        let with = |mut v: Value, extra: Value| {
            for (k, x) in extra.as_object().into_iter().flatten() {
                v[k] = x.clone();
            }
            v
        };
        {
            let s = sh.lock();
            if let Some(r) = s.running.iter().find(|r| r.job.id == id) {
                return Ok(with(base, json!({"state":"running","job":r.job.job,"started_unix":r.started_unix})));
            }
            let order = s.ordered_waiting();
            if let Some(i) = order.iter().position(|w| w.id == id) {
                return Ok(with(base, json!({"state":"waiting","job":order[i].job,"ahead":i,"paused":sh.paused()})));
            }
            if let Some(f) = s.last.get(&id) {
                if f.state != "completed" {
                    return Ok(with(base, json!({"state":"finished","result":f})));
                }
            }
        }
        if let Some((p, m, t)) = versions(&sh.bucket(&id)?).pop() {
            let unix = t.duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
            return Ok(with(base, json!({"state":"described","manifest":p,"words":m["length_measure"]["count"],"written_unix":unix})));
        }
        Ok(with(base, json!({"state":"not_described"})))
    }

    /// The latest description of one source: where its bundle is and how it begins.
    pub fn show(&self, source: &str) -> Result<Value> {
        let st = self.status(Some(source))?;
        if st["state"] != "described" {
            return Ok(st);
        }
        let sh = &self.shared;
        let manifest = PathBuf::from(st["manifest"].as_str().unwrap_or(""));
        let dir = manifest.parent().ok_or("manifest without a directory")?;
        let pointer = &sh.spec.output.length.pointer;
        let artifact = sh.spec.bundle.text_artifacts.iter().find(|a| &a.pointer == pointer).map(|a| a.path.clone());
        let mut v = st.clone();
        v["bundle"] = json!(dir);
        if let Some(a) = artifact {
            let p = dir.join(&a);
            let mut b = Vec::new();
            File::open(&p)?.take(DESCRIPTION_MAX as u64).read_to_end(&mut b)?;
            v["artifact"] = json!(p);
            v["opening"] = json!(answer::opening(&String::from_utf8_lossy(&b), 320));
        }
        Ok(v)
    }

    /// Write or remove the PAUSE marker of the run root.
    pub fn marker(&self, pause: bool) -> Result<Value> {
        let sh = &self.shared;
        let p = sh.cfg.run_root.join("PAUSE");
        let was = p.exists();
        if pause && !was {
            crate::util::create(&p, &json!({"owner":"mova-whale serve","unix":now()}))?;
        }
        if !pause && was {
            fs::remove_file(&p)?;
            File::open(&sh.cfg.run_root)?.sync_all()?;
        }
        sh.cond.notify_all();
        let s = sh.lock();
        Ok(json!({"paused":pause,"was_paused":was,"running":s.running.len(),"waiting":s.waiting.len(),"noun":sh.cfg.noun}))
    }

    /// Answer one prompt: plan and execute through the deputy with this animal's hands, then
    /// realize the English answer from the report.
    pub fn answer(&self, deputy: &Deputy, id: &str, prompt: &str, urgent: bool) -> anyhow::Result<Value> {
        let hands = Hands { svc: self, urgent };
        let overrides: BTreeMap<String, &dyn CapabilityHandler> = BTreeMap::from([("whale".to_owned(), &hands as &dyn CapabilityHandler), ("shark".to_owned(), &hands as &dyn CapabilityHandler)]);
        let report = deputy.run_with(prompt, id, true, &overrides)?;
        Ok(json!({"answer":mova_octopus::realize::report(&report),"report":compact(&report)}))
    }
}

impl Drop for Service {
    fn drop(&mut self) {
        self.stop();
    }
}

/// The structured part of a reply; the native observations stay in the deputy ledger.
fn compact(r: &Report) -> Value {
    let followups: Vec<Value> = r.followups.iter().map(|f| json!({"candidate_id":f["candidate_id"],"suggested_lane":f["suggested_lane"],"status":f["status"]})).collect();
    json!({"prompt_sha256":r.prompt_sha256,"plan":r.plan,"results":r.results,"failure":r.failure,"followups":followups,"training_authorized":r.training_authorized})
}

/// Check the deputy configuration against this animal: every `whale` capability names a known
/// operation with the slot that operation reads.
pub fn check_capabilities(d: &mova_octopus::Config) -> std::result::Result<(), String> {
    let mut seen = BTreeSet::new();
    for (name, c) in d.capabilities.iter().filter(|(_, c)| matches!(c.handler.as_str(), "whale" | "shark")) {
        let op = c.parameters["operation"].as_str().ok_or(format!("{name}: operation required"))?;
        let (_, slot) = OPERATIONS.iter().find(|(o, _)| *o == op).ok_or(format!("{name}: unknown operation {op}"))?;
        let names: Vec<&str> = c.slots.iter().map(|s| s.name.as_str()).collect();
        if names != slot.iter().copied().collect::<Vec<_>>() {
            return Err(format!("{name}: operation {op} reads slot {slot:?}, configured {names:?}"));
        }
        if c.parameters.as_object().is_some_and(|m| m.len() != 1) {
            return Err(format!("{name}: parameters are operation only"));
        }
        // Only reading operations may answer a question (`IntentRule::observes`).
        if c.intent.observes && ACTING.contains(&op) {
            return Err(format!("{name}: operation {op} acts; it cannot be declared observing"));
        }
        seen.insert(op);
    }
    if seen.is_empty() {
        return Err("no whale capability configured".into());
    }
    Ok(())
}

/// The capability handler for one request (its priority is known only per request).
struct Hands<'a> {
    svc: &'a Service,
    urgent: bool,
}

impl CapabilityHandler for Hands<'_> {
    fn execute(&self, p: &Value, a: &Action, invocation_id: &str) -> anyhow::Result<Value> {
        let svc = self.svc;
        let e = |x: Box<dyn std::error::Error>| anyhow::anyhow!("{x}");
        let one = a.arguments.get("source").and_then(|v| v.first()).map(String::as_str);
        let noun = svc.noun();
        let mut out = match p["operation"].as_str() {
            Some("describe") => {
                let sources = a.arguments.get("sources").cloned().unwrap_or_default();
                anyhow::ensure!(!sources.is_empty(), "no source to describe");
                let o = svc.describe(&sources, invocation_id, self.urgent);
                json!({"operation":"describe","answer":answer::describe(noun, &o),"result":o})
            }
            Some("count") => {
                let v = svc.count().map_err(e)?;
                json!({"operation":"count","answer":answer::count(&v),"result":v})
            }
            Some("status") => {
                let v = svc.status(one).map_err(e)?;
                let text = if one.is_some() { answer::source_status(noun, &v) } else { answer::status(&v) };
                json!({"operation":"status","answer":text,"result":v})
            }
            Some("show") => {
                let v = svc.show(one.ok_or_else(|| anyhow::anyhow!("slot source is empty"))?).map_err(e)?;
                let text = if v["state"] == "described" { answer::show(noun, &v) } else { answer::source_status(noun, &v) };
                json!({"operation":"show","answer":text,"result":v})
            }
            Some(op @ ("pause" | "resume")) => {
                let v = svc.marker(op == "pause").map_err(e)?;
                json!({"operation":op,"answer":answer::marker(&v),"result":v})
            }
            other => anyhow::bail!("unknown whale operation {other:?}"),
        };
        out["completion"] = json!("answered from the journal, the shared state and the store; descriptions run in the service's own workers");
        Ok(out)
    }
}

/// `mova-whale serve`: load both configurations, start the service, serve the private socket under
/// the angel (restarts after failures; `STOP` in the socket directory ends it), then stop
/// admitting and wait for running children.
pub fn serve(config: &Path, deputy_config: &Path, socket: &Path) -> anyhow::Result<()> {
    let cfg = AnimalConfig::load(config).map_err(|e| anyhow::anyhow!("{e}"))?;
    let dcfg: mova_octopus::Config = serde_json::from_slice(&fs::read(deputy_config)?)?;
    check_capabilities(&dcfg).map_err(anyhow::Error::msg)?;
    let deputy = Deputy::load(dcfg)?;
    let dir = socket.parent().ok_or_else(|| anyhow::anyhow!("socket needs a directory"))?;
    mova_octopus::socket::private_dir(dir)?;
    let mut svc = Service::start(cfg).map_err(|e| anyhow::anyhow!("{e}"))?;
    let r = mova_octopus::socket::angel_prioritized(
        socket,
        &|| Ok(|id: &str, _execute: bool, prompt: &str, urgent: bool| svc.answer(&deputy, id, prompt, urgent)),
        Duration::from_secs(60),
    );
    svc.stop();
    r
}

/// `mova-whale ask`: send arbitrary text; the reply is the English answer, then the JSON.
pub fn ask(socket: &Path, id: &str, prompt: &str, urgent: bool) -> anyhow::Result<Value> {
    mova_octopus::socket::ask_with(socket, id, true, prompt, urgent)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::spec::tests::fixture_spec;
    use std::os::unix::fs::PermissionsExt;
    use std::sync::atomic::{AtomicU64, Ordering};
    use std::time::Instant;
    static NEXT: AtomicU64 = AtomicU64::new(0);

    pub struct Fixture {
        pub dir: PathBuf,
        pub cfg: AnimalConfig,
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.dir);
        }
    }
    /// A catalog of `n` `loose` sources (the catalog is also the pinned metadata export) and a
    /// fake child executable: it waits for `<run_root>/go` when `gate` is set, then writes a
    /// terminal quarantine receipt and appends its task key to `<run_root>/order.log`. No teacher.
    fn fixture(n: usize, gate: bool) -> Fixture {
        let base = std::env::var_os("CARGO_TARGET_DIR").map(PathBuf::from).unwrap_or_else(std::env::temp_dir);
        fs::create_dir_all(&base).unwrap();
        let dir = base.join(format!("whale-animal-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
        let _ = fs::remove_dir_all(&dir);
        fs::create_dir(&dir).unwrap();
        fs::set_permissions(&dir, fs::Permissions::from_mode(0o700)).unwrap();
        let spec = fixture_spec(&dir, "words", 40, 50);
        let mut rows = String::new();
        for i in 0..n {
            let p = dir.join(format!("note{i}.txt"));
            fs::write(&p, format!("Note {i}: the counter increases by one per step.\n")).unwrap();
            rows.push_str(&json!({"id":format!("note-{i}-x"),"title":format!("Note {i}"),"text_path":p,"url":format!("https://notes.example.org/{i}")}).to_string());
            rows.push('\n');
        }
        let catalog = dir.join("catalog.jsonl");
        fs::write(&catalog, &rows).unwrap();
        // The fake child: find the prepared task key, optionally wait for the gate, then receipt.
        let exe = dir.join("fake-whale");
        let wait = if gate { "while [ ! -f \"$run/../../../go\" ]; do sleep 0.02; done\n" } else { "" };
        fs::write(
            &exe,
            format!(
                "#!/bin/sh\nrun=$(dirname \"$(dirname \"$(dirname \"$2\")\")\")\n{wait}key=$(sed -n 's/.*\"key\": *\"\\([0-9a-f]\\{{64\\}}\\)\".*/\\1/p' \"$run/manifest.json\" | head -n 1)\nprintf '{{\"task_sha256\":\"%s\",\"outcome\":\"quarantined\",\"reason\":\"length gate: fixture\"}}' \"$key\" > \"$run/jobs/$key/receipt.json\"\necho \"$key\" >> \"$run/../../../order.log\"\n"
            ),
        )
        .unwrap();
        fs::set_permissions(&exe, fs::Permissions::from_mode(0o700)).unwrap();
        let survey = json!({"spec":spec,"teacher":{"model":"gpt-fixture","effort":"medium"},"originality":{"comparison_roots":[]},"store":dir.join("store"),"storage_floor_bytes":1,
            "metadata_export":{"path":catalog,"sha256":file_hash(&catalog).unwrap()}});
        let cfg = AnimalConfig {
            run_root: dir.join("run"),
            executable: PinConfig { path: exe.clone(), sha256: file_hash(&exe).unwrap() },
            codex: Some(dir.join("no-codex-in-unit-tests")),
            survey,
            catalog: CatalogConfig {
                path: catalog.clone(),
                sha256: file_hash(&catalog).unwrap(),
                kind: "loose".into(),
                id_field: "id".into(),
                path_field: "text_path".into(),
                title_field: Some("title".into()),
                licence_field: None,
                formats: BTreeMap::from([(".txt".into(), "plain".into())]),
                url_fields: vec!["url".into()],
                url_prefixes: vec![],
            },
            workers: 1,
            max_waiting: 8,
            seaspider_socket: None,
            noun: "note".into(),
        };
        Fixture { dir, cfg }
    }
    fn wait_until(what: &str, f: impl Fn() -> bool) {
        let t = Instant::now();
        while !f() {
            assert!(t.elapsed() < Duration::from_secs(20), "timed out waiting for {what}");
            std::thread::sleep(Duration::from_millis(20));
        }
    }
    fn finished(svc: &Service) -> usize {
        svc.shared.lock().totals.values().sum::<u64>() as usize
    }

    /// Opt-in check of a deployment (never part of this crate's data): MOVA_WHALE_ANIMAL_CHECK =
    /// `CONFIG:DEPUTY_CONFIG`. Validates both files and the catalog pin without serving, starting
    /// workers or calling anyone; the executable pin is not checked. Without it: a skip.
    #[test]
    fn deployment_config_when_configured() {
        let Some(arg) = std::env::var_os("MOVA_WHALE_ANIMAL_CHECK") else {
            eprintln!("SKIPPED: MOVA_WHALE_ANIMAL_CHECK not configured");
            return;
        };
        let arg = arg.to_string_lossy().to_string();
        let (c, d) = arg.split_once(':').expect("CONFIG:DEPUTY_CONFIG");
        let mut raw = load(Path::new(c), 2 << 20).unwrap();
        if !raw["executable"]["sha256"].as_str().is_some_and(valid_hash) {
            raw["executable"]["sha256"] = json!("0".repeat(64)); // an unfrozen binary: placeholder
        }
        let cfg: AnimalConfig = serde_json::from_value(raw).unwrap();
        cfg.validate().unwrap();
        let dcfg: mova_octopus::Config = serde_json::from_slice(&fs::read(d).unwrap()).unwrap();
        dcfg.validate().unwrap();
        check_capabilities(&dcfg).unwrap();
        let catalog = Catalog::load(&cfg.catalog).unwrap();
        eprintln!("checked: {} catalog rows, noun {}, {} workers", catalog.len(), cfg.noun, cfg.workers);
    }
    #[test]
    fn config_gates_are_real() {
        let f = fixture(1, false);
        assert!(f.cfg.validate().is_ok());
        let mut c = f.cfg.clone();
        c.survey["items"] = json!([]);
        assert!(c.validate().unwrap_err().to_string().contains("items"));
        let mut c = f.cfg.clone();
        c.workers = 5;
        assert!(c.validate().is_err());
        let mut c = f.cfg.clone();
        c.catalog.formats = BTreeMap::from([(".txt".into(), "pdf".into())]);
        assert!(c.validate().is_err());
        let mut c = f.cfg.clone();
        c.catalog.sha256 = "0".repeat(64);
        assert!(Service::start(c).err().unwrap().to_string().contains("catalog hash"));
    }

    #[test]
    fn catalog_resolves_ids_and_urls_only() {
        let f = fixture(2, false);
        let c = Catalog::load(&f.cfg.catalog).unwrap();
        assert_eq!(c.len(), 2);
        assert_eq!(c.resolve("note-1-x").as_deref(), Some("note-1-x"));
        assert_eq!(c.resolve("https://notes.example.org/0/").as_deref(), Some("note-0-x"));
        assert_eq!(c.resolve("note-9-x"), None);
        assert_eq!(c.resolve("https://other.example.org/0"), None);
        let item = c.item("note-0-x", &c.row("note-0-x").unwrap()).unwrap();
        assert_eq!((item["format"].as_str(), item["kind"].as_str()), (Some("plain"), Some("loose")));
    }

    #[test]
    fn priority_orders_waiting_work_and_never_preempts() {
        let _serial = crate::serial();
        let f = fixture(4, true);
        let svc = Service::start(f.cfg.clone()).unwrap();
        let q = svc.describe(&["note-0-x".into()], "n0", false);
        assert_eq!(q[0].state, "queued");
        wait_until("the first request to run", || svc.shared.lock().running.len() == 1);
        svc.describe(&["note-1-x".into()], "n1", false);
        svc.describe(&["note-2-x".into()], "n2", false);
        let u = svc.describe(&["note-3-x".into()], "u3", true);
        // Ahead of an urgent request: only waiting urgent ones; the running one is not preempted.
        assert_eq!(u[0].detail.as_ref().unwrap()["ahead"], 0);
        assert_eq!(svc.shared.lock().running[0].job.job, "n0");
        assert_eq!(svc.status(Some("note-3-x")).unwrap()["ahead"], 0);
        assert_eq!(svc.status(Some("note-2-x")).unwrap()["ahead"], 2);
        fs::write(f.cfg.run_root.join("go"), b"").unwrap();
        wait_until("all four requests", || finished(&svc) == 4);
        assert_eq!(fs::read_to_string(f.cfg.run_root.join("order.log")).unwrap().lines().count(), 4);
        let jobs: Vec<String> = fs::read_to_string(f.cfg.run_root.join("requests.jsonl"))
            .unwrap()
            .lines()
            .filter_map(|l| serde_json::from_str::<Value>(l).ok())
            .filter(|v| v["event"] == "started")
            .map(|v| v["job"].as_str().unwrap().to_string())
            .collect();
        assert_eq!(jobs, ["n0", "u3", "n1", "n2"]);
    }

    #[test]
    fn duplicates_failures_and_unknown_sources_queue_nothing() {
        let _serial = crate::serial();
        let f = fixture(2, false);
        let svc = Service::start(f.cfg.clone()).unwrap();
        let o = svc.describe(&["note-0-x".into(), "https://notes.example.org/0".into(), "missing-2024-x".into()], "a", false);
        assert_eq!(o.iter().map(|o| o.state.as_str()).collect::<Vec<_>>(), ["queued", "unknown"]);
        wait_until("the request", || finished(&svc) == 1);
        // The fake child quarantines: the same source is not attempted again.
        let again = svc.describe(&["note-0-x".into()], "b", false);
        assert_eq!(again[0].state, "failed_before");
        assert_eq!(svc.status(Some("note-0-x")).unwrap()["result"]["state"], "quarantined");
        assert_eq!(svc.status(Some("note-1-x")).unwrap()["state"], "not_described");
        assert_eq!(svc.status(Some("nobody-1-x")).unwrap()["state"], "unknown");
        assert_eq!(svc.count().unwrap()["totals"]["quarantined"], 1);
    }

    #[test]
    fn pause_blocks_admission_and_restart_never_relaunches_a_started_request() {
        let _serial = crate::serial();
        let f = fixture(3, true);
        let mut svc = Service::start(f.cfg.clone()).unwrap();
        svc.describe(&["note-0-x".into()], "s0", false);
        wait_until("the first request to run", || svc.shared.lock().running.len() == 1);
        assert_eq!(svc.marker(true).unwrap()["was_paused"], false);
        svc.describe(&["note-1-x".into()], "s1", false);
        // Second service on the same run root: refused while the first holds the lease.
        assert!(Service::start(f.cfg.clone()).err().unwrap().to_string().contains("another Whale service"));
        fs::write(f.cfg.run_root.join("go"), b"").unwrap();
        wait_until("the running request", || finished(&svc) == 1);
        std::thread::sleep(Duration::from_millis(600));
        assert_eq!(svc.shared.lock().waiting.len(), 1, "paused: nothing admitted");
        svc.stop();
        drop(svc);
        // A forged started line without finished, for a request whose run has no receipt.
        let journal = f.cfg.run_root.join("requests.jsonl");
        let mut j = OpenOptions::new().append(true).open(&journal).unwrap();
        let ghost = Job { job: "g2".into(), id: "note-2-x".into(), title: None, item: json!({}), urgent: false, seq: 99, queued_unix: 0 };
        writeln!(j, "{}", json!({"event":"queued","job":"g2","request":ghost})).unwrap();
        writeln!(j, "{}", json!({"event":"started","job":"g2","id":"note-2-x","unix":1})).unwrap();
        drop(j);
        let svc = Service::start(f.cfg.clone()).unwrap();
        let st = svc.status(Some("note-2-x")).unwrap();
        assert_eq!((st["state"].as_str(), st["result"]["state"].as_str()), (Some("finished"), Some("uncertain")));
        // The queued, never started request waits again; the uncertain one is not relaunched.
        assert_eq!(svc.status(Some("note-1-x")).unwrap()["state"], "waiting");
        assert!(!f.cfg.run_root.join("runs/g2").exists());
        assert_eq!(svc.marker(false).unwrap()["was_paused"], true);
        wait_until("the resumed request", || finished(&svc) == 3);
        assert!(!f.cfg.run_root.join("runs/g2").exists());
    }
}
