//! Work pools: Crab hands out the work. A pool is configuration only — its members (each a configured Crab worker
//! with an assignment file it reads and a report file it writes) and its jobs (each with a priority, a backlog probe
//! and opaque parameters for the member). Crab has no member-specific code.
//!
//! Model (per pool, per tick at time t):
//!   a unit is (job, part), part < job.parts (a job is split only when it declares `parts` > 1);
//!   a lease binds one member to one unit until `expires_unix`;
//!   invariants: a member holds at most one lease; a unit is held by at most one member;
//!   a lease is renewed (expires = t + lease_seconds) only while its member's worker runs and the member's report
//!     names the lease and is fresh (written within lease_seconds), or the lease is younger than one lease period;
//!   a lease ends when it expires, or when its member reports it `finished`, `stopped` or `refused`;
//!   a finished or refused unit is not leased again while its job's backlog reading stays what it was then;
//!   an idle member (its worker running, no lease, its report not `stopped`) takes the waiting unit with the
//!     highest priority, then the largest known backlog, then the job and part order; a backlog that is unknown
//!     or zero is not work.
//! Delivery: the assignment file is rewritten atomically on every assignment, renewal and release (a released file
//! names no job), so a member reads it between its own steps and never works an expired lease. Every assignment,
//! renewal-less expiry and release is journaled in `run_root/flow/pools.jsonl`; leases persist in
//! `run_root/flow/pools.json`. Cost: per tick one bounded read per member report and per job backlog.
use crate::flow::{self, Source, WorkerControl};
use crate::Result;
use serde::{Deserialize, Serialize};
use serde_json::{json, Value};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, OpenOptions},
    io::Write,
    os::unix::fs::OpenOptionsExt,
    path::{Component, Path, PathBuf},
};

const MEMBERS_MAX: usize = 32;
const JOBS_MAX: usize = 64;
const PARTS_MAX: u32 = 16;
const PARAMS_MAX: usize = 4096;
const REPORT_MAX: u64 = 64 * 1024;
/// Report states that end a lease.
pub const ENDING: [&str; 3] = ["finished", "stopped", "refused"];

fn error(s: impl Into<String>) -> Box<dyn std::error::Error> {
    s.into().into()
}
fn d_lease() -> u64 {
    900
}
fn d_parts() -> u32 {
    1
}

