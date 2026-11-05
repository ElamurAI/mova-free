# mova-nautilus-work

## model

# mova-nautilus-work

## Descriptant

The bounded standing loop: Job trait (next, native, validate), Loop::step (stop/resume, pick, native or ask, validate into a review-only proposal, journal), pacing, limit stop, run with STOP, resume, summary; DictJob over layer tens. Source: `tools/nautilus/src/work.rs`.

## Forward approximant

step: refuse a state inside frozen paths -> stopped until resumed -> closed keys from journal -> job.next -> native Done -> journal native | Unsure -> pacing -> teacher::submit -> held on error | limit -> stopped | answer -> job.validate -> proposals/<id>.txt + proposed | rejected.

## Invariant

Every closed item is journaled once; a stop holds until a later resumed line; admitted stays 0.

## Cost

One job.next scan plus at most one question per step.

## Composition

Parent: `nautilus`. Parent: nautilus. Used by mova-turtle with LessonJob. State: work.jsonl, proposals/, work.lock.

## behavior_complement

Never writes sources, courses or assessment files; never admits; never reads held-out items (job responsibility); never asks two questions in one step or before min_gap_secs. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

work.jsonl recovers every item's outcome, reason, question id and answer hash; proposals hold the validated text. No unique inverse is claimed.

## tests

### Checks

These are finite fixtures; no teacher or model was called; not a proof.

## History

### 2026-11-04 · document

#### Epoch 2026-11-04

First model, written with the source; no prospective prediction is claimed beyond the checks.

#### Epoch 2026-11-04 (pool)

Loop.ledger (a job's shared ledger.jsonl) and member: seen = journal closes plus ledger closes and reservations (an `unreserved` line reopens a reservation the terminal never took); a question is reserved in the ledger before it is asked, its hash checked against the ledger (asked by any member: held), a limit holds the item in the ledger at once; Job::open counts open items; Job::describe goes into `proposed` lines; DictJob takes part of parts. Red control: ledger ignored (the moved job asks again).

#### Epoch 2026-11-04 (pool)

Loop.ledger (a job's shared ledger.jsonl) and member: seen = journal closes plus ledger closes and reservations (an `unreserved` line reopens a reservation the terminal never took); a question is reserved in the ledger before it is asked, its hash checked against the ledger (asked by any member: held), a limit holds the item in the ledger at once; Job::open counts open items; Job::describe goes into `proposed` lines; DictJob takes part of parts. Red control: ledger ignored (the moved job asks again).

#### Epoch 2026-11-04 (senses)

Job::native_proposal and Job::learn (defaults: none); the loop writes `<id>.native.txt`, names it in the closing event and lets the job learn (the sense store); an unsent hold (no session) is journaled without closing once and retried when the terminal is ready (ledger: unreserved + unsent), a second closes; seen() treats older holds by their reason. DictJob: items of `batch` asked words keyed `<layer>:<word>+…`, numeric keys of earlier tens still close their rows; draft_task/draft_validate with picks, `same`, `new:`, restatements and absent lines; summary counts native proposals. Red controls: unsent retry removed; junk length removed.

### 2026-11-04 · tests

Command: cargo test --manifest-path tools/nautilus/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin (15 unit + 2 English tests; two identical runs on 2026-11-04).

Negative controls 2026-11-04 (each red, restored): no operator-word check; no terminal-id check; a limit not stopping the loop; stats drafting instead of reading.

Pool epoch (2026-11-04): cargo test --release with the 0.4 en.bin, 26 unit + 2 English tests, two identical runs; red controls in the pool, improve and sources tests (each red, restored). Finite fixtures, not a proof.

2026-11-04 (senses): cargo test --release (0.4 en.bin), tools/nautilus 37 unit + 2 English tests, two identical runs; tools/turtle 9 + 2 pass. 2026-11-04 (senses): dictionary_job_plans_natively_and_asks_in_batches, earlier_tens_stay_done, the_store_fixes_a_sense_once_for_every_layer, an_unsent_question_is_retried_once (red control: a crash is held at once). Red: unsent retry removed, junk length removed, restatement gate removed.

### 2026-11-05 · document

#### Epoch 2026-11-05 (sampled Wikipedia)

`DictJob::ask_all`: layers whose words are never picked natively (their examples show different senses on purpose);
junk and thin words are still never asked. `dry(job, items)`: the first items in the loop's order, each taken as
closed once seen, with no terminal, journal or store write; per item the words, asked (`word:` lines), native
(`# pick` lines, listed as word=pick), never asked (`# not asked`) and question bytes. Red control: ask_all ignored
(an_ask_all_layer_has_no_native_picks red, restored).

### 2026-11-05 · tests

2026-11-05 (sampled Wikipedia): cargo test --release --offline --locked (0.4 en.bin), tools/nautilus 46 unit + 2 English tests, two identical runs. work::tests: an_ask_all_layer_has_no_native_picks, a_dry_run_counts_without_writing.
