# algorithm-kd-tree-tests

## descriptant

### Exact bounded 2D KD nearest neighbor tests

Original std-only Rust implementation and fixtures. Reference: Bentley, Multidimensional binary search trees used for associative searching, CACM 18(9), 1975.

#### Limits

Exact bounded APIs only. No generic 1/2/5/10% accuracy surrogate is supported. Finite tests do not prove universal correctness or runtime gains. Standard Vec/Box/BTreeMap allocation failure may abort or panic; allocator refusal is not injected. No callbacks, I/O or concurrency effects.
Partial Rust syntax is source-bound; macro expansion, semantic AST proof and whole-Mova quality remain gaps.

## model

# Mathematical model

## Forward

Execute deterministic fixtures, compare exact API values and state against independent finite oracles, then report success iff all assertions hold. Enumerate all512 subsets of a3x3 grid and25 query locations per subset; linear i128-distance minimization is independent of KD partition/pruning. Separate duplicates, equidistant points, query i32 extrema and constructor limits.

## Invariant

Fixtures are deterministic, production and oracle observations are compared exactly; state checks inspect parent edges and order where relevant. Assertions fail on disagreement; no benchmark thresholds enter correctness.

## Cost

Bounded finite enumeration plus production/oracle work; exhaustive here means the documented small fixture universe only. Runs use ordinary allocator behavior; resource failure is not injected.

## behavior_complement

For terminating finite harness executions, reports that disagree with the conjunction of assertions are excluded. Passing a candidate with any fixture mismatch is absent from the specified relation. Abort/nontermination are outside the observation universe; this relation is not a universal production correctness proof.

## reverse_reconstruction

A pass admits every implementation agreeing on these finite observations, including implementations differing arbitrarily outside the fixtures. A failure admits production defects or oracle defects; the report alone does not uniquely reconstruct the cause.

## oracle_independence

Enumerate all512 subsets of a3x3 grid and25 query locations per subset; linear i128-distance minimization is independent of KD partition/pruning. Separate duplicates, equidistant points, query i32 extrema and constructor limits.

## mutation_sensitivity

Replace final returned distance d by d+1; exhaustive_grid_oracle must fail. Source bytes are restored before the successful positive rerun. These controls establish sensitivity only to the chosen defects.

## finite_schedule

Three tests: exhaustive_grid_oracle, ties_duplicates_and_extremes, validation_and_capacity. Counts and repeat receipts are retained externally; neither repeatability nor test count guarantees unseen behavior.

## Gaps

Exact bounded APIs only. No generic 1/2/5/10% accuracy surrogate is supported. Finite tests do not prove universal correctness or runtime gains. Standard Vec/Box/BTreeMap allocation failure may abort or panic; allocator refusal is not injected. No callbacks, I/O or concurrency effects.

## evidence

### Evidence

Four identical normalized runs: 706 tests; 14 new tests.
kd_tree: compiled production mutant exit Some(101), named assertion exhaustive_grid_oracle; exact restoration and positive rerun exit Some(0).
bk_tree: compiled production mutant exit Some(101), named assertion exhaustive_metric_oracle; exact restoration and positive rerun exit Some(0).
interval_tree: compiled production mutant exit Some(101), named assertion exhaustive_closed_overlap_oracle; exact restoration and positive rerun exit Some(0).
treap: compiled production mutant exit Some(101), named assertion exhaustive_operation_oracle; exact restoration and positive rerun exit Some(0).
splay_tree: compiled production mutant exit Some(101), named assertion exhaustive_stateful_oracle; exact restoration and positive rerun exit Some(0).

For this module: Three tests: exhaustive_grid_oracle, ties_duplicates_and_extremes, validation_and_capacity.

## History

### 2026-10-30 · descriptant

#### Prospective epoch, 2026-10-30

Domain, recurrence, invariants, costs and validation plan were saved before implementation in $MOVA_DATA/runs/octopus-wave26-indexes-2026-10-30/prospective.md. No runtime gain was predicted or measured.

#### Executed epoch, 2026-10-30

Three tests: exhaustive_grid_oracle, ties_duplicates_and_extremes, validation_and_capacity.

Enumerate all512 subsets of a3x3 grid and25 query locations per subset; linear i128-distance minimization is independent of KD partition/pruning. Separate duplicates, equidistant points, query i32 extrema and constructor limits.

### 2026-10-30 · evidence

Heading `### Evidence, 2026-10-30` stays in evidence as `### Evidence`.

Strict mova-algorithms all-target Clippy passed; focused rustfmt passed. Receipts: $MOVA_DATA/runs/octopus-wave26-indexes-2026-10-30. Two freshly generated syntax projections must compare identical before syntax.json installation. No benchmark or training/quality measurement is claimed.
