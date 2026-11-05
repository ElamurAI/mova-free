# algorithm-zero-one-bfs

## descriptant

### algorithm-zero-one-bfs descriptant

The combiner joins the authoritative source hash and partial syntax with forward path semantics, first behavior complement and second result-preimage relation, invariants, cost and executed finite evidence. Agreement preserves a bounded correspondence claim; it does not turn AST syntax into a proof of Rust behavior. The implementation/test boundary is explicit: witness paths certify distances and finite oracle passes constrain implementations only on observed inputs.

The mathematical mechanism is binary-weight deque specialization of Dijkstra, as discussed by cp-algorithms, 0-1 BFS; implementation and fixtures are original, with no copied source.

Retained observations: disconnected invalid input remains observable; distance vectors have many compatible graph antecedents; allocation failure and defensive overflow branches have no finite fixture evidence. The reversed-relaxation control on cyclic inputs did not terminate promptly and was stopped; the final zero-increment control compiles and produces an actual assertion failure.

## model

# algorithm-zero-one-bfs

## Domain

Finite materialized directed adjacency lists of (usize,u8), a usize source, and Result<Vec<Option<usize>>,Error> observations. Allocation succeeds; immutable inputs. Binary valid graphs allow loops, parallel edges and cycles.

## Forward

F(G,s) checks s<n first; then scans rows and edges in stored order, checking endpoint before weight on each edge. Return the first InvalidSource/InvalidEdge/InvalidWeight. On valid graphs return D[v]=min of binary weight sums over s-to-v paths, None if no path. Empty path gives D[s]=0. Deque entries carry distance snapshots; stale entries are skipped; strict improvements push front for zero, back for one. Checked addition retains a defensive DistanceOverflow branch.

## Invariant

Every finite tentative label is a realized walk length. Fresh deque pops have nondecreasing distances, because pending candidate distances differ by at most one and zero improvements go to the front. A fresh pop settles its vertex; all later snapshots for it are stale, so each outgoing row is scanned once. Strict updates prevent zero-cycle churn. Removing nonnegative cycles gives a shortest simple path of at most n-1 edges. Each scanned candidate is at most n, fitting usize for materialized graphs.

## Cost

Validation O(V+E), search O(V+E) time; O(V+E) auxiliary space conservatively counts stale snapshots, each charged to a strict edge relaxation. Symbolic bounds; no measured runtime improvement claimed.

## nonbehavior

Type: behavior_complement.

In universe U of declared input/output pairs, the complement U\\graph(F) contains accepted invalid rows, wrong first errors, finite labels for unreachable vertices, nonminimal reachable labels, nonzero source labels, and length-mismatched vectors. This defines semantic nonbehavior, not merely the negative test list.

## distance_fibers

Type: reverse_reconstruction.

For an Ok vector d of length n, compatible inputs are all valid binary G and s<n whose reachability equals the Some support and whose shortest-path lengths equal d. A sufficient and necessary certificate is d[s]=0, no edge from finite to None, d[v]<=d[u]+w on every finite-source edge, and for each finite v a witnessed s-to-v path of sum d[v]. Tight edges alone are insufficient because disconnected zero cycles could receive fake finite labels. Invalid-error preimages are inputs whose first failing check has exactly the reported payload. Recovery is generally many-to-one.

## relabeling

Type: equivariance_relation.

For any vertex permutation p of [0,n), replacing every endpoint u by p(u), moving rows likewise, and source by p(s) gives D'[p(v)]=D[v] on valid graphs. Stored edge order can alter validation error priority for invalid inputs, so the relation is restricted to valid graphs.

## redundant_edges

Type: closure_relation.

Adding an edge (u,v,w) with finite D[u], finite D[v] and D[v]<=D[u]+w preserves all shortest distances: substitute the old distance inequality through any new-edge path. A duplicate edge always preserves distances. An edge from an unreachable vertex also preserves distances unless some other edit makes that vertex reachable.

## triangle_certificate

Type: certificate_relation.

Every finite-source edge satisfies finite D[v] and D[v]<=D[u]+w; every finite target has a realizing path. Combined with d[s]=0 these witness both upper and lower bounds by telescoping along arbitrary paths.

## unit_specialization

Type: boundary_specialization.

If every weight is one, output equals directed BFS hop distance. If every weight is zero, every reachable vertex has zero distance and others None. Isolated source has only its zero label; self loops cannot improve a nonnegative shortest path.

## validation_partition

Type: error_partition.

Source errors dominate all edge faults. For a valid source the earliest row-position edge fault wins, with endpoint checked before weight within that edge. Unreachable invalid edges are still faults. DistanceOverflow is defensive and not expected on materialized graphs by the n-bound; tests do not claim branch coverage.

## weight_monotonicity

Type: order_relation.

For a fixed graph topology and source, increasing any subset of weights from zero to one preserves reachability and cannot decrease any finite shortest distance. Compare only valid binary weights; topology changes are outside this relation.

## Limits

Source-derived mathematical models with finite executable correspondence, not formal Rust proofs. Partial syntax omits macro expansion, type resolution and semantic AST. Exact production implementation; no approximate surrogate or numerical error guarantee is claimed. Resource exhaustion is outside the successful-call model.

## evidence

### Evidence

Every finite tentative label is a realized walk length. Fresh deque pops have nondecreasing distances, because pending candidate distances differ by at most one and zero improvements go to the front. A fresh pop settles its vertex; all later snapshots for it are stale, so each outgoing row is scanned once. Strict updates prevent zero-cycle churn. Removing nonnegative cycles gives a shortest simple path of at most n-1 edges. Each scanned candidate is at most n, fitting usize for materialized graphs.

These observations establish finite correspondence only. No timing superiority, exhaustive machine-domain proof, approximation error budget or overflow-error test coverage is claimed.

## prospective

## History

### 2026-10-30 · descriptant

2026-10-30: F(G,s) checks s<n first; then scans rows and edges in stored order, checking endpoint before weight on each edge. Return the first InvalidSource/InvalidEdge/InvalidWeight. On valid graphs return D[v]=min of binary weight sums over s-to-v paths, None if no path. Empty path gives D[s]=0. Deque entries carry distance snapshots; stale entries are skipped; strict improvements push front for zero, back for one. Checked addition retains a defensive DistanceOverflow branch.

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

Source SHA-256: 494c1e36a75d80d042c09c02a4bb63f3a8e3f8689dc15db34b3b95666589b80f.

Six tests for this algorithm passed in two debug and two release full-crate runs (179 tests each); normalized named outcomes were identical. Strict all-target Clippy and formatting checks passed. Source syntax was projected twice and compared byte-for-byte. A compiled production mutant was rejected by the independent finite oracle assertion. External run bundle: MOVA_DATA/runs/octopus-wave4-graphs-2026-10-30 (debug1/2, release1/2, normalized outcomes, clippy, format, zero_one_bfs-mutant and syntax logs; validate.rs records commands).

### 2026-10-30 · prospective

#### Symbolic expectations and scope

2026-10-30: Expected exact path semantics and Validation O(V+E), search O(V+E) time; O(V+E) auxiliary space conservatively counts stale snapshots, each charged to a strict edge relaxation. Symbolic bounds; no measured runtime improvement claimed. These are mathematical expectations, not calibrated performance forecasts. The initial implementation plan selected deque snapshots and strict improvement to avoid stale scans and zero-cycle churn. Finite tests subsequently check the stated fixtures; no runtime benchmark or probabilistic approximation is proposed.
