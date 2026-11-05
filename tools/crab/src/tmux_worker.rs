//! A small holder that lets Crab supervise a program living in a detached tmux session, such
//! as a 🐬 Dolphin role (an LLM terminal). Crab starts the holder as an ordinary configured
//! worker. The holder first takes a host-wide exclusive lock for its role (a singleton unless
//! `--no-singleton` is given explicitly); if another holder owns the lock it reports that one
//! and exits. Otherwise it creates the session, or adopts an existing session of the same
//! name whose only pane runs the configured command, stays alive while the session and its
//! command live, and on a stop request sends SIGTERM to the command, waits a bounded grace and
//! removes the session. It exits when the session or command ends, so Crab observes the exit.
//!
//! An optional pin binds the program before every start or adoption and again every minute while
//! it runs: a pinned file must resolve to a regular file with the expected SHA-256, a version
//! command must print the expected first line, and an authentication file must name non-empty
//! values for the given keys (presence only; no value is read out, and no model is called). A
//! failed pin refuses the start in plain English, or ends a running session.
//!
//! The holder never types into the terminal: it uses only `has-session`, `new-session`,
//! `set-option`, `list-panes` and `kill-session`, never `send-keys`, `load-buffer` or
//! `paste-buffer`. A role prompt reaches the program only as a command-line argument at
//! creation; later tasks reach a terminal only through Crab's separate Dolphin protocol, which
//! refuses to overwrite an active task.
use crate::Result;
use std::{
    collections::BTreeMap,
    fs::{self, File, OpenOptions},
    io::{Read, Seek, Write},
    os::unix::{fs::OpenOptionsExt, io::AsRawFd},
    path::{Path, PathBuf},
    process::Command,
    sync::atomic::{AtomicBool, Ordering},
    time::{Duration, Instant},
};

/// Crab sends SIGKILL ten seconds after SIGTERM; the holder must finish well before.
pub const GRACE_MAX_SECONDS: u64 = 8;
/// Same budget as a rendered Fish request.
pub const PROMPT_MAX: usize = 100_000;
const PROC_SCAN_MAX: usize = 1 << 16;

pub const USAGE: &str = "usage: crab-tmux-worker (--lock ABS_FILE | --no-singleton) [--tmux ABS] \
[--socket NAME] [--grace SECONDS] [--poll-ms MS] [--owner-marker TEXT] [--prompt-file ABS \
[--prompt-flag FLAG] [--var KEY=VALUE]...] [--pin-file ABS --pin-sha256 HEX] [--version-cmd ARG]... \
[--pin-version TEXT] [--auth-file ABS --auth-key KEY...] SESSION CWD -- ABS_COMMAND [ARGS...]";
/// The pin is checked again at this interval while the session runs.
pub const PIN_RECHECK: Duration = Duration::from_secs(60);
const VERSION_TIMEOUT: Duration = Duration::from_secs(15);

fn error(s: impl Into<String>) -> Box<dyn std::error::Error> {
    s.into().into()
}

/// Trusted holder configuration, taken from Crab's worker argv.
#[derive(Clone, Debug, PartialEq)]
pub struct Spec {
    pub tmux: PathBuf,
    /// Optional private tmux server name (`tmux -L`), used by tests and isolated deployments.
    pub socket: Option<String>,
    pub session: String,
    pub cwd: PathBuf,
    pub command: Vec<String>,
    pub grace: Duration,
    pub poll: Duration,
    /// Value for the session option `@mova_crab_owner` on a session the holder creates.
    pub owner_marker: Option<String>,
    /// Host-wide singleton lock; `None` only with an explicit `--no-singleton`.
    pub lock: Option<PathBuf>,
    /// Role prompt template, appended to the command (after `prompt_flag`, if any).
    pub prompt_file: Option<PathBuf>,
    pub prompt_flag: Option<String>,
    /// `{{KEY}}` substitutions in the prompt template; every placeholder must be bound.
    pub vars: BTreeMap<String, String>,
    pub pin: Pin,
}
/// What the program must be before it may run (all parts optional, all given parts required).
#[derive(Clone, Debug, Default, PartialEq)]
pub struct Pin {
    /// File (resolved through symlinks) and its expected lowercase SHA-256.
    pub file: Option<(PathBuf, String)>,
    /// Absolute command printing the version, and its expected first output line.
    pub version: Option<(Vec<String>, String)>,
    /// JSON (full-line `//` comments allowed) file whose keys must hold non-empty values.
    pub auth: Option<(PathBuf, Vec<String>)>,
}
impl Pin {
    pub fn is_empty(&self) -> bool {
        self.file.is_none() && self.version.is_none() && self.auth.is_none()
    }
}

/// tmux session names and server names: ASCII identifiers (no `:` or `.` target syntax).
pub fn valid_ident(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 64
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
}
fn absolute(p: &Path) -> bool {
    p.is_absolute() && p.to_str().is_some_and(|s| s.len() <= 1024 && !s.contains('\0'))
}

