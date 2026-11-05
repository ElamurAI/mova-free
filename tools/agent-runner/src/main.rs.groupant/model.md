# agent-runner-cli

## descriptant

### Native runner CLI memory

The executable owns process lifetime, argument dispatch and reporting; the library owns typed scheduling, native calls, reservations and restart accounting. No model choice follows from an actor label.

A source review found that first/work/STOP-only reporting could leave a healthy last line after a completed artifact changed: the library already returned integrity_failure with zero new attempts, but watch suppressed it. The reporter now compares the serialized summary except the transient attempt counter, preserving work events and flushing emitted lines. A local red control extracted the original predicate unchanged; degradation and diagnostic-change assertions failed. The fixed reporter observes degradation and recovery once while identical idle polls remain quiet.

This correction is in observation, not scheduling. The core library source is unchanged; existing integrity quarantine and no automatic replay remain in force. A restored artifact's health can recover without rerunning its job. The fixture fabricates cached receipt metadata and calls actual pass observation; it does not call native trace/recall/collect or claim the fake report is a valid native conclusion.

Output remains a nontransactional pass snapshot. Quiet output is not proof of liveness, unobserved transient changes may be missed, and lease-free status can see directory creation before reservation publication. Stdout flush is not filesystem durability. External capture, deployment and any permitted operational demonstration remain operator responsibilities.

## model

# Native runner CLI: partial model

## forward

Domain: UTF-8 arguments and admitted external library configurations. `example CHECKPOINT_DIR NEW_DIR` prepares a read-only queue from an existing synthetic checkpoint. `once` runs one bounded pass; `status` reads a nontransactional snapshot without a lease; `watch` retains one leased Runner and repeats bounded passes. Unknown command shapes fail.

For returned summary S, let a(S) be attempted_this_pass and H(S) be its JSON value with that one field removed. The reporter remembers the last emitted H. It emits the full serialized S exactly when there is no previous observation, H differs, or a(S)>0. After emission it stores H. Thus job id/state/native_outcome/detail, STOP, queue identity and admission fields remain observable. First/work/STOP behavior is preserved. Output is flushed; then a stopped pass exits, otherwise the configured sleep follows.

The CLI directly calls typed library APIs. It evaluates no shell command or model prompt. Concrete tasks and runtime state remain external.

## invariant

A watch process retains its Runner/lease. Queue, capability, pin, reservation and no-replay gates remain in the unchanged library. Changing a cached artifact does not authorize execution. An example creates inputs only, with allow_trace=false. Successful scheduling or collection is not semantic approval.

## cost

Let B be serialized summary bytes and J the bounded job count. Per-pass observation comparison and emitted serialization cost O(B), with O(B) remembered projection; library observation and artifact hashing dominate when large artifacts are present. Idle sleep is configured100–5000ms. No wall-clock STOP bound or throughput benchmark is asserted. An unchanged idle pass produces no line; a positive-attempt pass still produces one even if health repeats.

## behavior-complement

Within admitted arguments exclude unknown command dispatch, shell evaluation, example inference/training and automatic reset of terminal/unresolved jobs. Library errors terminate the command. These are partial source-derived exclusions, not an OS proof.

## reverse-reconstruction

An output line does not uniquely identify once versus first watch. Silence does not reveal the number of idle passes or establish liveness. A restart can emit the same health again. Work counts describe one pass; they do not recover an entire execution history.

## stop-boundary

STOP is included in H, so the first stopped summary is emitted before watch exits. A marker arriving during a native call waits until return. A marker removed before observation may be missed. STOP does not interrupt a call or authorize replay.

## read-only-example

Preparation reads supported checkpoint/report metadata to create exact pins in a new external directory. It does not certify the original factual conclusion; later recall still validates native lineage and bytes.

## zero-work-health-transition

Type: observation-change detector. Domain: summaries returned by successful passes over one admitted immutable queue. Observation: the serialized fields H, including all job diagnostics. Assumptions: serialization succeeds and stdout remains writable. Formula: emit(S,Hprev) = (Hprev absent) or (H(S) != Hprev) or (a(S)>0). A completed cached artifact changed after queue exhaustion is the counterexample to the old first-or-work-or-STOP predicate: state becomes integrity_failure while a=0 and stopped=false. Local fixtures reproduce that path through actual Runner::pass. This is finite executable evidence and source reasoning, not a formal temporal proof or operational deployment.

