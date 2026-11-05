# fish-annotation-executor

## model

# Source-bound annotation execution
## forward
Domain: a valid native Fish Task, explicit Sonnet/CLI Profile, new output directory, Unix deadline and timeout in 1..900 seconds. Observation: launch/nonlaunch, retained exact request and raw diagnostics, strict draft and terminal review-only receipt. Model the visible state path as V -> R -> P -> L -> C -> N -> T: validate/evaluate/render, reserve and sync, publish request/input, launch at most one direct child, capture result, native validation, terminal receipt. Prefix failures retain available evidence. Declared evaluation use is excluded. Unknown use can obtain only a review draft with its not_teaching finding preserved; actual held-out exclusion still depends on the supervisor's pinned provenance. Before output creation the full rendered prompt must have at most 100000 UTF-8 bytes. Before L require clock < configured deadline. The active subprocess drains against its separate monotonic timeout.

## behavior_complement
Domain: the same bounded contract; observation is whether a subprocess starts or review artifacts confer authority. Invalid native binding, declared evaluation phase, unsupported profile, invalid timeout, expired deadline and oversized rendered text exclude a new launch. Existing output directories exclude replay through that directory. Invalid result envelopes, missing/unknown draft fields, stale task hashes and native source/target/label failures exclude validated_review_only. These are necessary exclusions; successful structure checks do not exclude shared semantic mistakes. Counterexample to a global exactly-once claim: the same task could be sent from a second fresh directory unless the supervisor's task ledger forbids it. Evidence: source inspection and finite fake-CLI nonlaunch/replay/schema controls.

## reverse_reconstruction
Domain: observed reservation/receipt/raw files; reconstruct only compatible local histories. A reservation without receipt has preimages including never-spawned, spawned-but-incomplete and completed-before-receipt-crash histories. It cannot identify whether the remote provider charged a request. A receipt with validated_review_only restricts the preimage to successful CLI envelope and native draft checks for the pinned task/request; it cannot reconstruct hidden reasoning, producer independence or a unique correct interpretation. Assumptions: collision-resistant hashes, trusted local filesystem and no external mutation of owned attempt artifacts. Evidence: prefix ordering and nonreplay checks; no power-loss or remote exactly-once proof.

## invariant
All successful receipts have schema=1, matching task id/hash, review_only=true and training_authorized=false. A parsed draft retains unresolved strings; findings from the original task and draft-primary native comparison remain separate. Only complete target-only native AST/pragmatic tables pass structural validation. No DB, corpus, trainer, weights or GPU-admission candidate is written. Requested model is present in actual argv; no API/provider/legacy-Codex fallback is implemented. A model alias does not prove the served model identity. Reservation and files are synced before launch and receipt before return; I/O failure remains a partial-artifact state.

## cost
Let B<=100000 be submitted UTF-8 bytes, O<=2097152 each captured stdout/stderr read, C<=4 input candidates and T<=8 targets with <=256 word rows each. Native evaluation/rendering and draft checks reuse bounded parser/map work. The supervisor sees one direct subprocess lifetime bounded by a 1..900-second configured timeout plus scheduling/termination/I/O overhead. Polling every 50ms detects rather than prevents output overshoot; no hard disk quota or hard real-time guarantee is claimed. Reading files bounds memory while raw disk writes may overshoot before detection. No inference speed, quality or percentage-error claim is made.

## composition
Public main dispatch -> exported annotation evaluate/render -> exclusive attempt -> configured authenticated Claude CLI -> strict envelope -> native draft validation -> review-only receipt. Claude receives safe mode, disabled tools/slash commands, medium effort, no session persistence and JSON output. Selected API/provider environment overrides are removed; authentication and executable/model availability remain deployment assumptions. The supervisor owns campaign-wide task uniqueness, held-out provenance, immutable queues, process independence and deadline scheduling. No theorem about those external components follows from this file's tests.

## descriptant
This is a narrow review transport, not the legacy teacher pipeline or a training authorization boundary. It reuses the original Rust syntax/pragmatic model rather than placing model-name labels on a Codex shim. The returned annotations are candidate observations. A partial or uncertain response remains diagnostic evidence; no automatic expensive retry repairs it. Current evidence is finite local fake-CLI checks; actual subscription behavior remains unexecuted here.

