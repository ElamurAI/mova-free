//! Batches: a pod of Whales, each on its own paper; rolling assignment, one paper per Whale at a time.
//!
//! State model: `run_root/batches.jsonl` (append-only, fsynced) holds `opened` events (batch
//! number, assignments whale -> paper with the text's SHA-256 pinned at opening, opening time,
//! deadline) and `closed` events (reason `all_passed`, `exhausted` or `deadline`, each learner's
//! status and outcome, the unpassed papers in `retry`). A batch opened by the rolling runner holds
//! one assignment: one Whale's paper with its own deadline (`hours`, half a day). A Whale holds at
//! most one unfinished assignment. When it is final (`final_reason`: passed, asked out, or its
//! deadline) its batch closes and the Whale takes the next paper at once: its own retry under a
//! changed writer first, else the next eligible paper in a deterministic order — open licence
//! (CC BY, CC0, public domain; no NC, ND, SA), a Markdown text that is not local-only, the
//! configured `priority` ids first (in their order), then papers our code cites (an `<id>.md`
//! page in `cited_dir`), then, when `include_uncited`, the rest of the catalog by relevance (see
//! `Selection`). A paper is assigned at most once over all batches. For every assignment `plan`
//! writes a learner configuration (`batch-NNN/<whale>.learn.json`) whose deadline is its batch's.
//! Each Whale also has its own journal, `run_root/whales/<whale>.jsonl` (`assigned`, `started`,
//! `ended`, `closed`); each paper its learner journal under the Whale's state root.
//!
//! `roll` (`batch` live) is the one runner of a run root (it holds `plan.lock`): it starts each
//! Whale's learner as soon as it may, in its own thread with its own terminal (`learn::run`), and
//! re-plans whenever a learner ends. A learner that ended unfinished (a limit, a timeout, a
//! terminal failure, an error) continues the same paper with new questions only after
//! `wait_minutes`, until that paper's deadline; a usage limit pauses the whole pod for
//! `wait_minutes`. Restarted, the runner continues from `batches.jsonl`, the Whales' journals and
//! the learners' states; nothing asked is asked again (`learn`).
use crate::{
    gates::Params,
    knowledge::{self, Store},
    learn::{self, LearnConfig},
    paper::{open_licence, PaperSource, Words},
    terminal::{MovaTerminal, NoTerminal, PrivateTerminal, TerminalConfig},
    util::{error, file_hash, now, private_dir, save, try_lease, write_sync},
    Result,
};
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::BTreeSet,
    fs::OpenOptions,
    io::{BufRead, Write},
    os::unix::fs::OpenOptionsExt,
    path::{Path, PathBuf},
};

pub const SCHEMA: &str = "mova-shark-batch-v1";

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Catalog {
    pub path: PathBuf,
    pub sha256: String,
    #[serde(default = "d_id")]
    pub id_field: String,
    #[serde(default = "d_path")]
    pub path_field: String,
    #[serde(default = "d_title")]
    pub title_field: String,
    #[serde(default = "d_licence")]
    pub licence_field: String,
    /// A boolean field that marks texts which may not leave the machine.
    #[serde(default)]
    pub local_only_field: Option<String>,
}
fn d_id() -> String {
    "id".into()
}
fn d_path() -> String {
    "text_path".into()
}
fn d_title() -> String {
    "title".into()
}
fn d_licence() -> String {
    "licence".into()
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Slot {
    /// The learner's label; it must equal its terminal profile's `animal`.
    #[serde(alias = "shark")]
    pub whale: String,
    /// Earlier labels of this learner (for example `shark-1` before the Whale rename): open
    /// batches, retries and knowledge recorded under them belong to this slot.
    #[serde(default)]
    pub formerly: Vec<String>,
    pub state_root: PathBuf,
    pub terminal: TerminalConfig,
    /// Earlier `mova-whale paper` run roots of this Whale to import (once) as retained knowledge.
    #[serde(default)]
    pub import: Vec<PathBuf>,
}

#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct BatchConfig {
    pub schema: String,
    pub run_root: PathBuf,
    pub catalog: Catalog,
    /// Pages of the papers our code cites: `<id>.md`.
    #[serde(default)]
    pub cited_dir: Option<PathBuf>,
    #[serde(default)]
    pub include_uncited: bool,
    /// Papers taken first, in this order (for example the papers the Whales already work on).
    #[serde(default)]
    pub priority: Vec<String>,
    #[serde(default)]
    pub exclude: Vec<String>,
    #[serde(default = "d_hours")]
    pub hours: f64,
    #[serde(alias = "sharks")]
    pub whales: Vec<Slot>,
    pub words: Words,
    #[serde(default = "d_questions")]
    pub max_questions: usize,
    #[serde(default = "d_rounds")]
    pub max_rounds: usize,
    #[serde(default)]
    pub gates: Params,
    /// Minutes the persistent `batch` waits before it continues learners stopped by a usage limit,
    /// a timeout or a terminal failure (new questions only), within the batch's deadline.
    #[serde(default = "d_wait")]
    pub wait_minutes: u64,
    /// The pod's shared knowledge directory (`pod.rs`): one store keyed by normalized term for all
    /// learners, so no term is asked twice across the pod. None: each learner on its own.
    #[serde(default)]
    pub pod: Option<PathBuf>,
    /// Rounds in a row without gain after which a learner stops its paper (`learn`, `stalled`).
    #[serde(default = "d_stall")]
    pub stall_rounds: usize,
    /// One terminal call per round (default).
    #[serde(default = "d_batched")]
    pub batched: bool,
    /// Extraction limits: a paper whose text exceeds one is refused before it is assigned and
    /// recorded for re-extraction (`refused` events).
    #[serde(default)]
    pub extraction: crate::reader::ExtractionLimits,
    /// Review hints from each learner's own terminal (`review.rs`).
    #[serde(default)]
    pub review: crate::review::ReviewConfig,
    /// How papers beyond `priority` are chosen (`Selection`).
    #[serde(default)]
    pub selection: Selection,
    /// The closed topic corpus (`topic.rs`): built with the paper world when a paper is assigned
    /// (`topic_corpus` events), seeded before each round's questions, cited per round.
    #[serde(default)]
    pub topic_corpus: Option<crate::topic::TopicCorpus>,
}
fn d_stall() -> usize {
    3
}

/// Which catalog papers a batch takes after its `priority` ids, in which order. Deterministic;
/// never alphabetical (the id only breaks exact ties).
///
/// Rank: papers our code cites (`cited_dir`) first, then the rest of the catalog only when it
/// matches a topic (row `concepts` and `topics`): core topics (`core_concepts`), then general NLP
/// (`nlp_concepts`); within each rank by citations inside our library (`library_citations_field`), then by all citations
/// (`citations_field`). A candidate must have at least `min_words` words of restatable prose
/// (the sentences Mova reads after the content filter, `reader::read`) and a healthy extraction
/// (`extraction`); a shared-task system description (title patterns) needs `min_citations_system`
/// citations unless our code cites it. A paper refused for length or extraction is recorded once
/// (`refused` event) and not read again.
#[derive(Clone, Debug, Deserialize, Serialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Selection {
    /// Core topics (syntax, UD, parsing, semantics, pragmatics, reading): ranked first after the
    /// papers our code cites.
    #[serde(default = "d_core")]
    pub core_concepts: Vec<String>,
    /// General NLP topics: ranked after the core topics.
    #[serde(default = "d_nlp")]
    pub nlp_concepts: Vec<String>,
    /// Uncited papers must match a core or NLP concept.
    #[serde(default = "d_on")]
    pub require_core: bool,
    #[serde(default = "d_min_words")]
    pub min_words: usize,
    #[serde(default = "d_lib_field")]
    pub library_citations_field: String,
    #[serde(default = "d_cit_field")]
    pub citations_field: String,
    #[serde(default = "d_system")]
    pub system_patterns: Vec<String>,
    #[serde(default = "d_min_sys")]
    pub min_citations_system: u64,
}
fn d_on() -> bool {
    true
}
fn d_core() -> Vec<String> {
    [
        "parsing", "dependency-parsing", "universal-dependencies", "treebanks", "syntax-grammar", "context-free-grammar", "construction-grammar", "semantic-parsing", "semantics",
        "lexical-semantics", "distributional-semantics", "semantic-role-labeling", "discourse-analysis", "ud-discourse", "coreference-resolution", "morphology", "part-of-speech-tagging",
        "grammatical-error-correction", "child-directed-speech", "pragmatics", "reading-comprehension",
    ]
    .iter()
    .map(|x| x.to_string())
    .collect()
}
fn d_nlp() -> Vec<String> {
    ["natural-language-processing", "natural-language-understanding", "annotation", "tokenization", "corpus-linguistics", "lexicography", "word-sense-disambiguation"].iter().map(|x| x.to_string()).collect()
}
fn d_min_words() -> usize {
    6000
}
fn d_lib_field() -> String {
    "our_citations".into()
}
fn d_cit_field() -> String {
    "cited_by".into()
}
fn d_system() -> Vec<String> {
    ["shared task", "system description", "semeval", "dravidianlangtech", "@ ", " at wmt", "submission to", "team "].iter().map(|x| x.to_string()).collect()
}
fn d_min_sys() -> u64 {
    50
}
impl Default for Selection {
    fn default() -> Self {
        Self { core_concepts: d_core(), nlp_concepts: d_nlp(), require_core: true, min_words: d_min_words(), library_citations_field: d_lib_field(), citations_field: d_cit_field(), system_patterns: d_system(), min_citations_system: d_min_sys() }
    }
}

/// Words of restatable prose: the sentences Mova reads after the content filter.
pub fn prose_words(markdown: &str) -> Result<usize> {
    let r = crate::reader::read(markdown)?;
    Ok(r.sentences.iter().map(|s| s.text.split_whitespace().count()).sum())
}
fn d_batched() -> bool {
    true
}
fn d_wait() -> u64 {
    30
}
fn d_hours() -> f64 {
    12.0
}
fn d_questions() -> usize {
    4
}
fn d_rounds() -> usize {
    24
}

impl Slot {
    /// Whether an assignment recorded under `label` belongs to this slot.
    pub fn answers_to(&self, label: &str) -> bool {
        self.whale == label || self.formerly.iter().any(|f| f == label)
    }
}

/// The slot an assignment (or a retry record) belongs to; old records name the learner `shark`.
fn slot_of<'a>(c: &'a BatchConfig, a: &Value) -> Option<&'a Slot> {
    let label = crate::util::legacy(a, "whale").as_str()?;
    c.whales.iter().find(|s| s.answers_to(label))
}

