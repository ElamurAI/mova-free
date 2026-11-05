//! Crab's durable queue of consultations for the 🐬 Dolphin consultant.
//!
//! The Octopus asks its consultant through Crab in English; Crab may also consult it itself
//! when a configured trigger fires (a request it refused or found ambiguous, a worker crash loop,
//! a failed GPU check). Every consultation waits here until the consultant terminal is idle and no
//! earlier consultation is pending; then Crab's existing consult protocol delivers it, which
//! never overwrites an active task. Urgent entries go first, otherwise arrival order; priority
//! reorders waiting entries only.
//!
//! The consultant's answer to a Crab-initiated consultation becomes an advice record attached to
//! its trigger (and, for a refused request, to the command school's review candidates). Advice
//! is evidence for the Octopus and teaching material. Crab never executes it, never turns it into
//! a request, and it never replaces Mova's own understanding or any assessment answer.
use crate::Result;
use serde::Deserialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, DirBuilder, OpenOptions},
    io::Write,
    os::unix::fs::{DirBuilderExt, MetadataExt, OpenOptionsExt},
    path::{Path, PathBuf},
};

/// Same bound as Crab's `consult_opus` task.
pub const TASK_MAX: usize = 12000;
const WAITING_MAX: usize = 64;
const ENTRIES_MAX: usize = 4096;
const ENTRY_MAX: usize = 64 * 1024;

fn error(s: impl Into<String>) -> Box<dyn std::error::Error> {
    s.into().into()
}
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn default_per_hour() -> u32 {
    2
}
fn default_task_bytes() -> usize {
    4000
}
/// Crab-initiated consultations. Absent from the configuration means all off.
#[derive(Clone, Debug, Default, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Triggers {
    /// A prompt Crab refused or found ambiguous.
    #[serde(default)]
    pub refusals: bool,
    /// A worker in a crash loop (three consecutive failures) or failed, or a failed GPU check.
    #[serde(default)]
    pub worker_failures: bool,
    /// Crab-initiated consultations admitted per rolling hour (1..12).
    #[serde(default = "default_per_hour")]
    pub max_per_hour: u32,
    /// Bound on a composed task in UTF-8 bytes (256..12000).
    #[serde(default = "default_task_bytes")]
    pub max_task_bytes: usize,
    /// The named Dolphin Crab consults; required when a trigger is on.
    #[serde(default)]
    pub dolphin: Option<String>,
}
impl Triggers {
    pub fn validate(&self) -> Result<()> {
        if !(1..=12).contains(&self.max_per_hour) || !(256..=TASK_MAX).contains(&self.max_task_bytes) {
            return Err(error("consult_triggers: max_per_hour 1..12 and max_task_bytes 256..12000"));
        }
        Ok(())
    }
}

/// Plain text a consultant can read: non-empty, bounded, no control characters but \n and \t.
pub fn valid_task(t: &str) -> bool {
    !t.trim().is_empty()
        && t.len() <= TASK_MAX
        && !t.chars().any(|x| x.is_control() && !matches!(x, '\n' | '\t'))
}
/// Cut at a character boundary and mark the cut.
fn bounded(s: &str, max: usize) -> String {
    if s.len() <= max {
        return s.to_string();
    }
    let mut end = max.saturating_sub(16);
    while !s.is_char_boundary(end) {
        end -= 1;
    }
    format!("{} [truncated]", &s[..end])
}
fn clean(s: &str) -> String {
    s.chars()
        .map(|c| if c.is_control() && c != '\n' && c != '\t' { ' ' } else { c })
        .collect()
}

