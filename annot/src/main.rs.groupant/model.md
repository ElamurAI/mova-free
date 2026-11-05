# annot-main

## descriptant

### annot-main

Annotation CLI dispatcher and shared selection, patch/gate, finish/report pipeline. Models distinguish pure bounded helpers from teacher/training/network/corpus effects. Existing annotation gates are owned by ann.rs and remain a cross-group dependency.

## approximant

### Forward approximant

CLI dispatch selects direct utilities or parses training/input/count/seed/output arguments. Drafts may load saved input or train a model; items are selected and annotated in patch/draft/two-look flows. apply_patches clones drafts, tracks seen/broken ids, mutates selected fields and PRAG side-map, then returns only seen unbroken ann::check successes. finish writes outputs, optionally replaces FEATS and calls UDPipe, appends accepted corpus items, then writes report.

Cost: B input/reply bytes, N tokens, I items: shuffle O(I), patch parse O(B) plus ordered-map lookups/cloning and delegated gates, potentially O(sum token gate work). Corpus reads and reports materialize full strings. Training, expert evaluation, teacher and UDPipe dominate separately. usize counters/letter multiplications unchecked; no hard resource cap.

Executable approximation gap: categorical parsing, effects and validation lack a useful measured percent metric here. No executable surrogate or 1/2/5/10% profile is attached; an exact test is not an approximant.

## invariant

### Invariants and limits

For unique pair ids and valid forms, output patch keys are a subset of supplied ids and every returned annotation passes ann::check. Unknown reply ids are ignored. Failed syntactic patches keep a sentence broken even after OK. PRAG side-map can retain entries for rejected sentences; finish restricts emission to final keys. This is not a transactional pipeline.

- No training, live network, live teacher, corpus/model write or end-to-end finish execution.
- silver n=0, duplicate ids, post-FEATS gate and unchecked indexing limitations modeled without production changes.

## composition

### Source composition

Nested scopes use UTF-8 byte offsets [start,end) against the exact source SHA-256. They share the parent lexical projection; no nested fragment becomes a second registry source. Source outside these scopes, including helper types/tests, remains parent-modeled.

- selection [16266,18649): Xorshift permutation, calibration projection and silver filtering form distinct selection contracts; zero count behavior differs.
- patches [22400,25886): Parse lines into cloned drafts and PRAG side-effects, enforcing missing/broken/gate rejection ordering.
- finish [33082,41976): Write/report/corpus composition is effectful and nontransactional; optional features occur after original gates.
- examples [51912,54068): Examples section state and candidate marking/character/word filters define the accepted fragment sequence.

CLI dispatch selects direct utilities or parses training/input/count/seed/output arguments. Drafts may load saved input or train a model; items are selected and annotated in patch/draft/two-look flows. apply_patches clones drafts, tracks seen/broken ids, mutates selected fields and PRAG side-map, then returns only seen unbroken ann::check successes. finish writes outputs, optionally replaces FEATS and calls UDPipe, appends accepted corpus items, then writes report.

Cross-file join: ann.rs supplies Ann/Item, parse/check/write; main supplies Outcome and finish; batch and llm prompts feed codex_batch; hints and repair influence draft content before gates. These joins do not establish remote correctness or crash durability.

## models

## behavior_complement

Missing-line, broken-patch and gate-failed sentences are excluded from patch output in that precedence order. An OK line does not excuse an earlier invalid patch. UDPipe output is comparison-only and never supplies final corpus annotation; optional Rust FEATS replacement is not re-gated here.

## reverse_reconstruction

A final accepted patch annotation has all compatible original drafts and patch sequences whose last valid assignments yield that annotation, provided that accepted id is seen and no line marks it broken. Acceptance is per sentence: unmentioned input ids are individually rejected and do not prevent other seen, unbroken ids from passing their annotation gate. Unknown-id lines and repeated OK are unobservable. Original draft or teacher reasoning cannot be recovered uniquely. Reports collapse per-token differences into aggregates.

## selection_zero

shuffle uses seed.max(1), so 0 and1 coincide and empty/singletons are identity. calib takes n after shuffle. silver checks out.len()==n after a possible push, so n=0 may return nonempty; duplicated source ids are not removed.

## patch_precedence

Recognized id becomes seen before parsing. Token number wraps subtract1 then bounds-checks. Unknown FEATS still mutates local draft but broken flag excludes it. PRAG validates closed values; unknown ids ignored; duplicate pair ids can panic on second map removal.

## finish_effect_order

prag/raw files precede optional Rust features and UDPipe, then dumps/calls/report construction, corpus append, report write. Failure can leave partial outputs. Corpus have-set is initial only, so duplicates within items are not prevented.

## extraction_filters

examples_in accepts only example-section candidates with ASCII/limited punctuation, >=3 words and>=2 lowercase-rich words, excluding marked incorrect lines. sentences requires redistributable CC BY4 metadata, clean >=3-word candidates and >=70% alphabetic chars; comments saying6-40 are not enforced.


## tests

### Bounded source evidence

Selected actual-code tests (twice debug, twice release):

- tests::seed_examples_skip_ungrammatical
- tests::prag_patches_are_checked
- wave27_tests::shuffle_seed_zero_alias_and_permutation
- batch::tests::results_parse_and_patch

Run command: cargo test -p annot --bin annot --test codex_batches [--release] -- --test-threads=1 --skip ann:: --skip bfs_transfer_checks. The fake executable fixture uses temporary files, no live teacher/API. Paired named outcomes must match. Six compiled production mutations target batch last-wins, llm closed-set, repair shape gate, hints severity, main seed alias and queue resume. Each compilation exits0, test exits101, exact source restored and focused positive rerun exits0. The fixture itself has no separate mutation. Existing repair test may early-return without dictionary; independent added repair test always runs.

Unexecuted limits:
- No training, live network, live teacher, corpus/model write or end-to-end finish execution.
- silver n=0, duplicate ids, post-FEATS gate and unchecked indexing limitations modeled without production changes.

Validation limits: strict scoped Clippy exited101 on six pre-existing lints (three collapsible_if, one needless_range_loop, one type_complexity, one redundant_locals). Re-running with those four known lint categories allowed exited0. changed with an external augmented registry exited0 and found no missing changed-source models. Local groupants inspection reports only expected ancestor child/source-binding updates owned by integration; no owned leaf/fragment diagnostics.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: source read before tests/models. Prospective notes and numeric receipts retained externally under the annotation wave27 run. Four new inline tests supplement existing bounded tests. Preserve distinctions: compatibility cost is not subscription cost; mock process tests do not establish live teacher behavior; registry presence is partial modeling.
