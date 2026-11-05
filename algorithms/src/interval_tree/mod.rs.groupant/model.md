# algorithm-interval-tree

## descriptant

### Closed interval overlap tree

Original std-only Rust implementation and fixtures. Reference: Cormen, Leiserson, Rivest and Stein, Introduction to Algorithms, interval-tree augmentation.

#### Limits

Exact bounded APIs only. No generic 1/2/5/10% accuracy surrogate is supported. Finite tests do not prove universal correctness or runtime gains. Standard Vec/Box/BTreeMap allocation failure may abort or panic; allocator refusal is not injected. No callbacks, I/O or concurrency effects.
Partial Rust syntax is source-bound; macro expansion, semantic AST proof and whole-Mova quality remain gaps.

## model

# Mathematical model

## Forward

Sort (start,end,index), recursively choose medians, cache maximum endpoint over each subtree. Visit left subtree unless cached maximum is below query start; stop current/right processing when current start exceeds query end. Emit intervals whose end reaches query start, then sort original indexes.

## Invariant

Inorder triples are sorted; cached max_end equals the maximum endpoint among all subtree entries. Every emitted interval has start<=query.end and query.start<=end. A pruned subtree cannot overlap the query.

## Cost

O(n log n) build, O(n) storage. O(n+k log k) worst query time and O(log n+k) scratch; k output indexes need sorting. Balanced immutable topology gives logarithmic stack depth.

## behavior_complement

For valid closed intervals and query, any candidate result containing a non-overlap or missing an overlap is excluded; indexes must be unique and ascending. Touching endpoints are not excluded: closed intervals intersect there. Invalid/reversed query paired with Ok is excluded.

## reverse_reconstruction

A result index set S admits every interval sequence whose members overlap iff their index lies in S. Intersection positions, interval widths and exact endpoints are generally unrecoverable. Empty result admits both empty input and arbitrary disjoint intervals; no unique inverse.

## closed_boundary

Overlap is !(end<query.start || query.end<start). Zero-width intervals and endpoint-touching pairs match. Changing < pruning or <= membership to strict variants loses valid outputs.

## augmentation

max_end is max(own end,left max,right max); i64::MIN is a safe absent-child identity. No addition/subtraction means full-width endpoints cannot overflow.

## stable_identity

Sorted build triples include index for duplicates; duplicate intervals appear separately in answers. Query results sort by original index independently of internal median traversal order.

## validation_order

Reject length>4096 before reversed endpoint check. Otherwise all starts<=ends are admitted, including full i64 width and empty sequence. A reversed query errors even on an empty index; allocations use standard failure behavior.

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

For this module: Three tests: exhaustive_closed_overlap_oracle, duplicate_touching_full_width, validation_and_capacity.

## History

### 2026-10-30 · descriptant

#### Prospective epoch, 2026-10-30

Domain, recurrence, invariants, costs and validation plan were saved before implementation in $MOVA_DATA/runs/octopus-wave26-indexes-2026-10-30/prospective.md. No runtime gain was predicted or measured.

#### Executed epoch, 2026-10-30

Three tests: exhaustive_closed_overlap_oracle, duplicate_touching_full_width, validation_and_capacity.

All64 subsets of the six closed intervals on {-1,0,1}, all15 ordered queries on {-2,-1,0,1,2}; direct disjointness predicate is independent of augmented pruning. Duplicate points, full-width endpoints and cap boundaries are explicit.

### 2026-10-30 · evidence

Heading `### Evidence, 2026-10-30` stays in evidence as `### Evidence`.

Strict mova-algorithms all-target Clippy passed; focused rustfmt passed. Receipts: $MOVA_DATA/runs/octopus-wave26-indexes-2026-10-30. Two freshly generated syntax projections must compare identical before syntax.json installation. No benchmark or training/quality measurement is claimed.
