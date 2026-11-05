//! A Mova animal's private LLM terminal.
//!
//! The code is shared; the AI is not. Every animal instance that needs a large language model
//! (a Whale, the Nautilus, a Sea Turtle, ...) holds its own interactive LLM session in its own
//! tmux session, with its own owner marker, lock, task directory and journal. Nothing here is
//! shared between animals, and an animal never borrows a Dolphin.
//!
//! The session itself is held by `crab-tmux-worker` (tools/crab), started by Crab like any
//! worker. This crate only *asks*: one bounded question at a time.
//!
//! Protocol (`ask`):
//! 1. Take the terminal's exclusive lock (`ROOT/terminal.lock`); a second asker is refused.
//! 2. Refuse while an earlier question has no answer file: a question is never replayed.
//! 3. Observe the pane: exactly one window and pane, the configured command, the owner marker,
//!    no working or modal text near the bottom, and the idle marker among the last
//!    `idle_window` non-empty lines. Anything else: no question is sent.
//! 4. Write the task file, journal `asked`, load it into a tmux buffer, observe again (the
//!    screen must be unchanged and idle), paste it bracketed and press Enter.
//! 5. The LLM writes its whole answer to `ROOT/answers/ID.txt` and prints the idle marker.
//!    `answer` reads that file (bounded) and journals `answered` once.
//!
//! The answer is evidence for the animal to read; it is never executed.
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    os::unix::{fs::OpenOptionsExt, io::AsRawFd},
    path::{Path, PathBuf},
    process::Command,
    time::{Duration, Instant, SystemTime, UNIX_EPOCH},
};

pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error + Send + Sync>>;

fn error(s: impl Into<String>) -> Box<dyn std::error::Error + Send + Sync> {
    s.into().into()
}

pub fn sha256(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}

fn default_idle_window() -> usize {
    4
}
fn default_question_max() -> usize {
    100_000
}
fn default_answer_max() -> usize {
    400_000
}
fn default_timeout() -> u64 {
    1800
}

/// One animal's terminal. All paths absolute.
#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Profile {
    /// The animal instance that owns this terminal, e.g. "whale-3".
    pub animal: String,
    /// The tmux session held for it by crab-tmux-worker.
    pub session: String,
    pub tmux: PathBuf,
    /// Optional tmux server socket name (`-L`), used by tests.
    #[serde(default)]
    pub socket: Option<String>,
    /// `pane_current_command` of the live LLM CLI (`node` for Codex, `claude` for Claude Code).
    pub pane_command: String,
    /// Value of the session option `@mova_crab_owner` set by the holder.
    pub owner_marker: String,
    /// Line the LLM prints when it is ready for the next question.
    pub idle_marker: String,
    #[serde(default = "default_idle_window")]
    pub idle_window: usize,
    /// Task, answer and journal directory; the LLM must be allowed to write `ROOT/answers`.
    pub root: PathBuf,
    #[serde(default = "default_question_max")]
    pub question_max_bytes: usize,
    #[serde(default = "default_answer_max")]
    pub answer_max_bytes: usize,
    #[serde(default = "default_timeout")]
    pub answer_timeout_seconds: u64,
}

pub fn valid_ident(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 64
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b == b'-' || b == b'_')
}

impl Profile {
    pub fn load(path: &Path) -> Result<Profile> {
        let b = fs::read(path)?;
        if b.len() > 64 * 1024 {
            return Err(error("terminal profile too large"));
        }
        let p: Profile = serde_json::from_slice(&b)?;
        p.validate()?;
        Ok(p)
    }
    pub fn validate(&self) -> Result<()> {
        if !valid_ident(&self.animal) || !valid_ident(&self.session) {
            return Err(error("animal and session must be short identifiers"));
        }
        if !self.tmux.is_absolute() || !self.root.is_absolute() {
            return Err(error("tmux and root must be absolute paths"));
        }
        if let Some(s) = &self.socket {
            if !valid_ident(s) {
                return Err(error("socket must be a short identifier"));
            }
        }
        for (k, v) in [
            ("pane_command", &self.pane_command),
            ("owner_marker", &self.owner_marker),
            ("idle_marker", &self.idle_marker),
        ] {
            if v.trim().is_empty() || v.contains('\n') || v.contains('|') || v.len() > 128 {
                return Err(error(format!("{k} must be one short line without '|'")));
            }
        }
        if self.idle_window == 0 || self.idle_window > 20 {
            return Err(error("idle_window must be 1..=20"));
        }
        if self.question_max_bytes == 0 || self.question_max_bytes > 100_000 {
            return Err(error("question_max_bytes must be 1..=100000"));
        }
        if self.answer_max_bytes == 0 || self.answer_max_bytes > 4_000_000 {
            return Err(error("answer_max_bytes must be 1..=4000000"));
        }
        Ok(())
    }
    fn dir(&self, name: &str) -> Result<PathBuf> {
        let d = self.root.join(name);
        fs::create_dir_all(&d)?;
        Ok(d)
    }
    pub fn answer_path(&self, id: &str) -> PathBuf {
        self.root.join("answers").join(format!("{id}.txt"))
    }
    fn journal_path(&self) -> PathBuf {
        self.root.join("journal.jsonl")
    }
    fn target(&self) -> String {
        format!("={}:", self.session)
    }
}

fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .map(|d| d.as_secs())
        .unwrap_or(0)
}

fn tmux(p: &Profile, args: &[&str]) -> Result<String> {
    let mut c = Command::new(&p.tmux);
    if let Some(s) = &p.socket {
        c.args(["-L", s]);
    }
    let out = c.args(args).output()?;
    if !out.status.success() {
        return Err(error(format!(
            "tmux {} failed: {}",
            args.first().unwrap_or(&""),
            String::from_utf8_lossy(&out.stderr).trim()
        )));
    }
    Ok(String::from_utf8_lossy(&out.stdout).into_owned())
}

#[derive(Clone, Debug, PartialEq, Eq, Serialize)]
#[serde(rename_all = "snake_case")]
pub enum State {
    Idle,
    Busy,
    Modal,
    Absent,
    Mismatch,
}

#[derive(Clone, Debug, Serialize)]
pub struct Observation {
    pub state: State,
    pub detail: String,
    #[serde(skip)]
    pub screen: String,
}

/// Text that means the LLM is working, near the bottom of the screen.
const WORKING: &[&str] = &["esc to interrupt", "esc to cancel", "working ("];
/// Text that means the terminal waits for a human decision.
const MODAL: &[&str] = &[
    "do you want to",
    "allow this",
    "approve this",
    "waiting for permission",
    "update available",
    "update now",
    "trust this",
    "sign in",
    "log in",
];
/// Only the bottom of the screen decides the state: an echoed question above may quote anything.
const BOTTOM_LINES: usize = 12;

/// Classify a captured screen. Pure, for tests.
pub fn classify(p: &Profile, display: &str, capture: &str) -> Observation {
    let fields: Vec<_> = display.trim().split('|').collect();
    let obs = |state, detail: &str| Observation {
        state,
        detail: detail.to_string(),
        screen: capture.to_string(),
    };
    if fields.len() != 6 {
        return obs(State::Mismatch, "unexpected tmux display");
    }
    if fields[0] != p.session || fields[1] != "1" || fields[2] != "1" {
        return obs(State::Mismatch, "session must have exactly one window and pane");
    }
    if fields[4] != "0" {
        return obs(State::Absent, "pane is dead");
    }
    if fields[5] != p.owner_marker {
        return obs(State::Mismatch, "owner marker differs: not this animal's terminal");
    }
    if fields[3] != p.pane_command {
        return obs(State::Mismatch, "pane runs another command");
    }
    let lines: Vec<&str> = capture
        .lines()
        .map(str::trim)
        .filter(|l| !l.is_empty())
        .collect();
    let bottom = &lines[lines.len().saturating_sub(BOTTOM_LINES)..];
    let low = bottom.join("\n").to_lowercase();
    if MODAL.iter().any(|m| low.contains(m)) {
        return obs(State::Modal, "the terminal waits for a human decision");
    }
    if WORKING.iter().any(|m| low.contains(m)) {
        return obs(State::Busy, "the LLM is working");
    }
    let tail = &lines[lines.len().saturating_sub(p.idle_window)..];
    // TUIs put a bullet before printed lines ("• TERMINAL READY", "⏺ TERMINAL READY").
    let bare = |l: &str| l.trim_start_matches(|c: char| !c.is_alphanumeric()).trim().to_string();
    if tail.iter().any(|l| bare(l) == bare(&p.idle_marker)) {
        obs(State::Idle, "ready")
    } else {
        obs(State::Busy, "idle marker not at the bottom")
    }
}

