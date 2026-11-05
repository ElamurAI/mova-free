//! Public Crab: bounded control, source-bound annotation storage and worker supervision.
//! Deployment queues, paths, credentials and specialized role handlers are supplied externally.
pub type Result<T> = std::result::Result<T, Box<dyn std::error::Error>>;
pub mod angel;
pub mod consult_queue;
pub mod dolphin;
pub mod flow;
pub mod pool;
pub mod council;
pub mod store;
pub mod supervisor;
pub mod tmux_worker;
pub mod workers;

/// Public default Fish pool when a deployment config names no `workers` count.
/// Two is the recommended minimum; deployments may run more, up to eight.
pub const DEFAULT_FISH_WORKERS: u64 = 2;

/// Public native Mova request/report types and capabilities; concrete tasks stay external.
pub use agent_mova as native_mova;

/// Call Mova's bounded table grouping and preserve the complete Fish review queue.
///
/// Native report references enforce schema, identity, source, findings and outcome binding.
/// External review references retain explicitly declared labels over hash-bound bytes.
/// This is deterministic systematization, not a learned classifier or semantic approval.
pub fn collect_fish_reports(
    root: &std::path::Path,
    index: &native_mova::ReportIndex,
    new_dir: &std::path::Path,
) -> Result<native_mova::CrabReport> {
    Ok(native_mova::collect(root, index, new_dir)?)
}

/// Recall a pinned existing Mova graph without source replay, training or mutation.
/// The native capability checks checkpoint, parent trace and source-window lineage.
pub fn recall_fish_state(
    task: &native_mova::RecallTask,
    state: &std::path::Path,
    trace_report: &std::path::Path,
) -> Result<native_mova::FishReport> {
    Ok(native_mova::recall(task, state, trace_report)?)
}

#[cfg(test)]
mod native_bridge_checks {
    use super::*;
    use native_mova::{ArtifactFormat, ArtifactRef, Finding, Outcome, ReportEnvelope, ReportIndex};
    use serde_json::json;
    use std::{
        fs,
        path::PathBuf,
        sync::atomic::{AtomicU64, Ordering},
    };
    static NEXT: AtomicU64 = AtomicU64::new(0);
    struct Fixture(PathBuf);
    impl Fixture {
        fn new() -> Self {
            let p = std::env::temp_dir().join(format!(
                "crab-native-{}-{}",
                std::process::id(),
                NEXT.fetch_add(1, Ordering::Relaxed)
            ));
            fs::create_dir(&p).unwrap();
            Self(p)
        }
        fn native(&self) -> ReportEnvelope {
            // A declared uncertain fixture, not a new inference or graph-ingestion run.
            let report = json!({
                "schema":1,"id":"native0","actor":"fish","capability":"world_location_trace",
                "outcome":"uncertain","input_sha256":"0".repeat(64),"source":null,
                "question":"Where is the apple?","answer":null,"native_witness":"fixture only",
                "native_analysis":"unknown","findings":[{"code":"native_answer_gap","detail":"fixture unknown"}],
                "before_state_sha256":"0".repeat(64),"after_state_sha256":"0".repeat(64),
                "new_frames":0,"changed_weights":0,"checkpoint":null,"scope":"local collection control"
            });
            let bytes = serde_json::to_vec(&report).unwrap();
            fs::write(self.0.join("native.json"), &bytes).unwrap();
            ReportEnvelope {
                id: "native0".into(),
                actor: "fish".into(),
                kind: "world_location_trace".into(),
                outcome: Outcome::Uncertain,
                source: None,
                findings: vec![Finding {
                    code: "native_answer_gap".into(),
                    detail: "fixture unknown".into(),
                }],
                artifacts: vec![ArtifactRef {
                    path: "native.json".into(),
                    sha256: native_mova::digest(&bytes),
                    format: ArtifactFormat::NativeFishReport,
                }],
            }
        }
    }
    impl Drop for Fixture {
        fn drop(&mut self) {
            let _ = fs::remove_dir_all(&self.0);
        }
    }

