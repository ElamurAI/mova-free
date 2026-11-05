# Prepared Fish queue supervisor

Mova's animals and the Octopus, the large language model that develops Mova, are described in [the sea animals of Mova](../../docs/animals.md).

Public provider-independent scheduling for prepared annotation tasks. Deployment configuration, content, selection and authorization stay outside this crate. It performs no database import, training or assessment. Accepted drafts require independent semantic review.

Check: `cargo test --manifest-path tools/fish-swarm/Cargo.toml --offline --locked -j2`. Commands: `mova-fish-swarm run CONFIG_JSON` and `mova-fish-swarm status CONFIG_JSON`. Each worker needs a separate runtime root and disjoint immutable queue.

## Configuration

JSON fields: `schema:1`, `root`, `queue`, `queue_sha256`, `fish_executable`, `fish_executable_sha256`, `not_after_unix`, optional `timeout_seconds` (default 300, range 1..900), `poll_seconds` (default 5, range 1..60). Actual heartbeat interval is capped at five seconds. Deadline is exclusive Unix seconds, chosen by deployment. Raw config identity is durable: even reformatting requires a new root.

Queue JSONL fields per line: `schema:1`, `id`, `task:{path,sha256}`, `profile:{path,sha256}`, `source_pins:[{path,sha256}]`, `provenance:{...}`. IDs contain 1–64 ASCII letters/digits/underscores/hyphens. Source pins require 1–64 entries. Provenance records caller-declared purpose/version; the supervisor does not infer source sufficiency or semantic correctness. Task JSON contains `id`; executor receipts call it `task_id`. Native executor validates full Task/Profile schemas.

Paths must be canonical absolute regular files without symlinks. Commitments are 64 lowercase SHA256 hex characters over raw bytes. Each file, including queue/binary/output, is bounded to 64 MiB. Unknown fields, wrong types, duplicate IDs and changed pins fail closed. Empty queues wait without repetition.

## Dispatch and recovery

A cooperative flock lease excludes another owner. All retained job directories are inspected before STOP/PAUSE/deadline can imply quiescence. Unknown or malformed terminal history stays uncertain. Signals and deadline are rechecked after pin validation and durable reservation. Task/profile bytes are copied into an exclusive job directory. Command: `FISH_EXECUTABLE annotation-execute TASK_COPY PROFILE_COPY NEW_OUTPUT_DIR DEADLINE_UNIX TIMEOUT_SECONDS`.

Reservation is durable before spawn; accepted PID has a separate record. Outputs, stderr and terminal receipts are never reused. Completed entries are skipped. Reservation without valid terminal, including an entry absent from the queue, stops as uncertain. A bound invalid_response may instead be durably quarantined only with explicit successful model exit, reaped child and absent process group. Quarantine skips that job permanently and permits the next distinct fresh job; it never marks annotation gold or retries the bad answer. Neither automatically retries. Inspect and explicitly admit a new queue/root if warranted; never delete reservations to force progress.

STOP/PAUSE are job boundaries. Active work drains under its timeout even after campaign deadline. A stalled executor gets timeout plus ten seconds cleanup grace, then its dedicated process group is signaled. Executor must bound separately grouped descendants. An outer timeout or unbound/missing/invalid receipt remains uncertain permanently, including after restart, STOP or expiry. A known-closed invalid_response is a separate review-only quarantine, not an uncertain timeout. Terminating the outer process group cannot establish death of separately grouped children. Supervisor crash can also leave children alive. Trusted cooperative filesystem is assumed; this is not a hostile-writer sandbox.

Exit zero alone is insufficient. Regular `output/receipt.json` requires schema 1, exact task ID, `task_file_sha256` matching the raw pinned input file, wellformed separate native typed `task_sha256`, status `validated_review_only`, `review_only:true`, `training_authorized:false`. Raw profile_file_sha256 must also match the copied profile. Explicit model_process_started, model_process_reaped and model_process_group_absent must all be true, model exit_code must be zero and replay_authorized false. Pretty and compact typed serialization may have different hashes; the generic scheduler does not reproduce native typed serialization. Known-closed quarantine applies these same bindings to status invalid_response, even though the executor command exits nonzero. Each retained success/quarantine audit rereads the hash-bound inner receipt and actual task/profile copies. Outer receipts retain exit, timeout, artifact hashes and pending semantic correctness. Transport/shape eligibility is not truth or training admission.

## Observation

Status fields: config hash, PID, deadline, state, `heartbeat_unix`, `observed_unix`, `active_model_call`, `no_unresolved_reservations`, review-only flags, details. Heartbeats refresh at most every five seconds subject to filesystem/scheduling delay. Status command adds `lease_held`, `status_is_snapshot:true`; stale snapshots do not prove liveness.

States: reserved, active, completed_review_only, completed_quarantined, paused, queue_exhausted, stopped, deadline_reached, uncertain, failed. Only active reports model-call true; uncertain reports null. Other states report false. Completed-review-only and completed-quarantined are per-job states, never campaign completion. Reservation and outer receipt raw profile commitment is named profile_file_sha256; native profile_sha256 remains the separate typed commitment. STOP/deadline exit leaves a final snapshot without continuing heartbeats. Priority coordination must observe fresh paused state or independently verified terminal quiescence; PAUSE-file presence alone cannot show a prior call finished.

Fixtures use a local Rust fake executor. No real model/GPU/database/training/assessment runs. Adjacent source-bound groupants preserve formulas and limits. Registry fragment requires explicit main-instance integration.

The 64-MiB bounded regular-file read also applies to the pinned Fish executor. Build the executor in release mode before deployment: a large debug executable can exceed that cap and fail before a job reservation. A retained paused status is only a past observation; verify the controller process and heartbeat before reporting a live worker. Changing a pinned executable or configuration requires a fresh immutable deployment root.

Provider busy (Terra/Codex): when the executor proves an exact Codex capacity refusal before any work (`teacher::provider_busy`; inner status `provider_busy`, nonzero exit, no reply, raw stdout hash bound), the attempt moves to `busy/<job>/<k>`, a line goes to `provider-busy.jsonl`, and the job returns to the queue. Retries wait 60, 120 and 240 seconds after the 1st, 2nd and 3rd refusal (`provider_busy_backoff`); a 4th refusal gives `provider_busy_held` with no automatic retry until review. Neither state is uncertain or a usage limit, and `no_unresolved_reservations` stays true. Usage limits, partial output, timeouts and any unverifiable busy attempt keep the uncertain path. `audit_history` verifies retained busy attempts and reports `provider_busy_attempts`.
