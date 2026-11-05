# mathematical-twin

## model

# Mathematical twin interaction model

## Forward approximant

For model M=(theta,P,n,SSE,lo,hi), input x and positive observed duration y,
the ideal update uses h=x^T P x, e=y-x^T theta and d=1+h:

    theta' = theta + P*x*e/d
    P' = P - (P*x)*(P*x)^T/d
    n' = n+1; SSE' = SSE+e^2.

The implementation clones, updates and validates the candidate before replacing
M. The residual is prospective: it compares y to the earlier forecast. The
matrix is inverse information, not calibrated predictive covariance. Prediction
may be negative even though accepted observed durations must be positive.

## Composition

CLI validate/fingerprint/domain -> library prediction -> forecast persistence ->
explicit requested command -> output/fingerprint checks -> observation persistence ->
library observation update -> checkpoint persistence. Predict mode stops before
execution. Failed observations retain evidence and reject the normal update path;
I/O errors can leave partial directories and files.

Cohort identity includes named context, fixed arguments/settings, feature names,
prior, ridge, output checks and fingerprints. Variable feature values and named
environment axes are allowed within a cohort. Parent preparation checks evidence
hashes and replays the previous update; filesystem trust and collision resistance
remain assumptions. This is a source-derived composition, not a new process run.

## Invariant

Feature ordering uses BTreeMap keys. Model dimensions, finite values and positive
information are checked at prediction/update boundaries. Forecast persistence
precedes command launch on the normal run path. Hash binding establishes byte
identity; it does not establish scientific adequacy or arbitrary external state.
No default clipping makes a negative forecast physically meaningful.

## Epoch memory

This group registers the existing mathematical-twin library and CLI.
Their source snapshots and child contracts now participate in root closure.
Known limits include a tiny-ridge constructor result later rejected by validation,
unvalidated min/max ordering/history consistency, count-overflow assumptions,
and the replay helper's zip-based comparison. These are scoped observations,
not newly fixed code or an end-to-end correctness claim. Exact arithmetic and
finite-precision behavior remain distinct.

## References

Sherman and Morrison (1950), doi:10.1214/aoms/1177729893, rank-one inverse.
Kalman (1960), doi:10.1115/1.3662552, sequential correction.
These are already cited in the library; neither source grants the runtime
model calibrated prediction intervals.

## catalog

# Typed interaction models

All relations have the assumptions and dependency limits in model.md. These are
source-derived partial models; check references do not imply universal proof.

## excluded-observations

Type: behavior_complement.

U consists of fixed-snapshot CLI run records and model transitions; an inner bound on U minus actual R contains a normal updated checkpoint after a rejected observation, or admitted prediction with invalid feature dimensions. I/O failures may retain prefixes.

Evidence: source inspection and the scoped child checks listed in model.md#tests.

## compatible-histories

Type: reverse_reconstruction.

Pre(K) contains all compatible specs, prospective forecasts and accepted measurements whose validated updates yield K. Aggregate parameters and residual totals leave ordering/history ambiguous; hashes constrain bytes under collision-resistance assumptions.

Evidence: source inspection and the scoped child checks listed in model.md#tests.

## cohort-boundary

Type: cohort_partition.

The CLI cohort hash includes binary/input fingerprints and declared fixed settings. Feature values and named variable environment axes can vary within the cohort. No claim that undeclared hardware or library state is captured.

Evidence: source inspection and the scoped child checks listed in model.md#tests.

## finite-update-domain

Type: numerical_domain.

Exact RLS relations hold algebraically under positive information and valid dimensions; implementation admission additionally requires finite arithmetic and Cholesky checks. Tiny positive ridge can create a nonfinite inverse before later validation rejects it.

Evidence: source inspection and the scoped child checks listed in model.md#tests.

## replay-converse

Type: evidence_replay_preimage.

Parent preparation replays the previous observation and compares the model within tolerance. Same-model is not an authenticated history proof; its helper uses zip and has no standalone dimension-equality gate.

Evidence: source inspection and the scoped child checks listed in model.md#tests.

## cost-decomposition

Type: resource_approximation.

Prediction calls Cholesky validation, so its actual bound includes O(d^3), not only dot products. Update validates through prediction and candidate validation; command execution and hashing are separate measured terms.

Evidence: source inspection and the scoped child checks listed in model.md#tests.

## tests

### Evidence and limits

- mova-twin existing library tests
- Structural source-bound groupant inventory

The integrated inventory checks child identities and transitive source hashes.
It does not execute model text or certify semantic composition. No new end-to-end
process trace is implied by registering this group. Executed checks and exact
scope are recorded in the child sidecars and integration worklog.

## History

### 2026-10-29 · Epoch memory

2026-10-29: this group registers the existing mathematical-twin library and CLI.
Their source snapshots and child contracts now participate in root closure.
Known limits include a tiny-ridge constructor result later rejected by validation,
unvalidated min/max ordering/history consistency, count-overflow assumptions,
and the replay helper's zip-based comparison. These are scoped observations,
not newly fixed code or an end-to-end correctness claim. Exact arithmetic and
finite-precision behavior remain distinct.