#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Member {
    pub name: String,
    /// The configured Crab worker that runs this member.
    pub worker: String,
    /// Written by Crab: the member's current lease (or a released file).
    pub assignment: PathBuf,
    /// Written by the member after each step: `{"lease_id","state","unix",…}`.
    pub report: PathBuf,
}
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct Job {
    pub name: String,
    /// Higher first.
    #[serde(default)]
    pub priority: u32,
    /// How much work the job has left (a count; unknown or zero is no work).
    pub backlog: Source,
    /// Split by chunk ranges into this many units, each leased to one member (default 1: never split).
    #[serde(default = "d_parts")]
    pub parts: u32,
    /// Shown in English and the digest; the member enforces its own licence rules.
    #[serde(default)]
    pub licence: Option<String>,
    /// Copied into the assignment for the member (a JSON object, at most 4 KiB).
    #[serde(default)]
    pub params: Value,
}
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct PoolConfig {
    pub name: String,
    /// The animal kind addressed in English ("nautilus").
    pub kind: String,
    pub members: Vec<Member>,
    pub jobs: Vec<Job>,
    #[serde(default = "d_lease")]
    pub lease_seconds: u64,
}
fn trusted(p: &Path) -> bool {
    p.is_absolute() && p.to_str().is_some_and(|s| s.len() <= 1024) && !p.components().any(|c| matches!(c, Component::ParentDir))
}
fn ident(s: &str) -> bool {
    crate::workers::valid_name(s)
}
impl PoolConfig {
    pub fn validate(&self, workers: &[String]) -> Result<()> {
        if !ident(&self.name) || !ident(&self.kind) || self.kind.len() > 32 {
            return Err(error("a pool has an ASCII name and kind"));
        }
        if self.members.is_empty() || self.members.len() > MEMBERS_MAX || self.jobs.is_empty() || self.jobs.len() > JOBS_MAX {
            return Err(error(format!("pool {}: 1..32 members and 1..64 jobs", self.name)));
        }
        if !(120..=86400).contains(&self.lease_seconds) {
            return Err(error(format!("pool {}: lease_seconds 120..86400", self.name)));
        }
        let (mut names, mut files, mut ws) = (BTreeSet::new(), BTreeSet::new(), BTreeSet::new());
        for m in &self.members {
            if !ident(&m.name) || !names.insert(m.name.clone()) || !workers.contains(&m.worker) || !ws.insert(m.worker.clone()) {
                return Err(error(format!("pool {}: member {} needs a unique name and its own configured worker", self.name, m.name)));
            }
            if !trusted(&m.assignment) || !trusted(&m.report) || !files.insert(m.assignment.clone()) || !files.insert(m.report.clone()) {
                return Err(error(format!("pool {}: member {} needs its own absolute assignment and report files", self.name, m.name)));
            }
        }
        let mut jobs = BTreeSet::new();
        for j in &self.jobs {
            if !ident(&j.name) || !jobs.insert(j.name.clone()) || !(1..=PARTS_MAX).contains(&j.parts) || j.priority > 1000 {
                return Err(error(format!("pool {}: job {} needs a unique name, parts 1..16, priority 0..1000", self.name, j.name)));
            }
            if !(j.params.is_null() || j.params.is_object()) || serde_json::to_vec(&j.params)?.len() > PARAMS_MAX {
                return Err(error(format!("pool {}: job {} params are an object of at most 4 KiB", self.name, j.name)));
            }
            if j.licence.as_ref().is_some_and(|l| l.len() > 256) {
                return Err(error(format!("pool {}: job {} licence line too long", self.name, j.name)));
            }
            check_backlog(&j.backlog).map_err(|e| error(format!("pool {}: job {}: {e}", self.name, j.name)))?;
        }
        Ok(())
    }
}
fn check_backlog(s: &Source) -> Result<()> {
    let probe = flow::Probe {
        name: "backlog".into(),
        group: "pool".into(),
        worker: None,
        input: Some(flow::Gauge { source: s.clone(), low_water: 1 }),
        stacks: vec![],
        progress: None,
        producer: None,
        limit: None,
        restart_on_stall: false,
        hysteresis: 1,
    };
    flow::FlowConfig { interval_seconds: 60, probes: vec![probe], pools: vec![] }.validate(&[])
}

#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct Lease {
    pub job: String,
    pub part: u32,
    pub lease_id: String,
    pub assigned_unix: u64,
    pub expires_unix: u64,
}
#[derive(Clone, Debug, Default, PartialEq, Serialize, Deserialize)]
pub struct PoolState {
    /// member → lease
    pub leases: BTreeMap<String, Lease>,
    /// "job#part" → the backlog reading when the unit was finished or refused
    pub done: BTreeMap<String, u64>,
    pub seq: u64,
    /// the last backlog readings, by job (for status)
    #[serde(default)]
    pub backlog: BTreeMap<String, Option<u64>>,
    /// the last report state, by member
    #[serde(default)]
    pub reports: BTreeMap<String, String>,
    /// the last worker state, by member
    #[serde(default)]
    pub workers: BTreeMap<String, String>,
    /// when the pool was last evaluated
    #[serde(default)]
    pub evaluated_unix: u64,
}
fn unit(job: &str, part: u32) -> String {
    format!("{job}#{part}")
}

/// A member's report, bounded; `None` when absent or unreadable.
pub fn read_report(p: &Path) -> Option<Value> {
    let b = fs::File::open(p).ok().and_then(|f| {
        let mut b = Vec::new();
        std::io::Read::read_to_end(&mut std::io::Read::take(f, REPORT_MAX + 1), &mut b).ok()?;
        Some(b)
    })?;
    if b.len() as u64 > REPORT_MAX {
        return None;
    }
    serde_json::from_slice(&b).ok()
}
fn write_atomic(p: &Path, v: &Value) -> Result<()> {
    if let Some(d) = p.parent() {
        fs::create_dir_all(d)?;
    }
    let tmp = p.with_extension("tmp");
    let mut f = OpenOptions::new().create(true).truncate(true).write(true).mode(0o644).open(&tmp)?;
    f.write_all(&serde_json::to_vec(v)?)?;
    f.write_all(b"\n")?;
    f.sync_all()?;
    fs::rename(tmp, p)?;
    Ok(())
}

/// One pool with its durable state.
pub struct Pool {
    pub config: PoolConfig,
    pub state: PoolState,
}

