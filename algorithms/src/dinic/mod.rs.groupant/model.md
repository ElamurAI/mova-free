# algorithm-dinic

## descriptant

### dinic mod.rs

Reject size caps, source, sink, same terminal, then first invalid edge. Allocate paired residual arcs. FIFO BFS fixes levels; monotone cursors and explicit path backtracking exhaust blocking flows. Augment bottlenecks with checked arithmetic. Return reverse residuals and final BFS reachability.

Composition: Reuses max_flow Edge/Flow identity; independent certificates plus secondary Edmonds-Karp values.

Two release runs, seven alternating-order batches: old/new median ratio 28.27/28.46 for 256 disjoint lanes (V=258,E=512,30 calls/batch), 119.88/120.98 for 1024 lanes (V=1026,E=2048,8 calls/batch). Single path V=256: 1.05/1.03 (100 calls/batch); one edge loses: 0.935/0.930, 199 ns versus 186/185 ns (2000 calls/batch). Allocations included, construction excluded. Shape-specific, shared-machine evidence; no universal speedup.

Prospective batching-cost prediction saved before implementation/timing. Wide independent paths support it; tiny cases expose overhead. Exact operations remain exact; no useful percentage-error surrogate metric identified. Partial syntax leaves macros opaque and does not resolve semantics.

## approximant

### Forward and cost

Directed u64-capacity edges and distinct indexed terminals, V and E at most 1,000,000. Return existing max_flow::Flow: u128 value, per-edge flows and residual source-side mask.

Reject size caps, source, sink, same terminal, then first invalid edge. Allocate paired residual arcs. FIFO BFS fixes levels; monotone cursors and explicit path backtracking exhaust blocking flows. Augment bottlenecks with checked arithmetic. Return reverse residuals and final BFS reachability.

O(V+E) initialization; each phase has O(V+E) failed scans and at most E paths of length at most V. Fewer than V reachable-sink phases: conservative O(V^2 E+V^2+E) time, O(V+E) storage. No recursion. u128 total <=1,000,000*u64::MAX; Overflow defensive under caps/invariants. Allocation failure untyped.

## invariant

### Invariant

Each residual pair sums to original capacity. Flows respect capacities and conserve mass off terminals. Level paths strictly increase depth; loops have zero flow. Final saturated cut certifies optimum; source-side mask equals residual reachability.

## catalog

# Typed model family

## behavior-complement

Domain: Directed u64-capacity edges and distinct indexed terminals, V and E at most 1,000,000. Return existing max_flow::Flow: u128 value, per-edge flows and residual source-side mask.
Observation: complete public return or named harness result, restricted to the explicitly named projection.
Operator: U is declared inputs/errors crossed with outputs. C=U minus forward relation R includes wrong first errors, infeasible witnesses and wrong optima/canonical fields. Exact specification, partial Rust correspondence only. Each residual pair sums to original capacity. Flows respect capacities and conserve mass off terminals. Level paths strictly increase depth; loops have zero flow. Final saturated cut certifies optimum; source-side mask equals residual reachability.
Approximation: exact specified relation, partial source correspondence; no numerical error budget.
Assumptions: standard Rust semantics, successful allocation, trusted compiler/harness and declared coordinate identity.
Evidence: Five tests: 729 ternary-capacity three-vertex networks times six terminals (4374 comparisons), all 4096 four-vertex loop-free unit graphs. Independent cuts, conservation, relaxed residual reachability; secondary Edmonds-Karp values. Cancellation, loops, antiparallel/parallel edges, u64::MAX scaling, error precedence, 20,000-vertex path, value ambiguity. See model.md#tests for compiled controls.
Status: partial_source_derived_model; no semantic AST or general proof.

## reverse-reconstruction

Domain: Directed u64-capacity edges and distinct indexed terminals, V and E at most 1,000,000. Return existing max_flow::Flow: u128 value, per-edge flows and residual source-side mask.
Observation: complete public return or named harness result, restricted to the explicitly named projection.
Operator: Pre(y)={x in domain | R(x,y)} preserves every compatible input. Traversal history is omitted, graph redundancy creates multiple antecedents, and errors have whole first-failure fibers. No unique inverse or reconstruction executor claimed.
Approximation: exact specified relation, partial source correspondence; no numerical error budget.
Assumptions: standard Rust semantics, successful allocation, trusted compiler/harness and declared coordinate identity.
Evidence: Five tests: 729 ternary-capacity three-vertex networks times six terminals (4374 comparisons), all 4096 four-vertex loop-free unit graphs. Independent cuts, conservation, relaxed residual reachability; secondary Edmonds-Karp values. Cancellation, loops, antiparallel/parallel edges, u64::MAX scaling, error precedence, 20,000-vertex path, value ambiguity. See model.md#tests for compiled controls.
Status: partial_source_derived_model; no semantic AST or general proof.

