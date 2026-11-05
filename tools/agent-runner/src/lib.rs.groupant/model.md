# agent-runner-library

## descriptant

### Native runner diagnostic memory

The runner owns scheduling and recovery; native agent-mova owns bounded inference, graph recall and report grouping. Role labels are data. Keeping these layers separate avoids treating a coordinator's model preference as authority to execute arbitrary commands.

The forward model tracks queue order, reservations, terminal receipts, native outcomes and output hashes independently. The reverse model deliberately retains the crash ambiguity between no native call, partial work and completed work whose terminal receipt was lost. The matching exclusion is no automatic replay, rather than an unsupported exactly-once claim.

Read-only capability closure is explicit: allow_trace defaults false, and a queue containing trace is refused before initialization. Existing checkpoint/report pins permit independent recall followed by collection without first learning a new episode. Trace remains an opt-in capability for separately authorized uses; the current bounded cycle performs no operational trace or training pilot.

A concrete integration boundary was the new native ArtifactFormat field: completed Fish reports must use the native schema/identity route, while scheduler failures and unresolved reservations remain declared external evidence. Merely hashing an arbitrary JSON record does not make it a native or semantically approved finding.

Lockfiles retain a stable inode; metadata publication uses no-overwrite links rather than overwriting old records. STOP checks are between operations and include a dangling marker. Ordinary reads compose the native descriptor gate; larger checkpoint reads also use Linux descriptor-level bounds to avoid a FIFO blocking before validation. Ancestor trust/TOCTOU and durable local-filesystem support remain explicit limits.

Initial lock generation encountered a yanked transitive private-gemm-x86 version. The tested agent-mova lock was retained as the dependency seed; offline Cargo metadata added the new root package without refreshing the pinned graph. This is setup evidence, not a failed mathematical or software negative control.

## model

# Native queue runner: partial source-bound models

## forward

Domain: schema1 configuration and a bounded immutable queue on a trusted local filesystem, with cooperating process-lease users and successful required bookkeeping I/O. Let Q=(j1,...,jJ), J<=64, Hq=SHA256(queue bytes), Hc=SHA256(config bytes), and L=max_jobs_per_pass in1..4. Native source windows, graph checkpoints and report artifacts retain their own API bounds.

A run binds Hc/Hq and copies the queue. Per-job observation is pending (no directory), reserved_unresolved (matching reservation and no terminal receipt), completed/failed (matching terminal receipt), or integrity_failure. A pass scans Q in order, skips every nonpending job, checks STOP and pins before each admission, and attempts at most L new jobs. Exclusive directory creation and synced reservation publication precede the native operation. An Ok native result plus output inventory produces completed; native/postprocessing Err produces failed with possible partial effects. A bookkeeping publication failure can leave an unresolved reservation and stop the process.

Trace calls native episode inference/retention only when allow_trace=true. Recall derives exact native lineage from an earlier supported trace job or confined hash-pinned existing checkpoint/report. Recall rechecks source bytes after the native call and writes a new report. Collect builds native/external evidence envelopes from earlier jobs and calls bounded Mova grouping. Actor names are metadata, not an authorization or model-selection mechanism.

## invariant

For fixed run identity, every admitted job has its reservation before native effects. An observed terminal or unresolved job is not automatically invoked again. Source pins, job hash, queue identity and terminal artifact hashes remain separate checks. At most L new jobs are attempted per pass; skipping cached jobs does not consume this count. A completed call is not necessarily a supported conclusion. Every summary and native collection retains semantic_admission=false.

With allow_trace=false, queue admission excludes Trace variants before run initialization. Admitted recall/collect operations read existing knowledge and write reports; they do not invoke trace or train. File-lease exclusivity assumes the platform's local lock semantics and cooperating callers. No universal transaction across native outputs and terminal receipt is asserted.

## cost

Let Bq/Bc be queue/config bytes, J the jobs, A the current bound artifact bytes and L the per-pass cap. Queue validation and job selection cost O(Bq+J); pin rechecks and cached-state inspection additionally hash Bc+Bq+A. Native capability costs remain separate. Recall can read at most2MiB of state and256KiB of parent report; collection has its own1MiB total artifact gate. Watch repeats bounded passes at configured100–5000ms waits; polling interval is not a wall-clock completion, fairness or preemption guarantee. No speedup or numerical approximation bound is measured here.