impl Pool {
    fn assignment(&self, m: &Member, l: &Lease) -> Value {
        let j = self.config.jobs.iter().find(|j| j.name == l.job);
        json!({"schema":1,"pool":self.config.name,"member":m.name,"job":l.job,"part":l.part,"parts":j.map_or(1, |j| j.parts),
            "lease_id":l.lease_id,"assigned_unix":l.assigned_unix,"expires_unix":l.expires_unix,
            "licence":j.and_then(|j| j.licence.clone()),"params":j.map_or(Value::Null, |j| j.params.clone())})
    }
    fn release(&mut self, m: &Member, why: &str, now: u64, events: &mut Vec<Value>) -> Result<()> {
        if let Some(l) = self.state.leases.remove(&m.name) {
            write_atomic(&m.assignment, &json!({"schema":1,"pool":self.config.name,"member":m.name,"job":null,"released":l.lease_id,"reason":why,"unix":now}))?;
            let mut e = json!({"at_unix":now,"pool":self.config.name,"probe":format!("{}/{}", self.config.name, m.name),"event":"released","member":m.name,"job":l.job,"part":l.part,"lease_id":l.lease_id,"reason":why});
            if matches!(why, "stopped" | "refused") {
                // a person decides: a limit-stopped member waits for a resume; a refused job is misconfigured
                e["result"] = json!({"action":"report","detail":format!("{} {why} job {}; it is released and not handed out to it again until its report changes", m.name, l.job)});
            }
            events.push(e);
        }
        Ok(())
    }

    /// One tick: end, renew and hand out leases. Returns the journal events (the caller appends them).
    pub fn tick(&mut self, now: u64, workers: &dyn WorkerControl) -> Result<Vec<Value>> {
        let mut events = Vec::new();
        let members = self.config.members.clone();
        let running = |m: &Member| workers.observe(&m.worker).is_some_and(|(s, _)| s == "running");
        // readings
        let backlog: BTreeMap<String, Option<u64>> = self.config.jobs.iter().map(|j| (j.name.clone(), flow::read(&j.backlog).ok())).collect();
        let reports: BTreeMap<String, Option<Value>> = members.iter().map(|m| (m.name.clone(), read_report(&m.report))).collect();
        // ends and renewals
        for m in &members {
            let Some(l) = self.state.leases.get(&m.name).cloned() else { continue };
            let r = reports[&m.name].as_ref().filter(|r| r["lease_id"] == l.lease_id.as_str());
            let state = r.and_then(|r| r["state"].as_str()).unwrap_or("");
            if !self.config.jobs.iter().any(|j| j.name == l.job && l.part < j.parts) {
                self.release(m, "the job is no longer configured", now, &mut events)?;
            } else if ENDING.contains(&state) {
                if state != "stopped" {
                    self.state.done.insert(unit(&l.job, l.part), backlog[&l.job].unwrap_or(0));
                }
                self.release(m, state, now, &mut events)?;
            } else if now >= l.expires_unix {
                self.release(m, "expired", now, &mut events)?;
            } else if running(m) {
                let fresh = r.and_then(|r| r["unix"].as_u64()).is_some_and(|t| t + self.config.lease_seconds > now);
                if fresh || now < l.assigned_unix + self.config.lease_seconds {
                    let l2 = Lease { expires_unix: now + self.config.lease_seconds, ..l };
                    write_atomic(&m.assignment, &self.assignment(m, &l2))?;
                    self.state.leases.insert(m.name.clone(), l2);
                }
            }
        }
        // a done unit waits until its job's backlog reading changes
        self.state.done.retain(|u, at| {
            let job = u.split('#').next().unwrap_or("");
            backlog.get(job).copied().flatten().is_some_and(|b| b == *at)
        });
        // hand out: idle members by name, waiting units by priority, backlog, name, part
        let held: BTreeSet<String> = self.state.leases.values().map(|l| unit(&l.job, l.part)).collect();
        let mut waiting: Vec<(&Job, u32, u64)> = Vec::new();
        for j in &self.config.jobs {
            let Some(b) = backlog[&j.name].filter(|b| *b > 0) else { continue };
            for part in 0..j.parts {
                let u = unit(&j.name, part);
                if !held.contains(&u) && !self.state.done.contains_key(&u) {
                    waiting.push((j, part, b));
                }
            }
        }
        waiting.sort_by(|a, b| b.0.priority.cmp(&a.0.priority).then(b.2.cmp(&a.2)).then(a.0.name.cmp(&b.0.name)).then(a.1.cmp(&b.1)));
        let idle: Vec<&Member> = members
            .iter()
            .filter(|m| running(m) && !self.state.leases.contains_key(&m.name))
            .filter(|m| reports[&m.name].as_ref().is_none_or(|r| r["state"] != "stopped"))
            .collect();
        let mut give = Vec::new();
        for (m, (j, part, b)) in idle.into_iter().zip(waiting) {
            self.state.seq += 1;
            let l = Lease { job: j.name.clone(), part, lease_id: format!("{}-{}", self.config.name, self.state.seq), assigned_unix: now, expires_unix: now + self.config.lease_seconds };
            give.push((m.clone(), l, b));
        }
        for (m, l, b) in give {
            write_atomic(&m.assignment, &self.assignment(&m, &l))?;
            events.push(json!({"at_unix":now,"pool":self.config.name,"event":"assigned","member":m.name,"job":l.job,"part":l.part,"lease_id":l.lease_id,"expires_unix":l.expires_unix,"backlog":b}));
            self.state.leases.insert(m.name.clone(), l);
        }
        self.check()?;
        self.state.backlog = backlog;
        self.state.reports = reports.iter().map(|(k, r)| (k.clone(), r.as_ref().and_then(|r| r["state"].as_str()).unwrap_or("none").to_string())).collect();
        self.state.workers = members.iter().map(|m| (m.name.clone(), workers.observe(&m.worker).map(|x| x.0).unwrap_or_else(|| "unknown".into()))).collect();
        self.state.evaluated_unix = now;
        Ok(events)
    }

