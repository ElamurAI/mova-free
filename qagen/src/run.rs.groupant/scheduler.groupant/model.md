# qagen-run-scheduler

## forward
Build todo, read existing call count, reserve calls, dispatch workers, persist raw/accounting, check format and stop outer loops on errors.

## invariant
Only first-stage format rejection requests retry; second-stage rejection never requests a third call. Transport errors exit current batch.

## behavior-complement
Universe: finite supplied values and filesystem observations, return values, errors and writes in this scope. No scheduler transport seam or live execution added. Stop flag cannot cancel already in-flight calls or prevent every retry after another worker fails.

## reverse-reconstruction
F^-1(y) contains all compatible inputs and initial states yielding y. Identical durable files/logs admit many worker schedules. Missing accounting cannot distinguish never-called from interrupted persistence. No complete inverse solver or unique recovery claim.

## cost
O(B*W) selection plus resume/read_calls; J threads and worker-sized prompts/text. Mutex serializes append and check while its guard remains live.

## composition
Reservation before ask, rename before append, append before check. Error aggregation is schedule-dependent; there is no transaction.

## reservation
Reservation counts attempts including failures; prior count >= cap prevents ask.

## retry
stage=1 and format.is_some() ->2; all other successful checks ->0.

## failure-stop
Global stop is checked at outer loop, not each inner retry.

## durability-gap
Rename-success/append-failure leaves raw without accounting. No fsync claim.

## checks
Finite synthetic Rust tests in owned sources; broad offline qagen lib suite twice per profile, explicitly skipping lemma::tests::doubt_with_en_model. Compiled production mutations reach named assertion failures. Exact outcomes and hashes are in the file groupant ../model.md#evidence.

## epoch-memory

Prospective model saved before repairs. Old-code witnesses demonstrated token overflow, JSON null from float overflow, premature report replacement, and masked metadata errors. Ordinary output keys, finite negative history and legacy Option lookup semantics are preserved. Scheduler was not redesigned.

## limitations
Partial source models and finite observations, not proof of all concurrent/filesystem states. Syntax projection is partial; macros opaque. Allocation failure, races and crashes may prevent completion. No live teacher/model/corpus/database was used. No useful certified metric for a numerical surrogate of these error-sensitive operations was identified; executable approximant is absent, with no invented error budget or speed claim.

## History

### 2026-10-30 · epoch-memory

2026-10-30 prospective model saved before repairs. Old-code witnesses demonstrated token overflow, JSON null from float overflow, premature report replacement, and masked metadata errors. Ordinary output keys, finite negative history and legacy Option lookup semantics are preserved. Scheduler was not redesigned.
