//! Crab: bounded role control, review journals and explicit external-role hooks.
#[path = "status_observation.rs"]
pub mod status_observation;
use crate::consult_queue;
use crate::council::{self, Attention, Council};
use crate::flow::{self, Flow};
use crate::workers::{self, Workers};
use crate::Result;
use serde::Deserialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, DirBuilder, File, OpenOptions},
    io::{BufRead, BufReader, Read, Write},
    os::{
        fd::AsRawFd,
        unix::{
            fs::{DirBuilderExt, FileTypeExt, MetadataExt, OpenOptionsExt, PermissionsExt},
            net::{UnixListener, UnixStream},
            process::CommandExt,
        },
    },
    path::{Path, PathBuf},
    process::{Command, Stdio},
    sync::{
        atomic::{AtomicU64, Ordering},
        Arc, Mutex,
    },
    thread,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};
const REQUEST_MAX: usize = 16 * 1024;
/// An `ask` prompt may be arbitrarily long in practice; the frame is still bounded so one
/// client cannot exhaust memory. Other operations keep REQUEST_MAX.
const ASK_FRAME_MAX: usize = 64 * 1024 * 1024;
const RESPONSE_MAX: usize = 64 * 1024;
const REPORT_MAX: usize = 16 * 1024;
const JOURNAL_MAX: usize = 128 * 1024;
const SESSION: &str = "octopus-chief-review";
static NEXT: AtomicU64 = AtomicU64::new(0);
fn error(s: impl Into<String>) -> Box<dyn std::error::Error> {
    s.into().into()
}
fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn absolute(p: &Path) -> Result<()> {
    if !p.is_absolute() {
        return Err(error("configuration paths must be absolute"));
    }
    Ok(())
}
fn read_limited(p: &Path, max: usize) -> Result<Vec<u8>> {
    let mut b = Vec::new();
    File::open(p)?.take(max as u64 + 1).read_to_end(&mut b)?;
    if b.len() > max {
        return Err(error("known artifact exceeds bounded read limit"));
    }
    Ok(b)
}
fn load(p: &Path, max: usize) -> Result<Value> {
    Ok(serde_json::from_slice(&read_limited(p, max)?)?)
}
fn atomic(p: &Path, v: &Value) -> Result<()> {
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
    File::open(p.parent().ok_or("missing parent")?)?.sync_all()?;
    Ok(())
}
fn lock(p: &Path) -> Result<File> {
    let f = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .mode(0o600)
        .open(p)?;
    if unsafe { libc::flock(f.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } != 0 {
        return Err(error("another Crab operation holds the lease"));
    }
    Ok(f)
}
fn private_dir(p: &Path) -> Result<()> {
    absolute(p)?;
    if !p.exists() {
        DirBuilder::new().recursive(true).mode(0o700).create(p)?;
    }
    let m = fs::symlink_metadata(p)?;
    if !m.is_dir() || m.uid() != unsafe { libc::geteuid() } || m.mode() & 0o077 != 0 {
        return Err(error("Crab directory must be owned and private (0700)"));
    }
    Ok(())
}
#[derive(Deserialize)]
struct Config {
    socket: PathBuf,
    run_root: PathBuf,
    fish_config: PathBuf,
    eel_config: PathBuf,
    #[serde(alias = "consultant_session")]
    opus_session: String,
    #[serde(default)]
    consultant_terminal: Option<TerminalProfile>,
    #[serde(default = "default_tmux")]
    tmux_bin: PathBuf,
    #[serde(default)]
    serve_requests: usize,
    #[serde(default)]
    status_attachments: Vec<status_observation::StatusAttachment>,
    /// Trusted long-running workers Crab starts, adopts, restarts and stops.
    #[serde(default)]
    workers: Vec<workers::WorkerSpec>,
    #[serde(default)]
    gpu_check: Option<workers::GpuCheck>,
    /// Crab-initiated consultations of a named Dolphin; absent means off.
    #[serde(default)]
    consult_triggers: Option<consult_queue::Triggers>,
    /// The Dolphin pod: any number of named consultant terminals. Absent means one Dolphin named
    /// `consultant` on `opus_session` with `consultant_terminal` (the earlier single form).
    #[serde(default)]
    dolphins: Vec<DolphinSpec>,
    /// Work-flow probes of the continuously running workers; absent means off.
    #[serde(default)]
    flow: Option<flow::FlowConfig>,
    /// Periodic council of consultant Dolphins over the supervision state; absent means off.
    #[serde(default)]
    council: Option<council::CouncilConfig>,
}
/// One addressable Dolphin terminal Crab may consult.
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct DolphinSpec {
    name: String,
    session: String,
    /// Ownership/readiness profile; only the legacy session may omit it.
    #[serde(default)]
    terminal: Option<TerminalProfile>,
    /// The configured worker (`crab-tmux-worker`) that holds this Dolphin, if any.
    #[serde(default)]
    worker: Option<String>,
    /// Checked before every consultation (for a self-updating CLI such as Copilot).
    #[serde(default)]
    pin: Option<PinSpec>,
}
#[derive(Clone, Deserialize)]
#[serde(deny_unknown_fields)]
struct PinSpec {
    #[serde(default)]
    file: Option<PathBuf>,
    #[serde(default)]
    sha256: Option<String>,
    #[serde(default)]
    version_cmd: Vec<String>,
    #[serde(default)]
    version: Option<String>,
    #[serde(default)]
    auth_file: Option<PathBuf>,
    #[serde(default)]
    auth_keys: Vec<String>,
}
impl PinSpec {
    fn pin(&self) -> Result<crate::tmux_worker::Pin> {
        let hex = |h: &str| h.len() == 64 && h.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b));
        let file = match (&self.file, &self.sha256) {
            (Some(f), Some(h)) if f.is_absolute() && hex(h) => Some((f.clone(), h.clone())),
            (None, None) => None,
            _ => return Err(error("pin file needs an absolute path and a lowercase SHA-256")),
        };
        let version = match (self.version_cmd.is_empty(), &self.version) {
            (false, Some(v)) if Path::new(&self.version_cmd[0]).is_absolute() && self.version_cmd.len() <= 16 => {
                Some((self.version_cmd.clone(), v.clone()))
            }
            (true, None) => None,
            _ => return Err(error("pin version needs an absolute version_cmd and the expected line")),
        };
        let auth = match (&self.auth_file, self.auth_keys.is_empty()) {
            (Some(f), false) if f.is_absolute() && self.auth_keys.len() <= 8 => Some((f.clone(), self.auth_keys.clone())),
            (None, true) => None,
            _ => return Err(error("pin auth needs an absolute auth_file and its keys")),
        };
        Ok(crate::tmux_worker::Pin { file, version, auth })
    }
}
/// A validated Dolphin target.
struct Dolphin {
    name: String,
    session: String,
    terminal: Option<TerminalProfile>,
    worker: Option<String>,
    pin: Option<crate::tmux_worker::Pin>,
}
/// Trusted attachment profile, independent of abstract Octopus/Dolphin roles.
/// Model is an operator preference/label; attaching never changes an existing session's model.
#[derive(Clone, Deserialize)]
struct TerminalProfile {
    #[serde(default = "default_provider")]
    provider: String,
    #[serde(default = "default_model")]
    model: String,
    pane_command: String,
    idle_marker: String,
    #[serde(default)]
    prompt_marker: Option<String>,
    #[serde(default)]
    required_text: Option<String>,
    owner_marker: String,
    /// The idle marker may stand on any of the last `idle_window` non-empty lines (1..32;
    /// default 1, the very last). TUIs such as Codex print footer lines below it.
    #[serde(default)]
    idle_window: Option<usize>,
}
/// Busy and modal phrases are looked for only in this many bottom non-empty lines, so old
/// scrollback cannot hold a terminal busy forever.
const BOTTOM_LINES: usize = 12;
fn default_provider() -> String {
    "claude-cli".into()
}
fn default_model() -> String {
    "opus".into()
}
fn label(s: &str, cap: usize) -> bool {
    !s.is_empty() && s.len() <= cap && !s.chars().any(char::is_control)
}
fn session_name(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 64
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
}
fn default_tmux() -> PathBuf {
    PathBuf::from("/usr/bin/tmux")
}
/// Specialized implementations are supplied by a trusted embedding application.
/// The Whale operations (`start_whales`, `whale_status`, role `whales`) are served by such a
/// handler; this crate contains no Whale code. The names used before the Whale rename
/// (`start_sharks`, `shark_status`, role `sharks`, configuration keys `shark_*`) are accepted for
/// one release.
/// Hooks may perform effects only after the ordinary pins/protocol/journal gates admit a request.
pub trait ExternalRoleHandler {
    fn status(&self, config: &Value, swarm: Option<&Value>) -> Result<Value>;
    fn start(&self, config: &Value, swarm: bool) -> Result<Value>;
    fn marker(&self, config: &Value, operation: &str) -> Result<Value>;
}
/// Understanding of arbitrary text prompts (`op:"ask"`), installed by the embedding binary.
/// `crab` answers nested read-only Crab operations in-process (status, read_reports).
pub trait PromptHandler {
    fn ask(
        &self,
        prompt: &str,
        invocation_id: &str,
        execute: bool,
        crab: &dyn Fn(&Value) -> std::result::Result<Value, String>,
    ) -> Result<Value>;
}
pub struct NoPromptHandler;
impl PromptHandler for NoPromptHandler {
    fn ask(&self, _: &str, _: &str, _: bool, _: &dyn Fn(&Value) -> std::result::Result<Value, String>) -> Result<Value> {
        Err(error("prompt understanding not installed"))
    }
}
/// Public standalone control never implicitly links or launches specialized private actors.
pub struct NoExternalRoles;
impl ExternalRoleHandler for NoExternalRoles {
    fn status(&self, _: &Value, _: Option<&Value>) -> Result<Value> {
        Err(error("external role handler not installed"))
    }
    fn start(&self, _: &Value, _: bool) -> Result<Value> {
        Err(error("external role handler not installed"))
    }
    fn marker(&self, _: &Value, _: &str) -> Result<Value> {
        Err(error("external role handler not installed"))
    }
}
struct Context {
    external: Arc<dyn ExternalRoleHandler>,
    prompt: Arc<dyn PromptHandler>,
    c: Config,
    fish: Value,
    eel: Value,
    fish_sha: String,
    eel_sha: String,
    swarm: Option<Value>,
    swarm_pin: Option<(PathBuf, String)>,
    owner: String,
    raw: Value,
    workers: Mutex<Workers>,
    consults: consult_queue::Queue,
    dolphins: Vec<Dolphin>,
    flow: Option<Mutex<Flow>>,
    council: Option<Mutex<Council>>,
    attention: Attention,
}
impl Context {
    #[cfg(test)]
    fn new(v: &Value) -> Result<Self> {
        Self::with_roles(v, Arc::new(NoExternalRoles))
    }
    fn new_checked(v: &Value) -> Result<Self> {
        Self::with_roles(v, Arc::new(NoExternalRoles))
    }
    fn with_roles(v: &Value, external: Arc<dyn ExternalRoleHandler>) -> Result<Self> {
        let c: Config = serde_json::from_value(v.clone())?;
        status_observation::validate(&c.status_attachments)?;
        let supervised = Workers::new(
            c.run_root.join("workers"),
            c.workers.clone(),
            c.gpu_check.clone(),
        )?;
        for p in [
            &c.socket,
            &c.run_root,
            &c.fish_config,
            &c.eel_config,
            &c.tmux_bin,
        ] {
            absolute(p)?;
        }
        let specs = if c.dolphins.is_empty() {
            vec![DolphinSpec { name: "consultant".into(), session: c.opus_session.clone(), terminal: c.consultant_terminal.clone(), worker: None, pin: None }]
        } else {
            c.dolphins.clone()
        };
        if specs.len() > 16 {
            return Err(error("at most 16 Dolphins"));
        }
        let mut dolphins: Vec<Dolphin> = Vec::new();
        for d in specs {
            if !workers::valid_name(&d.name) || dolphins.iter().any(|x| x.name == d.name || x.session == d.session) {
                return Err(error("Dolphin names and sessions must be unique ASCII identifiers"));
            }
            if !session_name(&d.session) {
                return Err(error("invalid exact consultant session name"));
            }
            if let Some(p) = &d.terminal {
                if !label(&p.provider, 128)
                    || !label(&p.model, 128)
                    || !label(&p.pane_command, 128)
                    || p.pane_command.contains('|')
                    || !label(&p.idle_marker, 256)
                    || !label(&p.owner_marker, 128)
                    || p.owner_marker.contains('|')
                    || p.prompt_marker.as_ref().is_some_and(|s| !label(s, 256))
                    || p.required_text.as_ref().is_some_and(|s| !label(s, 256))
                    || p.idle_window.is_some_and(|w| !(1..=32).contains(&w))
                {
                    return Err(error("invalid trusted consultant terminal profile"));
                }
            } else if d.session != SESSION {
                return Err(error("a nonlegacy consultant session requires an explicit terminal ownership/readiness profile"));
            }
            if d.worker.as_ref().is_some_and(|w| !c.workers.iter().any(|x| &x.name == w)) {
                return Err(error("a Dolphin's worker must be a configured worker"));
            }
            let pin = d.pin.as_ref().map(PinSpec::pin).transpose()?;
            dolphins.push(Dolphin { name: d.name, session: d.session, terminal: d.terminal, worker: d.worker, pin });
        }
        private_dir(&c.run_root)?;
        if c.socket.parent().ok_or("socket parent")?.canonicalize()? != c.run_root.canonicalize()? {
            return Err(error("socket must be inside the private Crab directory"));
        }
        for name in ["requests", "reports", "tasks", "tmp"] {
            private_dir(&c.run_root.join(name))?;
        }
        if let Some(t) = &c.consult_triggers {
            t.validate()?;
            if (t.refusals || t.worker_failures) && !t.dolphin.as_ref().is_some_and(|n| dolphins.iter().any(|d| &d.name == n)) {
                return Err(error("consult_triggers must name a configured Dolphin"));
            }
        }
        let consults = consult_queue::Queue::open(&c.run_root)?;
        let worker_names: Vec<String> = c.workers.iter().map(|w| w.name.clone()).collect();
        let flow = match &c.flow {
            Some(f) => {
                f.validate(&worker_names)?;
                Some(Mutex::new(Flow::open(f.clone(), &c.run_root)?))
            }
            None => None,
        };
        let council = match &c.council {
            Some(k) => {
                k.validate(&dolphins.iter().map(|d| d.name.clone()).collect::<Vec<_>>())?;
                if flow.is_none() {
                    return Err(error("the council reviews flow probes; configure flow first"));
                }
                Some(Mutex::new(Council::open(k.clone(), &c.run_root)?))
            }
            None => None,
        };
        let attention = Attention::open(&c.run_root)?;
        if !supervised.is_empty() {
            private_dir(supervised.dir())?;
        }
        let fb = read_limited(&c.fish_config, 256 * 1024)?;
        let eb = read_limited(&c.eel_config, 256 * 1024)?;
        let fish: Value = serde_json::from_slice(&fb)?;
        let eel: Value = serde_json::from_slice(&eb)?;
        role_root(&fish)?;
        role_root(&eel)?;
        let (swarm, swarm_pin) = if let Some(path) = legacy_key(v, "whale_swarm_config").as_str() {
            if !legacy_key(v, "whale_config").is_null() {
                return Err(error(
                    "configure either the Whale pool or the continuous swarm",
                ));
            }
            let path = PathBuf::from(path);
            absolute(&path)?;
            let bytes = read_limited(&path, 256 * 1024)?;
            let swarm: Value = serde_json::from_slice(&bytes)?;
            let root = role_root(&swarm)?;
            private_dir(&root)?;
            let physical = root.canonicalize()?;
            for other in [&c.run_root, &role_root(&fish)?, &role_root(&eel)?] {
                match other.canonicalize() {
                    Ok(path) if path == physical => {
                        return Err(error("swarm requires a physically separate run root"));
                    }
                    Ok(_) => {}
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                    Err(e) => return Err(e.into()),
                }
            }
            (Some(swarm), Some((path, hash(&bytes))))
        } else {
            (None, None)
        };
        let owner = format!(
            "peta-crab:{}\n",
            hash(&serde_json::to_vec(
                &json!({"socket":c.socket,"fish":c.fish_config,"eel":c.eel_config,"session":c.opus_session})
            )?)
        );
        Ok(Self {
            external,
            prompt: Arc::new(NoPromptHandler),
            c,
            fish,
            eel,
            fish_sha: hash(&fb),
            eel_sha: hash(&eb),
            swarm,
            swarm_pin,
            owner,
            raw: v.clone(),
            workers: Mutex::new(supervised),
            consults,
            dolphins,
            flow,
            council,
            attention,
        })
    }
    fn pins(&self) -> Result<()> {
        if hash(&read_limited(&self.c.fish_config, 256 * 1024)?) != self.fish_sha
            || hash(&read_limited(&self.c.eel_config, 256 * 1024)?) != self.eel_sha
        {
            return Err(error(
                "attached role configuration changed; restart Crab deliberately",
            ));
        }
        if let Some((path, expected)) = &self.swarm_pin {
            if hash(&read_limited(path, 256 * 1024)?) != *expected {
                return Err(error(
                    "swarm configuration changed; restart Crab deliberately",
                ));
            }
        }
        Ok(())
    }
    /// A named Dolphin, or the first (default) one.
    fn dolphin(&self, name: Option<&str>) -> Result<&Dolphin> {
        match name {
            None => Ok(&self.dolphins[0]),
            Some(n) => self
                .dolphins
                .iter()
                .find(|d| d.name == n)
                .ok_or_else(|| error(format!("no Dolphin named {n:?} is configured"))),
        }
    }
    fn journal(&self, id: &str) -> PathBuf {
        self.c
            .run_root
            .join("requests")
            .join(format!("{}.json", hash(id.as_bytes())))
    }
    fn report(&self, id: &str) -> PathBuf {
        self.c
            .run_root
            .join("reports")
            .join(format!("{}.md", hash(id.as_bytes())))
    }
}
fn role_root(v: &Value) -> Result<PathBuf> {
    let p = PathBuf::from(v["run_root"].as_str().ok_or("role config lacks run_root")?);
    absolute(&p)?;
    Ok(p)
}
fn valid_id(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 96
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"._:-".contains(&b))
}
#[derive(Deserialize)]
#[serde(deny_unknown_fields)]
struct Request {
    schema: u64,
    id: String,
    op: String,
    role: Option<String>,
    task: Option<String>,
    target_id: Option<String>,
    limit: Option<usize>,
    /// Arbitrary ordinary text for `op:"ask"`.
    prompt: Option<String>,
    /// `urgent` or `normal` (default) for `op:"ask"`: queue order only, never preemption.
    priority: Option<String>,
    /// A configured worker name for the worker_* operations.
    worker: Option<String>,
    /// Emergency interruption for worker_stop: do not wait for the worker's safe point.
    immediate: Option<bool>,
    /// A configured Dolphin name for consult_opus, consult_enqueue and consult_state.
    dolphin: Option<String>,
    /// An animal kind (probe group) for flow_status.
    group: Option<String>,
}
fn validate(r: &Request) -> Result<()> {
    if r.schema != 1 || !valid_id(&r.id) {
        return Err(error(
            "schema=1 and a bounded ASCII request id are required",
        ));
    }
    if r.role
        .as_deref()
        .is_some_and(|x| !matches!(x, "fish" | "eel" | "whales"))
    {
        return Err(error("role must be fish, eel or whales"));
    }
    if r.op != "ask" && r.prompt.is_some() {
        return Err(error("prompt belongs only to ask"));
    }
    if !matches!(r.op.as_str(), "ask" | "consult_enqueue") && r.priority.is_some() {
        return Err(error("priority belongs only to ask and consult_enqueue"));
    }
    if r.dolphin.is_some() && !matches!(r.op.as_str(), "consult_opus" | "consult_enqueue" | "consult_state") {
        return Err(error("dolphin belongs only to consult operations"));
    }
    if r.dolphin.as_deref().is_some_and(|d| !workers::valid_name(d)) {
        return Err(error("dolphin must be a configured ASCII Dolphin name"));
    }
    if r.group.is_some() && r.op != "flow_status" {
        return Err(error("group belongs only to flow_status"));
    }
    if r.group.as_deref().is_some_and(|g| !workers::valid_name(g)) {
        return Err(error("group must be an ASCII animal kind"));
    }
    let worker_op = workers::OPERATIONS.contains(&r.op.as_str());
    if !worker_op && r.worker.is_some() {
        return Err(error("worker belongs only to worker operations"));
    }
    if r.immediate.is_some() && r.op != "worker_stop" {
        return Err(error("immediate belongs only to worker_stop"));
    }
    if worker_op {
        if r.role.is_some() || r.task.is_some() || r.target_id.is_some() || r.limit.is_some() {
            return Err(error("worker operations take only a worker name"));
        }
        match r.worker.as_deref() {
            Some(w) if !workers::valid_name(w) => {
                return Err(error("worker must be a configured ASCII worker name"))
            }
            None if r.op != "worker_status" => return Err(error("worker required")),
            _ => {}
        }
        return Ok(());
    }
    match r.op.as_str() {
        "ask" => {
            let p = r.prompt.as_deref().ok_or("prompt required")?;
            if p.trim().is_empty() {
                return Err(error("prompt must be non-empty ordinary text"));
            }
            if !matches!(r.priority.as_deref(), None | Some("urgent" | "normal")) {
                return Err(error("priority must be urgent or normal"));
            }
            if r.role.is_some() || r.task.is_some() || r.target_id.is_some() || r.limit.is_some() {
                return Err(error("ask takes only prompt and priority"));
            }
        }
        "start_whales" | "whale_status" => {
            if r.role.is_some() || r.task.is_some() || r.target_id.is_some() || r.limit.is_some() {
                return Err(error(
                    "Whale pool operations use only trusted server configuration",
                ));
            }
        }
        "status" | "read_reports" => {
            if r.task.is_some() || r.target_id.is_some() {
                return Err(error("unexpected observation arguments"));
            }
            if r.limit.is_some_and(|n| !(1..=3).contains(&n)) {
                return Err(error("report limit must be 1..3"));
            }
        }
        "pause" | "resume" | "stop" => {
            if r.role.is_none() || r.task.is_some() || r.target_id.is_some() || r.limit.is_some() {
                return Err(error("marker operations take only one known role"));
            }
        }
        "consult_opus" => {
            let t = r.task.as_deref().ok_or("task required")?;
            if t.trim().is_empty()
                || t.len() > 12000
                || t.chars()
                    .any(|x| x.is_control() && !matches!(x, '\n' | '\t'))
                || r.role.is_some()
                || r.target_id.is_some()
                || r.limit.is_some()
            {
                return Err(error("bounded plain-text Opus task required"));
            }
        }
        "consult_enqueue" => {
            let t = r.task.as_deref().ok_or("task required")?;
            if !consult_queue::valid_task(t) || r.role.is_some() || r.target_id.is_some() || r.limit.is_some() {
                return Err(error("consult_enqueue takes one bounded plain-text task and an optional priority"));
            }
            if !matches!(r.priority.as_deref(), None | Some("urgent" | "normal")) {
                return Err(error("priority must be urgent or normal"));
            }
        }
        "flow_status" | "attention" => {
            if r.role.is_some() || r.task.is_some() || r.target_id.is_some() || r.limit.is_some() {
                return Err(error("flow_status takes only a group; attention takes no arguments"));
            }
        }
        "consult_state" | "dolphin_list" => {
            if r.role.is_some() || r.task.is_some() || r.target_id.is_some() || r.limit.is_some() {
                return Err(error("consult_state and dolphin_list take no arguments but a Dolphin name"));
            }
        }
        "opus_report" => {
            if !r.target_id.as_deref().is_some_and(valid_id)
                || r.role.is_some()
                || r.task.is_some()
                || r.limit.is_some()
            {
                return Err(error("opus_report requires one valid target_id"));
            }
        }
        _ => return Err(error("unsupported operation")),
    }
    Ok(())
}
fn fail(id: &str, code: &str, detail: &str) -> Value {
    json!({"schema":1,"id":id,"ok":false,"error":{"code":code,"detail":detail}})
}
fn trim_value(v: &Value, depth: usize) -> Value {
    if depth == 0 {
        return json!("[depth limit]");
    }
    match v {
        Value::String(s) => json!(s.chars().take(384).collect::<String>()),
        Value::Array(a) => json!(a
            .iter()
            .take(8)
            .map(|x| trim_value(x, depth - 1))
            .collect::<Vec<_>>()),
        Value::Object(m) => {
            let mut out = serde_json::Map::new();
            for (k, v) in m.iter().take(16) {
                out.insert(k.clone(), trim_value(v, depth - 1));
            }
            Value::Object(out)
        }
        _ => v.clone(),
    }
}
fn observed(p: &Path) -> Value {
    match load(p, 32 * 1024) {
        Ok(v) => {
            let age = v["heartbeat_unix"]
                .as_u64()
                .or_else(|| v["at"].as_u64())
                .map(|t| now().saturating_sub(t));
            json!({"path":p,"present":true,"read_unix":now(),"heartbeat_age_seconds":age,"stale_over_120_seconds":age.map(|x|x>120),"health_inferred":false,"value":trim_value(&v,4)})
        }
        Err(e) => json!({"path":p,"present":p.is_file(),"error":e.to_string()}),
    }
}
fn whales_status(ctx: &Context) -> Result<Value> {
    ctx.external.status(&ctx.raw, ctx.swarm.as_ref())
}
fn whales_start(ctx: &Context) -> Result<Value> {
    ctx.external.start(&ctx.raw, ctx.swarm.is_some())
}
fn status(ctx: &Context, role: Option<&str>) -> Result<Value> {
    let mut data = json!({"observed_unix":now(),"display_name":"Crab","attached_only":(legacy_key(&ctx.raw, "whale_config").is_null() && ctx.swarm.is_none())});
    if role.is_none() || role == Some("fish") {
        let root = role_root(&ctx.fish)?;
        let annotation = ctx.fish["annotation_root"]
            .as_str()
            .map(PathBuf::from)
            .unwrap_or_else(|| root.join("annotation"));
        absolute(&annotation)?;
        let n = ctx.fish["workers"].as_u64().unwrap_or(crate::DEFAULT_FISH_WORKERS).min(8);
        let mut workers = Vec::new();
        for i in 0..n {
            workers.push(observed(
                &annotation.join(format!("worker-{i:02}/campaign/status.json")),
            ));
        }
        data["fish"] = json!({"master":observed(&root.join("master-status.json")),"writer":observed(&root.join("writer-status.json")),"pool":observed(&annotation.join("status.json")),"workers":workers,"pause_present":root.join("PAUSE").exists(),"stop_present":root.join("STOP").exists()});
    }
    if role.is_none() || role == Some("eel") {
        let root = role_root(&ctx.eel)?;
        data["eel"] = json!({"actor":observed(&root.join("eel/status.json")),"trainer":observed(&root.join("trainer/status.json")),"pause_present":root.join("PAUSE").exists(),"stop_present":root.join("STOP").exists(),"trainer_failure_pause_present":root.join("trainer/PAUSE").exists()});
    }
    if role.is_none() || role == Some("whales") {
        data["whales"] = if legacy_key(&ctx.raw, "whale_config").is_null() && ctx.swarm.is_none() {
            json!({"configured":false,"process_spawned":false})
        } else {
            match whales_status(ctx) {
                Ok(v) => trim_value(&v, 6),
                Err(e) => json!({"error":e.to_string()}),
            }
        };
    }
    data["dolphin"] = if ctx.raw["dolphin_auto"] == true {
        match crate::dolphin::status(&ctx.raw) {
            Ok(v) => {
                json!({"auto_enabled":true,"queue":trim_value(&v,4),"idle_hook":observed(&ctx.c.run_root.join("dolphin-status.json"))})
            }
            Err(e) => json!({"auto_enabled":true,"error":e.to_string()}),
        }
    } else {
        json!({"auto_enabled":false,"queue_not_queried":true})
    };
    if let Ok(w) = ctx.workers.lock() {
        if !w.is_empty() {
            data["workers"] = w.summary();
        }
    }
    if !ctx.c.status_attachments.is_empty() {
        let read_unix = now();
        data["status_attachments"] = json!({
            "control_scope":"read_only_not_covered_by_role_controls",
            "observations":ctx.c.status_attachments.iter().map(|a| status_observation::observe(a,read_unix)).collect::<Vec<_>>()
        });
    }
    Ok(data)
}
fn owned_marker(ctx: &Context, p: &Path) -> Result<bool> {
    if !p.exists() {
        return Ok(false);
    }
    let m = fs::symlink_metadata(p)?;
    if !m.is_file()
        || m.uid() != unsafe { libc::geteuid() }
        || read_limited(p, 256)? != ctx.owner.as_bytes()
    {
        return Err(error("foreign marker; refusing to modify it"));
    }
    Ok(true)
}
fn marker(ctx: &Context, r: &Request) -> Result<Value> {
    let role = r.role.as_deref().ok_or("role required")?;
    if role == "whales" && ctx.swarm.is_none() {
        return ctx.external.marker(&ctx.raw, &r.op);
    }
    let root = match role {
        "whales" => role_root(ctx.swarm.as_ref().ok_or("swarm is not configured")?)?,
        "fish" => role_root(&ctx.fish)?,
        _ => role_root(&ctx.eel)?,
    };
    let target = root.join(if r.op == "stop" { "STOP" } else { "PAUSE" });
    if r.op == "resume" {
        if root.join("STOP").exists()
            || root.join("trainer/STOP").exists()
            || root.join("eel/STOP").exists()
        {
            return Err(error(
                "role has STOP; clearing PAUSE cannot restart a stopped process",
            ));
        }
        if role == "eel" && (root.join("trainer/PAUSE").exists() || root.join("eel/PAUSE").exists())
        {
            return Err(error(
                "trainer failure PAUSE requires investigation; refusing to resume",
            ));
        }
        if owned_marker(ctx, &target)? {
            fs::remove_file(&target)?;
            File::open(&root)?.sync_all()?;
        }
        return Ok(
            json!({"role":role,"pause_cleared":true,"process_spawned":false,"completion":"observe role status; no restart"}),
        );
    }
    if !owned_marker(ctx, &target)? {
        let mut f = OpenOptions::new()
            .create_new(true)
            .write(true)
            .mode(0o600)
            .open(&target)?;
        f.write_all(ctx.owner.as_bytes())?;
        f.sync_all()?;
        File::open(&root)?.sync_all()?;
    }
    Ok(
        json!({"role":role,"requested":r.op,"marker":target,"process_spawned":false,"completion":"request only; active work may still be running"}),
    )
}
fn report(ctx: &Context, id: &str) -> Result<Value> {
    let p = ctx.report(id);
    if !p.exists() {
        return Ok(json!({"target_id":id,"path":p,"status":"report_pending","present":false}));
    }
    let m = fs::symlink_metadata(&p)?;
    if !m.is_file() || m.uid() != unsafe { libc::geteuid() } {
        return Err(error("report is not an owned regular file"));
    }
    let mut bytes = Vec::new();
    File::open(&p)?
        .take(REPORT_MAX as u64)
        .read_to_end(&mut bytes)?;
    Ok(
        json!({"target_id":id,"path":p,"status":"report_present","present":true,"reviewed":false,"bytes":m.len(),"truncated":m.len()>bytes.len() as u64,"excerpt_sha256":hash(&bytes),"text":String::from_utf8_lossy(&bytes)}),
    )
}
fn known_reports(ctx: &Context, r: &Request) -> Result<Value> {
    let mut data = json!({"observed_unix":now(),"note":"bounded summaries; absence is explicit"});
    if r.role.is_none() || r.role.as_deref() == Some("fish") {
        let root = role_root(&ctx.fish)?;
        data["fish"] = json!({"writer":observed(&root.join("writer-status.json")),"master":observed(&root.join("master-status.json"))});
    }
    if r.role.is_none() || r.role.as_deref() == Some("eel") {
        let p = role_root(&ctx.eel)?.join("trainer/trained.json");
        let keys = [
            "state",
            "outcome",
            "generation",
            "generation_sha256",
            "train_ids",
            "dev_ids",
            "new_train",
            "counts",
            "baseline",
            "candidate",
            "metrics",
            "installed_sha256",
            "installed_weights_sha256",
            "model_sha256",
            "checkpoint",
            "reason",
            "train_seconds",
            "weights_sha256",
            "old_weights_sha256",
            "training_seconds",
            "before",
            "after",
            "parser_training_only",
            "development_interpretation",
            "error",
            "automatic_retry",
        ];
        data["eel"] = match load(&p, 2 * 1024 * 1024) {
            Ok(v) => {
                let mut m = serde_json::Map::new();
                for k in keys {
                    if let Some(x) = v.get(k) {
                        m.insert(k.into(), trim_value(x, 3));
                    }
                }
                json!({"path":p,"present":true,"summary":m})
            }
            Err(e) => json!({"path":p,"present":p.is_file(),"error":e.to_string()}),
        };
    }
    if let Some(g) = data["eel"]["summary"]["generation"]
        .as_str()
        .map(String::from)
    {
        if g.len() == 64 && g.bytes().all(|b| b.is_ascii_hexdigit()) {
            let p = role_root(&ctx.eel)?
                .join("trainer/generations")
                .join(g)
                .join("receipt.json");
            let keys = [
                "outcome",
                "generation",
                "new_train",
                "old_weights_sha256",
                "installed_weights_sha256",
                "training_seconds",
                "before",
                "after",
                "parser_training_only",
                "development_interpretation",
                "error",
                "automatic_retry",
            ];
            data["eel"]["last_receipt"] = match load(&p, 2 * 1024 * 1024) {
                Ok(v) => {
                    let mut m = serde_json::Map::new();
                    for k in keys {
                        if let Some(x) = v.get(k) {
                            m.insert(k.into(), trim_value(x, 3));
                        }
                    }
                    json!({"path":p,"present":true,"summary":m})
                }
                Err(e) => json!({"path":p,"present":p.is_file(),"error":e.to_string()}),
            };
        }
    }
    if r.role.is_none() || r.role.as_deref() == Some("eel") {
        data["eel"]["idle_actor"] = observed(&role_root(&ctx.eel)?.join("eel/status.json"));
    }
    let mut entries = Vec::new();
    for p in journal_files(ctx)? {
        let v = load(&p, JOURNAL_MAX)?;
        if v["op"] == "consult_opus" {
            entries.push((
                v["created_unix"].as_u64().unwrap_or(0),
                v["id"].as_str().ok_or("journal missing id")?.to_string(),
            ));
        }
    }
    entries.sort();
    let mut reports = Vec::new();
    for (_, id) in entries.iter().rev().take(r.limit.unwrap_or(2)) {
        reports.push(report(ctx, id)?);
    }
    data["opus"] = json!(reports);
    Ok(data)
}
fn journal_files(ctx: &Context) -> Result<Vec<PathBuf>> {
    let mut out = Vec::new();
    for e in fs::read_dir(ctx.c.run_root.join("requests"))? {
        let p = e?.path();
        if p.extension().and_then(|x| x.to_str()) == Some("json") {
            out.push(p);
        }
        if out.len() > 4096 {
            return Err(error(
                "journal observation limit reached; archive deliberately",
            ));
        }
    }
    out.sort();
    Ok(out)
}
#[cfg(test)]
fn idle(capture: &str, display: &str) -> bool {
    legacy_idle(capture, display, SESSION)
}
fn legacy_idle(capture: &str, display: &str, session: &str) -> bool {
    if display.trim() != "claude|0" {
        return false;
    }
    let lower = capture.to_lowercase();
    for x in [
        "how is claude doing",
        "esc to interrupt",
        "esc to cancel",
        "do you want to",
        "allow this",
        "approve this",
        "waiting for permission",
    ] {
        if lower.contains(x) {
            return false;
        }
    }
    let lines: Vec<_> = capture
        .lines()
        .map(str::trim)
        .filter(|x| !x.is_empty())
        .collect();
    lines.last() == Some(&"$")
        && lines.len() > 1
        && lines[lines.len() - 2] == session
        && lower.contains("done")
}
#[cfg(test)]
fn terminal_idle(ctx: &Context, capture: &str, display: &str) -> bool {
    terminal_idle_on(&ctx.dolphins[0], capture, display)
}
fn terminal_idle_on(d: &Dolphin, capture: &str, display: &str) -> bool {
    let Some(p) = &d.terminal else {
        return legacy_idle(capture, display, &d.session);
    };
    if display.trim() != format!("{}|0", p.pane_command) {
        return false;
    }
    let lines: Vec<_> = capture
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect();
    let bottom = lines[lines.len().saturating_sub(BOTTOM_LINES)..].join("\n").to_lowercase();
    let lower = capture.to_lowercase();
    for blocked in [
        "esc to interrupt",
        "esc to cancel",
        "do you want to",
        "allow this",
        "approve this",
        "waiting for permission",
        "update available",
        "update now",
        "trust this",
        "sign in",
        "log in",
    ] {
        if bottom.contains(blocked) {
            return false;
        }
    }
    let Some(last) = lines.last() else {
        return false;
    };
    let marker = p.idle_marker.trim();
    if let Some(prompt) = &p.prompt_marker {
        if *last != prompt.trim() || lines.len() < 2 || lines[lines.len() - 2] != marker {
            return false;
        }
    } else {
        // A printed line may carry a TUI bullet ("• DOLPHIN READY", "● DOLPHIN READY").
        let bare = |l: &str| l.trim_start_matches(|c: char| !c.is_alphanumeric()).trim().to_string();
        let window = p.idle_window.unwrap_or(1);
        if !lines[lines.len().saturating_sub(window)..].iter().any(|l| bare(l) == bare(marker)) {
            return false;
        }
    }
    !p.required_text
        .as_ref()
        .is_some_and(|t| !lower.contains(&t.to_lowercase()))
}
struct TempFiles(Vec<PathBuf>);
impl Drop for TempFiles {
    fn drop(&mut self) {
        for p in &self.0 {
            let _ = fs::remove_file(p);
        }
    }
}
fn tmux(ctx: &Context, args: &[&str]) -> Result<String> {
    let tag = format!(
        "{}-{}",
        std::process::id(),
        NEXT.fetch_add(1, Ordering::Relaxed)
    );
    let out = ctx.c.run_root.join("tmp").join(format!("{tag}.out"));
    let err = ctx.c.run_root.join("tmp").join(format!("{tag}.err"));
    let _files = TempFiles(vec![out.clone(), err.clone()]);
    let a = OpenOptions::new()
        .create_new(true)
        .write(true)
        .mode(0o600)
        .open(&out)?;
    let b = OpenOptions::new()
        .create_new(true)
        .write(true)
        .mode(0o600)
        .open(&err)?;
    let mut ch = Command::new(&ctx.c.tmux_bin)
        .args(args)
        .stdin(Stdio::null())
        .stdout(Stdio::from(a))
        .stderr(Stdio::from(b))
        .process_group(0)
        .spawn()?;
    let start = Instant::now();
    let status = loop {
        if let Some(x) = ch.try_wait()? {
            break x;
        }
        if start.elapsed() > Duration::from_secs(3) {
            unsafe {
                libc::kill(-(ch.id() as i32), libc::SIGKILL);
            }
            let _ = ch.wait();
            return Err(error(
                "terminal command timed out; delivery may be uncertain",
            ));
        }
        thread::sleep(Duration::from_millis(20));
    };
    let output = read_limited(&out, 32 * 1024)?;
    let stderr = read_limited(&err, 2048)?;
    if !status.success() {
        return Err(error(format!(
            "terminal command failed: {}",
            String::from_utf8_lossy(&stderr)
        )));
    }
    Ok(String::from_utf8(output)?)
}
/// Exact session target. tmux 3.5 resolves `=NAME` for has-session but not as a pane target for
/// display-message, capture-pane, paste-buffer or send-keys; `=NAME:` works for all of them.
fn consultant_target(d: &Dolphin) -> String {
    format!("={}:", d.session)
}
#[cfg(test)]
fn capture(ctx: &Context) -> Result<(String, String)> {
    capture_on(ctx, &ctx.dolphins[0])
}
fn capture_on(ctx: &Context, d: &Dolphin) -> Result<(String, String)> {
    let target = consultant_target(d);
    let format = if d.terminal.is_some() {
        "#{session_name}|#{session_windows}|#{window_panes}|#{pane_current_command}|#{pane_dead}|#{@mova_crab_owner}"
    } else {
        "#{pane_current_command}|#{pane_dead}"
    };
    let observed = tmux(ctx, &["display-message", "-p", "-t", &target, format])?;
    let display = if let Some(p) = &d.terminal {
        let fields: Vec<_> = observed.trim().split('|').collect();
        if fields.len() != 6
            || fields[0] != d.session
            || fields[1] != "1"
            || fields[2] != "1"
            || fields[3] != p.pane_command
            || fields[4] != "0"
            || fields[5] != p.owner_marker
        {
            return Err(error(
                "consultant terminal ownership/layout/command mismatch; no task sent",
            ));
        }
        format!("{}|0", p.pane_command)
    } else {
        observed
    };
    let capture = tmux(ctx, &["capture-pane", "-p", "-t", &target, "-S", "-80"])?;
    Ok((capture, display))
}
fn stage(ctx: &Context, r: &Request, name: &str) -> Result<()> {
    let p = ctx.journal(&r.id);
    let mut v = load(&p, JOURNAL_MAX)?;
    v["terminal_stage"] = json!(name);
    atomic(&p, &v)
}
fn consult(ctx: &Context, r: &Request) -> Result<Value> {
    let _lease = lock(&ctx.c.run_root.join("opus.lock"))?;
    let d = ctx.dolphin(r.dolphin.as_deref())?;
    if let Some(pin) = &d.pin {
        crate::tmux_worker::check_pin(&d.name, pin).map_err(error)?;
    }
    for p in journal_files(ctx)? {
        let v = load(&p, JOURNAL_MAX)?;
        if v["id"] == r.id || v["op"] != "consult_opus" || v["dolphin"].as_str().unwrap_or(&ctx.dolphins[0].name) != d.name {
            continue;
        }
        let pending = v["state"] == "reserved"
            || v["response"]["data"]["accepted"] == true
            || v["response"]["uncertain"] == true;
        if pending {
            let id = v["id"].as_str().ok_or("consult journal lacks id")?;
            if !ctx.report(id).exists() {
                return Err(error(
                    "prior consult remains pending/uncertain; inspect it before new work",
                ));
            }
        }
    }
    let (first, display) = capture_on(ctx, d)?;
    if !terminal_idle_on(d, &first, &display) {
        return Err(error(
            "Consultant is busy, modal, absent or ambiguous; no task sent",
        ));
    }
    let output = ctx.report(&r.id);
    if fs::symlink_metadata(&output).is_ok() {
        return Err(error("existing report artifact; refusing overwrite"));
    }
    let task = ctx
        .c
        .run_root
        .join("tasks")
        .join(format!("{}.md", hash(r.id.as_bytes())));
    let text = format!(
        "Crab requests one bounded read-only review at medium effort. Do not modify code, DB, models or workers; no teacher/training/API/external messages. Do not enable disabled roles. Use the existing repository context. Write the task's requested output as the complete report file only at {}. That exact report write is the sole tool exception if the task otherwise says no tools. A report file is evidence for later review, not approval to deploy.\n\nTask:\n{}\n",
        output.display(),
        r.task.as_deref().ok_or("task required")?
    );
    let mut f = OpenOptions::new()
        .create_new(true)
        .write(true)
        .mode(0o600)
        .open(&task)?;
    f.write_all(text.as_bytes())?;
    f.sync_all()?;
    let buffer = format!("crab-{}", &hash(r.id.as_bytes())[..20]);
    stage(ctx, r, "load_buffer_reserved")?;
    tmux(
        ctx,
        &[
            "load-buffer",
            "-b",
            &buffer,
            task.to_str().ok_or("task path UTF-8")?,
        ],
    )?;
    let (second, display2) = capture_on(ctx, d)?;
    if first != second || display != display2 || !terminal_idle_on(d, &second, &display2) {
        return Err(error(
            "Consultant state changed before paste; task was not knowingly sent",
        ));
    }
    let target = consultant_target(d);
    stage(ctx, r, "paste_reserved")?;
    tmux(
        ctx,
        &["paste-buffer", "-p", "-d", "-b", &buffer, "-t", &target],
    )?;
    stage(ctx, r, "enter_reserved")?;
    tmux(ctx, &["send-keys", "-t", &target, "Enter"])?;
    Ok(
        json!({"accepted":true,"dolphin":d.name,"session":d.session,"consultant_provider":d.terminal.as_ref().map(|p|p.provider.as_str()).unwrap_or("claude-cli"),"model_preference":d.terminal.as_ref().map(|p|p.model.as_str()).unwrap_or("opus"),"attached_model_verified":false,"report_path":output,"status":"report_pending","task_sha256":hash(text.as_bytes()),"task_sent_via":"existing terminal UI"}),
    )
}
/// A configuration key or its name before the Whale rename (`whale_*` read as `shark_*`).
fn legacy_key<'a>(v: &'a Value, key: &str) -> &'a Value {
    match &v[key] {
        Value::Null => &v[key.replacen("whale", "shark", 1).as_str()],
        x => x,
    }
}
/// Map the operation and role names used before the Whale rename to the current ones.
fn legacy_request(mut r: Request) -> Request {
    match r.op.as_str() {
        "start_sharks" => r.op = "start_whales".into(),
        "shark_status" => r.op = "whale_status".into(),
        _ => {}
    }
    if r.role.as_deref() == Some("sharks") {
        r.role = Some("whales".into());
    }
    r
}
fn dispatch(ctx: &Context, input: &Value) -> Value {
    let id = input["id"].as_str().unwrap_or("");
    let r: Request = match serde_json::from_value(input.clone()) {
        Ok(r) => legacy_request(r),
        Err(e) => return fail(id, "protocol", &e.to_string()),
    };
    if let Err(e) = validate(&r) {
        return fail(id, "protocol", &e.to_string());
    }
    let bytes = match serde_json::to_vec(input) {
        Ok(b) => b,
        Err(e) => return fail(id, "protocol", &e.to_string()),
    };
    let payload = hash(&bytes);
    let p = ctx.journal(id);
    if p.exists() {
        return match load(&p, JOURNAL_MAX) {
            Ok(v) if v["payload_sha256"] != payload => fail(
                id,
                "id_conflict",
                "request id already binds another payload",
            ),
            Ok(v) if v["state"] == "completed" => v["response"].clone(),
            Ok(_) => {
                json!({"schema":1,"id":id,"ok":false,"uncertain":true,"error":{"code":"uncertain","detail":"reserved request has no completed receipt; never automatically replayed"}})
            }
            Err(e) => fail(id, "journal", &e.to_string()),
        };
    }
    let mut record = json!({"schema":1,"id":id,"op":r.op,"payload_sha256":payload,"state":"reserved","created_unix":now()});
    if r.op == "consult_opus" {
        match ctx.dolphin(r.dolphin.as_deref()) {
            Ok(d) => record["dolphin"] = json!(d.name),
            Err(e) => return fail(id, "protocol", &e.to_string()),
        }
    }
    if let Err(e) = atomic(&p, &record) {
        return fail(id, "journal", &e.to_string());
    }
    let result = ctx.pins().and_then(|_| match r.op.as_str() {
        "status" => status(ctx, r.role.as_deref()),
        "read_reports" => known_reports(ctx, &r),
        "pause" | "resume" | "stop" => marker(ctx, &r),
        "start_whales" => whales_start(ctx),
        "whale_status" => whales_status(ctx),
        "consult_opus" => consult(ctx, &r),
        "opus_report" => report(ctx, r.target_id.as_deref().expect("validated target")),
        "consult_enqueue" => consult_enqueue(ctx, &r),
        "consult_state" => consult_state(ctx, r.dolphin.as_deref()),
        "dolphin_list" => dolphin_list(ctx),
        "flow_status" => flow_status(ctx, r.group.as_deref()),
        "attention" => attention(ctx),
        "ask" => ask_op(ctx, &r),
        "worker_status" | "worker_start" | "worker_stop" | "worker_restart" => worker_op(ctx, &r),
        _ => Err(error("unsupported operation")),
    });
    let mut response = match result {
        Ok(data) => json!({"schema":1,"id":id,"ok":true,"data":data}),
        Err(e) => {
            let uncertain = r.op == "start_whales"
                || load(&p, JOURNAL_MAX)
                    .ok()
                    .is_some_and(|v| v.get("terminal_stage").is_some());
            let mut v = fail(
                id,
                if uncertain {
                    if r.op == "start_whales" {
                        "uncertain_external"
                    } else {
                        "uncertain_terminal"
                    }
                } else {
                    "operation"
                },
                &e.to_string(),
            );
            if uncertain {
                v["uncertain"] = json!(true)
            }
            v
        }
    };
    if serde_json::to_vec(&response)
        .map(|b| b.len() + 1 > RESPONSE_MAX)
        .unwrap_or(true)
    {
        response = fail(
            id,
            "response_limit",
            "serialized summary exceeds reply budget; reduce report count or request one report",
        );
    }
    record = match load(&p, JOURNAL_MAX) {
        Ok(v) => v,
        Err(_) => record,
    };
    record["state"] = json!("completed");
    record["response"] = response.clone();
    record["completed_unix"] = json!(now());
    if let Err(e) = atomic(&p, &record) {
        return json!({"schema":1,"id":id,"ok":false,"uncertain":true,"error":{"code":"receipt_write","detail":e.to_string()}});
    }
    response
}
fn frame(s: &mut UnixStream, max: usize) -> Result<Vec<u8>> {
    let mut out = Vec::new();
    BufReader::new(s)
        .take(max as u64 + 1)
        .read_until(b'\n', &mut out)?;
    if out.len() > max {
        return Err(error("frame exceeds protocol size limit"));
    }
    if out.last() != Some(&b'\n') {
        return Err(error("one newline-terminated JSON frame required"));
    }
    Ok(out)
}
struct SocketFile {
    path: PathBuf,
    ino: u64,
}
impl Drop for SocketFile {
    fn drop(&mut self) {
        if fs::symlink_metadata(&self.path)
            .ok()
            .is_some_and(|m| m.ino() == self.ino && m.file_type().is_socket())
        {
            let _ = fs::remove_file(&self.path);
        }
    }
}
trait DolphinQueue {
    fn status(&self, c: &Value) -> Result<Value>;
    fn next(&self, c: &Value) -> Result<Option<Value>>;
    fn finish(&self, c: &Value, id: &str, report: &Value) -> Result<Value>;
}
struct ActualDolphin;
impl DolphinQueue for ActualDolphin {
    fn status(&self, c: &Value) -> Result<Value> {
        crate::dolphin::status(c)
    }
    fn next(&self, c: &Value) -> Result<Option<Value>> {
        crate::dolphin::next(c)
    }
    fn finish(&self, c: &Value, id: &str, r: &Value) -> Result<Value> {
        crate::dolphin::finish(c, id, r)
    }
}
fn pending_consult(ctx: &Context, d: &Dolphin) -> Result<bool> {
    for p in journal_files(ctx)? {
        let v = load(&p, JOURNAL_MAX)?;
        if v["op"] != "consult_opus" || v["dolphin"].as_str().unwrap_or(&ctx.dolphins[0].name) != d.name {
            continue;
        }
        let pending = v["state"] == "reserved"
            || v["response"]["data"]["accepted"] == true
            || v["response"]["uncertain"] == true;
        if pending {
            let id = v["id"].as_str().ok_or("pending consult lacks id")?;
            if !ctx.report(id).exists() {
                return Ok(true);
            }
        }
    }
    Ok(false)
}
fn auto_idle(
    ctx: &Context,
    quiet: Duration,
    manual_waiting: bool,
    q: &impl DolphinQueue,
) -> Result<Option<Value>> {
    if ctx.raw["dolphin_auto"] != true || quiet < Duration::from_secs(60) || manual_waiting {
        return Ok(None);
    }
    ctx.pins()?;
    let d = &ctx.dolphins[0];
    if pending_consult(ctx, d)? {
        return Ok(None);
    }
    let (view, display) = capture_on(ctx, d)?;
    if !terminal_idle_on(d, &view, &display) {
        return Ok(None);
    }
    let status = q.status(&ctx.raw)?;
    let pending = status["pending"]
        .as_array()
        .ok_or("Dolphin status lacks pending array")?;
    let mut finished = None;
    for item in pending {
        let id = item["request_id"]
            .as_str()
            .ok_or("Dolphin pending item lacks id")?;
        if !valid_id(id) {
            return Err(error("invalid Dolphin pending id"));
        }
        let path = ctx.report(id);
        if !path.exists() {
            return Ok(None);
        }
        let m = fs::symlink_metadata(&path)?;
        if !m.is_file() || m.uid() != unsafe { libc::geteuid() } {
            return Err(error("Dolphin report is not an owned regular file"));
        }
        let bytes = read_limited(&path, 128000)?;
        let report = serde_json::from_slice(&bytes)
            .unwrap_or_else(|_| json!({"critique":String::from_utf8_lossy(&bytes)}));
        finished = Some(q.finish(&ctx.raw, id, &report)?);
    }
    let Some(job) = q.next(&ctx.raw)? else {
        return Ok(finished.map(|v| json!({"finished":trim_value(&v,4),"next_reserved":false})));
    };
    let id = job["request_id"]
        .as_str()
        .ok_or("Dolphin job lacks request id")?;
    let task = job["task"].as_str().ok_or("Dolphin job lacks task")?;
    let n = job["reserved_selected_ids"]
        .as_u64()
        .ok_or("Dolphin job lacks selected count")?;
    let budget = job["reserved_budget"]
        .as_u64()
        .ok_or("Dolphin job lacks budget")?;
    let ppm = job["fraction_ppm"]
        .as_u64()
        .ok_or("Dolphin job lacks fraction")?;
    if job["review_only"] != true
        || job["training_allowed"] != false
        || job["priority"] != "low"
        || ppm > 10000
        || budget == 0
        || budget as u128 > (n as u128 * ppm as u128) / 1_000_000
    {
        return Err(error(
            "Dolphin reservation violates review-only one-percent quota; inspect reservation",
        ));
    }
    let r = json!({"schema":1,"id":id,"op":"consult_opus","task":task});
    if serde_json::to_vec(&r)?.len() + 1 > REQUEST_MAX {
        return Err(error(
            "Dolphin request exceeds frame budget; reservation retained",
        ));
    }
    let result = dispatch(ctx, &r);
    Ok(Some(
        json!({"request_id":id,"dispatch":trim_value(&result,4),"finished":finished.map(|v|trim_value(&v,4)),"review_only":true,"automatic_resubmit":false}),
    ))
}

