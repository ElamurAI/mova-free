# mova-whale-paper

## Descriptant

One Whale, one paper: pin check, new private run root, terminal lease, read through Mova, own description journaled before any question, bounded questions asked once each (limit/timeout/failure ends consulting), answers re-read into a glossary, teaching records with licence admissibility, metrics. Source: `tools/whale/src/paper.rs`.

## Forward approximant

run(config, terminal): validate (schema, labels, absolute paths, terminal id prefixed by the Whale name), terminal id match, paper SHA-256, refuse an existing run_root, private_dir, TerminalLease::take, journal start; reader::read -> reading.json, journal read; reader::write -> description.txt, journal draft; gaps -> questions; for each: journal question, ask once; answer -> journal answer, Mova skeletons into glossary.txt; error -> journal no_answer and stop asking; questions.jsonl, glossary.txt, teaching.jsonl (open_licence), metrics.json, journal finish. check_pod: no shared Whale, terminal, lock or run root.

## Invariant

The description is written before any answer and never contains answer text; a question is asked at most once and none after an error; a run root is never reused; one terminal per Whale.

## Cost

Reading cost plus at most max_questions terminal calls (<= 16).

## Composition

Parent: whale-workers; composes reader.rs and terminal.rs; CLI in main.rs.

## behavior_complement

Not done: revising the description from answers, retries, resuming an interrupted run, fetching papers, admitting teaching records to the open school (a reviewer does), any live terminal. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

From journal.jsonl and the run files one recovers the paper pin, terminal id, draft hash, each question and answer hash, how consulting ended and the metrics; the terminal's internal reasoning and timing inside a call are not recoverable. No unique inverse is claimed.

## History

### 2026-11-04 · document

#### Epoch 2026-11-04 (terminal adapter)

Epoch 2026-11-04 (terminal adapter): the terminal is a mova-terminal profile whose animal is the Whale; run refuses another animal's terminal, holds ROOT/holder.lock for the run, journals the terminal's question id; check_pod refuses a shared run root, profile file, session, owner marker or terminal root.

#### Epoch 2026-11-04

Unknown terms are normalized by the reader; questions use the paper's own spelling (`forms`).
