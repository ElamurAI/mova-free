//! Sea Spider as one of Mova's animals: it takes ordinary English through Mova's native deputy
//! (`mova_octopus::Deputy`: Mova's annotator, world reader, configured capabilities and typed URL
//! and host arguments bound through the parse) and answers in English realized by `answer.rs`,
//! with the structured result alongside.
//!
//! One fetch stream: a single worker thread owns the only `Crawler` (and so the store's writer
//! lock, the robots.txt cache and the per-IP timers) and runs queued jobs one at a time, urgent
//! jobs first, then by arrival. A prompt never interrupts running work; priority only orders the
//! waiting queue. Fetch prompts are queued and answered at once; questions (status, store counts,
//! a stored page, a robots decision with a fresh stored robots.txt) are answered from the job
//! journal, the shared job state and the store journal without waiting for the worker. A robots
//! question without a fresh stored robots.txt queues a check and waits for it only while the
//! worker is otherwise idle, at most `ROBOTS_WAIT`.
//!
//! The job journal `jobs.jsonl` (in the private socket directory) gets one fsynced line per
//! event: queued, started, url (one outcome), finished. A job queued or started before a restart
//! without a finished line is recorded as interrupted and never replayed automatically.
//!
//! Who may ask: the request's declared caller (`from`; none means the Octopus) must be on the
//! configured allowlist (`service.callers`; default the Octopus, Crab and the Whales).
//! `requests.jsonl` in the socket directory gets one fsynced line per request: id, caller, the
//! connecting pid and its executable, priority, the prompt's sha256 and whether it was admitted.
//! A caller off the list gets an English refusal and nothing runs.
//!
//! Singleton: `serve` first takes the host-wide lease (`singleton.rs`); a second start reports
//! the running Sea Spider and exits.
use crate::answer;
use crate::config::Config;
use crate::crawl::{self, Crawler};
use crate::store::Reader;
use mova_octopus::socket::Request;
use mova_octopus::{Action, CapabilityHandler, Deputy, Report};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::collections::{BTreeMap, BTreeSet, VecDeque};
use std::fs::{File, OpenOptions};
use std::io::Write;
use std::path::{Path, PathBuf};
use std::sync::{Arc, Condvar, Mutex};
use std::time::{Duration, Instant};

/// Most jobs waiting at once; a further fetch prompt is refused with an English reason.
pub const MAX_WAITING: usize = 64;
/// How long a robots question waits for its own check when nothing else is running.
pub const ROBOTS_WAIT: Duration = Duration::from_secs(15);
/// Finished jobs kept in memory for status answers.
const KEEP_FINISHED: usize = 32;
/// The capability operations this animal installs, and the slot each needs.
pub const OPERATIONS: [(&str, Option<&str>); 5] =
    [("fetch", Some("urls")), ("robots", Some("target")), ("stats", Some("host")), ("page", Some("url")), ("status", None)];
/// Operations that change state (start work); never declared `observes` in the deputy configuration.
pub const ACTING: [&str; 1] = ["fetch"];

