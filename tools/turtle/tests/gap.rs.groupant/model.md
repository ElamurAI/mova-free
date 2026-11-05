# mova-turtle-gap-tests

## Descriptant

The gap maker on real data with the 0.4 model and Open English WordNet: red controls on the sentences of whose wrong options also fitted (only/just, mum/parent, now/out/just/still, go/say/stay/stop), and the repository's commands dictionary layer made into lessons that keep the school's rule with none of those pairs as a wrong option, twice identically. Source: `tools/turtle/tests/gap.rs`.

## Forward approximant

Lex::load(layer words) + slot_of(annotator) → judge and wrong_options on six sentences; gap_lessons(commands layer) → check::lesson and world::knowledge::parse per lesson; second run equal.

## Invariant

Deterministic for a fixed model, WordNet and layer.

## Cost

One model load and one WordNet load per test (seconds).

## Composition

Parent: `turtle`. Exercises fit, make and check together.

## behavior_complement

Skips without MOVA_OCTOPUS_TEST_MODEL; no network or teacher. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

Assertion messages name the sentence, the answer and the option. No unique inverse is claimed.

## tests

### Checks

Red controls kept in the tests: only/just and mum/parent refused as near-synonyms, now/out/just and go/say as words that fit the gap.
These are finite fixtures; no teacher was called; not a proof.

## History

### 2026-11-04 · Descriptant

The gap maker on real data with the 0.4 model and Open English WordNet: red controls on the sentences of 2026-11-04 whose wrong options also fitted (only/just, mum/parent, now/out/just/still, go/say/stay/stop), and the repository's commands dictionary layer made into lessons that keep the school's rule with none of those pairs as a wrong option, twice identically. Source: `tools/turtle/tests/gap.rs`.

### 2026-11-04 · document

#### Epoch 2026-11-04

First model, written with the source; red with the fit check switched off (negative control).

### 2026-11-04 · tests

Command: cargo test --manifest-path tools/turtle/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin and Open English WordNet under $MOVA_DATA/raw/en-oewn/json (14 unit + 2 English + 2 gap tests; two identical runs on 2026-11-04).

Negative controls 2026-11-04 (each red, restored): check without the school's rule; held-out course names read; the deputy vocabulary without «command»; the fit check switched off (4 unit and both real-data gap tests red: «just» for «only», «dad» for «mum»); the sweep reading held-out courses; a teacher step after a sweep that closed lessons.

### 2026-11-05 · school data outside the repository

layer() is world::knowledge::dictionaries_dir()/commands.tsv; rows() fails with an install hint when the school data is absent.
