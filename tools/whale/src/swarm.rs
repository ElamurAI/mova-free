//! Durable multi-lane Whale scheduling by measured worker milliseconds, not job counts.
//! Lanes, shares, child roles and candidate discovery are configured; nothing is domain-specific.
use crate::{
    util::{absolute, alive, create, error, file_hash, hash, label, lease, load, now_ms, operation, pid_identity, pinned, private_dir, read_bounded, save, storage_available, storage_floor, valid_hash},
    Result,
};
use rusqlite::{params, Connection, OpenFlags, OptionalExtension, TransactionBehavior};
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    fs::{File, OpenOptions},
    os::{
        fd::AsRawFd,
        unix::{fs::OpenOptionsExt, process::CommandExt},
    },
    path::{Path, PathBuf},
    process::{Child, Command, Stdio},
    thread,
    time::{Duration, Instant},
};
const MAX: usize = 2 << 20;
const QUEUE_MAX: usize = 128 << 20;

/// Child role a lane launches through the same frozen executable.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Role {
    /// One-item Whale run (`whale CONFIG --once`): one teacher call through the pinned spec.
    Survey,
    /// Bounded agent session in an isolated worktree (`whale-research CONFIG`).
    Agent,
}
#[derive(Clone, Debug)]
struct Lane {
    name: String,
    ppm: i64,
    role: Role,
    child: Value,
    overrides: Vec<String>,
}
struct Context {
    raw: Value,
    root: PathBuf,
    exe: PathBuf,
    executable_sha256: String,
    workers: usize,
    estimate: i64,
    poll: u64,
    lanes: Vec<Lane>,
}
fn lanes(c: &Value) -> Result<Vec<Lane>> {
    let a = c["lanes"].as_array().ok_or("swarm requires lanes")?;
    if a.is_empty() || a.len() > 8 {
        return Err(error("swarm needs 1..8 lanes"));
    }
    let mut out = Vec::<Lane>::new();
    for l in a {
        let name = l["name"].as_str().filter(|s| label(s)).ok_or("lane name must be a label")?;
        let share = l["share"].as_f64().ok_or("lane share required")?;
        if !share.is_finite() || !(0.0..=1.0).contains(&share) || (a.len() > 1 && !(0.01..=0.99).contains(&share)) {
            return Err(error("lane share must be finite; with several lanes strictly between 0.01 and 0.99"));
        }
        let role = match l["role"].as_str() {
            Some("survey") => Role::Survey,
            Some("agent") => Role::Agent,
            _ => return Err(error("lane role must be survey or agent")),
        };
        if !l["child_config"].is_object() {
            return Err(error("lane child_config must be an object"));
        }
        let overrides: Vec<String> = serde_json::from_value(l.get("payload_overrides").cloned().unwrap_or(json!([])))?;
        if out.iter().any(|x| x.name == name) {
            return Err(error("duplicate lane name"));
        }
        out.push(Lane { name: name.into(), ppm: (share * 1e6).round() as i64, role, child: l["child_config"].clone(), overrides });
    }
    if out.iter().map(|l| l.ppm).sum::<i64>() != 1_000_000 {
        return Err(error("lane shares must sum to one"));
    }
    Ok(out)
}
impl Context {
    fn new(c: &Value) -> Result<Self> {
        let root = absolute(c, "run_root")?;
        let exe = absolute(c, "executable")?;
        let workers = c["workers"].as_u64().unwrap_or(4) as usize;
        if !(2..=8).contains(&workers) {
            return Err(error("swarm workers must be between 2 and 8"));
        }
        let estimate = c["estimated_job_ms"].as_i64().unwrap_or(120_000);
        if !(100..=3_600_000).contains(&estimate) {
            return Err(error("estimated_job_ms outside admitted bounds"));
        }
        storage_floor(c)?;
        let lanes = lanes(c)?;
        private_dir(&root)?;
        for dir in ["jobs", "worktrees", "targets"] {
            private_dir(&root.join(dir))?;
        }
        Ok(Self { raw: c.clone(), root, executable_sha256: file_hash(&exe)?, exe, workers, estimate, poll: c["poll_seconds"].as_u64().unwrap_or(5).clamp(1, 60), lanes })
    }
    fn lane(&self, name: &str) -> Result<&Lane> {
        self.lanes.iter().find(|l| l.name == name).ok_or_else(|| error(format!("unknown lane {name}")))
    }
}
fn db(c: &Context) -> Result<Connection> {
    let d = Connection::open(c.root.join("swarm.sqlite3"))?;
    d.busy_timeout(Duration::from_secs(5))?;
    d.execute_batch("PRAGMA journal_mode=WAL; PRAGMA synchronous=FULL; CREATE TABLE IF NOT EXISTS jobs(key TEXT PRIMARY KEY,lane TEXT NOT NULL,payload TEXT NOT NULL,state TEXT NOT NULL,started_ms INTEGER,elapsed_ms INTEGER NOT NULL DEFAULT 0,pid INTEGER,start_ticks INTEGER,result TEXT); CREATE TABLE IF NOT EXISTS meta(key TEXT PRIMARY KEY,value TEXT NOT NULL);")?;
    Ok(d)
}
fn snapshot(c: &Context, d: &Connection) -> Result<Value> {
    let mut states = BTreeMap::<String, i64>::new();
    let mut q = d.prepare("SELECT lane||':'||state,count(*) FROM jobs GROUP BY lane,state")?;
    for r in q.query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)?)))? {
        let (k, v) = r?;
        states.insert(k, v);
    }
    let mut spent = BTreeMap::<String, i64>::new();
    let mut q = d.prepare("SELECT lane,coalesce(sum(elapsed_ms),0) FROM jobs WHERE state NOT IN ('pending','reserved','running') GROUP BY lane")?;
    for r in q.query_map([], |r| Ok((r.get::<_, String>(0)?, r.get::<_, i64>(1)?)))? {
        let (k, v) = r?;
        spent.insert(k, v);
    }
    let total: i64 = c.lanes.iter().map(|l| spent.get(&l.name).copied().unwrap_or(0)).sum();
    let mut lanes = BTreeMap::new();
    for l in &c.lanes {
        let ms = spent.get(&l.name).copied().unwrap_or(0);
        lanes.insert(l.name.clone(), json!({"target_fraction":l.ppm as f64/1e6,"charged_worker_ms":ms,"actual_fraction":if total>0{Some(ms as f64/total as f64)}else{None}}));
    }
    let unknown: i64 = d.query_row("SELECT count(*) FROM jobs WHERE state='uncertain' AND elapsed_ms=0 AND started_ms IS NOT NULL", [], |r| r.get(0))?;
    Ok(json!({"schema":2,"pid":std::process::id(),"observed_ms":now_ms(),"workers":c.workers,"lanes":lanes,"unknown_worker_time_jobs":unknown,"ratio_scope":"terminal measured jobs; unfinished and lost measurements are not zero-duration work","states":states,"paused":c.root.join("PAUSE").exists(),"stop_requested":c.root.join("STOP").exists(),"accounting":"actual elapsed child worker milliseconds; in-flight estimates guide admission only","ratio_guarantee":"long-run scheduling target, not an instantaneous time guarantee","automatic_uncertain_retry":false}))
}
/// Observe durable state without launching workers.
pub fn status(config: &Value) -> Result<Value> {
    let c = Context::new(config)?;
    let d = db(&c)?;
    let mut out = snapshot(&c, &d)?;
    if c.root.join("status.json").exists() {
        out["actor"] = load(&c.root.join("status.json"), MAX)?;
    }
    if c.root.join("start.json").exists() {
        let start = load(&c.root.join("start.json"), MAX)?;
        out["actor_alive"] = json!(alive(&start));
        out["start"] = start;
    }
    Ok(out)
}
fn insert_candidates(c: &Context, d: &mut Connection, v: &Value) -> Result<usize> {
    let tx = d.transaction_with_behavior(TransactionBehavior::Immediate)?;
    let mut added = 0;
    for p in v["candidates"].as_array().ok_or("discovery omitted candidates")?.iter().take(256) {
        let key = p["key"].as_str().ok_or("candidate key missing")?;
        if !valid_hash(key) {
            return Err(error("invalid candidate key"));
        }
        let lane = c.lane(p["lane"].as_str().ok_or("candidate lane missing")?)?;
        if !p["payload"].is_object() {
            return Err(error("candidate payload must be an object"));
        }
        let bytes = serde_json::to_string(&p["payload"])?;
        if bytes.len() > MAX {
            return Err(error("candidate exceeds metadata bound"));
        }
        added += tx.execute("INSERT OR IGNORE INTO jobs(key,lane,payload,state)VALUES(?1,?2,?3,'pending')", params![key, lane.name, bytes])?;
    }
    if let Some(cursor) = v["next_cursor"].as_u64() {
        tx.execute("INSERT OR REPLACE INTO meta VALUES('cursor',?1)", params![cursor.to_string()])?;
    }
    tx.commit()?;
    Ok(added)
}
fn pending_capacity(c: &Context) -> usize {
    c.workers * c.raw["pending_per_worker"].as_u64().unwrap_or(4).clamp(2, 16) as usize
}
fn discovery_room(c: &Context, d: &Connection) -> Result<usize> {
    let pending: usize = d.query_row("SELECT count(*) FROM jobs WHERE state='pending'", [], |r| r.get(0))?;
    Ok(pending_capacity(c).saturating_sub(pending))
}
fn insert_bounded(c: &Context, d: &mut Connection, found: &Value) -> Result<usize> {
    let count = found["candidates"].as_array().ok_or("missing candidates")?.len();
    if count > discovery_room(c, d)? {
        return Err(error("discovery exceeded remaining pending capacity; cursor not advanced"));
    }
    insert_candidates(c, d, found)
}
fn cursor(d: &Connection) -> Result<u64> {
    let v: Option<String> = d.query_row("SELECT value FROM meta WHERE key='cursor'", [], |r| r.get(0)).optional()?;
    Ok(v.and_then(|s| s.parse().ok()).unwrap_or(0))
}
fn discovery_input(c: &Context, d: &Connection) -> Result<Value> {
    let mut raw = c.raw.clone();
    let mut q = d.prepare("SELECT key FROM jobs WHERE state IN ('pending','reserved','running') ORDER BY key LIMIT ?1")?;
    let keys = q.query_map([pending_capacity(c) + c.workers], |r| r.get::<_, String>(0))?.collect::<std::result::Result<Vec<_>, _>>()?;
    raw["exclude_keys"] = json!(keys);
    raw["known_jobs_db"] = json!(c.root.join("swarm.sqlite3"));
    raw["cursor"] = json!(cursor(d)?);
    raw["discovery_limit"] = json!((c.raw["discovery_limit"].as_u64().unwrap_or(32).clamp(1, 256) as usize).min(discovery_room(c, d)?));
    Ok(raw)
}
/// Candidate source. Output: `{"candidates":[{"key","lane","payload"}],"next_cursor":N}`.
pub trait Discovery {
    fn discover(&self, input: &Value) -> Result<Value>;
}
impl<F: Fn(&Value) -> Result<Value>> Discovery for F {
    fn discover(&self, input: &Value) -> Result<Value> {
        self(input)
    }
}
/// Default discovery: a pinned immutable JSONL candidate queue (`candidates {path, sha256}`)
/// consumed in order from the durable cursor. Rows already known to the swarm are skipped
/// by the database key; the queue is never rewritten.
pub struct QueueFileDiscovery;
impl Discovery for QueueFileDiscovery {
    fn discover(&self, input: &Value) -> Result<Value> {
        let (_, bytes) = pinned(&input["candidates"], None, QUEUE_MAX).map_err(|e| error(format!("candidate queue pin: {e}")))?;
        let start = input["cursor"].as_u64().unwrap_or(0) as usize;
        let limit = input["discovery_limit"].as_u64().unwrap_or(0) as usize;
        let known = match input["known_jobs_db"].as_str() {
            Some(p) if Path::new(p).exists() => Some(Connection::open_with_flags(p, OpenFlags::SQLITE_OPEN_READ_ONLY)?),
            _ => None,
        };
        let lines: Vec<&[u8]> = bytes.split(|b| *b == b'\n').filter(|l| !l.is_empty()).collect();
        let mut out = Vec::new();
        let mut next = start.min(lines.len());
        while next < lines.len() && out.len() < limit {
            let v: Value = serde_json::from_slice(lines[next])?;
            next += 1;
            let key = v["key"].as_str().ok_or("queue row key missing")?;
            if !valid_hash(key) || !v["lane"].is_string() || !v["payload"].is_object() {
                return Err(error("queue rows require hex key, lane and object payload"));
            }
            if let Some(db) = &known {
                if db.query_row("SELECT 1 FROM jobs WHERE key=?1", [key], |_| Ok(())).optional()?.is_some() {
                    continue;
                }
            }
            out.push(v);
        }
        Ok(json!({"candidates":out,"next_cursor":next,"queue_rows":lines.len(),"queue_exhausted":next>=lines.len()}))
    }
}
/// Preview the default discovery for a swarm config; no database write or launch.
pub fn discover_preview(config: &Value) -> Result<Value> {
    let c = Context::new(config)?;
    let d = db(&c)?;
    QueueFileDiscovery.discover(&discovery_input(&c, &d)?)
}
/// Underserved lane by largest share deficit of effective time; ties keep configured order.
fn select_lane<'a>(c: &'a Context, d: &Connection) -> Result<Option<&'a Lane>> {
    let mut effective = Vec::new();
    for l in &c.lanes {
        let pending: i64 = d.query_row("SELECT count(*) FROM jobs WHERE lane=?1 AND state='pending'", [&l.name], |r| r.get(0))?;
        let ms: i64 = d.query_row("SELECT coalesce(sum(CASE WHEN state IN ('reserved','running') OR (state='uncertain' AND elapsed_ms=0 AND started_ms IS NOT NULL) THEN ?2 WHEN state='pending' THEN 0 ELSE elapsed_ms END),0) FROM jobs WHERE lane=?1", params![l.name, c.estimate], |r| r.get(0))?;
        effective.push((l, pending, ms as i128));
    }
    let total: i128 = effective.iter().map(|x| x.2).sum();
    let mut best: Option<(&Lane, i128)> = None;
    for (l, pending, ms) in effective {
        if pending == 0 {
            continue;
        }
        let deficit = total * l.ppm as i128 - ms * 1_000_000;
        if best.is_none_or(|b| deficit > b.1) {
            best = Some((l, deficit));
        }
    }
    Ok(best.map(|b| b.0))
}
struct Active {
    key: String,
    lane: String,
    role: Role,
    child: Child,
    started: Instant,
    root: PathBuf,
}
fn child_config(c: &Context, key: &str, lane: &Lane, payload: &Value) -> Result<(PathBuf, PathBuf, &'static str)> {
    let dir = c.root.join("jobs").join(key);
    private_dir(&dir)?;
    let mut cfg = lane.child.clone();
    match lane.role {
        Role::Survey => {
            cfg["run_root"] = json!(dir.join("survey"));
            cfg["workers"] = json!(1);
            cfg["max_items"] = json!(1);
            cfg["items"] = json!([payload.get("item").cloned().ok_or("survey candidate requires item")?]);
            for (k, v) in payload.as_object().ok_or("payload")? {
                if k != "item" {
                    if !lane.overrides.contains(k) {
                        return Err(error(format!("payload key {k} is not an admitted override")));
                    }
                    cfg[k] = v.clone();
                }
            }
            let m = crate::whale::prepare(&cfg)?;
            let p = PathBuf::from(m["worker_configs"][0]["config"].as_str().ok_or("missing prepared worker config")?);
            Ok((p, dir, "whale"))
        }
        Role::Agent => {
            for (k, v) in payload.as_object().ok_or("agent candidate must be object")? {
                cfg[k] = v.clone();
            }
            let prefix = lane.child["branch_prefix"].as_str().ok_or("agent lane requires branch_prefix")?;
            cfg["run_root"] = json!(dir);
            cfg["worktree"] = json!(c.root.join("worktrees").join(key));
            cfg["branch"] = json!(format!("{prefix}{key}"));
            cfg["target_dir"] = json!(c.root.join("targets").join(key));
            let p = dir.join("agent-config.json");
            create(&p, &cfg)?;
            Ok((p, dir, "whale-research"))
        }
    }
}
fn git(repo: &Path, args: &[&str]) -> Result<std::process::Output> {
    Ok(Command::new("git").arg("-C").arg(repo).args(args).output()?)
}
/// Rebind an agent payload to the current clean HEAD only when every pinned file still matches.
fn refresh_agent_base(lane: &Lane, payload: &mut Value) -> Result<bool> {
    let repo = absolute(&lane.child, "repository")?;
    let head = git(&repo, &["rev-parse", "HEAD"])?;
    if !head.status.success() {
        return Err(error("cannot read current agent base"));
    }
    let clean = git(&repo, &["status", "--porcelain", "--untracked-files=no"])?;
    if !clean.status.success() {
        return Err(error("cannot verify clean agent base"));
    }
    if !clean.stdout.is_empty() {
        return Ok(false);
    }
    for field in ["source_bindings", "scope_bindings", "reference_bindings"] {
        for b in payload[field].as_array().map(Vec::as_slice).unwrap_or(&[]) {
            let rel = PathBuf::from(b["path"].as_str().ok_or("agent pin path missing")?);
            let path = if field == "reference_bindings" {
                if !rel.is_absolute() {
                    return Err(error("reference pin must be absolute"));
                }
                rel
            } else {
                if !crate::util::normal(b["path"].as_str().unwrap_or("")) {
                    return Err(error("invalid relative agent pin"));
                }
                repo.join(rel)
            };
            if !path.is_file() || file_hash(&path)? != b["sha256"].as_str().ok_or("agent pin hash missing")? {
                return Ok(false);
            }
        }
    }
    payload["base_commit"] = json!(std::str::from_utf8(&head.stdout)?.trim());
    Ok(true)
}
fn reserve(c: &Context, d: &mut Connection, lifetime: &File) -> Result<Option<Active>> {
    if file_hash(&c.exe)? != c.executable_sha256 {
        return Err(error("frozen swarm child executable changed"));
    }
    let Some(lane) = select_lane(c, d)? else {
        return Ok(None);
    };
    let tx = d.transaction_with_behavior(TransactionBehavior::Immediate)?;
    let row: Option<(String, String)> = tx.query_row("SELECT key,payload FROM jobs WHERE lane=?1 AND state='pending' ORDER BY rowid LIMIT 1", [&lane.name], |r| Ok((r.get(0)?, r.get(1)?))).optional()?;
    let Some((key, payload)) = row else {
        return Ok(None);
    };
    tx.execute("UPDATE jobs SET state='reserved',started_ms=?2 WHERE key=?1 AND state='pending'", params![key, now_ms()])?;
    tx.commit()?;
    let mut payload: Value = serde_json::from_str(&payload)?;
    if lane.role == Role::Agent && !lane.child["repository"].is_null() {
        if !refresh_agent_base(lane, &mut payload)? {
            d.execute("UPDATE jobs SET state='superseded',result=?2 WHERE key=?1", params![key, json!({"reason":"source, scope or reference pins no longer match the current clean HEAD","model_called":false}).to_string()])?;
            return Ok(None);
        }
        d.execute("UPDATE jobs SET payload=?2 WHERE key=?1", params![key, payload.to_string()])?;
    }
    let (config, root, role) = match child_config(c, &key, lane, &payload) {
        Ok(v) => v,
        Err(e) => {
            d.execute("UPDATE jobs SET state='preparation_failed',result=?2 WHERE key=?1", params![key, json!({"reason":e.to_string(),"teacher_called":false}).to_string()])?;
            return Err(e);
        }
    };
    let log = OpenOptions::new().create_new(true).write(true).mode(0o600).open(root.join("process.log"))?;
    let mut command = Command::new(&c.exe);
    command.arg(role).arg(&config);
    if lane.role == Role::Survey {
        command.arg("--once");
    }
    command.stdout(log.try_clone()?).stderr(log).stdin(Stdio::null()).env("CARGO_BUILD_JOBS", "2");
    let fd = lifetime.as_raw_fd();
    unsafe {
        command.pre_exec(move || {
            let flags = libc::fcntl(fd, libc::F_GETFD);
            if flags < 0 || libc::fcntl(fd, libc::F_SETFD, flags & !libc::FD_CLOEXEC) < 0 {
                return Err(std::io::Error::last_os_error());
            }
            Ok(())
        });
    }
    let child = match command.spawn() {
        Ok(v) => v,
        Err(e) => {
            d.execute("UPDATE jobs SET state='launch_uncertain',result=?2 WHERE key=?1", params![key, json!({"reason":e.to_string(),"automatic_retry":false}).to_string()])?;
            return Err(e.into());
        }
    };
    let identity = pid_identity(child.id());
    d.execute("UPDATE jobs SET state='running',pid=?2,start_ticks=?3 WHERE key=?1", params![key, child.id(), identity.map(|v| v.0)])?;
    create(&root.join("launch.json"), &json!({"key":key,"lane":lane.name,"role":role,"pid":child.id(),"start_ticks":identity.map(|v|v.0),"config":config,"executable":c.exe,"started_ms":now_ms()}))?;
    Ok(Some(Active { key, lane: lane.name.clone(), role: lane.role, child, started: Instant::now(), root }))
}
fn child_result(a: &Active, success: bool) -> Result<(String, Value)> {
    match a.role {
        Role::Agent => {
            let path = a.root.join("report.json");
            if path.is_file() {
                let v = load(&path, MAX)?;
                let state = v["outcome"].as_str().or_else(|| v["state"].as_str()).unwrap_or("unclassified");
                let s = if success && matches!(state, "ready_for_review" | "completed" | "no_change") {
                    "completed"
                } else if state.contains("uncertain") {
                    "uncertain"
                } else {
                    "quarantined"
                };
                return Ok((s.into(), v));
            }
        }
        Role::Survey => {
            let root = a.root.join("survey");
            let m = load(&root.join("manifest.json"), 64 << 20)?;
            let key = m["tasks"][0]["key"].as_str().ok_or("prepared survey task key")?;
            let path = root.join("jobs").join(key).join("receipt.json");
            if path.is_file() {
                let v = load(&path, MAX)?;
                if v["task_sha256"] != key {
                    return Err(error("survey receipt identity mismatch"));
                }
                let state = v["outcome"].as_str().unwrap_or("uncertain");
                let s = if success && matches!(state, "completed" | "local_only_no_remote_teacher") {
                    "completed"
                } else if state == "quarantined" {
                    "quarantined"
                } else {
                    "uncertain"
                };
                return Ok((s.into(), v));
            }
        }
    }
    Ok(("uncertain".into(), json!({"reason":"child exited without an admitted terminal receipt","success_exit":success,"automatic_retry":false})))
}
fn recover_unresolved(c: &Context, d: &Connection) -> Result<bool> {
    let count: i64 = d.query_row("SELECT count(*) FROM jobs WHERE state IN ('reserved','running','launch_uncertain')", [], |r| r.get(0))?;
    if count == 0 {
        return Ok(false);
    }
    d.execute("UPDATE jobs SET state='uncertain',result=?1 WHERE state IN ('reserved','running','launch_uncertain')", [json!({"reason":"previous manager ended with unresolved reservations; inspect PIDs and evidence","automatic_retry":false}).to_string()])?;
    if !c.root.join("PAUSE").exists() {
        create(&c.root.join("PAUSE"), &json!({"owner":"whale-swarm","reason":"unresolved prior process reservations"}))?;
    }
    Ok(true)
}
fn drive(c: &Context, once: bool, discovery: &dyn Discovery) -> Result<()> {
    let lifetime = lease(&c.root.join("manager.lock"))?;
    let mut d = db(c)?;
    let identity = json!({"config_sha256":hash(serde_json::to_vec(&c.raw)?),"executable_sha256":file_hash(&c.exe)?});
    let path = c.root.join("identity.json");
    if path.exists() {
        if load(&path, MAX)? != identity {
            return Err(error("swarm config or executable changed; use a new run root"));
        }
    } else {
        create(&path, &identity)?;
    }
    recover_unresolved(c, &d)?;
    let floor = storage_floor(&c.raw)?;
    let mut active = Vec::<Active>::new();
    let mut last_discovery: Option<Instant> = None;
    let mut discovered_once = false;
    loop {
        let mut index = 0;
        while index < active.len() {
            if let Some(exit) = active[index].child.try_wait()? {
                let a = active.remove(index);
                let elapsed = a.started.elapsed().as_millis().min(i64::MAX as u128) as i64;
                let (state, result) = child_result(&a, exit.success())?;
                create(&a.root.join("manager-receipt.json"), &json!({"key":a.key,"lane":a.lane,"state":state,"elapsed_ms":elapsed,"exit_code":exit.code(),"result":result,"automatic_retry":false}))?;
                d.execute("UPDATE jobs SET state=?2,elapsed_ms=?3,result=?4 WHERE key=?1", params![a.key, state, elapsed, result.to_string()])?;
            } else {
                index += 1;
            }
        }
        let stop = c.root.join("STOP").exists();
        if !stop && !c.root.join("PAUSE").exists() && !storage_available(&c.root, floor)? {
            create(&c.root.join("PAUSE"), &json!({"owner":"whale-swarm","reason":"storage admission floor; active children may finish"}))?;
        }
        let paused = c.root.join("PAUSE").exists();
        if !stop && !paused {
            let poll = Duration::from_secs(c.raw["discovery_poll_seconds"].as_u64().unwrap_or(60).clamp(5, 3600));
            if last_discovery.is_none_or(|t| t.elapsed() >= poll) && (!once || !discovered_once) && discovery_room(c, &d)? > 0 {
                let found = discovery.discover(&discovery_input(c, &d)?)?;
                insert_bounded(c, &mut d, &found)?;
                save(&c.root.join("discovery-status.json"), &found)?;
                last_discovery = Some(Instant::now());
                discovered_once = true;
            }
            while active.len() < c.workers {
                match reserve(c, &mut d, &lifetime) {
                    Ok(Some(a)) => active.push(a),
                    Ok(None) => break,
                    Err(e) => {
                        if !c.root.join("PAUSE").exists() {
                            create(&c.root.join("PAUSE"), &json!({"owner":"whale-swarm","reason":e.to_string()}))?;
                        }
                        break;
                    }
                }
            }
        }
        let mut state = snapshot(c, &d)?;
        state["state"] = json!(if stop && active.is_empty() {
            "stopped"
        } else if stop {
            "draining"
        } else if paused {
            "paused"
        } else {
            "running"
        });
        state["active"] = json!(active.iter().map(|a| json!({"key":a.key,"lane":a.lane,"pid":a.child.id(),"observed_elapsed_ms":a.started.elapsed().as_millis()})).collect::<Vec<_>>());
        save(&c.root.join("status.json"), &state)?;
        if stop && active.is_empty() {
            return Ok(());
        }
        if once && (discovered_once || paused) && active.is_empty() {
            let pending: i64 = d.query_row("SELECT count(*) FROM jobs WHERE state='pending'", [], |r| r.get(0))?;
            if pending == 0 || paused {
                return Ok(());
            }
        }
        thread::sleep(Duration::from_millis(if cfg!(test) { 20 } else { c.poll * 1000 }));
    }
}
/// Run persistently, or drain one bounded discovery cohort when once is true.
pub fn run_with(config: &Value, once: bool, discovery: &dyn Discovery) -> Result<()> {
    let c = Context::new(config)?;
    drive(&c, once, discovery)
}
/// Run with the default pinned candidate-queue discovery.
pub fn run(config: &Value, once: bool) -> Result<()> {
    run_with(config, once, &QueueFileDiscovery)
}
/// Start only the configured manager. Previous launch intents are observed, never replayed.
/// Crab config keys: whale_swarm_config, whale_swarm_executable.
pub fn start(crab_config: &Value) -> Result<Value> {
    let legacy = |k: &str| -> Result<PathBuf> {
        let mut v = serde_json::Map::new();
        v.insert(k.to_string(), crate::util::legacy(crab_config, k).clone());
        absolute(&Value::Object(v), k)
    };
    let path = legacy("whale_swarm_config")?;
    let exe = legacy("whale_swarm_executable")?;
    let raw = read_bounded(&path, MAX)?;
    let config: Value = serde_json::from_slice(&raw)?;
    let mut c = Context::new(&config)?;
    if c.exe.canonicalize()? != exe.canonicalize()? {
        return Err(error("swarm executable differs between Crab and manager config"));
    }
    c.exe = exe;
    let _op = operation(&c.root.join("start.lock"))?;
    let p = c.root.join("start.json");
    let binding = json!({"config_path":path,"config_sha256":hash(&raw),"executable":c.exe,"executable_sha256":file_hash(&c.exe)?});
    if p.exists() {
        let existing = load(&p, MAX)?;
        if existing["binding"] != binding {
            return Err(error("swarm launch binding changed"));
        }
        return Ok(json!({"launch":existing,"actor_alive":alive(&existing),"automatic_restart":false,"status":status(&config)?}));
    }
    if c.root.join("PAUSE").exists() || c.root.join("STOP").exists() {
        return Err(error("swarm controls prevent initial launch"));
    }
    create(&p, &json!({"state":"reserved","binding":binding,"automatic_restart":false,"at_ms":now_ms()}))?;
    let log = OpenOptions::new().create_new(true).write(true).mode(0o600).open(c.root.join("manager-process.log"))?;
    let child = Command::new(&c.exe).arg("whale-swarm").arg(&path).stdout(log.try_clone()?).stderr(log).stdin(Stdio::null()).spawn()?;
    let id = pid_identity(child.id());
    let record = json!({"state":"spawned","binding":binding,"pid":child.id(),"start_ticks":id.map(|v|v.0),"automatic_restart":false,"at_ms":now_ms()});
    save(&p, &record)?;
    thread::spawn(move || {
        let mut child = child;
        let _ = child.wait();
    });
    Ok(json!({"actor_alive":alive(&record),"launch":record,"automatic_restart":false}))
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::{
        fs,
        os::unix::fs::PermissionsExt,
        sync::atomic::{AtomicU64, Ordering},
    };
    static NEXT: AtomicU64 = AtomicU64::new(0);
    struct Fixture {
        dir: PathBuf,
        config: Value,
    }
    fn git_ok(dir: &Path, args: &[&str]) {
        assert!(Command::new("git").arg("-C").arg(dir).args(["-c", "user.name=Fixture", "-c", "user.email=fixture@example.invalid"]).args(args).status().unwrap().success());
    }
    impl Fixture {
        fn new() -> Self {
            let parent = std::env::var_os("CARGO_TARGET_DIR").map(PathBuf::from).unwrap_or_else(std::env::temp_dir);
            fs::create_dir_all(&parent).unwrap();
            let dir = parent.join(format!("whale-swarm-test-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
            let _ = fs::remove_dir_all(&dir);
            fs::create_dir(&dir).unwrap();
            fs::set_permissions(&dir, fs::Permissions::from_mode(0o700)).unwrap();
            let exe = dir.join("worker");
            fs::write(&exe, "#!/bin/sh\nroot=$(dirname \"$2\")\nprintf '%s' '{\"outcome\":\"ready_for_review\",\"elapsed_ms\":1}' > \"$root/report.json\"\n").unwrap();
            fs::set_permissions(&exe, fs::Permissions::from_mode(0o700)).unwrap();
            assert!(Command::new("git").args(["init", "-q"]).arg(&dir).status().unwrap().success());
            fs::write(dir.join("tracked.rs"), "first").unwrap();
            git_ok(&dir, &["add", "tracked.rs"]);
            git_ok(&dir, &["commit", "-q", "--no-verify", "--no-gpg-sign", "-m", "fixture"]);
            let lanes = json!([{"name":"expansion","share":0.7,"role":"survey","child_config":{}},{"name":"research","share":0.3,"role":"agent","child_config":{"repository":dir,"branch_prefix":"whale-review/"}}]);
            let config = json!({"run_root":dir.join("run"),"executable":exe,"workers":2,"estimated_job_ms":1000,"poll_seconds":1,"lanes":lanes,"storage_floor_bytes":1});
            Self { dir, config }
        }
        fn context(&self) -> Context {
            Context::new(&self.config).unwrap()
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.dir);
        }
    }
    fn candidate(ch: char, lane: &str) -> Value {
        json!({"key":ch.to_string().repeat(64),"lane":lane,"payload":{"base_commit":"0".repeat(40),"source_bindings":[],"task":"fixture only"}})
    }
    fn add(c: &Context, d: &mut Connection) -> Value {
        let x = json!({"candidates":[candidate('a',"expansion"),candidate('b',"research")],"next_cursor":9});
        insert_candidates(c, d, &x).unwrap();
        x
    }
    #[test]
    fn measured_time_not_job_counts_and_inflight_estimate() {
        let _serial = crate::serial();
        let f = Fixture::new();
        let c = f.context();
        let mut d = db(&c).unwrap();
        add(&c, &mut d);
        assert_eq!(select_lane(&c, &d).unwrap().unwrap().name, "expansion");
        d.execute("INSERT INTO jobs(key,lane,payload,state,elapsed_ms)VALUES(?1,'expansion','{}','completed',9000)", ["e".repeat(64)]).unwrap();
        assert_eq!(select_lane(&c, &d).unwrap().unwrap().name, "research");
        d.execute("INSERT INTO jobs(key,lane,payload,state,elapsed_ms)VALUES(?1,'research','{}','completed',5000)", ["f".repeat(64)]).unwrap();
        assert_eq!(select_lane(&c, &d).unwrap().unwrap().name, "expansion");
        d.execute("UPDATE jobs SET state='running' WHERE lane='expansion' AND state='pending'", []).unwrap();
        let v = snapshot(&c, &d).unwrap();
        assert_eq!(v["lanes"]["research"]["charged_worker_ms"], 5000);
        assert_eq!(v["lanes"]["expansion"]["charged_worker_ms"], 9000);
    }
    #[test]
    fn deficit_rule_matches_the_two_lane_fraction_rule() {
        let _serial = crate::serial();
        // Oracle: with two lanes, research is chosen iff research_ms < f * total (ties -> first lane).
        let f = Fixture::new();
        let c = f.context();
        for (e, r) in [(0, 0), (7000, 3000), (7000, 2999), (6999, 3000), (1, 0), (0, 1), (100, 900)] {
            let mut d = db(&c).unwrap();
            d.execute_batch("DELETE FROM jobs").unwrap();
            add(&c, &mut d);
            d.execute("INSERT INTO jobs(key,lane,payload,state,elapsed_ms)VALUES(?1,'expansion','{}','completed',?2)", params!["e".repeat(64), e]).unwrap();
            d.execute("INSERT INTO jobs(key,lane,payload,state,elapsed_ms)VALUES(?1,'research','{}','completed',?2)", params!["f".repeat(64), r]).unwrap();
            let oracle = if (r as i128) * 1_000_000 < (e + r) as i128 * 300_000 { "research" } else { "expansion" };
            assert_eq!(select_lane(&c, &d).unwrap().unwrap().name, oracle, "{e} {r}");
        }
    }
    #[test]
    fn keys_lanes_are_validated_and_cursor_persists() {
        let _serial = crate::serial();
        let f = Fixture::new();
        let c = f.context();
        let mut d = db(&c).unwrap();
        let x = add(&c, &mut d);
        assert_eq!(insert_candidates(&c, &mut d, &x).unwrap(), 0);
        let v = discovery_input(&c, &d).unwrap();
        assert_eq!(v["cursor"], 9);
        assert_eq!(v["exclude_keys"].as_array().unwrap().len(), 2);
        let mut bad = x.clone();
        bad["candidates"][1]["key"] = json!("../escape");
        assert!(insert_candidates(&c, &mut d, &bad).is_err());
        let mut bad = x;
        bad["candidates"][1]["lane"] = json!("unknown");
        assert!(insert_candidates(&c, &mut d, &bad).is_err());
    }
    #[test]
    fn queue_file_discovery_is_pinned_ordered_and_skips_known_keys() {
        let _serial = crate::serial();
        let f = Fixture::new();
        let q = f.dir.join("queue.jsonl");
        let rows = [candidate('a', "research"), candidate('b', "research"), candidate('c', "expansion")];
        fs::write(&q, rows.iter().map(|r| format!("{r}\n")).collect::<String>()).unwrap();
        let mut cfg = f.config.clone();
        cfg["candidates"] = json!({"path":q,"sha256":file_hash(&q).unwrap()});
        cfg["discovery_limit"] = json!(2);
        let c = Context::new(&cfg).unwrap();
        let mut d = db(&c).unwrap();
        insert_candidates(&c, &mut d, &json!({"candidates":[rows[0].clone()]})).unwrap();
        let out = QueueFileDiscovery.discover(&discovery_input(&c, &d).unwrap()).unwrap();
        assert_eq!(out["candidates"].as_array().unwrap().iter().map(|x| x["key"].as_str().unwrap().chars().next().unwrap()).collect::<String>(), "bc");
        assert_eq!(out["next_cursor"], 3);
        assert_eq!(QueueFileDiscovery.discover(&discovery_input(&c, &d).unwrap()).unwrap(), out);
        fs::write(&q, "changed\n").unwrap();
        assert!(QueueFileDiscovery.discover(&discovery_input(&c, &d).unwrap()).unwrap_err().to_string().contains("queue pin"));
    }
    #[test]
    fn unresolved_reservation_becomes_uncertain_and_pauses_without_replay() {
        let _serial = crate::serial();
        let f = Fixture::new();
        let c = f.context();
        let mut d = db(&c).unwrap();
        add(&c, &mut d);
        d.execute("UPDATE jobs SET state='running' WHERE lane='research'", []).unwrap();
        assert!(recover_unresolved(&c, &d).unwrap());
        assert!(c.root.join("PAUSE").is_file());
        let state: String = d.query_row("SELECT state FROM jobs WHERE lane='research'", [], |r| r.get(0)).unwrap();
        assert_eq!(state, "uncertain");
        assert!(!recover_unresolved(&c, &d).unwrap());
    }
    #[test]
    fn agent_child_runs_once_and_terminal_receipt_has_actual_time() {
        let _serial = crate::serial();
        let f = Fixture::new();
        let mut cfg = f.config.clone();
        cfg["lanes"][1]["child_config"].as_object_mut().unwrap().remove("repository");
        let c = Context::new(&cfg).unwrap();
        let found = json!({"candidates":[candidate('a',"research"),candidate('b',"research")],"next_cursor":2});
        drive(&c, true, &|_: &Value| Ok(found.clone())).unwrap();
        let d = db(&c).unwrap();
        let count: i64 = d.query_row("SELECT count(*) FROM jobs WHERE state='completed' AND elapsed_ms>0", [], |r| r.get(0)).unwrap();
        assert_eq!(count, 2);
        let report = load(&c.root.join("status.json"), MAX).unwrap();
        assert_eq!(report["states"]["research:completed"], 2);
        assert!(report["lanes"]["research"]["charged_worker_ms"].as_i64().unwrap() > 0);
        assert_eq!(report["lanes"]["research"]["actual_fraction"], 1.0);
        let launch = load(&c.root.join("jobs").join("a".repeat(64)).join("launch.json"), MAX).unwrap();
        assert_eq!(launch["role"], "whale-research");
        let child = load(&c.root.join("jobs").join("a".repeat(64)).join("agent-config.json"), MAX).unwrap();
        assert_eq!(child["branch"], format!("whale-review/{}", "a".repeat(64)));
    }
    #[test]
    fn survey_payload_overrides_are_limited_to_the_lane_list() {
        let _serial = crate::serial();
        let f = Fixture::new();
        let c = f.context();
        let lane = c.lane("expansion").unwrap();
        let e = child_config(&c, &"c".repeat(64), lane, &json!({"item":{},"store":"/elsewhere"})).unwrap_err();
        assert!(e.to_string().contains("not an admitted override"));
    }
    #[test]
    fn own_stop_prevents_discovery_or_launch() {
        let _serial = crate::serial();
        let f = Fixture::new();
        let c = f.context();
        create(&c.root.join("STOP"), &json!({"owner":"test"})).unwrap();
        drive(&c, true, &|_: &Value| -> Result<Value> { panic!("discovery must not run after STOP") }).unwrap();
        assert_eq!(load(&c.root.join("status.json"), MAX).unwrap()["state"], "stopped");
    }
    #[test]
    fn short_start_mutex_unlocks_across_duplicate_but_manager_lease_does_not() {
        let _serial = crate::serial();
        let f = Fixture::new();
        let c = f.context();
        let path = c.root.join("start.lock");
        let op = operation(&path).unwrap();
        let unrelated_copy = op.0.try_clone().unwrap();
        assert!(lease(&path).is_err());
        drop(op);
        assert!(lease(&path).is_ok());
        drop(unrelated_copy);
        let path = c.root.join("manager.lock");
        let lifetime = lease(&path).unwrap();
        let child_copy = lifetime.try_clone().unwrap();
        drop(lifetime);
        assert!(lease(&path).is_err());
        drop(child_copy);
    }
    #[test]
    fn crab_start_identity_is_pinned_and_never_restarted() {
        let _serial = crate::serial();
        let f = Fixture::new();
        let path = f.dir.join("config.json");
        create(&path, &f.config).unwrap();
        let crab = json!({"whale_swarm_config":path,"whale_swarm_executable":f.config["executable"]});
        let first = start(&crab).unwrap();
        assert!(first["launch"]["pid"].as_u64().is_some());
        assert_eq!(start(&crab).unwrap()["launch"], first["launch"]);
        let mut changed = f.config.clone();
        changed["workers"] = json!(3);
        fs::write(&path, serde_json::to_vec(&changed).unwrap()).unwrap();
        assert!(start(&crab).is_err());
    }
    #[test]
    fn child_keeps_lifetime_lease_and_parent_descriptor_stays_cloexec() {
        let _serial = crate::serial();
        let f = Fixture::new();
        fs::write(f.config["executable"].as_str().unwrap(), "#!/bin/sh\nroot=$(dirname \"$2\")\ntouch \"$root/ready\"\nwhile [ ! -f \"$root/finish\" ]; do sleep 0.01; done\nprintf '%s' '{\"outcome\":\"ready_for_review\"}' > \"$root/report.json\"\n").unwrap();
        let mut cfg = f.config.clone();
        cfg["lanes"][1]["child_config"].as_object_mut().unwrap().remove("repository");
        let c = Context::new(&cfg).unwrap();
        let mut d = db(&c).unwrap();
        insert_candidates(&c, &mut d, &json!({"candidates":[candidate('a',"research")]})).unwrap();
        let path = c.root.join("lifetime-test.lock");
        let held = lease(&path).unwrap();
        assert_ne!(unsafe { libc::fcntl(held.as_raw_fd(), libc::F_GETFD) } & libc::FD_CLOEXEC, 0);
        let mut a = reserve(&c, &mut d, &held).unwrap().unwrap();
        let deadline = Instant::now() + Duration::from_secs(5);
        while !a.root.join("ready").exists() {
            assert!(Instant::now() < deadline);
            thread::sleep(Duration::from_millis(5));
        }
        drop(held);
        assert!(lease(&path).is_err());
        fs::write(a.root.join("finish"), b"done").unwrap();
        assert!(a.child.wait().unwrap().success());
        assert!(lease(&path).is_ok());
        assert!(storage_available(&c.root, 0).unwrap());
        assert!(!storage_available(&c.root, u64::MAX).unwrap());
    }
    #[test]
    fn bounded_discovery_preserves_capacity_and_cursor() {
        let _serial = crate::serial();
        let f = Fixture::new();
        let c = f.context();
        let mut d = db(&c).unwrap();
        let candidates = (0..8).map(|i| json!({"key":format!("{i:064x}"),"lane":"research","payload":{}})).collect::<Vec<_>>();
        insert_candidates(&c, &mut d, &json!({"candidates":candidates,"next_cursor":8})).unwrap();
        assert_eq!(discovery_room(&c, &d).unwrap(), 0);
        assert!(insert_bounded(&c, &mut d, &json!({"candidates":[candidate('e',"expansion")],"next_cursor":900})).is_err());
        assert_eq!(discovery_input(&c, &d).unwrap()["cursor"], 8);
    }
    #[test]
    fn terminal_history_above_one_hundred_thousand_is_not_serialized() {
        let _serial = crate::serial();
        let f = Fixture::new();
        let c = f.context();
        let d = db(&c).unwrap();
        d.execute_batch("WITH RECURSIVE n(x) AS(SELECT 1 UNION ALL SELECT x+1 FROM n WHERE x<100001) INSERT INTO jobs(key,lane,payload,state,elapsed_ms) SELECT printf('%064x',x),'research','{}','completed',1 FROM n;").unwrap();
        let v = discovery_input(&c, &d).unwrap();
        assert_eq!(v["exclude_keys"].as_array().unwrap().len(), 0);
        assert!(serde_json::to_vec(&v).unwrap().len() < 10000);
    }
    #[test]
    fn missing_duration_estimates_affect_admission_but_not_measured_ratio() {
        let _serial = crate::serial();
        let f = Fixture::new();
        let c = f.context();
        let mut d = db(&c).unwrap();
        add(&c, &mut d);
        d.execute("INSERT INTO jobs(key,lane,payload,state,elapsed_ms)VALUES(?1,'expansion','{}','completed',2000)", ["e".repeat(64)]).unwrap();
        d.execute("INSERT INTO jobs(key,lane,payload,state,elapsed_ms,started_ms)VALUES(?1,'research','{}','uncertain',0,1)", ["f".repeat(64)]).unwrap();
        assert_eq!(select_lane(&c, &d).unwrap().unwrap().name, "expansion");
        let v = snapshot(&c, &d).unwrap();
        assert_eq!(v["lanes"]["research"]["actual_fraction"], 0.0);
        assert_eq!(v["unknown_worker_time_jobs"], 1);
    }
    #[test]
    fn stale_agent_base_rebinds_only_unchanged_clean_pins() {
        let _serial = crate::serial();
        let f = Fixture::new();
        let c = f.context();
        let lane = c.lane("research").unwrap();
        let mut v = json!({"source_bindings":[{"path":"tracked.rs","sha256":file_hash(&f.dir.join("tracked.rs")).unwrap()}]});
        assert!(refresh_agent_base(lane, &mut v).unwrap());
        let old = v["base_commit"].clone();
        fs::write(f.dir.join("other"), "unrelated").unwrap();
        git_ok(&f.dir, &["add", "other"]);
        git_ok(&f.dir, &["commit", "-q", "--no-verify", "--no-gpg-sign", "-m", "next"]);
        assert!(refresh_agent_base(lane, &mut v).unwrap());
        assert_ne!(old, v["base_commit"]);
        fs::write(f.dir.join("tracked.rs"), "changed").unwrap();
        assert!(!refresh_agent_base(lane, &mut v).unwrap());
    }
    #[test]
    fn configuration_limits_and_private_root_are_real_gates() {
        let _serial = crate::serial();
        let f = Fixture::new();
        let mut c = f.config.clone();
        c["workers"] = json!(9);
        assert!(Context::new(&c).is_err());
        c = f.config.clone();
        c["lanes"][0]["share"] = json!(0.8);
        assert!(Context::new(&c).err().unwrap().to_string().contains("sum to one"));
        c = f.config.clone();
        c["lanes"][1]["name"] = json!("expansion");
        assert!(Context::new(&c).is_err());
        c = f.config.clone();
        c["lanes"][1]["role"] = json!("shell");
        assert!(Context::new(&c).is_err());
        c = f.config.clone();
        c["run_root"] = json!(f.dir.join("public"));
        fs::create_dir(f.dir.join("public")).unwrap();
        fs::set_permissions(f.dir.join("public"), fs::Permissions::from_mode(0o755)).unwrap();
        assert!(Context::new(&c).is_err());
    }
}