#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(tag = "kind", rename_all = "snake_case")]
pub enum JobKind {
    Fetch { urls: Vec<String> },
    Robots { url: String },
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Job {
    pub job: String,
    #[serde(flatten)]
    pub kind: JobKind,
    pub urgent: bool,
    pub seq: u64,
    pub queued_unix: u64,
}

/// One URL of a fetch job as it ended.
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct UrlOutcome {
    pub url: String,
    pub outcome: String,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub status: Option<u16>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub rule: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub bytes: Option<u64>,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Finished {
    pub job: String,
    pub kind: String,
    pub state: String,
    pub started_unix: u64,
    pub finished_unix: u64,
    pub outcomes: Vec<UrlOutcome>,
    pub not_started: u64,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub robots: Option<Value>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub error: Option<String>,
}

struct Running {
    job: Job,
    started_unix: u64,
    done: Vec<UrlOutcome>,
    current: Option<String>,
}

#[derive(Default)]
struct State {
    waiting: Vec<Job>,
    running: Option<Running>,
    finished: VecDeque<Finished>,
    seq: u64,
    stop: bool,
}

impl State {
    fn next(&mut self) -> Option<Job> {
        let i = (0..self.waiting.len()).min_by_key(|&i| (!self.waiting[i].urgent, self.waiting[i].seq))?;
        Some(self.waiting.remove(i))
    }
    fn total_urls(job: &Job) -> u64 {
        match &job.kind {
            JobKind::Fetch { urls } => urls.len() as u64,
            JobKind::Robots { .. } => 1,
        }
    }
}

struct Shared {
    state: Mutex<State>,
    cond: Condvar,
    journal: Mutex<File>,
}

impl Shared {
    fn log(&self, v: Value) {
        let mut f = self.journal.lock().unwrap_or_else(|p| p.into_inner());
        let mut line = v.to_string();
        line.push('\n');
        if let Err(e) = f.write_all(line.as_bytes()).and_then(|_| f.sync_data()) {
            eprintln!("seaspider: job journal: {e}");
        }
    }
    fn lock(&self) -> std::sync::MutexGuard<'_, State> {
        self.state.lock().unwrap_or_else(|p| p.into_inner())
    }
}

pub struct Service {
    cfg: Config,
    store: PathBuf,
    shared: Arc<Shared>,
    requests: Mutex<File>,
    worker: Option<std::thread::JoinHandle<()>>,
}

/// Check the deputy configuration against this animal: every `seaspider` capability names a known
/// operation with the slot that operation reads.
pub fn check_capabilities(d: &mova_octopus::Config) -> Result<(), String> {
    let mut seen = BTreeSet::new();
    for (name, c) in d.capabilities.iter().filter(|(_, c)| c.handler == "seaspider") {
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
        return Err("no seaspider capability configured".into());
    }
    Ok(())
}

fn ok_url_outcome(c: &Crawler, url: &str, sum: &crawl::Summary) -> UrlOutcome {
    let mut o = UrlOutcome { url: url.to_string(), outcome: "error".into(), status: None, rule: None, error: None, bytes: None };
    let rec = c.store.index.latest_page(url);
    if sum.skipped_done == 1 {
        o.outcome = "already_stored".into();
    } else if let Some(r) = rec {
        o.outcome = r.outcome.clone();
        o.error = r.error.clone();
    } else {
        o.error = Some("no page record was written".into());
    }
    if let Some(r) = rec {
        o.status = r.status;
        o.rule = r.robots.as_ref().and_then(|n| n.rule.clone()).filter(|_| r.outcome == "disallowed");
        o.bytes = r.body_bytes;
    }
    o
}

fn worker(shared: Arc<Shared>, mut crawler: Crawler) {
    let rt = match tokio::runtime::Builder::new_current_thread().enable_all().build() {
        Ok(rt) => rt,
        Err(e) => {
            eprintln!("seaspider: worker runtime: {e}");
            return;
        }
    };
    loop {
        let job = {
            let mut s = shared.lock();
            loop {
                if s.stop {
                    break None;
                }
                if let Some(j) = s.next() {
                    break Some(j);
                }
                s = shared.cond.wait(s).unwrap_or_else(|p| p.into_inner());
            }
        };
        let Some(job) = job else { break };
        let started = crawl::unix_now();
        shared.lock().running = Some(Running { job: job.clone(), started_unix: started, done: vec![], current: None });
        shared.log(json!({"event":"started","job":job.job,"unix":started}));
        let mut fin = Finished {
            job: job.job.clone(),
            kind: match job.kind {
                JobKind::Fetch { .. } => "fetch".into(),
                JobKind::Robots { .. } => "robots".into(),
            },
            state: "done".into(),
            started_unix: started,
            finished_unix: 0,
            outcomes: vec![],
            not_started: 0,
            robots: None,
            error: None,
        };
        let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| match &job.kind {
            JobKind::Fetch { urls } => {
                for (i, url) in urls.iter().enumerate() {
                    if shared.lock().stop {
                        fin.state = "stopped".into();
                        fin.not_started = (urls.len() - i) as u64;
                        break;
                    }
                    if let Some(r) = shared.lock().running.as_mut() {
                        r.current = Some(url.clone());
                    }
                    let o = match rt.block_on(crawler.run(std::slice::from_ref(url))) {
                        Ok(sum) => ok_url_outcome(&crawler, url, &sum),
                        Err(e) => UrlOutcome { url: url.clone(), outcome: "error".into(), status: None, rule: None, error: Some(e), bytes: None },
                    };
                    shared.log(json!({"event":"url","job":job.job,"outcome":o,"unix":crawl::unix_now()}));
                    if let Some(r) = shared.lock().running.as_mut() {
                        r.done.push(o.clone());
                        r.current = None;
                    }
                    fin.outcomes.push(o);
                }
            }
            JobKind::Robots { url } => match rt.block_on(crawler.explain(url)) {
                Ok(v) => fin.robots = Some(v),
                Err(e) => {
                    fin.state = "failed".into();
                    fin.error = Some(e);
                }
            },
        }));
        if res.is_err() {
            fin.state = "failed".into();
            fin.error = Some("the job panicked; it is not retried".into());
        }
        fin.finished_unix = crawl::unix_now();
        shared.log(json!({"event":"finished","job":job.job,"result":fin}));
        let mut s = shared.lock();
        s.running = None;
        s.finished.push_back(fin);
        while s.finished.len() > KEEP_FINISHED {
            s.finished.pop_front();
        }
        shared.cond.notify_all();
    }
    // Waiting jobs at a stop are recorded, not run.
    let left: Vec<Job> = std::mem::take(&mut shared.lock().waiting);
    for j in left {
        shared.log(json!({"event":"finished","job":j.job,"result":{"job":j.job,"state":"not_started","unix":crawl::unix_now()}}));
    }
}

