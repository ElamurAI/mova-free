# world-story-reproduction

## forward

Iterate L,C,O,R,P,B, deferring same-registry parent/part dependencies for at most 64 passes; append unresolved residuals in input order.

## invariant

Each selected supported id occurs once; unsupported registries are omitted. Cycles and long adverse chains may use fallback order.

## cost

At most 64*N candidate scans plus BTree membership and World snapshot cost.

## composition

creation_dependency_order_and_cycle_fallback verifies a reverse-numbered location chain and a three-location cycle, distinct from unrestricted topological sorting.

## checks

Native story::wave9_models tests exercise the bounded domains described in ../model.md#evidence at the file root. Ordinary test bodies have partial syntax projections; macro token contents are opaque. The graph oracle uses a dense three-state Floyd closure, not the implementation BFS/bitset algorithm. Typed commands call the real public World::apply boundary.

## epoch-memory

Initial modeling epoch: distinguish admitted graph states from arbitrary public-field edits, and internal mutation from public commit. No production bug was established. In particular narr::relate restores its multi-edge batch, and World::apply discards a cloned failed command even when apply_root has already added a cause edge. Creation ordering has a deliberate 64-round fallback, not an unrestricted topological-sort guarantee. No percentage-error approximant or performance claim.

## behavior-complement

Subject: world-story-reproduction, source-bound revision. Domain and observation: the finite typed state/transition family in forward, with dependencies fixed. Operator: No event/claim/frame id is emitted by this six-registry projection. Cyclic dependencies cannot cause unbounded iteration.

Assurance: source-derived partial relation, with finite native evidence where named in checks. Unknown paths remain unknown; no universal coverage or unique inverse is inferred. Complement claims are inner absent-behavior bounds under their stated premises; reverse relations retain all compatible antecedents in their declared family.

## reverse-reconstruction

Subject: world-story-reproduction, source-bound revision. Domain and observation: the finite typed state/transition family in forward, with dependencies fixed. Operator: Many parent graphs induce the same list; retain all dependency assignments whose bounded schedule yields that list. Output does not prove acyclicity.

Assurance: source-derived partial relation, with finite native evidence where named in checks. Unknown paths remain unknown; no universal coverage or unique inverse is inferred. Complement claims are inner absent-behavior bounds under their stated premises; reverse relations retain all compatible antecedents in their declared family.

## coupling

Subject: world-story-reproduction, source-bound revision. Domain and observation: the finite typed state/transition family in forward, with dependencies fixed. Operator: creation_dependency_order_and_cycle_fallback verifies a reverse-numbered location chain and a three-location cycle, distinct from unrestricted topological sorting.

Assurance: source-derived partial relation, with finite native evidence where named in checks. Unknown paths remain unknown; no universal coverage or unique inverse is inferred. Complement claims are inner absent-behavior bounds under their stated premises; reverse relations retain all compatible antecedents in their declared family.

## History

### 2026-10-30 · document

Partial source-bound model, 2026-10-30. The reference is the exact sibling Rust source. The domain is valid finite typed values reachable through checked World::apply, except explicitly constructed graph/helper fixtures. Public mutable graph/frame fields are not defended against arbitrary corruption. No live parser, teacher, corpus, GPU or persistence is exercised.
