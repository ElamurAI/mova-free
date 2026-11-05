# hbo-text-tokenization

## descriptant

### Descriptant

Measured epoch: bounded supplied fixtures passed twice in debug and release; six behavioral mutants were rejected after compilation, each restored exactly. These observations support the scoped equations without extending them to corpus accuracy, semantic understanding or persistent learning. Production code bytes are unchanged; only isolated tests appended. No benchmark or percentage surrogate was introduced.

Retained caveats: Finite supplied-input checks only; no corpus, training, external annotator, GPU, throughput or semantic-understanding claim. Allocation exhaustion and malformed arbitrary model arithmetic are not exhaustively exercised. No useful numerical surrogate is implemented; future caps may only be 1/2/5/10% with a declared metric.

## approximant

### Forward relation

NFC then chunk scan emits maqaf-left and punctuation tokens; second pass closes sentences and consumes paragraph markers.

Domain: Token indices refer to normalized tokens; raw whitespace/normalization history discarded.

## models

# Typed model catalog

## behavior_complement

Apostrophe inside a chunk is not split by tokenize; colon does end a sentence.

## reverse_reconstruction

All whitespace and canonically equivalent raw variants yielding same tokens are compatible.

## maqaf_left

Append maqaf and emit current token; repeated maqaf can yield maqaf-only token.

## paragraph_attach

Following פ/ס tokens attach only after an ending token.

## empty_input

No chunks implies no tokens and no sentences.

## nfc_boundary

NFC occurs before character dispatch, so output slices are newly owned normalized strings.

## tests

### Bounded evidence

Primary actual-code fixture: text::wave32::coordinates_thinning_and_sentence_edges. Run using cargo test --offline --locked -p hbo --lib text::wave32::coordinates_thinning_and_sentence_edges -- --exact --test-threads=1; repeat with --release. The external wave32 Hbo receipt set contains all 14 named outcomes twice per profile, numeric exit statuses, six compiled behavioral source negatives and byte-exact restoration hashes, and two identical source syntax projections. Source-local primary tests include assertions and caught expected panics. Fragments share these source witnesses; no claim of one test per branch.

Finite supplied-input checks only; no corpus, training, external annotator, GPU, throughput or semantic-understanding claim. Allocation exhaustion and malformed arbitrary model arithmetic are not exhaustively exercised. No useful numerical surrogate is implemented; future caps may only be 1/2/5/10% with a declared metric.

## invariant-and-cost

### Invariant and cost

Token indices refer to normalized tokens; raw whitespace/normalization history discarded.

#### Cost

O(B+NFC reorder work+T) time and O(B+T) space.

Resources must permit allocations; cost variables are source bytes, units, words and table entries as explicitly stated. No measured speed or universal error claim.

## History

### 2026-10-31 · descriptant

2026-10-31, prospective epoch: inspected source before edits and predicted NFC then chunk scan emits maqaf-left and punctuation tokens; second pass closes sentences and consumes paragraph markers.

### Undated · approximant

Source snapshot SHA-256: a123d548879eaa2575f9ca17c5836bab9068a0615ce5dd11f996afaa579e44b7. This is a partial source-derived mathematical relation, not an executable surrogate. Tests exercise actual production helpers with synthetic inputs.

### Undated · document

Subject: hbo-text-tokenization at SHA-256 a123d548879eaa2575f9ca17c5836bab9068a0615ce5dd11f996afaa579e44b7. Domain and fixed state: Token indices refer to normalized tokens; raw whitespace/normalization history discarded.. Observation: return values, stated mutations and failure order of this scope. Models are partial source-derived relations; finite tests can falsify them but do not prove general completeness. Behavior complement is a scoped exclusion claim in the admitted transition universe. Reverse reconstruction retains compatible antecedents, not a unique historical cause. Specialized entries hold this domain fixed and isolate their named observation. Oracle: named actual-code fixtures in model.md#tests; unexecuted effect boundaries remain static hypotheses.
