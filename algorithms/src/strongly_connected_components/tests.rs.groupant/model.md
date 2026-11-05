# algorithm-strongly-connected-components-tests

## descriptant

### strongly_connected_components test groupant

Four original tests with independent bounded reference logic and explicit transformed/boundary cases.  Source remains adjacent; manifest hash binds this revision and syntax.json is a partial syn projection without macro expansion or semantic resolution.

#### Mathematical combiner

Inputs are source/AST, forward relation, complement, complete preimage relation, specialized observations, invariant, cost and checks. Intersect candidate behavior with invariant constraints and compare exact outputs to independent finite oracles. Transformations add cross-input relations. A compiled mutation must be rejected by one selected assertion test. Registration and source identity do not prove model semantics. Parent library wiring is reviewed separately.

#### Retained observations and limits

Four tests: 531 exhaustive simple directed graphs with 0..3 vertices; 64 seeded graphs with 4..16 vertices and transposed duplicate/reversed adjacency (seed 0x5cc020261029); validation/canonical fixtures; 10,000-vertex chain and closing cycle. Exhaustive and seeded closure O(sum V^3), matrix memory O(V^2); generated V<=16, 64 cases. Long chain uses 10,000 vertices and production explicit stacks, not recursive oracle. The long-chain case checks explicit-stack behavior at10,000 vertices; arbitrary allocation capacity remains unproved. No teacher call, timing gain or full mathematical verification is claimed. Prospective notes, logs and compiled mutants remain outside the repository.

## approximant

### Forward approximant

The fixed four-test schedule maps candidate returns to exact assertion verdicts. Four tests: 531 exhaustive simple directed graphs with 0..3 vertices; 64 seeded graphs with 4..16 vertices and transposed duplicate/reversed adjacency (seed 0x5cc020261029); validation/canonical fixtures; 10,000-vertex chain and closing cycle. Independent Boolean transitive closure computes all-pairs reachability; group u,v iff both directions hold. It does not use finish ordering, graph transposition or the implementation stacks.

#### Cost

Exhaustive and seeded closure O(sum V^3), matrix memory O(V^2); generated V<=16, 64 cases. Long chain uses 10,000 vertices and production explicit stacks, not recursive oracle.

Finite materialized inputs and successful allocation are assumptions. Source inspection and bounded Rust tests support correspondence; this is not a machine-checked proof.

## invariant

### Invariant

Fixed seeds use explicit wrapping u64 arithmetic; generated valid endpoints stay in range. Expected results are computed without invoking the algorithm under test. Oracle bounds keep exhaustive or recursive enumeration finite. Candidate inputs remain unchanged where asserted.

Transpose every arc, duplicate arcs and reverse adjacency order; the canonical SCC partition is unchanged.

## catalog

# Typed obernant family

## behavior-complement

Domain: deterministic terminating candidate return tables on this fixed finite fixture schedule, with four test verdicts.
Observation scope: four individual test verdicts and the first failing assertion within each test.
Operator/target: Declare U as all deterministic terminating candidate return tables on this fixed fixture schedule crossed with four pass/fail verdicts. R is assertion evaluation. U minus R excludes passing when any executed assertion disagrees with its independent expected value, and excludes failure when all assertions pass. Exact assertion specification; actual Rust correspondence is partial. Compilation failure, timeout and resource failure are outside U.
Approximation: exact specification relation under assumptions; partial source-derived correspondence to Rust.
Assumptions: trusted compiler/runner, successful allocations, standard library/integer semantics, fixed documented domains and <=64-bit usize where relevant.
Evidence: Four tests: 531 exhaustive simple directed graphs with 0..3 vertices; 64 seeded graphs with 4..16 vertices and transposed duplicate/reversed adjacency (seed 0x5cc020261029); validation/canonical fixtures; 10,000-vertex chain and closing cycle. Four tests pass twice with identical normalized logs; Replace second traversal reverse[vertex] with graph[vertex]. Compiled mutant is rejected by exactly all_small_graphs_match_mutual_reachability (one selected failed test, exit101). Specialized clauses explicitly marked unexecuted remain reasoning, not measured evidence.
Status: partial_source_derived_model; no general inverse executor or unrestricted proof.

## reverse-reconstruction

