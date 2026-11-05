# qagen-report-collection

## forward
Project supplied Lemmas into doubtful implicit marks; collect validates accounting, gathers outputs, optionally loads lemmas, renders distributions, serializes params and writes artifacts.

## invariant
All preparation errors precede artifact writes. Statistics share the same Collected object and batch-local coordinates.

## behavior-complement
Universe: finite supplied values and filesystem observations, return values, errors and writes in this scope. No locking, atomic publication or fsync. Filesystem races may mix observations from different instants.

## reverse-reconstruction
F^-1(y) contains all compatible inputs and initial states yielding y. QA retains accepted text/anchor IDs but loses raw formatting; aggregate params do not identify unique paragraphs, call lists or lemma model. No complete inverse solver or unique recovery claim.

## cost
O(paragraphs, pairs, rejection text and render bytes) with map log factors, gate/calls/model costs added; buffers proportional to artifact bytes.

## composition
Retries count selected batch retry targets; calls count all log rows. V1 bypasses lemma load; V2 adds doubts/span counters. Three writes can fail independently.

## preflight
Invalid rule, retry IO, calls parse/overflow, raw read, model or serialization errors all precede writes.

## distribution
Accepted-pair histogram excludes selected paragraphs with zero accepted pairs.

## span-accounting
Explicit offered=accepted explicit+pair rejections with explicit: prefix; percentages zero for denominator zero.

## artifact-order
After params serialization: qa.tsv, rejects.tsv, params.json. No rollback after write failure.

## checks
Finite synthetic Rust tests in owned sources; broad offline qagen lib suite twice per profile, explicitly skipping lemma::tests::doubt_with_en_model. Compiled production mutations reach named assertion failures. Exact outcomes and hashes are in the file groupant ../model.md#evidence.

## epoch-memory

Prospective model saved before repairs. Old-code witnesses demonstrated token overflow, JSON null from float overflow, premature report replacement, and masked metadata errors. Ordinary output keys, finite negative history and legacy Option lookup semantics are preserved. Scheduler was not redesigned.

## limitations
Partial source models and finite observations, not proof of all concurrent/filesystem states. Syntax projection is partial; macros opaque. Allocation failure, races and crashes may prevent completion. No live teacher/model/corpus/database was used. No useful certified metric for a numerical surrogate of these error-sensitive operations was identified; executable approximant is absent, with no invented error budget or speed claim.

## History

### 2026-10-30 · epoch-memory

2026-10-30 prospective model saved before repairs. Old-code witnesses demonstrated token overflow, JSON null from float overflow, premature report replacement, and masked metadata errors. Ordinary output keys, finite negative history and legacy Option lookup semantics are preserved. Scheduler was not redesigned.
