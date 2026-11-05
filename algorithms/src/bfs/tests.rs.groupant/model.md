# algorithm-bfs-tests

## descriptant

### bfs tests

Inline checks moved into tests.rs with original assertions preserved. Three new tests add wider graph oracles, transformations and boundaries. Source hash and partial syntax projection bind this revision; no semantic AST or expansion is claimed.

#### Mathematical combiner

Combine source/AST, assertion forward relation, independent expected-value oracle, typed family, invariant and bounded cost. Compare actual returns against independent expectations; transformed fixtures add cross-input constraints. Retain every compatible antecedent of observed verdicts. Compiled mutants show a new test can reject a plausible implementation error. Standalone mod.rs and crate checks establish test wiring.

#### Limits

Fixed seeds and bounded exhaustive domains are not statistical coverage or unrestricted proofs. Independent oracle control flow reduces shared implementation errors but is not a formal oracle proof. Earlier BFS unique-zero model correction remains preserved in production descriptant. Raw reports, prospectives and mutants remain outside the repository.

## approximant

### Forward model

Fixed deterministic fixtures map candidate returns to five verdicts by exact assertions. Two original tests plus 40 generated graphs with 4..20 vertices, 32 duplicate/reordered graphs with 5..16 vertices, all sources, a 2048-vertex chain and a competing-frontier fixture. Seeds 0xb15120261029 and 0xb15220261029; generated diagnostics include case/source.

Floyd-Warshall initializes diagonal zero, arcs one and other entries n+1. Each k applies D[u,v]=min(D[u,v],D[u,k]+D[k,v]); entries >n become None. It shares no FIFO discovery control flow with production. Explicit chain and competing-frontier expected vectors add independent fixtures.

#### Cost

New oracle work O(sum V^3), all-source production O(sum V(V+E)), oracle memory O(V^2). Bounds: 40 graphs <=20 vertices, 32 <=16, one 2048 chain.

## invariant

### Invariants

Wrapping u64 arithmetic makes fixture schedules deterministic. Existing test assertions are preserved. Generated graph targets stay in bounds. Oracle expected values do not invoke production. Inputs remain unchanged where asserted.

Reverse adjacency vectors and repeat each listed edge three times. Minimum edge counts must stay identical for every source; compare both graphs against Floyd, not just each other.

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
Operator/target: Floyd-Warshall initializes diagonal zero, arcs one and other entries n+1. Each k applies D[u,v]=min(D[u,v],D[u,k]+D[k,v]); entries >n become None. It shares no FIFO discovery control flow with production. Explicit chain and competing-frontier expected vectors add independent fixtures. Fix the generated graph/source and require candidate result equality with this independently specified result.
Approximation: exact bounded assertion specification; partial source-derived Rust correspondence.
Assumptions: trusted compiler/runner, successful allocation, fixed seeds and standard integer semantics.
Evidence: five targeted tests passed twice; normalized logs agree; filtered new-test mutation in model.md#tests fails. No inverse executor or unrestricted proof.
Status: partial_source_derived_model.

## transformations

Domain: deterministic terminating candidate returns on the fixed fixture schedule.
Observation scope: five individual test verdicts with first failing assertion.
Operator/target: Reverse adjacency vectors and repeat each listed edge three times. Minimum edge counts must stay identical for every source; compare both graphs against Floyd, not just each other. Retain all graph pairs satisfying these transformations and bounds; do not infer a unique original from an observed result.
Approximation: exact bounded assertion specification; partial source-derived Rust correspondence.
Assumptions: trusted compiler/runner, successful allocation, fixed seeds and standard integer semantics.
Evidence: five targeted tests passed twice; normalized logs agree; filtered new-test mutation in model.md#tests fails. No inverse executor or unrestricted proof.
Status: partial_source_derived_model.

## bounded-coverage

Domain: deterministic terminating candidate returns on the fixed fixture schedule.
Observation scope: five individual test verdicts with first failing assertion.
Operator/target: Two original tests plus 40 generated graphs with 4..20 vertices, 32 duplicate/reordered graphs with 5..16 vertices, all sources, a 2048-vertex chain and a competing-frontier fixture. Seeds 0xb15120261029 and 0xb15220261029; generated diagnostics include case/source. New oracle work O(sum V^3), all-source production O(sum V(V+E)), oracle memory O(V^2). Bounds: 40 graphs <=20 vertices, 32 <=16, one 2048 chain. Fix size ceilings and schedule; compatible executions have these bounded queries. This is not a wall-clock performance claim.
Approximation: exact bounded assertion specification; partial source-derived Rust correspondence.
Assumptions: trusted compiler/runner, successful allocation, fixed seeds and standard integer semantics.
Evidence: five targeted tests passed twice; normalized logs agree; filtered new-test mutation in model.md#tests fails. No inverse executor or unrestricted proof.
Status: partial_source_derived_model.

