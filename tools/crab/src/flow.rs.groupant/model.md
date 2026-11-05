# crab-flow

## model

# Crab work flow — source-bound model

## Descriptant

`flow.rs` makes sure Crab's continuously running workers always have work and that their stacks do
not clog. A probe is configuration only: where a worker's input queue, stacks and progress signal
are, how to count them, the water marks and the limit-marker files. Crab carries no worker-specific
code; it reads, latches, decides once per transition and journals.

## Forward

Sources: dir_entries(path, prefix?, suffix?) = |{e in path : not dot/.tmp, prefix/suffix match}|;
jsonl_lines = |non-blank lines| (missing file 0); json_field = unsigned integer at a dotted path
(declared `missing` when null, else unknown); json_equals = [field == value]; mtime; sum of plain
sources. Unknown readings are errors, never zero.

Per probe p and evaluation t (readings x, latches L with hysteresis h):

- raw.starved = input < low_water; raw.clogged = exists backlog stack s: x_s > (L.clogged ? resume_at_s : high_water_s),
  resume_at default high_water/2; raw.stalled = (input >= 1 or no gauge) and t - last_change(progress) >= stall;
  raw.limited = a limit pattern in a tail (<= tail_bytes) of a file modified within max_age; `*` segments
  expand to the 4 newest entries by mtime (ties by name).
- quarantine stack s (role quarantine; cumulative quarantined q and finished f): samples (t, q, f) within
  the window W (a decrease of q or f clears them; at most 256); with base b the oldest sample,
  dq = q - q_b, df = f - f_b; raw.quarantine_s = dq/df > max_rate if df >= min_finished, else unknown.
  Its own latch (hysteresis h) emits a report on the rise (with the dominant reason, a bounded string at a
  dotted field) and a cleared line on the fall. It never enters raw.clogged and never decides a pause.
- L_c turns on after h consecutive raw-true and off after h consecutive raw-false; unknown changes neither.
- decision, by precedence: limited or hold window -> Hold (on the flip only); clogged -> PauseProducer if a
  pausable stack is over and the pause file is absent, else Report on the flip; Crab-made pause and clog clear
  -> ResumeProducer; starved -> once per episode StartProducer(w) iff w is stopped/exited/failed and not
  explicitly stopped, else Report; stalled and raw.stalled and not yet restarted this episode -> Restart(w) iff
  restart_on_stall and w running, else Report on the flip.
- apply: start/restart through WorkerControl; pause creates the file exclusively with Crab's text; resume
  removes it only if it is still exactly Crab's regular file.
- capacity(t) per kind: queued = last input, rate = (v_last - v_first)*3600/(t_last - t_first) over <= 32
  samples when monotone, hours_to_drain = queued/rate, starved seconds since the latch turned on.

## Invariant

- A quarantine stack never pauses a producer: it is excluded from raw.clogged and from `pausable`; its
  configuration refuses high_water, resume_at and pause_producer; a backlog named like a quarantine that
  would pause is refused at load.
- Load migration lifts only Crab's own pause (exact text, regular file of Crab's user) and only for a clog
  whose recorded over-high-water stacks are all known and none still pauses.
- No process is signalled by this module; the only process effects are WorkerControl start and restart.
- A producer that was explicitly stopped is never started; a foreign pause file is never removed.
- An unknown reading never triggers an action.
- At most one decision per probe per evaluation; repeated evaluations in a steady condition decide None.
- Probes and memory are BTreeMaps / ordered vectors: evaluation and journal order are deterministic.

## Cost

Per evaluation of n <= 64 probes: each source one bounded read (directory <= 100000 entries, JSON <= 1 MiB,
JSONL <= 256 MiB streamed, tails <= 1 MiB), each glob segment one listing sorted by mtime. Measured
(prospective estimate was < 100 ms): ten probes over the live r2 Fish and the Eel took about 30 ms per read-only
pass, the same summary twice. One journal line per transition or action; the state file per evaluation.

## Composition

`angel.rs` holds one Flow behind a mutex: Config.flow, supervision_tick in the idle serve loop (every
interval_seconds; lock order flow -> workers), flow_status/attention operations, the CLI `flow-status`;
report/hold/errors become attention items (`council.rs`). `workers.rs` implements WorkerControl.

## behavior_complement

