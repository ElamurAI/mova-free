# algorithm-hopcroft-karp

## descriptant

### hopcroft_karp mod.rs

Reject combined vertex cap, counted edge cap, then first invalid neighbor. BFS from all free left vertices obtains shortest alternating depth. Explicit DFS stacks and shared cursors find shortest paths and prune dead left states. Flip each path; repeat until no free right is reachable.

Composition: Same coordinates as bipartite_matching; compare cardinality, not optimum mate identity.

Two release runs, seven alternating-order batches: old/new ratios 7.32/7.32 at 256 independent pairs (100 calls/batch), 69.15/70.67 at 4096 pairs (10 calls/batch), 1.35/1.38 for dense K64,64 (30 calls/batch), 4.78/4.73 for 1024-left reroute chain (20 calls/batch). One pair loses: 0.581/0.575, 74/73 ns versus 43/42 ns (3000 calls/batch). Allocation included, construction excluded; no universal speedup.

Prospective batching-cost prediction saved before implementation/timing. Wide independent paths support it; tiny cases expose overhead. Exact operations remain exact; no useful percentage-error surrogate metric identified. Partial syntax leaves macros opaque and does not resolve semantics.

## approximant

### Forward and cost

Left adjacency rows and right count. L+R<=1,000,000 and E<=1,000,000. Return one optional right mate per left.

Reject combined vertex cap, counted edge cap, then first invalid neighbor. BFS from all free left vertices obtains shortest alternating depth. Explicit DFS stacks and shared cursors find shortest paths and prune dead left states. Flip each path; repeat until no free right is reachable.

O((L+R+E)*sqrt(L+R+1)) time including isolated-array scans; O(L+R) auxiliary space. Each phase advances row cursors at most E times. Checked edge sums and short-circuited vertex subtraction precede allocation. Depth bounded by cap; allocation failure untyped.

## invariant

### Invariant

Mate arrays are reciprocal partial injections supported by input edges. Each augmentation increases cardinality by one. Matched-left transitions increase distance; only shortest free-right endpoints finish paths. No augmenting path remains on return.

## catalog

# Typed model family

## behavior-complement

Domain: Left adjacency rows and right count. L+R<=1,000,000 and E<=1,000,000. Return one optional right mate per left.
Observation: complete public return or named harness result, restricted to the explicitly named projection.
Operator: U is declared inputs/errors crossed with outputs. C=U minus forward relation R includes wrong first errors, infeasible witnesses and wrong optima/canonical fields. Exact specification, partial Rust correspondence only. Mate arrays are reciprocal partial injections supported by input edges. Each augmentation increases cardinality by one. Matched-left transitions increase distance; only shortest free-right endpoints finish paths. No augmenting path remains on return.
Approximation: exact specified relation, partial source correspondence; no numerical error budget.
Assumptions: standard Rust semantics, successful allocation, trusted compiler/harness and declared coordinate identity.
Evidence: Five tests: all topologies with 0..4 left and 0..3 right vertices against independent partial-injection enumeration. Membership/uniqueness, secondary existing matcher cardinality and exact repeats; reroute, duplicates/permutations, 20,000-left alternating path, disjoint additivity, ambiguity, limits/first errors. See model.md#tests for compiled controls.
Status: partial_source_derived_model; no semantic AST or general proof.

## reverse-reconstruction

Domain: Left adjacency rows and right count. L+R<=1,000,000 and E<=1,000,000. Return one optional right mate per left.
Observation: complete public return or named harness result, restricted to the explicitly named projection.
Operator: Pre(y)={x in domain | R(x,y)} preserves every compatible input. Traversal history is omitted, graph redundancy creates multiple antecedents, and errors have whole first-failure fibers. No unique inverse or reconstruction executor claimed.
Approximation: exact specified relation, partial source correspondence; no numerical error budget.
Assumptions: standard Rust semantics, successful allocation, trusted compiler/harness and declared coordinate identity.
Evidence: Five tests: all topologies with 0..4 left and 0..3 right vertices against independent partial-injection enumeration. Membership/uniqueness, secondary existing matcher cardinality and exact repeats; reroute, duplicates/permutations, 20,000-left alternating path, disjoint additivity, ambiguity, limits/first errors. See model.md#tests for compiled controls.
Status: partial_source_derived_model; no semantic AST or general proof.

## matching-feasibility

