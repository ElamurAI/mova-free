# world-story-time

## forward

Map Pt to first inserted index/world; admit edges by reverse reachability; retain dead edges after kill; compute weak/strict closures and prioritized interval labels.

## invariant

Admitted finite graphs have only weak cycles. Weak reflexivity is implicit in reach/le, even with no stored self-edge.

## cost

BFS O(V+E); closure bounded O(V E ceil(V/64)); witnesses O(E).

## composition

Endpoint constraint fragment; finite_time_oracle_and_kill exhausts 729 three-node input codes with sequential refusal and kill, comparing both closures and witness chains.

## checks

Native story::wave9_models tests exercise the bounded domains described in ../model.md#evidence at the file root. Ordinary test bodies have partial syntax projections; macro token contents are opaque. The graph oracle uses a dense three-state Floyd closure, not the implementation BFS/bitset algorithm. Typed commands call the real public World::apply boundary.

## epoch-memory

Initial modeling epoch: distinguish admitted graph states from arbitrary public-field edits, and internal mutation from public commit. No production bug was established. In particular narr::relate restores its multi-edge batch, and World::apply discards a cloned failed command even when apply_root has already added a cause edge. Creation ordering has a deliberate 64-round fallback, not an unrestricted topological-sort guarantee. No percentage-error approximant or performance claim.

## behavior-complement

Subject: world-story-time, source-bound revision. Domain and observation: the finite typed state/transition family in forward, with dependencies fixed. Operator: On add-generated graphs a strict self-edge or strict return cycle cannot be admitted; dead edges cannot witness paths.

Assurance: source-derived partial relation, with finite native evidence where named in checks. Unknown paths remain unknown; no universal coverage or unique inverse is inferred. Complement claims are inner absent-behavior bounds under their stated premises; reverse relations retain all compatible antecedents in their declared family.

## reverse-reconstruction

Subject: world-story-time, source-bound revision. Domain and observation: the finite typed state/transition family in forward, with dependencies fixed. Operator: A reachability matrix admits many edge sets and Why annotations; recover all admitted labeled edge graphs satisfying that matrix, not a chosen transitive reduction.

Assurance: source-derived partial relation, with finite native evidence where named in checks. Unknown paths remain unknown; no universal coverage or unique inverse is inferred. Complement claims are inner absent-behavior bounds under their stated premises; reverse relations retain all compatible antecedents in their declared family.

## coupling

Subject: world-story-time, source-bound revision. Domain and observation: the finite typed state/transition family in forward, with dependencies fixed. Operator: Endpoint constraint fragment; finite_time_oracle_and_kill exhausts 729 three-node input codes with sequential refusal and kill, comparing both closures and witness chains.

Assurance: source-derived partial relation, with finite native evidence where named in checks. Unknown paths remain unknown; no universal coverage or unique inverse is inferred. Complement claims are inner absent-behavior bounds under their stated premises; reverse relations retain all compatible antecedents in their declared family.

## History

### 2026-10-30 · document

Partial source-bound model, 2026-10-30. The reference is the exact sibling Rust source. The domain is valid finite typed values reachable through checked World::apply, except explicitly constructed graph/helper fixtures. Public mutable graph/frame fields are not defended against arbitrary corruption. No live parser, teacher, corpus, GPU or persistence is exercised.
