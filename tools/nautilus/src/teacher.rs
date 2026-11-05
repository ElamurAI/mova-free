//! Teacher terminals for the animals that make Mova's teaching material: 🐚 Nautilus (dictionaries) and 🐢 Sea
//! Turtle (questions, lessons, exams). The animals work natively first; a teacher answers only what they cannot.
//!
//! **The interface.** A teacher is reached only through [`PrivateTerminal`]: `id()` names the terminal (one per
//! animal instance, its own session and lock, never shared and never a Dolphin) and `ask(question)` returns the
//! terminal's whole answer as text, or a [`TerminalError`] — `Limit` when the provider's usage or rate limit is hit,
//! `Failed` otherwise. [`MovaTerminal`] is the thin adapter over `mova-terminal` (`tools/terminal`: its own lock,
//! journal and no-replay rule; `ask_wait`); [`Unplugged`] and the fakes of the tests stand in where no terminal is
//! held.
//!
//! **Profiles are configuration.** A profile names the terminal id it expects (the `animal` of its `mova-terminal`
//! profile), that profile's path, the provider and model labels (preferences, not attestations: public default Opus
//! through the `claude` CLI; Sol through Codex is the recommended alternative, and ours) and the purposes it serves
//! (`dictionary`, `questions`).
//!
//! **The rules of `submit`.** Refused unless the configuration carries `operator_word`, the profile serves the
//! purpose, the plugged terminal is that profile's own (`id()` equals the profile's `terminal`), and the question
//! is bounded plain text. Every question is journaled before the call (`reserved`, fsynced) and after it
//! (`answered` with the answer's hash, `limit` or `failed`). A question whose hash is already in the journal is
//! refused: a reservation without an outcome is `uncertain` and is never replayed, and nothing is retried blindly.
//! The answer is a review-only record; nothing is admitted or executed by itself.
use anyhow::{Context, Result, bail, ensure};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use std::fs::OpenOptions;
use std::io::Write;
use std::path::{Path, PathBuf};

/// The bound on a question's text: under the 100 000 bytes `mova-terminal` takes, with room for its wrapper.
pub const TASK_MAX: usize = 99_000;
/// The bound on an answer kept in the journal's record.
pub const ANSWER_MAX: usize = 256 * 1024;
pub const PURPOSES: [&str; 2] = ["dictionary", "questions"];
pub const PROVIDERS: [&str; 2] = ["claude-cli", "codex"];

/// Why a terminal gave no answer.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum TerminalError {
    /// the provider's usage or rate limit: the caller stops cleanly and waits for a person to resume
    Limit(String),
    /// anything else; never retried blindly
    Failed(String),
}

/// An animal instance's own teacher terminal.
pub trait PrivateTerminal {
    /// The terminal's id: one per animal instance (its own session and lock).
    fn id(&self) -> &str;
    /// Ask one question; the whole answer as text.
    fn ask(&self, question: &str) -> std::result::Result<String, TerminalError>;
    /// Whether a question could be sent now (held, owned, idle). `false` means nothing is asked and nothing is
    /// journaled: the caller waits and tries the same item later.
    fn ready(&self) -> bool {
        true
    }
}

/// No terminal plugged in: every question fails, so work that needs a teacher waits.
pub struct Unplugged(pub String);

impl PrivateTerminal for Unplugged {
    fn id(&self) -> &str {
        &self.0
    }
    fn ask(&self, _: &str) -> std::result::Result<String, TerminalError> {
        Err(TerminalError::Failed("no teacher terminal is plugged in".into()))
    }
}

/// Screen or error text of a provider's usage or rate limit.
pub const LIMIT: [&str; 6] = ["usage limit", "rate limit", "limit reached", "hit your limit", "too many requests", "try again later"];

pub fn limited(text: &str) -> bool {
    let low = text.to_lowercase();
    LIMIT.iter().any(|l| low.contains(l))
}

/// Error text of a terminal that was not there to take the question (no tmux session or server): nothing was sent.
pub const UNSENT: [&str; 3] = ["no such session", "no server running", "can't find session"];

