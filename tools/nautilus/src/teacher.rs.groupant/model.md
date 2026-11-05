# mova-nautilus-teacher

## model

# mova-nautilus-teacher

## Descriptant

The teacher interface of the animals that make teaching material: PrivateTerminal {id, ask(question) -> answer | Limit | Failed}, MovaTerminal (the thin adapter over mova-terminal ask_wait), Unplugged, plug, profiles as configuration (terminal id, mova-terminal profile, provider, model, purposes), submit with the operator's word, purpose, own-terminal, bound and journal-dedup gates, dictionary_task. Source: `tools/nautilus/src/teacher.rs`.

## Forward approximant

submit: validate config -> operator_word required -> profile serves purpose -> terminal id equals profile terminal -> valid bounded question -> hash not in journal -> journal reserved (fsync) -> ask -> journal answered (hash) | limit | failed.

## Invariant

At most one ask per question hash for the journal's life; nothing is asked without the operator's word.

## Cost

O(journal) per submit plus the terminal's time.

## Composition

Parent: `nautilus`. Parent: nautilus. Used by work and mova-turtle. State: Teacher journal (JSONL).

## behavior_complement

Holds no terminal itself (mova-terminal and crab-tmux-worker do); never retries, never replays an uncertain reservation, never admits an answer; profiles never share a terminal or its profile; nothing is asked without the operator's word. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

The journal recovers each question's hash, profile, terminal, provider/model labels, purpose, operator word and outcome with the answer's hash; not the question or answer text. No unique inverse is claimed.

## tests

### Checks

These are finite fixtures; no teacher or model was called; not a proof.

## History

### 2026-11-04 · document

#### Epoch 2026-11-04

First model, written with the source; no prospective prediction is claimed beyond the checks.

#### Epoch 2026-11-04 (terminal readiness)

`PrivateTerminal::ready` (default true; MovaTerminal: observed idle and no pending question). `submit` returns `{"state":"unavailable"}` before reserving when the terminal is not ready, so nothing is asked or journaled; the work loop answers `waiting` and keeps the item open. Measured: a loop started before its terminal had closed one item as held (2026-11-04); the new test `an_unready_terminal_leaves_the_item_open_and_asks_nothing` is red with the check removed.

#### Epoch 2026-11-04 (senses)

TASK_MAX 99 000 (under mova-terminal's 100 000); unsent(text) for no tmux session/server; submit allows a question whose only outcome is one unsent failure to be asked once more; dictionary_task = work::draft_task over the rows Mova does not settle, at most BATCH. Red control: a crashed question is never asked again.

### 2026-11-04 · tests

Command: cargo test --manifest-path tools/nautilus/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin (15 unit + 2 English tests; two identical runs on 2026-11-04).

Negative controls 2026-11-04 (each red, restored): no operator-word check; no terminal-id check; a limit not stopping the loop; stats drafting instead of reading.

2026-11-04 (senses): cargo test --release (0.4 en.bin), tools/nautilus 37 unit + 2 English tests, two identical runs; tools/turtle 9 + 2 pass. 2026-11-04 (senses): an_unsent_question_may_be_asked_once_more (red control: a crash is never asked again); profiles_validate_and_tasks_are_bounded: 25 words of 200 in one question under 20 000 bytes, examples cut, no counted line.