## behavior-complement

Within the stated typed transition universe, exclude native trace with allow_trace=false, a shell/model operation admitted as a queue variant, forward/cyclic dependency references, more than L new jobs in one pass, automatic replay of a matching reservation without a terminal receipt, silent replacement of existing output, and collection conferring semantic approval. These are useful source-derived exclusions, not the complete complement of arbitrary Rust/OS behavior. Filesystem faults and hostile ancestor replacement remain separate premises/limits.

## reverse-reconstruction

A summary plus Hq/Hc constrains compatible queue/config bytes under collision-resistance assumptions, but does not uniquely recover execution timing or an unrecorded native history. Reserved-unresolved admits at least never-started, partly-complete and completed-before-receipt-loss histories. A failed receipt can retain only a bounded error explanation and does not reconstruct every partial effect. The same recalled answer can arise from different graphs; state/report/window pins constrain lineage beyond the answer itself. Hashes establish integrity claims, not author identity or factual truth.

## disabled-capability-partition

For allow_trace=false, a queue containing a Trace job is rejected while reading inputs, before new run-root initialization. With only recall/collect, the allowed native call set is closed under those two variants. Counterexample to a broader learning claim: a supported cold recall with zero new_frames and changed_weights is not a training improvement. The software tests explicitly enable trace only for isolated temporary fixtures; this is not an operational trace pilot.

## checkpoint-lineage

The queue parent is either an earlier Trace job or an external pair of relative state/report pins confined to input_root. A supported earlier trace requires a valid terminal receipt and bound state artifact. Existing pins require exact bytes before the native call. Native RecallTask additionally binds decoded state fingerprint, trace report hash/id, source-window hash and the intended question/alternatives. The native API checks this lineage and sealed checkpoint decoding; the runner rehashes the two inputs afterwards. A malformed or changed checkpoint fails that job without a trace/training fallback.

## per-job-error-partition

Native task validation, dependency admission, capability execution or output-inventory errors become failed terminal receipts with partial_native_effects_possible=true. Later independent pending jobs may continue. Existing integrity-invalid jobs are skipped. Global input-pin, queue-structure, lease and initialization failures stop admission; bookkeeping publication I/O failures may also stop a pass and retain unresolved state. Failure-isolation claims therefore do not imply progress through disk exhaustion or a corrupted run identity.

## artifact-integrity

Completed receipts bind up to three admitted result files: report.json, state.json and review-queue.json. A changed/missing/nonregular artifact makes the observation integrity_failure and never grants replay. Native ordinary bounded reads are reused; larger state reads verify opened descriptors and use Linux no-follow/nonblocking flags before regular-file validation. Ancestors and ownership remain trusted, so this is not a hostile-filesystem sandbox.

## collection-partition

Completed native reports use native_fish_report format and preserve actual id, actor, capability, outcome, source and findings. Failed/unresolved scheduler records use external_review_evidence format with explicitly declared labels. The native collector verifies their bytes, groups declared outcomes and retains all envelopes. An unresolved reservation becomes an uncertainty report, not evidence that the native call never happened. A collection's completed receipt means report construction succeeded, not that the collected claims became true.

## stop-boundary

Any existing STOP marker, including a dangling symlink marker, prevents the next admission. An in-progress native call is not interrupted. A process death releases its OS lease but does not remove job reservations; a later worker may acquire the lease and still must skip unresolved jobs. Removing a marker is not a policy for retrying terminal or unresolved work.

## publication-crash-fiber

Reservation/receipt publication writes and syncs an exclusive pending inode, links it into the final name without replacement, syncs the directory, then removes the temporary link. A crash before the final link, after it, or before cleanup leaves different observable residues. A missing terminal never justifies replay; a valid final receipt can coexist with a retained pending link. Partial initialization or unowned contents are refused without deletion. Hard-link/fsync support and stable lock inodes are required premises.

## checks

