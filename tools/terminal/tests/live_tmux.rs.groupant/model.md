# mova-terminal-live-tmux

## model

# mova-terminal-live-tmux

## Descriptant

End-to-end tests of the ask protocol against a fake LLM (a bash script that reads the pasted task, writes the answer file and prints the marker, or in `silent` mode only prints working text) in a real tmux server on a private socket. Source: `tools/terminal/tests/live_tmux.rs`.

## Forward approximant

Each test creates its own socket and directory, spawns fake sessions with an owner marker, builds profiles with `pane_command` bash and `idle_window` 2, and asserts: two animals get their own answers from their own terminals and a second question works after the first is answered; a profile pointing at another animal's session is refused by the owner marker with an empty journal; an unanswered question blocks the next one (never replayed, one `sent`); after abandoning, a busy screen still gets no question.

## Invariant

Tests only use their private tmux socket and temporary directory, kill that server and remove the directory on drop, and never start a real LLM.

## Cost

Each test spawns one or two tmux sessions and waits up to 5 s for start-up and 10 s per answer; observed total about 0.35 s.

## Composition

Interaction checks for `lib.rs` with a real tmux: lock, pending check, two observations, bracketed paste and answer file together. They do not exercise Codex or Claude Code screens; those classifications are covered by the unit tests' captured footers and the live runs.

## behavior_complement

Not covered: modal dialogs in a live pane, the unchanged-screen refusal between two observations, answer files that are symlinks or too large, lock contention between two processes, `ask_wait` timeout, the CLI. Counterexample role: the busy test fails if `ask` sends without an Idle observation.

## reverse_reconstruction

A passing run shows the fixture behaviours held on that run; it does not reconstruct the screens of real LLM CLIs or prove behaviour across tmux versions.

## tests

cargo test --manifest-path tools/terminal/Cargo.toml --release --locked: each_animal_gets_its_own_answer_from_its_own_terminal, another_animals_terminal_is_refused_and_nothing_is_sent, an_unanswered_question_is_never_replayed, a_busy_terminal_gets_no_question (4 passed; two identical runs). Requires tmux at /usr/bin/tmux, /usr/local/bin/tmux or /bin/tmux; CI installs it for this workspace.

### Checks

## History

### 2026-11-04 · tests

cargo test --manifest-path tools/terminal/Cargo.toml --release --locked: each_animal_gets_its_own_answer_from_its_own_terminal, another_animals_terminal_is_refused_and_nothing_is_sent, an_unanswered_question_is_never_replayed, a_busy_terminal_gets_no_question (4 passed; two identical runs on 2026-11-04). Requires tmux at /usr/bin/tmux, /usr/local/bin/tmux or /bin/tmux; CI installs it for this workspace.
