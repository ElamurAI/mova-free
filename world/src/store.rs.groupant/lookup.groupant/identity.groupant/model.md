# world-store-record-identity

## descriptant

# world-store-record-identity

## Forward

Resolve each stored lemma ID through the immutable parent lemma table. Compare token count and the sequence (UPOS, relation, head, tag, normalized lemma) against the new annotation. Word form and feats are outside the stored projection. Numeric lemma IDs are representation coordinates, not lexical identity.

## Invariant

Valid enum bytes, representable heads and lemma IDs, canonical parent lemmas, fixed parent table and successful record reads. A true identity result preserves all stored semantic fields; it does not preserve omitted Word fields.

## Cost

Worst case O(T+B) for T tokens and B normalized lemma bytes inspected, versus the historical O(T) structural comparison. Short circuiting avoids later tokens; normalization allocates. The immutable parent lemma vector is retained and cloned into the mutable lexical map once, adding O(V+Btable) storage/work for V entries and their bytes. Updating the map and encoding changed trees has separate cost. No timing measurement.

## Epoch memory

## catalog

# world-store-record-identity

## nonbehavior

Type: behavior_complement.

Universe: admitted parent records/table and candidate Word sequence paired with a Boolean identity result. Its complement contains true with any differing stored semantic component or token count, and false for identical normalized projections. This is exact for the declared projection relation, conditional for actual source.

## antecedents

Type: reverse_reconstruction.

For a fixed parent projection P, the true fiber contains all candidate Word sequences with projection P, arbitrary forms/feats and any lemma spelling whose Rust lowercase equals the parent lemma. False contains all remaining admitted candidates. Observing false does not identify which coordinate differs. Missing parent IDs are outside valid identity and conservatively produce false.

## representation-fiber

Type: serialization_projection.

Equivalent semantic records may use different numeric lemma IDs if their respective tables resolve to the same strings. Equal IDs in different tables may denote different strings. Canonical lowercasing loses case and is not an inverse; words reconstructs forms through a tokenizer and defaults feats, so stored records do not recover complete annotations.

## historical-loss

Type: counterfactual.

The historical (UPOS,relation,head)-only predicate merges distinct tag and lemma fibers. Rejecting a lemma-only or tag-only difference repairs this gate. It does not verify update annotation, read-error handling, publication ordering, same-hash early return or full codec capacity.

## History

### 2026-10-29 · Epoch memory

2026-10-29: distinguish complete stored semantic identity from full Word equality and the coarser feature projection. The historical omission of lemma/tag comparison is retained as a rejected model. Source tests use real enum/decoder paths; the older extracted probe's scalar decoder cannot establish that correspondence. No live parser or store run.

### Undated · document

Source SHA-256 6801677c4f17273e615090a790f2e4018387ca5d006aab899bafda09f01b24d9. Source-derived conditional models; finite algebra evidence is separate from actual source tests.
