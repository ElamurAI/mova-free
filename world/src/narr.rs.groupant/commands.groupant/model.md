# world-narr-commands

## forward

Validate and execute frame, claim, time, branch, know and reveal commands. Ordering includes frame registration before some time checks, and reveal cause before relation/claims/history.

## invariant

Checks assume parser-valid variant fields; manually missing required narrator/level can panic and are excluded. Public command transaction provides rollback.

## cost

Validation scans registry/world/claim records; each temporal edge performs BFS; frame creation may clone/project full worlds.

## composition

typed_frames_close_resume_and_world_scope and canonical_claim_and_knowledge_identity execute typed public commands; live parsing is not involved.

## checks

Native narr::wave9_models tests exercise the bounded domains described in ../model.md#evidence at the file root. Ordinary test bodies have partial syntax projections; macro token contents are opaque. The graph oracle uses a dense three-state Floyd closure, not the implementation BFS/bitset algorithm. Typed commands call the real public World::apply boundary.

## epoch-memory

Initial modeling epoch: distinguish admitted graph states from arbitrary public-field edits, and internal mutation from public commit. No production bug was established. In particular narr::relate restores its multi-edge batch, and World::apply discards a cloned failed command even when apply_root has already added a cause edge. Creation ordering has a deliberate 64-round fallback, not an unrestricted topological-sort guarantee. No percentage-error approximant or performance claim.

## behavior-complement

Subject: world-narr-commands, source-bound revision. Domain and observation: the finite typed state/transition family in forward, with dependencies fixed. Operator: Future know.from beyond command anchors is refused. Claims require existing event metadata; reveals cannot create absent canonical events or overwrite established roles.

Assurance: source-derived partial relation, with finite native evidence where named in checks. Unknown paths remain unknown; no universal coverage or unique inverse is inferred. Complement claims are inner absent-behavior bounds under their stated premises; reverse relations retain all compatible antecedents in their declared family.

## reverse-reconstruction

Subject: world-narr-commands, source-bound revision. Domain and observation: the finite typed state/transition family in forward, with dependencies fixed. Operator: A single canonical event count corresponds to many observer claims and knowledge rows. To reconstruct commands, retain status, sentence, by and interpretation alternatives, not only event ids.

Assurance: source-derived partial relation, with finite native evidence where named in checks. Unknown paths remain unknown; no universal coverage or unique inverse is inferred. Complement claims are inner absent-behavior bounds under their stated premises; reverse relations retain all compatible antecedents in their declared family.

## coupling

Subject: world-narr-commands, source-bound revision. Domain and observation: the finite typed state/transition family in forward, with dependencies fixed. Operator: typed_frames_close_resume_and_world_scope and canonical_claim_and_knowledge_identity execute typed public commands; live parsing is not involved.

Assurance: source-derived partial relation, with finite native evidence where named in checks. Unknown paths remain unknown; no universal coverage or unique inverse is inferred. Complement claims are inner absent-behavior bounds under their stated premises; reverse relations retain all compatible antecedents in their declared family.

## History

### 2026-10-30 · document

Partial source-bound model, 2026-10-30. The reference is the exact sibling Rust source. The domain is valid finite typed values reachable through checked World::apply, except explicitly constructed graph/helper fixtures. Public mutable graph/frame fields are not defended against arbitrary corruption. No live parser, teacher, corpus, GPU or persistence is exercised.
