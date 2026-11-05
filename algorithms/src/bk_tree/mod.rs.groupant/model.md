# algorithm-bk-tree

## descriptant

### Exact byte metric BK tree

Original std-only Rust implementation and fixtures. Reference: Burkhard and Keller, Some approaches to best-match file searching, CACM 16(4), 1973.

#### Limits

Exact bounded APIs only. No generic 1/2/5/10% accuracy surrogate is supported. Finite tests do not prove universal correctness or runtime gains. Standard Vec/Box/BTreeMap allocation failure may abort or panic; allocator refusal is not injected. No callbacks, I/O or concurrency effects.
Partial Rust syntax is source-bound; macro expansion, semantic AST proof and whole-Mova quality remain gaps.

## model

# Mathematical model

## Forward

Insert each word by repeatedly following child edge equal to its byte Levenshtein distance from the node; distance zero appends the original index. Query computes d to each visited node, emits its indexes iff d<=radius, then visits child edges in [saturating(d-radius),saturating(d+radius)]. Sort matches by original index.

## Invariant

Every input index occurs once. Every descendant of an edge e from a node has distance e from that node because insertion takes that edge only at distance e. Triangle inequality implies a match must satisfy |d-e|<=radius. Duplicate words share a node.

## Cost

n<=4096 and L<=256. Worst build O(n²(1+L²)), query O(n(1+L²)+k log k), O(nL+n) resident storage; query O(n+k+L) scratch. BTreeMap lookup overhead is bounded by <=256 distance edges. No guaranteed sublinear query.

## behavior_complement

For admitted words W,q,r, an output pair (i,d) is excluded if i is outside W, d is not exact byte edit distance, or d>r. A proposed output list omitting any in-radius index, adding an outside index, duplicating an index or breaking ascending index order is excluded. Overlength query paired with Ok is excluded even for an empty tree.

## reverse_reconstruction

A returned list identifies distances only for reported indexes; all dictionaries and queries yielding those distances and excluding other indexes beyond r are compatible. Byte renaming by a bijection preserves edit distances. Empty output may mean empty dictionary or a nonempty dictionary with every word farther than r; inversion is nonunique.

## triangle_pruning

Edges outside inclusive [d-r,d+r] cannot contain a radius match by triangle inequality. Saturating usize endpoints safely cover huge radii; usize::MAX admits every word.

## byte_metric_join

Calls existing levenshtein::distance on u8 slices. Distances count bytes including NUL and255, not Unicode scalars or graphemes. L<=256 gives distance<=256 and bounded DP arithmetic.

## duplicate_fiber

Equal byte vectors merge into one metric node but retain every original index in insertion order. Query output sorting restores global input order across traversal branches.

## validation_order

More than4096 entries errors first, then any word longer than256. Query length is checked before empty-tree handling. An empty word is valid and an empty dictionary produces empty matches. Allocator exhaustion is outside recoverable errors.

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

For this module: Two tests: exhaustive_metric_oracle and byte_semantics_and_boundaries.

## History

### 2026-10-30 · descriptant

#### Prospective epoch, 2026-10-30

Domain, recurrence, invariants, costs and validation plan were saved before implementation in $MOVA_DATA/runs/octopus-wave26-indexes-2026-10-30/prospective.md. No runtime gain was predicted or measured.

#### Executed epoch, 2026-10-30

Two tests: exhaustive_metric_oracle and byte_semantics_and_boundaries.

31 binary byte words of lengths0..4, all31 cyclic insertion orders plus duplicate empty word, all31 queries and radii0,1,2,4,usize::MAX. Independent full2D dynamic programming oracle does not reuse production rolling-row kernel. Explicit255/NUL cases,256-byte and4096-entry boundaries.

### 2026-10-30 · evidence

Heading `### Evidence, 2026-10-30` stays in evidence as `### Evidence`.

Strict mova-algorithms all-target Clippy passed; focused rustfmt passed. Receipts: $MOVA_DATA/runs/octopus-wave26-indexes-2026-10-30. Two freshly generated syntax projections must compare identical before syntax.json installation. No benchmark or training/quality measurement is claimed.
