# algorithm-topological-sort

## descriptant

### topological_sort groupant

Original deterministic graph algorithm, with immutable adjacency input, explicit errors and bounded independent oracle tests. Source stays adjacent and authoritative; manifest hash and partial syntax projection bind the revision. No macro expansion or semantic AST is claimed.

#### Mathematical combiner

Take source/AST, forward relation R, typed family, invariant, cost and measurements as inputs. Restrict R by the invariant and compare resulting outputs against the independently implemented finite oracle. The complement uses the explicit one-call universe; the converse retains all compatible antecedents. Specialized models fix their own observations and interventions. Disagreement revises source or assumptions; artifact presence is not semantic proof.

#### Limits

External run artifacts preserve prospective prediction, tests and mutants. No theorem prover or general inverse executor was used. Parent crate wiring requires separate integration checks.

## approximant

### Forward approximant

Validate all edges in adjacency order; malformed graphs return first InvalidEdge. Valid graphs with a cycle return Cycle. Otherwise return the lexicographically least permutation p in which every edge tail precedes its head; empty graph returns [].

#### Cost

O(E+V log(V+1)) time and O(V) auxiliary space. E counts edge multiplicity; validation includes unreachable components. Successful allocation and materialized inputs are assumed. No throughput benchmark is claimed.

## invariant

### Invariant

Indegree equals incoming edge multiplicity from unprocessed vertices. Ready contains precisely unprocessed zero-indegree vertices. Pop chooses its minimum vertex. Each emitted prefix extends the lexicographically least valid ordering.

Inputs remain unchanged. Independent finite oracles support the specification; arbitrary-size correspondence is unproved.

## catalog

# Typed obernant family

## behavior-complement

Type: behavior_complement. Subject/revision: mod.rs at manifest source hash.
Domain: all finite directed adjacency lists with duplicates and self loops; outputs are tagged errors or vertex permutations.
Observation scope: one complete function return; timing and traces excluded.
Operator and target: In U=domain inputs crossed with all declared return values for one call, C=U minus R where R is the forward specification. This excludes every wrong successful vector/order, incorrect error and wrong first-failure precedence. Exact for the specification; partial source-derived absent-behavior approximant for Rust. Allocation failure, panic and elapsed time are outside U.
Approximation semantics: exact specification relation under assumptions; partial source-derived Rust correspondence.
Assumptions: finite immutable allocated graphs, successful allocations, standard Rust collections and integers, <=64-bit usize platform.
Evidence: independent lexicographic permutation enumeration on all 531 graphs with 0..=3 vertices; boundary fixtures. Projections of full results follow from the oracle check; separate inverse enumeration is not implemented.
Status: partial_source_derived_model. Unsampled correspondence remains unknown.

## reverse-reconstruction

Type: reverse_reconstruction. Subject/revision: mod.rs at manifest source hash.
Domain: all finite directed adjacency lists with duplicates and self loops; outputs are tagged errors or vertex permutations.
Observation scope: one complete function return; timing and traces excluded.
Operator and target: For every return y, Pre(y)={G | R(G,y)} retains ALL compatible graphs. Successful fibers contain all DAGs with y as lexicographically first valid ordering, including compatible parallel-edge multiplicities. Error fibers contain all graphs satisfying the exact invalid-edge or cycle rule. No source argument, unique recovery or implemented inverse executor is assumed.
Approximation semantics: exact specification relation under assumptions; partial source-derived Rust correspondence.
Assumptions: finite immutable allocated graphs, successful allocations, standard Rust collections and integers, <=64-bit usize platform.
Evidence: independent lexicographic permutation enumeration on all 531 graphs with 0..=3 vertices; boundary fixtures. Projections of full results follow from the oracle check; separate inverse enumeration is not implemented.
Status: partial_source_derived_model. Unsampled correspondence remains unknown.

## order-fiber

Type: ordering_preimage. Subject/revision: mod.rs at manifest source hash.
Domain: all finite directed adjacency lists with duplicates and self loops; outputs are tagged errors or vertex permutations.
Observation scope: one complete function return; timing and traces excluded.
Operator and target: For fixed permutation p, retain all DAGs whose edges point forward in p and for which each step p[i] is the least vertex in the remaining suffix having no incoming edges from that suffix. Forward edges alone do not characterize lexicographically selected orders. Oracle: brute-force first valid permutation.
Approximation semantics: exact specification relation under assumptions; partial source-derived Rust correspondence.
Assumptions: finite immutable allocated graphs, successful allocations, standard Rust collections and integers, <=64-bit usize platform.
Evidence: independent lexicographic permutation enumeration on all 531 graphs with 0..=3 vertices; boundary fixtures. Projections of full results follow from the oracle check; separate inverse enumeration is not implemented.
Status: partial_source_derived_model. Unsampled correspondence remains unknown.

## cycle-fiber