pub struct Queue {
    dir: PathBuf,
    advice: PathBuf,
}
fn private(p: &Path) -> Result<()> {
    if !p.exists() {
        DirBuilder::new().recursive(true).mode(0o700).create(p)?;
    }
    let m = fs::symlink_metadata(p)?;
    if !m.is_dir() || m.uid() != unsafe { libc::geteuid() } || m.mode() & 0o077 != 0 {
        return Err(error("consult queue directories must be owned and private (0700)"));
    }
    Ok(())
}
fn write_new(p: &Path, v: &Value) -> Result<()> {
    let mut f = OpenOptions::new().create_new(true).write(true).mode(0o600).open(p)?;
    f.write_all(&serde_json::to_vec(v)?)?;
    f.sync_all()?;
    Ok(())
}
fn replace(p: &Path, v: &Value) -> Result<()> {
    let tmp = p.with_extension("tmp");
    let mut f = OpenOptions::new().create(true).truncate(true).write(true).mode(0o600).open(&tmp)?;
    f.write_all(&serde_json::to_vec(v)?)?;
    f.sync_all()?;
    fs::rename(tmp, p)?;
    Ok(())
}
impl Queue {
    /// `run_root/consult-queue` and `run_root/consult-advice`, both private.
    pub fn open(run_root: &Path) -> Result<Queue> {
        let q = Queue { dir: run_root.join("consult-queue"), advice: run_root.join("consult-advice") };
        private(&q.dir)?;
        private(&q.advice)?;
        Ok(q)
    }
    fn path(&self, id: &str) -> PathBuf {
        self.dir.join(format!("{id}.json"))
    }
    /// Every entry, in id order (ids begin with the creation time).
    pub fn entries(&self) -> Result<Vec<Value>> {
        let mut names: Vec<PathBuf> = Vec::new();
        for e in fs::read_dir(&self.dir)? {
            let p = e?.path();
            if p.extension().and_then(|x| x.to_str()) == Some("json") {
                names.push(p);
            }
            if names.len() > ENTRIES_MAX {
                return Err(error("consult queue observation limit reached; archive deliberately"));
            }
        }
        names.sort();
        names
            .iter()
            .map(|p| {
                let b = fs::read(p)?;
                if b.len() > ENTRY_MAX {
                    return Err(error("consult queue entry exceeds its bound"));
                }
                Ok(serde_json::from_slice(&b)?)
            })
            .collect()
    }
    /// Waiting entries in delivery order: urgent first, then arrival.
    pub fn waiting(&self) -> Result<Vec<Value>> {
        let mut w: Vec<Value> = self.entries()?.into_iter().filter(|e| e["state"] == "waiting").collect();
        w.sort_by_key(|e| (e["priority"] != "urgent", e["id"].as_str().unwrap_or("").to_string()));
        Ok(w)
    }
    /// Waiting entries for one named Dolphin, in delivery order.
    pub fn waiting_for(&self, dolphin: &str) -> Result<Vec<Value>> {
        Ok(self.waiting()?.into_iter().filter(|e| e["dolphin"] == dolphin).collect())
    }
    /// Add one consultation for a named Dolphin. `origin` is `octopus` or `crab`; Crab's own
    /// entries carry a trigger.
    pub fn enqueue(&self, dolphin: &str, task: &str, urgent: bool, origin: &str, trigger: Option<Value>, now: u64) -> Result<Value> {
        if !valid_task(task) {
            return Err(error("a consultation needs bounded plain text (at most 12000 bytes)"));
        }
        if !matches!(origin, "octopus" | "crab") || (origin == "crab") != trigger.is_some() {
            return Err(error("origin is octopus, or crab with its trigger"));
        }
        let all = self.waiting()?;
        if all.len() >= WAITING_MAX {
            return Err(error("64 consultations are already waiting; nothing was queued"));
        }
        let n = self.entries()?.len();
        let id = format!("cq-{now:010}-{n:04}-{}", &hash(task.as_bytes())[..8]);
        let waiting: Vec<&Value> = all.iter().filter(|e| e["dolphin"] == dolphin).collect();
        let entry = json!({"schema":1,"id":id,"dolphin":dolphin,"state":"waiting","priority":if urgent {"urgent"} else {"normal"},
            "origin":origin,"trigger":trigger,"created_unix":now,"task_sha256":hash(task.as_bytes()),"task":task});
        write_new(&self.path(&id), &entry)?;
        let ahead = waiting.iter().filter(|e| urgent <= (e["priority"] == "urgent")).count();
        Ok(json!({"queued":true,"id":id,"dolphin":dolphin,"priority":entry["priority"],"origin":origin,"waiting_ahead":ahead,
            "delivery":"when the consultant is idle and no earlier consultation is pending; an active task is never overwritten"}))
    }
    /// Record a state change of one entry (`sent`, `dispatch_failed`, `answered`).
    pub fn mark(&self, id: &str, state: &str, detail: Value, now: u64) -> Result<()> {
        let p = self.path(id);
        let mut e: Value = serde_json::from_slice(&fs::read(&p)?)?;
        e["state"] = json!(state);
        e[format!("{state}_unix")] = json!(now);
        e[format!("{state}_detail")] = detail;
        replace(&p, &e)
    }
    /// Crab-initiated entries created within the last hour.
    pub fn crab_recent(&self, now: u64) -> Result<usize> {
        Ok(self
            .entries()?
            .iter()
            .filter(|e| e["origin"] == "crab" && e["created_unix"].as_u64().is_some_and(|t| t + 3600 > now))
            .count())
    }
    pub fn has_trigger(&self, key: &str) -> Result<bool> {
        Ok(self.entries()?.iter().any(|e| e["trigger"]["key"] == key))
    }
    /// Admit one Crab-initiated consultation if the trigger is new and the hourly bound allows.
    pub fn trigger(&self, t: &Triggers, key: &str, trigger: Value, task: &str, now: u64) -> Result<Option<Value>> {
        if self.has_trigger(key)? || self.crab_recent(now)? >= t.max_per_hour as usize {
            return Ok(None);
        }
        let dolphin = t.dolphin.as_deref().ok_or("consult_triggers name no Dolphin")?;
        let mut trigger = trigger;
        trigger["key"] = json!(key);
        self.enqueue(dolphin, &bounded(task, t.max_task_bytes), false, "crab", Some(trigger), now).map(Some)
    }
    /// Attach the consultant's answer to a sent Crab-initiated entry: an advice record that is
    /// evidence only. `report` is the report file once it exists.
    pub fn record_advice(&self, entry: &Value, report: &Path, now: u64) -> Result<Value> {
        let id = entry["id"].as_str().ok_or("entry lacks id")?;
        let m = fs::symlink_metadata(report)?;
        if !m.is_file() || m.uid() != unsafe { libc::geteuid() } {
            return Err(error("advice report is not an owned regular file"));
        }
        let bytes = fs::read(report)?;
        let record = json!({"schema":1,"consult_id":id,"origin":entry["origin"],"trigger":entry["trigger"],
            "review_candidates":entry["trigger"]["review_candidates"],"report":{"path":report,"bytes":bytes.len(),"sha256":hash(&bytes)},
            "recorded_unix":now,"standing":"advice from an LLM consultant: evidence for the Octopus and teaching material; not an action, not an assessment answer, not Mova's own understanding",
            "actions_taken":[],"automatic_execution":false});
        let p = self.advice.join(format!("{id}.json"));
        if !p.exists() {
            write_new(&p, &record)?;
        }
        self.mark(id, "answered", json!({"advice_record":p}), now)?;
        Ok(record)
    }
    pub fn advice_records(&self) -> Result<Vec<Value>> {
        let mut out = Vec::new();
        let mut names: Vec<PathBuf> = fs::read_dir(&self.advice)?.filter_map(|e| e.ok().map(|e| e.path())).collect();
        names.sort();
        for p in names.iter().rev().take(8) {
            if p.extension().and_then(|x| x.to_str()) == Some("json") {
                out.push(serde_json::from_slice(&fs::read(p)?)?);
            }
        }
        Ok(out)
    }
}

