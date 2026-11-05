//! A Whale's private terminal: the large language model a Whale holds for questions while it
//! reads a paper. The interface is narrow on purpose: the Whale asks one bounded question and
//! receives one plain-text answer; it never hands the terminal its draft or asks it to write the
//! description.
//!
//! The real terminal is `mova-terminal` (`tools/terminal`): one held interactive session per
//! animal instance, asked through its `Profile`. `MovaTerminal` is the thin adapter. Each Whale
//! config names exactly one profile whose `animal` is the Whale itself, and the Whale holds
//! `ROOT/holder.lock` of that profile for its whole run, so a second Whale cannot attach (the
//! terminal's own `terminal.lock` additionally serializes each single question). Which model the
//! session runs (Opus by default, or any configured model) is the holder's command line, not the
//! asker's. `NoTerminal` (dry runs) and `FakeTerminal` (tests) never start or ask anything.
use crate::util::{error, try_lease};
use crate::Result;
use serde::{Deserialize, Serialize};
use std::collections::BTreeMap;
use std::fs::File;
use std::path::PathBuf;
use std::time::{Duration, Instant};

fn d_poll() -> u64 {
    2000
}

/// The Whale side of its terminal: the profile path and how often to look for the answer.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct TerminalConfig {
    /// Absolute path of the `mova-terminal` profile JSON.
    pub profile: PathBuf,
    #[serde(default = "d_poll")]
    pub poll_ms: u64,
}

impl TerminalConfig {
    pub fn load(&self) -> Result<::terminal::Profile> {
        if !self.profile.is_absolute() || !(100..=60_000).contains(&self.poll_ms) {
            return Err(error("terminal.profile must be absolute and poll_ms 100..60000"));
        }
        ::terminal::Profile::load(&self.profile).map_err(|e| error(format!("terminal profile {}: {e}", self.profile.display())))
    }
}

/// The exclusive hold of one terminal by one Whale, for the Whale's whole run.
pub struct TerminalLease {
    pub animal: String,
    _file: File,
}

impl TerminalLease {
    /// Take `ROOT/holder.lock` of the profile; refused while another Whale holds it.
    pub fn take(p: &::terminal::Profile) -> Result<Self> {
        std::fs::create_dir_all(&p.root)?;
        let lock = p.root.join("holder.lock");
        match try_lease(&lock, true)? {
            Some(f) => Ok(Self { animal: p.animal.clone(), _file: f }),
            None => Err(error(format!("terminal {} is held by another Whale ({})", p.session, lock.display()))),
        }
    }
}

/// One question a Whale asks: about a term or a sentence it could not read, with the few
/// source sentences it needs. Never the draft, never "write the description".
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Question {
    pub id: String,
    /// `term` or `sentence`.
    pub kind: String,
    /// The term or the sentence asked about.
    pub subject: String,
    /// The full question text sent to the terminal (it includes the context).
    pub text: String,
    /// At most three source sentences, verbatim, for context.
    pub context: Vec<String>,
    /// A batched question (`kind` «batch»): the round's questions, numbered in this order in
    /// `text`; the answer carries their answers under the same numbers.
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub items: Vec<Question>,
}

/// A plain-text answer; the Whale keeps at most `ANSWER_MAX` characters.
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq)]
pub struct Answer {
    pub text: String,
    pub elapsed_ms: u64,
    /// The terminal's own question id (its journal), when there is one.
    pub terminal_id: Option<String>,
    /// Numbers (1-based) of batched items the terminal had no answer for (`Replay` only).
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub missing: Vec<usize>,
}

pub const ANSWER_MAX: usize = 4000;

/// Why a question got no answer. Each ends the Whale's consulting cleanly: no further question,
/// no retry. `Unavailable`: no terminal attached (dry run), the question is journaled only.
#[derive(Clone, Debug, PartialEq)]
pub enum AskError {
    /// The provider's usage or rate limit is on the screen; the question stays pending there.
    Limit(String),
    /// No answer in time; the question stays pending in the terminal and is never re-sent.
    Timeout(String),
    /// Nothing was sent (not idle, earlier question pending, lock held) or the terminal failed.
    Failed(String),
    Unavailable,
}

impl AskError {
    pub fn kind(&self) -> &'static str {
        match self {
            AskError::Limit(_) => "limit",
            AskError::Timeout(_) => "timeout",
            AskError::Failed(_) => "failed",
            AskError::Unavailable => "not_asked",
        }
    }
    pub fn reason(&self) -> String {
        match self {
            AskError::Limit(s) | AskError::Timeout(s) | AskError::Failed(s) => s.clone(),
            AskError::Unavailable => "no terminal is attached".into(),
        }
    }
}

