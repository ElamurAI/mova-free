# algorithm-sliding-window-minimum-tests

## descriptant

### sliding_window_minimum tests

Original English Rust implementation and independently authored fixtures for Mova; no external solution code or dependencies.

Useful epoch observation: Strict suffix comparison is necessary to retain earlier equal minima.

All 9,841 ternary arrays of lengths 0..8, every width 0..n+1 plus usize::MAX; independent per-window scans compare exact indices. Increasing, decreasing, equal, expired and extreme-value cases and shift metamorphism included. Compiled mutant changing suffix comparison > to >= was rejected by expiration_and_equal_values_preserve_leftmost_ties (exit 101): equal array width 2 produced [1,2,3] instead of [0,1,2].

Unknown: exhaustive generic-value semantics, panicking comparison behavior and resource failures are not verified. No executable approximation budget applies to this exact algorithm.

## model

# sliding_window_minimum tests model

## Forward

Run deterministic finite fixtures and independently scan candidate minima; assertions pass exactly when every declared equality and negative control holds. All 9,841 ternary arrays of lengths 0..8, every width 0..n+1 plus usize::MAX; independent per-window scans compare exact indices. Increasing, decreasing, equal, expired and extreme-value cases and shift metamorphism included. Compiled mutant changing suffix comparison > to >= was rejected by expiration_and_equal_values_preserve_leftmost_ties (exit 101): equal array width 2 produced [1,2,3] instead of [0,1,2].

## Invariant

Enumeration visits every ternary code once per length. Reference scans share no tree/deque logic. Inputs are checked unchanged; each complete window is compared with an independent linear scan. Tests depend on standard assertions and the implementation under test.

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

Passing requires the declared production observations to satisfy their decomposition, transport, containment, simultaneous-fiber and order relations on every enumerated fixture. Premises are checked separately by explicit counterexamples. Relations may miss correlated errors, so independent scan checks remain necessary.

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

All 9,841 ternary arrays of lengths 0..8, every width 0..n+1 plus usize::MAX; independent per-window scans compare exact indices. Increasing, decreasing, equal, expired and extreme-value cases and shift metamorphism included. Compiled mutant changing suffix comparison > to >= was rejected by expiration_and_equal_values_preserve_leftmost_ties (exit 101): equal array width 2 produced [1,2,3] instead of [0,1,2].

Four tests passed in debug and four in release. Strict crate Clippy with all targets and -D warnings passed. Mutants compiled successfully, failed assertions, and were restored before final checks. These are finite observations, not a general proof; costs are symbolic and timings are not performance claims.

## History

### 2026-10-30 · descriptant

2026-10-30 prospective model: Valid widths: O(n) comparisons, O(w) auxiliary indices, O(n-w+1) output indices. Invalid widths O(1). No clones; comparison costs are user-defined; allocation failure outside model. Correctness predicts exact leftmost indices, not merely equal minimum values. Measurements confirmed declared finite cases; no timing gain measured. Tests deliberately cover invalid and boundary domains, with compiled mutation controls.

#### Deep relation epoch — 2026-10-30

Prediction: decomposition and strict order transport preserve leftmost witnesses; reconstruction depends on which values are observed. Strengthen the models without changing production code or claiming a cost improvement. New correspondence tests distinguish valid relations from non-strict order collapse, index-only recovery and missing locality premises. Earlier evidence remains historical. Current source-bound checks and compiled controls are recorded in model.md#evidence. Symbolic costs are unchanged; no timing or approximate error profile is introduced. Supported executable approximation caps remain 1%, 2%, 5%, 10% if a future profile is supplied.

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

Earlier epoch source SHA-256: 4684f12ba0721e2b9925e4d1e62218e81a0deb640db8c09391c92ffec13791cf.

#### Deep relation epoch — 2026-10-30

Current bound source SHA-256: 58b0da209c410927291aa8471e834182399ff841686afb0dcdcabcf8631fdb27.

Three added tests exercise the real production functions. Seven tests per algorithm pass in debug and release, with repeated deterministic outcome sets. Strict Clippy and formatting pass. New syntax projections are regenerated twice and byte-compared. Details of compiled mutation rejection follow below. All observations are finite; there is no speed or unrestricted proof claim.

Compiled production control: replacing `values[i] > *value` with `values[i] >= *value` was rejected by `simultaneous_output_fibers_and_width_one_information_loss` with assertion failure and exit 101. The source was restored before final checks. Two debug and two release runs have identical sorted sets of seven passing test names per algorithm. External logs: `$MOVA_DATA/runs/octopus-deep-ranges-2026-10-30`.

New shift, decomposition and contained-witness oracle visits all 1,092 ternary arrays of lengths 1..6, every valid width, slice offset, split point and retained-witness subwindow. The simultaneous reverse-fiber oracle checks 81 length-four ternary arrays against all 64 candidate length-three index vectors: 5,184 membership checks. Invalid widths and non-strict order-collapse counterexamples remain explicit.
