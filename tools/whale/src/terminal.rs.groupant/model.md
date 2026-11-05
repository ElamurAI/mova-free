# mova-whale-terminal

## Descriptant

The narrow interface of a Whale's private terminal and its real implementation: TerminalConfig (path of a mova-terminal profile whose animal is the Whale, poll interval), TerminalLease (ROOT/holder.lock for the whole run), Question/Answer, AskError (limit, timeout, failed, unavailable), PrivateTerminal, MovaTerminal (thin adapter over mova-terminal), NoTerminal and FakeTerminal. Source: `tools/whale/src/terminal.rs`.

## Forward approximant

TerminalConfig::load -> terminal::Profile::load (absolute path, poll 100..60000 ms). TerminalLease::take(profile): create root, exclusive try_lease on ROOT/holder.lock or refuse. MovaTerminal::ask(q): terminal::ask(profile, q.text) (Failed on refusal: nothing sent); loop: terminal::answer -> Ok(answer, elapsed, terminal id); terminal::observe screen contains LIMIT_TEXT -> Limit (pending, never re-sent); elapsed > answer_timeout_seconds -> Timeout; sleep poll. NoTerminal: Unavailable. FakeTerminal: by subject, Limit after n answers.

## Invariant

One Whale holder per terminal root for a whole run; the adapter sends at most one task per ask and never re-sends; the screen is only read; answers are text, never executed.

## Cost

One mova-terminal ask plus one answer read and one screen observation per poll interval until answer, limit or timeout.

## Composition

Parent: whale-workers; used by paper.rs and main.rs; depends on mova-terminal (tools/terminal), whose session is held by crab-tmux-worker.

## behavior_complement

Not done: starting or holding a session, choosing the model, abandoning a pending question (an operator does it with mova-terminal abandon), retries, limit detection beyond the LIMIT_TEXT phrases on the screen. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

From the Whale journal (terminal question id, answer hash) and the terminal's own journal (reserved, sent, answered) one recovers which question each answer belongs to; the session's internal reasoning is not recoverable. No unique inverse is claimed.

## History

### 2026-11-04 · document

#### Epoch 2026-11-04 (learning Whales)

MovaTerminal::ask counts an answer only when its file exists and the pane is idle again (round 1 on 2026-11-04 sent its next question while the LLM was still working and the terminal refused it as not idle).

#### Epoch 2026-11-04 (batched questions, replay)

`Question::items` carries a round's batched questions; `Answer::missing` lists items a terminal had no answer for. `Replay` answers from recorded state and paper run roots (by normalized term or sentence text) for measurement; it never starts or asks anything.
