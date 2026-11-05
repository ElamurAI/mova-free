# algorithm-persistent-segment-tree

## model

# Persistent point replacement and range sums

## domain
Finite i64 snapshots, usize version IDs and indices, half-open ranges.
Sums and checked additions use i128. On targets with usize width <=64 every
mathematical sum of a realizable i64 slice fits i128. Version zero exists
even for empty input. No configured memory cap; allocation failure is outside
Result. Public errors distinguish version, index, range and overflow.

## forward
Build a balanced binary interval tree, splitting at floor(length/2).
Leaves store i64 widened to i128; internal nodes sum children. Arena appends
nodes and roots retain historical root IDs. set validates version, then index,
records arena length and recursively copies the chosen path. Successful
replacement publishes one new root; a returned error truncates appended
nodes before returning. range_sum validates version then endpoints, returns
zero for empty intervals, and recursively sums fully covered nodes using
checked i128 addition.

## invariant
All nodes reachable from a published root are immutable. Leaves correspond
to original positions; node sums equal the represented interval sum.
Every root denotes one complete snapshot; branches may share nodes.
Only newly allocated nodes may be truncated on failure. No root is published
until replacement finishes. Midpoints start+(end-start)/2 avoid overflow.
Queries stop before child access at disjoint or fully covered leaves.

## cost
For n>0, initial nodes exactly 2n-1. Successful update appends at most
ceil(log2(n))+1 nodes and one root, even if replacement equals old value.
Build O(n), query and replacement O(log(n+1)), stack O(log(n+1)).
After u updates storage O(n+u log(n+1)), plus u+1 root Options.
For n=1,000,000, initial payload is (1,999,999)*size_of(Node), plus roots;
each update appends at most 21 nodes. Node layout is target dependent.
No reclamation or compaction; historical versions remain reachable. Clone
copies the arena and roots. Allocation exhaustion is not caught or promised
atomic by the Result contract. Queries allocate nothing.

## behavior-complement
Type behavior_complement. Universe is valid snapshot histories, requested
operations and candidate returned states/results. R is the forward relation.
Excluded behaviors include mutation of any historical root/node, publication
of a partial update, invalid-range success, ignored invalid versions even on
empty ranges, wrapped sums, and partial arena extension after a returned
error. Allocation abort/panic is outside this relation.

## reverse-reconstruction
Type reverse_reconstruction. A fixed-version range sum s admits all i64
arrays whose selected entries sum to s; outside entries are free. A complete
set of singleton observations uniquely determines that version's array but
does not recover update ancestry, allocation IDs or repeated equal writes.
New version v and replacement (i,x) leave the prior value at i ambiguous.
Failed calls preserve known prior state but do not identify the rejected
parameters from the Error category alone.

## history-frame
Type persistent_frame. Root v refers only to immutable nodes existing when
it was published. New path nodes reuse untouched child IDs; no operation
rewrites old nodes. Tests compare the entire previous arena prefix after
each branching update and query all previous Vec snapshots.

## transactional-publication
Type rollback_atomicity. Before recursive replacement remember m=nodes.len.
If any checked add fails, truncate back to m and do not append a root.
Version/index validation fails before any append. Exact node and root equality
is asserted after invalid calls and a deliberately injected internal overflow.
The injected state is outside the public invariant; it exercises a defensive
branch that realizable <=64-bit public inputs cannot reach.

## wide-arithmetic
Type signed_sum_embedding. Widen i64 before addition. Cancellation patterns
such as [MAX,MAX,MIN,MIN,1] retain exact subrange sums beyond i64 and total -1.
i128 checked addition prevents silent wrapping if platform/domain assumptions
change. No decomposition-dependent i64 overflow is exposed.

## path-budget
Type structural_allocation_bound. The floor-half split yields maximum depth
ceil(log2 n). One node per visited level means at most depth+1 appended nodes;
other branches are shared. Tests inspect node_count deltas for n=1..17,
including non-powers of two and repeated updates from older roots.

## tests
Five tests: branching Vec snapshots n=1..17 with 24 updates each; all historical
versions/all intervals checked after every update; invalid-call exact state,
wide cancellation/empty version, replacement mutation witness, and defensive
overflow rollback. Repeated profiles and controls are recorded in model.md#evidence.

## descriptant
Composition scope: build establishes node sums; replace preserves interval
shape and historical immutability; set is the publication/rollback boundary;
query relies on both invariants. Branching versions detect accidental latest-
root use that a linear update trace misses. Partial syntax omits semantics
and macro expansion. No numerical surrogate is offered: signed cancellation
makes relative error unstable and version identity remains exact. Certified
approximate replacement and composition guarantees are explicit gaps.

## evidence

### Evidence

5 tests pass twice debug and twice release. Normalized named outcomes match within each profile. Compiled control replacement_control: replacement leaf value incremented by one fails by named assertion (exit 101), then source is restored byte-for-byte and all focused tests pass. Syntax projections run twice and compare byte-for-byte.

## prospective

### Prospective model
Finite i64 snapshots; versioned half-open sums with checked i128 node arithmetic. Initial binary interval tree, then copy only root-to-leaf path on replacement. Historical nodes and roots immutable. Returned update errors truncate appended nodes exactly. Query sums use i128. Empty input has one empty version. Initial 2n-1 nodes; at most ceil(log2 n)+1 appended nodes per update and one root; O(log n) updates/queries, O(n+u log n) storage. No configured cap; allocation failures outside Result. Complement excludes stale-version mutation, wrapped sums, partial errors. A sum s leaves all arrays totaling s compatible; all singleton observations reconstruct a snapshot but not update history. Signed cancellation precludes a useful relative-error surrogate here.

## History

### 2026-10-30 · document

2026-10-30. Original implementation. Origin: Driscoll et al.,
"Making Data Structures Persistent" (1989). Prospective model retained.

### 2026-10-30 · evidence

Heading `### Evidence, 2026-10-30` stays in evidence as `### Evidence`.

External logs: $MOVA_DATA/runs/octopus-wave18-structures-2026-10-30/. Finite tests provide no universal proof or timing guarantee.

Source algorithms/src/persistent_segment_tree/mod.rs: sha256 5f2af47008444ec0307365c7d0d189a3382510112a5bdfb4f65760984dc16f3b

- persistent_segment_tree-debug-1.log: sha256 17bc81ad86efc73497bb9408b21afdf2cd42b8413ae9e661efbc35e5cc0a02c5

- persistent_segment_tree-debug-2.log: sha256 41fa8b446fe847a65dae9b03eae10396e8f2444b46ae673e6fe2caaedcd43063

- persistent_segment_tree-release-1.log: sha256 29bc9d8f14663c5a85a0b56708d920494dabc618d879c33dd0039b278bdc3c13

- persistent_segment_tree-release-2.log: sha256 e2de0a8a92e7e1bcc389e4d67ee19ee974b4507bbf67d03453aa80195925f9d0

- persistent_segment_tree-mutant.log: sha256 45034d1881c87296bbc863b173cfa9c9cc944702199d1794324462e71c7b7488

- persistent_segment_tree-restored.log: sha256 10faf1f126daeeb9b952938663ec0fe546c3a7ca1cad658ac0b71b92f5662cbc

- syntax.json: sha256 d6e291c3fa311e03d9b498efb55ee35ddd142a21997c78263f3bde443acfc5c5

### 2026-10-30 · prospective

Heading `### Prospective model, 2026-10-30` stays in prospective as `### Prospective model`.
