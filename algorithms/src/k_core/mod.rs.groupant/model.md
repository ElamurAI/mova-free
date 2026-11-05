# algorithm-k-core

## descriptant

### k_core mod

Original std-only Rust. Prospective model is retained beside implementation. Source citations identify algorithm provenance; no copied implementation. Exact bounded graph semantics are useful independently of any unmeasured optimization. Finite oracle checks and mutant discrimination improve confidence; groupant count alone proves no improvement.

## model

# k_core mod model

## Forward

Simple undirected graph n<=256,m<=65536. Repeatedly delete vertices with live degree below k; return ascending survivors, unique maximal induced minimum-degree-k subgraph. FIFO deletion, each edge visited O(1) after O(V^2+E) validation. O(V^2+E) time/storage. k=0 retains all. Seidman (1983).

## Invariant

Deleted vertices cannot belong to any minimum-degree-k induced subgraph. For live vertices degree equals count of live neighbors; survivor set is maximal.

## Cost

Simple undirected graph n<=256,m<=65536. Repeatedly delete vertices with live degree below k; return ascending survivors, unique maximal induced minimum-degree-k subgraph. FIFO deletion, each edge visited O(1) after O(V^2+E) validation. O(V^2+E) time/storage. k=0 retains all. Seidman (1983).

## nonbehavior

Type: behavior_complement. Let R be the deterministic relation specified by Forward including validation precedence and tie rules, X the declared input type and Y its Result type. Behavior complement is (X times Y) minus R. Specifically outputs violating Deleted vertices cannot belong to any minimum-degree-k induced subgraph. For live vertices degree equals count of live neighbors; survivor set is maximal. are excluded; a valid-input Err or a wrong successful witness is excluded. Resource, cycle and structural rejections are determined before algorithmic output.

## compatible_inputs

Type: reverse_reconstruction. Given survivor set S, compatible graphs make G[S] minimum degree at least k and every vertex outside S eventually peelable. For k=1, a three-vertex path and a triangle differ but both can yield all vertices when no isolated vertices. Errors identify violated validation predicates.

## subset_certificate

Type: maximality_certificate. Output equals union of all vertex subsets whose induced minimum degree is at least k. Union preserves the minimum-degree predicate and gives unique maximality.

## nested_cores

Type: parameter_monotonicity. For k<=l, core_l(G) is a subset of core_k(G). core_0(G) is every vertex; k>=n gives empty on simple n-vertex graphs.

## idempotence

Type: fixed_point. Restrict G to returned vertices, retaining their labels. Applying k-peeling again deletes none.

## edge_extension

Type: monotone_extension. Adding an absent undirected edge cannot remove vertices from a k-core; the prior induced feasible set still witnesses membership.

## Limits

Partial syntax omits macro expansion and type resolution. No executable approximate profile: categorical graph semantics and feasibility have no justified percentage relaxation here. Integer operations are bounded by caps; flow uses checked i128/u64 arithmetic, with Overflow defensive under current caps. No callbacks. Allocator failure is outside typed status.

## evidence

Eleven tests across five algorithms, including independent exhaustive small-domain characterization. This file belongs to k_core. Two debug and two optimized runs matched normalized statuses. Strict clippy-driver --test -D warnings passed. A compiled production mutant failed a named assertion; exact bytes restored and positive rerun passed. Raw receipts: (see History). No timing or whole-system improvement claim. Source SHA-256: (see History).

### Evidence

## prospective

### Prospective model

Simple undirected graph n<=256,m<=65536. Repeatedly delete vertices with live degree below k; return ascending survivors, unique maximal induced minimum-degree-k subgraph. FIFO deletion, each edge visited O(1) after O(V^2+E) validation. O(V^2+E) time/storage. k=0 retains all. Seidman (1983).
Allocation failure is outside typed errors. Resource caps precede vertex/edge validation. No callbacks. Exact categorical output has no justified percentage surrogate; executable approximation profiles unsupported.

## History

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

Eleven tests across five algorithms, including independent exhaustive small-domain characterization. This file belongs to k_core. Two debug and two optimized runs matched normalized statuses. Strict clippy-driver --test -D warnings passed. A compiled production mutant failed a named assertion; exact bytes restored and positive rerun passed. Raw receipts: $MOVA_DATA/runs/octopus-wave25-graphs-2026-10-30/k_core-*.txt. No timing or whole-system improvement claim. Source SHA-256: 37a416e1386fa1f72fcf9fa8771e88af0a2a98d0bf51b94df3605c8462065707.

### 2026-10-30 · prospective

Heading `### Prospective model — 2026-10-30` stays in prospective as `### Prospective model`.

### 2026-11-04 · document

Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.
