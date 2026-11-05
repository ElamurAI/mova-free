# crab-workers

## model

# Crab worker supervision — source-bound model

## Descriptant

`workers.rs` lets Crab manage its little ones: trusted configured long-running workers that Crab starts after
boot, adopts across its own restarts, restarts after crashes and interrupts only on an explicit stop or restart.
GPU workers start only after a bounded `nvidia-smi` query (and an optional probe) has passed. Every effect advances
in non-blocking ticks of Crab's serve loop; requests only record intent and return the current state.

## Forward

State per worker w: s(w) in {not_booted, stopped, running, waiting_gpu, backoff, stopping_marker, stopping_term,
stopping_kill, exited, failed, unadopted_conflict}, with flags stop_requested, start_after_stop, counters restarts and
consecutive failures f, and an optional live identity L=(pid, pgid, starttime, child?, started_unix, adopted).

- boot(t), once, after Crab's lease: for a recorded (pid, starttime) adopt iff /proc/pid/stat has that starttime,
  is not a zombie, the configured argv hash is unchanged and argv is a suffix of /proc/pid/cmdline; starttime
  mismatch or absence is "gone"; changed argv or cmdline over a live identity is unadopted_conflict with the recorded
  identity R kept in memory and in the state file; a recorded stopping_* state adopts the live identity whatever its
  argv (a begun stop continues). Without an adoption: start_after_stop -> begin; autostart and not stop_requested and
  no marker -> begin; else stopped.
- begin(w): gpu -> waiting_gpu; otherwise spawn (remove a regular-file marker, append log, process group 0) ->
  running, or spawn failure -> backoff/failed.
- tick(t): exit observed (try_wait for own children; /proc identity plus WNOHANG reap for adopted) -> stopping_* ->
  stopped (and begin again if start_after_stop); marker present -> stopped; restart -> backoff with
  delay(f)=min(5*2^(f-1),300), f reset after a run of at least 300 s; else exited. stopping_marker for grace seconds
  -> SIGTERM group -> stopping_term; 10 s -> SIGKILL group -> stopping_kill. backoff at deadline -> begin (or stopped
  when a marker appeared). GPU gate: Idle/Failed(deadline passed) with a waiting worker -> run nvidia-smi; exit 0 with
  a row within the timeout -> optional probe; pass -> spawn every waiting_gpu worker; fail or timeout (group kill) ->
  Failed with the same doubling backoff.
- start/stop/restart/status(name, t) on configured names only; stop on a live worker creates the marker or sends
  SIGTERM; restart = stop then begin after the exit.
- With a conflict R, every start/stop/restart/tick first re-checks R's identity: gone -> R cleared, event
  recorded_instance_gone (cleared: unadopted_conflict), state stopped, then the request proceeds normally (a tick
  begins an autostart worker as boot would). Live: start -> no effect, a conflict report (started=false,
  signal_sent=false); stop -> refused; explicit restart -> R becomes the live identity (adopted) and the ordinary stop
  path runs, then begin; flow's restart -> refused.

## Invariant

- No worker is spawned from request or prompt data: argv, cwd, log and marker come from validated configuration.
- A gpu worker is spawned only in the same tick a GPU check passed after its begin.
- A signal is sent only while (pid, starttime) still matches and, for own children, before reaping.
- Only stop/restart requests lead to stopping_*; a tick never signals a running, non-stopping worker.
- Over a live unadopted conflict only an explicit restart leads to a signal, and only through the ordinary stop
  path; start, autostart, stop and flow never signal it and never spawn a second copy.
- The state file names the process Crab answers for (live or conflicted), so no Crab restart forgets a live
  conflicted process.
- Every transition is persisted (state file with at most 32 events) before the next tick.
- Workers are a BTreeMap: tick order and status order are deterministic by name.

## Cost

Per tick: O(n) for n<=32 workers, each one try_wait or one /proc stat read (<= 1 KiB), plus at most one state-file
write per transition and at most one GPU child poll. No waiting inside a tick or request; the GPU check and stop
grace advance across ticks. Memory: bounded events (32 per worker) and 4 KiB of GPU output.

## Composition

