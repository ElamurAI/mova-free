# mova-whale-learn

## Descriptant

One Whale learns on one paper round after round until its own description passes the gates: read, write with all retained knowledge, gates; on failure a bounded set of never-asked questions about gaps to its own terminal; answers re-read and retained as knowledge; journal per round; stops on pass, terminal limit/timeout/failure, dry run, nothing left to ask, round limit, deadline or a stop file before a round (`stop`, not final). Source: `tools/whale/src/learn.rs`.

## Forward approximant

run(config, terminal): validate; terminal animal == whale; paper hash; learner.lock + terminal lease; status.json (first_start, deadline = min(first_start + hours, deadline_unix)); passed -> return; import earlier paper runs once; read once; loop k: round_limit/deadline/stop_file checks; round-k dir (never reused); Known::of + Guard (all answers) -> compose -> gates -> files + journal draft/gates; pass -> stop; gaps (terms without facts and not asked, half the slots; one part with nothing restated; longest prose sentences without a clause; never asked keys) -> ask each once; answer: keep text privately, mark asked, facts -> retain item (provenance), journal; error: mark asked unless no terminal, journal no_answer, stop after the round; empty gaps -> nothing_to_ask.

## Invariant

No question key is ever sent twice (asked.jsonl before the next round, across invocations and papers); no retry after an error; answer text never enters a description (gate answer_overlap); one learner per Whale state and one Whale per terminal; a round directory is never reused; the deadline is fixed at the first round.

## Cost

Read once (about 1 s per paper, release); per round one compose + gates (milliseconds) plus at most max_questions terminal calls (seconds to minutes each with a live model); rounds <= max_rounds; wall time <= hours. Expected cost change before measuring: the learning loop adds no model call when the native draft passes and at most max_questions per round otherwise.

## Composition

Parent: whale-workers; composes reader.rs, compose.rs, gates.rs, knowledge.rs and terminal.rs; started by main.rs (learn) or batch.rs.

## behavior_complement

Not done: reaching a high configured minimum length by asking (term explanations add about 6-25 words per answered term), rewriting the writer from answers, resuming a question pending in the terminal, sharing knowledge across Whales, starting or stopping terminals. Measured with a fake tmux LLM (never a model): +6 to +12 words per round over 8 rounds; the round limit ends the loop below the configured minimum. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

journal.jsonl (round, draft words by kind, gates, question, answer hash, retained item, no_answer, stop) and round-NNN files reconstruct each round's description, its gates, what was asked and what was retained; the terminal's reasoning is not recoverable.

## History

### 2026-11-04 · behavior_complement

Not done: reaching a high configured minimum length by asking (term explanations add about 6-25 words per answered term), rewriting the writer from answers, resuming a question pending in the terminal, sharing knowledge across Whales, starting or stopping terminals. Measured 2026-11-04 with a fake tmux LLM (never a model): +6 to +12 words per round over 8 rounds; the round limit ends the loop below the configured minimum. Evidence status: source-derived model with finite checks, not a proof.

### 2026-11-04 · document

#### Epoch 2026-11-04 (reopening and re-reading)

A paper stopped unpassed and assigned again with a later deadline (a later batch's retry) is reopened: journal `reopened`, the new deadline, `max_rounds` counted from `rounds_base`; knowledge, asked questions and earlier rounds stay. `last_round.writer` records `compose::WRITER`. `reread` re-reads the kept answers to this paper's sentence and section questions once per `knowledge::READER`, retaining new facts as new items (provenance `reread`); nothing is sent.

#### Epoch 2026-11-04 (batched typed questions, pod, stall, review hints, topic corpus)

One call per round carries the round's questions, numbered, asking for typed shapes; parts are read per question. With a pod, claims stop cross-pod repeats and the pod's items join the writer's and the gates' knowledge. `stalled` after stall_rounds rounds without word gain after a round without a usable fact; an EXHAUSTED paper is not reopened under the same writer. Review hints when a round gains fewer than min_gain_words words and before a pass. Optional topic corpus: seed at round start (lock released for the child), cited.md per round. Replay measurement (fresh pod, today's 505 recorded answers, 10 papers, two identical runs): 62 rounds, 198 questions, 52 terminal calls (live today: 164 rounds, 549 questions, 549 calls); term facts kept per answered term 1.21 (live 1.04 with bare facts, which now go to review); sentence answers still give almost no facts (replayed answers are not typed); words per paper 462-4,882, 2 of 10 pass (live 3).

### 2026-11-05 · document

#### Epoch 2026-11-05

Test fixtures construct `compose::Item` with the new empty `cites` field; behaviour unchanged.

#### Epoch 2026-11-05 (stop between rounds)

`stop_file` (optional, absolute): when it exists before a round the learner stops with `stop` and writes its status; nothing is lost and the next invocation continues the paper. The batch runner sets it to its run root's `STOP`, so a runner can be stopped between rounds and resumed (deploying a new binary). Test: learn::tests::a_limit_stops_cleanly_a_pass_asks_nothing_and_the_deadline_holds (stop file: `stop`, 0 rounds, no call; red control without it: rounds run).
