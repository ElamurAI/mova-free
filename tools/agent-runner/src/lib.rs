//! Durable, role-neutral orchestration of bounded native Mova capabilities.
//! A reservation without a terminal receipt is unresolved, not permission to replay.
use agent_mova::{ArtifactRef, FishReport, Outcome, ReportEnvelope, ReportIndex, TraceTask};
use anyhow::{Context, Result, bail, ensure};
use serde::{Deserialize, Serialize};
use std::{
    collections::{BTreeMap, BTreeSet},
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    path::{Component, Path, PathBuf},
};

const MAX_BYTES: u64 = 262_144;
const MAX_ARTIFACT: u64 = 2_097_152;
fn identifier(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 64
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"_-".contains(&b))
}
fn valid_sha(s: &str) -> bool {
    s.len() == 64
        && s.bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
}
fn normal_relative(s: &str) -> bool {
    let p = Path::new(s);
    !s.is_empty() && !p.is_absolute() && p.components().all(|c| matches!(c, Component::Normal(_)))
}
fn bytes(path: &Path, limit: u64) -> Result<Vec<u8>> {
    if limit <= MAX_BYTES {
        let out = agent_mova::read_bounded(path)?;
        ensure!(out.len() as u64 <= limit, "file exceeds bound");
        return Ok(out);
    }
    #[cfg(target_os = "linux")]
    let file = {
        use std::os::unix::fs::OpenOptionsExt;
        OpenOptions::new()
            .read(true)
            .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
            .open(path)?
    };
    #[cfg(not(target_os = "linux"))]
    let file = {
        ensure!(
            !fs::symlink_metadata(path)?.file_type().is_symlink(),
            "symlink input refused"
        );
        File::open(path)?
    };
    let m = file.metadata()?;
    ensure!(
        m.file_type().is_file() && m.len() <= limit,
        "bounded regular file required"
    );
    let mut out = Vec::new();
    file.take(limit + 1).read_to_end(&mut out)?;
    ensure!(out.len() as u64 <= limit, "file grew beyond limit");
    Ok(out)
}
fn json<T: for<'a> Deserialize<'a>>(path: &Path) -> Result<T> {
    Ok(serde_json::from_slice(&bytes(path, MAX_BYTES)?)?)
}
fn no_symlink(path: &Path, create: bool) -> Result<()> {
    ensure!(path.is_absolute(), "absolute external path required");
    let mut at = PathBuf::new();
    for c in path.components() {
        match c {
            Component::RootDir => at.push(c.as_os_str()),
            Component::Normal(x) => at.push(x),
            _ => bail!("non-normal path"),
        }
        match fs::symlink_metadata(&at) {
            Ok(m) => ensure!(!m.file_type().is_symlink(), "symlink path refused"),
            Err(e) if e.kind() == std::io::ErrorKind::NotFound && create => {
                fs::create_dir(&at)?;
            }
            Err(e) => return Err(e.into()),
        }
    }
    Ok(())
}
fn sync_dir(path: &Path) -> Result<()> {
    File::open(path)?.sync_all()?;
    Ok(())
}
/// Publish a synced inode without replacing an existing artifact. Hard links require one local filesystem.
fn publish(path: &Path, value: &impl Serialize) -> Result<()> {
    let temp = path.with_extension("pending");
    let mut f = OpenOptions::new()
        .create_new(true)
        .write(true)
        .open(&temp)?;
    f.write_all(&serde_json::to_vec_pretty(value)?)?;
    f.sync_all()?;
    fs::hard_link(&temp, path)
        .context("exclusive artifact publication failed; inspect retained partial state")?;
    sync_dir(path.parent().context("artifact parent")?)?;
    fs::remove_file(&temp)?;
    sync_dir(path.parent().unwrap())
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub schema: u32,
    pub run_root: PathBuf,
    pub queue: PathBuf,
    pub queue_sha256: String,
    pub max_jobs_per_pass: usize,
    pub poll_ms: u64,
    #[serde(default)]
    pub allow_trace: bool,
    pub input_root: Option<PathBuf>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct InputPin {
    pub path: String,
    pub sha256: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum RecallParent {
    TraceJob { job: String },
    PinnedCheckpoint { state: InputPin, report: InputPin },
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecallIntent {
    pub actor: String,
    pub subject: String,
    pub locations: Vec<String>,
    pub parent: RecallParent,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Operation {
    Trace { task: TraceTask },
    Recall { intent: RecallIntent },
    Collect { jobs: Vec<String> },
}
impl Operation {
    fn name(&self) -> &'static str {
        match self {
            Self::Trace { .. } => "trace",
            Self::Recall { .. } => "recall",
            Self::Collect { .. } => "collect",
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Job {
    pub id: String,
    pub operation: Operation,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Queue {
    pub schema: u32,
    pub jobs: Vec<Job>,
}
fn job_sha(job: &Job) -> Result<String> {
    Ok(agent_mova::digest(&serde_json::to_vec(job)?))
}
fn validate_queue(q: &Queue) -> Result<()> {
    ensure!(
        q.schema == 1 && (1..=64).contains(&q.jobs.len()),
        "schema1 and1..64 jobs required"
    );
    let mut prior = BTreeMap::new();
    for j in &q.jobs {
        ensure!(
            identifier(&j.id) && !prior.contains_key(&j.id),
            "invalid or duplicate job id"
        );
        match &j.operation {
            Operation::Trace { task } => {
                ensure!(task.id == j.id, "trace identity differs from queue job")
            }
            Operation::Recall { intent } => match &intent.parent {
                RecallParent::TraceJob { job } => ensure!(
                    prior.get(job) == Some(&"trace"),
                    "recall requires an earlier trace job"
                ),
                RecallParent::PinnedCheckpoint { state, report } => {
                    for p in [state, report] {
                        ensure!(
                            valid_sha(&p.sha256) && normal_relative(&p.path),
                            "invalid existing-checkpoint pin"
                        );
                    }
                }
            },
            Operation::Collect { jobs } => {
                ensure!(
                    (1..=32).contains(&jobs.len()),
                    "collection needs1..32 references"
                );
                let mut refs = BTreeSet::new();
                for id in jobs {
                    ensure!(
                        refs.insert(id) && prior.get(id).is_some_and(|k| *k != "collect"),
                        "collection needs unique earlier trace/recall jobs"
                    );
                }
            }
        }
        prior.insert(j.id.clone(), j.operation.name());
    }
    Ok(())
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Binding {
    schema: u32,
    config_sha256: String,
    queue_sha256: String,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Reservation {
    schema: u32,
    id: String,
    operation: String,
    job_sha256: String,
    queue_sha256: String,
    worker_pid: u32,
}
#[derive(Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
struct Receipt {
    schema: u32,
    id: String,
    operation: String,
    job_sha256: String,
    queue_sha256: String,
    completed: bool,
    native_outcome: Option<Outcome>,
    error: Option<String>,
    partial_native_effects_possible: bool,
    artifacts: BTreeMap<String, String>,
}
#[derive(Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum State {
    Pending,
    ReservedUnresolved,
    Completed,
    Failed,
    IntegrityFailure,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Observation {
    pub id: String,
    pub state: State,
    pub native_outcome: Option<Outcome>,
    pub detail: Option<String>,
}
#[derive(Debug, Serialize, Deserialize)]
pub struct Summary {
    pub schema: u32,
    pub queue_sha256: String,
    pub stopped: bool,
    pub attempted_this_pass: usize,
    pub jobs: Vec<Observation>,
    pub semantic_admission: bool,
}
fn job_dir(c: &Config, j: &Job) -> PathBuf {
    c.run_root.join("jobs").join(&j.id)
}
fn load_receipt(c: &Config, j: &Job) -> Result<Receipt> {
    let dir = job_dir(c, j);
    no_symlink(&dir, false)?;
    let r: Reservation = json(&dir.join("reservation.json"))?;
    let hash = job_sha(j)?;
    ensure!(
        r.schema == 1
            && r.id == j.id
            && r.operation == j.operation.name()
            && r.job_sha256 == hash
            && r.queue_sha256 == c.queue_sha256,
        "reservation binding differs"
    );
    let receipt: Receipt = json(&dir.join("receipt.json"))?;
    ensure!(
        receipt.schema == 1
            && receipt.id == j.id
            && receipt.operation == j.operation.name()
            && receipt.job_sha256 == hash
            && receipt.queue_sha256 == c.queue_sha256,
        "terminal receipt binding differs"
    );
    ensure!(
        receipt.completed == receipt.error.is_none()
            && receipt.partial_native_effects_possible == !receipt.completed,
        "contradictory terminal flags"
    );
    ensure!(
        receipt.completed || receipt.native_outcome.is_none(),
        "failed job cannot assert a native outcome"
    );
    ensure!(receipt.artifacts.len() <= 3, "too many receipt artifacts");
    for (p, h) in &receipt.artifacts {
        ensure!(
            matches!(
                p.as_str(),
                "result/report.json" | "result/state.json" | "result/review-queue.json"
            ) && valid_sha(h),
            "unsupported receipt artifact"
        );
        let at = dir.join(p);
        no_symlink(&at, false)?;
        ensure!(
            agent_mova::digest(&bytes(&at, MAX_ARTIFACT)?) == *h,
            "receipt artifact changed"
        );
    }
    if receipt.completed {
        ensure!(
            receipt.artifacts.contains_key("result/report.json"),
            "completed job lacks report binding"
        );
    }
    Ok(receipt)
}
fn observe(c: &Config, j: &Job) -> Observation {
    let dir = job_dir(c, j);
    let result = (|| -> Result<(State, Option<Outcome>)> {
        match fs::symlink_metadata(&dir) {
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => {
                return Ok((State::Pending, None));
            }
            Err(e) => return Err(e.into()),
            Ok(m) => ensure!(
                m.is_dir() && !m.file_type().is_symlink(),
                "job is not an owned directory"
            ),
        }
        let reservation = dir.join("reservation.json");
        let r: Reservation = json(&reservation)?;
        ensure!(
            r.schema == 1
                && r.id == j.id
                && r.operation == j.operation.name()
                && r.job_sha256 == job_sha(j)?
                && r.queue_sha256 == c.queue_sha256,
            "reservation binding differs"
        );
        if !dir.join("receipt.json").try_exists()? {
            return Ok((State::ReservedUnresolved, None));
        }
        let receipt = load_receipt(c, j)?;
        Ok((
            if receipt.completed {
                State::Completed
            } else {
                State::Failed
            },
            receipt.native_outcome,
        ))
    })();
    match result {
        Ok((state, native_outcome)) => Observation {
            id: j.id.clone(),
            state,
            native_outcome,
            detail: None,
        },
        Err(e) => Observation {
            id: j.id.clone(),
            state: State::IntegrityFailure,
            native_outcome: None,
            detail: Some(format!("{e:#}").chars().take(2048).collect()),
        },
    }
}
fn read_inputs(path: &Path) -> Result<(Config, Queue, String, Vec<u8>)> {
    no_symlink(path, false)?;
    let config_bytes = bytes(path, MAX_BYTES)?;
    let c: Config = serde_json::from_slice(&config_bytes)?;
    ensure!(
        c.schema == 1
            && valid_sha(&c.queue_sha256)
            && (1..=4).contains(&c.max_jobs_per_pass)
            && (100..=5000).contains(&c.poll_ms),
        "unsupported runner bounds"
    );
    no_symlink(&c.queue, false)?;
    let queue_bytes = bytes(&c.queue, MAX_BYTES)?;
    ensure!(
        agent_mova::digest(&queue_bytes) == c.queue_sha256,
        "queue hash mismatch"
    );
    let q: Queue = serde_json::from_slice(&queue_bytes)?;
    validate_queue(&q)?;
    ensure!(
        c.allow_trace
            || q.jobs
                .iter()
                .all(|j| !matches!(&j.operation, Operation::Trace { .. })),
        "trace capability is disabled for this run"
    );
    if let Some(root) = &c.input_root {
        no_symlink(root, false)?;
        ensure!(root.is_dir(), "input root must be a directory");
    }
    for j in &q.jobs {
        if let Operation::Recall {
            intent:
                RecallIntent {
                    parent: RecallParent::PinnedCheckpoint { .. },
                    ..
                },
        } = &j.operation
        {
            ensure!(
                c.input_root.is_some(),
                "pinned recall needs a trusted input root"
            );
        }
    }
    Ok((c, q, agent_mova::digest(&config_bytes), queue_bytes))
}
fn check_binding(c: &Config, config_hash: &str) -> Result<()> {
    let b: Binding = json(&c.run_root.join("binding.json"))?;
    ensure!(
        b.schema == 1 && b.config_sha256 == config_hash && b.queue_sha256 == c.queue_sha256,
        "run identity differs"
    );
    ensure!(
        agent_mova::digest(&bytes(&c.run_root.join("queue.json"), MAX_BYTES)?) == c.queue_sha256,
        "frozen queue changed"
    );
    Ok(())
}
/// Lease is held until drop; lock-file inode is never replaced or deleted.
pub struct Runner {
    config_path: PathBuf,
    config_hash: String,
    config: Config,
    queue: Queue,
    _lease: File,
}
impl Runner {
    pub fn poll_ms(&self) -> u64 {
        self.config.poll_ms
    }
    pub fn open(path: &Path) -> Result<Self> {
        let (c, q, hash, queue_bytes) = read_inputs(path)?;
        no_symlink(&c.run_root, true)?;
        ensure!(c.run_root.is_dir(), "run root must be a directory");
        let lock = c.run_root.join("runner.lock");
        if let Ok(m) = fs::symlink_metadata(&lock) {
            ensure!(m.file_type().is_file(), "regular lease file required");
        }
        let lease = OpenOptions::new()
            .create(true)
            .truncate(false)
            .read(true)
            .write(true)
            .open(lock)?;
        lease
            .try_lock()
            .context("another cooperating runner holds the process lease")?;
        if c.run_root.join("binding.json").try_exists()? {
            check_binding(&c, &hash)?;
        } else {
            ensure!(
                fs::read_dir(&c.run_root)?.all(|e| e.is_ok_and(|e| e.file_name() == "runner.lock")),
                "partial/unowned run initialization; inspect instead of replay"
            );
            fs::create_dir(c.run_root.join("jobs"))?;
            let mut f = OpenOptions::new()
                .create_new(true)
                .write(true)
                .open(c.run_root.join("queue.json"))?;
            f.write_all(&queue_bytes)?;
            f.sync_all()?;
            publish(
                &c.run_root.join("binding.json"),
                &Binding {
                    schema: 1,
                    config_sha256: hash.clone(),
                    queue_sha256: c.queue_sha256.clone(),
                },
            )?;
        }
        Ok(Self {
            config_path: path.to_path_buf(),
            config_hash: hash,
            config: c,
            queue: q,
            _lease: lease,
        })
    }
    fn pins(&self) -> Result<()> {
        ensure!(
            agent_mova::digest(&bytes(&self.config_path, MAX_BYTES)?) == self.config_hash,
            "configured source changed"
        );
        ensure!(
            agent_mova::digest(&bytes(&self.config.queue, MAX_BYTES)?) == self.config.queue_sha256,
            "queue source changed"
        );
        check_binding(&self.config, &self.config_hash)
    }
    fn previous(&self, id: &str) -> Result<&Job> {
        self.queue
            .jobs
            .iter()
            .find(|j| j.id == id)
            .context("unknown dependency")
    }
    fn collect_index(&self, id: &str, refs: &[String]) -> Result<ReportIndex> {
        let mut reports = Vec::new();
        for id in refs {
            let j = self.previous(id)?;
            let o = observe(&self.config, j);
            let dir = job_dir(&self.config, j);
            let(artifact,mut envelope)=match o.state {
                State::Completed=>{
                    load_receipt(&self.config,j)?;
                    let r:FishReport=json(&dir.join("result/report.json"))?;
                    ensure!(r.id==j.id,"native report identity differs");
                    ("result/report.json",ReportEnvelope{id:r.id,actor:r.actor,kind:r.capability,outcome:r.outcome,source:r.source,findings:r.findings,artifacts:vec![]})
                },
                State::Failed=>("receipt.json",ReportEnvelope{id:j.id.clone(),actor:"runner".into(),kind:"native_job_failure".into(),outcome:Outcome::Failed,source:None,findings:vec![agent_mova::Finding{code:"job_returned_error".into(),detail:"The native operation returned an error; partial effects may remain; no retry".into()}],artifacts:vec![]}),
                State::ReservedUnresolved=>("reservation.json",ReportEnvelope{id:j.id.clone(),actor:"runner".into(),kind:"unresolved_reservation".into(),outcome:Outcome::Uncertain,source:None,findings:vec![agent_mova::Finding{code:"terminal_receipt_absent".into(),detail:"Reserved without a terminal receipt; native effects are unresolved and automatic replay is forbidden".into()}],artifacts:vec![]}),
                _=>bail!("dependency pending or integrity-invalid; collection withheld"),
            };
            let p = dir.join(artifact);
            let h = agent_mova::digest(&bytes(&p, MAX_BYTES)?);
            envelope.artifacts.push(ArtifactRef {
                path: format!("jobs/{}/{artifact}", j.id),
                sha256: h,
                format: if artifact == "result/report.json" {
                    agent_mova::ArtifactFormat::NativeFishReport
                } else {
                    agent_mova::ArtifactFormat::ExternalReviewEvidence
                },
            });
            reports.push(envelope);
        }
        Ok(ReportIndex {
            schema: 1,
            id: id.into(),
            reports,
        })
    }
    fn execute(&self, j: &Job) -> Result<Option<Outcome>> {
        let dir = job_dir(&self.config, j);
        let result = dir.join("result");
        match &j.operation {
            Operation::Trace { task } => {
                let r = agent_mova::run_trace(task, &result)?;
                Ok(Some(r.outcome))
            }
            Operation::Recall { intent } => self.recall(j, intent, &result),
            Operation::Collect { jobs } => {
                let index = self.collect_index(&j.id, jobs)?;
                agent_mova::collect(&self.config.run_root, &index, &result)?;
                Ok(None)
            }
        }
    }
    fn input_pin(&self, pin: &InputPin, limit: u64) -> Result<PathBuf> {
        ensure!(
            normal_relative(&pin.path) && valid_sha(&pin.sha256),
            "invalid input pin"
        );
        let root = self
            .config
            .input_root
            .as_ref()
            .context("trusted input root required")?;
        let path = root.join(&pin.path);
        no_symlink(&path, false)?;
        ensure!(
            path.canonicalize()?.starts_with(root.canonicalize()?),
            "input escapes trusted root"
        );
        ensure!(
            agent_mova::digest(&bytes(&path, limit)?) == pin.sha256,
            "existing checkpoint input hash mismatch"
        );
        Ok(path)
    }
    fn recall(&self, j: &Job, intent: &RecallIntent, result: &Path) -> Result<Option<Outcome>> {
        let (state, report) = match &intent.parent {
            RecallParent::TraceJob { job } => {
                let parent = self.previous(job)?;
                let receipt = load_receipt(&self.config, parent)?;
                ensure!(
                    receipt.completed
                        && matches!(receipt.native_outcome, Some(Outcome::Supported))
                        && receipt.artifacts.contains_key("result/state.json"),
                    "supported trace checkpoint required"
                );
                let dir = job_dir(&self.config, parent);
                (
                    dir.join("result/state.json"),
                    dir.join("result/report.json"),
                )
            }
            RecallParent::PinnedCheckpoint { state, report } => (
                self.input_pin(state, MAX_ARTIFACT)?,
                self.input_pin(report, MAX_BYTES)?,
            ),
        };
        let state_hash = agent_mova::digest(&bytes(&state, MAX_ARTIFACT)?);
        let report_hash = agent_mova::digest(&bytes(&report, MAX_BYTES)?);
        let parent: FishReport = json(&report)?;
        ensure!(
            parent.schema == 1
                && parent.capability == "world_location_trace"
                && matches!(parent.outcome, Outcome::Supported),
            "supported native trace report required"
        );
        let task = agent_mova::RecallTask {
            schema: 1,
            id: j.id.clone(),
            actor: intent.actor.clone(),
            subject: intent.subject.clone(),
            locations: intent.locations.clone(),
            state_sha256: state_hash.clone(),
            state_fingerprint: parent.after_state_sha256,
            trace_report_sha256: report_hash.clone(),
            trace_id: parent.id,
            source_window_sha256: parent
                .source
                .context("trace source required")?
                .window_sha256,
        };
        let recalled = agent_mova::recall(&task, &state, &report);
        ensure!(
            agent_mova::digest(&bytes(&state, MAX_ARTIFACT)?) == state_hash
                && agent_mova::digest(&bytes(&report, MAX_BYTES)?) == report_hash,
            "recall inputs changed"
        );
        let recalled = recalled?;
        fs::create_dir(result)?;
        agent_mova::json_new(&result.join("report.json"), &recalled)?;
        Ok(Some(recalled.outcome))
    }
    fn perform(&self, j: &Job) -> Result<()> {
        let dir = job_dir(&self.config, j);
        fs::create_dir(&dir)?;
        publish(
            &dir.join("reservation.json"),
            &Reservation {
                schema: 1,
                id: j.id.clone(),
                operation: j.operation.name().into(),
                job_sha256: job_sha(j)?,
                queue_sha256: self.config.queue_sha256.clone(),
                worker_pid: std::process::id(),
            },
        )?;
        let result = (|| -> Result<(Option<Outcome>, BTreeMap<String, String>)> {
            let outcome = self.execute(j)?;
            let mut artifacts = BTreeMap::new();
            for e in fs::read_dir(dir.join("result"))? {
                let e = e?;
                let name = e.file_name();
                let name = name.to_str().context("output filename")?;
                ensure!(
                    matches!(name, "report.json" | "state.json" | "review-queue.json"),
                    "unknown native output artifact"
                );
                artifacts.insert(
                    format!("result/{name}"),
                    agent_mova::digest(&bytes(&e.path(), MAX_ARTIFACT)?),
                );
            }
            Ok((outcome, artifacts))
        })();
        let (completed, native_outcome, error, artifacts) = match result {
            Ok((outcome, artifacts)) => (true, outcome, None, artifacts),
            Err(e) => (
                false,
                None,
                Some(format!("{e:#}").chars().take(4096).collect()),
                BTreeMap::new(),
            ),
        };
        publish(
            &dir.join("receipt.json"),
            &Receipt {
                schema: 1,
                id: j.id.clone(),
                operation: j.operation.name().into(),
                job_sha256: job_sha(j)?,
                queue_sha256: self.config.queue_sha256.clone(),
                completed,
                native_outcome,
                error,
                partial_native_effects_possible: !completed,
                artifacts,
            },
        )
    }
    pub fn pass(&self) -> Result<Summary> {
        self.pins()?;
        let mut attempted = 0;
        for j in &self.queue.jobs {
            if stopped(&self.config)? || attempted == self.config.max_jobs_per_pass {
                break;
            }
            if observe(&self.config, j).state != State::Pending {
                continue;
            }
            self.pins()?;
            self.perform(j)?;
            attempted += 1;
        }
        summarize(&self.config, &self.queue, attempted)
    }
}
fn stopped(c: &Config) -> Result<bool> {
    match fs::symlink_metadata(c.run_root.join("STOP")) {
        Ok(_) => Ok(true),
        Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(false),
        Err(e) => Err(e.into()),
    }
}
fn summarize(c: &Config, q: &Queue, attempted: usize) -> Result<Summary> {
    Ok(Summary {
        schema: 1,
        queue_sha256: c.queue_sha256.clone(),
        stopped: stopped(c)?,
        attempted_this_pass: attempted,
        jobs: q.jobs.iter().map(|j| observe(c, j)).collect(),
        semantic_admission: false,
    })
}
/// Read-only, nontransactional snapshot. Reserved does not assert that a worker is dead.
pub fn status(path: &Path) -> Result<Summary> {
    let (c, q, hash, _) = read_inputs(path)?;
    no_symlink(&c.run_root, false)?;
    check_binding(&c, &hash)?;
    summarize(&c, &q, 0)
}

/// Prepare a read-only recall/collection queue from an existing synthetic checkpoint.
pub fn example(checkpoint_dir: &Path, new_dir: &Path) -> Result<()> {
    no_symlink(checkpoint_dir, false)?;
    let input = checkpoint_dir.canonicalize()?;
    ensure!(
        new_dir.is_absolute() && !new_dir.starts_with(&input),
        "new external directory outside checkpoint required"
    );
    no_symlink(new_dir.parent().context("example parent")?, false)?;
    let state = input.join("state.json");
    let report = input.join("report.json");
    let prepared =
        agent_mova::prepare_synthetic_recall(&report, &state, "retained-recall", "observer")?;
    let intent = RecallIntent {
        actor: prepared.actor,
        subject: prepared.subject,
        locations: prepared.locations,
        parent: RecallParent::PinnedCheckpoint {
            state: InputPin {
                path: "state.json".into(),
                sha256: prepared.state_sha256,
            },
            report: InputPin {
                path: "report.json".into(),
                sha256: prepared.trace_report_sha256,
            },
        },
    };
    let q = Queue {
        schema: 1,
        jobs: vec![
            Job {
                id: "retained-recall".into(),
                operation: Operation::Recall { intent },
            },
            Job {
                id: "retained-collection".into(),
                operation: Operation::Collect {
                    jobs: vec!["retained-recall".into()],
                },
            },
        ],
    };
    validate_queue(&q)?;
    fs::create_dir(new_dir)?;
    let root = new_dir.canonicalize()?;
    publish(&root.join("queue.json"), &q)?;
    let c = Config {
        schema: 1,
        run_root: root.join("run"),
        queue: root.join("queue.json"),
        queue_sha256: agent_mova::digest(&bytes(&root.join("queue.json"), MAX_BYTES)?),
        max_jobs_per_pass: 2,
        poll_ms: 250,
        allow_trace: false,
        input_root: Some(input),
    };
    publish(&root.join("config.json"), &c)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(0);
    struct Temp(PathBuf);
    impl Temp {
        fn new() -> Self {
            let p = std::env::temp_dir().join(format!(
                "mova-native-runner-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&p).unwrap();
            Self(p)
        }
    }
    impl Drop for Temp {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
    fn trace(id: &str, subject: &str) -> Job {
        let sentences = vec![
            agent_mova::Sentence {
                id: "episode:1".into(),
                text: "Mary went to the kitchen.".into(),
                speaker: None,
            },
            agent_mova::Sentence {
                id: "episode:2".into(),
                text: "Mary picked up the apple.".into(),
                speaker: None,
            },
            agent_mova::Sentence {
                id: "episode:3".into(),
                text: "Mary went to the garden.".into(),
                speaker: None,
            },
        ];
        Job {
            id: id.into(),
            operation: Operation::Trace {
                task: TraceTask {
                    schema: 1,
                    id: id.into(),
                    actor: "observer".into(),
                    subject: subject.into(),
                    locations: vec![
                        "garden".into(),
                        "kitchen".into(),
                        "office".into(),
                        "hallway".into(),
                    ],
                    source: agent_mova::SourceWindow {
                        document: "synthetic".into(),
                        document_sha256: None,
                        window_sha256: agent_mova::digest(&serde_json::to_vec(&sentences).unwrap()),
                        sentences,
                    },
                },
            },
        }
    }
    fn setup(q: Queue, max: usize) -> (Temp, PathBuf) {
        let temp = Temp::new();
        let qp = temp.0.join("queue.json");
        fs::write(&qp, serde_json::to_vec_pretty(&q).unwrap()).unwrap();
        let config = Config {
            schema: 1,
            run_root: temp.0.join("run"),
            queue: qp.clone(),
            queue_sha256: agent_mova::digest(&fs::read(qp).unwrap()),
            max_jobs_per_pass: max,
            poll_ms: 100,
            allow_trace: true,
            input_root: None,
        };
        let cp = temp.0.join("config.json");
        fs::write(&cp, serde_json::to_vec_pretty(&config).unwrap()).unwrap();
        (temp, cp)
    }
    #[test]
    fn supported_trace_cold_recall_collect_and_twice() {
        let base = trace("episode", "apple");
        let Operation::Trace { task } = &base.operation else {
            unreachable!()
        };
        let q = Queue {
            schema: 1,
            jobs: vec![
                base.clone(),
                trace("unknown", "Daniel"),
                Job {
                    id: "recall".into(),
                    operation: Operation::Recall {
                        intent: RecallIntent {
                            actor: "observer".into(),
                            subject: "apple".into(),
                            locations: task.locations.clone(),
                            parent: RecallParent::TraceJob {
                                job: "episode".into(),
                            },
                        },
                    },
                },
                Job {
                    id: "summary".into(),
                    operation: Operation::Collect {
                        jobs: vec!["episode".into(), "unknown".into(), "recall".into()],
                    },
                },
            ],
        };
        let (a, ca) = setup(q.clone(), 4);
        let (b, cb) = setup(q, 4);
        for cp in [&ca, &cb] {
            let runner = Runner::open(cp).unwrap();
            let s = runner.pass().unwrap();
            assert_eq!(s.attempted_this_pass, 4);
            assert!(s.jobs.iter().all(|o| o.state == State::Completed));
            assert!(matches!(s.jobs[1].native_outcome, Some(Outcome::Uncertain)));
            assert!(!s.semantic_admission);
            assert_eq!(runner.pass().unwrap().attempted_this_pass, 0);
        }
        for id in ["episode", "unknown", "recall", "summary"] {
            let rel = format!("run/jobs/{id}/result/report.json");
            assert_eq!(
                fs::read(a.0.join(&rel)).unwrap(),
                fs::read(b.0.join(&rel)).unwrap()
            );
        }
        let report: FishReport = json(&a.0.join("run/jobs/recall/result/report.json")).unwrap();
        assert_eq!(report.answer.as_deref(), Some("garden"));
        assert_eq!(report.new_frames, 0);
        assert_eq!(report.changed_weights, 0);
    }
    #[test]
    fn a_native_failure_does_not_block_an_independent_job() {
        let (_t, cp) = setup(
            Queue {
                schema: 1,
                jobs: vec![trace("bad", "two words"), trace("good", "apple")],
            },
            4,
        );
        let runner = Runner::open(&cp).unwrap();
        let s = runner.pass().unwrap();
        assert_eq!(s.attempted_this_pass, 2);
        assert_eq!(s.jobs[0].state, State::Failed);
        assert_eq!(s.jobs[1].state, State::Completed);
        assert_eq!(runner.pass().unwrap().attempted_this_pass, 0);
    }
    #[test]
    fn unresolved_reservation_is_not_replayed_after_restart() {
        let (_t, cp) = setup(
            Queue {
                schema: 1,
                jobs: vec![trace("reserved", "apple"), trace("other", "apple")],
            },
            4,
        );
        let runner = Runner::open(&cp).unwrap();
        let job = &runner.queue.jobs[0];
        let dir = job_dir(&runner.config, job);
        fs::create_dir(&dir).unwrap();
        publish(
            &dir.join("reservation.json"),
            &Reservation {
                schema: 1,
                id: job.id.clone(),
                operation: "trace".into(),
                job_sha256: job_sha(job).unwrap(),
                queue_sha256: runner.config.queue_sha256.clone(),
                worker_pid: 1,
            },
        )
        .unwrap();
        drop(runner);
        let runner = Runner::open(&cp).unwrap();
        let s = runner.pass().unwrap();
        assert_eq!(s.attempted_this_pass, 1);
        assert_eq!(s.jobs[0].state, State::ReservedUnresolved);
        assert_eq!(s.jobs[1].state, State::Completed);
        assert!(!dir.join("receipt.json").exists());
        assert!(!dir.join("result").exists());
        drop(runner);
        assert_eq!(
            Runner::open(&cp)
                .unwrap()
                .pass()
                .unwrap()
                .attempted_this_pass,
            0
        );
    }
    #[test]
    fn lease_stop_and_read_only_status() {
        let (_t, cp) = setup(
            Queue {
                schema: 1,
                jobs: vec![trace("episode", "apple")],
            },
            4,
        );
        let runner = Runner::open(&cp).unwrap();
        assert!(Runner::open(&cp).is_err());
        assert_eq!(status(&cp).unwrap().jobs[0].state, State::Pending);
        fs::write(runner.config.run_root.join("STOP"), b"operator\n").unwrap();
        let s = runner.pass().unwrap();
        assert!(s.stopped);
        assert_eq!(s.attempted_this_pass, 0);
        assert!(!job_dir(&runner.config, &runner.queue.jobs[0]).exists());
    }
    #[test]
    fn changed_queue_and_configuration_refuse_before_effects() {
        let (_t, cp) = setup(
            Queue {
                schema: 1,
                jobs: vec![trace("episode", "apple")],
            },
            4,
        );
        let runner = Runner::open(&cp).unwrap();
        OpenOptions::new()
            .append(true)
            .open(&runner.config.queue)
            .unwrap()
            .write_all(b" ")
            .unwrap();
        assert!(runner.pass().is_err());
        assert!(!job_dir(&runner.config, &runner.queue.jobs[0]).exists());
        assert!(status(&cp).is_err());
        let (_u, dp) = setup(
            Queue {
                schema: 1,
                jobs: vec![trace("episode", "apple")],
            },
            4,
        );
        let other = Runner::open(&dp).unwrap();
        OpenOptions::new()
            .append(true)
            .open(&dp)
            .unwrap()
            .write_all(b" ")
            .unwrap();
        assert!(other.pass().is_err());
    }
    #[test]
    fn tampered_receipt_artifact_stays_quarantined_while_other_work_runs() {
        let (_t, cp) = setup(
            Queue {
                schema: 1,
                jobs: vec![trace("first", "apple"), trace("second", "apple")],
            },
            1,
        );
        let runner = Runner::open(&cp).unwrap();
        assert_eq!(runner.pass().unwrap().attempted_this_pass, 1);
        let report = runner.config.run_root.join("jobs/first/result/report.json");
        fs::write(&report, b"corrupt").unwrap();
        let s = runner.pass().unwrap();
        assert_eq!(s.attempted_this_pass, 1);
        assert_eq!(s.jobs[0].state, State::IntegrityFailure);
        assert_eq!(s.jobs[1].state, State::Completed);
        assert_eq!(fs::read(report).unwrap(), b"corrupt");
    }
    #[test]
    fn queue_identity_forward_dependencies_and_partial_initialization_are_rejected() {
        let mut q = Queue {
            schema: 1,
            jobs: vec![trace("same", "apple"), trace("same", "apple")],
        };
        assert!(validate_queue(&q).is_err());
        q.jobs[1].id = "../escape".into();
        assert!(validate_queue(&q).is_err());
        q.jobs = vec![
            Job {
                id: "summary".into(),
                operation: Operation::Collect {
                    jobs: vec!["later".into()],
                },
            },
            trace("later", "apple"),
        ];
        assert!(validate_queue(&q).is_err());
        let (_t, cp) = setup(
            Queue {
                schema: 1,
                jobs: vec![trace("one", "apple")],
            },
            1,
        );
        let c: Config = json(&cp).unwrap();
        fs::create_dir(&c.run_root).unwrap();
        fs::write(c.run_root.join("unowned"), b"retain").unwrap();
        assert!(Runner::open(&cp).is_err());
        assert_eq!(fs::read(c.run_root.join("unowned")).unwrap(), b"retain");
    }
    #[test]
    fn read_only_example_reuses_existing_fixture_without_trace() {
        let (t, cp) = setup(
            Queue {
                schema: 1,
                jobs: vec![trace("fixture", "apple")],
            },
            1,
        );
        let runner = Runner::open(&cp).unwrap();
        runner.pass().unwrap();
        drop(runner);
        let checkpoint = t.0.join("run/jobs/fixture/result");
        let state = fs::read(checkpoint.join("state.json")).unwrap();
        let report = fs::read(checkpoint.join("report.json")).unwrap();
        let p = t.0.join("read-only");
        example(&checkpoint, &p).unwrap();
        assert!(!p.join("run").exists());
        let c: Config = json(&p.join("config.json")).unwrap();
        assert!(!c.allow_trace);
        let readonly = Runner::open(&p.join("config.json")).unwrap();
        let s = readonly.pass().unwrap();
        assert_eq!(s.attempted_this_pass, 2);
        assert!(s.jobs.iter().all(|o| o.state == State::Completed));
        assert_eq!(fs::read(checkpoint.join("state.json")).unwrap(), state);
        assert_eq!(fs::read(checkpoint.join("report.json")).unwrap(), report);
        assert!(example(&checkpoint, &p).is_err());
    }
    #[test]
    fn trace_defaults_off_and_shell_operations_do_not_parse() {
        let (_t, cp) = setup(
            Queue {
                schema: 1,
                jobs: vec![trace("forbidden", "apple")],
            },
            1,
        );
        let mut c: serde_json::Value = json(&cp).unwrap();
        c.as_object_mut().unwrap().remove("allow_trace");
        fs::write(&cp, serde_json::to_vec(&c).unwrap()).unwrap();
        assert!(Runner::open(&cp).is_err());
        assert!(!cp.parent().unwrap().join("run").exists());
        let q = r#"{"schema":1,"jobs":[{"id":"bad","operation":{"kind":"shell","program":"anything"}}]}"#;
        assert!(serde_json::from_str::<Queue>(q).is_err());
    }
    #[test]
    fn pinned_input_failure_is_recorded_without_fallback_and_collected() {
        let (t, cp) = setup(
            Queue {
                schema: 1,
                jobs: vec![trace("fixture", "apple")],
            },
            1,
        );
        let runner = Runner::open(&cp).unwrap();
        runner.pass().unwrap();
        drop(runner);
        let checkpoint = t.0.join("run/jobs/fixture/result");
        let p = t.0.join("readonly-bad");
        example(&checkpoint, &p).unwrap();
        fs::write(checkpoint.join("state.json"), b"changed").unwrap();
        let readonly = Runner::open(&p.join("config.json")).unwrap();
        let s = readonly.pass().unwrap();
        assert_eq!(s.attempted_this_pass, 2);
        assert_eq!(s.jobs[0].state, State::Failed);
        assert_eq!(s.jobs[1].state, State::Completed);
        assert!(!p.join("run/jobs/retained-recall/result").exists());
        assert_eq!(fs::read(checkpoint.join("state.json")).unwrap(), b"changed");
    }
    #[cfg(target_os = "linux")]
    #[test]
    fn linux_reads_refuse_fifo_and_leaf_symlink_and_stop_sees_dangling_marker() {
        use std::{ffi::CString, os::unix::fs::symlink};
        let t = Temp::new();
        let fifo = t.0.join("fifo");
        let name = CString::new(fifo.to_str().unwrap()).unwrap();
        assert_eq!(unsafe { libc::mkfifo(name.as_ptr(), 0o600) }, 0);
        assert!(bytes(&fifo, MAX_ARTIFACT).is_err());
        let target = t.0.join("regular");
        fs::write(&target, b"data").unwrap();
        let link = t.0.join("link");
        symlink(&target, &link).unwrap();
        assert!(bytes(&link, MAX_ARTIFACT).is_err());
        let (_u, cp) = setup(
            Queue {
                schema: 1,
                jobs: vec![trace("episode", "apple")],
            },
            1,
        );
        let r = Runner::open(&cp).unwrap();
        symlink("missing-target", r.config.run_root.join("STOP")).unwrap();
        let s = r.pass().unwrap();
        assert!(s.stopped);
        assert_eq!(s.attempted_this_pass, 0);
    }
}
