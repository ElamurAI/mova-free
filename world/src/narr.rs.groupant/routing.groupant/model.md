# world-narr-routing

## forward

Find nearest ancestor owning a world; take and restore root Story; route local execution and register new canonical events. worlds returns root then each owned frame world.

## invariant

Valid ancestry is rooted and acyclic; frame worlds are restored after Result errors. Public transaction ownership belongs to World::apply.

## cost

Ancestor traversal O(depth); query scans O(frames plus entities/events searched); cloned world effects external.

## composition

public_reveal_rollback_after_cause_edge compares real public rollback against intentional internal partial mutation. No universal internal transaction claim.

## checks

Native narr::wave9_models tests exercise the bounded domains described in ../model.md#evidence at the file root. Ordinary test bodies have partial syntax projections; macro token contents are opaque. The graph oracle uses a dense three-state Floyd closure, not the implementation BFS/bitset algorithm. Typed commands call the real public World::apply boundary.

## epoch-memory

Initial modeling epoch: distinguish admitted graph states from arbitrary public-field edits, and internal mutation from public commit. No production bug was established. In particular narr::relate restores its multi-edge batch, and World::apply discards a cloned failed command even when apply_root has already added a cause edge. Creation ordering has a deliberate 64-round fallback, not an unrestricted topological-sort guarantee. No percentage-error approximant or performance claim.

## behavior-complement

Subject: world-narr-routing, source-bound revision. Domain and observation: the finite typed state/transition family in forward, with dependencies fixed. Operator: A returned Result error cannot permanently remove Story via normal apply_root completion. Panics are outside this guarantee.

Assurance: source-derived partial relation, with finite native evidence where named in checks. Unknown paths remain unknown; no universal coverage or unique inverse is inferred. Complement claims are inner absent-behavior bounds under their stated premises; reverse relations retain all compatible antecedents in their declared family.

## reverse-reconstruction

Subject: world-narr-routing, source-bound revision. Domain and observation: the finite typed state/transition family in forward, with dependencies fixed. Operator: World-only query results retain multiple frame origins when the same canonical id occurs across copies. event_any selects first root/frame occurrence; ent_any chooses most nonproject states with last maximum tie.

Assurance: source-derived partial relation, with finite native evidence where named in checks. Unknown paths remain unknown; no universal coverage or unique inverse is inferred. Complement claims are inner absent-behavior bounds under their stated premises; reverse relations retain all compatible antecedents in their declared family.

## coupling

Subject: world-narr-routing, source-bound revision. Domain and observation: the finite typed state/transition family in forward, with dependencies fixed. Operator: public_reveal_rollback_after_cause_edge compares real public rollback against intentional internal partial mutation. No universal internal transaction claim.

Assurance: source-derived partial relation, with finite native evidence where named in checks. Unknown paths remain unknown; no universal coverage or unique inverse is inferred. Complement claims are inner absent-behavior bounds under their stated premises; reverse relations retain all compatible antecedents in their declared family.

## History

### 2026-10-30 · document

Partial source-bound model, 2026-10-30. The reference is the exact sibling Rust source. The domain is valid finite typed values reachable through checked World::apply, except explicitly constructed graph/helper fixtures. Public mutable graph/frame fields are not defended against arbitrary corruption. No live parser, teacher, corpus, GPU or persistence is exercised.