## tests

### Checks

Two original tests plus 40 generated graphs with 4..20 vertices, 32 duplicate/reordered graphs with 5..16 vertices, all sources, a 2048-vertex chain and a competing-frontier fixture. Seeds 0xb15120261029 and 0xb15220261029; generated diagnostics include case/source.

Compiled new-test negative control: queue.pop_front() changed to queue.pop_back(); only new long_chain_and_competing_frontier_paths fails exit 101. Exactly one filtered NEW test failed; no timeout is counted as rejection. External Rust probe confirms production prefix bytes match base.

## History

### 2026-10-29 · descriptant

#### Epoch 2026-10-29

Prospective prediction: preserve six old graph tests, add nine, reject three finite compiled mutants. Production prefixes before cfg(test) match base 600f1d961 byte-for-byte. Five tests per graph pass twice with identical normalized logs, excluding elapsed time. Isolated crate:39 tests pass, strict Clippy and formatting pass. queue.pop_front() changed to queue.pop_back(); only new long_chain_and_competing_frontier_paths fails exit 101.

### 2026-10-29 · tests

2026-10-29: rustc --edition 2024 --test algorithms/src/bfs/mod.rs; run --test-threads=1. Five tests passed twice, normalized logs identical excluding time. Cargo test -p mova-algorithms --lib:39 passed in isolated branch. Cargo clippy -p mova-algorithms --all-targets -- -D warnings and rustfmt --check passed.

### 2026-11-01 · descriptant

#### Epoch 2026-11-01: error-order observation

The fixed tests admit a candidate that validates malformed edges in reverse
row and adjacency order: their malformed edge fixture has only one invalid
edge. A compiled mutation changing only that validation order passed all five
existing tests. On `[[2], [3]]` with source 0 the actual API reports `(0, 2)`;
the mutant reports `(1, 3)`. Thus a passing verdict does not establish the
production contract's first-invalid-edge ordering. The witness is outside the
fixed fixtures and does not establish general mutant equivalence. Allocation,
compilation and runner failures remain outside the return-table model.

### 2026-11-01 · bounded-coverage

#### Error-order partial exclusion (2026-11-01)

For a valid source, let `I(G)` be invalid edges in row-major adjacency order.
The production error observes `first(I(G))` when nonempty. The existing
malformed-edge assertion observes a singleton `I`, so it cannot distinguish
`first` from `last`. A compiled reverse-order validator passes all five tests,
but on `G=[[2],[3]]`, source 0, returns `(1,3)` instead of `(0,2)`.
This is a partial exclusion/coverage observation, not a converse or a claim
that either validator is equivalent on arbitrary test suites.

### 2026-11-01 · tests

2026-11-01: a compiled validation-order mutation passed all five existing
fixtures; a separate actual API witness on `[[2],[3]]`, source 0, distinguished
its `(1,3)` error from the actual `(0,2)` error. This exposes an untested
ordering boundary; it does not change source or establish general equivalence.

### Undated · behavior-complement

Type: behavior_complement. Subject/revision: tests.rs hash ca43fa9dec420468f0096916b7dae5b5b6012cdf02fa67230153146425f69bcc.

### Undated · reverse-reconstruction

Type: reverse_reconstruction. Subject/revision: tests.rs hash ca43fa9dec420468f0096916b7dae5b5b6012cdf02fa67230153146425f69bcc.

### Undated · oracle-independence

Type: independent_reference_relation. Subject/revision: tests.rs hash ca43fa9dec420468f0096916b7dae5b5b6012cdf02fa67230153146425f69bcc.

### Undated · transformations

Type: metamorphic_relation. Subject/revision: tests.rs hash ca43fa9dec420468f0096916b7dae5b5b6012cdf02fa67230153146425f69bcc.

### Undated · bounded-coverage

Type: coverage_resource_preimage. Subject/revision: tests.rs hash ca43fa9dec420468f0096916b7dae5b5b6012cdf02fa67230153146425f69bcc.
