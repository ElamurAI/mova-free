//! The standing work loop of an animal that improves teaching material (🐚 Nautilus: dictionary entries; 🐢 Sea
//! Turtle: lessons and their questions), bounded per step:
//!
//! pick the next item → Mova's own reading of it (`Job::native`) → when unsure, ask the animal's own teacher terminal
//! (`teacher::submit`) → validate the answer against the job's gates (`Job::validate`) → journal → next.
//!
//! **Gates.** A proposal is only ever written to the loop's own `proposals/` folder as a review-only record; the
//! loop never writes a source, a course or an assessment file, and refuses a state folder inside a configured frozen
//! path. Held-out items are never picked (`Job::next`; both jobs refuse test and control sets by name). Nothing is
//! admitted by the loop: admission is a separate, reviewed decision.
//!
//! **Pacing.** One question at a time, and a new one only `min_gap_secs` after the last. A usage or rate limit stops
//! the loop cleanly (journaled `stopped`, reason `limit`) until a person resumes it; a refused or failed question is
//! journaled `held` and never retried blindly.
//!
//! **Journal** (`work.jsonl` in the state folder): `native` (done natively), `held`, `proposed`, `rejected`,
//! `stopped`, `resumed`. `summary` counts them, with the throughput.
use crate::teacher::{self, PrivateTerminal};
use anyhow::{Result, ensure};
use serde_json::{Value, json};
use std::collections::{BTreeMap, BTreeSet};
use std::io::Write;
use std::path::{Path, PathBuf};

/// One item of work: a stable key and its content for the job.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Item {
    pub key: String,
    pub body: String,
}

/// What Mova's own reading says about an item.
pub enum Native {
    /// done natively; the note says what was found
    Done(String),
    /// unsure: the teacher question that would settle it
    Unsure(String),
}

pub trait Job {
    /// the purpose of its teacher terminal (`dictionary` or `questions`)
    fn purpose(&self) -> &'static str;
    /// the next item not in `seen`, never a held-out one
    fn next(&self, seen: &BTreeSet<String>) -> Result<Option<Item>>;
    fn native(&self, item: &Item) -> Native;
    /// the teacher's answer for an item: the proposal text, or why it is rejected
    fn validate(&self, item: &Item, answer: &str) -> std::result::Result<String, String>;
    /// what the job is, for the journal's `proposed` lines (the layer, the proposal format, the part)
    fn describe(&self) -> Value {
        Value::Null
    }
    /// how many items are left, not counting `seen` (each a teacher question at most)
    fn open(&self, seen: &BTreeSet<String>) -> Result<usize>;
    /// What Mova settled natively in an item, as review-only proposal text with its evidence (none by default). It is
    /// written when the item closes, next to the teacher's proposal.
    fn native_proposal(&self, _item: &Item) -> Option<String> {
        None
    }
    /// After an item closes with proposal texts (native, the teacher's): keep what the job learns from them for later
    /// items (Nautilus: the shared sense store). Nothing by default.
    fn learn(&self, _item: &Item, _proposals: &[&str], _now: u64) -> Result<()> {
        Ok(())
    }
}

pub struct Loop<'a> {
    pub state: PathBuf,
    pub teachers: &'a teacher::Config,
    pub profile: String,
    pub min_gap_secs: u64,
    /// paths the loop must never write under (courses, assessment sources, the layer files it improves)
    pub frozen: Vec<PathBuf>,
    /// The job's ledger (`ledger.jsonl`), shared by every instance that ever works the job: the item keys closed
    /// and the questions asked for it, so a job that moves to another instance (another terminal, another
    /// journal) never asks a question again. `None`: this loop's own journal only.
    pub ledger: Option<PathBuf>,
    /// this instance's name in the ledger
    pub member: String,
}

fn read(path: &Path) -> Result<Vec<Value>> {
    match std::fs::read_to_string(path) {
        Ok(t) => Ok(t.lines().filter_map(|l| serde_json::from_str(l).ok()).collect()),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(vec![]),
        Err(e) => Err(e.into()),
    }
}

fn append(path: &Path, v: &Value) -> Result<()> {
    if let Some(d) = path.parent() {
        std::fs::create_dir_all(d)?;
    }
    let mut f = std::fs::OpenOptions::new().create(true).append(true).open(path)?;
    f.write_all(format!("{v}\n").as_bytes())?;
    f.sync_data()?;
    Ok(())
}

