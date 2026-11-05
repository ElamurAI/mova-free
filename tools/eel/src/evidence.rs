//! Typed interpretation of declared receipt evidence; no filesystem authenticity claim.
use crate::Result;
use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum RoundOutcome {
    Deferred,
    BelowThreshold,
    Rejected,
    Installed,
}
#[derive(Clone, Copy, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum TrainingEvidence {
    /// No training is recorded by this outcome, even if the round counter advanced.
    NoTrainingRecorded,
    CandidateRejectionRecorded,
    InstallationRecorded,
    /// Old or partial receipts remain readable without manufacturing lineage.
    IncompleteLineage,
}
#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReceiptEvidence {
    pub outcome: RoundOutcome,
    pub training_evidence: TrainingEvidence,
    pub generation: Option<String>,
    pub parent_weights: Option<String>,
    pub candidate_weights: Option<String>,
    pub installed_weights: Option<String>,
}
pub(crate) fn is_sha256(s: &str) -> bool {
    s.len() == 64
        && s.bytes()
            .all(|b| b.is_ascii_hexdigit() && !b.is_ascii_uppercase())
}
fn digest(value: &Value, names: &[&str]) -> Option<String> {
    names.iter().find_map(|name| {
        value[*name]
            .as_str()
            .filter(|s| is_sha256(s))
            .map(str::to_owned)
    })
}
/// Read either trainer receipts or historical Eel summaries. A proposed-parent
/// hash is deliberately excluded: it precedes the independently acquired lease.
pub fn inspect(value: &Value) -> Result<ReceiptEvidence> {
    let outcome = match value["outcome"].as_str() {
        Some("deferred") => RoundOutcome::Deferred,
        Some("below_threshold") => RoundOutcome::BelowThreshold,
        Some("rejected") => RoundOutcome::Rejected,
        Some("installed") => RoundOutcome::Installed,
        _ => return Err("eel: unknown or nonterminal receipt outcome".into()),
    };
    let generation = digest(value, &["trainer_generation", "generation"]);
    let parent_weights = digest(
        value,
        &["actual_parent_weights_sha256", "old_weights_sha256"],
    );
    let candidate_weights = digest(value, &["candidate_weights_sha256"]);
    let installed_weights = digest(value, &["installed_weights_sha256"]);
    let training_evidence = match outcome {
        RoundOutcome::Deferred | RoundOutcome::BelowThreshold => {
            TrainingEvidence::NoTrainingRecorded
        }
        RoundOutcome::Rejected
            if generation.is_some() && parent_weights.is_some() && candidate_weights.is_some() =>
        {
            TrainingEvidence::CandidateRejectionRecorded
        }
        RoundOutcome::Installed
            if generation.is_some() && parent_weights.is_some() && installed_weights.is_some() =>
        {
            TrainingEvidence::InstallationRecorded
        }
        _ => TrainingEvidence::IncompleteLineage,
    };
    Ok(ReceiptEvidence {
        outcome,
        training_evidence,
        generation,
        parent_weights,
        candidate_weights,
        installed_weights,
    })
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde_json::json;
    #[test]
    fn advancing_deferred_counter_and_proposed_parent_are_not_training() {
        let r = json!({"outcome":"deferred","round":1709,"reject_streak":9,
            "proposed_parent_weights_sha256":"a".repeat(64),
            "actual_parent_weights_sha256":null,"trainer_generation":null});
        let e = inspect(&r).unwrap();
        assert_eq!(e.training_evidence, TrainingEvidence::NoTrainingRecorded);
        assert_eq!(e.parent_weights, None);
        assert_eq!(e.generation, None);
    }
    #[test]
    fn actual_lineage_and_outcome_jointly_classify_observations() {
        let mut r = json!({"outcome":"rejected","generation":"b".repeat(64),
            "old_weights_sha256":"c".repeat(64),"candidate_weights_sha256":"d".repeat(64)});
        assert_eq!(
            inspect(&r).unwrap().training_evidence,
            TrainingEvidence::CandidateRejectionRecorded
        );
        r["outcome"] = json!("installed");
        assert_eq!(
            inspect(&r).unwrap().training_evidence,
            TrainingEvidence::IncompleteLineage
        );
        r["installed_weights_sha256"] = json!("e".repeat(64));
        assert_eq!(
            inspect(&r).unwrap().training_evidence,
            TrainingEvidence::InstallationRecorded
        );
        r["outcome"] = json!("below_threshold");
        assert_eq!(
            inspect(&r).unwrap().training_evidence,
            TrainingEvidence::NoTrainingRecorded
        );
    }
    #[test]
    fn historical_partial_receipts_are_readable_but_failed_is_not_terminal_success() {
        let e = inspect(&json!({"outcome":"installed","generation":"old"})).unwrap();
        assert_eq!(e.training_evidence, TrainingEvidence::IncompleteLineage);
        assert!(inspect(&json!({"outcome":"failed"})).is_err());
        assert!(
            inspect(&json!({"outcome":"rejected","generation":"A".repeat(64)}))
                .unwrap()
                .generation
                .is_none()
        );
    }
}
