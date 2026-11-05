# math-solver-orchestration-batching

## descriptant

### Source-bound description

Cost/time divided by max(n,1), token integer division. Append under local Mutex. Chunks max(batch,1), workers max(par,1), atomic claims, sort records by id. parse_answers keeps last JSON answer per id, skips null.

## approximant

### Forward relation

Cost/time divided by max(n,1), token integer division. Append under local Mutex. Chunks max(batch,1), workers max(par,1), atomic claims, sort records by id. parse_answers keeps last JSON answer per id, skips null.

Domain: Finite batches; token sum/counters unchecked; callback/poison panic possible.

Executable Rust approximant unsupported: categorical/effect observations have no useful percentage metric yet. Future caps1/2/5/10% only; no automatic substitution.

## invariant

### Invariant and cost

Empty problems excludes callback work. Null answer excluded from parsed map insertion, but missing id may become empty key.

Ordering: Claim chunk→callback→append output vector; sort only after threads join. parse answer string/null/JSON representation.

Cost: O(problems+records log records)+callback work and JSON serialization; worker count not upper-clamped.

## composition

### Composition and epoch memory

run_a/run_b callback append order independent of final returned sorting; per-Run lock not cross-process protection.

Source scope bytes[2107,3906). Teacher/Lean loops and append-failure path unexecuted; no accuracy claim. Not every branch in this fragment executed.

Partial syntax and finite tests do not establish all caller assumptions.

## models

## behavior_complement

Empty problems excludes callback work. Null answer excluded from parsed map insertion, but missing id may become empty key.

## reverse_reconstruction

Shared tokens lose remainder, sorted records lose worker schedule. Last-answer map admits arbitrary earlier duplicate values.

## failure_precedence

Claim chunk→callback→append output vector; sort only after threads join. parse answer string/null/JSON representation.

## representation_transport

run_a/run_b callback append order independent of final returned sorting; per-Run lock not cross-process protection.

## resource_boundary

O(problems+records log records)+callback work and JSON serialization; worker count not upper-clamped.

## domain_boundary

Finite batches; token sum/counters unchecked; callback/poison panic possible.

## tests

### Evidence

Selected actual-source checks: `cargo test -p math --lib solve::wave28`. Exact commands, numeric statuses, normalized outcomes, compiled assertion controls and restoration receipts are in the external wave28 mathcontrol run.

Source scope bytes[2107,3906). Teacher/Lean loops and append-failure path unexecuted; no accuracy claim. Not every branch in this fragment executed.

No live teacher/API, trained model, corpus/held-out, GPU, DB or expensive benchmark run. Parent syntax generated twice identically. Scope remains partial.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: prospective model before bounded synthetic tests; production behavior unchanged. Source scope bytes[2107,3906). Teacher/Lean loops and append-failure path unexecuted; no accuracy claim. Not every branch in this fragment executed.

### 2026-10-31 · tests

Heading `### Evidence, 2026-10-31` stays in tests as `### Evidence`.
