# Native agent runner

A persistent Rust process executes an immutable queue of bounded Mova-native capabilities. Roles are metadata: the same runner can serve an observer, reviewer or coordinator. It calls the public `agent-mova` library directly. There is no shell program, model prompt, teacher transport, annotation database or neural-training command in its task schema.

## Commands

Build this standalone crate with at most two jobs:

```sh
cargo build --manifest-path tools/agent-runner/Cargo.toml --release --offline --locked -j2
agent-runner example "$CHECKPOINT_DIR" "$MOVA_DATA/runs/native-readonly-example"
agent-runner once "$MOVA_DATA/runs/native-readonly-example/config.json"
agent-runner watch "$MOVA_DATA/runs/native-readonly-example/config.json"
agent-runner status "$MOVA_DATA/runs/native-readonly-example/config.json"
```

`example` reads an existing supported synthetic checkpoint's `state.json` and `report.json`, then writes a new two-job recall/collection queue. It does not run inference or training. Its documented location palette is garden/kitchen/office/hallway; general callers construct explicit recall alternatives themselves. Concrete tasks, configurations, process logs and receipts stay outside Git.

`once` executes at most the configured one-to-four new jobs, skipping terminal or unresolved reservations. `watch` repeats bounded passes and waits when the immutable queue is exhausted. It remains an OS process independent of an open chat. Creating `run_root/STOP` prevents the next job and ends watch after an active native call returns. STOP is not a mid-call interrupt. `status` is a read-only, nontransactional snapshot and does not acquire the worker lease.

Watch emits one JSON line initially, whenever a completed pass changes the serialized health summary, and on every pass that attempts new work. The health comparison includes job identity, state, native outcome, diagnostic detail, queue identity, STOP and admission fields; it excludes only `attempted_this_pass`. Therefore a completed artifact becoming an integrity failure is reported even with no pending work, and restoring its original bytes reports recovery. Unchanged idle polls and the counter's fall to zero stay quiet. Each emitted line retains the full summary and is flushed to stdout. Durable log capture remains the caller's responsibility; restart emits the current state again.

These are pass observations, not a transactional event history or liveness oracle. A transient change repaired between observations can be missed. Lease-free `status` can observe initialization between job-directory creation and reservation publication. This reporting change does not alter reservation, lease or replay rules.

Consultations and their model defaults belong to a separate configured coordinator. This runner calls no LLM and has no role-specific model default.

## Typed configuration and queue

Configuration schema 1 has `run_root`, `queue`, `queue_sha256`, `max_jobs_per_pass` (1–4), `poll_ms` (100–5000), optional `input_root`, and `allow_trace` (false by default). Paths are absolute. A queue is at most 256 KiB, contains schema 1 and 1–64 ordered jobs, and must match the configured SHA-256. Unknown JSON fields and operation variants are rejected.

A job has a bounded path-safe `id` and an `operation`:

- `kind=trace`, with a native `TraceTask`. This capability is rejected before run initialization unless `allow_trace=true`. A read-only deployment leaves it false.
- `kind=recall`, with `intent` containing actor, subject, four location alternatives and a typed parent. `parent.kind=trace_job` references an earlier trace job. `parent.kind=pinned_checkpoint` supplies state/report references with relative paths and exact SHA-256, confined to `input_root`. The runner constructs native lineage pins from the checked prior bytes; no expected-output hashes are invented.
- `kind=collect`, with `jobs`: up to 32 unique earlier trace/recall job IDs. The runner constructs hash-bound envelopes and calls native Mova grouping. Failed and unresolved jobs remain explicitly failed or uncertain.

Queue dependency references point backwards; cycles are refused. Input paths refer only to bounded read-only checkpoint data; arbitrary commands and output paths are absent from jobs. Outputs stay under the configured run root.

A native task validation failure is per-job, so independent later work continues. Invalid queue structure, disabled capabilities, changed pins, lease conflicts and initialization failures stop admission before another native operation. Existing-checkpoint hash/lineage failures are retained as failed jobs; they never trigger trace or training fallback.

## Reservation and restart contract

The process holds an exclusive file lease for its lifetime; the lock-file inode is not removed or replaced. Initial configuration identity and a frozen queue copy bind the run. Original config/queue bytes are rechecked before passes and each new job. Reusing the root with different pins is refused.

Every new job gets an exclusive directory and a synced immutable reservation before native effects. Completed output files are hashed into a terminal receipt. Native errors receive a failed receipt and may leave partial outputs. Neither failure nor completion is automatically retried.

A reservation without a terminal receipt is **unresolved**: the operation may have completed before a crash, may be partly complete, or may not have begun. Restart preserves that ambiguity and skips it. A malformed reservation or changed output is an integrity failure, also excluded from execution. Independent new jobs may still run. Partial initialization stays available for inspection rather than being deleted or silently repaired.

Immutable metadata publication uses synced files plus no-overwrite hard links on the same local filesystem. Bookkeeping I/O failures can stop the process; they never authorize replay. This assumes cooperating lease users and a trusted local filesystem. Path checks reject symlinks/traversal but are not a hostile-filesystem sandbox, distributed consensus or universal exactly-once execution. Deployment binary/source pins remain an operator responsibility separate from queue identity. The Rust file-lease API requires Rust 1.89 or newer; native checkpoint recall currently requires Linux.

## Meaning and evidence

A completed runner job means the native function returned and its outputs were recorded. Native `supported`/`uncertain` outcomes remain separate. With `allow_trace=false`, the admitted operations only read existing graph knowledge and write new reports: no trace or graph-learning call is admitted.

The optional trace capability can retain a source-scoped episode fact in an isolated graph. That is narrow retained knowledge, not neural training, learned general tool execution or a measured curriculum gain. Software tests exercise synthetic temporary fixtures; they are not permission to run an operational trace pilot.

Collection verifies native report identity through the native format route. Scheduler failures/unresolved reservations use explicitly declared external-evidence envelopes. Grouped labels are not scientific approval, automatic correction or factual admission; `semantic_admission=false` remains explicit and the review queue preserves all entries.

Focused tests cover native fixtures, cold recall and collection, deterministic native reports twice, per-job failure isolation, unresolved restart, queue/config pins, lease/STOP, output tampering and initialization boundaries. Per-file groupants record the source-bound partial models and evidence limits. No live deployment is started by these checks.
