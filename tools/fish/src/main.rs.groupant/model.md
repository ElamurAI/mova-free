# annotation-review-guard

## descriptant

### Annotation audit boundary

The controller reserves immutable source work, runs small isolated batches and retains typed outcomes. The guard bounds teacher prompts. It does not promote annotations or train from assessment labels.

## model

# Partial mathematical model

## forward

In the legacy non-controller branch, a valid UTF-8 prompt with byte length B <= 25000 is forwarded with the exact argument vector to the configured Codex transport. Invalid UTF-8 or excessive bytes return failure before child launch.

For input state x, execution returns an observable outcome y or an explicit error. The relation F is partial, not a total semantic specification.

## behavior_complement

Given a declared boundary universe U, model forbidden observable pairs as B = U minus F. For the guard, invalid UTF-8 and B > 25000 have no child-launch successor. For the controller, held-out IDs have no teacher-input successor; receipt-covered jobs have no new-submission successor under the same campaign identity; uncertain reservations have no automatic retry successor. This is a source-derived relation, not an exhaustive proof of operating-system traces.

## reverse_reconstruction

R(y) = {x : (x,y) belongs to F}. Receipts and hashes restrict compatible queue slices. Multiple child schedules, parser states and model computations can share the same receipt, so reconstruction is relational. Hash binding assumes collision resistance and a trusted local filesystem.

## invariant

Accepted IDs belong to the expected set and preserve sentence text. Excluded IDs and missing outputs cannot be accepted. Reservation precedes launch; receipt precedes cursor advance. Audit artifacts stay separate from canonical data. Prompt input is valid UTF-8 within the byte cap. These are scoped intended invariants, with finite checks and external gates.

## cost

O(min(B,25001)) input bytes and memory plus one subprocess lifetime. It bounds input bytes, not inference cost or output tokens. This cost describes the legacy guard; annotation preparation delegates bounded parser/agreement/rendering cost to annotation.rs.

## composition

The annotation-prepare branch delegates to annotation.rs for native syntax/pragmatic evaluation and separate prepared review/admission artifacts, with no model call. The compatible legacy branch is Controller -> cycle -> GPU/open witnesses -> tree/pragmatic agreers -> annot -> shared teacher -> guard -> Codex. Leaf checks do not establish end-to-end correctness. Engine version, offline models, exact paths, result gates and asynchronous process lifetime remain composition assumptions.

## uncertain_reservation

A reservation without receipt admits never-started and partly-completed antecedents. It cannot justify another paid submission. Manual resolution is required; absence of receipt is not evidence of absence of a remote call.

## annotation_specialization
Current dispatch recognizes annotation-contract metadata and annotation-prepare with exact arity before the unchanged legacy controller/25000-byte guard. Annotation preparation delegates to the new native syntax/pragmatic contract; it never launches a transport. The generic Sonnet default applies only to the new path; explicit legacy Codex/Terra configuration remains unchanged. Unknown arity for reserved annotation commands returns failure, not legacy paid-transport fallback. Leaf correctness does not establish downstream training authorization.

## prepared_model_binding
For each new profile, the actual model string occurs in prepared executable argv and the exact bounded stdin/terminal text is retained. Public default Sonnet/CLI (and explicit Haiku) maps to Claude model/print arguments; explicit Terra maps to Codex model/exec or interactive medium-effort arguments. No model call or automatic fallback occurs. Counterexample to an unused model label: changing family/model changes the prepared argv, covered by deterministic profile fixtures. The command only prepares data; execution is caller-owned.

## admission_separation
Teacher-request output is independent of training eligibility. Annotation findings and unknowns remain visible; no-finding teaching inputs can create only an unauthorized admission candidate. Existing controller config/status/run modes and the legacy prompt guard retain their previous behavior. Source-derived model with finite local fixtures; no runtime migration or semantic approval is inferred.

## review_execution_dispatch
Domain: reserved command annotation-execute with task/profile/output/deadline and optional timeout arguments. Observation: delegated review-only receipt or nonzero error. Dispatch selects execute::dispatch before the legacy controller and raw Codex guard. Wrong arity is rejected inside that reserved branch and cannot fall through to the legacy paid transport. The exported annotation library is shared by preparation and execution; no second native parser or pragmatic schema is introduced.

This changes the earlier preparation-only composition: annotation-prepare still performs no model call, while annotation-execute is a separately selected Sonnet-only reviewed transport. Execution delegates source-bound evaluate/render, explicit safe CLI arguments, durable reservation and strict reply validation. The previous 25000-byte raw guard, legacy controller configuration, teacher policy and Codex transport are unchanged. Counterexample to blanket transport equivalence: a malformed annotation-execute invocation fails rather than forwarding its arguments to Codex. Evidence: focused crate and legacy guard checks; no live provider call or corpus assessment.

## tests

### Finite checks

Run cargo test --offline --locked with this nested manifest, then cargo build --offline --locked --release. A fake guard executable checks exact input/arguments and red on excessive or invalid UTF-8 input. Controller fixtures cover receipt skipping, STOP/resume, malformed output IDs and changed campaign config. External run artifacts retain exact commands/outcomes. No universal proof or inference-quality claim. Real results remain outside the repository.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: a controller lock alone cannot establish exactly-once paid inference because annot launches a detached worker. Conservative uncertainty is retained. Structural validity does not establish semantic correctness.

### 2026-11-01 · descriptant

2026-11-01: annotation-contract/annotation-prepare add a dedicated native syntax/pragmatic path. Public new requests default Haiku, while legacy explicit Codex/Terra deployment and its25000-byte guard remain intact. Request preparation does not execute a model or train.

### 2026-11-01 · tests

2026-11-01: seven local tests pass, including new native annotation, model/argv profile, exact100000-byte rendered budget and separate review/admission artifact fixtures. Legacy guard remains covered. No teacher/model/production-runtime call.

### 2026-11-02 · tests

2026-11-02: 14 library tests plus the preserved legacy guard test pass offline/locked with two jobs. New fake-CLI checks cover exact stdin and safe/medium argv, strict schema, stale task hash, changed source, missing target, canonical labels, new reply-context findings, unresolved retention, declared-evaluation/deadline/oversize nonlaunch, Unknown review with not_teaching retained, durable malformed/timeout outcomes and replay refusal. Exact100000-byte and multibyte-overflow tests remain green. No live model, training, DB or corpus assessment was invoked. These are finite checks, not semantic accuracy, power-loss durability or exactly-once proofs.

2026-11-02 protocol followup:17 library tests plus the legacy guard pass offline/locked j2. Bound Claude JSON Schema and exact schema-plus-stdin100000-byte budget, strict structured/result envelope branches, UTF8-byte/cardinality controls, raw/typed profile pins, reaped plus actual ESRCH process-group observation, and provider/error-envelope partition are checked. Known-closed inner invalid_response is distinct from transport_error/child_failed; no replay, semantic/gold/training promotion or historical pilot artifact mutation. Fake executables only; deployment pilots separate.

### 2026-11-04 · descriptant

2026-11-04: annotation-contract reports default_profile sonnet (operator model policy: Fish default Sonnet, Terra recommended). Only the metadata string changed; dispatch is unchanged.
