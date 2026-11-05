# Mova-native agent capabilities

Mova's animals and the Octopus, the large language model that develops Mova, are described in [the sea animals of Mova](../../docs/animals.md).

This public crate supplies three narrow capabilities that a persistent role runner can call: source-window location inference, retained location recall, and bounded report collection. Actor names are metadata. Fish and Crab are the first intended clients; other roles are not migrated by this crate.

## Native computation and retained experience

`run_trace(&TraceTask, new_dir)` reads a bounded third-person episode with `world::babi::Story`, asks for one subject's location, and keeps the native witness. It admits only an answer among four caller-specified alternatives and a context without reported parse gaps or dialogue markers. It then inserts the inferred, source-scoped conclusion into an isolated `world::knowledge::retention::Retained` course. Native `PragStudent::answer/why` must uniquely support the same location through the selected option's own lesson frame before a checkpoint is saved.

This changes a real Mova knowledge graph. With no exercise labels, no role weights are trained. Retention of one episode conclusion is not learned general tool execution, improved language understanding, or a measured curriculum gain.

`recall(&RecallTask, state, trace_report)` loads the saved graph and answers without parsing the passage or calling train. The task pins the state bytes, native payload fingerprint, trace report bytes/id and source-window identity. A supported parent trace and matching question are required. A Linux sealed anonymous snapshot supplies verified bounded bytes to Mova's existing path-only checkpoint decoder. Recall leaves the graph fingerprint unchanged.

`collect(trusted_root, &ReportIndex, new_dir)` checks bounded artifact bytes and groups declared kind/outcome columns through `mlab::table::groups`. It writes a deterministic systematized report and a lossless review queue. Every envelope survives, including failures, uncertainty and supported reports. Neither a teacher review nor a majority vote becomes truth.

## Public API

All request/report types derive serde serialization; task, window, envelope and index records reject unknown fields. The library exposes `TraceTask`, `RecallTask`, `SourceWindow`, `Sentence`, `FishReport`, `Finding`, `Outcome`, `ArtifactRef`, `ArtifactFormat`, `ReportEnvelope`, `ReportIndex` and `CrabReport`, plus `digest`, `read_bounded` and exclusive `json_new`.

A trace request contains schema=1, id, actor, one ASCII-word subject, four distinct ASCII-word locations, and a source window. A window has document id, optional declared document_sha256, window_sha256 and 1–16 sentences. Each sentence has id, text and optional speaker. The verified window digest hashes serde's serialized sentence array, including order, ids and speaker fields. The optional whole-document digest is a caller declaration: this operation does not read the whole document.

A recall request adds these required pins to schema/id/actor/subject/locations:

- `state_sha256`: exact checkpoint bytes;
- `state_fingerprint`: native decoded payload fingerprint;
- `trace_report_sha256`: exact parent trace report bytes;
- `trace_id`: parent trace id;
- `source_window_sha256`: parent source window identity.

The input digest includes these pins, so identical questions against different retained states are different invocations. `prepare_synthetic_recall` is only a convenience for the documented example's garden/kitchen/office/hallway palette. General callers construct explicit alternatives and lineage pins themselves.

A report index has schema=1, id and 1–32 envelopes. Each envelope declares id, actor, kind, outcome, optional source, findings and 1–3 artifact references. References have relative path, sha256 and format:

- `native_fish_report`: decode the actual FishReport and require matching schema, id, actor, capability/kind, outcome, source and findings.
- `external_review_evidence`: preserve arbitrary bounded review bytes under their hash. Envelope labels remain caller-declared manifest metadata; this route does not parse or certify the external report's identity or conclusions.

External review metadata must not be presented as a native Fish report. Groups explicitly contain `declared_outcome`; `semantic_admission` is always false. The queue retains artifact formats so a reviewer can distinguish both routes.

## Running an isolated synthetic pilot

Use an external empty run directory, isolated `MOVA_HOT_ROOT`, and CPU execution. Build from this directory with `cargo build --offline --locked -j2`. No teacher transport, remote model call, arbitrary shell tool, DB mutation or production publication is part of a capability.

```sh
agent-mova capabilities
agent-mova example "$RUN/inputs"
agent-mova trace "$RUN/inputs/trace.json" "$RUN/trace"
agent-mova prepare-synthetic-recall "$RUN/trace/report.json" "$RUN/trace/state.json" "$RUN/recall-task.json"
agent-mova recall "$RUN/recall-task.json" "$RUN/trace/state.json" "$RUN/trace/report.json" "$RUN/recall-report.json"
agent-mova collect "$RUN" "$RUN/report-index.json" "$RUN/collection"
```

The synthetic episode has a person carry an apple from kitchen to garden. The unknown-subject request is a negative control. The example's recall-template file intentionally has unfilled pins and cannot execute; prepare the bound request after a successful trace.

Effectful operations require a nonexistent output directory/file. They never overwrite an old checkpoint. Partial output after I/O failure remains for inspection; there is no exactly-once transaction or automatic retry guarantee. A runner must own reservations, task scheduling, recovery and invocation-specific private configuration. These are capabilities, not an autonomous daemon.

## Tests

Focused unit tests cover window/alternative bounds, traversal, descriptor-level regular-file bounds including FIFO/symlink rejection, conservative dialogue exclusions, and lossless deterministic collection with hash/schema/id controls. Two fresh trace or recall invocations on the same input are expected to give byte-identical reports and checkpoints; recall leaves the state fingerprint and weights unchanged. Native outcome relabeling, wrong state bytes and a wrong window hash are rejected before output publication. These are finite checks, not a formal semantic proof.

## Limits

The context gate is deliberately conservative: explicit speaker fields, quote marks and case-insensitive first-person markers require separate review. It does not resolve arbitrary pronouns, perspective, negation or narrative pragmatics, and it never silently corrects an annotation. Accepted third-person syntax can still be ambiguous or misparsed; native inference/analysis agreement is diagnostic evidence, not independent biological or real-world evidence.

Each task window is bounded to 16 sentences, 4096 bytes per sentence and 32768 text bytes. Regular input/artifact reads are capped at 256 KiB, checkpoints at 2 MiB, and collection artifact reads at 1 MiB in total. Linux descriptor flags prevent blocking on a replaced FIFO and reject leaf symlinks; opened descriptors are checked before bounded reads. Artifact ancestors and the configured root are an owned trusted directory premise, not a hostile filesystem sandbox. SHA-256 pins integrity and lineage claims, not author authenticity. Hash collisions are outside the model.

The native parsers, scoring rules, kind graph and checkpoint decoder remain dependencies with their own compatibility limits. No numerical approximation bound, general planner, automatic factual admission or generalization score is claimed.

The synthetic example and `prepare-synthetic-recall` label the task actor
`specialist-whale`; the label only names the requesting role and gives it no
extra capability. Production Fish are the concrete AST/syntax and
pragmatic-annotation workers in tools/fish; retention demonstrations do not
expand their remit into a general executor. Public Fish defaults and private
deployment overrides are separate policy.

The source-bound two-flag admission model conditions on all other validation
coordinates and checks every id/outcome match combination. Its refused fiber
contains three cases, so refusal alone cannot reconstruct the mismatched field.
Converse/complement catalogue entries derive from this one finite table; they
supply no independent semantic or Rust evidence.

Grouped summaries include a binding category. native_bound means a referenced
native report passed the declared field comparisons; declared_only keeps an
external review's labels as declarations. These categories never share one
kind/outcome group. Missing binding fields in historical summaries decode as
legacy_unclassified. Group counts do not measure independent or truthful claims.