impl Loop<'_> {
    fn journal(&self) -> PathBuf {
        self.state.join("work.jsonl")
    }
    fn log(&self, v: Value) -> Result<Value> {
        append(&self.journal(), &v)?;
        Ok(v)
    }

    /// One bounded step at time `now`. Returns what happened (`{"event": …}`); `stopped` ends the loop.
    pub fn step(&self, job: &dyn Job, term: &dyn PrivateTerminal, now: u64) -> Result<Value> {
        ensure!(self.state.is_absolute(), "the state folder is absolute");
        for f in &self.frozen {
            ensure!(!self.state.starts_with(f), "the state folder {} is inside the frozen path {}", self.state.display(), f.display());
        }
        let lines = read(&self.journal())?;
        // a stop holds until a later `resumed` line
        let stop = lines.iter().rposition(|l| l["event"] == "stopped");
        let resume = lines.iter().rposition(|l| l["event"] == "resumed");
        if let Some(i) = stop.filter(|i| resume.is_none_or(|r| r < *i)) {
            return Ok(json!({"event":"stopped","reason":lines[i]["reason"]}));
        }
        let ledger = match &self.ledger {
            Some(p) => read(p)?,
            None => vec![],
        };
        let closed = seen(&lines, &ledger);
        let Some(item) = job.next(&closed)? else { return Ok(json!({"event":"idle","reason":"nothing left to pick"})) };
        let native = job.native_proposal(&item);
        let question = match job.native(&item) {
            Native::Done(note) => {
                let mut v = json!({"event":"native","key":item.key,"note":note,"unix":now});
                self.attach_native(job, &item, native.as_deref(), &format!("n{now}-{:08x}", short_hash(&item.key)), &mut v, now)?;
                return self.close(v);
            }
            Native::Unsure(q) => q,
        };
        let task_sha = mova_octopus::digest(question.as_bytes());
        let id = format!("w{now}-{:08x}", short_hash(&item.key));
        if let Some(l) = ledger.iter().rev().find(|l| l["task_sha256"] == task_sha.as_str()).filter(|l| l["event"] != "unreserved") {
            // asked for this job before, by this instance or another: never again
            let mut v = json!({"event":"held","key":item.key,"reason":format!("this question was already asked for the job by {}", l["member"].as_str().unwrap_or("?")),"unix":now});
            self.attach_native(job, &item, native.as_deref(), &id, &mut v, now)?;
            return self.close(v);
        }
        let last = lines.iter().filter(|l| l["asked"] == true).filter_map(|l| l["unix"].as_u64()).max();
        if last.is_some_and(|t| now < t + self.min_gap_secs) {
            return Ok(json!({"event":"waiting","key":item.key,"reason":"pacing"}));
        }
        if self.ledger.is_some() && !term.ready() {
            // not logged nor reserved: the item stays open and is asked once the terminal is ready
            return Ok(json!({"event":"waiting","key":item.key,"reason":"the teacher terminal is not ready"}));
        }
        if let Some(p) = &self.ledger {
            // reserved in the job's ledger before the question: a job moved mid-question is never asked twice
            append(p, &json!({"event":"asked","key":item.key,"task_sha256":task_sha,"member":self.member,"unix":now}))?;
        }
        let a = match teacher::submit(self.teachers, term, &self.profile, job.purpose(), &question, &id, now) {
            Ok(a) => a,
            Err(e) => {
                let reason = format!("{e:#}");
                if teacher::unsent(&reason) {
                    // nothing reached the teacher (no terminal session): held once, retried once the terminal is ready
                    if let Some(p) = &self.ledger {
                        append(p, &json!({"event":"unreserved","key":item.key,"task_sha256":task_sha,"member":self.member,"unix":now}))?;
                    }
                    if unsent_holds(&lines, &ledger, &item.key) == 0 {
                        if let Some(p) = &self.ledger {
                            append(p, &json!({"event":"unsent","key":item.key,"member":self.member,"unix":now}))?;
                        }
                        return self.log(json!({"event":"held","key":item.key,"reason":reason,"unsent":true,"retry":"once, when the terminal is ready","unix":now}));
                    }
                }
                let mut v = json!({"event":"held","key":item.key,"reason":reason,"unsent":teacher::unsent(&reason),"unix":now});
                self.attach_native(job, &item, native.as_deref(), &id, &mut v, now)?;
                return self.close(v);
            }
        };
        if a["state"] == "unavailable" {
            // not logged: the item stays open and is asked once the terminal is ready
            if let Some(p) = &self.ledger {
                append(p, &json!({"event":"unreserved","key":item.key,"task_sha256":task_sha,"member":self.member,"unix":now}))?;
            }
            return Ok(json!({"event":"waiting","key":item.key,"reason":"the teacher terminal is not ready"}));
        }
        if a["state"] == "limit" {
            // the item stays open in this journal: after a resume it is asked again only if its question changed
            // (the journal of the terminal refuses the same question), else it is held for a person. In a job's
            // ledger it is held at once, so the job can move to another instance without asking it again.
            if let Some(p) = &self.ledger {
                append(p, &json!({"event":"held","key":item.key,"reason":"limit: the question is in the terminal journal of the instance that asked it","member":self.member,"unix":now}))?;
            }
            return self.log(json!({"event":"stopped","reason":"limit","key":item.key,"asked":true,"unix":now}));
        }
        let text = a["text"].as_str().unwrap_or("");
        match job.validate(&item, text) {
            Ok(p) => {
                let file = self.write_proposal(&format!("{id}.txt"), &p)?;
                let mut v = json!({"event":"proposed","key":item.key,"id":id,"proposal":file,"job":job.describe(),"answer_sha256":a["answer_sha256"],"admitted":false,"asked":true,"unix":now});
                self.attach_native(job, &item, native.as_deref(), &id, &mut v, now)?;
                job.learn(&item, &[&p], now)?;
                self.close(v)
            }
            Err(why) => {
                let mut v = json!({"event":"rejected","key":item.key,"id":id,"reason":why,"answer_sha256":a["answer_sha256"],"asked":true,"unix":now});
                self.attach_native(job, &item, native.as_deref(), &id, &mut v, now)?;
                self.close(v)
            }
        }
    }

    /// A review-only proposal file in the state's `proposals/` folder.
    fn write_proposal(&self, name: &str, text: &str) -> Result<PathBuf> {
        let dir = self.state.join("proposals");
        std::fs::create_dir_all(&dir)?;
        let file = dir.join(name);
        std::fs::write(&file, text)?;
        Ok(file)
    }

    /// Write what Mova settled natively in a closing item (`<id>.native.txt`), name it in the event, and let the job
    /// learn it.
    fn attach_native(&self, job: &dyn Job, item: &Item, native: Option<&str>, id: &str, v: &mut Value, now: u64) -> Result<()> {
        let Some(p) = native else { return Ok(()) };
        let file = self.write_proposal(&format!("{id}.native.txt"), p)?;
        v["native_proposal"] = json!(file);
        v["admitted"] = json!(false);
        job.learn(item, &[p], now)
    }

    /// Journal a closing event, and copy it into the job's ledger.
    fn close(&self, v: Value) -> Result<Value> {
        if let Some(p) = &self.ledger {
            let mut l = v.clone();
            l["member"] = json!(self.member);
            append(p, &l)?;
        }
        self.log(v)
    }
}

/// A hold of a question that never reached the teacher (no terminal session): `unsent`, or an older hold whose
/// reason says so.
fn unsent_hold(l: &Value) -> bool {
    (l["event"] == "held" && (l["unsent"] == true || l["reason"].as_str().is_some_and(teacher::unsent))) || l["event"] == "unsent"
}

/// How many unsent holds the journal and the ledger hold for a key.
pub fn unsent_holds(journal: &[Value], ledger: &[Value], key: &str) -> usize {
    journal.iter().chain(ledger).filter(|l| l["key"] == key && unsent_hold(l)).count()
}

