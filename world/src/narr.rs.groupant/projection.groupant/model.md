# world-narr-projection

## forward

Past projection strips DYNAMIC fields and PLACE_SETS, collapses history to one project state and clears mentions. Merge first imports missing events, then Past-only recreation and unknown-field updates, with per-command snapshots and warnings.

## invariant

Projection preserves entity Id and supported stable current fields; it loses versions, dynamic values and mentions. Existing parent values are not blanket overwritten.

## cost

Materialized snapshots and retained fields scale with entity histories; up to one cloned parent per reconstructed command.

## composition

projection_erases_dynamic_and_preserves_identity tests actual entity snapshot projection. Full merge recreation gate coverage remains unknown.

## checks

Native narr::wave9_models tests exercise the bounded domains described in ../model.md#evidence at the file root. Ordinary test bodies have partial syntax projections; macro token contents are opaque. The graph oracle uses a dense three-state Floyd closure, not the implementation BFS/bitset algorithm. Typed commands call the real public World::apply boundary.

## epoch-memory

Initial modeling epoch: distinguish admitted graph states from arbitrary public-field edits, and internal mutation from public commit. No production bug was established. In particular narr::relate restores its multi-edge batch, and World::apply discards a cloned failed command even when apply_root has already added a cause edge. Creation ordering has a deliberate 64-round fallback, not an unrestricted topological-sort guarantee. No percentage-error approximant or performance claim.

## behavior-complement

Subject: world-narr-projection, source-bound revision. Domain and observation: the finite typed state/transition family in forward, with dependencies fixed. Operator: Projected state cannot retain Field::At or place-derived sets. New/Branch frame merge cannot run Past entity recreation, though events are imported.

Assurance: source-derived partial relation, with finite native evidence where named in checks. Unknown paths remain unknown; no universal coverage or unique inverse is inferred. Complement claims are inner absent-behavior bounds under their stated premises; reverse relations retain all compatible antecedents in their declared family.

## reverse-reconstruction

Subject: world-narr-projection, source-bound revision. Domain and observation: the finite typed state/transition family in forward, with dependencies fixed. Operator: A projected entity admits every history having the same retained final fields/sets and id, regardless of erased dynamic values, causes and mentions. Reconstruction cannot recover that lost history.

Assurance: source-derived partial relation, with finite native evidence where named in checks. Unknown paths remain unknown; no universal coverage or unique inverse is inferred. Complement claims are inner absent-behavior bounds under their stated premises; reverse relations retain all compatible antecedents in their declared family.

## coupling

Subject: world-narr-projection, source-bound revision. Domain and observation: the finite typed state/transition family in forward, with dependencies fixed. Operator: projection_erases_dynamic_and_preserves_identity tests actual entity snapshot projection. Full merge recreation gate coverage remains unknown.

Assurance: source-derived partial relation, with finite native evidence where named in checks. Unknown paths remain unknown; no universal coverage or unique inverse is inferred. Complement claims are inner absent-behavior bounds under their stated premises; reverse relations retain all compatible antecedents in their declared family.

## History

### 2026-10-30 · document

Partial source-bound model, 2026-10-30. The reference is the exact sibling Rust source. The domain is valid finite typed values reachable through checked World::apply, except explicitly constructed graph/helper fixtures. Public mutable graph/frame fields are not defended against arbitrary corruption. No live parser, teacher, corpus, GPU or persistence is exercised.
