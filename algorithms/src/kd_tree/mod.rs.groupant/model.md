# algorithm-kd-tree

## descriptant

### Exact bounded 2D KD nearest neighbor

Original std-only Rust implementation and fixtures. Reference: Bentley, Multidimensional binary search trees used for associative searching, CACM 18(9), 1975.

#### Limits

Exact bounded APIs only. No generic 1/2/5/10% accuracy surrogate is supported. Finite tests do not prove universal correctness or runtime gains. Standard Vec/Box/BTreeMap allocation failure may abort or panic; allocator refusal is not injected. No callbacks, I/O or concurrency effects.
Partial Rust syntax is source-bound; macro expansion, semantic AST proof and whole-Mova quality remain gaps.

## model

# Mathematical model

## Forward

Build recursively at alternating-axis medians, ordered by coordinate then original index. For query q visit the near child first and compare (squared distance,index); visit the far child when squared split-plane distance <= current minimum. Return (index,distance), or None iff no points.

## Invariant

Every original point occurs once. Descendants respect the selected axis partition; median splitting bounds depth logarithmically. best is the lexicographically smallest (distance,index) among visited nodes. A skipped subtree has a strictly larger distance lower bound.

## Cost

Build O(n log² n), O(n) resident storage; query O(n) worst time with O(log n) recursion. Re-sorting each subproblem is explicit; no worst-case sublinear nearest-neighbor claim.

## behavior_complement

For any admitted point sequence P and query q, candidate None is excluded exactly when P is nonempty; candidate (i,d) is excluded if i is absent, d differs from squared distance, or another (distance,j) is lexicographically smaller. Invalid constructor inputs paired with Ok are excluded. The deterministic median shape further constrains internal state.

## reverse_reconstruction

An observed (i,d) admits all bounded point sequences containing point i at squared distance d from q with no lexicographically better competitor. Coordinate sign changes and extra farther points yield ambiguous preimages. None reconstructs an empty stored sequence but does not identify query q.

## plane_tie

The far-side pruning comparison is inclusive. A far point at equal distance can carry a smaller original index. Closed split-plane search is necessary even when a best distance already exists.

## finite_distance

Stored coordinate absolute value <=10^6 and arbitrary i32 query imply each absolute difference <=2^31+10^6; twice its square <2^64. i64 subtraction and u64 sum are exact; i128 test oracle independently computes the same mathematical metric.

## duplicate_identity

Duplicate coordinates remain separate points; nearest always selects their smallest original index. Reordering input can change reported identity without changing minimum distance.

## validation_order

More than4096 points errors before coordinates are checked; otherwise any coordinate outside +/-10^6 errors before build allocation. Empty is valid and allocates no node; successful nonempty builds depend on allocator success.

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