## residual-pairs

Domain: Directed u64-capacity edges and distinct indexed terminals, V and E at most 1,000,000. Return existing max_flow::Flow: u128 value, per-edge flows and residual source-side mask.
Observation: complete public return or named harness result, restricted to the explicitly named projection.
Operator: Fix e: r_forward+r_reverse=c_e and f_e=r_reverse. Conservation and terminal balance imply feasibility. Final residual cut capacity equals value; verify checks these separately.
Approximation: exact specified relation, partial source correspondence; no numerical error budget.
Assumptions: standard Rust semantics, successful allocation, trusted compiler/harness and declared coordinate identity.
Evidence: Five tests: 729 ternary-capacity three-vertex networks times six terminals (4374 comparisons), all 4096 four-vertex loop-free unit graphs. Independent cuts, conservation, relaxed residual reachability; secondary Edmonds-Karp values. Cancellation, loops, antiparallel/parallel edges, u64::MAX scaling, error precedence, 20,000-vertex path, value ambiguity. See model.md#tests for compiled controls.
Status: partial_source_derived_model; no semantic AST or general proof.

## phase-progress

Domain: Directed u64-capacity edges and distinct indexed terminals, V and E at most 1,000,000. Return existing max_flow::Flow: u128 value, per-edge flows and residual source-side mask.
Observation: complete public return or named harness result, restricted to the explicitly named projection.
Operator: With fixed BFS levels, cursors advance monotonically. Dead children are exhausted before their incoming arc is discarded. Each path saturates a level arc; reverse updates cannot reopen forward level arcs that phase. Next finite sink distance increases.
Approximation: exact specified relation, partial source correspondence; no numerical error budget.
Assumptions: standard Rust semantics, successful allocation, trusted compiler/harness and declared coordinate identity.
Evidence: Five tests: 729 ternary-capacity three-vertex networks times six terminals (4374 comparisons), all 4096 four-vertex loop-free unit graphs. Independent cuts, conservation, relaxed residual reachability; secondary Edmonds-Karp values. Cancellation, loops, antiparallel/parallel edges, u64::MAX scaling, error precedence, 20,000-vertex path, value ambiguity. See model.md#tests for compiled controls.
Status: partial_source_derived_model; no semantic AST or general proof.

## tie-trace

Domain: Directed u64-capacity edges and distinct indexed terminals, V and E at most 1,000,000. Return existing max_flow::Flow: u128 value, per-edge flows and residual source-side mask.
Observation: complete public return or named harness result, restricted to the explicitly named projection.
Operator: Fixed ordered inputs determine FIFO and path selection. Edmonds-Karp comparison equates values, not vectors. A repeated cancellation fixture fixes one rerouted edge to zero.
Approximation: exact specified relation, partial source correspondence; no numerical error budget.
Assumptions: standard Rust semantics, successful allocation, trusted compiler/harness and declared coordinate identity.
Evidence: Five tests: 729 ternary-capacity three-vertex networks times six terminals (4374 comparisons), all 4096 four-vertex loop-free unit graphs. Independent cuts, conservation, relaxed residual reachability; secondary Edmonds-Karp values. Cancellation, loops, antiparallel/parallel edges, u64::MAX scaling, error precedence, 20,000-vertex path, value ambiguity. See model.md#tests for compiled controls.
Status: partial_source_derived_model; no semantic AST or general proof.

## capacity-scaling

Domain: Directed u64-capacity edges and distinct indexed terminals, V and E at most 1,000,000. Return existing max_flow::Flow: u128 value, per-edge flows and residual source-side mask.
Observation: complete public return or named harness result, restricted to the explicitly named projection.
Operator: Positive k with all u64 products fitting preserves residual zero tests and bottleneck order. Flows/value scale by k, source-side bits stay fixed. Fixture executes k=u64::MAX.
Approximation: exact specified relation, partial source correspondence; no numerical error budget.
Assumptions: standard Rust semantics, successful allocation, trusted compiler/harness and declared coordinate identity.
Evidence: Five tests: 729 ternary-capacity three-vertex networks times six terminals (4374 comparisons), all 4096 four-vertex loop-free unit graphs. Independent cuts, conservation, relaxed residual reachability; secondary Edmonds-Karp values. Cancellation, loops, antiparallel/parallel edges, u64::MAX scaling, error precedence, 20,000-vertex path, value ambiguity. See model.md#tests for compiled controls.
Status: partial_source_derived_model; no semantic AST or general proof.

## value-fiber

