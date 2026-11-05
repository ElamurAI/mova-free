# mova-turtle-work

## Descriptant

Sea Turtle's standing loop with its native work in bulk: sweeps close every open lesson that keeps the school's rules (up to BATCH a sweep, the loop's own `native` journal line, one write per sweep) with no pause; only a sweep that closes nothing is followed by a paced teacher step (nautilus::work::Loop::step) and a pause. Source: `tools/turtle/src/work.rs`.

## Forward approximant

run: loop { STOP? break; s = sweep(batch); s stopped → break; s.native > 0 → continue; v = Loop::step; v stopped → break; sleep(SECS, ×4 when idle or waiting) } → summary. sweep: refuse a relative state, a state inside a frozen path, a ledger; a stopped line without a later resumed → stopped; seen = nautilus::work::seen(journal); for (key, file) in LessonJob::lessons() not seen: native < batch → check → Done appends {"event":"native","key","note","unix"}, Unsure stays open.

## Invariant

The journal format is the nautilus loop's: a key closed by a sweep is never picked by Loop::step. Held-out and control sets are never listed. No teacher question while sweeps close lessons; teacher questions keep Loop::step's pacing and gates.

## Cost

A sweep reads and checks each open lesson once (O(open lessons × lesson bytes)) and the journal once. Measured on a copy of the live journal: 4 451 open lessons in 27 courses, nine sweeps of 500 in about one second; 12 lessons left for the teacher.

## Composition

Parent: `turtle`. Uses make::LessonJob, check through Job::native, nautilus::work::{Loop, seen, summary}.

## behavior_complement

Never writes a course file, never asks the teacher itself, never reads held-out sets; a lesson with a problem is not closed by a sweep. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

work.jsonl recovers which lessons were closed natively and when; the printed sweep lines recover the counts. Not the lessons' texts at the time.

## tests

### Checks

Red controls kept in the tests: only/just and mum/parent refused as near-synonyms, now/out/just and go/say as words that fit the gap.
These are finite fixtures; no teacher was called; not a proof.

## History

### 2026-11-04 · Cost

A sweep reads and checks each open lesson once (O(open lessons × lesson bytes)) and the journal once. Measured 2026-11-04 on a copy of the live journal: 4 451 open lessons in 27 courses, nine sweeps of 500 in about one second; 12 lessons left for the teacher.

### 2026-11-04 · document

#### Epoch 2026-11-04

First model, written with the source; the throughput above is the prospective claim for the live loop.

### 2026-11-04 · tests

Command: cargo test --manifest-path tools/turtle/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin and Open English WordNet under $MOVA_DATA/raw/en-oewn/json (14 unit + 2 English + 2 gap tests; two identical runs on 2026-11-04).

Negative controls 2026-11-04 (each red, restored): check without the school's rule; held-out course names read; the deputy vocabulary without «command»; the fit check switched off (4 unit and both real-data gap tests red: «just» for «only», «dad» for «mum»); the sweep reading held-out courses; a teacher step after a sweep that closed lessons.
