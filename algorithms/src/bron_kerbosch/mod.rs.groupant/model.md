# algorithm-bron-kerbosch

## descriptant

### algorithm-bron-kerbosch: epoch memory

R is a clique and every P/X vertex neighbors all of R; P and X are disjoint. Each maximal clique is considered unless too small to improve cardinality. Equal bounds are retained so lexicographically smaller ties survive. u64 masks represent exactly vertices 0..n; n=64 uses u64::MAX without shifting by 64.

Exact exponential worst case; conservative O(n^2 2^n) time and O(n^2) auxiliary space including recursion and candidate vectors. n<=64 bounds representation only; no polynomial-time or practical-runtime promise at the cap. Validation O(n+E), E<=n(n-1)/2.

Retained observations: Every simple graph through five vertices compares independent subset clique enumeration, plus lexicographic ties, vertex 63, empty and complete 64-vertex graphs and typed validation boundaries. The compiled production mutation fails lexicographic_maximum_and_bit_63; exact original bytes precede the four positive runs. No timing comparison or speed improvement was attempted. Models describe intended relations and retain ambiguous reverse fibers; source identity and structural validation do not prove semantics.

Bron and Kerbosch (1973), Algorithm 457, Communications of the ACM 16(9). Original pivoted bitset implementation and fixtures.

## model

# algorithm-bron-kerbosch

## Forward
Validate a simple undirected graph of at most 64 vertices. Pivoted Bron-Kerbosch maintains clique R, possible P and excluded X; pick the ascending-first pivot maximizing |P intersect N(u)|, branch on P minus N(pivot), recurse with neighbor intersections, then transfer v from P to X. At P=X=empty compare sorted R by cardinality then lexicographic order. Prune only when |R|+|P| is strictly smaller than incumbent.

## Invariant
R is a clique and every P/X vertex neighbors all of R; P and X are disjoint. Each maximal clique is considered unless too small to improve cardinality. Equal bounds are retained so lexicographically smaller ties survive. u64 masks represent exactly vertices 0..n; n=64 uses u64::MAX without shifting by 64.

## Cost
Exact exponential worst case; conservative O(n^2 2^n) time and O(n^2) auxiliary space including recursion and candidate vectors. n<=64 bounds representation only; no polynomial-time or practical-runtime promise at the cap. Validation O(n+E), E<=n(n-1)/2.

## behavior_complement
U consists of (n,simple edge list,candidate Result) tuples. For valid inputs, noncliques, smaller cliques, unsorted results, or lexicographically later maximum cliques are absent. Invalid endpoints, loops, reversed duplicates and cap excess cannot yield Ok, under documented first-error ordering.

## reverse_reconstruction
Fix sorted output C. Its fiber is every valid graph on the fixed vertex universe in which C is a maximum clique and no lexicographically smaller equal-cardinality clique exists. Missing edges outside C are not recoverable; many distinct graphs share C. Empty C is reachable only for n=0.

## complement_independence
For fixed vertex universe, C is a clique in G iff it is independent in the loopless complement of G. Maximum cardinality and lexicographic ordering are preserved by this exact graph complement. This is a specialized relation, not an implemented independent-set solver or a faster exact bound.

## Provenance
Bron and Kerbosch (1973), Algorithm 457, Communications of the ACM 16(9). Original pivoted bitset implementation and fixtures.

## Limitations
Partial syntax is source-bound, not a semantic AST; macros remain opaque. Finite exhaustive small domains and mutation witnesses are not formal proof. No runtime speed claim, approximation cap, automatic substitution or unbounded resource guarantee.

## maximal_maximum_boundary
A terminal P=X=empty certifies only that R is maximal: no vertex can extend it. The result is maximum only after comparing all unpruned terminal candidates by cardinality and lexicographic order. For a triangle on vertices 1,2,3 plus isolated vertex 0, {0} is maximal but the returned maximum is [1,2,3]. The bound |R|+|P|<|best| is deliberately strict; equality must remain searchable because an equal-size candidate can improve the lexicographic result. Pivot choice changes search order, not the target objective.

## simple_graph_failure_order
Validation checks n>64 first, then E>n(n-1)/2 before allocating masks or checking endpoints. For each surviving input edge it checks u then v range, self-loop, and whether the adjacency bit already exists. Reversing an earlier edge is a duplicate because both directions were recorded. Thus n=0 with [(0,0)] returns TooManyEdges, not InvalidEndpoint or SelfLoop. On n=64 valid indices include 63 and all-mask construction uses u64::MAX; no shift by 64 occurs. The complete cap graph and vertex-63 witnesses exercise this representation boundary.

## evidence

### Native checks

Three named tests pass twice debug and twice release with identical normalized names/outcomes. Every simple graph through five vertices compares independent subset clique enumeration, plus lexicographic ties, vertex 63, empty and complete 64-vertex graphs and typed validation boundaries.

## History

### 2026-10-30 · descriptant

2026-10-30: prospective model selected exact capped algorithms with deterministic canonical outputs. Validate a simple undirected graph of at most 64 vertices. Pivoted Bron-Kerbosch maintains clique R, possible P and excluded X; pick the ascending-first pivot maximizing |P intersect N(u)|, branch on P minus N(pivot), recurse with neighbor intersections, then transfer v from P to X. At P=X=empty compare sorted R by cardinality then lexicographic order. Prune only when |R|+|P| is strictly smaller than incumbent.

Model refinement, 2026-10-30: five typed views now distinguish the two baseline relations and three concrete specializations. New validation, observation and oracle constraints retain which claims have native witnesses and which remain source-derived. Rust bytes, partial syntax and native evidence are unchanged.

### 2026-10-30 · evidence

Compiled production mutation for bron_kerbosch fails the named assertion lexicographic_maximum_and_bit_63; production bytes restored before positive repetitions. Strict all-target algorithms Clippy and narrow rustfmt pass. Six partial syntax projections generated twice identically; full scaffold groupants checked twice. Raw logs live in $MOVA_DATA/runs/octopus-wave16-graphs-2026-10-30; evidence.json binds hashes. No live corpus, parser, teacher, GPU or full-school execution.
