# algorithm-tarjan-scc-tests

## descriptant

### tarjan_scc tests.rs

Generate bounded cases and fixtures, independently evaluate reference relations, verify witnesses and assert equality. Five tests: 4166 loop-free topologies over 0..4 vertices against independent bounded-walk closure and existing Kosaraju. Active cross-edge/completed-component fixtures, transpose/duplicates/loops/relabeling, precedence, 30,000-vertex chain then cycle.

Composition: Same canonical output as strongly_connected_components; independent mutual-walk oracle.

No Tarjan runtime gain claimed. Avoiding reverse adjacency gives O(V) auxiliary storage; canonical sorting retains O(V log(V+1)) time.

Epoch records preserve iterative lowlink/canonical-order decisions; Tarjan has no runtime gain claim. Exact operations remain exact; no useful percentage-error surrogate metric identified. Partial syntax leaves macros opaque and does not resolve semantics.

## approximant

### Forward and cost

Finite inputs and harness outcomes for tarjan_scc. Directed adjacency rows, V<=1,000,000 and E<=1,000,000. Return sorted nonempty SCC lists ordered by minimum member, including isolates.

Generate bounded cases and fixtures, independently evaluate reference relations, verify witnesses and assert equality. Five tests: 4166 loop-free topologies over 0..4 vertices against independent bounded-walk closure and existing Kosaraju. Active cross-edge/completed-component fixtures, transpose/duplicates/loops/relabeling, precedence, 30,000-vertex chain then cycle.

Bounded exponential enumeration; oracle cost differs from production cost. Five tests: 4166 loop-free topologies over 0..4 vertices against independent bounded-walk closure and existing Kosaraju. Active cross-edge/completed-component fixtures, transpose/duplicates/loops/relabeling, precedence, 30,000-vertex chain then cycle.

## invariant

### Invariant

Each finite case supplies separately justified assertions. Passing totals do not cover omitted inputs, compiler defects or allocation behavior.

## catalog

# Typed model family

## behavior-complement

Domain: Finite inputs and harness outcomes for tarjan_scc. Directed adjacency rows, V<=1,000,000 and E<=1,000,000. Return sorted nonempty SCC lists ordered by minimum member, including isolates.
Observation: complete public return or named harness result, restricted to the explicitly named projection.
Operator: U is finite fixtures crossed with pass/fail observations. Complement includes passing traces with violated assertions and failing traces with every assertion true. Exact specification target with only partial compiled correspondence. Each finite case supplies separately justified assertions. Passing totals do not cover omitted inputs, compiler defects or allocation behavior.
Approximation: exact specified relation, partial source correspondence; no numerical error budget.
Assumptions: standard Rust semantics, successful allocation, trusted compiler/harness and declared coordinate identity.
Evidence: Five tests: 4166 loop-free topologies over 0..4 vertices against independent bounded-walk closure and existing Kosaraju. Active cross-edge/completed-component fixtures, transpose/duplicates/loops/relabeling, precedence, 30,000-vertex chain then cycle. See model.md#tests for compiled controls.
Status: partial_source_derived_model; no semantic AST or general proof.

## reverse-reconstruction

Domain: Finite inputs and harness outcomes for tarjan_scc. Directed adjacency rows, V<=1,000,000 and E<=1,000,000. Return sorted nonempty SCC lists ordered by minimum member, including isolates.
Observation: complete public return or named harness result, restricted to the explicitly named projection.
Operator: Pre(pass) keeps all finite observation tuples satisfying assertions, not one recoverable graph/trace. Pre(fail) keeps all tuples violating any assertion. Failure lines lose other fixture details. No unique inverse or complete reconstruction executor.
Approximation: exact specified relation, partial source correspondence; no numerical error budget.
Assumptions: standard Rust semantics, successful allocation, trusted compiler/harness and declared coordinate identity.
Evidence: Five tests: 4166 loop-free topologies over 0..4 vertices against independent bounded-walk closure and existing Kosaraju. Active cross-edge/completed-component fixtures, transpose/duplicates/loops/relabeling, precedence, 30,000-vertex chain then cycle. See model.md#tests for compiled controls.
Status: partial_source_derived_model; no semantic AST or general proof.

## independent-oracle

Domain: Finite inputs and harness outcomes for tarjan_scc. Directed adjacency rows, V<=1,000,000 and E<=1,000,000. Return sorted nonempty SCC lists ordered by minimum member, including isolates.
Observation: complete public return or named harness result, restricted to the explicitly named projection.
Operator: Fix graph coordinates; independently enumerate the objective/walk relation on those same coordinates. Library agreement is only secondary. Five tests: 4166 loop-free topologies over 0..4 vertices against independent bounded-walk closure and existing Kosaraju. Active cross-edge/completed-component fixtures, transpose/duplicates/loops/relabeling, precedence, 30,000-vertex chain then cycle.
Approximation: exact specified relation, partial source correspondence; no numerical error budget.
Assumptions: standard Rust semantics, successful allocation, trusted compiler/harness and declared coordinate identity.
Evidence: Five tests: 4166 loop-free topologies over 0..4 vertices against independent bounded-walk closure and existing Kosaraju. Active cross-edge/completed-component fixtures, transpose/duplicates/loops/relabeling, precedence, 30,000-vertex chain then cycle. See model.md#tests for compiled controls.
Status: partial_source_derived_model; no semantic AST or general proof.