/// The interface a terminal implements. `ask` blocks until one answer or one error; the Whale
/// calls it at most once per question and never after an error.
pub trait PrivateTerminal {
    /// The animal the terminal belongs to; must equal the Whale's name.
    fn animal(&self) -> &str;
    fn ask(&mut self, q: &Question) -> std::result::Result<Answer, AskError>;
}

/// Screen text of a provider's usage or rate limit (lowercase).
pub const LIMIT_TEXT: [&str; 5] = ["usage limit", "rate limit", "hit your limit", "limit reached", "too many requests"];

/// The real terminal: a thin adapter over `mova-terminal`.
pub struct MovaTerminal {
    pub profile: ::terminal::Profile,
    pub poll: Duration,
}

impl MovaTerminal {
    pub fn open(c: &TerminalConfig) -> Result<Self> {
        Ok(Self { profile: c.load()?, poll: Duration::from_millis(c.poll_ms) })
    }
}

impl PrivateTerminal for MovaTerminal {
    fn animal(&self) -> &str {
        &self.profile.animal
    }
    fn ask(&mut self, q: &Question) -> std::result::Result<Answer, AskError> {
        let start = Instant::now();
        let asked = ::terminal::ask(&self.profile, &q.text).map_err(|e| AskError::Failed(e.to_string()))?;
        let limit = Duration::from_secs(self.profile.answer_timeout_seconds);
        loop {
            // The answer counts once its file exists and the session is idle again: the LLM
            // writes the file and then prints the idle marker, and the next question needs an
            // idle pane (asking at once refused the next question as «not idle» on 2026-11-04).
            let observed = ::terminal::observe(&self.profile).ok();
            let idle = observed.as_ref().is_some_and(|o| o.state == ::terminal::State::Idle);
            if self.profile.answer_path(&asked.id).is_file() && idle {
                match ::terminal::answer(&self.profile, &asked.id) {
                    Ok(Some(text)) => return Ok(Answer { text, elapsed_ms: start.elapsed().as_millis() as u64, terminal_id: Some(asked.id), missing: vec![] }),
                    Ok(None) => {}
                    Err(e) => return Err(AskError::Failed(format!("{}: {e}", asked.id))),
                }
            }
            if let Some(o) = observed {
                let low = o.screen.to_lowercase();
                if let Some(t) = LIMIT_TEXT.iter().find(|t| low.contains(**t)) {
                    return Err(AskError::Limit(format!("{} pending: the screen shows «{t}»", asked.id)));
                }
            }
            if start.elapsed() > limit {
                return Err(AskError::Timeout(format!("no answer to {} within {}s; it stays pending and is never re-sent", asked.id, limit.as_secs())));
            }
            std::thread::sleep(self.poll);
        }
    }
}

/// Dry run: every question is journaled as not asked.
pub struct NoTerminal(pub String);
impl PrivateTerminal for NoTerminal {
    fn animal(&self) -> &str {
        &self.0
    }
    fn ask(&mut self, _q: &Question) -> std::result::Result<Answer, AskError> {
        Err(AskError::Unavailable)
    }
}

/// Test terminal: answers by subject from a table; optionally reports a usage limit after `n`
/// answers. It records every question it receives.
pub struct FakeTerminal {
    pub animal: String,
    pub answers: BTreeMap<String, String>,
    pub limit_after: Option<usize>,
    pub received: Vec<Question>,
}
impl PrivateTerminal for FakeTerminal {
    fn animal(&self) -> &str {
        &self.animal
    }
    fn ask(&mut self, q: &Question) -> std::result::Result<Answer, AskError> {
        self.received.push(q.clone());
        if self.limit_after.is_some_and(|n| self.received.len() > n) {
            return Err(AskError::Limit("fake usage limit reached".into()));
        }
        match self.answers.get(&q.subject) {
            Some(a) => Ok(Answer { text: a.clone(), elapsed_ms: 1, terminal_id: None, missing: vec![] }),
            None => Err(AskError::Failed("the fake terminal has no answer".into())),
        }
    }
}

/// Measurement terminal: replays answers a pod's terminals really gave, by question (a term by its
/// normalized form, a sentence or part by its text), from earlier state roots
/// (`papers/*/round-*/questions.jsonl` with `knowledge/answers/SHA.txt`) and `paper` run roots
/// (`questions.jsonl` with the answer inline). A batched question gets its recorded answers under
/// their numbers; items without a recorded answer are reported in `Answer::missing`. It never
/// starts or asks anything.
pub struct Replay {
    pub animal: String,
    pub answers: BTreeMap<String, String>,
    pub asked: usize,
    pub found: usize,
}