pub fn unsent(text: &str) -> bool {
    let low = text.to_lowercase();
    UNSENT.iter().any(|l| low.contains(l))
}

/// The thin adapter over `mova-terminal`: one held terminal of one animal instance. `ask` is
/// `terminal::ask_wait` (the terminal's lock, its idle and unchanged-screen checks, no replay while a question is
/// pending, the answer file); an error that names a usage or rate limit is `Limit`, any other `Failed`. A timeout
/// leaves the question pending in the terminal's journal, so the next `ask` is refused until it is answered or an
/// operator abandons it.
pub struct MovaTerminal {
    pub profile: terminal::Profile,
    pub poll: std::time::Duration,
}

impl MovaTerminal {
    pub fn load(path: &Path) -> Result<MovaTerminal> {
        let profile = terminal::Profile::load(path).map_err(|e| anyhow::anyhow!("{}: {e}", path.display()))?;
        Ok(MovaTerminal { profile, poll: std::time::Duration::from_secs(5) })
    }
}

impl PrivateTerminal for MovaTerminal {
    fn id(&self) -> &str {
        &self.profile.animal
    }
    fn ready(&self) -> bool {
        matches!(terminal::observe(&self.profile), Ok(o) if o.state == terminal::State::Idle)
            && matches!(terminal::pending(&self.profile), Ok(None))
    }
    fn ask(&self, question: &str) -> std::result::Result<String, TerminalError> {
        match terminal::ask_wait(&self.profile, question, self.poll) {
            Ok((_, text)) => Ok(text),
            Err(e) if limited(&e.to_string()) => Err(TerminalError::Limit(e.to_string())),
            Err(e) => Err(TerminalError::Failed(e.to_string())),
        }
    }
}

/// The private terminal of a profile: its `mova-terminal` profile loaded, else `Unplugged` with the reason.
pub fn plug(p: &Profile) -> Box<dyn PrivateTerminal> {
    match MovaTerminal::load(&p.terminal_profile) {
        Ok(t) => Box::new(t),
        Err(_) => Box::new(Unplugged(p.terminal.clone())),
    }
}

#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Profile {
    pub name: String,
    pub provider: String,
    pub model: String,
    /// the id of this animal instance's own terminal (the `animal` of its `mova-terminal` profile)
    pub terminal: String,
    /// that terminal's `mova-terminal` profile (its session, owner marker, lock and journal root)
    pub terminal_profile: PathBuf,
    pub purposes: Vec<String>,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    /// the animal's teacher journal (JSONL)
    pub journal: PathBuf,
    pub profiles: Vec<Profile>,
    /// The operator's word that allows teacher questions, as text ("<date>, <who>: allowed for the <layer>
    /// layer"); absent: every question is refused.
    #[serde(default)]
    pub operator_word: Option<String>,
}

pub fn ident(s: &str) -> bool {
    !s.is_empty() && s.len() <= 64 && s.bytes().all(|b| b.is_ascii_alphanumeric() || b"-_.".contains(&b))
}

impl Config {
    pub fn validate(&self) -> Result<()> {
        ensure!(self.journal.is_absolute(), "absolute journal required");
        ensure!(!self.profiles.is_empty() && self.profiles.len() <= 8, "1..8 teacher profiles");
        let (mut names, mut terminals, mut locks) = (std::collections::BTreeSet::new(), std::collections::BTreeSet::new(), std::collections::BTreeSet::new());
        for p in &self.profiles {
            ensure!(ident(&p.name) && ident(&p.model) && ident(&p.terminal), "{}: names are ASCII identifiers", p.name);
            ensure!(PROVIDERS.contains(&p.provider.as_str()), "{}: provider is claude-cli or codex", p.name);
            ensure!(!p.purposes.is_empty() && p.purposes.iter().all(|x| PURPOSES.contains(&x.as_str())), "{}: purposes are dictionary or questions", p.name);
            ensure!(p.terminal_profile.is_absolute(), "{}: absolute terminal profile required", p.name);
            ensure!(names.insert(p.name.clone()), "{}: profile named twice", p.name);
            ensure!(terminals.insert(p.terminal.clone()) && locks.insert(p.terminal_profile.clone()), "{}: a terminal is never shared between profiles", p.name);
        }
        Ok(())
    }
    pub fn profile(&self, name: &str) -> Option<&Profile> {
        self.profiles.iter().find(|p| p.name == name)
    }
}