pub fn parse_args(args: &[String]) -> Result<Spec> {
    let mut s = Spec {
        tmux: PathBuf::from("/usr/bin/tmux"),
        socket: None,
        session: String::new(),
        cwd: PathBuf::new(),
        command: Vec::new(),
        grace: Duration::from_secs(5),
        poll: Duration::from_millis(1000),
        owner_marker: None,
        lock: None,
        prompt_file: None,
        prompt_flag: None,
        vars: BTreeMap::new(),
        pin: Pin::default(),
    };
    let (mut pin_file, mut pin_sha, mut pin_version, mut auth_file) = (None, None, None, None);
    let (mut version_cmd, mut auth_keys) = (Vec::new(), Vec::new());
    let mut no_singleton = false;
    let mut positional = Vec::new();
    let mut separated = false;
    let mut i = 0;
    while i < args.len() {
        let a = args[i].as_str();
        if a == "--" {
            separated = true;
            i += 1;
            break;
        }
        if a == "--no-singleton" {
            no_singleton = true;
            i += 1;
            continue;
        }
        if !a.starts_with("--") {
            positional.push(a.to_string());
            i += 1;
            continue;
        }
        let v = args
            .get(i + 1)
            .ok_or_else(|| error(format!("{a} needs a value")))?
            .clone();
        match a {
            "--tmux" => s.tmux = PathBuf::from(v),
            "--socket" => s.socket = Some(v),
            "--grace" => s.grace = Duration::from_secs(v.parse()?),
            "--poll-ms" => s.poll = Duration::from_millis(v.parse()?),
            "--owner-marker" => s.owner_marker = Some(v),
            "--lock" => s.lock = Some(PathBuf::from(v)),
            "--prompt-file" => s.prompt_file = Some(PathBuf::from(v)),
            "--prompt-flag" => s.prompt_flag = Some(v),
            "--pin-file" => pin_file = Some(PathBuf::from(v)),
            "--pin-sha256" => pin_sha = Some(v),
            "--version-cmd" => version_cmd.push(v),
            "--pin-version" => pin_version = Some(v),
            "--auth-file" => auth_file = Some(PathBuf::from(v)),
            "--auth-key" => auth_keys.push(v),
            "--var" => {
                let (k, val) = v.split_once('=').ok_or("--var needs KEY=VALUE")?;
                if s.vars.insert(k.to_string(), val.to_string()).is_some() {
                    return Err(error(format!("--var {k} given twice")));
                }
            }
            _ => return Err(error(format!("unknown option {a}"))),
        }
        i += 2;
    }
    if !separated || positional.len() != 2 {
        return Err(error(USAGE));
    }
    if s.lock.is_some() == no_singleton {
        return Err(error("give exactly one of --lock FILE (singleton) or --no-singleton"));
    }
    s.pin.file = match (pin_file, pin_sha) {
        (Some(f), Some(h)) => Some((f, h)),
        (None, None) => None,
        _ => return Err(error("--pin-file and --pin-sha256 go together")),
    };
    s.pin.version = match (version_cmd.is_empty(), pin_version) {
        (false, Some(v)) => Some((version_cmd, v)),
        (true, None) => None,
        _ => return Err(error("--version-cmd and --pin-version go together")),
    };
    s.pin.auth = match (auth_file, auth_keys.is_empty()) {
        (Some(f), false) => Some((f, auth_keys)),
        (None, true) => None,
        _ => return Err(error("--auth-file needs at least one --auth-key")),
    };
    s.session = positional[0].clone();
    s.cwd = PathBuf::from(&positional[1]);
    s.command = args[i..].to_vec();
    validate(&s)?;
    Ok(s)
}

pub fn validate(s: &Spec) -> Result<()> {
    if !absolute(&s.tmux) || !absolute(&s.cwd) {
        return Err(error("tmux and cwd must be absolute paths"));
    }
    if !valid_ident(&s.session) || s.socket.as_deref().is_some_and(|x| !valid_ident(x)) {
        return Err(error("session and socket names must be ASCII identifiers"));
    }
    if s.command.is_empty()
        || s.command.len() > 64
        || !absolute(Path::new(&s.command[0]))
        || s.command.iter().any(|x| x.len() > 4096 || x.contains('\0'))
    {
        return Err(error("the command needs an absolute executable and bounded arguments"));
    }
    if !(1..=GRACE_MAX_SECONDS).contains(&s.grace.as_secs()) {
        return Err(error("grace must be 1..8 seconds (Crab kills after 10)"));
    }
    if !(50..=10_000).contains(&s.poll.as_millis()) {
        return Err(error("poll-ms must be 50..10000"));
    }
    if s.owner_marker.as_deref().is_some_and(|m| {
        m.is_empty() || m.len() > 128 || m.contains('|') || m.chars().any(char::is_control)
    }) {
        return Err(error("owner marker must be 1..128 printable characters without '|'"));
    }
    if s.lock.as_ref().is_some_and(|p| !absolute(p))
        || s.prompt_file.as_ref().is_some_and(|p| !absolute(p))
    {
        return Err(error("lock and prompt files must be absolute paths"));
    }
    if s.prompt_file.is_none() && (s.prompt_flag.is_some() || !s.vars.is_empty()) {
        return Err(error("--prompt-flag and --var need --prompt-file"));
    }
    if s.prompt_flag.as_deref().is_some_and(|f| !f.starts_with('-') || f.len() > 64)
        || s.vars.iter().any(|(k, v)| !valid_ident(k) || v.len() > 4096 || v.contains('\0'))
    {
        return Err(error("invalid prompt flag or variable"));
    }
    let hex = |h: &str| h.len() == 64 && h.bytes().all(|b| b.is_ascii_digit() || (b'a'..=b'f').contains(&b));
    if s.pin.file.as_ref().is_some_and(|(f, h)| !absolute(f) || !hex(h))
        || s.pin.version.as_ref().is_some_and(|(c, v)| {
            c.is_empty() || c.len() > 16 || !absolute(Path::new(&c[0])) || v.trim().is_empty() || v.len() > 256
        })
        || s.pin.auth.as_ref().is_some_and(|(f, k)| {
            !absolute(f) || k.len() > 8 || k.iter().any(|x| x.is_empty() || x.len() > 64 || !x.bytes().all(|b| b.is_ascii_alphanumeric() || b == b'_'))
        })
    {
        return Err(error("pin: absolute files, a lowercase SHA-256, an absolute version command and simple auth keys"));
    }
    Ok(())
}

