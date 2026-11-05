# algorithm-dinic-tests

## descriptant

### dinic tests.rs

Generate bounded cases and fixtures, independently evaluate reference relations, verify witnesses and assert equality. Five tests: 729 ternary-capacity three-vertex networks times six terminals (4374 comparisons), all 4096 four-vertex loop-free unit graphs. Independent cuts, conservation, relaxed residual reachability; secondary Edmonds-Karp values. Cancellation, loops, antiparallel/parallel edges, u64::MAX scaling, error precedence, 20,000-vertex path, value ambiguity.

Composition: Reuses max_flow Edge/Flow identity; independent certificates plus secondary Edmonds-Karp values.

Two release runs, seven alternating-order batches: old/new median ratio 28.27/28.46 for 256 disjoint lanes (V=258,E=512,30 calls/batch), 119.88/120.98 for 1024 lanes (V=1026,E=2048,8 calls/batch). Single path V=256: 1.05/1.03 (100 calls/batch); one edge loses: 0.935/0.930, 199 ns versus 186/185 ns (2000 calls/batch). Allocations included, construction excluded. Shape-specific, shared-machine evidence; no universal speedup.

Prospective batching-cost prediction saved before implementation/timing. Wide independent paths support it; tiny cases expose overhead. Exact operations remain exact; no useful percentage-error surrogate metric identified. Partial syntax leaves macros opaque and does not resolve semantics.

## approximant

### Forward and cost

Finite inputs and harness outcomes for dinic. Directed u64-capacity edges and distinct indexed terminals, V and E at most 1,000,000. Return existing max_flow::Flow: u128 value, per-edge flows and residual source-side mask.

Generate bounded cases and fixtures, independently evaluate reference relations, verify witnesses and assert equality. Five tests: 729 ternary-capacity three-vertex networks times six terminals (4374 comparisons), all 4096 four-vertex loop-free unit graphs. Independent cuts, conservation, relaxed residual reachability; secondary Edmonds-Karp values. Cancellation, loops, antiparallel/parallel edges, u64::MAX scaling, error precedence, 20,000-vertex path, value ambiguity.

Bounded exponential enumeration; oracle cost differs from production cost. Five tests: 729 ternary-capacity three-vertex networks times six terminals (4374 comparisons), all 4096 four-vertex loop-free unit graphs. Independent cuts, conservation, relaxed residual reachability; secondary Edmonds-Karp values. Cancellation, loops, antiparallel/parallel edges, u64::MAX scaling, error precedence, 20,000-vertex path, value ambiguity.

## invariant

### Invariant

Each finite case supplies separately justified assertions. Passing totals do not cover omitted inputs, compiler defects or allocation behavior.

## catalog

# Typed model family

## behavior-complement

Domain: Finite inputs and harness outcomes for dinic. Directed u64-capacity edges and distinct indexed terminals, V and E at most 1,000,000. Return existing max_flow::Flow: u128 value, per-edge flows and residual source-side mask.
Observation: complete public return or named harness result, restricted to the explicitly named projection.
Operator: U is finite fixtures crossed with pass/fail observations. Complement includes passing traces with violated assertions and failing traces with every assertion true. Exact specification target with only partial compiled correspondence. Each finite case supplies separately justified assertions. Passing totals do not cover omitted inputs, compiler defects or allocation behavior.
Approximation: exact specified relation, partial source correspondence; no numerical error budget.
Assumptions: standard Rust semantics, successful allocation, trusted compiler/harness and declared coordinate identity.
Evidence: Five tests: 729 ternary-capacity three-vertex networks times six terminals (4374 comparisons), all 4096 four-vertex loop-free unit graphs. Independent cuts, conservation, relaxed residual reachability; secondary Edmonds-Karp values. Cancellation, loops, antiparallel/parallel edges, u64::MAX scaling, error precedence, 20,000-vertex path, value ambiguity. See model.md#tests for compiled controls.
Status: partial_source_derived_model; no semantic AST or general proof.

## reverse-reconstruction