/// Read the job journal: the next sequence number, the last finished jobs, and the jobs that
/// were queued or started but never finished (recorded as interrupted now, not replayed).
fn recover(path: &Path) -> Result<(u64, VecDeque<Finished>, Vec<Value>), String> {
    let text = match std::fs::read_to_string(path) {
        Ok(t) => t,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => String::new(),
        Err(e) => return Err(format!("{}: {e}", path.display())),
    };
    let mut seq = 0;
    let mut open: BTreeMap<String, Value> = BTreeMap::new();
    let mut finished = VecDeque::new();
    for line in text.lines() {
        let Ok(v) = serde_json::from_str::<Value>(line) else { continue };
        let job = v["job"].as_str().unwrap_or("").to_string();
        match v["event"].as_str() {
            Some("queued") => {
                seq = seq.max(v["seq"].as_u64().unwrap_or(0) + 1);
                open.insert(job, v);
            }
            Some("finished") => {
                open.remove(&job);
                if let Ok(f) = serde_json::from_value::<Finished>(v["result"].clone()) {
                    finished.push_back(f);
                    if finished.len() > KEEP_FINISHED {
                        finished.pop_front();
                    }
                }
            }
            _ => {}
        }
    }
    Ok((seq, finished, open.into_values().collect()))
}

