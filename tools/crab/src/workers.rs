//! Crab worker supervision: trusted, configured long-running workers that Crab starts after
//! boot, adopts across its own restarts, restarts after crashes with doubling backoff and
//! interrupts only on an explicit stop or restart. GPU workers start only after a bounded check.
//!
//! The command lines come only from trusted Crab configuration, never from requests or prompts.
//! Every step is non-blocking: the serve loop calls `tick` with the current time, and slow
//! effects (the GPU check, a graceful stop) advance across ticks instead of waiting.
use crate::Result;
use serde::Deserialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    collections::{BTreeMap, VecDeque},
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    os::unix::{
        fs::OpenOptionsExt,
        process::{CommandExt, ExitStatusExt},
    },
    path::{Path, PathBuf},
    process::{Child, Command, ExitStatus, Stdio},
};

pub const BACKOFF_MIN_SECONDS: u64 = 5;
pub const BACKOFF_MAX_SECONDS: u64 = 300;
/// SIGKILL follows SIGTERM after this many seconds.
pub const KILL_AFTER_SECONDS: u64 = 10;
/// A run at least this long resets the crash backoff.
const STABLE_RUN_SECONDS: u64 = 300;
const EVENTS_MAX: usize = 32;
const WORKERS_MAX: usize = 32;
const STATE_MAX: usize = 64 * 1024;
const OUTPUT_MAX: usize = 4096;
/// A conflict report shows at most this many command-line arguments, each at most this long.
const REPORT_ARGS_MAX: usize = 64;
const REPORT_ARG_BYTES: usize = 512;
pub const OPERATIONS: [&str; 4] = ["worker_status", "worker_start", "worker_stop", "worker_restart"];

