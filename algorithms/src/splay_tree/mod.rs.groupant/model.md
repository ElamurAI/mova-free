# algorithm-splay-tree

## descriptant

### Stateful iterative splay search tree

Original std-only Rust implementation and fixtures. Reference: Sleator and Tarjan, Self-Adjusting Binary Search Trees, JACM 32(3), 1985.

#### Limits

Exact bounded APIs only. No generic 1/2/5/10% accuracy surrogate is supported. Finite tests do not prove universal correctness or runtime gains. Standard Vec/Box/BTreeMap allocation failure may abort or panic; allocator refusal is not injected. No callbacks, I/O or concurrency effects.
Partial Rust syntax is source-bound; macro expansion, semantic AST proof and whole-Mova quality remain gaps.

## model

# Mathematical model

## Forward

Locate key with ordinary BST search. Access splays the matching node, or last visited node on miss, using zig/zig-zig/zig-zag rotations. New insert attaches a leaf and splays it; duplicate insert splays the existing node. root and keys only observe state.

## Invariant

Strict BST with unique keys, reciprocal parent/child links and exactly one root reaching every arena node. Each rotation preserves inorder keys. Successful access leaves the requested key at root; missing access leaves the pre-search last visited key at root.

## Cost

Individual access/insertion O(n), n<=4096. Standard splay potential sum(log subtree size) gives amortized O(log n) per access/insertion under sequence accounting, including initial potential. No worst-case per-call logarithmic guarantee. O(n) arena and iterative O(1) rotation scratch; keys uses O(height+n) traversal/output.

## behavior_complement

For a pre-state and key, candidate return/state pairs are excluded if membership is wrong, hit root differs from requested key, miss root differs from last visited node, or inorder set/parent structure changes incorrectly. Empty access returning true or creating a root is excluded. A new-key capacity error that mutates shape is excluded.

## reverse_reconstruction

An observed root after access admits all predecessor BSTs whose search ends at that key plus appropriate queries; multiple shapes and access histories can converge on the same root and inorder set. On a miss the resulting root identifies last visited key, not the absent query or original shape. Duplicate insertion false implies prior membership but loses earlier shape.

## access_state

Both hits and misses modify shape by splaying. locate itself is observational; access uses its last visited node. Empty access returns false and leaves None. Observation methods keys/root never call splay.

## double_rotation

If child-parent and parent-grandparent orientations match, rotate parent then child (zig-zig); otherwise child twice (zig-zag). With no grandparent use a single zig. These are standard splay cases, not merely move-to-root single rotations.

## amortization_scope

A single operation can traverse all nodes. The logarithmic statement concerns total actual cost plus potential changes over sequences and an initial-potential term, assumes unit-cost comparisons/index operations, and excludes outputting all keys and allocator costs.

## capacity_precedence

Duplicate lookup succeeds before cap check and splays even at4096 keys. A new key at capacity errors without splaying the last visited node. Arbitrary full-width i64 keys are compared without arithmetic; allocator errors are not recoverable.

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

For this module: Three tests: exhaustive_stateful_oracle, zig_zig_zag_and_missing_access, capacity_and_full_width.

## History

### 2026-10-30 · descriptant

#### Prospective epoch, 2026-10-30

Domain, recurrence, invariants, costs and validation plan were saved before implementation in $MOVA_DATA/runs/octopus-wave26-indexes-2026-10-30/prospective.md. No runtime gain was predicted or measured.

#### Executed epoch, 2026-10-30

Three tests: exhaustive_stateful_oracle, zig_zig_zag_and_missing_access, capacity_and_full_width.

All46656 six-step insert/access schedules over three keys versus BTreeSet; hit roots and independently traversed pre-call last-visited roots checked after every operation. Independent BST bound/parent/reachability checks, targeted alternating deep accesses, empty/miss,4096-key cap and full-width keys.

### 2026-10-30 · evidence

Heading `### Evidence, 2026-10-30` stays in evidence as `### Evidence`.

Strict mova-algorithms all-target Clippy passed; focused rustfmt passed. Receipts: $MOVA_DATA/runs/octopus-wave26-indexes-2026-10-30. Two freshly generated syntax projections must compare identical before syntax.json installation. No benchmark or training/quality measurement is claimed.
