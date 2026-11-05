# Prepared queue, Fish executor and priority observations

## domain
Composition of public CLI and library with the separately versioned annotation-execute protocol. Runtime config, queue, provenance and deployment policy are external. Native Task.id is mapped to receipt.task_id. Source-binding covers two local Rust files; executor semantics remain an explicit interface assumption rather than a silently included proof.

## forward
CLI run obtains one leased library and repeats bounded steps. Library reserves, passes copied Task/Profile and deadline to executor, then joins exit and receipt identity into review-only eligibility. CLI stops on uncertain/failed and polls exhausted/paused states. A priority observer may consume fresh paused status, but must independently verify its expected identity and quiescence policy. This is a composition of operational observations, not recursive semantic self-improvement evidence.

## invariant
No library reservation implies no permitted executor start through this composition. No valid completed receipt implies no transport eligibility. Neither eligibility nor observer freshness implies semantic correctness, database permission or training authority. Deadline/PAUSE prevent new work at checked boundaries while an accepted bounded call can finish.

## cost
O(n+r+B) metadata step plus native executor and observer latency. File leaves alone do not establish the join: status labels, task identity mapping and deadline propagation must agree across components. No aggregate speed or scientific accuracy bound is claimed.

## behavior-complement
Type behavior_complement. Composition excludes a status query launching annotation, a per-job success ending the campaign, or a shape-valid draft being promoted to trusted training data. Counterexample: an apparently accepted AST/pragmatics shape can still contain a semantic error. Finite controls establish protocol gates only.

## reverse-reconstruction
Type reverse_reconstruction. Outer terminal plus inner receipt reconstruct a set of compatible pinned requests and transport histories. The set remains many-to-one over model reasoning, semantic alternatives and nondeterministic timing. A missing outer terminal may coexist with a completed inner artifact; it therefore preserves uncertainty rather than replay.

## interface-identity-join
Type compatibility_boundary. Define E as receipt.task_id=Task.id AND receipt.task_file_sha256=SHA256(raw copied task) AND wellformed receipt.task_sha256. Native typed serialization has its separate task_sha256; the generic supervisor does not recompute it. E is necessary but insufficient for semantic truth. Counterexamples found before deployment: Task.task_id does not exist, and a pretty task file has a different digest from compact typed serialization. Corrected field joins are exercised with distinct raw/typed positive commitments, wrong raw hash and malformed typed digest.

## quiescence-join
Type independent_evidence_join. PAUSE marker plus fresh expected-identity paused status plus active_model_call=false plus no_unresolved_reservations=true supplies a scoped job-boundary observation. STOP/deadline snapshots require independent terminal-liveness policy because they no longer heartbeat. Counterexample: STOP plus orphan reservation is uncertain, never a no-call terminal. Priority integration and real process liveness remain external checks.

## checks
Thirteen bounded fake-executor checks and deterministic source syntax projections. Public fragment is supplied for explicit attachment under the main orchestration tree; missing central attachment remains a documented integration gap. No model, GPU, full-corpus run, database import or training.

## epoch-memory
The important new result is uncertainty precedence, not a count of typed models. Preserve task/receipt naming, terminal versus per-job states and independent semantic review.

## process-liveness-join
Type process_liveness_exclusion. An exited or killed outer executor is not a proof that a separately grouped annotation CLI died. The composition admits quiescent terminal history for validated_review_only, not_started_boundary, spawn_failed, and separately audited known-closed quarantined_review_only. Failed/missing/invalid success receipts remain uncertain; a STOP or expired campaign cannot erase that history. The fake separate-group child survives the outer kill and is cleaned up explicitly by the test. This counterexample invalidates the former timeout-to-quiescent inference; no live model is used.

## quarantine-progress-join
Type progress_without_replay. Composition distinguishes native invalid_response, explicit closed-model-process evidence, and outer quarantined_review_only. The progress relation is a move from one durable quarantined job to a different fresh queued job, conditioned on revalidated raw task/profile, receipt and config/job commitments. This is progress in review scheduling, not improvement in semantic annotations. completed_quarantined is per-job transient and cannot stand in for campaign-terminal quiescence. False/null closure observations keep the whole composition uncertain; no deadline or STOP erases them. The finite two-task fixture completes exactly two executor calls across quarantine, success, exhaustion and restart, while drift/closure negative fixtures refuse another call.

## paging-audit-join
The additive public read-only history audit exposes the existing raw task/profile and native closure predicates to a bounded page parent. A parent may aggregate exact target cardinalities and receipt pins only after the no-unresolved audit succeeds. It does not acquire the child lease or authorize a launch; the execution step and observer lease check remain distinct effects. This group does not own the paging controller. Eight parent fake-Rust fixtures validate selected joins; neither counts nor an empty queue establish complete semantic review.
