//! Bounded native agent capabilities. No shell, teacher transport or deployment policy.
//! Location conclusions are source-scoped episode knowledge, not a learned universal executor.
use anyhow::{Context, Result, ensure};
use serde::{Deserialize, Serialize};
use sha2::{Digest, Sha256};
use std::{
    collections::BTreeSet,
    fs::{self, OpenOptions},
    io::{Read, Write},
    path::{Component, Path},
};
use world::{
    babi::Story,
    knowledge::{Lesson, retention::Retained},
};

pub const MAX_FILE_BYTES: u64 = 262_144;
pub const MAX_REPORTS: usize = 32;
pub fn digest(bytes: &[u8]) -> String {
    format!("{:x}", Sha256::digest(bytes))
}
fn identifier(s: &str) -> bool {
    !s.is_empty()
        && s.len() <= 96
        && s.bytes()
            .all(|b| b.is_ascii_alphanumeric() || b"._:-".contains(&b))
}
fn word(s: &str) -> bool {
    !s.is_empty() && s.len() <= 32 && s.bytes().all(|b| b.is_ascii_alphabetic())
}
fn sha(s: &str) -> bool {
    s.len() == 64
        && s.bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
}
fn read_limit(path: &Path, limit: u64) -> Result<Vec<u8>> {
    use std::os::unix::fs::OpenOptionsExt;
    let f = OpenOptions::new()
        .read(true)
        .custom_flags(libc::O_NONBLOCK | libc::O_NOFOLLOW)
        .open(path)?;
    let meta = f.metadata()?;
    ensure!(
        meta.file_type().is_file() && meta.len() <= limit,
        "bounded regular file required"
    );
    let mut bytes = Vec::new();
    f.take(limit + 1).read_to_end(&mut bytes)?;
    ensure!(bytes.len() as u64 <= limit, "file grew beyond bound");
    Ok(bytes)
}
pub fn read_bounded(path: &Path) -> Result<Vec<u8>> {
    read_limit(path, MAX_FILE_BYTES)
}
fn write_new(path: &Path, bytes: &[u8]) -> Result<()> {
    let mut f = OpenOptions::new().create_new(true).write(true).open(path)?;
    f.write_all(bytes)?;
    f.sync_all()?;
    Ok(())
}
pub fn json_new(path: &Path, value: &impl Serialize) -> Result<()> {
    write_new(path, &serde_json::to_vec_pretty(value)?)
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Sentence {
    pub id: String,
    pub text: String,
    pub speaker: Option<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct SourceWindow {
    pub document: String,
    /// Declared parent document identity; only window bytes are verified here.
    pub document_sha256: Option<String>,
    pub window_sha256: String,
    pub sentences: Vec<Sentence>,
}
impl SourceWindow {
    pub fn validate(&self) -> Result<()> {
        ensure!(identifier(&self.document), "invalid document id");
        ensure!(
            self.document_sha256.as_ref().is_none_or(|s| sha(s)),
            "invalid declared document hash"
        );
        ensure!(
            (1..=16).contains(&self.sentences.len()),
            "window needs 1..16 sentences"
        );
        let mut ids = BTreeSet::new();
        let mut bytes = 0;
        for s in &self.sentences {
            ensure!(
                identifier(&s.id) && ids.insert(&s.id),
                "invalid/duplicate sentence id"
            );
            ensure!(
                !s.text.trim().is_empty()
                    && s.text.len() <= 4096
                    && !s.text.contains(['\n', '\r', '\0']),
                "invalid sentence text"
            );
            ensure!(
                s.speaker.as_ref().is_none_or(|s| identifier(s)),
                "invalid speaker id"
            );
            bytes += s.text.len();
        }
        ensure!(bytes <= 32768, "window text budget exceeded");
        ensure!(
            self.window_sha256 == digest(&serde_json::to_vec(&self.sentences)?),
            "window hash mismatch"
        );
        Ok(())
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct TraceTask {
    pub schema: u32,
    pub id: String,
    pub actor: String,
    pub subject: String,
    pub locations: Vec<String>,
    pub source: SourceWindow,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct RecallTask {
    pub state_sha256: String,
    pub state_fingerprint: String,
    pub trace_report_sha256: String,
    pub trace_id: String,
    pub source_window_sha256: String,
    pub schema: u32,
    pub id: String,
    pub actor: String,
    pub subject: String,
    pub locations: Vec<String>,
}
fn task_fields(
    schema: u32,
    id: &str,
    actor: &str,
    subject: &str,
    locations: &[String],
) -> Result<()> {
    ensure!(
        schema == 1 && identifier(id) && identifier(actor),
        "schema1 and bounded identifiers required"
    );
    ensure!(
        word(subject) && locations.len() == 4 && locations.iter().all(|s| word(s)),
        "one-word subject and four location alternatives required"
    );
    ensure!(
        locations
            .iter()
            .map(|s| s.to_lowercase())
            .collect::<BTreeSet<_>>()
            .len()
            == 4,
        "duplicate locations"
    );
    Ok(())
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum Outcome {
    Supported,
    Uncertain,
    Rejected,
    Failed,
}
impl Outcome {
    fn key(&self) -> &'static str {
        match self {
            Self::Supported => "supported",
            Self::Uncertain => "uncertain",
            Self::Rejected => "rejected",
            Self::Failed => "failed",
        }
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct Finding {
    pub code: String,
    pub detail: String,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct FishReport {
    pub schema: u32,
    pub id: String,
    pub actor: String,
    pub capability: String,
    pub outcome: Outcome,
    pub input_sha256: String,
    pub source: Option<SourceWindow>,
    pub question: String,
    pub answer: Option<String>,
    pub native_witness: String,
    pub native_analysis: String,
    pub findings: Vec<Finding>,
    pub before_state_sha256: String,
    pub after_state_sha256: String,
    pub new_frames: usize,
    pub changed_weights: usize,
    pub checkpoint: Option<String>,
    pub scope: String,
}
fn analyze(st: &Retained, question: &str, locations: &[String]) -> (Option<String>, String, bool) {
    let a = st.student.answer(question, locations);
    let score = a
        .options
        .get(a.choice)
        .and_then(|o| o.best.as_ref())
        .map(|s| s.0);
    let supported = a.understood
        && score.is_some_and(|s| {
            s > 0
                && a.options
                    .iter()
                    .enumerate()
                    .all(|(i, o)| i == a.choice || o.best.as_ref().is_none_or(|b| b.0 < s))
        });
    let answer = supported.then(|| locations[a.choice].clone());
    (answer, st.student.why(&a, a.choice), supported)
}
fn outside_context(s: &Sentence) -> bool {
    s.speaker.is_some()
        || s.text.contains(['"', '\'', '“', '”', '«', '»', '‘', '’'])
        || s.text.split_whitespace().any(|w| {
            matches!(
                w.trim_matches(|c: char| !c.is_alphabetic())
                    .to_ascii_lowercase()
                    .as_str(),
                "i" | "my" | "me" | "we" | "our" | "us"
            )
        })
}
/// Native world inference, native graph-backed answer analysis, then a new retained checkpoint.
/// No teacher label enters this operation; facts are scoped to the supplied episode.
pub fn run_trace(task: &TraceTask, new_dir: &Path) -> Result<FishReport> {
    task_fields(
        task.schema,
        &task.id,
        &task.actor,
        &task.subject,
        &task.locations,
    )?;
    task.source.validate()?;
    fs::create_dir(new_dir)
        .context("new exclusive run directory required; inspect partial failures")?;
    let mut story = Story::default();
    let mut findings = Vec::new();
    for s in &task.source.sentences {
        // This capability admits third-person location episodes, not quoted dialogue interpretation.
        if outside_context(s) {
            findings.push(Finding {
                code: "context_outside_capability".into(),
                detail: format!(
                    "{}: quoted/first-person context requires separate speaker review",
                    s.id
                ),
            });
        }
        match story.read(&s.text) {
            Ok(None) => {}
            Ok(Some(why)) => findings.push(Finding {
                code: "native_read_gap".into(),
                detail: format!("{}: {why}", s.id),
            }),
            Err(e) => findings.push(Finding {
                code: "native_read_error".into(),
                detail: format!("{}: {e}", s.id),
            }),
        }
    }
    let question = format!("Where is the {}?", task.subject);
    let (value, native_witness) = match story.ask(&question) {
        Ok(x) => x,
        Err(e) => ("?".into(), e.to_string()),
    };
    let mut st = Retained::new(&format!("agent-location-{}", task.id));
    let before = st.fingerprint()?;
    let mut report = FishReport { schema: 1, id: task.id.clone(), actor: task.actor.clone(), capability: "world_location_trace".into(), outcome: Outcome::Uncertain,
        input_sha256: digest(&serde_json::to_vec(task)?), source: Some(task.source.clone()), question: question.clone(), answer: None,
        native_witness, native_analysis: String::new(), findings, before_state_sha256: before.clone(), after_state_sha256: before,
        new_frames: 0, changed_weights: 0, checkpoint: None, scope: "source-scoped retained episode conclusion; not a learned general executor, factual-world certification or automatic semantic correction".into() };
    if value == "?" || !task.locations.contains(&value) {
        report.findings.push(Finding {
            code: "native_answer_gap".into(),
            detail: "No admitted location conclusion; no experience promoted".into(),
        });
    }
    if report.findings.is_empty() {
        let lesson = Lesson {
            course: format!("agent-location-{}", task.id),
            n: 1,
            title: "Native episode conclusion".into(),
            facts: vec![format!("The {} is in the {}.", task.subject, value)],
            ..Default::default()
        };
        let change = st.train(&lesson, false)?;
        let (answer, explanation, supported) = analyze(&st, &question, &task.locations);
        report.native_analysis = explanation;
        let checked = st.student.answer(&question, &task.locations);
        let own_support = checked.options[checked.choice]
            .best
            .as_ref()
            .is_some_and(|b| st.student.frames[b.2].src.0 == 1);
        if supported && own_support && answer.as_deref() == Some(&value) {
            st.save(&new_dir.join("state.json"))?;
            report.outcome = Outcome::Supported;
            report.answer = answer;
            report.after_state_sha256 = st.fingerprint()?;
            report.new_frames = change["new_frames"].as_u64().unwrap_or(0) as usize;
            report.changed_weights = change["changed_weights"].as_u64().unwrap_or(0) as usize;
            report.checkpoint = Some("state.json".into());
        } else {
            report.findings.push(Finding { code: "native_analysis_disagreement".into(), detail: "Graph-backed analysis did not support the world conclusion; candidate not saved".into() });
        }
    }
    json_new(&new_dir.join("report.json"), &report)?;
    Ok(report)
}
/// Construct explicit lineage pins, without making a semantic admission.
pub fn prepare_synthetic_recall(
    trace_report: &Path,
    state: &Path,
    id: &str,
    actor: &str,
) -> Result<RecallTask> {
    let trace_bytes = read_bounded(trace_report)?;
    let parent: FishReport = serde_json::from_slice(&trace_bytes)?;
    ensure!(
        parent.schema == 1
            && parent.capability == "world_location_trace"
            && parent.outcome == Outcome::Supported,
        "supported native parent required"
    );
    let subject = parent
        .question
        .strip_prefix("Where is the ")
        .and_then(|q| q.strip_suffix('?'))
        .context("parent question shape")?
        .to_string();
    // Alternatives are explicit caller choices; the preparation helper uses the documented synthetic palette.
    let task = RecallTask {
        state_sha256: digest(&read_limit(state, 2_097_152)?),
        state_fingerprint: parent.after_state_sha256,
        trace_report_sha256: digest(&trace_bytes),
        trace_id: parent.id,
        source_window_sha256: parent
            .source
            .context("parent source required")?
            .window_sha256,
        schema: 1,
        id: id.into(),
        actor: actor.into(),
        subject,
        locations: vec![
            "garden".into(),
            "kitchen".into(),
            "office".into(),
            "hallway".into(),
        ],
    };
    task_fields(
        task.schema,
        &task.id,
        &task.actor,
        &task.subject,
        &task.locations,
    )?;
    Ok(task)
}
/// Fresh-process recall reads graph state only, with no source passage or train call.
pub fn recall(task: &RecallTask, state: &Path, trace_report: &Path) -> Result<FishReport> {
    task_fields(
        task.schema,
        &task.id,
        &task.actor,
        &task.subject,
        &task.locations,
    )?;
    ensure!(
        identifier(&task.trace_id)
            && [
                &task.state_sha256,
                &task.state_fingerprint,
                &task.trace_report_sha256,
                &task.source_window_sha256
            ]
            .iter()
            .all(|s| sha(s)),
        "bounded valid lineage pins required"
    );
    let bytes = read_limit(state, 2_097_152)?;
    ensure!(
        sha(&task.state_sha256) && digest(&bytes) == task.state_sha256,
        "state byte binding mismatch"
    );
    let trace_bytes = read_bounded(trace_report)?;
    ensure!(
        digest(&trace_bytes) == task.trace_report_sha256,
        "trace report binding mismatch"
    );
    let parent: FishReport = serde_json::from_slice(&trace_bytes)?;
    ensure!(
        parent.schema == 1
            && parent.id == task.trace_id
            && parent.capability == "world_location_trace"
            && parent.outcome == Outcome::Supported,
        "supported native trace parent required"
    );
    ensure!(
        parent
            .source
            .as_ref()
            .is_some_and(|s| s.window_sha256 == task.source_window_sha256),
        "source lineage mismatch"
    );
    ensure!(
        parent.question == format!("Where is the {}?", task.subject),
        "trace question mismatch"
    );
    // Native loader only accepts paths. A sealed anonymous snapshot pins verified bounded bytes.
    use std::os::fd::{AsRawFd, FromRawFd};
    let fd = unsafe {
        libc::memfd_create(
            c"agent-mova-state".as_ptr(),
            libc::MFD_CLOEXEC | libc::MFD_ALLOW_SEALING,
        )
    };
    ensure!(fd >= 0, "cannot create checkpoint snapshot");
    let mut snapshot = unsafe { fs::File::from_raw_fd(fd) };
    snapshot.write_all(&bytes)?;
    let seals = libc::F_SEAL_WRITE | libc::F_SEAL_GROW | libc::F_SEAL_SHRINK | libc::F_SEAL_SEAL;
    ensure!(
        unsafe { libc::fcntl(snapshot.as_raw_fd(), libc::F_ADD_SEALS, seals) } == 0,
        "cannot seal checkpoint snapshot"
    );
    let st = Retained::load(Path::new(&format!(
        "/proc/self/fd/{}",
        snapshot.as_raw_fd()
    )))?;
    let before = st.fingerprint()?;
    ensure!(
        before == task.state_fingerprint && before == parent.after_state_sha256,
        "state fingerprint lineage mismatch"
    );
    let question = format!("Where is the {}?", task.subject);
    let (answer, explanation, supported) = analyze(&st, &question, &task.locations);
    let after = st.fingerprint()?;
    ensure!(before == after, "recall changed retained state");
    Ok(FishReport {
        schema: 1,
        id: task.id.clone(),
        actor: task.actor.clone(),
        capability: "retained_location_recall".into(),
        outcome: if supported {
            Outcome::Supported
        } else {
            Outcome::Uncertain
        },
        input_sha256: digest(&serde_json::to_vec(task)?),
        source: None,
        question,
        answer,
        native_witness: "cold graph restore, no source replay or training".into(),
        native_analysis: explanation,
        findings: if supported {
            vec![]
        } else {
            vec![Finding {
                code: "native_recall_gap".into(),
                detail: "No supported answer in retained graph".into(),
            }]
        },
        before_state_sha256: before,
        after_state_sha256: after,
        new_frames: 0,
        changed_weights: 0,
        checkpoint: None,
        scope: "task-specific retained semantic knowledge, not learned tool-planning competence"
            .into(),
    })
}

#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ArtifactRef {
    pub path: String,
    pub sha256: String,
    pub format: ArtifactFormat,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum ArtifactFormat {
    NativeFishReport,
    ExternalReviewEvidence,
}
/// A caller's report claims remain claims: collection does not grant semantic admission.
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReportEnvelope {
    pub id: String,
    pub actor: String,
    pub kind: String,
    pub outcome: Outcome,
    pub source: Option<SourceWindow>,
    pub findings: Vec<Finding>,
    pub artifacts: Vec<ArtifactRef>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
#[serde(deny_unknown_fields)]
pub struct ReportIndex {
    pub schema: u32,
    pub id: String,
    pub reports: Vec<ReportEnvelope>,
}
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum EvidenceBinding {
    NativeBound,
    DeclaredOnly,
    #[default]
    LegacyUnclassified,
}
impl EvidenceBinding {
    fn key(self) -> &'static str {
        match self {
            Self::NativeBound => "native_bound",
            Self::DeclaredOnly => "declared_only",
            Self::LegacyUnclassified => "legacy_unclassified",
        }
    }
}
fn evidence_binding(report: &ReportEnvelope) -> EvidenceBinding {
    if report
        .artifacts
        .iter()
        .any(|a| matches!(a.format, ArtifactFormat::NativeFishReport))
    {
        EvidenceBinding::NativeBound
    } else {
        EvidenceBinding::DeclaredOnly
    }
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct NativeGroup {
    pub kind: String,
    pub declared_outcome: String,
    #[serde(default)]
    pub binding: EvidenceBinding,
    pub count: usize,
    pub report_ids: Vec<String>,
}
#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CrabReport {
    pub schema: u32,
    pub id: String,
    pub input_sha256: String,
    pub groups: Vec<NativeGroup>,
    pub review_queue: Vec<ReportEnvelope>,
    pub checked_artifact_bytes: usize,
    pub semantic_admission: bool,
    pub scope: String,
}
fn confined(root: &Path, relative: &str) -> Result<std::path::PathBuf> {
    let p = Path::new(relative);
    ensure!(
        !p.as_os_str().is_empty() && !p.is_absolute(),
        "relative artifact path required"
    );
    let mut at = root.to_path_buf();
    for c in p.components() {
        let Component::Normal(c) = c else {
            anyhow::bail!("non-normal artifact path")
        };
        at.push(c);
        ensure!(
            !fs::symlink_metadata(&at)?.file_type().is_symlink(),
            "symlink artifact path refused"
        );
    }
    ensure!(
        at.canonicalize()?.starts_with(root),
        "artifact escapes trusted root"
    );
    Ok(at)
}
/// Bounded actual Fish-report route through Crab: native Mova table grouping and a lossless review queue.
/// Input labels are declared outcomes, never newly certified facts; all reports remain reviewable.
pub fn collect(root: &Path, index: &ReportIndex, new_dir: &Path) -> Result<CrabReport> {
    ensure!(
        index.schema == 1 && identifier(&index.id),
        "schema1 and bounded collection id required"
    );
    ensure!(
        (1..=MAX_REPORTS).contains(&index.reports.len()),
        "collection needs 1..32 reports"
    );
    let root = root.canonicalize()?;
    ensure!(root.is_dir(), "trusted artifact root must be a directory");
    let mut ids = BTreeSet::new();
    let mut total = 0;
    for r in &index.reports {
        ensure!(
            identifier(&r.id) && identifier(&r.actor) && identifier(&r.kind) && ids.insert(&r.id),
            "invalid or duplicate report identity"
        );
        ensure!(
            r.findings.len() <= 16 && (1..=3).contains(&r.artifacts.len()),
            "bounded findings and 1..3 artifacts required"
        );
        for f in &r.findings {
            ensure!(
                identifier(&f.code) && f.detail.len() <= 4096,
                "finding outside bounds"
            );
        }
        if let Some(w) = &r.source {
            w.validate()?;
        }
        for a in &r.artifacts {
            ensure!(
                sha(&a.sha256) && a.path.len() <= 512,
                "invalid artifact hash or overlong path"
            );
            let bytes = read_bounded(&confined(&root, &a.path)?)?;
            total += bytes.len();
            ensure!(total <= 1_048_576, "collection byte budget exceeded");
            ensure!(digest(&bytes) == a.sha256, "artifact hash mismatch");
            if matches!(a.format, ArtifactFormat::NativeFishReport) {
                let native: FishReport =
                    serde_json::from_slice(&bytes).context("native Fish artifact schema")?;
                ensure!(
                    native.schema == 1
                        && native.id == r.id
                        && native.actor == r.actor
                        && native.capability == r.kind
                        && native.outcome == r.outcome,
                    "native Fish identity/outcome mismatch"
                );
                ensure!(
                    serde_json::to_vec(&native.source)? == serde_json::to_vec(&r.source)?
                        && serde_json::to_vec(&native.findings)?
                            == serde_json::to_vec(&r.findings)?,
                    "native Fish source/findings mismatch"
                );
            }
        }
    }
    use mlab::value::{StrArr, Value};
    let kind = Value::Str(std::rc::Rc::new(StrArr::col(
        index.reports.iter().map(|r| Some(r.kind.clone())).collect(),
    )));
    let outcome = Value::Str(std::rc::Rc::new(StrArr::col(
        index
            .reports
            .iter()
            .map(|r| Some(r.outcome.key().into()))
            .collect(),
    )));
    // NativeBound is assigned only after every referenced native artifact passed above.
    let bindings: Vec<_> = index.reports.iter().map(evidence_binding).collect();
    let binding = Value::Str(std::rc::Rc::new(StrArr::col(
        bindings.iter().map(|b| Some(b.key().into())).collect(),
    )));
    let groups = mlab::table::groups(&[&kind, &outcome, &binding], index.reports.len())
        .into_iter()
        .map(|g| {
            let first = &index.reports[g[0]];
            NativeGroup {
                kind: first.kind.clone(),
                declared_outcome: first.outcome.key().into(),
                binding: bindings[g[0]],
                count: g.len(),
                report_ids: g.iter().map(|&i| index.reports[i].id.clone()).collect(),
            }
        })
        .collect();
    let report = CrabReport { schema: 1, id: index.id.clone(), input_sha256: digest(&serde_json::to_vec(index)?), groups,
        review_queue: index.reports.clone(), checked_artifact_bytes: total, semantic_admission: false,
        scope: "Mova table grouping of hash-bound declared Fish outcomes; every failure/uncertainty/finding retained; collection is not semantic approval or a trained log classifier".into() };
    fs::create_dir(new_dir).context("new exclusive collection directory required")?;
    json_new(&new_dir.join("report.json"), &report)?;
    json_new(&new_dir.join("review-queue.json"), &report.review_queue)?;
    Ok(report)
}
/// Deterministic synthetic task preparation; no model or inference is run here.
pub fn example(new_dir: &Path) -> Result<()> {
    let sentences = vec![
        Sentence {
            id: "synthetic:1".into(),
            text: "Mary went to the kitchen.".into(),
            speaker: None,
        },
        Sentence {
            id: "synthetic:2".into(),
            text: "Mary picked up the apple.".into(),
            speaker: None,
        },
        Sentence {
            id: "synthetic:3".into(),
            text: "Mary went to the garden.".into(),
            speaker: None,
        },
    ];
    let hash = digest(&serde_json::to_vec(&sentences)?);
    let task = TraceTask {
        schema: 1,
        id: "synthetic-location".into(),
        actor: "specialist-whale".into(),
        subject: "apple".into(),
        locations: vec![
            "garden".into(),
            "kitchen".into(),
            "office".into(),
            "hallway".into(),
        ],
        source: SourceWindow {
            document: "synthetic-location".into(),
            document_sha256: Some(hash.clone()),
            window_sha256: hash,
            sentences,
        },
    };
    fs::create_dir(new_dir)?;
    json_new(&new_dir.join("trace.json"), &task)?;
    let mut unknown = task.clone();
    unknown.id = "synthetic-unknown".into();
    unknown.subject = "Daniel".into();
    json_new(&new_dir.join("unknown.json"), &unknown)?;
    json_new(
        &new_dir.join("recall-template.json"),
        &RecallTask {
            state_sha256: "prepare-after-trace".into(),
            state_fingerprint: "prepare-after-trace".into(),
            trace_report_sha256: "prepare-after-trace".into(),
            trace_id: task.id,
            source_window_sha256: task.source.window_sha256,
            schema: 1,
            id: "synthetic-recall".into(),
            actor: "fish".into(),
            subject: task.subject,
            locations: task.locations,
        },
    )?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    fn scratch(tag: &str) -> std::path::PathBuf {
        static NEXT: std::sync::atomic::AtomicUsize = std::sync::atomic::AtomicUsize::new(0);
        let path = std::env::temp_dir().join(format!(
            "agent-mova-{}-{}-{}",
            std::process::id(),
            tag,
            NEXT.fetch_add(1, std::sync::atomic::Ordering::Relaxed)
        ));
        fs::create_dir(&path).unwrap();
        path
    }
    #[test]
    fn opened_descriptor_rejects_fifo_symlink_and_oversize() {
        use std::os::unix::{ffi::OsStrExt, fs::symlink};
        let root = scratch("files");
        fs::write(root.join("regular"), b"bounded").unwrap();
        assert_eq!(read_bounded(&root.join("regular")).unwrap(), b"bounded");
        symlink("regular", root.join("link")).unwrap();
        assert!(read_bounded(&root.join("link")).is_err());
        let fifo = std::ffi::CString::new(root.join("fifo").as_os_str().as_bytes()).unwrap();
        assert_eq!(unsafe { libc::mkfifo(fifo.as_ptr(), 0o600) }, 0);
        assert!(read_bounded(&root.join("fifo")).is_err());
        fs::File::create(root.join("big"))
            .unwrap()
            .set_len(MAX_FILE_BYTES + 1)
            .unwrap();
        assert!(read_bounded(&root.join("big")).is_err());
        fs::remove_dir_all(root).unwrap();
    }
    #[test]
    fn dialogue_markers_are_exclusions_not_corrected_meanings() {
        for text in [
            "i went to the garden.",
            "MY apple is here.",
            "We went to the kitchen.",
            "‘Mary went to the garden.’",
            "'Mary went to the garden.'",
        ] {
            assert!(outside_context(&Sentence {
                id: "s".into(),
                text: text.into(),
                speaker: None
            }));
        }
        assert!(outside_context(&Sentence {
            id: "s".into(),
            text: "Mary went to the garden.".into(),
            speaker: Some("speaker".into())
        }));
        assert!(!outside_context(&Sentence {
            id: "s".into(),
            text: "Mary went to the garden.".into(),
            speaker: None
        }));
    }
    #[test]
    fn collection_preserves_declared_failures_and_rejects_hash_or_identity_substitution() {
        let root = scratch("collector");
        fs::write(root.join("evidence.txt"), b"uninterpreted external review").unwrap();
        let envelope = ReportEnvelope {
            id: "review-a".into(),
            actor: "fish".into(),
            kind: "external_review".into(),
            outcome: Outcome::Uncertain,
            source: None,
            findings: vec![Finding {
                code: "unknown".into(),
                detail: "retained uncertainty".into(),
            }],
            artifacts: vec![ArtifactRef {
                path: "evidence.txt".into(),
                sha256: digest(b"uninterpreted external review"),
                format: ArtifactFormat::ExternalReviewEvidence,
            }],
        };
        let mut second = envelope.clone();
        second.id = "review-b".into();
        second.outcome = Outcome::Failed;
        let index = ReportIndex {
            schema: 1,
            id: "collection".into(),
            reports: vec![envelope, second],
        };
        let a = collect(&root, &index, &root.join("a")).unwrap();
        let b = collect(&root, &index, &root.join("b")).unwrap();
        assert_eq!(
            serde_json::to_vec(&a).unwrap(),
            serde_json::to_vec(&b).unwrap()
        );
        assert_eq!(a.groups.len(), 2);
        assert_eq!(a.review_queue.len(), 2);
        assert!(!a.semantic_admission);
        assert_eq!(a.review_queue[1].outcome, Outcome::Failed);
        let mut changed = index.clone();
        changed.reports[0].artifacts[0].sha256 = "0".repeat(64);
        assert!(collect(&root, &changed, &root.join("bad-hash")).is_err());
        assert!(!root.join("bad-hash").exists());
        changed = index.clone();
        changed.reports[0].artifacts[0].format = ArtifactFormat::NativeFishReport;
        assert!(collect(&root, &changed, &root.join("plain-as-native")).is_err());
        changed = index.clone();
        changed.reports[1].id = changed.reports[0].id.clone();
        assert!(collect(&root, &changed, &root.join("duplicate")).is_err());
        // A correct content hash does not validate a different report identity or outcome.
        let native = FishReport {
            schema: 1,
            id: "native-a".into(),
            actor: "fish".into(),
            capability: "world_location_trace".into(),
            outcome: Outcome::Uncertain,
            input_sha256: "0".repeat(64),
            source: None,
            question: "Where is the apple?".into(),
            answer: None,
            native_witness: String::new(),
            native_analysis: String::new(),
            findings: vec![],
            before_state_sha256: "1".repeat(64),
            after_state_sha256: "1".repeat(64),
            new_frames: 0,
            changed_weights: 0,
            checkpoint: None,
            scope: "synthetic binding fixture only".into(),
        };
        let bytes = serde_json::to_vec(&native).unwrap();
        fs::write(root.join("native.json"), &bytes).unwrap();
        let native_envelope = ReportEnvelope {
            id: native.id.clone(),
            actor: native.actor.clone(),
            kind: native.capability.clone(),
            outcome: native.outcome.clone(),
            source: None,
            findings: vec![],
            artifacts: vec![ArtifactRef {
                path: "native.json".into(),
                sha256: digest(&bytes),
                format: ArtifactFormat::NativeFishReport,
            }],
        };
        let mut declared_only = native_envelope.clone();
        declared_only.id = "declared-a".into();
        declared_only.artifacts[0].format = ArtifactFormat::ExternalReviewEvidence;
        let mixed = ReportIndex {
            schema: 1,
            id: "mixed-binding".into(),
            reports: vec![native_envelope.clone(), declared_only],
        };
        let mixed = collect(&root, &mixed, &root.join("mixed-binding")).unwrap();
        assert_eq!(mixed.groups.len(), 2);
        assert_eq!(mixed.groups[0].kind, mixed.groups[1].kind);
        assert_eq!(
            mixed.groups[0].declared_outcome,
            mixed.groups[1].declared_outcome
        );
        assert_ne!(mixed.groups[0].binding, mixed.groups[1].binding);
        assert_eq!(mixed.groups.iter().map(|g| g.count).sum::<usize>(), 2);
        let legacy: NativeGroup = serde_json::from_value(serde_json::json!({
            "kind":"world_location_trace","declared_outcome":"supported","count":1,"report_ids":["old"]
        })).unwrap();
        assert_eq!(legacy.binding, EvidenceBinding::LegacyUnclassified);
        let native_index = ReportIndex {
            schema: 1,
            id: "native".into(),
            reports: vec![native_envelope],
        };
        // Exhaust the two-flag projection while every other checked coordinate and
        // the artifact bytes/digest stay fixed. Admission is not semantic approval.
        for (case, (id_matches, outcome_matches, admitted)) in [
            (false, false, false),
            (false, true, false),
            (true, false, false),
            (true, true, true),
        ]
        .into_iter()
        .enumerate()
        {
            let mut declared = native_index.clone();
            if !id_matches {
                declared.reports[0].id = "native-b".into();
            }
            if !outcome_matches {
                declared.reports[0].outcome = Outcome::Supported;
            }
            let out = root.join(format!("native-projection-{case}"));
            assert_eq!(collect(&root, &declared, &out).is_ok(), admitted);
            assert_eq!(out.exists(), admitted);
        }
        fs::remove_dir_all(root).unwrap();
    }

    #[test]
    fn malformed_windows_and_duplicate_alternatives_are_rejected() {
        assert!(
            task_fields(
                1,
                "a",
                "fish",
                "apple",
                &["a".into(), "a".into(), "b".into(), "c".into()]
            )
            .is_err()
        );
        let w = SourceWindow {
            document: "a".into(),
            document_sha256: None,
            window_sha256: "0".repeat(64),
            sentences: vec![Sentence {
                id: "a:1".into(),
                text: "Mary went to the kitchen.".into(),
                speaker: None,
            }],
        };
        assert!(w.validate().is_err());
    }
    #[test]
    fn artifact_paths_refuse_parent_traversal_before_reading() {
        assert!(confined(Path::new("/"), "../escape").is_err());
        assert!(confined(Path::new("/"), "/escape").is_err());
    }
}