    #[test]
    fn collection_reaches_native_mova_and_keeps_external_review_labels_explicit() {
        let f = Fixture::new();
        let native = f.native();
        let external = b"TUI review content: an unverified claim.";
        fs::write(f.0.join("external.txt"), external).unwrap();
        let envelope = ReportEnvelope {
            id: "external0".into(),
            actor: "fish".into(),
            kind: "external_review".into(),
            outcome: Outcome::Rejected,
            source: None,
            findings: vec![Finding {
                code: "root_review".into(),
                detail: "caller-declared rejection".into(),
            }],
            artifacts: vec![ArtifactRef {
                path: "external.txt".into(),
                sha256: native_mova::digest(external),
                format: ArtifactFormat::ExternalReviewEvidence,
            }],
        };
        let index = ReportIndex {
            schema: 1,
            id: "collection0".into(),
            reports: vec![native, envelope],
        };
        let first = collect_fish_reports(&f.0, &index, &f.0.join("first")).unwrap();
        let second = collect_fish_reports(&f.0, &index, &f.0.join("second")).unwrap();
        assert_eq!(first.groups.len(), 2);
        assert_eq!(first.groups.iter().map(|g| g.count).sum::<usize>(), 2);
        assert!(!first.semantic_admission);
        assert_eq!(
            serde_json::to_vec(&first).unwrap(),
            serde_json::to_vec(&second).unwrap()
        );
        assert_eq!(
            serde_json::to_value(&first.review_queue).unwrap(),
            serde_json::to_value(&index.reports).unwrap()
        );
        assert!(matches!(
            first.review_queue[1].artifacts[0].format,
            ArtifactFormat::ExternalReviewEvidence
        ));
        assert!(collect_fish_reports(&f.0, &index, &f.0.join("first")).is_err());
    }

    #[test]
    fn native_identity_source_outcome_and_hash_substitution_fail_before_output() {
        let f = Fixture::new();
        let original = f.native();
        for mode in 0..5 {
            let mut envelope = original.clone();
            match mode {
                0 => envelope.id = "wrong".into(),
                1 => envelope.outcome = Outcome::Supported,
                2 => envelope.artifacts[0].sha256 = "f".repeat(64),
                3 => envelope.findings.clear(),
                _ => {
                    let sentences = vec![native_mova::Sentence {
                        id: "s1".into(),
                        text: "Mary went to the kitchen.".into(),
                        speaker: None,
                    }];
                    envelope.source = Some(native_mova::SourceWindow {
                        document: "other".into(),
                        document_sha256: None,
                        window_sha256: native_mova::digest(
                            &serde_json::to_vec(&sentences).unwrap(),
                        ),
                        sentences,
                    });
                }
            }
            let out = f.0.join(format!("refused-{mode}"));
            let index = ReportIndex {
                schema: 1,
                id: "negative".into(),
                reports: vec![envelope],
            };
            assert!(collect_fish_reports(&f.0, &index, &out).is_err());
            assert!(!out.exists());
        }
        let mut report: serde_json::Value =
            serde_json::from_slice(&fs::read(f.0.join("native.json")).unwrap()).unwrap();
        report["schema"] = json!(2);
        let bytes = serde_json::to_vec(&report).unwrap();
        fs::write(f.0.join("native.json"), &bytes).unwrap();
        let mut envelope = original;
        envelope.artifacts[0].sha256 = native_mova::digest(&bytes);
        let index = ReportIndex {
            schema: 1,
            id: "wrong-schema".into(),
            reports: vec![envelope],
        };
        assert!(collect_fish_reports(&f.0, &index, &f.0.join("schema-refused")).is_err());
    }

    #[test]
    fn recall_bridge_rejects_changed_checkpoint_before_native_decoding() {
        let f = Fixture::new();
        fs::write(f.0.join("state.json"), b"not a checkpoint").unwrap();
        let task = native_mova::RecallTask {
            schema: 1,
            id: "recall0".into(),
            actor: "fish".into(),
            subject: "apple".into(),
            locations: vec![
                "garden".into(),
                "kitchen".into(),
                "office".into(),
                "hallway".into(),
            ],
            state_sha256: "0".repeat(64),
            state_fingerprint: "0".repeat(64),
            trace_report_sha256: "0".repeat(64),
            trace_id: "trace0".into(),
            source_window_sha256: "0".repeat(64),
        };
        let error = recall_fish_state(
            &task,
            &f.0.join("state.json"),
            &f.0.join("absent-trace.json"),
        )
        .unwrap_err();
        assert!(error.to_string().contains("state byte binding mismatch"));
        assert_eq!(
            fs::read(f.0.join("state.json")).unwrap(),
            b"not a checkpoint"
        );
    }
}
