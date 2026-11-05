# algorithm-hopcroft-karp-tests

## descriptant

### hopcroft_karp tests.rs

Generate bounded cases and fixtures, independently evaluate reference relations, verify witnesses and assert equality. Five tests: all topologies with 0..4 left and 0..3 right vertices against independent partial-injection enumeration. Membership/uniqueness, secondary existing matcher cardinality and exact repeats; reroute, duplicates/permutations, 20,000-left alternating path, disjoint additivity, ambiguity, limits/first errors.

Composition: Same coordinates as bipartite_matching; compare cardinality, not optimum mate identity.

Two release runs, seven alternating-order batches: old/new ratios 7.32/7.32 at 256 independent pairs (100 calls/batch), 69.15/70.67 at 4096 pairs (10 calls/batch), 1.35/1.38 for dense K64,64 (30 calls/batch), 4.78/4.73 for 1024-left reroute chain (20 calls/batch). One pair loses: 0.581/0.575, 74/73 ns versus 43/42 ns (3000 calls/batch). Allocation included, construction excluded; no universal speedup.

Prospective batching-cost prediction saved before implementation/timing. Wide independent paths support it; tiny cases expose overhead. Exact operations remain exact; no useful percentage-error surrogate metric identified. Partial syntax leaves macros opaque and does not resolve semantics.

## approximant

### Forward and cost

Finite inputs and harness outcomes for hopcroft_karp. Left adjacency rows and right count. L+R<=1,000,000 and E<=1,000,000. Return one optional right mate per left.

Generate bounded cases and fixtures, independently evaluate reference relations, verify witnesses and assert equality. Five tests: all topologies with 0..4 left and 0..3 right vertices against independent partial-injection enumeration. Membership/uniqueness, secondary existing matcher cardinality and exact repeats; reroute, duplicates/permutations, 20,000-left alternating path, disjoint additivity, ambiguity, limits/first errors.

Bounded exponential enumeration; oracle cost differs from production cost. Five tests: all topologies with 0..4 left and 0..3 right vertices against independent partial-injection enumeration. Membership/uniqueness, secondary existing matcher cardinality and exact repeats; reroute, duplicates/permutations, 20,000-left alternating path, disjoint additivity, ambiguity, limits/first errors.

## invariant

### Invariant

Each finite case supplies separately justified assertions. Passing totals do not cover omitted inputs, compiler defects or allocation behavior.

## catalog

# Typed model family

## behavior-complement

Domain: Finite inputs and harness outcomes for hopcroft_karp. Left adjacency rows and right count. L+R<=1,000,000 and E<=1,000,000. Return one optional right mate per left.
Observation: complete public return or named harness result, restricted to the explicitly named projection.
Operator: U is finite fixtures crossed with pass/fail observations. Complement includes passing traces with violated assertions and failing traces with every assertion true. Exact specification target with only partial compiled correspondence. Each finite case supplies separately justified assertions. Passing totals do not cover omitted inputs, compiler defects or allocation behavior.
Approximation: exact specified relation, partial source correspondence; no numerical error budget.
Assumptions: standard Rust semantics, successful allocation, trusted compiler/harness and declared coordinate identity.
Evidence: Five tests: all topologies with 0..4 left and 0..3 right vertices against independent partial-injection enumeration. Membership/uniqueness, secondary existing matcher cardinality and exact repeats; reroute, duplicates/permutations, 20,000-left alternating path, disjoint additivity, ambiguity, limits/first errors. See model.md#tests for compiled controls.
Status: partial_source_derived_model; no semantic AST or general proof.

## reverse-reconstruction