Domain: Finite inputs and harness outcomes for dinic. Directed u64-capacity edges and distinct indexed terminals, V and E at most 1,000,000. Return existing max_flow::Flow: u128 value, per-edge flows and residual source-side mask.
Observation: complete public return or named harness result, restricted to the explicitly named projection.
Operator: Pre(pass) keeps all finite observation tuples satisfying assertions, not one recoverable graph/trace. Pre(fail) keeps all tuples violating any assertion. Failure lines lose other fixture details. No unique inverse or complete reconstruction executor.
Approximation: exact specified relation, partial source correspondence; no numerical error budget.
Assumptions: standard Rust semantics, successful allocation, trusted compiler/harness and declared coordinate identity.
Evidence: Five tests: 729 ternary-capacity three-vertex networks times six terminals (4374 comparisons), all 4096 four-vertex loop-free unit graphs. Independent cuts, conservation, relaxed residual reachability; secondary Edmonds-Karp values. Cancellation, loops, antiparallel/parallel edges, u64::MAX scaling, error precedence, 20,000-vertex path, value ambiguity. See model.md#tests for compiled controls.
Status: partial_source_derived_model; no semantic AST or general proof.

## independent-oracle

Domain: Finite inputs and harness outcomes for dinic. Directed u64-capacity edges and distinct indexed terminals, V and E at most 1,000,000. Return existing max_flow::Flow: u128 value, per-edge flows and residual source-side mask.
Observation: complete public return or named harness result, restricted to the explicitly named projection.
Operator: Fix graph coordinates; independently enumerate the objective/walk relation on those same coordinates. Library agreement is only secondary. Five tests: 729 ternary-capacity three-vertex networks times six terminals (4374 comparisons), all 4096 four-vertex loop-free unit graphs. Independent cuts, conservation, relaxed residual reachability; secondary Edmonds-Karp values. Cancellation, loops, antiparallel/parallel edges, u64::MAX scaling, error precedence, 20,000-vertex path, value ambiguity.
Approximation: exact specified relation, partial source correspondence; no numerical error budget.
Assumptions: standard Rust semantics, successful allocation, trusted compiler/harness and declared coordinate identity.
Evidence: Five tests: 729 ternary-capacity three-vertex networks times six terminals (4374 comparisons), all 4096 four-vertex loop-free unit graphs. Independent cuts, conservation, relaxed residual reachability; secondary Edmonds-Karp values. Cancellation, loops, antiparallel/parallel edges, u64::MAX scaling, error precedence, 20,000-vertex path, value ambiguity. See model.md#tests for compiled controls.
Status: partial_source_derived_model; no semantic AST or general proof.

## witness-observation

Domain: Finite inputs and harness outcomes for dinic. Directed u64-capacity edges and distinct indexed terminals, V and E at most 1,000,000. Return existing max_flow::Flow: u128 value, per-edge flows and residual source-side mask.
Observation: complete public return or named harness result, restricted to the explicitly named projection.
Operator: Check actual-return membership/conservation/canonical assertions. Objective-only equality loses witness identity; retain explicit witness constraints. Each finite case supplies separately justified assertions. Passing totals do not cover omitted inputs, compiler defects or allocation behavior.
Approximation: exact specified relation, partial source correspondence; no numerical error budget.
Assumptions: standard Rust semantics, successful allocation, trusted compiler/harness and declared coordinate identity.
Evidence: Five tests: 729 ternary-capacity three-vertex networks times six terminals (4374 comparisons), all 4096 four-vertex loop-free unit graphs. Independent cuts, conservation, relaxed residual reachability; secondary Edmonds-Karp values. Cancellation, loops, antiparallel/parallel edges, u64::MAX scaling, error precedence, 20,000-vertex path, value ambiguity. See model.md#tests for compiled controls.
Status: partial_source_derived_model; no semantic AST or general proof.

## compiled-negative-control

Domain: Finite inputs and harness outcomes for dinic. Directed u64-capacity edges and distinct indexed terminals, V and E at most 1,000,000. Return existing max_flow::Flow: u128 value, per-edge flows and residual source-side mask.
Observation: complete public return or named harness result, restricted to the explicitly named projection.
Operator: Change the independent oracle while retaining compilability, require named runtime assertion failure, restore exact bytes and rerun positives. Only sensitivity to this chosen defect is established.
Approximation: exact specified relation, partial source correspondence; no numerical error budget.
Assumptions: standard Rust semantics, successful allocation, trusted compiler/harness and declared coordinate identity.
Evidence: Five tests: 729 ternary-capacity three-vertex networks times six terminals (4374 comparisons), all 4096 four-vertex loop-free unit graphs. Independent cuts, conservation, relaxed residual reachability; secondary Edmonds-Karp values. Cancellation, loops, antiparallel/parallel edges, u64::MAX scaling, error precedence, 20,000-vertex path, value ambiguity. See model.md#tests for compiled controls.
Status: partial_source_derived_model; no semantic AST or general proof.