/// What paper selection reads (`candidates`): the catalog and the ranking rules. A batch and an
/// epoch (`epoch.rs`) choose papers by the same rules.
pub struct Pick<'a> {
    pub catalog: &'a Catalog,
    pub cited_dir: Option<&'a Path>,
    pub include_uncited: bool,
    pub priority: &'a [String],
    pub exclude: &'a [String],
    pub extraction: &'a crate::reader::ExtractionLimits,
    pub selection: &'a Selection,
}

impl BatchConfig {
    pub fn pick(&self) -> Pick<'_> {
        Pick { catalog: &self.catalog, cited_dir: self.cited_dir.as_deref(), include_uncited: self.include_uncited, priority: &self.priority, exclude: &self.exclude, extraction: &self.extraction, selection: &self.selection }
    }
    pub fn load(p: &Path) -> Result<Self> {
        let c: BatchConfig = serde_json::from_value(crate::util::load(p, 1 << 20)?)?;
        c.validate()?;
        Ok(c)
    }
    pub fn validate(&self) -> Result<()> {
        if self.schema != SCHEMA || !self.run_root.is_absolute() || !self.catalog.path.is_absolute() || !crate::util::valid_hash(&self.catalog.sha256) {
            return Err(error("batch config: schema, absolute run_root and a pinned absolute catalog required"));
        }
        if self.whales.is_empty() || self.whales.len() > 16 || !(self.hours > 0.0 && self.hours <= 168.0) || self.words.min == 0 || self.words.min > self.words.max || !(1..=720).contains(&self.wait_minutes) {
            return Err(error("batch config: 1..16 learners, 0 < hours <= 168, 0 < words.min <= words.max, wait_minutes 1..720"));
        }
        if self.cited_dir.as_ref().is_some_and(|d| !d.is_absolute()) || self.pod.as_ref().is_some_and(|d| !d.is_absolute()) {
            return Err(error("batch config: cited_dir and pod must be absolute"));
        }
        if !(1..=50).contains(&self.stall_rounds) {
            return Err(error("batch config: stall_rounds 1..50"));
        }
        if let Some(t) = &self.topic_corpus {
            t.validate()?;
        }
        self.review.validate()?;
        let mut seen = BTreeSet::new();
        for s in &self.whales {
            let p = s.terminal.load()?;
            if p.animal != s.whale || !s.state_root.is_absolute() {
                return Err(error(format!("batch config: {} needs its own terminal and an absolute state_root", s.whale)));
            }
            for k in s.formerly.iter().cloned().chain([s.whale.clone(), s.state_root.display().to_string(), s.terminal.profile.display().to_string(), p.session.clone(), p.owner_marker.clone(), p.root.display().to_string()]) {
                if !seen.insert(k.clone()) {
                    return Err(error(format!("batch config: {k} is shared between Whales")));
                }
            }
        }
        self.gates.validate()
    }
}

fn events(c: &BatchConfig) -> Result<Vec<Value>> {
    let p = c.run_root.join("batches.jsonl");
    if !p.exists() {
        return Ok(vec![]);
    }
    std::fs::read_to_string(p)?.lines().filter(|l| !l.trim().is_empty()).map(|l| Ok(serde_json::from_str(l)?)).collect()
}
fn append(c: &BatchConfig, v: &Value) -> Result<()> {
    let mut f = OpenOptions::new().create(true).append(true).mode(0o600).open(c.run_root.join("batches.jsonl"))?;
    let mut line = v.to_string();
    line.push('\n');
    f.write_all(line.as_bytes())?;
    f.sync_data()?;
    Ok(())
}

/// Stop reasons no further invocation can change under the same writer (`learn::EXHAUSTED`).
pub use crate::learn::EXHAUSTED;

/// Papers to retry: those whose latest batch closed with them unpassed (`retry`), under a writer
/// other than the current one (`compose::WRITER`), with the learner that worked on them.
fn retries(ev: &[Value]) -> Vec<(String, Value)> {
    let mut last: std::collections::BTreeMap<String, (u64, Option<(String, Value)>)> = Default::default();
    for e in ev {
        let b = e["batch"].as_u64().unwrap_or(0);
        if e["event"] == "opened" {
            for a in e["assignments"].as_array().cloned().unwrap_or_default() {
                if let Some(id) = a["paper"]["id"].as_str() {
                    last.insert(id.to_string(), (b, None));
                }
            }
        } else if e["event"] == "closed" {
            for r in e["retry"].as_array().cloned().unwrap_or_default() {
                let Some(id) = r["paper"]["id"].as_str() else { continue };
                if last.get(id).is_some_and(|x| x.0 == b) && r["writer"].as_str() != Some(crate::compose::WRITER) {
                    last.insert(id.to_string(), (b, Some((crate::util::legacy(&r, "whale").as_str().unwrap_or("").to_string(), r["paper"].clone()))));
                }
            }
        }
    }
    let mut out: Vec<(u64, String, String, Value)> = last.into_iter().filter_map(|(id, (b, r))| r.map(|(sh, p)| (b, id, sh, p))).collect();
    out.sort_by(|a, b| (a.0, &a.1).cmp(&(b.0, &b.1)));
    out.into_iter().map(|(_, _, sh, p)| (sh, p)).collect()
}

/// The learner configuration of one assignment.
pub fn learn_config(c: &BatchConfig, batch: &Value, a: &Value) -> Result<LearnConfig> {
    let slot = slot_of(c, a).ok_or_else(|| error(format!("no Whale slot for {}", crate::util::legacy(a, "whale"))))?;
    let lc = LearnConfig {
        schema: learn::SCHEMA.into(),
        whale: slot.whale.clone(),
        state_root: slot.state_root.clone(),
        paper: serde_json::from_value(a["paper"].clone())?,
        words: c.words.clone(),
        max_questions: c.max_questions,
        max_rounds: c.max_rounds,
        hours: c.hours,
        deadline_unix: batch["deadline"].as_u64(),
        gates: c.gates.clone(),
        import: slot.import.clone(),
        terminal: slot.terminal.clone(),
        stall_rounds: c.stall_rounds,
        pod: c.pod.clone(),
        batched: c.batched,
        review: c.review.clone(),
        topic_corpus: c.topic_corpus.clone(),
        stop_file: Some(c.run_root.join("STOP")),
    };
    lc.validate()?;
    Ok(lc)
}

/// Ids refused for a broken extraction (`refused` events).
fn refused(ev: &[Value]) -> BTreeSet<String> {
    ev.iter().filter(|e| e["event"] == "refused").filter_map(|e| e["paper"]["id"].as_str().map(String::from)).collect()
}

/// A paper's extraction check: None when fit to read, else the `refused` event to record.
fn extraction_refusal(c: &Pick, p: &Value) -> Option<Value> {
    let path = p["path"].as_str()?;
    let text = std::fs::read_to_string(path).ok()?;
    let m = crate::reader::extraction(&text);
    let broken = m.broken(&c.extraction);
    (!broken.is_empty()).then(|| json!({"event":"refused","paper":p,"reason":"extraction","broken":broken,"extraction":m,"limits":c.extraction,"action":"re-extract the text, then assign it under a new id or hash","unix":now()}))
}

/// A paper too short to describe: a quick raw word count first, then the prose Mova really reads
/// (`prose_words`); None when long enough, else the `refused` event to record.
fn short_refusal(c: &Pick, p: &Value) -> Option<Value> {
    let min = c.selection.min_words;
    if min == 0 {
        return None;
    }
    let text = std::fs::read_to_string(p["path"].as_str()?).ok()?;
    let raw = text.split_whitespace().count();
    let prose = if raw < min { raw } else { prose_words(&text).unwrap_or(0) };
    (prose < min).then(|| json!({"event":"refused","paper":p,"reason":"short","prose_words":prose,"raw_words":raw,"min_words":min,"unix":now()}))
}

/// Eligible papers in the deterministic order, skipping `taken`. Papers whose extraction is broken
/// are refused (returned as `refused` events to record) and passed over.
pub fn candidates(c: &Pick, taken: &BTreeSet<String>, n: usize) -> Result<(Vec<PaperSource>, Vec<Value>)> {
    if file_hash(&c.catalog.path)? != c.catalog.sha256 {
        return Err(error("catalog hash mismatch; no batch opened"));
    }
    let cited: BTreeSet<String> = match &c.cited_dir {
        Some(d) => std::fs::read_dir(d)?.filter_map(|e| e.ok()).filter_map(|e| e.file_name().to_str().and_then(|n| n.strip_suffix(".md")).map(String::from)).collect(),
        None => BTreeSet::new(),
    };
    let sel = &c.selection;
    let num = |v: &Value, f: &str| v[f].as_u64().or_else(|| v[f].as_f64().map(|x| x as u64)).unwrap_or(0);
    let mut rows: Vec<((usize, usize, u64, u64), String, PaperSource)> = vec![];
    let f = std::io::BufReader::new(std::fs::File::open(&c.catalog.path)?);
    for line in f.lines() {
        let line = line?;
        if line.trim().is_empty() {
            continue;
        }
        let v: Value = serde_json::from_str(&line)?;
        let (Some(id), Some(path)) = (v[&c.catalog.id_field].as_str(), v[&c.catalog.path_field].as_str()) else { continue };
        let licence = v[&c.catalog.licence_field].as_str().unwrap_or("");
        let local = c.catalog.local_only_field.as_ref().is_some_and(|f| v[f] == true);
        let is_cited = cited.contains(id);
        let prio = c.priority.iter().position(|x| x == id);
        if taken.contains(id) || c.exclude.iter().any(|x| x == id) || !open_licence(licence) || local || !path.ends_with(".md") || !Path::new(path).is_absolute() || (!is_cited && !c.include_uncited && prio.is_none()) {
            continue;
        }
        let lib = num(&v, &sel.library_citations_field);
        let all = num(&v, &sel.citations_field);
        let tags: Vec<&str> = ["concepts", "topics"].iter().flat_map(|f| v[*f].as_array().map(|a| a.iter().filter_map(|x| x.as_str()).collect::<Vec<_>>()).unwrap_or_default()).collect();
        let core = tags.iter().any(|t| sel.core_concepts.iter().any(|k| k == t));
        let nlp = tags.iter().any(|t| sel.nlp_concepts.iter().any(|k| k == t));
        if prio.is_none() && !is_cited {
            if sel.require_core && !core && !nlp {
                continue;
            }
            let title = v[&c.catalog.title_field].as_str().unwrap_or("").to_lowercase();
            if sel.system_patterns.iter().any(|p| title.contains(&p.to_lowercase())) && all < sel.min_citations_system {
                continue;
            }
        }
        // Most cited first (inside our library, then overall): u64::MAX - n sorts descending.
        let rank = match prio {
            Some(k) => (0, k, 0, 0),
            None if is_cited => (1, 0, u64::MAX - lib, u64::MAX - all),
            None if core => (2, 0, u64::MAX - lib, u64::MAX - all),
            None => (3, 0, u64::MAX - lib, u64::MAX - all),
        };
        rows.push((rank, id.to_string(), PaperSource { id: id.into(), path: path.into(), sha256: String::new(), title: v[&c.catalog.title_field].as_str().unwrap_or("").into(), licence: licence.into() }));
    }
    rows.sort_by(|a, b| (a.0, &a.1).cmp(&(b.0, &b.1)));
    let mut out = vec![];
    let mut refusals = vec![];
    for (_, _, mut p) in rows {
        if out.len() >= n {
            break;
        }
        // A text that cannot be read now is passed over (it stays eligible for a later batch).
        let Ok(h) = file_hash(&p.path) else { continue };
        p.sha256 = h;
        let v = json!({"id":p.id,"path":p.path,"sha256":p.sha256,"title":p.title,"licence":p.licence});
        if let Some(e) = extraction_refusal(c, &v) {
            refusals.push(e);
            continue;
        }
        if let Some(e) = short_refusal(c, &v) {
            refusals.push(e);
            continue;
        }
        out.push(p);
    }
    Ok((out, refusals))
}

