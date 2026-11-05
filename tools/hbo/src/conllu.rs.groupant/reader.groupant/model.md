# hbo-conllu-reader

## descriptant

### Descriptant

Measured epoch: bounded supplied fixtures passed twice in debug and release; six behavioral mutants were rejected after compilation, each restored exactly. These observations support the scoped equations without extending them to corpus accuracy, semantic understanding or persistent learning. Production code bytes are unchanged; only isolated tests appended. No benchmark or percentage surrogate was introduced.

Retained caveats: Finite supplied-input checks only; no corpus, training, external annotator, GPU, throughput or semantic-understanding claim. Allocation exhaustion and malformed arbitrary model arithmetic are not exhaustively exercised. No useful numerical surrogate is implemented; future caps may only be 1/2/5/10% with a declared metric.

## approximant

### Forward relation

Read loops blank/comments/column check/MWT/ordinary rows in that order, maintaining owed and cur.

Domain: b+1 representable, owed consumes accepted ordinary rows only; EOF synthetic blank flushes.

## models

# Typed model catalog

## behavior_complement

Malformed-width and decimal rows cannot decrement owed or append a word.

## reverse_reconstruction

Same decoded tokens admit arbitrarily many skipped rows and nonnumeric ordinary IDs.

## blank_reset

Blank drops comment-only cur and resets owed even for unfinished MWT.

## range_replace

New range pushes a token and overwrites owed; no nested range validation.

## comment_order

Comments append in encounter order and do not consume owed.

## owed_count

owed=b+1-min(a,b+1); valid reverse ranges yield0 unless b+1 overflow.

## tests

### Bounded evidence

Primary actual-code fixture: conllu::wave32::lossy_rows_ids_and_empty_tokens. Run using cargo test --offline --locked -p hbo --lib conllu::wave32::lossy_rows_ids_and_empty_tokens -- --exact --test-threads=1; repeat with --release. The external wave32 Hbo receipt set contains all 14 named outcomes twice per profile, numeric exit statuses, six compiled behavioral source negatives and byte-exact restoration hashes, and two identical source syntax projections. Source-local primary tests include assertions and caught expected panics. Fragments share these source witnesses; no claim of one test per branch.

Finite supplied-input checks only; no corpus, training, external annotator, GPU, throughput or semantic-understanding claim. Allocation exhaustion and malformed arbitrary model arithmetic are not exhaustively exercised. No useful numerical surrogate is implemented; future caps may only be 1/2/5/10% with a declared metric.

## invariant-and-cost

### Invariant and cost

b+1 representable, owed consumes accepted ordinary rows only; EOF synthetic blank flushes.

#### Cost

O(B) text scan/copy and O(B) retained state.

Resources must permit allocations; cost variables are source bytes, units, words and table entries as explicitly stated. No measured speed or universal error claim.

## History

### 2026-10-31 · descriptant

2026-10-31, prospective epoch: inspected source before edits and predicted Read loops blank/comments/column check/MWT/ordinary rows in that order, maintaining owed and cur.

### Undated · approximant

Source snapshot SHA-256: a29f83c67bb7f75cadfda6aea9d1675b0ae1ea188862ac8c3a6453295447f4f7. This is a partial source-derived mathematical relation, not an executable surrogate. Tests exercise actual production helpers with synthetic inputs.

### Undated · document

Subject: hbo-conllu-reader at SHA-256 a29f83c67bb7f75cadfda6aea9d1675b0ae1ea188862ac8c3a6453295447f4f7. Domain and fixed state: b+1 representable, owed consumes accepted ordinary rows only; EOF synthetic blank flushes.. Observation: return values, stated mutations and failure order of this scope. Models are partial source-derived relations; finite tests can falsify them but do not prove general completeness. Behavior complement is a scoped exclusion claim in the admitted transition universe. Reverse reconstruction retains compatible antecedents, not a unique historical cause. Specialized entries hold this domain fixed and isolate their named observation. Oracle: named actual-code fixtures in model.md#tests; unexecuted effect boundaries remain static hypotheses.
