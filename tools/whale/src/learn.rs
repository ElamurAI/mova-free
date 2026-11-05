//! One Whale learns on one paper, round after round, until its own description passes the gates.
//!
//! State model (all under the Whale's private `state_root`, which outlives papers and batches):
//!
//! - `knowledge/` — the Whale's retained knowledge (`knowledge.rs`): `items.jsonl`, `asked.jsonl`,
//!   `answers/`. Shared by every round and every later paper of this Whale.
//! - `papers/<id>/` — one directory per paper: `journal.jsonl` (fsynced round events),
//!   `status.json` (`learning`, `passed` or `stopped` with its reason, first-start time and
//!   deadline), and `round-NNN/` with `description.txt`, `draft.json`, `gates.json`,
//!   `questions.jsonl`, `teaching.jsonl` and `metrics.json`.
//! - `learner.lock` — one learner per Whale at a time.
//!
//! Round k: read (Mova) -> write (`compose`, with all retained knowledge) -> gates -> pass: stop.
//! Otherwise at most `max_questions` questions about gaps never asked before — unknown terms the
//! Whale has no facts for (acronyms first, by spread), sentences it could not restate (longest
//! first), parts it could not restate at all — each asked once through its own terminal; every
//! answer is re-read by Mova and retained as facts with provenance (never as text for the
//! description); the next round starts. Ends: `passed`; `limit`, `timeout` or `failed` at the
//! terminal (no retry; a later invocation continues with new questions only); `no_terminal` (dry
//! run); `nothing_to_ask` (every gap asked: the writer, not the knowledge, is the limit);
//! `stalled` (`stall_rounds` rounds in a row without a word gained or a usable fact learned);
//! `round_limit`; `deadline` (half a day from the first round by default); `stop` (the `stop_file`
//! appeared: not final, a later invocation continues).
//!
//! A round's questions go to the terminal in one call (`batched`, the default): numbered, each
//! asking for an answer in a typed shape built from the source's own words («X stands for Y.»,
//! «X is a Y.», «X does Y.», «In the paper's words: …»), and the answer is split by number. With
//! a pod (`pod`), terms are counted under their normalized form across all Whales of the pod: a
//! term one Whale asked is not asked by another, and what one learned the others explain with
//! (`pod.rs`).
use crate::{
    compose::{self, Known},
    gates::{self, Guard, Params},
    knowledge::{self, norm_term, question_key, Item as KItem, Store},
    pod::Pod,
    paper::{open_licence, PaperSource, Words},
    reader::{self, Reading},
    terminal::{AskError, PrivateTerminal, Question, TerminalConfig, TerminalLease, ANSWER_MAX},
    util::{error, file_hash, hash, now, private_dir, save, try_lease, write_sync},
    Result,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::{
    fs::OpenOptions,
    io::Write,
    os::unix::fs::OpenOptionsExt,
    path::{Path, PathBuf},
};

pub const SCHEMA: &str = "mova-shark-learn-v1";

/// Stop reasons no further invocation can change under the same writer: every gap was asked, the
/// rounds stopped gaining, or the round limit was reached. A paper stopped on one of them is not
/// reopened under the same writer version (`compose::WRITER`).
pub const EXHAUSTED: [&str; 3] = ["nothing_to_ask", "round_limit", "stalled"];

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LearnConfig {
    pub schema: String,
    #[serde(alias = "shark")]
    pub whale: String,
    /// The Whale's private state (knowledge and per-paper rounds); kept across papers.
    pub state_root: PathBuf,
    pub paper: PaperSource,
    pub words: Words,
    #[serde(default = "d_questions")]
    pub max_questions: usize,
    #[serde(default = "d_rounds")]
    pub max_rounds: usize,
    /// Hours from the first round of this paper after which no new round starts (half a day).
    #[serde(default = "d_hours")]
    pub hours: f64,
    /// An absolute deadline (Unix seconds) that overrides `hours` when earlier.
    #[serde(default)]
    pub deadline_unix: Option<u64>,
    #[serde(default)]
    pub gates: Params,
    /// Earlier `mova-whale paper` run roots of this Whale: their answered questions become
    /// retained knowledge and every question they sent counts as asked.
    #[serde(default)]
    pub import: Vec<PathBuf>,
    pub terminal: TerminalConfig,
    /// Rounds in a row without a word gained and without a usable fact learned after which the
    /// paper stops (`stalled`).
    #[serde(default = "d_stall")]
    pub stall_rounds: usize,
    /// The pod's shared knowledge directory (`pod.rs`); none: this Whale's own knowledge only.
    #[serde(default)]
    pub pod: Option<PathBuf>,
    /// One terminal call per round carrying all its questions (default), or one call per question.
    #[serde(default = "d_batched")]
    pub batched: bool,
    /// Review hints from the Whale's own terminal (`review.rs`).
    #[serde(default)]
    pub review: crate::review::ReviewConfig,
    /// The paper's closed topic corpus (`topic.rs`): seed before each round's questions, a cited
    /// description per round. None: not used.
    #[serde(default)]
    pub topic_corpus: Option<crate::topic::TopicCorpus>,
    /// A file whose presence stops the learner before its next round (`stop`, not final: a later
    /// invocation continues the paper). A batch sets its run root's `STOP`.
    #[serde(default)]
    pub stop_file: Option<PathBuf>,
}
fn d_stall() -> usize {
    3
}
fn d_batched() -> bool {
    true
}
fn d_questions() -> usize {
    4
}
fn d_rounds() -> usize {
    24
}
fn d_hours() -> f64 {
    12.0
}

impl LearnConfig {
    pub fn load(p: &Path) -> Result<Self> {
        let c: LearnConfig = serde_json::from_value(crate::util::load(p, 1 << 20)?)?;
        c.validate()?;
        Ok(c)
    }
    pub fn validate(&self) -> Result<()> {
        if self.schema != SCHEMA || !crate::util::label(&self.whale) || !self.state_root.is_absolute() || !self.paper.path.is_absolute() {
            return Err(error("learn config: schema, whale label, absolute state_root and paper.path required"));
        }
        if !crate::util::valid_hash(&self.paper.sha256) || self.paper.id.is_empty() || self.paper.licence.is_empty() {
            return Err(error("learn config: paper id, licence and SHA-256 required"));
        }
        if self.words.min == 0 || self.words.min > self.words.max || !(1..=16).contains(&self.max_questions) || !(1..=200).contains(&self.max_rounds) || !(self.hours > 0.0 && self.hours <= 168.0) {
            return Err(error("learn config: 0 < words.min <= words.max, max_questions 1..16, max_rounds 1..200, 0 < hours <= 168"));
        }
        if self.import.iter().any(|p| !p.is_absolute()) || self.pod.as_ref().is_some_and(|p| !p.is_absolute()) || self.stop_file.as_ref().is_some_and(|p| !p.is_absolute()) {
            return Err(error("learn config: import, pod and stop_file paths must be absolute"));
        }
        if !(1..=50).contains(&self.stall_rounds) {
            return Err(error("learn config: stall_rounds 1..50"));
        }
        self.review.validate()?;
        if let Some(t) = &self.topic_corpus {
            t.validate()?;
        }
        self.gates.validate()?;
        let p = self.terminal.load()?;
        if p.animal != self.whale {
            return Err(error(format!("learn config: terminal profile belongs to {}, not {}", p.animal, self.whale)));
        }
        Ok(())
    }
    pub fn paper_dir(&self) -> PathBuf {
        let safe: String = self.paper.id.chars().map(|c| if c.is_ascii_alphanumeric() || c == '-' || c == '.' { c } else { '_' }).take(96).collect();
        self.state_root.join("papers").join(safe)
    }
}

struct Journal(std::fs::File);
impl Journal {
    fn open(p: &Path) -> Result<Self> {
        Ok(Self(OpenOptions::new().create(true).append(true).mode(0o600).open(p)?))
    }
    fn log(&mut self, v: Value) -> Result<()> {
        let mut line = v.to_string();
        line.push('\n');
        self.0.write_all(line.as_bytes())?;
        self.0.sync_data()?;
        Ok(())
    }
}

/// One round's questions as one numbered request; each answer is asked for in a typed shape that
/// Mova's answer reader keeps (`knowledge::term_facts`, `knowledge::reading_facts`).
fn batch_text(items: &[Question]) -> String {
    let mut t = format!(
        "I am a Whale of Mova reading a scientific paper. I have {} questions about it. Answer each under its own number (1., 2., …), in plain English, using only these sentence shapes, so that I can read your answers:\n- about a term: one to three short sentences that each start with the term itself: «X stands for …» (only for an abbreviation), «X is a …», «X does …»;\n- about a sentence or a part: «In the paper's words: …» followed by one to three simple sentences of the form «subject verb object» that reuse the paper's own words from that sentence or part.\nDo not describe or summarize the paper. Do not add anything else.\n",
        items.len()
    );
    for (n, q) in items.iter().enumerate() {
        let ctx = q.context.iter().map(|c| format!("   - {c}")).collect::<Vec<_>>().join("\n");
        let body = match q.kind.as_str() {
            "term" => format!("The term «{}». Context from the paper:\n{ctx}", q.subject),
            "section" => format!("The part «{}». Its first sentences:\n{ctx}", q.subject),
            _ => format!("The sentence «{}».", q.subject),
        };
        t.push_str(&format!("\n{}. {body}\n", n + 1));
    }
    t
}

fn question_text(kind: &str, subject: &str, context: &[String]) -> String {
    let ctx = context.iter().map(|c| format!("- {c}")).collect::<Vec<_>>().join("\n");
    match kind {
        "term" => format!("I am a Whale of Mova reading a scientific paper. I do not understand the term «{subject}». What does it mean in this paper? Answer in at most 80 words of plain English, starting with «{subject} is» or «{subject} means». Do not describe or summarize the paper. Context from the paper:\n{ctx}"),
        "section" => format!("I am a Whale of Mova reading a scientific paper. I could not restate any sentence of its part «{subject}». What is that part's main point, in at most 60 words of plain English, in simple sentences of the form «subject verb object»? Do not describe the rest of the paper. Its first sentences:\n{ctx}"),
        _ => format!("I am a Whale of Mova reading a scientific paper. I could not read this sentence of it: «{subject}». What does it claim, in at most 60 words of plain English, in simple sentences of the form «subject verb object»? Do not describe or summarize the paper."),
    }
}

/// A sentence worth asking about: whole (it starts with a capital letter and ends with a full
/// stop or question mark), balanced brackets, not a list bullet, and content (`reader::furniture`).
fn askable(s: &str) -> bool {
    let t = s.trim();
    t.chars().next().is_some_and(char::is_uppercase)
        && t.ends_with(['.', '?', '!'])
        && t.matches('(').count() == t.matches(')').count()
        && !t.starts_with(['•', '-', '*'])
        && reader::furniture(t).is_none()
}

/// Gaps worth a question, never asked before by this Whale or (with a pod) by any Whale of the
/// pod: (kind, subject, context). A term's subject is the paper's own form of it.
fn gaps(r: &Reading, store: &Store, pod: Option<&Pod>, paper: &str, known: &Known, n: usize, focus: &std::collections::BTreeSet<String>) -> Vec<(String, String, Vec<String>)> {
    let mut out = vec![];
    let have = store.known_terms();
    let (terms, _) = reader::gaps(r, usize::MAX, 0);
    let mut term_q: Vec<(String, String, Vec<String>)> = terms
        .into_iter()
        .filter(|(k, _)| !have.contains(k) && !known.glosses.contains_key(k) && !store.asked.contains_key(&question_key("term", paper, k)) && !pod.is_some_and(|p| p.has(k)))
        .map(|(k, _)| {
            let ctx: Vec<String> = r.sentences.iter().filter(|s| s.unknown.contains(&k)).take(3).map(|s| s.text.clone()).collect();
            ("term".to_string(), r.forms.get(&k).cloned().unwrap_or(k), ctx)
        })
        .collect();
    // Review hints first: terms they mention (stable, so the order within stays).
    term_q.sort_by_key(|(_, t, _)| !focus.contains(&t.to_lowercase()));
    // Parts with nothing restated.
    let mut sec_q = vec![];
    for (i, s) in r.sections.iter().enumerate() {
        if s.skipped || s.sentences.len() < 3 || known.sections.contains_key(&i) {
            continue;
        }
        if s.sentences.iter().all(|&j| r.sentences[j].clauses.is_empty() && !known.readings.contains_key(&j)) && !store.asked.contains_key(&question_key("section", paper, &s.title)) {
            sec_q.push(("section".to_string(), s.title.clone(), s.sentences.iter().take(3).map(|&j| r.sentences[j].text.clone()).collect()));
        }
    }
    // Prose sentences it could not restate, longest first.
    let mut sent: Vec<&reader::Sentence> = r
        .sentences
        .iter()
        .filter(|s| s.clauses.is_empty() && !known.readings.contains_key(&s.index) && s.text.split_whitespace().count() >= 8 && reader::alpha_share(&s.text) >= 0.8 && askable(&s.text) && !store.asked.contains_key(&question_key("sentence", paper, &s.text)))
        .collect();
    sent.sort_by(|a, b| b.text.split_whitespace().count().cmp(&a.text.split_whitespace().count()).then(a.index.cmp(&b.index)));
    // Sentences of parts a review hint names, or sharing at least two words with a hint, first.
    let hinted = |s: &reader::Sentence| focus.contains(&r.sections[s.section].title.to_lowercase()) || reader::content(&s.text).iter().filter(|w| focus.contains(*w)).count() >= 2;
    sent.sort_by_key(|s| !hinted(s));
    // Half the slots for terms, one for a part when there is one, the rest for sentences.
    let term_slots = n.div_ceil(2).min(term_q.len());
    out.extend(term_q.drain(..term_slots));
    if out.len() < n && !sec_q.is_empty() {
        out.push(sec_q.remove(0));
    }
    for s in sent {
        if out.len() >= n {
            break;
        }
        out.push(("sentence".to_string(), s.text.clone(), vec![]));
    }
    for t in term_q {
        if out.len() >= n {
            break;
        }
        out.push(t);
    }
    out
}

/// Import earlier `paper` runs: answered questions become knowledge, every sent one is asked.
fn import(store: &mut Store, c: &LearnConfig, a: &en::annotate::Annotator, j: &mut Journal) -> Result<()> {
    for root in &c.import {
        let key = format!("import:{}", root.display());
        if store.asked.contains_key(&key) {
            continue;
        }
        let q = std::fs::read_to_string(root.join("questions.jsonl"))?;
        let mut n = 0;
        for line in q.lines().filter(|l| !l.trim().is_empty()) {
            let o: Value = serde_json::from_str(line)?;
            let kind = o["question"]["kind"].as_str().unwrap_or("");
            let subject = o["question"]["subject"].as_str().unwrap_or("");
            let state = o["state"].as_str().unwrap_or("");
            if state == "not_asked" || subject.is_empty() {
                continue;
            }
            let qkey = question_key(kind, &c.paper.id, subject);
            if state == "answered" {
                let text = o["answer"].as_str().unwrap_or("");
                let sha = store.keep_answer(text)?;
                let facts = if kind == "term" { knowledge::term_facts(a, subject, text) } else { knowledge::reading_facts(a, subject, text) };
                let item = KItem { id: store.next_id(), whale: c.whale.clone(), kind: kind.into(), key: subject.into(), paper: c.paper.id.clone(), facts, provenance: json!({"source":"terminal","imported_from":root,"question":o["question"]["id"],"answer_sha256":sha,"round":0}), unix: now() };
                j.log(json!({"event":"retained","round":0,"item":item.id,"kind":item.kind,"key":item.key,"facts":item.facts.len(),"imported":true,"unix":now()}))?;
                store.retain(item)?;
                n += 1;
            }
            store.mark_asked(&qkey, json!({"state":state,"paper":c.paper.id,"round":0,"imported_from":root}))?;
        }
        store.mark_asked(&key, json!({"state":"imported","answers":n}))?;
    }
    Ok(())
}

/// Re-read the kept answers to this paper's sentence and section questions with the current
/// answer reader (`knowledge::READER`), once per reader version and item: facts the earlier reader
/// missed are retained as a new item (provenance `reread`, the same answer hash). No question is
/// sent; the answer text still never reaches a description (the overlap gate holds).
fn reread(store: &mut Store, c: &LearnConfig, r: &Reading, a: &en::annotate::Annotator, j: &mut Journal) -> Result<()> {
    let todo: Vec<KItem> = store.items.iter().filter(|i| i.paper == c.paper.id && matches!(i.kind.as_str(), "sentence" | "section") && i.provenance["source"] != "reread").cloned().collect();
    for it in todo {
        let key = format!("reread:{}:{}", knowledge::READER, it.id);
        let Some(sha) = it.provenance["answer_sha256"].as_str() else { continue };
        if store.asked.contains_key(&key) {
            continue;
        }
        let Ok(text) = std::fs::read_to_string(store.dir.join("answers").join(format!("{sha}.txt"))) else { continue };
        let source = match it.kind.as_str() {
            "section" => r.sections.iter().find(|s| s.title == it.key).map(|s| s.sentences.iter().map(|&i| r.sentences[i].text.clone()).collect::<Vec<_>>().join(" ")).unwrap_or_default(),
            _ => it.key.clone(),
        };
        let facts = knowledge::reading_facts(a, &source, &text);
        let latest = store.items.iter().rev().find(|x| x.paper == it.paper && x.kind == it.kind && x.key == it.key).map(|x| x.facts.clone()).unwrap_or_default();
        if !facts.is_empty() && facts != latest {
            let item = KItem { id: store.next_id(), whale: c.whale.clone(), kind: it.kind.clone(), key: it.key.clone(), paper: it.paper.clone(), facts, provenance: json!({"source":"reread","of":it.id,"answer_sha256":sha,"reader":knowledge::READER}), unix: now() };
            j.log(json!({"event":"retained","round":0,"item":item.id,"kind":item.kind,"key":item.key,"facts":item.facts.len(),"reread_of":it.id,"reader":knowledge::READER,"unix":now()}))?;
            store.retain(item)?;
        }
        store.mark_asked(&key, json!({"state":"reread","item":it.id,"paper":c.paper.id,"reader":knowledge::READER}))?;
    }
    Ok(())
}

fn status(dir: &Path) -> Result<Option<Value>> {
    let p = dir.join("status.json");
    if !p.exists() {
        return Ok(None);
    }
    Ok(Some(serde_json::from_slice(&std::fs::read(p)?)?))
}

/// Read the latest state of a Whale's paper (for batches and English answers).
pub fn paper_status(c: &LearnConfig) -> Result<Option<Value>> {
    status(&c.paper_dir())
}

/// Learn on one paper with the given terminal until a stop. Returns the final status.
pub fn run(c: &LearnConfig, t: &mut dyn PrivateTerminal) -> Result<Value> {
    c.validate()?;
    if t.animal() != c.whale {
        return Err(error(format!("the terminal belongs to {}, not to {}", t.animal(), c.whale)));
    }
    if file_hash(&c.paper.path)? != c.paper.sha256 {
        return Err(error("paper hash mismatch; nothing read"));
    }
    private_dir(&c.state_root)?;
    let mut learner = Some(try_lease(&c.state_root.join("learner.lock"), true)?.ok_or_else(|| error(format!("{} is already learning (learner.lock held)", c.whale)))?);
    let profile = c.terminal.load()?;
    let _lease = TerminalLease::take(&profile)?;
    let dir = c.paper_dir();
    std::fs::create_dir_all(&dir)?;
    let mut st = status(&dir)?.unwrap_or_else(|| {
        let start = now();
        let by_hours = start + (c.hours * 3600.0) as u64;
        json!({"whale":c.whale,"paper":c.paper.id,"paper_sha256":c.paper.sha256,"state":"learning","first_start":start,"deadline":c.deadline_unix.map_or(by_hours, |d| d.min(by_hours)),"rounds":0})
    });
    if st["paper_sha256"] != json!(c.paper.sha256) {
        return Err(error("this paper's state was made from another text (hash differs); a changed text needs a new state"));
    }
    if st["state"] == "passed" {
        return Ok(st);
    }
    let mut j = Journal::open(&dir.join("journal.jsonl"))?;
    // A paper that stopped unpassed and is assigned again with a later deadline (a later batch
    // retrying it after a writer change) is reopened: the new deadline, and `max_rounds` counted
    // from here. Its knowledge, asked questions and earlier rounds stay. A paper that stopped asked
    // out (`EXHAUSTED`) under the current writer is not reopened: the same writer and the same
    // knowledge would only repeat its rounds.
    let later = c.deadline_unix.is_some_and(|d| d > st["deadline"].as_u64().unwrap_or(0));
    let same_writer_out = st["reason"].as_str().is_some_and(|r| EXHAUSTED.contains(&r)) && st["last_round"]["writer"] == json!(compose::WRITER);
    if st["state"] == "stopped" && later && same_writer_out {
        j.log(json!({"event":"not_reopened","reason":st["reason"],"writer":compose::WRITER,"unix":now()}))?;
        return Ok(st);
    }
    if st["state"] == "stopped" && later {
        let start = now();
        let d = c.deadline_unix.map_or(start, |d| d.min(start + (c.hours * 3600.0) as u64));
        j.log(json!({"event":"reopened","previous_reason":st["reason"],"previous_deadline":st["deadline"],"deadline":d,"rounds":st["rounds"],"writer":compose::WRITER,"unix":start}))?;
        st["state"] = json!("learning");
        st["deadline"] = json!(d);
        st["rounds_base"] = st["rounds"].clone();
        if let Some(o) = st.as_object_mut() {
            o.remove("reason");
            o.remove("ended");
            o.remove("stall");
            o.remove("best_words");
        }
    }
    let a = world::tree::annotator().map_err(|e| error(e.to_string()))?;
    let mut store = Store::open(&c.state_root.join("knowledge"))?;
    import(&mut store, c, a, &mut j)?;
    let mut pod = match &c.pod {
        Some(p) => Some(Pod::open(p)?),
        None => None,
    };
    if let Some(p) = pod.as_mut() {
        let n = p.publish(&c.whale, &store)?;
        j.log(json!({"event":"pod","published":n,"pod_terms":p.entries.len(),"claims":p.claims.len(),"unix":now()}))?;
    }
    let text = std::fs::read_to_string(&c.paper.path)?;
    let started = std::time::Instant::now();
    let r = reader::read_with(&text, c.gates.verbatim_max)?;
    let read_ms = started.elapsed().as_millis() as u64;
    j.log(json!({"event":"read","sentences":r.sentences.len(),"unknown_terms":r.unknown.len(),"filtered":r.filtered,"names":r.names.len(),"writer":compose::WRITER,"unix":now()}))?;
    reread(&mut store, c, &r, a, &mut j)?;
    // Terms that can reach the description: those of the paper's restated clauses.
    let usable: std::collections::BTreeSet<String> = r.sentences.iter().flat_map(|s| s.clauses.iter()).flat_map(|cl| cl.text.split(|ch: char| !(ch.is_alphanumeric() || ch == '-')).filter(|w| !w.is_empty()).map(norm_term).collect::<Vec<_>>()).collect();
    let deadline = st["deadline"].as_u64().unwrap_or(u64::MAX);
    let open = open_licence(&c.paper.licence);
    let mut learned_last: usize = st["learned_last"].as_u64().unwrap_or(1) as usize;
    let reason = loop {
        let k = st["rounds"].as_u64().unwrap_or(0) as usize + 1;
        if k > c.max_rounds + st["rounds_base"].as_u64().unwrap_or(0) as usize {
            break "round_limit".to_string();
        }
        if now() >= deadline {
            break "deadline".to_string();
        }
        if c.stop_file.as_ref().is_some_and(|p| p.exists()) {
            break "stop".to_string();
        }
        let rd = dir.join(format!("round-{k:03}"));
        if rd.exists() {
            return Err(error(format!("{} exists; a round is never repeated into the same directory", rd.display())));
        }
        private_dir(&rd)?;
        if let Some(p) = pod.as_mut() {
            p.reload()?;
        }
        // The closed topic corpus explains terms natively before anything is asked: its seed
        // writes this Whale's store, so the learner lets go of its lock for the seed only.
        if let Some(tc) = c.topic_corpus.as_ref().filter(|t| t.seed) {
            learner = None;
            let v = tc.seed(&c.paper.id, &c.whale, &c.state_root);
            learner = try_lease(&c.state_root.join("learner.lock"), true)?;
            j.log(json!({"event":"seeded","round":k,"result":v,"unix":now()}))?;
            if learner.is_none() {
                break "lock_lost".to_string();
            }
            store = Store::open(&c.state_root.join("knowledge"))?;
        }
        let pod_items: Vec<KItem> = pod.as_ref().map(|p| p.items(&c.whale)).unwrap_or_default();
        j.log(json!({"event":"round","round":k,"knowledge_items":store.items.len(),"pod_items":pod_items.len(),"asked":store.asked.len(),"unix":now()}))?;
        // Own items first: per term and relation the first fact is written, so a Whale prefers
        // what it learned itself and adds what the pod learned.
        let mut all_items = store.items.clone();
        all_items.extend(pod_items.iter().cloned());
        let (known, native) = Known::of(&r, &c.paper.id, &all_items);
        let mut answers = store.answers()?;
        if let Some(p) = &pod {
            answers.extend(p.answers()?);
        }
        let guard = Guard::new(&reader::repair(&text), &answers, c.gates.clone());
        let d = compose::compose(&r, &known, &guard, c.words.max);
        let mut cited: Vec<KItem> = all_items.clone();
        cited.extend(native.iter().cloned());
        let g = gates::check(&d, &r, &cited, &guard, c.words.min, c.words.max);
        write_sync(&rd.join("description.txt"), d.text.as_bytes())?;
        save(&rd.join("draft.json"), &json!({"items":d.items,"available_words":d.available_words}))?;
        save(&rd.join("gates.json"), &g)?;
        let count = |kind: &str| d.items.iter().filter(|i| i.kind == kind).map(|i| crate::contract::word_count(&i.text)).sum::<usize>();
        j.log(json!({"event":"draft","round":k,"words":d.words,"sha256":hash(&d.text),"source_words":count("source"),"explain_words":count("explain"),"reading_words":count("reading"),"structure_words":count("structure"),"available_words":d.available_words,"unix":now()}))?;
        j.log(json!({"event":"gates","round":k,"pass":g["pass"],"failed":g["failed"],"verbatim_max":g["verbatim_max"],"answer_overlap":g["answer_overlap"],"near_duplicates":g["near_duplicates"],"traceability":g["traceability"],"ungrammatical":g["ungrammatical"],"unix":now()}))?;
        // Early stop: a round that gained no word over the best earlier draft, after a round that
        // learned nothing the writer can use, is a stalled round.
        let best = st["best_words"].as_u64().map(|b| b as usize);
        let gained = best.is_none_or(|b| d.words > b);
        let stall = if gained || learned_last > 0 { 0 } else { st["stall"].as_u64().unwrap_or(0) as usize + 1 };
        st["stall"] = json!(stall);
        st["best_words"] = json!(best.map_or(d.words, |b| b.max(d.words)));
        st["rounds"] = json!(k);
        st["last_round"] = json!({"round":k,"words":d.words,"pass":g["pass"],"failed":g["failed"],"writer":compose::WRITER});
        // Review hints: when the round gained few words, and once before the paper passes.
        let gain = best.map_or(d.words, |b| d.words.saturating_sub(b));
        let mut review_stop: Option<String> = None;
        let mut review_calls = 0;
        if c.review.enabled && (g["pass"] == true || (best.is_some() && gain < c.review.min_gain_words)) {
            let hd = dir.join("hints");
            let dsha = hash(&d.text);
            if let Some(v) = crate::review::cached(&hd, &dsha, &c.paper.sha256)? {
                j.log(json!({"event":"review_cached","round":k,"of_round":v["round"],"unix":now()}))?;
            } else {
                let parts: std::collections::BTreeSet<usize> = d.items.iter().filter(|i| !i.sources.is_empty()).filter_map(|i| i.section).collect();
                let parts: Vec<usize> = parts.into_iter().collect();
                match crate::review::requests(&r, &d.text, &parts, c.review.max_bytes, "\u{1}") {
                    None => j.log(json!({"event":"review_skipped","round":k,"reason":"the description alone exceeds the request bound","unix":now()}))?,
                    Some(reqs) => {
                        let mut hints = vec![];
                        let mut shas = vec![];
                        for (n, text) in reqs.iter().enumerate() {
                            let q = Question { id: format!("{}-r{k}-review-{}", c.whale, n + 1), kind: "review".into(), subject: format!("review of round {k}"), text: text.clone(), context: vec![], items: vec![] };
                            j.log(json!({"event":"call","round":k,"id":q.id,"kind":"review","bytes":text.len(),"sha256":hash(text),"unix":now()}))?;
                            review_calls += 1;
                            match t.ask(&q) {
                                Ok(ans) => {
                                    let a: String = ans.text.chars().take(ANSWER_MAX * 4).collect();
                                    // Kept with the answers: the overlap gate keeps hint text out of descriptions.
                                    let sha = store.keep_answer(&a)?;
                                    shas.push(sha.clone());
                                    j.log(json!({"event":"answer","round":k,"id":q.id,"sha256":sha,"words":a.split_whitespace().count(),"ms":ans.elapsed_ms,"unix":now()}))?;
                                    hints.extend(crate::review::parse(&a));
                                }
                                Err(e) => {
                                    j.log(json!({"event":"no_answer","round":k,"id":q.id,"kind":e.kind(),"reason":e.reason(),"retry":false,"unix":now()}))?;
                                    if e != AskError::Unavailable {
                                        review_stop = Some(e.kind().into());
                                    }
                                    break;
                                }
                            }
                        }
                        if !shas.is_empty() {
                            let v = crate::review::store(&hd, k, &dsha, &c.paper.sha256, &hints, &reqs, &shas)?;
                            j.log(json!({"event":"review","round":k,"hints":hints.len(),"requests":reqs.len(),"file":v["file"],"unix":now()}))?;
                        }
                    }
                }
            }
        }
        if g["pass"] == true {
            round_files(&rd, &r, c, &store, &[], open, k, &d, &g, read_ms, review_calls)?;
            cite_round(c, &rd, &mut j, k)?;
            break "passed".to_string();
        }
        if let Some(s) = review_stop {
            round_files(&rd, &r, c, &store, &[], open, k, &d, &g, read_ms, review_calls)?;
            cite_round(c, &rd, &mut j, k)?;
            save(&dir.join("status.json"), &st)?;
            break s;
        }
        if stall >= c.stall_rounds {
            j.log(json!({"event":"stalled","round":k,"rounds_without_gain":stall,"best_words":st["best_words"],"words":d.words,"unix":now()}))?;
            round_files(&rd, &r, c, &store, &[], open, k, &d, &g, read_ms, 0)?;
            cite_round(c, &rd, &mut j, k)?;
            save(&dir.join("status.json"), &st)?;
            break "stalled".to_string();
        }
        // Questions about gaps never asked before (by this Whale or the pod), those the latest
        // review hints point at first.
        let hints = crate::review::latest(&dir.join("hints"))?;
        let focus = crate::review::focus(&hints);
        let mut qs = gaps(&r, &store, pod.as_ref(), &c.paper.id, &known, c.max_questions, &focus);
        if !hints.is_empty() {
            let used: Vec<&String> = qs.iter().filter(|q| focus.contains(&q.1.to_lowercase()) || reader::content(&q.1).iter().filter(|w| focus.contains(*w)).count() >= 2).map(|q| &q.1).collect();
            let ignored: Vec<&str> = hints.iter().filter(|h| matches!(h.kind.as_str(), "rephrase" | "order")).map(|h| h.kind.as_str()).collect();
            j.log(json!({"event":"hints","round":k,"hints":hints.len(),"used_for_questions":used,"ignored":{"count":ignored.len(),"reason":"the writer restates in the paper's order and keeps every restatable clause; presentation hints are kept for review"},"unix":now()}))?;
        }
        if let Some(p) = pod.as_mut() {
            let mut kept = vec![];
            for q in qs {
                if q.0 != "term" || p.claim(&q.1, &c.whale, &c.paper.id)? {
                    kept.push(q);
                } else {
                    j.log(json!({"event":"pod_claimed","round":k,"term":q.1,"unix":now()}))?;
                }
            }
            qs = kept;
        }
        let items: Vec<Question> = qs.iter().enumerate().map(|(n, (kind, subject, ctx))| Question { id: format!("{}-r{k}-q{}", c.whale, n + 1), kind: kind.clone(), subject: subject.clone(), text: question_text(kind, subject, ctx), context: ctx.clone(), items: vec![] }).collect();
        // One call per round (batched), or one per question.
        let calls: Vec<Question> = if c.batched && !items.is_empty() {
            vec![Question { id: format!("{}-r{k}", c.whale), kind: "batch".into(), subject: format!("{} questions", items.len()), text: batch_text(&items), context: vec![], items: items.clone() }]
        } else {
            items.clone()
        };
        let mut outcomes: Vec<Value> = vec![];
        let mut stop: Option<String> = None;
        let mut learned = 0usize;
        for call in &calls {
            let parts: Vec<Question> = if call.items.is_empty() { vec![call.clone()] } else { call.items.clone() };
            if let Some(why) = &stop {
                outcomes.extend(parts.iter().map(|q| json!({"question":q,"state":"not_asked","reason":why})));
                continue;
            }
            for q in &parts {
                j.log(json!({"event":"question","round":k,"id":q.id,"kind":q.kind,"subject":q.subject,"sha256":hash(&q.text),"call":call.id,"unix":now()}))?;
            }
            j.log(json!({"event":"call","round":k,"id":call.id,"questions":parts.len(),"sha256":hash(&call.text),"unix":now()}))?;
            match t.ask(call) {
                Ok(ans) => {
                    let full: String = ans.text.chars().take(ANSWER_MAX).collect();
                    let sha = store.keep_answer(&full)?;
                    j.log(json!({"event":"answer","round":k,"id":call.id,"terminal_question":ans.terminal_id,"sha256":sha,"words":full.split_whitespace().count(),"ms":ans.elapsed_ms,"unix":now()}))?;
                    let texts = if call.items.is_empty() { vec![full.clone()] } else { knowledge::split_answer(&full, parts.len()) };
                    for (n, (q, part)) in parts.iter().zip(texts.iter()).enumerate() {
                        let key = question_key(&q.kind, &c.paper.id, &q.subject);
                        if ans.missing.contains(&(n + 1)) || part.trim().is_empty() {
                            store.mark_asked(&key, json!({"state":"no_answer_part","question":q.id,"call":call.id,"paper":c.paper.id,"round":k}))?;
                            outcomes.push(json!({"question":q,"state":"no_answer_part","answer_sha256":sha}));
                            continue;
                        }
                        store.mark_asked(&key, json!({"state":"answered","question":q.id,"call":call.id,"paper":c.paper.id,"round":k,"terminal_question":ans.terminal_id}))?;
                        let source = match q.kind.as_str() {
                            "section" => r.sections.iter().find(|s| s.title == q.subject).map(|s| s.sentences.iter().map(|&i| r.sentences[i].text.clone()).collect::<Vec<_>>().join(" ")).unwrap_or_default(),
                            _ => q.subject.clone(),
                        };
                        let facts = if q.kind == "term" { knowledge::term_facts(a, &q.subject, part) } else { knowledge::reading_facts(a, &source, &knowledge::paper_words(part)) };
                        // Bare-category facts are flagged for review, not kept.
                        let (facts, bare) = crate::pod::split_bare(facts);
                        for f in &bare {
                            j.log(json!({"event":"review","round":k,"key":q.subject,"fact":f,"reason":"bare_category","unix":now()}))?;
                            match &pod {
                                Some(p) => p.review(&c.whale, &c.paper.id, &q.subject, f)?,
                                None => store.review(&json!({"term":q.subject,"paper":c.paper.id,"fact":f,"reason":"bare_category"}))?,
                            }
                        }
                        let usable_n = if q.kind == "term" { if usable.contains(&norm_term(&q.subject)) { facts.len() } else { 0 } } else { facts.len() };
                        learned += usable_n;
                        let item = KItem { id: store.next_id(), whale: c.whale.clone(), kind: q.kind.clone(), key: q.subject.clone(), paper: c.paper.id.clone(), facts, provenance: json!({"source":"terminal","question":q.id,"call":call.id,"part":n + 1,"terminal_question":ans.terminal_id,"answer_sha256":sha,"round":k}), unix: now() };
                        j.log(json!({"event":"retained","round":k,"item":item.id,"kind":item.kind,"key":item.key,"facts":item.facts.len(),"usable":usable_n,"bare":bare.len(),"unix":now()}))?;
                        outcomes.push(json!({"question":q,"state":"answered","answer_sha256":sha,"retained":item.id,"facts":item.facts,"bare":bare}));
                        store.retain(item)?;
                    }
                }
                Err(e) => {
                    for q in &parts {
                        if e != AskError::Unavailable {
                            // Sent (or possibly sent): never asked again.
                            store.mark_asked(&question_key(&q.kind, &c.paper.id, &q.subject), json!({"state":e.kind(),"question":q.id,"call":call.id,"paper":c.paper.id,"round":k,"reason":e.reason()}))?;
                        }
                        outcomes.push(json!({"question":q,"state":e.kind(),"reason":e.reason()}));
                    }
                    j.log(json!({"event":"no_answer","round":k,"id":call.id,"kind":e.kind(),"reason":e.reason(),"retry":false,"unix":now()}))?;
                    stop = Some(match e {
                        AskError::Unavailable => "no_terminal".into(),
                        other => other.kind().into(),
                    });
                }
            }
        }
        if let Some(p) = pod.as_mut() {
            p.publish(&c.whale, &store)?;
        }
        learned_last = learned;
        st["learned_last"] = json!(learned);
        round_files(&rd, &r, c, &store, &outcomes, open, k, &d, &g, read_ms, calls.len() + review_calls)?;
        cite_round(c, &rd, &mut j, k)?;
        save(&dir.join("status.json"), &st)?;
        if let Some(s) = stop {
            break s;
        }
        if qs.is_empty() {
            break "nothing_to_ask".to_string();
        }
    };
    drop(learner);
    st["state"] = json!(if reason == "passed" { "passed" } else { "stopped" });
    st["reason"] = json!(reason);
    st["ended"] = json!(now());
    save(&dir.join("status.json"), &st)?;
    j.log(json!({"event":"stop","reason":reason,"rounds":st["rounds"],"unix":now()}))?;
    Ok(st)
}

/// A summary of a Whale's learning, per paper, from its state (`mova-whale learn-report`): rounds,
/// words of the first, best and last draft, rounds that gained no word over the best earlier
/// draft, questions by kind, terminal calls, answered questions, facts kept (bare-category facts
/// counted apart) per answered question by kind, and the stop. Read-only.
pub fn report(state_root: &Path) -> Result<Value> {
    let store = Store::open(&state_root.join("knowledge")).ok();
    let items: std::collections::BTreeMap<String, KItem> = store.map(|s| s.items.into_iter().map(|i| (i.id.clone(), i)).collect()).unwrap_or_default();
    let mut papers = vec![];
    let mut dirs: Vec<PathBuf> = std::fs::read_dir(state_root.join("papers"))?.filter_map(|e| e.ok().map(|e| e.path())).collect();
    dirs.sort();
    for d in dirs {
        let Ok(text) = std::fs::read_to_string(d.join("journal.jsonl")) else { continue };
        let ev: Vec<Value> = text.lines().filter(|l| !l.trim().is_empty()).filter_map(|l| serde_json::from_str(l).ok()).collect();
        let words: Vec<u64> = ev.iter().filter(|e| e["event"] == "draft").filter_map(|e| e["words"].as_u64()).collect();
        let mut best = 0u64;
        let mut no_gain = 0;
        for (i, w) in words.iter().enumerate() {
            if i > 0 && *w <= best {
                no_gain += 1;
            }
            best = best.max(*w);
        }
        let mut questions: std::collections::BTreeMap<String, usize> = Default::default();
        for e in ev.iter().filter(|e| e["event"] == "question") {
            *questions.entry(e["kind"].as_str().unwrap_or("").to_string()).or_insert(0) += 1;
        }
        let calls = ev.iter().filter(|e| e["event"] == "call").count();
        let asked: usize = questions.values().sum();
        let calls = if calls == 0 { ev.iter().filter(|e| e["event"] == "answer" || e["event"] == "no_answer").count() } else { calls };
        let mut answered: std::collections::BTreeMap<String, usize> = Default::default();
        let mut kept: std::collections::BTreeMap<String, usize> = Default::default();
        let mut with_facts: std::collections::BTreeMap<String, usize> = Default::default();
        let mut bare_n = 0usize;
        for e in ev.iter().filter(|e| e["event"] == "retained" && e["round"].as_u64().unwrap_or(0) > 0 && e["reread_of"].is_null()) {
            let kind = e["kind"].as_str().unwrap_or("").to_string();
            *answered.entry(kind.clone()).or_insert(0) += 1;
            let (good, bad) = match items.get(e["item"].as_str().unwrap_or("")) {
                Some(it) => (it.facts.iter().filter(|f| !knowledge::bare(f)).count(), it.facts.iter().filter(|f| knowledge::bare(f)).count() + e["bare"].as_u64().unwrap_or(0) as usize),
                None => (e["facts"].as_u64().unwrap_or(0) as usize, e["bare"].as_u64().unwrap_or(0) as usize),
            };
            *kept.entry(kind.clone()).or_insert(0) += good;
            if good > 0 {
                *with_facts.entry(kind).or_insert(0) += 1;
            }
            bare_n += bad;
        }
        let st: Value = std::fs::read(d.join("status.json")).ok().and_then(|b| serde_json::from_slice(&b).ok()).unwrap_or(Value::Null);
        let read = ev.iter().rev().find(|e| e["event"] == "read").cloned().unwrap_or(Value::Null);
        papers.push(json!({
            "paper": st["paper"], "state": st["state"], "reason": st["reason"], "rounds": words.len(),
            "words_first": words.first(), "words_best": best, "words_last": words.last(), "rounds_without_gain": no_gain,
            "questions": questions, "asked": asked, "terminal_calls": calls,
            "questions_per_call": if calls == 0 { 0.0 } else { asked as f64 / calls as f64 },
            "answered": answered, "answers_with_facts": with_facts, "facts_kept": kept, "bare_facts": bare_n,
            "filtered": read["filtered"],
        }));
    }
    Ok(json!({"state_root":state_root,"papers":papers}))
}

/// The round's description with citations and a bibliography (`cited.md`), when configured.
fn cite_round(c: &LearnConfig, rd: &Path, j: &mut Journal, k: usize) -> Result<()> {
    if let Some(tc) = c.topic_corpus.as_ref().filter(|t| t.cite) {
        let v = tc.cite(&c.paper.id, rd, &c.state_root, &c.whale, &rd.join("cited.md"));
        j.log(json!({"event":"cited","round":k,"result":v,"unix":now()}))?;
    }
    Ok(())
}

#[allow(clippy::too_many_arguments)]
fn round_files(rd: &Path, r: &Reading, c: &LearnConfig, store: &Store, outcomes: &[Value], open: bool, k: usize, d: &compose::Draft, g: &Value, read_ms: u64, calls: usize) -> Result<()> {
    let mut q = String::new();
    for o in outcomes {
        q.push_str(&o.to_string());
        q.push('\n');
    }
    write_sync(&rd.join("questions.jsonl"), q.as_bytes())?;
    let source = json!({"paper":c.paper.id,"title":r.title,"licence":c.paper.licence,"open_school_admissible":open,"admission":"review_required"});
    let mut teach = String::new();
    let have = store.known_terms();
    for (term, n) in reader::gaps(r, 64, 0).0 {
        let item = store.items.iter().rev().find(|i| i.kind == "term" && norm_term(&i.key) == term);
        let display = r.forms.get(&term).cloned().unwrap_or_else(|| term.clone());
        let learned = item.map(|i| knowledge::explain(&display, &i.facts.iter().filter(|f| !knowledge::bare(f)).cloned().collect::<Vec<_>>())).unwrap_or_default();
        teach.push_str(&json!({"kind":"unknown_term","term":display,"occurrences":n,"asked":store.asked.get(&question_key("term", &c.paper.id, &term)).map(|v| v["state"].clone()),"understood":have.contains(&term),"mova_explanation":learned,"knowledge_item":item.map(|i| i.id.clone()),"source":source}).to_string());
        teach.push('\n');
    }
    for s in r.sentences.iter().filter(|s| s.clauses.is_empty()).take(64) {
        teach.push_str(&json!({"kind":if s.admitted {"not_restated"} else {"unparsed"},"sentence":s.index,"section":r.sections[s.section].title,"text":s.text,"source":source}).to_string());
        teach.push('\n');
    }
    write_sync(&rd.join("teaching.jsonl"), teach.as_bytes())?;
    let mut m = reader::measure(r, &reader::Description { text: String::new(), sentences: vec![], words: 0, available_words: 0 });
    let count = |kind: &str| d.items.iter().filter(|i| i.kind == kind).map(|i| crate::contract::word_count(&i.text)).sum::<usize>();
    for (key, v) in [
        ("whale", json!(c.whale)),
        ("paper", json!(c.paper.id)),
        ("round", json!(k)),
        ("description_words", json!(d.words)),
        ("available_words", json!(d.available_words)),
        ("source_words", json!(count("source"))),
        ("explain_words", json!(count("explain"))),
        ("reading_words", json!(count("reading"))),
        ("structure_words", json!(count("structure"))),
        ("sentences_restated", json!(d.items.iter().flat_map(|i| i.sources.iter()).collect::<std::collections::BTreeSet<_>>().len())),
        ("sentences_without_clause", json!(r.sentences.iter().filter(|s| s.clauses.is_empty()).count())),
        ("knowledge_items", json!(store.items.len())),
        ("questions", json!(outcomes.len())),
        ("terminal_calls", json!(calls)),
        ("filtered", json!(r.filtered)),
        ("answered", json!(outcomes.iter().filter(|o| o["state"] == "answered").count())),
        ("gates", g.clone()),
        ("read_ms", json!(read_ms)),
    ] {
        m[key] = v;
    }
    for drop in ["description_words_v1", "sections_described", "section_coverage", "sentences_used", "source_sentences_used", "traceable_sentences", "traceability", "traceability_method", "verbatim_run_max", "verbatim_run_median", "sentences_with_verbatim_run_ge_12", "available_skeleton_words"] {
        if let Some(o) = m.as_object_mut() {
            o.remove(drop);
        }
    }
    save(&rd.join("metrics.json"), &m)
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use crate::terminal::Answer;
    use std::collections::BTreeMap;

    pub fn model_ok() -> bool {
        match std::env::var_os("MOVA_OCTOPUS_TEST_MODEL") {
            Some(m) => {
                std::env::set_var("WORLD_EN_MODEL", m);
                true
            }
            None => {
                eprintln!("SKIPPED: MOVA_OCTOPUS_TEST_MODEL not configured");
                false
            }
        }
    }
    pub const PAPER: &str = "# Zorbic Trees for Parsing\n\nA. Author · 2024\n\n## Abstract\n\nWe introduce zorbic trees for dependency parsing (DP). Zorbic trees store every head of a sentence in one array. The DP community has used many kinds of trees before.\n\n## Method\n\nThe parser reads each word of the sentence once. It attaches every word to its head with a zorbic rule. The rule never creates a cycle in the tree. Each QZW is checked by the parser before it is stored. The QZW of a word holds its head and its label.\n\n## Results\n\nThe parser reaches an accuracy of 91.5% on three treebanks. Zorbic parsing is faster than the baseline on long sentences, and it uses 30% less memory.\n\n## References\n\nSmith, J. 2020. Trees. Journal of Trees.\n";

    /// A scripted terminal: answers terms from a table and every other question with a fixed
    /// text; reports a usage limit after `limit_after` answers.
    pub struct Scripted {
        pub animal: String,
        pub terms: BTreeMap<String, String>,
        pub other: String,
        pub limit_after: Option<usize>,
        /// Every call (a batched call is one).
        pub received: Vec<Question>,
        /// The answer to review requests.
        pub review: String,
    }
    impl Scripted {
        pub fn new(animal: &str, terms: BTreeMap<String, String>, other: &str, limit_after: Option<usize>) -> Self {
            Self { animal: animal.into(), terms, other: other.into(), limit_after, received: vec![], review: String::new() }
        }
        /// The questions received, batched items one by one (reviews left out).
        pub fn questions(&self) -> Vec<Question> {
            self.received.iter().filter(|q| q.kind != "review").flat_map(|q| if q.items.is_empty() { vec![q.clone()] } else { q.items.clone() }).collect()
        }
        fn one(&self, q: &Question) -> String {
            if q.kind == "term" { self.terms.get(&q.subject).cloned().unwrap_or_else(|| "I cannot tell from these lines.".to_string()) } else { self.other.clone() }
        }
    }
    impl PrivateTerminal for Scripted {
        fn animal(&self) -> &str {
            &self.animal
        }
        fn ask(&mut self, q: &Question) -> std::result::Result<Answer, AskError> {
            self.received.push(q.clone());
            if self.limit_after.is_some_and(|n| self.received.len() > n) {
                return Err(AskError::Limit("scripted usage limit".into()));
            }
            let text = match q.kind.as_str() {
                "review" => self.review.clone(),
                "batch" => q.items.iter().enumerate().map(|(n, it)| format!("{}. {}", n + 1, self.one(it))).collect::<Vec<_>>().join("\n"),
                _ => self.one(q),
            };
            Ok(Answer { text, elapsed_ms: 1, terminal_id: None, missing: vec![] })
        }
    }

    pub fn tmp(tag: &str) -> PathBuf {
        let d = std::env::var_os("CARGO_TARGET_DIR").map(PathBuf::from).unwrap_or_else(std::env::temp_dir).join(format!("whale-learn-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }
    pub fn config(dir: &Path, name: &str, min: usize) -> LearnConfig {
        let p = dir.join(format!("{name}.md"));
        std::fs::write(&p, PAPER).unwrap();
        serde_json::from_value(json!({"schema":SCHEMA,"whale":name,"state_root":dir.join(format!("{name}-state")),
            "paper":{"id":"zorbic-2024-trees","path":p,"sha256":file_hash(&p).unwrap(),"title":"Zorbic Trees for Parsing","licence":"CC BY 4.0"},
            "words":{"min":min,"max":2000},"max_questions":2,"max_rounds":3,"review":{"enabled":false},
            "terminal":crate::terminal::tests::profile(dir, name)})).unwrap()
    }
    fn events(c: &LearnConfig) -> Vec<Value> {
        std::fs::read_to_string(c.paper_dir().join("journal.jsonl")).unwrap().lines().map(|l| serde_json::from_str(l).unwrap()).collect()
    }

    #[test]
    fn rounds_retain_knowledge_reuse_it_and_never_ask_twice() {
        if !model_ok() {
            return;
        }
        let d = tmp("rounds");
        let c = config(&d, "whale-1", 1500);
        let answer = "QZW is a record of one word. It stores the head index and the dependency label of that word.";
        let mut t = Scripted::new("whale-1", BTreeMap::from([("QZW".to_string(), answer.to_string())]), "The parser reads each word once.", None);
        let st = run(&c, &mut t).unwrap();
        // The description stays short of 1,500 words: no padding, so the rounds run out.
        assert_eq!(st["state"], "stopped", "{st}");
        assert!(matches!(st["reason"].as_str(), Some("round_limit" | "nothing_to_ask")), "{st}");
        // Every question was asked once.
        let subjects: Vec<String> = t.questions().iter().map(|q| format!("{}:{}", q.kind, q.subject)).collect();
        // One terminal call per round carries all its questions.
        assert!(t.received.iter().all(|q| q.kind == "batch" && !q.items.is_empty() && q.text.contains("In the paper's words")), "{:?}", t.received);
        let distinct: std::collections::BTreeSet<&String> = subjects.iter().collect();
        assert_eq!(distinct.len(), subjects.len(), "a question was asked twice: {subjects:?}");
        assert!(subjects.contains(&"term:QZW".to_string()), "{subjects:?}");
        // «DP» is defined by the paper itself: understood natively, never asked.
        assert!(!subjects.iter().any(|s| s == "term:DP"), "{subjects:?}");
        // The answer was retained as facts with provenance and reused by the next round's writer.
        let store = Store::open(&c.state_root.join("knowledge")).unwrap();
        let q = store.items.iter().find(|i| i.key == "QZW").unwrap();
        assert_eq!(q.facts[0].relation, "is_a");
        assert_eq!(q.provenance["source"], "terminal");
        let r2 = std::fs::read_to_string(c.paper_dir().join("round-002/description.txt")).unwrap();
        assert!(r2.contains("I understand «QZW» as a record"), "{r2}");
        assert!(!r2.contains("It stores the head index"), "the answer reached the description: {r2}");
        let g2: Value = serde_json::from_slice(&std::fs::read(c.paper_dir().join("round-002/gates.json")).unwrap()).unwrap();
        assert_eq!(g2["answer_overlap"], 0);
        assert_eq!(g2["traceability"], 1.0, "{g2}");
        assert!(g2["failed"].as_array().unwrap().contains(&json!("words")), "{g2}");
        let ev = events(&c);
        assert!(ev.iter().any(|e| e["event"] == "retained" && e["key"] == "QZW"));
        assert_eq!(ev.iter().filter(|e| e["event"] == "draft").count(), st["rounds"].as_u64().unwrap() as usize);
        // A second invocation continues the same paper with new questions only.
        let before = t.questions().len();
        let mut c2 = c.clone();
        c2.max_rounds = 6;
        let _ = run(&c2, &mut t).unwrap();
        let again: Vec<String> = t.questions()[before..].iter().map(|q| format!("{}:{}", q.kind, q.subject)).collect();
        assert!(again.iter().all(|s| !subjects.contains(s)), "{again:?}");
        // The knowledge outlives the paper: another paper's state reads the same items.
        assert!(Store::open(&c.state_root.join("knowledge")).unwrap().known_terms().contains("qzw"));
        let _ = std::fs::remove_dir_all(d);
    }

    #[test]
    fn a_limit_stops_cleanly_a_pass_asks_nothing_and_the_deadline_holds() {
        if !model_ok() {
            return;
        }
        let d = tmp("stops");
        let c = config(&d, "whale-1", 1500);
        let mut t = Scripted::new("whale-1", BTreeMap::new(), "The parser reads each word once.", Some(0));
        let st = run(&c, &mut t).unwrap();
        assert_eq!(st["reason"], "limit", "{st}");
        assert_eq!(t.received.len(), 1, "nothing is asked after the limit");
        let asked = Store::open(&c.state_root.join("knowledge")).unwrap().asked;
        assert!(asked.values().any(|v| v["state"] == "limit"), "the limited question counts as asked");
        // A low bound passes in the first round without a question.
        let mut c2 = config(&d, "whale-2", 40);
        // The fixture is tiny: its counts and titles are a large share of any description of it.
        c2.gates.structure_share_max = 1.0;
        let mut t2 = Scripted::new("whale-2", BTreeMap::new(), "", None);
        let st2 = run(&c2, &mut t2).unwrap();
        assert_eq!(st2["state"], "passed", "{st2}");
        assert!(t2.received.is_empty());
        // Passed stays passed.
        assert_eq!(run(&c2, &mut t2).unwrap()["state"], "passed");
        // A deadline in the past starts no round.
        let mut c3 = config(&d, "whale-3", 1500);
        c3.deadline_unix = Some(1);
        let mut t3 = Scripted::new("whale-3", BTreeMap::new(), "", None);
        let st3 = run(&c3, &mut t3).unwrap();
        assert_eq!((st3["reason"].as_str(), st3["rounds"].as_u64()), (Some("deadline"), Some(0)));
        // A stop file stops the learner before its next round (`stop`); without it the same
        // learner runs rounds (red control).
        let mut c4 = config(&d, "whale-4", 1500);
        c4.stop_file = Some(d.join("STOP"));
        std::fs::write(d.join("STOP"), "").unwrap();
        let mut t4 = Scripted::new("whale-4", BTreeMap::new(), "", None);
        let st4 = run(&c4, &mut t4).unwrap();
        assert_eq!((st4["reason"].as_str(), st4["rounds"].as_u64(), t4.received.len()), (Some("stop"), Some(0), 0));
        std::fs::remove_file(d.join("STOP")).unwrap();
        assert!(run(&c4, &mut t4).unwrap()["rounds"].as_u64().unwrap() > 0);
        // Another Whale's terminal is refused.
        assert!(run(&c3, &mut NoTerm("whale-9")).unwrap_err().to_string().contains("belongs to whale-9"));
        let _ = std::fs::remove_dir_all(d);
    }
    struct NoTerm(&'static str);
    impl PrivateTerminal for NoTerm {
        fn animal(&self) -> &str {
            self.0
        }
        fn ask(&mut self, _q: &Question) -> std::result::Result<Answer, AskError> {
            Err(AskError::Unavailable)
        }
    }

    #[test]
    fn two_runs_are_identical() {
        if !model_ok() {
            return;
        }
        let d = tmp("determinism");
        let mut texts = vec![];
        for name in ["whale-1", "whale-2"] {
            let c = config(&d, name, 1500);
            let mut t = Scripted::new(name, BTreeMap::new(), "The parser reads each word once.", None);
            run(&c, &mut t).unwrap();
            let mut all = String::new();
            for k in 1..=3 {
                if let Ok(s) = std::fs::read_to_string(c.paper_dir().join(format!("round-{k:03}/description.txt"))) {
                    all.push_str(&s);
                }
            }
            texts.push(all);
        }
        assert_eq!(texts[0], texts[1]);
        let _ = std::fs::remove_dir_all(d);
    }

    /// A paper that stopped asked out under the current writer is not reopened by a later deadline;
    /// rounds that gain nothing stop the paper (`stalled`), with a red control (a high
    /// `stall_rounds` lets it run to the round limit).
    #[test]
    fn stalls_stop_and_asked_out_papers_stay_closed() {
        if !model_ok() {
            return;
        }
        let d = tmp("stall");
        // A paper with eight unknown terms: enough questions to go on for many rounds.
        let terms = ["blorptak", "frimzle", "quandrix", "zelvoor", "trimbalk", "vosquent", "plaxorin", "dremwick"];
        let mut body = String::from("# Many Terms\n\n## Method\n\n");
        for t in terms {
            body.push_str(&format!("The parser applies the {t} rule to every word of the sentence. Each {t} rule keeps the head of a word in the tree.\n\n"));
        }
        let paper = d.join("many.md");
        std::fs::write(&paper, &body).unwrap();
        let mine = |name: &str| {
            let mut c = config(&d, name, 1500);
            c.paper.path = paper.clone();
            c.paper.id = "many-2024-terms".into();
            c.paper.sha256 = file_hash(&paper).unwrap();
            c
        };
        let mut c = mine("whale-1");
        c.max_rounds = 12;
        c.max_questions = 1;
        c.stall_rounds = 2;
        // Answers with nothing Mova can keep: no word is gained and nothing is learned.
        let mut t = Scripted::new("whale-1", BTreeMap::new(), "Hmm.", None);
        let st = run(&c, &mut t).unwrap();
        assert_eq!(st["reason"], "stalled", "{st}");
        // Red control: without the early stop the same Whale goes on asking.
        let mut c2 = mine("whale-2");
        c2.max_rounds = 12;
        c2.max_questions = 1;
        c2.stall_rounds = 50;
        let mut t2 = Scripted::new("whale-2", BTreeMap::new(), "Hmm.", None);
        let st2 = run(&c2, &mut t2).unwrap();
        assert_ne!(st2["reason"], "stalled", "{st2}");
        assert!(t2.questions().len() > t.questions().len(), "the stall saved no question: {st} {st2}");
        // A later deadline does not reopen it under the same writer.
        let mut c3 = c.clone();
        c3.deadline_unix = Some(now() + 48 * 3600);
        let before = t.received.len();
        let again = run(&c3, &mut t).unwrap();
        assert_eq!(again["state"], "stopped");
        assert_eq!(t.received.len(), before, "an asked-out paper was reopened under the same writer");
        assert!(events(&c).iter().any(|e| e["event"] == "not_reopened"));
        let _ = std::fs::remove_dir_all(d);
    }

    /// Review hints: asked when a round gains few words, stored per paper (json and md), never asked
    /// twice for the same description, used to choose questions; the hint text never reaches the
    /// description, and a description that copies it fails the overlap gate (red control).
    #[test]
    fn review_hints_are_cached_guide_questions_and_stay_out_of_descriptions() {
        if !model_ok() {
            return;
        }
        let d = tmp("review");
        let mut c = config(&d, "whale-1", 1500);
        c.review = crate::review::ReviewConfig { enabled: true, min_gain_words: 10_000, max_bytes: 100_000 };
        c.max_rounds = 4;
        c.max_questions = 1;
        let hint = "MISSING: Method | how the zorbic rule attaches every stored head";
        let mut t = Scripted::new("whale-1", BTreeMap::new(), "The parser reads each word once.", None);
        t.review = format!("{hint}\nORDER: put the results before the method");
        let st = run(&c, &mut t).unwrap();
        let reviews: Vec<&Question> = t.received.iter().filter(|q| q.kind == "review").collect();
        assert!(!reviews.is_empty(), "{st}");
        assert!(reviews.iter().all(|q| q.text.len() <= 100_000 && q.text.contains("=== MY DESCRIPTION ===") && q.text.contains("## Method")));
        let hd = c.paper_dir().join("hints");
        let index = std::fs::read_to_string(hd.join("index.jsonl")).unwrap_or_else(|e| panic!("{e}: {:?}", events(&c)));
        let first: Value = serde_json::from_str(index.lines().next().unwrap()).unwrap();
        let file = first["file"].as_str().unwrap();
        assert!(hd.join(file).exists() && hd.join(file.replace(".json", ".md")).exists());
        // The same description is never reviewed twice.
        let shas: Vec<&str> = index.lines().map(|l| l.split("description_sha256\":\"").nth(1).unwrap().split('"').next().unwrap()).collect();
        let distinct: std::collections::BTreeSet<&&str> = shas.iter().collect();
        assert_eq!(distinct.len(), shas.len(), "{index}");
        // The hint pointed at «zorbic»: the term is asked first after the review.
        let ev = events(&c);
        assert!(ev.iter().any(|e| e["event"] == "hints" && e["used_for_questions"].as_array().is_some_and(|u| u.iter().any(|x| x.as_str().is_some_and(|x| x.eq_ignore_ascii_case("zorbic"))))), "{ev:?}");
        // The hint text is in no description.
        for k in 1..=st["rounds"].as_u64().unwrap() {
            let text = std::fs::read_to_string(c.paper_dir().join(format!("round-{k:03}/description.txt"))).unwrap();
            assert!(!text.contains("every stored head"), "{text}");
        }
        // Red control: a description that copies the hint fails the answer-overlap gate.
        let store = Store::open(&c.state_root.join("knowledge")).unwrap();
        let text = std::fs::read_to_string(&c.paper.path).unwrap();
        let r = reader::read(&text).unwrap();
        let guard = Guard::new(&text, &store.answers().unwrap(), c.gates.clone());
        let copied_text = "The paper shows how the zorbic rule attaches every stored head.";
        let copied = compose::Draft { text: copied_text.into(), items: vec![compose::Item { text: copied_text.into(), kind: "source", sources: vec![0], knowledge: vec![], section: Some(1), basis: String::new(), cites: vec![] }], words: 11, available_words: 0 };
        let g = gates::check(&copied, &r, &[], &guard, 1, 2000);
        assert!(g["failed"].as_array().unwrap().contains(&json!("answer_overlap")), "{g}");
        let _ = std::fs::remove_dir_all(d);
    }

    /// The replay terminal answers batched questions with recorded answers by normalized term and
    /// reports the items it has no answer for.
    #[test]
    fn replay_answers_recorded_questions() {
        let d = tmp("replay");
        std::fs::write(d.join("questions.jsonl"), format!("{}\n", json!({"question":{"kind":"term","subject":"MRFs"},"state":"answered","answer":"MRF stands for meaning representation framework."}))).unwrap();
        let mut t = crate::terminal::Replay::load("whale-1", &[d.clone()]).unwrap();
        let item = |s: &str| Question { id: "q".into(), kind: "term".into(), subject: s.into(), text: String::new(), context: vec![], items: vec![] };
        let q = Question { id: "b".into(), kind: "batch".into(), subject: "2 questions".into(), text: String::new(), context: vec![], items: vec![item("MRF"), item("QZW")] };
        let a = t.ask(&q).unwrap();
        assert_eq!(a.missing, vec![2]);
        let parts = knowledge::split_answer(&a.text, 2);
        assert!(parts[0].starts_with("MRF stands for"), "{parts:?}");
        assert_eq!((t.asked, t.found), (2, 1));
        let _ = std::fs::remove_dir_all(d);
    }
}