/// What became of one assignment: `Some(reason)` once it is final — `passed`; asked out
/// (`EXHAUSTED`) in this assignment, or under the current writer (such a paper is not reopened);
/// `deadline` (the learner stopped on this assignment's deadline, or the deadline has come) —
/// else None: its Whale can still go on with it (not started, learning, or stopped by a limit, a
/// timeout, a terminal failure or `STOP`).
pub fn final_reason(st: &Option<Value>, deadline: u64, at: u64) -> Option<String> {
    if let Some(s) = st {
        // The status is this assignment's when its deadline is this one's (a retry reopens an older
        // status with the new deadline).
        let this = s["deadline"].as_u64().unwrap_or(0) >= deadline;
        let reason = s["reason"].as_str().unwrap_or("");
        if s["state"] == "passed" {
            return Some("passed".into());
        }
        let out = EXHAUSTED.contains(&reason) && (this || s["last_round"]["writer"] == json!(crate::compose::WRITER));
        if s["state"] == "stopped" && (out || (reason == "deadline" && this)) {
            return Some(reason.into());
        }
    }
    (at >= deadline).then(|| "deadline".into())
}

/// The Whale's own journal (`whales/<whale>.jsonl`, append-only, fsynced): `assigned`, `started`,
/// `ended`, `closed`.
fn whale_log(c: &BatchConfig, whale: &str, v: &Value) -> Result<()> {
    let dir = c.run_root.join("whales");
    private_dir(&dir)?;
    let mut f = OpenOptions::new().create(true).append(true).mode(0o600).open(dir.join(format!("{whale}.jsonl")))?;
    let mut line = v.to_string();
    line.push('\n');
    f.write_all(line.as_bytes())?;
    f.sync_data()?;
    Ok(())
}
fn whale_events(c: &BatchConfig, whale: &str) -> Vec<Value> {
    std::fs::read_to_string(c.run_root.join("whales").join(format!("{whale}.jsonl"))).unwrap_or_default().lines().filter_map(|l| serde_json::from_str(l).ok()).collect()
}

/// When this assignment's learner last ended unfinished, and why: the later of the Whale's journal
/// (`ended`) and a status of this assignment (written by a learner of an earlier runner).
fn last_end(journal: &[Value], batch: u64, st: &Option<Value>, deadline: u64) -> Option<(u64, String)> {
    let a = journal.iter().rev().find(|e| e["event"] == "ended" && e["batch"].as_u64() == Some(batch)).map(|e| (e["unix"].as_u64().unwrap_or(0), e["reason"].as_str().unwrap_or("").to_string()));
    let b = st.as_ref().filter(|s| s["state"] == "stopped" && s["deadline"].as_u64().unwrap_or(0) >= deadline).map(|s| (s["ended"].as_u64().unwrap_or(0), s["reason"].as_str().unwrap_or("").to_string()));
    match (a, b) {
        (Some(x), Some(y)) => Some(if y.0 > x.0 { y } else { x }),
        (x, y) => x.or(y),
    }
}

/// The open batches (`opened` without `closed`), in opening order.
fn open_batches(ev: &[Value]) -> Vec<Value> {
    let closed: BTreeSet<u64> = ev.iter().filter(|e| e["event"] == "closed").filter_map(|e| e["batch"].as_u64()).collect();
    ev.iter().filter(|e| e["event"] == "opened" && !e["batch"].as_u64().is_some_and(|b| closed.contains(&b))).cloned().collect()
}

/// Plan once (under `plan.lock`): close what is done, give every free Whale its next paper.
pub fn plan(c: &BatchConfig) -> Result<Value> {
    c.validate()?;
    private_dir(&c.run_root)?;
    let _l = try_lease(&c.run_root.join("plan.lock"), true)?.ok_or_else(|| error("another plan or runner holds this run root (plan.lock)"))?;
    plan_at(c, &Default::default(), true, c.wait_minutes * 60)
}

/// Rolling assignment, one step (the caller holds `plan.lock`). Whales in `busy` are learning now, each in the batch given (a Whale is busy only in that batch: a paper it finished in an older, still-open batch is final there).
///
/// 1. Every open batch whose assignments are all final (`final_reason`) closes: `all_passed`,
///    `deadline` when an assignment ran out of time, else `exhausted`; unpassed papers go to `retry`.
/// 2. Each Whale's open assignment is its one non-final assignment of an open batch. It may start
///    at once, or `wait` seconds after its learner last ended unfinished (not after `stop`).
/// 3. A usage limit pauses the whole pod (one subscription): `wait` seconds from the latest
///    `limit` end of an open assignment. Meanwhile no paper is assigned and no learner starts.
/// 4. With `assign` and no pause, every free Whale (no open assignment), in slot order, takes its
///    next paper at once: its own retry under a changed writer first, else the next eligible paper
///    of the selection that no Whale holds or has had. Each assignment is its own batch (one
///    `opened` event with one assignment; deadline `hours` from now).
pub fn plan_at(c: &BatchConfig, busy: &std::collections::BTreeMap<String, u64>, assign: bool, wait: u64) -> Result<Value> {
    let at = now();
    let ev = events(c)?;
    let mut closed_now = vec![];
    for b in open_batches(&ev) {
        let deadline = b["deadline"].as_u64().unwrap_or(u64::MAX);
        let mut states = vec![];
        for a in b["assignments"].as_array().cloned().unwrap_or_default() {
            let lc = learn_config(c, &b, &a)?;
            let st = learn::paper_status(&lc)?;
            let fin = if busy.get(&lc.whale) == b["batch"].as_u64().as_ref() { None } else { final_reason(&st, deadline, at) };
            states.push((lc.whale.clone(), a, st, fin));
        }
        if states.is_empty() || states.iter().any(|s| s.3.is_none()) {
            continue;
        }
        let n = states.len();
        let passed = states.iter().filter(|s| s.3.as_deref() == Some("passed")).count();
        let reason = if passed == n {
            "all_passed"
        } else if states.iter().any(|s| s.3.as_deref() == Some("deadline")) {
            "deadline"
        } else {
            "exhausted"
        };
        // Unpassed papers are kept for a retry under a later writer, with their learner and the
        // writer of their last round (none recorded: an earlier writer).
        let retry: Vec<Value> = states.iter().filter(|s| s.3.as_deref() != Some("passed")).map(|(w, a, st, _)| {
            let st = st.clone().unwrap_or(Value::Null);
            json!({"whale":w,"paper":a["paper"],"reason":st["reason"],"words":st["last_round"]["words"],"writer":st["last_round"]["writer"]})
        }).collect();
        let learners: Vec<Value> = states.iter().map(|(w, a, st, f)| json!({"whale":w,"paper":a["paper"]["id"],"status":st,"outcome":f})).collect();
        let e = json!({"event":"closed","batch":b["batch"],"reason":reason,"passed":passed,"of":n,"learners":learners,"retry":retry,"unix":at});
        append(c, &e)?;
        for (w, a, _, f) in &states {
            whale_log(c, w, &json!({"event":"closed","batch":b["batch"],"paper":a["paper"]["id"],"outcome":f,"unix":at}))?;
        }
        closed_now.push(e);
    }
    let ev = events(c)?;
    // Each Whale's open assignment and when it may start.
    let mut open: std::collections::BTreeMap<String, Value> = Default::default();
    let mut pause_until = 0u64;
    for b in open_batches(&ev) {
        let n = b["batch"].as_u64().unwrap_or(0);
        let deadline = b["deadline"].as_u64().unwrap_or(u64::MAX);
        for a in b["assignments"].as_array().cloned().unwrap_or_default() {
            let lc = learn_config(c, &b, &a)?;
            let st = learn::paper_status(&lc)?;
            let running = busy.get(&lc.whale) == Some(&n);
            if !running && final_reason(&st, deadline, at).is_some() {
                continue;
            }
            let end = last_end(&whale_events(c, &lc.whale), n, &st, deadline);
            let start_at = match &end {
                Some((t, r)) if r != "stop" => t + wait,
                _ => at,
            };
            if let Some((t, r)) = &end {
                if r == "limit" && !running {
                    pause_until = pause_until.max(t + wait);
                }
            }
            if let Some(prev) = open.get(&lc.whale) {
                return Err(error(format!("{} holds two open assignments (batches {} and {n})", lc.whale, prev["batch"])));
            }
            open.insert(lc.whale.clone(), json!({"whale":lc.whale,"batch":n,"paper":lc.paper.id,"deadline":deadline,"start_at":start_at,"running":running,"last_end":end.map(|(t, r)| json!({"unix":t,"reason":r})),"opened":b,"assignment":a}));
        }
    }
    let paused = pause_until > at;
    let free: Vec<&Slot> = c.whales.iter().filter(|s| !open.contains_key(&s.whale)).collect();
    let mut opened = vec![];
    let mut refusals: Vec<Value> = vec![];
    let mut topics = vec![];
    let mut unassigned = 0;
    if assign && !paused && !free.is_empty() {
        let mut taken: BTreeSet<String> = ev.iter().filter(|e| e["event"] == "opened").flat_map(|e| e["assignments"].as_array().cloned().unwrap_or_default()).filter_map(|a| a["paper"]["id"].as_str().map(String::from)).collect();
        let no = refused(&ev);
        taken.extend(no.iter().cloned());
        // Retries first, each with the Whale that holds its knowledge (its text unchanged).
        let mut picks: Vec<(&Slot, Value, bool)> = vec![];
        let mut retried: BTreeSet<String> = BTreeSet::new();
        for (label, paper) in retries(&ev) {
            let Some(slot) = free.iter().find(|s| s.answers_to(&label)) else { continue };
            let id = paper["id"].as_str().unwrap_or("").to_string();
            if no.contains(&id) || retried.contains(&slot.whale) {
                continue;
            }
            let same = paper["path"].as_str().and_then(|p| file_hash(Path::new(p)).ok()).is_some_and(|h| paper["sha256"].as_str() == Some(h.as_str()));
            if !same {
                continue;
            }
            // A retry whose text is broken or too short is refused too (recorded once), not retried.
            if let Some(e) = extraction_refusal(&c.pick(), &paper).or_else(|| short_refusal(&c.pick(), &paper)) {
                refusals.push(e);
                continue;
            }
            retried.insert(slot.whale.clone());
            picks.push((slot, paper, true));
        }
        let rest: Vec<&Slot> = free.iter().filter(|s| !retried.contains(&s.whale)).copied().collect();
        let (papers, more) = candidates(&c.pick(), &taken, rest.len())?;
        refusals.extend(more);
        unassigned = rest.len() - papers.len().min(rest.len());
        for (s, p) in rest.iter().zip(papers.iter()) {
            picks.push((s, json!({"id":p.id,"path":p.path,"sha256":p.sha256,"title":p.title,"licence":p.licence}), false));
        }
        for e in &refusals {
            append(c, e)?;
        }
        picks.sort_by_key(|(s, _, _)| c.whales.iter().position(|x| x.whale == s.whale));
        let mut number = ev.iter().filter(|e| e["event"] == "opened").count() as u64;
        for (slot, paper, retry) in picks {
            number += 1;
            let mut a = json!({"whale":slot.whale,"paper":paper});
            if retry {
                a["retry"] = json!(true);
            }
            let deadline = at + (c.hours * 3600.0) as u64;
            let b = json!({"event":"opened","batch":number,"assignments":[a],"opened":at,"deadline":deadline,"catalog_sha256":c.catalog.sha256,"rolling":true});
            let dir = c.run_root.join(format!("batch-{number:03}"));
            private_dir(&dir)?;
            let lc = learn_config(c, &b, &a)?;
            let v = json!({"schema":learn::SCHEMA,"whale":lc.whale,"state_root":lc.state_root,"paper":a["paper"],"words":{"min":lc.words.min,"max":lc.words.max},
                "max_questions":lc.max_questions,"max_rounds":lc.max_rounds,"hours":lc.hours,"deadline_unix":deadline,"gates":lc.gates,"import":lc.import,"terminal":{"profile":lc.terminal.profile,"poll_ms":lc.terminal.poll_ms},
                "stall_rounds":lc.stall_rounds,"pod":lc.pod,"batched":lc.batched,"review":lc.review,"topic_corpus":lc.topic_corpus,"stop_file":lc.stop_file});
            save(&dir.join(format!("{}.learn.json", lc.whale)), &v)?;
            append(c, &b)?;
            whale_log(c, &lc.whale, &json!({"event":"assigned","batch":number,"paper":lc.paper.id,"retry":retry,"deadline":deadline,"unix":at}))?;
            // The paper's closed topic corpus and small world (a failure is recorded; the learner
            // then learns without them).
            if let Some(tc) = c.topic_corpus.as_ref().filter(|t| t.build_on_assign) {
                let v = tc.build(&lc.paper.id);
                append(c, &json!({"event":"topic_corpus","batch":number,"paper":lc.paper.id,"build":v["build"]["ok"],"world":v["world"]["ok"],"result":v,"unix":now()}))?;
                topics.push(json!({"paper":lc.paper.id,"build":v["build"]["ok"],"world":v["world"]["ok"]}));
            }
            open.insert(lc.whale.clone(), json!({"whale":lc.whale,"batch":number,"paper":lc.paper.id,"deadline":deadline,"start_at":at,"running":false,"last_end":null,"opened":b,"assignment":a}));
            opened.push(b);
        }
    }
    let whales: Vec<Value> = c.whales.iter().map(|s| open.get(&s.whale).cloned().unwrap_or_else(|| json!({"whale":s.whale,"free":true}))).collect();
    let state = if !opened.is_empty() {
        "opened"
    } else if !open.is_empty() {
        "open"
    } else if paused {
        "paused"
    } else if !assign {
        "idle"
    } else {
        "no_papers_left"
    };
    Ok(json!({"state":state,"opened":opened,"closed":closed_now,"refused":refusals,"paused_until":if paused { json!(pause_until) } else { Value::Null },"unassigned":unassigned,"whales":whales,"topic_corpus":topics}))
}

