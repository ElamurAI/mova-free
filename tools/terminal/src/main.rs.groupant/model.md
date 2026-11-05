# mova-terminal-main

## model

# mova-terminal-main

## Descriptant

mova-terminal CLI: status PROFILE, ask PROFILE [--wait] (question on stdin), answer PROFILE ID, abandon PROFILE ID REASON. Source: `tools/terminal/src/main.rs`.

## Forward approximant

Load and validate the profile from the second argument; match exact argument shapes; ask reads at most question_max_bytes + 1 bytes of stdin (so an oversized question is refused by the library) and calls ask or ask_wait (500 ms poll); every command prints one JSON line on success; any error prints `mova-terminal: ERROR` to stderr and exits 1.

## Invariant

Unknown commands, extra or missing arguments, or an invalid profile have no effect on the terminal or journal beyond the error message and a non-zero exit.

## Cost

O(1) dispatch plus profile load (at most 64 KiB) plus the chosen library operation.

## Composition

Executable over `lib.rs`.

## behavior_complement

No flags besides `--wait`, no daemon mode, no queue, no retry, no reading of a question from arguments. Counterexample: `mova-terminal ask PROFILE --now` prints the usage and exits 1. Evidence status: source-derived model, not covered by an end-to-end CLI test.

## reverse_reconstruction

The printed JSON names the id, answer path or answer; the question is not printed and is recoverable only from the task file under the profile root. No unique inverse is claimed.

## tests

No dedicated CLI test: the binary is built by `cargo test --manifest-path tools/terminal/Cargo.toml --release --locked` (two identical runs) and was used for the live runs of (one question each to Sol and Opus). Argument-shape errors are a source-derived claim only.

### Checks

## History

### 2026-11-04 · tests

No dedicated CLI test: the binary is built by `cargo test --manifest-path tools/terminal/Cargo.toml --release --locked` (two identical runs on 2026-11-04) and was used for the live runs of 2026-11-04 (one question each to Sol and Opus). Argument-shape errors are a source-derived claim only.
