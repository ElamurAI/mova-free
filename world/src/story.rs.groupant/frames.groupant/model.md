# world-story-frames

## forward

Frame stores narrator/listeners, parent, level, branch/world/line, bounds, open intervals and optional owned world. is_open reads only the final span.

## invariant

No span means closed; root has its own distinguished identity. Parent/world consistency is a caller obligation.

## cost

is_open O(1); frame clone includes owned World cost.

## composition

Frame storage is separate from narr command validation. typed_frames_close_resume_and_world_scope checks open/close/resume via public commands.

## checks

Native story::wave9_models tests exercise the bounded domains described in ../model.md#evidence at the file root. Ordinary test bodies have partial syntax projections; macro token contents are opaque. The graph oracle uses a dense three-state Floyd closure, not the implementation BFS/bitset algorithm. Typed commands call the real public World::apply boundary.

## epoch-memory

Initial modeling epoch: distinguish admitted graph states from arbitrary public-field edits, and internal mutation from public commit. No production bug was established. In particular narr::relate restores its multi-edge batch, and World::apply discards a cloned failed command even when apply_root has already added a cause edge. Creation ordering has a deliberate 64-round fallback, not an unrestricted topological-sort guarantee. No percentage-error approximant or performance claim.

## behavior-complement

Subject: world-story-frames, source-bound revision. Domain and observation: the finite typed state/transition family in forward, with dependencies fixed. Operator: A final closed span cannot be observed as open; name cannot invent a supplied id when id is absent.

Assurance: source-derived partial relation, with finite native evidence where named in checks. Unknown paths remain unknown; no universal coverage or unique inverse is inferred. Complement claims are inner absent-behavior bounds under their stated premises; reverse relations retain all compatible antecedents in their declared family.

## reverse-reconstruction

Subject: world-story-frames, source-bound revision. Domain and observation: the finite typed state/transition family in forward, with dependencies fixed. Operator: An open boolean admits every span sequence with a final None end; earlier spans and narration identity cannot be recovered.

Assurance: source-derived partial relation, with finite native evidence where named in checks. Unknown paths remain unknown; no universal coverage or unique inverse is inferred. Complement claims are inner absent-behavior bounds under their stated premises; reverse relations retain all compatible antecedents in their declared family.

## coupling

Subject: world-story-frames, source-bound revision. Domain and observation: the finite typed state/transition family in forward, with dependencies fixed. Operator: Frame storage is separate from narr command validation. typed_frames_close_resume_and_world_scope checks open/close/resume via public commands.

Assurance: source-derived partial relation, with finite native evidence where named in checks. Unknown paths remain unknown; no universal coverage or unique inverse is inferred. Complement claims are inner absent-behavior bounds under their stated premises; reverse relations retain all compatible antecedents in their declared family.

## History

### 2026-10-30 · document

Partial source-bound model, 2026-10-30. The reference is the exact sibling Rust source. The domain is valid finite typed values reachable through checked World::apply, except explicitly constructed graph/helper fixtures. Public mutable graph/frame fields are not defended against arbitrary corruption. No live parser, teacher, corpus, GPU or persistence is exercised.