impl Service {
    /// Open the store (taking its writer lock in the worker), recover the job journal in `dir`
    /// and start the single worker.
    pub fn start(cfg: Config, store: PathBuf, dir: &Path) -> Result<Service, String> {
        let path = dir.join("jobs.jsonl");
        let (seq, mut finished, interrupted) = recover(&path)?;
        let journal = OpenOptions::new().create(true).append(true).open(&path).map_err(|e| format!("{}: {e}", path.display()))?;
        let rpath = dir.join("requests.jsonl");
        let requests = OpenOptions::new().create(true).append(true).open(&rpath).map_err(|e| format!("{}: {e}", rpath.display()))?;
        let shared = Arc::new(Shared { state: Mutex::new(State::default()), cond: Condvar::new(), journal: Mutex::new(journal) });
        for j in interrupted {
            let f = Finished {
                job: j["job"].as_str().unwrap_or("").into(),
                kind: j["kind"].as_str().unwrap_or("").into(),
                state: "interrupted".into(),
                started_unix: 0,
                finished_unix: crawl::unix_now(),
                outcomes: vec![],
                not_started: j["urls"].as_array().map_or(1, |a| a.len() as u64),
                robots: None,
                error: Some("the service restarted before the job finished; it is not replayed".into()),
            };
            shared.log(json!({"event":"finished","job":f.job,"result":f}));
            finished.push_back(f);
        }
        while finished.len() > KEEP_FINISHED {
            finished.pop_front();
        }
        {
            let mut s = shared.lock();
            s.seq = seq;
            s.finished = finished;
        }
        let (tx, rx) = std::sync::mpsc::sync_channel(1);
        let (c2, s2, sh) = (cfg.clone(), store.clone(), shared.clone());
        let worker = std::thread::spawn(move || match Crawler::new(c2, &s2) {
            Ok(c) => {
                let _ = tx.send(Ok(()));
                worker(sh, c)
            }
            Err(e) => {
                let _ = tx.send(Err(e));
            }
        });
        rx.recv().map_err(|e| e.to_string())??;
        Ok(Service { cfg, store, shared, requests: Mutex::new(requests), worker: Some(worker) })
    }

    /// Ask the worker to stop after the URL it is fetching and wait for it.
    pub fn stop(&mut self) {
        self.shared.lock().stop = true;
        self.shared.cond.notify_all();
        if let Some(w) = self.worker.take() {
            let _ = w.join();
        }
    }

    fn enqueue(&self, job: &str, kind: JobKind, urgent: bool, asked_by: &str) -> Result<(usize, bool), String> {
        let mut s = self.shared.lock();
        if s.stop {
            return Err("I am stopping".into());
        }
        if s.waiting.len() >= MAX_WAITING {
            return Err(format!("{MAX_WAITING} jobs are already waiting"));
        }
        if s.waiting.iter().any(|j| j.job == job) || s.running.as_ref().is_some_and(|r| r.job.job == job) {
            return Err(format!("{job} is already queued"));
        }
        let j = Job { job: job.into(), kind, urgent, seq: s.seq, queued_unix: crawl::unix_now() };
        s.seq += 1;
        // Ahead of it: every waiting urgent job, and every waiting normal job if it is normal.
        let ahead = s.waiting.iter().filter(|w| w.urgent || !urgent).count();
        let busy = s.running.is_some();
        self.shared.log(json!({"event":"queued","job":j.job,"kind":match &j.kind { JobKind::Fetch{..} => "fetch", JobKind::Robots{..} => "robots" },
            "urls":match &j.kind { JobKind::Fetch{urls} => json!(urls), JobKind::Robots{url} => json!([url]) },"urgent":urgent,"asked_by":asked_by,"seq":j.seq,"unix":j.queued_unix}));
        s.waiting.push(j);
        self.shared.cond.notify_all();
        Ok((ahead, busy))
    }

    /// The status of the stream as JSON (`answer::status` input).
    pub fn status(&self) -> Value {
        let s = self.shared.lock();
        let running = s.running.as_ref().map(|r| {
            json!({"job":r.job.job,"started_unix":r.started_unix,"urls":State::total_urls(&r.job),"done":r.done.len(),
                "current":r.current,"urgent":r.job.urgent})
        });
        let waiting: Vec<Value> = {
            let mut w: Vec<&Job> = s.waiting.iter().collect();
            w.sort_by_key(|j| (!j.urgent, j.seq));
            w.iter().map(|j| json!({"job":j.job,"urgent":j.urgent,"urls":State::total_urls(j)})).collect()
        };
        json!({"running":running,"waiting":waiting.len(),"queue":waiting,"last":s.finished.back()})
    }

    fn reader(&self) -> Result<Reader, String> {
        Reader::open(&self.store)
    }