`angel.rs` owns one `Workers` behind a mutex in its Context: Config adds `workers` and `gpu_check`; `serve_full`
boots after the lease and ticks every loop iteration; `worker_*` operations and nested prompt calls (`in_process`)
call start/stop/restart/status. `tools/octopus` maps English prompts to those operations through configured
`crab_control` capabilities. Deployment configurations (which workers, which GPU probe) stay outside public Mova.

## behavior_complement

Type: behavior_complement. Universe: (configuration, request sequence, process/proc observations, clock) tuples with
supervisor outputs. Exclusions derived from source and checked on finite fixtures: no spawn of an unconfigured name;
no gpu spawn after a failed, hanging or probe-failed check; no restart of a worker stopped by request or marker; no
adoption across a start-time mismatch (pid reuse) or changed argv; no signal to an unadopted conflict except
through an explicit restart; no duplicate start over a live conflict, also across Crab restarts; no blocking wait in
a request. Not done at all: English messaging to workers, health inference (running is not healthy), GPU
memory admission beyond the probe, hostile-process resistance (a worker leaving its process group escapes group
signals), cross-host supervision. Counterexample to "alive pid = our worker": a recycled pid with another start time.

## reverse_reconstruction

Type: reverse_reconstruction. From a state file one recovers the last state, live identity, counters and the last
32 transitions, which constrain but do not uniquely determine the request and exit history (older events are
dropped; an adopted exit has no exit code). From a running process, (pid, starttime, cmdline suffix) recovers the
worker it belongs to under cooperative same-user operation. gpu-check.json recovers the last gate result, not every
earlier attempt. No unique inverse is claimed.

## identity_adoption_partition

Type: error_partition. Domain: recorded (pid, starttime, argv hash) against /proc. Disjoint outcomes: adopted;
gone (absent, start time differs, zombie); conflict (live and same start time, but argv changed or cmdline suffix
differs). Only adopted yields control; gone permits a fresh start; conflict blocks start and stop, and is
resolved by an explicit restart or by the process ending. Negative control: disabling the start-time comparison adopts a different process (test red).

## gpu_admission_gate

Type: partial_exclusion. Domain: gate outcomes per begin. exit 0 with a nonempty row then probe exit 0, both within
the timeout, is the only admitting path; spawn errors, nonzero exits, empty output and timeouts all keep waiting_gpu
and schedule a retry after 5..300 s. Negative control: bypassing the gate starts a gpu worker after a failing check
(test red). The check observes driver answers, not free memory or other GPU users.

## emergency_interruption

## unadopted_conflict_resolution

Type: partial_exclusion. Domain: (worker with recorded identity R=(pid, starttime, recorded argv hash),
request in {autostart, start, stop, restart, flow restart, tick}, /proc observation of R). Observed on the live run
the same day: three workers (dolphin-sol, nautilus-work, turtle-work) entered unadopted_conflict after configuration
changes; the state file was then written with pid=null, so the next Crab boot started the configured argv with no
check of the recorded process (an unchecked possible duplicate). Now the file keeps R (`argv_sha256` = argv of the
recorded process, `configured_argv_sha256` beside it) and the partition is:
R gone -> cleared and journaled, then the normal path; R live -> start/autostart: nothing, report; stop: refused;
flow restart: refused; explicit restart: ordinary stop of R (identity-checked signals), then the configured argv.
Expected cost change: one /proc stat read per conflicted worker per tick and per request; a bounded cmdline read
(64 args x 512 bytes) only in status/start reports. Negative controls (all shown red, then restored): start ignoring
the live conflict; conflict identity not persisted; begun stop not continued across a Crab restart; flow guard
removed; dead conflict never cleared. Limit: the takeover trusts (pid, start time) only; the recorded argv hash is
reported, not re-verified against /proc; stopping a crab-tmux-worker ends the command it holds, so a holder of
someone else's terminal must not be restarted this way.

## checks

### Checks

Fake workers are small `#!/bin/sh` scripts; fake `nvidia-smi` and probe scripts. No real GPU, model or training.

Module tests (7): configuration refusals (relative paths, bad names, duplicates, gpu without gate, unknown fields,
backoff bounds); start, stop by marker, start removes the marker, restart; SIGTERM without marker and SIGKILL
escalation after grace for a TERM-ignoring worker; crash restart at 5, 10, 20 s and cancellation by a marker,
restart=false; adoption after a Crab restart, stop of an adopted worker, no autostart after an explicit stop,
start-time mismatch not adopted, changed argv is a conflict; GPU gate failing, hanging (killed at timeout),
probe failing, then passing; unknown worker and unbooted refusals.

