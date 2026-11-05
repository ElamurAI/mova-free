//! Crab's work-flow probes: does every continuously running worker have work, and is any of its
//! stacks clogged? A probe is pure configuration — where a worker's input queue, its stacks
//! (pending, quarantine, uncertain, unconsumed output) and its progress signal are, how to count
//! them, and the low/high water marks. Crab has no worker-specific code.
//!
//! Model (per probe p, per evaluation t; readings are unsigned counts or "unknown"):
//!   starved_raw   = input < low_water
//!   clogged_raw   = some stack > high_water          (clears only at or below resume_at)
//!   stalled_raw   = input >= 1 and now - last_progress_change >= stall_seconds
//!   limited_raw   = a configured limit pattern in a recent log tail
//! A condition becomes active after `hysteresis` consecutive raw-true evaluations and clears
//! after `hysteresis` consecutive raw-false ones; an unknown reading changes neither streak.
//! Actions happen only on a transition (or to restore the configured pause state), so a
//! condition sitting active does not repeat its action. Precedence: limited > clogged > starved
//! > stalled. Every action and every transition is journaled with the probe values.
//!
//! Actions are bounded to: start a configured producer worker that is stopped and was not
//! explicitly stopped; create (pause) or remove (resume) the producer's PAUSE file — only a
//! file Crab itself wrote; restart the probe's worker after a stall when the probe allows it;
//! hold (do nothing automatic) while a limit marker is present. Everything else is a report
//! and an attention item for the Octopus. No process is killed by this module.
use crate::Result;
use serde::Deserialize;
use serde_json::{json, Value};
use std::{
    collections::BTreeMap,
    fs::{self, File, OpenOptions},
    io::{BufRead, BufReader, Read, Seek, SeekFrom, Write},
    os::unix::fs::{DirBuilderExt, MetadataExt, OpenOptionsExt},
    path::{Component, Path, PathBuf},
    time::UNIX_EPOCH,
};

const PROBES_MAX: usize = 64;
const STACKS_MAX: usize = 8;
const SUM_MAX: usize = 16;
const DIR_ENTRIES_MAX: usize = 100_000;
const JSON_MAX: u64 = 1 << 20;
const JSONL_MAX: u64 = 256 << 20;
const TAIL_MAX: u64 = 1 << 20;
const GLOB_TOP: usize = 4;
const PATTERNS_MAX: usize = 16;
const JOURNAL_MAX: u64 = 64 << 20;
/// Limit markers of the ChatGPT/Codex and Claude CLIs; matched ASCII-case-insensitively.
pub const DEFAULT_LIMIT_PATTERNS: [&str; 6] = [
    "hit your usage limit",
    "usage limit reached",
    "rate limit",
    "rate_limit_exceeded",
    "too many requests",
    "limit will reset",
];
/// Actions a council verdict may name; flow executes the same set.
pub const ACTIONS: [&str; 5] = ["start_producer", "pause_producer", "resume_producer", "hold", "restart"];

fn error(s: impl Into<String>) -> Box<dyn std::error::Error> {
    s.into().into()
}
fn d_interval() -> u64 {
    60
}
fn d_hysteresis() -> u32 {
    2
}
fn d_tail() -> u64 {
    65536
}
fn d_age() -> u64 {
    3600
}
fn d_low() -> u64 {
    1
}

