//! One bounded agent session in an isolated Git worktree (`whale-research`). The prompt
//! template, model, write scopes and protected file classes come from configuration.
//! Results are review-only candidates; nothing is merged, pushed or replayed.
use crate::{
    util::{create, error, hash, load, normal, now, pinned, read_bounded, storage_available, storage_floor, valid_hash},
    Result,
};
use serde::Deserialize;
use serde_json::{json, Map, Value};
use std::{
    collections::BTreeSet,
    fs::{self, File, OpenOptions},
    io::Write,
    os::{
        fd::AsRawFd,
        unix::{
            fs::{OpenOptionsExt, PermissionsExt},
            process::CommandExt,
        },
    },
    path::{Path, PathBuf},
    process::{Command, Stdio},
    thread,
    time::{Duration, Instant},
};
const LIMIT: usize = 8 << 20;
pub const SLOTS: [&str; 7] = ["{{SOURCES_JSON}}", "{{REFERENCES_JSON}}", "{{SCOPES_JSON}}", "{{ALLOWED_DIRS_JSON}}", "{{ALLOWED_FILES_JSON}}", "{{CONTEXT_JSON}}", "{{TASK}}"];
fn d_timeout() -> u64 {
    300
}
fn d_retained() -> usize {
    64
}
fn d_prompt_max() -> usize {
    48_000
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Binding {
    path: String,
    sha256: String,
}
#[derive(Clone, Debug, Default, Deserialize)]
#[serde(deny_unknown_fields)]
struct FileRule {
    suffix: String,
    #[serde(default)]
    path_contains_any: Vec<String>,
    #[serde(default)]
    name_equals_any: Vec<String>,
    #[serde(default)]
    name_ends_with_any: Vec<String>,
}
#[derive(Debug, Deserialize)]
#[serde(deny_unknown_fields)]
struct Config {
    run_root: PathBuf,
    repository: PathBuf,
    base_commit: String,
    worktree: PathBuf,
    branch: String,
    branch_prefix: String,
    codex: PathBuf,
    model: String,
    effort: String,
    prompt_template: Value,
    #[serde(default = "d_prompt_max")]
    prompt_max_bytes: usize,
    #[serde(default = "d_timeout")]
    timeout_seconds: u64,
    #[serde(default)]
    retain_successful_worktree: bool,
    #[serde(default = "d_retained")]
    max_retained_worktrees: usize,
    source_bindings: Vec<Binding>,
    #[serde(default)]
    reference_bindings: Vec<Binding>,
    #[serde(default)]
    scope_bindings: Vec<Binding>,
    #[serde(default)]
    scope_manifest_equals: Map<String, Value>,
    #[serde(default)]
    sidecar_suffixes: Vec<String>,
    allowed_write_dirs: Vec<String>,
    #[serde(default)]
    allowed_write_files: Vec<String>,
    #[serde(default)]
    write_file_rule: Option<FileRule>,
    protected_suffixes: Vec<String>,
    #[serde(default)]
    staged_guard: Option<String>,
    #[serde(default)]
    source_coverage: Value,
    #[serde(default)]
    reference_metadata_only: bool,
    #[serde(default)]
    reference_gap: Value,
    #[serde(default)]
    task: String,
    #[serde(default)]
    target_dir: Option<PathBuf>,
    #[serde(default)]
    storage_floor_bytes: Option<u64>,
}
fn git(repo: &Path, args: &[&str]) -> Result<Vec<u8>> {
    let out = Command::new("git").arg("-C").arg(repo).args(args).output()?;
    if !out.status.success() {
        return Err(error(format!("git failed: {}", String::from_utf8_lossy(&out.stderr).chars().take(300).collect::<String>())));
    }
    if out.stdout.len() > LIMIT {
        return Err(error("git observation exceeds limit"));
    }
    Ok(out.stdout)
}
impl Config {
    fn parse(v: &Value) -> Result<Self> {
        let c: Self = serde_json::from_value(v.clone())?;
        mova_teacher::validate_model(&c.model).map_err(|e| error(e.to_string()))?;
        if !matches!(c.effort.as_str(), "low" | "medium" | "high" | "xhigh") {
            return Err(error("effort must be low, medium, high or xhigh"));
        }
        if [&c.run_root, &c.repository, &c.worktree, &c.codex].iter().any(|p| !p.is_absolute()) || c.base_commit.len() != 40 || !c.base_commit.bytes().all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase()) || !(1..=600).contains(&c.timeout_seconds) || !(1..=4096).contains(&c.max_retained_worktrees) || !(1..=1_000_000).contains(&c.prompt_max_bytes) {
            return Err(error("absolute paths, full commit, timeout 1..600 and bounded caps required"));
        }
        let branch_ok = |b: &str| b.len() <= 128 && !b.contains("//") && !b.starts_with('/') && b.bytes().all(|x| x.is_ascii_alphanumeric() || matches!(x, b'/' | b'-' | b'_'));
        if c.branch_prefix.is_empty() || !branch_ok(&c.branch_prefix) || !c.branch.starts_with(&c.branch_prefix) || c.branch.len() == c.branch_prefix.len() || !branch_ok(&c.branch) {
            return Err(error("bounded branch under the configured branch_prefix required"));
        }
        if c.source_bindings.is_empty() || c.source_bindings.len() > 32 || c.reference_bindings.len() > 128 || c.scope_bindings.len() > 64 || c.allowed_write_dirs.is_empty() || c.allowed_write_dirs.len() > 64 || c.allowed_write_files.len() > 32 || c.task.len() > 12000 || c.protected_suffixes.is_empty() || c.protected_suffixes.len() > 16 {
            return Err(error("agent scope outside bounded limits"));
        }
        if c.protected_suffixes.iter().any(|s| s.is_empty() || s.contains('/')) || c.sidecar_suffixes.iter().any(|s| s.is_empty() || s.contains('/')) {
            return Err(error("protected/sidecar suffixes must be nonempty file-name suffixes"));
        }
        if c.staged_guard.as_deref().is_some_and(|g| !normal(g)) {
            return Err(error("staged_guard must be a normal repository-relative path"));
        }
        let mut seen = BTreeSet::new();
        for b in &c.source_bindings {
            if !normal(&b.path) || !valid_hash(&b.sha256) || !seen.insert(&b.path) {
                return Err(error("invalid or duplicate source pin"));
            }
        }
        for b in &c.reference_bindings {
            if !Path::new(&b.path).is_absolute() || b.path.len() > 4096 || !valid_hash(&b.sha256) {
                return Err(error("invalid reference pin"));
            }
        }
        for b in &c.scope_bindings {
            if !normal(&b.path) || !b.path.contains('/') || !valid_hash(&b.sha256) {
                return Err(error("invalid scope manifest pin"));
            }
        }
        for p in &c.allowed_write_dirs {
            let sidecar = c.sidecar_suffixes.iter().any(|s| c.source_bindings.iter().any(|b| p == &format!("{}{s}", b.path)));
            let scoped = c.scope_bindings.iter().any(|b| Path::new(&b.path).parent() == Some(Path::new(p)));
            if !normal(p) || !(sidecar || scoped) {
                return Err(error("write directories must be sidecars of pinned sources or directories of pinned scope manifests"));
            }
        }
        if !c.allowed_write_files.is_empty() {
            let rule = c.write_file_rule.as_ref().ok_or("allowed_write_files require write_file_rule")?;
            for p in &c.allowed_write_files {
                let name = Path::new(p).file_name().and_then(|s| s.to_str()).unwrap_or("");
                let shaped = rule.path_contains_any.iter().any(|x| p.contains(x.as_str())) || rule.name_equals_any.iter().any(|x| x == name) || rule.name_ends_with_any.iter().any(|x| name.ends_with(x.as_str()));
                if !normal(p) || rule.suffix.is_empty() || !p.ends_with(&rule.suffix) || !shaped || !c.source_bindings.iter().any(|b| b.path == *p) {
                    return Err(error("file writes require an exact pinned path matching write_file_rule"));
                }
            }
        }
        Ok(c)
    }
    fn allowed(&self, p: &str) -> bool {
        normal(p) && (self.allowed_write_files.iter().any(|x| x == p) || self.allowed_write_dirs.iter().any(|d| p.starts_with(&format!("{d}/"))))
    }
    fn template(&self) -> Result<String> {
        let (_, b) = pinned(&self.prompt_template, None, 256 * 1024).map_err(|e| error(format!("agent prompt template pin: {e}")))?;
        if !Path::new(self.prompt_template["path"].as_str().unwrap_or("")).is_absolute() {
            return Err(error("agent prompt_template.path must be absolute"));
        }
        Ok(String::from_utf8(b)?)
    }
    fn pins(&self, after: bool) -> Result<()> {
        for b in &self.source_bindings {
            if after && self.allowed_write_files.contains(&b.path) {
                continue;
            }
            let bytes = if after { read_bounded(&self.worktree.join(&b.path), LIMIT)? } else { git(&self.repository, &["show", &format!("{}:{}", self.base_commit, b.path)])? };
            if hash(&bytes) != b.sha256 {
                return Err(error(format!("protected/source pin changed: {}", b.path)));
            }
        }
        for b in &self.scope_bindings {
            let bytes = git(&self.repository, &["show", &format!("{}:{}", self.base_commit, b.path)])?;
            if hash(&bytes) != b.sha256 {
                return Err(error("tracked scope manifest pin changed"));
            }
            let v: Value = serde_json::from_slice(&bytes)?;
            for (k, want) in &self.scope_manifest_equals {
                if &v[k] != want {
                    return Err(error(format!("pinned scope manifest field {k} does not match")));
                }
            }
        }
        for b in &self.reference_bindings {
            if hash(read_bounded(Path::new(&b.path), LIMIT)?) != b.sha256 {
                return Err(error("reference pin changed"));
            }
        }
        Ok(())
    }
}
/// Render the pinned template; every slot must occur exactly once in the template.
fn prompt(c: &Config) -> Result<String> {
    let t = c.template()?;
    let pins = |v: &[Binding]| serde_json::to_string(&v.iter().map(|b| json!({"path":b.path,"sha256":b.sha256})).collect::<Vec<_>>());
    let values = [
        pins(&c.source_bindings)?,
        pins(&c.reference_bindings)?,
        pins(&c.scope_bindings)?,
        serde_json::to_string(&c.allowed_write_dirs)?,
        serde_json::to_string(&c.allowed_write_files)?,
        serde_json::to_string(&json!({"source_coverage":c.source_coverage,"reference_metadata_only":c.reference_metadata_only,"reference_gap":c.reference_gap}))?,
        c.task.clone(),
    ];
    let mut at = Vec::new();
    for (slot, value) in SLOTS.iter().zip(values) {
        if t.matches(slot).count() != 1 {
            return Err(error(format!("agent prompt template must contain {slot} exactly once")));
        }
        at.push((t.find(slot).ok_or("slot")?, slot.len(), value));
    }
    at.sort_by_key(|x| x.0);
    let (mut out, mut pos) = (String::new(), 0);
    for (p, len, v) in at {
        out.push_str(&t[pos..p]);
        out.push_str(&v);
        pos = p + len;
    }
    out.push_str(&t[pos..]);
    if out.len() > c.prompt_max_bytes {
        return Err(error(format!("agent prompt exceeds {} bytes", c.prompt_max_bytes)));
    }
    Ok(out)
}
/// Workspace-write agent transport: no network, no web search, no user config or rules.
fn command(c: &Config) -> Command {
    let mut cmd = Command::new(&c.codex);
    cmd.args(["exec", "--ignore-user-config", "--ignore-rules", "--ephemeral", "--sandbox", "workspace-write", "--json", "--color", "never", "--model", &c.model, "-c", "approval_policy=\"never\"", "-c", "forced_login_method=\"chatgpt\"", "-c"]);
    cmd.arg(format!("model_reasoning_effort={}", json!(c.effort)));
    cmd.args(["-c", "web_search=\"disabled\"", "-c", "sandbox_workspace_write.network_access=false", "-c", "project_doc_max_bytes=0"]);
    for f in ["multi_agent", "apps", "plugins", "hooks", "browser_use", "browser_use_external", "computer_use", "image_generation", "view_image", "memories", "skill_search", "unbounded_connection_retries"] {
        cmd.args(["--disable", f]);
    }
    let target = c.target_dir.clone().unwrap_or_else(|| c.worktree.join("target/whale-agent"));
    cmd.arg("-").current_dir(&c.worktree).stdin(Stdio::piped()).env("CARGO_BUILD_JOBS", "2").env("CARGO_NET_OFFLINE", "true").env("CARGO_TARGET_DIR", target);
    for k in mova_teacher::CLEARED_ENV {
        cmd.env_remove(k);
    }
    cmd.process_group(0);
    cmd
}
/// Completed-turn decoding. Command events are counted, never treated as test evidence.
fn decode(b: &[u8]) -> Result<Value> {
    let (mut done, mut final_text, mut commands) = (false, None, 0u64);
    for l in std::str::from_utf8(b)?.lines().filter(|s| !s.trim().is_empty()) {
        let v: Value = serde_json::from_str(l)?;
        match v["type"].as_str() {
            Some("error" | "turn.failed") => return Err(error("agent turn failed")),
            Some("turn.completed") => {
                if done {
                    return Err(error("repeated agent completion"));
                }
                done = true
            }
            Some("item.started" | "item.updated" | "item.completed") => {
                if done {
                    return Err(error("agent item after completion"));
                }
                if v["type"] == "item.completed" {
                    if v["item"]["type"] == "agent_message" {
                        final_text = v["item"]["text"].as_str().map(str::to_owned)
                    }
                    if v["item"]["type"] == "command_execution" {
                        commands += 1
                    }
                }
            }
            _ => {}
        }
    }
    if !done {
        return Err(error("agent turn incomplete"));
    }
    Ok(json!({"final_text":final_text,"completed_command_events":commands,"tests_verified_from_events":false}))
}
struct AgentChild(std::process::Child);
impl Drop for AgentChild {
    fn drop(&mut self) {
        unsafe {
            libc::kill(-(self.0.id() as i32), libc::SIGKILL);
        }
        let _ = self.0.kill();
        let _ = self.0.wait();
    }
}
fn execute(c: &Config) -> Result<Value> {
    let payload = prompt(c)?;
    let out_path = c.run_root.join("agent-events.jsonl");
    let err_path = c.run_root.join("agent-stderr.log");
    let out = OpenOptions::new().create_new(true).write(true).mode(0o600).open(&out_path)?;
    let err = OpenOptions::new().create_new(true).write(true).mode(0o600).open(&err_path)?;
    let mut cmd = command(c);
    cmd.stdout(out).stderr(err);
    let mut child = AgentChild(cmd.spawn()?);
    create(&c.run_root.join("call.json"), &json!({"pid":child.0.id(),"process_group":child.0.id(),"started_unix":now(),"configured_model":c.model,"effort":c.effort,"sandbox":"workspace-write","network_access":false,"prompt_sha256":hash(&payload)}))?;
    let mut stdin = child.0.stdin.take().ok_or("agent stdin unavailable")?;
    let writer = thread::spawn(move || stdin.write_all(payload.as_bytes()));
    let start = Instant::now();
    let mut reason = None;
    let exit = loop {
        if let Some(s) = child.0.try_wait()? {
            break s;
        }
        if start.elapsed() >= Duration::from_secs(c.timeout_seconds) {
            reason = Some("timeout")
        }
        if fs::metadata(&out_path)?.len() > LIMIT as u64 || fs::metadata(&err_path)?.len() > LIMIT as u64 {
            reason = Some("output_limit")
        }
        if reason.is_some() {
            unsafe {
                libc::kill(-(child.0.id() as i32), libc::SIGKILL);
            }
            let _ = child.0.kill();
            break child.0.wait()?;
        }
        thread::sleep(Duration::from_millis(50));
    };
    unsafe {
        libc::kill(-(child.0.id() as i32), libc::SIGKILL);
    }
    let written = writer.join().map_err(|_| "stdin writer panicked")?;
    if let Some(r) = reason {
        return Ok(json!({"outcome":"uncertain","reason":r,"pid":child.0.id(),"automatic_retry":false}));
    }
    written?;
    if !exit.success() {
        return Ok(json!({"outcome":"uncertain","reason":"agent_exit","exit_code":exit.code(),"automatic_retry":false}));
    }
    let b = read_bounded(&out_path, LIMIT)?;
    match decode(&b) {
        Ok(v) => Ok(json!({"outcome":"completed_turn","events_sha256":hash(&b),"result":v})),
        Err(e) => Ok(json!({"outcome":"uncertain","reason":e.to_string(),"events_sha256":hash(&b),"automatic_retry":false})),
    }
}
fn changed(c: &Config) -> Result<Vec<String>> {
    let mut paths = BTreeSet::new();
    let root = c.worktree.canonicalize()?;
    for args in [vec!["diff", "--name-only", "-z", c.base_commit.as_str(), "--"], vec!["ls-files", "--others", "--exclude-standard", "-z"]] {
        for p in git(&c.worktree, &args)?.split(|&b| b == 0).filter(|b| !b.is_empty()) {
            let p = std::str::from_utf8(p)?.to_owned();
            if !c.allowed(&p) {
                return Err(error(format!("outside allowed agent path: {p}")));
            }
            let full = c.worktree.join(&p);
            let m = fs::symlink_metadata(&full)?;
            if !m.is_file() || m.file_type().is_symlink() || m.len() > 4 << 20 || !full.canonicalize()?.starts_with(&root) {
                return Err(error("changed artifact absent, symlink, escaped or oversized"));
            }
            paths.insert(p);
        }
    }
    if paths.len() > 256 {
        return Err(error("too many agent changes"));
    }
    Ok(paths.into_iter().collect())
}
/// Compare every tracked protected blob (by configured suffix) without trusting index flags.
fn protected_bytes(c: &Config) -> Result<usize> {
    let tree = git(&c.worktree, &["ls-tree", "-r", "-z", "--full-tree", &c.base_commit])?;
    let mut files = Vec::new();
    let root = c.worktree.canonicalize()?;
    for row in tree.split(|&x| x == 0).filter(|x| !x.is_empty()) {
        let row = std::str::from_utf8(row)?;
        let (meta, path) = row.split_once('\t').ok_or("invalid Git tree entry")?;
        if !c.protected_suffixes.iter().any(|s| path.ends_with(s.as_str())) || c.allowed(path) {
            continue;
        }
        if !normal(path) {
            return Err(error("unsupported tracked protected path"));
        }
        let fields: Vec<_> = meta.split_whitespace().collect();
        if fields.len() != 3 || fields[1] != "blob" || !matches!(fields[0], "100644" | "100755") {
            return Err(error("unsupported tracked protected object"));
        }
        let full = c.worktree.join(path);
        let m = fs::symlink_metadata(&full)?;
        if !m.is_file() || m.file_type().is_symlink() || !full.canonicalize()?.starts_with(&root) {
            return Err(error("protected source absent or escaped"));
        }
        files.push((path.to_owned(), fields[2].to_owned()));
    }
    if files.is_empty() {
        return Ok(0);
    }
    let input = files.iter().map(|(p, _)| format!("{p}\n")).collect::<String>();
    let mut child = Command::new("git").arg("-C").arg(&c.worktree).args(["hash-object", "--no-filters", "--stdin-paths"]).stdin(Stdio::piped()).stdout(Stdio::piped()).stderr(Stdio::null()).spawn()?;
    let mut stdin = child.stdin.take().ok_or("hash stdin")?;
    let writer = thread::spawn(move || stdin.write_all(input.as_bytes()));
    let out = child.wait_with_output()?;
    writer.join().map_err(|_| "hash writer panicked")??;
    if !out.status.success() || out.stdout.len() > LIMIT {
        return Err(error("protected hash observation failed"));
    }
    let actual: Vec<_> = std::str::from_utf8(&out.stdout)?.lines().collect();
    if actual.len() != files.len() || actual.iter().zip(&files).any(|(a, (_, want))| *a != want) {
        return Err(error("tracked protected bytes changed"));
    }
    Ok(files.len())
}
fn finalize(c: &Config) -> Result<Value> {
    let head = String::from_utf8(git(&c.worktree, &["rev-parse", "HEAD"])?)?;
    if head.trim() != c.base_commit {
        return Err(error("agent changed HEAD"));
    }
    c.pins(true)?;
    let protected_checked = protected_bytes(c)?;
    let paths = changed(c)?;
    if paths.is_empty() {
        return Ok(json!({"outcome":"no_change","changed_paths":paths,"protected_unchanged":true,"protected_files_checked":protected_checked}));
    }
    let mut args = vec!["add", "--"];
    args.extend(paths.iter().map(String::as_str));
    git(&c.worktree, &args)?;
    let guard = match &c.staged_guard {
        Some(g) => {
            let out = Command::new(c.worktree.join(g)).current_dir(&c.worktree).env("CARGO_BUILD_JOBS", "2").output()?;
            if !out.status.success() {
                return Err(error("staged guard rejected agent candidate"));
            }
            "passed"
        }
        None => "not_configured",
    };
    let patch = git(&c.worktree, &["diff", "--cached", "--binary", &c.base_commit, "--"])?;
    crate::util::create_bytes(&c.run_root.join("candidate.patch"), &patch)?;
    git(&c.worktree, &["-c", "core.hooksPath=/dev/null", "-c", "user.name=Mova Whale", "-c", "user.email=whale@localhost.invalid", "commit", "--no-gpg-sign", "-m", "Retain guarded Whale agent candidate for review"])?;
    let candidate_commit = String::from_utf8(git(&c.worktree, &["rev-parse", "HEAD"])?)?.trim().to_owned();
    create(&c.run_root.join("candidate.json"), &json!({"schema":1,"base_commit":c.base_commit,"candidate_commit":candidate_commit,"patch_sha256":hash(&patch),"changed_paths":paths,"staged_guard":guard,"automatic_merge":false,"automatic_push":false}))?;
    Ok(json!({"outcome":"ready_for_review","candidate_commit":candidate_commit,"changed_paths":paths,"protected_unchanged":true,"protected_files_checked":protected_checked,"patch_sha256":hash(&patch),"staged_guard":guard,"automatic_merge":false,"automatic_push":false,"verification_scope":"pins, path confinement and configured guard only; agent-reported semantic/test evidence requires review"}))
}
fn archive_success(c: &Config, result: &mut Value) -> Result<()> {
    if !matches!(result["outcome"].as_str(), Some("ready_for_review" | "no_change")) {
        return Ok(());
    }
    if c.retain_successful_worktree {
        result["worktree_retained"] = json!(true);
        result["retention_reason"] = json!("explicit retention");
        return Ok(());
    }
    if !git(&c.worktree, &["status", "--porcelain", "--untracked-files=all"])?.is_empty() {
        result["worktree_retained"] = json!(true);
        result["retention_reason"] = json!("dirty evidence retained");
        return Ok(());
    }
    let head = String::from_utf8(git(&c.worktree, &["rev-parse", "HEAD"])?)?.trim().to_owned();
    if head != result["candidate_commit"].as_str().unwrap_or(&c.base_commit) {
        return Err(error("refusing to archive changed candidate HEAD"));
    }
    match git(&c.repository, &["worktree", "remove", c.worktree.to_str().ok_or("worktree utf8")?]) {
        Ok(_) => {
            result["worktree_retained"] = json!(false);
            result["worktree_archived"] = json!(true);
        }
        Err(e) => {
            result["worktree_retained"] = json!(true);
            result["retention_reason"] = json!(e.to_string());
        }
    }
    Ok(())
}
// This lock is deliberately NOT a descendant lifetime lease: it explicitly unlocks so an
// unrelated fork holding a CLOEXEC copy until exec cannot block the next observation.
struct JobOperation(File);
impl Drop for JobOperation {
    fn drop(&mut self) {
        unsafe {
            libc::flock(self.0.as_raw_fd(), libc::LOCK_UN);
        }
    }
}
fn job_operation(path: &Path) -> Result<JobOperation> {
    let file = OpenOptions::new().create(true).truncate(false).read(true).write(true).mode(0o600).open(path)?;
    if unsafe { libc::flock(file.as_raw_fd(), libc::LOCK_EX | libc::LOCK_NB) } != 0 {
        return Err(error("agent job already running"));
    }
    Ok(JobOperation(file))
}
/// Execute one pinned job. Results and uncertain reservations are never replayed.
pub fn run(config: &Value) -> Result<Value> {
    let c = Config::parse(config)?;
    fs::create_dir_all(&c.run_root)?;
    fs::set_permissions(&c.run_root, fs::Permissions::from_mode(0o700))?;
    let _operation = job_operation(&c.run_root.join("agent.lock"))?;
    let identity = hash(serde_json::to_vec(config)?);
    let report = c.run_root.join("report.json");
    if report.exists() {
        let mut v = load(&report, LIMIT)?;
        if v["config_sha256"] != identity {
            return Err(error("agent config identity changed"));
        }
        v["cached"] = json!(true);
        return Ok(v);
    }
    if c.run_root.join("reservation.json").exists() {
        return Err(error("unresolved agent reservation; no automatic replay"));
    }
    prompt(&c)?;
    c.pins(false)?;
    let actual = String::from_utf8(git(&c.repository, &["rev-parse", "--verify", &format!("{}^{{commit}}", c.base_commit)])?)?;
    if actual.trim() != c.base_commit {
        return Err(error("base commit must be exact"));
    }
    if c.worktree.exists() {
        return Err(error("agent worktree must be new"));
    }
    let repo = c.repository.canonicalize()?;
    let parent = c.worktree.parent().ok_or("worktree parent")?;
    fs::create_dir_all(parent)?;
    let target = parent.canonicalize()?.join(c.worktree.file_name().ok_or("worktree filename")?);
    if target.starts_with(&repo) || repo.starts_with(&target) {
        return Err(error("agent worktree must be outside the source repository"));
    }
    let retained = fs::read_dir(parent)?.filter_map(std::result::Result::ok).filter(|e| e.file_type().is_ok_and(|t| t.is_dir())).take(c.max_retained_worktrees).count();
    if retained >= c.max_retained_worktrees {
        return Err(error("retained worktree parent at admission cap; lane share may be short"));
    }
    let floor = storage_floor(&json!({"storage_floor_bytes":c.storage_floor_bytes}))?;
    if !storage_available(&c.run_root, floor)? || !storage_available(parent, floor)? {
        return Err(error("agent storage below configured floor"));
    }
    create(&c.run_root.join("reservation.json"), &json!({"schema":1,"config_sha256":identity,"started_unix":now(),"pid":std::process::id(),"model":c.model,"effort":c.effort,"worktree":c.worktree,"branch":c.branch,"automatic_retry":false}))?;
    let start = Instant::now();
    let result = (|| -> Result<Value> {
        git(&c.repository, &["-c", "core.hooksPath=/dev/null", "worktree", "add", "-b", &c.branch, c.worktree.to_str().ok_or("worktree utf8")?, &c.base_commit])?;
        let call = execute(&c)?;
        if call["outcome"] != "completed_turn" {
            return Ok(call);
        }
        Ok(match finalize(&c) {
            Ok(mut v) => {
                if let Err(e) = archive_success(&c, &mut v) {
                    v["worktree_retained"] = json!(true);
                    v["retention_reason"] = json!(e.to_string());
                }
                v["agent"] = call["result"].clone();
                v["events_sha256"] = call["events_sha256"].clone();
                v
            }
            Err(e) => json!({"outcome":"quarantined","reason":e.to_string(),"agent":call["result"],"events_sha256":call["events_sha256"]}),
        })
    })();
    let mut v = result.unwrap_or_else(|e| json!({"outcome":"failed","reason":e.to_string(),"automatic_retry":false}));
    v["schema"] = json!(1);
    v["config_sha256"] = json!(identity);
    v["elapsed_ms"] = json!(start.elapsed().as_millis().min(u64::MAX as u128) as u64);
    v["worktree"] = json!(c.worktree);
    v["branch"] = json!(c.branch);
    v["model"] = json!(c.model);
    v["effort"] = json!(c.effort);
    v["review_only"] = json!(true);
    v["finished_unix"] = json!(now());
    create(&report, &v)?;
    Ok(v)
}
/// Observe a retained result/reservation without launching an agent.
pub fn status(config: &Value) -> Result<Value> {
    let c = Config::parse(config)?;
    if c.run_root.join("report.json").exists() {
        return load(&c.run_root.join("report.json"), LIMIT);
    }
    if c.run_root.join("reservation.json").exists() {
        return Ok(json!({"state":"reserved_or_uncertain","reservation":load(&c.run_root.join("reservation.json"), LIMIT)?,"automatic_retry":false}));
    }
    Ok(json!({"state":"unstarted","run_root":c.run_root}))
}
#[cfg(test)]
mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(0);
    struct Fixture {
        root: PathBuf,
        c: Value,
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.root);
        }
    }
    fn commit(repo: &Path, msg: &str) -> String {
        git(repo, &["add", "."]).unwrap();
        git(repo, &["-c", "user.email=fixture@example.invalid", "-c", "user.name=Fixture", "commit", "--quiet", "--no-gpg-sign", "-m", msg]).unwrap();
        String::from_utf8(git(repo, &["rev-parse", "HEAD"]).unwrap()).unwrap().trim().to_owned()
    }
    fn fixture(body: &str) -> Fixture {
        let root = std::env::var_os("CARGO_TARGET_DIR").map(PathBuf::from).unwrap_or_else(std::env::temp_dir).join(format!("whale-agent-fixture-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
        let _ = fs::remove_dir_all(&root);
        let repo = root.join("repo");
        fs::create_dir_all(repo.join("src/lib.rs.notes")).unwrap();
        fs::write(repo.join("src/lib.rs"), "pub fn answer()->u32{42}\n").unwrap();
        fs::write(repo.join("src/tests.rs"), "#[test]fn original(){assert_eq!(2+2,4)}\n").unwrap();
        fs::write(repo.join("src/lib.rs.notes/model.md"), "source model\n").unwrap();
        fs::write(repo.join("src/other.rs"), "pub fn other()->u32{7}\n").unwrap();
        fs::write(repo.join("guard.sh"), "#!/bin/sh\nexit 0\n").unwrap();
        fs::set_permissions(repo.join("guard.sh"), fs::Permissions::from_mode(0o755)).unwrap();
        git(&repo, &["init", "--quiet"]).unwrap();
        let base = commit(&repo, "fixture");
        let fake = root.join("fake-codex");
        let script = format!("#!/bin/sh\nprintf called >> \"$(dirname \"$0\")/calls\"\nprintf '%s\\n' \"$@\" > \"$(dirname \"$0\")/args\"\ncat > \"$(dirname \"$0\")/prompt\"\n{body}\nprintf '%s\\n' '{{\"type\":\"item.completed\",\"item\":{{\"type\":\"command_execution\",\"command\":\"fixture\"}}}}' '{{\"type\":\"item.completed\",\"item\":{{\"type\":\"agent_message\",\"text\":\"fixture completed; not a real model\"}}}}' '{{\"type\":\"turn.completed\",\"usage\":{{\"input_tokens\":1,\"output_tokens\":1}}}}'\n");
        fs::write(&fake, script).unwrap();
        fs::set_permissions(&fake, fs::Permissions::from_mode(0o755)).unwrap();
        let tpl = root.join("agent-template.txt");
        fs::write(&tpl, SLOTS.iter().map(|s| format!("{s}\n")).collect::<String>()).unwrap();
        let sha = |p: &str| hash(fs::read(repo.join(p)).unwrap());
        let c = json!({"run_root":root.join("job"),"repository":repo,"base_commit":base,"worktree":root.join("review"),"branch":"agent-review/fixture","branch_prefix":"agent-review/","codex":fake,"model":"gpt-fixture","effort":"medium",
            "prompt_template":{"path":tpl,"sha256":hash(fs::read(&tpl).unwrap())},"timeout_seconds":5,"retain_successful_worktree":true,
            "source_bindings":[{"path":"src/lib.rs","sha256":sha("src/lib.rs")},{"path":"src/tests.rs","sha256":sha("src/tests.rs")}],
            "sidecar_suffixes":[".notes"],"allowed_write_dirs":["src/lib.rs.notes"],"allowed_write_files":["src/tests.rs"],
            "write_file_rule":{"suffix":".rs","path_contains_any":["/tests/"],"name_equals_any":["tests.rs"],"name_ends_with_any":["_tests.rs"]},
            "protected_suffixes":[".rs"],"staged_guard":"guard.sh","task":"Add one source-grounded note and a focused separate test.","storage_floor_bytes":1});
        Fixture { root, c }
    }
    #[test]
    fn isolated_worktree_edits_are_staged_not_merged() {
        let _serial = crate::serial();
        let f = fixture("printf 'new note\\n' > src/lib.rs.notes/research.md\nprintf '#[test]fn added(){assert!(true)}\\n' >> src/tests.rs");
        let r = run(&f.c).unwrap();
        assert_eq!(r["outcome"], "ready_for_review");
        assert_eq!((r["protected_unchanged"].clone(), r["automatic_merge"].clone(), r["staged_guard"].clone()), (json!(true), json!(false), json!("passed")));
        assert_eq!(fs::read(f.root.join("calls")).unwrap(), b"called");
        assert_eq!(fs::read_to_string(f.root.join("repo/src/tests.rs")).unwrap(), "#[test]fn original(){assert_eq!(2+2,4)}\n");
        assert!(f.root.join("review/src/lib.rs.notes/research.md").is_file());
        assert!(f.root.join("job/candidate.patch").is_file());
        let args = fs::read_to_string(f.root.join("args")).unwrap();
        assert!(args.contains("workspace-write") && args.contains("sandbox_workspace_write.network_access=false") && args.contains("gpt-fixture"));
        assert!(!args.contains("dangerously"));
        let p = fs::read_to_string(f.root.join("prompt")).unwrap();
        assert!(p.contains("Add one source-grounded note") && p.contains("src/lib.rs.notes"));
    }
    #[test]
    fn altered_or_slotless_prompt_template_is_refused_without_calling() {
        let _serial = crate::serial();
        let f = fixture(":");
        let tpl = PathBuf::from(f.c["prompt_template"]["path"].as_str().unwrap());
        fs::write(&tpl, "changed {{TASK}}").unwrap();
        assert!(run(&f.c).unwrap_err().to_string().contains("template pin"));
        let mut c = f.c.clone();
        c["prompt_template"]["sha256"] = json!(hash(fs::read(&tpl).unwrap()));
        assert!(run(&c).unwrap_err().to_string().contains("exactly once"));
        assert!(!f.root.join("calls").exists());
        assert!(!f.root.join("job/reservation.json").exists());
    }
    #[test]
    fn cached_result_does_not_call_again_or_accept_changed_config() {
        let _serial = crate::serial();
        let mut f = fixture("printf 'review\\n' > src/lib.rs.notes/review.md");
        assert_eq!(run(&f.c).unwrap()["outcome"], "ready_for_review");
        assert_eq!(run(&f.c).unwrap()["cached"], true);
        assert_eq!(fs::read(f.root.join("calls")).unwrap(), b"called");
        f.c["task"] = json!("different");
        assert!(run(&f.c).unwrap_err().to_string().contains("identity"));
    }
    #[test]
    fn stale_source_fails_before_reservation_or_call() {
        let _serial = crate::serial();
        let mut f = fixture(":");
        f.c["source_bindings"][0]["sha256"] = json!("0".repeat(64));
        assert!(run(&f.c).is_err());
        assert!(!f.root.join("calls").exists());
        assert!(!f.root.join("job/reservation.json").exists());
    }
    #[test]
    fn protected_change_is_quarantined_parent_untouched() {
        let _serial = crate::serial();
        let f = fixture("printf 'pub fn answer()->u32{0}\\n' > src/lib.rs");
        let r = run(&f.c).unwrap();
        assert_eq!(r["outcome"], "quarantined");
        assert!(r["reason"].as_str().unwrap().contains("pin changed"));
        assert_eq!(fs::read_to_string(f.root.join("repo/src/lib.rs")).unwrap(), "pub fn answer()->u32{42}\n");
        assert!(!f.root.join("job/candidate.patch").exists());
    }
    #[test]
    fn outside_paths_and_symlinks_are_not_admitted() {
        let _serial = crate::serial();
        for body in ["printf evil > outside.txt", "ln -s ../tests.rs src/lib.rs.notes/link.rs", "mkdir -p src/lib.rs.notes-evil; printf evil > src/lib.rs.notes-evil/a.md"] {
            let f = fixture(body);
            let r = run(&f.c).unwrap();
            assert_eq!(r["outcome"], "quarantined", "{body}");
            assert!(!f.root.join("job/candidate.patch").exists());
        }
    }
    #[test]
    fn timeout_is_uncertain_and_never_replayed() {
        let _serial = crate::serial();
        let mut f = fixture("sleep 5");
        f.c["timeout_seconds"] = json!(1);
        let r = run(&f.c).unwrap();
        assert_eq!((r["outcome"].clone(), r["reason"].clone()), (json!("uncertain"), json!("timeout")));
        assert_eq!(run(&f.c).unwrap()["cached"], true);
        assert_eq!(fs::read(f.root.join("calls")).unwrap(), b"called");
    }
    #[test]
    fn unresolved_reservation_and_bad_scope_do_not_call() {
        let _serial = crate::serial();
        let f = fixture(":");
        fs::create_dir_all(f.root.join("job")).unwrap();
        create(&f.root.join("job/reservation.json"), &json!({"reserved":true})).unwrap();
        assert!(run(&f.c).unwrap_err().to_string().contains("unresolved"));
        assert!(!f.root.join("calls").exists());
        let mut c = f.c.clone();
        c["allowed_write_files"] = json!(["src/lib.rs"]);
        assert!(Config::parse(&c).is_err());
        c = f.c.clone();
        c["allowed_write_dirs"] = json!(["../outside.notes"]);
        assert!(Config::parse(&c).is_err());
        c = f.c.clone();
        c["branch"] = json!("main");
        assert!(Config::parse(&c).is_err());
        c = f.c.clone();
        c["model"] = json!("claude-fixture");
        assert!(Config::parse(&c).is_err());
    }
    #[test]
    fn hidden_unselected_protected_change_is_detected_by_raw_blob_hashes() {
        let _serial = crate::serial();
        let f = fixture("git update-index --assume-unchanged src/other.rs; printf changed > src/other.rs");
        let r = run(&f.c).unwrap();
        assert_eq!(r["outcome"], "quarantined");
        assert!(r["reason"].as_str().unwrap().contains("protected bytes changed"));
    }
    #[test]
    fn staged_guard_has_a_real_rejection_control() {
        let _serial = crate::serial();
        let mut f = fixture("printf review > src/lib.rs.notes/review.md");
        let repo = f.root.join("repo");
        fs::write(repo.join("guard.sh"), "#!/bin/sh\nexit 1\n").unwrap();
        f.c["base_commit"] = json!(commit(&repo, "guard negative control"));
        let r = run(&f.c).unwrap();
        assert_eq!(r["outcome"], "quarantined");
        assert!(r["reason"].as_str().unwrap().contains("staged guard"));
        assert!(!f.root.join("job/candidate.patch").exists());
    }
    #[test]
    fn pinned_scope_manifest_is_admitted_but_unpinned_or_mismatched_scope_is_not() {
        let _serial = crate::serial();
        let mut f = fixture("printf reviewed > models/groups/research/review.md");
        let repo = f.root.join("repo");
        let dir = "models/groups/research";
        fs::create_dir_all(repo.join(dir)).unwrap();
        let path = format!("{dir}/manifest.json");
        let bytes = serde_json::to_vec(&json!({"schema":1,"kind":"group","id":"research-fixture"})).unwrap();
        fs::write(repo.join(&path), &bytes).unwrap();
        f.c["base_commit"] = json!(commit(&repo, "scope fixture"));
        f.c["allowed_write_dirs"] = json!([dir]);
        assert!(Config::parse(&f.c).is_err());
        f.c["scope_bindings"] = json!([{"path":path,"sha256":hash(&bytes)}]);
        f.c["scope_manifest_equals"] = json!({"kind":"other"});
        f.c["reference_gap"] = json!("restricted assets remain unread");
        assert!(run(&f.c).unwrap_err().to_string().contains("does not match"));
        assert!(!f.root.join("calls").exists());
        f.c["scope_manifest_equals"] = json!({"kind":"group"});
        let cfg = Config::parse(&f.c).unwrap();
        assert!(prompt(&cfg).unwrap().contains("restricted assets remain unread"));
        assert_eq!(run(&f.c).unwrap()["outcome"], "ready_for_review");
    }
    #[test]
    fn successful_candidate_is_reconstructable_after_clean_worktree_archive() {
        let _serial = crate::serial();
        let mut f = fixture("printf reviewed > src/lib.rs.notes/review.md");
        f.c["retain_successful_worktree"] = json!(false);
        let r = run(&f.c).unwrap();
        assert_eq!((r["outcome"].clone(), r["worktree_archived"].clone()), (json!("ready_for_review"), json!(true)));
        assert!(!f.root.join("review").exists());
        let candidate = load(&f.root.join("job/candidate.json"), LIMIT).unwrap();
        assert_eq!(candidate["candidate_commit"], r["candidate_commit"]);
        let object = format!("{}:src/lib.rs.notes/review.md", r["candidate_commit"].as_str().unwrap());
        assert_eq!(git(&f.root.join("repo"), &["show", &object]).unwrap(), b"reviewed");
        assert_eq!(run(&f.c).unwrap()["cached"], true);
    }
    #[test]
    fn retained_parent_cap_refuses_before_reservation_or_call() {
        let _serial = crate::serial();
        let mut f = fixture(":");
        f.c["max_retained_worktrees"] = json!(1);
        assert!(run(&f.c).unwrap_err().to_string().contains("admission cap"));
        assert!(!f.root.join("calls").exists());
        assert!(!f.root.join("job/reservation.json").exists());
    }
    #[test]
    fn completion_is_not_test_pass_evidence() {
        let _serial = crate::serial();
        let b = b"{\"type\":\"item.completed\",\"item\":{\"type\":\"command_execution\"}}\n{\"type\":\"turn.completed\"}\n";
        let v = decode(b).unwrap();
        assert_eq!((v["completed_command_events"].clone(), v["tests_verified_from_events"].clone()), (json!(1), json!(false)));
        assert!(decode(b"{\"type\":\"turn.failed\"}\n").is_err());
        assert!(decode(b"{\"type\":\"item.completed\"}\n").is_err());
        assert!(decode(b"{\"type\":\"turn.completed\"}\n{\"type\":\"turn.completed\"}\n").is_err());
    }
}
