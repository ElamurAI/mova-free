# algorithm-tarjan-scc

## descriptant

### tarjan_scc mod.rs

Validate vertex cap, checked arc count, then endpoints. DFS frames track next neighbor; separate active SCC stack tracks unfinished vertices. Child returns propagate lowlinks; active non-tree arcs use discovery indices. Pop at low[v]=index[v], then canonically sort.

Composition: Same canonical output as strongly_connected_components; independent mutual-walk oracle.

No Tarjan runtime gain claimed. Avoiding reverse adjacency gives O(V) auxiliary storage; canonical sorting retains O(V log(V+1)) time.

Epoch records preserve iterative lowlink/canonical-order decisions; Tarjan has no runtime gain claim. Exact operations remain exact; no useful percentage-error surrogate metric identified. Partial syntax leaves macros opaque and does not resolve semantics.

## approximant

### Forward and cost

Directed adjacency rows, V<=1,000,000 and E<=1,000,000. Return sorted nonempty SCC lists ordered by minimum member, including isolates.

Validate vertex cap, checked arc count, then endpoints. DFS frames track next neighbor; separate active SCC stack tracks unfinished vertices. Child returns propagate lowlinks; active non-tree arcs use discovery indices. Pop at low[v]=index[v], then canonically sort.

Traversal O(V+E), sorting O(V log(V+1)); O(V) auxiliary storage excluding output and borrowed graph. No recursion or reverse adjacency graph. Checked edge count precedes allocation, clock bounded by V; allocation failure untyped.

## invariant

### Invariant

Discovery indices unique, active vertices occur once on pending stack. Finished SCCs leave active membership and cannot lower non-tree lowlinks. Output partitions mutual reachability with every vertex once in canonical order.

## catalog

# Typed model family

## behavior-complement

Domain: Directed adjacency rows, V<=1,000,000 and E<=1,000,000. Return sorted nonempty SCC lists ordered by minimum member, including isolates.
Observation: complete public return or named harness result, restricted to the explicitly named projection.
Operator: U is declared inputs/errors crossed with outputs. C=U minus forward relation R includes wrong first errors, infeasible witnesses and wrong optima/canonical fields. Exact specification, partial Rust correspondence only. Discovery indices unique, active vertices occur once on pending stack. Finished SCCs leave active membership and cannot lower non-tree lowlinks. Output partitions mutual reachability with every vertex once in canonical order.
Approximation: exact specified relation, partial source correspondence; no numerical error budget.
Assumptions: standard Rust semantics, successful allocation, trusted compiler/harness and declared coordinate identity.
Evidence: Five tests: 4166 loop-free topologies over 0..4 vertices against independent bounded-walk closure and existing Kosaraju. Active cross-edge/completed-component fixtures, transpose/duplicates/loops/relabeling, precedence, 30,000-vertex chain then cycle. See model.md#tests for compiled controls.
Status: partial_source_derived_model; no semantic AST or general proof.

## reverse-reconstruction

Domain: Directed adjacency rows, V<=1,000,000 and E<=1,000,000. Return sorted nonempty SCC lists ordered by minimum member, including isolates.
Observation: complete public return or named harness result, restricted to the explicitly named projection.
Operator: Pre(y)={x in domain | R(x,y)} preserves every compatible input. Traversal history is omitted, graph redundancy creates multiple antecedents, and errors have whole first-failure fibers. No unique inverse or reconstruction executor claimed.
Approximation: exact specified relation, partial source correspondence; no numerical error budget.
Assumptions: standard Rust semantics, successful allocation, trusted compiler/harness and declared coordinate identity.
Evidence: Five tests: 4166 loop-free topologies over 0..4 vertices against independent bounded-walk closure and existing Kosaraju. Active cross-edge/completed-component fixtures, transpose/duplicates/loops/relabeling, precedence, 30,000-vertex chain then cycle. See model.md#tests for compiled controls.
Status: partial_source_derived_model; no semantic AST or general proof.

## active-lowlink

