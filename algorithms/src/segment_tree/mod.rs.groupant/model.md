# algorithm-segment-tree

## descriptant

### segment_tree mod

Original English Rust implementation and independently authored fixtures for Mova; no external solution code or dependencies.

Useful epoch observation: A compact non-power-of-two tree requires index-based ties rather than tree traversal ties.

All 1,093 ternary arrays of lengths 0..6; every point independently replaced by each value -2..2; all start/end pairs through n+1 checked against a linear scan, then restoration checked by complete state equality. Sequential updates and invalid-index atomicity also checked. Compiled mutant changing ancestor loop node > 0 to node > 1 was rejected by exhaustive_point_updates_and_ranges (exit 101), witness [0,-1], range 0..2.

Unknown: exhaustive generic-value semantics, panicking comparison behavior and resource failures are not verified. No executable approximation budget applies to this exact algorithm.

## model

# segment_tree mod model

## Forward

For lawful ordered values A, query_index(s,e) returns the smallest i in [s,e) attaining its minimum if 0<=s<e<=n, otherwise None. query returns its value reference. update(i,x) replaces only A[i] if i<n; otherwise returns InvalidIndex and preserves the complete state.

## Invariant

Leaves n+i contain i; every internal node stores argmin (A[i],i) over its descendant leaves. Non-power-of-two layout need not preserve leaf traversal order: explicit index tie selection makes the commutative minimum reduction sound. Query maintains a disjoint cover of consumed and pending leaves; update recomputes exactly the leaf's ancestors.

## Cost

Construction O(n) comparisons and storage, n lawful clones. Query/update O(log(n+1)) comparisons, O(1) workspace. Invalid queries/updates O(1). Comparisons and clones have user-defined cost. Capacity checked before doubling; allocation/capacity panic outside result model.

## nonbehavior

Type: behavior_complement. Approximation direction: proposed inner bound of absent behavior under the stated premises; finite observations do not prove universal exclusion. In the universe of arrays, operations, outputs and successor states, exclude any minimum outside the requested range, nonminimum or nonleftmost ties, Some for invalid/empty ranges, changed state on invalid update, and changes to other points after valid update.

## compatible_antecedents

Type: reverse_reconstruction. A returned index j admits arrays with s<=j<e, A[j]<=A[k] for all k in the range, and A[j]<A[k] for k<j. None admits exactly invalid or empty ranges. A post-update array admits all antecedents agreeing outside i; the overwritten value is unconstrained. Invalid update admits only an identical prior state.

## Limits

Finite evidence is not proof. Syntax projection is partial; no macro expansion or semantic resolution. Exact reference implementation; no executable lossy approximant or measured error profile is claimed.

## argmin_semilattice

Type: decomposition_relation. Domain: Finite lawful ordered value arrays, successful allocation, nonpanicking operations; fixed lengths, ranges and transformations as stated.

For fixed A define i meet j = argmin of (A[i],i),(A[j],j), with None as identity. The operation is associative, commutative and idempotent. Q(s,e)=Q(s,m) meet Q(m,e) for 0<=s<=m<=e<=n. Empty pieces are None. This explains why compact tree layout and reduction traversal cannot change tie choices.

Oracle: semilattice_decomposition_and_singleton_reconstruction. Finite observations support this relation; the general statement remains source-derived.

## update_locality

Type: frame_relation. Domain: Finite lawful ordered value arrays, successful allocation, nonpanicking operations; fixed lengths, ranges and transformations as stated.

For valid i and range R excluding i, Q(update(A,i,x),R)=Q(A,R), including returned values. At the internal-state level only A[i] and ancestors of leaf n+i may change; unchanged ancestors are allowed. Invalid indices preserve every field. This is not an exception-safety promise for a panicking comparison.

Oracle: disjoint_updates_commute_and_preserve_snapshot_ranges. Finite observations support this relation; the general statement remains source-derived.

## update_commutation

Type: commutation_relation. Domain: Finite lawful ordered value arrays, successful allocation, nonpanicking operations; fixed lengths, ranges and transformations as stated.

For distinct valid i,j and fixed replacement values x,y, U(j,y)(U(i,x)(S))=U(i,x)(U(j,y)(S)) as complete deterministic tree states. When i=j equality requires x=y; otherwise the last write wins, so dropping distinctness is false.

Oracle: disjoint_updates_commute_and_preserve_snapshot_ranges. Finite observations support this relation; the general statement remains source-derived.

## singleton_reconstruction

Type: reverse_reconstruction. Domain: Finite lawful ordered value arrays, successful allocation, nonpanicking operations; fixed lengths, ranges and transformations as stated.

For a fixed length n, observe query(i,i+1) values for every i. The compatible value vector is uniquely A[i]=observed[i]. Rebuilding from A reconstructs the canonical tree, not arbitrary corrupted private state or update history. Observing only singleton indices always yields i and cannot recover values.

Oracle: semilattice_decomposition_and_singleton_reconstruction. Finite observations support this relation; the general statement remains source-derived.

