# mova-turtle-main

## Descriptant

The turtle CLI: check, gap, made, lesson-task, work, work-summary, work-resume, plan, serve (singleton), ask. Source: `tools/turtle/src/main.rs`.

## Forward approximant

Parse argv -> the matching library call (gap takes MODEL, the en.bin that reads the slots; work takes STEPS, SECS (30) and BATCH (500) and runs turtle::work::run); serve and work take their singleton locks first (exit 5 when held); work plugs the profile's own mova-terminal (Unplugged when its profile is unreadable); no question without the operator's word.

## Invariant

Exit status 0 only on success.

## Cost

As the called functions.

## Composition

Parent: `turtle`. Parent: turtle. State: None of its own.

## behavior_complement

No command calls a model; check fails on any problem. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

Argv and printed JSON recover what was asked. No unique inverse is claimed.

## tests

### Checks

Red controls kept in the tests: only/just and mum/parent refused as near-synonyms, now/out/just and go/say as words that fit the gap.
These are finite fixtures; no teacher was called; not a proof.

## History

### 2026-11-04 · document

#### Epoch 2026-11-04

First model, written with the source; no prospective prediction is claimed beyond the checks.

#### Epoch 2026-11-04 (pool loop API)

Implements `Job::open` (lessons left, held-out courses never counted) and passes the pool fields `ledger: None, member` to `Loop`; build was broken on main by the Nautilus pool change.

#### Epoch 2026-11-04 (bulk sweeps)

`work` runs the Turtle's own loop (`work::run`: native sweeps without a pause, teacher steps paced); `gap` needs the annotator model.

### 2026-11-04 · tests

Command: cargo test --manifest-path tools/turtle/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin and Open English WordNet under $MOVA_DATA/raw/en-oewn/json (14 unit + 2 English + 2 gap tests; two identical runs on 2026-11-04).

Negative controls 2026-11-04 (each red, restored): check without the school's rule; held-out course names read; the deputy vocabulary without «command»; the fit check switched off (4 unit and both real-data gap tests red: «just» for «only», «dad» for «mum»); the sweep reading held-out courses; a teacher step after a sweep that closed lessons.