Domain: Directed adjacency rows, V<=1,000,000 and E<=1,000,000. Return sorted nonempty SCC lists ordered by minimum member, including isolates.
Observation: complete public return or named harness result, restricted to the explicitly named projection.
Operator: Only active discovered neighbors lower low[from] using index[to]. Emitted SCCs do not rejoin. Child return propagates low[child] to its parent. Fixtures distinguish active cross-edges and completed components.
Approximation: exact specified relation, partial source correspondence; no numerical error budget.
Assumptions: standard Rust semantics, successful allocation, trusted compiler/harness and declared coordinate identity.
Evidence: Five tests: 4166 loop-free topologies over 0..4 vertices against independent bounded-walk closure and existing Kosaraju. Active cross-edge/completed-component fixtures, transpose/duplicates/loops/relabeling, precedence, 30,000-vertex chain then cycle. See model.md#tests for compiled controls.
Status: partial_source_derived_model; no semantic AST or general proof.

## partition

Domain: Directed adjacency rows, V<=1,000,000 and E<=1,000,000. Return sorted nonempty SCC lists ordered by minimum member, including isolates.
Observation: complete public return or named harness result, restricted to the explicitly named projection.
Operator: u~v means walks both ways, including zero steps. Output is its equivalence partition. Independent V-round edge relaxation obtains finite reachability without DFS indices/lowlinks.
Approximation: exact specified relation, partial source correspondence; no numerical error budget.
Assumptions: standard Rust semantics, successful allocation, trusted compiler/harness and declared coordinate identity.
Evidence: Five tests: 4166 loop-free topologies over 0..4 vertices against independent bounded-walk closure and existing Kosaraju. Active cross-edge/completed-component fixtures, transpose/duplicates/loops/relabeling, precedence, 30,000-vertex chain then cycle. See model.md#tests for compiled controls.
Status: partial_source_derived_model; no semantic AST or general proof.

## canonical-order

Domain: Directed adjacency rows, V<=1,000,000 and E<=1,000,000. Return sorted nonempty SCC lists ordered by minimum member, including isolates.
Observation: complete public return or named harness result, restricted to the explicitly named projection.
Operator: Sort members then blocks by minimum. Adjacency permutation/duplicate arcs preserve exact output despite changed DFS/pop traces. Block order is not promised topological.
Approximation: exact specified relation, partial source correspondence; no numerical error budget.
Assumptions: standard Rust semantics, successful allocation, trusted compiler/harness and declared coordinate identity.
Evidence: Five tests: 4166 loop-free topologies over 0..4 vertices against independent bounded-walk closure and existing Kosaraju. Active cross-edge/completed-component fixtures, transpose/duplicates/loops/relabeling, precedence, 30,000-vertex chain then cycle. See model.md#tests for compiled controls.
Status: partial_source_derived_model; no semantic AST or general proof.

## transpose

Domain: Directed adjacency rows, V<=1,000,000 and E<=1,000,000. Return sorted nonempty SCC lists ordered by minimum member, including isolates.
Observation: complete public return or named harness result, restricted to the explicitly named projection.
Operator: Walk u->v in transpose corresponds to v->u in G. Mutual reachability/canonical output remain equal. Fixture also duplicates reversed edges and adds loops.
Approximation: exact specified relation, partial source correspondence; no numerical error budget.
Assumptions: standard Rust semantics, successful allocation, trusted compiler/harness and declared coordinate identity.
Evidence: Five tests: 4166 loop-free topologies over 0..4 vertices against independent bounded-walk closure and existing Kosaraju. Active cross-edge/completed-component fixtures, transpose/duplicates/loops/relabeling, precedence, 30,000-vertex chain then cycle. See model.md#tests for compiled controls.
Status: partial_source_derived_model; no semantic AST or general proof.

## relabel

Domain: Directed adjacency rows, V<=1,000,000 and E<=1,000,000. Return sorted nonempty SCC lists ordered by minimum member, including isolates.
Observation: complete public return or named harness result, restricted to the explicitly named projection.
Operator: For bijection p, relabel sources/targets; map old blocks by p and sort both levels. This equals output on relabeled graph. Sorting needed because minimum labels change.
Approximation: exact specified relation, partial source correspondence; no numerical error budget.
Assumptions: standard Rust semantics, successful allocation, trusted compiler/harness and declared coordinate identity.
Evidence: Five tests: 4166 loop-free topologies over 0..4 vertices against independent bounded-walk closure and existing Kosaraju. Active cross-edge/completed-component fixtures, transpose/duplicates/loops/relabeling, precedence, 30,000-vertex chain then cycle. See model.md#tests for compiled controls.
Status: partial_source_derived_model; no semantic AST or general proof.