## suppressed-observation-fiber

Type: partial observation exclusion / ambiguity preservation. Domain: successive summaries with equal H and zero attempts after an emitted summary. Such observations are intentionally silent, including the first idle counter decay after work. Silence only excludes a newly returned differing H under successful execution of this reporter; it cannot exclude a crash, blocked native call, stdout error or an unobserved transient filesystem change. Repeated corruption summaries emit once; restoring cached bytes yields a distinct H and one recovery event. Tests check both fibers without invoking native graph operations.

## checks

Three focused CLI tests exercise the real pass observer using a fabricated hash-bound cached receipt, and separately check all retained diagnostic/STOP fields and activity-counter behavior. The old production predicate extracted unchanged into the same reporter fails the zero-work transition controls. Debug/release checks are finite local fixtures. Historical delegated library tests are not rerun or promoted to proof by this change.

## tests

### Focused watch checks and evidence

This change runs only the three CLI reporter tests. The unchanged library's historical eleven tests, documented in its own groupant, are not rerun here because they include native synthetic trace setup.

The baseline control extracts the canonical first-or-work-or-STOP predicate unchanged into the production reporter call site. With the new fixtures, one test passes and two fail: an actual exhausted-queue integrity transition has no emitted line, and retained diagnostic changes are suppressed. This is a regression control on the extracted production predicate, not a claim that an operational old binary ran.

Final source checks use an isolated target and at most two build jobs:

```sh
MOVA_HOT_ROOT="$RUN/hot-debug" CARGO_TARGET_DIR="$TARGET" CARGO_BUILD_JOBS=2 cargo test --manifest-path tools/agent-runner/Cargo.toml --bin agent-runner --offline --locked -j2 watch_ -- --test-threads=1
MOVA_HOT_ROOT="$RUN/hot-release" CARGO_TARGET_DIR="$TARGET" CARGO_BUILD_JOBS=2 cargo test --manifest-path tools/agent-runner/Cargo.toml --bin agent-runner --release --offline --locked -j2 watch_ -- --test-threads=1
CARGO_TARGET_DIR="$TARGET" CARGO_BUILD_JOBS=2 cargo build --manifest-path tools/agent-runner/Cargo.toml --bin agent-runner --release --offline --locked -j2
```

- `watch_reports_actual_integrity_transition_and_recovery_once`: builds a temporary allow_trace=false queue with fabricated completed receipt metadata, calls actual Runner::pass, mutates the cached artifact, checks zero-attempt integrity_failure emission once, three unchanged bad polls silent, then exact-byte restoration emits recovery once. A fresh reporter emits current health again.
- `watch_ignores_idle_counter_decay_but_keeps_work_events`: formatter-only positive attempt counts still emit, while counter decay and repeated zero-attempt observations do not.
- `watch_preserves_other_meaningful_fields_and_stop`: formatter-only changes to native_outcome, detail, admission field and STOP emit; identical repeats are quiet. The synthetic admission flag does not confer runtime authority.

No native graph operation, teacher, model/API, training, annotation database, GPU, corpus or held-out assessment is invoked by these tests. Missing pinned checkpoint inputs are deliberately never opened because the fabricated cached job is terminal. Fixture artifact integrity is a scheduler observation, not native report-schema validation.

All three CLI tests passed in both profiles: debug0.01s and release0.00s test execution, excluding compilation; the dedicated release build passed. The source projection was regenerated twice with identical bytes. Native groupant validation found no owned-leaf diagnostics; its only three failures were the deliberately untouched ancestor source bindings, which require integration updates. Changed-source coverage passed with no missing owner. Exact logs, baseline red, source/binary hashes and projection receipts are recorded externally. These are finite software assertions and source-derived models, not a proof of event delivery, a throughput measurement or a live process deployment. Stdout flush is source-reviewed; the tests inspect the production reporter's emitted strings rather than launching a persistent operational watch.

## History

### 2026-11-01 · tests

Heading `### Focused watch checks and evidence — 2026-11-01` stays in tests as `### Focused watch checks and evidence`.
