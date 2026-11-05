//! Nautilus as a member of a work pool: Crab hands out the work.
//!
//! Crab (`tools/crab`, `flow.pools`) leases one job at a time to each idle member by writing the member's assignment
//! file: `{"schema":1,"pool","member","job","part","parts","lease_id","assigned_unix","expires_unix","params":{"layer":…}}`.
//! The member reads it between steps, never mid-question: each step works the assigned layer (part `part` of
//! `parts`), and an expired, released or missing lease means no work. After every step the member writes its report
//! (`{"lease_id","job","state","open_items","unix"}`, `state` one of `working`, `finished`, `stopped`, `idle`,
//! `refused`) and the job's backlog file, which Crab's backlog probe reads.
//!
//! **A job moves without repeating questions.** Every job has its own folder under the ledgers root:
//! `ledger.jsonl` (the item keys closed and the questions asked for the job, by whichever member; `work::Loop`
//! writes it) and `backlog.json`. A member's own `work.jsonl`, its teacher journal and its pacing stay its own (one
//! terminal per member). A draft layer (`commands`, `wikipedia`, `course`) not yet made is first made natively.
use crate::animal::{self, Config};
use crate::teacher::{self, PrivateTerminal};
use crate::work::{self, Loop};
use anyhow::{Context, Result, ensure};
use serde::Deserialize;
use serde_json::{Value, json};
use std::path::{Path, PathBuf};

/// The bound on an assignment file.
const ASSIGNMENT_MAX: u64 = 64 * 1024;

#[derive(Clone, Debug, Deserialize, PartialEq)]
pub struct Assignment {
    pub schema: u32,
    #[serde(default)]
    pub pool: String,
    #[serde(default)]
    pub member: String,
    /// `None`: released
    #[serde(default)]
    pub job: Option<String>,
    #[serde(default)]
    pub part: u32,
    #[serde(default = "one")]
    pub parts: u32,
    #[serde(default)]
    pub lease_id: Option<String>,
    #[serde(default)]
    pub expires_unix: u64,
    #[serde(default)]
    pub params: Value,
}
fn one() -> u32 {
    1
}

/// The member's current assignment at `now`: the file's lease when it names a job and has not expired.
pub fn current(path: &Path, member: &str, now: u64) -> Result<Option<Assignment>> {
    let f = match std::fs::File::open(path) {
        Ok(f) => f,
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(None),
        Err(e) => return Err(e.into()),
    };
    let mut b = Vec::new();
    std::io::Read::read_to_end(&mut std::io::Read::take(f, ASSIGNMENT_MAX + 1), &mut b)?;
    ensure!(b.len() as u64 <= ASSIGNMENT_MAX, "the assignment is over the bound");
    let a: Assignment = serde_json::from_slice(&b).context("the assignment")?;
    ensure!(a.schema == 1, "assignment schema 1 expected");
    if a.job.is_none() || a.lease_id.is_none() || now >= a.expires_unix {
        return Ok(None);
    }
    ensure!(a.member == member, "the assignment is for {}, not {member}", a.member);
    ensure!(a.parts >= 1 && a.part < a.parts, "part {} of {}", a.part, a.parts);
    Ok(Some(a))
}

fn atomic(path: &Path, v: &Value) -> Result<()> {
    if let Some(d) = path.parent() {
        std::fs::create_dir_all(d)?;
    }
    let tmp = path.with_extension("tmp");
    std::fs::write(&tmp, format!("{v}\n"))?;
    std::fs::rename(tmp, path)?;
    Ok(())
}

/// A job's folder under the ledgers root.
pub fn job_dir(ledgers: &Path, layer: &str) -> PathBuf {
    ledgers.join(layer)
}

