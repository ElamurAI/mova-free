# annot-batch

## descriptant

### annot-batch

Prepared patch requests and historical reply envelopes feed the same Outcome patch gates as direct annotation. State persists model, effort, training paths, corpus target, ordered items/drafts and request index chunks. Blind preparation clears corpus/hints but retains drafts for comparison.

## approximant

### Forward approximant

prepare validates teacher model, requires patch flow, loads hints, chunks at batch_size>=1, encodes fixed/variable blocks and then writes requests before state. parse_results maps each nonblank JSON line to id and success Reply or error; duplicate ids overwrite. apply looks up chunk index strings, parses blind blocks or adds patch replies, then invokes finish. Missing/errored replies refuse all their chunk items.

Cost: For B request/result bytes, R unique replies and I items: encoding O(B), ordered result map O(B+R log R), retained state O(B+I+draft tokens). apply adds downstream gate and optional training/finish costs. u64 usage addition and usize indices lack overflow guards. USD is weighted tokens/1000000 times 0.5, f64 rounded; no elapsed-speed claim.

Executable approximation gap: categorical parsing, effects and validation lack a useful measured percent metric here. No executable surrogate or 1/2/5/10% profile is attached; an exact test is not an approximant.

## invariant

### Invariants and limits

With valid unique item ids, complete matching drafts and in-range chunks, request k refers to exactly that chunk. Parsing skips thinking blocks and preserves text-block concatenation order. These assumptions are unchecked on deserialized State; indexing can panic. The two preparation writes are not atomic together.

- No prepare/apply/blind end-to-end fixture; optional training, corpus append and external teacher are unexecuted.
- No corrupt State index/panic or numeric overflow injection.

## composition

### Source composition

Nested scopes use UTF-8 byte offsets [start,end) against the exact source SHA-256. They share the parent lexical projection; no nested fragment becomes a second registry source. Source outside these scopes, including helper types/tests, remains parent-modeled.

- envelopes [1809,6541): request builds the compatibility envelope; parse_results consumes replies with text-only, last-id-wins semantics.
- accounting [6938,10467): Historical pricing and cost report aggregate successful usage, count failed/cut replies and retain model-dependent compatibility zero.
- application [10467,16209): apply uses indexed chunks and calls shared gates/finish; blind rewrites requests while clearing corpus and hints.

prepare validates teacher model, requires patch flow, loads hints, chunks at batch_size>=1, encodes fixed/variable blocks and then writes requests before state. parse_results maps each nonblank JSON line to id and success Reply or error; duplicate ids overwrite. apply looks up chunk index strings, parses blind blocks or adds patch replies, then invokes finish. Missing/errored replies refuse all their chunk items.

Cross-file join: ann.rs supplies Ann/Item, parse/check/write; main supplies Outcome and finish; batch and llm prompts feed codex_batch; hints and repair influence draft content before gates. These joins do not establish remote correctness or crash durability.

## models

## behavior_complement

For every succeeded Reply, text excludes non-text content and usage missing/non-u64 fields contributes zero. Failed result types cannot become success Reply. Unknown pricing model has compatibility zero; no subscription cost inference. prepare rejects non-Codex model before flow checking or output writes.

## reverse_reconstruction

The preimage of a result map includes any blank lines, any overwritten earlier duplicate ids, and arbitrary discarded thinking blocks; final occurrences must decode to the observed Reply/errors. State roundtrip recovers serialized fields, not external files or teacher execution. Cost alone has many usage/model antecedents.

## reply_overwrite

Result ids are BTreeMap keys; last line for each id wins, while any malformed earlier JSON aborts the whole parse.

## pricing_threshold

Haiku5.5 prompt=input+cache_read+cache_write uses strict >100000. Dated eight-digit suffix is removed before lookup. Unknown model returns 0; arithmetic has no adversarial-overflow assurance.

## preparation_effects

Model then patch-flow then hint-load failure precedence precedes serialization and request/state writes. Empty items makes empty JSONL and zero chunks. Missing drafts panic before writes.

## timestamp_projection

epoch reads fixed numeric slices and optional decimal fraction; it does not validate separators/calendar/timezone. processing_time may be negative and missing status returns None. Seconds are f64, not measured worker duration.


## tests

### Bounded source evidence

Selected actual-code tests (twice debug, twice release):

- batch::tests::request_has_cached_fixed_block_and_same_prompt
- batch::tests::results_parse_and_patch
- batch::tests::state_roundtrip_and_epoch
- batch::wave27_tests::duplicate_result_last_wins_and_price_boundary

Run command: cargo test -p annot --bin annot --test codex_batches [--release] -- --test-threads=1 --skip ann:: --skip bfs_transfer_checks. The fake executable fixture uses temporary files, no live teacher/API. Paired named outcomes must match. Six compiled production mutations target batch last-wins, llm closed-set, repair shape gate, hints severity, main seed alias and queue resume. Each compilation exits0, test exits101, exact source restored and focused positive rerun exits0. The fixture itself has no separate mutation. Existing repair test may early-return without dictionary; independent added repair test always runs.

Unexecuted limits:
- No prepare/apply/blind end-to-end fixture; optional training, corpus append and external teacher are unexecuted.
- No corrupt State index/panic or numeric overflow injection.

Validation limits: strict scoped Clippy exited101 on six pre-existing lints (three collapsible_if, one needless_range_loop, one type_complexity, one redundant_locals). Re-running with those four known lint categories allowed exited0. changed with an external augmented registry exited0 and found no missing changed-source models. Local groupants inspection reports only expected ancestor child/source-binding updates owned by integration; no owned leaf/fragment diagnostics.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: source read before tests/models. Prospective notes and numeric receipts retained externally under the annotation wave27 run. Four new inline tests supplement existing bounded tests. Preserve distinctions: compatibility cost is not subscription cost; mock process tests do not establish live teacher behavior; registry presence is partial modeling.