Domain: Left adjacency rows and right count. L+R<=1,000,000 and E<=1,000,000. Return one optional right mate per left.
Observation: complete public return or named harness result, restricted to the explicitly named projection.
Operator: Every mate[u] belongs to graph[u]; no two left vertices own a right. Reciprocal maps are restored after complete path flips; intermediate rewiring is not publicly observable.
Approximation: exact specified relation, partial source correspondence; no numerical error budget.
Assumptions: standard Rust semantics, successful allocation, trusted compiler/harness and declared coordinate identity.
Evidence: Five tests: all topologies with 0..4 left and 0..3 right vertices against independent partial-injection enumeration. Membership/uniqueness, secondary existing matcher cardinality and exact repeats; reroute, duplicates/permutations, 20,000-left alternating path, disjoint additivity, ambiguity, limits/first errors. See model.md#tests for compiled controls.
Status: partial_source_derived_model; no semantic AST or general proof.

## shortest-phase

Domain: Left adjacency rows and right count. L+R<=1,000,000 and E<=1,000,000. Return one optional right mate per left.
Observation: complete public return or named harness result, restricted to the explicitly named projection.
Operator: BFS shortest free-right depth starts at all free left roots. DFS terminates only at that depth, follows next-left distance+1, and prunes exhausted vertices. Shared cursors avoid repeated failed edges; maximal shortest-path augmentation supports classical phase bound.
Approximation: exact specified relation, partial source correspondence; no numerical error budget.
Assumptions: standard Rust semantics, successful allocation, trusted compiler/harness and declared coordinate identity.
Evidence: Five tests: all topologies with 0..4 left and 0..3 right vertices against independent partial-injection enumeration. Membership/uniqueness, secondary existing matcher cardinality and exact repeats; reroute, duplicates/permutations, 20,000-left alternating path, disjoint additivity, ambiguity, limits/first errors. See model.md#tests for compiled controls.
Status: partial_source_derived_model; no semantic AST or general proof.

## tie-order

Domain: Left adjacency rows and right count. L+R<=1,000,000 and E<=1,000,000. Return one optional right mate per left.
Observation: complete public return or named harness result, restricted to the explicitly named projection.
Operator: K2,2 rows [0,1] choose [0,1]; rows [1,0] choose [1,0]. Neighbor permutation preserves optimum cardinality but not witness. Fixed input gives deterministic output.
Approximation: exact specified relation, partial source correspondence; no numerical error budget.
Assumptions: standard Rust semantics, successful allocation, trusted compiler/harness and declared coordinate identity.
Evidence: Five tests: all topologies with 0..4 left and 0..3 right vertices against independent partial-injection enumeration. Membership/uniqueness, secondary existing matcher cardinality and exact repeats; reroute, duplicates/permutations, 20,000-left alternating path, disjoint additivity, ambiguity, limits/first errors. See model.md#tests for compiled controls.
Status: partial_source_derived_model; no semantic AST or general proof.

## disjoint-union

Domain: Left adjacency rows and right count. L+R<=1,000,000 and E<=1,000,000. Return one optional right mate per left.
Observation: complete public return or named harness result, restricted to the explicitly named projection.
Operator: Shift B right coordinates by A right count and concatenate rows. Feasible injections separate; optimum cardinalities add. Executed join preserves row/right identity.
Approximation: exact specified relation, partial source correspondence; no numerical error budget.
Assumptions: standard Rust semantics, successful allocation, trusted compiler/harness and declared coordinate identity.
Evidence: Five tests: all topologies with 0..4 left and 0..3 right vertices against independent partial-injection enumeration. Membership/uniqueness, secondary existing matcher cardinality and exact repeats; reroute, duplicates/permutations, 20,000-left alternating path, disjoint additivity, ambiguity, limits/first errors. See model.md#tests for compiled controls.
Status: partial_source_derived_model; no semantic AST or general proof.

## duplicate-fiber

Domain: Left adjacency rows and right count. L+R<=1,000,000 and E<=1,000,000. Return one optional right mate per left.
Observation: complete public return or named harness result, restricted to the explicitly named projection.
Operator: Duplicate existing neighbors preserve support/cardinality. Rows [0] and [0,0] share even complete output. Mates cannot recover multiplicity or rejected alternatives.
Approximation: exact specified relation, partial source correspondence; no numerical error budget.
Assumptions: standard Rust semantics, successful allocation, trusted compiler/harness and declared coordinate identity.
Evidence: Five tests: all topologies with 0..4 left and 0..3 right vertices against independent partial-injection enumeration. Membership/uniqueness, secondary existing matcher cardinality and exact repeats; reroute, duplicates/permutations, 20,000-left alternating path, disjoint additivity, ambiguity, limits/first errors. See model.md#tests for compiled controls.
Status: partial_source_derived_model; no semantic AST or general proof.