/// Plain text a terminal can read: non-empty, bounded, no control characters but newline and tab.
pub fn valid_task(t: &str) -> bool {
    !t.trim().is_empty() && t.len() <= TASK_MAX && !t.chars().any(|c| c.is_control() && !matches!(c, '\n' | '\t'))
}

/// The journal's lines.
pub fn journal(path: &Path) -> Result<Vec<Value>> {
    match std::fs::read_to_string(path) {
        Ok(t) => Ok(t.lines().filter_map(|l| serde_json::from_str(l).ok()).collect()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(vec![]),
        Err(e) => Err(e).with_context(|| format!("{}", path.display())),
    }
}

fn append(path: &Path, v: &Value) -> Result<()> {
    if let Some(d) = path.parent() {
        std::fs::create_dir_all(d)?;
    }
    let mut f = OpenOptions::new().create(true).append(true).open(path)?;
    f.write_all(format!("{v}\n").as_bytes())?;
    f.sync_data()?;
    Ok(())
}

/// Each question's state by its hash: `answered`, `limit`, `failed` or `uncertain` (reserved, no outcome).
pub fn states(lines: &[Value]) -> std::collections::BTreeMap<String, String> {
    let mut m = std::collections::BTreeMap::new();
    for l in lines {
        let h = l["task_sha256"].as_str().unwrap_or("").to_string();
        let s = match l["event"].as_str() {
            Some("reserved") => "uncertain",
            Some(e @ ("answered" | "limit" | "failed")) => e,
            _ => continue,
        };
        m.insert(h, s.to_string());
    }
    m
}

/// Ask the animal's own teacher terminal one question, journaled; see the module's rules. `Ok` is
/// `{"state":"answered","text":…}` or `{"state":"limit",…}`; refusals and failures are `Err`.
pub fn submit(cfg: &Config, t: &dyn PrivateTerminal, profile: &str, purpose: &str, task: &str, id: &str, now: u64) -> Result<Value> {
    cfg.validate()?;
    let Some(word) = cfg.operator_word.as_deref().filter(|w| !w.trim().is_empty()) else {
        bail!("the operator has not given the word for teacher questions");
    };
    ensure!(ident(id), "the question id is an ASCII identifier");
    let p = cfg.profile(profile).with_context(|| format!("no teacher profile {profile}"))?;
    ensure!(p.purposes.iter().any(|x| x == purpose), "{profile} does not serve {purpose}");
    ensure!(t.id() == p.terminal, "the terminal {} is not this profile's own ({})", t.id(), p.terminal);
    ensure!(valid_task(task), "the question is empty, too long or has control characters");
    let h = mova_octopus::digest(task.as_bytes());
    let lines = journal(&cfg.journal)?;
    if let Some(s) = states(&lines).get(&h) {
        // a question that never reached a terminal (no session) may be asked once more
        let mine: Vec<&Value> = lines.iter().filter(|l| l["task_sha256"] == h.as_str()).collect();
        let unsent_once = s == "failed"
            && mine.iter().filter(|l| l["event"] == "failed").count() == 1
            && mine.iter().any(|l| l["event"] == "failed" && l["error"].as_str().is_some_and(unsent));
        if !unsent_once {
            bail!("this question is already in the journal as {s}; it is not asked again");
        }
    }
    if !t.ready() {
        // Nothing sent, nothing journaled: the terminal is not held yet, busy, or still owes an answer.
        return Ok(json!({"state":"unavailable","id":id,"reason":"the terminal is not ready; nothing was asked"}));
    }
    append(&cfg.journal, &json!({"event":"reserved","id":id,"task_sha256":h,"profile":p.name,"terminal":p.terminal,
        "provider":p.provider,"model":p.model,"purpose":purpose,"operator_word":word,"unix":now}))?;
    match t.ask(task) {
        Ok(text) if text.len() <= ANSWER_MAX => {
            let a = mova_octopus::digest(text.as_bytes());
            append(&cfg.journal, &json!({"event":"answered","id":id,"task_sha256":h,"answer_sha256":a,"unix":now}))?;
            Ok(json!({"state":"answered","id":id,"task_sha256":h,"answer_sha256":a,"text":text,"admitted":false}))
        }
        Ok(text) => {
            append(&cfg.journal, &json!({"event":"failed","id":id,"task_sha256":h,"error":"answer over the bound","bytes":text.len(),"unix":now}))?;
            bail!("the answer is over {ANSWER_MAX} bytes; it is not kept")
        }
        Err(TerminalError::Limit(why)) => {
            append(&cfg.journal, &json!({"event":"limit","id":id,"task_sha256":h,"reason":why,"unix":now}))?;
            Ok(json!({"state":"limit","id":id,"reason":why}))
        }
        Err(TerminalError::Failed(why)) => {
            append(&cfg.journal, &json!({"event":"failed","id":id,"task_sha256":h,"error":why,"unix":now}))?;
            bail!("the terminal did not answer: {why}; it is not retried")
        }
    }
}

/// A dictionary question for a teacher on draft rows (word, part of speech, the pupils' meaning, examples), as the
/// standing loop asks it (`work::draft_task`) without the sense store: the words Mova does not settle natively, at
/// most `work::BATCH`. Returns the question (empty when nothing is asked) and how many words it holds.
pub fn dictionary_task(name: &str, rows: &[crate::layer::Row]) -> (String, usize) {
    let plans: Vec<crate::senses::WordPlan> = rows
        .iter()
        .map(|r| crate::senses::plan(&r.word, &r.examples, 2, crate::senses::parse(&r.meaning), None, true))
        .filter(|p| p.fate == crate::senses::Fate::Ask)
        .take(crate::work::BATCH)
        .collect();
    let refs: Vec<&crate::senses::WordPlan> = plans.iter().collect();
    let t = crate::work::draft_task(name, &refs).unwrap_or_default();
    let n = plans.iter().filter(|p| t.contains(&format!("word: {}\n", p.word))).count();
    (t, n)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    /// A fake private terminal: answers from a script, records what it was asked.
    pub(crate) struct Fake {
        pub id: String,
        pub replies: RefCell<Vec<std::result::Result<String, TerminalError>>>,
        pub asked: RefCell<Vec<String>>,
    }
    impl Fake {
        pub fn new(id: &str, replies: Vec<std::result::Result<String, TerminalError>>) -> Fake {
            Fake { id: id.into(), replies: RefCell::new(replies), asked: RefCell::new(vec![]) }
        }
    }
    impl PrivateTerminal for Fake {
        fn id(&self) -> &str {
            &self.id
        }
        fn ask(&self, q: &str) -> std::result::Result<String, TerminalError> {
            self.asked.borrow_mut().push(q.into());
            let mut r = self.replies.borrow_mut();
            if r.is_empty() { Ok("an answer".into()) } else { r.remove(0) }
        }
    }
    fn profile(name: &str, purpose: &str, dir: &Path) -> Profile {
        Profile { name: name.into(), provider: "claude-cli".into(), model: "opus".into(), terminal: format!("nautilus-{name}"), terminal_profile: dir.join(format!("{name}-terminal.json")), purposes: vec![purpose.into()] }
    }
    fn cfg(dir: &Path, word: Option<&str>) -> Config {
        Config { journal: dir.join("teacher.jsonl"), profiles: vec![profile("opus", "dictionary", dir), profile("sol", "dictionary", dir), profile("qopus", "questions", dir)], operator_word: word.map(String::from) }
    }
    fn dir(name: &str) -> PathBuf {
        let d = std::env::temp_dir().join(format!("nautilus-teacher-{name}-{}-{}", std::process::id(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        std::fs::create_dir_all(&d).unwrap();
        d
    }
    #[test]
    fn no_word_no_question() {
        let d = dir("word");
        let f = Fake::new("nautilus-opus", vec![]);
        let e = submit(&cfg(&d, None), &f, "opus", "dictionary", "Write senses.", "t1", 0).unwrap_err();
        assert!(e.to_string().contains("operator has not given the word"));
        assert!(f.asked.borrow().is_empty());
        assert!(journal(&d.join("teacher.jsonl")).unwrap().is_empty());
        assert!(submit(&cfg(&d, Some("  ")), &f, "opus", "dictionary", "Write senses.", "t1", 0).is_err());
        assert!(Unplugged("x".into()).ask("q").is_err());
    }
    #[test]
    fn journaled_once_never_replayed() {
        let d = dir("once");
        let c = cfg(&d, Some("test: the operator allowed it"));
        let f = Fake::new("nautilus-opus", vec![Ok("pass\tto hand over".into())]);
        let v = submit(&c, &f, "opus", "dictionary", "Write senses.", "t1", 7).unwrap();
        assert_eq!((v["state"].as_str(), v["text"].as_str(), v["admitted"].as_bool()), (Some("answered"), Some("pass\tto hand over"), Some(false)));
        assert_eq!(f.asked.borrow().as_slice(), ["Write senses."]);
        // the same question again is refused, even with another id or profile
        let f2 = Fake::new("nautilus-sol", vec![]);
        assert!(submit(&c, &f2, "sol", "dictionary", "Write senses.", "t2", 8).unwrap_err().to_string().contains("as answered"));
        // a profile that does not serve the purpose, an unknown one, another's terminal, a bad question
        assert!(submit(&c, &f, "qopus", "dictionary", "Other.", "t3", 9).is_err());
        assert!(submit(&c, &f, "haiku", "dictionary", "Other.", "t3", 9).is_err());
        assert!(submit(&c, &f2, "opus", "dictionary", "Other.", "t3", 9).unwrap_err().to_string().contains("not this profile's own"));
        assert!(submit(&c, &f, "opus", "dictionary", "bad\u{7}", "t3", 9).is_err());
        assert!(submit(&c, &f, "opus", "dictionary", &"x".repeat(TASK_MAX + 1), "t3", 9).is_err());
        assert_eq!(f.asked.borrow().len() + f2.asked.borrow().len(), 1);
    }
    #[test]
    fn limits_failures_and_uncertainty_are_not_retried() {
        let d = dir("fail");
        let c = cfg(&d, Some("test"));
        let f = Fake::new("nautilus-qopus", vec![Err(TerminalError::Limit("usage limit".into())), Err(TerminalError::Failed("crashed".into()))]);
        assert_eq!(submit(&c, &f, "qopus", "questions", "One.", "t1", 1).unwrap()["state"], "limit");
        assert!(submit(&c, &f, "qopus", "questions", "Two.", "t2", 2).is_err());
        for (q, s) in [("One.", "as limit"), ("Two.", "as failed")] {
            assert!(submit(&c, &f, "qopus", "questions", q, "t3", 3).unwrap_err().to_string().contains(s));
        }
        // a reservation without an outcome (a crash during the question) is uncertain and refused
        append(&c.journal, &json!({"event":"reserved","task_sha256":mova_octopus::digest(b"Lost."),"unix":3})).unwrap();
        assert!(submit(&c, &f, "qopus", "questions", "Lost.", "t4", 4).unwrap_err().to_string().contains("as uncertain"));
        assert_eq!(f.asked.borrow().len(), 2);
    }
    #[test]
    fn an_unsent_question_may_be_asked_once_more() {
        let d = dir("unsent");
        let c = cfg(&d, Some("test"));
        let gone = || Err(TerminalError::Failed("terminal not idle: no such session".into()));
        let f = Fake::new("nautilus-opus", vec![gone(), gone(), Err(TerminalError::Failed("crashed".into()))]);
        assert!(submit(&c, &f, "opus", "dictionary", "One.", "t1", 1).is_err());
        assert!(submit(&c, &f, "opus", "dictionary", "One.", "t2", 2).is_err(), "asked once more");
        assert!(submit(&c, &f, "opus", "dictionary", "One.", "t3", 3).unwrap_err().to_string().contains("as failed"), "not a third time");
        assert_eq!(f.asked.borrow().len(), 2);
        // red control: another failure is never asked again
        assert!(submit(&c, &f, "opus", "dictionary", "Two.", "t4", 4).is_err());
        assert!(submit(&c, &f, "opus", "dictionary", "Two.", "t5", 5).unwrap_err().to_string().contains("as failed"));
        assert_eq!(f.asked.borrow().len(), 3);
        assert!(unsent("Error: can't find session: x") && !unsent("crashed"));
    }
    #[test]
    fn the_mova_terminal_adapter() {
        let d = dir("adapter");
        let prof = json!({"animal":"nautilus-1","session":"nautilus-1-sol","tmux":"/bin/false","pane_command":"node",
            "owner_marker":"mova-nautilus-1","idle_marker":"TERMINAL READY","root":d.join("terminal")});
        std::fs::write(d.join("t.json"), prof.to_string()).unwrap();
        let mut p = profile("sol", "dictionary", &d);
        p.terminal = "nautilus-1".into();
        p.terminal_profile = d.join("t.json");
        let t = plug(&p);
        assert_eq!(t.id(), "nautilus-1");
        // no tmux server answers here (/bin/false): the question fails before anything is sent, and is not a limit
        assert!(matches!(t.ask("Q?"), Err(TerminalError::Failed(_))));
        // an unreadable profile leaves the terminal unplugged
        p.terminal_profile = d.join("absent.json");
        assert!(matches!(plug(&p).ask("Q?"), Err(TerminalError::Failed(m)) if m.contains("no teacher terminal")));
        assert!(limited("Error: You've hit your usage limit.") && !limited("tmux observe failed"));
    }
    #[test]
    fn profiles_validate_and_tasks_are_bounded() {
        let d = dir("cfg");
        let mut c = cfg(&d, None);
        assert!(c.validate().is_ok());
        c.profiles[0].provider = "api".into();
        assert!(c.validate().is_err());
        let mut c = cfg(&d, None);
        c.profiles[1].purposes = vec!["training".into()];
        assert!(c.validate().is_err());
        // one terminal (or lock) is never shared between two profiles
        let mut c = cfg(&d, None);
        c.profiles[1].terminal = c.profiles[0].terminal.clone();
        assert!(c.validate().unwrap_err().to_string().contains("never shared"));
        let mut c = cfg(&d, None);
        c.profiles[1].terminal_profile = c.profiles[0].terminal_profile.clone();
        assert!(c.validate().is_err());
        // a draft question holds a batch of words, well within the bound
        let rows: Vec<crate::layer::Row> = (0..200)
            .map(|i| {
                let w = format!("pas{}{}", (b'a' + (i / 26) as u8) as char, (b'a' + (i % 26) as u8) as char);
                let long = "and so the long sentence goes on ".repeat(12);
                crate::layer::Row { word: w.clone(), pos: "verb".into(), meaning: "1) To give. 2) To go by.".into(), sense: "Counted.".into(),
                    examples: vec![format!("{long} {w} it. «a:{i}»"), format!("Do {w} it {long}. «b:{i}»")], note: String::new() }
            })
            .collect();
        let (t, n) = dictionary_task("the commands dictionary", &rows);
        assert!(valid_task(&t) && n == crate::work::BATCH && t.len() < 20_000, "{n} words, {} bytes", t.len());
        assert!(t.contains("examples: …") && !t.contains("counted"), "examples are cut around the word");
    }
}
