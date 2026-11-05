# turtle

## Descriptant

Sea Turtle (mova-turtle): the school maker, one of Mova's animals. Course checks with the school's own rule and the answer marks, native gap questions from Nautilus layers held to a fit check (no near-synonyms, options that break the slot) in lessons that keep the rule, English through the native deputy, and the standing loop over the courses with native checks in bulk sweeps.

## Forward

course -> check::course -> report; layer + en.bin + WordNet -> fit::Lex, slots -> gap lessons -> folder + made.jsonl; prompt -> Hands(check|made|gap) -> English; work: sweeps of LessonJob lessons (native, no pause), then nautilus::work::Loop::step for the teacher (paced).

## Invariant

The format check is world knowledge check's own function; a proposal is the same lesson passing the same check; a sweep closes only lessons that pass it, with the nautilus loop's journal line; a gap option passes fit::judge.

## Cost

O(course bytes) per check; a sweep checks every open lesson once (the configured school, about 4 500 lessons, in about a second); one teacher question per teacher step at most, at most every 600 s.

## Composition

Leaves are file groupants under `tools/turtle/src/*.rs.groupant` and `tools/turtle/tests/*.rs.groupant`. Correct leaves do not prove the wiring: the interaction checks are the English tests, which run the deputy, the handler and the real work together.

## behavior_complement

No model calls; course files are never written; held-out and control sets are never read; nothing is admitted.

## reverse_reconstruction

Reports, made.jsonl, draft folders and work.jsonl recover what was checked, made and proposed. Evidence status: source-derived with finite checks.

## tests

### Checks

Red controls kept in the tests: only/just and mum/parent refused as near-synonyms, now/out/just and go/say as words that fit the gap.
These are finite fixtures; no teacher was called; not a proof.

## History

### 2026-11-04 · Cost

O(course bytes) per check; a sweep checks every open lesson once (the configured school, about 4 500 lessons, in about a second on 2026-11-04); one teacher question per teacher step at most, at most every 600 s.

### 2026-11-04 · document

#### Epoch 2026-11-04 (bulk sweeps and fit check)

Children fit (the option check), work (the bulk loop) and gap tests (red controls on real data) added. The interaction checks are the English and gap tests and the sweep test, which runs the sweep and Loop::step on one journal.

### 2026-11-04 · tests

Command: cargo test --manifest-path tools/turtle/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin and Open English WordNet under $MOVA_DATA/raw/en-oewn/json (14 unit + 2 English + 2 gap tests; two identical runs on 2026-11-04).

Negative controls 2026-11-04 (each red, restored): check without the school's rule; held-out course names read; the deputy vocabulary without «command»; the fit check switched off (4 unit and both real-data gap tests red: «just» for «only», «dad» for «mum»); the sweep reading held-out courses; a teacher step after a sweep that closed lessons.