/// The item keys a loop does not pick again: closed in its journal, or closed or asked in the job's ledger. The
/// first unsent hold of a key does not close it (the question is retried once when the terminal is ready).
pub fn seen(journal: &[Value], ledger: &[Value]) -> BTreeSet<String> {
    let mut unsent: BTreeMap<String, usize> = BTreeMap::new();
    let mut closing = |l: &Value| {
        if !matches!(l["event"].as_str(), Some("native" | "held" | "proposed" | "rejected")) {
            return false;
        }
        if unsent_hold(l) {
            let n = unsent.entry(l["key"].as_str().unwrap_or("").to_string()).or_default();
            *n += 1;
            return *n > 1;
        }
        true
    };
    let mut out: BTreeSet<String> = journal.iter().filter(|l| closing(l)).filter_map(|l| l["key"].as_str().map(String::from)).collect();
    let mut closed = BTreeSet::new();
    let mut unsent_l: BTreeMap<String, usize> = BTreeMap::new();
    for l in ledger {
        let Some(k) = l["key"].as_str() else { continue };
        match l["event"].as_str() {
            Some("asked") => {
                out.insert(k.to_string());
            }
            // a reservation the terminal never took (it was not ready): open again
            Some("unreserved") if !closed.contains(k) => {
                out.remove(k);
            }
            Some("unsent") => {
                *unsent_l.entry(k.to_string()).or_default() += 1;
            }
            Some("native" | "held" | "proposed" | "rejected") => {
                if unsent_hold(l) {
                    let n = unsent_l.entry(k.to_string()).or_default();
                    *n += 1;
                    if *n <= 1 {
                        continue;
                    }
                }
                closed.insert(k.to_string());
                out.insert(k.to_string());
            }
            _ => {}
        }
    }
    out
}

/// The keys a ledger file holds (closed or asked); empty when it does not exist yet.
pub fn ledger_seen(ledger: &Path) -> Result<BTreeSet<String>> {
    Ok(seen(&[], &read(ledger)?))
}

/// Words a teacher question asks at most (the operator's 20-25, well within the 100 000-byte bound).
pub const BATCH: usize = 25;

/// Nautilus's standing job on native drafts: the configured layers' words, layer by layer in name order, `batch` asked
/// words to a question. Each word is planned natively (`senses::plan`): junk (under three letters, scan
/// abbreviations) and words fewer than two of whose examples use them (Mova's lemmas, not prefixes) are not asked;
/// a confident overlap of the examples with one of the pupils' numbered senses is Mova's own pick, recorded with its
/// evidence; the rest are asked which numbered sense the examples show (`new:` and a sense only when none fits), with
/// two examples cut around the word. A word whose general sense the shared store already fixed is asked only whether
/// this layer uses it differently (`same`). The item key is `<layer>:<word>+<word>…`.
pub struct DictJob {
    /// (layer name, layer file, the dictionary's name)
    pub layers: Vec<(String, PathBuf, String)>,
    /// this job's part of `parts` (words whose hash modulo `parts` is `part`); 0 of 1 is the whole job
    pub part: u32,
    pub parts: u32,
    /// words asked per question
    pub batch: usize,
    /// the shared sense store
    pub store: crate::senses::Store,
    /// layers whose words are always asked, never picked natively (`animal::LayerConfig::native_picks` false): their
    /// examples are chosen to show different senses, and a single native pick would hide the others
    pub ask_all: BTreeSet<String>,
}

/// One row of a draft and its plan.
type Planned = (crate::layer::Row, crate::senses::WordPlan);

impl DictJob {
    pub fn whole(layers: Vec<(String, PathBuf, String)>) -> DictJob {
        DictJob { layers, part: 0, parts: 1, batch: BATCH, store: crate::senses::Store::default(), ask_all: BTreeSet::new() }
    }
    /// The words of a layer already in item keys: word keys, and the numeric keys of earlier tens (`<layer>:<k>`,
    /// the k-th ten rows of the file).
    fn done(name: &str, rows: &[crate::layer::Row], seen: &BTreeSet<String>) -> BTreeSet<String> {
        let p = format!("{name}:");
        let mut out = BTreeSet::new();
        for rest in seen.iter().filter_map(|k| k.strip_prefix(&p)) {
            match rest.parse::<usize>() {
                Ok(k) if k >= 1 => out.extend(rows.iter().skip((k - 1) * 10).take(10).map(|r| r.word.clone())),
                _ => out.extend(rest.split('+').map(String::from)),
            }
        }
        out
    }
    fn plan(&self, layer: &str, row: &crate::layer::Row, fixed: &BTreeMap<String, crate::senses::Fixed>) -> crate::senses::WordPlan {
        crate::senses::plan(&row.word, &row.examples, 2, crate::senses::parse(&row.meaning), fixed.get(&row.word), !self.ask_all.contains(layer))
    }
    /// The open rows of a layer in this part, in file order.
    fn open_rows(&self, name: &str, path: &Path, seen: &BTreeSet<String>) -> Result<Vec<crate::layer::Row>> {
        let rows = crate::layer::parse_rows(&std::fs::read_to_string(path)?);
        let done = Self::done(name, &rows, seen);
        Ok(rows.into_iter().filter(|r| short_hash(&r.word) % self.parts.max(1) == self.part && !done.contains(&r.word)).collect())
    }
    /// An item's layer, dictionary name and planned rows.
    fn planned(&self, item: &Item) -> (String, String, Vec<Planned>) {
        let (head, rows) = item.body.split_once('\n').unwrap_or((&item.body, ""));
        let (layer, name) = head.split_once('\t').unwrap_or(("", head));
        let fixed = self.store.fixed();
        let rows = crate::layer::parse_rows(rows).into_iter().map(|r| {
            let p = self.plan(layer, &r, &fixed);
            (r, p)
        });
        (layer.to_string(), name.to_string(), rows.collect())
    }
}

/// The teacher question for the asked words of a draft layer; `None` when none is asked.
pub fn draft_task(name: &str, plans: &[&crate::senses::WordPlan]) -> Option<String> {
    if plans.is_empty() {
        return None;
    }
    let mut t = format!(
        "You are choosing senses for {name}, one of Mova's dictionary layers, for a clever pupil of 9-10. For each word \
below, the examples come from the layer's source and the senses are the numbered senses of the pupils' dictionary. \
Answer with exactly one line per word: the word, a TAB, then the number of the sense the examples show (two numbers \
like 2,5 only when the examples show both). Only when no listed sense fits, write new: and one or two sentences on the \
sense the examples show. For a word with a fixed sense (fixed: N), write same when these examples use that sense. Use \
only the examples; do not invent facts. Write - when the examples do not use the word. No other lines.\n\n"
    );
    for p in plans {
        let item = format!("word: {}\n{}\n", p.word, p.block());
        if t.len() + item.len() > teacher::TASK_MAX {
            break;
        }
        t.push_str(&item);
    }
    Some(t)
}

