# mova-turtle-animal

## Descriptant

Sea Turtle as one of Mova's animals: configuration (journal, drafts, layers, courses, the annotator model for gap slots), English answers for check/made/gap, deputy capability check, the turtle handler and answer through the deputy. Source: `tools/turtle/src/animal.rs`.

## Forward approximant

Hands: check -> check::course + say_check; made -> make::made + say_made; gap -> make_gap_course(model from the configuration; refused without one) into drafts/<course>-<invocation> + say_gap; answer: Deputy::run_with with Hands, nautilus::animal::report.

## Invariant

Operations and their course or layer come from trusted parameters only.

## Cost

As the called functions.

## Composition

Parent: `turtle`. Parent: turtle. State: Journal and drafts.

## behavior_complement

No slots; no teacher calls; no course file is written. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

The deputy ledger recovers prompt, plan and outputs per request. No unique inverse is claimed.

## tests

### Checks

Red controls kept in the tests: only/just and mum/parent refused as near-synonyms, now/out/just and go/say as words that fit the gap.
These are finite fixtures; no teacher was called; not a proof.

## History

### 2026-11-04 · document

#### Epoch 2026-11-04

First model, written with the source; no prospective prediction is claimed beyond the checks.

#### Epoch 2026-11-04 (model field)

The configuration takes an optional `model`; the gap operation refuses to run without it rather than make options without a slot check.

### 2026-11-04 · tests

Command: cargo test --manifest-path tools/turtle/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin and Open English WordNet under $MOVA_DATA/raw/en-oewn/json (14 unit + 2 English + 2 gap tests; two identical runs on 2026-11-04).

Negative controls 2026-11-04 (each red, restored): check without the school's rule; held-out course names read; the deputy vocabulary without «command»; the fit check switched off (4 unit and both real-data gap tests red: «just» for «only», «dad» for «mum»); the sweep reading held-out courses; a teacher step after a sweep that closed lessons.

### 2026-11-05 · document

#### Epoch 2026-11-05 (questions only observe)

`check_capabilities` refuses a deputy configuration that declares `check` or `gap` `observes`; only `made` answers questions.
