use anyhow::{Context as AnyhowContext, Result, ensure};
use serde::{Deserialize, Serialize};
use serde_json::{Value, json};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    fs::{self, File, OpenOptions},
    io::{Read, Write},
    os::{
        fd::AsRawFd,
        unix::fs::{OpenOptionsExt, PermissionsExt},
    },
    path::{Path, PathBuf},
    thread,
    time::{Duration, SystemTime, UNIX_EPOCH},
};
pub mod terminal;
const MAX_FILE: usize = 128 * 1024;
/// Recommended Fish terminal model (ChatGPT through Codex CLI).
pub const DEFAULT_MODEL: &str = "gpt-6.1-terra";
fn default_model() -> String {
    DEFAULT_MODEL.into()
}
/// The terminal supervisor runs Codex only: a ChatGPT `gpt-*` model id, no provider fallback.
pub fn valid_model(model: &str) -> bool {
    model.starts_with("gpt-")
        && model.len() <= 128
        && !model.chars().any(|c| c.is_whitespace() || c.is_control())
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Config {
    pub schema: u32,
    pub root: PathBuf,
    pub queue: PathBuf,
    pub queue_sha256: String,
    pub tmux: PathBuf,
    pub codex: PathBuf,
    /// Codex model for every Fish terminal; Terra is the recommended default, Sol selectable.
    #[serde(default = "default_model")]
    pub model: String,
    pub fish: Vec<Fish>,
    pub max_dispatches: usize,
    pub poll_ms: u64,
    pub task_timeout_secs: u64,
    pub idle_marker: String,
    pub pane_command: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Fish {
    pub id: String,
    pub session: String,
    pub workdir: PathBuf,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Input {
    pub path: PathBuf,
    pub sha256: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Task {
    pub id: String,
    pub fish: String,
    pub prompt: String,
    pub inputs: Vec<Input>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct Job {
    pub id: String,
    pub fish: String,
    pub task_sha256: String,
    pub nonce: String,
    pub state: String,
    pub created: u64,
    pub report_sha256: Option<String>,
    pub detail: String,
}
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Screen {
    pub command: String,
    pub dead: bool,
    pub text: String,
}
pub trait Terminal {
    fn launch(&mut self, c: &Config, f: &Fish, owner: &str) -> Result<()>;
    fn owned(&mut self, f: &Fish, owner: &str) -> Result<bool>;
    fn capture(&mut self, f: &Fish) -> Result<Screen>;
    fn load(&mut self, buffer: &str, path: &Path) -> Result<()>;
    fn paste(&mut self, f: &Fish, buffer: &str) -> Result<()>;
    fn enter(&mut self, f: &Fish) -> Result<()>;
}
pub fn hash(b: &[u8]) -> String {
    format!("{:x}", Sha256::digest(b))
}
fn now() -> u64 {
    SystemTime::now()
        .duration_since(UNIX_EPOCH)
        .unwrap_or_default()
        .as_secs()
}
fn ident(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 64
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"-_".contains(&b))
}
fn digest(s: &str) -> bool {
    s.len() == 64
        && s.bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
}
pub fn read_limited(p: &Path, n: usize) -> Result<Vec<u8>> {
    ensure!(
        !fs::symlink_metadata(p)?.file_type().is_symlink(),
        "symlink file refused"
    );
    let mut f = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NOFOLLOW | libc::O_NONBLOCK)
        .open(p)?;
    ensure!(f.metadata()?.is_file(), "regular file required");
    let mut out = Vec::new();
    Read::take(&mut f, (n + 1) as u64).read_to_end(&mut out)?;
    ensure!(out.len() <= n, "file exceeds bound");
    Ok(out)
}
fn canonical_dir(p: &Path) -> Result<PathBuf> {
    ensure!(p.is_absolute(), "absolute directory required");
    ensure!(
        !fs::symlink_metadata(p)?.file_type().is_symlink(),
        "symlink directory refused"
    );
    let q = p.canonicalize()?;
    ensure!(q == p, "canonical directory required");
    ensure!(q.is_dir(), "directory required");
    Ok(q)
}
fn private_dir(p: &Path) -> Result<()> {
    fs::create_dir(p)?;
    fs::set_permissions(p, fs::Permissions::from_mode(0o700))?;
    Ok(())
}
fn create(p: &Path, b: &[u8]) -> Result<()> {
    let mut f = OpenOptions::new()
        .create_new(true)
        .write(true)
        .mode(0o600)
        .open(p)?;
    f.write_all(b)?;
    f.sync_all()?;
    File::open(p.parent().context("parent")?)?.sync_all()?;
    Ok(())
}
fn exact_or_create(p: &Path, b: &[u8]) -> Result<()> {
    if fs::symlink_metadata(p).is_ok() {
        ensure!(read_limited(p, MAX_FILE)? == b, "existing artifact differs");
        Ok(())
    } else {
        create(p, b)
    }
}
fn save(p: &Path, v: &impl Serialize) -> Result<()> {
    let b = serde_json::to_vec_pretty(v)?;
    let tmp = p.with_extension(format!("{}.new", std::process::id()));
    create(&tmp, &b)?;
    fs::rename(&tmp, p)?;
    File::open(p.parent().context("parent")?)?.sync_all()?;
    Ok(())
}
struct Lease(File);
impl Drop for Lease {
    fn drop(&mut self) {
        unsafe {
            libc::flock(self.0.as_raw_fd(), libc::LOCK_UN);
        }
    }
}
fn lease(c: &Config) -> Result<Lease> {
    let f = OpenOptions::new()
        .create(true)
        .truncate(false)
        .read(true)
        .write(true)
        .mode(0o600)
        .custom_flags(libc::O_NOFOLLOW)
        .open(c.root.join("controller.lock"))?;
    ensure!(
        unsafe { libc::flock(f.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } == 0,
        "another Fish controller owns this run"
    );
    Ok(Lease(f))
}
pub struct Context {
    pub config: Config,
    pub owner: String,
    pub tasks: Vec<Task>,
}
impl Context {
    pub fn load(path: &Path) -> Result<Self> {
        let bytes = read_limited(path, MAX_FILE)?;
        let c: Config = serde_json::from_slice(&bytes)?;
        ensure!(
            c.schema == 1 && (1..=8).contains(&c.fish.len()),
            "schema/pool bound"
        );
        ensure!(
            (1..=1024).contains(&c.max_dispatches)
                && (100..=60000).contains(&c.poll_ms)
                && (1..=86400).contains(&c.task_timeout_secs),
            "configured resource bounds"
        );
        ensure!(
            !c.idle_marker.is_empty()
                && c.idle_marker.len() <= 128
                && !c.idle_marker.chars().any(char::is_control),
            "idle marker"
        );
        ensure!(ident(&c.pane_command), "pane command");
        ensure!(
            valid_model(&c.model),
            "Codex model id (gpt-*) required; no provider fallback"
        );
        ensure!(
            digest(&c.queue_sha256) && c.tmux.is_absolute() && c.codex.is_absolute(),
            "executable/queue pin"
        );
        ensure!(
            c.root.is_absolute()
                && c.root.parent().context("run parent")?.canonicalize()?
                    == c.root.parent().unwrap(),
            "canonical run parent"
        );
        let mut ids = BTreeSet::new();
        let mut sessions: Vec<String> = Vec::new();
        let mut dirs: Vec<PathBuf> = Vec::new();
        for f in &c.fish {
            ensure!(
                ident(&f.id)
                    && f.session == format!("peta-fish-{}", f.id)
                    && ids.insert(f.id.clone()),
                "unique bounded Fish identity/session"
            );
            ensure!(
                !sessions
                    .iter()
                    .any(|s| s.starts_with(&f.session) || f.session.starts_with(s)),
                "Fish session names must not prefix another session"
            );
            sessions.push(f.session.clone());
            let d = canonical_dir(&f.workdir)?;
            ensure!(
                !d.starts_with(&c.root) && !c.root.starts_with(&d),
                "run root and workdir must be disjoint"
            );
            for old in &dirs {
                ensure!(
                    !d.starts_with(old) && !old.starts_with(&d),
                    "overlapping Fish workdirs"
                );
            }
            dirs.push(d);
        }
        let q = read_limited(&c.queue, MAX_FILE)?;
        ensure!(hash(&q) == c.queue_sha256, "queue hash mismatch");
        let mut tasks = Vec::new();
        let mut tids = BTreeSet::new();
        for line in std::str::from_utf8(&q)?.lines() {
            if line.is_empty() {
                continue;
            }
            let t: Task = serde_json::from_str(line)?;
            ensure!(
                ident(&t.id) && tids.insert(t.id.clone()) && ids.contains(&t.fish),
                "task identity"
            );
            ensure!(
                !t.prompt.trim().is_empty()
                    && t.prompt.len() <= 12000
                    && !t
                        .prompt
                        .chars()
                        .any(|x| x.is_control() && !matches!(x, '\n' | '\t')),
                "bounded prompt"
            );
            ensure!(t.inputs.len() <= 32, "input cap");
            let f = c.fish.iter().find(|f| f.id == t.fish).unwrap();
            let mut names = BTreeSet::new();
            for i in &t.inputs {
                ensure!(
                    i.path.is_absolute()
                        && i.path.starts_with(&f.workdir)
                        && digest(&i.sha256)
                        && names.insert(i.path.clone()),
                    "confined input binding"
                );
                ensure!(i.path.canonicalize()? == i.path, "noncanonical input");
            }
            tasks.push(t);
        }
        ensure!(tasks.len() <= 1024, "queue task bound");
        let owner = hash(&bytes);
        Ok(Self {
            config: c,
            owner,
            tasks,
        })
    }
    fn job_path(&self, id: &str) -> PathBuf {
        self.config.root.join("jobs").join(format!("{id}.json"))
    }
    fn jobs(&self) -> Result<Vec<Job>> {
        let mut out = Vec::new();
        for t in &self.tasks {
            let p = self.job_path(&t.id);
            if p.exists() {
                let j: Job = serde_json::from_slice(&read_limited(&p, MAX_FILE)?)?;
                ensure!(
                    j.id == t.id && j.fish == t.fish && j.task_sha256 == task_hash(t),
                    "job binding mismatch"
                );
                out.push(j);
            }
        }
        Ok(out)
    }
    fn ready(&self) -> Result<()> {
        canonical_dir(&self.config.root)?;
        ensure!(
            read_limited(&self.config.root.join("config.sha256"), 128)? == self.owner.as_bytes(),
            "run config mismatch"
        );
        Ok(())
    }
    pub fn prepare(&self) -> Result<()> {
        private_dir(&self.config.root)?;
        for d in [
            "jobs",
            "terminals",
            "observations",
            "prompts",
            "reports",
            "review",
            "decisions",
            "tmp",
        ] {
            private_dir(&self.config.root.join(d))?;
        }
        create(
            &self.config.root.join("config.sha256"),
            self.owner.as_bytes(),
        )?;
        create(
            &self.config.root.join("queue.jsonl"),
            &read_limited(&self.config.queue, MAX_FILE)?,
        )?;
        for f in &self.config.fish {
            private_dir(&f.workdir.join(".peta-fish-reports"))?;
        }
        Ok(())
    }
    pub fn launch(&self, id: &str, term: &mut impl Terminal) -> Result<Value> {
        self.ready()?;
        let _lease = lease(&self.config)?;
        let f = self
            .config
            .fish
            .iter()
            .find(|f| f.id == id)
            .context("unknown Fish")?;
        let p = self
            .config
            .root
            .join("terminals")
            .join(format!("{id}.json"));
        ensure!(
            fs::symlink_metadata(&p).is_err(),
            "launch already reserved; inspect existing session"
        );
        let mut record =
            json!({"fish":id,"session":f.session,"owner":self.owner,"state":"launch_reserved"});
        create(&p, &serde_json::to_vec(&record)?)?;
        term.launch(&self.config, f, &self.owner)?;
        ensure!(
            term.owned(f, &self.owner)?,
            "created session ownership not established"
        );
        record["state"] = json!("launched");
        save(&p, &record)?;
        Ok(record)
    }
    fn inputs(&self, t: &Task) -> Result<()> {
        for i in &t.inputs {
            ensure!(
                hash(&read_limited(&i.path, MAX_FILE)?) == i.sha256,
                "bound input changed"
            );
        }
        Ok(())
    }
    fn report_dir(&self, f: &Fish) -> Result<PathBuf> {
        canonical_dir(&f.workdir.join(".peta-fish-reports"))
    }
    // Read-only failures belong to this Fish. Persistence failures still stop the controller.
    fn stable_idle(&self, f: &Fish, term: &mut impl Terminal) -> Result<bool> {
        let (state, screen, ready) = match self.probe_idle(f, term) {
            Ok(v) => v,
            Err(_) => ("terminal_unavailable", None, false),
        };
        self.observe(f, state, screen.as_ref())?;
        Ok(ready)
    }
    fn probe_idle(
        &self,
        f: &Fish,
        term: &mut impl Terminal,
    ) -> Result<(&'static str, Option<Screen>, bool)> {
        let p = self
            .config
            .root
            .join("terminals")
            .join(format!("{}.json", f.id));
        if !p.exists() {
            return Ok(("not_launched", None, false));
        }
        let v: Value = serde_json::from_slice(&read_limited(&p, 4096)?)?;
        if v["state"] != "launched"
            || v["owner"] != self.owner
            || v["session"] != f.session
            || v["fish"] != f.id
        {
            return Ok(("launch_ownership_unresolved", None, false));
        }
        if !term.owned(f, &self.owner)? {
            return Ok(("foreign_or_changed_session", None, false));
        }
        let a = term.capture(f)?;
        if !idle(&self.config, &a) {
            return Ok(("awaiting_idle_ui", Some(a), false));
        }
        thread::sleep(Duration::from_millis(20));
        let b = term.capture(f)?;
        let ready = a == b && idle(&self.config, &b) && term.owned(f, &self.owner)?;
        Ok((
            if ready {
                "ready"
            } else {
                "unstable_or_changed_ui"
            },
            Some(b),
            ready,
        ))
    }
    fn refuse_before_effect(&self, f: &Fish, t: &Task, e: &anyhow::Error) -> Result<()> {
        let v = json!({"fish":f.id,"session":f.session,"state":"refused_before_effect",
            "task_id":t.id,"observed_unix":now(),
            "reason_sha256":hash(format!("{e:#}").as_bytes()),"screen_content_stored":false});
        save(
            &self
                .config
                .root
                .join("observations")
                .join(format!("{}.json", f.id)),
            &v,
        )
    }
    fn observe(&self, f: &Fish, state: &str, s: Option<&Screen>) -> Result<()> {
        let v = json!({"fish":f.id,"session":f.session,"state":state,"observed_unix":now(),"screen_sha256":s.map(|s|hash(s.text.as_bytes())),"command":s.map(|s|s.command.as_str()),"screen_content_stored":false});
        save(
            &self
                .config
                .root
                .join("observations")
                .join(format!("{}.json", f.id)),
            &v,
        )
    }
    pub fn once(&self, term: &mut impl Terminal) -> Result<Value> {
        self.ready()?;
        let _lease = lease(&self.config)?;
        let mut jobs = self.jobs()?;
        for j in &mut jobs {
            let t = self.tasks.iter().find(|t| t.id == j.id).unwrap();
            if matches!(
                j.state.as_str(),
                "dispatch_reserved" | "paste_reserved" | "enter_reserved"
            ) {
                j.state = "quarantine".into();
                j.detail =
                    "Interrupted dispatch boundary: no replay; inspect terminal and report.".into();
                save(&self.job_path(&j.id), j)?;
                continue;
            }
            if j.state == "review_pending" {
                let dp = self
                    .config
                    .root
                    .join("decisions")
                    .join(format!("{}.json", j.id));
                if dp.exists() {
                    let d: Value = serde_json::from_slice(&read_limited(&dp, 8192)?)?;
                    ensure!(
                        d["task_id"] == j.id
                            && d["nonce"] == j.nonce
                            && d["report_sha256"] == json!(j.report_sha256),
                        "decision binding"
                    );
                    ensure!(
                        matches!(d["decision"].as_str(), Some("accept" | "reject")),
                        "decision value"
                    );
                    j.state = "reviewed".into();
                    j.detail = d["decision"].as_str().unwrap().into();
                    save(&self.job_path(&j.id), j)?;
                }
                continue;
            }
            if j.state != "waiting_report" {
                continue;
            }
            if self.inputs(t).is_err() {
                j.state = "quarantine".into();
                j.detail = "Bound input changed while waiting.".into();
                save(&self.job_path(&j.id), j)?;
                continue;
            }
            let f = self.config.fish.iter().find(|f| f.id == j.fish).unwrap();
            let dir = match self.report_dir(f) {
                Ok(dir) => dir,
                Err(_) => {
                    j.state = "quarantine".into();
                    j.detail =
                        "Report directory unavailable or symlinked after dispatch; no replay."
                            .into();
                    save(&self.job_path(&j.id), j)?;
                    continue;
                }
            };
            let report = dir.join(format!("{}.json", j.nonce));
            if report.exists() && self.stable_idle(f, term)? {
                match self.admit(j, &report) {
                    Ok(()) => {}
                    Err(e) => {
                        j.state = "quarantine".into();
                        j.detail = format!("Report refused: {e:#}");
                    }
                }
                save(&self.job_path(&j.id), j)?;
            } else if now().saturating_sub(j.created) >= self.config.task_timeout_secs {
                j.state = "quarantine".into();
                j.detail = "Timed out; terminal/report effects uncertain, no retry.".into();
                save(&self.job_path(&j.id), j)?;
            }
        }
        for f in &self.config.fish {
            if jobs.iter().any(|j| j.fish == f.id && j.state != "reviewed") {
                continue;
            }
            if jobs.len() >= self.config.max_dispatches {
                break;
            }
            let Some(t) = self
                .tasks
                .iter()
                .find(|t| t.fish == f.id && !jobs.iter().any(|j| j.id == t.id))
            else {
                continue;
            };
            if !self.stable_idle(f, term)? {
                continue;
            }
            let nonce = hash(format!("{}:{}:{}", self.owner, t.id, task_hash(t)).as_bytes());
            let preflight = (|| -> Result<PathBuf> {
                self.inputs(t)?;
                let report = self.report_dir(f)?.join(format!("{nonce}.json"));
                match fs::symlink_metadata(&report) {
                    Err(e) if e.kind() == std::io::ErrorKind::NotFound => {}
                    Err(e) => return Err(e.into()),
                    Ok(_) => anyhow::bail!("report predates task reservation"),
                }
                Ok(report)
            })();
            let report = match preflight {
                Ok(report) => report,
                Err(e) => {
                    self.refuse_before_effect(f, t, &e)?;
                    continue;
                }
            };
            let mut j = Job {
                id: t.id.clone(),
                fish: f.id.clone(),
                task_sha256: task_hash(t),
                nonce: nonce.clone(),
                state: "dispatch_reserved".into(),
                created: now(),
                report_sha256: None,
                detail: "Reservation precedes all terminal effects.".into(),
            };
            create(&self.job_path(&j.id), &serde_json::to_vec_pretty(&j)?)?;
            let prompt = format!(
                "Peta Fish task. Work in this isolated directory only. Use medium effort. Do not modify source, DB, models or workers; no network, training, other model calls or external messages. Read only the bound inputs listed below. Return a review draft, not approval. Write exactly one complete JSON report by temporary-file rename to {}. Schema: {{\"schema\":1,\"task_id\":{:?},\"nonce\":{:?},\"input_sha256\":{:?},\"result\":{{...}}}}. Preserve unknowns and missing evidence. The report is the sole requested write.\nInputs: {}\nTask:\n{}\n",
                report.display(),
                t.id,
                nonce,
                j.task_sha256,
                serde_json::to_string(&t.inputs)?,
                t.prompt
            );
            let pp = self
                .config
                .root
                .join("prompts")
                .join(format!("{}.txt", t.id));
            create(&pp, prompt.as_bytes())?;
            let buffer = format!("peta-fish-{}", &nonce[..20]);
            term.load(&buffer, &pp)?;
            j.state = "paste_reserved".into();
            save(&self.job_path(&j.id), &j)?;
            ensure!(
                self.stable_idle(f, term)?,
                "terminal changed before paste; reservation retained"
            );
            term.paste(f, &buffer)?;
            j.state = "enter_reserved".into();
            save(&self.job_path(&j.id), &j)?;
            term.enter(f)?;
            j.state = "waiting_report".into();
            j.detail = "Awaiting bound report and stable idle terminal.".into();
            save(&self.job_path(&j.id), &j)?;
            jobs.push(j);
        }
        self.status()
    }
    fn admit(&self, j: &mut Job, p: &Path) -> Result<()> {
        let f = self
            .config
            .fish
            .iter()
            .find(|f| f.id == j.fish)
            .context("Fish missing")?;
        ensure!(
            p.parent() == Some(self.report_dir(f)?.as_path()),
            "report directory binding"
        );
        let bytes = read_limited(p, MAX_FILE)?;
        let v: Value = serde_json::from_slice(&bytes)?;
        ensure!(
            v["schema"] == 1
                && v["task_id"] == j.id
                && v["nonce"] == j.nonce
                && v["input_sha256"] == j.task_sha256,
            "report identity mismatch"
        );
        ensure!(v["result"].is_object(), "report result must be an object");
        let sha = hash(&bytes);
        exact_or_create(
            &self
                .config
                .root
                .join("reports")
                .join(format!("{}.json", j.id)),
            &bytes,
        )?;
        let review = json!({"schema":1,"task_id":j.id,"fish":j.fish,"nonce":j.nonce,"input_sha256":j.task_sha256,"report_sha256":sha,"config_sha256":self.owner,"status":"needs_root_review","training_allowed":false,"automatic_application":false});
        exact_or_create(
            &self
                .config
                .root
                .join("review")
                .join(format!("{}.json", j.id)),
            &serde_json::to_vec_pretty(&review)?,
        )?;
        j.report_sha256 = Some(sha);
        j.state = "review_pending".into();
        j.detail = "Durable local queue only; no automatic root-chat review.".into();
        Ok(())
    }
    pub fn status(&self) -> Result<Value> {
        self.ready()?;
        let jobs = self.jobs()?;
        let fish:Vec<_>=self.config.fish.iter().map(|f|{
   let active=jobs.iter().find(|j|j.fish==f.id&&j.state!="reviewed");
   let p=self.config.root.join("observations").join(format!("{}.json",f.id));
   let observation=read_limited(&p,4096).ok().and_then(|b|serde_json::from_slice::<Value>(&b).ok()).unwrap_or(json!({"state":"unobserved"}));
   json!({"fish":f.id,"session":f.session,"state":active.map_or("idle",|j|j.state.as_str()),"task_id":active.map(|j|j.id.as_str()),"terminal_observation":observation})
  }).collect();
        Ok(
            json!({"schema":1,"config_sha256":self.owner,"reservations":jobs.len(),"cap":self.config.max_dispatches,"fish":fish,"jobs":jobs,"training_allowed":false,"root_review_automatic":false}),
        )
    }
    pub fn review(&self, id: &str, decision: &str, note: &str) -> Result<Value> {
        self.ready()?;
        let _lease = lease(&self.config)?;
        ensure!(
            matches!(decision, "accept" | "reject")
                && note.len() <= 2000
                && !note.chars().any(|c| c.is_control() && c != '\n'),
            "review parameters"
        );
        let j = self
            .jobs()?
            .into_iter()
            .find(|j| j.id == id)
            .context("task missing")?;
        ensure!(
            j.state == "review_pending",
            "only completed pending reports can be reviewed"
        );
        let p = self.config.root.join("reports").join(format!("{id}.json"));
        ensure!(
            Some(hash(&read_limited(&p, MAX_FILE)?)) == j.report_sha256,
            "stored report changed"
        );
        let v = json!({"schema":1,"task_id":id,"nonce":j.nonce,"report_sha256":j.report_sha256,"decision":decision,"note":note,"training_allowed":false,"automatic_application":false});
        exact_or_create(
            &self
                .config
                .root
                .join("decisions")
                .join(format!("{id}.json")),
            &serde_json::to_vec_pretty(&v)?,
        )?;
        Ok(v)
    }
}
pub fn task_hash(t: &Task) -> String {
    hash(&serde_json::to_vec(t).expect("serializable task"))
}
pub fn idle(c: &Config, s: &Screen) -> bool {
    if s.dead || s.command != c.pane_command || s.text.len() > 32768 {
        return false;
    }
    // Viewport padding follows the interactive content on the observed Codex TUI.
    let lines: Vec<_> = s.text.lines().collect();
    let end = lines
        .iter()
        .rposition(|l| !l.trim().is_empty())
        .map_or(0, |i| i + 1);
    let tail = lines[..end]
        .iter()
        .rev()
        .take(16)
        .copied()
        .collect::<Vec<_>>()
        .join("\n");
    let low = tail.to_lowercase();
    // The update dialog retains an older idle prompt above its action choices.
    if low.contains("update available") && low.contains("update now") {
        return false;
    }
    if [
        "esc to interrupt",
        "esc to cancel",
        "approve this",
        "waiting for permission",
        "approval required",
        "permission required",
        "trust this directory",
        "do you trust",
        "sign in to continue",
        "authentication required",
        "press enter to continue",
        "enter to continue",
        "enter continue",
        "select an option",
        "sign in with chatgpt",
        "choose how to sign in",
        "paste your api key",
        "would you like to run",
    ]
    .iter()
    .any(|w| low.contains(w))
    {
        return false;
    }
    tail.lines().any(|l| l.trim() == c.idle_marker)
}

#[cfg(test)]
mod tests {
    use super::*;
    struct Fixture {
        parent: PathBuf,
        path: PathBuf,
        ctx: Context,
    }
    impl Fixture {
        fn new(count: usize) -> Self {
            Self::capped(count, count.max(1))
        }
        fn capped(count: usize, cap: usize) -> Self {
            let stamp = SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_nanos();
            let parent =
                std::env::temp_dir().join(format!("peta-fish-test-{}-{stamp}", std::process::id()));
            fs::create_dir(&parent).unwrap();
            let work = parent.join("work");
            fs::create_dir(&work).unwrap();
            fs::write(work.join("input.txt"), "bounded teaching excerpt").unwrap();
            let fish = Fish {
                id: "one".into(),
                session: "peta-fish-one".into(),
                workdir: work.clone(),
            };
            let tasks: Vec<_> = (0..count)
                .map(|i| Task {
                    id: format!("task{i}"),
                    fish: "one".into(),
                    prompt: "Review the bound excerpt in English; retain uncertainty.".into(),
                    inputs: vec![Input {
                        path: work.join("input.txt"),
                        sha256: hash(b"bounded teaching excerpt"),
                    }],
                })
                .collect();
            let queue = tasks
                .iter()
                .map(|t| serde_json::to_string(t).unwrap() + "\n")
                .collect::<String>();
            let qp = parent.join("queue.jsonl");
            fs::write(&qp, &queue).unwrap();
            let c = Config {
                schema: 1,
                root: parent.join("run"),
                queue: qp,
                queue_sha256: hash(queue.as_bytes()),
                tmux: "/usr/bin/tmux".into(),
                codex: "/usr/bin/codex".into(),
                model: DEFAULT_MODEL.into(),
                fish: vec![fish],
                max_dispatches: cap,
                poll_ms: 100,
                task_timeout_secs: 600,
                idle_marker: "›".into(),
                pane_command: "codex".into(),
            };
            let path = parent.join("config.json");
            fs::write(&path, serde_json::to_vec(&c).unwrap()).unwrap();
            let ctx = Context::load(&path).unwrap();
            ctx.prepare().unwrap();
            Self { parent, path, ctx }
        }
        fn two() -> Self {
            let mut f = Self::new(1);
            fs::remove_dir_all(&f.ctx.config.root).unwrap();
            fs::remove_dir(f.ctx.config.fish[0].workdir.join(".peta-fish-reports")).unwrap();
            let work = f.parent.join("work-two");
            fs::create_dir(&work).unwrap();
            fs::write(work.join("input.txt"), "second input").unwrap();
            f.ctx.config.fish.push(Fish {
                id: "two".into(),
                session: "peta-fish-two".into(),
                workdir: work.clone(),
            });
            f.ctx.config.max_dispatches = 2;
            f.ctx.tasks.push(Task {
                id: "tasktwo".into(),
                fish: "two".into(),
                prompt: "Review this input in English.".into(),
                inputs: vec![Input {
                    path: work.join("input.txt"),
                    sha256: hash(b"second input"),
                }],
            });
            let queue = f
                .ctx
                .tasks
                .iter()
                .map(|t| serde_json::to_string(t).unwrap() + "\n")
                .collect::<String>();
            fs::write(&f.ctx.config.queue, &queue).unwrap();
            f.ctx.config.queue_sha256 = hash(queue.as_bytes());
            fs::write(&f.path, serde_json::to_vec(&f.ctx.config).unwrap()).unwrap();
            f.ctx = Context::load(&f.path).unwrap();
            f.ctx.prepare().unwrap();
            f
        }
        fn ready_two(&self) -> Fake {
            let mut t = self.ready();
            t.exists = false;
            self.ctx.launch("two", &mut t).unwrap();
            t
        }
        fn ready(&self) -> Fake {
            let mut t = Fake::default();
            self.ctx.launch("one", &mut t).unwrap();
            t
        }
        fn report(&self, j: &Job, nonce: &str) -> PathBuf {
            let p = self.ctx.config.fish[0]
                .workdir
                .join(".peta-fish-reports")
                .join(format!("{}.json", j.nonce));
            fs::write(&p,serde_json::to_vec(&json!({"schema":1,"task_id":j.id,"nonce":nonce,"input_sha256":j.task_sha256,"result":{"finding":"A review claim, not validated biology."}})).unwrap()).unwrap();
            p
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.parent);
        }
    }
    #[derive(Default)]
    struct Fake {
        exists: bool,
        foreign: bool,
        busy: bool,
        changing: bool,
        captures: usize,
        loaded: usize,
        pasted: usize,
        entered: usize,
        fail_enter: bool,
        unavailable: Option<String>,
    }
    impl Terminal for Fake {
        fn launch(&mut self, _: &Config, _: &Fish, _: &str) -> Result<()> {
            ensure!(!self.exists, "existing session");
            self.exists = true;
            Ok(())
        }
        fn owned(&mut self, f: &Fish, _: &str) -> Result<bool> {
            ensure!(
                self.unavailable.as_deref() != Some(f.id.as_str()),
                "missing exact session"
            );
            Ok(self.exists && !self.foreign)
        }
        fn capture(&mut self, _: &Fish) -> Result<Screen> {
            self.captures += 1;
            Ok(Screen {
                command: "codex".into(),
                dead: false,
                text: if self.busy {
                    "Working · esc to interrupt".into()
                } else if self.changing {
                    format!("›\n{}", self.captures)
                } else {
                    "Codex\n›\n100% context left".into()
                },
            })
        }
        fn load(&mut self, _: &str, p: &Path) -> Result<()> {
            ensure!(p.is_file(), "prompt missing");
            self.loaded += 1;
            Ok(())
        }
        fn paste(&mut self, _: &Fish, _: &str) -> Result<()> {
            self.pasted += 1;
            Ok(())
        }
        fn enter(&mut self, _: &Fish) -> Result<()> {
            self.entered += 1;
            ensure!(!self.fail_enter, "uncertain Enter");
            Ok(())
        }
    }
    #[test]
    fn configured_model_defaults_to_terra_and_refuses_non_codex_ids() {
        let f = Fixture::new(1);
        assert_eq!(f.ctx.config.model, DEFAULT_MODEL);
        let mut v: Value = serde_json::from_slice(&fs::read(&f.path).unwrap()).unwrap();
        v.as_object_mut().unwrap().remove("model");
        fs::write(&f.path, serde_json::to_vec(&v).unwrap()).unwrap();
        assert_eq!(Context::load(&f.path).unwrap().config.model, DEFAULT_MODEL);
        v["model"] = json!("gpt-6.1-sol");
        fs::write(&f.path, serde_json::to_vec(&v).unwrap()).unwrap();
        assert_eq!(Context::load(&f.path).unwrap().config.model, "gpt-6.1-sol");
        for bad in ["sonnet", "opus", "", "gpt-6.1 terra", "gpt-\n"] {
            v["model"] = json!(bad);
            fs::write(&f.path, serde_json::to_vec(&v).unwrap()).unwrap();
            assert!(Context::load(&f.path).is_err(), "{bad}");
        }
    }
    #[test]
    fn known_nonmodal_help_does_not_mask_real_modal_or_busy_state() {
        let f = Fixture::new(1);
        let mut c = f.ctx.config.clone();
        c.pane_command = "node".into();
        c.idle_marker = "› Ask Codex to do anything".into();
        let screen = Screen { command: "node".into(), dead: false, text: "/permissions to choose sandbox\nWarning: less than25% weekly limit left · F2\n› Ask Codex to do anything".into() };
        assert!(idle(&c, &screen));
        for message in [
            "Waiting for permission",
            "Do you trust this directory?",
            "Working · esc to interrupt",
            "Sign in to continue",
        ] {
            let mut s = screen.clone();
            s.text.push_str(&format!("\n{message}"));
            assert!(!idle(&c, &s));
        }
    }
    #[test]
    fn padded_idle_viewport_retains_its_observed_prompt() {
        let f = Fixture::new(1);
        let mut c = f.ctx.config.clone();
        c.pane_command = "node".into();
        c.idle_marker = "› Ask Codex to do anything".into();
        let text = format!(
            "/permissions to choose sandbox\nWarning: less than25% weekly limit left · F2\n› Ask Codex to do anything{}",
            "\n".repeat(32)
        );
        let screen = Screen {
            command: "node".into(),
            dead: false,
            text,
        };
        assert!(
            idle(&c, &screen),
            "blank viewport padding is not terminal activity"
        );
    }
    #[test]
    fn padded_update_modal_blocks_the_old_idle_prompt() {
        let f = Fixture::new(1);
        let mut c = f.ctx.config.clone();
        c.pane_command = "node".into();
        c.idle_marker = "› Ask Codex to do anything".into();
        let text = format!(
            "Codex0.165.0\n/permissions help\n\n› Ask Codex to do anything\n\nUpdate available ·0.165.0→0.166.0\n›1. Update now (runs npm install)\n2. Skip\n3. Skip until next version\nenter continue · esc skip{}",
            "\n".repeat(24)
        );
        let screen = Screen {
            command: "node".into(),
            dead: false,
            text,
        };
        assert!(!idle(&c, &screen), "Enter would activate Update now");
        let lines: Vec<_> = screen.text.lines().collect();
        let end = lines.iter().rposition(|l| !l.trim().is_empty()).unwrap() + 1;
        assert!(
            lines[..end]
                .iter()
                .rev()
                .take(16)
                .any(|l| l.trim() == c.idle_marker),
            "a trim-only repair would wrongly admit this actual modal layout"
        );
    }
    #[test]
    fn unknown_busy_foreign_and_changing_terminal_never_dispatch() {
        let f = Fixture::new(1);
        let mut unknown = Fake {
            exists: true,
            ..Default::default()
        };
        assert_eq!(f.ctx.once(&mut unknown).unwrap()["reservations"], 0);
        let mut t = f.ready();
        for mode in 0..3 {
            t.busy = mode == 0;
            t.foreign = mode == 1;
            t.changing = mode == 2;
            assert_eq!(f.ctx.once(&mut t).unwrap()["reservations"], 0);
        }
        assert_eq!(t.entered, 0);
    }
    #[test]
    fn completion_requires_bound_report_and_root_decision_before_next_task() {
        let f = Fixture::new(2);
        let mut t = f.ready();
        f.ctx.once(&mut t).unwrap();
        assert_eq!(t.entered, 1);
        f.ctx.once(&mut t).unwrap();
        assert_eq!(t.entered, 1);
        let j = f.ctx.jobs().unwrap().remove(0);
        f.report(&j, &j.nonce);
        f.ctx.once(&mut t).unwrap();
        assert_eq!(f.ctx.jobs().unwrap()[0].state, "review_pending");
        assert_eq!(t.entered, 1);
        assert!(!f.ctx.config.root.join("decisions/task0.json").exists());
        f.ctx
            .review(
                "task0",
                "accept",
                "Checked against source; no automatic application.",
            )
            .unwrap();
        f.ctx.once(&mut t).unwrap();
        assert_eq!(t.entered, 2);
        assert_eq!(f.ctx.jobs().unwrap().len(), 2);
        let q: Value =
            serde_json::from_slice(&fs::read(f.ctx.config.root.join("review/task0.json")).unwrap())
                .unwrap();
        assert_eq!(q["training_allowed"], false);
        assert_eq!(q["automatic_application"], false);
    }
    #[test]
    fn wrong_report_is_quarantined_and_original_bytes_retained() {
        let f = Fixture::new(1);
        let mut t = f.ready();
        f.ctx.once(&mut t).unwrap();
        let j = f.ctx.jobs().unwrap().remove(0);
        let p = f.report(&j, "wrong-nonce");
        let bytes = fs::read(&p).unwrap();
        f.ctx.once(&mut t).unwrap();
        assert_eq!(f.ctx.jobs().unwrap()[0].state, "quarantine");
        assert_eq!(fs::read(p).unwrap(), bytes);
        assert!(!f.ctx.config.root.join("review/task0.json").exists());
        assert_eq!(t.entered, 1);
    }
    #[test]
    fn interrupted_enter_is_never_replayed_after_reload() {
        let f = Fixture::new(2);
        let mut t = f.ready();
        t.fail_enter = true;
        assert!(f.ctx.once(&mut t).is_err());
        assert_eq!(f.ctx.jobs().unwrap()[0].state, "enter_reserved");
        let before = t.entered;
        t.fail_enter = false;
        let ctx = Context::load(&f.path).unwrap();
        ctx.once(&mut t).unwrap();
        assert_eq!(ctx.jobs().unwrap()[0].state, "quarantine");
        assert_eq!(t.entered, before);
    }
    #[test]
    fn changed_source_and_expired_report_preserve_uncertainty() {
        let f = Fixture::new(1);
        let mut t = f.ready();
        f.ctx.once(&mut t).unwrap();
        fs::write(f.ctx.config.fish[0].workdir.join("input.txt"), "changed").unwrap();
        f.ctx.once(&mut t).unwrap();
        assert_eq!(f.ctx.jobs().unwrap()[0].state, "quarantine");
        assert_eq!(t.entered, 1);
        let f = Fixture::new(1);
        let mut t = f.ready();
        f.ctx.once(&mut t).unwrap();
        let mut j = f.ctx.jobs().unwrap().remove(0);
        j.created = 0;
        save(&f.ctx.job_path(&j.id), &j).unwrap();
        f.ctx.once(&mut t).unwrap();
        assert_eq!(f.ctx.jobs().unwrap()[0].state, "quarantine");
        assert_eq!(t.entered, 1);
    }
    #[test]
    fn report_admission_publication_is_byte_idempotent_but_changed_report_fails() {
        let f = Fixture::new(1);
        let mut t = f.ready();
        f.ctx.once(&mut t).unwrap();
        let mut j = f.ctx.jobs().unwrap().remove(0);
        let p = f.report(&j, &j.nonce);
        f.ctx.admit(&mut j, &p).unwrap();
        let q = fs::read(f.ctx.config.root.join("review/task0.json")).unwrap();
        f.ctx.admit(&mut j, &p).unwrap();
        assert_eq!(
            fs::read(f.ctx.config.root.join("review/task0.json")).unwrap(),
            q
        );
        save(&f.ctx.job_path(&j.id), &j).unwrap();
        fs::write(&p, "{}").unwrap();
        assert!(f.ctx.admit(&mut j, &p).is_err());
        fs::write(f.ctx.config.root.join("reports/task0.json"), "{}").unwrap();
        assert!(
            format!("{:#}", f.ctx.review("task0", "accept", "").unwrap_err())
                .contains("stored report changed")
        );
    }
    #[test]
    fn cap_and_preexisting_launch_are_not_reset_or_adopted() {
        let f = Fixture::capped(2, 1);
        let mut t = f.ready();
        f.ctx.once(&mut t).unwrap();
        let j = f.ctx.jobs().unwrap().remove(0);
        f.report(&j, &j.nonce);
        f.ctx.once(&mut t).unwrap();
        f.ctx.review("task0", "reject", "No promotion.").unwrap();
        f.ctx.once(&mut t).unwrap();
        assert_eq!(t.entered, 1);
        assert!(f.ctx.launch("one", &mut t).is_err());
    }
    #[test]
    fn config_queue_pin_and_symlink_report_are_rejected() {
        let f = Fixture::new(1);
        fs::write(&f.ctx.config.queue, "").unwrap();
        assert!(Context::load(&f.path).is_err());
        let f = Fixture::new(1);
        let mut t = f.ready();
        f.ctx.once(&mut t).unwrap();
        let j = f.ctx.jobs().unwrap().remove(0);
        let p = f.ctx.config.fish[0]
            .workdir
            .join(".peta-fish-reports")
            .join(format!("{}.json", j.nonce));
        std::os::unix::fs::symlink(f.ctx.config.fish[0].workdir.join("input.txt"), p).unwrap();
        f.ctx.once(&mut t).unwrap();
        assert_eq!(f.ctx.jobs().unwrap()[0].state, "quarantine");
    }
    #[test]
    fn real_subprocess_keeps_literal_argv_and_times_out_without_shell() {
        let f = Fixture::new(1);
        let mut tm = terminal::Tmux {
            binary: "/usr/bin/printf".into(),
            temporary: f.ctx.config.root.join("tmp"),
            timeout: Duration::from_secs(1),
        };
        let literal = "space ' \" $(touch nope) ; echo nope";
        assert_eq!(tm.run(&["%s", literal]).unwrap(), literal);
        assert!(!f.parent.join("nope").exists());
        tm.binary = "/usr/bin/sleep".into();
        tm.timeout = Duration::from_millis(20);
        assert!(tm.run(&["2"]).is_err());
    }
    #[test]
    fn exact_targets_refuse_missing_prefix_in_native_fake_subprocess() {
        let f = Fixture::new(0);
        let script = f.parent.join("fake-tmux");
        // Fake prefix fallback: only a2 exists. No live tmux server or model.
        fs::write(
            &script,
            r#"#!/bin/sh
printf '%s\n' "$@" >>"$0.argv"
case "$1" in
show-options)
  [ "$4" = "=peta-fish-a" ] && exit 1
  printf 'owner\n';;
list-panes) printf '%%1\n';;
display-message) printf 'codex|0\n';;
capture-pane) printf 'Codex\n›\n';;
esac
"#,
        )
        .unwrap();
        fs::set_permissions(&script, fs::Permissions::from_mode(0o700)).unwrap();
        let mut tm = terminal::Tmux {
            binary: script.clone(),
            temporary: f.ctx.config.root.join("tmp"),
            timeout: Duration::from_secs(1),
        };
        let a = Fish {
            id: "a".into(),
            session: "peta-fish-a".into(),
            workdir: f.ctx.config.fish[0].workdir.clone(),
        };
        let a2 = Fish {
            id: "a2".into(),
            session: "peta-fish-a2".into(),
            workdir: a.workdir.clone(),
        };
        assert_eq!(
            tm.run(&[
                "show-options",
                "-v",
                "-t",
                "peta-fish-a",
                "@peta-fish-owner"
            ])
            .unwrap()
            .trim(),
            "owner"
        );
        fs::write(script.with_extension("argv"), "").unwrap();
        assert!(tm.owned(&a, "owner").is_err());
        assert!(tm.owned(&a2, "owner").unwrap());
        tm.launch(&f.ctx.config, &a2, "owner").unwrap();
        tm.capture(&a2).unwrap();
        tm.paste(&a2, "bound-buffer").unwrap();
        tm.enter(&a2).unwrap();
        let log = fs::read_to_string(script.with_extension("argv")).unwrap();
        let args: Vec<_> = log.lines().collect();
        for pair in args.windows(2).filter(|w| w[0] == "-t") {
            assert!(["=peta-fish-a", "=peta-fish-a2", "=peta-fish-a2:"].contains(&pair[1]));
        }
        assert_eq!(terminal::pane_target(&a), "=peta-fish-a:");
        assert!(args.contains(&"=peta-fish-a2:"));
    }
    #[test]
    fn prefix_config_ids_are_rejected_without_changing_serialization() {
        let mut f = Fixture::two();
        for (fish, id) in f.ctx.config.fish.iter_mut().zip(["a", "a2"]) {
            fish.id = id.into();
            fish.session = format!("peta-fish-{id}");
        }
        fs::write(&f.path, serde_json::to_vec(&f.ctx.config).unwrap()).unwrap();
        assert!(format!("{:#}", Context::load(&f.path).err().unwrap()).contains("must not prefix"));
        let task = Task {
            id: "t".into(),
            fish: "a".into(),
            prompt: "p".into(),
            inputs: vec![],
        };
        assert_eq!(
            serde_json::to_string(&task).unwrap(),
            r#"{"id":"t","fish":"a","prompt":"p","inputs":[]}"#
        );
        assert_eq!(
            task_hash(&task),
            hash(br#"{"id":"t","fish":"a","prompt":"p","inputs":[]}"#)
        );
    }
    #[test]
    fn fifo_inputs_and_reports_are_refused_without_a_writer() {
        use std::{ffi::CString, os::unix::ffi::OsStrExt, time::Instant};
        let f = Fixture::new(1);
        let fifo = f.parent.join("unopened-fifo");
        let name = CString::new(fifo.as_os_str().as_bytes()).unwrap();
        assert_eq!(unsafe { libc::mkfifo(name.as_ptr(), 0o600) }, 0);
        let start = Instant::now();
        assert!(format!("{:#}", read_limited(&fifo, 1024).unwrap_err()).contains("regular file"));
        assert!(start.elapsed() < Duration::from_secs(1));
        let mut t = f.ready();
        f.ctx.once(&mut t).unwrap();
        let j = f.ctx.jobs().unwrap().remove(0);
        let report = f
            .ctx
            .report_dir(&f.ctx.config.fish[0])
            .unwrap()
            .join(format!("{}.json", j.nonce));
        let name = CString::new(report.as_os_str().as_bytes()).unwrap();
        assert_eq!(unsafe { libc::mkfifo(name.as_ptr(), 0o600) }, 0);
        f.ctx.once(&mut t).unwrap();
        assert_eq!(f.ctx.jobs().unwrap()[0].state, "quarantine");
        assert_eq!(t.entered, 1);
        assert!(!f.ctx.config.root.join("review/task0.json").exists());
    }
    #[test]
    fn symlink_report_directory_cannot_admit_external_report() {
        let f = Fixture::new(1);
        let mut t = f.ready();
        f.ctx.once(&mut t).unwrap();
        let j = f.ctx.jobs().unwrap().remove(0);
        let dir = f.ctx.config.fish[0].workdir.join(".peta-fish-reports");
        let moved = f.parent.join("external-reports");
        fs::rename(&dir, &moved).unwrap();
        std::os::unix::fs::symlink(&moved, &dir).unwrap();
        assert!(f.report(&j, &j.nonce).is_file());
        f.ctx.once(&mut t).unwrap();
        assert_eq!(f.ctx.jobs().unwrap()[0].state, "quarantine");
        assert!(!f.ctx.config.root.join("review/task0.json").exists());
        assert_eq!(t.entered, 1);
    }
    #[test]
    fn preflight_unavailable_or_refused_fish_does_not_starve_another() {
        for mode in 0..4 {
            let f = Fixture::two();
            let mut t = f.ready_two();
            match mode {
                0 => t.unavailable = Some("one".into()),
                1 => fs::write(&f.ctx.tasks[0].inputs[0].path, "changed source").unwrap(),
                2 => {
                    let task = &f.ctx.tasks[0];
                    let nonce =
                        hash(format!("{}:{}:{}", f.ctx.owner, task.id, task_hash(task)).as_bytes());
                    fs::write(
                        f.ctx
                            .report_dir(&f.ctx.config.fish[0])
                            .unwrap()
                            .join(format!("{nonce}.json")),
                        "{}",
                    )
                    .unwrap();
                }
                _ => {
                    let dir = f.ctx.config.fish[0].workdir.join(".peta-fish-reports");
                    fs::remove_dir(&dir).unwrap();
                    std::os::unix::fs::symlink(&f.ctx.config.fish[1].workdir, dir).unwrap();
                }
            }
            let state = f.ctx.once(&mut t).unwrap();
            assert_eq!(t.entered, 1);
            let jobs = f.ctx.jobs().unwrap();
            assert_eq!(jobs.len(), 1);
            assert_eq!(jobs[0].fish, "two");
            let obs = &state["fish"][0]["terminal_observation"];
            assert_eq!(
                obs["state"],
                if mode == 0 {
                    "terminal_unavailable"
                } else {
                    "refused_before_effect"
                }
            );
            if mode != 0 {
                assert_eq!(obs["task_id"], "task0");
            }
        }
    }
    #[test]
    fn waiting_unavailable_terminal_retains_job_but_allows_another_fish() {
        let f = Fixture::two();
        let mut t = f.ready_two();
        t.unavailable = Some("two".into());
        f.ctx.once(&mut t).unwrap();
        let j = f.ctx.jobs().unwrap().remove(0);
        f.report(&j, &j.nonce);
        t.unavailable = Some("one".into());
        f.ctx.once(&mut t).unwrap();
        let jobs = f.ctx.jobs().unwrap();
        assert_eq!(jobs.len(), 2);
        assert_eq!(
            jobs.iter().find(|j| j.fish == "one").unwrap().state,
            "waiting_report"
        );
        assert_eq!(
            jobs.iter().find(|j| j.fish == "two").unwrap().state,
            "waiting_report"
        );
        assert_eq!(t.entered, 2);
    }
}
