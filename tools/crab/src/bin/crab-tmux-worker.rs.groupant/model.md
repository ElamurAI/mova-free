# crab-tmux-worker-cli

## model

# Crab Dolphin holder binary — source-bound model

## Descriptant

`bin/crab-tmux-worker.rs` is the executable Crab starts for a Dolphin; it forwards to `tmux_worker::main_with_args`.

## Forward

Collect argv after the program name and exit with main_with_args(args).

## Invariant

- No behaviour of its own besides forwarding arguments and the exit status.

## Cost

Constant.

## Composition

Configured as a worker argv in Crab's configuration; see `tmux_worker.rs`.

## behavior_complement

Type: behavior_complement. Not done: any parsing, signalling or tmux operation of its own.

## reverse_reconstruction

Type: reverse_reconstruction. The exit status maps back to the holder outcome class (0, 2, 3, 4, 5, 6).

## checks

### Checks

Tests use private tmux servers (`tmux -L`) and temporary directories; no real LLM terminal, model or network call.

## History

### 2026-11-04 · checks

Heading `### Checks (2026-11-04)` stays in checks as `### Checks`.

2026-11-04: covered through tmux_worker tests of main_with_args parts (parse_args, run, outcomes).