/// Open items of a whole job (every part) against its ledger, and write its `backlog.json`.
pub fn backlog(cfg: &Config, layer: &str, ledgers: &Path, now: u64) -> Result<Value> {
    let l = cfg.layers.get(layer).with_context(|| format!("layer {layer} is not configured"))?;
    let dir = job_dir(ledgers, layer);
    let seen = work::ledger_seen(&dir.join("ledger.jsonl"))?;
    let made = l.rows_file().is_some_and(Path::exists);
    let open = if made { cfg.job(layer, 0, 1)?.open(&seen)? } else { 1 };
    let v = json!({"job":layer,"name":l.name,"open_items":open,"made":made,"closed_or_asked":seen.len(),
        "licence":l.licence,"licence_class":l.class(),"shipped":l.shipped,"unix":now,
        "note":if made { "each open item is one teacher question at most" } else { "not made yet: the first step makes the draft natively" }});
    atomic(&dir.join("backlog.json"), &v)?;
    Ok(v)
}

/// What one member needs to work.
pub struct Member<'a> {
    pub name: String,
    pub config: &'a Config,
    pub teachers: &'a teacher::Config,
    pub profile: String,
    pub state: PathBuf,
    pub assignment: PathBuf,
    pub report: PathBuf,
    pub ledgers: PathBuf,
    pub min_gap_secs: u64,
}

impl Member<'_> {
    fn report(&self, lease: Option<&Assignment>, state: &str, extra: Value, now: u64) -> Result<()> {
        let mut v = json!({"member":self.name,"lease_id":lease.and_then(|a| a.lease_id.clone()),"job":lease.and_then(|a| a.job.clone()),
            "part":lease.map(|a| a.part),"state":state,"unix":now});
        if let (Some(o), Value::Object(m)) = (v.as_object_mut(), extra) {
            o.extend(m);
        }
        atomic(&self.report, &v)
    }

    /// One step: read the assignment, work one item of it, report.
    pub fn step(&self, term: &dyn PrivateTerminal, now: u64) -> Result<Value> {
        let a = match current(&self.assignment, &self.name, now) {
            Ok(a) => a,
            Err(e) => {
                self.report(None, "idle", json!({"reason":format!("{e:#}")}), now)?;
                return Ok(json!({"event":"idle","reason":format!("unreadable assignment: {e:#}")}));
            }
        };
        let Some(a) = a else {
            self.report(None, "idle", json!({}), now)?;
            return Ok(json!({"event":"idle","reason":"no assignment"}));
        };
        let Some(layer) = a.params["layer"].as_str().or(a.job.as_deref()).map(String::from) else {
            self.report(Some(&a), "refused", json!({"reason":"no layer"}), now)?;
            return Ok(json!({"event":"refused","reason":"the assignment names no layer"}));
        };
        let Some(l) = self.config.layers.get(&layer) else {
            self.report(Some(&a), "refused", json!({"reason":format!("layer {layer} is not configured here")}), now)?;
            return Ok(json!({"event":"refused","reason":format!("layer {layer} is not configured")}));
        };
        if l.source != "layer" && !l.rows_file().is_some_and(Path::exists) {
            // the draft first, natively (no teacher)
            let v = animal::make(l)?;
            backlog(self.config, &layer, &self.ledgers, now)?;
            self.report(Some(&a), "working", json!({"made":v["out"]}), now)?;
            return Ok(json!({"event":"made","layer":layer,"result":v}));
        }
        let job = match self.config.job(&layer, a.part, a.parts) {
            Ok(j) => j,
            Err(e) => {
                self.report(Some(&a), "refused", json!({"reason":format!("{e:#}")}), now)?;
                return Ok(json!({"event":"refused","reason":format!("{e:#}")}));
            }
        };
        let dir = job_dir(&self.ledgers, &layer);
        let lp = Loop {
            state: self.state.clone(),
            teachers: self.teachers,
            profile: self.profile.clone(),
            min_gap_secs: self.min_gap_secs,
            frozen: l.frozen(),
            ledger: Some(dir.join("ledger.jsonl")),
            member: self.name.clone(),
        };
        let mut v = lp.step(job.as_ref(), term, now)?;
        let seen = work::ledger_seen(&dir.join("ledger.jsonl"))?;
        let open = job.open(&seen)?;
        backlog(self.config, &layer, &self.ledgers, now)?;
        let state = match v["event"].as_str() {
            Some("idle") => "finished",
            Some("stopped") => "stopped",
            _ => "working",
        };
        self.report(Some(&a), state, json!({"open_items":open}), now)?;
        v["lease_id"] = json!(a.lease_id);
        v["layer"] = json!(layer);
        Ok(v)
    }
}