Type: failure_preimage. Subject/revision: mod.rs at manifest source hash.
Domain: all finite directed adjacency lists with duplicates and self loops; outputs are tagged errors or vertex permutations.
Observation scope: one complete function return; timing and traces excluded.
Operator and target: Pre(Cycle) contains exactly valid finite graphs with a directed cycle, including self loops; invalid-edge graphs are excluded even when also cyclic. Oracle: exhaustive absence of valid permutations and explicit invalid-plus-cycle fixture.
Approximation semantics: exact specification relation under assumptions; partial source-derived Rust correspondence.
Assumptions: finite immutable allocated graphs, successful allocations, standard Rust collections and integers, <=64-bit usize platform.
Evidence: independent lexicographic permutation enumeration on all 531 graphs with 0..=3 vertices; boundary fixtures. Projections of full results follow from the oracle check; separate inverse enumeration is not implemented.
Status: partial_source_derived_model. Unsampled correspondence remains unknown.

## parallel-edge-quotient

Type: multiplicity_equivalence. Subject/revision: mod.rs at manifest source hash.
Domain: all finite directed adjacency lists with duplicates and self loops; outputs are tagged errors or vertex permutations.
Observation scope: one complete function return; timing and traces excluded.
Operator and target: Hold vertices and existence of each edge fixed; any positive multiplicity gives identical ordering or Cycle. Increment/decrement counts must use the same multiplicity. Malformed targets are excluded from the quotient. Oracle: duplicated-edge boundary fixture.
Approximation semantics: exact specification relation under assumptions; partial source-derived Rust correspondence.
Assumptions: finite immutable allocated graphs, successful allocations, standard Rust collections and integers, <=64-bit usize platform.
Evidence: independent lexicographic permutation enumeration on all 531 graphs with 0..=3 vertices; boundary fixtures. Projections of full results follow from the oracle check; separate inverse enumeration is not implemented.
Status: partial_source_derived_model. Unsampled correspondence remains unknown.

## expanded-test-evidence

Current source subject is mod.rs; test implementation and its independent source hash are in tests.rs.groupant. Two original tests plus 32 generated DAGs with 4..7 vertices and shuffled hidden order, a seven-vertex dynamic-ready fixture with duplicate/reordered arcs, and a 96-vertex chain with two isolates and a witnessed back edge. Seed 0x701020261029; generated diagnostics include case. Reverse wrappers removed from ready insertions/extraction, yielding a max-heap; only new duplicate_reordered_edges_preserve_dynamic_lexicographic_choice fails exit 101. The mathematical production relations above are unchanged; the additional observations improve their finite evidence only.

## tests

### Checks

Two original tests plus 32 generated DAGs with 4..7 vertices and shuffled hidden order, a seven-vertex dynamic-ready fixture with duplicate/reordered arcs, and a 96-vertex chain with two isolates and a witnessed back edge. Seed 0x701020261029; generated diagnostics include case.

Enumerate vertex permutations lexicographically; independently require position[u]<position[v] for every edge; select the first valid permutation or Cycle. No heap or indegrees are shared. Hidden order guarantees generated acyclicity but is not used as the expected answer.

Reverse adjacency vectors and triple each arc; exact lexicographic order remains [5,2,3,1,0,4,6], checked against the permutation oracle. Adding 95->17 to an existing chain witnesses a cycle; removing it restores the order. No arbitrary-label-permutation equivariance is claimed.

New compiled source mutation: Reverse wrappers removed from ready insertions/extraction, yielding a max-heap; only new duplicate_reordered_edges_preserve_dynamic_lexicographic_choice fails exit 101. Exactly one filtered new test failed, with exit101, not a timeout. External probe compared the full production prefix before cfg(test) with main 600f1d961 byte-for-byte. Cargo crate checks passed39 tests; strict Clippy and formatting passed. No production algorithm was changed.

## History

### 2026-10-29 · descriptant

#### Modeling epoch 2026-10-29

Prospective prediction was bounded oracle agreement, deterministic output, input preservation and a rejected source mutation. Two tests passed twice for this file. independent lexicographic permutation enumeration on all 531 graphs with 0..=3 vertices. Normalized logs agree excluding elapsed timing. The finite source mutant fails with exit 101. A preliminary Dijkstra greater-than relaxation mutant timed out on cycles; the reported negative control instead adds one to edge weights and yields a finite assertion failure. Retain this distinction between liveness and value evidence. No speed claim is made.

#### Expanded test epoch 2026-10-29

Original inline tests now reside in adjacent tests.rs, registered separately as algorithm-topological-sort-tests. The production prefix is byte-identical to main 600f1d961; only test-module wiring changes. Two original tests plus 32 generated DAGs with 4..7 vertices and shuffled hidden order, a seven-vertex dynamic-ready fixture with duplicate/reordered arcs, and a 96-vertex chain with two isolates and a witnessed back edge. Seed 0x701020261029; generated diagnostics include case. Five tests passed twice with normalized-identical output; strict Clippy and formatting passed. Reverse wrappers removed from ready insertions/extraction, yielding a max-heap; only new duplicate_reordered_edges_preserve_dynamic_lexicographic_choice fails exit 101. This newer evidence extends, rather than erases, the earlier small-domain observations. Tests and production syntax have separate source hashes; production AST does not inline the test module.

### 2026-10-29 · tests

2026-10-29 expanded test epoch: rustc --edition 2024 --test algorithms/src/topological_sort/mod.rs runs adjacent tests.rs. Five tests passed twice with normalized identical results. The original two tests and their assertions remain; earlier source mutation observations are retained in model.md#descriptant.
