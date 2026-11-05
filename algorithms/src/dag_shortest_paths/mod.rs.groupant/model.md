# algorithm-dag-shortest-paths

## descriptant

### algorithm-dag-shortest-paths descriptant

The combiner joins the authoritative source hash and partial syntax with forward path semantics, first behavior complement and second result-preimage relation, invariants, cost and executed finite evidence. Agreement preserves a bounded correspondence claim; it does not turn AST syntax into a proof of Rust behavior. The implementation/test boundary is explicit: witness paths certify distances and finite oracle passes constrain implementations only on observed inputs.

The mathematical mechanism is DAG shortest-path relaxation, Cormen et al., Introduction to Algorithms, 3rd edition, section 24.2. Endpoint projection deliberately composes with the existing deterministic topological_sort.

Retained observations: disconnected invalid input remains observable; distance vectors have many compatible graph antecedents; allocation failure and defensive overflow branches have no finite fixture evidence. Signed full-width weights require widened sums even though each individual edge is i64.

## model

# algorithm-dag-shortest-paths

## Domain

Finite materialized directed adjacency lists of (usize,i64), usize source, and Result<Vec<Option<i128>>,Error> observations. Successful allocation and usize width at most 64 for the stated signed bound; arbitrary labels, parallel edges and disconnected components.

## Forward

F(G,s) rejects s>=n first; then projects endpoints and reuses topological_sort. That helper rejects the first invalid endpoint in row order before globally rejecting cycles. On a DAG, initialize source to zero and others None, visit the helper's topological order, and strictly relax outgoing edges of finite labels using checked i128 addition. Return shortest signed path sums and None for unreachable vertices. Cycles in unreachable components still reject.

## Invariant

The endpoint projection preserves every edge, including multiplicity. The reused sorter returns an order with every edge directed forward or a global error. At each processed vertex, every predecessor is already processed, so its finite label is the minimum of all source paths to it. Paths in a DAG have at most n-1 edges. With usize<=64 and i64 weights each path sum and scanned candidate has magnitude below 2^127, so checked i128 overflow is defensive; larger architecture widths would require retaining the overflow outcome.

## Cost

Endpoint projection O(V+E) time/space; reused binary-heap Kahn sort O(E+V log(V+1)) time; relaxation O(V+E). Overall O(E+V log(V+1)) time and O(V+E) extra space. No timing or approximation gain claimed.

## nonbehavior

Type: behavior_complement.

For U of declared input/output pairs, U\\graph(F) excludes success on any cyclic component, wrong endpoint/source error priority, nonminimal signed path sums, finite labels on unreachable vertices and vector-length mismatches. Negative edges are permitted on DAGs, so rejecting negativity alone is also excluded behavior.

## distance_fibers

Type: reverse_reconstruction.

For Ok(d), reconstruct the set of valid acyclic G and s with d[s]=0, reachability equal to the Some support, d[v]<=d[u]+w for each edge from a finite u, and a realizing source path of total d[v] for each finite v. These conditions characterize the preimage by telescoping path inequalities and witness paths. Multiple graphs, parallel edges and dominated edges share the same vector. Error fibers preserve validation priority; Cycle requires valid source/endpoints and a cycle anywhere.

## relabeling

Type: equivariance_relation.

A vertex permutation p maps D'[p(v)]=D[v] for valid DAGs. The sorter's tie order may change, but no predecessor relation or path sum does. Invalid-input error payloads require separate ordered validation modeling.

## potential_transform

Type: algebraic_relation.

Let h:V->Z and replace w(u,v) by w(u,v)+h(v)-h(u), provided transformed weights fit i64 and arithmetic stays in i128. Every path telescopes, hence D'[v]=D[v]+h(v)-h(s) for reachable v, with reachability unchanged. This justifies signed reweighting without assuming nonnegative edges.

## redundant_edges

Type: closure_relation.

Adding a duplicate edge, or any edge that preserves acyclicity and satisfies D[v]<=D[u]+w for finite endpoints, cannot improve distances. Acyclicity is essential: adding a distance-neutral reverse edge may create a globally rejected cycle.

## optimality_certificate

Type: certificate_relation.

For a valid DAG, source zero, reachability closure, all edge inequalities and a realizing path for every finite label certify exact minima. No claim of uniquely recovering paths: equal-cost alternatives and parallel edges can share the output.

## global_error_order

Type: error_partition.

InvalidSource precedes the first row-order InvalidEdge, which precedes Cycle. Negative/zero/positive cycles all reject, even disconnected ones. DistanceOverflow is retained defensively but cannot be exercised with practical materialized graphs on <=64-bit usize; full-width weight tests cover widening, not overflow rejection.

## weight_monotonicity

Type: order_relation.

For fixed acyclic topology, increasing any weights within i64 leaves reachability unchanged and cannot decrease finite shortest distances. Uniform multiplication by a nonnegative integer k preserves D'=kD provided all transformed weights/sums fit; k=0 makes all reachable distances zero.

## Limits

Source-derived mathematical models with finite executable correspondence, not formal Rust proofs. Partial syntax omits macro expansion, type resolution and semantic AST. Exact production implementation; no approximate surrogate or numerical error guarantee is claimed. Resource exhaustion is outside the successful-call model.

## evidence

### Evidence

The endpoint projection preserves every edge, including multiplicity. The reused sorter returns an order with every edge directed forward or a global error. At each processed vertex, every predecessor is already processed, so its finite label is the minimum of all source paths to it. Paths in a DAG have at most n-1 edges. With usize<=64 and i64 weights each path sum and scanned candidate has magnitude below 2^127, so checked i128 overflow is defensive; larger architecture widths would require retaining the overflow outcome.

These observations establish finite correspondence only. No timing superiority, exhaustive machine-domain proof, approximation error budget or overflow-error test coverage is claimed.

## prospective

## History

### 2026-10-30 · descriptant

2026-10-30: F(G,s) rejects s>=n first; then projects endpoints and reuses topological_sort. That helper rejects the first invalid endpoint in row order before globally rejecting cycles. On a DAG, initialize source to zero and others None, visit the helper's topological order, and strictly relax outgoing edges of finite labels using checked i128 addition. Return shortest signed path sums and None for unreachable vertices. Cycles in unreachable components still reject.

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

Source SHA-256: 4e2600f35317074040e42a5ba47e63848dad58793208226f422defaf065cf4a7.

Six tests for this algorithm passed in two debug and two release full-crate runs (179 tests each); normalized named outcomes were identical. Strict all-target Clippy and formatting checks passed. Source syntax was projected twice and compared byte-for-byte. A compiled production mutant was rejected by the independent finite oracle assertion. External run bundle: MOVA_DATA/runs/octopus-wave4-graphs-2026-10-30 (debug1/2, release1/2, normalized outcomes, clippy, format, dag_shortest_paths-mutant and syntax logs; validate.rs records commands).

### 2026-10-30 · prospective

#### Symbolic expectations and scope

2026-10-30: Expected exact path semantics and Endpoint projection O(V+E) time/space; reused binary-heap Kahn sort O(E+V log(V+1)) time; relaxation O(V+E). Overall O(E+V log(V+1)) time and O(V+E) extra space. No timing or approximation gain claimed. These are mathematical expectations, not calibrated performance forecasts. The initial implementation plan selected existing topological ordering to reject cycles globally before relaxation. Finite tests subsequently check the stated fixtures; no runtime benchmark or probabilistic approximation is proposed.
