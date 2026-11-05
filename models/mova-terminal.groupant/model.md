# mova-terminal

## model

# mova-terminal

## Descriptant

Composition of the public `mova-terminal` crate: an animal's private LLM terminal. Each animal instance that needs a large model holds its own interactive LLM session (Opus through Claude Code by default; Sol through Codex as a configurable alternative), held by `crab-tmux-worker` and started by Crab; this crate asks it one bounded question at a time and reads the answer file. See `tools/terminal/README.md` and `docs/animals.md`.

## Forward

profile (lib) validates -> CLI (main) reads the question -> lib takes the lock, refuses a pending question, observes the pane twice around loading the task buffer, pastes bracketed, journals -> the LLM writes ROOT/answers/ID.txt and prints the idle marker -> lib reads the bounded answer file and journals `answered`.

## Invariant

One terminal per animal: nothing is sent to a session whose `@mova_crab_owner` or pane command differs from the profile; at most one unanswered question per terminal; no question is re-sent; nothing is pasted unless the pane is idle on two equal observations; answers are evidence, never executed.

## Cost

Per question: three tmux processes per observation, two observations, three tmux calls to send, O(question) bytes and a few fsyncs; then the LLM's own latency (live: Sol about 11 s, Opus about 3 s of thinking, one question each).

## Composition

Leaves: `tools/terminal/src/lib.rs.groupant`, `tools/terminal/src/main.rs.groupant`, `tools/terminal/tests/live_tmux.rs.groupant`. Correct leaves do not prove the wiring: the interaction checks are the four end-to-end tests against a fake LLM in a real tmux server, and the live runs of. The parent `crab-orchestration` supplies the holder (`crab-tmux-worker`) and starts it.

## behavior_complement

Domain: all configured animal terminals. The system does not: hold or restart the LLM session itself; share a terminal, lock, root or journal between animals; use a Dolphin; queue questions; re-send after a timeout; stream; answer dialogs (a folder-trust or permission dialog is reported as `modal` and blocks sending); execute answers. Counterexample: another animal's session is refused by its owner marker with an empty journal.

## reverse_reconstruction

The journal plus task directory recover per-id order of events, the exact task and question, and the answer's hash and length at `answered` time. Screens, LLM-internal work and exact timings are not recoverable. Evidence status: source-derived with finite checks.

## tests

cargo test --manifest-path tools/terminal/Cargo.toml --release --locked (4 unit + 4 end-to-end tests with a fake LLM in a real tmux server on a private socket; two identical runs).

Negative controls: ignoring the pending check turns an_unanswered_question_is_never_replayed red; ignoring the owner marker turns working_modal_and_ownership_are_not_idle red; restored byte-identical afterwards.

Live one question each: Sol (Codex) answered in about 11 s, Opus (Claude Code) in about 3 s of thinking.

### Checks

## History

### 2026-11-04 · Cost

Per question: three tmux processes per observation, two observations, three tmux calls to send, O(question) bytes and a few fsyncs; then the LLM's own latency (live 2026-11-04: Sol about 11 s, Opus about 3 s of thinking, one question each).

### 2026-11-04 · Composition

Leaves: `tools/terminal/src/lib.rs.groupant`, `tools/terminal/src/main.rs.groupant`, `tools/terminal/tests/live_tmux.rs.groupant`. Correct leaves do not prove the wiring: the interaction checks are the four end-to-end tests against a fake LLM in a real tmux server, and the live runs of 2026-11-04. The parent `crab-orchestration` supplies the holder (`crab-tmux-worker`) and starts it.

### 2026-11-04 · tests

cargo test --manifest-path tools/terminal/Cargo.toml --release --locked (4 unit + 4 end-to-end tests with a fake LLM in a real tmux server on a private socket; two identical runs on 2026-11-04).

Negative controls 2026-11-04: ignoring the pending check turns an_unanswered_question_is_never_replayed red; ignoring the owner marker turns working_modal_and_ownership_are_not_idle red; restored byte-identical afterwards.

Live 2026-11-04, one question each: Sol (Codex) answered in about 11 s, Opus (Claude Code) in about 3 s of thinking.
