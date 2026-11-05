# algorithm-boruvka

## descriptant

### algorithm-boruvka: epoch memory

Every selected edge joins distinct current components. Cut-minimal edges are safe; each successful round reduces every nonisolated remaining component population by at least a factor two. |F|=n-components. Sum uses i128 and at most 999999 signed i64 terms, so no arithmetic overflow.

O((V+E) log(V+1) alpha(V)) time including component scans and final O(V log(V+1)) sort; O(V) auxiliary plus output. V<=1000000, E<=4000000. Resource caps reject before allocation/endpoint scans. Allocation exhaustion is not a recoverable API error.

Retained observations: 4096 signed/absent graphs on four vertices compare independent subset enumeration and transitive closure, plus exact signed extremes, parallel ties, loops, isolates, invalid endpoints and both caps. The compiled production mutation fails signed_wide_total_and_ties; exact original bytes precede the four positive runs. No timing comparison or speed improvement was attempted. Models describe intended relations and retain ambiguous reverse fibers; source identity and structural validation do not prove semantics.

Boruvka (1926), minimum spanning tree cut algorithm. Implementation and fixtures are original.

## model

# algorithm-boruvka

## Forward
Validated signed multigraph G maps to one minimum spanning forest F. Repeatedly choose each DSU component's least outgoing (weight,input index), union candidates in ascending root order, and stop at an unchanged partition; sort selected original indices. Minimize sum of edge weights separately in every connected component; retain isolates. The distinct total order resolves ties.

## Invariant
Every selected edge joins distinct current components. Cut-minimal edges are safe; each successful round reduces every nonisolated remaining component population by at least a factor two. |F|=n-components. Sum uses i128 and at most 999999 signed i64 terms, so no arithmetic overflow.

## Cost
O((V+E) log(V+1) alpha(V)) time including component scans and final O(V log(V+1)) sort; O(V) auxiliary plus output. V<=1000000, E<=4000000. Resource caps reject before allocation/endpoint scans. Allocation exhaustion is not a recoverable API error.

## behavior_complement
U is all (n,ordered signed edges,candidate Result) triples. On valid graphs, outputs containing loops, cycles, a nonspanning partition, wrong total, or greater-than-minimum weight are excluded; alternative equal-weight forests are also excluded if their index list differs from deterministic rounds. On invalid inputs, Ok is excluded; the first typed validation error is fixed.

## reverse_reconstruction
Fix observed F. The fiber includes every ordered multigraph whose deterministic rounds select those indices, whose selected weights sum to F.total_weight, and whose connectivity gives F.components. Unselected heavier edges, loops, and redistributed selected weights may be invisible. Even fixing n and selected indices does not reconstruct edge endpoints or all weights.

## cut_tie_order
Holding endpoints and weights fixed, reindexing equal-weight edges may change the chosen index forest. In a two-vertex parallel pair with equal weights the first input edge wins; endpoint direction is immaterial. Ordering is an API contract, not uniqueness of the unconstrained minimum forest.

## Provenance
Boruvka (1926), minimum spanning tree cut algorithm. Implementation and fixtures are original.

## Limitations
Partial syntax is source-bound, not a semantic AST; macros remain opaque. Finite exhaustive small domains and mutation witnesses are not formal proof. No runtime speed claim, approximation cap, automatic substitution or unbounded resource guarantee.

## validation_precedence
Fix an ordered edge list and n. If n>1000000, TooManyVertices wins even if edge count or labels are invalid. Otherwise E>4000000 gives TooManyEdges before inspecting endpoints. Otherwise the first invalid endpoint in edge-index order, u before v, determines InvalidEndpoint; a self-loop outside 0..n is rejected rather than silently ignored. Errors therefore identify a prefix boundary and do not reconstruct later edges. Witnesses boundary_and_validation cover caps and invalid endpoints; simultaneous-invalid precedence is source-derived, not separately exhaustively tested.

## forest_observation_loss
Fix n and the full returned (sorted edge_indices,total_weight,components). The selected indices identify original input occurrences, not normalized endpoint pairs; reversing endpoints leaves their identity intact. Projecting further to only (total_weight,components) loses both selected indices and tree shape. For n=3 the unit-weight paths [(0,1),(1,2)] and [(0,2),(1,2)] both yield weight 2 and one component despite different trees. signed_wide_total_and_ties separately constrains index identity [0,1], count 2 and signed sum, so matching only scalar cost is insufficient.

## evidence

### Native checks

Three named tests pass twice debug and twice release with identical normalized names/outcomes. 4096 signed/absent graphs on four vertices compare independent subset enumeration and transitive closure, plus exact signed extremes, parallel ties, loops, isolates, invalid endpoints and both caps.

## History

### 2026-10-30 · descriptant

2026-10-30: prospective model selected exact capped algorithms with deterministic canonical outputs. Validated signed multigraph G maps to one minimum spanning forest F. Repeatedly choose each DSU component's least outgoing (weight,input index), union candidates in ascending root order, and stop at an unchanged partition; sort selected original indices. Minimize sum of edge weights separately in every connected component; retain isolates. The distinct total order resolves ties.

Model refinement, 2026-10-30: five typed views now distinguish the two baseline relations and three concrete specializations. New validation, observation and oracle constraints retain which claims have native witnesses and which remain source-derived. Rust bytes, partial syntax and native evidence are unchanged.

### 2026-10-30 · evidence

Compiled production mutation for boruvka fails the named assertion signed_wide_total_and_ties; production bytes restored before positive repetitions. Strict all-target algorithms Clippy and narrow rustfmt pass. Six partial syntax projections generated twice identically; full scaffold groupants checked twice. Raw logs live in $MOVA_DATA/runs/octopus-wave16-graphs-2026-10-30; evidence.json binds hashes. No live corpus, parser, teacher, GPU or full-school execution.
