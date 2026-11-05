//! One Whale, one paper: read it through Mova, write the description itself, consult its private
//! terminal about what it did not understand, and journal all of it.
//!
//! Order: pin check -> new run root -> terminal lease -> read -> write and journal the draft ->
//! questions (at most `max_questions`, each asked once, never retried; a limit, timeout or failure
//! ends consulting cleanly) -> each answer journaled and re-read by Mova into `glossary.txt` ->
//! `teaching.jsonl` (the gap between what was understood and what had to be asked) -> metrics.
//! The description is the draft: it is written before any answer exists and never receives the
//! terminal's text. Files: `journal.jsonl` (fsynced events), `description.txt`, `reading.json`,
//! `questions.jsonl`, `glossary.txt`, `teaching.jsonl`, `metrics.json`.
use crate::{
    reader,
    terminal::{AskError, PrivateTerminal, Question, TerminalConfig, TerminalLease, ANSWER_MAX},
    util::{error, file_hash, hash, now, private_dir, save, valid_hash, write_sync},
    Result,
};
use serde::Deserialize;
use serde_json::{json, Value};
use std::{
    fs::{File, OpenOptions},
    io::Write,
    os::unix::fs::OpenOptionsExt,
    path::{Path, PathBuf},
};

pub const SCHEMA: &str = "mova-shark-paper-v1";

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PaperSource {
    pub id: String,
    pub path: PathBuf,
    pub sha256: String,
    #[serde(default)]
    pub title: String,
    pub licence: String,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Words {
    pub min: usize,
    pub max: usize,
}

/// One Whale run (`mova-whale paper CONFIG`).
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct PaperConfig {
    pub schema: String,
    /// The Whale's name (`whale-1`).
    #[serde(alias = "shark")]
    pub whale: String,
    /// A new private directory for this run.
    pub run_root: PathBuf,
    pub paper: PaperSource,
    pub words: Words,
    #[serde(default = "d_questions")]
    pub max_questions: usize,
    /// The Whale's own private terminal (a `mova-terminal` profile whose `animal` is this Whale);
    /// no other Whale may name the same profile, session, owner marker or root.
    pub terminal: TerminalConfig,
}
fn d_questions() -> usize {
    4
}

impl PaperConfig {
    pub fn load(p: &Path) -> Result<Self> {
        let c: PaperConfig = serde_json::from_value(crate::util::load(p, 1 << 20)?)?;
        c.validate()?;
        Ok(c)
    }
    pub fn validate(&self) -> Result<()> {
        if self.schema != SCHEMA || !crate::util::label(&self.whale) || !self.run_root.is_absolute() || !self.paper.path.is_absolute() {
            return Err(error("paper config: schema, whale label, absolute run_root and paper.path required"));
        }
        if !valid_hash(&self.paper.sha256) || self.paper.id.is_empty() || self.paper.licence.is_empty() {
            return Err(error("paper config: paper id, licence and SHA-256 required"));
        }
        if self.words.min == 0 || self.words.min > self.words.max || self.max_questions > 16 {
            return Err(error("paper config: 0 < words.min <= words.max and max_questions <= 16"));
        }
        let p = self.terminal.load()?;
        if p.animal != self.whale {
            return Err(error(format!("paper config: terminal profile belongs to {}, not {}", p.animal, self.whale)));
        }
        Ok(())
    }
}

/// Several Whale configs side by side: every Whale names its own terminal and lock.
pub fn check_pod(configs: &[PaperConfig]) -> Result<()> {
    let mut ids = std::collections::BTreeSet::new();
    for c in configs {
        c.validate()?;
        let p = c.terminal.load()?;
        let keys = [
            ("whale", c.whale.clone()),
            ("run root", c.run_root.display().to_string()),
            ("terminal profile", c.terminal.profile.display().to_string()),
            ("terminal session", p.session.clone()),
            ("terminal owner marker", p.owner_marker.clone()),
            ("terminal root", p.root.display().to_string()),
        ];
        for k in keys {
            if !ids.insert(k.clone()) {
                return Err(error(format!("{}: {} {} is shared", c.whale, k.0, k.1)));
            }
        }
    }
    Ok(())
}

/// Text under this licence may feed the open school and dictionary: CC BY, CC0 or public domain,
/// without NC, ND or SA terms (CC BY-SA is kept out of what is built).
pub fn open_licence(licence: &str) -> bool {
    let lic = licence.to_lowercase();
    (lic.starts_with("cc by ") || lic == "cc by" || lic.starts_with("cc0") || lic.starts_with("public domain"))
        && !["nc", "nd", "sa"].iter().any(|x| lic.split(|ch: char| !ch.is_alphanumeric()).any(|w| w == *x))
}

struct Journal(File);
impl Journal {
    fn log(&mut self, v: Value) -> Result<()> {
        let mut line = v.to_string();
        line.push('\n');
        self.0.write_all(line.as_bytes())?;
        self.0.sync_data()?;
        Ok(())
    }
}

fn question(id: String, kind: &str, subject: &str, context: Vec<String>) -> Question {
    let text = match kind {
        "term" => format!(
            "I am a Whale of Mova reading a scientific paper. I do not understand the term «{subject}». What does it mean in this paper? Answer in at most 80 words of plain English. Do not describe or summarize the paper. Context from the paper:\n{}",
            context.iter().map(|c| format!("- {c}")).collect::<Vec<_>>().join("\n")
        ),
        _ => format!(
            "I am a Whale of Mova reading a scientific paper. I could not read this sentence of it: «{subject}». What does it claim, in at most 60 words of plain English? Do not describe or summarize the paper."
        ),
    };
    Question { id, kind: kind.into(), subject: subject.into(), text, context, items: vec![] }
}

/// Run one Whale on one paper with the given private terminal. Returns the metrics.
pub fn run(c: &PaperConfig, t: &mut dyn PrivateTerminal) -> Result<Value> {
    c.validate()?;
    if t.animal() != c.whale {
        return Err(error(format!("the terminal belongs to {}, not to {}", t.animal(), c.whale)));
    }
    if file_hash(&c.paper.path)? != c.paper.sha256 {
        return Err(error("paper hash mismatch; nothing read"));
    }
    if c.run_root.exists() {
        return Err(error("run_root exists; a Whale run is never repeated into the same root"));
    }
    private_dir(&c.run_root)?;
    let profile = c.terminal.load()?;
    let _lease = TerminalLease::take(&profile)?;
    let mut j = Journal(OpenOptions::new().create_new(true).append(true).mode(0o600).open(c.run_root.join("journal.jsonl"))?);
    j.log(json!({"event":"start","whale":c.whale,"paper":c.paper.id,"paper_sha256":c.paper.sha256,"licence":c.paper.licence,"terminal_profile":c.terminal.profile,"terminal_session":profile.session,"unix":now()}))?;
    let text = std::fs::read_to_string(&c.paper.path)?;
    let started = std::time::Instant::now();
    let r = reader::read(&text)?;
    let read_ms = started.elapsed().as_millis() as u64;
    save(&c.run_root.join("reading.json"), &serde_json::to_value(&r)?)?;
    j.log(json!({"event":"read","sentences":r.sentences.len(),"sections":r.sections.len(),"unknown_types":r.unknown.len(),"ms":read_ms,"unix":now()}))?;
    // The Whale's own description, written and journaled before any question.
    let d = reader::write(&r, c.words.max);
    write_sync(&c.run_root.join("description.txt"), d.text.as_bytes())?;
    j.log(json!({"event":"draft","words":d.words,"sha256":hash(&d.text),"within_bounds":(c.words.min..=c.words.max).contains(&d.words),"unix":now()}))?;
    // Questions: unknown terms first, then sentences it could not reduce.
    let sentence_slots = if c.max_questions >= 2 { 1 } else { 0 };
    let (terms, hard) = reader::gaps(&r, c.max_questions.saturating_sub(sentence_slots), sentence_slots);
    let mut questions = vec![];
    for (k, _) in &terms {
        let ctx: Vec<String> = r.sentences.iter().filter(|s| s.unknown.contains(k)).take(3).map(|s| s.text.clone()).collect();
        let display = r.forms.get(k).cloned().unwrap_or_else(|| k.clone());
        questions.push(question(format!("{}-q{}", c.whale, questions.len() + 1), "term", &display, ctx));
    }
    for &i in &hard {
        questions.push(question(format!("{}-q{}", c.whale, questions.len() + 1), "sentence", &r.sentences[i].text, vec![]));
    }
    let mut asked = 0;
    let mut answered = 0;
    let mut stop: Option<AskError> = None;
    let mut qlog = String::new();
    let mut glossary = String::new();
    let mut outcomes = vec![];
    for q in &questions {
        let outcome = if let Some(e) = &stop {
            json!({"question":q,"state":"not_asked","reason":format!("consulting ended: {}", e.kind())})
        } else {
            j.log(json!({"event":"question","id":q.id,"kind":q.kind,"subject":q.subject,"sha256":hash(&q.text),"unix":now()}))?;
            match t.ask(q) {
                Ok(a) => {
                    asked += 1;
                    let text: String = a.text.chars().take(ANSWER_MAX).collect();
                    answered += 1;
                    // Mova re-reads the answer; only its skeletons enter the glossary.
                    let read = reader::read(&format!("## Answer\n\n{text}\n"))?;
                    let skel: Vec<String> = read.sentences.iter().flat_map(|s| s.skeleton.iter().cloned()).collect();
                    glossary.push_str(&format!("«{}»: {}\n", q.subject, skel.iter().map(|s| mova_octopus::realize::sentence(s)).collect::<Vec<_>>().join(" ")));
                    j.log(json!({"event":"answer","id":q.id,"terminal_question":a.terminal_id,"sha256":hash(&text),"words":text.split_whitespace().count(),"ms":a.elapsed_ms,"unix":now()}))?;
                    json!({"question":q,"state":"answered","answer":text,"answer_skeletons":skel})
                }
                Err(e) => {
                    if e != AskError::Unavailable {
                        asked += 1;
                    }
                    j.log(json!({"event":"no_answer","id":q.id,"kind":e.kind(),"reason":e.reason(),"retry":false,"unix":now()}))?;
                    let v = json!({"question":q,"state":e.kind(),"reason":e.reason()});
                    stop = Some(e);
                    v
                }
            }
        };
        qlog.push_str(&outcome.to_string());
        qlog.push('\n');
        outcomes.push(outcome);
    }
    write_sync(&c.run_root.join("questions.jsonl"), qlog.as_bytes())?;
    write_sync(&c.run_root.join("glossary.txt"), glossary.as_bytes())?;
    // Teaching material: every gap, asked or not, with its answer state, its paper and licence.
    // Admissible to the open school and dictionary only from open-licensed text: CC BY / CC0 /
    // public domain. NC, ND, research-only and SA text stays internal (decided here
    // conservatively by the licence name; a reviewer still admits each record).
    let open = open_licence(&c.paper.licence);
    let source = json!({"paper":c.paper.id,"title":r.title,"licence":c.paper.licence,"open_school_admissible":open,"admission":"review_required"});
    let mut teach = String::new();
    for (key, n) in reader::gaps(&r, 64, 0).0 {
        let k = r.forms.get(&key).cloned().unwrap_or(key);
        let state = outcomes.iter().find(|o| o["question"]["subject"] == k.as_str()).map(|o| o["state"].clone()).unwrap_or(json!("not_selected"));
        teach.push_str(&json!({"kind":"unknown_term","term":k,"occurrences":n,"state":state,"source":source}).to_string());
        teach.push('\n');
    }
    for s in r.sentences.iter().filter(|s| !s.admitted || s.skeleton.is_empty()).take(64) {
        teach.push_str(&json!({"kind":if s.admitted {"no_skeleton"} else {"unparsed"},"sentence":s.index,"section":r.sections[s.section].title,"text":s.text,"source":source}).to_string());
        teach.push('\n');
    }
    write_sync(&c.run_root.join("teaching.jsonl"), teach.as_bytes())?;
    let mut m = reader::measure(&r, &d);
    let glossary_words = crate::contract::word_count(&glossary);
    for (k, v) in [
        ("whale", json!(c.whale)),
        ("paper", json!(c.paper.id)),
        ("terminal_session", json!(profile.session)),
        ("words_min", json!(c.words.min)),
        ("words_max", json!(c.words.max)),
        ("within_bounds", json!((c.words.min..=c.words.max).contains(&d.words))),
        ("questions_selected", json!(questions.len())),
        ("questions_asked", json!(asked)),
        ("questions_answered", json!(answered)),
        ("consulting_end", json!(stop.as_ref().map(|e| e.kind()))),
        ("glossary_words", json!(glossary_words)),
        ("read_ms", json!(read_ms)),
        ("description_written_before_answers", json!(true)),
    ] {
        m[k] = v;
    }
    save(&c.run_root.join("metrics.json"), &m)?;
    j.log(json!({"event":"finish","metrics_sha256":hash(serde_json::to_vec(&m)?),"unix":now()}))?;
    Ok(m)
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::terminal::{FakeTerminal, NoTerminal};
    use std::collections::BTreeMap;

    fn model_ok() -> bool {
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
    const PAPER: &str = "# Zorbic Trees for Parsing\n\nA. Author · 2024\n\n<!-- с. 1 -->\n\n## Abstract\n\nWe introduce zorbic trees for dependency parsing. Zorbic trees store every head of a sentence in one array.\n\n## Method\n\nThe parser reads each word once. It attaches every word to its head with a zorbic rule. The rule never creates a cycle in the tree.\n\n## Results\n\nThe parser reaches high accuracy on three treebanks. Zorbic parsing is faster than the baseline on long sentences.\n\n## References\n\nSmith, J. 2020. Trees. Journal of Trees.\n";

    fn config(dir: &Path, name: &str, max_q: usize) -> PaperConfig {
        let p = dir.join(format!("{name}.md"));
        std::fs::write(&p, PAPER).unwrap();
        serde_json::from_value(json!({"schema":SCHEMA,"whale":name,"run_root":dir.join(format!("{name}-run")),
            "paper":{"id":"zorbic-2024-trees","path":p,"sha256":file_hash(&p).unwrap(),"licence":"CC BY 4.0"},
            "words":{"min":40,"max":400},"max_questions":max_q,
            "terminal":crate::terminal::tests::profile(dir, name)})).unwrap()
    }
    fn tmp(tag: &str) -> PathBuf {
        let d = std::env::var_os("CARGO_TARGET_DIR").map(PathBuf::from).unwrap_or_else(std::env::temp_dir).join(format!("whale-paper-{tag}-{}", std::process::id()));
        let _ = std::fs::remove_dir_all(&d);
        std::fs::create_dir_all(&d).unwrap();
        d
    }

    #[test]
    fn only_open_licences_are_marked_admissible() {
        let open = open_licence;
        assert!(open("CC BY 4.0 (ACL Anthology)") && open("CC0 1.0") && open("Public domain"));
        for closed in ["CC BY-NC-SA 3.0 (ACL Anthology, before 2016)", "CC BY-NC 4.0 (ELRA)", "CC BY-SA 4.0", "CC BY-ND 4.0", "research only", ""] {
            assert!(!open(closed), "{closed}");
        }
    }
    #[test]
    fn pod_configs_never_share_a_terminal() {
        let d = tmp("pod");
        let (a, b) = (config(&d, "whale-1", 2), config(&d, "whale-2", 2));
        assert!(check_pod(&[a.clone(), b.clone()]).is_ok());
        let mut shared = a.clone();
        shared.run_root = d.join("other-root");
        assert!(check_pod(&[a.clone(), shared]).unwrap_err().to_string().contains("shared"));
        let mut foreign = b.clone();
        foreign.terminal = a.terminal.clone();
        assert!(foreign.validate().unwrap_err().to_string().contains("belongs to whale-1"), "another Whale's terminal");
        // Same session in a second profile file: refused by the pod check.
        let p = d.join("copy.json");
        let mut v: Value = serde_json::from_slice(&std::fs::read(&a.terminal.profile).unwrap()).unwrap();
        v["animal"] = json!("whale-2");
        v["root"] = json!(d.join("copy-root"));
        v["owner_marker"] = json!("mova-copy");
        std::fs::write(&p, v.to_string()).unwrap();
        let mut same_session = b.clone();
        same_session.terminal.profile = p;
        assert!(check_pod(&[a.clone(), same_session]).unwrap_err().to_string().contains("terminal session"));
        let _ = std::fs::remove_dir_all(d);
    }

    #[test]
    fn whale_writes_first_asks_after_and_stops_on_limit() {
        if !model_ok() {
            return;
        }
        let d = tmp("run");
        let c = config(&d, "whale-1", 3);
        let mut fake = FakeTerminal { animal: "whale-1".into(), answers: BTreeMap::from([("Zorbic".to_string(), "A zorbic rule attaches a word to its head.".to_string())]), limit_after: Some(1), received: vec![] };
        let m = run(&c, &mut fake).unwrap();
        // The draft is journaled before the first question and is the final description.
        let journal = std::fs::read_to_string(c.run_root.join("journal.jsonl")).unwrap();
        let events: Vec<String> = journal.lines().map(|l| serde_json::from_str::<Value>(l).unwrap()["event"].as_str().unwrap().to_string()).collect();
        let draft = events.iter().position(|e| e == "draft").unwrap();
        assert!(events.iter().position(|e| e == "question").unwrap() > draft, "{events:?}");
        let desc = std::fs::read_to_string(c.run_root.join("description.txt")).unwrap();
        assert!(!desc.contains("attaches a word to its head."), "the answer reached the description");
        assert!(desc.contains("«Method»"), "{desc}");
        assert!(!desc.contains("Journal of Trees"), "references are not read as content");
        // «Zorbic» and «zorbic» are one term (normalized): asked once, answered; a further
        // question, if any, meets the limit.
        assert_eq!(m["questions_answered"], 1);
        assert!(m["consulting_end"].is_null() || m["consulting_end"] == "limit", "{m}");
        assert!(fake.received.iter().filter(|q| q.subject.eq_ignore_ascii_case("zorbic")).count() == 1, "{:?}", fake.received);
        assert!(fake.received.len() <= 2, "asked after the limit");
        assert_eq!(m["traceability"], 1.0);
        assert!(m["section_coverage"].as_f64().unwrap() >= 0.99, "{m}");
        assert!(std::fs::read_to_string(c.run_root.join("glossary.txt")).unwrap().starts_with("«Zorbic»"));
        let teach = std::fs::read_to_string(c.run_root.join("teaching.jsonl")).unwrap();
        assert!(teach.lines().all(|l| serde_json::from_str::<Value>(l).unwrap()["source"]["open_school_admissible"] == true), "{teach}");
        // The terminal is held for the run only; a repeated run into the same root is refused.
        assert!(run(&c, &mut NoTerminal("whale-1".into())).unwrap_err().to_string().contains("never repeated"));
        // Another Whale's terminal is refused before anything is read.
        let c2 = config(&d, "whale-2", 1);
        assert!(run(&c2, &mut NoTerminal("whale-1".into())).unwrap_err().to_string().contains("belongs to whale-1, not to whale-2"));
        assert!(!c2.run_root.exists());
        let _ = std::fs::remove_dir_all(d);
    }
}
