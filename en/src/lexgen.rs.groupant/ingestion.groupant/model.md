# en-lexgen-ingestion

## descriptant

### en-lexgen-ingestion
UD triples aggregate first occurrence and counts; AGID text is read before output effects. Main cell membership, cleanliness and doubtful headers determine a/ar/r flags. Three-field verbs share Past with PastPart.

## approximant

### Scoped forward relation
UD triples aggregate first occurrence and counts; AGID text is read before output effects. Main cell membership, cleanliness and doubtful headers determine a/ar/r flags. Three-field verbs share Past with PastPart.
No executable percentage approximant attached.

## invariant

### Scoped assumptions and costs
No atomicity; absent AGID fails before writes; duplicate old heap rows retained, last ID wins; no TSV escaping or u32 index cap.
O(bytes+U log U+A log A+output); retained rows/heap linear; two sequential buffered files.

## composition

### Parent composition
UD triples aggregate first occurrence and counts; AGID text is read before output effects. Main cell membership, cleanliness and doubtful headers determine a/ar/r flags. Three-field verbs share Past with PastPart.
Parent combines this fragment with the other named source fragments and helpers outside their ranges. No coverage claim for unscoped code.

## models

## behavior_complement
Within this fragment's byte scope: UD triples aggregate first occurrence and counts; AGID text is read before output effects. Main cell membership, cleanliness and doubtful headers determine a/ar/r flags. Three-field verbs share Past with PastPart.
Parent relation constraining compatible states: No atomicity; absent AGID fails before writes; duplicate old heap rows retained, last ID wins; no TSV escaping or u32 index cap.

## reverse_reconstruction
Within this fragment's byte scope: UD triples aggregate first occurrence and counts; AGID text is read before output effects. Main cell membership, cleanliness and doubtful headers determine a/ar/r flags. Three-field verbs share Past with PastPart.
Parent relation constraining compatible states: Outputs admit input changes preserving counts/first occurrence; old heap prefix retained but flags recomputed; failures admit partially written files.

## effect_order
Within this fragment's byte scope: UD triples aggregate first occurrence and counts; AGID text is read before output effects. Main cell membership, cleanliness and doubtful headers determine a/ar/r flags. Three-field verbs share Past with PastPart.
Parent relation constraining compatible states: Missing AGID read errors precede output directory/write; lemmas file truncates and flushes before forms is created, so later failure leaves partial update.

## append_identity
Within this fragment's byte scope: UD triples aggregate first occurrence and counts; AGID text is read before output effects. Main cell membership, cleanliness and doubtful headers determine a/ar/r flags. Three-field verbs share Past with PastPart.
Parent relation constraining compatible states: Existing heap order retained, duplicate strings map last index; newly observed lemmas append, never reorder old rows.

## variant_precedence
Within this fragment's byte scope: UD triples aggregate first occurrence and counts; AGID text is read before output effects. Main cell membership, cleanliness and doubtful headers determine a/ar/r flags. Three-field verbs share Past with PastPart.
Parent relation constraining compatible states: First clean comma variant wins; if no clean use first nonempty dirty; questionable POS clears clean for all forms.

## frequency_ties
Within this fragment's byte scope: UD triples aggregate first occurrence and counts; AGID text is read before output effects. Main cell membership, cleanliness and doubtful headers determine a/ar/r flags. Three-field verbs share Past with PastPart.
Parent relation constraining compatible states: UD lemmas sort descending frequency then lexical, lowercase companion appended immediately; AGID remainder sorted/deduped lexically.


## tests

### Bounded parent evidence
See ../model.md#tests. Actual check: lexgen::wave27_tests::synthetic_files_preserve_ids_and_source_flags; shared source evidence, not independent exhaustive fragment testing.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: source-scoped partial model; parent tests provide finite shared evidence.
