# algorithm-treap

## descriptant

### Explicit-priority bounded treap

Original std-only Rust implementation and fixtures. Reference: Seidel and Aragon, Randomized Search Trees, Algorithmica 16, 1996.

#### Limits

Exact bounded APIs only. No generic 1/2/5/10% accuracy surrogate is supported. Finite tests do not prove universal correctness or runtime gains. Standard Vec/Box/BTreeMap allocation failure may abort or panic; allocator refusal is not injected. No callbacks, I/O or concurrency effects.
Partial Rust syntax is source-bound; macro expansion, semantic AST proof and whole-Mova quality remain gaps.

## model

# Mathematical model

## Forward

Search by unique i64 key. New insertion attaches a leaf and rotates it upward while its (priority,key) is less than parent's. Deletion rotates the target downward with the smaller-ranked child until it is a leaf, detaches it and recycles the arena slot. Return membership booleans and sorted keys.

## Invariant

Live reachable nodes form a strict BST and a strict min heap in (priority,key); parent pointers agree with child edges. Every reachable node has one path from root; free indexes are unreachable and available for reuse. Root has no parent.

## Cost

Search/insert/remove O(height); height<=n<=4096 but may be n. Expected O(log n) requires independently random priorities, not explicit arbitrary inputs. Keys O(n) traversal and output; arena O(maximum live count) with slot reuse.

## behavior_complement

A state/output transition is excluded when its key set differs from set insertion/removal semantics, a duplicate changes its stored priority, an absent removal changes shape, or BST/heap/parent invariants fail. Returning Ok(true) for existing keys or accepting a new key at4096 live keys is excluded. This constrains state, not only a returned boolean.

## reverse_reconstruction

Observed keys admit many insertion/removal histories and explicit priority assignments; the full live (key,priority) map determines a unique Cartesian tree under total rank order but not free-slot indexes or history. Ok(false) insertion reconstructs prior presence, not the previous priority. A removed key's priority cannot be recovered from its success boolean.

## priority_ties

Heap rank is total lexicographic (u64 priority,i64 key); equal priorities order by key. Duplicate insertion does not replace priority. Determinism needs no RNG and supplies no probabilistic balancing guarantee.

## rotation_conservation

A rotation moves middle subtree under former parent, updates middle parent, grandparent edge and root as needed. Inorder keys and external subtrees are conserved; heap violations are resolved during insertion/deletion.

## arena_reclamation

Deletion first removes all live links to target before adding its slot to free. A subsequent insertion overwrites the slot completely. live_count=nodes.len-free.len; resident arena never exceeds4096 slots despite repeated churn.

## failure_precedence

Search for duplicates precedes cap check, so a duplicate at capacity returns Ok(false). A new key at capacity returns Err with state unchanged. Full i64 keys/u64 priorities use comparison only; standard allocation failures are not caught.

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

For this module: Two tests: exhaustive_operation_oracle and priorities_capacity_and_reuse.

## History

### 2026-10-30 · descriptant

#### Prospective epoch, 2026-10-30

Domain, recurrence, invariants, costs and validation plan were saved before implementation in $MOVA_DATA/runs/octopus-wave26-indexes-2026-10-30/prospective.md. No runtime gain was predicted or measured.

#### Executed epoch, 2026-10-30

Two tests: exhaustive_operation_oracle and priorities_capacity_and_reuse.

All46656 six-step operation schedules over insert/remove of three keys, alternating explicit priorities; compare to BTreeSet after each step and independently check BST bounds, heap, parent pointers, reachability and free-list partition. Capacity4096, linear-height priorities, reuse after2048 deletions, ties and integer extrema.

### 2026-10-30 · evidence

Heading `### Evidence, 2026-10-30` stays in evidence as `### Evidence`.

Strict mova-algorithms all-target Clippy passed; focused rustfmt passed. Receipts: $MOVA_DATA/runs/octopus-wave26-indexes-2026-10-30. Two freshly generated syntax projections must compare identical before syntax.json installation. No benchmark or training/quality measurement is claimed.
