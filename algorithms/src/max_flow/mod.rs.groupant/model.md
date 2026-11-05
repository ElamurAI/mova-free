# algorithm-max-flow

## descriptant

### max_flow mod.rs

Original std-only implementation and adjacent bounded checks. Classical concept: Edmonds-Karp shortest augmenting paths, Edmonds and Karp (1972). No source copied.

The prospective artifact preceded implementation. The runtime checks refine this mathematical description; no timing claim, universal proof, macro expansion or semantic AST is supplied. Executable approximation is intentionally absent: these are exact APIs, with no justified lossy metric or substitution budget.

## model

# max_flow mod.rs model

## Forward

Validate source then sink bounds, distinct terminals, then all edges in original order. Build a separate forward/reverse pair per edge (reverse index xor 1), preserving loops and duplicates. Repeated full FIFO BFS in insertion order selects shortest residual paths. Augment by path minimum; on no path return u128 total, reverse residuals in original order and final reachable vertices. Exact selected output depends on ordered input, not merely optimality.

## Invariant

Every paired residual sum equals its original capacity; each reverse residual is a feasible edge flow. Path augmentation preserves internal conservation and increases source net flow by amount. Final reachable S excludes sink; every S-to-outside edge is saturated and every outside-to-S edge carries zero. Therefore flow value equals cut capacity and is maximum. Visited-once BFS prevents loops from joining paths, so loop flows remain zero.

## Cost

O(V+E) memory. O(V+E+VE(V+E)) conservative time: Edmonds-Karp has O(VE) augmentations; full BFS and array initialization cost O(V+E) each. The additive term covers E=0. No runtime timing claim or capacity-dependent iteration count.

## nonbehavior

Type: behavior_complement. Let X be ordered finite graphs with usize terminals and u64 capacities, Y typed Result<Flow,Error>, and U=X times Y under sufficient allocation. R is the exact deterministic relation in Forward, including validation precedence. U minus R excludes wrong validation, infeasible/nonmaximum certificates, and alternate feasible optima not chosen by the specified traversal. This is a source-derived exact target with only bounded correspondence evidence, not a proof.

## compatible_inputs

Type: reverse_reconstruction. For a successful (value,flows,cut), compatible inputs must have matching lengths, capacities at least flows, conservation at every nonterminal, net terminal flow equal value, and cut capacity equal value. These are necessary filters; the complete preimage additionally requires Forward to select exactly those flows and cut. Capacities and ordered graphs are generally not uniquely recovered. Error preimages follow the first failing validation or checked arithmetic site.

## cut_duality

Type: optimality_certificate. For any feasible output f and separating set S, value(f)<=sum of capacities leaving S. Equality certifies maximum flow. Enumerated cut minima and independent conservation checks test this certificate for all 4096 directed simple unit-capacity graphs on four vertices.

## residual_obstruction

Type: augmenting_path_obstruction. A feasible flow is maximum exactly when no positive residual source-sink path remains. Reverse residual arcs permit cancellation: the seven-edge fixture requires replacing edge 1-to-3 flow to achieve value two. Setting reverse updates to zero violates the capacity/certificate relation.

## capacity_scaling

Type: homogeneity. For positive integer k with every multiplied capacity in u64 range, multiplying capacities by k preserves BFS decisions, scales every flow and total by k and preserves residual reachability. Zero factor preserves value but may change reachability. Tests check total scaling by three for 25 length-two paths.

## transpose

Type: terminal_duality. Reverse all directed edges and exchange source and sink; feasible flows biject and maximum value is preserved. Selected flow vectors/cuts need not be invariant. Tests compare values for all 4096 directed unit graphs.

## parallel_capacity

Type: numeric_boundary. Two parallel source-sink edges of capacity u64::MAX carry total 2*(2^64-1), representable in u128 but not u64. Antiparallel edges have distinct residual pairs, not merged capacities; loops remain zero. The numeric fixture checks exact indexed values and conservation.

## direct_preimage

Type: constructive_preimage. For c in u64, the one-edge two-vertex graph 0-to-1 with capacity c maps to (c,[c],[true,false]), including c=0. This constructs one exact antecedent. Adding isolated vertices preserves only value and edge-flow projection; full cut shape grows. Bounded tests c=0..9 retain this distinction and do not assert unique graph reconstruction.

## evidence

### Evidence

For c in u64, the one-edge two-vertex graph 0-to-1 with capacity c maps to (c,[c],[true,false]), including c=0. This constructs one exact antecedent. Adding isolated vertices preserves only value and edge-flow projection; full cut shape grows. Bounded tests c=0..9 retain this distinction and do not assert unique graph reconstruction.

#### Measured outcome

All four focused tests passed in each of two debug and two release runs; named assertion transcripts were byte-identical after timing removal. Strict all-target Clippy with -D warnings and cargo fmt --check passed. Each source syntax projection was generated twice and was byte-identical. Each isolated production mutant compiled successfully and failed the named equality assertion.

Artifact SHA-256 bindings (external run label above):

## History

### 2026-10-30 · descriptant

2026-10-30 prospective epoch: O(V+E) memory. O(V+E+VE(V+E)) conservative time: Edmonds-Karp has O(VE) augmentations; full BFS and array initialization cost O(V+E) each. The additive term covers E=0. No runtime timing claim or capacity-dependent iteration count.

### 2026-10-30 · evidence

2026-10-30: planned and then measured in the dedicated graph-optimization run. See external run label octopus-wave5-graph-optimization-2026-10-30: prospective.md, debug/release twice normalized, strict-clippy.log, format.log, syntax twice and compiled production mutant logs. Four tests per algorithm. Bounded results are not universal verification.

- max_flow-debug-1.log: f142efe017fa72285eb1e5b9d7317978ca17f94fedc3173273076bba30cfe785

- max_flow-debug-2.log: 078399c2ec1e45bd6d39cb6add9e941a7347afe38fd405c53a49a2e2a999081e

- max_flow-release-1.log: b1150dabd69f362a686fa82f40c7510e321fcca1826e40647cd72b4a9ad848b3

- max_flow-release-2.log: 9c03e7af88a53b370d15febdcc3a59326f21f5f0283d4eaa59f9c5606024fbf6

- max_flow-mutant-compile.log: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

- max_flow-mutant-rejected.log: 0f04d8aff7a839221de2a2eee224c3e91ed4fb148836759654348be71a723338

- strict-clippy.log: fba0904fdec91250d0aafb0674967baaad449a27cd510117ea1fb9defea286f1

- format.log: e3b0c44298fc1c149afbf4c8996fb92427ae41e4649b934ca495991b7852b855

- prospective.md: c041f70a6de4c683ebc42555609c330a5d55d45d83086f796a86ba4562c996c0

### 2026-11-04 · document

Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.
