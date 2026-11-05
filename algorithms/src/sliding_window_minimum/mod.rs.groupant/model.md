# algorithm-sliding-window-minimum

## descriptant

### sliding_window_minimum mod

Original English Rust implementation and independently authored fixtures for Mova; no external solution code or dependencies.

Useful epoch observation: Strict suffix comparison is necessary to retain earlier equal minima.

All 9,841 ternary arrays of lengths 0..8, every width 0..n+1 plus usize::MAX; independent per-window scans compare exact indices. Increasing, decreasing, equal, expired and extreme-value cases and shift metamorphism included. Compiled mutant changing suffix comparison > to >= was rejected by expiration_and_equal_values_preserve_leftmost_ties (exit 101): equal array width 2 produced [1,2,3] instead of [0,1,2].

Unknown: exhaustive generic-value semantics, panicking comparison behavior and resource failures are not verified. No executable approximation budget applies to this exact algorithm.

## model

# sliding_window_minimum mod model

## Forward

For input A length n and width w, return [] if w=0 or w>n. Otherwise output n-w+1 indices; output[s] is the smallest index minimizing A on [s,s+w). Input is borrowed and unchanged.

## Invariant

Deque indices strictly increase, are unexpired, and their values are nondecreasing. A removed suffix index has a strictly smaller, later dominator that expires no sooner. Equal values remain, so the front is always the leftmost minimum. Each input index is pushed once and removed at most once.

## Cost

Valid widths: O(n) comparisons, O(w) auxiliary indices, O(n-w+1) output indices. Invalid widths O(1). No clones; comparison costs are user-defined; allocation failure outside model.

## nonbehavior

Type: behavior_complement. Approximation direction: proposed inner bound of absent behavior under the stated premises; finite observations do not prove universal exclusion. In the universe of (A,w,index vector), exclude nonempty outputs for zero/oversized widths, wrong output lengths for valid widths, out-of-window indices, nonminimal values and rightward tie choices. Input mutation is excluded.

## compatible_antecedents

Type: reverse_reconstruction. An output vector J admits widths and arrays satisfying n=|J|+w-1 and, for each s, s<=J[s]<s+w, A[J[s]]<=A[k] throughout that window and strict inequality for k<J[s]. Empty output admits w=0 or w>n. Many arrays satisfy these constraints; unique reconstruction is not claimed.

## Limits

Finite evidence is not proof. Syntax projection is partial; no macro expansion or semantic resolution. Exact reference implementation; no executable lossy approximant or measured error profile is claimed.

## window_decomposition

Type: decomposition_relation. Domain: Finite lawful ordered value arrays, successful allocation, nonpanicking operations; fixed lengths, ranges and transformations as stated.

For any valid window R partitioned into adjacent subranges, its witness is the lexicographic minimum (A[i],i) of the subrange witnesses. None is the identity for an empty subrange. Equal values require minimum source index, not arbitrary deque insertion position.

Oracle: witness_shift_and_contained_window_retention. Finite observations support this relation; the general statement remains source-derived.

## slice_witness_shift

Type: index_transport_relation. Domain: Finite lawful ordered value arrays, successful allocation, nonpanicking operations; fixed lengths, ranges and transformations as stated.

For fixed positive w and 0<=k<=n-w, let J=minimum_indices(A,w), L=minimum_indices(A[k..],w). For every local start s, L[s]+k=J[s+k]. Local indices must be shifted before comparison; raw equality is not a correct sliced-window contract.

Oracle: witness_shift_and_contained_window_retention. Finite observations support this relation; the general statement remains source-derived.

## contained_witness

Type: containment_relation. Domain: Finite lawful ordered value arrays, successful allocation, nonpanicking operations; fixed lengths, ranges and transformations as stated.

If j is the leftmost argmin of R and j belongs to nonempty subwindow S contained in R, j is also the leftmost argmin of S. Containment alone is insufficient when S drops j: [0,2,1] has global witness 0, while suffix [2,1] has local witness 1.

Oracle: witness_shift_and_contained_window_retention. Finite observations support this relation; the general statement remains source-derived.

## order_embedding

Type: order_embedding_relation. Domain: Finite lawful ordered value arrays, successful allocation, nonpanicking operations; fixed lengths, ranges and transformations as stated.

An order embedding f preserves the entire output vector for every width, including invalid widths. Equalities and strict order on observed values must survive. A nondecreasing collapse of [1,0] to [0,0] changes the leftmost witness, disproving the weaker premise.

Oracle: order_embedding_and_invalid_width_preserve_observations. Finite observations support this relation; the general statement remains source-derived.

