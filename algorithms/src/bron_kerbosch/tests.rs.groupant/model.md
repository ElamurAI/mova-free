# algorithm-bron-kerbosch-tests

## descriptant

### algorithm-bron-kerbosch-tests: epoch memory

R is a clique and every P/X vertex neighbors all of R; P and X are disjoint. Each maximal clique is considered unless too small to improve cardinality. Equal bounds are retained so lexicographically smaller ties survive. u64 masks represent exactly vertices 0..n; n=64 uses u64::MAX without shifting by 64.

Exact exponential worst case; conservative O(n^2 2^n) time and O(n^2) auxiliary space including recursion and candidate vectors. n<=64 bounds representation only; no polynomial-time or practical-runtime promise at the cap. Validation O(n+E), E<=n(n-1)/2.

Retained observations: Every simple graph through five vertices compares independent subset clique enumeration, plus lexicographic ties, vertex 63, empty and complete 64-vertex graphs and typed validation boundaries. The compiled production mutation fails lexicographic_maximum_and_bit_63; exact original bytes precede the four positive runs. No timing comparison or speed improvement was attempted. Models describe intended relations and retain ambiguous reverse fibers; source identity and structural validation do not prove semantics.

Bron and Kerbosch (1973), Algorithm 457, Communications of the ACM 16(9). Original pivoted bitset implementation and fixtures.

## model

# algorithm-bron-kerbosch-tests

## Forward
Execute three named deterministic tests over the actual public API. Every simple graph through five vertices compares independent subset clique enumeration, plus lexicographic ties, vertex 63, empty and complete 64-vertex graphs and typed validation boundaries. Acceptance is conjunction of all named assertions, not a proof over unenumerated inputs.

## Invariant
Independent oracles do not call production graph algorithms. Fixtures are deterministic and do not modify process-global state; positive runs use exactly restored production source.

## Cost
Bounded exhaustive enumeration only: subset/closure MST oracle, short word tree checks, or subset clique oracle. These test costs are exponential in their tiny fixed domains, not production performance measurements.

## behavior_complement
Universe U is candidate implementations x named outcomes. A wrong implementation violating a sampled assertion cannot yield all-pass. The compiled production mutant targeting lexicographic_maximum_and_bit_63 is a witnessed exclusion. Unsampled defects can still pass; finite coverage is explicit.

## reverse_reconstruction
The all-pass observation admits every implementation agreeing on sampled assertions, including ones wrong elsewhere. A failing assertion constrains the mismatching observation but need not uniquely identify a defect. Named identities preserve more information than only a count.

## oracle_independence
Independent witness: Every simple graph through five vertices compares independent subset clique enumeration, plus lexicographic ties, vertex 63, empty and complete 64-vertex graphs and typed validation boundaries. The oracle uses a distinct representation or exhaustive candidate enumeration; self-roundtrip alone is insufficient.

## Provenance
Bron and Kerbosch (1973), Algorithm 457, Communications of the ACM 16(9). Original pivoted bitset implementation and fixtures.

## Limitations
Partial syntax is source-bound, not a semantic AST; macros remain opaque. Finite exhaustive small domains and mutation witnesses are not formal proof. No runtime speed claim, approximation cap, automatic substitution or unbounded resource guarantee.

## subset_objective_discrimination
brute enumerates all vertex subsets and tests every unordered pair against the supplied edge list; it does not use production neighbors, pivot selection, maximal-clique traversal or pruning. It ranks feasible subsets by cardinality then vector lexicographic order. All 1100 simple graphs through n=5 include a triangle with an isolated vertex, distinguishing maximal from maximum, and graphs with tied largest cliques, distinguishing size-only selection from the specified canonical set.

## tie_and_complement_witness
lexicographic_maximum_and_bit_63 asserts [0,2] for edges [(0,2),(1,2),(1,3)], so a later tied clique [1,2] or [1,3] is rejected; the compiled candidate>best mutant fails this named test. On the same fixed universe, complementing edges turns those size-two cliques into independent sets without changing lexicographic ranking. This specialization constrains the clique/independent-set identity mathematically; the owned tests execute clique enumeration, while the complement join belongs to the integration harness. No independent-set native coverage is inferred here.

## evidence

### Native checks

Three named tests pass twice debug and twice release with identical normalized names/outcomes. Every simple graph through five vertices compares independent subset clique enumeration, plus lexicographic ties, vertex 63, empty and complete 64-vertex graphs and typed validation boundaries.

## History

### 2026-10-30 · descriptant

2026-10-30: prospective model selected exact capped algorithms with deterministic canonical outputs. Validate a simple undirected graph of at most 64 vertices. Pivoted Bron-Kerbosch maintains clique R, possible P and excluded X; pick the ascending-first pivot maximizing |P intersect N(u)|, branch on P minus N(pivot), recurse with neighbor intersections, then transfer v from P to X. At P=X=empty compare sorted R by cardinality then lexicographic order. Prune only when |R|+|P| is strictly smaller than incumbent.

Model refinement, 2026-10-30: five typed views now distinguish the two baseline relations and three concrete specializations. New validation, observation and oracle constraints retain which claims have native witnesses and which remain source-derived. Rust bytes, partial syntax and native evidence are unchanged.

### 2026-10-30 · evidence

Compiled production mutation for bron_kerbosch fails the named assertion lexicographic_maximum_and_bit_63; production bytes restored before positive repetitions. Strict all-target algorithms Clippy and narrow rustfmt pass. Six partial syntax projections generated twice identically; full scaffold groupants checked twice. Raw logs live in $MOVA_DATA/runs/octopus-wave16-graphs-2026-10-30; evidence.json binds hashes. No live corpus, parser, teacher, GPU or full-school execution.