Domain: Directed u64-capacity edges and distinct indexed terminals, V and E at most 1,000,000. Return existing max_flow::Flow: u128 value, per-edge flows and residual source-side mask.
Observation: complete public return or named harness result, restricted to the explicitly named projection.
Operator: Path capacities (1,b), b>=1, all give value 1. Test b=1..7. Objective cannot reconstruct slack capacities; even full Flow remains noninjective.
Approximation: exact specified relation, partial source correspondence; no numerical error budget.
Assumptions: standard Rust semantics, successful allocation, trusted compiler/harness and declared coordinate identity.
Evidence: Five tests: 729 ternary-capacity three-vertex networks times six terminals (4374 comparisons), all 4096 four-vertex loop-free unit graphs. Independent cuts, conservation, relaxed residual reachability; secondary Edmonds-Karp values. Cancellation, loops, antiparallel/parallel edges, u64::MAX scaling, error precedence, 20,000-vertex path, value ambiguity. See model.md#tests for compiled controls.
Status: partial_source_derived_model; no semantic AST or general proof.

## validation

Domain: Directed u64-capacity edges and distinct indexed terminals, V and E at most 1,000,000. Return existing max_flow::Flow: u128 value, per-edge flows and residual source-side mask.
Observation: complete public return or named harness result, restricted to the explicitly named projection.
Operator: Size cap precedes source, sink, same terminal, then first edge error. Even zero-capacity endpoints are validated. Overflow is defensive, not an allocation guarantee.
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

Epoch 2026-10-30: original code/fixtures; scientific attribution in mod.rs. O(V+E) initialization; each phase has O(V+E) failed scans and at most E paths of length at most V. Fewer than V reachable-sink phases: conservative O(V^2 E+V^2+E) time, O(V+E) storage. No recursion. u128 total <=1,000,000*u64::MAX; Overflow defensive under caps/invariants. Allocation failure untyped.

### 2026-10-30 · tests

2026-10-30: 356 library unit tests pass twice debug and twice release; identical normalized names/results SHA-256 56550bbf2b2f3d4999f9d5f3ea0230762f4ef2bb54a8a850d3687daf0129943c. This file contributes five tests or their production subject. Strict algorithm all-target Clippy and narrow rustfmt pass.

Compiled control: Initial zero value becomes one; exhaustive_weighted_cuts rejects. Log SHA-256 b496ace121fd13bf8bc0ec50c96f297d0a3c0024bd6edee016416f2c98ca9d12. Named assertion failure, not compile failure. Six mutants restored byte-for-byte before repeated positives.

Timing logs SHA-256 b3fca25c0729a0edecefdf1c4914f973f761be705db618171b159f4ac96ad7c3 and 9e8b9c417dfc0dfda48f1446825054bfca985f2aa96dea370843c1184e7b020d; Rust harness SHA-256 f78acc0796bf87f2e32ae559c46519badb9b3219b1ed449a56ca5362cb166799; prospective derivation SHA-256 41f9013788fa8bdce512d82f7922287ed2bd9c87bd84dbfca26180aa4d34a03d.

### Undated · behavior-complement

Type: behavior_complement. Subject/revision: algorithms/src/dinic/mod.rs, SHA-256 672bc5ddd53b228e96dc9bcc1021af80ffc3e8ba084c9fe85e53e23dad2deb55.

### Undated · reverse-reconstruction

Type: reverse_reconstruction. Subject/revision: algorithms/src/dinic/mod.rs, SHA-256 672bc5ddd53b228e96dc9bcc1021af80ffc3e8ba084c9fe85e53e23dad2deb55.

### Undated · residual-pairs

Type: conservation_certificate. Subject/revision: algorithms/src/dinic/mod.rs, SHA-256 672bc5ddd53b228e96dc9bcc1021af80ffc3e8ba084c9fe85e53e23dad2deb55.

### Undated · phase-progress

Type: blocking_flow_progress. Subject/revision: algorithms/src/dinic/mod.rs, SHA-256 672bc5ddd53b228e96dc9bcc1021af80ffc3e8ba084c9fe85e53e23dad2deb55.

### Undated · tie-trace

Type: deterministic_witness. Subject/revision: algorithms/src/dinic/mod.rs, SHA-256 672bc5ddd53b228e96dc9bcc1021af80ffc3e8ba084c9fe85e53e23dad2deb55.

### Undated · capacity-scaling

Type: positive_scale_equivariance. Subject/revision: algorithms/src/dinic/mod.rs, SHA-256 672bc5ddd53b228e96dc9bcc1021af80ffc3e8ba084c9fe85e53e23dad2deb55.

### Undated · value-fiber

Type: objective_projection_preimage. Subject/revision: algorithms/src/dinic/mod.rs, SHA-256 672bc5ddd53b228e96dc9bcc1021af80ffc3e8ba084c9fe85e53e23dad2deb55.

### Undated · validation

Type: first_failure_partition. Subject/revision: algorithms/src/dinic/mod.rs, SHA-256 672bc5ddd53b228e96dc9bcc1021af80ffc3e8ba084c9fe85e53e23dad2deb55.
