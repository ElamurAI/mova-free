# mova-turtle-make

## Descriptant

What Sea Turtle makes: gap questions from a Nautilus layer as draft lessons (wrong options drawn by FNV-1a from the layer, its class first, each held to the fit check; of the first twelve accepted the three shown least; the right one starred at a drawn place; a gap asked once per making; lessons dealt three exercises to two tests and closed when they keep the school's rule, a tail joining the lesson before), the made journal and its counts, the lessons_task teacher question, and LessonJob for the standing loop (held-out names never read; same lesson and passing check to be proposed). Source: `tools/turtle/src/make.rs`.

## Forward approximant

layer rows -> per entry: gap(example, word) -> Fit.slot -> wrong_options(judge, uses) -> question (dropped with < 3 options) -> fact + questions dealt to exercises/tests -> lesson closed at 12/8/5 -> tail joined -> lesson texts; make_gap_course loads the annotator and Lex, write_gap_course writes a new folder and appends made.jsonl (source and model hashes, dropped, refused by reason); LessonJob::lessons lists (key, file) of the non-held-out courses in order; LessonJob: next lesson -> check::lesson -> Done or Unsure(rewrite task) -> validate(identity, check).

## Invariant

Same layer, model and lexicons give the same lessons; a question's wrong options never appear in its sentence and pass fit::judge; every lesson written keeps lesson_problem; a second making into an existing folder is refused.

## Cost

O(gaps × rows log rows) for the pool order, plus at most rows judge calls per gap (in practice until twelve are accepted); 27 s for the 1000-word commands layer of the Nautilus (2363 questions, 113 lessons).

## Composition

Parent: `turtle`. Parent: turtle. State: Draft folders and made.jsonl.

## behavior_complement

Does not write course folders, mark answers in existing lessons, read held-out or control sets, or ask a teacher itself. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

made.jsonl recovers each making's source hash, folder, lessons and questions; a lesson's gaps recover their source examples. No unique inverse is claimed.

## Epoch notes

## tests

### Checks

Red controls kept in the tests: only/just and mum/parent refused as near-synonyms, now/out/just and go/say as words that fit the gap.
These are finite fixtures; no teacher was called; not a proof.

## History

### 2026-11-04 · Cost

O(gaps × rows log rows) for the pool order, plus at most rows judge calls per gap (in practice until twelve are accepted); 27 s for the 1000-word commands layer of the Nautilus (2363 questions, 113 lessons) on 2026-11-04.

### 2026-11-04 · document

#### Epoch 2026-11-04

First model, written with the source; no prospective prediction is claimed beyond the checks.

### 2026-11-04 · Epoch notes

- Comments repointed to consolidated docs, 2026-11-04.

### 2026-11-04 · document

#### Epoch 2026-11-04 (pool loop API)

Implements `Job::open` (lessons left, held-out courses never counted) and passes the pool fields `ledger: None, member` to `Loop`; build was broken on main by the Nautilus pool change.

#### Epoch 2026-11-04 (fit check and the school's rule)

The old draw (same part of speech, no fit check, ten entries a lesson) gave lessons below the rule and options that also fitted (only/just, mum/parent, now/out/just, go/say). Now each option passes fit::judge and each lesson keeps the rule; the commands dictionary layer gives 8 lessons, 171 questions, all passing turtle check, twice identically.

### 2026-11-04 · tests

Command: cargo test --manifest-path tools/turtle/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin and Open English WordNet under $MOVA_DATA/raw/en-oewn/json (14 unit + 2 English + 2 gap tests; two identical runs on 2026-11-04).

Negative controls 2026-11-04 (each red, restored): check without the school's rule; held-out course names read; the deputy vocabulary without «command»; the fit check switched off (4 unit and both real-data gap tests red: «just» for «only», «dad» for «mum»); the sweep reading held-out courses; a teacher step after a sweep that closed lessons.