impl Replay {
    fn key(kind: &str, subject: &str) -> String {
        if kind == "term" {
            format!("term:{}", crate::knowledge::norm_term(subject))
        } else {
            format!("{kind}:{subject}")
        }
    }
    /// Load recorded answers from earlier state roots and `paper` run roots.
    pub fn load(animal: &str, roots: &[PathBuf]) -> Result<Self> {
        let mut answers = BTreeMap::new();
        let mut add = |o: &serde_json::Value, text: String| {
            let (kind, subject) = (o["question"]["kind"].as_str().unwrap_or(""), o["question"]["subject"].as_str().unwrap_or(""));
            if o["state"] == "answered" && !subject.is_empty() && !text.trim().is_empty() {
                answers.entry(Self::key(kind, subject)).or_insert(text);
            }
        };
        for root in roots {
            let mut files: Vec<(PathBuf, Option<PathBuf>)> = vec![];
            if root.join("questions.jsonl").is_file() {
                files.push((root.join("questions.jsonl"), None));
            }
            let papers = root.join("papers");
            if papers.is_dir() {
                let mut ps: Vec<PathBuf> = std::fs::read_dir(&papers)?.filter_map(|e| e.ok().map(|e| e.path())).collect();
                ps.sort();
                for p in ps {
                    let mut rs: Vec<PathBuf> = std::fs::read_dir(&p)?.filter_map(|e| e.ok().map(|e| e.path())).filter(|d| d.file_name().and_then(|n| n.to_str()).is_some_and(|n| n.starts_with("round-"))).collect();
                    rs.sort();
                    files.extend(rs.into_iter().map(|r| (r.join("questions.jsonl"), Some(root.join("knowledge/answers")))));
                }
            }
            for (f, answers_dir) in files {
                let Ok(text) = std::fs::read_to_string(&f) else { continue };
                for line in text.lines().filter(|l| !l.trim().is_empty()) {
                    let o: serde_json::Value = serde_json::from_str(line)?;
                    let a = match (&answers_dir, o["answer"].as_str(), o["answer_sha256"].as_str()) {
                        (_, Some(a), _) => a.to_string(),
                        (Some(d), None, Some(sha)) => std::fs::read_to_string(d.join(format!("{sha}.txt"))).unwrap_or_default(),
                        _ => String::new(),
                    };
                    add(&o, a);
                }
            }
        }
        Ok(Self { animal: animal.into(), answers, asked: 0, found: 0 })
    }
}

impl PrivateTerminal for Replay {
    fn animal(&self) -> &str {
        &self.animal
    }
    fn ask(&mut self, q: &Question) -> std::result::Result<Answer, AskError> {
        let items: Vec<&Question> = if q.items.is_empty() { vec![q] } else { q.items.iter().collect() };
        let mut text = String::new();
        let mut missing = vec![];
        for (n, it) in items.iter().enumerate() {
            self.asked += 1;
            match self.answers.get(&Self::key(&it.kind, &it.subject)) {
                Some(a) => {
                    self.found += 1;
                    text.push_str(&format!("{}. {}\n", n + 1, a.split_whitespace().collect::<Vec<_>>().join(" ")));
                }
                None => {
                    missing.push(n + 1);
                    text.push_str(&format!("{}. I have no recorded answer.\n", n + 1));
                }
            }
        }
        Ok(Answer { text, elapsed_ms: 0, terminal_id: Some("replay".into()), missing })
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use std::path::Path;
    /// A Sol-shaped profile for `animal` under `dir` (never started in tests).
    pub fn profile(dir: &Path, animal: &str) -> TerminalConfig {
        let p = dir.join(format!("{animal}-terminal.json"));
        let v = serde_json::json!({"animal":animal,"session":format!("{animal}-sol"),"tmux":"/usr/bin/tmux","pane_command":"node",
            "owner_marker":format!("mova-{animal}-sol"),"idle_marker":"TERMINAL READY","idle_window":6,"root":dir.join(format!("{animal}-terminal"))});
        std::fs::write(&p, serde_json::to_vec_pretty(&v).unwrap()).unwrap();
        TerminalConfig { profile: p, poll_ms: 2000 }
    }
    #[test]
    fn one_terminal_one_holder_and_config_gates() {
        let dir = std::env::var_os("CARGO_TARGET_DIR").map(PathBuf::from).unwrap_or_else(std::env::temp_dir).join(format!("whale-terminal-{}", std::process::id()));
        std::fs::create_dir_all(&dir).unwrap();
        let c = profile(&dir, "whale-1");
        let p = c.load().unwrap();
        let held = TerminalLease::take(&p).unwrap();
        // A second Whale naming the same terminal is refused while the first holds it.
        assert!(TerminalLease::take(&p).err().unwrap().to_string().contains("held by another Whale"));
        drop(held);
        assert!(TerminalLease::take(&p).is_ok());
        let mut bad = c.clone();
        bad.poll_ms = 10;
        assert!(bad.load().is_err());
        let mut rel = c.clone();
        rel.profile = PathBuf::from("relative.json");
        assert!(rel.load().is_err());
        let _ = std::fs::remove_dir_all(dir);
    }
}
