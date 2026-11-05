# qagen-run-batching

## forward
batches partitions Para references by key; Tales exposes ascending keys, total sentence count, prompt and gate summary.

## invariant
Every input reference appears once; check excludes note-level entries from rejection count.

## behavior-complement
Universe: finite supplied values and filesystem observations, return values, errors and writes in this scope. No paragraph deduplication or within-batch sorting. Unknown direct Tales batch index can panic.

## reverse-reconstruction
F^-1(y) contains all compatible inputs and initial states yielding y. Within-key order survives, cross-key input interleavings are lost. Summary counts lose individual pair/rejection identities. No complete inverse solver or unique recovery claim.

## cost
O(P log B) grouping and O(P+B) storage; prompt and check delegate text costs.

## composition
prompt, size and check share all[b]; gate local coordinates must index the same ordered paragraph list.

## grouping
Partition conserves total reference count.

## coordinate
Gate paragraph p indexes ordered batch position p-1.

## summary
Format rejection is separate from count excluding Level::Note.

## trait-domain
Job: Sync permits scoped threads but arbitrary implementations can panic or return duplicate IDs.

## checks
Finite synthetic Rust tests in owned sources; broad offline qagen lib suite twice per profile, explicitly skipping lemma::tests::doubt_with_en_model. Compiled production mutations reach named assertion failures. Exact outcomes and hashes are in the file groupant ../model.md#evidence.

## epoch-memory

Prospective model saved before repairs. Old-code witnesses demonstrated token overflow, JSON null from float overflow, premature report replacement, and masked metadata errors. Ordinary output keys, finite negative history and legacy Option lookup semantics are preserved. Scheduler was not redesigned.

## limitations
Partial source models and finite observations, not proof of all concurrent/filesystem states. Syntax projection is partial; macros opaque. Allocation failure, races and crashes may prevent completion. No live teacher/model/corpus/database was used. No useful certified metric for a numerical surrogate of these error-sensitive operations was identified; executable approximant is absent, with no invented error budget or speed claim.

## History

### 2026-10-30 · epoch-memory

2026-10-30 prospective model saved before repairs. Old-code witnesses demonstrated token overflow, JSON null from float overflow, premature report replacement, and masked metadata errors. Ordinary output keys, finite negative history and legacy Option lookup semantics are preserved. Scheduler was not redesigned.