## deadline_admission
Type: temporal_launch_exclusion. Domain: valid tasks and wall-clock launch admission. Formula: Launch(t) implies clock_before_spawn(t)<deadline. A second check follows durable writes to close the ordinary preparation delay. Clock changes, scheduling between check and spawn and process startup latency remain assumptions; this is not an atomic clock/process theorem. Once launched, monotonic timeout governs draining rather than the wall deadline. Counterexample to a stronger claim: a request admitted one second before cutoff may finish after cutoff. Evidence: expired-deadline fake-CLI nonlaunch; no altered-clock integration test.

## uncertain_replay
Type: crash_state_preimage. Domain: same output directory and its prefix artifacts. Formula: Exists(output_dir) implies reject another run through that directory. Reservation is durable before the configured CLI is spawned. Therefore missing receipt cannot authorize automatic replay. Counterexample to unique reconstruction: a crash immediately before and immediately after spawn can leave the same reservation prefix. Evidence: successful receipt followed by replay refusal and durable timeout/invalid-response receipts; no exhaustive crash-injection experiment.

## reply_authority_partition
Type: protocol_error_partition. Domain: bounded stdout, Claude JSON envelope and strict AnnotationDraft. Partition failures into CLI exit/error/nonresult, invalid result JSON/schema, task binding, native AST/source/coverage, canonical pragmatic labels and retained semantic/context uncertainty. Only the first structural classes reject transport validation; unresolved observations can remain on a validated review-only receipt. Counterexample to promoting receipt status to gold: a well-formed reply can introduce character voice without a known speaker; draft-native evaluation preserves the new finding. Evidence: fake envelope/source/target/label/unknown-field/context controls; no semantic completeness guarantee.

## source_binding
Type: observation_alignment_exclusion. Domain: draft target rows and bound task context. Necessary conditions are exact target sets, matching sentence id/text and concatenate(token forms)=remove_whitespace(source text). Rehashing a changed token cannot make its characters equal to the source. Token-boundary uniqueness, attachment correctness and full treeagree equivalence are outside this condition. Evidence: wrong-token and missing-target controls together with the reused native parser/registry checks.


## serialization_preimage
Type: typed_vs_raw_binding. Domain: bounded task-file bytes b parsed as typed Task t. Define H_file=SHA256(b) and H_task=SHA256(compact_typed_JSON(t)). Both are retained in CLI reservation and receipt. The parsing preimage of t includes differently formatted b, so H_file need not equal H_task. Queue raw-file identity must compare H_file; reply identity must compare H_task. Direct in-memory library run has no observed b and explicitly records a null file hash. Counterexample: pretty JSON and compact typed JSON represent the same t but differ in bytes and file hash. Evidence: a focused fake-CLI dispatch test pins both values and their distinction; no hash-collision theorem is claimed.

## structured_reply_and_closure
Domain: bounded Claude envelopes and owned child-process observations. Parse object-valued structured_output when present; otherwise require result to be a strict JSON string. A malformed present structured field has no compatibility fallback. Fences/prose are not stripped. Native task/source/target/label/UTF8-byte validation follows both branches and rejected raw bytes remain retained. First classify outer-envelope failure as transport_error; only type=result and is_error=false proceeds to inner invalid_response. Nonzero CLI exit is child_failed. Provider/quota/auth/nonresult/malformed envelopes must stop even if locally reaped.

Closure predicates are separate: started, reaped, group_absent. Absence is true only after reaping and kill(-owned_pgid,0)=>ESRCH. Exit alone does not establish it. A live group, EPERM or unknown probe stays false. Counterexample: a live independently grouped fake child fails the absence probe until killed/reaped. This observes one local group; no remote accounting or intentionally regrouped-descendant theorem. Explicit raw profile_file_sha256 and typed profile_sha256 join the task pins.

A fully bound invalid_response with affirmative reaped/group_absent is a closed rejected observation, suitable only for quarantine and a distinct next task. It cannot become accepted annotation or authorize replay. Missing proof remains uncertain. Evidence: live-group negative control, closed malformed receipt, strict structured/fenced controls, profile raw/typed regression and provider-error negative partition; no exhaustive OS/crash/semantic theorem.

## family_admission_2026_11_04

Execution admits Sonnet and Terra-family CLI profiles; a Terra-family profile with `gpt-6.1-sol` is accepted by the shared `teacher::validate_model`; a Haiku profile is refused before any reservation directory exists (negative control in the Terra transport test). A profile file with no family now deserializes as Sonnet and still needs an absolute executable.

## provider_busy_2026_11_04

## transient_reconnect_2026_11_05

## tests

Terra control: a fake Codex executable receives exactly `teacher::exec_args` and the prompt on
stdin; a valid JSONL turn validates review-only; a command_execution item before the answer gives
`transport_error`; a `sonnet` model under Terra is refused without creating the attempt directory.

