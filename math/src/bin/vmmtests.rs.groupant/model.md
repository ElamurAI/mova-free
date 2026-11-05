# math-vmmtests-cli

## descriptant

### Source-bound description

Require ask/jobs, parse first options and exactly3-column TSV, initialize Vmm, spawn clamp(par,1,2). Nonempty output skips job. Call failure moves next far forward; already claimed jobs may finish. Output write failure increments failures without stopping future claims.

## approximant

### Forward relation

Require ask/jobs, parse first options and exactly3-column TSV, initialize Vmm, spawn clamp(par,1,2). Nonempty output skips job. Call failure moves next far forward; already claimed jobs may finish. Output write failure increments failures without stopping future claims.

Domain: Only synthetic option vectors tested; paths relative to TSV parent, skip policy byte length.

Executable Rust approximant unsupported: categorical/effect observations have no useful percentage metric yet. Future caps1/2/5/10% only; no automatic substitution.

## invariant

### Invariant and cost

Missing trailing option value excludes extraction. Failure does not cancel already running calls; nonempty output excludes only that job's teacher call.

Ordering: Args→jobs→prior log→workers; prompt read→ask→write.

Cost: O(TSV+jobs) storage,<=2 workers; external claimed-call time/cost delegated, no atomic budget reservation.

## composition

### Composition and epoch memory

Workers share Vmm, whose mutex precheck is not reservation. Atomic stop index does not enforce total spend.

Pure opt helper only; no jobs or teacher transport.

Partial syntax and finite tests do not establish all caller assumptions.

## models

## behavior_complement

Missing trailing option value excludes extraction. Failure does not cancel already running calls; nonempty output excludes only that job's teacher call.

## reverse_reconstruction

Skipped job admits any metadata-visible nonempty output regardless validity. Failure count admits many schedules and errors.

## failure_precedence

Args→jobs→prior log→workers; prompt read→ask→write.

## representation_transport

Workers share Vmm, whose mutex precheck is not reservation. Atomic stop index does not enforce total spend.

## resource_boundary

O(TSV+jobs) storage,<=2 workers; external claimed-call time/cost delegated, no atomic budget reservation.

## domain_boundary

Only synthetic option vectors tested; paths relative to TSV parent, skip policy byte length.

## tests

### Evidence

Selected actual-source checks: `cargo test -p math --bin vmmtests wave28`. Exact commands, numeric statuses, normalized outcomes, compiled assertion controls and restoration receipts are in the external wave28 mathcontrol run.

Pure opt helper only; no jobs or teacher transport.

No live teacher/API, trained model, corpus/held-out, GPU, DB or expensive benchmark run. Parent syntax generated twice identically. Scope remains partial.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: prospective model before bounded synthetic tests; production behavior unchanged. Pure opt helper only; no jobs or teacher transport.

### 2026-10-31 · tests

Heading `### Evidence, 2026-10-31` stays in tests as `### Evidence`.
