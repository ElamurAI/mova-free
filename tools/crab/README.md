# Crab

Crab is the single Rust deputy of the Octopus, a large language model (LLM). Crab
understands the Octopus's English through Mova and supervises the other animals.
See [the animals of Mova](../../docs/animals.md).

Crab is Mova's public Rust control and provenance layer: bounded socket protocol, request journals, result/report indexing, annotation version store, disjoint worker supervisor and low-priority independent Dolphin review adapter. These are real implementations, reusable without private code.

Run local fixtures with a dedicated target: CARGO_BUILD_JOBS=2 cargo test --manifest-path tools/crab/Cargo.toml --release --offline --locked -j2 -- --test-threads=2. Compiling/testing does not authorize a live launch.

```text
mova-crab serve|prepare|annotate|ingest|status|dolphin-status|check|flow-status CONFIG [--once]
mova-crab export CONFIG NEW_DIR
mova-crab dolphin-finish CONFIG ID REPORT
mova-crab ctl SOCKET REQUEST
mova-crab native-collect TRUSTED_ROOT INDEX NEW_DIR
mova-crab native-recall TASK STATE TRACE_REPORT NEW_REPORT
crab-tmux-worker (--lock FILE | --no-singleton) [options] SESSION CWD -- COMMAND [ARGS...]
```

`check` validates a configuration without serving, booting workers or touching terminals (it creates the private run-root directories if they are missing). `flow-status` reads the configured flow probes once, like "Show the queues.", without acting or changing their memory. Configuration paths, queues, credentials, model choices and limits are external inputs; none is supplied as a live default.