## validation

Domain: Left adjacency rows and right count. L+R<=1,000,000 and E<=1,000,000. Return one optional right mate per left.
Observation: complete public return or named harness result, restricted to the explicitly named projection.
Operator: Combined L+R cap precedes E cap, both precede endpoint checks. Checked counting prevents overflow. Right 0 invalid when R=0; empty rows return None.
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

Epoch 2026-10-30: original code/fixtures; scientific attribution in mod.rs. O((L+R+E)*sqrt(L+R+1)) time including isolated-array scans; O(L+R) auxiliary space. Each phase advances row cursors at most E times. Checked edge sums and short-circuited vertex subtraction precede allocation. Depth bounded by cap; allocation failure untyped.

### 2026-10-30 · tests

2026-10-30: 356 library unit tests pass twice debug and twice release; identical normalized names/results SHA-256 56550bbf2b2f3d4999f9d5f3ea0230762f4ef2bb54a8a850d3687daf0129943c. This file contributes five tests or their production subject. Strict algorithm all-target Clippy and narrow rustfmt pass.

Compiled control: Return all-unmatched output; exhaustive_partial_injections rejects. Log SHA-256 0c0de47eb78f9a63f316fb75b7eb8ac434a7c2e5022426867fd7186c8daf4fd7. Named assertion failure, not compile failure. Six mutants restored byte-for-byte before repeated positives.

Timing logs SHA-256 b3fca25c0729a0edecefdf1c4914f973f761be705db618171b159f4ac96ad7c3 and 9e8b9c417dfc0dfda48f1446825054bfca985f2aa96dea370843c1184e7b020d; Rust harness SHA-256 f78acc0796bf87f2e32ae559c46519badb9b3219b1ed449a56ca5362cb166799; prospective derivation SHA-256 41f9013788fa8bdce512d82f7922287ed2bd9c87bd84dbfca26180aa4d34a03d.

### Undated · behavior-complement

Type: behavior_complement. Subject/revision: algorithms/src/hopcroft_karp/mod.rs, SHA-256 9791e2f96dc1063cc1c73fff563fd4dbb953ad1fab922baf7d1ed549e41b045a.

### Undated · reverse-reconstruction

Type: reverse_reconstruction. Subject/revision: algorithms/src/hopcroft_karp/mod.rs, SHA-256 9791e2f96dc1063cc1c73fff563fd4dbb953ad1fab922baf7d1ed549e41b045a.

### Undated · matching-feasibility

Type: partial_injection. Subject/revision: algorithms/src/hopcroft_karp/mod.rs, SHA-256 9791e2f96dc1063cc1c73fff563fd4dbb953ad1fab922baf7d1ed549e41b045a.

### Undated · shortest-phase

Type: augmenting_layer_progress. Subject/revision: algorithms/src/hopcroft_karp/mod.rs, SHA-256 9791e2f96dc1063cc1c73fff563fd4dbb953ad1fab922baf7d1ed549e41b045a.

### Undated · tie-order

Type: deterministic_noncanonical_witness. Subject/revision: algorithms/src/hopcroft_karp/mod.rs, SHA-256 9791e2f96dc1063cc1c73fff563fd4dbb953ad1fab922baf7d1ed549e41b045a.

### Undated · disjoint-union

Type: cardinality_additivity. Subject/revision: algorithms/src/hopcroft_karp/mod.rs, SHA-256 9791e2f96dc1063cc1c73fff563fd4dbb953ad1fab922baf7d1ed549e41b045a.

### Undated · duplicate-fiber

Type: edge_multiplicity_preimage. Subject/revision: algorithms/src/hopcroft_karp/mod.rs, SHA-256 9791e2f96dc1063cc1c73fff563fd4dbb953ad1fab922baf7d1ed549e41b045a.

### Undated · validation

Type: resource_error_precedence. Subject/revision: algorithms/src/hopcroft_karp/mod.rs, SHA-256 9791e2f96dc1063cc1c73fff563fd4dbb953ad1fab922baf7d1ed549e41b045a.