/// How the rolling runner runs a learner: the terminal of a learner configuration and the learner
/// (`learn::run`); tests replace both. `wait` is the backoff and pause in seconds.
pub struct Roller {
    pub terminal: std::sync::Arc<dyn Fn(&LearnConfig) -> Result<Box<dyn PrivateTerminal>> + Send + Sync>,
    pub learn: std::sync::Arc<dyn Fn(&LearnConfig, &mut dyn PrivateTerminal) -> Result<Value> + Send + Sync>,
    pub wait: u64,
    pub tick: std::time::Duration,
    /// Return as soon as no learner runs and none may start now (tests; a restart continues).
    pub until_idle: bool,
}

impl Roller {
    /// The live runner: each Whale its own terminal (`MovaTerminal`), `wait_minutes`.
    pub fn live(c: &BatchConfig) -> Self {
        Self {
            terminal: std::sync::Arc::new(|lc: &LearnConfig| MovaTerminal::open(&lc.terminal).map(|t| Box::new(t) as Box<dyn PrivateTerminal>)),
            learn: std::sync::Arc::new(|lc: &LearnConfig, t: &mut dyn PrivateTerminal| learn::run(lc, t)),
            wait: c.wait_minutes * 60,
            tick: std::time::Duration::from_secs(10),
            until_idle: false,
        }
    }
}

/// The persistent pod with rolling assignment: one runner per run root (it holds `plan.lock`).
/// Each Whale works on its own: when its paper is final it takes the next one at once; when its
/// learner stopped unfinished it continues the same paper (new questions only) after `wait`, until
/// that paper's deadline; a usage limit pauses every Whale for `wait`. `STOP` in the run root: no
/// learner starts, running learners stop before their next round (`stop`, resumed by the next
/// runner), and the runner returns. Ends when no paper is left and no Whale has one.
pub fn roll(c: &BatchConfig, r: &Roller) -> Result<Value> {
    c.validate()?;
    private_dir(&c.run_root)?;
    let _runner = try_lease(&c.run_root.join("plan.lock"), true)?.ok_or_else(|| error("another runner or plan holds this run root (plan.lock)"))?;
    let (tx, rx) = std::sync::mpsc::channel::<(String, u64, Value)>();
    let mut running: std::collections::BTreeMap<String, u64> = Default::default();
    let mut handles = vec![];
    let mut ended = vec![];
    // The last plan found no paper for a free Whale: no new catalog scan until something changes.
    let mut dry = false;
    loop {
        let stop = c.run_root.join("STOP").exists();
        let busy = running.clone();
        let p = plan_at(c, &busy, !stop && !dry, r.wait)?;
        if p["closed"].as_array().is_some_and(|v| !v.is_empty()) {
            dry = false;
        } else if p["unassigned"].as_u64().unwrap_or(0) > 0 {
            dry = true;
        }
        let at = now();
        let paused = p["paused_until"].as_u64().is_some_and(|u| u > at);
        let mut started = 0;
        if !stop && !paused {
            for w in p["whales"].as_array().cloned().unwrap_or_default() {
                if w["batch"].is_null() || w["running"] == true || w["start_at"].as_u64().unwrap_or(u64::MAX) > at {
                    continue;
                }
                let n = w["batch"].as_u64().unwrap_or(0);
                let lc = learn_config(c, &w["opened"], &w["assignment"])?;
                whale_log(c, &lc.whale, &json!({"event":"started","batch":n,"paper":lc.paper.id,"after":w["last_end"],"unix":at}))?;
                running.insert(lc.whale.clone(), n);
                started += 1;
                let (tx, term, learn) = (tx.clone(), r.terminal.clone(), r.learn.clone());
                handles.push(std::thread::spawn(move || {
                    let res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| term(&lc).and_then(|mut t| learn(&lc, t.as_mut()))));
                    let v = match res {
                        Ok(Ok(v)) => v,
                        Ok(Err(e)) => json!({"whale":lc.whale,"paper":lc.paper.id,"state":"error","reason":"error","error":e.to_string()}),
                        Err(_) => json!({"whale":lc.whale,"paper":lc.paper.id,"state":"error","reason":"panicked"}),
                    };
                    let _ = tx.send((lc.whale.clone(), n, v));
                }));
            }
        }
        if running.is_empty() {
            let holding = p["whales"].as_array().is_some_and(|v| v.iter().any(|w| !w["batch"].is_null()));
            if stop {
                return Ok(json!({"state":"stopped","ended":ended}));
            }
            if !holding && (dry || p["state"] == "no_papers_left") {
                return Ok(json!({"state":"no_papers_left","ended":ended}));
            }
            if r.until_idle && started == 0 {
                return Ok(json!({"state":"idle","plan":p,"ended":ended}));
            }
        }
        match rx.recv_timeout(r.tick) {
            Ok((whale, n, v)) => {
                running.remove(&whale);
                dry = false;
                let reason = v["reason"].as_str().unwrap_or("error").to_string();
                whale_log(c, &whale, &json!({"event":"ended","batch":n,"paper":v["paper"],"state":v["state"],"reason":reason,"rounds":v["rounds"],"words":v["last_round"]["words"],"error":v["error"],"unix":now()}))?;
                if let Some(b) = events(c)?.into_iter().find(|e| e["event"] == "opened" && e["batch"].as_u64() == Some(n)) {
                    let configs: Vec<LearnConfig> = b["assignments"].as_array().cloned().unwrap_or_default().iter().map(|a| learn_config(c, &b, a)).collect::<Result<_>>()?;
                    collect(c, &b, &configs)?;
                }
                ended.push(json!({"whale":whale,"batch":n,"reason":reason}));
                if ended.len() > 1_000_000 {
                    return Err(error("batch: too many learner runs"));
                }
            }
            Err(std::sync::mpsc::RecvTimeoutError::Timeout) => {}
            Err(e) => return Err(error(e.to_string())),
        }
        handles.retain(|h: &std::thread::JoinHandle<()>| !h.is_finished());
    }
}

