# biconnected_components: traversal

Source hash and half-open byte scope are in manifest.json. Domain: reachable entry states from the parent function with successful allocation, trusted Rust compiler/library and fixed input. Observation: scoped exit state or early returned error.

## Forward
Input is validated adjacency with stable edge IDs. Preassign singleton loop labels. Use discovery/lowlink state and an explicit vertex stack to label each non-loop block by popping the pending edge stack through its parent identity whenever low[child] >= discovery[parent]. Output is a total internal label vector; labels need not be canonical.

## Invariant
Pending edges are exactly discovered edges not yet assigned a block. Each edge is pushed at most once and assigned once. Only the parent edge identity is skipped; a second parallel edge remains an ancestor back edge.

## Cost
O(V+E) work and storage; adjacency already exists. Each of 2E adjacency records advances one cursor. No recursive call depth.

## behavior-complement
Type: behavior_complement. Within the declared reachable entry states and scoped return/state observations, let R be the forward relation above. Its complement consists of all incompatible outputs or state transitions: violations of the scoped invariant, incorrect early errors or absent transitions required by R. This is a specification complement, not merely a collection of negative fixtures. Input is validated adjacency with stable edge IDs. Preassign singleton loop labels. Use discovery/lowlink state and an explicit vertex stack to label each non-loop block by popping the pending edge stack through its parent identity whenever low[child] >= discovery[parent]. Output is a total internal label vector; labels need not be canonical.
Status: partial_source_derived_model. The domain, observation and assumptions above apply. Evidence is inherited from ../model.md#evidence; scoped generalizations beyond those finite cases remain unproved.

## reverse-reconstruction
Type: reverse_reconstruction. After projecting a completed label vector to its equality partition, compatible graphs have that partition; exact numerical labels additionally constrain loop preassignment and traversal order. Thus partition equality alone is necessary but not sufficient for the exact-label fiber. A partial traversal state admits graph continuations compatible with its discovery, low and stack constraints, usually many; this is not an exhaustive construction of those continuations.
Status: partial_source_derived_model. The domain, observation and assumptions above apply. Evidence is inherited from ../model.md#evidence; scoped generalizations beyond those finite cases remain unproved.

## parent-identity
Type: parent_identity_exclusion. Skip exactly the tree edge identity, preserving a parallel edge to the same parent as a back edge.
Status: partial_source_derived_model. The domain, observation and assumptions above apply. Evidence is inherited from ../model.md#evidence; scoped generalizations beyond those finite cases remain unproved.

## cut-boundary
Type: lowlink_boundary. At low[child] equal to discovery[parent], a block closes; strict greater-than would leave a cycle pending.
Status: partial_source_derived_model. The domain, observation and assumptions above apply. Evidence is inherited from ../model.md#evidence; scoped generalizations beyond those finite cases remain unproved.

## loop-labels
Type: loop_partition. Each loop gets a fresh label before DFS and neither of its adjacency records enters the edge stack.
Status: partial_source_derived_model. The domain, observation and assumptions above apply. Evidence is inherited from ../model.md#evidence; scoped generalizations beyond those finite cases remain unproved.

## stack-budget
Type: monotone_cursor. Each adjacency cursor increases once per consumed entry; no return to a vertex resets its cursor.
Status: partial_source_derived_model. The domain, observation and assumptions above apply. Evidence is inherited from ../model.md#evidence; scoped generalizations beyond those finite cases remain unproved.

## Composition
This fragment relies on its parent's entry preconditions and establishes only the exit conditions stated here. Parent composition wires traversal labels into canonicalization, or global ordering into checked counting. The next stage's preconditions require all preceding invariants, not merely local test success.

## Epoch memory
Prospective predictions and finite outcomes are retained in the parent model. The child scope clarifies interaction obligations; no independent execution or universal proof is claimed.

## Limits
Allocation failure excluded. Partial syntax has no semantic resolution or macro expansion. No useful certified approximation metric was established; exact finite checks are not approximate surrogates.

## History

### 2026-11-01 · Epoch memory

2026-11-01 review: loop preassignment makes exact internal labels observable in a stronger state model than the returned partition. For edges [(0,0),(0,1)], labels are [0,1]; for [(0,1),(2,3)], labels are [0,1], while the reversed arrangement [(0,1),(0,0)] has labels [1,0]. All three return [[0],[1]]. This follows from the scoped source; the public API hides these labels. The fresh-bridge API probe in ../model.md#evidence constrains the composed outcome, not every internal stack state.