Focused local software fixtures cover positive trace/recall/collection and deterministic native reports twice; unsupported native input; per-job failure isolation; reserved restart without replay; queue/config mutation; lease/STOP; output tampering; queue identifiers/forward dependencies/partial initialization; read-only example and pinned-input failure; default-disabled trace/unknown shell variant; Linux FIFO/symlink/dangling-marker handling. Actual commands/results are recorded in model.md#tests after execution. These tests use invented temporary data only, with no teacher, annotation DB, neural training, full corpus, GPU or held-out assessment.

## tests

### Checks and evidence

Final source-matched commands used isolated temporary-fixture roots and at most two build jobs:

```sh
MOVA_HOT_ROOT="$RUN/test-hot-debug" CARGO_TARGET_DIR="$TARGET" CARGO_BUILD_JOBS=2 cargo test --manifest-path tools/agent-runner/Cargo.toml --offline --locked -j2 -- --test-threads=1
MOVA_HOT_ROOT="$RUN/test-hot-release" CARGO_TARGET_DIR="$TARGET" CARGO_BUILD_JOBS=2 cargo test --manifest-path tools/agent-runner/Cargo.toml --release --offline --locked -j2 -- --test-threads=1
CARGO_TARGET_DIR="$TARGET" CARGO_BUILD_JOBS=2 cargo build --manifest-path tools/agent-runner/Cargo.toml --bin agent-runner --release --offline --locked -j2
```

Eleven distinct library tests passed in each final profile, zero failed/ignored: debug6.32s and release1.40s for the test executions, excluding compilation. CLI and doc-test targets had zero separate tests. Dedicated release executable build passed. These are finite software fixtures, not a throughput benchmark or a semantic proof.

Named controls:
- supported_trace_cold_recall_collect_and_twice
- a_native_failure_does_not_block_an_independent_job
- unresolved_reservation_is_not_replayed_after_restart
- lease_stop_and_read_only_status
- changed_queue_and_configuration_refuse_before_effects
- tampered_receipt_artifact_stays_quarantined_while_other_work_runs
- queue_identity_forward_dependencies_and_partial_initialization_are_rejected
- read_only_example_reuses_existing_fixture_without_trace
- trace_defaults_off_and_shell_operations_do_not_parse
- pinned_input_failure_is_recorded_without_fallback_and_collected
- linux_reads_refuse_fifo_and_leaf_symlink_and_stop_sees_dangling_marker

The positive native fixture compares report bytes across two independent temporary runs, retains an unknown native outcome and cold-recalls the saved garden conclusion. Repeated passes attempt zero terminal work. Restart control leaves a matching reservation without result/receipt and confirms it remains untouched while independent work executes. Changed queue/config bytes and disabled trace are refused before new native effects. A changed pinned checkpoint fails recall and is collected as external failure evidence without fallback; original checkpoint files are never repaired or overwritten.

Temporary unit setup invokes the optional trace capability solely to construct invented local fixtures. It is not an operational trace/training pilot. No teacher/LLM, annotation DB, neural training, GPU, full corpus or held-out assessment ran. Persistent operational watch was not launched before root review.

Raw commands/setup logs, isolated profile results and source/binary hashes remain under the external agent-runner-native run. Earlier nonisolated-environment and eight-test development logs remain separate; they are not additional distinct test identities. Source-bound syntax and structural checks remain distinct from these runtime assertions. Root owns central registry/ancestor integration and any permitted read-only operational demonstration.

## History

### 2026-11-01 · tests

Heading `### Checks and evidence — 2026-11-01` stays in tests as `### Checks and evidence`.

Dependency context: the copied agent-mova subtree is exactly commit d0b4f1a0a333d19d90551e3fe6229274be4f3c07. Fresh offline lock generation initially hit a yanked transitive version; the tested dependency lock seeded offline metadata resolution, retaining that graph. This setup failure is not counted as a negative control. Final Cargo.lock SHA-256:6bac2cd65232827cb5a3d27104c87932840c4d0a673e7f746270bc1be813a537.

Final source hashes: library67219e7a3b9e8d7162a7223ff04ea11bed153257380e59f0605223ed93829233; CLIedfd2fe67e81266dbfb29b2774cda6c2da3e93b648a14668f62cd35d373aa8c5. Release executable SHA-256:d5580df9e7ce9165ff778fa4d5f1ef09281e21983e1ab608bdd4f2eda72e597b. Build artifact identity does not establish a deployment or semantic gain.