/// Explicit worker control; the worker name must be configured. Effects advance in ticks.
fn worker_op(ctx: &Context, r: &Request) -> Result<Value> {
    let mut w = ctx.workers.lock().map_err(|_| error("worker supervisor poisoned"))?;
    if w.is_empty() {
        return Err(error("no workers are configured"));
    }
    let name = r.worker.as_deref();
    match r.op.as_str() {
        "worker_status" => w.status(name, now()),
        "worker_start" => w.start(name.ok_or("worker required")?, now()),
        "worker_stop" => w.stop_with(name.ok_or("worker required")?, now(), r.immediate == Some(true)),
        _ => w.restart(name.ok_or("worker required")?, now()),
    }
}
/// Nested operation for the prompt handler: same validation, no journal entry of its own (the
/// enclosing ask is journaled). Admits observation (status, read_reports), explicit worker
/// control and the consult queue (consult_enqueue, consult_state); never legacy role markers,
/// a direct consult_opus or Whale operations.
fn in_process(ctx: &Context, q: &Value) -> std::result::Result<Value, String> {
    let r: Request = legacy_request(serde_json::from_value(q.clone()).map_err(|e| e.to_string())?);
    validate(&r).map_err(|e| e.to_string())?;
    let data = match r.op.as_str() {
        "status" => status(ctx, r.role.as_deref()),
        "read_reports" => known_reports(ctx, &r),
        "worker_status" | "worker_start" | "worker_stop" | "worker_restart" => worker_op(ctx, &r),
        "consult_enqueue" => consult_enqueue(ctx, &r),
        "consult_state" => consult_state(ctx, r.dolphin.as_deref()),
        "dolphin_list" => dolphin_list(ctx),
        "flow_status" => flow_status(ctx, r.group.as_deref()),
        "attention" => attention(ctx),
        _ => {
            return Err(
                "in-process prompt calls admit only status, read_reports, worker operations, flow, attention and the consult queue"
                    .into(),
            )
        }
    }
    .map_err(|e| e.to_string())?;
    Ok(json!({"schema":1,"id":r.id,"ok":true,"data":data}))
}
fn ask_op(ctx: &Context, r: &Request) -> Result<Value> {
    // Crab decides what to do: it executes what it understands and refuses (with a review
    // candidate) what it does not. The caller supplies only text and priority; an urgent ask
    // queues its consultation as urgent (queue order only, never preemption).
    let prompt = r.prompt.as_deref().expect("validated prompt");
    let urgent = r.priority.as_deref() == Some("urgent");
    let nested = |q: &Value| {
        let mut q = q.clone();
        if q["op"] == "consult_enqueue" && q.get("priority").is_none() && urgent {
            q["priority"] = json!("urgent");
        }
        in_process(ctx, &q)
    };
    let answer = ctx.prompt.ask(prompt, &r.id, true, &nested)?;
    if !answer["failure"].is_null() {
        if let Some(c) = &ctx.council {
            if let Ok(c) = c.lock() {
                let excerpt: String = prompt.chars().take(200).collect();
                let _ = flow::append(&c.refusals_path(), &json!({"at_unix":now(),"request_id":r.id,"excerpt":excerpt,"stage":answer["failure"]["stage"]}));
            }
        }
        if let Some(t) = ctx.c.consult_triggers.as_ref().filter(|t| t.refusals) {
            let candidates: Vec<Value> = answer["followups"]
                .as_array()
                .map(|a| a.iter().map(|f| f["candidate_id"].clone()).collect())
                .unwrap_or_default();
            let task = consult_queue::refusal_task(prompt, &r.id, &answer["failure"], &json!(candidates), t.max_task_bytes);
            let trigger = json!({"kind":"refusal","request_id":r.id,"stage":answer["failure"]["stage"],"review_candidates":candidates});
            if let Err(e) = ctx.consults.trigger(t, &format!("refusal:{}", r.id), trigger, &task, now()) {
                eprintln!("consult trigger not queued: {e}");
            }
        }
    }
    Ok(answer)
}
fn consult_enqueue(ctx: &Context, r: &Request) -> Result<Value> {
    let task = r.task.as_deref().ok_or("task required")?;
    let d = ctx.dolphin(r.dolphin.as_deref())?;
    ctx.consults.enqueue(&d.name, task, r.priority.as_deref() == Some("urgent"), "octopus", None, now())
}
/// A Dolphin's terminal as Crab sees it: idle, busy, or unavailable (with the reason).
fn consultant_terminal_state(ctx: &Context, d: &Dolphin) -> Value {
    match capture_on(ctx, d) {
        Ok((view, display)) if terminal_idle_on(d, &view, &display) => json!("idle"),
        Ok(_) => json!("busy"),
        Err(e) => json!(format!("unavailable: {}", e.to_string().chars().take(200).collect::<String>())),
    }
}
/// One Dolphin's queue, pending consultation and latest answer, for "Is the Opus Dolphin busy?"
/// and "What did the Copilot Dolphin answer?".
fn consult_state(ctx: &Context, name: Option<&str>) -> Result<Value> {
    let d = ctx.dolphin(name)?;
    let entries: Vec<Value> = ctx.consults.entries()?.into_iter().filter(|e| e["dolphin"] == d.name.as_str()).collect();
    let waiting: Vec<Value> = ctx
        .consults
        .waiting_for(&d.name)?
        .iter()
        .map(|e| json!({"id":e["id"],"priority":e["priority"],"origin":e["origin"],"created_unix":e["created_unix"],
            "task_excerpt":e["task"].as_str().unwrap_or("").chars().take(200).collect::<String>()}))
        .collect();
    let mut delivered: Vec<&Value> = entries.iter().filter(|e| e["state"] == "sent" || e["state"] == "answered").collect();
    delivered.sort_by_key(|e| e["sent_unix"].as_u64().unwrap_or(0));
    let latest = match delivered.last() {
        Some(e) => {
            let id = e["id"].as_str().ok_or("entry lacks id")?;
            let mut v = report(ctx, id)?;
            v["origin"] = e["origin"].clone();
            v["task_excerpt"] = json!(e["task"].as_str().unwrap_or("").chars().take(200).collect::<String>());
            v
        }
        None => Value::Null,
    };
    let pending = pending_consult(ctx, d)?;
    let terminal = consultant_terminal_state(ctx, d);
    let busy = pending || terminal != "idle";
    Ok(json!({"dolphin":d.name,"busy":busy,"terminal":terminal,"pending_consultation":pending,"waiting":waiting,
        "failed":entries.iter().filter(|e| e["state"]=="dispatch_failed").map(|e| e["id"].clone()).collect::<Vec<_>>(),
        "latest":latest,"advice_records":ctx.consults.advice_records()?.len(),
        "standing":"the consultant's answers are evidence, never authority or an action"}))
}
/// Every configured Dolphin with its worker state and terminal state, for "Which Dolphins are
/// running?" and "List the Dolphins.".
fn dolphin_list(ctx: &Context) -> Result<Value> {
    let workers = ctx.workers.lock().map_err(|_| error("worker supervisor poisoned"))?.summary();
    let mut out = Vec::new();
    for d in &ctx.dolphins {
        out.push(json!({"dolphin":d.name,"session":d.session,"worker":d.worker,
            "worker_state":d.worker.as_ref().map(|w| workers[w].clone()),
            "terminal":consultant_terminal_state(ctx, d),"pending_consultation":pending_consult(ctx, d)?,
            "waiting":ctx.consults.waiting_for(&d.name)?.len(),"pinned":d.pin.is_some()}));
    }
    Ok(json!({"dolphins":out,"count":out.len(),"health_inferred":false}))
}
/// "Do the Fish have work?", "Is any worker clogged?", "Show the Fish queues.": fresh counts
/// with the latched conditions, for one animal kind or all.
fn flow_status(ctx: &Context, group: Option<&str>) -> Result<Value> {
    let f = ctx.flow.as_ref().ok_or("no flow probes are configured")?;
    let f = f.lock().map_err(|_| error("flow poisoned"))?;
    if let Some(g) = group {
        if !f.kinds().iter().any(|k| k == g) {
            return Err(error(format!("no flow probe watches {g:?}; configured kinds: {}", f.kinds().join(", "))));
        }
    }
    let mut v = f.status(group, now());
    v["capacity"] = f.capacity(now());
    Ok(v)
}
/// "What needs my attention?"
fn attention(ctx: &Context) -> Result<Value> {
    let mut v = ctx.attention.view(now());
    if let Some(c) = &ctx.council {
        v["council"] = c.lock().map_err(|_| error("council poisoned"))?.view();
    }
    Ok(v)
}
/// Crab's view of its Dolphins for the council.
struct CouncilPod<'a>(&'a Context);
impl council::Pod for CouncilPod<'_> {
    fn availability(&self, name: &str) -> council::Availability {
        use council::Availability::*;
        let ctx = self.0;
        let Ok(d) = ctx.dolphin(Some(name)) else {
            return NotRunning("not configured".into());
        };
        if let Some(w) = &d.worker {
            let state = ctx.workers.lock().ok().and_then(|x| x.observe(w)).map(|x| x.0).unwrap_or_default();
            if state != "running" {
                return NotRunning(format!("worker {w} is {state}"));
            }
        }
        match (pending_consult(ctx, d), ctx.consults.waiting_for(&d.name)) {
            (Ok(false), Ok(w)) if w.is_empty() => {}
            (Ok(true), _) => return Busy("a consultation is pending".into()),
            (Ok(false), Ok(w)) => return Busy(format!("{} consultation(s) waiting", w.len())),
            (Err(e), _) | (_, Err(e)) => return Busy(e.to_string()),
        }
        match consultant_terminal_state(ctx, d) {
            t if t == "idle" => Idle,
            t if t == "busy" => Busy("terminal busy".into()),
            t => NotRunning(t.as_str().unwrap_or("unavailable").to_string()),
        }
    }
    fn enqueue(&self, dolphin: &str, task: &str, trigger: Value, now: u64) -> Result<String> {
        let v = self.0.consults.enqueue(dolphin, task, false, "crab", Some(trigger), now)?;
        Ok(v["id"].as_str().ok_or("queue returned no id")?.to_string())
    }
    fn answer(&self, id: &str) -> Result<Option<council::Answer>> {
        let Some(e) = self.0.consults.entries()?.into_iter().find(|e| e["id"] == id) else {
            return Ok(Some(council::Answer::Failed("consultation entry is missing".into())));
        };
        match e["state"].as_str() {
            Some("answered") => {
                let r = report(self.0, id)?;
                Ok(Some(council::Answer::Text(r["text"].as_str().unwrap_or("").to_string())))
            }
            Some("dispatch_failed") => Ok(Some(council::Answer::Failed(trim_value(&e["dispatch_failed_detail"], 2).to_string()))),
            _ => Ok(None),
        }
    }
}
fn tail_jsonl(p: &Path, n: usize) -> Vec<Value> {
    let Ok(b) = read_limited(p, 1 << 20).or_else(|_| -> Result<Vec<u8>> {
        let mut f = File::open(p)?;
        let len = f.metadata()?.len();
        std::io::Seek::seek(&mut f, std::io::SeekFrom::Start(len.saturating_sub(64 * 1024)))?;
        let mut b = Vec::new();
        f.read_to_end(&mut b)?;
        Ok(b)
    }) else {
        return vec![];
    };
    let all: Vec<Value> = String::from_utf8_lossy(&b).lines().filter_map(|l| serde_json::from_str(l).ok()).collect();
    all[all.len().saturating_sub(n)..].to_vec()
}
/// One supervision step of the flow probes (when due) and the council: evaluate, act within
/// bounds, journal, and put what Crab may not fix itself into the Octopus's attention queue.
fn supervision_tick(ctx: &Context, t: u64) -> Value {
    let mut out = json!({});
    if let Some(f) = &ctx.flow {
        let Ok(mut f) = f.lock() else { return json!({"error":"flow poisoned"}) };
        if f.due(t) {
            let events = match ctx.workers.lock() {
                Ok(mut w) => f.tick(t, &mut *w),
                Err(_) => Err(error("worker supervisor poisoned")),
            };
            match events {
                Ok(events) => {
                    for e in &events {
                        let action = e["result"]["action"].as_str().unwrap_or("");
                        if matches!(action, "report" | "hold") || !e["result"]["error"].is_null() {
                            let what = e["result"]["detail"].as_str().or(e["result"]["error"].as_str()).unwrap_or(action);
                            // Facts, not times: probe, action, conditions, stack counts; digits in
                            // free text are masked (ages and seconds move on every report).
                            let stacks: Vec<Value> = e["values"]["stacks"].as_array().into_iter().flatten().map(|s| json!({"stack":s["stack"],"value":s["value"]})).collect();
                            let facts = json!({"kind":"flow","probe":e["probe"],"action":action,"anomaly":e["result"]["anomaly"],"stack":e["result"]["stack"],
                                "conditions":e["conditions"],"over_high_water":e["values"]["over_high_water"],"stacks":stacks,"detail":council::mask_digits(what)});
                            let _ = ctx.attention.add_facts(t, "flow", &format!("{}: {what}", e["probe"].as_str().unwrap_or("")), e.clone(), &facts);
                        }
                    }
                    out["flow_events"] = json!(events.len());
                }
                Err(e) => out["flow_error"] = json!(e.to_string()),
            }
        }
    }
    if let (Some(c), Some(f)) = (&ctx.council, &ctx.flow) {
        let Ok(mut c) = c.lock() else { return json!({"error":"council poisoned"}) };
        let (probes, kinds) = match f.lock() {
            Ok(f) => (f.config.probes.iter().map(|p| p.name.clone()).collect::<Vec<_>>(), f.kinds()),
            Err(_) => return json!({"error":"flow poisoned"}),
        };
        let refusals = c.refusals_path();
        let facts = || {
            let (status, capacity, actions, state) = match f.lock() {
                Ok(f) => (f.status(None, t), f.capacity(t), f.recent(8), f.stable_facts()),
                Err(_) => (Value::Null, Value::Null, vec![], Value::Null),
            };
            let workers = ctx.workers.lock().map(|w| w.summary()).unwrap_or(Value::Null);
            json!({"flow":{"summary":status["summary"],"lines":status["lines"]},"state":state,"capacity":capacity,"actions":actions,"workers":workers,"refusals":tail_jsonl(&refusals, 5)})
        };
        let period = c.config.period_seconds;
        let mut exec = |probe: &str, action: &str| -> std::result::Result<Value, String> {
            let mut f = f.lock().map_err(|_| "flow poisoned".to_string())?;
            let mut w = ctx.workers.lock().map_err(|_| "worker supervisor poisoned".to_string())?;
            let v = f.council_apply(probe, action, t, period, &mut *w)?;
            let _ = flow::append(&f.journal_path(), &json!({"at_unix":t,"probe":probe,"origin":"council","result":v}));
            Ok(v)
        };
        out["council"] = match c.tick(t, &CouncilPod(ctx), &ctx.attention, &probes, &kinds, &facts, &mut exec) {
            Ok(v) => v,
            Err(e) => json!({"error":e.to_string(),"automatic_retry":false}),
        };
    }
    out
}
/// One non-blocking consult-queue step: fire configured triggers, attach answers, and deliver
/// the next waiting consultation when the consultant is idle and nothing is pending.
fn consult_tick(ctx: &Context) -> Result<Value> {
    let t_now = now();
    if let Some(t) = ctx.c.consult_triggers.as_ref().filter(|t| t.worker_failures) {
        let view = ctx.workers.lock().map_err(|_| error("worker supervisor poisoned"))?.status(None, t_now)?;
        for w in view["workers"].as_array().into_iter().flatten() {
            if let Some((key, task)) = consult_queue::worker_trigger(w) {
                let trigger = json!({"kind":"worker_failure","worker":w["worker"],"state":w["state"]});
                ctx.consults.trigger(t, &key, trigger, &task, t_now)?;
            }
        }
        if let Some((key, task)) = consult_queue::gpu_trigger(&view["gpu_check"]) {
            ctx.consults.trigger(t, &key, json!({"kind":"gpu_check_failed"}), &task, t_now)?;
        }
    }
    for e in ctx.consults.entries()?.iter().filter(|e| e["state"] == "sent") {
        let id = e["id"].as_str().ok_or("entry lacks id")?;
        let path = ctx.report(id);
        if path.exists() {
            if e["origin"] == "crab" {
                ctx.consults.record_advice(e, &path, t_now)?;
            } else {
                ctx.consults.mark(id, "answered", json!({"report":path}), t_now)?;
            }
        }
    }
    // Each Dolphin has its own terminal: deliver at most one consultation per Dolphin per step.
    let mut steps = serde_json::Map::new();
    let mut total = 0;
    for d in &ctx.dolphins {
        let waiting = ctx.consults.waiting_for(&d.name)?;
        let Some(next) = waiting.first() else {
            continue;
        };
        total += waiting.len();
        let step = if pending_consult(ctx, d)? {
            json!({"waiting":waiting.len(),"blocked":"an earlier consultation is pending"})
        } else {
            let terminal = consultant_terminal_state(ctx, d);
            if terminal != "idle" {
                json!({"waiting":waiting.len(),"blocked":terminal})
            } else {
                let id = next["id"].as_str().ok_or("entry lacks id")?;
                let r = json!({"schema":1,"id":id,"op":"consult_opus","task":next["task"],"dolphin":d.name});
                let result = dispatch(ctx, &r);
                if result["ok"] == true {
                    ctx.consults.mark(id, "sent", trim_value(&result["data"], 3), t_now)?;
                    total -= 1;
                } else {
                    // Fail fast: a failed delivery stays visible and is not retried blindly.
                    ctx.consults.mark(id, "dispatch_failed", trim_value(&result, 3), t_now)?;
                    total -= 1;
                }
                json!({"delivered":id,"ok":result["ok"]})
            }
        };
        steps.insert(d.name.clone(), step);
    }
    Ok(json!({"waiting":total,"dolphins":steps}))
}
/// Validate a Crab configuration without serving, booting workers or touching terminals: the
/// same checks as a start (it creates the private run-root directories if they are missing).
pub fn check(config: &Value) -> Result<Value> {
    let ctx = Context::new_checked(config)?;
    Ok(json!({"valid":true,"workers":ctx.workers.lock().map_err(|_| error("worker supervisor poisoned"))?.summary(),
        "dolphins":ctx.dolphins.iter().map(|d| json!({"dolphin":d.name,"session":d.session,"worker":d.worker,"pinned":d.pin.is_some(),"legacy_terminal":d.terminal.is_none()})).collect::<Vec<_>>(),
        "consult_triggers":ctx.c.consult_triggers.as_ref().map(|t| json!({"refusals":t.refusals,"worker_failures":t.worker_failures,"dolphin":t.dolphin,"max_per_hour":t.max_per_hour})),
        "flow":ctx.c.flow.as_ref().map(|f| json!({"interval_seconds":f.interval_seconds,"probes":f.probes.iter().map(|p| json!({"probe":p.name,"kind":p.group})).collect::<Vec<_>>(),
            "pools":f.pools.iter().map(|p| json!({"pool":p.name,"kind":p.kind,"members":p.members.iter().map(|m| &m.name).collect::<Vec<_>>(),"jobs":p.jobs.iter().map(|j| &j.name).collect::<Vec<_>>(),"lease_seconds":p.lease_seconds})).collect::<Vec<_>>()})),
        "council":ctx.c.council.as_ref().map(|k| json!({"dolphins":k.dolphins,"period_seconds":k.period_seconds,"min_spacing_seconds":k.min_spacing(),"quorum":k.quorum(),"member_timeout_seconds":k.timeout()}))}))
}
/// Read the configured flow probes once without serving, acting or changing their memory: the
/// same answer as "Show the queues." (it creates the private run-root directories if missing).
pub fn flow_observe(config: &Value) -> Result<Value> {
    let ctx = Context::new_checked(config)?;
    flow_status(&ctx, None)
}
pub fn serve(config: &Value) -> Result<()> {
    serve_with_roles(config, Arc::new(NoExternalRoles))
}
/// Embed explicitly supplied specialized handlers without a reverse Cargo dependency.
pub fn serve_with_roles(config: &Value, external: Arc<dyn ExternalRoleHandler>) -> Result<()> {
    serve_full(config, external, Arc::new(NoPromptHandler))
}
/// Crab with installed prompt understanding: the single Rust deputy answering arbitrary text.
pub fn serve_full(
    config: &Value,
    external: Arc<dyn ExternalRoleHandler>,
    prompt: Arc<dyn PromptHandler>,
) -> Result<()> {
    let mut ctx = Context::with_roles(config, external)?;
    ctx.prompt = prompt;
    let _lease = lock(&ctx.c.run_root.join("angel.lock"))?;
    // Only the lease holder adopts or starts workers, so two Crabs never double-start one.
    ctx.workers
        .lock()
        .map_err(|_| error("worker supervisor poisoned"))?
        .boot(now());
    if ctx.c.socket.exists() {
        let m = fs::symlink_metadata(&ctx.c.socket)?;
        if !m.file_type().is_socket() || m.uid() != unsafe { libc::geteuid() } {
            return Err(error("refusing foreign/non-socket endpoint"));
        }
        if UnixStream::connect(&ctx.c.socket).is_ok() {
            return Err(error("socket already has a listener"));
        }
        fs::remove_file(&ctx.c.socket)?;
    }
    let listener = UnixListener::bind(&ctx.c.socket)?;
    fs::set_permissions(&ctx.c.socket, fs::Permissions::from_mode(0o600))?;
    let _socket = SocketFile {
        path: ctx.c.socket.clone(),
        ino: fs::symlink_metadata(&ctx.c.socket)?.ino(),
    };
    listener.set_nonblocking(true)?;
    let mut count = 0;
    let mut heartbeat: Option<Instant> = None;
    let mut last_manual = Instant::now();
    let mut auto_poll = Instant::now();
    let mut consult_poll = Instant::now();
    let mut arrival = 0u64;
    let mut waiting: Vec<(bool, u64, UnixStream, Result<Value>)> = Vec::new();
    loop {
        if ctx.c.run_root.join("STOP").exists() {
            break;
        }
        // Non-blocking supervision step: reap, observe, escalate stops, restart after backoff.
        if let Ok(mut w) = ctx.workers.lock() {
            w.tick(now());
        }
        if heartbeat.is_none_or(|at| at.elapsed() >= Duration::from_secs(10)) {
            atomic(
                &ctx.c.run_root.join("status.json"),
                &json!({"display_name":"Crab","state":"listening","pid":std::process::id(),"socket":ctx.c.socket,"heartbeat_unix":now(),"attached_only":(legacy_key(&ctx.raw, "whale_config").is_null() && ctx.swarm.is_none())}),
            )?;
            heartbeat = Some(Instant::now());
        }
        // Drain waiting connections, then answer one: urgent asks first, arrival order within a
        // class. A request being answered is never interrupted.
        loop {
            match listener.accept() {
                Ok((mut stream, _)) => {
                    stream.set_nonblocking(false)?;
                    stream.set_read_timeout(Some(Duration::from_secs(10)))?;
                    stream.set_write_timeout(Some(Duration::from_secs(10)))?;
                    let parsed = frame(&mut stream, ASK_FRAME_MAX)
                        .and_then(|b| Ok(serde_json::from_slice::<Value>(&b)?))
                        .and_then(|v| {
                            let small = v["op"] != "ask";
                            if small && serde_json::to_vec(&v)?.len() > REQUEST_MAX {
                                return Err(error("frame exceeds protocol size limit"));
                            }
                            Ok(v)
                        });
                    let urgent = parsed
                        .as_ref()
                        .is_ok_and(|v| v["op"] == "ask" && v["priority"] == "urgent");
                    waiting.push((!urgent, arrival, stream, parsed));
                    arrival += 1;
                }
                Err(e) if e.kind() == std::io::ErrorKind::WouldBlock => break,
                Err(e) => return Err(e.into()),
            }
        }
        if !waiting.is_empty() {
            waiting.sort_by_key(|w| (w.0, w.1));
            let (_, _, mut stream, parsed) = waiting.remove(0);
            {
                let response = match parsed {
                    Ok(v) => dispatch(&ctx, &v),
                    Err(e) => fail("", "frame", &e.to_string()),
                };
                let mut bytes = serde_json::to_vec(&response)?;
                if bytes.len() + 1 > RESPONSE_MAX {
                    bytes = serde_json::to_vec(&fail(
                        response["id"].as_str().unwrap_or(""),
                        "response_limit",
                        "summary exceeds response budget",
                    ))?;
                }
                bytes.push(b'\n');
                let _ = stream.write_all(&bytes);
                last_manual = Instant::now();
                count += 1;
                if ctx.c.serve_requests > 0 && count >= ctx.c.serve_requests {
                    break;
                }
            }
        } else {
            {
                let mut consults_waiting = false;
                if consult_poll.elapsed() >= Duration::from_secs(10) {
                    consult_poll = Instant::now();
                    let event = match consult_tick(&ctx) {
                        Ok(v) => v,
                        Err(e) => json!({"error":e.to_string(),"automatic_retry":false}),
                    };
                    consults_waiting = event["waiting"].as_u64().is_some_and(|n| n > 0);
                    atomic(&ctx.c.run_root.join("consult-status.json"), &json!({"event":trim_value(&event,4),"heartbeat_unix":now()}))?;
                    let step = supervision_tick(&ctx, now());
                    atomic(&ctx.c.run_root.join("flow-status.json"), &json!({"event":trim_value(&step,4),"heartbeat_unix":now()}))?;
                }
                if !consults_waiting
                    && ctx.raw["dolphin_auto"] == true
                    && last_manual.elapsed() >= Duration::from_secs(60)
                    && auto_poll.elapsed() >= Duration::from_secs(10)
                {
                    auto_poll = Instant::now();
                    let event = match auto_idle(&ctx, last_manual.elapsed(), false, &ActualDolphin)
                    {
                        Ok(v) => json!({"event":v,"heartbeat_unix":now(),"review_only":true}),
                        Err(e) => {
                            json!({"error":e.to_string(),"heartbeat_unix":now(),"automatic_retry":false})
                        }
                    };
                    atomic(
                        &ctx.c.run_root.join("dolphin-status.json"),
                        &trim_value(&event, 5),
                    )?;
                }
                thread::sleep(Duration::from_millis(100))
            }
        }
    }
    atomic(
        &ctx.c.run_root.join("status.json"),
        &json!({"display_name":"Crab","state":"stopped","pid":std::process::id(),"attached_only":(legacy_key(&ctx.raw, "whale_config").is_null() && ctx.swarm.is_none()),"heartbeat_unix":now()}),
    )?;
    Ok(())
}
/// One bounded request/reply exchange. Reuse an ID only to obtain its cached receipt.
pub fn call(socket: &Path, request: &Value) -> Result<Value> {
    exchange(socket, request, REQUEST_MAX, Duration::from_secs(30))
}
/// Send arbitrary text with an optional priority (`urgent`/`normal`); Crab decides the rest.
/// Waits up to an hour: the request may queue behind earlier work, which is never interrupted.
pub fn ask(socket: &Path, id: &str, prompt: &str, urgent: bool) -> Result<Value> {
    let q = json!({"schema":1,"id":id,"op":"ask","priority":if urgent {"urgent"} else {"normal"},"prompt":prompt});
    exchange(socket, &q, ASK_FRAME_MAX, Duration::from_secs(3600))
}
fn exchange(socket: &Path, request: &Value, max: usize, wait: Duration) -> Result<Value> {
    absolute(socket)?;
    let mut bytes = serde_json::to_vec(request)?;
    if bytes.len() + 1 > max {
        return Err(error("request exceeds the protocol size limit"));
    }
    bytes.push(b'\n');
    let mut stream = UnixStream::connect(socket)?;
    stream.set_read_timeout(Some(wait))?;
    stream.set_write_timeout(Some(Duration::from_secs(60)))?;
    stream.write_all(&bytes)?;
    let response: Value = serde_json::from_slice(&frame(&mut stream, RESPONSE_MAX)?)?;
    if response["schema"] != 1 || response["id"] != request["id"] {
        return Err(error("response schema or request id mismatch"));
    }
    Ok(response)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::BTreeMap;
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
            "peta-crab-{}-{}",
            std::process::id(),
            NEXT.fetch_add(1, Ordering::Relaxed)
        ));
        fs::create_dir(&dir).unwrap();
        let fish = dir.join("fish");
        let eel = dir.join("eel");
        fs::create_dir_all(fish.join("annotation")).unwrap();
        fs::create_dir_all(eel.join("trainer")).unwrap();
        atomic(
            &dir.join("fish.json"),
            &json!({"run_root":fish,"workers":3}),
        )
        .unwrap();
        atomic(&dir.join("eel.json"), &json!({"run_root":eel})).unwrap();
        atomic(
            &eel.join("trainer/status.json"),
            &json!({"state":"waiting","heartbeat_unix":now()}),
        )
        .unwrap();
        let pane = dir.join("pane");
        fs::write(&pane, "Review done\n octopus-chief-review\n$\n").unwrap();
        let fake = dir.join("fake-tmux");
        fs::write(&fake,format!("#!/bin/sh\ncase \"$1\" in\n display-message) printf 'claude|0\\n';;\n capture-pane) cat '{}';;\n load-buffer|paste-buffer|send-keys) printf '%s\\n' \"$1\" >> '{}'; if [ \"$1\" = send-keys ] && [ -e '{}' ]; then exit 1; fi;;\n *) exit 9;;\nesac\n",pane.display(),dir.join("calls").display(),dir.join("fail-enter").display())).unwrap();
        fs::set_permissions(&fake, fs::Permissions::from_mode(0o700)).unwrap();
        let run = dir.join("crab");
        let c = json!({"socket":run.join("control.sock"),"run_root":run,"fish_config":dir.join("fish.json"),"eel_config":dir.join("eel.json"),"opus_session":SESSION,"tmux_bin":fake});
        Fixture { dir, c }
    }
    fn request(id: &str, op: &str) -> Value {
        json!({"schema":1,"id":id,"op":op})
    }
    #[test]
    fn read_only_status_attachments_preserve_legacy_controls_and_loaded_catalog() {
        let mut f = fixture();
        let old = Context::new(&f.c).unwrap();
        assert!(status(&old, None)
            .unwrap()
            .get("status_attachments")
            .is_none());
        let attached = f.dir.join("native-status.json");
        let bytes = serde_json::to_vec(&json!({"kind":"native_worker","config_sha256":"a".repeat(64),"heartbeat_unix":now(),"state":"active","counts":{"validated_targets":3},"prompt":"excluded"})).unwrap();
        fs::write(&attached, &bytes).unwrap();
        f.c["status_attachments"] = json!([{"id":"native-0","path":attached,"expected_kind":"native_worker","expected_config_sha256":"a".repeat(64)}]);
        let ctx = Context::new(&f.c).unwrap();
        let got = status(&ctx, Some("fish")).unwrap();
        assert!(got.get("fish").is_some());
        assert_eq!(
            got["status_attachments"]["control_scope"],
            "read_only_not_covered_by_role_controls"
        );
        assert_eq!(
            got["status_attachments"]["observations"][0]["value"]["counts"]["validated_targets"],
            3
        );
        f.c["status_attachments"][0]["expected_kind"] = json!("other");
        assert_eq!(
            status(&ctx, None).unwrap()["status_attachments"]["observations"][0]["status"],
            "observed"
        );
        assert_eq!(
            status(&Context::new(&f.c).unwrap(), None).unwrap()["status_attachments"]
                ["observations"][0]["status"],
            "identity_mismatch"
        );
        let mut pause = request("legacy-pause-with-attachment", "pause");
        pause["role"] = json!("fish");
        assert_eq!(dispatch(&ctx, &pause)["ok"], true);
        assert!(f.dir.join("fish/PAUSE").is_file());
        assert!(!f.dir.join("PAUSE").exists());
        assert_eq!(fs::read(attached).unwrap(), bytes);
    }
    #[test]
    fn invalid_attachment_catalog_fails_before_endpoint_creation() {
        let mut f = fixture();
        f.c["status_attachments"] = json!([{"id":"bad","path":"relative","expected_kind":"worker","expected_config_sha256":"a".repeat(64)}]);
        assert!(Context::new(&f.c).is_err());
        assert!(!f.dir.join("crab").exists());
    }
    #[test]
    fn idle_detection_refuses_busy_modal_and_wrong_process() {
        let good = "Review done\n octopus-chief-review\n$\n";
        assert!(idle(good, "claude|0\n"));
        for s in [
            format!("Esc to interrupt\n{good}"),
            format!("How is Claude doing this session?\n{good}"),
            "Working\n".into(),
        ] {
            assert!(!idle(&s, "claude|0"));
        }
        assert!(!idle(good, "bash|0"));
    }
    #[test]
    fn marker_dedup_conflicts_and_foreign_failure_guards() {
        let f = fixture();
        let ctx = Context::new(&f.c).unwrap();
        let mut p = request("pause1", "pause");
        p["role"] = json!("fish");
        let first = dispatch(&ctx, &p);
        assert_eq!(first["ok"], true);
        assert_eq!(dispatch(&ctx, &p), first);
        let mut conflict = p.clone();
        conflict["op"] = json!("stop");
        assert_eq!(dispatch(&ctx, &conflict)["error"]["code"], "id_conflict");
        assert!(!f.dir.join("fish/STOP").exists());
        let mut resume = request("resume1", "resume");
        resume["role"] = json!("fish");
        assert_eq!(dispatch(&ctx, &resume)["ok"], true);
        assert!(!f.dir.join("fish/PAUSE").exists());
        assert_eq!(dispatch(&ctx, &p), first);
        assert!(
            !f.dir.join("fish/PAUSE").exists(),
            "cached pause acknowledgment does not reapply an old action"
        );
        fs::write(f.dir.join("fish/PAUSE"), "operator\n").unwrap();
        resume["id"] = json!("resume2");
        assert_eq!(dispatch(&ctx, &resume)["ok"], false);
        assert_eq!(
            fs::read_to_string(f.dir.join("fish/PAUSE")).unwrap(),
            "operator\n"
        );
        fs::write(f.dir.join("eel/trainer/PAUSE"), "failure\n").unwrap();
        let mut ep = request("eel-pause", "pause");
        ep["role"] = json!("eel");
        assert_eq!(dispatch(&ctx, &ep)["ok"], true);
        ep["id"] = json!("eel-resume");
        ep["op"] = json!("resume");
        assert_eq!(dispatch(&ctx, &ep)["ok"], false);
        assert!(f.dir.join("eel/PAUSE").exists());
        fs::remove_file(f.dir.join("eel/trainer/PAUSE")).unwrap();
        fs::create_dir_all(f.dir.join("eel/eel")).unwrap();
        fs::write(f.dir.join("eel/eel/PAUSE"), "idle actor failure\n").unwrap();
        ep["id"] = json!("idle-eel-resume");
        assert_eq!(dispatch(&ctx, &ep)["ok"], false);
        fs::remove_file(f.dir.join("eel/eel/PAUSE")).unwrap();
        fs::write(f.dir.join("eel/eel/STOP"), "operator\n").unwrap();
        ep["id"] = json!("stopped-idle-eel-resume");
        assert_eq!(dispatch(&ctx, &ep)["ok"], false);
        atomic(
            &f.dir.join("eel/eel/status.json"),
            &json!({"state":"idle","heartbeat_unix":now()}),
        )
        .unwrap();
        assert_eq!(
            dispatch(&ctx, &request("idle-eel-observe", "status"))["data"]["eel"]["actor"]
                ["present"],
            true
        );
        fs::write(f.dir.join("fish/STOP"), "operator\n").unwrap();
        resume["id"] = json!("resume3");
        assert!(dispatch(&ctx, &resume)["error"]["detail"]
            .as_str()
            .unwrap()
            .contains("STOP"));
        let mut arbitrary = request("bad", "status");
        arbitrary["path"] = json!("/etc/passwd");
        assert_eq!(dispatch(&ctx, &arbitrary)["error"]["code"], "protocol");
    }
    #[test]
    fn reserved_consult_is_never_replayed() {
        let f = fixture();
        let ctx = Context::new(&f.c).unwrap();
        let mut r = request("uncertain", "consult_opus");
        r["task"] = json!("Review a bounded model.");
        atomic(&ctx.journal("uncertain"),&json!({"id":"uncertain","op":"consult_opus","payload_sha256":hash(&serde_json::to_vec(&r).unwrap()),"state":"reserved"})).unwrap();
        assert_eq!(dispatch(&ctx, &r)["uncertain"], true);
        r["id"] = json!("other");
        assert_eq!(dispatch(&ctx, &r)["ok"], false);
        assert!(!f.dir.join("calls").exists());
    }
    #[test]
    fn terminal_consult_is_exactly_once_and_waits_for_report() {
        let f = fixture();
        let ctx = Context::new(&f.c).unwrap();
        let mut r = request("review1", "consult_opus");
        r["task"] = json!("Review the admission boundary only.");
        let accepted = dispatch(&ctx, &r);
        assert_eq!(accepted["data"]["accepted"], true);
        assert_eq!(dispatch(&ctx, &r), accepted);
        assert_eq!(
            fs::read_to_string(f.dir.join("calls")).unwrap(),
            "load-buffer\npaste-buffer\nsend-keys\n"
        );
        let mut next = r.clone();
        next["id"] = json!("review2");
        assert_eq!(dispatch(&ctx, &next)["ok"], false);
        let mut poll = request("poll1", "opus_report");
        poll["target_id"] = json!("review1");
        assert_eq!(dispatch(&ctx, &poll)["data"]["present"], false);
        fs::write(ctx.report("review1"), "Finite review evidence.\n").unwrap();
        poll["id"] = json!("poll2");
        assert_eq!(dispatch(&ctx, &poll)["data"]["status"], "report_present");
        fs::write(
            f.dir.join("pane"),
            "Esc to interrupt\nReview done\noctopus-chief-review\n$\n",
        )
        .unwrap();
        next["id"] = json!("review3");
        assert_eq!(dispatch(&ctx, &next)["ok"], false);
        assert_eq!(
            fs::read_to_string(f.dir.join("calls"))
                .unwrap()
                .lines()
                .count(),
            3
        );
    }
    #[test]
    fn partial_terminal_failure_is_uncertain_and_not_retried() {
        let f = fixture();
        let ctx = Context::new(&f.c).unwrap();
        fs::write(f.dir.join("fail-enter"), "").unwrap();
        let mut r = request("partial", "consult_opus");
        r["task"] = json!("Review one boundary.");
        let got = dispatch(&ctx, &r);
        assert_eq!(got["uncertain"], true);
        assert_eq!(dispatch(&ctx, &r), got);
        assert_eq!(
            fs::read_to_string(f.dir.join("calls"))
                .unwrap()
                .lines()
                .count(),
            3
        );
    }
    struct EchoPrompt(std::sync::Mutex<Vec<String>>);
    impl PromptHandler for EchoPrompt {
        fn ask(
            &self,
            prompt: &str,
            id: &str,
            execute: bool,
            crab: &dyn Fn(&Value) -> std::result::Result<Value, String>,
        ) -> Result<Value> {
            self.0.lock().unwrap().push(id.to_owned());
            if id == "slow" {
                thread::sleep(Duration::from_millis(700));
            }
            let status = crab(&json!({"schema":1,"id":format!("{id}-s"),"op":"status"}))?;
            let control = crab(&json!({"schema":1,"id":format!("{id}-c"),"op":"stop","role":"fish"}));
            Ok(json!({"bytes":prompt.len(),"execute":execute,"status_ok":status["ok"],"control_refused":control.is_err()}))
        }
    }
    #[test]
    fn ask_takes_any_text_and_priority_without_preemption() {
        let mut f = fixture();
        f.c["serve_requests"] = json!(6);
        let cfg = f.c.clone();
        let socket = PathBuf::from(cfg["socket"].as_str().unwrap());
        let seen = Arc::new(EchoPrompt(std::sync::Mutex::new(vec![])));
        let handler = seen.clone();
        let server = thread::spawn(move || {
            serve_full(&cfg, Arc::new(NoExternalRoles), handler).map_err(|e| e.to_string())
        });
        for _ in 0..200 {
            if socket.exists() {
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }
        // Long arbitrary text (over the 16 KiB control limit) is accepted; Crab decides to run.
        let long = "Please look at everything. ".repeat(2000);
        let r = ask(&socket, "long", &long, false).unwrap();
        assert_eq!(r["ok"], true, "{r}");
        assert_eq!(r["data"]["bytes"], long.len());
        assert_eq!(r["data"]["execute"], true);
        assert_eq!(r["data"]["status_ok"], true);
        assert_eq!(r["data"]["control_refused"], true);
        // The caller cannot choose a mode; only text and priority exist.
        let mut bad = json!({"schema":1,"id":"mode","op":"ask","prompt":"x","mode":"plan"});
        assert_eq!(call(&socket, &bad).unwrap()["ok"], false);
        bad = json!({"schema":1,"id":"prio","op":"ask","prompt":"x","priority":"now"});
        assert_eq!(call(&socket, &bad).unwrap()["ok"], false);
        // While a slow ask runs, a normal then an urgent ask wait; urgent goes first.
        let s1 = socket.clone();
        let slow = thread::spawn(move || ask(&s1, "slow", "a", false).unwrap());
        thread::sleep(Duration::from_millis(250));
        let s2 = socket.clone();
        let normal = thread::spawn(move || ask(&s2, "normal", "b", false).unwrap());
        thread::sleep(Duration::from_millis(100));
        let s3 = socket.clone();
        let urgent = thread::spawn(move || ask(&s3, "urgent", "c", true).unwrap());
        for t in [slow, normal, urgent] {
            assert_eq!(t.join().unwrap()["ok"], true);
        }
        assert_eq!(*seen.0.lock().unwrap(), ["long", "slow", "urgent", "normal"]);
        assert!(server.join().unwrap().is_ok());
    }
    #[test]
    fn ask_without_installed_understanding_is_refused() {
        let f = fixture();
        let ctx = Context::new(&f.c).unwrap();
        fs::create_dir_all(ctx.c.run_root.join("requests")).unwrap();
        let v = dispatch(&ctx, &json!({"schema":1,"id":"a1","op":"ask","prompt":"Hello there."}));
        assert_eq!(v["ok"], false);
        assert!(v.to_string().contains("not installed"), "{v}");
    }
    #[test]
    fn socket_roundtrip_dedup_conflict_and_oversize() {
        let mut f = fixture();
        f.c["serve_requests"] = json!(4);
        let cfg = f.c.clone();
        let socket = PathBuf::from(cfg["socket"].as_str().unwrap());
        let thread = thread::spawn(move || serve(&cfg).map_err(|e| e.to_string()));
        for _ in 0..200 {
            if socket.exists() {
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }
        assert_eq!(fs::metadata(&socket).unwrap().mode() & 0o777, 0o600);
        let r = request("status1", "status");
        let got = call(&socket, &r).unwrap();
        assert_eq!(got["data"]["fish"]["workers"].as_array().unwrap().len(), 3);
        assert_eq!(call(&socket, &r).unwrap(), got);
        let mut conflict = r.clone();
        conflict["op"] = json!("read_reports");
        assert_eq!(
            call(&socket, &conflict).unwrap()["error"]["code"],
            "id_conflict"
        );
        let mut large = request("large", "status");
        large["padding"] = json!("x".repeat(REQUEST_MAX));
        assert!(call(&socket, &large).is_err());
        let mut raw = UnixStream::connect(&socket).unwrap();
        raw.set_read_timeout(Some(Duration::from_secs(3))).unwrap();
        let mut huge = vec![b'x'; REQUEST_MAX + 1];
        huge.push(b'\n');
        raw.write_all(&huge).unwrap();
        let reply: Value = serde_json::from_slice(&frame(&mut raw, RESPONSE_MAX).unwrap()).unwrap();
        assert_eq!(reply["error"]["code"], "frame");
        assert!(thread.join().unwrap().is_ok());
        assert!(!socket.exists());
    }
    #[test]
    fn report_projection_is_bounded_and_private_endpoint_refuses_clobber() {
        let f = fixture();
        let ctx = Context::new(&f.c).unwrap();
        let items: BTreeMap<_, _> = (0..5000)
            .map(|n| (format!("id{n}"), "x".repeat(70)))
            .collect();
        atomic(&f.dir.join("eel/trainer/trained.json"),&json!({"outcome":"installed","generation_sha256":"abc","trained_items":items,"candidate":{"las":85.46}})).unwrap();
        let got = dispatch(&ctx, &request("reports", "read_reports"));
        let bytes = serde_json::to_vec(&got).unwrap();
        assert!(bytes.len() < 4096);
        assert!(!String::from_utf8(bytes).unwrap().contains("trained_items"));
        fs::write(ctx.report("long"), "A".repeat(50000)).unwrap();
        let mut p = request("long-report", "opus_report");
        p["target_id"] = json!("long");
        let got = dispatch(&ctx, &p);
        assert_eq!(got["data"]["truncated"], true);
        assert_eq!(got["data"]["text"].as_str().unwrap().len(), REPORT_MAX);
        fs::write(&ctx.c.socket, "keep\n").unwrap();
        assert!(serve(&f.c).is_err());
        assert_eq!(fs::read_to_string(&ctx.c.socket).unwrap(), "keep\n");
    }
    #[test]
    fn escaped_report_budget_is_checked_before_journal_persistence() {
        let f = fixture();
        let ctx = Context::new(&f.c).unwrap();
        for n in 0..3 {
            let id = format!("control-report-{n}");
            atomic(&ctx.journal(&id),&json!({"id":id,"op":"consult_opus","state":"completed","created_unix":n,"payload_sha256":"fixture","response":{"ok":true,"data":{"accepted":true}}})).unwrap();
            fs::write(ctx.report(&id), vec![0u8; REPORT_MAX]).unwrap();
        }
        let mut r = request("budget", "read_reports");
        r["limit"] = json!(3);
        let response = dispatch(&ctx, &r);
        assert_eq!(response["error"]["code"], "response_limit");
        assert!(fs::metadata(ctx.journal("budget")).unwrap().len() < 4096);
        assert_eq!(dispatch(&ctx, &r), response);
        assert!(load(&ctx.journal("budget"), JOURNAL_MAX).is_ok());
        assert_eq!(
            dispatch(&ctx, &request("still-readable", "status"))["ok"],
            true
        );
    }
    #[test]
    fn client_rejects_response_id_and_schema_mismatch() {
        let f = fixture();
        let ctx = Context::new(&f.c).unwrap();
        let socket = ctx.c.socket.clone();
        let listener = UnixListener::bind(&socket).unwrap();
        let server = thread::spawn(move || {
            for n in 0..2 {
                let (mut stream, _) = listener.accept().unwrap();
                let _: Value =
                    serde_json::from_slice(&frame(&mut stream, REQUEST_MAX).unwrap()).unwrap();
                let response = if n == 0 {
                    json!({"schema":1,"id":"other","ok":true})
                } else {
                    json!({"schema":2,"id":"wanted","ok":true})
                };
                let mut b = serde_json::to_vec(&response).unwrap();
                b.push(b'\n');
                stream.write_all(&b).unwrap();
            }
        });
        for _ in 0..2 {
            assert!(call(&socket, &request("wanted", "status"))
                .unwrap_err()
                .to_string()
                .contains("mismatch"));
        }
        server.join().unwrap();
    }
    struct FakeQueue {
        events: std::cell::RefCell<Vec<String>>,
        pending: std::cell::RefCell<Value>,
        job: std::cell::RefCell<Option<Value>>,
    }
    impl FakeQueue {
        fn new(job: Option<Value>) -> Self {
            Self {
                events: std::cell::RefCell::new(Vec::new()),
                pending: std::cell::RefCell::new(json!({"pending":[]})),
                job: std::cell::RefCell::new(job),
            }
        }
    }
    impl DolphinQueue for FakeQueue {
        fn status(&self, _: &Value) -> Result<Value> {
            self.events.borrow_mut().push("status".into());
            Ok(self.pending.borrow().clone())
        }
        fn next(&self, _: &Value) -> Result<Option<Value>> {
            self.events.borrow_mut().push("next".into());
            let job = self.job.borrow_mut().take();
            if let Some(j) = &job {
                *self.pending.borrow_mut() = json!({"pending":[{"request_id":j["request_id"]}]});
            }
            Ok(job)
        }
        fn finish(&self, _: &Value, id: &str, _: &Value) -> Result<Value> {
            self.events.borrow_mut().push("finish".into());
            *self.pending.borrow_mut() = json!({"pending":[]});
            Ok(
                json!({"request_id":id,"state":"quarantined","review_only":true,"training_allowed":false}),
            )
        }
    }
    fn dolphin_job() -> Value {
        json!({"request_id":"dolphin-next","task":"Review this bounded sentence only and return JSON.","priority":"low","review_only":true,"training_allowed":false,"reserved_selected_ids":100,"reserved_budget":1,"fraction_ppm":10000})
    }
    #[test]
    fn idle_dolphin_autooff_quiet_manual_and_busy_gates() {
        let mut f = fixture();
        let q = FakeQueue::new(Some(dolphin_job()));
        let ctx = Context::new(&f.c).unwrap();
        assert!(auto_idle(&ctx, Duration::from_secs(61), false, &q)
            .unwrap()
            .is_none());
        assert!(q.events.borrow().is_empty());
        f.c["dolphin_auto"] = json!(true);
        let ctx = Context::new(&f.c).unwrap();
        assert!(auto_idle(&ctx, Duration::from_secs(59), false, &q)
            .unwrap()
            .is_none());
        assert!(auto_idle(&ctx, Duration::from_secs(61), true, &q)
            .unwrap()
            .is_none());
        fs::write(f.dir.join("pane"), "Esc to interrupt\n").unwrap();
        assert!(auto_idle(&ctx, Duration::from_secs(61), false, &q)
            .unwrap()
            .is_none());
        assert!(q.events.borrow().is_empty());
        assert!(!f.dir.join("calls").exists());
    }
    #[test]
    fn idle_dolphin_finishes_report_before_one_reservation_and_never_replays() {
        let mut f = fixture();
        f.c["dolphin_auto"] = json!(true);
        let ctx = Context::new(&f.c).unwrap();
        let q = FakeQueue::new(Some(dolphin_job()));
        *q.pending.borrow_mut() = json!({"pending":[{"request_id":"dolphin-old"}]});
        fs::write(
            ctx.report("dolphin-old"),
            "{\"critique\":\"retain uncertainty\"}",
        )
        .unwrap();
        let event = auto_idle(&ctx, Duration::from_secs(61), false, &q)
            .unwrap()
            .unwrap();
        assert_eq!(event["dispatch"]["data"]["accepted"], true);
        assert_eq!(*q.events.borrow(), vec!["status", "finish", "next"]);
        let prior = q.events.borrow().len();
        assert!(auto_idle(&ctx, Duration::from_secs(61), false, &q)
            .unwrap()
            .is_none());
        assert_eq!(q.events.borrow().len(), prior);
        assert_eq!(
            fs::read_to_string(f.dir.join("calls"))
                .unwrap()
                .lines()
                .count(),
            3
        );
    }
    #[test]
    fn idle_dolphin_hard_quota_adapter_refuses_dispatch_after_reservation() {
        let mut f = fixture();
        f.c["dolphin_auto"] = json!(true);
        let ctx = Context::new(&f.c).unwrap();
        let mut bad = dolphin_job();
        bad["fraction_ppm"] = json!(20000);
        let q = FakeQueue::new(Some(bad));
        assert!(auto_idle(&ctx, Duration::from_secs(61), false, &q)
            .unwrap_err()
            .to_string()
            .contains("quota"));
        assert!(!f.dir.join("calls").exists());
        let events = q.events.borrow().len();
        assert!(auto_idle(&ctx, Duration::from_secs(61), false, &q)
            .unwrap()
            .is_none());
        assert_eq!(q.events.borrow().len(), events + 1);
        assert_eq!(
            q.events
                .borrow()
                .iter()
                .filter(|x| x.as_str() == "next")
                .count(),
            1
        );
    }
    #[test]
    fn compact_journal_does_not_expand_pretty_nested_receipts() {
        let f = fixture();
        let ctx = Context::new(&f.c).unwrap();
        let input = json!(vec![vec![vec![vec![Value::Null; 8]; 8]; 8]; 7]);
        let response = json!({"schema":1,"id":"nested","ok":true,"data":{"fish":{"master":{"value":trim_value(&input,4)}}}});
        assert!(serde_json::to_vec(&response).unwrap().len() < RESPONSE_MAX);
        let r = request("nested", "status");
        let record = json!({"state":"completed","id":"nested","op":"status","payload_sha256":hash(&serde_json::to_vec(&r).unwrap()),"response":response});
        assert!(serde_json::to_vec_pretty(&record).unwrap().len() > JOURNAL_MAX);
        atomic(&ctx.journal("nested"), &record).unwrap();
        assert!(fs::metadata(ctx.journal("nested")).unwrap().len() < JOURNAL_MAX as u64);
        assert_eq!(dispatch(&ctx, &r), record["response"]);
    }
    #[test]
    fn whale_operations_require_server_configuration_and_no_request_arguments() {
        let f = fixture();
        let ctx = Context::new(&f.c).unwrap();
        let mut r = request("whale-arguments", "start_whales");
        r["role"] = json!("whales");
        assert_eq!(dispatch(&ctx, &r)["error"]["code"], "protocol");
        let absent = dispatch(&ctx, &request("whale-no-config", "whale_status"));
        assert_eq!(absent["ok"], false);
        // The names before the Whale rename reach the same gates.
        let legacy = dispatch(&ctx, &request("shark-no-config", "shark_status"));
        assert_eq!(legacy["ok"], false);
        assert_eq!(legacy["error"]["detail"], absent["error"]["detail"]);
        assert!(!f.dir.join("whales").exists());
        let mut r = request("whale-client-path", "start_whales");
        r["whale_config"] = json!("/tmp/client-selected.json");
        assert_eq!(dispatch(&ctx, &r)["error"]["code"], "protocol");
    }
    #[test]
    fn swarm_configuration_is_pinned_and_modes_are_exclusive() {
        let mut f = fixture();
        let path = f.dir.join("swarm.json");
        atomic(&path, &json!({"run_root":f.dir.join("swarm-run")})).unwrap();
        f.c["whale_swarm_config"] = json!(path);
        let ctx = Context::new(&f.c).unwrap();
        assert!(ctx.swarm.is_some());
        assert!(ctx.pins().is_ok());
        atomic(&path, &json!({"run_root":f.dir.join("changed-run")})).unwrap();
        assert!(ctx
            .pins()
            .unwrap_err()
            .to_string()
            .contains("swarm configuration changed"));
        f.c["shark_config"] = json!(f.dir.join("legacy.json"));
        assert!(Context::new(&f.c)
            .err()
            .unwrap()
            .to_string()
            .contains("either the Whale pool"));
    }
    #[test]
    fn swarm_root_alias_cannot_control_the_fish_root() {
        let mut f = fixture();
        let fish: Value =
            serde_json::from_slice(&fs::read(f.dir.join("fish.json")).unwrap()).unwrap();
        let fish_root = role_root(&fish).unwrap();
        fs::set_permissions(&fish_root, fs::Permissions::from_mode(0o700)).unwrap();
        private_dir(&fish_root).unwrap();
        fs::create_dir(fish_root.join("alias-child")).unwrap();
        let path = f.dir.join("aliased-swarm.json");
        atomic(&path, &json!({"run_root":fish_root.join("alias-child/..")})).unwrap();
        f.c["shark_swarm_config"] = json!(path);
        let error = Context::new(&f.c).err().unwrap().to_string();
        assert!(error.contains("physically separate"));
        assert!(!fish_root.join("PAUSE").exists());
        assert!(!fish_root.join("STOP").exists());
    }
    #[test]
    fn permissions_role_pins_and_single_service_lease() {
        let f = fixture();
        let ctx = Context::new(&f.c).unwrap();
        assert_eq!(fs::metadata(&ctx.c.run_root).unwrap().mode() & 0o777, 0o700);
        let _held = lock(&ctx.c.run_root.join("angel.lock")).unwrap();
        assert!(serve(&f.c).is_err());
        atomic(
            &f.dir.join("fish.json"),
            &json!({"run_root":f.dir.join("different")}),
        )
        .unwrap();
        assert!(
            dispatch(&ctx, &request("changed", "status"))["error"]["detail"]
                .as_str()
                .unwrap()
                .contains("configuration changed")
        );
        let mut c = f.c.clone();
        c["opus_session"] = json!("unrelated");
        assert!(Context::new(&c).is_err());
        fs::set_permissions(&ctx.c.run_root, fs::Permissions::from_mode(0o755)).unwrap();
        assert!(Context::new(&f.c).is_err());
    }
    #[test]
    fn external_operations_are_disabled_until_an_explicit_handler_is_installed() {
        let mut f = fixture();
        f.c["whale_config"] = json!(f.dir.join("external.json"));
        let denied = Context::new(&f.c).unwrap();
        assert!(whales_start(&denied)
            .unwrap_err()
            .to_string()
            .contains("not installed"));
        struct Fake;
        impl ExternalRoleHandler for Fake {
            fn status(&self, _: &Value, swarm: Option<&Value>) -> Result<Value> {
                Ok(json!({"fixture":true,"swarm":swarm.is_some()}))
            }
            fn start(&self, _: &Value, swarm: bool) -> Result<Value> {
                Ok(json!({"fixture":true,"swarm":swarm,"process_spawned":false}))
            }
            fn marker(&self, _: &Value, op: &str) -> Result<Value> {
                Ok(json!({"fixture":true,"op":op}))
            }
        }
        let admitted = Context::with_roles(&f.c, Arc::new(Fake)).unwrap();
        assert_eq!(whales_start(&admitted).unwrap()["fixture"], true);
        assert_eq!(whales_status(&admitted).unwrap()["swarm"], false);
        let mut request = request("handler-marker", "pause");
        request["role"] = json!("whales");
        let request: Request = serde_json::from_value(request).unwrap();
        assert_eq!(marker(&admitted, &request).unwrap()["op"], "pause");
        assert!(!f.dir.join("external.json").exists());
    }

    #[test]
    fn configured_model_agnostic_terminal_requires_exact_ownership_and_readiness() {
        let mut f = fixture();
        f.c["opus_session"] = json!("other-consultant");
        assert!(
            Context::new(&f.c).is_err(),
            "nonlegacy identity must declare trusted readiness"
        );
        f.c["consultant_terminal"] = json!({"provider":"other-cli","model":"custom-model","pane_command":"node","idle_marker":"READY","owner_marker":"fixture-owner"});
        let ctx = Context::new(&f.c).unwrap();
        assert!(terminal_idle(&ctx, "READY\n", "node|0"));
        assert!(!terminal_idle(&ctx, "Update available\nREADY\n", "node|0"));
        assert!(!terminal_idle(&ctx, "READY\n", "foreign|0"));
        let fake = f.dir.join("fake-tmux");
        fs::write(&fake,"#!/bin/sh\ncase \"$1\" in\n display-message) printf 'other-consultant|1|1|node|0|foreign-owner\\n';;\n *) exit 9;;\nesac\n").unwrap();
        assert!(capture(&ctx).unwrap_err().to_string().contains("ownership"));
        fs::write(&fake,"#!/bin/sh\ncase \"$1\" in\n display-message) test \"$4\" = '=other-consultant:' || exit 8; printf 'other-consultant|1|1|node|0|fixture-owner\\n';;\n capture-pane) test \"$4\" = '=other-consultant:' || exit 8; printf 'READY\\n';;\n *) exit 9;;\nesac\n").unwrap();
        let (pane, display) = capture(&ctx).unwrap();
        assert!(terminal_idle(&ctx, &pane, &display));
        assert!(!f.dir.join("calls").exists());
        f.c["consultant_terminal"]
            .as_object_mut()
            .unwrap()
            .remove("model");
        f.c["consultant_terminal"]
            .as_object_mut()
            .unwrap()
            .remove("provider");
        let defaults = Context::new(&f.c).unwrap();
        assert_eq!(
            defaults.c.consultant_terminal.as_ref().unwrap().model,
            "opus"
        );
        assert_eq!(
            defaults.c.consultant_terminal.as_ref().unwrap().provider,
            "claude-cli"
        );
        f.c["opus_session"] = json!("bad:target");
        assert!(Context::new(&f.c).is_err());
    }
    fn with_worker(f: &mut Fixture, autostart: bool) -> PathBuf {
        let script = f.dir.join("polite.sh");
        fs::write(&script, "#!/bin/sh\nwhile [ ! -e \"$1\" ] && [ -d \"${1%/*}\" ]; do sleep 0.05; done; exit 0\n").unwrap();
        fs::set_permissions(&script, fs::Permissions::from_mode(0o700)).unwrap();
        let marker = f.dir.join("EEL-STOP");
        f.c["workers"] = json!([{"name":"eel","argv":[script,marker],"cwd":f.dir,"log":f.dir.join("eel.log"),
            "autostart":autostart,"stop_marker":marker,"stop_grace_seconds":30}]);
        marker
    }
    fn settle(ctx: &Context, want: &str) {
        for _ in 0..250 {
            let mut w = ctx.workers.lock().unwrap();
            w.tick(now());
            if w.summary()["eel"] == want {
                return;
            }
            drop(w);
            thread::sleep(Duration::from_millis(20));
        }
        panic!("eel never reached {want}");
    }
    #[test]
    fn worker_operations_are_validated_journaled_and_limited_to_configured_names() {
        let mut f = fixture();
        let marker = with_worker(&mut f, false);
        let ctx = Context::new(&f.c).unwrap();
        let before = dispatch(&ctx, &json!({"schema":1,"id":"w0","op":"worker_start","worker":"eel"}));
        assert!(before.to_string().contains("not booted"), "{before}");
        ctx.workers.lock().unwrap().boot(now());
        let mut q = json!({"schema":1,"id":"w1","op":"worker_start","worker":"eel"});
        let started = dispatch(&ctx, &q);
        assert_eq!(started["ok"], true, "{started}");
        assert_eq!(started["data"]["state"], "running");
        // The same id replays the receipt; it never starts twice.
        assert_eq!(dispatch(&ctx, &q), started);
        q["worker"] = json!("whale");
        q["id"] = json!("w2");
        let unknown = dispatch(&ctx, &q);
        assert_eq!(unknown["error"]["code"], "operation");
        assert!(unknown.to_string().contains("unknown worker"));
        for (id, bad) in [
            ("p1", json!({"op":"worker_start"})),
            ("p2", json!({"op":"worker_stop","worker":"../eel"})),
            ("p3", json!({"op":"worker_start","worker":"eel","role":"eel"})),
            ("p4", json!({"op":"status","worker":"eel"})),
            ("p5", json!({"op":"worker_start","worker":"eel","argv":["/bin/sh"]})),
        ] {
            let mut r = bad.clone();
            r["schema"] = json!(1);
            r["id"] = json!(id);
            assert_eq!(dispatch(&ctx, &r)["error"]["code"], "protocol", "{bad}");
        }
        let all = dispatch(&ctx, &json!({"schema":1,"id":"w3","op":"worker_status"}));
        assert_eq!(all["data"]["workers"][0]["worker"], "eel");
        assert_eq!(status(&ctx, None).unwrap()["workers"]["eel"], "running");
        let stop = dispatch(&ctx, &json!({"schema":1,"id":"w4","op":"worker_stop","worker":"eel"}));
        assert_eq!(stop["data"]["state"], "stopping_marker", "{stop}");
        assert!(marker.is_file());
        settle(&ctx, "stopped");
        let restart = dispatch(&ctx, &json!({"schema":1,"id":"w5","op":"worker_restart","worker":"eel"}));
        assert_eq!(restart["data"]["state"], "running");
        assert!(!marker.exists());
        dispatch(&ctx, &json!({"schema":1,"id":"w6","op":"worker_stop","worker":"eel"}));
        settle(&ctx, "stopped");
        assert!(load(&ctx.journal("w4"), JOURNAL_MAX).unwrap()["state"] == "completed");
        // Without configured workers the operations are refused.
        let other = fixture();
        let plain = Context::new(&other.c).unwrap();
        let none = dispatch(&plain, &json!({"schema":1,"id":"n1","op":"worker_status"}));
        assert!(none.to_string().contains("no workers"), "{none}");
    }
    struct WorkerPrompt;
    impl PromptHandler for WorkerPrompt {
        fn ask(
            &self,
            _: &str,
            id: &str,
            _: bool,
            crab: &dyn Fn(&Value) -> std::result::Result<Value, String>,
        ) -> Result<Value> {
            let status = crab(&json!({"schema":1,"id":format!("{id}-s"),"op":"worker_status","worker":"eel"}))?;
            let stop = crab(&json!({"schema":1,"id":format!("{id}-t"),"op":"worker_stop","worker":"eel"}))?;
            let refused: Vec<bool> = [
                json!({"schema":1,"id":format!("{id}-o"),"op":"consult_opus","task":"review"}),
                json!({"schema":1,"id":format!("{id}-m"),"op":"stop","role":"fish"}),
                json!({"schema":1,"id":format!("{id}-k"),"op":"start_whales"}),
                json!({"schema":1,"id":format!("{id}-u"),"op":"worker_start","worker":"whale"}),
            ]
            .iter()
            .map(|q| crab(q).is_err())
            .collect();
            Ok(json!({"status_state":status["data"]["workers"][0]["state"],"stop_state":stop["data"]["state"],"refused":refused}))
        }
    }
    #[test]
    fn serve_autostarts_and_prompts_admit_worker_control_but_not_consult_or_markers() {
        let mut f = fixture();
        let marker = with_worker(&mut f, true);
        f.c["serve_requests"] = json!(1);
        let cfg = f.c.clone();
        let socket = PathBuf::from(cfg["socket"].as_str().unwrap());
        let server = thread::spawn(move || {
            serve_full(&cfg, Arc::new(NoExternalRoles), Arc::new(WorkerPrompt)).map_err(|e| e.to_string())
        });
        for _ in 0..200 {
            if socket.exists() {
                break;
            }
            thread::sleep(Duration::from_millis(10));
        }
        let r = ask(&socket, "eel-prompt", "Stop Eel.", false).unwrap();
        assert!(server.join().unwrap().is_ok());
        assert_eq!(r["ok"], true, "{r}");
        assert_eq!(r["data"]["status_state"], "running", "autostarted at boot");
        assert_eq!(r["data"]["stop_state"], "stopping_marker");
        assert_eq!(r["data"]["refused"], json!([true, true, true, true]));
        assert!(marker.is_file());
        assert!(!f.dir.join("fish/STOP").exists());
        let saved = load(&f.dir.join("crab/workers/eel.json"), 65536).unwrap();
        assert_eq!(saved["state"], "stopping_marker");
        assert_eq!(saved["stop_requested"], true);
        // The worker honours the marker by itself; wait so the fixture outlives it.
        let stat = format!("/proc/{}/stat", saved["pid"]);
        for _ in 0..250 {
            if fs::read_to_string(&stat).map_or(true, |s| s.contains(") Z ")) {
                break;
            }
            thread::sleep(Duration::from_millis(20));
        }
    }

    /// A fake prompt handler: urgent asks to consult are queued through the nested channel; any
    /// other prompt is refused with one review candidate.
    struct ConsultPrompts;
    impl PromptHandler for ConsultPrompts {
        fn ask(&self, prompt: &str, id: &str, _: bool, crab: &dyn Fn(&Value) -> std::result::Result<Value, String>) -> Result<Value> {
            if let Some(task) = prompt.strip_prefix("Ask the consultant: ") {
                let v = crab(&json!({"schema":1,"id":id,"op":"consult_enqueue","task":task})).map_err(error)?;
                return Ok(json!({"failure":null,"results":[v],"followups":[]}));
            }
            if crab(&json!({"schema":1,"id":id,"op":"consult_opus","task":"x"})).is_ok() {
                return Err(error("a nested direct consult must stay refused"));
            }
            Ok(json!({"failure":{"stage":"unsupported_capability","reason":"no capability"},"followups":[{"candidate_id":"cand-1"}]}))
        }
    }
    /// The Octopus and Crab consult a real (fake) consultant terminal on a private tmux server:
    /// urgent first, one at a time, only when idle; Crab's own consultation is a review record
    /// whose advice ("Start Eel.") is never executed.
    #[test]
    fn consult_queue_delivers_through_crab_and_advice_is_never_an_action() {
        let mut f = fixture();
        with_worker(&mut f, false);
        let sock = format!("mova-crab-consult-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed));
        struct Server(String);
        impl Drop for Server {
            fn drop(&mut self) {
                let _ = Command::new("/usr/bin/tmux").args(["-L", &self.0, "kill-server"]).output();
                let _ = fs::remove_file(crate::tmux_worker::tests::test_socket_path(&self.0));
            }
        }
        let _server = Server(sock.clone());
        let wrapper = f.dir.join("tmux-private");
        fs::write(&wrapper, format!("#!/bin/sh\nexec /usr/bin/tmux -L {sock} \"$@\"\n")).unwrap();
        fs::set_permissions(&wrapper, fs::Permissions::from_mode(0o700)).unwrap();
        // The consultant writes advice that names an action, prints READY and waits for the next task.
        let consultant = f.dir.join("consultant.sh");
        fs::write(&consultant, format!("#!/bin/sh\necho working\nwhile [ ! -e '{}' ]; do sleep 0.05; done\necho READY\nwhile IFS= read -r line; do\n case \"$line\" in *'report file only at '*) p=${{line#*report file only at }}; p=${{p%%. That exact*}}; printf 'Advice: Start Eel. Restart Eel now.\\n' > \"$p\"; echo \"$p\" >> \"$0.tasks\"; sleep 0.2; echo READY;; esac\ndone\n", f.dir.join("GO").display())).unwrap();
        fs::set_permissions(&consultant, fs::Permissions::from_mode(0o700)).unwrap();
        let ok = Command::new(&wrapper).args(["new-session", "-d", "-s", "fake-consultant", "-x", "200", "-y", "50", "/bin/sh"]).arg(&consultant).status().unwrap();
        assert!(ok.success());
        Command::new(&wrapper).args(["set-option", "-t", "=fake-consultant:", "@mova_crab_owner", "fixture-owner"]).status().unwrap();
        f.c["tmux_bin"] = json!(wrapper);
        f.c["dolphins"] = json!([{"name":"fake","session":"fake-consultant","terminal":{"pane_command":"sh","idle_marker":"READY","owner_marker":"fixture-owner"}}]);
        f.c["consult_triggers"] = json!({"refusals":true,"dolphin":"fake","max_per_hour":2,"max_task_bytes":2000});
        let mut ctx = Context::new(&f.c).unwrap();
        ctx.prompt = Arc::new(ConsultPrompts);
        ctx.workers.lock().unwrap().boot(now());
        let wait_idle = |ctx: &Context| {
            for _ in 0..250 {
                if consultant_terminal_state(ctx, &ctx.dolphins[0]) == "idle" {
                    return;
                }
                thread::sleep(Duration::from_millis(20));
            }
            panic!("consultant never idle: {}", consultant_terminal_state(ctx, &ctx.dolphins[0]));
        };
        thread::sleep(Duration::from_millis(300));
        // Crab refuses a request it does not understand and queues its own consultation.
        let v = dispatch(&ctx, &json!({"schema":1,"id":"r1","op":"ask","prompt":"Fly Eel to the moon."}));
        assert_eq!(v["ok"], true, "{v}");
        // The Octopus asks, urgently, through the English channel; queued ahead of Crab's entry.
        let v = dispatch(&ctx, &json!({"schema":1,"id":"o1","op":"ask","priority":"urgent","prompt":"Ask the consultant: Review the plan."}));
        assert_eq!(v["ok"], true, "{v}");
        let waiting = ctx.consults.waiting().unwrap();
        assert_eq!(waiting.len(), 2);
        assert_eq!((waiting[0]["origin"].as_str(), waiting[0]["priority"].as_str()), (Some("octopus"), Some("urgent")));
        assert_eq!(waiting[1]["trigger"]["review_candidates"], json!(["cand-1"]));
        assert_eq!(waiting[1]["trigger"]["kind"], "refusal");
        // Busy terminal (its last line is not the idle marker): nothing is delivered.
        let t = consult_tick(&ctx).unwrap();
        assert_eq!(t["dolphins"]["fake"]["blocked"], "busy", "{t}");
        assert!(!PathBuf::from(format!("{}.tasks", consultant.display())).exists());
        fs::write(f.dir.join("GO"), "").unwrap();
        let tasks = PathBuf::from(format!("{}.tasks", consultant.display()));
        let deliver = |ctx: &Context, n: usize| {
            for _ in 0..100 {
                let _ = consult_tick(ctx);
                if fs::read_to_string(&tasks).map(|t| t.lines().count()).unwrap_or(0) >= n {
                    return;
                }
                thread::sleep(Duration::from_millis(100));
            }
            panic!("consultation {n} never delivered: {}", consult_tick(ctx).unwrap());
        };
        wait_idle(&ctx);
        deliver(&ctx, 1);
        let first = fs::read_to_string(&tasks).unwrap();
        assert!(first.contains(&hash(waiting[0]["id"].as_str().unwrap().as_bytes())), "urgent Octopus entry first");
        wait_idle(&ctx);
        deliver(&ctx, 2);
        for _ in 0..50 {
            consult_tick(&ctx).unwrap();
            if !ctx.consults.advice_records().unwrap().is_empty() {
                break;
            }
            thread::sleep(Duration::from_millis(100));
        }
        let advice = ctx.consults.advice_records().unwrap();
        assert_eq!(advice.len(), 1, "one advice record, for Crab's own consultation");
        assert_eq!(advice[0]["actions_taken"], json!([]));
        assert_eq!(advice[0]["automatic_execution"], false);
        assert_eq!(advice[0]["review_candidates"], json!(["cand-1"]));
        // The advice said "Start Eel." and "Restart Eel now."; Eel was never started.
        for _ in 0..5 {
            consult_tick(&ctx).unwrap();
            ctx.workers.lock().unwrap().tick(now());
        }
        assert_eq!(ctx.workers.lock().unwrap().summary()["eel"], "stopped", "advice is never an action");
        let ops: Vec<String> = journal_files(&ctx).unwrap().iter().map(|p| load(p, JOURNAL_MAX).unwrap()["op"].as_str().unwrap().to_string()).collect();
        assert!(!ops.iter().any(|o| o.starts_with("worker_")), "{ops:?}");
        assert_eq!(ops.iter().filter(|o| *o == "ask").count(), 2, "{ops:?}");
        let state = consult_state(&ctx, Some("fake")).unwrap();
        assert!(state["latest"]["text"].as_str().unwrap().contains("Start Eel"), "{state}");
        assert_eq!(state["waiting"], json!([]));
        // Crab's own channel caps itself: the same refusal is consulted once.
        dispatch(&ctx, &json!({"schema":1,"id":"r2","op":"ask","prompt":"Fly Eel to the moon."}));
        dispatch(&ctx, &json!({"schema":1,"id":"r3","op":"ask","prompt":"Fly Eel to the moon."}));
        assert_eq!(ctx.consults.crab_recent(now()).unwrap(), 2, "hourly bound of two");
        // Direct consult_opus stays outside nested prompt calls; queue ops are admitted.
        assert!(in_process(&ctx, &json!({"schema":1,"id":"n1","op":"consult_opus","task":"x"})).is_err());
        assert!(in_process(&ctx, &json!({"schema":1,"id":"n2","op":"consult_state"})).is_ok());
    }

    /// Four named Dolphins on one private tmux server: every consultation reaches only the
    /// session of the Dolphin it names; an unpinned Copilot Dolphin is refused in plain English.
    #[test]
    fn a_pod_of_named_dolphins_routes_each_consultation_to_its_own_session() {
        let mut f = fixture();
        let sock = format!("mova-crab-pod-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed));
        struct Server(String);
        impl Drop for Server {
            fn drop(&mut self) {
                let _ = Command::new("/usr/bin/tmux").args(["-L", &self.0, "kill-server"]).output();
                let _ = fs::remove_file(crate::tmux_worker::tests::test_socket_path(&self.0));
            }
        }
        let _server = Server(sock.clone());
        let wrapper = f.dir.join("tmux-private");
        fs::write(&wrapper, format!("#!/bin/sh\nexec /usr/bin/tmux -L {sock} \"$@\"\n")).unwrap();
        fs::set_permissions(&wrapper, fs::Permissions::from_mode(0o700)).unwrap();
        let consultant = f.dir.join("consultant.sh");
        // argv[1] names the Dolphin; it records which task paths reached it.
        fs::write(&consultant, format!("#!/bin/sh\necho READY\nwhile IFS= read -r line; do\n case \"$line\" in *'report file only at '*) p=${{line#*report file only at }}; p=${{p%%. That exact*}}; printf 'answer from %s\\n' \"$1\" > \"$p\"; echo \"$p\" >> '{}/'\"$1\".tasks; sleep 0.1; echo READY;; esac\ndone\n", f.dir.display())).unwrap();
        fs::set_permissions(&consultant, fs::Permissions::from_mode(0o700)).unwrap();
        let names = ["opus", "copilot", "documenter", "sol"];
        let mut dolphins = vec![];
        for n in names {
            let session = format!("dolphin-{n}");
            assert!(Command::new(&wrapper).args(["new-session", "-d", "-s", &session, "-x", "200", "-y", "50", "/bin/sh"]).arg(&consultant).arg(n).status().unwrap().success());
            Command::new(&wrapper).args(["set-option", "-t", &format!("={session}:"), "@mova_crab_owner", "fixture-owner"]).status().unwrap();
            dolphins.push(json!({"name":n,"session":session,"terminal":{"pane_command":"sh","idle_marker":"READY","owner_marker":"fixture-owner"}}));
        }
        // The Copilot Dolphin is pinned to a version its executable does not print.
        let exe = f.dir.join("copilot");
        fs::write(&exe, "#!/bin/sh\necho 'Fake Copilot 1.0.99.'\n").unwrap();
        fs::set_permissions(&exe, fs::Permissions::from_mode(0o700)).unwrap();
        dolphins[1]["pin"] = json!({"version_cmd":[exe, "--version"],"version":"Fake Copilot 1.0.98."});
        f.c["tmux_bin"] = json!(wrapper);
        f.c["dolphins"] = json!(dolphins);
        let ctx = Context::new(&f.c).unwrap();
        thread::sleep(Duration::from_millis(300));
        for n in names {
            let v = dispatch(&ctx, &json!({"schema":1,"id":format!("q-{n}"),"op":"consult_enqueue","dolphin":n,"task":format!("Task for {n}.")}));
            assert_eq!(v["ok"], true, "{v}");
        }
        let unknown = dispatch(&ctx, &json!({"schema":1,"id":"q-x","op":"consult_enqueue","dolphin":"whale","task":"x"}));
        assert_eq!(unknown["ok"], false, "an unknown Dolphin is refused");
        let reached = |n: &str| fs::read_to_string(f.dir.join(format!("{n}.tasks"))).map(|t| t.lines().count()).unwrap_or(0);
        for _ in 0..100 {
            consult_tick(&ctx).unwrap();
            if ["opus", "documenter", "sol"].iter().all(|n| reached(n) == 1) {
                break;
            }
            thread::sleep(Duration::from_millis(100));
        }
        for n in ["opus", "documenter", "sol"] {
            assert_eq!(reached(n), 1, "{n} received exactly its own consultation");
            let state = consult_state(&ctx, Some(n)).unwrap();
            for _ in 0..50 {
                if consult_state(&ctx, Some(n)).unwrap()["latest"]["present"] == true {
                    break;
                }
                thread::sleep(Duration::from_millis(50));
            }
            let state = if state["latest"]["present"] == true { state } else { consult_state(&ctx, Some(n)).unwrap() };
            assert!(state["latest"]["text"].as_str().unwrap_or("").contains(&format!("answer from {n}")), "{state}");
            assert!(state["latest"]["task_excerpt"].as_str().unwrap().contains(&format!("Task for {n}")));
        }
        assert_eq!(reached("copilot"), 0, "an unpinned Copilot Dolphin receives nothing");
        let failed = consult_state(&ctx, Some("copilot")).unwrap();
        assert_eq!(failed["failed"].as_array().unwrap().len(), 1);
        let entry = ctx.consults.entries().unwrap().into_iter().find(|e| e["dolphin"] == "copilot").unwrap();
        assert_eq!(entry["state"], "dispatch_failed");
        assert!(entry.to_string().contains("copilot is not pinned: version"), "{entry}");
        let list = dolphin_list(&ctx).unwrap();
        assert_eq!(list["count"], 4);
        let listed: Vec<&str> = list["dolphins"].as_array().unwrap().iter().map(|d| d["dolphin"].as_str().unwrap()).collect();
        assert_eq!(listed, names);
        assert_eq!(list["dolphins"][1]["pinned"], true);
        // Names stay unique; a trigger must name a configured Dolphin.
        let mut bad = f.c.clone();
        bad["dolphins"][1]["name"] = json!("opus");
        assert!(Context::new(&bad).is_err());
        let mut bad = f.c.clone();
        bad["consult_triggers"] = json!({"refusals":true,"dolphin":"whale"});
        assert!(Context::new(&bad).is_err());
        bad["consult_triggers"] = json!({"refusals":true});
        assert!(Context::new(&bad).is_err());
    }

    /// The public standard pod (three Dolphins, two prompts) is a valid Crab configuration and
    /// every holder command line parses; only the Opus consultant starts by default.
    #[test]
    fn standard_dolphin_pod_example_is_valid() {
        let mut f = fixture();
        let pod: Value = serde_json::from_str(include_str!("../dolphins/crab-pod.example.json")).unwrap();
        for k in ["workers", "dolphins", "consult_triggers"] {
            f.c[k] = pod[k].clone();
        }
        let v = check(&f.c).unwrap();
        assert_eq!(v["dolphins"].as_array().unwrap().len(), 3);
        for w in pod["workers"].as_array().unwrap() {
            let argv: Vec<String> = serde_json::from_value(w["argv"].clone()).unwrap();
            let spec = crate::tmux_worker::parse_args(&argv[1..]).unwrap();
            assert!(spec.lock.is_some(), "singleton by default");
            assert_eq!(w["autostart"], w["name"] == "dolphin-opus", "{}", w["name"]);
            assert_eq!(w["restart"], false);
            if let Some(p) = spec.prompt_file {
                let name = p.file_name().unwrap().to_str().unwrap().to_string();
                let text = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join("dolphins").join(&name)).unwrap();
                crate::tmux_worker::render_prompt(&text, &spec.vars).unwrap();
            }
        }
        assert!(pod["dolphins"][1]["pin"].is_object(), "the Copilot Dolphin is pinned");
        assert_eq!(pod["consult_triggers"]["refusals"], false, "Crab-initiated consultations are off publicly");
    }

    #[test]
    fn idle_window_admits_tui_footers_but_not_a_longer_marker() {
        let profile = |w: Option<usize>| TerminalProfile { provider: "codex-cli".into(), model: "gpt".into(), pane_command: "node".into(),
            idle_marker: "DOLPHIN READY".into(), prompt_marker: None, required_text: None, owner_marker: "o".into(), idle_window: w };
        let d = |w| Dolphin { name: "sol".into(), session: "s".into(), terminal: Some(profile(w)), worker: None, pin: None };
        let codex = "old work\n\n• DOLPHIN READY\n\n─ Worked for 2m 10s ─\n› Ask Codex to do anything\n  gpt-6.1-sol medium · 80% left\n  ? for shortcuts\n";
        assert!(terminal_idle_on(&d(Some(6)), codex, "node|0"));
        assert!(!terminal_idle_on(&d(None), codex, "node|0"), "window 1: the marker is not the last line");
        assert!(!terminal_idle_on(&d(Some(1)), codex, "node|0"));
        assert!(!terminal_idle_on(&d(Some(6)), &codex.replace("DOLPHIN READY", "DOLPHIN READY NOW"), "node|0"));
        // Busy phrases count only near the bottom: old scrollback does not hold it busy.
        let scrolled = format!("esc to interrupt\n{}{codex}", "line\n".repeat(20));
        assert!(terminal_idle_on(&d(Some(6)), &scrolled, "node|0"));
        assert!(!terminal_idle_on(&d(Some(6)), &format!("{codex}esc to interrupt\n"), "node|0"));
        // Claude Code: bullet marker then footer lines; default exact last line still works.
        assert!(terminal_idle_on(&d(Some(4)), "● DOLPHIN READY\n\n> \n  ? for shortcuts\n", "node|0"));
        assert!(terminal_idle_on(&d(None), "work\nDOLPHIN READY\n", "node|0"));
        // The bound is validated.
        let mut f = fixture();
        f.c["dolphins"] = json!([{"name":"sol","session":"dolphin-sol","terminal":{"pane_command":"node","idle_marker":"DOLPHIN READY","owner_marker":"o","idle_window":33}}]);
        assert!(Context::new(&f.c).is_err());
        f.c["dolphins"][0]["terminal"]["idle_window"] = json!(6);
        assert!(Context::new(&f.c).is_ok());
    }
    #[test]
    fn flow_probes_act_journal_and_answer_through_dispatch() {
        let mut f = fixture();
        let w = f.dir.join("work");
        fs::create_dir_all(w.join("ready")).unwrap();
        let polite = crate::workers::tests::polite(&f.dir);
        f.c["workers"] = json!([{"name":"materializer","argv":[polite,w.join("PRODUCER-STOP"),w.join("starts")],"cwd":f.dir,"log":f.dir.join("m.log"),"stop_marker":w.join("PRODUCER-STOP")}]);
        f.c["flow"] = json!({"interval_seconds":10,"probes":[{"name":"fish-ready","kind":"fish",
            "input":{"source":{"kind":"dir_entries","path":w.join("ready"),"suffix":".json"},"low_water":1},
            "stacks":[{"name":"ready","source":{"kind":"dir_entries","path":w.join("ready")},"high_water":3}],
            "producer":{"worker":"materializer","pause_file":w.join("PAUSE")},"hysteresis":1}]});
        // A council without flow, or naming an unknown Dolphin, is refused.
        f.c["council"] = json!({"dolphins":["consultant","sol"]});
        assert!(Context::new(&f.c).is_err());
        f.c.as_object_mut().unwrap().remove("council");
        let ctx = Context::new(&f.c).unwrap();
        ctx.workers.lock().unwrap().boot(now());
        assert_eq!(ctx.workers.lock().unwrap().observe("materializer").unwrap().0, "stopped");
        // Starved: the stopped (not explicitly stopped) producer is started.
        supervision_tick(&ctx, 1000);
        assert_eq!(ctx.workers.lock().unwrap().observe("materializer").unwrap().0, "running");
        // Clogged: the producer is paused through its file; nothing is killed.
        for i in 0..5 {
            fs::write(w.join(format!("ready/p{i}.json")), "{}").unwrap();
        }
        supervision_tick(&ctx, 1010);
        assert!(w.join("PAUSE").is_file());
        assert_eq!(ctx.workers.lock().unwrap().observe("materializer").unwrap().0, "running");
        let v = dispatch(&ctx, &json!({"schema":1,"id":"fl1","op":"flow_status","group":"fish"}));
        assert_eq!(v["ok"], true, "{v}");
        assert_eq!(v["data"]["probes"][0]["conditions"]["clogged"], true);
        assert!(v["data"]["summary"].as_str().unwrap().contains("1 of 1 have work; 1 are clogged"), "{v}");
        assert!(v["data"]["capacity"]["kinds"]["fish"].is_object());
        let bad = dispatch(&ctx, &json!({"schema":1,"id":"fl2","op":"flow_status","group":"whale"}));
        assert_eq!(bad["ok"], false);
        assert_eq!(dispatch(&ctx, &json!({"schema":1,"id":"fl3","op":"status","group":"fish"}))["ok"], false, "group belongs only to flow_status");
        let journal = fs::read_to_string(f.c["run_root"].as_str().map(PathBuf::from).unwrap().join("flow/journal.jsonl")).unwrap();
        assert!(journal.contains("start_producer") && journal.contains("pause_producer"));
        // Nested prompt calls admit flow_status and attention.
        assert!(in_process(&ctx, &json!({"schema":1,"id":"n1","op":"flow_status"})).is_ok());
        assert!(in_process(&ctx, &json!({"schema":1,"id":"n2","op":"attention"})).is_ok());
        // A producer that was explicitly stopped is not restarted: reported to the attention queue.
        for i in 0..5 {
            let _ = fs::remove_file(w.join(format!("ready/p{i}.json")));
        }
        ctx.workers.lock().unwrap().stop_with("materializer", now(), true).unwrap();
        for _ in 0..200 {
            ctx.workers.lock().unwrap().tick(now());
            if ctx.workers.lock().unwrap().observe("materializer").unwrap().0 == "stopped" {
                break;
            }
            thread::sleep(Duration::from_millis(20));
        }
        supervision_tick(&ctx, 1020);
        supervision_tick(&ctx, 1030);
        assert!(!w.join("PAUSE").exists(), "Crab lifted its own pause after the clog cleared");
        assert_eq!(ctx.workers.lock().unwrap().observe("materializer").unwrap().0, "stopped");
        let a = dispatch(&ctx, &json!({"schema":1,"id":"at1","op":"attention"}));
        assert!(a["data"]["summary"].as_str().unwrap().contains("explicitly stopped"), "{a}");
    }
}