## validation

Domain: Directed adjacency rows, V<=1,000,000 and E<=1,000,000. Return sorted nonempty SCC lists ordered by minimum member, including isolates.
Observation: complete public return or named harness result, restricted to the explicitly named projection.
Operator: Vertex and checked arc caps precede endpoint errors, even an earlier malformed neighbor. Within caps first invalid neighbor in row order wins. Loops/duplicates valid; empty graph gives empty partition.
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

Epoch 2026-10-30: original code/fixtures; scientific attribution in mod.rs. Traversal O(V+E), sorting O(V log(V+1)); O(V) auxiliary storage excluding output and borrowed graph. No recursion or reverse adjacency graph. Checked edge count precedes allocation, clock bounded by V; allocation failure untyped.

### 2026-10-30 · tests

2026-10-30: 356 library unit tests pass twice debug and twice release; identical normalized names/results SHA-256 56550bbf2b2f3d4999f9d5f3ea0230762f4ef2bb54a8a850d3687daf0129943c. This file contributes five tests or their production subject. Strict algorithm all-target Clippy and narrow rustfmt pass.

Compiled control: Sort component members descending; exhaustive_mutual_reachability rejects. Log SHA-256 4b48e2d8c0b14bae37fe2e97bdf128b0d7ac0df5df896d46455dc7eb328ad1a0. Named assertion failure, not compile failure. Six mutants restored byte-for-byte before repeated positives.

Timing logs SHA-256 b3fca25c0729a0edecefdf1c4914f973f761be705db618171b159f4ac96ad7c3 and 9e8b9c417dfc0dfda48f1446825054bfca985f2aa96dea370843c1184e7b020d; Rust harness SHA-256 f78acc0796bf87f2e32ae559c46519badb9b3219b1ed449a56ca5362cb166799; prospective derivation SHA-256 41f9013788fa8bdce512d82f7922287ed2bd9c87bd84dbfca26180aa4d34a03d.

### Undated · behavior-complement

Type: behavior_complement. Subject/revision: algorithms/src/tarjan_scc/mod.rs, SHA-256 a8461f6cff64816d5346d26f0790fffbe10df806685d5c8556615e6c567ee5a2.

### Undated · reverse-reconstruction

Type: reverse_reconstruction. Subject/revision: algorithms/src/tarjan_scc/mod.rs, SHA-256 a8461f6cff64816d5346d26f0790fffbe10df806685d5c8556615e6c567ee5a2.

### Undated · active-lowlink

Type: stack_membership_boundary. Subject/revision: algorithms/src/tarjan_scc/mod.rs, SHA-256 a8461f6cff64816d5346d26f0790fffbe10df806685d5c8556615e6c567ee5a2.

### Undated · partition

Type: mutual_reachability_quotient. Subject/revision: algorithms/src/tarjan_scc/mod.rs, SHA-256 a8461f6cff64816d5346d26f0790fffbe10df806685d5c8556615e6c567ee5a2.

### Undated · canonical-order

Type: order_normalization. Subject/revision: algorithms/src/tarjan_scc/mod.rs, SHA-256 a8461f6cff64816d5346d26f0790fffbe10df806685d5c8556615e6c567ee5a2.

### Undated · transpose

Type: direction_reversal_invariance. Subject/revision: algorithms/src/tarjan_scc/mod.rs, SHA-256 a8461f6cff64816d5346d26f0790fffbe10df806685d5c8556615e6c567ee5a2.

### Undated · relabel

Type: partition_equivariance. Subject/revision: algorithms/src/tarjan_scc/mod.rs, SHA-256 a8461f6cff64816d5346d26f0790fffbe10df806685d5c8556615e6c567ee5a2.

### Undated · validation

Type: first_failure_partition. Subject/revision: algorithms/src/tarjan_scc/mod.rs, SHA-256 a8461f6cff64816d5346d26f0790fffbe10df806685d5c8556615e6c567ee5a2.