## simultaneous_fiber

Type: reverse_constraint_relation. Domain: Finite lawful ordered value arrays, successful allocation, nonpanicking operations; fixed lengths, ranges and transformations as stated.

Fix n,w with 1<=w<=n and candidate J of length n-w+1. Its preimage is the intersection over starts s of constraints s<=J[s]<s+w, A[J[s]]<=A[k] throughout the window and strict inequality for k<J[s]. All constraints concern one shared A; independent per-window witnesses cannot be pasted without consistency. This relation retains every compatible A and does not reconstruct unique magnitudes.

Oracle: simultaneous_output_fibers_and_width_one_information_loss. Finite observations support this relation; the general statement remains source-derived.

## singleton_information_loss

Type: information_loss_relation. Domain: Finite lawful ordered value arrays, successful allocation, nonpanicking operations; fixed lengths, ranges and transformations as stated.

For w=1 and fixed n, J=[0,...,n-1] for every value vector. Thus the index observation has the entire lawful value-vector domain as preimage; no value recovery is possible. For n=0 width one is invalid and the output is empty. Minimum values supplied separately would permit reconstruction.

Oracle: simultaneous_output_fibers_and_width_one_information_loss. Finite observations support this relation; the general statement remains source-derived.

## evidence

### Evidence

All 9,841 ternary arrays of lengths 0..8, every width 0..n+1 plus usize::MAX; independent per-window scans compare exact indices. Increasing, decreasing, equal, expired and extreme-value cases and shift metamorphism included. Compiled mutant changing suffix comparison > to >= was rejected by expiration_and_equal_values_preserve_leftmost_ties (exit 101): equal array width 2 produced [1,2,3] instead of [0,1,2].

Four tests passed in debug and four in release. Strict crate Clippy with all targets and -D warnings passed. Mutants compiled successfully, failed assertions, and were restored before final checks. These are finite observations, not a general proof; costs are symbolic and timings are not performance claims.

## History

### 2026-10-30 · descriptant

2026-10-30 prospective model: Valid widths: O(n) comparisons, O(w) auxiliary indices, O(n-w+1) output indices. Invalid widths O(1). No clones; comparison costs are user-defined; allocation failure outside model. Correctness predicts exact leftmost indices, not merely equal minimum values. Measurements confirmed declared finite cases; no timing gain measured. Deque indices strictly increase, are unexpired, and their values are nondecreasing. A removed suffix index has a strictly smaller, later dominator that expires no sooner. Equal values remain, so the front is always the leftmost minimum. Each input index is pushed once and removed at most once.

#### Deep relation epoch — 2026-10-30

Prediction: decomposition and strict order transport preserve leftmost witnesses; reconstruction depends on which values are observed. Strengthen the models without changing production code or claiming a cost improvement. New correspondence tests distinguish valid relations from non-strict order collapse, index-only recovery and missing locality premises. Earlier evidence remains historical. Current source-bound checks and compiled controls are recorded in model.md#evidence. Symbolic costs are unchanged; no timing or approximate error profile is introduced. Supported executable approximation caps remain 1%, 2%, 5%, 10% if a future profile is supplied.

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

Earlier epoch source SHA-256: b47de6b09f42ba6d6a4a36156568c030eb2452205d06eb2ffc1f437175f8d829.

#### Deep relation epoch — 2026-10-30

Current bound source SHA-256: b47de6b09f42ba6d6a4a36156568c030eb2452205d06eb2ffc1f437175f8d829.

Three added tests exercise the real production functions. Seven tests per algorithm pass in debug and release, with repeated deterministic outcome sets. Strict Clippy and formatting pass. New syntax projections are regenerated twice and byte-compared. Details of compiled mutation rejection follow below. All observations are finite; there is no speed or unrestricted proof claim.

Compiled production control: replacing `values[i] > *value` with `values[i] >= *value` was rejected by `simultaneous_output_fibers_and_width_one_information_loss` with assertion failure and exit 101. The source was restored before final checks. Two debug and two release runs have identical sorted sets of seven passing test names per algorithm. External logs: `$MOVA_DATA/runs/octopus-deep-ranges-2026-10-30`.

New shift, decomposition and contained-witness oracle visits all 1,092 ternary arrays of lengths 1..6, every valid width, slice offset, split point and retained-witness subwindow. The simultaneous reverse-fiber oracle checks 81 length-four ternary arrays against all 64 candidate length-three index vectors: 5,184 membership checks. Invalid widths and non-strict order-collapse counterexamples remain explicit.