/// How to count one thing on disk.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(tag = "kind", rename_all = "snake_case", deny_unknown_fields)]
pub enum Source {
    /// Entries of a directory (files or directories), optionally by name prefix/suffix.
    DirEntries {
        path: PathBuf,
        #[serde(default)]
        prefix: Option<String>,
        #[serde(default)]
        suffix: Option<String>,
    },
    /// Non-empty lines of a JSONL file (missing file: 0).
    JsonlLines { path: PathBuf },
    /// An unsigned integer at a dotted field path of a JSON file; `missing` when absent.
    JsonField {
        path: PathBuf,
        field: String,
        #[serde(default)]
        missing: Option<u64>,
    },
    /// 1 when a dotted field of a JSON file equals this string, else 0 (a state as a count).
    JsonEquals { path: PathBuf, field: String, value: String },
    /// Modification time (Unix seconds); for progress.
    Mtime { path: PathBuf },
    /// Sum of plain sources (no nested sums).
    Sum { of: Vec<Source> },
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Gauge {
    pub source: Source,
    #[serde(default = "d_low")]
    pub low_water: u64,
}
/// What a stack is. A backlog (input, ready or in-flight work) drains by itself once its
/// producer pauses; a quarantine is a review backlog that never drains by itself, so pausing on
/// it would park a working worker forever: it is watched for its rate instead.
#[derive(Clone, Copy, Debug, Default, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum StackRole {
    #[default]
    Backlog,
    Quarantine,
}
/// Where a quarantine's dominant reason is read: a string at a dotted field of a JSON file.
#[derive(Clone, Debug, Deserialize, PartialEq)]
#[serde(deny_unknown_fields)]
pub struct ReasonSource {
    pub path: PathBuf,
    pub field: String,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Stack {
    pub name: String,
    /// A backlog: the current count. A quarantine: a cumulative count of quarantined items.
    pub source: Source,
    #[serde(default)]
    pub role: StackRole,
    /// Required for a backlog; refused for a quarantine.
    #[serde(default)]
    pub high_water: Option<u64>,
    /// Clogged clears only at or below this (default high_water / 2): a hysteresis band.
    #[serde(default)]
    pub resume_at: Option<u64>,
    /// Pausing the producer helps this backlog (default true); otherwise it is only reported,
    /// as for an "uncertain" state that pausing cannot clear. Refused for a quarantine.
    #[serde(default)]
    pub pause_producer: Option<bool>,
    /// Quarantine: the cumulative count of finished items (quarantined ones included).
    #[serde(default)]
    pub finished: Option<Source>,
    /// Quarantine: the window of the rate Δquarantined/Δfinished (default 3600, 300..86400).
    #[serde(default)]
    pub rate_window_seconds: Option<u64>,
    /// Quarantine: the rate above which the anomaly is raised (default 0.5, (0, 1]).
    #[serde(default)]
    pub max_rate: Option<f64>,
    /// Quarantine: the fewest finished items in the window that make a rate (default 4).
    #[serde(default)]
    pub min_finished: Option<u64>,
    /// Quarantine: an optional dominant reason, reported with the anomaly.
    #[serde(default)]
    pub reason_source: Option<ReasonSource>,
}
impl Stack {
    /// The high water of a backlog (a quarantine has none).
    pub fn hw(&self) -> u64 {
        self.high_water.unwrap_or(0)
    }
    pub fn resume(&self) -> u64 {
        self.resume_at.unwrap_or(self.hw() / 2)
    }
    /// Only a backlog that declares (or defaults to) pause_producer pauses its producer.
    pub fn pauses(&self) -> bool {
        self.role == StackRole::Backlog && self.pause_producer.unwrap_or(true)
    }
    fn window(&self) -> u64 {
        self.rate_window_seconds.unwrap_or(3600)
    }
    fn max_rate(&self) -> f64 {
        self.max_rate.unwrap_or(0.5)
    }
    fn min_finished(&self) -> u64 {
        self.min_finished.unwrap_or(4)
    }
    fn validate(&self, probe: &str) -> Result<()> {
        let bad = |why: &str| Err(error(format!("probe {probe}: stack {}: {why}", self.name)));
        match self.role {
            StackRole::Backlog => {
                let Some(hw) = self.high_water else { return bad("a backlog stack needs high_water") };
                if self.finished.is_some() || self.rate_window_seconds.is_some() || self.max_rate.is_some() || self.min_finished.is_some() || self.reason_source.is_some() {
                    return bad("finished, rate_window_seconds, max_rate, min_finished and reason_source belong to a quarantine stack");
                }
                if self.resume_at.is_some_and(|r| r > hw || (r == hw && r > 0)) {
                    return bad("resume_at < high_water");
                }
                // A quarantine never drains by itself: pausing on it parks a working worker.
                if self.name.to_ascii_lowercase().contains("quarantin") && self.pauses() {
                    return bad("a quarantine stack never drains by itself: declare role quarantine (a rate anomaly, never a pause) or pause_producer false (reporting only)");
                }
            }
            StackRole::Quarantine => {
                if self.high_water.is_some() || self.resume_at.is_some() || self.pause_producer.is_some() {
                    return bad("a quarantine stack never pauses: high_water, resume_at and pause_producer are refused");
                }
                let Some(f) = &self.finished else { return bad("a quarantine stack needs a finished count source") };
                check_source(f, false)?;
                if !(300..=86400).contains(&self.window()) || !(self.max_rate() > 0.0 && self.max_rate() <= 1.0) || !(1..=100_000).contains(&self.min_finished()) {
                    return bad("rate_window_seconds 300..86400, max_rate in (0, 1], min_finished 1..100000");
                }
                if let Some(r) = &self.reason_source {
                    if !trusted_path(&r.path) || r.field.is_empty() || r.field.len() > 256 || r.field.split('.').any(str::is_empty) {
                        return bad("reason_source needs an absolute trusted path and a dotted field");
                    }
                }
            }
        }
        Ok(())
    }
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Progress {
    /// Any change of this value is progress.
    pub source: Source,
    pub stall_seconds: u64,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Producer {
    /// A configured Crab worker Crab may start when the input starves.
    #[serde(default)]
    pub worker: Option<String>,
    /// The PAUSE file the producer honours; Crab creates it when a stack clogs.
    #[serde(default)]
    pub pause_file: Option<PathBuf>,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct LimitWatch {
    /// Absolute paths; a `*` segment matches any entry (the newest few by mtime are read).
    pub files: Vec<PathBuf>,
    #[serde(default)]
    pub patterns: Vec<String>,
    #[serde(default = "d_tail")]
    pub tail_bytes: u64,
    /// Only files modified within this window count.
    #[serde(default = "d_age")]
    pub max_age_seconds: u64,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Probe {
    pub name: String,
    /// The animal kind addressed in English and reviewed for capacity ("fish", "eel",
    /// "nautilus", "turtle", "whale", "owl", any configured kind). Also accepted as `kind`.
    #[serde(alias = "kind")]
    pub group: String,
    /// The Crab worker this probe watches (restart policy); absent for unsupervised workers.
    #[serde(default)]
    pub worker: Option<String>,
    #[serde(default)]
    pub input: Option<Gauge>,
    #[serde(default)]
    pub stacks: Vec<Stack>,
    #[serde(default)]
    pub progress: Option<Progress>,
    #[serde(default)]
    pub producer: Option<Producer>,
    #[serde(default)]
    pub limit: Option<LimitWatch>,
    #[serde(default)]
    pub restart_on_stall: bool,
    #[serde(default = "d_hysteresis")]
    pub hysteresis: u32,
}
#[derive(Clone, Debug, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FlowConfig {
    #[serde(default = "d_interval")]
    pub interval_seconds: u64,
    #[serde(default)]
    pub probes: Vec<Probe>,
    /// Work pools: Crab hands out jobs to idle members (`pool.rs`).
    #[serde(default)]
    pub pools: Vec<crate::pool::PoolConfig>,
}
fn ident(s: &str) -> bool {
    crate::workers::valid_name(s)
}
fn trusted_path(p: &Path) -> bool {
    p.is_absolute()
        && p.to_str().is_some_and(|s| s.len() <= 1024 && !s.contains('\0'))
        && !p.components().any(|c| matches!(c, Component::ParentDir))
}
fn check_source(s: &Source, nested: bool) -> Result<()> {
    let ok = match s {
        Source::DirEntries { path, prefix, suffix } => {
            trusted_path(path)
                && prefix.as_ref().is_none_or(|x| x.len() <= 64 && !x.contains('/'))
                && suffix.as_ref().is_none_or(|x| x.len() <= 64 && !x.contains('/'))
        }
        Source::JsonlLines { path } | Source::Mtime { path } => trusted_path(path),
        Source::JsonEquals { value, .. } if value.len() > 128 => false,
        Source::JsonField { path, field, .. } | Source::JsonEquals { path, field, .. } => {
            trusted_path(path)
                && !field.is_empty()
                && field.len() <= 256
                && field.split('.').all(|k| !k.is_empty())
        }
        Source::Sum { of } => {
            if nested || of.is_empty() || of.len() > SUM_MAX {
                return Err(error("a sum holds 1..16 plain sources"));
            }
            for x in of {
                check_source(x, true)?;
            }
            true
        }
    };
    if !ok {
        return Err(error("probe sources need absolute trusted paths without '..' and bounded names"));
    }
    Ok(())
}
impl FlowConfig {
    /// Validate against the configured worker names.
    pub fn validate(&self, workers: &[String]) -> Result<()> {
        if !(10..=3600).contains(&self.interval_seconds) {
            return Err(error("flow interval_seconds must be 10..3600"));
        }
        if self.probes.len() > PROBES_MAX {
            return Err(error("at most 64 flow probes"));
        }
        if self.pools.len() > 8 {
            return Err(error("at most 8 work pools"));
        }
        let mut pool_names = std::collections::BTreeSet::new();
        let mut pool_workers = std::collections::BTreeSet::new();
        for p in &self.pools {
            p.validate(workers)?;
            if !pool_names.insert(p.name.clone()) || !p.members.iter().all(|m| pool_workers.insert(m.worker.clone())) {
                return Err(error("pool names are unique and a worker belongs to one pool"));
            }
        }
        let mut names = std::collections::BTreeSet::new();
        for p in &self.probes {
            if !ident(&p.name) || !names.insert(p.name.clone()) || !ident(&p.group) || p.group.len() > 32 {
                return Err(error("probe names must be unique ASCII identifiers with an ASCII group"));
            }
            let known = |w: &Option<String>| w.as_ref().is_none_or(|w| workers.contains(w));
            if !known(&p.worker) || p.producer.as_ref().is_some_and(|x| !known(&x.worker)) {
                return Err(error(format!("probe {}: a named worker must be a configured worker", p.name)));
            }
            if !(1..=10).contains(&p.hysteresis) || p.stacks.len() > STACKS_MAX {
                return Err(error(format!("probe {}: hysteresis 1..10 and at most 8 stacks", p.name)));
            }
            if p.input.is_none() && p.stacks.is_empty() && p.progress.is_none() && p.limit.is_none() {
                return Err(error(format!("probe {} observes nothing", p.name)));
            }
            if let Some(g) = &p.input {
                check_source(&g.source, false)?;
            }
            for s in &p.stacks {
                check_source(&s.source, false)?;
                if !ident(&s.name) {
                    return Err(error(format!("probe {}: stack names are identifiers", p.name)));
                }
                s.validate(&p.name)?;
            }
            if let Some(g) = &p.progress {
                check_source(&g.source, false)?;
                if !(60..=7 * 86400).contains(&g.stall_seconds) {
                    return Err(error(format!("probe {}: stall_seconds 60..604800", p.name)));
                }
            }
            if let Some(x) = &p.producer {
                if x.worker.is_none() && x.pause_file.is_none() {
                    return Err(error(format!("probe {}: a producer names a worker or a pause_file", p.name)));
                }
                if x.pause_file.as_ref().is_some_and(|f| !trusted_path(f)) {
                    return Err(error(format!("probe {}: pause_file must be an absolute trusted path", p.name)));
                }
            }
            if let Some(l) = &p.limit {
                if l.files.is_empty()
                    || l.files.len() > 8
                    || !l.files.iter().all(|f| trusted_path(f))
                    || l.patterns.len() > PATTERNS_MAX
                    || l.patterns.iter().any(|x| x.trim().is_empty() || x.len() > 128)
                    || !(1024..=TAIL_MAX).contains(&l.tail_bytes)
                    || !(60..=7 * 86400).contains(&l.max_age_seconds)
                {
                    return Err(error(format!("probe {}: limit watch needs 1..8 trusted files, <= 16 patterns, tail 1024..1048576, age 60..604800", p.name)));
                }
            }
            if p.restart_on_stall && (p.worker.is_none() || p.progress.is_none()) {
                return Err(error(format!("probe {}: restart_on_stall needs its worker and a progress signal", p.name)));
            }
        }
        Ok(())
    }
}

// ---------------------------------------------------------------- reading
fn read_bounded(p: &Path, max: u64) -> std::result::Result<Vec<u8>, String> {
    let mut b = Vec::new();
    File::open(p)
        .and_then(|f| f.take(max + 1).read_to_end(&mut b))
        .map_err(|e| format!("{}: {e}", p.display()))?;
    if b.len() as u64 > max {
        return Err(format!("{}: exceeds the bounded read", p.display()));
    }
    Ok(b)
}
/// One reading. Unknown readings are explicit errors, never zeros.
pub fn read(s: &Source) -> std::result::Result<u64, String> {
    match s {
        Source::DirEntries { path, prefix, suffix } => {
            let mut n = 0u64;
            for (i, e) in fs::read_dir(path).map_err(|e| format!("{}: {e}", path.display()))?.enumerate() {
                if i >= DIR_ENTRIES_MAX {
                    return Err(format!("{}: more than {DIR_ENTRIES_MAX} entries", path.display()));
                }
                let name = e.map_err(|e| e.to_string())?.file_name();
                let name = name.to_string_lossy();
                if name.starts_with('.') || name.ends_with(".tmp") {
                    continue;
                }
                if prefix.as_ref().is_none_or(|x| name.starts_with(x.as_str())) && suffix.as_ref().is_none_or(|x| name.ends_with(x.as_str())) {
                    n += 1;
                }
            }
            Ok(n)
        }
        Source::JsonlLines { path } => {
            let f = match File::open(path) {
                Ok(f) => f,
                Err(e) if e.kind() == std::io::ErrorKind::NotFound => return Ok(0),
                Err(e) => return Err(format!("{}: {e}", path.display())),
            };
            if f.metadata().map(|m| m.len()).unwrap_or(0) > JSONL_MAX {
                return Err(format!("{}: exceeds the bounded read", path.display()));
            }
            let mut n = 0;
            for line in BufReader::new(f.take(JSONL_MAX)).split(b'\n') {
                if !line.map_err(|e| e.to_string())?.iter().all(u8::is_ascii_whitespace) {
                    n += 1;
                }
            }
            Ok(n)
        }
        Source::JsonField { path, field, missing } => {
            let v: Value = serde_json::from_slice(&read_bounded(path, JSON_MAX)?).map_err(|e| format!("{}: {e}", path.display()))?;
            let mut at = &v;
            for k in field.split('.') {
                at = &at[k];
            }
            match (at.as_u64(), missing) {
                (Some(x), _) => Ok(x),
                (None, Some(m)) if at.is_null() => Ok(*m),
                _ => Err(format!("{}: {field} is not an unsigned integer", path.display())),
            }
        }
        Source::JsonEquals { path, field, value } => {
            let v: Value = serde_json::from_slice(&read_bounded(path, JSON_MAX)?).map_err(|e| format!("{}: {e}", path.display()))?;
            let mut at = &v;
            for k in field.split('.') {
                at = &at[k];
            }
            Ok(u64::from(at.as_str() == Some(value.as_str())))
        }
        Source::Mtime { path } => fs::metadata(path)
            .and_then(|m| m.modified())
            .map(|t| t.duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0))
            .map_err(|e| format!("{}: {e}", path.display())),
        Source::Sum { of } => of.iter().try_fold(0u64, |a, s| read(s).map(|x| a.saturating_add(x))),
    }
}
/// Expand `*` segments: at each, the newest GLOB_TOP entries by mtime (ties by name).
fn expand(pattern: &Path) -> Vec<PathBuf> {
    let mut out = vec![PathBuf::from("/")];
    for c in pattern.components().skip(1) {
        let seg = c.as_os_str().to_string_lossy().into_owned();
        let mut next = Vec::new();
        for base in &out {
            if seg == "*" {
                let Ok(rd) = fs::read_dir(base) else { continue };
                let mut v: Vec<(u64, PathBuf)> = rd
                    .take(DIR_ENTRIES_MAX)
                    .filter_map(|e| e.ok())
                    .map(|e| {
                        let t = e.metadata().ok().and_then(|m| m.modified().ok()).and_then(|t| t.duration_since(UNIX_EPOCH).ok()).map(|d| d.as_secs()).unwrap_or(0);
                        (t, e.path())
                    })
                    .collect();
                v.sort_by(|a, b| b.0.cmp(&a.0).then(a.1.cmp(&b.1)));
                next.extend(v.into_iter().take(GLOB_TOP).map(|x| x.1));
            } else {
                next.push(base.join(&seg));
            }
        }
        out = next;
    }
    out
}
fn tail(p: &Path, n: u64) -> Option<Vec<u8>> {
    let mut f = File::open(p).ok()?;
    let len = f.metadata().ok()?.len();
    f.seek(SeekFrom::Start(len.saturating_sub(n))).ok()?;
    let mut b = Vec::new();
    f.take(n).read_to_end(&mut b).ok()?;
    Some(b)
}
/// The first configured limit pattern found in a recent tail, with its file.
pub fn limit_hit(l: &LimitWatch, now: u64) -> Option<(String, PathBuf)> {
    let patterns: Vec<String> = if l.patterns.is_empty() {
        DEFAULT_LIMIT_PATTERNS.iter().map(|s| s.to_string()).collect()
    } else {
        l.patterns.clone()
    };
    let mut files: Vec<PathBuf> = l.files.iter().flat_map(|f| expand(f)).collect();
    files.sort();
    files.dedup();
    for f in files {
        let fresh = fs::metadata(&f)
            .ok()
            .filter(|m| m.is_file())
            .and_then(|m| m.modified().ok())
            .and_then(|t| t.duration_since(UNIX_EPOCH).ok())
            .is_some_and(|d| d.as_secs() + l.max_age_seconds >= now);
        if !fresh {
            continue;
        }
        let Some(b) = tail(&f, l.tail_bytes) else { continue };
        let text = String::from_utf8_lossy(&b).to_ascii_lowercase();
        if let Some(p) = patterns.iter().find(|p| text.contains(&p.to_ascii_lowercase())) {
            return Some((p.clone(), f));
        }
    }
    None
}
/// A quarantine's dominant reason: a string (bounded to 200 bytes, control characters blanked)
/// at a dotted field; absent, unreadable or not a string reads as no reason.
pub fn read_reason(r: &ReasonSource) -> Option<String> {
    let v: Value = serde_json::from_slice(&read_bounded(&r.path, JSON_MAX).ok()?).ok()?;
    let mut at = &v;
    for k in r.field.split('.') {
        at = &at[k];
    }
    let t: String = at.as_str()?.trim().chars().map(|c| if c.is_control() { ' ' } else { c }).collect();
    if t.is_empty() {
        return None;
    }
    let mut end = t.len().min(200);
    while !t.is_char_boundary(end) {
        end -= 1;
    }
    Some(t[..end].to_string())
}
/// Everything read for one probe at one time.
#[derive(Clone, Debug, Default)]
pub struct Reading {
    pub input: Option<std::result::Result<u64, String>>,
    pub stacks: Vec<(String, std::result::Result<u64, String>)>,
    /// Per stack, aligned with `stacks`: a quarantine's finished count.
    pub finished: Vec<Option<std::result::Result<u64, String>>>,
    /// Per stack, aligned with `stacks`: a quarantine's dominant reason.
    pub reasons: Vec<Option<String>>,
    pub progress: Option<std::result::Result<u64, String>>,
    pub limit: Option<(String, PathBuf)>,
}
pub fn observe(p: &Probe, now: u64) -> Reading {
    Reading {
        input: p.input.as_ref().map(|g| read(&g.source)),
        stacks: p.stacks.iter().map(|s| (s.name.clone(), read(&s.source))).collect(),
        finished: p.stacks.iter().map(|s| s.finished.as_ref().map(read)).collect(),
        reasons: p.stacks.iter().map(|s| s.reason_source.as_ref().and_then(read_reason)).collect(),
        progress: p.progress.as_ref().map(|g| read(&g.source)),
        limit: p.limit.as_ref().and_then(|l| limit_hit(l, now)),
    }
}

// ---------------------------------------------------------------- assessment
pub const CONDITIONS: [&str; 4] = ["limited", "clogged", "starved", "stalled"];
/// Hysteresis state of one condition.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, Deserialize)]
pub struct Latch {
    pub active: bool,
    pub on: u32,
    pub off: u32,
}
impl Latch {
    /// Feed one raw observation (None: unknown, streaks unchanged). Returns Some(new) on a flip.
    fn feed(&mut self, raw: Option<bool>, n: u32) -> Option<bool> {
        match raw {
            None => None,
            Some(true) => {
                self.on += 1;
                self.off = 0;
                if !self.active && self.on >= n {
                    self.active = true;
                    return Some(true);
                }
                None
            }
            Some(false) => {
                self.off += 1;
                self.on = 0;
                if self.active && self.off >= n {
                    self.active = false;
                    return Some(false);
                }
                None
            }
        }
    }
}
/// Durable memory of one probe across evaluations.
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, Deserialize)]
pub struct Memory {
    pub latches: BTreeMap<String, Latch>,
    pub last_progress: Option<u64>,
    pub last_progress_change: Option<u64>,
    pub paused_by_crab: bool,
    pub restarted_for_stall: bool,
    pub hold_until: u64,
    pub last_values: Value,
    pub last_evaluated: u64,
    /// Recent (time, progress) samples for the throughput estimate; at most SAMPLES_MAX.
    #[serde(default)]
    pub samples: Vec<(u64, u64)>,
    /// Since when the starved condition has been active.
    #[serde(default)]
    pub starved_since: Option<u64>,
    /// The current starved episode has had its one decision.
    #[serde(default)]
    pub starved_acted: bool,
    /// Per quarantine stack: the rate window and its anomaly latch.
    #[serde(default)]
    pub quarantine: BTreeMap<String, QuarantineMemory>,
}
const SAMPLES_MAX: usize = 32;
const QSAMPLES_MAX: usize = 256;
/// The rate memory of one quarantine stack: (time, quarantined, finished) samples within the
/// window, and the anomaly latch (same hysteresis as the conditions).
#[derive(Clone, Debug, Default, PartialEq, serde::Serialize, Deserialize)]
pub struct QuarantineMemory {
    pub samples: Vec<(u64, u64, u64)>,
    pub latch: Latch,
}
/// One quarantine-rate observation. `raw` is None while the window holds fewer than
/// min_finished finished items or a reading is unknown (the latch is then unchanged).
fn quarantine_rate(s: &Stack, q: &std::result::Result<u64, String>, f: Option<&std::result::Result<u64, String>>, qm: &mut QuarantineMemory, now: u64) -> (Option<bool>, Value) {
    let (Ok(q), Some(Ok(f))) = (q, f) else {
        return (None, json!({"unknown":true}));
    };
    // A counter that went down (a restart, a sealed or new audit) starts a new window.
    if qm.samples.last().is_some_and(|l| *q < l.1 || *f < l.2) {
        qm.samples.clear();
    }
    qm.samples.push((now, *q, *f));
    let from = now.saturating_sub(s.window());
    while qm.samples.len() > 1 && qm.samples[0].0 < from {
        qm.samples.remove(0);
    }
    if qm.samples.len() > QSAMPLES_MAX {
        qm.samples.remove(1);
    }
    let b = qm.samples[0];
    let (dq, df) = (q - b.1, f - b.2);
    let rate = (df > 0).then(|| dq as f64 / df as f64);
    let raw = if df >= s.min_finished() { rate.map(|r| r > s.max_rate()) } else { None };
    (raw, json!({"quarantined":dq,"finished":df,"window_seconds":now.saturating_sub(b.0),"rate":rate.map(|r| (r * 1000.0).round() / 1000.0),"max_rate":s.max_rate(),"min_finished":s.min_finished()}))
}
/// One deterministic decision for one probe.
#[derive(Clone, Debug, PartialEq)]
pub enum Decision {
    None,
    StartProducer(String),
    PauseProducer(PathBuf),
    ResumeProducer(PathBuf),
    Restart(String),
    Hold,
    /// Nothing Crab may do itself: report to the Octopus.
    Report(String),
}
/// The control surface flow needs from the worker supervisor; tests use a fake.
pub trait WorkerControl {
    /// (state, explicitly stopped) of a configured worker.
    fn observe(&self, name: &str) -> Option<(String, bool)>;
    fn start(&mut self, name: &str, now: u64) -> Result<Value>;
    fn restart(&mut self, name: &str, now: u64) -> Result<Value>;
}
const PAUSE_TEXT: &str = "crab flow pause";
fn pause_text(probe: &str) -> String {
    format!("{PAUSE_TEXT}: {probe}\n")
}
/// The pause file holds exactly Crab's text and is a regular file owned by this user.
fn ours(path: &Path, probe: &str) -> bool {
    fs::symlink_metadata(path).is_ok_and(|m| m.is_file() && m.uid() == unsafe { libc::geteuid() })
        && read_bounded(path, 256).is_ok_and(|b| b == pause_text(probe).as_bytes())
}
/// Pure assessment: feed latches, derive values, choose at most one decision.
pub fn assess(p: &Probe, r: &Reading, m: &mut Memory, now: u64, workers: &dyn WorkerControl) -> (Value, BTreeMap<String, Option<bool>>, Decision) {
    let n = p.hysteresis;
    let input = r.input.as_ref().map(|x| x.clone().ok());
    // Progress: any change of the value; the first observation starts the window.
    if let Some(Ok(v)) = &r.progress {
        if m.last_progress != Some(*v) {
            if m.last_progress.is_some() {
                m.restarted_for_stall = false;
            }
            m.last_progress = Some(*v);
            m.last_progress_change = Some(now);
        }
        m.samples.push((now, *v));
        if m.samples.len() > SAMPLES_MAX {
            m.samples.remove(0);
        }
    }
    let starved = match (&p.input, &input) {
        (Some(g), Some(Some(v))) => Some(*v < g.low_water),
        (Some(_), _) => None,
        _ => Some(false),
    };
    let clog_active = m.latches.get("clogged").is_some_and(|l| l.active);
    let mut clog_unknown = false;
    let mut over = Vec::new();
    for (s, (_, v)) in p.stacks.iter().zip(&r.stacks).filter(|(s, _)| s.role == StackRole::Backlog) {
        match v {
            Ok(x) => {
                let limit = if clog_active { s.resume() } else { s.hw() };
                if *x > limit {
                    over.push(s.name.clone());
                }
            }
            Err(_) => clog_unknown = true,
        }
    }
    let clogged = if !over.is_empty() { Some(true) } else if clog_unknown { None } else { Some(false) };
    let stalled = match (&p.progress, &r.progress, &input) {
        (Some(_), Some(Err(_)), _) | (Some(_), _, Some(None)) => None,
        (Some(g), Some(Ok(_)), i) => {
            let has_input = i.as_ref().is_none_or(|v| v.is_some_and(|v| v >= 1));
            Some(has_input && m.last_progress_change.is_some_and(|t| now.saturating_sub(t) >= g.stall_seconds))
        }
        _ => Some(false),
    };
    let limited = Some(r.limit.is_some());
    let raw: BTreeMap<String, Option<bool>> = [("limited", limited), ("clogged", clogged), ("starved", starved), ("stalled", stalled)]
        .into_iter()
        .map(|(k, v)| (k.to_string(), v))
        .collect();
    let mut flips = BTreeMap::new();
    for (k, v) in &raw {
        if let Some(f) = m.latches.entry(k.clone()).or_default().feed(*v, n) {
            flips.insert(k.clone(), f);
        }
    }
    match (m.latches.get("starved").is_some_and(|l| l.active), m.starved_since) {
        (true, None) => m.starved_since = Some(now),
        (false, Some(_)) => m.starved_since = None,
        _ => {}
    }
    if !m.latches.get("starved").is_some_and(|l| l.active) {
        m.starved_acted = false;
    }
    // Quarantine stacks: the rate anomaly, reported on its own flips, never a pause.
    let mut quarantine = Vec::new();
    for (i, s) in p.stacks.iter().enumerate().filter(|(_, s)| s.role == StackRole::Quarantine) {
        let qm = m.quarantine.entry(s.name.clone()).or_default();
        let (raw_q, mut v) = quarantine_rate(s, &r.stacks[i].1, r.finished.get(i).and_then(Option::as_ref), qm, now);
        let flip = qm.latch.feed(raw_q, n);
        v["stack"] = json!(s.name);
        v["anomaly"] = json!(qm.latch.active);
        v["flip"] = json!(flip);
        v["reason"] = json!(r.reasons.get(i).cloned().flatten());
        quarantine.push(v);
    }
    m.quarantine.retain(|k, _| p.stacks.iter().any(|s| &s.name == k && s.role == StackRole::Quarantine));
    let active = |k: &str| m.latches.get(k).is_some_and(|l| l.active);
    let values = json!({
        "input": r.input.as_ref().map(|x| match x { Ok(v) => json!(v), Err(e) => json!({"unknown":e}) }),
        "low_water": p.input.as_ref().map(|g| g.low_water),
        "stacks": r.stacks.iter().zip(&p.stacks).map(|((n, v), s)| {
            let value = match v { Ok(v) => json!(v), Err(e) => json!({"unknown":e}) };
            match s.role {
                StackRole::Backlog => json!({"stack":n,"value":value,"high_water":s.hw(),"resume_at":s.resume()}),
                StackRole::Quarantine => json!({"stack":n,"role":"quarantine","value":value}),
            }
        }).collect::<Vec<_>>(),
        "over_high_water": over,
        "quarantine": quarantine,
        "progress": r.progress.as_ref().map(|x| match x { Ok(v) => json!(v), Err(e) => json!({"unknown":e}) }),
        "seconds_since_progress": m.last_progress_change.map(|t| now.saturating_sub(t)),
        "limit_marker": r.limit.as_ref().map(|(p, f)| json!({"pattern":p,"file":f})),
    });
    m.last_values = values.clone();
    m.last_evaluated = now;
    let producer = p.producer.as_ref();
    let pause = producer.and_then(|x| x.pause_file.clone());
    let decision = if active("limited") || now < m.hold_until {
        if flips.get("limited") == Some(&true) { Decision::Hold } else { Decision::None }
    } else if active("clogged") {
        let pausable = p.stacks.iter().any(|s| s.pauses() && over.contains(&s.name));
        match &pause {
            Some(_) if !pausable && flips.get("clogged") == Some(&true) => Decision::Report(format!("{} has a full stack ({}) that pausing cannot clear", p.name, over.join(", "))),
            Some(_) if !pausable => Decision::None,
            Some(f) if !f.exists() => Decision::PauseProducer(f.clone()),
            Some(_) if flips.get("clogged") == Some(&true) => Decision::Report(format!("{} is clogged and its producer's pause file already exists", p.name)),
            None if flips.get("clogged") == Some(&true) => Decision::Report(format!("{} is clogged and no producer pause file is configured", p.name)),
            _ => Decision::None,
        }
    } else if let (Some(f), true) = (&pause, m.paused_by_crab) {
        // The clog cleared: lift Crab's own pause.
        Decision::ResumeProducer(f.clone())
    } else if active("starved") {
        if m.starved_acted {
            Decision::None
        } else {
            m.starved_acted = true;
            match producer.and_then(|x| x.worker.as_ref()) {
                Some(w) => match workers.observe(w) {
                    Some((s, false)) if matches!(s.as_str(), "stopped" | "exited" | "failed") => Decision::StartProducer(w.clone()),
                    Some((s, true)) if s == "stopped" => Decision::Report(format!("{} has no work; its producer {w} was explicitly stopped", p.name)),
                    Some((s, _)) => Decision::Report(format!("{} has no work although its producer {w} is {s}", p.name)),
                    None => Decision::Report(format!("{} has no work; producer {w} is unknown", p.name)),
                },
                None => Decision::Report(format!("{} has no work and no supervised producer is configured", p.name)),
            }
        }
    } else if active("stalled") && stalled == Some(true) && !m.restarted_for_stall {
        match (&p.worker, p.restart_on_stall) {
            (Some(w), true) if workers.observe(w).is_some_and(|(s, _)| s == "running") => Decision::Restart(w.clone()),
            _ if flips.get("stalled") == Some(&true) => Decision::Report(format!("{} made no progress for {} s although it has input", p.name, values["seconds_since_progress"])),
            _ => Decision::None,
        }
    } else {
        Decision::None
    };
    (values, raw, decision)
}
/// Apply one decision; returns what was done. Pause files are created exclusively and only
/// Crab's own are removed.
pub fn apply(p: &Probe, d: &Decision, m: &mut Memory, now: u64, workers: &mut dyn WorkerControl) -> Value {
    match d {
        Decision::None => json!({"action":"none"}),
        Decision::Hold => json!({"action":"hold","detail":"limit marker present: no start or restart until it clears; a limit-stopped worker may be left uncertain and need a fresh run directory, which the Octopus decides"}),
        Decision::Report(s) => json!({"action":"report","detail":s}),
        Decision::StartProducer(w) => match workers.start(w, now) {
            Ok(v) => json!({"action":"start_producer","worker":w,"state":v["state"]}),
            Err(e) => json!({"action":"start_producer","worker":w,"error":e.to_string()}),
        },
        Decision::Restart(w) => {
            m.restarted_for_stall = true;
            match workers.restart(w, now) {
                Ok(v) => json!({"action":"restart","worker":w,"state":v["state"]}),
                Err(e) => json!({"action":"restart","worker":w,"error":e.to_string()}),
            }
        }
        Decision::PauseProducer(f) => {
            let made = OpenOptions::new()
                .create_new(true)
                .write(true)
                .mode(0o644)
                .open(f)
                .and_then(|mut x| x.write_all(pause_text(&p.name).as_bytes()).and_then(|_| x.sync_all()));
            match made {
                Ok(()) => {
                    m.paused_by_crab = true;
                    json!({"action":"pause_producer","pause_file":f})
                }
                Err(e) => json!({"action":"pause_producer","pause_file":f,"error":e.to_string()}),
            }
        }
        Decision::ResumeProducer(f) => {
            m.paused_by_crab = false;
            if !f.exists() {
                return json!({"action":"resume_producer","pause_file":f,"detail":"already absent"});
            }
            if !ours(f, &p.name) {
                return json!({"action":"report","detail":format!("pause file {} is no longer Crab's; left in place", f.display())});
            }
            match fs::remove_file(f) {
                Ok(()) => json!({"action":"resume_producer","pause_file":f}),
                Err(e) => json!({"action":"resume_producer","pause_file":f,"error":e.to_string()}),
            }
        }
    }
}
/// Apply a council-agreed action to a probe, within the same bounds as automatic flow.
pub fn council_decision(p: &Probe, action: &str, m: &Memory, workers: &dyn WorkerControl) -> std::result::Result<Decision, String> {
    let pause = p.producer.as_ref().and_then(|x| x.pause_file.clone());
    match action {
        "hold" => Ok(Decision::Hold),
        "pause_producer" => pause.filter(|f| !f.exists()).map(Decision::PauseProducer).ok_or_else(|| "no absent pause file to create".into()),
        "resume_producer" => pause.filter(|f| m.paused_by_crab && ours(f, &p.name)).map(Decision::ResumeProducer).ok_or_else(|| "no Crab-made pause file to remove".into()),
        "start_producer" => match p.producer.as_ref().and_then(|x| x.worker.clone()) {
            Some(w) if workers.observe(&w).is_some_and(|(s, stopped)| !stopped && matches!(s.as_str(), "stopped" | "exited" | "failed")) => Ok(Decision::StartProducer(w)),
            _ => Err("no stopped, not explicitly stopped producer worker".into()),
        },
        "restart" => match (&p.worker, p.restart_on_stall) {
            (Some(w), true) if workers.observe(w).is_some_and(|(s, _)| s == "running") => Ok(Decision::Restart(w.clone())),
            _ => Err("this probe's policy does not allow a restart".into()),
        },
        _ => Err("not an allowed action".into()),
    }
}

// ---------------------------------------------------------------- runtime
/// All probes with their durable memory under `run_root/flow`.
pub struct Flow {
    pub config: FlowConfig,
    dir: PathBuf,
    pub memory: BTreeMap<String, Memory>,
    pub last_tick: u64,
    pub pools: crate::pool::Pools,
    /// Load-time migrations, emitted with the first evaluation.
    pub pending: Vec<Value>,
}
/// Load-time migration: a clog latched (with Crab's own pause) on stacks that no longer pause
/// their producer — a quarantine declared as such, or a stack now reporting only — is cleared,
/// so the first evaluation lifts Crab's own pause file (`ResumeProducer` removes it only when it
/// still holds exactly Crab's text). A clog whose over-high-water stacks are unknown, or of
/// which one still pauses, is left to the ordinary hysteresis.
pub fn migrate(config: &FlowConfig, memory: &mut BTreeMap<String, Memory>, now: u64) -> Vec<Value> {
    let mut out = Vec::new();
    for p in &config.probes {
        let Some(m) = memory.get_mut(&p.name) else { continue };
        if !m.paused_by_crab || !m.latches.get("clogged").is_some_and(|l| l.active) {
            continue;
        }
        let over: Vec<String> = m.last_values["over_high_water"].as_array().into_iter().flatten().filter_map(|x| x.as_str().map(String::from)).collect();
        if over.is_empty() || over.iter().any(|n| p.stacks.iter().any(|s| &s.name == n && s.pauses())) {
            continue;
        }
        m.latches.insert("clogged".into(), Latch::default());
        out.push(json!({"at_unix":now,"probe":p.name,"group":p.group,"origin":"load","result":{"action":"migrate",
            "detail":format!("the clog on {} no longer pauses the producer; Crab lifts its own pause file at the first evaluation", over.join(", "))}}));
    }
    out
}
fn atomic(p: &Path, v: &Value) -> Result<()> {
    let tmp = p.with_extension("tmp");
    let mut f = OpenOptions::new().create(true).truncate(true).write(true).mode(0o600).open(&tmp)?;
    f.write_all(&serde_json::to_vec(v)?)?;
    f.sync_all()?;
    fs::rename(tmp, p)?;
    Ok(())
}
/// Append one JSON line, refusing to grow past the bound.
pub fn append(p: &Path, v: &Value) -> Result<()> {
    if fs::metadata(p).is_ok_and(|m| m.len() > JOURNAL_MAX) {
        return Err(error("journal bound reached; archive deliberately"));
    }
    let mut f = OpenOptions::new().create(true).append(true).mode(0o600).open(p)?;
    let mut b = serde_json::to_vec(v)?;
    b.push(b'\n');
    f.write_all(&b)?;
    f.sync_all()?;
    Ok(())
}
impl Flow {
    pub fn open(config: FlowConfig, run_root: &Path) -> Result<Flow> {
        let dir = run_root.join("flow");
        if !dir.exists() {
            std::fs::DirBuilder::new().recursive(true).mode(0o700).create(&dir)?;
        }
        let state = dir.join("state.json");
        let mut memory: BTreeMap<String, Memory> = if state.exists() {
            serde_json::from_slice(&read_bounded(&state, 4 << 20).map_err(error)?)?
        } else {
            BTreeMap::new()
        };
        memory.retain(|k, _| config.probes.iter().any(|p| &p.name == k));
        let pools = crate::pool::Pools::open(&config.pools, &dir)?;
        let now = std::time::SystemTime::now().duration_since(UNIX_EPOCH).map(|d| d.as_secs()).unwrap_or(0);
        let pending = migrate(&config, &mut memory, now);
        for e in &pending {
            append(&dir.join("journal.jsonl"), e)?;
        }
        Ok(Flow { config, dir, memory, last_tick: 0, pools, pending })
    }
    pub fn journal_path(&self) -> PathBuf {
        self.dir.join("journal.jsonl")
    }
    pub fn due(&self, now: u64) -> bool {
        now >= self.last_tick + self.config.interval_seconds
    }
    /// One evaluation of every probe; journals transitions and actions. Returns the events.
    pub fn tick(&mut self, now: u64, workers: &mut dyn WorkerControl) -> Result<Vec<Value>> {
        self.last_tick = now;
        let mut events = std::mem::take(&mut self.pending);
        for p in &self.config.probes {
            let r = observe(p, now);
            let m = self.memory.entry(p.name.clone()).or_default();
            let before: BTreeMap<String, bool> = m.latches.iter().map(|(k, l)| (k.clone(), l.active)).collect();
            let (values, raw, d) = assess(p, &r, m, now, workers);
            let done = apply(p, &d, m, now, workers);
            let after: BTreeMap<String, bool> = m.latches.iter().map(|(k, l)| (k.clone(), l.active)).collect();
            if before != after || d != Decision::None {
                let e = json!({"at_unix":now,"probe":p.name,"group":p.group,"conditions":after,"raw":raw,"values":values,"result":done});
                append(&self.journal_path(), &e)?;
                events.push(e);
            }
            // A quarantine-rate flip is its own event: a report when it rises, never a pause.
            for q in values["quarantine"].as_array().into_iter().flatten().filter(|q| !q["flip"].is_null()) {
                let why = q["reason"].as_str().map(|r| format!("; dominant reason: {r}")).unwrap_or_default();
                let result = if q["flip"] == true {
                    json!({"action":"report","anomaly":"quarantine_rate","stack":q["stack"],"detail":format!("{}: the quarantine rate of {} is high: {} of {} finished items quarantined in {} s (max rate {}){why}; a review backlog, never a pause",
                        p.name, q["stack"].as_str().unwrap_or(""), q["quarantined"], q["finished"], q["window_seconds"], q["max_rate"])})
                } else {
                    json!({"action":"none","anomaly":"quarantine_rate","stack":q["stack"],"detail":format!("{}: the quarantine rate of {} is back to normal", p.name, q["stack"].as_str().unwrap_or(""))})
                };
                let e = json!({"at_unix":now,"probe":p.name,"group":p.group,"quarantine":q,"result":result});
                append(&self.journal_path(), &e)?;
                events.push(e);
            }
        }
        atomic(&self.dir.join("state.json"), &serde_json::to_value(&self.memory)?)?;
        events.extend(self.pools.tick(now, &*workers)?);
        Ok(events)
    }
    /// Execute a council-agreed action on a probe; journaled by the caller.
    pub fn council_apply(&mut self, probe: &str, action: &str, now: u64, period: u64, workers: &mut dyn WorkerControl) -> std::result::Result<Value, String> {
        let p = self.config.probes.iter().find(|p| p.name == probe).ok_or("unknown probe")?.clone();
        let m = self.memory.entry(p.name.clone()).or_default();
        let d = council_decision(&p, action, m, workers)?;
        if d == Decision::Hold {
            m.hold_until = now + period;
        }
        let v = apply(&p, &d, m, now, workers);
        let _ = atomic(&self.dir.join("state.json"), &serde_json::to_value(&self.memory).unwrap_or_default());
        Ok(v)
    }
    /// The facts that should make the council meet again when they change: per probe the active
    /// conditions, the stacks over high water and the raised quarantine-rate anomalies. Counts,
    /// rates, ages and times are left out (they move every minute).
    pub fn stable_facts(&self) -> Value {
        let probes: Vec<Value> = self
            .config
            .probes
            .iter()
            .map(|p| {
                let m = self.memory.get(&p.name).cloned().unwrap_or_default();
                let on: Vec<&String> = m.latches.iter().filter(|(_, l)| l.active).map(|(k, _)| k).collect();
                let q: Vec<&String> = m.quarantine.iter().filter(|(_, q)| q.latch.active).map(|(k, _)| k).collect();
                json!({"probe":p.name,"conditions":on,"over_high_water":m.last_values["over_high_water"],"quarantine_rate_high":q,"paused_by_crab":m.paused_by_crab})
            })
            .collect();
        json!({"probes":probes})
    }
    /// The last `n` journal lines (bounded tail read).
    pub fn recent(&self, n: usize) -> Vec<Value> {
        let Some(b) = tail(&self.journal_path(), 64 * 1024) else { return vec![] };
        let text = String::from_utf8_lossy(&b);
        let lines: Vec<&str> = text.lines().filter(|l| l.starts_with('{')).collect();
        lines.iter().rev().take(n).rev().filter_map(|l| serde_json::from_str(l).ok()).collect()
    }
    /// Configured animal kinds (probe groups and pool kinds), in order.
    pub fn kinds(&self) -> Vec<String> {
        let s: std::collections::BTreeSet<String> = self.config.probes.iter().map(|p| p.group.clone()).chain(self.pools.kinds()).collect();
        s.into_iter().collect()
    }
    /// Backlog versus throughput per animal kind, from the last evaluations: queued work (the
    /// input gauges), progress per hour over the recent samples (a monotone counter is assumed;
    /// a decrease makes the rate unknown), estimated hours to drain, and starved seconds.
    /// Units are each probe's own (pages, targets, lines); kinds are not compared by this.
    pub fn capacity(&self, now: u64) -> Value {
        let mut kinds = serde_json::Map::new();
        for k in self.kinds() {
            let mut probes = Vec::new();
            let (mut backlog, mut rate_sum, mut rate_known, mut starved) = (0u64, 0f64, true, 0u64);
            for p in self.config.probes.iter().filter(|p| p.group == k) {
                let m = self.memory.get(&p.name).cloned().unwrap_or_default();
                let queued = m.last_values["input"].as_u64();
                let rate = match (m.samples.first(), m.samples.last()) {
                    (Some(a), Some(b)) if b.0 > a.0 && b.1 >= a.1 => Some((b.1 - a.1) as f64 * 3600.0 / (b.0 - a.0) as f64),
                    _ => None,
                };
                let drain = match (queued, rate) {
                    (Some(q), Some(r)) if r > 0.0 => Some((q as f64 / r * 10.0).round() / 10.0),
                    _ => None,
                };
                let idle = m.starved_since.map(|t| now.saturating_sub(t)).unwrap_or(0);
                backlog += queued.unwrap_or(0);
                match rate {
                    Some(r) => rate_sum += r,
                    None => rate_known = false,
                }
                starved += idle;
                probes.push(json!({"probe":p.name,"queued":queued,"progress_per_hour":rate.map(|r| (r * 10.0).round() / 10.0),
                    "hours_to_drain":drain,"starved_seconds":idle,"window_seconds":m.samples.first().map(|a| now.saturating_sub(a.0))}));
            }
            kinds.insert(k.clone(), json!({"probes":probes.len(),"queued_total":backlog,
                "progress_per_hour_total":if rate_known { json!((rate_sum * 10.0).round() / 10.0) } else { Value::Null },
                "starved_seconds_total":starved,"by_probe":probes}));
        }
        json!({"observed_unix":now,"kinds":kinds,"pools":self.pools.capacity(),"units":"each probe's own count; rates assume monotone progress counters"})
    }
    /// Fresh read-only readings with the latched conditions, for a group or all; plus English.
    pub fn status(&self, group: Option<&str>, now: u64) -> Value {
        let mut probes = Vec::new();
        let mut lines = Vec::new();
        let (mut with_work, mut total, mut clogged, mut unknown) = (0, 0, 0, 0);
        for p in self.config.probes.iter().filter(|p| group.is_none_or(|g| g == p.group)) {
            let r = observe(p, now);
            let m = self.memory.get(&p.name).cloned().unwrap_or_default();
            let active: BTreeMap<String, bool> = CONDITIONS.iter().map(|c| (c.to_string(), m.latches.get(*c).is_some_and(|l| l.active))).collect();
            let input = r.input.as_ref().map(|x| x.as_ref().ok().copied());
            if p.input.is_some() {
                total += 1;
            }
            let mut parts = Vec::new();
            match (&p.input, &input) {
                (Some(g), Some(Some(v))) => {
                    if *v >= g.low_water {
                        with_work += 1;
                        parts.push(format!("has work ({v}, low water {})", g.low_water));
                    } else {
                        parts.push(format!("has no work ({v}, low water {})", g.low_water));
                    }
                }
                (Some(_), _) => {
                    unknown += 1;
                    parts.push("input unknown".into());
                }
                _ => {}
            }
            for (s, (_, v)) in p.stacks.iter().zip(&r.stacks) {
                match (v, s.role) {
                    (Ok(x), StackRole::Backlog) => parts.push(format!("{} {x} of {}", s.name, s.hw())),
                    (Ok(x), StackRole::Quarantine) => {
                        let q = m.last_values["quarantine"].as_array().into_iter().flatten().find(|q| q["stack"] == s.name.as_str()).cloned().unwrap_or(Value::Null);
                        let rate = match q["rate"].as_f64() {
                            Some(r) => format!("rate {r} of {} finished in {} s, max {}", q["finished"], q["window_seconds"], s.max_rate()),
                            None => "rate not yet known".into(),
                        };
                        let high = if q["anomaly"] == true { "; the quarantine rate is high" } else { "" };
                        let why = q["reason"].as_str().map(|r| format!("; dominant reason: {r}")).unwrap_or_default();
                        parts.push(format!("{} {x} quarantined ({rate}{high}{why})", s.name));
                    }
                    (Err(_), _) => parts.push(format!("{} unknown", s.name)),
                }
            }
            match &r.progress {
                Some(Ok(v)) => parts.push(format!("progress {v}")),
                Some(Err(_)) => parts.push("progress unknown".into()),
                None => {}
            }
            if let Some(t) = m.last_progress_change {
                parts.push(format!("last change {} s ago", now.saturating_sub(t)));
            }
            if let Some((pat, _)) = &r.limit {
                parts.push(format!("limit marker \"{pat}\""));
            }
            let on: Vec<&String> = active.iter().filter(|(_, a)| **a).map(|(k, _)| k).collect();
            if active["clogged"] {
                clogged += 1;
            }
            if !on.is_empty() {
                parts.push(format!("conditions: {}", on.iter().map(|s| s.as_str()).collect::<Vec<_>>().join(", ")));
            }
            if parts.is_empty() {
                parts.push("no reading yet".into());
            }
            lines.push(format!("{}: {}.", p.name, parts.join("; ")));
            probes.push(json!({"probe":p.name,"group":p.group,"worker":p.worker,"conditions":active,
                "input":r.input.as_ref().map(|x| match x { Ok(v) => json!(v), Err(e) => json!({"unknown":e}) }),
                "low_water":p.input.as_ref().map(|g| g.low_water),
                "stacks":r.stacks.iter().zip(&p.stacks).map(|((n, v), s)| json!({"stack":n,"value":v.as_ref().ok(),"unknown":v.as_ref().err(),"high_water":s.high_water,"role":if s.role == StackRole::Quarantine { "quarantine" } else { "backlog" }})).collect::<Vec<_>>(),
                "quarantine":m.last_values["quarantine"],
                "seconds_since_progress":m.last_progress_change.map(|t| now.saturating_sub(t)),
                "limit_marker":r.limit.as_ref().map(|(p, _)| p),"paused_by_crab":m.paused_by_crab,"last_evaluated_unix":m.last_evaluated}));
        }
        let who = group.map(|g| format!("the {g} probes")).unwrap_or_else(|| "all probes".into());
        let (pool_lines, pool_views) = self.pools.status(group, now);
        let summary = if probes.is_empty() && !pool_lines.is_empty() {
            pool_lines[0].clone()
        } else if probes.is_empty() {
            format!("No flow probe is configured for {}.", group.unwrap_or("any worker"))
        } else if total == 0 {
            format!("Of {who}: none has an input queue; {} clogged.", if clogged == 0 { "none is".to_string() } else { format!("{clogged} are") })
        } else {
            format!(
                "Of {who}: {with_work} of {total} have work{}; {} clogged.",
                if unknown > 0 { format!(", {unknown} unknown") } else { String::new() },
                if clogged == 0 { "none is".to_string() } else { format!("{clogged} are") }
            )
        };
        if !(probes.is_empty() && !pool_lines.is_empty()) {
            lines.extend(pool_lines);
        } else {
            lines.extend(pool_lines.into_iter().skip(1));
        }
        json!({"observed_unix":now,"group":group,"summary":summary,"lines":lines,"probes":probes,"pools":pool_views,
            "health_inferred":false,"standing":"counts on disk under trusted configuration; not a quality judgement"})
    }
}

#[cfg(test)]
pub(crate) mod tests {
    use super::*;
    use std::sync::atomic::{AtomicU64, Ordering};
    static NEXT: AtomicU64 = AtomicU64::new(0);
    pub(crate) struct Fx(pub PathBuf);
    impl Drop for Fx {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }
    pub(crate) fn fx() -> Fx {
        let d = std::env::temp_dir().join(format!("crab-flow-{}-{}", std::process::id(), NEXT.fetch_add(1, Ordering::Relaxed)));
        fs::create_dir_all(d.join("ready")).unwrap();
        Fx(d)
    }
    #[derive(Default)]
    pub(crate) struct Fake {
        pub states: BTreeMap<String, (String, bool)>,
        pub calls: Vec<String>,
    }
    impl WorkerControl for Fake {
        fn observe(&self, n: &str) -> Option<(String, bool)> {
            self.states.get(n).cloned()
        }
        fn start(&mut self, n: &str, _: u64) -> Result<Value> {
            self.calls.push(format!("start {n}"));
            self.states.insert(n.into(), ("running".into(), false));
            Ok(json!({"state":"running"}))
        }
        fn restart(&mut self, n: &str, _: u64) -> Result<Value> {
            self.calls.push(format!("restart {n}"));
            Ok(json!({"state":"stopping_marker"}))
        }
    }
    pub(crate) fn probe(d: &Path) -> Probe {
        serde_json::from_value(json!({
            "name":"fish-ready","group":"fish","worker":"pager",
            "input":{"source":{"kind":"dir_entries","path":d.join("ready"),"suffix":".json"},"low_water":2},
            "stacks":[{"name":"in_flight","source":{"kind":"json_field","path":d.join("status.json"),"field":"counts.in_flight","missing":0},"high_water":4}],
            "progress":{"source":{"kind":"sum","of":[{"kind":"json_field","path":d.join("status.json"),"field":"cursor"}]},"stall_seconds":600},
            "producer":{"worker":"materializer","pause_file":d.join("PAUSE")},
            "limit":{"files":[d.join("jobs/*/raw.stdout")]},
            "restart_on_stall":true,"hysteresis":2
        }))
        .unwrap()
    }
    fn status(d: &Path, cursor: u64, q: u64) {
        fs::write(d.join("status.json"), json!({"cursor":cursor,"counts":{"in_flight":q}}).to_string()).unwrap();
    }
    fn ready(d: &Path, n: usize) {
        for i in 0..n {
            fs::write(d.join(format!("ready/p{i}.json")), "{}").unwrap();
        }
    }
    fn step(p: &Probe, m: &mut Memory, now: u64, w: &mut Fake) -> Decision {
        let r = observe(p, now);
        let (_, _, d) = assess(p, &r, m, now, w);
        apply(p, &d, m, now, w);
        d
    }
    #[test]
    fn readers_count_and_unknown_is_not_zero() {
        let f = fx();
        ready(&f.0, 3);
        fs::write(f.0.join("ready/.hidden"), "").unwrap();
        fs::write(f.0.join("ready/x.tmp"), "").unwrap();
        assert_eq!(read(&Source::DirEntries { path: f.0.join("ready"), prefix: None, suffix: Some(".json".into()) }), Ok(3));
        assert_eq!(read(&Source::DirEntries { path: f.0.join("ready"), prefix: Some("p1".into()), suffix: None }), Ok(1));
        assert!(read(&Source::DirEntries { path: f.0.join("absent"), prefix: None, suffix: None }).is_err());
        fs::write(f.0.join("q.jsonl"), "{}\n\n{}\n{}").unwrap();
        assert_eq!(read(&Source::JsonlLines { path: f.0.join("q.jsonl") }), Ok(3));
        assert_eq!(read(&Source::JsonlLines { path: f.0.join("none.jsonl") }), Ok(0));
        status(&f.0, 7, 2);
        let field = |field: &str, missing| Source::JsonField { path: f.0.join("status.json"), field: field.into(), missing };
        assert_eq!(read(&field("counts.in_flight", None)), Ok(2));
        assert_eq!(read(&field("counts.absent", Some(0))), Ok(0));
        assert!(read(&field("counts.absent", None)).is_err(), "absent without a declared default is unknown");
        assert!(read(&field("counts", Some(0))).is_err(), "an object is not a count");
        assert_eq!(read(&Source::Sum { of: vec![field("cursor", None), field("counts.in_flight", None)] }), Ok(9));
        // Configuration bounds.
        let mut c = FlowConfig { interval_seconds: 60, probes: vec![probe(&f.0)], pools: vec![] };
        let w = vec!["pager".to_string(), "materializer".to_string()];
        c.validate(&w).unwrap();
        assert!(c.validate(&["pager".into()]).is_err(), "unknown producer worker");
        c.probes[0].stacks[0].resume_at = Some(9);
        assert!(c.validate(&w).is_err());
        c.probes[0].stacks[0].resume_at = None;
        c.probes[0].input.as_mut().unwrap().source = Source::JsonlLines { path: "relative".into() };
        assert!(c.validate(&w).is_err());
        c.probes[0].input.as_mut().unwrap().source = Source::JsonlLines { path: f.0.join("../x") };
        assert!(c.validate(&w).is_err());
        let bad: std::result::Result<Probe, _> = serde_json::from_value(json!({"name":"x","group":"fish","stacks":[],"shell":"rm"}));
        assert!(bad.is_err());
    }
    #[test]
    fn starved_starts_a_stopped_producer_once_with_hysteresis() {
        let f = fx();
        let p = probe(&f.0);
        status(&f.0, 0, 0);
        let mut w = Fake::default();
        w.states.insert("materializer".into(), ("stopped".into(), false));
        w.states.insert("pager".into(), ("running".into(), false));
        let mut m = Memory::default();
        assert_eq!(step(&p, &mut m, 100, &mut w), Decision::None, "one starved reading is not enough");
        assert_eq!(step(&p, &mut m, 160, &mut w), Decision::StartProducer("materializer".into()));
        assert_eq!(step(&p, &mut m, 220, &mut w), Decision::None, "no repeated action while starved");
        assert_eq!(w.calls, vec!["start materializer"]);
        // Work arrives; a single dip does not flap.
        ready(&f.0, 3);
        step(&p, &mut m, 280, &mut w);
        step(&p, &mut m, 340, &mut w);
        assert!(!m.latches["starved"].active);
        fs::remove_file(f.0.join("ready/p0.json")).unwrap();
        fs::remove_file(f.0.join("ready/p1.json")).unwrap();
        step(&p, &mut m, 400, &mut w);
        assert!(!m.latches["starved"].active, "hysteresis holds after one low reading");
        // An explicitly stopped producer is reported, not started.
        w.states.insert("materializer".into(), ("stopped".into(), true));
        let d = step(&p, &mut m, 460, &mut w);
        assert!(matches!(d, Decision::Report(ref s) if s.contains("explicitly stopped")), "{d:?}");
        assert_eq!(w.calls.len(), 1);
    }
    #[test]
    fn clogged_pauses_through_the_marker_and_resumes_below_the_band() {
        let f = fx();
        let p = probe(&f.0);
        ready(&f.0, 3);
        status(&f.0, 1, 5);
        let mut w = Fake::default();
        w.states.insert("pager".into(), ("running".into(), false));
        let mut m = Memory::default();
        step(&p, &mut m, 100, &mut w);
        assert_eq!(step(&p, &mut m, 160, &mut w), Decision::PauseProducer(f.0.join("PAUSE")));
        assert!(ours(&f.0.join("PAUSE"), "fish-ready"));
        assert!(w.calls.is_empty(), "never kills or restarts in-flight work");
        // Inside the band (resume_at = 2): stays paused.
        status(&f.0, 2, 3);
        for t in [220, 280, 340] {
            assert_eq!(step(&p, &mut m, t, &mut w), Decision::None);
        }
        assert!(f.0.join("PAUSE").exists());
        status(&f.0, 3, 1);
        step(&p, &mut m, 400, &mut w);
        assert_eq!(step(&p, &mut m, 460, &mut w), Decision::ResumeProducer(f.0.join("PAUSE")));
        assert!(!f.0.join("PAUSE").exists());
        // A foreign pause file is never removed.
        status(&f.0, 4, 9);
        step(&p, &mut m, 520, &mut w);
        step(&p, &mut m, 580, &mut w);
        fs::write(f.0.join("PAUSE"), "operator\n").unwrap();
        status(&f.0, 5, 0);
        step(&p, &mut m, 640, &mut w);
        step(&p, &mut m, 700, &mut w);
        assert_eq!(fs::read_to_string(f.0.join("PAUSE")).unwrap(), "operator\n");
    }
    #[test]
    fn stall_restarts_only_by_policy_and_limit_holds() {
        let f = fx();
        let mut p = probe(&f.0);
        ready(&f.0, 3);
        status(&f.0, 1, 0);
        let mut w = Fake::default();
        w.states.insert("pager".into(), ("running".into(), false));
        let mut m = Memory::default();
        step(&p, &mut m, 1000, &mut w);
        step(&p, &mut m, 1600, &mut w);
        assert_eq!(step(&p, &mut m, 1700, &mut w), Decision::Restart("pager".into()));
        assert_eq!(step(&p, &mut m, 1800, &mut w), Decision::None, "one restart per stall episode");
        status(&f.0, 2, 0);
        step(&p, &mut m, 1900, &mut w);
        assert!(!m.restarted_for_stall);
        // Without the policy: a report, no restart.
        p.restart_on_stall = false;
        let mut m2 = Memory::default();
        step(&p, &mut m2, 1000, &mut w);
        step(&p, &mut m2, 1600, &mut w);
        assert!(matches!(step(&p, &mut m2, 1700, &mut w), Decision::Report(_)));
        assert_eq!(w.calls, vec!["restart pager"]);
        // No input: no stall.
        for e in fs::read_dir(f.0.join("ready")).unwrap() {
            fs::remove_file(e.unwrap().path()).unwrap();
        }
        let mut m3 = Memory::default();
        for t in [1000, 1700, 1800, 1900] {
            step(&probe(&f.0), &mut m3, t, &mut w);
        }
        assert!(!m3.latches["stalled"].active);
        // A fresh limit marker holds: no start, no restart.
        ready(&f.0, 3);
        fs::create_dir_all(f.0.join("jobs/a")).unwrap();
        fs::write(f.0.join("jobs/a/raw.stdout"), "{\"type\":\"error\",\"message\":\"You’ve hit your usage limit. Try again later.\"}").unwrap();
        let now = std::time::SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();
        let p = probe(&f.0);
        let mut m4 = Memory::default();
        step(&p, &mut m4, now, &mut w);
        assert_eq!(step(&p, &mut m4, now + 1, &mut w), Decision::Hold);
        for t in [now + 700, now + 800, now + 900] {
            assert_eq!(step(&p, &mut m4, t, &mut w), Decision::None, "held: no restart while limited");
        }
        assert_eq!(w.calls, vec!["restart pager"]);
        // An old marker does not count.
        assert!(limit_hit(p.limit.as_ref().unwrap(), now + 7200).is_none());
    }
    #[test]
    fn an_uncertain_state_is_reported_never_paused_or_restarted() {
        let f = fx();
        let mut p = probe(&f.0);
        p.stacks = vec![serde_json::from_value(json!({"name":"uncertain","source":{"kind":"json_equals","path":f.0.join("status.json"),"field":"state","value":"uncertain"},"high_water":0,"pause_producer":false})).unwrap()];
        FlowConfig { interval_seconds: 60, probes: vec![p.clone()], pools: vec![] }.validate(&["pager".into(), "materializer".into()]).unwrap();
        ready(&f.0, 3);
        fs::write(f.0.join("status.json"), json!({"state":"active","cursor":1}).to_string()).unwrap();
        let mut w = Fake::default();
        w.states.insert("pager".into(), ("running".into(), false));
        let mut m = Memory::default();
        assert_eq!(step(&p, &mut m, 10, &mut w), Decision::None);
        fs::write(f.0.join("status.json"), json!({"state":"uncertain","cursor":1}).to_string()).unwrap();
        step(&p, &mut m, 20, &mut w);
        assert!(matches!(step(&p, &mut m, 30, &mut w), Decision::Report(ref s) if s.contains("uncertain")));
        for t in [700, 1400, 2100] {
            step(&p, &mut m, t, &mut w);
        }
        assert!(!f.0.join("PAUSE").exists(), "pausing cannot clear an uncertain state");
        assert!(w.calls.is_empty(), "never restarted in place");
    }
    #[test]
    fn the_public_example_validates() {
        let v: Value = serde_json::from_str(include_str!("../flow/crab-flow.example.json")).unwrap();
        let c: FlowConfig = serde_json::from_value(v["flow"].clone()).unwrap();
        c.validate(&["fish-producer".into(), "eel".into(), "nautilus-a-work".into(), "nautilus-b-work".into()]).unwrap();
        assert!(c.validate(&["eel".into(), "nautilus-a-work".into(), "nautilus-b-work".into()]).is_err(), "the producer must be a configured worker");
        assert!(c.validate(&["fish-producer".into(), "eel".into()]).is_err(), "a pool member's worker must be configured");
        let council: crate::council::CouncilConfig = serde_json::from_value(v["council"].clone()).unwrap();
        council.validate(&["opus".into(), "sol".into()]).unwrap();
    }
    /// Negative controls: with the starved/clogged comparisons inverted or the hysteresis
    /// ignored, the expected decisions above cannot be produced; these checks pin the
    /// boundaries exactly (equal to low water is fed, equal to high water is not clogged).
    #[test]
    fn boundaries_are_exact_and_unknown_never_triggers() {
        let f = fx();
        let p = probe(&f.0);
        ready(&f.0, 2);
        status(&f.0, 1, 4);
        let mut w = Fake::default();
        w.states.insert("materializer".into(), ("stopped".into(), false));
        let mut m = Memory::default();
        for t in [10, 20, 30, 40] {
            assert_eq!(step(&p, &mut m, t, &mut w), Decision::None);
        }
        assert!(!m.latches["starved"].active && !m.latches["clogged"].active);
        fs::remove_file(f.0.join("status.json")).unwrap();
        fs::remove_dir_all(f.0.join("ready")).unwrap();
        for t in [50, 60, 70] {
            assert_eq!(step(&p, &mut m, t, &mut w), Decision::None, "unknown readings act on nothing");
        }
        let v = Flow { config: FlowConfig { interval_seconds: 60, probes: vec![p], pools: vec![] }, dir: f.0.clone(), memory: BTreeMap::new(), last_tick: 0, pools: crate::pool::Pools::open(&[], &f.0).unwrap(), pending: vec![] }.status(Some("fish"), 80);
        assert!(v["summary"].as_str().unwrap().contains("1 unknown"), "{v}");
    }
    fn quarantine_stack(d: &Path) -> Stack {
        serde_json::from_value(json!({"name":"quarantined_jobs","role":"quarantine",
            "source":{"kind":"json_field","path":d.join("status.json"),"field":"q","missing":0},
            "finished":{"kind":"json_field","path":d.join("status.json"),"field":"f","missing":0},
            "rate_window_seconds":600,"max_rate":0.5,"min_finished":4,
            "reason_source":{"path":d.join("status.json"),"field":"why"}}))
        .unwrap()
    }
    fn qstatus(d: &Path, q: u64, f: u64) {
        fs::write(d.join("status.json"), json!({"cursor":f,"q":q,"f":f,"counts":{"in_flight":0},"why":"schema: missing frame"}).to_string()).unwrap();
    }
    #[test]
    fn a_quarantine_stack_is_validated_strictly() {
        let f = fx();
        let w = vec!["pager".to_string(), "materializer".to_string()];
        let mut p = probe(&f.0);
        p.stacks.push(quarantine_stack(&f.0));
        let c = |p: &Probe| FlowConfig { interval_seconds: 60, probes: vec![p.clone()], pools: vec![] }.validate(&w);
        c(&p).unwrap();
        let mut bad = p.clone();
        bad.stacks[1].high_water = Some(8);
        assert!(c(&bad).is_err(), "a quarantine has no high water");
        let mut bad = p.clone();
        bad.stacks[1].pause_producer = Some(true);
        assert!(c(&bad).is_err(), "a quarantine never pauses");
        let mut bad = p.clone();
        bad.stacks[1].finished = None;
        assert!(c(&bad).is_err(), "a quarantine needs its finished count");
        let mut bad = p.clone();
        bad.stacks[1].max_rate = Some(1.5);
        assert!(c(&bad).is_err());
        let mut bad = p.clone();
        bad.stacks[0].finished = bad.stacks[1].finished.clone();
        assert!(c(&bad).is_err(), "rate fields belong to a quarantine");
        // The old form: a quarantine with a high-water pause is refused at load...
        let mut old = p.clone();
        old.stacks[1] = serde_json::from_value(json!({"name":"quarantined_jobs","source":{"kind":"json_field","path":f.0.join("status.json"),"field":"q"},"high_water":8,"resume_at":4})).unwrap();
        assert!(c(&old).unwrap_err().to_string().contains("never drains by itself"));
        // ...unless it is explicitly reporting only.
        old.stacks[1].pause_producer = Some(false);
        c(&old).unwrap();
        let bad: std::result::Result<Stack, _> = serde_json::from_value(json!({"name":"x","role":"quarantine","source":{"kind":"mtime","path":"/x"},"finished":{"kind":"mtime","path":"/x"},"shell":"rm"}));
        assert!(bad.is_err(), "unknown fields are refused");
    }
    #[test]
    fn a_quarantine_rate_is_reported_and_never_pauses() {
        let f = fx();
        let mut p = probe(&f.0);
        p.stacks.push(quarantine_stack(&f.0));
        ready(&f.0, 3);
        let mut flow = Flow::open(FlowConfig { interval_seconds: 60, probes: vec![p], pools: vec![] }, &f.0).unwrap();
        let mut w = Fake::default();
        w.states.insert("pager".into(), ("running".into(), false));
        let mut events = Vec::new();
        // 3 of every 4 finished items are quarantined: far above 0.5, and the count is already
        // over any old high water of 8.
        for (i, t) in (1000..1300).step_by(60).enumerate() {
            let i = i as u64;
            qstatus(&f.0, 3 * i, 4 * i);
            events.extend(flow.tick(t, &mut w).unwrap());
        }
        let reports: Vec<&Value> = events.iter().filter(|e| e["result"]["anomaly"] == "quarantine_rate").collect();
        assert_eq!(reports.len(), 1, "one report per rise: {events:?}");
        let d = reports[0]["result"]["detail"].as_str().unwrap();
        assert_eq!(reports[0]["result"]["action"], "report");
        assert!(d.contains("quarantine rate of quarantined_jobs is high") && d.contains("dominant reason: schema: missing frame"), "{d}");
        assert!(!f.0.join("PAUSE").exists(), "a quarantine never pauses its producer");
        assert!(!flow.memory["fish-ready"].latches["clogged"].active, "a quarantine count is not a clog");
        assert!(!events.iter().any(|e| e["result"]["action"] == "report" && e["result"]["anomaly"].is_null()), "no clog report: {events:?}");
        assert!(w.calls.is_empty());
        let st = flow.status(None, 1300);
        assert!(st["lines"][0].as_str().unwrap().contains("the quarantine rate is high"), "{st}");
        assert!(flow.stable_facts()["probes"][0]["quarantine_rate_high"][0] == "quarantined_jobs");
        // Healthy again: only finished items for longer than the window.
        let mut back = Vec::new();
        for (i, t) in (1300..2200).step_by(60).enumerate() {
            qstatus(&f.0, 12, 16 + 10 * (i as u64 + 1));
            back.extend(flow.tick(t, &mut w).unwrap());
        }
        assert!(back.iter().any(|e| e["result"]["anomaly"] == "quarantine_rate" && e["result"]["action"] == "none"), "{back:?}");
        // A counter reset starts a new window instead of a negative rate.
        qstatus(&f.0, 0, 0);
        flow.tick(2300, &mut w).unwrap();
        assert_eq!(flow.memory["fish-ready"].quarantine["quarantined_jobs"].samples.len(), 1);
        // Too few finished items make no rate: unknown, the latch is unchanged.
        let mut m = QuarantineMemory::default();
        let s = quarantine_stack(&f.0);
        assert_eq!(quarantine_rate(&s, &Ok(0), Some(&Ok(0)), &mut m, 10).0, None);
        assert_eq!(quarantine_rate(&s, &Ok(3), Some(&Ok(3)), &mut m, 70).0, None, "3 < min_finished 4");
        assert_eq!(quarantine_rate(&s, &Ok(4), Some(&Ok(8)), &mut m, 130).0, Some(false), "4 of 8 is not above 0.5");
        assert_eq!(quarantine_rate(&s, &Ok(5), Some(&Ok(9)), &mut m, 190).0, Some(true));
        assert_eq!(quarantine_rate(&s, &Err("x".into()), Some(&Ok(9)), &mut m, 250).0, None);
    }
    /// The live latch of a Fish pager paused on its quarantine (state.json of 2026-11-04), with the
    /// configuration converted to the quarantine form: the first evaluation lifts Crab's own
    /// pause; a foreign pause file and a still-pausing backlog clog are left alone.
    #[test]
    fn a_clog_on_a_stack_that_no_longer_pauses_lifts_crabs_own_pause_at_load() {
        let f = fx();
        ready(&f.0, 3);
        qstatus(&f.0, 11, 11);
        let live = |over: &str| -> Memory {
            serde_json::from_value(json!({"latches":{"clogged":{"active":true,"off":0,"on":101},"limited":{"active":false,"off":324,"on":0},
                "stalled":{"active":true,"off":0,"on":57},"starved":{"active":false,"off":324,"on":0}},
                "last_progress":90,"last_progress_change":1793806800u64,"paused_by_crab":true,"restarted_for_stall":false,"hold_until":0,
                "last_values":{"input":166,"over_high_water":[over],"stacks":[{"high_water":8,"resume_at":4,"stack":over,"value":11}]},
                "last_evaluated":1793811600u64,"samples":[],"starved_since":null,"starved_acted":false}))
            .unwrap()
        };
        let mk = |name: &str, quarantine: bool| {
            let mut p = probe(&f.0);
            p.name = name.into();
            p.restart_on_stall = false;
            p.producer = Some(Producer { worker: None, pause_file: Some(f.0.join(format!("{name}.PAUSE"))) });
            if quarantine {
                p.stacks = vec![quarantine_stack(&f.0)];
            }
            p
        };
        let probes = vec![mk("fish-t05", true), mk("fish-t07", true), mk("fish-ready", false)];
        fs::write(f.0.join("fish-t05.PAUSE"), pause_text("fish-t05")).unwrap();
        fs::write(f.0.join("fish-t07.PAUSE"), "operator\n").unwrap();
        fs::write(f.0.join("fish-ready.PAUSE"), pause_text("fish-ready")).unwrap();
        fs::write(f.0.join("status.json"), json!({"cursor":90,"q":11,"f":11,"counts":{"in_flight":9}}).to_string()).unwrap();
        let mem: BTreeMap<String, Memory> = [("fish-t05".to_string(), live("quarantined_jobs")), ("fish-t07".to_string(), live("quarantined_jobs")), ("fish-ready".to_string(), live("in_flight"))].into();
        fs::create_dir_all(f.0.join("flow")).unwrap();
        fs::write(f.0.join("flow/state.json"), serde_json::to_vec(&mem).unwrap()).unwrap();
        let mut flow = Flow::open(FlowConfig { interval_seconds: 60, probes, pools: vec![] }, &f.0).unwrap();
        assert_eq!(flow.pending.len(), 2, "{:?}", flow.pending);
        assert!(fs::read_to_string(f.0.join("flow/journal.jsonl")).unwrap().contains("\"action\":\"migrate\""));
        let mut w = Fake::default();
        let events = flow.tick(1793811700, &mut w).unwrap();
        assert!(!f.0.join("fish-t05.PAUSE").exists(), "Crab's own pause is lifted: {events:?}");
        assert!(!flow.memory["fish-t05"].paused_by_crab);
        assert_eq!(fs::read_to_string(f.0.join("fish-t07.PAUSE")).unwrap(), "operator\n", "a foreign pause file stays");
        assert!(events.iter().any(|e| e["probe"] == "fish-t07" && e["result"]["detail"].as_str().is_some_and(|d| d.contains("no longer Crab's"))));
        assert!(f.0.join("fish-ready.PAUSE").exists(), "a backlog still over high water keeps its pause");
        assert!(flow.memory["fish-ready"].latches["clogged"].active);
        assert!(w.calls.is_empty());
    }
}
