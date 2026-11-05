# qagen-run-lookup

## forward
Format minimum-width-three paths; strict final lookup probes retry then raw, while need probes raw then retry and may gate raw text.

## invariant
Strict metadata errors propagate with path context. Dangling links count as missing; directories exist but raw read subsequently rejects them.

## behavior-complement
Universe: finite supplied values and filesystem observations, return values, errors and writes in this scope. ELOOP and permission errors are not absence in strict functions. Compatibility final_raw deliberately still masks errors.

## reverse-reconstruction
F^-1(y) contains all compatible inputs and initial states yielding y. None admits absent targets and dangling links; selected retry leaves raw unconstrained. need=0 admits any retry contents or valid original. No complete inverse solver or unique recovery claim.

## cost
Up to two metadata probes and path allocations; need may read O(T) raw text and invoke check.

## composition
Final lookup prioritizes retry even without raw; need preserves raw-first historical stage behavior. Neither makes existence/read atomic.

## precedence
Retry wins final lookup without reading contents.

## error-projection
Strict errors retain path context; Option API erases metadata errors.

## stage-table
Six normal raw/retry states include retry-only -> need 1 but selected retry.

## symlink-domain
Self-links witness ELOOP independent of permission privileges; dangling targets remain absent.

## checks
Finite synthetic Rust tests in owned sources; broad offline qagen lib suite twice per profile, explicitly skipping lemma::tests::doubt_with_en_model. Compiled production mutations reach named assertion failures. Exact outcomes and hashes are in the file groupant ../model.md#evidence.

## epoch-memory

Prospective model saved before repairs. Old-code witnesses demonstrated token overflow, JSON null from float overflow, premature report replacement, and masked metadata errors. Ordinary output keys, finite negative history and legacy Option lookup semantics are preserved. Scheduler was not redesigned.

## limitations
Partial source models and finite observations, not proof of all concurrent/filesystem states. Syntax projection is partial; macros opaque. Allocation failure, races and crashes may prevent completion. No live teacher/model/corpus/database was used. No useful certified metric for a numerical surrogate of these error-sensitive operations was identified; executable approximant is absent, with no invented error budget or speed claim.

## History

### 2026-10-30 · epoch-memory

2026-10-30 prospective model saved before repairs. Old-code witnesses demonstrated token overflow, JSON null from float overflow, premature report replacement, and masked metadata errors. Ordinary output keys, finite negative history and legacy Option lookup semantics are preserved. Scheduler was not redesigned.
