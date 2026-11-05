# algorithm-dijkstra-tests

## descriptant

### dijkstra tests

Inline checks moved into tests.rs with original assertions preserved. Three new tests add wider graph oracles, transformations and boundaries. Source hash and partial syntax projection bind this revision; no semantic AST or expansion is claimed.

#### Mathematical combiner

Combine source/AST, assertion forward relation, independent expected-value oracle, typed family, invariant and bounded cost. Compare actual returns against independent expectations; transformed fixtures add cross-input constraints. Retain every compatible antecedent of observed verdicts. Compiled mutants show a new test can reject a plausible implementation error. Standalone mod.rs and crate checks establish test wiring.

#### Limits

Fixed seeds and bounded exhaustive domains are not statistical coverage or unrestricted proofs. Independent oracle control flow reduces shared implementation errors but is not a formal oracle proof. Earlier BFS unique-zero model correction remains preserved in production descriptant. Raw reports, prospectives and mutants remain outside the repository.

## approximant

### Forward model

Fixed deterministic fixtures map candidate returns to five verdicts by exact assertions. Two original tests plus 36 generated graphs with 4..16 vertices and full-width u64/zero/MAX weights, 24 scaling/splitting graphs with 4..9 vertices, all sources, and a zero-cycle/stale-entry chain with sums beyond two u64::MAX. Seeds 0xd15120261029 and 0xd15220261029; diagnostics include case/source.

Synchronous Bellman-Ford copies the previous distance vector for each of V-1 rounds and relaxes from that previous vector only. Its path-length recurrence shares neither heap ordering nor stale-entry handling with production; sums are u128. An explicit wide-sum fixture adds hand-computed expected values.

#### Cost

All-source reference work O(sum V^2 E). Wider cases: 36 with V<=16. Transform cases: 24 with V<=9,E=2V; splitting produces V+E vertices. Memory O(V+E) outside result copies.

## invariant

### Invariants

Wrapping u64 arithmetic makes fixture schedules deterministic. Existing test assertions are preserved. Generated graph targets stay in bounds. Oracle expected values do not invoke production. Inputs remain unchanged where asserted.

Weights are <1,000,000 before multiplication by seven, so u64 products are safe; finite distances scale sevenfold. Separately replace each edge with two segments through its own fresh vertex, weights floor(w/2) and w-floor(w/2). Restrict results to original vertices and compare against the original independent oracle.

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
Operator/target: Synchronous Bellman-Ford copies the previous distance vector for each of V-1 rounds and relaxes from that previous vector only. Its path-length recurrence shares neither heap ordering nor stale-entry handling with production; sums are u128. An explicit wide-sum fixture adds hand-computed expected values. Fix the generated graph/source and require candidate result equality with this independently specified result.
Approximation: exact bounded assertion specification; partial source-derived Rust correspondence.
Assumptions: trusted compiler/runner, successful allocation, fixed seeds and standard integer semantics.
Evidence: five targeted tests passed twice; normalized logs agree; filtered new-test mutation in model.md#tests fails. No inverse executor or unrestricted proof.
Status: partial_source_derived_model.

## transformations

Domain: deterministic terminating candidate returns on the fixed fixture schedule.
Observation scope: five individual test verdicts with first failing assertion.
Operator/target: Weights are <1,000,000 before multiplication by seven, so u64 products are safe; finite distances scale sevenfold. Separately replace each edge with two segments through its own fresh vertex, weights floor(w/2) and w-floor(w/2). Restrict results to original vertices and compare against the original independent oracle. Retain all graph pairs satisfying these transformations and bounds; do not infer a unique original from an observed result.
Approximation: exact bounded assertion specification; partial source-derived Rust correspondence.
Assumptions: trusted compiler/runner, successful allocation, fixed seeds and standard integer semantics.
Evidence: five targeted tests passed twice; normalized logs agree; filtered new-test mutation in model.md#tests fails. No inverse executor or unrestricted proof.
Status: partial_source_derived_model.

## bounded-coverage

Domain: deterministic terminating candidate returns on the fixed fixture schedule.
Observation scope: five individual test verdicts with first failing assertion.
Operator/target: Two original tests plus 36 generated graphs with 4..16 vertices and full-width u64/zero/MAX weights, 24 scaling/splitting graphs with 4..9 vertices, all sources, and a zero-cycle/stale-entry chain with sums beyond two u64::MAX. Seeds 0xd15120261029 and 0xd15220261029; diagnostics include case/source. All-source reference work O(sum V^2 E). Wider cases: 36 with V<=16. Transform cases: 24 with V<=9,E=2V; splitting produces V+E vertices. Memory O(V+E) outside result copies. Fix size ceilings and schedule; compatible executions have these bounded queries. This is not a wall-clock performance claim.
Approximation: exact bounded assertion specification; partial source-derived Rust correspondence.
Assumptions: trusted compiler/runner, successful allocation, fixed seeds and standard integer semantics.
Evidence: five targeted tests passed twice; normalized logs agree; filtered new-test mutation in model.md#tests fails. No inverse executor or unrestricted proof.
Status: partial_source_derived_model.

## tests

### Checks

Two original tests plus 36 generated graphs with 4..16 vertices and full-width u64/zero/MAX weights, 24 scaling/splitting graphs with 4..9 vertices, all sources, and a zero-cycle/stale-entry chain with sums beyond two u64::MAX. Seeds 0xd15120261029 and 0xd15220261029; diagnostics include case/source.

Compiled new-test negative control: u128::from(weight) changed to u128::from(weight as u32); only new wider_seeded_full_width_weights_match_bellman_ford fails exit 101. Exactly one filtered NEW test failed; no timeout is counted as rejection. External Rust probe confirms production prefix bytes match base.

## History

### 2026-10-29 · descriptant

#### Epoch 2026-10-29

Prospective prediction: preserve six old graph tests, add nine, reject three finite compiled mutants. Production prefixes before cfg(test) match base 600f1d961 byte-for-byte. Five tests per graph pass twice with identical normalized logs, excluding elapsed time. Isolated crate:39 tests pass, strict Clippy and formatting pass. u128::from(weight) changed to u128::from(weight as u32); only new wider_seeded_full_width_weights_match_bellman_ford fails exit 101.

### 2026-10-29 · tests

2026-10-29: rustc --edition 2024 --test algorithms/src/dijkstra/mod.rs; run --test-threads=1. Five tests passed twice, normalized logs identical excluding time. Cargo test -p mova-algorithms --lib:39 passed in isolated branch. Cargo clippy -p mova-algorithms --all-targets -- -D warnings and rustfmt --check passed.

### Undated · behavior-complement

Type: behavior_complement. Subject/revision: tests.rs hash c401390169699bcb073b1ab96655c66dc9c5f4eb6608d7b9f7031f16206e869f.

### Undated · reverse-reconstruction

Type: reverse_reconstruction. Subject/revision: tests.rs hash c401390169699bcb073b1ab96655c66dc9c5f4eb6608d7b9f7031f16206e869f.

### Undated · oracle-independence

Type: independent_reference_relation. Subject/revision: tests.rs hash c401390169699bcb073b1ab96655c66dc9c5f4eb6608d7b9f7031f16206e869f.

### Undated · transformations

Type: metamorphic_relation. Subject/revision: tests.rs hash c401390169699bcb073b1ab96655c66dc9c5f4eb6608d7b9f7031f16206e869f.

### Undated · bounded-coverage

Type: coverage_resource_preimage. Subject/revision: tests.rs hash c401390169699bcb073b1ab96655c66dc9c5f4eb6608d7b9f7031f16206e869f.