/// Run the member: a step every `every_secs` (four times that while idle or waiting), at most `steps` steps. A `STOP`
/// file in the state folder ends this run.
pub fn run(m: &Member, term: &dyn PrivateTerminal, every_secs: u64, steps: Option<u64>) -> Result<Value> {
    let now = || std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
    let mut n = 0u64;
    loop {
        if m.state.join("STOP").exists() {
            break;
        }
        let v = m.step(term, now())?;
        println!("{v}");
        n += 1;
        if steps.is_some_and(|k| n >= k) {
            break;
        }
        let pause = if matches!(v["event"].as_str(), Some("idle" | "waiting" | "stopped" | "refused")) { every_secs * 4 } else { every_secs };
        std::thread::sleep(std::time::Duration::from_secs(pause.max(1)));
    }
    work::summary(&m.state)
}

/// Seed a job's ledger from an instance's own journal (`work.jsonl`), once: its closing lines for the layer's keys.
/// Lets a layer worked before the pool join it without asking its closed items again. Refused when the ledger exists.
pub fn seed(state: &Path, ledgers: &Path, layer: &str, member: &str) -> Result<usize> {
    let ledger = job_dir(ledgers, layer).join("ledger.jsonl");
    ensure!(!ledger.exists(), "{} exists; a ledger is seeded once", ledger.display());
    let text = std::fs::read_to_string(state.join("work.jsonl")).unwrap_or_default();
    let p = format!("{layer}:");
    let mut out = String::new();
    for l in text.lines().filter_map(|l| serde_json::from_str::<Value>(l).ok()) {
        if matches!(l["event"].as_str(), Some("native" | "held" | "proposed" | "rejected")) && l["key"].as_str().is_some_and(|k| k.starts_with(&p)) {
            let mut l = l;
            l["member"] = json!(member);
            l["seeded"] = json!(true);
            out.push_str(&format!("{l}\n"));
        }
    }
    std::fs::create_dir_all(job_dir(ledgers, layer))?;
    std::fs::write(&ledger, &out)?;
    Ok(out.lines().count())
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::teacher::{Profile, TerminalError};
    use std::cell::RefCell;

    struct Term {
        id: String,
        asked: RefCell<Vec<String>>,
    }
    impl PrivateTerminal for Term {
        fn id(&self) -> &str {
            &self.id
        }
        fn ask(&self, q: &str) -> std::result::Result<String, TerminalError> {
            self.asked.borrow_mut().push(q.into());
            // a sense for every word of the question
            Ok(q.lines().filter_map(|l| l.strip_prefix("word: ")).map(|w| format!("{w}\t-\tToday it means something else; here it means the old thing.\n")).collect())
        }
    }
    fn dir() -> PathBuf {
        let d = std::env::temp_dir().join(format!("nautilus-pool-{}-{}", std::process::id(), std::time::SystemTime::now().duration_since(std::time::UNIX_EPOCH).unwrap().as_nanos()));
        std::fs::create_dir_all(&d).unwrap();
        d
    }
    fn teachers(d: &Path, n: &str) -> teacher::Config {
        let p = Profile { name: "sol".into(), provider: "codex".into(), model: "gpt-6.1-sol".into(), terminal: n.into(), terminal_profile: d.join(format!("{n}.json")), purposes: vec!["dictionary".into()] };
        teacher::Config { journal: d.join(format!("{n}/teacher.jsonl")), profiles: vec![p], operator_word: Some("test".into()) }
    }
    fn assign(path: &Path, member: &str, lease: &str, expires: u64) {
        atomic(path, &json!({"schema":1,"pool":"p","member":member,"job":"old","part":0,"parts":1,"lease_id":lease,"expires_unix":expires,"params":{"layer":"old"}})).unwrap();
    }
    fn setup() -> (PathBuf, Config, Vec<u8>) {
        let d = dir();
        std::fs::create_dir_all(d.join("layers")).unwrap();
        // 25 weak emode rows (an empty difference), each with two examples that use the word
        let rows: String = (0..25u8).map(|i| { let w = format!("wd{}", (b'a' + i) as char); format!("{w}\tarchaic sense\tstay\tverb\tTo remain.\t\tAbide {w}. «Torah» | They {w} {i}. «Torah»\t{w}\trate\n") }).collect();
        let text = format!("# The Early Modern English dictionary's records\n{rows}");
        std::fs::write(d.join("layers/emode-senses.tsv"), &text).unwrap();
        std::fs::write(d.join("base.tsv"), "").unwrap();
        let c: Config = serde_json::from_value(json!({"layers":{"old":{"source":"layer","file":d.join("layers/emode-senses.tsv"),"format":"emode-senses",
            "base":d.join("base.tsv"),"licence":"ours","licence_class":"ours","shipped":true,"name":"the Early Modern English dictionary"}},"batch":10})).unwrap();
        (d, c, text.into_bytes())
    }
    #[test]
    fn a_job_moves_between_members_without_asking_again() {
        let (d, c, before) = setup();
        let (ta, tb) = (teachers(&d, "nautilus-a"), teachers(&d, "nautilus-b"));
        let member = |name: &str, t: &'static teacher::Config| Member {
            name: name.into(), config: &c, teachers: t, profile: "sol".into(), state: d.join(format!("{name}/work")),
            assignment: d.join(format!("{name}/assignment.json")), report: d.join(format!("{name}/report.json")), ledgers: d.join("jobs"), min_gap_secs: 0,
        };
        let (ta, tb): (&'static teacher::Config, &'static teacher::Config) = (Box::leak(Box::new(ta)), Box::leak(Box::new(tb)));
        let (a, b) = (member("nautilus-a", ta), member("nautilus-b", tb));
        let terma = Term { id: "nautilus-a".into(), asked: RefCell::new(vec![]) };
        let termb = Term { id: "nautilus-b".into(), asked: RefCell::new(vec![]) };
        // no assignment: idle, nothing asked
        assert_eq!(a.step(&terma, 1).unwrap()["event"], "idle");
        // A holds the lease: two questions (20 words)
        assign(&a.assignment, "nautilus-a", "p-1", 100);
        assert_eq!(a.step(&terma, 10).unwrap()["event"], "proposed");
        assert_eq!(a.step(&terma, 11).unwrap()["event"], "proposed");
        let r: Value = serde_json::from_slice(&std::fs::read(&a.report).unwrap()).unwrap();
        assert_eq!((r["state"].as_str(), r["open_items"].as_u64(), r["lease_id"].as_str()), (Some("working"), Some(1), Some("p-1")));
        // the lease expires: A stops working the job
        assert_eq!(a.step(&terma, 100).unwrap()["event"], "idle");
        // the job moves to B: B asks only the last five words, never A's questions
        assign(&b.assignment, "nautilus-b", "p-2", 300);
        let v = b.step(&termb, 120).unwrap();
        assert_eq!(v["event"], "proposed", "{v}");
        let asked_b = termb.asked.borrow().join("\n");
        assert!(asked_b.contains("word: wdu") && asked_b.contains("word: wdy") && !asked_b.contains("word: wda") && !asked_b.contains("word: wdt"), "{asked_b}");
        assert_eq!(b.step(&termb, 121).unwrap()["event"], "idle");
        let r: Value = serde_json::from_slice(&std::fs::read(&b.report).unwrap()).unwrap();
        assert_eq!(r["state"], "finished");
        assert_eq!(terma.asked.borrow().len() + termb.asked.borrow().len(), 3, "three questions in all, none repeated");
        // the layer file is never written; the backlog says nothing is left
        assert_eq!(std::fs::read(c.layers["old"].file.as_ref().unwrap()).unwrap(), before);
        let bl: Value = serde_json::from_slice(&std::fs::read(d.join("jobs/old/backlog.json")).unwrap()).unwrap();
        assert_eq!((bl["open_items"].as_u64(), bl["licence_class"].as_str()), (Some(0), Some("ours")));
        // proposals are in the builder's format: nine columns, kind/examples/forms/tests kept
        let props: Vec<PathBuf> = std::fs::read_dir(d.join("nautilus-a/work/proposals")).unwrap().flatten().map(|e| e.path()).collect();
        let p = std::fs::read_to_string(&props[0]).unwrap();
        assert!(p.lines().all(|l| l.split('\t').count() == 9 && l.contains("\tarchaic sense\tstay\tverb\tTo remain.\tToday it means")), "{p}");
    }
    #[test]
    fn without_the_ledger_a_moved_job_asks_again() {
        // red control: the same move with each member's own journal only repeats A's questions
        let (d, c, _) = setup();
        let (ta, tb) = (teachers(&d, "nautilus-a"), teachers(&d, "nautilus-b"));
        let terma = Term { id: "nautilus-a".into(), asked: RefCell::new(vec![]) };
        let termb = Term { id: "nautilus-b".into(), asked: RefCell::new(vec![]) };
        let job = c.job("old", 0, 1).unwrap();
        let la = Loop { state: d.join("a"), teachers: &ta, profile: "sol".into(), min_gap_secs: 0, frozen: vec![], ledger: None, member: "a".into() };
        let lb = Loop { state: d.join("b"), teachers: &tb, profile: "sol".into(), min_gap_secs: 0, frozen: vec![], ledger: None, member: "b".into() };
        la.step(job.as_ref(), &terma, 1).unwrap();
        lb.step(job.as_ref(), &termb, 2).unwrap();
        assert_eq!(terma.asked.borrow()[0], termb.asked.borrow()[0], "without a ledger the moved job repeats the question");
    }
    #[test]
    fn expired_foreign_and_released_assignments_are_no_work() {
        let d = dir();
        let p = d.join("a.json");
        assert!(current(&p, "m", 1).unwrap().is_none());
        assign(&p, "m", "l-1", 50);
        assert!(current(&p, "m", 49).unwrap().is_some());
        assert!(current(&p, "m", 50).unwrap().is_none(), "expired at expires_unix");
        assert!(current(&p, "other", 10).is_err(), "another member's lease");
        atomic(&p, &json!({"schema":1,"job":null,"released":"l-1"})).unwrap();
        assert!(current(&p, "m", 10).unwrap().is_none());
    }
    #[test]
    fn a_member_never_works_inside_the_layer_folder() {
        let (d, c, before) = setup();
        let t = teachers(&d, "nautilus-a");
        let m = Member { name: "nautilus-a".into(), config: &c, teachers: &t, profile: "sol".into(), state: d.join("layers/work"),
            assignment: d.join("a.json"), report: d.join("r.json"), ledgers: d.join("jobs"), min_gap_secs: 0 };
        assign(&m.assignment, "nautilus-a", "p-1", 100);
        let term = Term { id: "nautilus-a".into(), asked: RefCell::new(vec![]) };
        assert!(m.step(&term, 1).is_err(), "a state folder inside the improved layer's folder is refused");
        assert!(term.asked.borrow().is_empty());
        assert_eq!(std::fs::read(c.layers["old"].file.as_ref().unwrap()).unwrap(), before);
    }
    #[test]
    fn a_shipped_layer_refuses_cc_by_sa_input() {
        let (d, c, _) = setup();
        std::fs::write(c.layers["old"].file.as_ref().unwrap(), "# License: CC BY-SA 4.0\nw\tarchaic sense\t\tverb\t\t\tx «y»\tw\trate\n").unwrap();
        let e = c.job("old", 0, 1).err().unwrap().to_string();
        assert!(e.contains("never takes CC BY-SA"), "{e}");
        let mut c2 = c.clone();
        c2.layers.get_mut("old").unwrap().shipped = false;
        assert!(c2.job("old", 0, 1).is_ok(), "an internal layer may read it");
        let _ = d;
    }
}
