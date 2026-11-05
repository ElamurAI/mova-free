# algorithm-topological-sort-tests

## descriptant

### topological_sort tests

Inline checks moved into tests.rs with original assertions preserved. Three new tests add wider graph oracles, transformations and boundaries. Source hash and partial syntax projection bind this revision; no semantic AST or expansion is claimed.

#### Mathematical combiner

Combine source/AST, assertion forward relation, independent expected-value oracle, typed family, invariant and bounded cost. Compare actual returns against independent expectations; transformed fixtures add cross-input constraints. Retain every compatible antecedent of observed verdicts. Compiled mutants show a new test can reject a plausible implementation error. Standalone mod.rs and crate checks establish test wiring.

#### Limits

Fixed seeds and bounded exhaustive domains are not statistical coverage or unrestricted proofs. Independent oracle control flow reduces shared implementation errors but is not a formal oracle proof. Earlier BFS unique-zero model correction remains preserved in production descriptant. Raw reports, prospectives and mutants remain outside the repository.

## approximant

### Forward model

Fixed deterministic fixtures map candidate returns to five verdicts by exact assertions. Two original tests plus 32 generated DAGs with 4..7 vertices and shuffled hidden order, a seven-vertex dynamic-ready fixture with duplicate/reordered arcs, and a 96-vertex chain with two isolates and a witnessed back edge. Seed 0x701020261029; generated diagnostics include case.

Enumerate vertex permutations lexicographically; independently require position[u]<position[v] for every edge; select the first valid permutation or Cycle. No heap or indegrees are shared. Hidden order guarantees generated acyclicity but is not used as the expected answer.

#### Cost

Oracle O(V!*(V+E)) time and O(V!*V) stored labels, bounded at V<=7 over 32 cases plus one transformation fixture; long-cycle graph has 98 vertices. Factorial bounds must stay explicit.

## invariant

### Invariants

Wrapping u64 arithmetic makes fixture schedules deterministic. Existing test assertions are preserved. Generated graph targets stay in bounds. Oracle expected values do not invoke production. Inputs remain unchanged where asserted.

Reverse adjacency vectors and triple each arc; exact lexicographic order remains [5,2,3,1,0,4,6], checked against the permutation oracle. Adding 95->17 to an existing chain witnesses a cycle; removing it restores the order. No arbitrary-label-permutation equivariance is claimed.

## catalog

# Typed test models

## behavior-complement

Domain: deterministic terminating candidate returns on the fixed fixture schedule.
Observation scope: five individual test verdicts with first failing assertion.
Operator/target: Universe U is deterministic terminating candidate return tables on the fixed fixture schedule crossed with five test verdicts. R maps each table to exact assertion verdicts. C=U minus R excludes passing when any assertion disagrees with expected values and failing when all executed assertions pass. This is exact at assertion-specification level, partial for actual runners; compilation failure, timeout and resource failure are outside U.
Approximation: exact bounded assertion specification; partial source-derived Rust correspondence.
Assumptions: trusted compiler/runner, successful allocation, fixed seeds and standard integer semantics.
Evidence: five targeted tests passed twice; normalized logs agree; filtered new-test mutation in model.md#tests fails. No inverse executor or unrestricted proof.
Status: partial_source_derived_model.

## reverse-reconstruction

Domain: deterministic terminating candidate returns on the fixed fixture schedule.
Observation scope: five individual test verdicts with first failing assertion.
Operator/target: Pre(y) retains ALL candidate return tables giving observed verdict vector y. Passing constrains only checked calls; all untested inputs remain free. A failure retains every compatible first failing assertion and earlier passing return. No unique implementation or source reconstruction follows.
Approximation: exact bounded assertion specification; partial source-derived Rust correspondence.
Assumptions: trusted compiler/runner, successful allocation, fixed seeds and standard integer semantics.
Evidence: five targeted tests passed twice; normalized logs agree; filtered new-test mutation in model.md#tests fails. No inverse executor or unrestricted proof.
Status: partial_source_derived_model.

## oracle-independence

Domain: deterministic terminating candidate returns on the fixed fixture schedule.
Observation scope: five individual test verdicts with first failing assertion.
Operator/target: Enumerate vertex permutations lexicographically; independently require position[u]<position[v] for every edge; select the first valid permutation or Cycle. No heap or indegrees are shared. Hidden order guarantees generated acyclicity but is not used as the expected answer. Fix the generated graph and require candidate result equality with this independently specified result.
Approximation: exact bounded assertion specification; partial source-derived Rust correspondence.
Assumptions: trusted compiler/runner, successful allocation, fixed seeds and standard integer semantics.
Evidence: five targeted tests passed twice; normalized logs agree; filtered new-test mutation in model.md#tests fails. No inverse executor or unrestricted proof.
Status: partial_source_derived_model.