## order_embedding

Type: order_embedding_relation. Domain: Finite lawful ordered value arrays, successful allocation, nonpanicking operations; fixed lengths, ranges and transformations as stated.

For an order embedding f on the observed value set, Q_f(A)(s,e)=Q_A(s,e): strict comparisons and equalities are preserved. Strictly increasing integer affine maps require no overflow. Merely nondecreasing f is insufficient: [1,0] maps to [0,0], changing index 1 to 0.

Oracle: strict_order_embedding_preserves_argmin_but_collapse_does_not. Finite observations support this relation; the general statement remains source-derived.

## snapshot_isolation

Type: snapshot_relation. Domain: Finite lawful ordered value arrays, successful allocation, nonpanicking operations; fixed lengths, ranges and transformations as stated.

For lawful independent value clones, let C=clone(S). Updates on C leave every query and complete state of S unchanged. Equality after restoring a point uses deterministic canonical construction. Clones with shared interior mutable order state violate the premise and are outside this relation.

Oracle: disjoint_updates_commute_and_preserve_snapshot_ranges. Finite observations support this relation; the general statement remains source-derived.

## evidence

### Evidence

All 1,093 ternary arrays of lengths 0..6; every point independently replaced by each value -2..2; all start/end pairs through n+1 checked against a linear scan, then restoration checked by complete state equality. Sequential updates and invalid-index atomicity also checked. Compiled mutant changing ancestor loop node > 0 to node > 1 was rejected by exhaustive_point_updates_and_ranges (exit 101), witness [0,-1], range 0..2.

Four tests passed in debug and four in release. Strict crate Clippy with all targets and -D warnings passed. Mutants compiled successfully, failed assertions, and were restored before final checks. These are finite observations, not a general proof; costs are symbolic and timings are not performance claims.

## History

### 2026-10-30 · descriptant

2026-10-30 prospective model: Construction O(n) comparisons and storage, n lawful clones. Query/update O(log(n+1)) comparisons, O(1) workspace. Invalid queries/updates O(1). Comparisons and clones have user-defined cost. Capacity checked before doubling; allocation/capacity panic outside result model. Correctness predicts exact leftmost indices, not merely equal minimum values. Measurements confirmed declared finite cases; no timing gain measured. Leaves n+i contain i; every internal node stores argmin (A[i],i) over its descendant leaves. Non-power-of-two layout need not preserve leaf traversal order: explicit index tie selection makes the commutative minimum reduction sound. Query maintains a disjoint cover of consumed and pending leaves; update recomputes exactly the leaf's ancestors.

#### Deep relation epoch — 2026-10-30

Prediction: decomposition and strict order transport preserve leftmost witnesses; reconstruction depends on which values are observed. Strengthen the models without changing production code or claiming a cost improvement. New correspondence tests distinguish valid relations from non-strict order collapse, index-only recovery and missing locality premises. Earlier evidence remains historical. Current source-bound checks and compiled controls are recorded in model.md#evidence. Symbolic costs are unchanged; no timing or approximate error profile is introduced. Supported executable approximation caps remain 1%, 2%, 5%, 10% if a future profile is supplied.

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

Earlier epoch source SHA-256: 7017145934520b83a8792e0885238cc7ee6a5f63c013f5f34717d87363f63ac1.

#### Deep relation epoch — 2026-10-30

Current bound source SHA-256: 7017145934520b83a8792e0885238cc7ee6a5f63c013f5f34717d87363f63ac1.

Three added tests exercise the real production functions. Seven tests per algorithm pass in debug and release, with repeated deterministic outcome sets. Strict Clippy and formatting pass. New syntax projections are regenerated twice and byte-compared. Details of compiled mutation rejection follow below. All observations are finite; there is no speed or unrestricted proof claim.

Compiled production control: replacing `std::cmp::Ordering::Equal => a.min(b)` with `std::cmp::Ordering::Equal => a.max(b)` was rejected by `semilattice_decomposition_and_singleton_reconstruction` with assertion failure and exit 101. The source was restored before final checks. Two debug and two release runs have identical sorted sets of seven passing test names per algorithm. External logs: `$MOVA_DATA/runs/octopus-deep-ranges-2026-10-30`.

Compiled production control: replacing `std::cmp::Ordering::Equal => a.min(b)` with `std::cmp::Ordering::Equal => a.max(b)` was rejected by `semilattice_decomposition_and_singleton_reconstruction` with assertion failure and exit 101. The source was restored before final checks. Two debug and two release runs have identical sorted sets of seven passing test names per algorithm. External logs: `$MOVA_DATA/runs/octopus-deep-ranges-2026-10-30`.

New decomposition and singleton-value oracle visits all 1,092 ternary arrays of lengths 1..6 and all valid split points, including empty split pieces. Update locality, commutation, same-index noncommutation, invalid-index preservation and snapshot isolation use every ordered pair of seven indices. The order-embedding test checks every valid range of a fixed seven-value array and rejects a non-strict collapse.