The public server uses NoExternalRoles. Specialized handler calls fail closed unless an embedding application supplies ExternalRoleHandler through serve_with_roles. The wire operations `start_whales` and `whale_status` (role `whales`, configuration keys `whale_config`, `whale_swarm_config`) are served by the handler `mova_whale::WhaleRoles` in `tools/whale` (`mova-whale crab CONFIG`); the dependency points from Whale and deployments to Crab, never the reverse. Accepted compatibility identifiers are listed in [the Whale README](../whale/README.md#compatibility-identifiers).

Crab is an independent process that works on durable signals and checkpoints rather than a live chat ([durable signals](../../docs/animals.md#durable-signals-not-a-live-chat)). Review, DB selection, code application and model activation remain separate boundaries.

See [the pipeline](../../docs/animals.md) and [storage contract](../../docs/training.md). Ordinary checks use local fixtures and make no live terminal, teacher, model or training calls.

## Abstract roles and terminal profiles

Octopus and Dolphin are abstract AI roles rather than provider/model restrictions. Opus plus Sol is a recommended pairing; the default consultant preference is the CLI alias opus (latest available Opus), never a guessed versioned endpoint. A trusted deployment may select another provider/model. Training still follows the separate Codex teacher policy.

The legacy opus_session field accepts consultant_session as an alias. A nonlegacy session needs consultant_terminal with provider/model labels, pane_command, exact idle_marker, optional prompt_marker/required_text, optional idle_window and owner_marker. The idle marker must stand on one of the last `idle_window` non-empty lines (1..32, default 1), a leading TUI bullet ignored; the Codex TUI prints footer lines below it and needs 6. Busy and modal phrases are looked for only in the bottom twelve lines. The corresponding trusted tmux session must already carry @mova_crab_owner equal to owner_marker. Capture requires the exact session name, one window/pane, alive expected command and matching owner marker. Crab attaches through argv and never starts/switches/authenticates that consultant. Provider/model labels express a preference; attached_model_verified=false keeps the lack of actual model attestation explicit. Missing provider/model labels default to claude-cli/opus. Unknown UI releases need separate real smoke evidence.

## Supervised workers

Crab manages its little ones: it starts, adopts, restarts and stops the long-running workers listed in its own trusted configuration (`workers.rs`). Command lines come only from that configuration, never from a request or prompt.

```json
"workers": [{"name": "eel", "argv": ["/abs/eel", "--config", "/abs/eel.json"], "cwd": "/abs/run",
             "log": "/abs/run/eel.log", "gpu": true, "autostart": true,
             "stop_marker": "/abs/run/STOP", "stop_grace_seconds": 300, "restart": true}],
"gpu_check": {"nvidia_smi": "/usr/bin/nvidia-smi", "timeout_seconds": 15, "probe_argv": ["/abs/probe"]}
```

`name` is an ASCII identifier; `argv[0]`, `cwd`, `log` and `stop_marker` are absolute. Defaults: `gpu=false`, `autostart=false`, no marker, `stop_grace_seconds=300` (1..86400), `restart=true`; `timeout_seconds=15` (1..600). A `gpu` worker requires `gpu_check`. Unknown fields are refused.

- **Boot.** After taking its lease, Crab reads `run_root/workers/<name>.json`. A recorded process is adopted only if `/proc/<pid>/stat` has the recorded start time, it is not a zombie, the configured argv is unchanged and is the suffix of `/proc/<pid>/cmdline` (a `#!` interpreter may precede it). A changed argv over a live recorded process is `unadopted_conflict` (below). A recorded process that is gone (no pid, another start time, a zombie) is journaled as `recorded_instance_gone` and forgotten. Otherwise an `autostart` worker starts, unless it was explicitly stopped (persisted) or its stop marker exists.
- **Unadopted conflict.** The state file keeps the recorded identity (pid, start time, the argv hash it was started with, beside `configured_argv_sha256`), so every later Crab checks it again and never starts a duplicate. `worker_status` and `worker_start` answer with a `conflict` object: `recorded_pid`, `recorded_cmdline` (bounded, read from `/proc`), `configured_argv`, `reason`, a one-line `summary` and the `resolution`. A plain `worker_start` (and autostart) never signals: over a live conflict it starts nothing (`started: false`, `signal_sent: false`); `worker_stop` is refused. Only an explicit `worker_restart` takes the recorded process over: the identity is checked again (pid, start time, not a zombie), it is stopped exactly as a normal stop (marker and grace, SIGTERM, SIGKILL after 10 s; signals only while the identity holds), then the configured argv starts; a stop begun this way continues on the same process after a Crab restart. Flow's automatic restart never takes a conflict over. When the recorded process ends (for example stopped by hand), the next start, restart or tick clears the record (`recorded_instance_gone`, `cleared: unadopted_conflict`), and an `autostart` worker then starts as at boot. Take care with a holder of someone else's terminal: stopping a `crab-tmux-worker` ends the command it holds.
- **GPU gate.** A GPU worker waits in `waiting_gpu` until `nvidia-smi --query-gpu=index,name,memory.used,memory.total --format=csv,noheader` exits 0 with at least one row within the timeout, then the optional probe exits 0 within the same timeout. A failed check is recorded in `gpu-check.json` and retried after 5, 10, 20 … 300 s; a GPU worker is never started blind. Each start, including a crash restart, needs a fresh pass.
- **Ticks.** Every serve-loop iteration runs one non-blocking step: `waitpid(WNOHANG)` for own children, `/proc` identity for adopted ones, stop escalation, backoff restarts and GPU-check polling. Nothing waits inside a request.
- **Crashes.** An unexpected exit with `restart=true` restarts after 5 s, doubling to at most 300 s; a run of 300 s or more resets the backoff. If the stop marker exists at exit time, the exit is taken as a requested stop and is not restarted.
- **Stop.** With a marker: create it, wait up to `stop_grace_seconds`, then SIGTERM the process group, then SIGKILL after 10 s. Without a marker: SIGTERM, then SIGKILL after 10 s. Signals are sent only while pid and start time still match. Restart is stop followed by start; a start removes the marker first. A worker is interrupted only by an explicit stop or restart.
- **Record.** Every transition rewrites the state file with a bounded log of the last 32 events. Crab's own exit leaves workers running for the next Crab to adopt.

Journaled operations: `worker_status` (optional `worker`), `worker_start`, `worker_stop`, `worker_restart` (required `worker`, a configured name). They answer immediately with the recorded state; completion is observed through `worker_status`. `status` adds a `workers` name→state map. Nested prompt calls (`ask`) admit these four operations plus `status`, `read_reports` and the Dolphin consult queue (`consult_enqueue`, `consult_state`, `dolphin_list`); legacy `pause`/`resume`/`stop` markers, a direct `consult_opus` and Whale operations stay refused there.

Limits: Crab does not talk to workers in English; control is the configured process lifecycle plus the stop marker. Process identity assumes cooperative same-user operation; a worker that leaves its process group escapes group signals. No health is inferred from `running`.

## Work flow: every worker has work and no stack clogs

Crab makes sure its continuously running workers always have work and that their stacks do not
clog (`flow.rs`). Each worker is described by configured probes; Crab has no worker-specific
code. See [flow/crab-flow.example.json](flow/crab-flow.example.json).

```json
"flow": {"interval_seconds": 60, "probes": [{
  "name": "fish-t00", "kind": "fish", "worker": null,
  "input":  {"source": {"kind": "json_field", "path": "/abs/status.json", "field": "details.current_audit.pending_targets", "missing": 0}, "low_water": 1},
  "stacks": [{"name": "in_flight", "source": {...}, "high_water": 8, "resume_at": 4},
             {"name": "quarantined_jobs", "role": "quarantine", "source": {...}, "finished": {...},
              "rate_window_seconds": 3600, "max_rate": 0.5, "min_finished": 32, "reason_source": {"path": "/abs/status.json", "field": "dominant_reason"}},
             {"name": "uncertain", "source": {"kind": "json_equals", "path": "/abs/status.json", "field": "state", "value": "uncertain"}, "high_water": 0, "pause_producer": false}],
  "progress": {"source": {...}, "stall_seconds": 2400},
  "producer": {"worker": "materializer", "pause_file": "/abs/PAUSE"},
  "limit": {"files": ["/abs/pages/*/run/jobs/*/output/raw.stdout"], "patterns": [], "max_age_seconds": 3600},
  "restart_on_stall": false, "hysteresis": 3}]}
```

Count sources: `dir_entries` (optional prefix/suffix; dot and `.tmp` names skipped),
`jsonl_lines` (non-empty lines; a missing file is 0), `json_field` (an unsigned integer at a
dotted path; `missing` when absent), `json_equals` (1 when a dotted field equals a string),
`mtime`, and `sum` of plain sources. A reading that cannot be taken is *unknown*, never zero.
`kind` (alias `group`) is the animal kind used in English and in the capacity review; any kind
name is accepted. Limit files may use `*` segments: at each, the four newest entries are read.
The default limit patterns are the ChatGPT/Codex and Claude CLI markers (`hit your usage limit`,
`rate limit`, `too many requests`, …), matched case-insensitively in a recent tail.

**Stacks have a role.** A `backlog` (the default: input, ready or in-flight work) drains by itself once its
producer pauses, so it may pause the producer. A `quarantine` is a review backlog that never drains by itself:
pausing on it would park a working worker forever, so it never pauses. Its `source` is a cumulative count of
quarantined items and `finished` a cumulative count of finished items (quarantined ones included). Crab keeps
(time, quarantined, finished) samples over `rate_window_seconds` (default 3600, 300..86400) and raises a
quarantine-rate anomaly when Δquarantined/Δfinished > `max_rate` (default 0.5, (0, 1]) with at least
`min_finished` (default 4) finished in the window; fewer finished items make no rate (unknown, the latch is
unchanged), and a counter that goes down starts a new window. The anomaly uses the probe's hysteresis, is
reported to the attention queue once per rise with the dominant reason from the optional `reason_source` (a
string at a dotted field, at most 200 bytes), and is journaled when it clears. A quarantine refuses
`high_water`, `resume_at` and `pause_producer`; a backlog refuses the rate fields. A backlog whose name says
"quarantine" and that would pause is **refused at load**: declare `role: quarantine`, or `pause_producer: false`
to keep it reporting only. Refusing, rather than silently reporting, makes the configuration say what it means.

**Model.** Per probe and evaluation: starved = input < low_water; clogged = a backlog stack > high_water,
cleared only at or below resume_at (default high_water / 2); stalled = input ≥ 1 and the
progress value unchanged for stall_seconds; limited = a limit pattern in a tail modified within
max_age_seconds. A condition turns on after `hysteresis` consecutive raw readings and off after
as many contrary ones; unknown readings change neither. Precedence: limited, clogged, starved,
stalled. Cost: one bounded read per source per evaluation (a directory listing ≤ 100000 entries,
JSON ≤ 1 MiB, JSONL ≤ 256 MiB, tails ≤ 1 MiB).

**Actions.** Crab acts once per transition and only within these bounds:

| Condition | Crab does | Never |
|---|---|---|
| starved | starts the configured producer worker if it is stopped and was not explicitly stopped; otherwise reports | starts an explicitly stopped producer |
| clogged | creates the producer's PAUSE file (create-new, Crab's own text) and removes only its own file when the clog clears; a stack with `pause_producer: false` is reported | kills or interrupts in-flight work |
| quarantine rate high | reports it with the window's counts and the dominant reason | pauses anything |
| stalled | restarts the probe's worker once per stall when `restart_on_stall` allows; otherwise reports | restarts without the policy |
| limited | holds: no start or restart for that probe while the marker is fresh | restarts into a limit |

Everything Crab may not fix itself becomes an attention item: an unsupervised producer, an
explicitly stopped one, a limit hold (a limit-stopped worker may be left uncertain and need a fresh
run directory), an uncertain state, an exhausted plan. Transitions and actions are appended with
their probe values to `run_root/flow/journal.jsonl`; latches persist in `run_root/flow/state.json`.

**Migration at load.** Crab's own pause file holds exactly `crab flow pause: <probe>` and is a regular file of
Crab's user; that is how ownership is recorded. When Crab loads a state in which a probe is clogged with
`paused_by_crab` and none of the stacks recorded over high water (`last_values.over_high_water`) still pauses
under the current configuration (a quarantine now declared as such, or a stack now reporting only), it clears
that clog latch and journals `migrate`; the first evaluation then removes its own pause file. A file with any
other content is left in place and reported. A clog with an unknown over-list, or on a backlog that still
pauses, is left to the ordinary hysteresis. A limit marker or a council hold delays the removal like any other
automatic action.

Operations: `flow_status` (optional `group`, a configured kind) and `attention`, also admitted in
nested prompt calls. In English, through the deputy handler `crab_flow`
([flow/deputy-flow.example.json](flow/deputy-flow.example.json)): "Do the Fish have work?",
"Show the Fish queues.", "Is any worker clogged?", "Do the workers have work?", "What needs my
attention?". A phrasing the planner cannot map is refused and becomes teaching material for
Mova's reader.

## Work pools

Crab also hands out the work (`pool.rs`): a pool in the flow configuration names its **members** — each a
configured worker, an assignment file Crab writes and a report file the member writes — and its **jobs**, each with a
priority, a backlog probe (any count source above), an optional number of `parts`, a licence line shown in English and
opaque `params` copied into the assignment. Crab has no member-specific code; the member decides what a job means.
See [flow/crab-flow.example.json](flow/crab-flow.example.json).

```json
"pools": [{"name": "dictionaries", "kind": "nautilus", "lease_seconds": 1800,
  "members": [{"name": "nautilus-a", "worker": "nautilus-a-work",
               "assignment": "/abs/nautilus-a/assignment.json", "report": "/abs/nautilus-a/report.json"}],
  "jobs": [{"name": "judaism", "priority": 30, "licence": "ours", "params": {"layer": "judaism"},
            "backlog": {"kind": "json_field", "path": "/abs/jobs/judaism/backlog.json", "field": "open_items", "missing": 1}}]}]
```

**Model.** A unit is (job, part), part < `parts` (default 1: a job is never split unless it says so). A lease binds one
member to one unit until `expires_unix`. Every flow evaluation:

- a lease ends when its member reports it `finished`, `stopped` or `refused`, when it expires, or when its job is no
  longer configured; a finished or refused unit is not handed out again while its job's backlog reading stays the
  same;
- a lease is renewed (`expires = now + lease_seconds`) only while its member's worker runs and the member's report
  names the lease and was written within `lease_seconds`, or the lease is younger than one period; a dead or silent
  member's lease runs out, and only then can the job move;
- an idle member (worker running, no lease, its report not `stopped`) takes the waiting unit with the highest
  priority, then the largest known backlog, then job and part order; an unknown or zero backlog is not work;
- invariants, checked every evaluation: a member holds at most one lease, a unit at most one member.

The assignment file is rewritten atomically on every assignment, renewal and release (a released file names no job),
so a member reads it between its own steps. Every assignment and release is journaled in `run_root/flow/pools.jsonl`;
leases persist in `run_root/flow/pools.json`. A release of a `stopped` or `refused` member becomes an attention item.
Bounds: 8 pools, 32 members and 64 jobs a pool, 16 parts a job, `lease_seconds` 120..86400, params ≤ 4 KiB; a
worker belongs to one pool. Cost per evaluation: one bounded read per job backlog and member report.

The pools show in `flow_status` (a pool's kind is an animal kind: "Which Nautilus works on what?", "Show the Nautilus
jobs." with a `crab_flow` capability for the kind), in `check`, in the council digest and in the capacity review
(backlog per pool). Lines read: "The dictionaries pool: 1 of 2 members are at work.", "nautilus-a works on judaism;
the lease runs 1790 s more.", "nautilus-b is idle.", "school-science waits with a backlog of 50."

The pool tests use a fake supervisor and have negative controls.

## The council

Every `check_seconds` (default 60, 10..600) Crab composes one compact English digest (probe values and
conditions, recent flow actions, capacity per animal kind: queued work, progress per hour, hours to drain,
starved seconds; worker states and requests it did not understand) and fingerprints its stable facts: per
probe the active conditions, the stacks over high water and the raised quarantine anomalies; the flow summary
with digits masked; worker states; recent actions without times or values; refused requests. Times, round
ids, counts that move every minute, rates and ages are not facts. A round starts only when the fingerprint
differs from the last round's and `min_spacing_seconds` (default 300, 60..period) passed, or when
`period_seconds` (default 1800, 600..86400; the longest interval) passed even with unchanged facts. Any other
evaluation is a skip, journaled compactly: one line when the skip reason changes, the repeats counted in
`council/state.json`.

A round asks the idle members, the same digest to each through the consult queue, when at least `quorum`
(default 2, 2..members) are idle. A busy member's active task is never overwritten; a busy or stopped member is
recorded as absent in the journal. Below the quorum the round is skipped. A member silent after
`member_timeout_seconds` (default 600, 60..86400; `answer_timeout_seconds`, kept for compatibility, bounds it
from above) is recorded as timed out and the round closes with the others. It never blocks the next round, but
its pending consultation keeps it busy (absent) until it answers. Each Dolphin sees only the digest.

```json
"council": {"dolphins": ["opus", "sol"], "period_seconds": 1800, "min_spacing_seconds": 300,
            "check_seconds": 60, "quorum": 2, "member_timeout_seconds": 600, "max_task_bytes": 6000}
```

The quorum is at least two so that "unanimous" never means one voice; with two members both must be idle and
answer. Crab parses only the first non-empty line of each answer:

```text
VERDICT: none
VERDICT: octopus
VERDICT: action:<start_producer|pause_producer|resume_producer|hold|restart>:<probe>
VERDICT: capacity:<kind>:<more|fewer>:<n>
```

followed by a reason. Combination is over the members that answered; fewer answers than the quorum escalate
as an expired round. All `none`: the round is journaled only. The same allowed action from every member that
answered: Crab executes it within the flow bounds above (a `hold` suspends that probe's automatic actions for
one period). Anything else (disagreement, `octopus`, a capacity verdict, a malformed or failed answer, an
action Crab may not take now, an expired round) becomes an attention item with every member's verdict and
reason and the digest's numbers. A capacity verdict is never executed: the Octopus decides and tells the
operator. Answers are evidence, never executable text. Rounds are journaled in
`run_root/council/journal.jsonl` (with `absent` and `timed_out` members); refused requests for the digest in
`run_root/council/refusals.jsonl`.

The attention queue is `run_root/attention/`. Each item is fingerprinted by its source and facts (flow: probe,
action, conditions, stacks over high water, stack counts, the detail with digits masked; council: the kind of
escalation, the members' verdict labels and the digest fingerprint), never by its time, id or round. A repeat
of a known fingerprint is merged: its count, first and last time and newest summary are kept in `state.json`
(at most 512 fingerprints, the least recently seen dropped), and only a new fingerprint is appended to
`attention.jsonl`. `attention.md` and "What needs my attention?" show the sixteen most recently seen items
with their repeat counts. A queue from before fingerprints is seeded from its newest lines. Items carry no
acknowledgement yet.

The flow, council and angel tests use a fake worker supervisor and a fake Dolphin pod and have
negative controls.

## Read-only worker status attachments

The optional `status_attachments` configuration list connects additional native
workers to Crab's `status` response without changing legacy Fish or Eel control.
Each entry declares `id`, an absolute `path`, `expected_kind` and the exact
lowercase `expected_config_sha256` from that worker's immutable configuration.
An omitted or empty list adds nothing to the response. The list is
loaded with the Crab context; changing it requires a deliberate Crab restart.
No request can supply an attachment path or register a new attachment.

At most eight entries are admitted. IDs and kinds are bounded ASCII identifiers;
paths are UTF-8, at most 1024 bytes, and cannot contain parent traversal. Reads
accept regular files only, reject a final symlink and use a nonblocking open
so a FIFO cannot hold the observer waiting for a writer. Each read is capped at
256 KiB. Missing, oversized, malformed and identity-mismatched artifacts remain
separate observations; an identity mismatch withholds the projected value.

The fixed projection retains unsigned scalar counters, bounded state/page labels,
selected Boolean flags, and at most sixteen unsigned counters from each of
`counts` and `source_counts`. Excess counter projection is explicitly marked.
Prompts, arbitrary nested objects and artifact lists are not copied into the
response. A raw artifact hash preserves byte identity. Heartbeat freshness is
`fresh`, `stale` (more than 120 seconds), `future` or `unknown`; a future timestamp
never becomes a fresh zero-age observation. Projection omits invalid field types.
The ordinary serialized-response budget still applies to the aggregate response.

Attachments are explicitly read-only: legacy `pause`, `resume` and `stop` still
address their existing configured role roots. A reported PID is not a process
probe; a fresh heartbeat is not health, model quality or training admission.
Reported worker flags also remain observations rather than authority. The API
`angel::status_observation::{validate, observe}` is usable independently for a
bounded local status read without starting the Crab server. Both functions use
trusted caller configuration; parent-directory changes and truthful worker
reporting remain premises, not filesystem isolation or authentication proofs.

## Dolphins

Dolphins are LLMs in terminals ([the sea animals](../../docs/animals.md)). Crab holds them as ordinary workers and talks to them through a queue; it never types into a terminal except through the consult protocol below. An ordinary consultation answer is review evidence and is never executed; only the council can trigger an allowed flow action, when its quorum and agreement rules pass ([the council](#the-council)).

**The pod.** The optional `dolphins` list names any number of Dolphins, each `{"name", "session", "terminal", "worker", "pin"}`: a unique ASCII name, its exact tmux session, the ownership/readiness profile described above (only the legacy session may omit it), the configured worker that holds it, and an optional pin checked before every consultation. Without the list, one Dolphin named `consultant` uses `opus_session` and `consultant_terminal`. The first Dolphin is the default for `consult_opus` without a name and for the low-priority review queue (`dolphin_auto`). The standard pod is in [dolphins/](dolphins): `crab-pod.example.json` (the Opus consultant, autostarted; the Copilot consultant and the documenter, off), `deputy-pod.example.json` (their English capabilities), and the public role prompts `consultant.md` and `documenter.md`.

**The holder.** `crab-tmux-worker` runs one Dolphin as a Crab worker. It first takes an exclusive host-wide `flock` on `--lock FILE` (a singleton unless `--no-singleton` is given); if another holder owns it, it reports that holder (pid, session) and exits with status 5. Otherwise it creates the detached session `SESSION` in `CWD` running `COMMAND`, or adopts an existing session of that name whose only pane runs the command (directly or as a child of a shell; the executable is compared by file name, the arguments exactly); another command there is a conflict (status 3) and the session is left untouched. A role prompt (`--prompt-file`, `{{KEY}}` placeholders bound by `--var KEY=VALUE`, at most 100000 bytes) is appended to the command line, after `--prompt-flag` if given — never typed. The holder stays alive while the session and its command live and exits when either ends, so Crab sees the exit. On SIGTERM it sends SIGTERM to the command, waits `--grace` seconds (1..8, below Crab's 10-second kill) and removes the session. It uses only `has-session`, `new-session`, `set-option`, `list-panes` and `kill-session`.

**The pin.** `--pin-file FILE --pin-sha256 HEX`, `--version-cmd ARG… --pin-version LINE` and `--auth-file FILE --auth-key KEY…` bind a self-updating CLI such as GitHub Copilot: the file (resolved through links) must have the hash, the version command's first line must match, and the auth file (JSON with `//` comment lines) must hold non-empty values for the keys — presence only, nothing is printed and no model is called. A failed pin refuses the start in plain English ("dolphin-copilot is not pinned: version "…", expected "…"", status 6) and ends a running session at the next check (every minute). Crab checks a Dolphin's `pin` again before each consultation and records a failure instead of sending.

**The consult queue.** `consult_enqueue {dolphin, task, priority}` stores a consultation in `run_root/consult-queue/`; every serve-loop idle step delivers, per Dolphin, the next waiting entry (urgent first, then arrival) through `consult_opus` — only when that Dolphin has no pending consultation and its terminal is idle. A failed delivery is kept as `dispatch_failed` and not retried. `consult_state {dolphin}` returns the Dolphin's waiting entries, pending consultation, terminal state and latest report; `dolphin_list` lists every Dolphin with its worker and terminal state. Nested prompt calls admit these three, never a direct `consult_opus`; an urgent ask queues its consultation as urgent.

**Crab's own consultations.** `consult_triggers {refusals, worker_failures, dolphin, max_per_hour, max_task_bytes}` (absent: off) lets Crab consult the named Dolphin about a prompt it refused and about a worker that fails three times in a row, has failed, or whose GPU check failed. Each trigger fires once, at most `max_per_hour` (1..12) times an hour, with a bounded plain-text task. The answer becomes a record in `run_root/consult-advice/` attached to the trigger (and the command school's review candidates): evidence and teaching material with `actions_taken: []` — never executed, never a substitute for Mova's understanding or an assessment answer.

**In English.** With the deputy capabilities (`crab_consult` operations `enqueue`, `state`, `list`; a `task` slot of kind `text`; `group_aliases` and a `collective` list capability in `tools/octopus`), the Octopus says "Ask the Opus Dolphin: «Review this plan.»", "Is the Copilot Dolphin busy?", "Show the Opus Dolphin's answer.", "Start the Opus Dolphin.", "Stop the Copilot Dolphin.", "Is Documenter running?", "Start the documenter.", "Which Dolphins are running?". Only the quoted text is sent. A bare "the Dolphin" is refused as ambiguous with a review candidate.

**Target syntax.** Crab addresses a terminal as `=SESSION:`; tmux 3.5 resolves `=SESSION` for `has-session` but not as a pane target for `display-message`, `capture-pane`, `paste-buffer` or `send-keys`.

The holder tests run on private tmux servers; the queue and routing tests use a fake consultant terminal; all have negative controls. Real Claude and Copilot terminal UIs are not exercised by these tests; their idle markers need a live smoke.