Domain: Finite inputs and harness outcomes for hopcroft_karp. Left adjacency rows and right count. L+R<=1,000,000 and E<=1,000,000. Return one optional right mate per left.
Observation: complete public return or named harness result, restricted to the explicitly named projection.
Operator: Pre(pass) keeps all finite observation tuples satisfying assertions, not one recoverable graph/trace. Pre(fail) keeps all tuples violating any assertion. Failure lines lose other fixture details. No unique inverse or complete reconstruction executor.
Approximation: exact specified relation, partial source correspondence; no numerical error budget.
Assumptions: standard Rust semantics, successful allocation, trusted compiler/harness and declared coordinate identity.
Evidence: Five tests: all topologies with 0..4 left and 0..3 right vertices against independent partial-injection enumeration. Membership/uniqueness, secondary existing matcher cardinality and exact repeats; reroute, duplicates/permutations, 20,000-left alternating path, disjoint additivity, ambiguity, limits/first errors. See model.md#tests for compiled controls.
Status: partial_source_derived_model; no semantic AST or general proof.

## independent-oracle

Domain: Finite inputs and harness outcomes for hopcroft_karp. Left adjacency rows and right count. L+R<=1,000,000 and E<=1,000,000. Return one optional right mate per left.
Observation: complete public return or named harness result, restricted to the explicitly named projection.
Operator: Fix graph coordinates; independently enumerate the objective/walk relation on those same coordinates. Library agreement is only secondary. Five tests: all topologies with 0..4 left and 0..3 right vertices against independent partial-injection enumeration. Membership/uniqueness, secondary existing matcher cardinality and exact repeats; reroute, duplicates/permutations, 20,000-left alternating path, disjoint additivity, ambiguity, limits/first errors.
Approximation: exact specified relation, partial source correspondence; no numerical error budget.
Assumptions: standard Rust semantics, successful allocation, trusted compiler/harness and declared coordinate identity.
Evidence: Five tests: all topologies with 0..4 left and 0..3 right vertices against independent partial-injection enumeration. Membership/uniqueness, secondary existing matcher cardinality and exact repeats; reroute, duplicates/permutations, 20,000-left alternating path, disjoint additivity, ambiguity, limits/first errors. See model.md#tests for compiled controls.
Status: partial_source_derived_model; no semantic AST or general proof.

## witness-observation

Domain: Finite inputs and harness outcomes for hopcroft_karp. Left adjacency rows and right count. L+R<=1,000,000 and E<=1,000,000. Return one optional right mate per left.
Observation: complete public return or named harness result, restricted to the explicitly named projection.
Operator: Check actual-return membership/conservation/canonical assertions. Objective-only equality loses witness identity; retain explicit witness constraints. Each finite case supplies separately justified assertions. Passing totals do not cover omitted inputs, compiler defects or allocation behavior.
Approximation: exact specified relation, partial source correspondence; no numerical error budget.
Assumptions: standard Rust semantics, successful allocation, trusted compiler/harness and declared coordinate identity.
Evidence: Five tests: all topologies with 0..4 left and 0..3 right vertices against independent partial-injection enumeration. Membership/uniqueness, secondary existing matcher cardinality and exact repeats; reroute, duplicates/permutations, 20,000-left alternating path, disjoint additivity, ambiguity, limits/first errors. See model.md#tests for compiled controls.
Status: partial_source_derived_model; no semantic AST or general proof.

## compiled-negative-control

Domain: Finite inputs and harness outcomes for hopcroft_karp. Left adjacency rows and right count. L+R<=1,000,000 and E<=1,000,000. Return one optional right mate per left.
Observation: complete public return or named harness result, restricted to the explicitly named projection.
Operator: Change the independent oracle while retaining compilability, require named runtime assertion failure, restore exact bytes and rerun positives. Only sensitivity to this chosen defect is established.
Approximation: exact specified relation, partial source correspondence; no numerical error budget.
Assumptions: standard Rust semantics, successful allocation, trusted compiler/harness and declared coordinate identity.
Evidence: Five tests: all topologies with 0..4 left and 0..3 right vertices against independent partial-injection enumeration. Membership/uniqueness, secondary existing matcher cardinality and exact repeats; reroute, duplicates/permutations, 20,000-left alternating path, disjoint additivity, ambiguity, limits/first errors. See model.md#tests for compiled controls.
Status: partial_source_derived_model; no semantic AST or general proof.

