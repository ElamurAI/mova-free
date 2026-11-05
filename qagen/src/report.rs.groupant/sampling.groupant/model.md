# qagen-report-sampling

## forward
Gather pairs, filter optional explicit weak support, sort seeded (hash,index) keys, render up to n pairs and write sample.md or weak.md.

## invariant
Hash ties resolve by original pair index. Selected order deterministic for identical source inputs and seed.

## behavior-complement
Universe: finite supplied values and filesystem observations, return values, errors and writes in this scope. No external entropy, semantic correctness or recall guarantee; omitted pairs never appear in sample.

## reverse-reconstruction
F^-1(y) contains all compatible inputs and initial states yielding y. Output leaves unselected pairs and most paragraph metadata unconstrained. Same sample may arise under different seeds producing the same selected ordering. No complete inverse solver or unique recovery claim.

## cost
O(N log N) sorting plus support/gather/render cost; O(N) keys and selected output bytes.

## composition
Gate-valid anchors feed support and rendering; weak predicate selects explicit pairs below threshold before seeded sort.

## hash-key
Uses paragraph key#pair-k with supplied seed, not answer text.

## tie-order
Original gathered pair index resolves equal hashes.

## weak-filter
Some(t) requires explicit and support<t; NaN t selects none.

## output-size
take(n) bounds pair count, not rendered bytes; long paragraph text remains unbounded by n.

## checks
Finite synthetic Rust tests in owned sources; broad offline qagen lib suite twice per profile, explicitly skipping lemma::tests::doubt_with_en_model. Compiled production mutations reach named assertion failures. Exact outcomes and hashes are in the file groupant ../model.md#evidence.

## epoch-memory

Prospective model saved before repairs. Old-code witnesses demonstrated token overflow, JSON null from float overflow, premature report replacement, and masked metadata errors. Ordinary output keys, finite negative history and legacy Option lookup semantics are preserved. Scheduler was not redesigned.

## limitations
Partial source models and finite observations, not proof of all concurrent/filesystem states. Syntax projection is partial; macros opaque. Allocation failure, races and crashes may prevent completion. No live teacher/model/corpus/database was used. No useful certified metric for a numerical surrogate of these error-sensitive operations was identified; executable approximant is absent, with no invented error budget or speed claim.

## History

### 2026-10-30 · epoch-memory

2026-10-30 prospective model saved before repairs. Old-code witnesses demonstrated token overflow, JSON null from float overflow, premature report replacement, and masked metadata errors. Ordinary output keys, finite negative history and legacy Option lookup semantics are preserved. Scheduler was not redesigned.