    /// Store counts: distinct requested URLs whose latest page record is a fetched body (status
    /// below 400), disallowed or failed ones, optionally for one host.
    pub fn stats(&self, host: Option<&str>) -> Result<Value, String> {
        let r = self.reader()?;
        let mut latest: BTreeMap<&str, &crate::store::Record> = BTreeMap::new();
        for rec in r.index.recs.iter().filter(|x| x.kind == "page") {
            latest.insert(&rec.url, rec);
        }
        let host_of = |u: &str| reqwest::Url::parse(u).ok().and_then(|u| u.host_str().map(str::to_ascii_lowercase));
        let mut v = crate::store::stats(&r);
        let (mut pages, mut dis, mut failed, mut bytes) = (0u64, 0u64, 0u64, 0u64);
        let mut hosts = BTreeSet::new();
        for (url, rec) in &latest {
            let h = host_of(url);
            if host.is_some_and(|want| h.as_deref() != Some(want)) {
                continue;
            }
            match rec.outcome.as_str() {
                "fetched" if rec.has_body() && rec.status.is_some_and(|s| s < 400) => {
                    pages += 1;
                    bytes += rec.body_bytes.unwrap_or(0);
                    hosts.extend(h);
                }
                "disallowed" | "deferred" => dis += 1,
                _ => failed += 1,
            }
        }
        v["pages"] = pages.into();
        v["hosts"] = (hosts.len() as u64).into();
        v["disallowed"] = dis.into();
        v["failed"] = failed.into();
        v["page_bytes"] = bytes.into();
        if let Some(h) = host {
            v["host"] = h.into();
        }
        Ok(v)
    }

    /// What the store holds for one URL, or where it stands in the queue.
    pub fn page(&self, url: &str) -> Result<Value, String> {
        let u = crawl::normalize_url(url)?.to_string();
        {
            let s = self.shared.lock();
            if s.running.as_ref().is_some_and(|r| r.current.as_deref() == Some(u.as_str())) {
                return Ok(json!({"url":u,"state":"running"}));
            }
            if let Some(j) = s.waiting.iter().find(|j| matches!(&j.kind, JobKind::Fetch { urls } if urls.contains(&u))) {
                return Ok(json!({"url":u,"state":"queued","job":j.job}));
            }
        }
        let r = self.reader()?;
        let Some(rec) = r.index.latest_page(&u) else { return Ok(json!({"url":u,"state":"absent"})) };
        let mut v = json!({"url":u,"state":"record","record":rec});
        if rec.has_body() {
            let body = crate::store::read_body(&r.dir, rec)?;
            v["title"] = json!(answer::title(&body));
            let is_html = rec.content_type.as_deref().is_none_or(|c| c.to_ascii_lowercase().contains("html"));
            if is_html {
                let base = reqwest::Url::parse(rec.final_url.as_deref().unwrap_or(&u)).map_err(|e| e.to_string())?;
                v["links"] = json!(crate::links::outgoing(&base, &body).len());
            }
        }
        Ok(v)
    }

    /// A robots decision: from a fresh stored robots.txt, else from a queued check that is
    /// awaited only while the worker would otherwise be idle.
    pub fn robots(&self, target: &str, job: &str, urgent: bool, asked_by: &str) -> Result<Value, String> {
        let url = if target.contains("://") { target.to_string() } else { format!("https://{target}/") };
        let url = crawl::normalize_url(&url)?.to_string();
        if let Some(v) = crawl::stored_explanation(&self.cfg, &self.reader()?, &url, crawl::unix_now())? {
            return Ok(json!({"state":"answered","explanation":v}));
        }
        let (ahead, busy) = self.enqueue(job, JobKind::Robots { url: url.clone() }, urgent, asked_by)?;
        if ahead == 0 && !busy {
            let until = Instant::now() + ROBOTS_WAIT;
            let mut s = self.shared.lock();
            loop {
                if let Some(f) = s.finished.iter().rev().find(|f| f.job == job) {
                    return match &f.robots {
                        Some(v) => Ok(json!({"state":"answered","explanation":v})),
                        None => Err(f.error.clone().unwrap_or_else(|| "the robots check failed".into())),
                    };
                }
                let left = until.saturating_duration_since(Instant::now());
                if left.is_zero() {
                    break;
                }
                s = self.shared.cond.wait_timeout(s, left).unwrap_or_else(|p| p.into_inner()).0;
            }
        }
        Ok(json!({"state":"queued","job":job,"url":url}))
    }