/// A draft question's answer → pick lines (`senses::pick_line`), the word's first well-formed line winning; at least
/// half of the asked words need one. A new sense is 20-600 characters and does not say the word is absent; a new
/// sense that restates a numbered one is that sense ("restates sense N").
pub fn draft_validate(layer: &str, plans: &[&crate::senses::WordPlan], answer: &str) -> std::result::Result<String, String> {
    use crate::senses::{Pick, pick_line, text_of};
    let by: BTreeMap<&str, &crate::senses::WordPlan> = plans.iter().map(|p| (p.word.as_str(), *p)).collect();
    let mut formed = BTreeSet::new();
    let mut out: BTreeMap<String, String> = BTreeMap::new();
    let mut absent = 0;
    for l in answer.lines() {
        let Some((w, rest)) = l.split_once('\t') else { continue };
        let (w, rest) = (w.trim(), rest.trim());
        let Some(p) = by.get(w) else { continue };
        if formed.contains(w) || rest.contains('\t') {
            continue;
        }
        if crate::senses::absent(rest) {
            absent += 1;
            continue;
        }
        let line = match Pick::parse(rest, p.senses.len()) {
            None => continue,
            Some(Pick::Keep) => None,
            Some(Pick::Same) => match &p.fixed {
                Some(f) => Some(pick_line(w, "same", &f.sense, "teacher", &format!("as the sense {} fixed by {}", f.pick, f.layer))),
                None => continue,
            },
            Some(Pick::Senses(v)) => Some(pick_line(w, &Pick::Senses(v.clone()).label(), &text_of(&v, &p.senses), "teacher", "the teacher's pick")),
            Some(Pick::New(t)) => {
                if !(20..=600).contains(&t.chars().count()) {
                    continue;
                }
                match crate::senses::restates(w, &t, &p.senses) {
                    Some(n) => Some(pick_line(w, &n.to_string(), &text_of(&[n], &p.senses), "teacher", &format!("restates sense {n}: {t}"))),
                    None => Some(pick_line(w, "new", &t, "teacher", "a sense none of the numbered ones gives")),
                }
            }
        };
        formed.insert(w.to_string());
        if let Some(line) = line {
            out.insert(w.to_string(), line);
        }
    }
    if formed.is_empty() || formed.len() * 2 < plans.len() {
        return Err(format!("{} of {} asked words got a usable pick ({absent} said the word is absent)", formed.len(), plans.len()));
    }
    if out.is_empty() {
        return Err("no word got a sense".into());
    }
    Ok(format!("# senses of {layer}: review-only picks; a number is a sense of the pupils' dictionary, new a sense none of them gives\n{}", out.values().map(|l| format!("{l}\n")).collect::<String>()))
}

impl Job for DictJob {
    fn purpose(&self) -> &'static str {
        "dictionary"
    }
    fn open(&self, seen: &BTreeSet<String>) -> Result<usize> {
        // the items `next` would make, one after another
        let (fixed, b) = (self.store.fixed(), self.batch.max(1));
        let mut n = 0;
        for (name, path, _) in &self.layers {
            let (mut asked, mut taken) = (0, 0);
            for r in self.open_rows(name, path, seen)? {
                if asked >= b || taken >= 4 * b {
                    n += 1;
                    (asked, taken) = (0, 0);
                }
                asked += usize::from(self.plan(name, &r, &fixed).fate == crate::senses::Fate::Ask);
                taken += 1;
            }
            n += usize::from(taken > 0);
        }
        Ok(n)
    }
    fn describe(&self) -> Value {
        json!({"layers":self.layers.iter().map(|l| &l.0).collect::<Vec<_>>(),"target":"draft senses","part":self.part,"parts":self.parts,"batch":self.batch})
    }
    fn next(&self, seen: &BTreeSet<String>) -> Result<Option<Item>> {
        let fixed = self.store.fixed();
        for (name, path, source) in &self.layers {
            let mut take = Vec::new();
            let mut asked = 0;
            for r in self.open_rows(name, path, seen)? {
                if asked >= self.batch.max(1) || take.len() >= 4 * self.batch.max(1) {
                    break;
                }
                asked += usize::from(self.plan(name, &r, &fixed).fate == crate::senses::Fate::Ask);
                take.push(r);
            }
            if !take.is_empty() {
                return Ok(Some(Item {
                    key: format!("{name}:{}", take.iter().map(|r| r.word.as_str()).collect::<Vec<_>>().join("+")),
                    body: format!("{name}\t{source}\n{}", take.iter().map(|r| r.tsv()).collect::<Vec<_>>().join("\n")),
                }));
            }
        }
        Ok(None)
    }
    fn native(&self, item: &Item) -> Native {
        let (_, name, rows) = self.planned(item);
        let ask: Vec<&crate::senses::WordPlan> = rows.iter().map(|r| &r.1).filter(|p| p.fate == crate::senses::Fate::Ask).collect();
        match draft_task(&name, &ask) {
            Some(q) => Native::Unsure(q),
            None => {
                let n = |f: fn(&crate::senses::Fate) -> bool| rows.iter().filter(|r| f(&r.1.fate)).count();
                Native::Done(format!(
                    "{} words: {} picked by Mova, {} junk, {} with fewer than two examples using them",
                    rows.len(),
                    n(|f| matches!(f, crate::senses::Fate::Native { .. })),
                    n(|f| matches!(f, crate::senses::Fate::Junk(_))),
                    n(|f| matches!(f, crate::senses::Fate::Thin(_)))
                ))
            }
        }
    }
    fn validate(&self, item: &Item, answer: &str) -> std::result::Result<String, String> {
        let (layer, _, rows) = self.planned(item);
        let ask: Vec<&crate::senses::WordPlan> = rows.iter().map(|r| &r.1).filter(|p| p.fate == crate::senses::Fate::Ask).collect();
        draft_validate(&layer, &ask, answer)
    }
    fn native_proposal(&self, item: &Item) -> Option<String> {
        let (layer, _, rows) = self.planned(item);
        let lines: Vec<String> = rows.iter().filter_map(|r| r.1.native_line()).collect();
        (!lines.is_empty()).then(|| format!("# senses of {layer} settled by Mova: review-only\n{}", lines.iter().map(|l| format!("{l}\n")).collect::<String>()))
    }
    fn learn(&self, item: &Item, proposals: &[&str], now: u64) -> Result<()> {
        let layer = item.key.split(':').next().unwrap_or("");
        for p in proposals {
            self.store.append(&crate::senses::records(p, layer, now))?;
        }
        Ok(())
    }
}

