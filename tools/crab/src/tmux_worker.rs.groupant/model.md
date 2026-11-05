# crab-tmux-worker

## model

# Crab Dolphin holder — source-bound model

## Descriptant

`tmux_worker.rs` holds one long-running terminal program, a 🐬 Dolphin, inside a detached tmux session so that Crab can supervise it as an ordinary worker. It is the only bridge between Crab's process supervision and a terminal session, and it never types into the terminal.

## Forward

Input: trusted argv (lock or --no-singleton, tmux path and optional private server name, session, cwd, command, grace 1..8 s, poll 50..10000 ms, owner marker, prompt template and variables, pin). run: acquire flock(LOCK_EX|LOCK_NB) on the lock file or return AlreadyRunning(owner record); check_pin or return Unpinned(reason); render the prompt (every {{KEY}} bound, every variable used, <= 100000 bytes) and append it to the command; if `=SESSION:` exists, require exactly one live pane and a pane process or direct child whose cmdline matches (basename(argv0), equal arguments, interpreter prefix allowed) -> adopted, else Conflict; otherwise new-session -d -s SESSION -c CWD -- command, set @mova_crab_owner, wait <= 5 s for the command to appear. Loop: stop flag -> graceful_stop (SIGTERM to the matched pid while its starttime matches, wait grace, kill-session if still present) -> Stopped; session gone -> Ended(session_ended); matched process gone -> Ended(command_ended), the session is left; pin recheck each 60 s.

## Invariant

- At most one holder per lock file is past `acquire` at any time (flock, close-on-exec, never inherited by tmux or the program).
- The holder issues only has-session, new-session, set-option, list-panes, kill-session; never send-keys, load-buffer or paste-buffer.
- A conflicting session is never killed, signalled or changed.
- A signal goes only to the matched pid while its kernel start time is unchanged.
- No program starts while its pin fails; a running one is stopped at the next failed check.

## Cost

Each poll: one tmux client process and one /proc read; stop: at most grace seconds of 25 ms sleeps plus one kill-session; adoption: one /proc directory scan; pin: one file hash and one version command bounded by 15 s.

## Composition

Crab (`workers.rs`) starts `crab-tmux-worker` from a configured worker argv and observes its exit; its SIGTERM on stop reaches the holder's process group, and the holder ends the session before Crab's 10 s SIGKILL. Crab's consult protocol (`angel.rs`) addresses the same session by its exact name to deliver tasks; `angel.rs` reuses `check_pin` before each consultation. `bin/crab-tmux-worker.rs` installs signal handlers and maps outcomes to exit statuses.

## behavior_complement

Type: behavior_complement. Universe: (argv, lock state, tmux server state, /proc observations, clock) with holder outputs. Not done: typing or pasting into any pane; killing or modifying a session that does not run the configured command; starting a second holder for a held lock; starting a program whose pin fails; restarting the program (restart policy belongs to Crab); judging the program's health or its model; restricting what the program itself may do (its own flags).

## reverse_reconstruction

Type: reverse_reconstruction. From an exit status: 0 ended or stopped, 3 conflict, 4 error, 5 already running, 6 unpinned; the event log lines recover the path taken. From the lock file: the last owner's pid, session and server. From a session: its command line constrains the configured command up to argv0 path (basename only). No unique inverse is claimed.

## singleton_lock_partition

Type: error_partition. Domain: holders started with one lock file. Disjoint outcomes: acquired (exactly one at a time), busy (reported owner, exit 5). Negative control: with flock disabled a second holder on another tmux server keeps running and creates a second session (test red).

## checks

### Checks

Tests use private tmux servers (`tmux -L`) and temporary directories; no real LLM terminal, model or network call.

## History

### 2026-11-04 · checks

Heading `### Checks (2026-11-04)` stays in checks as `### Checks`.

2026-11-04: 10 tests on private tmux servers (create/stop, session ends, adopt under a shell, conflict untouched, command ends inside a surviving shell, singleton across two tmux servers with an in-test --no-singleton red control, lock release, prompt as argument with a keystroke audit, pin/login refusal, argument refusals); red controls: lock disabled, match always true, send-keys added, pin check removed.
