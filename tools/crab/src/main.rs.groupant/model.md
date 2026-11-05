# crab-main

## descriptant

### crab-main

Explicit bounded generic Crab CLI.

This source/privacy checkpoint separates reusable worker machinery from concrete deployment policy. It does not change live configs, binary pins, DBs or uncertain jobs.

#### Native capability commands

These commands are selected before legacy deployment CONFIG parsing:

```sh
mova-crab native-collect "$ARTIFACT_ROOT" "$REPORT_INDEX" "$NEW_COLLECTION_DIR"
mova-crab native-recall "$RECALL_TASK" "$STATE" "$PARENT_TRACE_REPORT" "$NEW_REPORT"
```

native-collect reads a bounded ReportIndex (schema1/id/reports) and calls collect_fish_reports. Each report declares id, actor, kind, outcome, optional source, findings and 1–3 relative artifact references {path,sha256,format}. Use format=native_fish_report only for actual native reports whose schema/identity/source/findings/outcome match; use external_review_evidence for external TUI review bytes. External review labels remain caller declarations. Mova groups (kind,declared_outcome) and writes report.json plus a complete review-queue.json under an exclusive new output directory; semantic_admission=false.

native-recall requires the native RecallTask fields schema/id/actor/subject/four locations plus state_sha256, state_fingerprint, trace_report_sha256, trace_id and source_window_sha256. It calls Mova's read-only graph recall using the existing checkpoint and parent trace; exclusive JSON publication follows success. It does not parse the original passage or call train. Parent/source/state pins are enforced by the native library.

Inputs use the native bounded regular-file reader. Wrong arity fails before config/file admission. Existing serve/ctl/config behavior is unchanged. No new socket opcode or daemon auto-polling is introduced. Root or a separately configured runner owns scheduling and reviewing outputs. Partial I/O output is retained for inspection and never silently retried. Tests establish finite routing and refusal behavior, not semantic correctness or deployment migration.

## model

# crab-main

## forward

Explicit bounded generic Crab CLI.

Parse admitted JSON bytes and route one declared operation; no command means no work.

## behavior_complement

Domain: explicit bounded CLI/configuration or trusted embedding calls in this file. Observation: returned errors/results and whether the selected downstream entrypoint was reached. Assume trusted local code/filesystem. Exclude oversized JSON, unknown/extra arguments and a rejected socket response from successful CLI completion. Formula: accepted action implies declared dispatch preconditions; violating them excludes successful dispatch. This is a partial source-derived exclusion rather than a complete model of downstream behavior.

## reverse_reconstruction

A successful generic operation admits all configurations and downstream histories compatible with its artifact; returned status does not identify a unique task. Formula: R(y) = {x | declared dispatch relation relates x to y}. Preserve every compatible input/configuration and hidden downstream history; do not infer a unique source task from a generic error. Counterexample: many invalid commands share the same usage error.

## invariant

Public code has no private actor dependency. Private embedding passes only server-side configuration to an explicitly installed handler. Scientific validity, learning admission and external-process success remain separate.

## cost

Argument/configuration decoding scales with admitted bytes; selected downstream cost is not bounded by this wrapper. No speedup or numerical approximation claim.

## composition

The public core owns control/store/supervisor/Dolphin logic. Concrete deployment handlers import it through a one-way Cargo edge. Running a daemon is independent of an active root chat. Durable review checkpoints retain explicit semantic/training/code-application boundaries.

## evidence

Checks: tests::absent_or_unknown_cli_does_not_launch_anything; tests::bounded_json_refuses_oversized_and_invalid_inputs. Evidence remains finite local Rust checks; no theorem or live deployment claim.

## native_dispatch_partition

Type: command_effect_partition. Domain: explicit argv selecting native-collect ROOT INDEX NEW_DIR (arity4) or native-recall TASK STATE TRACE NEW_REPORT (arity5). Observation: returned error or native result and exclusive output. Native commands are selected before the deployment CONFIG branch; wrong arity fails before any input/config admission. Native JSON inputs use the bounded regular-file reader; the existing deployment command parser remains unchanged. No new daemon socket action, worker scheduling or shell command is implied.

Counterexample: native-collect plus an absent config-looking argument must return native usage, not enter legacy CONFIG loading. A writerless FIFO supplied as the native index must fail promptly rather than block. Collection of an explicit external review must preserve format=external_review_evidence and declared_outcome, with semantic_admission=false. Local actual dispatch tests cover those boundaries and refusal to reuse an existing collection directory.

## native_reverse_reconstruction

Type: dispatch_preimage. A successful native collection constrains argv command/arity, admitted index and output location, while root paths and underlying artifact histories remain nonunique. A usage error is compatible with many malformed argv arrays. A published recall report constrains the pinned task and native result; it does not uniquely identify a model execution history or prove root review.

## native_invariant_and_cost

Native CLI routing invokes direct public Rust capabilities and never starts a subprocess, teacher, graph-ingestion task or live daemon. Collection and recall outputs preserve downstream source/evidence status; external declarations do not become native conclusions. Native inputs are capped256KiB, collection artifacts1MiB total, recall checkpoints2MiB. Partial output after I/O failure is not automatically retried or marked complete. Existing legacy CONFIG/file/protocol handling retains its previous limits and compatibility assumptions; the new native reader does not silently harden or rewrite those unrelated paths.

## tests

### Local checks

tests::absent_or_unknown_cli_does_not_launch_anything; tests::bounded_json_refuses_oversized_and_invalid_inputs

No model/teacher/training call; normal Rust checks use temporary fixtures. Keep negative controls and downstream limitations distinct.

Native CLI controls:
- native_wrong_arity_is_refused_before_deployment_config_read.
- native_collection_cli_keeps_external_evidence_declared: actual direct native collector and existing-output refusal.
- native_input_fifo_is_rejected_without_a_writer: actual FIFO, prompt bounded failure, no output.

Legacy CLI checks remain. No daemon, terminal or model is started by these new controls.

## History

### 2026-11-01 · tests

Evidence on 2026-11-01: offline locked debug and release checks pass (68 library and5 CLI tests, including the six new bridge controls), max2 build jobs. Native collection ran twice on fresh local fixture outputs with byte-identical reports. No operational graph ingestion or positive checkpoint recall was performed by this bridge change.

### 2026-11-04 · document

#### Dolphin pod (2026-11-04)

`check CONFIG` validates a Crab configuration through angel::check without serving.

#### flow-status (2026-11-04)

`mova-crab flow-status CONFIG` -> angel::flow_observe: a read-only probe pass; no action, no latch change.
