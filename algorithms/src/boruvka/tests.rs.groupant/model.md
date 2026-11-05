# algorithm-boruvka-tests

## descriptant

### algorithm-boruvka-tests: epoch memory

Every selected edge joins distinct current components. Cut-minimal edges are safe; each successful round reduces every nonisolated remaining component population by at least a factor two. |F|=n-components. Sum uses i128 and at most 999999 signed i64 terms, so no arithmetic overflow.

O((V+E) log(V+1) alpha(V)) time including component scans and final O(V log(V+1)) sort; O(V) auxiliary plus output. V<=1000000, E<=4000000. Resource caps reject before allocation/endpoint scans. Allocation exhaustion is not a recoverable API error.

Retained observations: 4096 signed/absent graphs on four vertices compare independent subset enumeration and transitive closure, plus exact signed extremes, parallel ties, loops, isolates, invalid endpoints and both caps. The compiled production mutation fails signed_wide_total_and_ties; exact original bytes precede the four positive runs. No timing comparison or speed improvement was attempted. Models describe intended relations and retain ambiguous reverse fibers; source identity and structural validation do not prove semantics.

Boruvka (1926), minimum spanning tree cut algorithm. Implementation and fixtures are original.

## model

# algorithm-boruvka-tests

## Forward
Execute three named deterministic tests over the actual public API. 4096 signed/absent graphs on four vertices compare independent subset enumeration and transitive closure, plus exact signed extremes, parallel ties, loops, isolates, invalid endpoints and both caps. Acceptance is conjunction of all named assertions, not a proof over unenumerated inputs.

## Invariant
Independent oracles do not call production graph algorithms. Fixtures are deterministic and do not modify process-global state; positive runs use exactly restored production source.

## Cost
Bounded exhaustive enumeration only: subset/closure MST oracle, short word tree checks, or subset clique oracle. These test costs are exponential in their tiny fixed domains, not production performance measurements.

## behavior_complement
Universe U is candidate implementations x named outcomes. A wrong implementation violating a sampled assertion cannot yield all-pass. The compiled production mutant targeting signed_wide_total_and_ties is a witnessed exclusion. Unsampled defects can still pass; finite coverage is explicit.

## reverse_reconstruction
The all-pass observation admits every implementation agreeing on sampled assertions, including ones wrong elsewhere. A failing assertion constrains the mismatching observation but need not uniquely identify a defect. Named identities preserve more information than only a count.

## oracle_independence
Independent witness: 4096 signed/absent graphs on four vertices compare independent subset enumeration and transitive closure, plus exact signed extremes, parallel ties, loops, isolates, invalid endpoints and both caps. The oracle uses a distinct representation or exhaustive candidate enumeration; self-roundtrip alone is insufficient.

## Provenance
Boruvka (1926), minimum spanning tree cut algorithm. Implementation and fixtures are original.

## Limitations
Partial syntax is source-bound, not a semantic AST; macros remain opaque. Finite exhaustive small domains and mutation witnesses are not formal proof. No runtime speed claim, approximation cap, automatic substitution or unbounded resource guarantee.

## subset_oracle_rank
In exhaustive_small_signed_forests the oracle fixes target transitive closure and considers subsets with exactly n-components edges. Equality of closure plus this edge count forces an acyclic spanning forest; deleting the rank condition would admit extra negative edges on cycles and solve a different optimization. Each of six potential edges is independently absent or weighted -1,0,1, giving 4096 assignments. A triangle of three -1 edges distinguishes unconstrained connected-subgraph weight -3 from forest optimum -2.

## signed_sum_discrimination
signed_wide_total_and_ties fixes two selected i64::MIN edges and asserts total=2*i128::from(i64::MIN), outside i64 range, plus exact selected indices and isolates. The compiled production subtraction mutant returns the opposite signed sum and fails this named assertion after successful compilation. The positive i64::MAX path also requires a sum above i64::MAX. These witnesses reject sign reversal and narrow accumulation but do not prove every arithmetic implementation correct.

## evidence

### Native checks

Three named tests pass twice debug and twice release with identical normalized names/outcomes. 4096 signed/absent graphs on four vertices compare independent subset enumeration and transitive closure, plus exact signed extremes, parallel ties, loops, isolates, invalid endpoints and both caps.

## History

### 2026-10-30 · descriptant

2026-10-30: prospective model selected exact capped algorithms with deterministic canonical outputs. Validated signed multigraph G maps to one minimum spanning forest F. Repeatedly choose each DSU component's least outgoing (weight,input index), union candidates in ascending root order, and stop at an unchanged partition; sort selected original indices. Minimize sum of edge weights separately in every connected component; retain isolates. The distinct total order resolves ties.

Model refinement, 2026-10-30: five typed views now distinguish the two baseline relations and three concrete specializations. New validation, observation and oracle constraints retain which claims have native witnesses and which remain source-derived. Rust bytes, partial syntax and native evidence are unchanged.

### 2026-10-30 · evidence

Compiled production mutation for boruvka fails the named assertion signed_wide_total_and_ties; production bytes restored before positive repetitions. Strict all-target algorithms Clippy and narrow rustfmt pass. Six partial syntax projections generated twice identically; full scaffold groupants checked twice. Raw logs live in $MOVA_DATA/runs/octopus-wave16-graphs-2026-10-30; evidence.json binds hashes. No live corpus, parser, teacher, GPU or full-school execution.