fn error(s: impl Into<String>) -> Box<dyn std::error::Error> {
    s.into().into()
}
fn default_grace() -> u64 {
    300
}
fn default_true() -> bool {
    true
}
fn default_timeout() -> u64 {
    15
}
/// One worker from trusted Crab configuration.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct WorkerSpec {
    pub name: String,
    /// Absolute executable followed by its arguments.
    pub argv: Vec<String>,
    pub cwd: PathBuf,
    /// Absolute file; stdout and stderr are appended.
    pub log: PathBuf,
    #[serde(default)]
    pub gpu: bool,
    #[serde(default)]
    pub autostart: bool,
    /// File the worker honours as a graceful STOP request.
    #[serde(default)]
    pub stop_marker: Option<PathBuf>,
    #[serde(default = "default_grace")]
    pub stop_grace_seconds: u64,
    #[serde(default = "default_true")]
    pub restart: bool,
    /// Extra environment for the worker (the command line stays exactly `argv`, so a restarted Crab can adopt it).
    #[serde(default)]
    pub env: std::collections::BTreeMap<String, String>,
}
/// GPU admission before any GPU worker starts.
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct GpuCheck {
    pub nvidia_smi: PathBuf,
    #[serde(default = "default_timeout")]
    pub timeout_seconds: u64,
    /// Optional extra command that must exit 0 within the same timeout.
    #[serde(default)]
    pub probe_argv: Option<Vec<String>>,
}
/// Worker names are ASCII identifiers; they double as state-file names.
pub fn valid_name(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 64
        && s.as_bytes()[0].is_ascii_alphanumeric()
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
}
fn absolute_path(p: &Path) -> bool {
    p.is_absolute() && p.to_str().is_some_and(|s| s.len() <= 1024 && !s.contains('\0'))
}
fn valid_argv(a: &[String]) -> bool {
    !a.is_empty()
        && a.len() <= 64
        && absolute_path(Path::new(&a[0]))
        && a.iter().all(|x| x.len() <= 4096 && !x.contains('\0'))
}
fn validate(specs: &[WorkerSpec], gpu: Option<&GpuCheck>) -> Result<()> {
    if specs.len() > WORKERS_MAX {
        return Err(error("at most 32 configured workers"));
    }
    let mut names = std::collections::BTreeSet::new();
    for w in specs {
        if !valid_name(&w.name) || !names.insert(w.name.clone()) {
            return Err(error("worker names must be unique ASCII identifiers"));
        }
        if !valid_argv(&w.argv)
            || !absolute_path(&w.cwd)
            || !absolute_path(&w.log)
            || w.stop_marker.as_ref().is_some_and(|p| !absolute_path(p))
        {
            return Err(error(
                "worker argv[0], cwd, log and stop_marker must be absolute trusted paths",
            ));
        }
        if !(1..=86400).contains(&w.stop_grace_seconds) {
            return Err(error("stop_grace_seconds must be 1..86400"));
        }
        if w.gpu && gpu.is_none() {
            return Err(error("a gpu worker requires a configured gpu_check"));
        }
    }
    if let Some(g) = gpu {
        if !absolute_path(&g.nvidia_smi)
            || !(1..=600).contains(&g.timeout_seconds)
            || g.probe_argv.as_ref().is_some_and(|a| !valid_argv(a))
        {
            return Err(error(
                "gpu_check needs an absolute nvidia_smi, timeout 1..600 and an absolute probe",
            ));
        }
    }
    Ok(())
}
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn argv_hash(argv: &[String]) -> String {
    hash(&serde_json::to_vec(argv).unwrap_or_default())
}
/// `(state, pgrp, starttime)` from /proc/<pid>/stat; starttime is in clock ticks since boot.
fn proc_stat(pid: u32) -> Option<(char, i32, String)> {
    let s = fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
    let f: Vec<&str> = s.rsplit_once(") ")?.1.split_whitespace().collect();
    Some((
        f.first()?.chars().next()?,
        f.get(2)?.parse().ok()?,
        f.get(19)?.to_string(),
    ))
}
fn proc_cmdline(pid: u32) -> Option<Vec<String>> {
    let b = fs::read(format!("/proc/{pid}/cmdline")).ok()?;
    let mut v: Vec<String> = b
        .split(|c| *c == 0)
        .map(|x| String::from_utf8_lossy(x).into_owned())
        .collect();
    if v.last().is_some_and(String::is_empty) {
        v.pop();
    }
    Some(v)
}
/// The configured argv must be a suffix of the kernel command line (a `#!` interpreter may
/// precede it).
fn cmdline_matches(cmdline: &[String], argv: &[String]) -> bool {
    cmdline.len() >= argv.len() && cmdline[cmdline.len() - argv.len()..] == *argv
}
/// Identity of a recorded process: `Ok(pgrp)` while /proc/<pid>/stat has the recorded start
/// time and is not a zombie; otherwise why the recorded instance is gone.
fn recorded_identity(pid: u32, starttime: &str) -> std::result::Result<i32, String> {
    let Some((state, pgrp, now_start)) = proc_stat(pid) else {
        return Err("no such process".into());
    };
    if now_start != starttime {
        return Err("pid reused by another process (start time differs)".into());
    }
    if state == 'Z' || state == 'X' {
        return Err("process has exited".into());
    }
    Ok(pgrp)
}
/// Bounded copy of a command line for a report.
fn bounded_args(a: &[String]) -> Vec<String> {
    a.iter()
        .take(REPORT_ARGS_MAX)
        .map(|x| {
            let mut end = x.len().min(REPORT_ARG_BYTES);
            while !x.is_char_boundary(end) {
                end -= 1;
            }
            x[..end].to_string()
        })
        .collect()
}
fn exit_value(s: ExitStatus) -> Value {
    json!({"code":s.code(),"signal":s.signal(),"observed":"waitpid"})
}
struct Live {
    pid: u32,
    pgid: i32,
    starttime: String,
    child: Option<Child>,
    started_unix: u64,
    adopted: bool,
    /// Hash of the argv this process was started with (the configured one unless it was taken
    /// over from an unadopted conflict by an explicit restart).
    argv_sha256: String,
}
/// A live recorded instance that was not adopted because its command line differs from the
/// configured argv. Crab neither starts a second copy nor signals it; only an explicit
/// `worker_restart` takes it over (stop, then start the configured argv).
#[derive(Clone)]
struct Recorded {
    pid: u32,
    pgid: i32,
    starttime: String,
    started_unix: u64,
    argv_sha256: String,
    detail: String,
}
impl Live {
    /// Identity still holds: same pid and kernel start time, not a zombie.
    fn same_process(&self) -> bool {
        proc_stat(self.pid).is_some_and(|(s, _, t)| t == self.starttime && s != 'Z' && s != 'X')
    }
    /// Non-blocking exit observation: waitpid for own children, /proc for adopted ones.
    fn poll_exit(&mut self) -> Option<Value> {
        if let Some(c) = &mut self.child {
            return match c.try_wait() {
                Ok(Some(s)) => Some(exit_value(s)),
                Ok(None) => None,
                Err(e) => Some(json!({"observed":"waitpid_error","detail":e.to_string()})),
            };
        }
        if self.same_process() {
            return None;
        }
        // Reap if the adopted process happens to be our child; harmless otherwise.
        let mut status = 0;
        let reaped = unsafe { libc::waitpid(self.pid as i32, &mut status, libc::WNOHANG) };
        if reaped == self.pid as i32 {
            return Some(exit_value(ExitStatus::from_raw(status)));
        }
        Some(json!({"code":null,"signal":null,"observed":"proc_absent_or_identity_changed"}))
    }
    /// Signal the process group only while the recorded identity still holds.
    fn signal(&mut self, sig: i32) -> bool {
        if let Some(c) = &mut self.child {
            if !matches!(c.try_wait(), Ok(None)) {
                return false;
            }
        }
        if !self.same_process() {
            return false;
        }
        let target = if self.pgid == self.pid as i32 {
            -self.pgid
        } else {
            self.pid as i32
        };
        unsafe { libc::kill(target, sig) == 0 }
    }
}
struct Worker {
    spec: WorkerSpec,
    argv_sha256: String,
    live: Option<Live>,
    conflict: Option<Recorded>,
    state: String,
    stop_requested: bool,
    start_after_stop: bool,
    phase_since: u64,
    restarts: u64,
    failures: u32,
    next_start: u64,
    last_exit: Value,
    last_error: Option<String>,
    events: VecDeque<Value>,
}
impl Worker {
    fn marker_present(&self) -> bool {
        self.spec
            .stop_marker
            .as_ref()
            .is_some_and(|p| fs::symlink_metadata(p).is_ok())
    }
}
enum Gate {
    Idle,
    Smi { child: Child, deadline: u64 },
    Probe { child: Child, deadline: u64 },
    Failed { next_try: u64 },
}
/// Supervisor of all configured workers. Single-threaded; Crab holds it behind a mutex.
pub struct Workers {
    dir: PathBuf,
    workers: BTreeMap<String, Worker>,
    gpu: Option<GpuCheck>,
    gate: Gate,
    gate_record: Value,
    gate_failures: u32,
    booted: bool,
}
fn backoff(failures: u32) -> u64 {
    let shift = failures.saturating_sub(1).min(16);
    (BACKOFF_MIN_SECONDS << shift).min(BACKOFF_MAX_SECONDS)
}
/// The state file records the identity of the process Crab answers for: the live one, or the
/// recorded instance of an unadopted conflict, so the next Crab checks it again instead of
/// starting a duplicate. `argv_sha256` is the argv of that process; `configured_argv_sha256`
/// is the configuration's.
fn save(dir: &Path, w: &mut Worker, now: u64) {
    let (pid, pgid, starttime, started, argv_sha256) = match (&w.live, &w.conflict) {
        (Some(l), _) => (Some(l.pid), Some(l.pgid), Some(l.starttime.clone()), Some(l.started_unix), l.argv_sha256.clone()),
        (None, Some(c)) => (Some(c.pid), Some(c.pgid), Some(c.starttime.clone()), Some(c.started_unix), c.argv_sha256.clone()),
        (None, None) => (None, None, None, None, w.argv_sha256.clone()),
    };
    let v = json!({"schema":1,"name":w.spec.name,"argv_sha256":argv_sha256,
        "configured_argv_sha256":w.argv_sha256,"state":w.state,
        "pid":pid,"pgid":pgid,"starttime":starttime,"started_unix":started,
        "adopted":w.live.as_ref().map(|l|l.adopted),
        "conflict":w.conflict.as_ref().map(|c|c.detail.clone()),
        "stop_requested":w.stop_requested,"start_after_stop":w.start_after_stop,
        "restarts":w.restarts,"consecutive_failures":w.failures,"next_start_unix":w.next_start,
        "last_exit":w.last_exit,"last_error":w.last_error,"updated_unix":now,"events":w.events});
    if let Err(e) = write_atomic(&dir.join(format!("{}.json", w.spec.name)), &v) {
        w.last_error = Some(format!("state write failed: {e}"));
    }
}
fn write_atomic(p: &Path, v: &Value) -> Result<()> {
    let tmp = p.with_extension("tmp");
    let mut f = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .mode(0o600)
        .open(&tmp)?;
    f.write_all(&serde_json::to_vec(v)?)?;
    f.sync_all()?;
    fs::rename(tmp, p)?;
    Ok(())
}
/// Record one transition durably: new state, a bounded event and the state file.
fn transition(dir: &Path, w: &mut Worker, now: u64, state: &str, event: Value) {
    w.state = state.into();
    w.phase_since = now;
    let mut e = json!({"at_unix":now,"state":state});
    if let (Some(m), Value::Object(x)) = (e.as_object_mut(), event) {
        m.extend(x);
    }
    w.events.push_back(e);
    while w.events.len() > EVENTS_MAX {
        w.events.pop_front();
    }
    save(dir, w, now);
}
fn read_bounded(p: &Path, max: usize) -> Result<Vec<u8>> {
    let mut b = Vec::new();
    File::open(p)?.take(max as u64 + 1).read_to_end(&mut b)?;
    if b.len() > max {
        return Err(error("file exceeds bounded read"));
    }
    Ok(b)
}
/// Spawn a worker now (any GPU admission has already happened).
fn spawn(dir: &Path, w: &mut Worker, now: u64) {
    let result = (|| -> Result<Live> {
        if let Some(m) = &w.spec.stop_marker {
            if let Ok(meta) = fs::symlink_metadata(m) {
                if !meta.is_file() {
                    return Err(error("stop marker is not a regular file; refusing start"));
                }
                fs::remove_file(m)?;
            }
        }
        let log = OpenOptions::new()
            .create(true)
            .append(true)
            .mode(0o600)
            .open(&w.spec.log)?;
        let child = Command::new(&w.spec.argv[0])
            .args(&w.spec.argv[1..])
            .envs(&w.spec.env)
            .current_dir(&w.spec.cwd)
            .stdin(Stdio::null())
            .stdout(Stdio::from(log.try_clone()?))
            .stderr(Stdio::from(log))
            .process_group(0)
            .spawn()?;
        let pid = child.id();
        let starttime = proc_stat(pid).map(|x| x.2).unwrap_or_default();
        Ok(Live {
            pid,
            pgid: pid as i32,
            starttime,
            child: Some(child),
            started_unix: now,
            adopted: false,
            argv_sha256: w.argv_sha256.clone(),
        })
    })();
    match result {
        Ok(live) => {
            let pid = live.pid;
            w.live = Some(live);
            w.last_error = None;
            transition(dir, w, now, "running", json!({"event":"started","pid":pid}));
        }
        Err(e) => {
            w.last_error = Some(e.to_string());
            if w.spec.restart {
                w.failures += 1;
                let delay = backoff(w.failures);
                w.next_start = now + delay;
                transition(dir, w, now, "backoff", json!({"event":"spawn_failed","detail":e.to_string(),"retry_in_seconds":delay}));
            } else {
                transition(dir, w, now, "failed", json!({"event":"spawn_failed","detail":e.to_string()}));
            }
        }
    }
}
/// Start or queue for the GPU gate.
fn begin(dir: &Path, w: &mut Worker, now: u64, event: &str) {
    if w.spec.gpu {
        transition(dir, w, now, "waiting_gpu", json!({"event":event,"gpu_check":"required"}));
    } else {
        spawn(dir, w, now);
    }
}
fn handle_exit(dir: &Path, w: &mut Worker, exit: Value, now: u64) {
    let live = w.live.take();
    let ran = live.as_ref().map(|l| now.saturating_sub(l.started_unix)).unwrap_or(0);
    w.last_exit = json!({"exit":exit,"ran_seconds":ran,"at_unix":now,"pid":live.as_ref().map(|l|l.pid)});
    if w.state.starts_with("stopping") {
        transition(dir, w, now, "stopped", json!({"event":"stopped","exit":exit}));
        if w.start_after_stop {
            w.start_after_stop = false;
            w.stop_requested = false;
            w.failures = 0;
            w.restarts += 1;
            begin(dir, w, now, "restart_after_stop");
        }
    } else if w.marker_present() {
        // Someone else asked the worker to stop through its marker; honour it, never restart.
        w.stop_requested = true;
        transition(dir, w, now, "stopped", json!({"event":"exited_with_stop_marker","exit":exit}));
    } else if w.spec.restart {
        if ran >= STABLE_RUN_SECONDS {
            w.failures = 0;
        }
        w.failures += 1;
        let delay = backoff(w.failures);
        w.next_start = now + delay;
        transition(dir, w, now, "backoff", json!({"event":"crashed","exit":exit,"retry_in_seconds":delay}));
    } else {
        transition(dir, w, now, "exited", json!({"event":"exited","exit":exit}));
    }
}
fn gate_child(argv: &[String], out: &Path) -> Result<Child> {
    let f = OpenOptions::new()
        .create(true)
        .truncate(true)
        .write(true)
        .mode(0o600)
        .open(out)?;
    Ok(Command::new(&argv[0])
        .args(&argv[1..])
        .stdin(Stdio::null())
        .stdout(Stdio::from(f.try_clone()?))
        .stderr(Stdio::from(f))
        .process_group(0)
        .spawn()?)
}
fn kill_group(c: &mut Child) {
    if matches!(c.try_wait(), Ok(None)) {
        unsafe {
            libc::kill(-(c.id() as i32), libc::SIGKILL);
        }
    }
    let _ = c.wait();
}
enum Poll {
    Running,
    Passed(String),
    Failed(String),
}
fn poll_gate_child(c: &mut Child, deadline: u64, now: u64, out: &Path, what: &str) -> Poll {
    match c.try_wait() {
        Ok(Some(s)) => {
            let text = read_bounded(out, 1 << 20)
                .map(|b| String::from_utf8_lossy(&b[..b.len().min(OUTPUT_MAX)]).into_owned())
                .unwrap_or_default();
            if s.success() {
                Poll::Passed(text)
            } else {
                Poll::Failed(format!("{what} exited {:?}/{:?}: {}", s.code(), s.signal(), text.trim()))
            }
        }
        Ok(None) if now >= deadline => {
            kill_group(c);
            Poll::Failed(format!("{what} did not answer within the timeout"))
        }
        Ok(None) => Poll::Running,
        Err(e) => Poll::Failed(format!("{what} wait failed: {e}")),
    }
}
/// Re-check an unadopted conflict's recorded process. `true`: it still runs (nothing is done
/// to it). `false`: there was no conflict, or the recorded process is gone, in which case the
/// record is cleared, journaled and the worker is `stopped`, ready for a normal start.
fn conflict_still_alive(dir: &Path, w: &mut Worker, now: u64) -> bool {
    let Some(rec) = w.conflict.clone() else {
        return false;
    };
    match recorded_identity(rec.pid, &rec.starttime) {
        Ok(pgid) => {
            if let Some(c) = w.conflict.as_mut() {
                c.pgid = pgid;
            }
            true
        }
        Err(why) => {
            w.conflict = None;
            transition(dir, w, now, "stopped", json!({"event":"recorded_instance_gone","pid":rec.pid,"detail":why,"cleared":"unadopted_conflict"}));
            false
        }
    }
}
/// What an operator needs to resolve an unadopted conflict, in plain fields.
fn conflict_report(w: &Worker) -> Value {
    let Some(c) = &w.conflict else {
        return Value::Null;
    };
    let name = &w.spec.name;
    let stop = match &w.spec.stop_marker {
        Some(_) => format!("stop marker, up to {} s grace, then SIGTERM and SIGKILL after {KILL_AFTER_SECONDS} s", w.spec.stop_grace_seconds),
        None => format!("SIGTERM, then SIGKILL after {KILL_AFTER_SECONDS} s"),
    };
    json!({"recorded_pid":c.pid,"recorded_pgid":c.pgid,"recorded_starttime":c.starttime,
        "recorded_started_unix":c.started_unix,"recorded_argv_sha256":c.argv_sha256,
        "recorded_cmdline":proc_cmdline(c.pid).map(|a| bounded_args(&a)),
        "configured_argv":bounded_args(&w.spec.argv),"configured_argv_sha256":w.argv_sha256,
        "reason":c.detail,
        "summary":format!("Worker {name}: the recorded process {} still runs, but its command line differs from the configured argv ({}). Crab did not start a second copy and sent no signal.", c.pid, c.detail),
        "resolution":format!("Send worker_restart {name} to stop process {} ({stop}) and then start the configured argv. Or stop process {} by hand; Crab then clears the record, and worker_start {name} (or autostart) starts the configured argv.", c.pid, c.pid)})
}
impl Workers {
    /// Validate trusted configuration; no filesystem or process effects.
    pub fn new(dir: PathBuf, specs: Vec<WorkerSpec>, gpu: Option<GpuCheck>) -> Result<Self> {
        validate(&specs, gpu.as_ref())?;
        let workers = specs
            .into_iter()
            .map(|spec| {
                let w = Worker {
                    argv_sha256: argv_hash(&spec.argv),
                    spec,
                    live: None,
                    conflict: None,
                    state: "not_booted".into(),
                    stop_requested: false,
                    start_after_stop: false,
                    phase_since: 0,
                    restarts: 0,
                    failures: 0,
                    next_start: 0,
                    last_exit: Value::Null,
                    last_error: None,
                    events: VecDeque::new(),
                };
                (w.spec.name.clone(), w)
            })
            .collect();
        Ok(Self {
            dir,
            workers,
            gpu,
            gate: Gate::Idle,
            gate_record: json!({"state":"not_run"}),
            gate_failures: 0,
            booted: false,
        })
    }
    pub fn is_empty(&self) -> bool {
        self.workers.is_empty()
    }
    pub fn dir(&self) -> &Path {
        &self.dir
    }
    /// After Crab holds its lease: adopt verified running instances, then start autostart
    /// workers that were not explicitly stopped (persisted stop or a present stop marker).
    pub fn boot(&mut self, now: u64) {
        if self.booted {
            return;
        }
        self.booted = true;
        let dir = self.dir.clone();
        for w in self.workers.values_mut() {
            let path = dir.join(format!("{}.json", w.spec.name));
            let saved = if path.exists() {
                match read_bounded(&path, STATE_MAX).and_then(|b| Ok(serde_json::from_slice::<Value>(&b)?)) {
                    Ok(v) => Some(v),
                    Err(e) => {
                        w.last_error = Some(format!("unreadable state file ignored: {e}"));
                        None
                    }
                }
            } else {
                None
            };
            if let Some(s) = &saved {
                w.stop_requested = s["stop_requested"] == true;
                w.start_after_stop = s["start_after_stop"] == true;
                w.restarts = s["restarts"].as_u64().unwrap_or(0);
                w.last_exit = s["last_exit"].clone();
                let previous = s["state"].as_str().unwrap_or("").to_string();
                if let Some(ev) = s["events"].as_array() {
                    w.events = ev.iter().rev().take(EVENTS_MAX).rev().cloned().collect();
                }
                if let (Some(pid), Some(start)) = (s["pid"].as_u64(), s["starttime"].as_str()) {
                    // A stop Crab already began continues on the very same process even when
                    // its argv is not the configured one (an explicit restart of a conflict).
                    match adopt(pid as u32, start, s, w, previous.starts_with("stopping")) {
                        Adoption::Adopted(live) => {
                            w.live = Some(live);
                            let state = if previous.starts_with("stopping") {
                                previous.as_str()
                            } else {
                                "running"
                            };
                            transition(&dir, w, now, state, json!({"event":"adopted","pid":pid}));
                            continue;
                        }
                        Adoption::Gone(why) => {
                            w.events.push_back(json!({"at_unix":now,"event":"recorded_instance_gone","pid":pid,"detail":why}));
                        }
                        Adoption::Conflict(rec) => {
                            let detail = rec.detail.clone();
                            w.conflict = Some(rec);
                            transition(&dir, w, now, "unadopted_conflict", json!({"event":"unadopted_conflict","pid":pid,"detail":detail}));
                            continue;
                        }
                    }
                }
            }
            if w.start_after_stop {
                w.start_after_stop = false;
                w.stop_requested = false;
                begin(&dir, w, now, "restart_after_crab_restart");
            } else if w.spec.autostart && !w.stop_requested && !w.marker_present() {
                begin(&dir, w, now, "autostart");
            } else {
                let why = if !w.spec.autostart {
                    "not_autostart"
                } else if w.stop_requested {
                    "explicitly_stopped"
                } else {
                    "stop_marker_present"
                };
                transition(&dir, w, now, "stopped", json!({"event":"boot_not_started","reason":why}));
            }
        }
    }
    /// One non-blocking supervision step.
    pub fn tick(&mut self, now: u64) {
        if !self.booted {
            return;
        }
        let dir = self.dir.clone();
        for w in self.workers.values_mut() {
            if let Some(exit) = w.live.as_mut().and_then(Live::poll_exit) {
                handle_exit(&dir, w, exit, now);
                continue;
            }
            if w.conflict.is_some() {
                // The recorded process ended (for example stopped by hand): clear the record
                // and autostart as boot would have; a live one is left alone.
                if !conflict_still_alive(&dir, w, now) && w.spec.autostart && !w.stop_requested && !w.marker_present() {
                    begin(&dir, w, now, "autostart_after_recorded_instance_gone");
                }
                continue;
            }
            match w.state.as_str() {
                "stopping_marker" if now >= w.phase_since + w.spec.stop_grace_seconds => {
                    let sent = w.live.as_mut().is_some_and(|l| l.signal(libc::SIGTERM));
                    transition(&dir, w, now, "stopping_term", json!({"event":"grace_expired_sigterm","sent":sent}));
                }
                "stopping_term" if now >= w.phase_since + KILL_AFTER_SECONDS => {
                    let sent = w.live.as_mut().is_some_and(|l| l.signal(libc::SIGKILL));
                    transition(&dir, w, now, "stopping_kill", json!({"event":"sigkill","sent":sent}));
                }
                "backoff" if now >= w.next_start => {
                    if w.marker_present() {
                        w.stop_requested = true;
                        transition(&dir, w, now, "stopped", json!({"event":"restart_cancelled_by_stop_marker"}));
                    } else {
                        w.restarts += 1;
                        begin(&dir, w, now, "restart_after_backoff");
                    }
                }
                _ => {}
            }
        }
        self.tick_gate(now);
    }
    fn tick_gate(&mut self, now: u64) {
        let Some(g) = self.gpu.clone() else {
            return;
        };
        let waiting = self.workers.values().any(|w| w.state == "waiting_gpu");
        let smi_out = self.dir.join("gpu-check.out");
        let probe_out = self.dir.join("gpu-probe.out");
        let outcome = match std::mem::replace(&mut self.gate, Gate::Idle) {
            Gate::Smi { mut child, deadline } => {
                match poll_gate_child(&mut child, deadline, now, &smi_out, "nvidia-smi") {
                    Poll::Running => {
                        self.gate = Gate::Smi { child, deadline };
                        None
                    }
                    Poll::Passed(text) if text.trim().is_empty() => {
                        Some(Err("nvidia-smi gave no GPU rows".to_string()))
                    }
                    Poll::Passed(text) => match &g.probe_argv {
                        Some(p) => match gate_child(p, &probe_out) {
                            Ok(child) => {
                                self.gate_record = json!({"state":"probe_running","at_unix":now,"failures":self.gate_failures,"nvidia_smi":text.trim()});
                                self.gate = Gate::Probe { child, deadline: now + g.timeout_seconds };
                                None
                            }
                            Err(e) => Some(Err(format!("probe spawn failed: {e}"))),
                        },
                        None => Some(Ok(text)),
                    },
                    Poll::Failed(why) => Some(Err(why)),
                }
            }
            Gate::Probe { mut child, deadline } => {
                match poll_gate_child(&mut child, deadline, now, &probe_out, "probe") {
                    Poll::Running => {
                        self.gate = Gate::Probe { child, deadline };
                        None
                    }
                    Poll::Passed(_) => Some(Ok(self.gate_record["nvidia_smi"].as_str().unwrap_or("").to_string())),
                    Poll::Failed(why) => Some(Err(why)),
                }
            }
            Gate::Idle => {
                if waiting {
                    self.start_gate(&g, now);
                }
                None
            }
            Gate::Failed { next_try } => {
                if waiting && now >= next_try {
                    self.start_gate(&g, now);
                } else {
                    self.gate = Gate::Failed { next_try };
                }
                None
            }
        };
        match outcome {
            None => return,
            Some(Ok(text)) => {
                self.gate_failures = 0;
                self.gate_record = json!({"state":"passed","at_unix":now,"output":text.trim()});
                let dir = self.dir.clone();
                for w in self.workers.values_mut().filter(|w| w.state == "waiting_gpu") {
                    spawn(&dir, w, now);
                }
            }
            Some(Err(why)) => self.gate_failed(now, why),
        }
        let _ = write_atomic(&self.dir.join("gpu-check.json"), &self.gate_record);
    }
    fn gate_failed(&mut self, now: u64, why: String) {
        self.gate_failures += 1;
        let delay = backoff(self.gate_failures);
        self.gate = Gate::Failed { next_try: now + delay };
        self.gate_record = json!({"state":"failed","at_unix":now,"reason":why,"failures":self.gate_failures,"retry_in_seconds":delay,"workers_started":false});
    }
    fn start_gate(&mut self, g: &GpuCheck, now: u64) {
        let argv = vec![
            g.nvidia_smi.to_string_lossy().into_owned(),
            "--query-gpu=index,name,memory.used,memory.total".into(),
            "--format=csv,noheader".into(),
        ];
        match gate_child(&argv, &self.dir.join("gpu-check.out")) {
            Ok(child) => {
                self.gate = Gate::Smi { child, deadline: now + g.timeout_seconds };
                self.gate_record = json!({"state":"running","at_unix":now,"failures":self.gate_failures});
            }
            Err(e) => self.gate_failed(now, format!("nvidia-smi spawn failed: {e}")),
        }
        let _ = write_atomic(&self.dir.join("gpu-check.json"), &self.gate_record);
    }
    fn get(&mut self, name: &str) -> Result<&mut Worker> {
        self.workers
            .get_mut(name)
            .ok_or_else(|| error(format!("unknown worker {name:?}; only configured workers are controlled")))
    }
    /// Start a worker. A plain start never signals: over a live unadopted conflict it starts
    /// nothing and answers with the conflict report; a conflict whose process is gone is
    /// cleared and the worker starts normally.
    pub fn start(&mut self, name: &str, now: u64) -> Result<Value> {
        self.require_booted()?;
        let dir = self.dir.clone();
        let w = self.get(name)?;
        if conflict_still_alive(&dir, w, now) {
            return Ok(self.conflict_view(name, now, "start"));
        }
        match w.state.as_str() {
            s if s.starts_with("stopping") => {
                w.start_after_stop = true;
                save(&dir, w, now);
            }
            "running" | "waiting_gpu" => {}
            _ => {
                w.stop_requested = false;
                w.failures = 0;
                begin(&dir, w, now, "start_requested");
            }
        }
        Ok(self.view(name, now, "start"))
    }
    pub fn stop(&mut self, name: &str, now: u64) -> Result<Value> {
        self.stop_with(name, now, false)
    }
    /// Stop a worker. `immediate` is the emergency interruption: it skips the graceful stop
    /// marker (which waits for the worker's next safe point) and sends SIGTERM to the worker's
    /// process group at once; the usual SIGKILL escalation follows. A stopped worker is not
    /// restarted by ticks either way.
    pub fn stop_with(&mut self, name: &str, now: u64, immediate: bool) -> Result<Value> {
        self.require_booted()?;
        let dir = self.dir.clone();
        let w = self.get(name)?;
        if conflict_still_alive(&dir, w, now) {
            let pid = w.conflict.as_ref().map(|c| c.pid).unwrap_or(0);
            return Err(error(format!(
                "worker {name}: the recorded process {pid} is not adopted (its command line differs from the configured argv); worker_stop does not signal it. Use worker_restart {name} to stop it and start the configured argv, or stop it by hand"
            )));
        }
        w.stop_requested = true;
        w.start_after_stop = false;
        match w.state.as_str() {
            s if s.starts_with("stopping") => save(&dir, w, now),
            _ if w.live.is_some() => {
                if let (false, Some(m)) = (immediate, w.spec.stop_marker.clone()) {
                    let made = OpenOptions::new()
                        .create_new(true)
                        .write(true)
                        .mode(0o600)
                        .open(&m)
                        .and_then(|mut f| f.write_all(format!("crab worker stop: {}\n", w.spec.name).as_bytes()));
                    match made {
                        Ok(()) => {}
                        Err(e) if e.kind() == std::io::ErrorKind::AlreadyExists => {}
                        Err(e) => return Err(error(format!("cannot create stop marker: {e}"))),
                    }
                    transition(&dir, w, now, "stopping_marker", json!({"event":"stop_requested","marker":m}));
                } else {
                    let sent = w.live.as_mut().is_some_and(|l| l.signal(libc::SIGTERM));
                    let event = if immediate { "stop_requested_immediate" } else { "stop_requested_sigterm" };
                    transition(&dir, w, now, "stopping_term", json!({"event":event,"sent":sent}));
                }
            }
            _ => transition(&dir, w, now, "stopped", json!({"event":"stop_requested_while_not_running"})),
        }
        Ok(self.view(name, now, "stop"))
    }
    /// Explicit restart: stop, then start the configured argv. Over a live unadopted conflict it
    /// takes the recorded process over (identity re-checked: pid, start time, not a zombie) and
    /// stops it exactly as a normal stop would (marker and grace, SIGTERM, SIGKILL; signals only
    /// while the identity holds), then starts the configured argv.
    pub fn restart(&mut self, name: &str, now: u64) -> Result<Value> {
        self.require_booted()?;
        let dir = self.dir.clone();
        let w = self.get(name)?;
        if conflict_still_alive(&dir, w, now) {
            let Some(rec) = w.conflict.take() else {
                return Err(error("conflict record vanished"));
            };
            let previous = std::mem::replace(&mut w.state, "running".into());
            w.live = Some(Live {
                pid: rec.pid,
                pgid: rec.pgid,
                starttime: rec.starttime.clone(),
                child: None,
                started_unix: rec.started_unix,
                adopted: true,
                argv_sha256: rec.argv_sha256.clone(),
            });
            w.events.push_back(json!({"at_unix":now,"event":"conflict_takeover","pid":rec.pid,"state":"running",
                "detail":"explicit worker_restart: stop the recorded process, then start the configured argv"}));
            if let Err(e) = self.stop(name, now) {
                let w = self.get(name)?;
                w.live = None;
                w.conflict = Some(rec);
                w.state = previous;
                w.events.pop_back();
                return Err(e);
            }
            let w = self.get(name)?;
            w.start_after_stop = true;
            save(&dir, w, now);
            let mut v = self.view(name, now, "restart");
            v["took_over_conflict_pid"] = json!(rec.pid);
            return Ok(v);
        }
        let running = w.live.is_some();
        if !running {
            return self.start(name, now).map(|mut v| {
                v["requested"] = json!("restart");
                v
            });
        }
        self.stop(name, now)?;
        let w = self.get(name)?;
        w.start_after_stop = true;
        save(&dir, w, now);
        Ok(self.view(name, now, "restart"))
    }
    fn require_booted(&self) -> Result<()> {
        if !self.booted {
            return Err(error("worker supervision has not booted"));
        }
        Ok(())
    }
    fn view(&self, name: &str, now: u64, requested: &str) -> Value {
        let mut v = self.one(name, now);
        v["requested"] = json!(requested);
        v["completion"] = json!("request recorded; Crab advances it without blocking; observe worker_status");
        v
    }
    fn conflict_view(&self, name: &str, now: u64, requested: &str) -> Value {
        let mut v = self.one(name, now);
        v["requested"] = json!(requested);
        v["started"] = json!(false);
        v["signal_sent"] = json!(false);
        v["completion"] = json!("not started: unadopted conflict; see conflict.resolution");
        v
    }
    fn one(&self, name: &str, now: u64) -> Value {
        let Some(w) = self.workers.get(name) else {
            return Value::Null;
        };
        let events: Vec<_> = w.events.iter().rev().take(8).rev().cloned().collect();
        json!({"worker":name,"state":w.state,"pid":w.live.as_ref().map(|l|l.pid),
            "adopted":w.live.as_ref().map(|l|l.adopted),
            "uptime_seconds":w.live.as_ref().map(|l|now.saturating_sub(l.started_unix)),
            "gpu":w.spec.gpu,"autostart":w.spec.autostart,"restart":w.spec.restart,
            "stop_requested":w.stop_requested,"start_after_stop":w.start_after_stop,
            "restarts":w.restarts,"consecutive_failures":w.failures,
            "next_start_unix":if w.state=="backoff" {json!(w.next_start)} else {Value::Null},
            "last_exit":w.last_exit,"last_error":w.last_error,"recent_events":events,
            "conflict":conflict_report(w),"health_inferred":false})
    }
    pub fn status(&self, name: Option<&str>, now: u64) -> Result<Value> {
        if let Some(n) = name {
            if !self.workers.contains_key(n) {
                return Err(error(format!("unknown worker {n:?}")));
            }
        }
        let list: Vec<_> = self
            .workers
            .keys()
            .filter(|k| name.is_none_or(|n| n == k.as_str()))
            .map(|k| self.one(k, now))
            .collect();
        Ok(json!({"observed_unix":now,"booted":self.booted,"workers":list,"gpu_check":self.gate_record}))
    }
    /// (state, explicitly stopped) of one configured worker.
    pub fn observe(&self, name: &str) -> Option<(String, bool)> {
        self.workers.get(name).map(|w| (w.state.clone(), w.stop_requested))
    }
    /// Compact name -> state map for Crab's general status.
    pub fn summary(&self) -> Value {
        Value::Object(
            self.workers
                .iter()
                .map(|(k, w)| (k.clone(), json!(w.state)))
                .collect(),
        )
    }
}
impl crate::flow::WorkerControl for Workers {
    fn observe(&self, name: &str) -> Option<(String, bool)> {
        Workers::observe(self, name)
    }
    fn start(&mut self, name: &str, now: u64) -> Result<Value> {
        Workers::start(self, name, now)
    }
    /// Automatic restarts never take over an unadopted conflict; only an explicit request does.
    fn restart(&mut self, name: &str, now: u64) -> Result<Value> {
        if self.workers.get(name).is_some_and(|w| w.conflict.is_some()) {
            return Err(error(format!("worker {name}: an automatic restart never resolves an unadopted conflict")));
        }
        Workers::restart(self, name, now)
    }
}
enum Adoption {
    Adopted(Live),
    Gone(String),
    Conflict(Recorded),
}
/// Adopt only the very process Crab started: same pid and kernel start time, configured argv
/// as the command-line suffix and unchanged configured argv. `continuing_stop` adopts a live
/// identity whatever its argv, so that a stop Crab already began is finished, never abandoned.
fn adopt(pid: u32, starttime: &str, saved: &Value, w: &Worker, continuing_stop: bool) -> Adoption {
    let pgrp = match recorded_identity(pid, starttime) {
        Ok(g) => g,
        Err(why) => return Adoption::Gone(why),
    };
    let recorded_hash = saved["argv_sha256"].as_str().unwrap_or("").to_string();
    let started_unix = saved["started_unix"].as_u64().unwrap_or(0);
    let conflict = if recorded_hash != w.argv_sha256 {
        Some("configured argv changed while the recorded instance runs")
    } else if !proc_cmdline(pid).is_some_and(|c| cmdline_matches(&c, &w.spec.argv)) {
        Some("process command line no longer matches the configured argv")
    } else {
        None
    };
    match conflict {
        Some(why) if !continuing_stop => Adoption::Conflict(Recorded {
            pid,
            pgid: pgrp,
            starttime: starttime.into(),
            started_unix,
            argv_sha256: recorded_hash,
            detail: why.into(),
        }),
        _ => Adoption::Adopted(Live {
            pid,
            pgid: pgrp,
            starttime: starttime.into(),
            child: None,
            started_unix,
            adopted: true,
            argv_sha256: recorded_hash,
        }),
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use std::{
        os::unix::fs::PermissionsExt,
        sync::atomic::{AtomicU64, Ordering},
        thread,
        time::{Duration, SystemTime, UNIX_EPOCH},
    };
    static NEXT: AtomicU64 = AtomicU64::new(0);
    pub(crate) struct Fx {
        pub dir: PathBuf,
    }
    impl Drop for Fx {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.dir);
        }
    }
    fn wall() -> u64 {
        SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .unwrap()
            .as_secs()
    }
    pub(crate) fn fx() -> Fx {
        let dir = std::env::temp_dir().join(format!(
            "crab-workers-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir_all(dir.join("state")).unwrap();
        Fx { dir }
    }
    pub(crate) fn script(dir: &Path, name: &str, body: &str) -> String {
        let p = dir.join(name);
        fs::write(&p, format!("#!/bin/sh\n{body}\n")).unwrap();
        fs::set_permissions(&p, fs::Permissions::from_mode(0o700)).unwrap();
        p.to_string_lossy().into_owned()
    }
    /// Honours its marker (argv[1]); counts starts in argv[2].
    pub(crate) fn polite(dir: &Path) -> String {
        script(dir, "polite.sh", "echo start >> \"$2\"; while [ ! -e \"$1\" ] && [ -d \"${1%/*}\" ]; do sleep 0.05; done; echo graceful; exit 0")
    }
    fn spec(f: &Fx, name: &str, argv: Vec<String>) -> WorkerSpec {
        WorkerSpec {
            name: name.into(),
            argv,
            cwd: f.dir.clone(),
            log: f.dir.join(format!("{name}.log")),
            gpu: false,
            autostart: true,
            stop_marker: None,
            stop_grace_seconds: 300,
            restart: true,
            env: Default::default(),
        }
    }
    #[test]
    fn a_worker_gets_its_configured_environment_and_keeps_its_argv() {
        let f = fx();
        let out = f.dir.join("env.out");
        let mut w = spec(&f, "envy", vec!["/bin/sh".into(), "-c".into(), format!("printf %s \"$MOVA_TEST_ENV\" > {}; sleep 30", out.display())]);
        w.env.insert("MOVA_TEST_ENV".into(), "from-config".into());
        let mut ws = Workers::new(f.dir.join("state"), vec![w], None).unwrap();
        ws.boot(1);
        until(&mut ws, 1, "envy", "running");
        for _ in 0..100 {
            if std::fs::read_to_string(&out).map(|s| s == "from-config").unwrap_or(false) { break; }
            std::thread::sleep(std::time::Duration::from_millis(20));
        }
        assert_eq!(std::fs::read_to_string(&out).unwrap(), "from-config");
        ws.stop_with("envy", 2, true).unwrap();
    }
    fn until(ws: &mut Workers, t: u64, name: &str, state: &str) {
        for _ in 0..250 {
            ws.tick(t);
            if ws.workers[name].state == state {
                return;
            }
            thread::sleep(Duration::from_millis(20));
        }
        panic!("{name} never reached {state}: {}", ws.status(Some(name), t).unwrap());
    }
    fn pid(ws: &Workers, name: &str) -> u32 {
        ws.workers[name].live.as_ref().unwrap().pid
    }
    fn starts(f: &Fx) -> usize {
        fs::read_to_string(f.dir.join("starts")).map(|s| s.lines().count()).unwrap_or(0)
    }
    fn alive(pid: u32) -> bool {
        proc_stat(pid).is_some_and(|(s, _, _)| s != 'Z')
    }
    #[test]
    fn configuration_refuses_untrusted_shapes() {
        let f = fx();
        let good = spec(&f, "eel", vec!["/bin/true".into()]);
        assert!(Workers::new(f.dir.clone(), vec![good.clone()], None).is_ok());
        for bad in [
            WorkerSpec { name: "../eel".into(), ..good.clone() },
            WorkerSpec { argv: vec!["true".into()], ..good.clone() },
            WorkerSpec { argv: vec![], ..good.clone() },
            WorkerSpec { log: "eel.log".into(), ..good.clone() },
            WorkerSpec { stop_marker: Some("STOP".into()), ..good.clone() },
            WorkerSpec { stop_grace_seconds: 0, ..good.clone() },
            WorkerSpec { gpu: true, ..good.clone() },
        ] {
            assert!(Workers::new(f.dir.clone(), vec![bad], None).is_err());
        }
        assert!(Workers::new(f.dir.clone(), vec![good.clone(), good.clone()], None).is_err());
        let bad_gpu = GpuCheck { nvidia_smi: "nvidia-smi".into(), timeout_seconds: 15, probe_argv: None };
        assert!(Workers::new(f.dir.clone(), vec![], Some(bad_gpu)).is_err());
        let unknown: std::result::Result<WorkerSpec, _> = serde_json::from_value(
            json!({"name":"eel","argv":["/bin/true"],"cwd":"/","log":"/tmp/x","shell":"rm -rf /"}),
        );
        assert!(unknown.is_err());
        assert_eq!(backoff(1), 5);
        assert_eq!(backoff(2), 10);
        assert_eq!(backoff(7), 300);
        assert_eq!(backoff(100), 300);
    }
    #[test]
    fn start_stop_by_marker_and_restart() {
        let f = fx();
        let marker = f.dir.join("STOP");
        let mut s = spec(&f, "eel", vec![polite(&f.dir), marker.to_string_lossy().into(), f.dir.join("starts").to_string_lossy().into()]);
        s.stop_marker = Some(marker.clone());
        let mut ws = Workers::new(f.dir.join("state"), vec![s], None).unwrap();
        let t = wall();
        ws.boot(t);
        assert_eq!(ws.workers["eel"].state, "running");
        let first = pid(&ws, "eel");
        let v = ws.stop("eel", t).unwrap();
        assert_eq!(v["state"], "stopping_marker");
        assert!(marker.is_file());
        until(&mut ws, t, "eel", "stopped");
        assert!(!alive(first));
        assert_eq!(ws.workers["eel"].last_exit["exit"]["code"], 0);
        assert!(fs::read_to_string(f.dir.join("eel.log")).unwrap().contains("graceful"));
        // Stopped workers are never restarted by ticks.
        ws.tick(t + 1000);
        assert_eq!(ws.workers["eel"].state, "stopped");
        ws.start("eel", t).unwrap();
        assert!(!marker.exists(), "start removes the stop marker first");
        until(&mut ws, t, "eel", "running");
        let second = pid(&ws, "eel");
        let v = ws.restart("eel", t).unwrap();
        assert_eq!(v["start_after_stop"], true);
        for _ in 0..250 {
            ws.tick(t);
            if ws.workers["eel"].live.as_ref().is_some_and(|l| l.pid != second) {
                break;
            }
            thread::sleep(Duration::from_millis(20));
        }
        assert_eq!(ws.workers["eel"].state, "running");
        assert_ne!(pid(&ws, "eel"), second);
        assert!(!alive(second));
        for _ in 0..100 {
            if starts(&f) == 3 {
                break;
            }
            thread::sleep(Duration::from_millis(20));
        }
        assert_eq!(starts(&f), 3);
        let saved: Value = serde_json::from_slice(&fs::read(f.dir.join("state/eel.json")).unwrap()).unwrap();
        assert_eq!(saved["pid"], pid(&ws, "eel"));
        assert_eq!(saved["restarts"], 1);
        ws.stop("eel", t).unwrap();
        until(&mut ws, t, "eel", "stopped");
    }
    #[test]
    fn immediate_stop_interrupts_without_waiting_for_the_marker() {
        let f = fx();
        let marker = f.dir.join("STOP");
        let mut s = spec(&f, "eel", vec![polite(&f.dir), marker.to_string_lossy().into(), f.dir.join("starts").to_string_lossy().into()]);
        s.stop_marker = Some(marker.clone());
        let mut ws = Workers::new(f.dir.join("state"), vec![s], None).unwrap();
        let t = wall();
        ws.boot(t);
        let first = pid(&ws, "eel");
        let v = ws.stop_with("eel", t, true).unwrap();
        assert_eq!(v["state"], "stopping_term");
        assert!(!marker.exists(), "an emergency stop does not wait at the safe-point marker");
        until(&mut ws, t, "eel", "stopped");
        assert!(!alive(first));
        ws.tick(t + 1000);
        assert_eq!(ws.workers["eel"].state, "stopped", "an interrupted worker is not restarted");
    }
    #[test]
    fn sigterm_without_marker_and_kill_escalation_after_grace() {
        let f = fx();
        let term = script(&f.dir, "term.sh", &format!("trap 'echo term; exit 0' TERM; while [ -d '{}' ]; do sleep 0.05; done", f.dir.display()));
        let stubborn = script(&f.dir, "stubborn.sh", &format!("trap '' TERM; while [ -d '{}' ]; do sleep 0.05; done", f.dir.display()));
        let marker = f.dir.join("IGNORED");
        let mut s2 = spec(&f, "stubborn", vec![stubborn]);
        s2.stop_marker = Some(marker.clone());
        s2.stop_grace_seconds = 30;
        let mut ws = Workers::new(f.dir.join("state"), vec![spec(&f, "term", vec![term]), s2], None).unwrap();
        let t = wall();
        ws.boot(t);
        thread::sleep(Duration::from_millis(200));
        let p = pid(&ws, "term");
        assert_eq!(ws.stop("term", t).unwrap()["state"], "stopping_term");
        until(&mut ws, t, "term", "stopped");
        assert!(!alive(p));
        let p = pid(&ws, "stubborn");
        ws.stop("stubborn", t).unwrap();
        ws.tick(t + 29);
        assert_eq!(ws.workers["stubborn"].state, "stopping_marker", "grace is honoured");
        ws.tick(t + 30);
        assert_eq!(ws.workers["stubborn"].state, "stopping_term");
        thread::sleep(Duration::from_millis(300));
        ws.tick(t + 31);
        assert!(alive(p), "TERM is ignored by this worker");
        ws.tick(t + 40);
        assert_eq!(ws.workers["stubborn"].state, "stopping_kill");
        until(&mut ws, t + 40, "stubborn", "stopped");
        assert_eq!(ws.workers["stubborn"].last_exit["exit"]["signal"], libc::SIGKILL);
    }
    #[test]
    fn crash_restart_uses_doubling_backoff_and_marker_cancels() {
        let f = fx();
        let crash = script(&f.dir, "crash.sh", &format!("echo start >> '{}'; exit 3", f.dir.join("starts").display()));
        let marker = f.dir.join("STOP");
        let mut s = spec(&f, "crashy", vec![crash]);
        s.stop_marker = Some(marker.clone());
        let mut ws = Workers::new(f.dir.join("state"), vec![s], None).unwrap();
        let t = wall();
        ws.boot(t);
        until(&mut ws, t, "crashy", "backoff");
        assert_eq!(ws.workers["crashy"].next_start, t + 5);
        assert_eq!(ws.workers["crashy"].last_exit["exit"]["code"], 3);
        ws.tick(t + 4);
        assert_eq!(starts(&f), 1, "no restart before the backoff");
        ws.tick(t + 5);
        until(&mut ws, t + 5, "crashy", "backoff");
        assert_eq!(ws.workers["crashy"].next_start, t + 5 + 10);
        ws.tick(t + 15);
        until(&mut ws, t + 15, "crashy", "backoff");
        assert_eq!(ws.workers["crashy"].next_start, t + 15 + 20);
        assert_eq!(starts(&f), 3);
        assert_eq!(ws.workers["crashy"].restarts, 2);
        // A STOP marker placed by anyone cancels the pending restart.
        fs::write(&marker, "operator").unwrap();
        ws.tick(t + 35);
        assert_eq!(ws.workers["crashy"].state, "stopped");
        assert_eq!(starts(&f), 3);
        // restart=false: one exit, no restart.
        let mut once = spec(&f, "once", vec![script(&f.dir, "once.sh", "exit 1")]);
        once.restart = false;
        let mut ws = Workers::new(f.dir.join("state"), vec![once], None).unwrap();
        ws.boot(t);
        until(&mut ws, t, "once", "exited");
        ws.tick(t + 1000);
        assert_eq!(ws.workers["once"].state, "exited");
    }
    #[test]
    fn adoption_after_crab_restart_verifies_identity() {
        let f = fx();
        let marker = f.dir.join("STOP");
        let mut s = spec(&f, "eel", vec![polite(&f.dir), marker.to_string_lossy().into(), f.dir.join("starts").to_string_lossy().into()]);
        s.stop_marker = Some(marker.clone());
        let t = wall();
        let mut a = Workers::new(f.dir.join("state"), vec![s.clone()], None).unwrap();
        a.boot(t);
        let first = pid(&a, "eel");
        drop(a); // Crab exits; the worker keeps running.
        thread::sleep(Duration::from_millis(100));
        let mut b = Workers::new(f.dir.join("state"), vec![s.clone()], None).unwrap();
        b.boot(t);
        assert_eq!(b.workers["eel"].state, "running");
        assert_eq!(pid(&b, "eel"), first, "adopted, not duplicated");
        assert_eq!(b.workers["eel"].live.as_ref().unwrap().adopted, true);
        assert_eq!(starts(&f), 1);
        // Stop of an adopted worker: marker, then exit observed through /proc.
        b.stop("eel", t).unwrap();
        until(&mut b, t, "eel", "stopped");
        assert!(!alive(first));
        // A stopped worker is not autostarted by the next Crab.
        let mut c = Workers::new(f.dir.join("state"), vec![s.clone()], None).unwrap();
        c.boot(t);
        assert_eq!(c.workers["eel"].state, "stopped");
        c.start("eel", t).unwrap();
        let live = pid(&c, "eel");
        drop(c);
        // A recorded pid whose start time differs is another process: never adopted.
        let path = f.dir.join("state/eel.json");
        let mut saved: Value = serde_json::from_slice(&fs::read(&path).unwrap()).unwrap();
        saved["starttime"] = json!("1");
        fs::write(&path, serde_json::to_vec(&saved).unwrap()).unwrap();
        let mut d = Workers::new(f.dir.join("state"), vec![s.clone()], None).unwrap();
        d.boot(t);
        let fresh = pid(&d, "eel");
        assert_ne!(fresh, live, "start-time mismatch must not be adopted");
        assert!(d.workers["eel"].events.iter().any(|e| e["event"] == "recorded_instance_gone"));
        unsafe {
            libc::kill(-(live as i32), libc::SIGKILL);
        }
        // Changed configured argv while the recorded instance runs: conflict, no duplicate.
        drop(d);
        let mut changed = s.clone();
        changed.argv.push("extra".into());
        let mut e = Workers::new(f.dir.join("state"), vec![changed], None).unwrap();
        e.boot(t);
        assert_eq!(e.workers["eel"].state, "unadopted_conflict");
        assert_eq!(e.start("eel", t).unwrap()["started"], false);
        assert!(e.stop("eel", t).is_err());
        unsafe {
            libc::kill(-(fresh as i32), libc::SIGKILL);
        }
    }

    fn saved(f: &Fx, name: &str) -> Value {
        serde_json::from_slice(&fs::read(f.dir.join(format!("state/{name}.json"))).unwrap()).unwrap()
    }
    fn wait_starts(f: &Fx, n: usize) {
        for _ in 0..100 {
            if starts(f) >= n {
                break;
            }
            thread::sleep(Duration::from_millis(20));
        }
    }
    #[test]
    fn live_conflict_start_never_signals_and_explicit_restart_takes_over() {
        let f = fx();
        let marker = f.dir.join("STOP");
        let mut s = spec(&f, "eel", vec![polite(&f.dir), marker.to_string_lossy().into(), f.dir.join("starts").to_string_lossy().into()]);
        s.stop_marker = Some(marker.clone());
        let t = wall();
        let mut a = Workers::new(f.dir.join("state"), vec![s.clone()], None).unwrap();
        a.boot(t);
        let first = pid(&a, "eel");
        wait_starts(&f, 1);
        drop(a);
        let mut changed = s.clone();
        changed.argv.push("extra".into());
        let mut b = Workers::new(f.dir.join("state"), vec![changed.clone()], None).unwrap();
        b.boot(t);
        assert_eq!(b.workers["eel"].state, "unadopted_conflict");
        let rec = saved(&f, "eel");
        assert_eq!(rec["pid"], first, "the conflict keeps the recorded identity on disk");
        assert_ne!(rec["argv_sha256"], rec["configured_argv_sha256"]);
        // A plain start: no second copy, no signal, a report with what to do.
        let v = b.start("eel", t).unwrap();
        assert_eq!(v["started"], false);
        assert_eq!(v["signal_sent"], false);
        assert_eq!(v["state"], "unadopted_conflict");
        assert_eq!(v["conflict"]["recorded_pid"], first);
        assert_eq!(v["conflict"]["configured_argv"].as_array().unwrap().last().unwrap(), "extra");
        assert_ne!(v["conflict"]["recorded_cmdline"].as_array().unwrap().last().unwrap(), "extra");
        assert!(v["conflict"]["resolution"].as_str().unwrap().contains("worker_restart eel"));
        for i in 0..5 {
            b.tick(t + i);
        }
        assert!(!marker.exists(), "no stop marker for a plain start");
        assert!(alive(first));
        assert_eq!(starts(&f), 1);
        drop(b);
        // The next Crab finds the same live conflict instead of starting a duplicate.
        let mut c = Workers::new(f.dir.join("state"), vec![changed.clone()], None).unwrap();
        c.boot(t);
        assert_eq!(c.workers["eel"].state, "unadopted_conflict");
        thread::sleep(Duration::from_millis(200));
        assert_eq!(starts(&f), 1, "no duplicate after a Crab restart");
        // Neither an automatic (flow) restart nor a stop touches it.
        assert!(crate::flow::WorkerControl::restart(&mut c, "eel", t).is_err());
        assert!(c.stop("eel", t).unwrap_err().to_string().contains("worker_restart eel"));
        assert!(!c.workers["eel"].stop_requested, "a refused stop records nothing");
        assert!(alive(first));
        // The explicit restart stops the recorded process as a normal stop, then starts.
        let v = c.restart("eel", t).unwrap();
        assert_eq!(v["took_over_conflict_pid"], first);
        assert_eq!(v["state"], "stopping_marker");
        assert!(marker.is_file());
        for _ in 0..250 {
            c.tick(t);
            if c.workers["eel"].live.as_ref().is_some_and(|l| l.pid != first) {
                break;
            }
            thread::sleep(Duration::from_millis(20));
        }
        assert_eq!(c.workers["eel"].state, "running");
        assert!(!alive(first));
        wait_starts(&f, 2);
        assert_eq!(starts(&f), 2);
        let rec = saved(&f, "eel");
        assert_eq!(rec["argv_sha256"], rec["configured_argv_sha256"]);
        assert!(c.workers["eel"].events.iter().any(|e| e["event"] == "conflict_takeover"));
        c.stop("eel", t).unwrap();
        until(&mut c, t, "eel", "stopped");
    }
    #[test]
    fn dead_conflict_is_cleared_and_started_by_start_or_tick() {
        let f = fx();
        let marker = f.dir.join("STOP");
        let mut s = spec(&f, "eel", vec![polite(&f.dir), marker.to_string_lossy().into(), f.dir.join("starts").to_string_lossy().into()]);
        s.stop_marker = Some(marker.clone());
        let mut changed = s.clone();
        changed.argv.push("extra".into());
        let t = wall();
        let mut a = Workers::new(f.dir.join("state"), vec![s.clone()], None).unwrap();
        a.boot(t);
        let first = pid(&a, "eel");
        wait_starts(&f, 1);
        drop(a);
        let mut b = Workers::new(f.dir.join("state"), vec![changed.clone()], None).unwrap();
        b.boot(t);
        assert_eq!(b.workers["eel"].state, "unadopted_conflict");
        unsafe {
            libc::kill(-(first as i32), libc::SIGKILL);
        }
        thread::sleep(Duration::from_millis(100));
        // worker_start: the record is cleared, journaled and the configured argv starts.
        let v = b.start("eel", t).unwrap();
        assert_eq!(v["state"], "running");
        assert!(v["conflict"].is_null());
        assert!(b.workers["eel"].events.iter().any(|e| e["event"] == "recorded_instance_gone" && e["cleared"] == "unadopted_conflict"));
        let second = pid(&b, "eel");
        wait_starts(&f, 2);
        assert_eq!(starts(&f), 2);
        drop(b);
        // Tick: the recorded process ends while in conflict; an autostart worker starts.
        let mut c = Workers::new(f.dir.join("state"), vec![s.clone()], None).unwrap();
        c.boot(t);
        assert_eq!(c.workers["eel"].state, "unadopted_conflict");
        unsafe {
            libc::kill(-(second as i32), libc::SIGKILL);
        }
        until(&mut c, t, "eel", "running");
        assert!(c.workers["eel"].events.iter().any(|e| e["event"] == "recorded_instance_gone"));
        wait_starts(&f, 3);
        assert_eq!(starts(&f), 3);
        c.stop("eel", t).unwrap();
        until(&mut c, t, "eel", "stopped");
    }
    #[test]
    fn conflict_takeover_stop_survives_a_crab_restart() {
        let f = fx();
        let marker = f.dir.join("IGNORED");
        let body = format!("trap 'exit 0' TERM; echo start >> '{}'; while [ -d '{}' ]; do sleep 0.05; done", f.dir.join("starts").display(), f.dir.display());
        let mut s = spec(&f, "eel", vec![script(&f.dir, "deaf.sh", &body)]);
        s.stop_marker = Some(marker.clone());
        let mut changed = s.clone();
        changed.argv.push("extra".into());
        let t = wall();
        let mut a = Workers::new(f.dir.join("state"), vec![s.clone()], None).unwrap();
        a.boot(t);
        let first = pid(&a, "eel");
        wait_starts(&f, 1);
        drop(a);
        let mut b = Workers::new(f.dir.join("state"), vec![changed.clone()], None).unwrap();
        b.boot(t);
        b.restart("eel", t).unwrap();
        assert_eq!(b.workers["eel"].state, "stopping_marker");
        drop(b); // Crab exits during the grace; the old process ignores the marker.
        let mut c = Workers::new(f.dir.join("state"), vec![changed], None).unwrap();
        c.boot(t);
        assert_eq!(c.workers["eel"].state, "stopping_marker", "the begun stop continues, not a new conflict");
        assert!(alive(first));
        c.tick(t + 300);
        assert_eq!(c.workers["eel"].state, "stopping_term");
        for _ in 0..250 {
            c.tick(t + 300);
            if c.workers["eel"].live.as_ref().is_some_and(|l| l.pid != first) {
                break;
            }
            thread::sleep(Duration::from_millis(20));
        }
        assert_eq!(c.workers["eel"].state, "running");
        assert!(!alive(first));
        wait_starts(&f, 2);
        assert_eq!(starts(&f), 2);
        let p = pid(&c, "eel");
        unsafe {
            libc::kill(-(p as i32), libc::SIGKILL);
        }
    }
    #[test]
    fn gpu_gate_blocks_on_failure_and_admits_on_pass() {
        let f = fx();
        let smi = f.dir.join("nvidia-smi");
        let set_smi = |body: &str| {
            let _ = fs::remove_file(&smi);
            script(&f.dir, "nvidia-smi", body);
        };
        set_smi("exit 9");
        let gpu = GpuCheck { nvidia_smi: smi.clone(), timeout_seconds: 15, probe_argv: Some(vec![script(&f.dir, "probe.sh", &format!("test -e '{}'", f.dir.join("probe-ok").display()))]) };
        let marker = f.dir.join("STOP");
        let mut s = spec(&f, "eel", vec![polite(&f.dir), marker.to_string_lossy().into(), f.dir.join("starts").to_string_lossy().into()]);
        s.gpu = true;
        s.stop_marker = Some(marker);
        let mut ws = Workers::new(f.dir.join("state"), vec![s], Some(gpu)).unwrap();
        let t = wall();
        ws.boot(t);
        assert_eq!(ws.workers["eel"].state, "waiting_gpu");
        let settle = |ws: &mut Workers, now: u64, want: &str, failures: u64| {
            for _ in 0..250 {
                ws.tick(now);
                if ws.gate_record["state"] == want && ws.gate_record["failures"].as_u64().unwrap_or(0) == failures {
                    return;
                }
                thread::sleep(Duration::from_millis(20));
            }
            panic!("gate never {want}: {}", ws.gate_record);
        };
        settle(&mut ws, t, "failed", 1);
        assert_eq!(ws.workers["eel"].state, "waiting_gpu");
        assert!(ws.workers["eel"].live.is_none(), "never start blind");
        assert_eq!(ws.gate_record["retry_in_seconds"], 5);
        // Hanging nvidia-smi: killed at the timeout, counted as a failure.
        set_smi("exec sleep 30");
        ws.tick(t + 5);
        assert_eq!(ws.gate_record["state"], "running");
        ws.tick(t + 19);
        assert_eq!(ws.gate_record["state"], "running");
        ws.tick(t + 20);
        assert_eq!(ws.gate_record["state"], "failed");
        assert_eq!(ws.gate_record["retry_in_seconds"], 10);
        // nvidia-smi answers but the probe fails: still blocked.
        set_smi("echo '0, Fake GPU, 1 MiB, 2 MiB'");
        settle(&mut ws, t + 30, "failed", 3);
        assert!(ws.gate_record["reason"].as_str().unwrap().contains("probe"));
        assert_eq!(ws.workers["eel"].state, "waiting_gpu");
        assert_eq!(starts(&f), 0);
        fs::write(f.dir.join("probe-ok"), "").unwrap();
        settle(&mut ws, t + 100, "passed", 0);
        assert_eq!(ws.workers["eel"].state, "running");
        assert!(ws.gate_record["output"].as_str().unwrap().contains("Fake GPU"));
        let saved: Value = serde_json::from_slice(&fs::read(f.dir.join("state/gpu-check.json")).unwrap()).unwrap();
        assert_eq!(saved["state"], "passed");
        ws.stop("eel", t + 100).unwrap();
        until(&mut ws, t + 100, "eel", "stopped");
    }
    #[test]
    fn unknown_worker_and_unbooted_supervision_are_refused() {
        let f = fx();
        let mut ws = Workers::new(f.dir.join("state"), vec![spec(&f, "eel", vec!["/bin/true".into()])], None).unwrap();
        assert!(ws.start("eel", 0).is_err(), "not booted");
        let mut s = spec(&f, "eel", vec!["/bin/true".into()]);
        s.autostart = false;
        let mut ws2 = Workers::new(f.dir.join("state"), vec![s], None).unwrap();
        ws2.boot(1);
        assert_eq!(ws2.workers["eel"].state, "stopped");
        assert!(ws2.start("whale", 1).is_err());
        assert!(ws2.status(Some("whale"), 1).is_err());
        assert_eq!(ws2.summary(), json!({"eel":"stopped"}));
        ws.boot(1);
        let _ = ws;
    }
}