    /// Answer one prompt: plan and execute through the deputy with this animal's hands, then
    /// realize the English answer from the report.
    pub fn answer(&self, deputy: &Deputy, id: &str, prompt: &str, urgent: bool) -> anyhow::Result<Value> {
        self.answer_as(deputy, id, prompt, urgent, crate::config::IMPLICIT_CALLER)
    }

    fn answer_as(&self, deputy: &Deputy, id: &str, prompt: &str, urgent: bool, caller: &str) -> anyhow::Result<Value> {
        let hands = Hands { svc: self, urgent, caller };
        let overrides: BTreeMap<String, &dyn CapabilityHandler> = BTreeMap::from([("seaspider".to_owned(), &hands as &dyn CapabilityHandler)]);
        let report = deputy.run_with(prompt, id, true, &overrides)?;
        Ok(json!({"answer":answer::report(&report),"report":compact(&report)}))
    }
}

impl Service {
    /// Answer one socket request: check the caller against the allowlist, journal who asked,
    /// then answer (or refuse in English without running anything).
    pub fn answer_request(&self, deputy: &Deputy, r: &Request) -> anyhow::Result<Value> {
        let caller = r.from.as_deref().unwrap_or(crate::config::IMPLICIT_CALLER);
        let admitted = self.cfg.admits(caller);
        let exe = std::fs::read_link(format!("/proc/{}/exe", r.peer_pid)).ok().map(|p| p.display().to_string());
        let line = json!({"unix":crawl::unix_now(),"id":r.id,"from":caller,"declared":r.from.is_some(),"peer_pid":r.peer_pid,
            "peer_exe":exe,"urgent":r.urgent,"execute":r.execute,"prompt_sha256":mova_octopus::digest(r.prompt.as_bytes()),"admitted":admitted});
        {
            let mut f = self.requests.lock().unwrap_or_else(|p| p.into_inner());
            f.write_all(format!("{line}\n").as_bytes()).and_then(|_| f.sync_data())?;
        }
        if !admitted {
            return Ok(json!({"answer":answer::caller_refused(caller, &self.cfg.service.callers),
                "report":{"plan":[],"results":[],"failure":{"stage":"access","detail":format!("caller {caller} is not admitted")},"followups":[]}}));
        }
        self.answer_as(deputy, &r.id, &r.prompt, r.urgent, caller)
    }
}

impl Drop for Service {
    fn drop(&mut self) {
        self.stop();
    }
}

/// The structured part of a reply: plan, results, failure and review candidates; the native
/// observations stay in the deputy ledger.
fn compact(r: &Report) -> Value {
    let followups: Vec<Value> = r
        .followups
        .iter()
        .map(|f| json!({"candidate_id":f["candidate_id"],"suggested_lane":f["suggested_lane"],"status":f["status"]}))
        .collect();
    json!({"prompt_sha256":r.prompt_sha256,"plan":r.plan,"results":r.results,"failure":r.failure,"followups":followups,
        "training_authorized":r.training_authorized})
}

/// The capability handler for one request (its priority is known only per request).
struct Hands<'a> {
    svc: &'a Service,
    urgent: bool,
    caller: &'a str,
}

fn first<'v>(a: &'v Action, slot: &str) -> anyhow::Result<&'v str> {
    a.arguments.get(slot).and_then(|v| v.first()).map(String::as_str).ok_or_else(|| anyhow::anyhow!("slot {slot} is empty"))
}

