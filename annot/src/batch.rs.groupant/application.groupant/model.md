# annot-batch-application

## descriptant

### annot-batch-application

apply uses indexed chunks and calls shared gates/finish; blind rewrites requests while clearing corpus and hints.

## approximant

### Scoped forward relation

apply uses indexed chunks and calls shared gates/finish; blind rewrites requests while clearing corpus and hints.

Parent composition and symbolic resource costs remain in ../model.md#approximant. No executable percentage surrogate is supplied for this categorical/effect scope.

## invariant

### Scoped fragment: application

Local scope: apply uses indexed chunks and calls shared gates/finish; blind rewrites requests while clearing corpus and hints.

The following parent relations supply composition context; claims apply only where this byte scope implements the stated operation. Other operations remain sibling/parent obligations.

### Invariants and limits

With valid unique item ids, complete matching drafts and in-range chunks, request k refers to exactly that chunk. Parsing skips thinking blocks and preserves text-block concatenation order. These assumptions are unchecked on deserialized State; indexing can panic. The two preparation writes are not atomic together.

- No prepare/apply/blind end-to-end fixture; optional training, corpus append and external teacher are unexecuted.
- No corrupt State index/panic or numeric overflow injection.

## models

# Scoped fragment: application

Local scope: apply uses indexed chunks and calls shared gates/finish; blind rewrites requests while clearing corpus and hints.

The following parent relations supply composition context; claims apply only where this byte scope implements the stated operation. Other operations remain sibling/parent obligations.

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

### Scoped fragment: application

Local scope: apply uses indexed chunks and calls shared gates/finish; blind rewrites requests while clearing corpus and hints.

The following parent relations supply composition context; claims apply only where this byte scope implements the stated operation. Other operations remain sibling/parent obligations.

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

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: scoped source-derived model. Byte scope [10467,16209); AST is shared partial projection, not a separate semantic proof.