## witness-observation

Domain: Finite inputs and harness outcomes for tarjan_scc. Directed adjacency rows, V<=1,000,000 and E<=1,000,000. Return sorted nonempty SCC lists ordered by minimum member, including isolates.
Observation: complete public return or named harness result, restricted to the explicitly named projection.
Operator: Check actual-return membership/conservation/canonical assertions. Objective-only equality loses witness identity; retain explicit witness constraints. Each finite case supplies separately justified assertions. Passing totals do not cover omitted inputs, compiler defects or allocation behavior.
Approximation: exact specified relation, partial source correspondence; no numerical error budget.
Assumptions: standard Rust semantics, successful allocation, trusted compiler/harness and declared coordinate identity.
Evidence: Five tests: 4166 loop-free topologies over 0..4 vertices against independent bounded-walk closure and existing Kosaraju. Active cross-edge/completed-component fixtures, transpose/duplicates/loops/relabeling, precedence, 30,000-vertex chain then cycle. See model.md#tests for compiled controls.
Status: partial_source_derived_model; no semantic AST or general proof.

## compiled-negative-control

Domain: Finite inputs and harness outcomes for tarjan_scc. Directed adjacency rows, V<=1,000,000 and E<=1,000,000. Return sorted nonempty SCC lists ordered by minimum member, including isolates.
Observation: complete public return or named harness result, restricted to the explicitly named projection.
Operator: Change the independent oracle while retaining compilability, require named runtime assertion failure, restore exact bytes and rerun positives. Only sensitivity to this chosen defect is established.
Approximation: exact specified relation, partial source correspondence; no numerical error budget.
Assumptions: standard Rust semantics, successful allocation, trusted compiler/harness and declared coordinate identity.
Evidence: Five tests: 4166 loop-free topologies over 0..4 vertices against independent bounded-walk closure and existing Kosaraju. Active cross-edge/completed-component fixtures, transpose/duplicates/loops/relabeling, precedence, 30,000-vertex chain then cycle. See model.md#tests for compiled controls.
Status: partial_source_derived_model; no semantic AST or general proof.

## tests

### Finite evidence

Five tests: 4166 loop-free topologies over 0..4 vertices against independent bounded-walk closure and existing Kosaraju. Active cross-edge/completed-component fixtures, transpose/duplicates/loops/relabeling, precedence, 30,000-vertex chain then cycle.

No Tarjan runtime gain claimed. Avoiding reverse adjacency gives O(V) auxiliary storage; canonical sorting retains O(V log(V+1)) time.

Raw logs external. No live parser/teacher/GPU/course/held-out runs.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: original code/fixtures; scientific attribution in mod.rs. Bounded exponential enumeration; oracle cost differs from production cost. Five tests: 4166 loop-free topologies over 0..4 vertices against independent bounded-walk closure and existing Kosaraju. Active cross-edge/completed-component fixtures, transpose/duplicates/loops/relabeling, precedence, 30,000-vertex chain then cycle.

### 2026-10-30 · tests

2026-10-30: 356 library unit tests pass twice debug and twice release; identical normalized names/results SHA-256 56550bbf2b2f3d4999f9d5f3ea0230762f4ef2bb54a8a850d3687daf0129943c. This file contributes five tests or their production subject. Strict algorithm all-target Clippy and narrow rustfmt pass.

Compiled control: Reverse walk-partition blocks; exhaustive_mutual_reachability rejects. Log SHA-256 79bfb1e26eb3f12ba6557c6aff76cc561da6409be07d31fcc1b441d58f098f3d. Named assertion failure, not compile failure. Six mutants restored byte-for-byte before repeated positives.

Timing logs SHA-256 b3fca25c0729a0edecefdf1c4914f973f761be705db618171b159f4ac96ad7c3 and 9e8b9c417dfc0dfda48f1446825054bfca985f2aa96dea370843c1184e7b020d; Rust harness SHA-256 f78acc0796bf87f2e32ae559c46519badb9b3219b1ed449a56ca5362cb166799; prospective derivation SHA-256 41f9013788fa8bdce512d82f7922287ed2bd9c87bd84dbfca26180aa4d34a03d.

### Undated · behavior-complement

Type: behavior_complement. Subject/revision: algorithms/src/tarjan_scc/tests.rs, SHA-256 339a99c1ca852a341427d33094b10b221b1bae089fa64eb445c2fcc74c513184.

### Undated · reverse-reconstruction

Type: reverse_reconstruction. Subject/revision: algorithms/src/tarjan_scc/tests.rs, SHA-256 339a99c1ca852a341427d33094b10b221b1bae089fa64eb445c2fcc74c513184.

### Undated · independent-oracle

Type: finite_reference_relation. Subject/revision: algorithms/src/tarjan_scc/tests.rs, SHA-256 339a99c1ca852a341427d33094b10b221b1bae089fa64eb445c2fcc74c513184.

### Undated · witness-observation

Type: certificate_projection. Subject/revision: algorithms/src/tarjan_scc/tests.rs, SHA-256 339a99c1ca852a341427d33094b10b221b1bae089fa64eb445c2fcc74c513184.

### Undated · compiled-negative-control

Type: counterfactual_oracle_intervention. Subject/revision: algorithms/src/tarjan_scc/tests.rs, SHA-256 339a99c1ca852a341427d33094b10b221b1bae089fa64eb445c2fcc74c513184.
