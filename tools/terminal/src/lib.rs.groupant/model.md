# mova-terminal-lib

## model

# mova-terminal-lib

## Descriptant

An animal's private LLM terminal: profile validation, pane observation and classification, the exclusive ask lock, the append-only journal, the one-question-at-a-time ask protocol (bracketed paste of a task file) and bounded answer-file reading. Source: `tools/terminal/src/lib.rs`. See `tools/terminal/README.md`.

## Forward approximant

ask(profile, question) = validate -> bounded non-empty question -> flock ROOT/terminal.lock (non-blocking) -> pending id with an answer file is closed as answered, without one refuses -> observe (has-session, display-message, capture-pane) -> classify must be Idle -> id = sha256(animal|journal length|time|question)[..16] -> create_new task file -> journal reserved -> load-buffer -> observe again: Idle and the same screen, else delete the buffer, journal abandoned and refuse -> paste-buffer -p -d -> send-keys Enter -> journal sent. classify(display, capture) is pure: display shape, session/window/pane counts, pane dead, owner marker, pane command (Mismatch/Absent), then the bottom 12 non-empty lines for modal text (Modal) and working text (Busy), then the idle marker (bullets stripped) among the last idle_window non-empty lines (Idle, else Busy). answer(id) reads a regular file of at most answer_max_bytes and journals answered once. ask_wait polls answer file and Idle until answer_timeout_seconds.

## Invariant

At most one question is sent per terminal between two answered/abandoned events (lock plus the pending check); nothing is pasted unless two observations both classify Idle and the two captured screens are equal; nothing is ever sent to a session whose owner marker or pane command differs from the profile; each id gets at most one `sent` and at most one `answered` line; an answer is never executed.

## Cost

O(question + task) bytes written, two fsyncs for the task and per journal line; each observation spawns three tmux processes and reads at most 60 lines of scroll-back plus the screen; journal reads are O(journal lines) per pending/answer/status call; ask_wait adds one answer-file stat and one observation per poll.

## Composition

Used by `main.rs` (CLI) and the end-to-end tests in `tests/live_tmux.rs`. The held session itself comes from `crab-tmux-worker` (tools/crab), which sets `@mova_crab_owner`; this file only reads that option.

## behavior_complement

Domain: all valid profiles and questions. The library does not: start, adopt, restart or stop the LLM session; type anything but one bracketed paste and one Enter per question; send while a question is pending, while another asker holds the lock, or while the pane is busy, modal, absent or foreign; re-send a question after a timeout; stream partial answers; follow a symlinked or oversized answer file; execute or parse the answer; share a root or journal between animals. Counterexample: with a silent fake LLM, a second ask fails with "never replayed" and the journal holds one `sent` (an_unanswered_question_is_never_replayed). Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

From the journal: the order of reserved/sent/answered/abandoned events per id, question and task SHA-256 and answer SHA-256 and length; with the task directory, the exact task text and therefore the question. Not recoverable: the screen at sending time, the LLM's intermediate output, the timing within a second, or the answer if the answer file was later changed (only its hash at `answered` time remains). A `reserved` without `sent` admits both "crashed before paste" and "crashed after paste before journaling"; it is therefore treated as pending, not as unsent. No unique inverse is claimed.

## tests

### Checks

Unit tests (pure classification and validation): codex_footer_below_the_marker_is_idle, working_modal_and_ownership_are_not_idle, a_quoted_question_far_above_does_not_block, profile_validation_refuses_bad_values.

## History

### 2026-11-04 · document

#### Epoch 2026-11-04 (settled screen, retried paste)

`ask` waits for a settled idle screen (two equal observations) and, when the screen changes between load and paste, abandons that attempt (journaled, nothing sent) and tries again, up to four attempts. Live Owls had stopped on a single changed screen.

### 2026-11-04 · tests

Command: cargo test --manifest-path tools/terminal/Cargo.toml --release --locked (4 unit + 4 end-to-end tests against a fake LLM in a real tmux server on a private socket; two identical runs on 2026-11-04). Live, 2026-11-04, one question each through a held Sol (Codex) and Opus (Claude Code) terminal. These are finite fixtures, not a proof.

Negative controls 2026-11-04: ignoring the pending check in `ask` turns an_unanswered_question_is_never_replayed red; ignoring the owner marker in `classify` turns working_modal_and_ownership_are_not_idle red; restored byte-identical afterwards.
