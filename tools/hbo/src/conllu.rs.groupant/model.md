# hbo-conllu

## descriptant

### Descriptant

Measured epoch: bounded supplied fixtures passed twice in debug and release; six behavioral mutants were rejected after compilation, each restored exactly. These observations support the scoped equations without extending them to corpus accuracy, semantic understanding or persistent learning. Production code bytes are unchanged; only isolated tests appended. No benchmark or percentage surrogate was introduced.

Retained caveats: Finite supplied-input checks only; no corpus, training, external annotator, GPU, throughput or semantic-understanding claim. Allocation exhaustion and malformed arbitrary model arithmetic are not exhaustively exercised. No useful numerical surrogate is implemented; future caps may only be 1/2/5/10% with a declared metric.

## approximant

### Forward relation

Read scans lines with owed MWT count; ten-column nondecimal rows create words, range rows open tokens, blank/EOF flush tokenful sentences. Write emits comments and MWT only for >1 words then renumbers word IDs from1, leaving HEAD unchanged.

Domain: UTF-8 strings; representable b+1 and ID/count sums. Rows may carry arbitrary IDs/HEAD; this codec does not validate trees. Empty MWT tokens survive read but emit no row. Comment-only sentences disappear.

## composition

### Local composition

Read scans lines with owed MWT count; ten-column nondecimal rows create words, range rows open tokens, blank/EOF flush tokenful sentences. Write emits comments and MWT only for >1 words then renumbers word IDs from1, leaving HEAD unchanged.

Nested scopes isolate the transitions described in their manifests and share the whole-source syntax projection. Other declarations and test helpers remain represented by the parent model. Cross-file joins: text units/norm feed seg Piece coordinates; conllu Token/Word carries segmenter and lattice representations; library cut_as realizes lattice callbacks; main routes supplied helpers and external workflows. These joins do not validate arbitrary external parser alignment.

## models

# Typed model catalog

## behavior_complement

Within the stated domain, decimal-ID and wrong-column rows cannot create words; write cannot repair HEAD to match changed IDs or preserve empty-token rows. Filesystem reads cannot commit a partial returned vector after Err.

## reverse_reconstruction

A decoded sentence admits any source differing only by skipped malformed/decimal rows or discarded comment-only sentences. A written result admits prior word IDs independently of preserved HEAD; unique original text recovery is impossible.

## owed_word_state

Range a-b sets owed=b+1-min(a,b+1); each accepted ordinary row consumes one regardless of its ID. A new range replaces owed; blank clears it. Overflow is excluded, not normalized.

## id_head_transport

write maps word-position j to ID=j+1 but HEAD remains stored text. A noncanonical input ID8/HEAD9 may become ID1/HEAD9: serialization is not dependency repair.

## empty_token_projection

A token with zero words contributes no row and one-word tokens lose their separate form/misc wrapper. Equality read(write(s))=s requires canonical IDs, nonempty compatible tokens and retained comments.

## misc_key_boundary

misc(k) matches k followed immediately by '=' and returns first pipe field; Key cannot match KeyLong. head parse failure becomes0, conflating malformed values with roots.

## tests

### Bounded evidence

Primary actual-code fixture: conllu::wave32::lossy_rows_ids_and_empty_tokens. Run using cargo test --offline --locked -p hbo --lib conllu::wave32::lossy_rows_ids_and_empty_tokens -- --exact --test-threads=1; repeat with --release. The external wave32 Hbo receipt set contains all 14 named outcomes twice per profile, numeric exit statuses, six compiled behavioral source negatives and byte-exact restoration hashes, and two identical source syntax projections. Source-local primary tests include assertions and caught expected panics. Fragments share these source witnesses; no claim of one test per branch.

Finite supplied-input checks only; no corpus, training, external annotator, GPU, throughput or semantic-understanding claim. Allocation exhaustion and malformed arbitrary model arithmetic are not exhaustively exercised. No useful numerical surrogate is implemented; future caps may only be 1/2/5/10% with a declared metric.

## invariant-and-cost

### Invariant and cost

UTF-8 strings; representable b+1 and ID/count sums. Rows may carry arbitrary IDs/HEAD; this codec does not validate trees. Empty MWT tokens survive read but emit no row. Comment-only sentences disappear.

#### Cost

B input/output bytes, W words: O(B+W) copying and ten columns per word, O(B+W) stored text; read_files holds all decoded sentences and stops first read error.

Resources must permit allocations; cost variables are source bytes, units, words and table entries as explicitly stated. No measured speed or universal error claim.

## History

### 2026-10-31 · descriptant

2026-10-31, prospective epoch: inspected source before edits and predicted Read scans lines with owed MWT count; ten-column nondecimal rows create words, range rows open tokens, blank/EOF flush tokenful sentences. Write emits comments and MWT only for >1 words then renumbers word IDs from1, leaving HEAD unchanged.

### Undated · approximant

Source snapshot SHA-256: a29f83c67bb7f75cadfda6aea9d1675b0ae1ea188862ac8c3a6453295447f4f7. This is a partial source-derived mathematical relation, not an executable surrogate. Tests exercise actual production helpers with synthetic inputs.

### Undated · document

Subject: hbo-conllu at SHA-256 a29f83c67bb7f75cadfda6aea9d1675b0ae1ea188862ac8c3a6453295447f4f7. Domain and fixed state: UTF-8 strings; representable b+1 and ID/count sums. Rows may carry arbitrary IDs/HEAD; this codec does not validate trees. Empty MWT tokens survive read but emit no row. Comment-only sentences disappear.. Observation: return values, stated mutations and failure order of this scope. Models are partial source-derived relations; finite tests can falsify them but do not prove general completeness. Behavior complement is a scoped exclusion claim in the admitted transition universe. Reverse reconstruction retains compatible antecedents, not a unique historical cause. Specialized entries hold this domain fixed and isolate their named observation. Oracle: named actual-code fixtures in model.md#tests; unexecuted effect boundaries remain static hypotheses.
