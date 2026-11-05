# world-store-delta-closure

## descriptant

# world-store-delta-closure

## Forward

Let C be the union of changed sentence IDs along an ancestor chain. If unchanged records retain semantic identity and inherited lemma meanings, feature maps agree outside C. Every pointwise query with fixed metadata therefore has equal membership outside C. The carry algebra then returns fresh membership.

## Invariant

Fixed corpus/metadata, canonical coherent lemma extension, valid acyclic ancestry, complete delta writes, successful reads/indexing, immutable indexes and truthful saved stamps. For chains, each edge needs these conditions. Sample/prefix coupling is handled by reselection, not this pointwise law.

## Cost

Identity inspects every successfully read sentence; index_full still rebuilds all features. Carry computes fresh selection over all IDs then merges only C, so this proof gives correctness conditions and no asymptotic query speedup.

## Epoch memory

## catalog

# world-store-delta-closure

## nonbehavior

Type: behavior_complement.

Universe: admitted pointwise-query transitions satisfying the listed conditions, paired with returned membership. Complement excludes a changed membership outside C and a carried set different from fresh selection. Without delta completeness or table preservation these exclusions are not established.

## antecedents

Type: reverse_reconstruction.

For fresh F, changed C and carried Y, compatible predecessors O satisfy O outside C = Y outside C and F inside C = Y inside C. Correctness Y=F further requires O outside C=F outside C. Prior membership within C is unconstrained; neither old tags nor old lemma values can be reconstructed from the ID set.

## closure-chain

Type: dependency_closure.

On each edge, identity outside its delta implies feature equality there. For an ID outside the union C, edge equality composes transitively, giving endpoint feature equality and pointwise membership equality. A tag-only delta may enlarge C while preserving features. A lemma-only subject change must enter C when normalized strings differ.

## failure-boundary

Type: admission_constraint.

update continues after text or parent-record read errors; index_full skips failed tree reads. Same-hash early return performs no comparison. Invalid/sparse tables, mutation under a fixed hash and a forged stable stamp can violate premises. The helper repair and synthetic index/carry tests do not verify whole update or remove these boundaries.

## History

### 2026-10-29 · Epoch memory

2026-10-29: distinguish complete stored semantic identity from full Word equality and the coarser feature projection. The historical omission of lemma/tag comparison is retained as a rejected model. Source tests use real enum/decoder paths; the older extracted probe's scalar decoder cannot establish that correspondence. No live parser or store run.

### Undated · document

Source SHA-256 6801677c4f17273e615090a790f2e4018387ca5d006aab899bafda09f01b24d9. Source-derived conditional models; finite algebra evidence is separate from actual source tests.