impl CapabilityHandler for Hands<'_> {
    fn execute(&self, p: &Value, a: &Action, invocation_id: &str) -> anyhow::Result<Value> {
        let svc = self.svc;
        let e = anyhow::Error::msg;
        let mut out = match p["operation"].as_str() {
            Some("fetch") => {
                let urls: Vec<String> = a
                    .arguments
                    .get("urls")
                    .map(|v| v.iter().map(|u| crawl::normalize_url(u).map(|u| u.to_string())).collect::<Result<_, _>>())
                    .transpose()
                    .map_err(e)?
                    .unwrap_or_default();
                anyhow::ensure!(!urls.is_empty(), "no URL to fetch");
                let (ahead, busy) = svc.enqueue(invocation_id, JobKind::Fetch { urls: urls.clone() }, self.urgent, self.caller).map_err(e)?;
                json!({"operation":"fetch","job":invocation_id,"urls":urls,"ahead":ahead,"busy":busy,"urgent":self.urgent,
                    "answer":answer::fetch_queued(invocation_id, &urls, ahead, busy)})
            }
            Some("robots") => {
                let target = first(a, "target")?;
                let v = svc.robots(target, invocation_id, self.urgent, self.caller).map_err(e)?;
                let text = match v["state"].as_str() {
                    Some("answered") => answer::robots(&v["explanation"]),
                    _ => answer::robots_queued(target, invocation_id),
                };
                json!({"operation":"robots","result":v,"answer":text})
            }
            Some("stats") => {
                let host = a.arguments.get("host").and_then(|v| v.first()).map(String::as_str);
                let v = svc.stats(host).map_err(e)?;
                json!({"operation":"stats","answer":answer::stats(&v),"result":v})
            }
            Some("page") => {
                let v = svc.page(first(a, "url")?).map_err(e)?;
                json!({"operation":"page","answer":answer::page(&v),"result":v})
            }
            Some("status") => {
                let v = svc.status();
                json!({"operation":"status","answer":answer::status(&v),"result":v})
            }
            other => anyhow::bail!("unknown seaspider operation {other:?}"),
        };
        out["completion"] = json!("answered from the journals; queued work runs in the single fetch stream");
        Ok(out)
    }
}

/// `seaspider serve`: take the host-wide singleton lease (a second start fails with
/// `singleton::Refused::Running`), load both configurations, start the worker, serve the private
/// socket under the angel (restarts after failures, `STOP` in the socket directory ends it), then
/// stop the worker after its current URL.
pub fn serve(config: &Path, deputy_config: &Path, socket: &Path) -> anyhow::Result<()> {
    let (cfg, store) = Config::load(config).map_err(anyhow::Error::msg)?;
    let _lease = crate::singleton::acquire(&cfg.lock_path(), &format!("serve {}", socket.display()))?;
    let dbytes = std::fs::read(deputy_config).map_err(|e| anyhow::anyhow!("{}: {e}", deputy_config.display()))?;
    let dcfg: mova_octopus::Config = serde_json::from_slice(&dbytes).map_err(|e| anyhow::anyhow!("{}: {e}", deputy_config.display()))?;
    check_capabilities(&dcfg).map_err(anyhow::Error::msg)?;
    let deputy = Deputy::load(dcfg)?;
    let dir = socket.parent().ok_or_else(|| anyhow::anyhow!("socket needs a directory"))?;
    mova_octopus::socket::private_dir(dir)?;
    let mut svc = Service::start(cfg, store, dir).map_err(anyhow::Error::msg)?;
    let r = mova_octopus::socket::angel_requests(socket, &|| Ok(|r: &Request| svc.answer_request(&deputy, r)), Duration::from_secs(60));
    svc.stop();
    r
}

/// `seaspider ask`: send arbitrary text; the reply is the English answer, then the JSON.
pub fn ask(socket: &Path, id: &str, prompt: &str, urgent: bool) -> anyhow::Result<Value> {
    mova_octopus::socket::ask_with(socket, id, true, prompt, urgent)
}

/// As `ask`, declaring the caller (`seaspider ask --from NAME`).
pub fn ask_from(socket: &Path, from: &str, id: &str, prompt: &str, urgent: bool) -> anyhow::Result<Value> {
    mova_octopus::socket::ask_from(socket, id, true, prompt, urgent, Some(from), None)
}
