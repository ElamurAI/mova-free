# hbo-text

## descriptant

### Descriptant

Measured epoch: bounded supplied fixtures passed twice in debug and release; six behavioral mutants were rejected after compilation, each restored exactly. These observations support the scoped equations without extending them to corpus accuracy, semantic understanding or persistent learning. Production code bytes are unchanged; only isolated tests appended. No benchmark or percentage surrogate was introduced.

Retained caveats: Finite supplied-input checks only; no corpus, training, external annotator, GPU, throughput or semantic-understanding claim. Allocation exhaustion and malformed arbitrary model arithmetic are not exhaustively exercised. No useful numerical surrogate is implemented; future caps may only be 1/2/5/10% with a declared metric.

## approximant

### Forward relation

norm filters Hebrew accent/joiner set and optionally points, then NFC. units slices original UTF-8 at each noncombining scalar; tokenize NFC-normalizes, splits whitespace/punctuation/maqaf and groups after closing marks plus paragraph markers. thin chooses points by raw-byte hash and scalar position.

Domain: Units partition exact original bytes; coordinates are neither Unicode scalar positions nor bytes. Point removal preserves nonpoint/nonaccent scalars before NFC. Leading marks form their own first unit. Hash overflow is explicit wrapping.

## composition

### Local composition

norm filters Hebrew accent/joiner set and optionally points, then NFC. units slices original UTF-8 at each noncombining scalar; tokenize NFC-normalizes, splits whitespace/punctuation/maqaf and groups after closing marks plus paragraph markers. thin chooses points by raw-byte hash and scalar position.

Nested scopes isolate the transitions described in their manifests and share the whole-source syntax projection. Other declarations and test helpers remain represented by the parent model. Cross-file joins: text units/norm feed seg Piece coordinates; conllu Token/Word carries segmenter and lattice representations; library cut_as realizes lattice callbacks; main routes supplied helpers and external workflows. These joins do not validate arbitrary external parser alignment.

## models

# Typed model catalog

## behavior_complement

norm cannot retain selected accent/joiner characters; unpointed cannot retain selected points. tokenize cannot cut apostrophes inside a word merely because is_mark recognizes a standalone apostrophe.

## reverse_reconstruction

A normalized string admits all filtered-mark insertions and canonically equivalent original strings. Token sentences additionally lose whitespace spelling; units plus original slices reconstruct exact bytes, unlike normalization.

## unit_coordinates

units emits contiguous byte ranges from char_indices. Combining marks join the previous unit when one exists, and maqaf is a separate unit although tokenize keeps it on the left token.

## thin_threshold

For hash-derived u in [0,1), keep point iff u>=share: share0 keeps all, share1 drops all, NaN drops all. Equal inputs/seed repeat; this is deterministic thinning, not an independent random sample.

## sentence_delimiter

Colon also closes a sentence; following standalone פ/ס attach to it. Maqaf appends left and forces token emission without ending sentence. Empty whitespace yields no sentences.

## normalization_level

Pointed removes accents but retains vowels/dagesh/dots; Unpointed removes both sets. Level parse admits 'none' as Unpointed, unknown labels return None; is_pointed tests raw selected points.

## tests

### Bounded evidence

Primary actual-code fixture: text::wave32::coordinates_thinning_and_sentence_edges. Run using cargo test --offline --locked -p hbo --lib text::wave32::coordinates_thinning_and_sentence_edges -- --exact --test-threads=1; repeat with --release. The external wave32 Hbo receipt set contains all 14 named outcomes twice per profile, numeric exit statuses, six compiled behavioral source negatives and byte-exact restoration hashes, and two identical source syntax projections. Source-local primary tests include assertions and caught expected panics. Fragments share these source witnesses; no claim of one test per branch.

Finite supplied-input checks only; no corpus, training, external annotator, GPU, throughput or semantic-understanding claim. Allocation exhaustion and malformed arbitrary model arithmetic are not exhaustively exercised. No useful numerical surrogate is implemented; future caps may only be 1/2/5/10% with a declared metric.

## invariant-and-cost

### Invariant and cost

Units partition exact original bytes; coordinates are neither Unicode scalar positions nor bytes. Point removal preserves nonpoint/nonaccent scalars before NFC. Leading marks form their own first unit. Hash overflow is explicit wrapping.

#### Cost

B bytes,C scalars,T tokens: scanning O(B), NFC O(C+combining-order work), units O(C) time/O(C) slice pointers; tokenize stores O(B+T). Pathological combining sequences may increase normalization work.

Resources must permit allocations; cost variables are source bytes, units, words and table entries as explicitly stated. No measured speed or universal error claim.

## History

### 2026-10-31 · descriptant

2026-10-31, prospective epoch: inspected source before edits and predicted norm filters Hebrew accent/joiner set and optionally points, then NFC. units slices original UTF-8 at each noncombining scalar; tokenize NFC-normalizes, splits whitespace/punctuation/maqaf and groups after closing marks plus paragraph markers. thin chooses points by raw-byte hash and scalar position.

### Undated · approximant

Source snapshot SHA-256: a123d548879eaa2575f9ca17c5836bab9068a0615ce5dd11f996afaa579e44b7. This is a partial source-derived mathematical relation, not an executable surrogate. Tests exercise actual production helpers with synthetic inputs.

### Undated · document

Subject: hbo-text at SHA-256 a123d548879eaa2575f9ca17c5836bab9068a0615ce5dd11f996afaa579e44b7. Domain and fixed state: Units partition exact original bytes; coordinates are neither Unicode scalar positions nor bytes. Point removal preserves nonpoint/nonaccent scalars before NFC. Leading marks form their own first unit. Hash overflow is explicit wrapping.. Observation: return values, stated mutations and failure order of this scope. Models are partial source-derived relations; finite tests can falsify them but do not prove general completeness. Behavior complement is a scoped exclusion claim in the admitted transition universe. Reverse reconstruction retains compatible antecedents, not a unique historical cause. Specialized entries hold this domain fixed and isolate their named observation. Oracle: named actual-code fixtures in model.md#tests; unexecuted effect boundaries remain static hypotheses.
