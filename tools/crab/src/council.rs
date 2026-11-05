//! Crab's council: an event-driven, collegial review of Crab's supervision by the configured
//! consultant Dolphins (for example Opus and Sol).
//!
//! Every `check_seconds` Crab composes one compact English digest — flow probe values and
//! conditions, recent flow actions, capacity per animal kind (backlog, throughput, time to drain,
//! starved time), worker states and requests it did not understand — and fingerprints its stable
//! facts (no times, round ids, moving counts or ages). A round starts only when the fingerprint
//! changed since the last round (and `min_spacing_seconds` passed) or when `period_seconds`, the
//! longest interval, passed; otherwise the evaluation is a compact skip. A round queues the same
//! digest to each idle member through the ordinary consult queue when at least `quorum` members
//! are idle (an active Dolphin task is never overwritten; busy or stopped members are recorded as
//! absent). A member silent after `member_timeout_seconds` is recorded as timed out and the round
//! closes with the others. The Dolphins answer independently: each sees only the digest.
//!
//! Answers are evidence, never executable text. Crab parses only the first non-empty line of each
//! answer as a typed verdict:
//!   VERDICT: none
//!   VERDICT: octopus
//!   VERDICT: action:<start_producer|pause_producer|resume_producer|hold|restart>:<probe>
//!   VERDICT: capacity:<kind>:<more|fewer>:<n>
//! followed by an optional reason. Combination is deterministic, over the members that answered
//! (fewer answers than the quorum: escalated as expired):
//!   all none                         -> journal only
//!   all the same allowed action      -> Crab may execute it (flow bounds still apply)
//!   anything else (disagreement, octopus, capacity, malformed, refused action, expired round)
//!                                    -> an attention item for the Octopus.
//! A capacity verdict is never executed: it always becomes an attention item with both reasons
//! and the numbers; the Octopus decides and tells the operator.
use crate::flow::{self, ACTIONS};
use crate::Result;
use serde::Deserialize;
use serde_json::{json, Value};
use sha2::{Digest, Sha256};
use std::{
    fs::{self, File, OpenOptions},
    io::{Read, Seek, SeekFrom, Write},
    os::unix::fs::{DirBuilderExt, OpenOptionsExt},
    path::{Path, PathBuf},
};

