# annotation-review-controller

## descriptant

### Annotation audit boundary

The controller reserves immutable source work, runs small isolated batches and retains typed outcomes. The guard bounds teacher prompts. It does not promote annotations or train from assessment labels.

## model

# Partial mathematical model

## forward

An immutable snapshot and queue are joined by IDs and source hashes; batches of at most eight queue entries are reserved before one isolated cycle. Accepted, refused, missing and uncertain results are distinct. Durable receipts precede advancing the logical cursor; prior receipts are skipped only under the pinned campaign identity.

For input state x, execution returns an observable outcome y or an explicit error. The relation F is partial, not a total semantic specification.

## behavior_complement

Given a declared boundary universe U, model forbidden observable pairs as B = U minus F. For the guard, invalid UTF-8 and B > 25000 have no child-launch successor. For the controller, held-out IDs have no teacher-input successor; receipt-covered jobs have no new-submission successor under the same campaign identity; uncertain reservations have no automatic retry successor. This is a source-derived relation, not an exhaustive proof of operating-system traces.

## reverse_reconstruction

R(y) = {x : (x,y) belongs to F}. Receipts and hashes restrict compatible queue slices. Multiple child schedules, parser states and model computations can share the same receipt, so reconstruction is relational. Hash binding assumes collision resistance and a trusted local filesystem.

## invariant

Accepted IDs belong to the expected set and preserve sentence text. Excluded IDs and missing outputs cannot be accepted. Reservation precedes launch; receipt precedes cursor advance. Audit artifacts stay separate from canonical data. Prompt input is valid UTF-8 within the byte cap. These are scoped intended invariants, with finite checks and external gates.

## cost

Startup O(source bytes + queue bytes), source/index retained in memory. Per job includes bounded context, GPU witnesses and one small teacher batch; campaign disk traversal adds O(total files) per job. No throughput improvement claimed.

## composition

Controller -> cycle -> GPU/open witnesses -> tree/pragmatic agreers -> annot -> shared teacher -> guard -> Codex. Leaf checks do not establish end-to-end correctness. Engine version, offline models, exact paths, result gates and asynchronous process lifetime remain composition assumptions.

## uncertain_reservation

A reservation without receipt admits never-started and partly-completed antecedents. It cannot justify another paid submission. Manual resolution is required; absence of receipt is not evidence of absence of a remote call.

## configured_model

## tests

### Finite checks

Run cargo test --offline --locked with this nested manifest, then cargo build --offline --locked --release. A fake guard executable checks exact input/arguments and red on excessive or invalid UTF-8 input. Controller fixtures cover receipt skipping, STOP/resume, malformed output IDs and changed campaign config. External run artifacts retain exact commands/outcomes. No universal proof or inference-quality claim. Real results remain outside the repository.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: a controller lock alone cannot establish exactly-once paid inference because annot launches a detached worker. Conservative uncertainty is retained. Structural validity does not establish semantic correctness.

### 2026-11-04 · configured_model

Config.model (serde default `gpt-6.1-terra`) is validated by `teacher::validate_model` at load and exported as CYCLE_MODEL and MOVA_TEACHER_MODEL; Claude ids (`sonnet`, `opus`) and whitespace-bearing ids are refused before any lease or launch. Existing configs without the field keep Terra behavior byte-for-byte in the child environment. Checked by `config_model_defaults_to_terra_and_refuses_claude_ids` (2026-11-04).