fn sha256_file(p: &Path) -> Result<String> {
    use sha2::{Digest, Sha256};
    let mut h = Sha256::new();
    let mut f = File::open(p)?;
    let mut buf = vec![0u8; 1 << 16];
    loop {
        let n = f.read(&mut buf)?;
        if n == 0 {
            break;
        }
        h.update(&buf[..n]);
    }
    Ok(format!("{:x}", h.finalize()))
}
/// First stdout line of a bounded version command; the child is killed after the timeout.
fn version_line(cmd: &[String]) -> Result<String> {
    let mut child = Command::new(&cmd[0])
        .args(&cmd[1..])
        .stdin(std::process::Stdio::null())
        .stdout(std::process::Stdio::piped())
        .stderr(std::process::Stdio::null())
        .spawn()?;
    let end = Instant::now() + VERSION_TIMEOUT;
    loop {
        if child.try_wait()?.is_some() {
            break;
        }
        if Instant::now() >= end {
            let _ = child.kill();
            let _ = child.wait();
            return Err(error("the version command did not finish within 15 s"));
        }
        std::thread::sleep(Duration::from_millis(20));
    }
    let mut out = String::new();
    if let Some(o) = child.stdout.take() {
        o.take(4096).read_to_string(&mut out)?;
    }
    Ok(out.lines().next().unwrap_or("").trim().to_string())
}
/// Check the pin. `Err` is a plain English reason, ready to be shown to a person.
pub fn check_pin(name: &str, pin: &Pin) -> std::result::Result<(), String> {
    if let Some((file, expected)) = &pin.file {
        let real = fs::canonicalize(file).map_err(|e| format!("{name} is not pinned: {} cannot be resolved ({e})", file.display()))?;
        if !fs::metadata(&real).is_ok_and(|m| m.is_file()) {
            return Err(format!("{name} is not pinned: {} is not a regular file", real.display()));
        }
        let got = sha256_file(&real).map_err(|e| format!("{name} is not pinned: {} cannot be read ({e})", real.display()))?;
        if &got != expected {
            return Err(format!("{name} is not pinned: {} has SHA-256 {got}, expected {expected}", real.display()));
        }
    }
    if let Some((cmd, expected)) = &pin.version {
        let got = version_line(cmd).map_err(|e| format!("{name} is not pinned: the version check failed ({e})"))?;
        if got != expected.trim() {
            return Err(format!("{name} is not pinned: version {got:?}, expected {:?}", expected.trim()));
        }
    }
    if let Some((file, keys)) = &pin.auth {
        let text = fs::read_to_string(file).map_err(|_| format!("{name} is not authenticated: {} cannot be read", file.display()))?;
        let json: String = text.lines().filter(|l| !l.trim_start().starts_with("//")).collect::<Vec<_>>().join("\n");
        let v: serde_json::Value = serde_json::from_str(&json).map_err(|_| format!("{name} is not authenticated: {} is not JSON", file.display()))?;
        for k in keys {
            let present = match &v[k] {
                serde_json::Value::Null => false,
                serde_json::Value::String(x) => !x.is_empty(),
                serde_json::Value::Array(x) => !x.is_empty(),
                serde_json::Value::Object(x) => !x.is_empty(),
                _ => true,
            };
            if !present {
                return Err(format!("{name} is not authenticated: {k} is missing in {}", file.display()));
            }
        }
    }
    Ok(())
}

/// Render the prompt template: every `{{KEY}}` must be bound by `vars`, every var used.
pub fn render_prompt(template: &str, vars: &BTreeMap<String, String>) -> Result<String> {
    let mut out = String::with_capacity(template.len());
    let mut used = std::collections::BTreeSet::new();
    let mut rest = template;
    while let Some(open) = rest.find("{{") {
        out.push_str(&rest[..open]);
        let after = &rest[open + 2..];
        let close = after.find("}}").ok_or("unclosed {{ in the prompt")?;
        let key = &after[..close];
        let v = vars
            .get(key)
            .ok_or_else(|| error(format!("prompt placeholder {{{{{key}}}}} is not bound")))?;
        used.insert(key.to_string());
        out.push_str(v);
        rest = &after[close + 2..];
    }
    out.push_str(rest);
    if used.len() != vars.len() {
        return Err(error("a --var is not used by the prompt"));
    }
    if out.len() > PROMPT_MAX || out.contains('\0') {
        return Err(error("rendered prompt exceeds 100000 bytes or contains NUL"));
    }
    Ok(out)
}

/// The full argv the session runs: the command plus the rendered role prompt.
pub fn effective_command(s: &Spec) -> Result<Vec<String>> {
    let mut c = s.command.clone();
    if let Some(p) = &s.prompt_file {
        let mut text = String::new();
        File::open(p)?
            .take(PROMPT_MAX as u64 + 1)
            .read_to_string(&mut text)?;
        let rendered = render_prompt(&text, &s.vars)?;
        if let Some(f) = &s.prompt_flag {
            c.push(f.clone());
        }
        c.push(rendered);
    }
    Ok(c)
}

/// How the holder finished.
#[derive(Clone, Debug, PartialEq)]
pub enum Outcome {
    /// The session or its command ended on its own.
    Ended(&'static str),
    /// A stop request ended the command and the session.
    Stopped,
    /// A session of that name exists but does not run the configured command; untouched.
    Conflict(String),
    /// Another holder owns the role's singleton lock; its recorded identity is reported.
    AlreadyRunning(String),
    /// The pin or authentication check failed (before a start, or while running: then the
    /// session was ended). The text is the English reason.
    Unpinned(String),
}
impl Outcome {
    pub fn exit_code(&self) -> i32 {
        match self {
            Outcome::Ended(_) | Outcome::Stopped => 0,
            Outcome::Conflict(_) => 3,
            Outcome::AlreadyRunning(_) => 5,
            Outcome::Unpinned(_) => 6,
        }
    }
}

/// The singleton lock: exclusive `flock` on a fixed file, held for the holder's lifetime.
/// The descriptor is close-on-exec, so tmux and the program never inherit it.
pub struct Lock {
    _file: File,
}
pub enum Locked {
    Acquired(Lock),
    /// Held by another holder; the recorded owner text (pid, session, socket).
    Busy(String),
}
pub fn acquire(path: &Path, s: &Spec) -> Result<Locked> {
    let mut file = OpenOptions::new()
        .read(true)
        .write(true)
        .create(true)
        .truncate(false)
        .mode(0o600)
        .open(path)?;
    if unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } != 0 {
        let e = std::io::Error::last_os_error();
        if e.raw_os_error() != Some(libc::EWOULDBLOCK) {
            return Err(e.into());
        }
        let mut owner = String::new();
        let _ = (&file).take(4096).read_to_string(&mut owner);
        return Ok(Locked::Busy(owner.trim().to_string()));
    }
    file.set_len(0)?;
    file.rewind()?;
    let owner = serde_json::json!({"pid": std::process::id(), "session": s.session, "socket": s.socket});
    writeln!(file, "{owner}")?;
    file.flush()?;
    Ok(Locked::Acquired(Lock { _file: file }))
}

