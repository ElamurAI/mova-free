# mova-turtle-check

## model

# mova-turtle-check

## Descriptant

Course checks: world::knowledge::parse and world::knowledge::lesson_problem (the rule of world knowledge check) per lesson, plus the answer marks of lessons mark (one leading star, no mixing, not all at one place). Source: `tools/turtle/src/check.rs`.

## Forward approximant

folder(course) -> sorted .lesson files -> marks(raw) -> parse -> lesson_problem -> mixed/one-place problems -> Report{lessons, questions, with_problems}.

## Invariant

The format rule is the school's own function, so the two checks agree.

## Cost

O(lesson bytes).

## Composition

Parent: `turtle`. Parent: turtle. State: Reads lessons only.

## behavior_complement

Does not grade answers, judge language or facts, or write any file. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

The report recovers per-lesson counts, mark places and problems. No unique inverse is claimed.

## Epoch notes

## tests

### Checks

Red controls kept in the tests: only/just and mum/parent refused as near-synonyms, now/out/just and go/say as words that fit the gap.
These are finite fixtures; no teacher was called; not a proof.

## History

### 2026-11-04 · document

#### Epoch 2026-11-04

First model, written with the source; no prospective prediction is claimed beyond the checks.

### 2026-11-04 · Epoch notes

- Comments repointed to consolidated docs, 2026-11-04.

### 2026-11-04 · tests

Command: cargo test --manifest-path tools/turtle/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin and Open English WordNet under $MOVA_DATA/raw/en-oewn/json (14 unit + 2 English + 2 gap tests; two identical runs on 2026-11-04).

Negative controls 2026-11-04 (each red, restored): check without the school's rule; held-out course names read; the deputy vocabulary without «command»; the fit check switched off (4 unit and both real-data gap tests red: «just» for «only», «dad» for «mum»); the sweep reading held-out courses; a teacher step after a sweep that closed lessons.

### 2026-11-05 · school data outside the repository

Doc comment: a course name is courses/<name> of the school data.