Domain: deterministic terminating candidate return tables on this fixed finite fixture schedule, with four test verdicts.
Observation scope: four individual test verdicts and the first failing assertion within each test.
Operator/target: Pre(y) retains ALL candidate return tables whose assertions produce observed verdicts y. Pass constrains only tested calls, leaving untested inputs arbitrary. Failure preserves every compatible first failing assertion and earlier passing returns. No unique source implementation follows from these observations.
Approximation: exact specification relation under assumptions; partial source-derived correspondence to Rust.
Assumptions: trusted compiler/runner, successful allocations, standard library/integer semantics, fixed documented domains and <=64-bit usize where relevant.
Evidence: Four tests: 531 exhaustive simple directed graphs with 0..3 vertices; 64 seeded graphs with 4..16 vertices and transposed duplicate/reversed adjacency (seed 0x5cc020261029); validation/canonical fixtures; 10,000-vertex chain and closing cycle. Four tests pass twice with identical normalized logs; Replace second traversal reverse[vertex] with graph[vertex]. Compiled mutant is rejected by exactly all_small_graphs_match_mutual_reachability (one selected failed test, exit101). Specialized clauses explicitly marked unexecuted remain reasoning, not measured evidence.
Status: partial_source_derived_model; no general inverse executor or unrestricted proof.

## oracle-independence

Domain: deterministic terminating candidate return tables on this fixed finite fixture schedule, with four test verdicts.
Observation scope: four individual test verdicts and the first failing assertion within each test.
Operator/target: Independent Boolean transitive closure computes all-pairs reachability; group u,v iff both directions hold. It does not use finish ordering, graph transposition or the implementation stacks. Hold fixture inputs fixed; require exact equality with this independently specified reference and explicit boundary assertions. No unrestricted oracle proof is claimed.
Approximation: exact specification relation under assumptions; partial source-derived correspondence to Rust.
Assumptions: trusted compiler/runner, successful allocations, standard library/integer semantics, fixed documented domains and <=64-bit usize where relevant.
Evidence: Four tests: 531 exhaustive simple directed graphs with 0..3 vertices; 64 seeded graphs with 4..16 vertices and transposed duplicate/reversed adjacency (seed 0x5cc020261029); validation/canonical fixtures; 10,000-vertex chain and closing cycle. Four tests pass twice with identical normalized logs; Replace second traversal reverse[vertex] with graph[vertex]. Compiled mutant is rejected by exactly all_small_graphs_match_mutual_reachability (one selected failed test, exit101). Specialized clauses explicitly marked unexecuted remain reasoning, not measured evidence.
Status: partial_source_derived_model; no general inverse executor or unrestricted proof.

## transformations

Domain: deterministic terminating candidate return tables on this fixed finite fixture schedule, with four test verdicts.
Observation scope: four individual test verdicts and the first failing assertion within each test.
Operator/target: Transpose every arc, duplicate arcs and reverse adjacency order; the canonical SCC partition is unchanged. Keep all compatible original/transformed graph pairs under these constraints; do not infer a unique original graph from results.
Approximation: exact specification relation under assumptions; partial source-derived correspondence to Rust.
Assumptions: trusted compiler/runner, successful allocations, standard library/integer semantics, fixed documented domains and <=64-bit usize where relevant.
Evidence: Four tests: 531 exhaustive simple directed graphs with 0..3 vertices; 64 seeded graphs with 4..16 vertices and transposed duplicate/reversed adjacency (seed 0x5cc020261029); validation/canonical fixtures; 10,000-vertex chain and closing cycle. Four tests pass twice with identical normalized logs; Replace second traversal reverse[vertex] with graph[vertex]. Compiled mutant is rejected by exactly all_small_graphs_match_mutual_reachability (one selected failed test, exit101). Specialized clauses explicitly marked unexecuted remain reasoning, not measured evidence.
Status: partial_source_derived_model; no general inverse executor or unrestricted proof.

## coverage-budget