    /// The invariants: one lease per member (by construction), one member per unit.
    pub fn check(&self) -> Result<()> {
        let mut units = BTreeSet::new();
        for l in self.state.leases.values() {
            if !units.insert(unit(&l.job, l.part)) {
                return Err(error(format!("pool {}: two members hold {}", self.config.name, unit(&l.job, l.part))));
            }
        }
        Ok(())
    }

    /// English lines and a structured view from the last evaluation: who works on what, and which jobs wait.
    pub fn status(&self, now: u64) -> (Vec<String>, Value) {
        let mut lines = Vec::new();
        let mut members = Vec::new();
        let mut working = 0;
        for m in &self.config.members {
            let w = self.state.workers.get(&m.name).cloned().unwrap_or_else(|| "unknown".into());
            let rep = self.state.reports.get(&m.name).cloned().unwrap_or_else(|| "none".into());
            match self.state.leases.get(&m.name) {
                Some(l) => {
                    working += 1;
                    let j = self.config.jobs.iter().find(|j| j.name == l.job);
                    let part = j.filter(|j| j.parts > 1).map(|j| format!(" (part {} of {})", l.part + 1, j.parts)).unwrap_or_default();
                    lines.push(format!("{} works on {}{part}; the lease runs {} s more.", m.name, l.job, l.expires_unix.saturating_sub(now)));
                }
                None if w == "running" && rep == "stopped" => lines.push(format!("{} is stopped by a limit and waits for a person.", m.name)),
                None if w == "running" => lines.push(format!("{} is idle.", m.name)),
                None => lines.push(format!("{} is not running ({w}).", m.name)),
            }
            members.push(json!({"member":m.name,"worker":m.worker,"worker_state":w,"report":rep,"lease":self.state.leases.get(&m.name)}));
        }
        let held: BTreeSet<&str> = self.state.leases.values().map(|l| l.job.as_str()).collect();
        let mut jobs = Vec::new();
        for j in &self.config.jobs {
            let b = self.state.backlog.get(&j.name).copied().flatten();
            if !held.contains(j.name.as_str()) {
                match b {
                    Some(n) if n > 0 => lines.push(format!("{} waits with a backlog of {n}.", j.name)),
                    Some(_) => lines.push(format!("{} has no work left.", j.name)),
                    None => lines.push(format!("{} has an unknown backlog.", j.name)),
                }
            }
            jobs.push(json!({"job":j.name,"priority":j.priority,"parts":j.parts,"backlog":b,"licence":j.licence,"held_by":self.state.leases.iter().filter(|(_, l)| l.job == j.name).map(|(m, _)| m).collect::<Vec<_>>()}));
        }
        lines.insert(0, format!("The {} pool: {working} of {} {} work.", self.config.name, self.config.members.len(), if self.config.members.len() == 1 { "member is at" } else { "members are at" }));
        (lines, json!({"pool":self.config.name,"kind":self.config.kind,"evaluated_unix":self.state.evaluated_unix,"members":members,"jobs":jobs}))
    }
}

