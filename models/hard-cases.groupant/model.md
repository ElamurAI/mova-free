# hard-cases

## Descriptant

The loop from reading failures to learning: the one shared hard-cases list (`tools/hard-case-list`, format `mova-hard-case-v2`, written by the Wikipedia corpus and the topic corpora) fed to the Fish in a priority lane (`tools/hard-cases` wave, guard), validated replies harvested per licence track and document split, the three open witnesses run, one Eel round per track through the public engine, and the list re-checked and a fixed sample measured with every new gparse model.

## Forward

list (found per source, per-parser views) -> recheck (gparse view) -> wave (select, bundle, task, pack, pages, pager configs, ledger) -> fish-pages pagers (outside: Terra, review-only) -> harvest (learn/<track>/<split>) -> witness (Stanza, Trankit, spaCy trf; agreed input) -> eel (engine round: agreement, held-out, dev, EWT, install) -> new weights -> watch: measure (fixed sample) + recheck.

## Invariant

Nothing here calls a teacher, marks an annotation gold or authorizes training by itself; licences decide tracks (CC BY-SA internal by default, NC/ND/research/unknown measure-only); documents decide splits; a case is sent to the Fish once (ledger); the internal track never installs into the deployed model directory; held-out texts are excluded at selection, packing and harvest.

## Cost

Wave: one bundle command per document and one gparse CPU pass (4 s for 131 cases). Recheck/measure: one gparse and one Stanza CPU pass (about 10-30 s per 131 sentences). Witness: three neural parsers under the GPU lease. Eel: the engine's CUDA round.

## Composition

Leaves: `tools/hard-case-list/src/lib.rs` and the ten `tools/hard-cases/src/*.rs` file groupants. The wiring checks: a scratch run on a copy of the live list (sync, recheck, wave, real fish-pages acceptance with a failing fake Fish, witness, treeagree on the witness output, measurements of four gparse models). Interfaces: `wikicorpus` (`hard-sync`, `bundle`, sync after check/recheck), `mova-topic-corpus` (`bundle`, v2 records, source-scoped recheck), `mova-fish-batch` (pack), `mova-fish-pages` (input contract), `mova-eel` (`TrainingData`, `fresh_train_prefixes`). Parent: `crab-orchestration`.

## behavior_complement

Domain: all configured runs. It does not: call Terra or any model, start or restart pagers, import annotations into the parse database, train without the Eel engine's gates, install the internal model where the shipped one lives, ship or publish CC BY-SA material, judge correctness (only disagreement and broken constraints), or touch held-out test sets. Counterexamples covered by tests: a topic-corpus recheck never learns a Wikipedia case; a restricted licence never reaches a training track; an uncertain lane pager stops every pager.

## reverse_reconstruction

From the lane root: the ledger names every selected case and its fate; each wave keeps its bundles, tasks, queues and ready manifests; learn/annotations.jsonl points to wave, job and receipt; measure/runs keeps per-case verdicts per model. The list before a sync is recoverable from its own prefix (append-only). No unique inverse from a model to the cases that trained it beyond the engine's generation receipts.

## Tests

`cargo test --manifest-path tools/hard-case-list/Cargo.toml --offline` (3) and `cargo test --manifest-path tools/hard-cases/Cargo.toml --offline` (9), two identical runs; the Eel engine's prefix test; wikicorpus `list_records_point_back_to_their_article`; topic-corpus `found_once_open_until_learned`. Finite fixtures and one scratch run; not a proof.

## History

### 2026-11-04 · Tests

`cargo test --manifest-path tools/hard-case-list/Cargo.toml --offline` (3) and `cargo test --manifest-path tools/hard-cases/Cargo.toml --offline` (9), 2026-11-04, two identical runs; the Eel engine's prefix test; wikicorpus `list_records_point_back_to_their_article`; topic-corpus `found_once_open_until_learned`. Finite fixtures and one scratch run; not a proof.
