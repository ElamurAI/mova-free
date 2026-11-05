# mova-eel-receipt-evidence

## model

# Declared terminal evidence

## Descriptant

Typed interpretation of historical trainer receipts and Eel summaries. This is
an observation model of declared JSON, not a verifier of model files or a claim
that a subprocess executed. Only installed, rejected, below_threshold and deferred
are admitted outcomes. No filesystem/provider/GPU effects originate in this file.

## Forward

Let O be the outcome, G a valid generation digest, P an actual-parent digest,
C a candidate digest and I an installed digest. Digest validity is exactly 64
lowercase ASCII hexadecimal characters. Generation and actual-parent fields accept
both old trainer names and Eel summary aliases. Proposed-parent is never P.
The classifier is E(O,G,P,C,I): deferred/below_threshold -> NoTrainingRecorded;
rejected and G&P&C -> CandidateRejectionRecorded; installed and G&P&I ->
InstallationRecorded; otherwise -> IncompleteLineage. Unknown/failed outcomes error.
No numeric approximation or probabilistic error rate is admitted.

## Invariant

Changing only proposed-parent, round count, rejection streak or wall time cannot
promote a receipt to recorded training. Missing/invalid digest fields stay None.
Returned lineage is a projection of declared fields, never reconstructed bytes.
Historical partial installed/rejected receipts stay readable as incomplete.

## Cost

Fixed field count and bounded 64-byte digest validation; output clones at most
four valid digest strings. Input JSON parsing/allocation belongs to the caller.
No measured speed, real execution or freshness guarantee follows.

## Composition

Native trainer terminal receipt -> Eel alias summary -> this classification. Full
legacy digest tuples retain outcome-specific interpretation across aliases. The
projection loses the original error text, metrics, config and artifact existence.
It does not certify that a summary and trainer receipt agree on disk.

## behavior_complement

Domain: JSON terminal outcome/digest tuples. Observation: TrainingEvidence.
Assumptions: this is interpretation of claims. Relation: deferred and threshold
outcomes exclude CandidateRejectionRecorded and InstallationRecorded regardless
of counter/streak/proposed-parent fields. Counterexample: deferred round1709 with
reject_streak9 did not itself train or reject. Evidence: three finite Rust controls;
source-derived model, no formal proof or artifact-authentication claim.

## reverse_reconstruction

Domain: JSON receipts. Observation: one classified ReceiptEvidence. Assumptions:
same alias and digest admission rules. Relation: preimages preserve all variations
in omitted metrics/errors/counters/proposed-parent and invalid/missing lineage.
Counterexample: two deferred receipts with different proposed parent weights
classify identically; original lineage intent cannot be uniquely reconstructed.
Evidence: source inspection and finite non-promotion/alias controls, not a proof.

## actual_parent_exclusion

Domain: partial receipt lineage. Observation: P presence and classified evidence.
Assumptions: polling and acquisition may race; proposed weights precede trainer
lease. Relation: proposed-parent alone cannot inhabit actual-parent fiber; require
actual_parent_weights_sha256 or old_weights_sha256. Counterexample: another trainer
may install between probe and actual lease. Evidence: explicit proposed-only test.
This partial exclusion does not prove that a supplied actual-parent digest is true.

## outcome_error_partition

Domain: known outcomes plus malformed/missing outcome. Observation: Result and E.
Assumptions: JSON values remain caller-owned. Relation: unknown/failed -> Err;
known partial -> Ok(IncompleteLineage), rather than inventing training or dropping
historical records. Counterexample: valid-generation installed without installed
weights stays incomplete. Evidence: missing-output and uppercase/partial controls.

## tests

Three unit controls cover proposed-only/deferred non-promotion, outcome-specific
actual lineage, and historical partial/failed/uppercase partitions. Public native
entrypoint additionally checks full installed receipt interpretation. Finite
JSON/script fixtures do not authenticate operational artifacts.