## tests

### Finite evidence

Five tests: all topologies with 0..4 left and 0..3 right vertices against independent partial-injection enumeration. Membership/uniqueness, secondary existing matcher cardinality and exact repeats; reroute, duplicates/permutations, 20,000-left alternating path, disjoint additivity, ambiguity, limits/first errors.

Two release runs, seven alternating-order batches: old/new ratios 7.32/7.32 at 256 independent pairs (100 calls/batch), 69.15/70.67 at 4096 pairs (10 calls/batch), 1.35/1.38 for dense K64,64 (30 calls/batch), 4.78/4.73 for 1024-left reroute chain (20 calls/batch). One pair loses: 0.581/0.575, 74/73 ns versus 43/42 ns (3000 calls/batch). Allocation included, construction excluded; no universal speedup.

Raw logs external. No live parser/teacher/GPU/course/held-out runs.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: original code/fixtures; scientific attribution in mod.rs. Bounded exponential enumeration; oracle cost differs from production cost. Five tests: all topologies with 0..4 left and 0..3 right vertices against independent partial-injection enumeration. Membership/uniqueness, secondary existing matcher cardinality and exact repeats; reroute, duplicates/permutations, 20,000-left alternating path, disjoint additivity, ambiguity, limits/first errors.

### 2026-10-30 · tests

2026-10-30: 356 library unit tests pass twice debug and twice release; identical normalized names/results SHA-256 56550bbf2b2f3d4999f9d5f3ea0230762f4ef2bb54a8a850d3687daf0129943c. This file contributes five tests or their production subject. Strict algorithm all-target Clippy and narrow rustfmt pass.

Compiled control: Empty suffix oracle returns one; exhaustive_partial_injections rejects. Log SHA-256 bce3e06e962a14c1c59e157d9d7118ed97cb40f0f6dbc4fb9a4a72102fe1ac87. Named assertion failure, not compile failure. Six mutants restored byte-for-byte before repeated positives.

Timing logs SHA-256 b3fca25c0729a0edecefdf1c4914f973f761be705db618171b159f4ac96ad7c3 and 9e8b9c417dfc0dfda48f1446825054bfca985f2aa96dea370843c1184e7b020d; Rust harness SHA-256 f78acc0796bf87f2e32ae559c46519badb9b3219b1ed449a56ca5362cb166799; prospective derivation SHA-256 41f9013788fa8bdce512d82f7922287ed2bd9c87bd84dbfca26180aa4d34a03d.

### Undated · behavior-complement

Type: behavior_complement. Subject/revision: algorithms/src/hopcroft_karp/tests.rs, SHA-256 8dbcb5199b475c3f57425aa0524c79388e20461c99c4fc8ad1771c0c021e5630.

### Undated · reverse-reconstruction

Type: reverse_reconstruction. Subject/revision: algorithms/src/hopcroft_karp/tests.rs, SHA-256 8dbcb5199b475c3f57425aa0524c79388e20461c99c4fc8ad1771c0c021e5630.

### Undated · independent-oracle

Type: finite_reference_relation. Subject/revision: algorithms/src/hopcroft_karp/tests.rs, SHA-256 8dbcb5199b475c3f57425aa0524c79388e20461c99c4fc8ad1771c0c021e5630.

### Undated · witness-observation

Type: certificate_projection. Subject/revision: algorithms/src/hopcroft_karp/tests.rs, SHA-256 8dbcb5199b475c3f57425aa0524c79388e20461c99c4fc8ad1771c0c021e5630.

### Undated · compiled-negative-control

Type: counterfactual_oracle_intervention. Subject/revision: algorithms/src/hopcroft_karp/tests.rs, SHA-256 8dbcb5199b475c3f57425aa0524c79388e20461c99c4fc8ad1771c0c021e5630.