fn tmux(s: &Spec, args: &[&str]) -> Result<(bool, String)> {
    let mut c = Command::new(&s.tmux);
    if let Some(sock) = &s.socket {
        c.args(["-L", sock]);
    }
    let o = c.args(args).stdin(std::process::Stdio::null()).output()?;
    Ok((o.status.success(), String::from_utf8_lossy(&o.stdout).into_owned()))
}
/// Exact session target: `=NAME:` (without the colon, display/set commands may not resolve it).
fn target(s: &Spec) -> String {
    format!("={}:", s.session)
}
fn session_exists(s: &Spec) -> Result<bool> {
    Ok(tmux(s, &["has-session", "-t", &target(s)])?.0)
}

/// `(state, starttime)` from /proc/<pid>/stat.
fn proc_identity(pid: u32) -> Option<(char, String)> {
    let s = fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
    let f: Vec<&str> = s.rsplit_once(") ")?.1.split_whitespace().collect();
    Some((f.first()?.chars().next()?, f.get(19)?.to_string()))
}
fn proc_ppid(pid: u32) -> Option<u32> {
    let s = fs::read_to_string(format!("/proc/{pid}/stat")).ok()?;
    s.rsplit_once(") ")?.1.split_whitespace().nth(1)?.parse().ok()
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
fn basename(s: &str) -> &str {
    s.rsplit('/').next().unwrap_or(s)
}
/// A process runs the configured command when its arguments are equal and its executable has
/// the same file name (a shell may have resolved it through PATH); a `#!` interpreter may
/// precede the executable, as in Crab's own worker identity check.
pub fn command_matches(cmdline: &[String], command: &[String]) -> bool {
    if command.is_empty() || cmdline.len() < command.len() {
        return false;
    }
    let k = cmdline.len() - command.len();
    basename(&cmdline[k]) == basename(&command[0]) && cmdline[k + 1..] == command[1..]
}
/// The pane process itself or one of its direct children running the command, lowest pid first.
fn find_command(pane_pid: u32, command: &[String]) -> Option<u32> {
    if proc_cmdline(pane_pid).is_some_and(|c| command_matches(&c, command)) {
        return Some(pane_pid);
    }
    let mut children: Vec<u32> = fs::read_dir("/proc")
        .ok()?
        .take(PROC_SCAN_MAX)
        .filter_map(|e| e.ok()?.file_name().to_str()?.parse().ok())
        .filter(|p| proc_ppid(*p) == Some(pane_pid))
        .collect();
    children.sort_unstable();
    children
        .into_iter()
        .find(|p| proc_cmdline(*p).is_some_and(|c| command_matches(&c, command)))
}
/// The only pane of the session: (pane_pid, dead).
fn single_pane(s: &Spec) -> Result<Option<(u32, bool)>> {
    let (ok, out) = tmux(
        s,
        &["list-panes", "-s", "-t", &target(s), "-F", "#{pane_pid}|#{pane_dead}"],
    )?;
    let lines: Vec<&str> = out.lines().collect();
    if !ok || lines.len() != 1 {
        return Ok(None);
    }
    let (pid, dead) = lines[0].split_once('|').ok_or("pane format")?;
    Ok(Some((pid.parse()?, dead == "1")))
}

struct Held {
    pid: u32,
    starttime: String,
}
impl Held {
    fn alive(&self) -> bool {
        proc_identity(self.pid).is_some_and(|(st, t)| t == self.starttime && st != 'Z' && st != 'X')
    }
}

fn event(log: &mut dyn Write, s: &Spec, what: &str, detail: &str) {
    let line = serde_json::json!({"holder":"crab-tmux-worker","session":s.session,"event":what,"detail":detail});
    let _ = writeln!(log, "{line}");
    let _ = log.flush();
}

/// Run the holder until the session ends or `stop` is set. Never sends keys to the terminal.
pub fn run(s: &Spec, stop: &AtomicBool, log: &mut dyn Write) -> Result<Outcome> {
    validate(s)?;
    let _lock = match &s.lock {
        None => None,
        Some(p) => match acquire(p, s)? {
            Locked::Acquired(l) => Some(l),
            Locked::Busy(owner) => {
                event(log, s, "already_running", &owner);
                return Ok(Outcome::AlreadyRunning(owner));
            }
        },
    };
    if let Err(why) = check_pin(&s.session, &s.pin) {
        event(log, s, "unpinned", &why);
        return Ok(Outcome::Unpinned(why));
    }
    let command = effective_command(s)?;
    let held = if session_exists(s)? {
        let Some((pane, false)) = single_pane(s)? else {
            let why = "existing session is not exactly one live pane";
            event(log, s, "conflict", why);
            return Ok(Outcome::Conflict(why.into()));
        };
        let Some(pid) = find_command(pane, &command) else {
            let why = "existing session does not run the configured command";
            event(log, s, "conflict", why);
            return Ok(Outcome::Conflict(why.into()));
        };
        event(log, s, "adopted", &format!("pid {pid}"));
        pid
    } else {
        let cwd = s.cwd.to_str().ok_or("cwd")?;
        let mut args = vec!["new-session", "-d", "-s", &s.session, "-c", cwd, "--"];
        args.extend(command.iter().map(String::as_str));
        if !tmux(s, &args)?.0 {
            return Err(error("tmux new-session failed"));
        }
        if let Some(m) = &s.owner_marker {
            tmux(s, &["set-option", "-t", &target(s), "@mova_crab_owner", m])?;
        }
        // The pane process may still be between fork and exec; wait briefly for the command.
        let deadline = Instant::now() + Duration::from_secs(5);
        loop {
            if let Some((pane, false)) = single_pane(s)? {
                if let Some(pid) = find_command(pane, &command) {
                    event(log, s, "created", &format!("pid {pid}"));
                    break pid;
                }
            }
            if !session_exists(s)? {
                event(log, s, "ended", "session ended right after creation");
                return Ok(Outcome::Ended("session_ended"));
            }
            if Instant::now() >= deadline {
                return Err(error("created session does not run the configured command"));
            }
            std::thread::sleep(Duration::from_millis(50));
        }
    };
    let held = Held {
        starttime: proc_identity(held).ok_or("command vanished")?.1,
        pid: held,
    };
    let mut pinned_at = Instant::now();
    loop {
        if stop.load(Ordering::SeqCst) {
            return graceful_stop(s, &held, log);
        }
        if !s.pin.is_empty() && pinned_at.elapsed() >= PIN_RECHECK {
            pinned_at = Instant::now();
            if let Err(why) = check_pin(&s.session, &s.pin) {
                event(log, s, "unpinned", &why);
                graceful_stop(s, &held, log)?;
                return Ok(Outcome::Unpinned(why));
            }
        }
        if !session_exists(s)? {
            event(log, s, "ended", "session ended");
            return Ok(Outcome::Ended("session_ended"));
        }
        if !held.alive() {
            event(log, s, "ended", "command ended; the session is left as it is");
            return Ok(Outcome::Ended("command_ended"));
        }
        // Sleep in small steps so a stop request is noticed promptly.
        let until = Instant::now() + s.poll;
        while Instant::now() < until && !stop.load(Ordering::SeqCst) {
            std::thread::sleep(Duration::from_millis(25));
        }
    }
}

fn graceful_stop(s: &Spec, held: &Held, log: &mut dyn Write) -> Result<Outcome> {
    if held.alive() {
        unsafe { libc::kill(held.pid as i32, libc::SIGTERM) };
        event(log, s, "stopping", "SIGTERM to the command");
    }
    let deadline = Instant::now() + s.grace;
    while Instant::now() < deadline && held.alive() {
        std::thread::sleep(Duration::from_millis(25));
    }
    if session_exists(s)? {
        tmux(s, &["kill-session", "-t", &target(s)])?;
        let how = if held.alive() { "grace expired; session killed" } else { "session removed" };
        event(log, s, "stopped", how);
    } else {
        event(log, s, "stopped", "session ended");
    }
    Ok(Outcome::Stopped)
}

static STOP: AtomicBool = AtomicBool::new(false);
extern "C" fn on_signal(_: libc::c_int) {
    STOP.store(true, Ordering::SeqCst);
}
/// Binary entry: SIGTERM, SIGINT and SIGHUP request the graceful stop.
pub fn main_with_args(args: &[String]) -> i32 {
    let spec = match parse_args(args) {
        Ok(s) => s,
        Err(e) => {
            eprintln!("{e}");
            return 2;
        }
    };
    let handler = on_signal as extern "C" fn(libc::c_int) as libc::sighandler_t;
    unsafe {
        libc::signal(libc::SIGTERM, handler);
        libc::signal(libc::SIGINT, handler);
        libc::signal(libc::SIGHUP, handler);
    }
    let mut out = std::io::stdout();
    match run(&spec, &STOP, &mut out) {
        Ok(o) => {
            match &o {
                Outcome::Unpinned(why) | Outcome::Conflict(why) => eprintln!("{why}"),
                Outcome::AlreadyRunning(owner) => eprintln!("{} is already running: {owner}", spec.session),
                _ => {}
            }
            o.exit_code()
        }
        Err(e) => {
            eprintln!("crab-tmux-worker: {e}");
            4
        }
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use std::{
        os::unix::fs::PermissionsExt,
        sync::{atomic::AtomicU64, Arc},
        thread,
    };
    static NEXT: AtomicU64 = AtomicU64::new(0);

    /// Private tmux servers (`-L`) and a directory per test; the family's sessions are invisible.
    struct Fx {
        dir: PathBuf,
        socket: String,
        tmux: PathBuf,
    }
    impl Drop for Fx {
        fn drop(&mut self) {
            for sock in [self.socket.clone(), format!("{}-b", self.socket)] {
                let _ = Command::new("/usr/bin/tmux").args(["-L", &sock, "kill-server"]).output();
                let _ = fs::remove_file(test_socket_path(&sock));
            }
            let _ = fs::remove_dir_all(&self.dir);
        }
    }
    /// Where tmux keeps a private server's socket, so a test can remove it after kill-server.
    pub(crate) fn test_socket_path(name: &str) -> PathBuf {
        let base = std::env::var_os("TMUX_TMPDIR").map(PathBuf::from).unwrap_or_else(|| PathBuf::from("/tmp"));
        base.join(format!("tmux-{}", unsafe { libc::geteuid() })).join(name)
    }
    fn fx() -> Fx {
        let n = NEXT.fetch_add(1, Ordering::Relaxed);
        let dir = std::env::temp_dir().join(format!("crab-tmux-worker-{}-{n}", std::process::id()));
        fs::create_dir_all(&dir).unwrap();
        // Wrapper that logs every tmux subcommand, so tests can prove no keys are typed.
        let tmux = dir.join("tmux");
        fs::write(
            &tmux,
            format!(
                "#!/bin/sh\necho \"$3\" >> '{}'\nexec /usr/bin/tmux \"$@\"\n",
                dir.join("calls").display()
            ),
        )
        .unwrap();
        fs::set_permissions(&tmux, fs::Permissions::from_mode(0o700)).unwrap();
        Fx { dir, socket: format!("mova-crab-test-{}-{n}", std::process::id()), tmux }
    }
    const T: &str = "=dolphin-test:";
    impl Fx {
        fn spec(&self, command: Vec<String>) -> Spec {
            Spec {
                tmux: self.tmux.clone(),
                socket: Some(self.socket.clone()),
                session: "dolphin-test".into(),
                cwd: self.dir.clone(),
                command,
                grace: Duration::from_secs(2),
                poll: Duration::from_millis(50),
                owner_marker: Some("fixture-owner".into()),
                lock: Some(self.dir.join("role.lock")),
                prompt_file: None,
                prompt_flag: None,
                vars: BTreeMap::new(),
                pin: Pin::default(),
            }
        }
        fn raw_on(&self, sock: &str, args: &[&str]) -> (bool, String) {
            let o = Command::new("/usr/bin/tmux").args(["-L", sock]).args(args).output().unwrap();
            (o.status.success(), String::from_utf8_lossy(&o.stdout).into_owned())
        }
        fn raw(&self, args: &[&str]) -> (bool, String) {
            self.raw_on(&self.socket, args)
        }
        fn exists_on(&self, sock: &str) -> bool {
            self.raw_on(sock, &["has-session", "-t", T]).0
        }
        fn exists(&self) -> bool {
            self.exists_on(&self.socket)
        }
        fn pane(&self) -> u32 {
            self.raw(&["display-message", "-p", "-t", T, "#{pane_pid}"]).1.trim().parse().unwrap()
        }
        /// A fake LLM terminal: records SIGTERM in `term` and exits; otherwise runs forever.
        fn consultant(&self) -> Vec<String> {
            let p = self.dir.join("consultant.sh");
            fs::write(
                &p,
                format!(
                    "#!/bin/sh\ntrap 'echo term >> {}; exit 0' TERM\nwhile :; do sleep 0.05; done\n",
                    self.dir.join("term").display()
                ),
            )
            .unwrap();
            fs::set_permissions(&p, fs::Permissions::from_mode(0o700)).unwrap();
            vec![p.to_string_lossy().into_owned(), "--resume".into(), "fixture".into()]
        }
        fn calls(&self) -> String {
            fs::read_to_string(self.dir.join("calls")).unwrap_or_default()
        }
        fn wait(&self, what: impl Fn() -> bool) -> bool {
            let end = Instant::now() + Duration::from_secs(5);
            while Instant::now() < end {
                if what() {
                    return true;
                }
                thread::sleep(Duration::from_millis(20));
            }
            false
        }
    }
    fn spawn(s: Spec) -> (Arc<AtomicBool>, thread::JoinHandle<(Outcome, String)>) {
        let stop = Arc::new(AtomicBool::new(false));
        let st = stop.clone();
        let h = thread::spawn(move || {
            let mut log = Vec::new();
            let o = run(&s, &st, &mut log).unwrap();
            (o, String::from_utf8(log).unwrap())
        });
        (stop, h)
    }
    /// Run a holder that is expected to finish by itself; a holder still running after 3 s is
    /// stopped and reported as `None`, so a broken guard fails fast instead of hanging.
    fn run_bounded(f: &Fx, s: Spec) -> Option<(Outcome, String)> {
        let (stop, h) = spawn(s);
        let finished = f.wait(|| h.is_finished());
        stop.store(true, Ordering::SeqCst);
        let r = h.join().unwrap();
        finished.then_some(r)
    }
    fn assert_never_typed(f: &Fx) {
        let calls = f.calls();
        for forbidden in ["send-keys", "load-buffer", "paste-buffer", "send", "paste"] {
            assert!(!calls.lines().any(|l| l == forbidden), "holder typed into the terminal: {calls}");
        }
    }

    #[test]
    fn creates_holds_and_stops_gracefully() {
        let f = fx();
        let (stop, h) = spawn(f.spec(f.consultant()));
        assert!(f.wait(|| f.exists()), "session created");
        assert!(f.wait(|| f.raw(&["show-options", "-v", "-t", T, "@mova_crab_owner"]).1.trim() == "fixture-owner"));
        thread::sleep(Duration::from_millis(300));
        assert!(!h.is_finished(), "holder stays alive while the session lives");
        stop.store(true, Ordering::SeqCst);
        let (o, log) = h.join().unwrap();
        assert_eq!(o, Outcome::Stopped);
        assert!(!f.exists(), "session removed");
        let term = fs::read_to_string(f.dir.join("term")).unwrap_or_default();
        assert!(term.contains("term"), "SIGTERM reached the command: {log}");
        assert!(log.contains("\"created\"") && log.contains("\"stopped\""));
        assert_never_typed(&f);
    }

    #[test]
    fn exits_when_the_session_ends_on_its_own() {
        let f = fx();
        let (_stop, h) = spawn(f.spec(f.consultant()));
        assert!(f.wait(|| f.exists()));
        thread::sleep(Duration::from_millis(200));
        f.raw(&["kill-session", "-t", T]);
        let (o, _) = h.join().unwrap();
        assert!(matches!(o, Outcome::Ended(_)), "{o:?}");
        assert_eq!(o.exit_code(), 0);
    }

    #[test]
    fn adopts_a_matching_session_without_restarting_it() {
        let f = fx();
        let cmd = f.consultant();
        // Started by someone else through a shell, as a terminal tab would: the command is a child.
        let line = format!("{} --resume fixture; sleep 30", cmd[0]);
        assert!(f.raw(&["new-session", "-d", "-s", "dolphin-test", "/bin/sh", "-c", &line]).0);
        let before = f.pane();
        assert!(f.wait(|| find_command(before, &cmd).is_some()));
        let (stop, h) = spawn(f.spec(cmd));
        thread::sleep(Duration::from_millis(300));
        assert!(!h.is_finished());
        assert_eq!(f.pane(), before, "same pane, not restarted");
        assert!(!f.calls().lines().any(|l| l == "new-session"), "no duplicate session");
        stop.store(true, Ordering::SeqCst);
        let (o, log) = h.join().unwrap();
        assert!(log.contains("\"adopted\""), "{log}");
        assert_eq!(o, Outcome::Stopped);
        assert!(fs::read_to_string(f.dir.join("term")).unwrap_or_default().contains("term"));
        assert!(!f.exists(), "the leftover shell session is removed on stop");
        assert_never_typed(&f);
    }

    #[test]
    fn conflicting_session_is_left_untouched() {
        let f = fx();
        assert!(f.raw(&["new-session", "-d", "-s", "dolphin-test", "/bin/sleep", "30"]).0);
        let pane = f.pane();
        let o = run_bounded(&f, f.spec(f.consultant())).expect("holder must not hold a foreign session").0;
        assert!(matches!(o, Outcome::Conflict(_)), "{o:?}");
        assert_eq!(o.exit_code(), 3);
        assert!(f.exists(), "conflicting session still exists");
        assert_eq!(f.pane(), pane);
        let calls = f.calls();
        assert!(calls.lines().all(|l| ["has-session", "list-panes"].contains(&l)), "{calls}");
    }

    #[test]
    fn exits_when_the_command_ends_inside_a_surviving_shell() {
        let f = fx();
        let cmd = f.consultant();
        let line = format!("{} --resume fixture; sleep 30", cmd[0]);
        assert!(f.raw(&["new-session", "-d", "-s", "dolphin-test", "/bin/sh", "-c", &line]).0);
        let pane = f.pane();
        assert!(f.wait(|| find_command(pane, &cmd).is_some()));
        let (_stop, h) = spawn(f.spec(cmd.clone()));
        thread::sleep(Duration::from_millis(200));
        let pid = find_command(pane, &cmd).unwrap();
        unsafe { libc::kill(pid as i32, libc::SIGTERM) };
        let (o, _) = h.join().unwrap();
        assert_eq!(o, Outcome::Ended("command_ended"));
        assert!(f.exists(), "an ended command does not remove someone else's shell");
    }

    /// Two holders of one role on two different tmux servers: the lock lets only one run.
    /// Negative control in the same test: with `--no-singleton` both start a session.
    #[test]
    fn singleton_lock_admits_one_holder_per_role() {
        let f = fx();
        let first = f.spec(f.consultant());
        let mut second = first.clone();
        second.socket = Some(format!("{}-b", f.socket));
        let (stop, h) = spawn(first.clone());
        assert!(f.wait(|| f.exists()));
        let second_result = run_bounded(&f, second.clone());
        let second_session = f.exists_on(second.socket.as_deref().unwrap());
        if second_result.is_none() || second_session {
            stop.store(true, Ordering::SeqCst);
        }
        let o = second_result.expect("a second holder of the role must not keep running").0;
        let Outcome::AlreadyRunning(owner) = &o else { panic!("{o:?}") };
        assert!(owner.contains(&format!("\"pid\":{}", std::process::id())) && owner.contains("dolphin-test"), "{owner}");
        assert_eq!(o.exit_code(), 5);
        assert!(!second_session, "no second session");
        assert!(!h.is_finished(), "the running holder is untouched");
        // Red control: without the singleton lock the second holder starts a second Dolphin.
        let (mut a, mut b) = (first, second);
        a.lock = None;
        b.lock = None;
        stop.store(true, Ordering::SeqCst);
        h.join().unwrap();
        let (sa, ha) = spawn(a);
        let (sb, hb) = spawn(b.clone());
        assert!(f.wait(|| f.exists() && f.exists_on(b.socket.as_deref().unwrap())), "two Dolphins without the lock");
        sa.store(true, Ordering::SeqCst);
        sb.store(true, Ordering::SeqCst);
        ha.join().unwrap();
        hb.join().unwrap();
    }

    #[test]
    fn lock_is_released_when_the_holder_ends() {
        let f = fx();
        let s = f.spec(f.consultant());
        let (stop, h) = spawn(s.clone());
        assert!(f.wait(|| f.exists()));
        stop.store(true, Ordering::SeqCst);
        assert_eq!(h.join().unwrap().0, Outcome::Stopped);
        let (stop, h) = spawn(s);
        assert!(f.wait(|| f.exists()), "a later start takes the free lock");
        stop.store(true, Ordering::SeqCst);
        assert_eq!(h.join().unwrap().0, Outcome::Stopped);
    }

    #[test]
    fn role_prompt_is_an_argument_not_keystrokes() {
        let f = fx();
        fs::write(f.dir.join("prompt.md"), "Write reports to {{report_dir}}.\n").unwrap();
        let mut s = f.spec(f.consultant());
        s.prompt_file = Some(f.dir.join("prompt.md"));
        s.prompt_flag = Some("--append-system-prompt".into());
        s.vars.insert("report_dir".into(), "/srv/reports".into());
        let (stop, h) = spawn(s.clone());
        assert!(f.wait(|| f.exists()));
        let full = effective_command(&s).unwrap();
        assert_eq!(full[3..], ["--append-system-prompt".to_string(), "Write reports to /srv/reports.\n".to_string()]);
        assert!(f.wait(|| find_command(f.pane(), &full).is_some()), "the session runs command + prompt");
        stop.store(true, Ordering::SeqCst);
        h.join().unwrap();
        assert_never_typed(&f);
        let v = |k: &str, x: &str| BTreeMap::from([(k.to_string(), x.to_string())]);
        assert!(render_prompt("to {{a}}", &BTreeMap::new()).is_err(), "unbound placeholder");
        assert!(render_prompt("to {{a", &v("a", "x")).is_err(), "unclosed placeholder");
        assert!(render_prompt("plain", &v("a", "x")).is_err(), "unused variable");
        assert_eq!(render_prompt("{{a}}/{{a}}", &v("a", "x")).unwrap(), "x/x");
    }

    /// A fake Copilot: an executable whose `--version` prints a version, plus an auth file.
    fn fake_copilot(f: &Fx, version: &str) -> (PathBuf, Pin) {
        let exe = f.dir.join("copilot");
        fs::write(&exe, format!("#!/bin/sh\nif [ \"$1\" = --version ]; then echo 'Fake Copilot {version}.'; echo 'Run update.'; exit 0; fi\ntrap 'exit 0' TERM\nwhile :; do sleep 0.05; done\n")).unwrap();
        fs::set_permissions(&exe, fs::Permissions::from_mode(0o700)).unwrap();
        let auth = f.dir.join("config.json");
        fs::write(&auth, "// comment line\n{\"lastLoggedInUser\": {\"login\": \"x\"}, \"authTokens\": {\"k\": \"v\"}}\n").unwrap();
        let pin = Pin {
            file: Some((exe.clone(), sha256_file(&exe).unwrap())),
            version: Some((vec![exe.to_string_lossy().into_owned(), "--version".into()], "Fake Copilot 1.0.98.".into())),
            auth: Some((auth, vec!["lastLoggedInUser".into(), "authTokens".into()])),
        };
        (exe, pin)
    }

    #[test]
    fn pinned_program_starts_and_an_unpinned_one_is_refused() {
        let f = fx();
        let (exe, pin) = fake_copilot(&f, "1.0.98");
        assert_eq!(check_pin("copilot", &pin), Ok(()));
        let mut s = f.spec(vec![exe.to_string_lossy().into_owned(), "--screen-reader".into()]);
        s.pin = pin.clone();
        let (stop, h) = spawn(s.clone());
        assert!(f.wait(|| f.exists()), "a pinned program starts");
        stop.store(true, Ordering::SeqCst);
        assert_eq!(h.join().unwrap().0, Outcome::Stopped);
        // The program updated itself: another version, another hash.
        fake_copilot(&f, "1.0.99");
        let (o, log) = run_bounded(&f, s.clone()).expect("refused at once");
        let Outcome::Unpinned(why) = &o else { panic!("{o:?}") };
        assert!(why.contains("dolphin-test is not pinned") && why.contains("SHA-256"), "{why}");
        assert!(log.contains("\"unpinned\""));
        assert_eq!(o.exit_code(), 6);
        assert!(!f.exists(), "no session for an unpinned program");
        // Same bytes, but the version line differs from the pin: refused by the version check.
        let mut v = pin.clone();
        v.file = None;
        v.version.as_mut().unwrap().1 = "Fake Copilot 1.0.97.".into();
        let why = check_pin("Copilot", &v).unwrap_err();
        assert!(why.starts_with("Copilot is not pinned: version \"Fake Copilot 1.0.99.\", expected \"Fake Copilot 1.0.97.\""), "{why}");
        // Not logged in: the key is missing; nothing is printed from the file.
        fs::write(f.dir.join("config.json"), "{\"lastLoggedInUser\": null, \"authTokens\": {}}").unwrap();
        let a = Pin { auth: pin.auth.clone(), ..Pin::default() };
        let why = check_pin("Copilot", &a).unwrap_err();
        assert!(why.starts_with("Copilot is not authenticated: lastLoggedInUser is missing"), "{why}");
    }

    #[test]
    fn matching_and_argument_refusals() {
        let c = |v: &[&str]| v.iter().map(|s| s.to_string()).collect::<Vec<_>>();
        assert!(command_matches(&c(&["claude", "--resume", "x"]), &c(&["/opt/bin/claude", "--resume", "x"])));
        assert!(command_matches(&c(&["/bin/sh", "/opt/w.sh", "a"]), &c(&["/opt/w.sh", "a"])));
        assert!(!command_matches(&c(&["claude", "--resume", "y"]), &c(&["/opt/bin/claude", "--resume", "x"])));
        assert!(!command_matches(&c(&["bash", "-l"]), &c(&["/opt/bin/claude"])));
        assert!(!command_matches(&c(&["claude", "--resume", "x", "-v"]), &c(&["/opt/bin/claude", "--resume", "x"])));
        let ok = c(&["--lock", "/srv/l", "--socket", "t1", "--grace", "5", "dolphin", "/srv/run", "--", "/opt/bin/claude", "--model", "opus"]);
        let s = parse_args(&ok).unwrap();
        assert_eq!(s.session, "dolphin");
        assert_eq!(s.lock, Some(PathBuf::from("/srv/l")));
        assert_eq!(s.command, c(&["/opt/bin/claude", "--model", "opus"]));
        assert!(parse_args(&c(&["--no-singleton", "d", "/srv", "--", "/bin/x"])).unwrap().lock.is_none());
        let pinned = parse_args(&c(&["--lock", "/srv/l", "--pin-file", "/srv/c", "--pin-sha256", &"a".repeat(64),
            "--version-cmd", "/srv/c", "--version-cmd", "--version", "--pin-version", "C 1.0.",
            "--auth-file", "/srv/a", "--auth-key", "authTokens", "d", "/srv", "--", "/srv/c"])).unwrap();
        assert_eq!(pinned.pin.version, Some((c(&["/srv/c", "--version"]), "C 1.0.".to_string())));
        for bad in [
            c(&["d", "/srv/run", "--", "/opt/bin/claude"]),
            c(&["--no-singleton", "--lock", "/srv/l", "d", "/srv/run", "--", "/opt/bin/claude"]),
            c(&["--lock", "rel", "d", "/srv/run", "--", "/opt/bin/claude"]),
            c(&["--lock", "/srv/l", "d", "/srv/run", "--", "claude"]),
            c(&["--lock", "/srv/l", "d", "relative", "--", "/opt/bin/claude"]),
            c(&["--lock", "/srv/l", "bad:target", "/srv/run", "--", "/opt/bin/claude"]),
            c(&["--lock", "/srv/l", "d", "/srv/run", "/opt/bin/claude"]),
            c(&["--lock", "/srv/l", "d", "/srv/run", "--"]),
            c(&["--lock", "/srv/l", "--grace", "9", "d", "/srv/run", "--", "/opt/bin/claude"]),
            c(&["--lock", "/srv/l", "--grace", "0", "d", "/srv/run", "--", "/opt/bin/claude"]),
            c(&["--lock", "/srv/l", "--tmux", "tmux", "d", "/srv/run", "--", "/opt/bin/claude"]),
            c(&["--lock", "/srv/l", "--socket", "a.b", "d", "/srv/run", "--", "/opt/bin/claude"]),
            c(&["--lock", "/srv/l", "--owner-marker", "a|b", "d", "/srv/run", "--", "/opt/bin/claude"]),
            c(&["--lock", "/srv/l", "--var", "a=b", "d", "/srv/run", "--", "/opt/bin/claude"]),
            c(&["--lock", "/srv/l", "--unknown", "x", "d", "/srv/run", "--", "/opt/bin/claude"]),
            c(&["--lock", "/srv/l", "--pin-file", "/srv/c", "d", "/srv/run", "--", "/opt/bin/claude"]),
            c(&["--lock", "/srv/l", "--pin-file", "/srv/c", "--pin-sha256", "ABC", "d", "/srv/run", "--", "/opt/bin/claude"]),
            c(&["--lock", "/srv/l", "--pin-version", "1.0", "d", "/srv/run", "--", "/opt/bin/claude"]),
            c(&["--lock", "/srv/l", "--version-cmd", "rel", "--pin-version", "1.0", "d", "/srv/run", "--", "/opt/bin/claude"]),
            c(&["--lock", "/srv/l", "--auth-file", "/srv/a", "d", "/srv/run", "--", "/opt/bin/claude"]),
        ] {
            assert!(parse_args(&bad).is_err(), "{bad:?}");
        }
    }
}