Domain: deterministic terminating candidate return tables on this fixed finite fixture schedule, with four test verdicts.
Observation scope: four individual test verdicts and the first failing assertion within each test.
Operator/target: Four tests: 531 exhaustive simple directed graphs with 0..3 vertices; 64 seeded graphs with 4..16 vertices and transposed duplicate/reversed adjacency (seed 0x5cc020261029); validation/canonical fixtures; 10,000-vertex chain and closing cycle. Exhaustive and seeded closure O(sum V^3), matrix memory O(V^2); generated V<=16, 64 cases. Long chain uses 10,000 vertices and production explicit stacks, not recursive oracle. These counts and bounds define the measured finite domain; compatible runs share the schedule, not a promised wall-clock duration.
Approximation: exact specification relation under assumptions; partial source-derived correspondence to Rust.
Assumptions: trusted compiler/runner, successful allocations, standard library/integer semantics, fixed documented domains and <=64-bit usize where relevant.
Evidence: Four tests: 531 exhaustive simple directed graphs with 0..3 vertices; 64 seeded graphs with 4..16 vertices and transposed duplicate/reversed adjacency (seed 0x5cc020261029); validation/canonical fixtures; 10,000-vertex chain and closing cycle. Four tests pass twice with identical normalized logs; Replace second traversal reverse[vertex] with graph[vertex]. Compiled mutant is rejected by exactly all_small_graphs_match_mutual_reachability (one selected failed test, exit101). Specialized clauses explicitly marked unexecuted remain reasoning, not measured evidence.
Status: partial_source_derived_model; no general inverse executor or unrestricted proof.

## tests

### Checks and measured evidence

Independent oracle: Independent Boolean transitive closure computes all-pairs reachability; group u,v iff both directions hold. It does not use finish ordering, graph transposition or the implementation stacks.

Transformation: Transpose every arc, duplicate arcs and reverse adjacency order; the canonical SCC partition is unchanged.

Negative control: Replace second traversal reverse[vertex] with graph[vertex]. Compiled mutant is rejected by exactly all_small_graphs_match_mutual_reachability (one selected failed test, exit101). Only that exact filtered test ran; the report asserts one failed test and exit101, excluding timeout. Source mutation exists only in external scratch.

Run cargo test -p mova-algorithms strongly_connected_components::tests -- --test-threads=1 after parent export integration. The external probe also compiled standalone modules; Kruskal used a small wrapper exposing the unchanged disjoint-set implementation. Isolated crate total:66 passing tests. Strict cargo clippy --all-targets -- -D warnings and formatting passed. Reports preserve fixed seeds/cases and domain bounds.

## History

### 2026-10-29 · descriptant

#### Modeling epoch 2026-10-29

Prospective prediction was four tests per algorithm, bounded independent oracle agreement, deterministic repeated outcomes and finite compiled mutant rejection. Each module's four tests passed twice; normalized logs agree after omitting elapsed time. The isolated crate with temporary local exports passed66 tests; strict Clippy and rustfmt checks passed. Replace second traversal reverse[vertex] with graph[vertex]. Compiled mutant is rejected by exactly all_small_graphs_match_mutual_reachability (one selected failed test, exit101). No production bug was silently patched; initial lint fixes used is_multiple_of in fixtures only.

### 2026-10-29 · tests

2026-10-29: four targeted tests pass twice with normalized-identical output. Four tests: 531 exhaustive simple directed graphs with 0..3 vertices; 64 seeded graphs with 4..16 vertices and transposed duplicate/reversed adjacency (seed 0x5cc020261029); validation/canonical fixtures; 10,000-vertex chain and closing cycle.

### Undated · behavior-complement

Type: behavior_complement. Subject/revision: tests.rs at sha256 47e691e3b316cafc8a891e81dfa4197ca7461f7691ba4c757a3960c0814a0483.

### Undated · reverse-reconstruction

Type: reverse_reconstruction. Subject/revision: tests.rs at sha256 47e691e3b316cafc8a891e81dfa4197ca7461f7691ba4c757a3960c0814a0483.

### Undated · oracle-independence

Type: independent_reference_relation. Subject/revision: tests.rs at sha256 47e691e3b316cafc8a891e81dfa4197ca7461f7691ba4c757a3960c0814a0483.

### Undated · transformations

Type: metamorphic_relation. Subject/revision: tests.rs at sha256 47e691e3b316cafc8a891e81dfa4197ca7461f7691ba4c757a3960c0814a0483.

### Undated · coverage-budget

Type: bounded_resource_preimage. Subject/revision: tests.rs at sha256 47e691e3b316cafc8a891e81dfa4197ca7461f7691ba4c757a3960c0814a0483.