pub fn observe(p: &Profile) -> Result<Observation> {
    let has = {
        let mut c = Command::new(&p.tmux);
        if let Some(s) = &p.socket {
            c.args(["-L", s]);
        }
        c.args(["has-session", "-t", &format!("={}", p.session)])
            .output()?
            .status
            .success()
    };
    if !has {
        return Ok(Observation {
            state: State::Absent,
            detail: "no such session".into(),
            screen: String::new(),
        });
    }
    let t = p.target();
    let display = tmux(
        p,
        &[
            "display-message",
            "-p",
            "-t",
            &t,
            "#{session_name}|#{session_windows}|#{window_panes}|#{pane_current_command}|#{pane_dead}|#{@mova_crab_owner}",
        ],
    )?;
    let capture = tmux(p, &["capture-pane", "-p", "-t", &t, "-S", "-60"])?;
    Ok(classify(p, &display, &capture))
}

struct Lock(File);
impl Drop for Lock {
    fn drop(&mut self) {
        unsafe {
            libc::flock(self.0.as_raw_fd(), libc::LOCK_UN);
        }
    }
}
fn lock(p: &Profile) -> Result<Lock> {
    fs::create_dir_all(&p.root)?;
    let f = OpenOptions::new()
        .create(true)
        .append(true)
        .mode(0o600)
        .open(p.root.join("terminal.lock"))?;
    if unsafe { libc::flock(f.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } != 0 {
        return Err(error("another question to this terminal is being sent"));
    }
    Ok(Lock(f))
}

fn append(p: &Profile, v: &Value) -> Result<()> {
    let mut f = OpenOptions::new()
        .create(true)
        .append(true)
        .mode(0o600)
        .open(p.journal_path())?;
    let mut line = serde_json::to_vec(v)?;
    line.push(b'\n');
    f.write_all(&line)?;
    f.sync_all()?;
    Ok(())
}

/// Journal lines, oldest first.
pub fn journal(p: &Profile) -> Result<Vec<Value>> {
    let path = p.journal_path();
    if !path.exists() {
        return Ok(vec![]);
    }
    let mut out = vec![];
    for l in fs::read_to_string(path)?.lines() {
        if !l.trim().is_empty() {
            out.push(serde_json::from_str(l)?);
        }
    }
    Ok(out)
}

/// The id of a question that was sent (or may have been) and has no recorded answer yet.
pub fn pending(p: &Profile) -> Result<Option<String>> {
    let mut open: Vec<String> = vec![];
    for v in journal(p)? {
        let id = v["id"].as_str().unwrap_or_default().to_string();
        match v["event"].as_str() {
            Some("reserved") => open.push(id),
            Some("answered") | Some("abandoned") => open.retain(|x| *x != id),
            _ => {}
        }
    }
    Ok(open.pop())
}

fn read_answer(p: &Profile, id: &str) -> Result<Option<String>> {
    let path = p.answer_path(id);
    let meta = match fs::symlink_metadata(&path) {
        Ok(m) => m,
        Err(_) => return Ok(None),
    };
    if !meta.file_type().is_file() {
        return Err(error("answer path is not a regular file"));
    }
    if meta.len() as usize > p.answer_max_bytes {
        return Err(error("answer exceeds answer_max_bytes"));
    }
    let mut s = String::new();
    File::open(&path)?.read_to_string(&mut s)?;
    Ok(Some(s))
}

/// The answer to `id` if the LLM has written it; journals `answered` once.
pub fn answer(p: &Profile, id: &str) -> Result<Option<String>> {
    if !valid_ident(id) {
        return Err(error("bad question id"));
    }
    let Some(text) = read_answer(p, id)? else {
        return Ok(None);
    };
    let done = journal(p)?
        .iter()
        .any(|v| v["id"] == id && v["event"] == "answered");
    if !done {
        append(
            p,
            &json!({"event":"answered","id":id,"at":now(),"answer_sha256":sha256(text.as_bytes()),"bytes":text.len()}),
        )?;
    }
    Ok(Some(text))
}

/// Mark a pending question abandoned (an operator decision); its answer, if it ever comes, is kept.
pub fn abandon(p: &Profile, id: &str, reason: &str) -> Result<()> {
    if pending(p)?.as_deref() != Some(id) {
        return Err(error("that question is not pending"));
    }
    append(p, &json!({"event":"abandoned","id":id,"at":now(),"reason":reason}))
}

pub fn task_text(p: &Profile, id: &str, question: &str) -> String {
    format!(
        "Question from {animal}, a Mova animal (question {id}).\n\
Answer it from your own knowledge and judgment, in plain English. Do not change any file, run no \
programs and send no messages, with one exception: write your complete answer, as plain text, to \
the answer file below. Then print exactly this line on its own: {marker}\n\
Answer file: {path}\n\n\
Question:\n{question}\n\
End of question.\n",
        animal = p.animal,
        marker = p.idle_marker,
        path = p.answer_path(id).display(),
    )
}

#[derive(Clone, Debug, Serialize)]
pub struct Asked {
    pub id: String,
    pub answer_path: PathBuf,
}

/// Send one question. Never replays: refuses while an earlier question is unanswered.
pub fn ask(p: &Profile, question: &str) -> Result<Asked> {
    p.validate()?;
    if question.trim().is_empty() {
        return Err(error("empty question"));
    }
    if question.len() > p.question_max_bytes {
        return Err(error("question exceeds question_max_bytes"));
    }
    let _l = lock(p)?;
    if let Some(id) = pending(p)? {
        if read_answer(p, &id)?.is_some() {
            answer(p, &id)?;
        } else {
            return Err(error(format!(
                "question {id} has no answer yet; a question is never replayed"
            )));
        }
    }
    // A TUI may repaint between observations (a footer, a timer). Nothing is sent unless the screen is idle and
    // unchanged from just before the paste; a changed screen abandons that attempt (journaled, nothing sent) and a
    // fresh attempt follows, up to ATTEMPTS.
    const ATTEMPTS: usize = 4;
    let mut last = String::new();
    for attempt in 0..ATTEMPTS {
        if attempt > 0 {
            std::thread::sleep(Duration::from_millis(700));
        }
        let first = stable_idle(p)?;
        let seq = journal(p)?.len();
        let id = sha256(format!("{}|{}|{}|{}", p.animal, seq, now(), question).as_bytes())[..16].to_string();
        p.dir("answers")?;
        let task = p.dir("tasks")?.join(format!("{id}.md"));
        let text = task_text(p, &id, question);
        let mut f = OpenOptions::new()
            .create_new(true)
            .write(true)
            .mode(0o600)
            .open(&task)?;
        f.write_all(text.as_bytes())?;
        f.sync_all()?;
        append(
            p,
            &json!({"event":"reserved","id":id,"at":now(),"question_sha256":sha256(question.as_bytes()),"task_sha256":sha256(text.as_bytes())}),
        )?;
        let buffer = format!("mova-terminal-{id}");
        tmux(p, &["load-buffer", "-b", &buffer, task.to_str().ok_or("task path UTF-8")?])?;
        let second = observe(p)?;
        if second.state != State::Idle || second.screen != first.screen {
            let _ = tmux(p, &["delete-buffer", "-b", &buffer]);
            append(p, &json!({"event":"abandoned","id":id,"at":now(),"reason":"terminal changed before paste; not sent","attempt":attempt+1}))?;
            last = second.detail;
            continue;
        }
        let t = p.target();
        tmux(p, &["paste-buffer", "-p", "-d", "-b", &buffer, "-t", &t])?;
        tmux(p, &["send-keys", "-t", &t, "Enter"])?;
        append(p, &json!({"event":"sent","id":id,"at":now()}))?;
        return Ok(Asked {
            answer_path: p.answer_path(&id),
            id,
        });
    }
    Err(error(format!(
        "terminal kept changing before the paste ({ATTEMPTS} attempts; last: {last}); nothing was sent"
    )))
}

/// An idle observation whose screen equals the one taken just before it (a settled repaint).
fn stable_idle(p: &Profile) -> Result<Observation> {
    let mut prev = observe(p)?;
    for _ in 0..10 {
        if prev.state != State::Idle {
            return Err(error(format!("terminal not idle: {}", prev.detail)));
        }
        std::thread::sleep(Duration::from_millis(300));
        let next = observe(p)?;
        if next.state == State::Idle && next.screen == prev.screen {
            return Ok(next);
        }
        prev = next;
    }
    Err(error("terminal screen does not settle; nothing was sent"))
}

/// Ask and wait for the answer file and the idle marker.
pub fn ask_wait(p: &Profile, question: &str, poll: Duration) -> Result<(String, String)> {
    let a = ask(p, question)?;
    let start = Instant::now();
    let limit = Duration::from_secs(p.answer_timeout_seconds);
    loop {
        if let Some(text) = read_answer(p, &a.id)? {
            if observe(p)?.state == State::Idle {
                answer(p, &a.id)?;
                return Ok((a.id, text));
            }
        }
        if start.elapsed() > limit {
            return Err(error(format!(
                "no answer to {} within {}s; it stays pending and is never re-sent",
                a.id, p.answer_timeout_seconds
            )));
        }
        std::thread::sleep(poll);
    }
}

pub fn status(p: &Profile) -> Result<Value> {
    let o = observe(p)?;
    let j = journal(p)?;
    let count = |e: &str| j.iter().filter(|v| v["event"] == e).count();
    Ok(json!({
        "animal": p.animal,
        "session": p.session,
        "state": o.state,
        "detail": o.detail,
        "pending": pending(p)?,
        "sent": count("sent"),
        "answered": count("answered"),
        "abandoned": count("abandoned"),
    }))
}

#[cfg(test)]
mod tests {
    use super::*;
    fn prof() -> Profile {
        Profile {
            animal: "whale-1".into(),
            session: "whale-1-sol".into(),
            tmux: "/usr/bin/tmux".into(),
            socket: None,
            pane_command: "node".into(),
            owner_marker: "mova-whale-1-sol".into(),
            idle_marker: "TERMINAL READY".into(),
            idle_window: 4,
            root: "/tmp/x".into(),
            question_max_bytes: 100_000,
            answer_max_bytes: 400_000,
            answer_timeout_seconds: 10,
        }
    }
    const D: &str = "whale-1-sol|1|1|node|0|mova-whale-1-sol";
    #[test]
    fn codex_footer_below_the_marker_is_idle() {
        let s = "• ready\n  TERMINAL READY\n  Worked for 37s\n› Ask Codex to do anything\n  GPT-6.1-Sol medium · /x\n  ? for shortcuts";
        assert_eq!(classify(&prof(), D, s).state, State::Busy, "window 4 is too small for 5 footer lines");
        let mut p = prof();
        p.idle_window = 6;
        assert_eq!(classify(&p, D, s).state, State::Idle);
        let bullet = "• TERMINAL READY\n  Worked for 5s\n› Ask Codex to do anything\n  GPT-6.1-Sol\n  ? for shortcuts";
        assert_eq!(classify(&p, D, bullet).state, State::Idle);
        assert_eq!(classify(&p, D, "• TERMINAL READY NOW\n").state, State::Busy);
    }
    #[test]
    fn working_modal_and_ownership_are_not_idle() {
        let p = prof();
        assert_eq!(classify(&p, D, "TERMINAL READY\nWorking (3s • esc to interrupt)").state, State::Busy);
        assert_eq!(classify(&p, D, "Do you want to allow this?\nTERMINAL READY").state, State::Modal);
        assert_eq!(classify(&p, "whale-1-sol|1|1|node|0|mova-whale-2-sol", "TERMINAL READY").state, State::Mismatch);
        assert_eq!(classify(&p, "whale-1-sol|1|2|node|0|mova-whale-1-sol", "TERMINAL READY").state, State::Mismatch);
        assert_eq!(classify(&p, "whale-1-sol|1|1|bash|0|mova-whale-1-sol", "TERMINAL READY").state, State::Mismatch);
        assert_eq!(classify(&p, "whale-1-sol|1|1|node|1|mova-whale-1-sol", "TERMINAL READY").state, State::Absent);
    }
    #[test]
    fn a_quoted_question_far_above_does_not_block() {
        let mut s = String::from("Question: how do I sign in?\n");
        for i in 0..20 {
            s.push_str(&format!("line {i}\n"));
        }
        s.push_str("TERMINAL READY\n");
        assert_eq!(classify(&prof(), D, &s).state, State::Idle);
    }
    #[test]
    fn profile_validation_refuses_bad_values() {
        let mut p = prof();
        assert!(p.validate().is_ok());
        p.idle_marker = "a|b".into();
        assert!(p.validate().is_err());
        let mut p = prof();
        p.root = "rel".into();
        assert!(p.validate().is_err());
        let mut p = prof();
        p.question_max_bytes = 100_001;
        assert!(p.validate().is_err());
    }
}
