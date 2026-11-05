# world-store-lexical-projection

## descriptant

# world-store-lexical-projection

## Forward

Structural features count UPOS/relation and selected predicates/subjects with u8 saturation. Lexical features add subj:lemma only for Nsubj/NsubjPass and alphabetic lemma strings resolved from the version table. index_full rebuilds these observations from effective records, including inherited records.

## Invariant

Inherited IDs must keep their parent meaning in the child lemma table. Valid dense unique canonical tables suffice; sparse/duplicate table construction and ID-width overflow remain separate risks. Csubj contributes structural subj but no lexical subj:lemma. Empty strings pass all(is_alphabetic) vacuously.

## Cost

Per tree O(T+B) feature traversal plus ordered-map costs; full index rebuild visits all sentence IDs and materializes postings. Identity improvements do not eliminate rebuilding.

## Epoch memory

## catalog

# world-store-lexical-projection

## nonbehavior

Type: behavior_complement.

Universe: valid decoded trees, fixed lemma table and feature-count maps. Inner complement excludes subj:alice from a tree with no Nsubj/NsubjPass token resolving to alice; excludes counts above 255; excludes tag-only changes changing this feature map. This projection ignores tags and heads, so record inequality need not change any index feature.

## antecedents

Type: reverse_reconstruction.

A feature map admits every token sequence whose structural and lexical counts saturate to those observations. Token order, heads and tags are generally free. Count 255 admits every unsaturated count at least 255. A missing lexical feature may mean no subject, nonalphabetic lemma, absent ID, or a different relation; no unique lexical tree follows.

## table-extension

Type: dependency_preservation.

If a child table agrees with the parent on every inherited ID, inherited records preserve lexical features. Agreement of numeric IDs alone is insufficient. Changed-ID completeness fails if a child reinterprets inherited IDs without listing those sentences; canonical dense table extension is an explicit condition, not a general file-format guarantee.

## feature-quotient

Type: information_loss.

The feature map is coarser than stored semantic identity: changing a tag, a head, a non-subject lemma or a saturated excess count may preserve all postings. Therefore the delta dictionary can be a strict superset of feature-changing sentences. Such extra IDs preserve carry correctness but increase work.

## History

### 2026-10-29 · Epoch memory

2026-10-29: distinguish complete stored semantic identity from full Word equality and the coarser feature projection. The historical omission of lemma/tag comparison is retained as a rejected model. Source tests use real enum/decoder paths; the older extracted probe's scalar decoder cannot establish that correspondence. No live parser or store run.

### Undated · document

Source SHA-256 6801677c4f17273e615090a790f2e4018387ca5d006aab899bafda09f01b24d9. Source-derived conditional models; finite algebra evidence is separate from actual source tests.
