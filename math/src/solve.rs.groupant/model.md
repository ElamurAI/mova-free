# math-solver-orchestration

## descriptant

### Source-bound description

Batch/workers clamp>=1, sort final records by id. A parses last answer/id. B retries up to max_retries+1, shares costs/tokens, correct=verified&&grade. Memos retain verified B regardless correctness. C trains10epochs, tries top3 same-method/template/count/different-id skeleton matches, gates then accepts else copies B fields with by_slm=false.

## approximant

### Forward relation

Batch/workers clamp>=1, sort final records by id. A parses last answer/id. B retries up to max_retries+1, shares costs/tokens, correct=verified&&grade. Memos retain verified B regardless correctness. C trains10epochs, tries top3 same-method/template/count/different-id skeleton matches, gates then accepts else copies B fields with by_slm=false.

Domain: Synthetic parsers/batching/fallback only. Sort does not determine append order; duplicate ids collapse maps.

Executable Rust approximant unsupported: categorical/effect observations have no useful percentage metric yet. Future caps1/2/5/10% only; no automatic substitution.

## invariant

### Invariant and cost

Unverified B excludes correct B. Empty memory excludes template success, but fallback may verified=true/by_slm=false. Append failures do not exclude returned records because ignored.

Ordering: Ask→parse→gate→retry. Memo config/verified before id lookup. C chooses unique numeric option via preliminary ungated-options run then reruns gate.

Cost: Teacher retries+plan costs; C trains then scores memory and<=3templates/problem. O(records+memory+plans); unchecked token/counter addition.

## composition

### Composition and epoch memory

curric→Expect→plan→Record→Memo→C. CLI must enforce held exclusion, run_c does not.

Teacher/Lean loops and append-failure path unexecuted; no accuracy claim.

Partial syntax and finite tests do not establish all caller assumptions.

## models

## behavior_complement

Unverified B excludes correct B. Empty memory excludes template success, but fallback may verified=true/by_slm=false. Append failures do not exclude returned records because ignored.

## reverse_reconstruction

Summaries admit permutations/many assignments. Integer token sharing loses remainder. C fallback omits B trace/history, so cannot reconstruct B record. Memo requires matching verified B but not correct.

## failure_precedence

Ask→parse→gate→retry. Memo config/verified before id lookup. C chooses unique numeric option via preliminary ungated-options run then reruns gate.

## representation_transport

curric→Expect→plan→Record→Memo→C. CLI must enforce held exclusion, run_c does not.

## resource_boundary

Teacher retries+plan costs; C trains then scores memory and<=3templates/problem. O(records+memory+plans); unchecked token/counter addition.

## domain_boundary

Synthetic parsers/batching/fallback only. Sort does not determine append order; duplicate ids collapse maps.

## tests

### Evidence

Selected actual-source checks: `cargo test -p math --lib solve::wave28`. Exact commands, numeric statuses, normalized outcomes, compiled assertion controls and restoration receipts are in the external wave28 mathcontrol run.

Teacher/Lean loops and append-failure path unexecuted; no accuracy claim.

No live teacher/API, trained model, corpus/held-out, GPU, DB or expensive benchmark run. Parent syntax generated twice identically. Scope remains partial.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: prospective model before bounded synthetic tests; production behavior unchanged. Teacher/Lean loops and append-failure path unexecuted; no accuracy claim.

### 2026-10-31 · tests

Heading `### Evidence, 2026-10-31` stays in tests as `### Evidence`.
