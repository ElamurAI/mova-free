# annot-llm

## descriptant

### annot-llm

Teacher prompt construction, cached optional context/pragmatic labels, closed pragmatic value validation and call accounting. Production transport delegates to shared Codex CLI; prompt generation remains deterministic only for fixed registry/environment cache state.

## approximant

### Forward approximant

annotate_prompt returns registry rules and numbered forms; patch_prompt appends draft blocks, optional context/prag/hints with instructions. world_context and prag_labels initialize OnceLock once from environment files, swallowing missing/unreadable input into empty maps. Opus::ask creates cwd then calls teacher and records usage/time. log_calls serializes lines then writes once.

Cost: Prompt construction O(registry bytes+input draft/text/hint/context output bytes) plus map lookup O(log context rows) per item. OnceLock loading retains file maps for process lifetime. log_calls O(serialized bytes). Transport runtime and token counts are unbounded externally; elapsed seconds are wall-clock f64.

Executable approximation gap: categorical parsing, effects and validation lack a useful measured percent metric here. No executable surrogate or 1/2/5/10% profile is attached; an exact test is not an approximant.

## invariant

### Invariants and limits

Fixed forms are enumerated one-based in input order and are never tokenized by prompt builder. prag_ok accepts only case-sensitive listed values; loading labels does not itself enforce prag_ok. Missing numeric usage defaults zero; cost field is reported input, not independently priced.

- Live ask, adjudicate/correct transport, environment cache variants and log write failures are unexecuted.
- Categorical prompts/schema have no useful percent surrogate.

## composition

### Source composition

Nested scopes use UTF-8 byte offsets [start,end) against the exact source SHA-256. They share the parent lexical projection; no nested fragment becomes a second registry source. Source outside these scopes, including helper types/tests, remains parent-modeled.

- calls [844,5358): Call methods construct flow-specific prompts and delegate process effects/accounting to ask.
- context [6285,9238): World/context and pragmatic label OnceLocks parse optional TSV maps, while prag_ok separately enforces closed value domains.
- prompt [9444,15373): Patch prompt combines registry rules and ordered item drafts with optional context, pragmatic labels and advisory hints.

annotate_prompt returns registry rules and numbered forms; patch_prompt appends draft blocks, optional context/prag/hints with instructions. world_context and prag_labels initialize OnceLock once from environment files, swallowing missing/unreadable input into empty maps. Opus::ask creates cwd then calls teacher and records usage/time. log_calls serializes lines then writes once.

Cross-file join: ann.rs supplies Ann/Item, parse/check/write; main supplies Outcome and finish; batch and llm prompts feed codex_batch; hints and repair influence draft content before gates. These joins do not establish remote correctness or crash durability.

## models

## behavior_complement

prag_ok excludes unknown fields and out-of-set values, including uppercase variants. annotate_prompt excludes Item.text and gold from variable token list. Pure prompt construction performs no teacher call; the Opus call methods do.

## reverse_reconstruction

A variable annotation prompt constrains ordered ids/forms but has arbitrary compatible Item.text/gold fields. Cached context map has many antecedent TSVs because duplicate ids overwrite and malformed lines disappear. Reply/accounting does not uniquely identify prompt or remote execution.

## schema_domains

form has8, act16, polarity3, voice3 values; indirect/hedge accept exactly 0 or1. Empty or unknown values reject.

## cache_epoch

First read fixes each OnceLock for process lifetime. Later environment/file edits do not update it; failed read also caches empty.

## prompt_transport

Direct methods concatenate fixed+variable; batch queue joins message text blocks with two newlines. They carry equivalent sections but are not claimed byte-identical prompts.

## accounting_effects

mkdir occurs before teacher validation/launch inside transport. Successful ask reads result string or empty and u64 usage or zero, wall seconds. log_calls serializes entire buffer before write, but write itself is not transactional.


## tests

### Bounded source evidence

Selected actual-code tests (twice debug, twice release):

- llm::wave27_tests::closed_pragmatic_domains_and_fixed_tokens
- batch::tests::request_has_cached_fixed_block_and_same_prompt

Run command: cargo test -p annot --bin annot --test codex_batches [--release] -- --test-threads=1 --skip ann:: --skip bfs_transfer_checks. The fake executable fixture uses temporary files, no live teacher/API. Paired named outcomes must match. Six compiled production mutations target batch last-wins, llm closed-set, repair shape gate, hints severity, main seed alias and queue resume. Each compilation exits0, test exits101, exact source restored and focused positive rerun exits0. The fixture itself has no separate mutation. Existing repair test may early-return without dictionary; independent added repair test always runs.

Unexecuted limits:
- Live ask, adjudicate/correct transport, environment cache variants and log write failures are unexecuted.
- Categorical prompts/schema have no useful percent surrogate.

Validation limits: strict scoped Clippy exited101 on six pre-existing lints (three collapsible_if, one needless_range_loop, one type_complexity, one redundant_locals). Re-running with those four known lint categories allowed exited0. changed with an external augmented registry exited0 and found no missing changed-source models. Local groupants inspection reports only expected ancestor child/source-binding updates owned by integration; no owned leaf/fragment diagnostics.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: source read before tests/models. Prospective notes and numeric receipts retained externally under the annotation wave27 run. Four new inline tests supplement existing bounded tests. Preserve distinctions: compatibility cost is not subscription cost; mock process tests do not establish live teacher behavior; registry presence is partial modeling.