/// A dry measurement of a job: its first `items` items in the loop's order, each taken as closed once seen, with no
/// terminal, no journal and nothing written. Per item: the words it takes, those a question would ask (`word:`
/// lines), those Mova settles natively (`# pick` lines of the native proposal), those never asked (`# not asked`) and
/// the question's bytes.
pub fn dry(job: &dyn Job, items: usize) -> Result<Value> {
    let mut seen = BTreeSet::new();
    let mut rows = Vec::new();
    let (mut words, mut asked, mut native, mut not_asked, mut questions, mut bytes) = (0, 0, 0, 0, 0, 0);
    while rows.len() < items {
        let Some(item) = job.next(&seen)? else { break };
        let n_words = item.key.split_once(':').map_or(0, |(_, w)| w.split('+').count());
        let np = job.native_proposal(&item).unwrap_or_default();
        let (n_native, n_not) = (np.lines().filter(|l| l.starts_with("# pick\t")).count(), np.lines().filter(|l| l.starts_with("# not asked\t")).count());
        let (n_asked, q_bytes) = match job.native(&item) {
            Native::Unsure(q) => (q.lines().filter(|l| l.starts_with("word: ")).count(), q.len()),
            Native::Done(_) => (0, 0),
        };
        (words, asked, native, not_asked, bytes) = (words + n_words, asked + n_asked, native + n_native, not_asked + n_not, bytes + q_bytes);
        questions += usize::from(n_asked > 0);
        // the native picks as `word=pick`, for review
        let picks: Vec<String> = np.lines().filter_map(|l| l.strip_prefix("# pick\t")).filter_map(|l| {
            let mut c = l.split('\t');
            Some(format!("{}={}", c.next()?, c.next()?))
        }).collect();
        rows.push(json!({"key":item.key,"words":n_words,"asked":n_asked,"native":n_native,"not_asked":n_not,"question_bytes":q_bytes,"native_picks":picks}));
        seen.insert(item.key);
    }
    Ok(json!({"items":rows.len(),"questions":questions,"words":words,"asked":asked,"native":native,"not_asked":not_asked,"question_bytes":bytes,"open_after":job.open(&seen)?,"per_item":rows}))
}

/// Resume a loop stopped by a limit, once the limit has passed: one `resumed` line.
pub fn resume(state: &Path, now: u64) -> Result<()> {
    append(&state.join("work.jsonl"), &json!({"event":"resumed","unix":now}))
}

/// Run the loop: a step every `every_secs` (four times that while idle or waiting), at most `steps` steps if given.
/// A `STOP` file in the state folder ends this run; a limit ends it and holds it (journaled `stopped`) until `resume`.
/// Prints each step's line.
pub fn run(l: &Loop, job: &dyn Job, term: &dyn PrivateTerminal, every_secs: u64, steps: Option<u64>) -> Result<Value> {
    let now = || std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    let mut n = 0u64;
    loop {
        // a STOP file ends this run only (Crab's stop marker); the next start continues
        if l.state.join("STOP").exists() {
            break;
        }
        let v = l.step(job, term, now())?;
        println!("{v}");
        n += 1;
        if v["event"] == "stopped" || steps.is_some_and(|k| n >= k) {
            break;
        }
        let pause = if matches!(v["event"].as_str(), Some("idle" | "waiting")) { every_secs * 4 } else { every_secs };
        std::thread::sleep(std::time::Duration::from_secs(pause.max(1)));
    }
    summary(&l.state)
}

/// FNV-1a, 32 bits, for question ids.
pub fn short_hash(s: &str) -> u32 {
    let mut h: u32 = 0x811c_9dc5;
    for b in s.bytes() {
        h ^= u32::from(b);
        h = h.wrapping_mul(0x0100_0193);
    }
    h
}