const REASON_MAX: usize = 600;
const ATTENTION_SHOWN: usize = 16;
fn error(s: impl Into<String>) -> Box<dyn std::error::Error> {
    s.into().into()
}
fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn d_period() -> u64 {
    1800
}
fn d_task() -> usize {
    6000
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct CouncilConfig {
    /// Configured Dolphin names, 2..4 (for example ["opus", "sol"]).
    pub dolphins: Vec<String>,
    /// The longest interval between rounds: a round is held after this long even when the facts
    /// did not change (600..86400).
    #[serde(default = "d_period")]
    pub period_seconds: u64,
    #[serde(default = "d_task")]
    pub max_task_bytes: usize,
    /// Compatibility: an upper bound of member_timeout_seconds.
    #[serde(default)]
    pub answer_timeout_seconds: Option<u64>,
    /// The shortest interval between rounds when the facts change (default 300, 60..period).
    #[serde(default)]
    pub min_spacing_seconds: Option<u64>,
    /// How often the digest facts are composed and fingerprinted (default 60, 10..600).
    #[serde(default)]
    pub check_seconds: Option<u64>,
    /// The fewest idle members a round needs and the fewest answers it combines (default 2,
    /// 2..the number of members).
    #[serde(default)]
    pub quorum: Option<usize>,
    /// A member that has not answered after this long is recorded as timed out and the round
    /// closes with the others (default 600, 60..86400).
    #[serde(default)]
    pub member_timeout_seconds: Option<u64>,
}
impl CouncilConfig {
    pub fn validate(&self, dolphins: &[String]) -> Result<()> {
        let mut seen = std::collections::BTreeSet::new();
        if self.dolphins.len() < 2 || self.dolphins.len() > 4 || !self.dolphins.iter().all(|d| dolphins.contains(d) && seen.insert(d)) {
            return Err(error("council names 2..4 distinct configured Dolphins"));
        }
        if !(600..=86400).contains(&self.period_seconds) || !(1000..=crate::consult_queue::TASK_MAX).contains(&self.max_task_bytes) {
            return Err(error("council period_seconds 600..86400 and max_task_bytes 1000..12000"));
        }
        if self.answer_timeout_seconds.is_some_and(|t| !(600..=7 * 86400).contains(&t)) {
            return Err(error("council answer_timeout_seconds 600..604800"));
        }
        if !(60..=self.period_seconds).contains(&self.min_spacing()) || !(10..=600).contains(&self.check()) {
            return Err(error("council min_spacing_seconds 60..period_seconds and check_seconds 10..600"));
        }
        if !(2..=self.dolphins.len()).contains(&self.quorum()) {
            return Err(error("council quorum 2..the number of members"));
        }
        if self.member_timeout_seconds.is_some_and(|t| !(60..=86400).contains(&t)) {
            return Err(error("council member_timeout_seconds 60..86400"));
        }
        Ok(())
    }
    pub fn min_spacing(&self) -> u64 {
        self.min_spacing_seconds.unwrap_or(300)
    }
    pub fn check(&self) -> u64 {
        self.check_seconds.unwrap_or(60)
    }
    pub fn quorum(&self) -> usize {
        self.quorum.unwrap_or(2)
    }
    /// The member timeout (bounded above by answer_timeout_seconds when that is set).
    pub fn timeout(&self) -> u64 {
        let t = self.member_timeout_seconds.unwrap_or(600);
        self.answer_timeout_seconds.map_or(t, |a| t.min(a))
    }
}

/// One parsed verdict.
#[derive(Clone, Debug, PartialEq)]
pub enum Verdict {
    None,
    Octopus,
    Action { action: String, probe: String },
    Capacity { kind: String, direction: String, n: u32 },
    Malformed(String),
}
impl Verdict {
    pub fn label(&self) -> String {
        match self {
            Verdict::None => "none".into(),
            Verdict::Octopus => "octopus".into(),
            Verdict::Action { action, probe } => format!("action:{action}:{probe}"),
            Verdict::Capacity { kind, direction, n } => format!("capacity:{kind}:{direction}:{n}"),
            Verdict::Malformed(why) => format!("malformed ({why})"),
        }
    }
}
fn clean(s: &str) -> String {
    s.chars().map(|c| if c.is_control() { ' ' } else { c }).collect()
}
fn bounded(s: &str, max: usize) -> String {
    if s.len() <= max {
        return s.to_string();
    }
    let mut end = max;
    while !s.is_char_boundary(end) {
        end -= 1;
    }
    format!("{}…", &s[..end])
}
/// Parse the first non-empty line of an answer; the rest of the text is the reason (bounded,
/// kept as evidence). Unknown probes, kinds, actions and numbers are malformed.
pub fn parse_verdict(text: &str, probes: &[String], kinds: &[String]) -> (Verdict, String) {
    let mut lines = text.lines().map(str::trim).skip_while(|l| l.is_empty());
    let Some(first) = lines.next() else {
        return (Verdict::Malformed("empty answer".into()), String::new());
    };
    let rest: Vec<&str> = lines.collect();
    let Some(body) = first.strip_prefix("VERDICT:") else {
        return (Verdict::Malformed("the first line is not VERDICT:".into()), bounded(&clean(text), REASON_MAX));
    };
    let body = body.trim();
    let (token, inline) = body.split_once(char::is_whitespace).unwrap_or((body, ""));
    let inline = inline.trim().trim_start_matches(['-', '—', ':']).trim();
    let reason = bounded(&clean(&[inline].into_iter().chain(rest.iter().copied()).filter(|s| !s.is_empty()).collect::<Vec<_>>().join(" ")), REASON_MAX);
    let parts: Vec<&str> = token.split(':').collect();
    let v = match parts.as_slice() {
        ["none"] => Verdict::None,
        ["octopus"] => Verdict::Octopus,
        ["action", a, p] if ACTIONS.contains(a) && probes.iter().any(|x| x == p) => Verdict::Action { action: a.to_string(), probe: p.to_string() },
        ["action", ..] => Verdict::Malformed(format!("unknown action or probe in {token:?}")),
        ["capacity", k, d, n] if kinds.iter().any(|x| x == k) && matches!(*d, "more" | "fewer") => match n.parse::<u32>() {
            Ok(n) if (1..=64).contains(&n) => Verdict::Capacity { kind: k.to_string(), direction: d.to_string(), n },
            _ => Verdict::Malformed(format!("capacity count must be 1..64 in {token:?}")),
        },
        ["capacity", ..] => Verdict::Malformed(format!("unknown kind or direction in {token:?}")),
        _ => Verdict::Malformed(format!("unknown verdict {:?}", bounded(token, 80))),
    };
    (v, reason)
}
/// The collegial decision.
#[derive(Clone, Debug, PartialEq)]
pub enum Outcome {
    JournalOnly,
    Execute { action: String, probe: String },
    Escalate(String),
}
pub fn combine(verdicts: &[Verdict]) -> Outcome {
    if verdicts.len() < 2 {
        return Outcome::Escalate("fewer than two council answers".into());
    }
    if let Some(m) = verdicts.iter().find_map(|v| if let Verdict::Malformed(w) = v { Some(w) } else { None }) {
        return Outcome::Escalate(format!("a malformed verdict: {m}"));
    }
    if verdicts.iter().any(|v| matches!(v, Verdict::Capacity { .. })) {
        return Outcome::Escalate("a capacity verdict: the Octopus decides and tells the operator".into());
    }
    if verdicts.contains(&Verdict::Octopus) {
        return Outcome::Escalate("a Dolphin asks for the Octopus".into());
    }
    if verdicts.iter().all(|v| *v == Verdict::None) {
        return Outcome::JournalOnly;
    }
    match &verdicts[0] {
        Verdict::Action { action, probe } if verdicts.iter().all(|v| *v == verdicts[0]) => Outcome::Execute { action: action.clone(), probe: probe.clone() },
        _ => Outcome::Escalate("the Dolphins disagree".into()),
    }
}

/// Replace every run of ASCII digits by `#`: free text whose only change is a count, an age or
/// a time keeps its fingerprint.
pub fn mask_digits(s: &str) -> String {
    let mut out = String::with_capacity(s.len());
    let mut in_run = false;
    for c in s.chars() {
        if c.is_ascii_digit() {
            if !in_run {
                out.push('#');
            }
            in_run = true;
        } else {
            out.push(c);
            in_run = false;
        }
    }
    out
}
/// The fingerprint of an attention item: its source and its facts (canonical JSON: object keys
/// are ordered), never its time, id or round.
pub fn fingerprint(source: &str, facts: &Value) -> String {
    hash(json!({"source":source,"facts":facts}).to_string().as_bytes())
}
const ATTENTION_KEEP: usize = 512;
/// The Octopus's attention queue: `attention.jsonl` (append-only: only new fingerprints),
/// `state.json` (every fingerprint with its count, first and last time, newest summary) and
/// `attention.md` (the sixteen most recently seen items in English, with repeat counts).
pub struct Attention {
    dir: PathBuf,
    lock: std::sync::Mutex<()>,
}
impl Attention {
    pub fn open(run_root: &Path) -> Result<Attention> {
        let dir = run_root.join("attention");
        if !dir.exists() {
            fs::DirBuilder::new().recursive(true).mode(0o700).create(&dir)?;
        }
        Ok(Attention { dir, lock: std::sync::Mutex::new(()) })
    }
    /// The merged items, keyed by fingerprint. A queue from before fingerprints is seeded from
    /// its newest lines (each its own fingerprint).
    fn state(&self) -> serde_json::Map<String, Value> {
        let p = self.dir.join("state.json");
        if let Some(Value::Object(m)) = fs::read(&p).ok().filter(|b| b.len() <= 8 << 20).and_then(|b| serde_json::from_slice::<Value>(&b).ok()).map(|v| v["items"].clone()) {
            return m;
        }
        self.recent(ATTENTION_SHOWN)
            .into_iter()
            .map(|i| {
                let fp = i["fingerprint"].as_str().map(String::from).unwrap_or_else(|| format!("legacy:{}", i["id"].as_str().unwrap_or("")));
                let at = i["at_unix"].clone();
                (fp.clone(), json!({"id":i["id"],"fingerprint":fp,"source":i["source"],"summary":i["summary"],"first_unix":at,"last_unix":at,"count":1}))
            })
            .collect()
    }
    /// The merged items, most recently seen first (ties by id).
    pub fn merged(&self, n: usize) -> Vec<Value> {
        let mut v: Vec<Value> = self.state().into_values().collect();
        v.sort_by(|a, b| b["last_unix"].as_u64().cmp(&a["last_unix"].as_u64()).then_with(|| a["id"].as_str().cmp(&b["id"].as_str())));
        v.truncate(n);
        v
    }
    /// Add an item whose facts are its source and summary (an exact repeat merges).
    pub fn add(&self, now: u64, source: &str, summary: &str, evidence: Value) -> Result<Value> {
        let facts = json!({"summary":summary});
        self.add_facts(now, source, summary, evidence, &facts)
    }
    /// Add an item fingerprinted by `facts`. A known fingerprint is merged (count and last time
    /// in `state.json`, the newest summary kept); only a new fingerprint is appended.
    pub fn add_facts(&self, now: u64, source: &str, summary: &str, evidence: Value, facts: &Value) -> Result<Value> {
        let _g = self.lock.lock().map_err(|_| error("attention poisoned"))?;
        let fp = fingerprint(source, facts);
        let summary = bounded(&clean(summary), 1200);
        let mut st = self.state();
        let item = if let Some(e) = st.get_mut(&fp) {
            e["count"] = json!(e["count"].as_u64().unwrap_or(1) + 1);
            e["last_unix"] = json!(now);
            e["summary"] = json!(summary);
            let mut v = e.clone();
            v["merged"] = json!(true);
            v
        } else {
            let item = json!({"schema":1,"id":format!("at-{now:010}-{}",&fp[..8]),"fingerprint":fp,"facts":facts,
                "at_unix":now,"source":source,"summary":summary,"evidence":evidence,"actions_taken":[]});
            flow::append(&self.dir.join("attention.jsonl"), &item)?;
            st.insert(fp.clone(), json!({"id":item["id"],"fingerprint":fp,"source":source,"summary":summary,"first_unix":now,"last_unix":now,"count":1}));
            if st.len() > ATTENTION_KEEP {
                let mut by_age: Vec<(u64, String)> = st.iter().map(|(k, v)| (v["last_unix"].as_u64().unwrap_or(0), k.clone())).collect();
                by_age.sort();
                for (_, k) in by_age.into_iter().take(st.len() - ATTENTION_KEEP) {
                    st.remove(&k);
                }
            }
            item
        };
        atomic(&self.dir.join("state.json"), &json!({"schema":1,"items":st}))?;
        let mut md = String::from("# What needs the Octopus's attention\n\nMost recently seen first. Each item is evidence for a decision; Crab took no action on it. A repeat of the same facts is merged and counted.\n\n");
        for i in self.merged(ATTENTION_SHOWN) {
            let seen = match i["count"].as_u64().unwrap_or(1) {
                1 => format!("at {}", i["first_unix"]),
                n => format!("seen {n} times, first at {}, last at {}", i["first_unix"], i["last_unix"]),
            };
            md.push_str(&format!("- {} ({}, {seen}): {}\n", i["id"].as_str().unwrap_or(""), i["source"].as_str().unwrap_or(""), i["summary"].as_str().unwrap_or("")));
        }
        let tmp = self.dir.join("attention.md.tmp");
        let mut f = OpenOptions::new().create(true).truncate(true).write(true).mode(0o600).open(&tmp)?;
        f.write_all(md.as_bytes())?;
        f.sync_all()?;
        fs::rename(tmp, self.dir.join("attention.md"))?;
        Ok(item)
    }
    /// The newest `n` appended items (distinct fingerprints), oldest first (bounded tail read).
    pub fn recent(&self, n: usize) -> Vec<Value> {
        let p = self.dir.join("attention.jsonl");
        let Ok(mut f) = File::open(&p) else { return vec![] };
        let len = f.metadata().map(|m| m.len()).unwrap_or(0);
        let _ = f.seek(SeekFrom::Start(len.saturating_sub(256 * 1024)));
        let mut b = Vec::new();
        let _ = f.take(256 * 1024).read_to_end(&mut b);
        let text = String::from_utf8_lossy(&b);
        let all: Vec<Value> = text.lines().filter_map(|l| serde_json::from_str(l).ok()).collect();
        all[all.len().saturating_sub(n)..].to_vec()
    }
    /// "What needs my attention?"
    pub fn view(&self, now: u64) -> Value {
        let items = self.merged(ATTENTION_SHOWN);
        let lines: Vec<String> = items
            .iter()
            .map(|i| {
                let ago = now.saturating_sub(i["last_unix"].as_u64().unwrap_or(now));
                let seen = match i["count"].as_u64().unwrap_or(1) {
                    1 => format!("{ago} s ago"),
                    n => format!("seen {n} times, last {ago} s ago"),
                };
                format!("{} ({seen}): {}", i["source"].as_str().unwrap_or(""), i["summary"].as_str().unwrap_or(""))
            })
            .collect();
        let summary = match items.len() {
            0 => "Nothing needs your attention.".to_string(),
            n => format!("{n} recent item(s) need your attention; the newest: {}", lines[0]),
        };
        json!({"observed_unix":now,"summary":summary,"lines":lines,"items":items,"queue":self.dir.join("attention.jsonl")})
    }
}

/// How Crab sees one council Dolphin when a round is due.
#[derive(Clone, Debug, PartialEq)]
pub enum Availability {
    Idle,
    Busy(String),
    NotRunning(String),
}
/// An answer to one council consultation.
#[derive(Clone, Debug, PartialEq)]
pub enum Answer {
    Text(String),
    Failed(String),
}
/// What the council needs from Crab's Dolphin machinery; tests use a fake.
pub trait Pod {
    fn availability(&self, dolphin: &str) -> Availability;
    /// Queue the task through the consult queue; returns the consultation id.
    fn enqueue(&self, dolphin: &str, task: &str, trigger: Value, now: u64) -> Result<String>;
    fn answer(&self, consult_id: &str) -> Result<Option<Answer>>;
}
/// The council's durable state: `run_root/council/{state.json, journal.jsonl}`.
pub struct Council {
    pub config: CouncilConfig,
    dir: PathBuf,
    pub state: Value,
}
fn atomic(p: &Path, v: &Value) -> Result<()> {
    let tmp = p.with_extension("tmp");
    let mut f = OpenOptions::new().create(true).truncate(true).write(true).mode(0o600).open(&tmp)?;
    f.write_all(&serde_json::to_vec(v)?)?;
    f.sync_all()?;
    fs::rename(tmp, p)?;
    Ok(())
}
/// Compose the digest; `facts` carries flow status, capacity, recent actions, workers and
/// refusals as already-bounded JSON.
pub fn digest(round: &str, facts: &Value, probes: &[String], kinds: &[String], max: usize) -> String {
    let mut lines = vec![
        format!("Crab's council round {round}. You are one of Crab's council Dolphins. Another Dolphin receives the same digest independently; you do not see its answer."),
        "Review Crab's supervision state below and answer with ONE first line, exactly one of:".into(),
        "VERDICT: none".into(),
        "VERDICT: octopus".into(),
        format!("VERDICT: action:<{}>:<probe>", ACTIONS.join("|")),
        "VERDICT: capacity:<kind>:<more|fewer>:<n>".into(),
        "then a short reason (at most ten lines). Crab executes an action only when every council Dolphin names the same one; any capacity verdict goes to the Octopus, never executed. Your answer is evidence, not a command.".into(),
        format!("Probes: {}. Animal kinds: {}.", probes.join(", "), kinds.join(", ")),
        String::new(),
    ];
    if let Some(s) = facts["flow"]["summary"].as_str() {
        lines.push(format!("Flow: {s}"));
    }
    for l in facts["flow"]["lines"].as_array().into_iter().flatten().filter_map(Value::as_str) {
        lines.push(format!("- {l}"));
    }
    lines.push("Capacity per kind (queued, progress per hour, hours to drain, starved seconds):".into());
    for (k, v) in facts["capacity"]["kinds"].as_object().into_iter().flatten() {
        lines.push(format!("- {k}: {} probes, queued {}, {}/h, starved {} s", v["probes"], v["queued_total"], v["progress_per_hour_total"], v["starved_seconds_total"]));
        for p in v["by_probe"].as_array().into_iter().flatten() {
            lines.push(format!("  - {}: queued {}, {}/h, drain {} h, starved {} s", p["probe"].as_str().unwrap_or(""), p["queued"], p["progress_per_hour"], p["hours_to_drain"], p["starved_seconds"]));
        }
    }
    lines.push(format!("Workers: {}", facts["workers"]));
    lines.push("Recent flow actions:".into());
    for e in facts["actions"].as_array().into_iter().flatten() {
        lines.push(format!("- at {} {}: {} {}", e["at_unix"], e["probe"].as_str().unwrap_or(""), e["result"]["action"].as_str().unwrap_or(""), e["result"]["detail"].as_str().or(e["result"]["error"].as_str()).unwrap_or("")));
    }
    lines.push("Requests Crab did not understand:".into());
    for r in facts["refusals"].as_array().into_iter().flatten() {
        lines.push(format!("- {} ({})", r["excerpt"].as_str().unwrap_or(""), r["stage"].as_str().unwrap_or("")));
    }
    bounded(&lines.iter().map(|l| clean(l)).collect::<Vec<_>>().join("\n"), max)
}
/// The fingerprint of the digest facts: the stable flow facts (conditions, stacks over high
/// water, raised quarantine anomalies), the flow summary, worker states, recent actions without
/// their times and values, and the refused requests. Timestamps, round ids, counts that move
/// every minute, rates and ages are left out; free text has its digits masked.
pub fn facts_fingerprint(facts: &Value) -> String {
    let actions: Vec<Value> = facts["actions"]
        .as_array()
        .into_iter()
        .flatten()
        .map(|e| json!({"probe":e["probe"],"origin":e["origin"],"action":e["result"]["action"],"detail":mask_digits(e["result"]["detail"].as_str().or(e["result"]["error"].as_str()).unwrap_or(""))}))
        .collect();
    let refusals: Vec<Value> = facts["refusals"].as_array().into_iter().flatten().map(|r| json!({"excerpt":r["excerpt"],"stage":r["stage"]})).collect();
    let stable = json!({"state":facts["state"],"flow":mask_digits(facts["flow"]["summary"].as_str().unwrap_or("")),"workers":facts["workers"],"actions":actions,"refusals":refusals});
    hash(stable.to_string().as_bytes())
}
impl Council {
    pub fn open(config: CouncilConfig, run_root: &Path) -> Result<Council> {
        let dir = run_root.join("council");
        if !dir.exists() {
            fs::DirBuilder::new().recursive(true).mode(0o700).create(&dir)?;
        }
        let p = dir.join("state.json");
        let state = if p.exists() { serde_json::from_slice(&fs::read(&p)?)? } else { json!({"last_round_unix":0,"open":null}) };
        Ok(Council { config, dir, state })
    }
    fn journal(&self, v: &Value) -> Result<()> {
        flow::append(&self.dir.join("journal.jsonl"), v)
    }
    fn save(&self) -> Result<()> {
        atomic(&self.dir.join("state.json"), &self.state)
    }
    pub fn refusals_path(&self) -> PathBuf {
        self.dir.join("refusals.jsonl")
    }
    pub fn view(&self) -> Value {
        json!({"members":self.config.dolphins,"period_seconds":self.config.period_seconds,"min_spacing_seconds":self.config.min_spacing(),
            "quorum":self.config.quorum(),"member_timeout_seconds":self.config.timeout(),"state":self.state})
    }
    /// One compact skip: journaled only when its reason changes; repeats are counted in state.
    fn skip(&mut self, now: u64, reason: &str, fp: &str, dolphins: Value) -> Result<Value> {
        let same = self.state["skip"]["reason"] == reason;
        let count = if same { self.state["skip"]["count"].as_u64().unwrap_or(0) + 1 } else { 1 };
        let since = if same { self.state["skip"]["since_unix"].clone() } else { json!(now) };
        let e = json!({"at_unix":now,"round":"skipped","reason":reason,"facts_fingerprint":&fp[..16.min(fp.len())],"dolphins":dolphins});
        if !same {
            self.journal(&e)?;
        }
        self.state["skip"] = json!({"reason":reason,"count":count,"since_unix":since,"last_unix":now,"facts_fingerprint":fp});
        self.save()?;
        Ok(json!({"council":"skipped","reason":reason,"count":count,"journaled":!same}))
    }
    /// One non-blocking step. `facts` composes the digest facts; `exec` executes an agreed action
    /// on a probe within flow's bounds.
    ///
    /// Every `check_seconds` the facts are composed and fingerprinted. A round starts when the
    /// fingerprint differs from the last round's and `min_spacing_seconds` passed, or when
    /// `period_seconds` passed in any case; otherwise the evaluation is a compact skip. A round
    /// asks the idle members when at least `quorum` are idle; busy or stopped members are
    /// recorded as absent.
    pub fn tick(
        &mut self,
        now: u64,
        pod: &dyn Pod,
        attention: &Attention,
        probes: &[String],
        kinds: &[String],
        facts: &dyn Fn() -> Value,
        exec: &mut dyn FnMut(&str, &str) -> std::result::Result<Value, String>,
    ) -> Result<Value> {
        if !self.state["open"].is_null() {
            return self.close_if_answered(now, pod, attention, probes, kinds, exec);
        }
        let last_check = self.state["last_check_unix"].as_u64();
        if last_check.is_some_and(|t| now < t + self.config.check()) {
            return Ok(json!({"council":"waiting_for_check"}));
        }
        self.state["last_check_unix"] = json!(now);
        let f = facts();
        let fp = facts_fingerprint(&f);
        let last = self.state["last_round_unix"].as_u64().unwrap_or(0);
        let changed = self.state["last_fingerprint"].as_str() != Some(fp.as_str());
        let due = (changed && now >= last + self.config.min_spacing()) || now >= last + self.config.period_seconds;
        if !due {
            let why = if changed { "the facts changed; waiting for the minimum spacing" } else { "the facts are unchanged since the last round" };
            return self.skip(now, why, &fp, Value::Null);
        }
        let avail: Vec<(String, Availability)> = self.config.dolphins.iter().map(|d| (d.clone(), pod.availability(d))).collect();
        let shape = |a: &Availability| match a {
            Availability::Idle => json!("idle"),
            Availability::Busy(w) => json!({"busy":w}),
            Availability::NotRunning(w) => json!({"not_running":w}),
        };
        let seen: serde_json::Map<String, Value> = avail.iter().map(|(d, a)| (d.clone(), shape(a))).collect();
        let idle: Vec<&String> = avail.iter().filter(|(_, a)| *a == Availability::Idle).map(|(d, _)| d).collect();
        let absent: serde_json::Map<String, Value> = avail.iter().filter(|(_, a)| *a != Availability::Idle).map(|(d, a)| (d.clone(), shape(a))).collect();
        if idle.len() < self.config.quorum() {
            // A busy member's active task is never overwritten; below the quorum no round runs.
            let why = format!("below the quorum: {} of {} members idle, {} needed", idle.len(), self.config.dolphins.len(), self.config.quorum());
            return self.skip(now, &why, &fp, Value::Object(seen));
        }
        let round = format!("council-{now:010}");
        let text = digest(&round, &f, probes, kinds, self.config.max_task_bytes);
        self.state["last_round_unix"] = json!(now);
        self.state["last_fingerprint"] = json!(fp);
        self.state["skip"] = Value::Null;
        let mut asked = serde_json::Map::new();
        for d in &idle {
            let trigger = json!({"kind":"council","round":round,"key":format!("council:{round}:{d}")});
            match pod.enqueue(d, &text, trigger, now) {
                Ok(id) => {
                    asked.insert((*d).clone(), json!(id));
                }
                Err(e) => {
                    // A partial round is never combined: close it and escalate.
                    let why = format!("could not queue the digest for {d}: {e}");
                    attention.add_facts(now, "council", &format!("Council round {round} could not start: {why}."), json!({"round":round,"asked":asked}),
                        &json!({"kind":"council_start_failed","dolphin":d,"error":mask_digits(&e.to_string())}))?;
                    self.save()?;
                    let e = json!({"at_unix":now,"round":round,"outcome":"escalated","reason":why});
                    self.journal(&e)?;
                    return Ok(e);
                }
            }
        }
        self.state["open"] = json!({"round":round,"asked_unix":now,"asked":asked,"absent":absent,"digest_sha256":hash(text.as_bytes()),"digest_bytes":text.len(),"facts_fingerprint":fp,"facts":f});
        self.save()?;
        let e = json!({"at_unix":now,"round":round,"state":"asked","asked":asked,"absent":absent,"digest_sha256":hash(text.as_bytes()),"facts_fingerprint":&fp[..16]});
        self.journal(&e)?;
        Ok(e)
    }
    fn close_if_answered(
        &mut self,
        now: u64,
        pod: &dyn Pod,
        attention: &Attention,
        probes: &[String],
        kinds: &[String],
        exec: &mut dyn FnMut(&str, &str) -> std::result::Result<Value, String>,
    ) -> Result<Value> {
        let open = self.state["open"].clone();
        let round = open["round"].as_str().unwrap_or("").to_string();
        let fp = open["facts_fingerprint"].clone();
        let mut answers = Vec::new();
        let mut pending = Vec::new();
        for (d, id) in open["asked"].as_object().ok_or("open round lacks members")? {
            match pod.answer(id.as_str().unwrap_or(""))? {
                Some(a) => answers.push((d.clone(), a)),
                None => pending.push(d.clone()),
            }
        }
        if !pending.is_empty() && now < open["asked_unix"].as_u64().unwrap_or(0) + self.config.timeout() {
            return Ok(json!({"council":"waiting_for_answers","round":round}));
        }
        // Past the member timeout the silent members are recorded as timed out; the round closes
        // with the others and never blocks the next one.
        let mut verdicts = Vec::new();
        let mut evidence = Vec::new();
        for (d, a) in &answers {
            let (v, reason) = match a {
                Answer::Text(t) => parse_verdict(t, probes, kinds),
                Answer::Failed(w) => (Verdict::Malformed(format!("delivery failed: {w}")), String::new()),
            };
            evidence.push(json!({"dolphin":d,"verdict":v.label(),"reason":reason,
                "answer_sha256":match a { Answer::Text(t) => json!(hash(t.as_bytes())), Answer::Failed(_) => Value::Null }}));
            verdicts.push(v);
        }
        for d in &pending {
            evidence.push(json!({"dolphin":d,"verdict":"timed_out","reason":format!("no answer within {} s", self.config.timeout())}));
        }
        let labels: serde_json::Map<String, Value> = evidence.iter().map(|e| (e["dolphin"].as_str().unwrap_or("").to_string(), e["verdict"].clone())).collect();
        let numbers = json!({"capacity":open["facts"]["capacity"],"flow":open["facts"]["flow"]["summary"]});
        let base = json!({"at_unix":now,"round":round,"absent":open["absent"],"timed_out":pending,"digest_sha256":open["digest_sha256"]});
        let finish = |me: &mut Self, mut e: Value, extra: Value| -> Result<Value> {
            for (k, v) in extra.as_object().into_iter().flatten() {
                e[k] = v.clone();
            }
            me.state["open"] = Value::Null;
            me.save()?;
            me.journal(&e)?;
            Ok(e)
        };
        if answers.len() < self.config.quorum() {
            let who = pending.join(", ");
            attention.add_facts(now, "council", &format!("Council round {round} expired: {who} did not answer within {} s; {} of {} needed answers arrived.", self.config.timeout(), answers.len(), self.config.quorum()),
                json!({"round":round,"asked":open["asked"],"answers":evidence}), &json!({"kind":"council_expired","verdicts":labels,"facts_fingerprint":fp}))?;
            return finish(self, base, json!({"outcome":"escalated","reason":"expired","answers":evidence}));
        }
        // Unanimity over the members that answered.
        let outcome = combine(&verdicts);
        let (label, done) = match &outcome {
            Outcome::JournalOnly => ("journal_only", Value::Null),
            Outcome::Execute { action, probe } => match exec(probe, action) {
                Ok(v) if v["error"].is_null() => ("executed", v),
                Ok(v) => {
                    attention.add_facts(now, "council", &format!("Council round {round} agreed on {action} for {probe}, but it failed: {}.", v["error"]), json!({"round":round,"answers":evidence,"result":v}),
                        &json!({"kind":"council_exec_failed","action":action,"probe":probe,"error":mask_digits(&v["error"].to_string())}))?;
                    ("escalated", v)
                }
                Err(why) => {
                    attention.add_facts(now, "council", &format!("Council round {round} agreed on {action} for {probe}, which Crab may not do now: {why}."), json!({"round":round,"answers":evidence}),
                        &json!({"kind":"council_refused","action":action,"probe":probe,"why":why}))?;
                    ("escalated", json!({"refused":why}))
                }
            },
            Outcome::Escalate(why) => {
                let parts: Vec<String> = evidence.iter().map(|e| format!("{} says {}: {}", e["dolphin"].as_str().unwrap_or(""), e["verdict"].as_str().unwrap_or(""), e["reason"].as_str().unwrap_or(""))).collect();
                let capacity = verdicts.iter().any(|v| matches!(v, Verdict::Capacity { .. }));
                attention.add_facts(now, if capacity { "council-capacity" } else { "council" }, &format!("Council round {round}: {why}. {}", parts.join(" | ")), json!({"round":round,"answers":evidence,"numbers":numbers}),
                    &json!({"kind":"council_escalation","why":why,"verdicts":labels,"facts_fingerprint":fp}))?;
                ("escalated", Value::Null)
            }
        };
        finish(self, base, json!({"outcome":label,"answers":evidence,"result":done}))
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::{cell::RefCell, collections::BTreeMap};
    struct FakePod {
        avail: BTreeMap<String, Availability>,
        queued: RefCell<Vec<(String, String)>>,
        answers: RefCell<BTreeMap<String, Answer>>,
    }
    impl Pod for FakePod {
        fn availability(&self, d: &str) -> Availability {
            self.avail.get(d).cloned().unwrap_or(Availability::NotRunning("absent".into()))
        }
        fn enqueue(&self, d: &str, task: &str, _: Value, _: u64) -> Result<String> {
            self.queued.borrow_mut().push((d.into(), task.into()));
            Ok(format!("cq-{d}"))
        }
        fn answer(&self, id: &str) -> Result<Option<Answer>> {
            Ok(self.answers.borrow().get(id).cloned())
        }
    }
    fn pod() -> FakePod {
        FakePod {
            avail: [("opus".to_string(), Availability::Idle), ("sol".to_string(), Availability::Idle)].into(),
            queued: RefCell::new(vec![]),
            answers: RefCell::new(BTreeMap::new()),
        }
    }
    fn probes() -> Vec<String> {
        vec!["fish-ready".into(), "fish-t00".into()]
    }
    fn kinds() -> Vec<String> {
        vec!["fish".into(), "whale".into()]
    }
    fn v(t: &str) -> Verdict {
        parse_verdict(t, &probes(), &kinds()).0
    }
    #[test]
    fn verdicts_parse_strictly() {
        assert_eq!(v("VERDICT: none\nall fine"), Verdict::None);
        assert_eq!(v("\n  VERDICT: octopus — unclear"), Verdict::Octopus);
        assert_eq!(v("VERDICT: action:pause_producer:fish-ready quarantine high"), Verdict::Action { action: "pause_producer".into(), probe: "fish-ready".into() });
        assert_eq!(v("VERDICT: capacity:whale:more:2 backlog grows"), Verdict::Capacity { kind: "whale".into(), direction: "more".into(), n: 2 });
        let (_, reason) = parse_verdict("VERDICT: none - the queues drain\nsecond line", &probes(), &kinds());
        assert_eq!(reason, "the queues drain second line");
        for bad in [
            "",
            "verdict: none",
            "Sure! VERDICT: none",
            "VERDICT: action:kill:fish-ready",
            "VERDICT: action:restart:eel",
            "VERDICT: action:restart",
            "VERDICT: capacity:fish:double:2",
            "VERDICT: capacity:kraken:more:2",
            "VERDICT: capacity:fish:more:0",
            "VERDICT: capacity:fish:more:999",
            "VERDICT: rm -rf /",
            "VERDICT: none; also start the Eel",
        ] {
            assert!(matches!(v(bad), Verdict::Malformed(_)), "{bad:?} -> {:?}", v(bad));
        }
    }
    #[test]
    fn combination_escalates_everything_but_agreement() {
        let a = || Verdict::Action { action: "hold".into(), probe: "fish-t00".into() };
        assert_eq!(combine(&[Verdict::None, Verdict::None]), Outcome::JournalOnly);
        assert_eq!(combine(&[a(), a()]), Outcome::Execute { action: "hold".into(), probe: "fish-t00".into() });
        // Red controls: disagreement, one-sided action, octopus, capacity and malformed escalate.
        for pair in [
            vec![a(), Verdict::Action { action: "restart".into(), probe: "fish-t00".into() }],
            vec![a(), Verdict::Action { action: "hold".into(), probe: "fish-ready".into() }],
            vec![a(), Verdict::None],
            vec![Verdict::None, Verdict::Octopus],
            vec![Verdict::Capacity { kind: "fish".into(), direction: "more".into(), n: 2 }, Verdict::Capacity { kind: "fish".into(), direction: "more".into(), n: 2 }],
            vec![Verdict::None, Verdict::Malformed("x".into())],
            vec![a()],
        ] {
            assert!(matches!(combine(&pair), Outcome::Escalate(_)), "{pair:?}");
        }
    }
    fn dir() -> crate::flow::tests::Fx {
        crate::flow::tests::fx()
    }
    fn cfg() -> CouncilConfig {
        CouncilConfig {
            dolphins: vec!["opus".into(), "sol".into()],
            period_seconds: 1800,
            max_task_bytes: 6000,
            answer_timeout_seconds: None,
            min_spacing_seconds: None,
            check_seconds: None,
            quorum: None,
            member_timeout_seconds: None,
        }
    }
    fn facts_with(summary: &str, queued: u64, at: u64) -> Value {
        json!({"flow":{"summary":summary,"lines":[format!("fish-t00: has work ({queued}, low water 1); last change {at} s ago.")]},
            "state":{"probes":[{"probe":"fish-t00","conditions":[],"over_high_water":[],"quarantine_rate_high":[]}]},
            "capacity":{"kinds":{"fish":{"probes":2,"queued_total":queued,"progress_per_hour_total":12.5,"starved_seconds_total":600,"by_probe":[]}}},
            "workers":{"eel":"running"},"actions":[{"at_unix":at,"probe":"fish-t00","result":{"action":"report","detail":format!("no progress for {at} s")}}],
            "refusals":[{"excerpt":"Feed the fish now","stage":"unsupported_capability"}]})
    }
    #[test]
    fn rounds_are_event_driven_and_never_execute_capacity() {
        let f = dir();
        let cfg = cfg();
        cfg.validate(&["opus".into(), "sol".into(), "copilot".into()]).unwrap();
        assert!(CouncilConfig { dolphins: vec!["opus".into()], ..cfg.clone() }.validate(&["opus".into()]).is_err());
        assert!(CouncilConfig { dolphins: vec!["opus".into(), "opus".into()], ..cfg.clone() }.validate(&["opus".into()]).is_err());
        assert!(CouncilConfig { quorum: Some(1), ..cfg.clone() }.validate(&["opus".into(), "sol".into()]).is_err(), "a quorum is at least two");
        assert!(CouncilConfig { quorum: Some(3), ..cfg.clone() }.validate(&["opus".into(), "sol".into()]).is_err(), "a quorum cannot exceed the members");
        assert!(CouncilConfig { min_spacing_seconds: Some(3600), ..cfg.clone() }.validate(&["opus".into(), "sol".into()]).is_err(), "min spacing within the period");
        let att = Attention::open(&f.0).unwrap();
        let mut c = Council::open(cfg, &f.0).unwrap();
        // Counts, ages and times move every minute; the fingerprint stays.
        let moving = RefCell::new((3u64, 100u64));
        let summary = RefCell::new("Of all probes: 1 of 2 have work; none is clogged.".to_string());
        let facts = || {
            let (q, a) = *moving.borrow();
            facts_with(&summary.borrow(), q, a)
        };
        assert_eq!(facts_fingerprint(&facts_with("x 1", 3, 100)), facts_fingerprint(&facts_with("x 2", 99, 7)), "digits, counts and ages are not facts");
        let executed = RefCell::new(Vec::<String>::new());
        let mut exec = |p: &str, a: &str| -> std::result::Result<Value, String> {
            executed.borrow_mut().push(format!("{a}:{p}"));
            Ok(json!({"action":a}))
        };
        // Busy below the quorum: skipped, nothing queued, journaled once while the reason holds.
        let mut p = pod();
        p.avail.insert("sol".into(), Availability::Busy("pending consultation".into()));
        let e = c.tick(20_000, &p, &att, &probes(), &kinds(), &facts, &mut exec).unwrap();
        assert!(e["reason"].as_str().unwrap().contains("below the quorum"), "{e}");
        assert_eq!(c.tick(20_030, &p, &att, &probes(), &kinds(), &facts, &mut exec).unwrap()["council"], "waiting_for_check");
        assert_eq!(c.tick(20_060, &p, &att, &probes(), &kinds(), &facts, &mut exec).unwrap()["journaled"], false);
        assert!(p.queued.borrow().is_empty());
        // A round: the same digest to both, independently.
        let p = pod();
        let e = c.tick(22_000, &p, &att, &probes(), &kinds(), &facts, &mut exec).unwrap();
        assert_eq!(e["state"], "asked");
        let q = p.queued.borrow().clone();
        assert_eq!(q.len(), 2);
        assert_eq!(q[0].1, q[1].1, "both see the same digest");
        assert!(q[0].1.contains("Feed the fish now") && q[0].1.contains("fish: 2 probes, queued 3"));
        // One answer only: waiting.
        p.answers.borrow_mut().insert("cq-opus".into(), Answer::Text("VERDICT: capacity:fish:more:2 drain time too long".into()));
        assert_eq!(c.tick(22_100, &p, &att, &probes(), &kinds(), &facts, &mut exec).unwrap()["council"], "waiting_for_answers");
        // Both say capacity: escalated with both reasons and the numbers, never executed.
        p.answers.borrow_mut().insert("cq-sol".into(), Answer::Text("VERDICT: capacity:fish:more:2 starved pagers".into()));
        let e = c.tick(22_200, &p, &att, &probes(), &kinds(), &facts, &mut exec).unwrap();
        assert_eq!(e["outcome"], "escalated");
        assert!(executed.borrow().is_empty(), "a capacity verdict is never executed");
        let item = att.recent(1)[0].clone();
        assert_eq!(item["source"], "council-capacity");
        assert!(item["summary"].as_str().unwrap().contains("drain time too long") && item["summary"].as_str().unwrap().contains("starved pagers"));
        assert_eq!(item["evidence"]["numbers"]["capacity"]["kinds"]["fish"]["queued_total"], 3);
        // Unchanged facts (only moving counts): no round before the longest interval; one
        // journal line for the whole run of skips.
        let lines = || fs::read_to_string(f.0.join("council/journal.jsonl")).unwrap().lines().count();
        let before = lines();
        for t in (22_300..23_800).step_by(60) {
            *moving.borrow_mut() = (t % 97, t);
            let e = c.tick(t, &pod(), &att, &probes(), &kinds(), &facts, &mut exec).unwrap();
            assert_eq!(e["council"], "skipped", "{t}: {e}");
        }
        assert_eq!(lines(), before + 1, "a run of unchanged skips is one journal line");
        assert_eq!(c.state["skip"]["count"], 25);
        // The longest interval passed: a round on unchanged facts.
        let p = pod();
        assert_eq!(c.tick(23_800, &p, &att, &probes(), &kinds(), &facts, &mut exec).unwrap()["state"], "asked");
        p.answers.borrow_mut().insert("cq-opus".into(), Answer::Text("VERDICT: none".into()));
        p.answers.borrow_mut().insert("cq-sol".into(), Answer::Text("VERDICT: none".into()));
        assert_eq!(c.tick(23_810, &p, &att, &probes(), &kinds(), &facts, &mut exec).unwrap()["outcome"], "journal_only");
        // Changed facts: a round once the minimum spacing (300 s) passed, not before.
        *summary.borrow_mut() = "Of all probes: 1 of 2 have work; 1 are clogged.".into();
        let e = c.tick(23_900, &pod(), &att, &probes(), &kinds(), &facts, &mut exec).unwrap();
        assert!(e["reason"].as_str().unwrap().contains("minimum spacing"), "{e}");
        let p = pod();
        assert_eq!(c.tick(24_100, &p, &att, &probes(), &kinds(), &facts, &mut exec).unwrap()["state"], "asked");
        // Agreement on an allowed action executes it.
        p.answers.borrow_mut().insert("cq-opus".into(), Answer::Text("VERDICT: action:hold:fish-t00 limit".into()));
        p.answers.borrow_mut().insert("cq-sol".into(), Answer::Text("VERDICT: action:hold:fish-t00 wait".into()));
        assert_eq!(c.tick(24_110, &p, &att, &probes(), &kinds(), &facts, &mut exec).unwrap()["outcome"], "executed");
        assert_eq!(*executed.borrow(), vec!["hold:fish-t00"]);
        // Disagreement and malformed escalate; nothing executes.
        for (a, b) in [("VERDICT: action:hold:fish-t00", "VERDICT: none"), ("VERDICT: none", "I think all is fine.")] {
            let p = pod();
            let t = c.state["last_round_unix"].as_u64().unwrap() + 1800;
            c.tick(t, &p, &att, &probes(), &kinds(), &facts, &mut exec).unwrap();
            p.answers.borrow_mut().insert("cq-opus".into(), Answer::Text(a.into()));
            p.answers.borrow_mut().insert("cq-sol".into(), Answer::Text(b.into()));
            assert_eq!(c.tick(t + 1, &p, &att, &probes(), &kinds(), &facts, &mut exec).unwrap()["outcome"], "escalated");
        }
        assert_eq!(executed.borrow().len(), 1);
        // A silent member below the quorum: the round expires after the member timeout (600 s),
        // not after two periods, and the next round is not blocked.
        let p = pod();
        let t = c.state["last_round_unix"].as_u64().unwrap() + 1800;
        c.tick(t, &p, &att, &probes(), &kinds(), &facts, &mut exec).unwrap();
        p.answers.borrow_mut().insert("cq-opus".into(), Answer::Text("VERDICT: none".into()));
        assert_eq!(c.tick(t + 599, &p, &att, &probes(), &kinds(), &facts, &mut exec).unwrap()["council"], "waiting_for_answers");
        let e = c.tick(t + 600, &p, &att, &probes(), &kinds(), &facts, &mut exec).unwrap();
        assert_eq!(e["reason"], "expired");
        assert_eq!(e["timed_out"], json!(["sol"]));
        assert!(att.view(t + 600)["summary"].as_str().unwrap().contains("expired"));
        assert!(c.state["open"].is_null());
    }
    fn pod3() -> FakePod {
        let mut p = pod();
        p.avail.insert("copilot".into(), Availability::Idle);
        p
    }
    #[test]
    fn a_quorum_runs_without_a_busy_member_and_closes_without_a_silent_one() {
        let f = dir();
        let cfg = CouncilConfig { dolphins: vec!["opus".into(), "sol".into(), "copilot".into()], ..cfg() };
        cfg.validate(&["opus".into(), "sol".into(), "copilot".into()]).unwrap();
        let att = Attention::open(&f.0).unwrap();
        let mut c = Council::open(cfg, &f.0).unwrap();
        let facts = || facts_with("Of all probes: 1 of 2 have work; none is clogged.", 3, 1);
        let executed = RefCell::new(Vec::<String>::new());
        let mut exec = |p: &str, a: &str| -> std::result::Result<Value, String> {
            executed.borrow_mut().push(format!("{a}:{p}"));
            Ok(json!({"action":a}))
        };
        // One member busy: the two idle ones meet; the busy one is recorded absent and untouched.
        let mut p = pod3();
        p.avail.insert("sol".into(), Availability::Busy("pending consultation".into()));
        let e = c.tick(10_000, &p, &att, &probes(), &kinds(), &facts, &mut exec).unwrap();
        assert_eq!(e["state"], "asked", "{e}");
        assert!(e["absent"]["sol"]["busy"].is_string());
        assert_eq!(p.queued.borrow().iter().map(|q| q.0.clone()).collect::<Vec<_>>(), vec!["opus", "copilot"]);
        // Unanimity over those who answered executes.
        p.answers.borrow_mut().insert("cq-opus".into(), Answer::Text("VERDICT: action:hold:fish-t00".into()));
        p.answers.borrow_mut().insert("cq-copilot".into(), Answer::Text("VERDICT: action:hold:fish-t00".into()));
        let e = c.tick(10_010, &p, &att, &probes(), &kinds(), &facts, &mut exec).unwrap();
        assert_eq!(e["outcome"], "executed", "{e}");
        assert_eq!(e["absent"]["sol"]["busy"], "pending consultation");
        // All three asked; one silent past the member timeout: closed with the other two.
        let p = pod3();
        c.tick(12_000, &p, &att, &probes(), &kinds(), &facts, &mut exec).unwrap();
        p.answers.borrow_mut().insert("cq-opus".into(), Answer::Text("VERDICT: action:hold:fish-t00".into()));
        p.answers.borrow_mut().insert("cq-sol".into(), Answer::Text("VERDICT: action:hold:fish-t00".into()));
        assert_eq!(c.tick(12_300, &p, &att, &probes(), &kinds(), &facts, &mut exec).unwrap()["council"], "waiting_for_answers");
        let e = c.tick(12_600, &p, &att, &probes(), &kinds(), &facts, &mut exec).unwrap();
        assert_eq!(e["outcome"], "executed", "{e}");
        assert_eq!(e["timed_out"], json!(["copilot"]));
        assert!(e["answers"].as_array().unwrap().iter().any(|a| a["dolphin"] == "copilot" && a["verdict"] == "timed_out"));
        assert_eq!(executed.borrow().len(), 2);
        // A dissent among those who answered still escalates (no majority vote).
        let p = pod3();
        c.tick(15_000, &p, &att, &probes(), &kinds(), &facts, &mut exec).unwrap();
        p.answers.borrow_mut().insert("cq-opus".into(), Answer::Text("VERDICT: action:hold:fish-t00".into()));
        p.answers.borrow_mut().insert("cq-sol".into(), Answer::Text("VERDICT: action:hold:fish-t00".into()));
        p.answers.borrow_mut().insert("cq-copilot".into(), Answer::Text("VERDICT: none".into()));
        assert_eq!(c.tick(15_010, &p, &att, &probes(), &kinds(), &facts, &mut exec).unwrap()["outcome"], "escalated");
        assert_eq!(executed.borrow().len(), 2);
        // Two of three busy: below the quorum, nothing queued.
        let mut p = pod3();
        p.avail.insert("sol".into(), Availability::Busy("x".into()));
        p.avail.insert("copilot".into(), Availability::NotRunning("stopped".into()));
        let e = c.tick(17_000, &p, &att, &probes(), &kinds(), &facts, &mut exec).unwrap();
        assert_eq!(e["council"], "skipped");
        assert!(p.queued.borrow().is_empty());
    }
    #[test]
    fn attention_merges_unchanged_facts_and_appends_changes() {
        let f = dir();
        let att = Attention::open(&f.0).unwrap();
        let facts = |v: u64| json!({"kind":"flow","probe":"fish-t05","action":"report","stacks":[{"stack":"uncertain","value":v}]});
        att.add_facts(100, "flow", "fish-t05: round council-100 says one thing", json!({"round":"council-100"}), &facts(1)).unwrap();
        let m = att.add_facts(200, "flow", "fish-t05: round council-200 says it again", json!({"round":"council-200"}), &facts(1)).unwrap();
        assert_eq!(m["merged"], true);
        att.add_facts(300, "flow", "fish-t05: round council-300", json!({}), &facts(1)).unwrap();
        let lines = || fs::read_to_string(f.0.join("attention/attention.jsonl")).unwrap().lines().count();
        assert_eq!(lines(), 1, "a repeat of unchanged facts is not appended");
        let v = att.view(400);
        assert_eq!(v["items"][0]["count"], 3);
        assert_eq!(v["items"][0]["first_unix"], 100);
        assert_eq!(v["items"][0]["last_unix"], 300);
        assert!(v["lines"][0].as_str().unwrap().contains("seen 3 times, last 100 s ago"), "{v}");
        assert!(v["items"][0]["summary"].as_str().unwrap().contains("council-300"), "the newest summary is kept");
        let md = fs::read_to_string(f.0.join("attention/attention.md")).unwrap();
        assert!(md.contains("seen 3 times, first at 100, last at 300"), "{md}");
        // A change of facts is appended and shown first.
        att.add_facts(500, "flow", "fish-t05: the count changed", json!({}), &facts(2)).unwrap();
        assert_eq!(lines(), 2);
        assert_eq!(att.view(500)["items"][0]["count"], 1);
        // The same facts from another source are another item.
        att.add_facts(600, "council", "x", json!({}), &facts(2)).unwrap();
        assert_eq!(lines(), 3);
        // The plain add merges an exact repeat only.
        att.add(700, "council", "Council round council-1 expired.", json!({})).unwrap();
        att.add(800, "council", "Council round council-1 expired.", json!({})).unwrap();
        att.add(900, "council", "Council round council-2 expired.", json!({})).unwrap();
        assert_eq!(lines(), 5);
        assert_eq!(mask_digits("for 2400 s at 17938"), "for # s at #");
    }
    #[test]
    fn a_legacy_queue_is_seeded_into_the_merged_view() {
        let f = dir();
        fs::create_dir_all(f.0.join("attention")).unwrap();
        fs::write(f.0.join("attention/attention.jsonl"), "{\"id\":\"at-1-aa\",\"at_unix\":1,\"source\":\"council\",\"summary\":\"old\"}\n").unwrap();
        let att = Attention::open(&f.0).unwrap();
        assert_eq!(att.view(10)["items"][0]["summary"], "old");
        att.add(20, "flow", "new", json!({})).unwrap();
        let v = att.view(30);
        assert_eq!(v["items"].as_array().unwrap().len(), 2, "{v}");
        assert_eq!(v["items"][0]["summary"], "new");
    }
}