Angel tests (2): journaled worker operations with validation, receipt replay, unknown worker and configuration
absence; serve boot autostart and the nested prompt path admitting worker_status/worker_stop while refusing
consult_opus, legacy stop markers, start_whales and an unknown worker.

Negative controls shown red, then restored: GPU gate bypass (gpu test red); start-time verification disabled
(adoption test red); legacy markers admitted on the prompt path (serve test red). All 87 Crab library tests passed
twice.

## History

### 2026-11-03 · emergency_interruption

2026-11-03: `worker_stop` with `immediate:true` skips the graceful stop marker (which waits for the worker's next
safe point) and sends SIGTERM to the process group at once, with the usual SIGKILL escalation; a stopped worker is not
restarted by ticks. People do interrupt in emergencies; ordinary stops still wait for the safe point. Negative
control: honouring the marker even when immediate leaves the marker and fails the test.

### 2026-11-03 · checks

Heading `### Checks (2026-11-03)` stays in checks as `### Checks`.

### 2026-11-04 · document

#### Flow control surface (2026-11-04)

observe(name) -> (state, stop_requested) and the flow::WorkerControl implementation (observe, start,
restart) let flow start a stopped producer or restart a stalled worker through the same transitions as an
explicit request. No new transition or signal path.

### 2026-11-04 · unadopted_conflict_resolution

2026-11-04. Type: partial_exclusion. Domain: (worker with recorded identity R=(pid, starttime, recorded argv hash),
request in {autostart, start, stop, restart, flow restart, tick}, /proc observation of R). Observed on the live run
the same day: three workers (dolphin-sol, nautilus-work, turtle-work) entered unadopted_conflict after configuration
changes; the state file was then written with pid=null, so the next Crab boot started the configured argv with no
check of the recorded process (an unchecked possible duplicate). Now the file keeps R (`argv_sha256` = argv of the
recorded process, `configured_argv_sha256` beside it) and the partition is:
R gone -> cleared and journaled, then the normal path; R live -> start/autostart: nothing, report; stop: refused;
flow restart: refused; explicit restart: ordinary stop of R (identity-checked signals), then the configured argv.
Expected cost change: one /proc stat read per conflicted worker per tick and per request; a bounded cmdline read
(64 args x 512 bytes) only in status/start reports. Negative controls (all shown red, then restored): start ignoring
the live conflict; conflict identity not persisted; begun stop not continued across a Crab restart; flow guard
removed; dead conflict never cleared. Limit: the takeover trusts (pid, start time) only; the recorded argv hash is
reported, not re-verified against /proc; stopping a crab-tmux-worker ends the command it holds, so a holder of
someone else's terminal must not be restarted this way.

### 2026-11-04 · checks

#### 2026-11-04: unadopted conflict resolution

Module tests (+3, 10 in all): a live conflict keeps its recorded identity on disk, a plain start reports it and
neither signals nor starts, a Crab restart finds the same conflict (no duplicate), flow restart and stop are refused,
an explicit restart stops the recorded process through the marker and starts the configured argv; a dead conflict is
cleared and started by worker_start and, after a later conflict, by a tick (autostart); a takeover stop begun before
a Crab restart continues on the same process (marker ignored, grace, SIGTERM) and then starts the configured argv.
The adoption test now expects a conflict start to answer `started: false`.

Negative controls shown red, then restored byte-identical: start ignoring a live conflict; conflict identity not
persisted (pid=null); begun stop not continued across a Crab restart; flow restart guard removed; dead conflict never
cleared. Full Crab tests (123 library + 5 binary) passed twice. Known unrelated flakes seen once each under parallel
load: ETXTBSY in an angel terminal test and one supervisor pool-lock assertion (both pass alone and on rerun).

### 2026-11-04 · document

#### Worker environment (2026-11-04)

`WorkerSpec.env` (optional map) is passed to the spawned process; the command line stays exactly `argv`, so a restarted Crab adopts the process (wrapping a command in `/usr/bin/env` made adoption fail).
