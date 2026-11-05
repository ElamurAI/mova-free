# annot-main-examples

## descriptant

### annot-main-examples

Examples section state and candidate marking/character/word filters define the accepted fragment sequence.

## approximant

### Scoped forward relation

Examples section state and candidate marking/character/word filters define the accepted fragment sequence.

Parent composition and symbolic resource costs remain in ../model.md#approximant. No executable percentage surrogate is supplied for this categorical/effect scope.

## invariant

### Scoped fragment: examples

Local scope: Examples section state and candidate marking/character/word filters define the accepted fragment sequence.

The following parent relations supply composition context; claims apply only where this byte scope implements the stated operation. Other operations remain sibling/parent obligations.

### Invariants and limits

For unique pair ids and valid forms, output patch keys are a subset of supplied ids and every returned annotation passes ann::check. Unknown reply ids are ignored. Failed syntactic patches keep a sentence broken even after OK. PRAG side-map can retain entries for rejected sentences; finish restricts emission to final keys. This is not a transactional pipeline.

- No training, live network, live teacher, corpus/model write or end-to-end finish execution.
- silver n=0, duplicate ids, post-FEATS gate and unchecked indexing limitations modeled without production changes.

## models

# Scoped fragment: examples

Local scope: Examples section state and candidate marking/character/word filters define the accepted fragment sequence.

The following parent relations supply composition context; claims apply only where this byte scope implements the stated operation. Other operations remain sibling/parent obligations.

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

### Scoped fragment: examples

Local scope: Examples section state and candidate marking/character/word filters define the accepted fragment sequence.

The following parent relations supply composition context; claims apply only where this byte scope implements the stated operation. Other operations remain sibling/parent obligations.

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

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: scoped source-derived model. Byte scope [51912,54068); AST is shared partial projection, not a separate semantic proof.