## transformations

Domain: deterministic terminating candidate returns on the fixed fixture schedule.
Observation scope: five individual test verdicts with first failing assertion.
Operator/target: Reverse adjacency vectors and triple each arc; exact lexicographic order remains [5,2,3,1,0,4,6], checked against the permutation oracle. Adding 95->17 to an existing chain witnesses a cycle; removing it restores the order. No arbitrary-label-permutation equivariance is claimed. Retain all graph pairs satisfying these transformations and bounds; do not infer a unique original from an observed result.
Approximation: exact bounded assertion specification; partial source-derived Rust correspondence.
Assumptions: trusted compiler/runner, successful allocation, fixed seeds and standard integer semantics.
Evidence: five targeted tests passed twice; normalized logs agree; filtered new-test mutation in model.md#tests fails. No inverse executor or unrestricted proof.
Status: partial_source_derived_model.

## bounded-coverage

Domain: deterministic terminating candidate returns on the fixed fixture schedule.
Observation scope: five individual test verdicts with first failing assertion.
Operator/target: Two original tests plus 32 generated DAGs with 4..7 vertices and shuffled hidden order, a seven-vertex dynamic-ready fixture with duplicate/reordered arcs, and a 96-vertex chain with two isolates and a witnessed back edge. Seed 0x701020261029; generated diagnostics include case. Oracle O(V!*(V+E)) time and O(V!*V) stored labels, bounded at V<=7 over 32 cases plus one transformation fixture; long-cycle graph has 98 vertices. Factorial bounds must stay explicit. Fix size ceilings and schedule; compatible executions have these bounded queries. This is not a wall-clock performance claim.
Approximation: exact bounded assertion specification; partial source-derived Rust correspondence.
Assumptions: trusted compiler/runner, successful allocation, fixed seeds and standard integer semantics.
Evidence: five targeted tests passed twice; normalized logs agree; filtered new-test mutation in model.md#tests fails. No inverse executor or unrestricted proof.
Status: partial_source_derived_model.

## tests

### Checks

Two original tests plus 32 generated DAGs with 4..7 vertices and shuffled hidden order, a seven-vertex dynamic-ready fixture with duplicate/reordered arcs, and a 96-vertex chain with two isolates and a witnessed back edge. Seed 0x701020261029; generated diagnostics include case.

Compiled new-test negative control: Reverse wrappers removed from ready insertions/extraction, yielding a max-heap; only new duplicate_reordered_edges_preserve_dynamic_lexicographic_choice fails exit 101. Exactly one filtered NEW test failed; no timeout is counted as rejection. External Rust probe confirms production prefix bytes match base.

## History

### 2026-10-29 · descriptant

#### Epoch 2026-10-29

Prospective prediction: preserve six old graph tests, add nine, reject three finite compiled mutants. Production prefixes before cfg(test) match base 600f1d961 byte-for-byte. Five tests per graph pass twice with identical normalized logs, excluding elapsed time. Isolated crate:39 tests pass, strict Clippy and formatting pass. Reverse wrappers removed from ready insertions/extraction, yielding a max-heap; only new duplicate_reordered_edges_preserve_dynamic_lexicographic_choice fails exit 101.

### 2026-10-29 · tests

2026-10-29: rustc --edition 2024 --test algorithms/src/topological_sort/mod.rs; run --test-threads=1. Five tests passed twice, normalized logs identical excluding time. Cargo test -p mova-algorithms --lib:39 passed in isolated branch. Cargo clippy -p mova-algorithms --all-targets -- -D warnings and rustfmt --check passed.

### Undated · behavior-complement

Type: behavior_complement. Subject/revision: tests.rs hash e1685f2d533c61becd64f4ecd37064d5cc01c69bffbd431dfd8518c9c6ee8d75.

### Undated · reverse-reconstruction

Type: reverse_reconstruction. Subject/revision: tests.rs hash e1685f2d533c61becd64f4ecd37064d5cc01c69bffbd431dfd8518c9c6ee8d75.

### Undated · oracle-independence

Type: independent_reference_relation. Subject/revision: tests.rs hash e1685f2d533c61becd64f4ecd37064d5cc01c69bffbd431dfd8518c9c6ee8d75.

### Undated · transformations

Type: metamorphic_relation. Subject/revision: tests.rs hash e1685f2d533c61becd64f4ecd37064d5cc01c69bffbd431dfd8518c9c6ee8d75.

### Undated · bounded-coverage

Type: coverage_resource_preimage. Subject/revision: tests.rs hash e1685f2d533c61becd64f4ecd37064d5cc01c69bffbd431dfd8518c9c6ee8d75.