/// The consultation Crab composes for a request it refused or found ambiguous.
pub fn refusal_task(prompt: &str, invocation_id: &str, failure: &Value, candidates: &Value, max: usize) -> String {
    let text = format!(
        "Crab could not execute an English request and asks for your advice as teaching material.\n\
Request id: {invocation_id}\nRefusal stage: {}\nReason: {}\nReview candidates: {}\n\
Request text (verbatim, bounded):\n<<<\n{}\n>>>\n\
In at most twelve lines: is the refusal right? If the request is a reasonable instruction, what \
would Mova's own reader need to understand it (vocabulary, syntax, pragmatics, a capability)? \
Your advice is evidence for the Octopus; Crab will not execute it.",
        clean(failure["stage"].as_str().unwrap_or("unknown")),
        clean(&bounded(failure["reason"].as_str().unwrap_or(""), 600)),
        clean(&bounded(&candidates.to_string(), 400)),
        clean(&bounded(prompt, max / 2)),
    );
    bounded(&text, max)
}
/// The trigger key and consultation for a worker in trouble, or None when it is fine.
pub fn worker_trigger(view: &Value) -> Option<(String, String)> {
    let name = view["worker"].as_str()?;
    let state = view["state"].as_str()?;
    let failures = view["consecutive_failures"].as_u64().unwrap_or(0);
    if !(state == "failed" || failures >= 3) {
        return None;
    }
    let key = format!("worker:{name}:{}:{failures}:{state}", view["restarts"].as_u64().unwrap_or(0));
    let events = bounded(&view["recent_events"].to_string(), 1500);
    let task = format!(
        "Crab supervises a worker that keeps failing and asks for your read-only diagnosis.\n\
Worker: {name}\nState: {state}\nConsecutive failures: {failures}\nLast exit: {}\nLast error: {}\n\
Recent events: {}\n\
In at most twelve lines: the likely cause, what evidence would confirm it, and what the Octopus \
should check. Do not start, stop or change anything; Crab will not execute your advice.",
        clean(&view["last_exit"].to_string()),
        clean(&bounded(&view["last_error"].to_string(), 400)),
        clean(&events)
    );
    Some((key, task))
}
/// The trigger for a failed GPU check, or None.
pub fn gpu_trigger(record: &Value) -> Option<(String, String)> {
    if record["state"] != "failed" {
        return None;
    }
    let at = record["at_unix"].as_u64().unwrap_or(0);
    let key = format!("gpu:{at}:{}", &hash(record.to_string().as_bytes())[..12]);
    let task = format!(
        "Crab's GPU check failed before a GPU worker could start. Record (bounded): {}\n\
In at most eight lines: the likely cause and what the Octopus should check. Crab will not execute \
your advice.",
        clean(&bounded(&record.to_string(), 1500))
    );
    Some((key, task))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(0);
    struct Fx(PathBuf);
    impl Drop for Fx {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
    fn fx() -> Fx {
        let d = std::env::temp_dir().join(format!("crab-consult-queue-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
        DirBuilder::new().recursive(true).mode(0o700).create(&d).unwrap();
        Fx(d)
    }
    #[test]
    fn urgent_first_then_arrival_and_bounds() {
        let f = fx();
        let q = Queue::open(&f.0).unwrap();
        q.enqueue("opus", "first", false, "octopus", None, 100).unwrap();
        q.enqueue("opus", "second", false, "octopus", None, 101).unwrap();
        let u = q.enqueue("opus", "urgent", true, "octopus", None, 102).unwrap();
        assert_eq!(u["waiting_ahead"], 0);
        let order: Vec<_> = q.waiting().unwrap().iter().map(|e| e["task"].as_str().unwrap().to_string()).collect();
        assert_eq!(order, ["urgent", "first", "second"]);
        q.enqueue("copilot", "other dolphin", true, "octopus", None, 102).unwrap();
        assert_eq!(q.waiting_for("opus").unwrap().len(), 3);
        assert_eq!(q.waiting_for("copilot").unwrap()[0]["task"], "other dolphin");
        let id = q.waiting().unwrap()[0]["id"].as_str().unwrap().to_string();
        q.mark(&id, "sent", json!({}), 103).unwrap();
        assert_eq!(q.waiting_for("opus").unwrap().len(), 2);
        for bad in ["", "   ", "a\u{7}b"] {
            assert!(q.enqueue("opus", bad, false, "octopus", None, 104).is_err());
        }
        assert!(q.enqueue("opus", &"x".repeat(TASK_MAX + 1), false, "octopus", None, 104).is_err());
        assert!(q.enqueue("opus", "t", false, "crab", None, 104).is_err(), "crab entries carry a trigger");
        assert!(q.enqueue("opus", "t", false, "someone", None, 104).is_err());
    }
    #[test]
    fn triggers_are_deduplicated_rate_bounded_and_size_bounded() {
        let f = fx();
        let q = Queue::open(&f.0).unwrap();
        let t = Triggers { refusals: true, worker_failures: true, max_per_hour: 2, max_task_bytes: 300, dolphin: Some("opus".into()) };
        t.validate().unwrap();
        let long = "y".repeat(5000);
        let a = q.trigger(&t, "k1", json!({"kind":"refusal"}), &long, 1000).unwrap().unwrap();
        let e = &q.entries().unwrap()[0];
        assert!(e["task"].as_str().unwrap().len() <= 300 && e["task"].as_str().unwrap().ends_with("[truncated]"));
        assert_eq!(a["origin"], "crab");
        assert!(q.trigger(&t, "k1", json!({}), "again", 1001).unwrap().is_none(), "same trigger once");
        assert!(q.trigger(&t, "k2", json!({}), "two", 1002).unwrap().is_some());
        assert!(q.trigger(&t, "k3", json!({}), "three", 1003).unwrap().is_none(), "hourly bound");
        assert!(q.trigger(&t, "k3", json!({}), "three", 1000 + 3601).unwrap().is_some(), "after an hour");
        assert!(Triggers { max_per_hour: 13, ..t.clone() }.validate().is_err());
        assert!(Triggers { max_task_bytes: 12001, ..t }.validate().is_err());
        assert_eq!(Triggers::default().refusals, false);
    }
    #[test]
    fn composed_tasks_are_bounded_plain_text() {
        let t = refusal_task("Kill \u{1b}[31m Eel.", "inv-1", &json!({"stage":"unsupported_capability","reason":"no kill"}), &json!(["cand-1"]), 2000);
        assert!(valid_task(&t) && t.contains("inv-1") && t.contains("cand-1") && t.contains("Crab will not execute"));
        let ok = json!({"worker":"eel","state":"running","consecutive_failures":0});
        assert!(worker_trigger(&ok).is_none());
        let bad = json!({"worker":"eel","state":"backoff","consecutive_failures":3,"restarts":5,"recent_events":[{"event":"exit"}]});
        let (k, task) = worker_trigger(&bad).unwrap();
        assert_eq!(k, "worker:eel:5:3:backoff");
        assert!(valid_task(&task));
        assert!(gpu_trigger(&json!({"state":"passed"})).is_none());
        assert!(gpu_trigger(&json!({"state":"failed","at_unix":5})).is_some());
    }
}
