# algorithm-maximum-cardinality-search

## descriptant

### maximum_cardinality_search mod

Original std-only Rust. Prospective model is retained beside implementation. Source citations identify algorithm provenance; no copied implementation. Exact bounded graph semantics are useful independently of any unmeasured optimization. Finite oracle checks and mutant discrimination improve confidence; groupant count alone proves no improvement.

## model

# maximum_cardinality_search mod model

## Forward

Simple undirected graph n<=256,m<=65536. Reject endpoints, loops, duplicates, asymmetry. Select unnumbered vertex with maximal number of selected neighbors, smallest vertex on ties. O(V^2+E) time and O(V^2) validation storage. Original implementation following Tarjan and Yannakakis (1984). No chordality certificate promised.

## Invariant

After t selections weights count exactly neighbors in the t-prefix. Output is a permutation; selected vertices are never selected twice.

## Cost

Simple undirected graph n<=256,m<=65536. Reject endpoints, loops, duplicates, asymmetry. Select unnumbered vertex with maximal number of selected neighbors, smallest vertex on ties. O(V^2+E) time and O(V^2) validation storage. Original implementation following Tarjan and Yannakakis (1984). No chordality certificate promised.

## nonbehavior

Type: behavior_complement. Let R be the deterministic relation specified by Forward including validation precedence and tie rules, X the declared input type and Y its Result type. Behavior complement is (X times Y) minus R. Specifically outputs violating After t selections weights count exactly neighbors in the t-prefix. Output is a permutation; selected vertices are never selected twice. are excluded; a valid-input Err or a wrong successful witness is excluded. Resource, cycle and structural rejections are determined before algorithmic output.

## compatible_inputs

Type: reverse_reconstruction. An order p admits every simple undirected graph for which at step t p[t] maximizes prefix-neighbor count with smallest-ID ties. Both empty and complete graphs yield ascending order; graph recovery is nonunique.

## prefix_certificate

Type: greedy_certificate. For each prefix P and next v, count(N(v) intersect P)>=count(N(w) intersect P) for every unselected w; equality requires v<=w. These inequalities characterize the complete deterministic order.

## row_permutation

Type: representation_invariance. Permuting each adjacency row preserves the successful order because only neighbor membership matters.

## complete_fiber

Type: constructive_preimage. Every complete graph K_n and every edgeless graph on n vertices produces [0,...,n-1]. These distinct antecedents demonstrate ambiguity for n>=2.

## isolated_extension

Type: extension_relation. Appending one isolated largest-ID vertex leaves the old order as a prefix: its weight is zero and its ID loses every zero-weight tie.

## Limits

Partial syntax omits macro expansion and type resolution. No executable approximate profile: categorical graph semantics and feasibility have no justified percentage relaxation here. Integer operations are bounded by caps; flow uses checked i128/u64 arithmetic, with Overflow defensive under current caps. No callbacks. Allocator failure is outside typed status.

## evidence

Eleven tests across five algorithms, including independent exhaustive small-domain characterization. This file belongs to maximum_cardinality_search. Two debug and two optimized runs matched normalized statuses. Strict clippy-driver --test -D warnings passed. A compiled production mutant failed a named assertion; exact bytes restored and positive rerun passed. Raw receipts: (see History). No timing or whole-system improvement claim. Source SHA-256: (see History).

### Evidence

## prospective

### Prospective model

Simple undirected graph n<=256,m<=65536. Reject endpoints, loops, duplicates, asymmetry. Select unnumbered vertex with maximal number of selected neighbors, smallest vertex on ties. O(V^2+E) time and O(V^2) validation storage. Original implementation following Tarjan and Yannakakis (1984). No chordality certificate promised.
Allocation failure is outside typed errors. Resource caps precede vertex/edge validation. No callbacks. Exact categorical output has no justified percentage surrogate; executable approximation profiles unsupported.

## History

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

Eleven tests across five algorithms, including independent exhaustive small-domain characterization. This file belongs to maximum_cardinality_search. Two debug and two optimized runs matched normalized statuses. Strict clippy-driver --test -D warnings passed. A compiled production mutant failed a named assertion; exact bytes restored and positive rerun passed. Raw receipts: $MOVA_DATA/runs/octopus-wave25-graphs-2026-10-30/maximum_cardinality_search-*.txt. No timing or whole-system improvement claim. Source SHA-256: d6910cc7bbbc852669798f09345feb08754f09ca7d9934750f419b7a1d8ccaac.

### 2026-10-30 · prospective

Heading `### Prospective model — 2026-10-30` stays in prospective as `### Prospective model`.

### 2026-11-04 · document

Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.
