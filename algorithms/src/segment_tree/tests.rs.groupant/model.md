# algorithm-segment-tree-tests

## descriptant

### segment_tree tests

Original English Rust implementation and independently authored fixtures for Mova; no external solution code or dependencies.

Useful epoch observation: A compact non-power-of-two tree requires index-based ties rather than tree traversal ties.

All 1,093 ternary arrays of lengths 0..6; every point independently replaced by each value -2..2; all start/end pairs through n+1 checked against a linear scan, then restoration checked by complete state equality. Sequential updates and invalid-index atomicity also checked. Compiled mutant changing ancestor loop node > 0 to node > 1 was rejected by exhaustive_point_updates_and_ranges (exit 101), witness [0,-1], range 0..2.

Unknown: exhaustive generic-value semantics, panicking comparison behavior and resource failures are not verified. No executable approximation budget applies to this exact algorithm.

## model

# segment_tree tests model

## Forward

Run deterministic finite fixtures and independently scan candidate minima; assertions pass exactly when every declared equality and negative control holds. All 1,093 ternary arrays of lengths 0..6; every point independently replaced by each value -2..2; all start/end pairs through n+1 checked against a linear scan, then restoration checked by complete state equality. Sequential updates and invalid-index atomicity also checked. Compiled mutant changing ancestor loop node > 0 to node > 1 was rejected by exhaustive_point_updates_and_ranges (exit 101), witness [0,-1], range 0..2.

## Invariant

Enumeration visits every ternary code once per length. Reference scans share no tree/deque logic. Inputs are checked unchanged; tree updates are compared with copied arrays. Tests depend on standard assertions and the implementation under test.

## Cost

Finite exponential enumeration in maximum length with polynomial range/update scans; maximum lengths are fixed in source. No runtime or all-input proof claim.

## nonbehavior

Type: behavior_complement. Approximation direction: proposed inner bound of absent behavior under the stated premises; finite observations do not prove universal exclusion. In the universe of implementations and test outcomes, any passing implementation that disagrees with an asserted sampled oracle equality is excluded, assuming faithful assertions. Unsampled bugs may still pass.

## compatible_antecedents

Type: reverse_reconstruction. A pass reconstructs the set of implementations agreeing on all sampled observations; a failure reconstructs those violating at least one assertion. Neither observation uniquely recovers implementation or universal correctness.

## Limits

Finite evidence is not proof. Syntax projection is partial; no macro expansion or semantic resolution. Exact reference implementation; no executable lossy approximant or measured error profile is claimed.

## metamorphic_oracle

Type: metamorphic_test_relation. Domain: The exact finite fixtures and assertions in this test source, with the production implementation under test.

Passing requires the declared production observations to satisfy their decomposition, transport, reconstruction and update relations on every enumerated fixture. Premises are checked separately by explicit counterexamples. Relations may miss correlated errors, so independent scan checks remain necessary.

Oracle: three new correspondence tests. Finite observations support this relation; the general statement remains source-derived.

## premise_boundary

Type: assumption_boundary_relation. Domain: The exact finite fixtures and assertions in this test source, with the production implementation under test.

Observation of the non-strict order-collapse counterexample excludes extending strict order-embedding claims to merely monotone maps. Index-only observations retain ambiguous value preimages. Passing these assertions constrains the claims on these concrete inputs; it does not establish their maximal domains.

Oracle: explicit missing-premise assertions. Finite observations support this relation; the general statement remains source-derived.

## mutation_partition

Type: mutation_detection_relation. Domain: The exact finite fixtures and assertions in this test source, with the production implementation under test.

The finite test suite partitions implementations into accepted and rejected sets. The recorded compiled production mutant belongs to the rejected set at a named correspondence assertion. This is sensitivity to that defect, not completeness for all defects or a general proof of the model.

Oracle: compiled mutant in model.md#evidence. Finite observations support this relation; the general statement remains source-derived.

## evidence

### Evidence

All 1,093 ternary arrays of lengths 0..6; every point independently replaced by each value -2..2; all start/end pairs through n+1 checked against a linear scan, then restoration checked by complete state equality. Sequential updates and invalid-index atomicity also checked. Compiled mutant changing ancestor loop node > 0 to node > 1 was rejected by exhaustive_point_updates_and_ranges (exit 101), witness [0,-1], range 0..2.

Four tests passed in debug and four in release. Strict crate Clippy with all targets and -D warnings passed. Mutants compiled successfully, failed assertions, and were restored before final checks. These are finite observations, not a general proof; costs are symbolic and timings are not performance claims.

## History

### 2026-10-30 · descriptant

2026-10-30 prospective model: Construction O(n) comparisons and storage, n lawful clones. Query/update O(log(n+1)) comparisons, O(1) workspace. Invalid queries/updates O(1). Comparisons and clones have user-defined cost. Capacity checked before doubling; allocation/capacity panic outside result model. Correctness predicts exact leftmost indices, not merely equal minimum values. Measurements confirmed declared finite cases; no timing gain measured. Tests deliberately cover invalid and boundary domains, with compiled mutation controls.

#### Deep relation epoch — 2026-10-30

Prediction: decomposition and strict order transport preserve leftmost witnesses; reconstruction depends on which values are observed. Strengthen the models without changing production code or claiming a cost improvement. New correspondence tests distinguish valid relations from non-strict order collapse, index-only recovery and missing locality premises. Earlier evidence remains historical. Current source-bound checks and compiled controls are recorded in model.md#evidence. Symbolic costs are unchanged; no timing or approximate error profile is introduced. Supported executable approximation caps remain 1%, 2%, 5%, 10% if a future profile is supplied.

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

Earlier epoch source SHA-256: 6805c0bc3de0c914363b38f8a93d839fb0d352ebd6ce268caa973a1426f29357.

#### Deep relation epoch — 2026-10-30

Current bound source SHA-256: cbef6c9887ed574315295467305832f92644f442aea3c1e377097984d2211a37.

Three added tests exercise the real production functions. Seven tests per algorithm pass in debug and release, with repeated deterministic outcome sets. Strict Clippy and formatting pass. New syntax projections are regenerated twice and byte-compared. Details of compiled mutation rejection follow below. All observations are finite; there is no speed or unrestricted proof claim.

Compiled production control: replacing `std::cmp::Ordering::Equal => a.min(b)` with `std::cmp::Ordering::Equal => a.max(b)` was rejected by `semilattice_decomposition_and_singleton_reconstruction` with assertion failure and exit 101. The source was restored before final checks. Two debug and two release runs have identical sorted sets of seven passing test names per algorithm. External logs: `$MOVA_DATA/runs/octopus-deep-ranges-2026-10-30`.

New decomposition and singleton-value oracle visits all 1,092 ternary arrays of lengths 1..6 and all valid split points, including empty split pieces. Update locality, commutation, same-index noncommutation, invalid-index preservation and snapshot isolation use every ordered pair of seven indices. The order-embedding test checks every valid range of a fixed seven-value array and rejects a non-strict collapse.