/// The pod: live, the rolling runner (`roll`); without a terminal, one dry pass (`once`).
pub fn run(c: &BatchConfig, live: bool) -> Result<Value> {
    if live {
        roll(c, &Roller::live(c))
    } else {
        once(c)
    }
}

/// One dry pass without a terminal: plan, run every open assignment's learner in parallel
/// (`NoTerminal`: each stops at its first question), collect candidates, plan again.
pub fn once(c: &BatchConfig) -> Result<Value> {
    let p = plan(c)?;
    let ev = events(c)?;
    let mut work: Vec<(Value, LearnConfig)> = vec![];
    for w in p["whales"].as_array().cloned().unwrap_or_default().iter().filter(|w| !w["batch"].is_null()) {
        work.push((w["opened"].clone(), learn_config(c, &w["opened"], &w["assignment"])?));
    }
    let results: Vec<Value> = std::thread::scope(|s| {
        let hs: Vec<_> = work
            .iter()
            .map(|(_, lc)| {
                s.spawn(move || match learn::run(lc, &mut NoTerminal(lc.whale.clone())) {
                    Ok(v) => v,
                    Err(e) => json!({"whale":lc.whale,"paper":lc.paper.id,"state":"error","error":e.to_string()}),
                })
            })
            .collect();
        hs.into_iter().map(|h| h.join().unwrap_or_else(|_| json!({"state":"panicked"}))).collect()
    });
    let (mut school, mut dict, mut per) = (0, 0, vec![]);
    let batches: BTreeSet<u64> = work.iter().filter_map(|(b, _)| b["batch"].as_u64()).collect();
    for n in batches {
        let Some(b) = ev.iter().find(|e| e["event"] == "opened" && e["batch"].as_u64() == Some(n)) else { continue };
        let configs: Vec<LearnConfig> = b["assignments"].as_array().cloned().unwrap_or_default().iter().map(|a| learn_config(c, b, a)).collect::<Result<_>>()?;
        let v = collect(c, b, &configs)?;
        school += v["school"].as_u64().unwrap_or(0);
        dict += v["dictionary"].as_u64().unwrap_or(0);
        per.push(json!({"batch":n,"candidates":v}));
    }
    let after = plan(c)?;
    Ok(json!({"before":p,"learners":results,"candidates":{"school":school,"dictionary":dict,"batches":per},"plan":after}))
}