## tests

### Finite evidence

Five tests: 729 ternary-capacity three-vertex networks times six terminals (4374 comparisons), all 4096 four-vertex loop-free unit graphs. Independent cuts, conservation, relaxed residual reachability; secondary Edmonds-Karp values. Cancellation, loops, antiparallel/parallel edges, u64::MAX scaling, error precedence, 20,000-vertex path, value ambiguity.

Two release runs, seven alternating-order batches: old/new median ratio 28.27/28.46 for 256 disjoint lanes (V=258,E=512,30 calls/batch), 119.88/120.98 for 1024 lanes (V=1026,E=2048,8 calls/batch). Single path V=256: 1.05/1.03 (100 calls/batch); one edge loses: 0.935/0.930, 199 ns versus 186/185 ns (2000 calls/batch). Allocations included, construction excluded. Shape-specific, shared-machine evidence; no universal speedup.

Raw logs external. No live parser/teacher/GPU/course/held-out runs.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: original code/fixtures; scientific attribution in mod.rs. Bounded exponential enumeration; oracle cost differs from production cost. Five tests: 729 ternary-capacity three-vertex networks times six terminals (4374 comparisons), all 4096 four-vertex loop-free unit graphs. Independent cuts, conservation, relaxed residual reachability; secondary Edmonds-Karp values. Cancellation, loops, antiparallel/parallel edges, u64::MAX scaling, error precedence, 20,000-vertex path, value ambiguity.

### 2026-10-30 · tests

2026-10-30: 356 library unit tests pass twice debug and twice release; identical normalized names/results SHA-256 56550bbf2b2f3d4999f9d5f3ea0230762f4ef2bb54a8a850d3687daf0129943c. This file contributes five tests or their production subject. Strict algorithm all-target Clippy and narrow rustfmt pass.

Compiled control: Minimum cut oracle becomes maximum cut; exhaustive_weighted_cuts rejects. Log SHA-256 912179ddea3c4c058aef1f6918e8b151c928ee11a0238df2f09364b0d89d7728. Named assertion failure, not compile failure. Six mutants restored byte-for-byte before repeated positives.

Timing logs SHA-256 b3fca25c0729a0edecefdf1c4914f973f761be705db618171b159f4ac96ad7c3 and 9e8b9c417dfc0dfda48f1446825054bfca985f2aa96dea370843c1184e7b020d; Rust harness SHA-256 f78acc0796bf87f2e32ae559c46519badb9b3219b1ed449a56ca5362cb166799; prospective derivation SHA-256 41f9013788fa8bdce512d82f7922287ed2bd9c87bd84dbfca26180aa4d34a03d.

### Undated · behavior-complement

Type: behavior_complement. Subject/revision: algorithms/src/dinic/tests.rs, SHA-256 bcc94f4265cdb508975fc11420f0cf84e2452f938dc37ccd399f4873c181477d.

### Undated · reverse-reconstruction

Type: reverse_reconstruction. Subject/revision: algorithms/src/dinic/tests.rs, SHA-256 bcc94f4265cdb508975fc11420f0cf84e2452f938dc37ccd399f4873c181477d.

### Undated · independent-oracle

Type: finite_reference_relation. Subject/revision: algorithms/src/dinic/tests.rs, SHA-256 bcc94f4265cdb508975fc11420f0cf84e2452f938dc37ccd399f4873c181477d.

### Undated · witness-observation

Type: certificate_projection. Subject/revision: algorithms/src/dinic/tests.rs, SHA-256 bcc94f4265cdb508975fc11420f0cf84e2452f938dc37ccd399f4873c181477d.

### Undated · compiled-negative-control

Type: counterfactual_oracle_intervention. Subject/revision: algorithms/src/dinic/tests.rs, SHA-256 bcc94f4265cdb508975fc11420f0cf84e2452f938dc37ccd399f4873c181477d.