/// The pools of a flow configuration with their persisted state.
pub struct Pools {
    pub pools: Vec<Pool>,
    path: PathBuf,
    journal: PathBuf,
}

impl Pools {
    pub fn open(config: &[PoolConfig], dir: &Path) -> Result<Pools> {
        let path = dir.join("pools.json");
        let mut saved: BTreeMap<String, PoolState> = match fs::read(&path) {
            Ok(b) => serde_json::from_slice(&b)?,
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => BTreeMap::new(),
            Err(e) => return Err(e.into()),
        };
        let pools = config.iter().map(|c| Pool { config: c.clone(), state: saved.remove(&c.name).unwrap_or_default() }).collect();
        Ok(Pools { pools, path, journal: dir.join("pools.jsonl") })
    }
    pub fn journal_path(&self) -> &Path {
        &self.journal
    }
    pub fn tick(&mut self, now: u64, workers: &dyn WorkerControl) -> Result<Vec<Value>> {
        let mut all = Vec::new();
        for p in &mut self.pools {
            for e in p.tick(now, workers)? {
                flow::append(&self.journal, &e)?;
                all.push(e);
            }
        }
        let state: BTreeMap<&str, &PoolState> = self.pools.iter().map(|p| (p.config.name.as_str(), &p.state)).collect();
        write_atomic(&self.path, &serde_json::to_value(state)?)?;
        Ok(all)
    }
    pub fn kinds(&self) -> Vec<String> {
        self.pools.iter().map(|p| p.config.kind.clone()).collect()
    }
    /// Lines and views of the pools of a kind, or of all.
    pub fn status(&self, group: Option<&str>, now: u64) -> (Vec<String>, Vec<Value>) {
        let mut lines = Vec::new();
        let mut views = Vec::new();
        for p in self.pools.iter().filter(|p| group.is_none_or(|g| g == p.config.kind)) {
            let (l, v) = p.status(now);
            lines.extend(l);
            views.push(v);
        }
        (lines, views)
    }
    /// Backlog per pool for the capacity review: the known backlog of the jobs and how many members hold a lease.
    pub fn capacity(&self) -> Value {
        json!(self.pools.iter().map(|p| json!({"pool":p.config.name,"kind":p.config.kind,
            "backlog_total":p.state.backlog.values().flatten().sum::<u64>(),"unknown_backlogs":p.state.backlog.values().filter(|b| b.is_none()).count(),
            "members":p.config.members.len(),"leases":p.state.leases.len()})).collect::<Vec<_>>())
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::flow::tests::{fx, Fake};

    fn cfg(d: &Path, parts: u32) -> PoolConfig {
        serde_json::from_value(json!({"name":"dicts","kind":"nautilus","lease_seconds":300,
            "members":[{"name":"n3","worker":"n3-work","assignment":d.join("n3/assignment.json"),"report":d.join("n3/report.json")},
                       {"name":"n4","worker":"n4-work","assignment":d.join("n4/assignment.json"),"report":d.join("n4/report.json")}],
            "jobs":[{"name":"emode","priority":5,"parts":parts,"licence":"ours","backlog":{"kind":"json_field","path":d.join("emode.json"),"field":"open_items","missing":1},"params":{"layer":"emode"}},
                    {"name":"mystic","priority":1,"backlog":{"kind":"json_field","path":d.join("mystic.json"),"field":"open_items"},"params":{"layer":"mystic"}}]}))
        .unwrap()
    }
    fn workers(up: &[&str]) -> Fake {
        let mut f = Fake::default();
        for w in ["n3-work", "n4-work"] {
            f.states.insert(w.into(), (if up.contains(&w) { "running" } else { "stopped" }.into(), false));
        }
        f
    }
    fn assigned(d: &Path, m: &str) -> Value {
        serde_json::from_slice(&fs::read(d.join(format!("{m}/assignment.json"))).unwrap()).unwrap()
    }
    fn report(d: &Path, m: &str, lease: &str, state: &str, unix: u64) {
        fs::create_dir_all(d.join(m)).unwrap();
        fs::write(d.join(format!("{m}/report.json")), json!({"lease_id":lease,"state":state,"unix":unix}).to_string()).unwrap();
    }
    #[test]
    fn one_member_per_job_and_priority_first() {
        let f = fx();
        fs::write(f.0.join("emode.json"), r#"{"open_items":40}"#).unwrap();
        fs::write(f.0.join("mystic.json"), r#"{"open_items":7}"#).unwrap();
        let c = cfg(&f.0, 1);
        c.validate(&["n3-work".into(), "n4-work".into()]).unwrap();
        let mut p = Pool { config: c, state: PoolState::default() };
        let w = workers(&["n3-work", "n4-work"]);
        let e = p.tick(100, &w).unwrap();
        assert_eq!(e.len(), 2);
        // the higher priority to the first idle member; the second job to the second; never both on one job
        assert_eq!((assigned(&f.0, "n3")["job"].as_str(), assigned(&f.0, "n4")["job"].as_str()), (Some("emode"), Some("mystic")));
        assert_eq!(assigned(&f.0, "n3")["params"]["layer"], "emode");
        // a second tick hands out nothing more: both jobs are held
        assert!(p.tick(110, &w).unwrap().is_empty());
        // only one job with work: the other member stays idle (no double assignment)
        let f2 = fx();
        fs::write(f2.0.join("mystic.json"), r#"{"open_items":0}"#).unwrap();
        fs::write(f2.0.join("emode.json"), r#"{"open_items":40}"#).unwrap();
        let mut p2 = Pool { config: cfg(&f2.0, 1), state: PoolState::default() };
        p2.tick(100, &w).unwrap();
        p2.tick(110, &w).unwrap();
        assert_eq!(p2.state.leases.len(), 1, "the idle member never takes a held job");
        assert!(!f2.0.join("n4/assignment.json").exists());
        // a splittable job takes both members, one part each
        let mut p3 = Pool { config: cfg(&f2.0, 2), state: PoolState::default() };
        p3.tick(100, &w).unwrap();
        let parts: BTreeSet<u64> = ["n3", "n4"].iter().map(|m| assigned(&f2.0, m)["part"].as_u64().unwrap()).collect();
        assert_eq!(parts, BTreeSet::from([0, 1]));
        // the check refuses two members on one unit
        let mut bad = p2.state.clone();
        bad.leases.insert("n4".into(), bad.leases["n3"].clone());
        assert!(Pool { config: cfg(&f2.0, 1), state: bad }.check().is_err());
        let (lines, _) = p.status(120);
        assert!(lines.iter().any(|l| l.starts_with("n3 works on emode")), "{lines:?}");
    }
    #[test]
    fn a_lease_expires_and_the_job_moves() {
        let f = fx();
        fs::write(f.0.join("emode.json"), r#"{"open_items":40}"#).unwrap();
        fs::write(f.0.join("mystic.json"), r#"{"open_items":0}"#).unwrap();
        let mut p = Pool { config: cfg(&f.0, 1), state: PoolState::default() };
        let mut w = workers(&["n3-work"]);
        p.tick(100, &w).unwrap();
        let lease = assigned(&f.0, "n3")["lease_id"].as_str().unwrap().to_string();
        assert_eq!(assigned(&f.0, "n3")["expires_unix"], 400);
        // a fresh report renews the lease
        report(&f.0, "n3", &lease, "working", 350);
        p.tick(360, &w).unwrap();
        assert_eq!(assigned(&f.0, "n3")["expires_unix"], 660);
        // the member's worker dies; n4 starts: the job stays with n3 until the lease expires
        w.states.insert("n3-work".into(), ("exited".into(), false));
        w.states.insert("n4-work".into(), ("running".into(), false));
        p.tick(600, &w).unwrap();
        assert!(!f.0.join("n4/assignment.json").exists(), "never two members on one job before expiry");
        let e = p.tick(660, &w).unwrap();
        assert_eq!(e[0]["event"], "released");
        assert_eq!(e[0]["reason"], "expired");
        assert_eq!(assigned(&f.0, "n3")["job"], Value::Null);
        assert_eq!(assigned(&f.0, "n4")["job"], "emode");
        // a stale report does not renew: the lease runs out
        let l4 = assigned(&f.0, "n4")["lease_id"].as_str().unwrap().to_string();
        report(&f.0, "n4", &l4, "working", 100);
        // within its first period a lease is renewed; after it, only with a fresh report
        p.tick(950, &w).unwrap();
        assert_eq!(assigned(&f.0, "n4")["expires_unix"], 1250);
        p.tick(1000, &w).unwrap();
        assert_eq!(assigned(&f.0, "n4")["expires_unix"], 1250, "a stale report does not renew");
        let e = p.tick(1250, &w).unwrap();
        assert_eq!((e[0]["event"].as_str(), e[0]["reason"].as_str()), (Some("released"), Some("expired")));
    }
    #[test]
    fn finished_and_stopped_end_the_lease() {
        let f = fx();
        fs::write(f.0.join("emode.json"), r#"{"open_items":40}"#).unwrap();
        fs::write(f.0.join("mystic.json"), r#"{"open_items":3}"#).unwrap();
        let mut p = Pool { config: cfg(&f.0, 1), state: PoolState::default() };
        let w = workers(&["n3-work"]);
        p.tick(100, &w).unwrap();
        let l = assigned(&f.0, "n3")["lease_id"].as_str().unwrap().to_string();
        // finished: released, and the job is not handed out again while its backlog reading is unchanged
        report(&f.0, "n3", &l, "finished", 150);
        p.tick(160, &w).unwrap();
        assert_eq!(assigned(&f.0, "n3")["job"], "mystic");
        let l = assigned(&f.0, "n3")["lease_id"].as_str().unwrap().to_string();
        // stopped by a limit: released, and the member takes nothing while its report says stopped
        report(&f.0, "n3", &l, "stopped", 170);
        let e = p.tick(180, &w).unwrap();
        assert_eq!(e.iter().filter(|e| e["event"] == "assigned").count(), 0, "{e:?}");
        assert!(p.state.leases.is_empty());
        let j: Value = serde_json::from_slice(&fs::read(f.0.join("n3/assignment.json")).unwrap()).unwrap();
        assert_eq!(j["released"], l.as_str());
    }
    #[test]
    fn the_flow_shows_and_journals_the_pool() {
        let f = fx();
        fs::write(f.0.join("emode.json"), r#"{"open_items":40}"#).unwrap();
        fs::write(f.0.join("mystic.json"), r#"{"open_items":0}"#).unwrap();
        let c = flow::FlowConfig { interval_seconds: 60, probes: vec![], pools: vec![cfg(&f.0, 1)] };
        c.validate(&["n3-work".into(), "n4-work".into()]).unwrap();
        // a worker in two pools, or an unconfigured worker, is refused
        let twice = flow::FlowConfig { pools: vec![cfg(&f.0, 1), PoolConfig { name: "other".into(), ..cfg(&f.0, 1) }], ..c.clone() };
        assert!(twice.validate(&["n3-work".into(), "n4-work".into()]).is_err());
        assert!(c.validate(&["n3-work".into()]).is_err());
        let mut fl = flow::Flow::open(c, &f.0).unwrap();
        let mut w = workers(&["n3-work", "n4-work"]);
        let e = fl.tick(100, &mut w).unwrap();
        assert_eq!(e.iter().filter(|e| e["event"] == "assigned").count(), 1);
        let st = fl.status(Some("nautilus"), 110);
        assert_eq!(st["summary"], "The dicts pool: 1 of 2 members are at work.");
        let lines: Vec<&str> = st["lines"].as_array().unwrap().iter().filter_map(|l| l.as_str()).collect();
        assert!(lines.contains(&"n3 works on emode; the lease runs 290 s more.") && lines.contains(&"n4 is idle.") && lines.contains(&"mystic has no work left."), "{lines:?}");
        assert!(fl.kinds().contains(&"nautilus".to_string()));
        assert_eq!(fl.capacity(110)["pools"][0]["backlog_total"], 40);
        // every assignment is journaled; the state survives a reopen
        let j = fs::read_to_string(f.0.join("flow/pools.jsonl")).unwrap();
        assert!(j.contains("\"assigned\"") && j.contains("\"lease_id\":\"dicts-1\""), "{j}");
        let again = flow::Flow::open(fl.config.clone(), &f.0).unwrap();
        assert_eq!(again.pools.pools[0].state.leases["n3"].lease_id, "dicts-1");
    }
}