/// The journal's counts and the throughput (items closed per hour between the first and the last line).
pub fn summary(state: &Path) -> Result<Value> {
    let lines = read(&state.join("work.jsonl"))?;
    let mut by: BTreeMap<String, u64> = BTreeMap::new();
    let mut reasons: BTreeMap<String, u64> = BTreeMap::new();
    for l in &lines {
        *by.entry(l["event"].as_str().unwrap_or("?").to_string()).or_default() += 1;
        if l["event"] == "rejected" || l["event"] == "held" {
            let r: String = l["reason"].as_str().unwrap_or("").chars().take(80).collect();
            *reasons.entry(r).or_default() += 1;
        }
    }
    let times: Vec<u64> = lines.iter().filter_map(|l| l["unix"].as_u64()).collect();
    let closed = ["native", "proposed", "rejected", "held"].iter().map(|e| by.get(*e).copied().unwrap_or(0)).sum::<u64>();
    let span = times.iter().max().zip(times.iter().min()).map(|(a, b)| a - b).unwrap_or(0);
    let per_hour = if span == 0 { None } else { Some((closed as f64) * 3600.0 / span as f64) };
    let native = lines.iter().filter(|l| l["native_proposal"].is_string()).count();
    Ok(json!({"events":by,"closed":closed,"span_secs":span,"closed_per_hour":per_hour,"admitted":0,"native_proposals":native,
        "rejection_and_hold_reasons":reasons,"note":"proposals are review-only; admission is a separate reviewed decision"}))
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::teacher::{Profile, TerminalError};
    use std::cell::RefCell;

    struct Words(Vec<&'static str>);
    impl Job for Words {
        fn purpose(&self) -> &'static str {
            "dictionary"
        }
        fn next(&self, seen: &BTreeSet<String>) -> Result<Option<Item>> {
            Ok(self.0.iter().find(|w| !seen.contains(**w) && !w.ends_with("-test")).map(|w| Item { key: w.to_string(), body: w.to_string() }))
        }
        fn native(&self, i: &Item) -> Native {
            if i.key.starts_with('n') { Native::Done("known".into()) } else { Native::Unsure(format!("What does {} mean here?", i.key)) }
        }
        fn validate(&self, i: &Item, a: &str) -> std::result::Result<String, String> {
            if a.starts_with(&format!("{}\t", i.key)) { Ok(a.into()) } else { Err("not an answer for this word".into()) }
        }
        fn open(&self, seen: &BTreeSet<String>) -> Result<usize> {
            Ok(self.0.iter().filter(|w| !seen.contains(**w)).count())
        }
    }
    /// A fake private terminal answering from a script.
    struct Term {
        replies: RefCell<Vec<std::result::Result<String, TerminalError>>>,
        asked: RefCell<Vec<String>>,
        down: std::cell::Cell<bool>,
    }
    impl Term {
        fn new(r: Vec<std::result::Result<String, TerminalError>>) -> Term {
            Term { replies: RefCell::new(r), asked: RefCell::new(vec![]), down: std::cell::Cell::new(false) }
        }
    }
    impl PrivateTerminal for Term {
        fn id(&self) -> &str {
            "nautilus-sol"
        }
        fn ask(&self, q: &str) -> std::result::Result<String, TerminalError> {
            self.asked.borrow_mut().push(q.into());
            self.replies.borrow_mut().remove(0)
        }
        fn ready(&self) -> bool {
            !self.down.get()
        }
    }
    fn setup(word: Option<&str>) -> (PathBuf, teacher::Config) {
        let d = std::env::temp_dir().join(format!("nautilus-work-{}-{}", std::process::id(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        std::fs::create_dir_all(&d).unwrap();
        let p = Profile { name: "sol".into(), provider: "codex".into(), model: "gpt-6.1-sol".into(), terminal: "nautilus-sol".into(), terminal_profile: d.join("sol-terminal.json"), purposes: vec!["dictionary".into()] };
        (d.clone(), teacher::Config { journal: d.join("teacher.jsonl"), profiles: vec![p], operator_word: word.map(String::from) })
    }
    #[test]
    fn an_unready_terminal_leaves_the_item_open_and_asks_nothing() {
        let (d, t) = setup(Some("test"));
        let l = Loop { state: d.join("state"), teachers: &t, profile: "sol".into(), min_gap_secs: 60, frozen: vec![d.join("courses")], ledger: None, member: "a".into() };
        let job = Words(vec!["pass"]);
        let term = Term::new(vec![Ok("pass\tto hand over".into())]);
        term.down.set(true);
        let w = l.step(&job, &term, 100).unwrap();
        assert_eq!((w["event"].as_str(), w["key"].as_str()), (Some("waiting"), Some("pass")), "{w}");
        assert!(term.asked.borrow().is_empty());
        assert!(teacher::journal(&t.journal).unwrap().is_empty(), "nothing journaled while unready");
        term.down.set(false);
        assert_eq!(l.step(&job, &term, 101).unwrap()["event"], "proposed");
    }
    #[test]
    fn native_then_teacher_then_review_only_proposal() {
        let (d, t) = setup(Some("test"));
        let l = Loop { state: d.join("state"), teachers: &t, profile: "sol".into(), min_gap_secs: 60, frozen: vec![d.join("courses")], ledger: None, member: "a".into() };
        let job = Words(vec!["nod", "x-test", "pass", "shut"]);
        let term = Term::new(vec![Ok("pass\tto hand over".into()), Ok("nope".into())]);
        assert_eq!(l.step(&job, &term, 100).unwrap()["event"], "native");
        let p = l.step(&job, &term, 101).unwrap();
        assert_eq!((p["event"].as_str(), p["key"].as_str(), p["admitted"].as_bool()), (Some("proposed"), Some("pass"), Some(false)));
        assert_eq!(std::fs::read_to_string(p["proposal"].as_str().unwrap()).unwrap(), "pass\tto hand over");
        // pacing: the next question waits until 60 s after the last; nothing more is asked
        assert_eq!(l.step(&job, &term, 102).unwrap()["reason"], "pacing");
        assert_eq!(term.asked.borrow().len(), 1);
        assert_eq!(l.step(&job, &term, 161).unwrap()["event"], "rejected");
        // the held-out item is never picked
        assert_eq!(l.step(&job, &term, 222).unwrap()["event"], "idle");
        let sum = summary(&l.state).unwrap();
        assert_eq!(sum["events"], json!({"native":1,"proposed":1,"rejected":1}));
        assert_eq!((sum["closed"].as_u64(), sum["admitted"].as_u64()), (Some(3), Some(0)));
    }
    fn row(word: &str, meaning: &str, examples: &[&str]) -> crate::layer::Row {
        crate::layer::Row { word: word.into(), pos: "noun".into(), meaning: meaning.into(), sense: "Counted.".into(), examples: examples.iter().map(|e| e.to_string()).collect(), note: String::new() }
    }
    const WATER: &str = "1) Water is the clear liquid in rivers, lakes, rain and the sea; animals and plants need it to live. 2) To water plants is to give them water. 3) Your eyes or mouth water when tears come.";
    /// A draft of 30 asked words (two uses each, senses the examples do not settle), a native pick, junk and a thin word.
    fn draft(d: &Path) -> PathBuf {
        let mut rows = vec![
            row("water", WATER, &["Rain fills the rivers and lakes with fresh water. «a:1»", "The sea and the lakes hold most of the water. «a:2»"]),
            row("br", "", &["It bred worms. «a:3»", "Steeds bred of the stud. «a:4»"]),
            row("lonely", "1) Sad because you are alone.", &["A lonely night. «a:5»"]),
        ];
        for i in 0..30u8 {
            let w = format!("wor{}{}", (b'a' + i / 26) as char, (b'a' + i % 26) as char);
            rows.push(row(&w, "1) A round stone for grinding grain. 2) A wooden box for keeping letters.", &[&format!("The {w} is here. «b:{i}»"), &format!("We saw a {w} there. «c:{i}»")]));
        }
        let f = d.join("commands.tsv");
        std::fs::write(&f, rows.iter().map(|r| r.tsv() + "\n").collect::<String>()).unwrap();
        f
    }
    #[test]
    fn dictionary_job_plans_natively_and_asks_in_batches() {
        let (d, _) = setup(None);
        let mut job = DictJob::whole(vec![("commands".into(), draft(&d), "the commands dictionary".into())]);
        job.store = crate::senses::Store::at(Some(d.join("senses.jsonl")));
        let first = job.next(&BTreeSet::new()).unwrap().unwrap();
        // 25 asked words, the native and the junk and thin ones riding along
        assert!(first.key.starts_with("commands:water+br+lonely+woraa+"), "{}", first.key);
        assert_eq!(first.key.split('+').count(), 28);
        let Native::Unsure(task) = job.native(&first) else { panic!("unsure expected") };
        assert!(task.contains("for the commands dictionary, one of Mova's dictionary layers") && !task.contains("dictionary dictionary"), "{task}");
        assert!(!task.contains("word: water") && !task.contains("word: br\n") && !task.contains("word: lonely") && !task.contains("counted"), "{task}");
        assert!(task.contains("senses: 1) A round stone for grinding grain. | 2) A wooden box for keeping letters.") && task.contains("examples: The worab is here. | We saw a worab there.\n"), "{task}");
        let native = job.native_proposal(&first).unwrap();
        assert!(native.contains("# pick\twater\t1\t1) Water is the clear liquid") && native.contains("\tnative\tMova's overlap"), "{native}");
        assert!(native.contains("# not asked\tbr\tshorter than three letters") && native.contains("# not asked\tlonely\t1 example(s)"), "{native}");
        // the answer: picks, a new sense, a restatement, an absent line, a word not asked
        let words: Vec<String> = task.lines().filter_map(|l| l.strip_prefix("word: ")).map(String::from).collect();
        assert_eq!(words.len(), BATCH);
        let mut ans: String = words.iter().skip(3).map(|w| format!("{w}\t2\n")).collect();
        ans.push_str(&format!("{}\tnew: A small tool that the lessons use for cutting paper.\n", words[0]));
        ans.push_str(&format!("{}\tnew: A wooden box that people use for keeping their letters.\n", words[1]));
        ans.push_str(&format!("{}\tThe examples do not contain the word.\nwater\t3\n", words[2]));
        let p = job.validate(&first, &ans).unwrap();
        assert!(p.contains(&format!("# pick\t{}\tnew\tA small tool", words[0])), "{p}");
        assert!(p.contains(&format!("# pick\t{}\t2\t2) A wooden box for keeping letters.\tteacher\trestates sense 2", words[1])), "{p}");
        assert!(!p.contains(&format!("\t{}\t", words[2])) && !p.contains("\twater\t"), "absent and not asked: {p}");
        // red: too few usable picks, numbers out of range, «same» without a fixed sense
        let few: String = words.iter().take(12).map(|w| format!("{w}\t1\n")).collect();
        assert!(job.validate(&first, &few).is_err());
        assert!(job.validate(&first, &words.iter().map(|w| format!("{w}\t9\n")).collect::<String>()).is_err());
        assert!(job.validate(&first, &words.iter().map(|w| format!("{w}\tsame\n")).collect::<String>()).is_err());
        // the second item: the last five words
        let second = job.next(&BTreeSet::from([first.key.clone()])).unwrap().unwrap();
        assert_eq!(second.key.split('+').count(), 5);
        assert_eq!(job.open(&BTreeSet::from([first.key.clone()])).unwrap(), 1);
        assert!(job.next(&BTreeSet::from([first.key, second.key])).unwrap().is_none());
    }
    #[test]
    fn an_ask_all_layer_has_no_native_picks() {
        let (d, _) = setup(None);
        let mut job = DictJob::whole(vec![("commands".into(), draft(&d), "the commands dictionary".into())]);
        job.store = crate::senses::Store::at(Some(d.join("senses.jsonl")));
        let usual = dry(&job, 10).unwrap();
        assert!(usual["native"].as_u64().unwrap() >= 1, "the control: the fixture has a native pick ({usual})");
        job.ask_all.insert("commands".into());
        let v = dry(&job, 10).unwrap();
        assert_eq!(v["native"].as_u64(), Some(0), "{v}");
        // the word Mova picked natively is asked now; junk and thin words are still never asked
        assert_eq!(v["asked"].as_u64().unwrap(), usual["asked"].as_u64().unwrap() + usual["native"].as_u64().unwrap(), "{v}");
        assert_eq!(v["not_asked"], usual["not_asked"]);
        let first = job.next(&BTreeSet::new()).unwrap().unwrap();
        let Native::Unsure(q) = job.native(&first) else { panic!("unsure expected") };
        assert!(q.contains("word: water\n"), "{q}");
    }
    #[test]
    fn a_dry_run_counts_without_writing() {
        let (d, _) = setup(None);
        let mut job = DictJob::whole(vec![("commands".into(), draft(&d), "the commands dictionary".into())]);
        job.store = crate::senses::Store::at(Some(d.join("senses.jsonl")));
        let before: Vec<_> = std::fs::read_dir(&d).unwrap().flatten().map(|e| e.file_name()).collect();
        let v = dry(&job, 10).unwrap();
        // the two items of the fixture: 25 asked in the first, the native, junk and thin words riding along
        assert_eq!((v["items"].as_u64(), v["questions"].as_u64(), v["words"].as_u64(), v["open_after"].as_u64()), (Some(2), Some(2), Some(33), Some(0)), "{v}");
        assert_eq!(v["per_item"][0]["asked"].as_u64(), Some(BATCH as u64));
        assert!(v["native"].as_u64().unwrap() >= 1 && v["not_asked"].as_u64().unwrap() >= 2, "{v}");
        assert_eq!(v["asked"].as_u64().unwrap() + v["native"].as_u64().unwrap() + v["not_asked"].as_u64().unwrap(), 33, "every word is counted once: {v}");
        // nothing written: no journal, no store
        let after: Vec<_> = std::fs::read_dir(&d).unwrap().flatten().map(|e| e.file_name()).collect();
        assert_eq!(before, after);
        // negative control: a job with nothing left measures nothing
        assert_eq!(dry(&job, 0).unwrap()["items"].as_u64(), Some(0));
    }
    #[test]
    fn earlier_tens_stay_done() {
        let (d, _) = setup(None);
        let job = DictJob::whole(vec![("commands".into(), draft(&d), "the commands dictionary".into())]);
        // the numeric key of the first ten of the file closes its ten words
        let first = job.next(&BTreeSet::from(["commands:1".to_string()])).unwrap().unwrap();
        assert!(first.key.starts_with("commands:worah+"), "{}", first.key);
    }
    #[test]
    fn the_store_fixes_a_sense_once_for_every_layer() {
        let (d, t) = setup(Some("test"));
        let store = crate::senses::Store::at(Some(d.join("senses.jsonl")));
        let f = draft(&d);
        let one = DictJob { layers: vec![("science".into(), f.clone(), "the school science dictionary".into())], part: 0, parts: 1, batch: 40, store: store.clone(), ask_all: BTreeSet::new() };
        let l = Loop { state: d.join("one"), teachers: &t, profile: "sol".into(), min_gap_secs: 0, frozen: vec![], ledger: None, member: "a".into() };
        let words: Vec<String> = (0..30u8).map(|i| format!("wor{}{}", (b'a' + i / 26) as char, (b'a' + i % 26) as char)).collect();
        let term = Term::new(vec![Ok(words.iter().map(|w| format!("{w}\t2\n")).collect())]);
        let v = l.step(&one, &term, 1).unwrap();
        assert_eq!(v["event"], "proposed", "{v}");
        assert!(v["native_proposal"].is_string(), "{v}");
        let fixed = store.fixed();
        assert_eq!((fixed["worab"].pick.as_str(), fixed["worab"].layer.as_str(), fixed["water"].by.as_str()), ("2", "science", "native"));
        // another layer: the fixed words are asked only whether it uses them differently
        let two = DictJob { layers: vec![("everyday".into(), f, "the school everyday dictionary".into())], part: 0, parts: 1, batch: 40, store: store.clone(), ask_all: BTreeSet::new() };
        let item = two.next(&BTreeSet::new()).unwrap().unwrap();
        let Native::Unsure(q) = two.native(&item) else { panic!() };
        assert!(q.contains("word: worab\nsenses: 1) A round stone for grinding grain. | 2) A wooden box for keeping letters.\nfixed: 2 (by science)"), "{q}");
        let p = two.validate(&item, &words.iter().map(|w| format!("{w}\tsame\n")).collect::<String>()).unwrap();
        assert!(p.contains("# pick\tworab\tsame\t2) A wooden box for keeping letters.\tteacher\tas the sense 2 fixed by science"), "{p}");
        // water: Mova's pick agrees with the fixed sense, natively again
        assert!(two.native_proposal(&item).unwrap().contains("agrees with sense 1 fixed by science"));
    }
    #[test]
    fn an_unsent_question_is_retried_once() {
        let (d, t) = setup(Some("test"));
        let l = Loop { state: d.join("state"), teachers: &t, profile: "sol".into(), min_gap_secs: 0, frozen: vec![], ledger: Some(d.join("ledger.jsonl")), member: "a".into() };
        let job = Words(vec!["pass", "shut"]);
        let gone = || Err(TerminalError::Failed("terminal not idle: no such session".into()));
        let term = Term::new(vec![gone(), Ok("pass\tto hand over".into())]);
        let h = l.step(&job, &term, 1).unwrap();
        assert!(h["event"] == "held" && h["unsent"] == true, "{h}");
        // the same item again: asked once more, and answered
        let p = l.step(&job, &term, 2).unwrap();
        assert_eq!((p["event"].as_str(), p["key"].as_str()), (Some("proposed"), Some("pass")), "{p}");
        assert_eq!(term.asked.borrow().len(), 2);
        // twice unsent: held for good
        let term2 = Term::new(vec![gone(), gone()]);
        assert_eq!(l.step(&job, &term2, 3).unwrap()["key"], "shut");
        assert_eq!(l.step(&job, &term2, 4).unwrap()["key"], "shut");
        assert_eq!(l.step(&job, &term2, 5).unwrap()["event"], "idle");
        assert_eq!(term2.asked.borrow().len(), 2);
        // red control: another failure is held at once, never retried
        let (d3, t3) = setup(Some("test"));
        let l3 = Loop { state: d3.join("state"), teachers: &t3, profile: "sol".into(), min_gap_secs: 0, frozen: vec![], ledger: None, member: "a".into() };
        let term3 = Term::new(vec![Err(TerminalError::Failed("crashed".into()))]);
        assert_eq!(l3.step(&Words(vec!["pass"]), &term3, 1).unwrap()["event"], "held");
        assert_eq!(l3.step(&Words(vec!["pass"]), &term3, 2).unwrap()["event"], "idle");
        // an older unsent hold (its reason only) reopens its key once
        let old = json!({"event":"held","key":"commands:1","reason":"the terminal did not answer: terminal not idle: no such session; it is not retried"});
        assert!(!seen(std::slice::from_ref(&old), &[]).contains("commands:1"));
        assert!(seen(&[old.clone(), old], &[]).contains("commands:1"));
    }
    #[test]
    fn limits_words_and_frozen_paths_stop_cleanly() {
        let (d, t) = setup(Some("test"));
        let l = Loop { state: d.join("state"), teachers: &t, profile: "sol".into(), min_gap_secs: 0, frozen: vec![], ledger: None, member: "a".into() };
        let job = Words(vec!["pass", "shut"]);
        let term = Term::new(vec![Err(TerminalError::Limit("usage limit".into())), Ok("shut\tto close".into())]);
        assert_eq!(l.step(&job, &term, 1).unwrap()["reason"], "limit");
        // stopped stays stopped; nothing more is asked
        assert_eq!(l.step(&job, &term, 2).unwrap()["event"], "stopped");
        assert_eq!(term.asked.borrow().len(), 1);
        // resumed: the item cut by the limit is not asked again blindly (held), the next one is asked
        resume(&l.state, 3).unwrap();
        let h = l.step(&job, &term, 3).unwrap();
        assert!(h["event"] == "held" && h["reason"].as_str().unwrap().contains("as limit"), "{h}");
        assert_eq!(l.step(&job, &term, 4).unwrap()["event"], "proposed");
        assert_eq!(term.asked.borrow().len(), 2);
        // without the operator's word the item is held, nothing is asked, and it is not picked again
        let (d2, t2) = setup(None);
        let l2 = Loop { state: d2.join("state"), teachers: &t2, profile: "sol".into(), min_gap_secs: 0, frozen: vec![], ledger: None, member: "a".into() };
        let term2 = Term::new(vec![]);
        let h = l2.step(&Words(vec!["pass"]), &term2, 1).unwrap();
        assert!(h["reason"].as_str().unwrap().contains("operator has not given the word"), "{h}");
        assert_eq!(l2.step(&Words(vec!["pass"]), &term2, 2).unwrap()["event"], "idle");
        assert!(term2.asked.borrow().is_empty());
        // a state folder inside a frozen path is refused before anything
        let l3 = Loop { state: d.join("courses/state"), teachers: &t, profile: "sol".into(), min_gap_secs: 0, frozen: vec![d.join("courses")], ledger: None, member: "a".into() };
        assert!(l3.step(&job, &term2, 1).is_err());
    }
}