Type: behavior_complement. Not done: killing or interrupting in-flight work; starting an explicitly
stopped producer; restarting in place after a limit or an uncertain state (a fresh run directory is the
Octopus's decision); creating a run directory or a new plan; judging the quality of worker output;
reading anything outside configured paths. Pausing a producer on a quarantine stack; removing a pause file whose text is not exactly Crab's at load migration.

## reverse_reconstruction

Type: reverse_reconstruction. From a journal line (values, raw, latches, result) the inputs compatible with
the action are recovered as the set of readings that put those raw conditions on for h evaluations; the
exact files between evaluations are not unique (many directory contents give one count).

## checks

### Checks

Module tests: readers_count_and_unknown_is_not_zero, starved_starts_a_stopped_producer_once_with_hysteresis,
clogged_pauses_through_the_marker_and_resumes_below_the_band, stall_restarts_only_by_policy_and_limit_holds,
an_uncertain_state_is_reported_never_paused_or_restarted, boundaries_are_exact_and_unknown_never_triggers,
the_public_example_validates. Angel: flow_probes_act_journal_and_answer_through_dispatch (a real shell producer).

## History

### 2026-11-04 · Cost

Per evaluation of n <= 64 probes: each source one bounded read (directory <= 100000 entries, JSON <= 1 MiB,
JSONL <= 256 MiB streamed, tails <= 1 MiB), each glob segment one listing sorted by mtime. Measured 2026-11-04
(prospective estimate was < 100 ms): ten probes over the live r2 Fish and the Eel took about 30 ms per read-only
pass, the same summary twice. One journal line per transition or action; the state file per evaluation.

### 2026-11-04 · document

#### Epoch 2026-11-04 (pool)

FlowConfig.pools (validated: unique pools, a worker in one pool); Flow owns pool::Pools, ticks them after the probes, adds their lines to status (a pool-only group gets the pool's summary), their kinds to kinds() and their backlog to capacity().

#### Epoch 2026-11-04 (quarantine role)

Live evidence: Fish pagers t05 and t07 (and later t04) were paused at quarantine 11/8 and 10/8 and stayed
paused, stalled for over an hour: a quarantine is a review backlog that never drains by itself, so a
high-water pause on it parks a working pager. Stacks now carry a role; quarantine stacks feed a rate anomaly
(dquarantined/dfinished over a window) reported to attention, never a pause. `migrate` at Flow::open clears
the clog latch of a probe paused by Crab on stacks that no longer pause, so the first evaluation removes Crab's
own pause file. `stable_facts` gives the council's fingerprint the conditions, over-high-water stacks and
raised quarantine anomalies without moving counts. Prospective prediction: with the converted configuration,
t04/t05/t07 resume within one evaluation of the deploy, and a pager with a near-total quarantine rate
(cumulative t05 88 of 90 targets, t07 80 of 101 on 2026-11-04) is reported once, not paused; the cost per
evaluation grows by one bounded read per finished source.

### 2026-11-04 · checks

Heading `### Checks (2026-11-04)` stays in checks as `### Checks`.

Red controls (each turned a test red, 2026-11-04): starved boundary `<` -> `<=`; hysteresis ignored; starved
never true; clogged never true. A first run found a second restart after progress resumed inside the latch
window; the decision now also requires the raw stall.

Live read-only pass (`mova-crab flow-status`, 2026-11-04, twice, identical summary): "9 of 9 have work; none
is clogged" over the r2 Fish; a wide-age limit watch over the 11-02 run found "hit your usage limit".

2026-11-04 (quarantine role): a_quarantine_stack_is_validated_strictly, a_quarantine_rate_is_reported_and_never_pauses,
a_clog_on_a_stack_that_no_longer_pauses_lifts_crabs_own_pause_at_load (the live t05 memory of 2026-11-04: Crab's
pause lifted; a foreign pause file and a still-over backlog left alone). Red controls (each turned its test
red, 2026-11-04): quarantine stacks counted in the clog; load migration removed; the refusal of a pausing
quarantine-named backlog removed. The first clog control stayed green until the test also asserted no clog
latch: the pause itself is guarded twice (the clog excludes quarantines and `pauses()` excludes them), so that
property holds structurally. The private configuration validated with a temporary test (not kept); the old
configuration is refused by the new code.