/// Review-only teaching candidates of a batch for the open papers school
/// (`courses/papers/` of the school data, `$MOVA_DATA/school`) and the dictionary of scientific writing: only from open-licensed
/// papers; Mova's own explanations and restatements, the paper's own context sentence, never a
/// terminal answer.
pub fn collect(c: &BatchConfig, b: &Value, configs: &[LearnConfig]) -> Result<Value> {
    let dir = c.run_root.join("candidates");
    private_dir(&dir)?;
    let n = b["batch"].as_u64().unwrap_or(0);
    let (mut school, mut dict) = (String::new(), String::new());
    let (mut ns, mut nd) = (0, 0);
    for lc in configs {
        if !open_licence(&lc.paper.licence) {
            continue;
        }
        let store = Store::open(&lc.state_root.join("knowledge"))?;
        let text = std::fs::read_to_string(&lc.paper.path).unwrap_or_default();
        let context = |term: &str| world::quant::sentences(&text).into_iter().find(|s| s.split(|ch: char| !(ch.is_alphanumeric() || ch == '-')).any(|w| w == term) && s.split_whitespace().count() <= 60);
        let source = json!({"paper":lc.paper.id,"title":lc.paper.title,"licence":lc.paper.licence,"open_school_admissible":true});
        for it in store.items.iter().filter(|i| i.paper == lc.paper.id) {
            let mova = match it.kind.as_str() {
                "term" => knowledge::explain(&it.key, &it.facts.iter().filter(|f| !knowledge::bare(f)).cloned().collect::<Vec<_>>()),
                _ => it.facts.iter().filter_map(knowledge::restate).collect(),
            };
            let prompt = match it.kind.as_str() {
                "term" => format!("What does «{}» mean in «{}»?", it.key, lc.paper.title),
                "section" => format!("What is the main point of the part «{}» of «{}»?", it.key, lc.paper.title),
                _ => format!("What does this sentence of «{}» claim? «{}»", lc.paper.title, it.key),
            };
            school.push_str(&json!({"target":"school/courses/papers/","kind":it.kind,"prompt":prompt,"mova_answer":mova,"knowledge_item":it.id,"whale":lc.whale,"source":source,"admission":"review_required"}).to_string());
            school.push('\n');
            ns += 1;
            if it.kind == "term" && !mova.is_empty() {
                dict.push_str(&json!({"target":"dictionary of scientific writing","term":it.key,"explanation":mova,"context":context(&it.key),"knowledge_item":it.id,"whale":lc.whale,"source":source,"admission":"review_required"}).to_string());
                dict.push('\n');
                nd += 1;
            }
        }
    }
    write_sync(&dir.join(format!("batch-{n:03}-school.jsonl")), school.as_bytes())?;
    write_sync(&dir.join(format!("batch-{n:03}-dictionary.jsonl")), dict.as_bytes())?;
    Ok(json!({"school":ns,"dictionary":nd,"dir":dir}))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::learn::tests::{model_ok, tmp, PAPER};
    use std::sync::{Arc, Mutex};

    /// The assignments a plan opened: `whale:paper`, in opening order.
    fn ids(p: &Value) -> Vec<String> {
        p["opened"].as_array().unwrap().iter().flat_map(|b| b["assignments"].as_array().unwrap().iter().map(|a| format!("{}:{}", crate::util::legacy(a, "whale").as_str().unwrap(), a["paper"]["id"].as_str().unwrap())).collect::<Vec<_>>()).collect()
    }
    fn papers(p: &Value) -> Vec<String> {
        ids(p).into_iter().map(|s| s.split(':').nth(1).unwrap().to_string()).collect()
    }
    /// A Whale's open assignment in a plan's `whales`.
    fn holding<'a>(p: &'a Value, whale: &str) -> &'a Value {
        p["whales"].as_array().unwrap().iter().find(|w| w["whale"] == whale).unwrap()
    }

    #[test]
    fn batches_choose_cited_open_papers_and_close_on_the_deadline() {
        if !model_ok() {
            return;
        }
        let d = tmp("batch");
        let cited = d.join("cited");
        std::fs::create_dir_all(&cited).unwrap();
        let mut rows = String::new();
        for (id, lic, cite) in [("b-2020-cited", "CC BY 4.0", true), ("a-2021-cited", "CC BY 4.0 (ACL Anthology)", true), ("c-2019-nc", "CC BY-NC 4.0", true), ("d-2018-free", "CC0 1.0", false), ("e-2017-sa", "CC BY-SA 4.0", false)] {
            let p = d.join(format!("{id}.md"));
            std::fs::write(&p, PAPER).unwrap();
            rows.push_str(&json!({"id":id,"text_path":p,"title":id,"licence":lic}).to_string());
            rows.push('\n');
            if cite {
                std::fs::write(cited.join(format!("{id}.md")), "page").unwrap();
            }
        }
        let cat = d.join("catalog.jsonl");
        std::fs::write(&cat, rows).unwrap();
        let whales: Vec<Value> = ["whale-1", "whale-2"].iter().map(|s| json!({"whale":s,"state_root":d.join(format!("{s}-state")),"terminal":crate::terminal::tests::profile(&d, s)})).collect();
        let c: BatchConfig = serde_json::from_value(json!({"schema":SCHEMA,"run_root":d.join("run"),"catalog":{"path":cat,"sha256":file_hash(&cat).unwrap()},
            "cited_dir":cited,"include_uncited":true,"selection":{"min_words":0,"require_core":false},"priority":["b-2020-cited"],"hours":0.0015,"whales":whales,"words":{"min":1500,"max":2000},"max_questions":2,"max_rounds":2})).unwrap();
        // The two cited open papers, by id, one batch each; the NC paper is never chosen.
        let p1 = plan(&c).unwrap();
        assert_eq!(p1["state"], "opened");
        assert_eq!(ids(&p1), ["whale-1:b-2020-cited", "whale-2:a-2021-cited"], "priority first, then cited by id");
        assert!(c.run_root.join("batch-001/whale-1.learn.json").exists() && c.run_root.join("batch-002/whale-2.learn.json").exists());
        // A dry run stops each learner at its first question; the papers stay open until their deadline.
        let r = run(&c, false).unwrap();
        assert!(r["learners"].as_array().unwrap().iter().all(|l| l["reason"] == "no_terminal"), "{r}");
        assert_eq!(r["candidates"]["dictionary"], 0, "{r}");
        assert_eq!(r["plan"]["state"], "open", "{r}");
        std::thread::sleep(std::time::Duration::from_millis(5600));
        let p2 = plan(&c).unwrap();
        assert!(p2["closed"].as_array().unwrap().iter().all(|e| e["reason"] == "deadline") && p2["closed"].as_array().unwrap().len() == 2, "{p2}");
        assert_eq!(ids(&p2), ["whale-1:d-2018-free"], "uncited open papers follow; SA and NC never");
        // Shared terminals are refused.
        let mut bad = c.clone();
        bad.whales[1].terminal = bad.whales[0].terminal.clone();
        assert!(bad.validate().is_err());
        let _ = std::fs::remove_dir_all(d);
    }

    /// Write a Whale's status for its open assignment as `learn::run` leaves it.
    fn stop(c: &BatchConfig, p: &Value, whale: &str, state: &str, reason: &str, writer: &str) {
        let w = holding(p, whale);
        let lc = learn_config(c, &w["opened"], &w["assignment"]).unwrap();
        std::fs::create_dir_all(lc.paper_dir()).unwrap();
        save(&lc.paper_dir().join("status.json"), &json!({"whale":lc.whale,"paper":lc.paper.id,"paper_sha256":lc.paper.sha256,"state":state,"reason":reason,"deadline":w["deadline"],"ended":now(),"rounds":3,"last_round":{"round":3,"words":3100,"pass":state == "passed","writer":writer}})).unwrap();
    }

    fn catalog(d: &Path, ids: &[&str]) -> (PathBuf, String) {
        let mut rows = String::new();
        for id in ids {
            let p = d.join(format!("{id}.md"));
            std::fs::write(&p, PAPER).unwrap();
            rows.push_str(&json!({"id":id,"text_path":p,"title":id,"licence":"CC BY 4.0"}).to_string());
            rows.push('\n');
        }
        let cat = d.join("catalog.jsonl");
        std::fs::write(&cat, rows).unwrap();
        let h = file_hash(&cat).unwrap();
        (cat, h)
    }
    fn pod(d: &Path, whales: &[&str], papers: &[&str], hours: f64) -> BatchConfig {
        let (cat, h) = catalog(d, papers);
        let whales: Vec<Value> = whales.iter().map(|s| json!({"whale":s,"state_root":d.join(format!("{s}-state")),"terminal":crate::terminal::tests::profile(d, s)})).collect();
        serde_json::from_value(json!({"schema":SCHEMA,"run_root":d.join("run"),"catalog":{"path":cat,"sha256":h},
            "include_uncited":true,"selection":{"min_words":0,"require_core":false},"hours":hours,"whales":whales,"words":{"min":4000,"max":5000}})).unwrap()
    }

    /// Plan level: an asked-out paper closes its own batch at once and is retried by its Whale
    /// under a changed writer; a usage limit pauses assignment for the whole pod (red control:
    /// with the pause over the free Whale is assigned); no retry under the same writer.
    #[test]
    fn an_asked_out_paper_closes_at_once_and_a_limit_pauses_the_pod() {
        let d = tmp("batch-exhausted");
        let c = pod(&d, &["learner-1", "learner-2"], &["a-1", "b-2", "c-3", "d-4", "e-5", "f-6"], 12.0);
        let b1 = plan(&c).unwrap();
        assert_eq!(ids(&b1), ["learner-1:a-1", "learner-2:b-2"]);
        // learner-1 asked out under an older writer, learner-2 stopped by a usage limit.
        stop(&c, &b1, "learner-1", "stopped", "nothing_to_ask", "writer-old");
        stop(&c, &b1, "learner-2", "stopped", "limit", "writer-old");
        let p = plan(&c).unwrap();
        // learner-1's batch closed at once; learner-2 keeps its paper (it can go on).
        assert_eq!(p["closed"].as_array().unwrap().len(), 1, "{p}");
        assert_eq!((p["closed"][0]["reason"].as_str(), p["closed"][0]["retry"][0]["paper"]["id"].as_str()), (Some("exhausted"), Some("a-1")), "{p}");
        assert_eq!(holding(&p, "learner-2")["paper"], "b-2");
        // The limit pauses the pod: the free learner-1 is not assigned yet.
        assert!(p["paused_until"].as_u64().is_some_and(|u| u > now()), "{p}");
        assert!(p["opened"].as_array().unwrap().is_empty() && holding(&p, "learner-1")["free"] == true, "{p}");
        assert!(holding(&p, "learner-2")["start_at"].as_u64().unwrap() > now(), "a limited learner waits");
        // Pause over (wait 0): learner-1 retries its own paper first (another writer).
        let p = plan_at(&c, &Default::default(), true, 0).unwrap();
        assert_eq!(ids(&p), ["learner-1:a-1"], "{p}");
        assert_eq!(p["opened"][0]["assignments"][0]["retry"], true);
        assert!(holding(&p, "learner-2")["start_at"].as_u64().unwrap() <= now());
        // One passes, one is asked out under the current writer: both close; no second retry
        // under the same writer, so both take new papers.
        stop(&c, &p, "learner-1", "passed", "passed", crate::compose::WRITER);
        stop(&c, &p, "learner-2", "stopped", "nothing_to_ask", crate::compose::WRITER);
        let p = plan(&c).unwrap();
        let reasons: Vec<&str> = p["closed"].as_array().unwrap().iter().map(|e| e["reason"].as_str().unwrap()).collect();
        assert_eq!(reasons, ["exhausted", "all_passed"], "{p}");
        assert_eq!(ids(&p), ["learner-1:c-3", "learner-2:d-4"], "{p}");
        // Each Whale's journal tells its own story.
        let j = whale_events(&c, "learner-1");
        let kinds: Vec<&str> = j.iter().map(|e| e["event"].as_str().unwrap()).collect();
        assert_eq!(kinds, ["assigned", "closed", "assigned", "closed", "assigned"], "{j:?}");
        let _ = std::fs::remove_dir_all(d);
    }

    #[test]
    fn renamed_learners_keep_their_open_batch_retries_and_state() {
        let d = crate::learn::tests::tmp("batch-renamed");
        let (cat, h) = catalog(&d, &["a-1", "b-2", "c-3", "d-4"]);
        let config = |names: [&str; 2], formerly: Option<[&str; 2]>, legacy: bool| -> BatchConfig {
            let slots: Vec<Value> = names
                .iter()
                .enumerate()
                .map(|(i, s)| {
                    let mut v = json!({"whale":s,"state_root":d.join(format!("state-{i}")),"terminal":crate::terminal::tests::profile(&d, s)});
                    if let Some(f) = formerly {
                        v["formerly"] = json!([f[i]]);
                    }
                    if legacy {
                        v["shark"] = v["whale"].take();
                        v.as_object_mut().unwrap().remove("whale");
                    }
                    v
                })
                .collect();
            let key = if legacy { "sharks" } else { "whales" };
            let mut v = json!({"schema":SCHEMA,"run_root":d.join("run"),"catalog":{"path":cat,"sha256":h},
                "include_uncited":true,"selection":{"min_words":0,"require_core":false},"hours":12,"words":{"min":4000,"max":5000}});
            v[key] = json!(slots);
            serde_json::from_value(v).unwrap()
        };
        // Batches opened under the old labels, by a configuration with the old field names.
        let old = config(["shark-1", "shark-2"], None, true);
        let b1 = plan(&old).unwrap();
        assert_eq!(b1["state"], "opened");
        // Its events as written before the rename: the learner under `shark`.
        let ev = c_events_as_legacy(&old);
        assert!(ev.contains("\"shark\":\"shark-1\"") && !ev.contains("\"whale\""));
        // Negative control: new labels without `formerly` cannot take the open batches over.
        let renamed = config(["owl-1", "owl-2"], None, false);
        assert!(plan(&renamed).unwrap_err().to_string().contains("no Whale slot for"));
        // With `formerly`, the open assignments continue under the new labels and the old state roots.
        let renamed = config(["owl-1", "owl-2"], Some(["shark-1", "shark-2"]), false);
        let open = plan(&renamed).unwrap();
        assert_eq!(open["state"], "open", "{open}");
        let w = holding(&open, "owl-1");
        let lc = learn_config(&renamed, &w["opened"], &w["assignment"]).unwrap();
        assert_eq!((lc.whale.as_str(), lc.state_root.clone()), ("owl-1", d.join("state-0")));
        // Unpassed papers are retried with the renamed learner that holds their knowledge.
        stop(&renamed, &open, "owl-1", "stopped", "nothing_to_ask", "writer-old");
        stop(&renamed, &open, "owl-2", "stopped", "nothing_to_ask", "writer-old");
        let b2 = plan(&renamed).unwrap();
        assert!(b2["closed"].as_array().unwrap().iter().all(|e| e["reason"] == "exhausted"), "{b2}");
        let ids: Vec<String> = b2["opened"].as_array().unwrap().iter().map(|b| &b["assignments"][0]).map(|a| format!("{}:{}:{}", a["whale"].as_str().unwrap(), a["paper"]["id"].as_str().unwrap(), a["retry"])).collect();
        assert_eq!(ids, ["owl-1:a-1:true", "owl-2:b-2:true"], "{b2}");
        // A label may not be both a current and a former one.
        let clash = config(["owl-1", "owl-2"], Some(["owl-2", "shark-2"]), false);
        assert!(clash.validate().unwrap_err().to_string().contains("shared between Whales"));
        let _ = std::fs::remove_dir_all(d);
    }

    /// A paper whose extraction lost its section headings is refused before it is assigned and
    /// recorded once for re-extraction; the learner takes the next paper. The learners'
    /// configurations carry the pod, the stall limit, batching, review settings and `STOP`.
    #[test]
    fn broken_extractions_are_refused_and_recorded() {
        let d = crate::learn::tests::tmp("batch-extraction");
        let flat = format!("# Flat\n\n{}\n\n## References\n\nA. B. 2020. C.\n", "The parser reads each word of the sentence once and attaches it to its head word. ".repeat(40));
        let mut rows = String::new();
        for (id, text) in [("a-1-flat", flat.as_str()), ("b-2-good", PAPER), ("c-3-good", PAPER)] {
            let p = d.join(format!("{id}.md"));
            std::fs::write(&p, text).unwrap();
            rows.push_str(&json!({"id":id,"text_path":p,"title":id,"licence":"CC BY 4.0"}).to_string());
            rows.push('\n');
        }
        let cat = d.join("catalog.jsonl");
        std::fs::write(&cat, rows).unwrap();
        let whales: Vec<Value> = ["learner-1"].iter().map(|s| json!({"whale":s,"state_root":d.join(format!("{s}-state")),"terminal":crate::terminal::tests::profile(&d, s)})).collect();
        let c: BatchConfig = serde_json::from_value(json!({"schema":SCHEMA,"run_root":d.join("run"),"catalog":{"path":cat,"sha256":file_hash(&cat).unwrap()},
            "include_uncited":true,"selection":{"min_words":0,"require_core":false},"hours":12,"whales":whales,"words":{"min":4000,"max":5000},"pod":d.join("pod"),"stall_rounds":4})).unwrap();
        let b1 = plan(&c).unwrap();
        assert_eq!(papers(&b1), ["b-2-good"], "{b1}");
        assert_eq!(b1["refused"][0]["paper"]["id"], "a-1-flat", "{b1}");
        assert_eq!(b1["refused"][0]["broken"], json!(["no_section_headings"]));
        let lc = learn_config(&c, &b1["opened"][0], &b1["opened"][0]["assignments"][0]).unwrap();
        assert_eq!((lc.pod.clone(), lc.stall_rounds, lc.batched, lc.stop_file.clone()), (Some(d.join("pod")), 4, true, Some(c.run_root.join("STOP"))));
        let written: Value = serde_json::from_slice(&std::fs::read(c.run_root.join("batch-001/learner-1.learn.json")).unwrap()).unwrap();
        assert_eq!((&written["pod"], &written["stop_file"]), (&json!(d.join("pod")), &json!(c.run_root.join("STOP"))));
        // Red control: with the check off (limits above any text), the flat paper is assigned.
        let mut off = c.clone();
        off.run_root = d.join("run-off");
        off.extraction = crate::reader::ExtractionLimits { max_spacing_residue_per_1000: 1e9, max_unknown_share: 1.0, max_truncated_share: 1.0, max_front_share: 1.0 };
        assert_eq!(papers(&plan(&off).unwrap()), ["a-1-flat"]);
        // Recorded once: the next plan does not refuse it again.
        let ev = events(&c).unwrap();
        assert_eq!(ev.iter().filter(|e| e["event"] == "refused").count(), 1);
        let _ = std::fs::remove_dir_all(d);
    }

    /// With a topic corpus configured, an assigned paper's corpus and world are built at once,
    /// and the learner seeds between rounds (its lock released for the seed: the fake seed refuses
    /// a held lock, as the real one does) and writes a cited description per round.
    #[test]
    fn topic_corpus_is_built_seeded_and_cited() {
        if !crate::learn::tests::model_ok() {
            return;
        }
        let d = crate::learn::tests::tmp("batch-topic");
        let p = d.join("a-1.md");
        std::fs::write(&p, PAPER).unwrap();
        let cat = d.join("catalog.jsonl");
        std::fs::write(&cat, format!("{}\n", json!({"id":"a-1","text_path":p,"title":"a","licence":"CC BY 4.0"}))).unwrap();
        let tc = crate::topic::tests::fake(&d);
        let whales = vec![json!({"whale":"learner-1","state_root":d.join("state-1"),"terminal":crate::terminal::tests::profile(&d, "learner-1")})];
        let c: BatchConfig = serde_json::from_value(json!({"schema":SCHEMA,"run_root":d.join("run"),"catalog":{"path":cat,"sha256":file_hash(&cat).unwrap()},
            "include_uncited":true,"selection":{"min_words":0,"require_core":false},"hours":12,"whales":whales,"words":{"min":4000,"max":5000},"max_rounds":2,"review":{"enabled":false},"topic_corpus":tc})).unwrap();
        let b = plan(&c).unwrap();
        assert_eq!(b["topic_corpus"][0], json!({"paper":"a-1","build":true,"world":true}), "{b}");
        let lc = learn_config(&c, &b["opened"][0], &b["opened"][0]["assignments"][0]).unwrap();
        let st = learn::run(&lc, &mut NoTerminal(lc.whale.clone())).unwrap();
        let journal = std::fs::read_to_string(lc.paper_dir().join("journal.jsonl")).unwrap();
        assert!(journal.contains("\"event\":\"seeded\"") && journal.contains("\"seeded\":[]"), "the seed ran with the lock held: {journal}");
        assert!(lc.paper_dir().join("round-001/cited.md").exists(), "{st}");
        let calls = std::fs::read_to_string(d.join("topic-calls.log")).unwrap();
        assert!(calls.contains("build ") && calls.contains("world ") && calls.contains("seed ") && calls.contains("cite "), "{calls}");
        let _ = std::fs::remove_dir_all(d);
    }

    /// Selection: never alphabetical. Cited papers first, then core-topic papers by citations in
    /// the library and overall; short papers are refused once; uncited off-topic papers and
    /// little-cited shared-task system descriptions are not taken.
    #[test]
    fn selection_ranks_by_relevance_and_length() {
        let d = crate::learn::tests::tmp("batch-selection");
        let cited = d.join("cited");
        std::fs::create_dir_all(&cited).unwrap();
        let long = format!("# Long\n\n## Method\n\n{}\n", "The parser reads each word of the sentence once and attaches it to its head word. ".repeat(60));
        let mut rows = String::new();
        // (id, text, concepts, our_citations, cited_by, title, cited by our code)
        for (id, text, concepts, lib, all, title, cite) in [
            ("a-2020-alpha", long.as_str(), json!(["parsing"]), 1, 2, "Alpha", false),
            ("b-2019-short", PAPER, json!(["parsing"]), 90, 900, "Short", false),
            ("c-2018-system", long.as_str(), json!(["parsing"]), 50, 10, "Team X at the BEA Shared Task", false),
            ("d-2017-offtopic", long.as_str(), json!(["robotics"]), 99, 999, "Robots", false),
            ("e-2016-core", long.as_str(), json!(["universal-dependencies"]), 40, 100, "UD", false),
            ("f-2014-nlp", long.as_str(), json!(["natural-language-processing"]), 95, 950, "General", false),
            ("z-2015-cited", long.as_str(), json!([]), 0, 0, "Cited", true),
        ] {
            let p = d.join(format!("{id}.md"));
            std::fs::write(&p, text).unwrap();
            rows.push_str(&json!({"id":id,"text_path":p,"title":title,"licence":"CC BY 4.0","concepts":concepts,"our_citations":lib,"cited_by":all}).to_string());
            rows.push('\n');
            if cite {
                std::fs::write(cited.join(format!("{id}.md")), "page").unwrap();
            }
        }
        let cat = d.join("catalog.jsonl");
        std::fs::write(&cat, rows).unwrap();
        let whales: Vec<Value> = ["learner-1", "learner-2", "learner-3", "learner-4"].iter().map(|s| json!({"whale":s,"state_root":d.join(format!("{s}-state")),"terminal":crate::terminal::tests::profile(&d, s)})).collect();
        let c: BatchConfig = serde_json::from_value(json!({"schema":SCHEMA,"run_root":d.join("run"),"catalog":{"path":cat,"sha256":file_hash(&cat).unwrap()},"cited_dir":cited,
            "include_uncited":true,"selection":{"min_words":500},"hours":12,"whales":whales,"words":{"min":4000,"max":5000}})).unwrap();
        if !crate::learn::tests::model_ok() {
            return;
        }
        let b = plan(&c).unwrap();
        // Cited first; then core papers by library citations (the short one refused); the
        // off-topic and the little-cited system description never; alphabetical order nowhere.
        // General NLP comes after the core topics, however cited.
        assert_eq!(papers(&b), ["z-2015-cited", "e-2016-core", "a-2020-alpha", "f-2014-nlp"], "{b}");
        let refused: Vec<&str> = b["refused"].as_array().unwrap().iter().map(|r| r["paper"]["id"].as_str().unwrap()).collect();
        assert_eq!(refused, ["b-2019-short"], "{b}");
        // Red control: alphabetical order would have taken a-2020-alpha first.
        assert_ne!(papers(&b)[0], "a-2020-alpha");
        let _ = std::fs::remove_dir_all(d);
    }

    /// Rewrite the run root's events with the learner field name used before the Whale rename.
    fn c_events_as_legacy(c: &BatchConfig) -> String {
        let p = c.run_root.join("batches.jsonl");
        let s = std::fs::read_to_string(&p).unwrap().replace("\"whale\":", "\"shark\":");
        std::fs::write(&p, &s).unwrap();
        s
    }

    // ------------------------------------------------------------ the rolling runner

    /// A scripted learner (no model, no terminal call): each run of a Whale takes its next stop
    /// reason (`passed` when the script is empty), writes the status `learn::run` would leave and
    /// records (whale, paper). `limit` returns at once, the others after `delay_ms`, so the order
    /// in which the runner sees them is fixed.
    #[derive(Clone, Default)]
    struct Script {
        reasons: Arc<Mutex<std::collections::BTreeMap<String, std::collections::VecDeque<&'static str>>>>,
        runs: Arc<Mutex<Vec<String>>>,
        terminals: Arc<Mutex<Vec<String>>>,
        delay_ms: u64,
    }
    impl Script {
        fn new(plan: &[(&str, &[&'static str])], delay_ms: u64) -> Self {
            let s = Script { delay_ms, ..Default::default() };
            for (w, r) in plan {
                s.reasons.lock().unwrap().insert(w.to_string(), r.iter().copied().collect());
            }
            s
        }
        fn roller(&self, wait: u64) -> Roller {
            let (s1, s2) = (self.clone(), self.clone());
            Roller {
                terminal: Arc::new(move |lc: &LearnConfig| {
                    s1.terminals.lock().unwrap().push(lc.whale.clone());
                    Ok(Box::new(NoTerminal(lc.whale.clone())) as Box<dyn PrivateTerminal>)
                }),
                learn: Arc::new(move |lc: &LearnConfig, t: &mut dyn PrivateTerminal| {
                    assert_eq!(t.animal(), lc.whale, "a Whale got another's terminal");
                    let reason = s2.reasons.lock().unwrap().get_mut(&lc.whale).and_then(|q| q.pop_front()).unwrap_or("passed");
                    if reason != "limit" {
                        std::thread::sleep(std::time::Duration::from_millis(s2.delay_ms));
                    }
                    s2.runs.lock().unwrap().push(format!("{}:{}", lc.whale, lc.paper.id));
                    std::fs::create_dir_all(lc.paper_dir()).unwrap();
                    let deadline = lc.deadline_unix.unwrap();
                    let state = if reason == "passed" { "passed" } else { "stopped" };
                    let st = json!({"whale":lc.whale,"paper":lc.paper.id,"paper_sha256":lc.paper.sha256,"state":state,"reason":reason,"deadline":deadline,"ended":now(),"rounds":1,"last_round":{"round":1,"words":4100,"writer":crate::compose::WRITER}});
                    save(&lc.paper_dir().join("status.json"), &st)?;
                    Ok(st)
                }),
                wait,
                tick: std::time::Duration::from_millis(20),
                until_idle: true,
            }
        }
        fn runs(&self) -> Vec<String> {
            self.runs.lock().unwrap().clone()
        }
    }
    fn assigned(c: &BatchConfig, whale: &str) -> Vec<String> {
        whale_events(c, whale).iter().filter(|e| e["event"] == "assigned").map(|e| e["paper"].as_str().unwrap().to_string()).collect()
    }

    /// A Whale whose paper passed takes the next paper at once, while another Whale's terminal
    /// failure keeps that Whale on its own paper (waiting for its backoff), never on a paper
    /// another holds; no paper is assigned twice.
    #[test]
    fn a_passed_whale_takes_the_next_paper_at_once_and_a_failed_one_keeps_its_own() {
        let d = tmp("roll-pass");
        let c = pod(&d, &["owl-1", "owl-2"], &["p-1", "p-2", "p-3", "p-4", "p-5"], 12.0);
        let s = Script::new(&[("owl-2", &["failed"])], 30);
        let t = std::time::Instant::now();
        let r = roll(&c, &s.roller(3600)).unwrap();
        assert!(t.elapsed().as_secs() < 60, "a passed Whale waited for the others");
        assert_eq!(r["state"], "idle", "{r}");
        // owl-1 went on through the rest of the selection; owl-2 stayed on its paper, ran once.
        assert_eq!(assigned(&c, "owl-1"), ["p-1", "p-3", "p-4", "p-5"], "{r}");
        assert_eq!(assigned(&c, "owl-2"), ["p-2"], "a failed Whale took another paper");
        assert_eq!(s.runs().iter().filter(|x| x.starts_with("owl-2")).count(), 1, "{:?}", s.runs());
        let p = &r["plan"];
        assert_eq!((holding(p, "owl-2")["paper"].as_str(), holding(p, "owl-1")["free"].as_bool()), (Some("p-2"), Some(true)), "{p}");
        assert!(holding(p, "owl-2")["start_at"].as_u64().unwrap() > now() + 3000, "the backoff holds");
        // Every paper once; each passed paper's batch closed, owl-2's open.
        let ev = events(&c).unwrap();
        let opened: Vec<&str> = ev.iter().filter(|e| e["event"] == "opened").map(|e| e["assignments"][0]["paper"]["id"].as_str().unwrap()).collect();
        assert_eq!(opened.iter().collect::<BTreeSet<_>>().len(), opened.len());
        assert_eq!(open_batches(&ev).len(), 1);
        // Each Whale used only its own terminal (asserted in the learner) — and opened one per run.
        assert_eq!(s.terminals.lock().unwrap().len(), s.runs().len());
        // Red control: had owl-2's failure counted as final, owl-2 would have moved on.
        assert_eq!(final_reason(&Some(json!({"state":"stopped","reason":"failed","deadline":u64::MAX})), u64::MAX, now()), None);
        let _ = std::fs::remove_dir_all(d);
    }

    /// A failed Whale keeps its paper only until that paper's own deadline: then the paper is given
    /// up (`deadline`, kept in `retry` with its reason) and the Whale takes the next one.
    #[test]
    fn a_failed_paper_is_given_up_at_its_deadline_and_the_whale_moves_on() {
        let d = tmp("roll-deadline");
        let c = pod(&d, &["owl-1", "owl-2"], &["p-1", "p-2", "p-3", "p-4", "p-5"], 0.0008);
        let s = Script::new(&[("owl-1", &["passed", "failed"]), ("owl-2", &["failed"])], 30);
        let r = roll(&c, &s.roller(3600)).unwrap();
        assert_eq!(r["state"], "idle", "{r}");
        assert_eq!((assigned(&c, "owl-1"), assigned(&c, "owl-2")), (vec!["p-1".to_string(), "p-3".into()], vec!["p-2".to_string()]));
        std::thread::sleep(std::time::Duration::from_millis(3200));
        let r = roll(&c, &s.roller(3600)).unwrap();
        assert_eq!(r["state"], "no_papers_left", "{r}");
        assert_eq!((assigned(&c, "owl-1"), assigned(&c, "owl-2")), (vec!["p-1".to_string(), "p-3".into(), "p-4".into()], vec!["p-2".to_string(), "p-5".into()]));
        let closed: Vec<Value> = events(&c).unwrap().into_iter().filter(|e| e["event"] == "closed" && e["reason"] == "deadline").collect();
        assert_eq!(closed.len(), 2, "{closed:?}");
        assert!(closed.iter().all(|e| e["retry"][0]["reason"] == "failed"), "{closed:?}");
        // Under the same writer a given-up paper is not retried.
        assert!(retries(&events(&c).unwrap()).is_empty());
        let _ = std::fs::remove_dir_all(d);
    }

    /// A usage limit pauses every Whale: a Whale that passed meanwhile gets no new paper and no
    /// learner starts until the pause is over; then both go on (red control: wait 0).
    #[test]
    fn a_usage_limit_pauses_every_whale() {
        let d = tmp("roll-limit");
        let c = pod(&d, &["owl-1", "owl-2"], &["p-1", "p-2", "p-3", "p-4"], 12.0);
        let s = Script::new(&[("owl-1", &["limit", "passed"])], 300);
        let r = roll(&c, &s.roller(3600)).unwrap();
        assert_eq!(r["state"], "idle", "{r}");
        assert_eq!(s.runs(), ["owl-1:p-1", "owl-2:p-2"]);
        assert!(r["plan"]["paused_until"].as_u64().is_some_and(|u| u > now() + 3000), "{r}");
        assert_eq!(assigned(&c, "owl-2"), ["p-2"], "a Whale got a new paper during the pause");
        assert_eq!(holding(&r["plan"], "owl-2")["free"], true);
        // The pause over: owl-1 continues its own paper, owl-2 takes the next.
        let r = roll(&c, &s.roller(0)).unwrap();
        assert_eq!(r["state"], "no_papers_left", "{r}");
        let later: BTreeSet<String> = s.runs()[2..4].iter().cloned().collect();
        assert_eq!(later, ["owl-1:p-1".to_string(), "owl-2:p-3".into()].into_iter().collect(), "{:?}", s.runs());
        let mut all = [assigned(&c, "owl-1"), assigned(&c, "owl-2")].concat();
        all.sort();
        assert_eq!(all, ["p-1", "p-2", "p-3", "p-4"], "every paper once");
        assert_eq!(assigned(&c, "owl-1")[0], "p-1");
        let _ = std::fs::remove_dir_all(d);
    }

    /// One runner per run root; `STOP` starts nothing and returns; a restarted runner continues
    /// the open assignment (no new assignment, no paper twice).
    #[test]
    fn one_runner_per_root_stop_and_restart_resume() {
        let d = tmp("roll-restart");
        let c = pod(&d, &["owl-1"], &["p-1", "p-2"], 12.0);
        let s = Script::new(&[("owl-1", &["timeout"])], 0);
        let r = roll(&c, &s.roller(3600)).unwrap();
        assert_eq!((r["state"].as_str(), s.runs().len()), (Some("idle"), 1), "{r}");
        // Another runner (or plan) on the same root is refused.
        let held = try_lease(&c.run_root.join("plan.lock"), true).unwrap().unwrap();
        assert!(roll(&c, &s.roller(0)).unwrap_err().to_string().contains("plan.lock"));
        assert!(plan(&c).is_err());
        drop(held);
        // STOP: nothing starts, the runner returns.
        std::fs::write(c.run_root.join("STOP"), "").unwrap();
        assert_eq!(roll(&c, &s.roller(0)).unwrap()["state"], "stopped");
        assert_eq!(s.runs().len(), 1);
        std::fs::remove_file(c.run_root.join("STOP")).unwrap();
        // Restarted (backoff over): the same paper continues, then the next.
        let r = roll(&c, &s.roller(0)).unwrap();
        assert_eq!(r["state"], "no_papers_left", "{r}");
        assert_eq!(s.runs(), ["owl-1:p-1", "owl-1:p-1", "owl-1:p-2"]);
        assert_eq!(assigned(&c, "owl-1"), ["p-1", "p-2"]);
        let started = whale_events(&c, "owl-1").iter().filter(|e| e["event"] == "started" && e["paper"] == "p-1").count();
        assert_eq!(started, 2);
        let _ = std::fs::remove_dir_all(d);
    }

    /// With the real learner and a fake terminal: a usage limit ends the first runner; the next
    /// runner continues the same paper and never asks a question again.
    #[test]
    fn a_restarted_runner_resumes_without_re_asking() {
        if !model_ok() {
            return;
        }
        let d = tmp("roll-learn");
        let mut c = pod(&d, &["owl-1"], &["p-1"], 12.0);
        c.words = Words { min: 1500, max: 2000 };
        c.max_questions = 2;
        c.max_rounds = 4;
        c.review.enabled = false;
        let asked: Arc<Mutex<Vec<String>>> = Default::default();
        let limited = Arc::new(Mutex::new(true));
        let roller = |wait: u64| {
            let (asked, limited) = (asked.clone(), limited.clone());
            Roller {
                terminal: Arc::new(move |lc: &LearnConfig| {
                    let t = crate::learn::tests::Scripted::new(&lc.whale, Default::default(), "The parser reads each word once.", if *limited.lock().unwrap() { Some(0) } else { None });
                    Ok(Box::new(Rec(t, asked.clone())) as Box<dyn PrivateTerminal>)
                }),
                learn: Arc::new(|lc: &LearnConfig, t: &mut dyn PrivateTerminal| learn::run(lc, t)),
                wait,
                tick: std::time::Duration::from_millis(20),
                until_idle: true,
            }
        };
        let r = roll(&c, &roller(3600)).unwrap();
        assert_eq!(r["ended"][0]["reason"], "limit", "{r}");
        let first = asked.lock().unwrap().clone();
        assert!(!first.is_empty());
        *limited.lock().unwrap() = false;
        let r = roll(&c, &roller(0)).unwrap();
        assert_eq!(r["state"], "no_papers_left", "{r}");
        let all = asked.lock().unwrap().clone();
        // The second runner continued round 2 of the same paper: everything was asked, nothing again.
        assert_eq!((r["ended"][0]["batch"].as_u64(), r["ended"][0]["reason"].as_str()), (Some(1), Some("nothing_to_ask")), "{r}");
        assert_eq!(all, first, "a question was asked again: {all:?}");
        assert_eq!(all.iter().collect::<BTreeSet<_>>().len(), all.len());
        assert_eq!(assigned(&c, "owl-1"), ["p-1"]);
        let w = json!(events(&c).unwrap()[0].clone());
        let lc = learn_config(&c, &w, &w["assignments"][0]).unwrap();
        assert_eq!(learn::paper_status(&lc).unwrap().unwrap()["rounds"], 2);
        // Red control: a Whale without that state asks the same questions again.
        let mut fresh = lc.clone();
        fresh.state_root = d.join("fresh-state");
        let again: Arc<Mutex<Vec<String>>> = Default::default();
        learn::run(&fresh, &mut Rec(crate::learn::tests::Scripted::new("owl-1", Default::default(), "", None), again.clone())).unwrap();
        assert!(again.lock().unwrap().iter().any(|q| first.contains(q)), "{again:?}");
        let _ = std::fs::remove_dir_all(d);
    }
    /// A terminal that records every question subject it receives.
    struct Rec(crate::learn::tests::Scripted, Arc<Mutex<Vec<String>>>);
    impl PrivateTerminal for Rec {
        fn animal(&self) -> &str {
            self.0.animal()
        }
        fn ask(&mut self, q: &crate::terminal::Question) -> std::result::Result<crate::terminal::Answer, crate::terminal::AskError> {
            let subjects: Vec<String> = if q.items.is_empty() { vec![format!("{}:{}", q.kind, q.subject)] } else { q.items.iter().map(|i| format!("{}:{}", i.kind, i.subject)).collect() };
            self.1.lock().unwrap().extend(subjects);
            self.0.ask(q)
        }
    }
}