## History

### 2026-11-02 · document

#### Terra transport (2026-11-02)

Family Sonnet or Terra with the CLI adapter is admitted; any other family/adapter is refused
before reservation. Terra requires a `gpt-` model (shared `teacher::validate_model`), so a Claude
model name under Terra is refused pre-launch: no provider fallback. The rendered native request
(prompt, budget, argv record) is unchanged, so implementation bindings and prepared Tasks stay
valid; for Terra the actual transport argv is `teacher::exec_args(model, "medium")` (stdin prompt,
read-only sandbox, tools/plugins disabled, ChatGPT login, JSONL events) and is recorded as
`transport_argv` in the reservation. Codex events are decoded by `teacher::decode_events`; an
error, tool use, repeated or late item fails as `transport_error`; the final agent message must be
the same strict JSON draft as for Sonnet. Provider credential variables of both families are
cleared. Throughput is not modeled here: first live Terra call 2026-11-02 answered eight targets
in 40.6 s and failed native validation (UD registry), i.e. transport worked, the draft did not.

### 2026-11-02 · tests

2026-11-02: 14 library tests plus the preserved legacy guard test pass offline/locked with two jobs. New fake-CLI checks cover exact stdin and safe/medium argv, strict schema, stale task hash, changed source, missing target, canonical labels, new reply-context findings, unresolved retention, declared-evaluation/deadline/oversize nonlaunch, Unknown review with not_teaching retained, durable malformed/timeout outcomes and replay refusal. Exact100000-byte and multibyte-overflow tests remain green. No live model, training, DB or corpus assessment was invoked. These are finite checks, not semantic accuracy, power-loss durability or exactly-once proofs.

2026-11-02 protocol followup:17 library tests plus the legacy guard pass offline/locked j2. Bound Claude JSON Schema and exact schema-plus-stdin100000-byte budget, strict structured/result envelope branches, UTF8-byte/cardinality controls, raw/typed profile pins, reaped plus actual ESRCH process-group observation, and provider/error-envelope partition are checked. Known-closed inner invalid_response is distinct from transport_error/child_failed; no replay, semantic/gold/training promotion or historical pilot artifact mutation. Fake executables only; deployment pilots separate.

### 2026-11-04 · provider_busy_2026_11_04

Type error_partition. Domain: Terra family only, after the Codex child was reaped with a nonzero exit. If teacher::provider_busy accepts the bounded raw.stdout (only lifecycle headers plus error/turn.failed events whose messages are all exact Codex capacity texts, no item, no completion), the receipt status is provider_busy with provider_busy_message, the attempt still fails (nonzero executor exit) and remains a non-replayed directory; the supervisor decides requeue. Otherwise the status remains child_failed. Complement: Sonnet output, a usage-limit message, a capacity error after any item, timeouts and output-limit terminations never become provider_busy. Observed trigger: Fish r2 pager t01, 2026-11-04 15:38, "Selected model is at capacity. Please try a different model." then turn.failed, exit 1, empty stderr. Evidence: one fake-CLI test with busy, usage-limit and partial-output cases and a Sonnet control.

### 2026-11-05 · transient_reconnect_2026_11_05

Type error_partition. Terra family only. A Codex JSONL whose only error events are exact stream-retry
notices ("Reconnecting... N/M (cause)", teacher::transient_reconnect) before the single
turn.completed with a final agent message decodes normally, so the draft is validated as usual
(validated_review_only or invalid_response). Any other error, a turn.failed, a missing completion
or a notice after completion keeps status transport_error, as before. Every Terra receipt now has
transient_reconnects, the number of such notices in the bounded raw.stdout, on every terminal path
(null for Sonnet and before the child output exists). It is observability only; it neither decides
success nor authorizes replay. Observed trigger: Fish r3 pager t04, 2026-11-05 03:32, one notice
then the answer and turn.completed, receipt transport_error, which stopped all eight pagers.

### 2026-11-05 · tests

2026-11-05 transient reconnect: terra_stream_reconnect_before_a_completed_turn_is_not_a_failure
runs the reduced r3 sequence (notice, synthetic draft, completion) through a fake Codex executable:
validated_review_only with transient_reconnects 1. Red controls with the same notice: turn.failed
instead of completion and no completion give transport_error with count 1; a usage-limit error in
its place gives transport_error with count 0; none writes reply.json. 22 library tests pass;
fish-swarm (15), fish-pages (9), prag and enrich test suites pass in release.
