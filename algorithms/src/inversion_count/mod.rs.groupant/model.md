# algorithm-inversion-count

## descriptant

### inversion_count mod.rs

Retained observation: Tie orientation, residue normalization and first-error order are part of the public behavior, not incidental implementation choices.

The reference attribution is in production module documentation. No copied solution text or implementation is used. No approximate surrogate is useful for the exact API here, and no performance gain is claimed. Partial syntax is generated twice identically; macro bodies remain opaque and semantic AST/proofs are absent. These small files have no nested fragments.

## model

# inversion_count mod.rs model

## Forward

Return I(x)=sum over i<j of indicator(x[i]>x[j]). Recursively sort index halves, merge stably by <=; when the right index wins add the number of unconsumed left indices. Original values remain borrowed and never cloned or written.

## Invariant

Each merged index interval is a permutation of its original indices and nondecreasing by value. Each counted cross pair is strict and counted exactly once. 0<=I<=n(n-1)/2, fitting u128 at n<=2^20.

## Cost

O(n log n) comparisons, two O(n) usize buffers and O(log n) recursion depth. Word arithmetic exact under cap; generic Ord comparison cost is not assumed constant in wall time.

## behavior_complement

Universe: lawful-Ord sequence and candidate count. Counts differing from the strict pair sum are absent; equality contributes zero. Input-changing transitions are absent because only index buffers are written. Over-cap Ok is absent; allocation refusal is a separate result.

## reverse_reconstruction

For count k retain every admitted sequence with exactly k strict inversions. [1,2], [7,7], and empty all produce zero; [2,1] and [9,3] both produce one. Length, magnitudes and ties cannot be recovered from count alone. With fixed n fibers remain ambiguous.

## merge_cross_pairs

When right[j]<left[i], sortedness implies it is smaller than all remaining left values, adding left.len-i. Equal values choose left and add zero. Quadratic direct pair enumeration independently observes this relation.

## input_preservation

Read-only &[T] plus index copies require neither Clone nor mutations of T; a lawful non-Clone Key fixture compiles and a saved vector equals input after counting. Interior mutability or unlawful comparison effects are outside the premise.

## large_pair_bound

Descending n distinct values attain n(n-1)/2. At n=2^20 the exact result is 549755289600, beyond u32. A full-cap descending fixture checks this bound and a cap+1 fixture checks rejection.

## evidence

### Evidence

Cartesian and inversion enumerate sum(3^n,n=0..8)=9841 vectors each; subset files enumerate 3+9+81+6561=6654 vectors at five moduli, 33270 comparisons each. Cap fixtures dominate memory/time; no benchmark claim.

Raw logs are external. Bound evidence hashes:

No whole-repository test, allocator injection, general proof, live parser/teacher/GPU, approximation-error or runtime improvement claim.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: prospective model selected bounded exact computation before implementation. O(n log n) comparisons, two O(n) usize buffers and O(log n) recursion depth. Word arithmetic exact under cap; generic Ord comparison cost is not assumed constant in wall time.

### 2026-10-30 · evidence

Heading `### Evidence, 2026-10-30` stays in evidence as `### Evidence`.

Source SHA-256: `3dfd310fccfd46f5646eed66f903cabe87604ada69ddda02c0e36a0fad985aa2`. Twelve focused named tests (three per module) pass twice in debug and twice in release with identical normalized outcomes. Strict all-target Clippy and narrow rustfmt pass. Each production module has a compiled negative control rejected by its exhaustive assertion; test false-oracle controls are separately rejected. Exact source bytes are restored and positives rerun.

- debug1.log: `021da34b29ae9f5cc1d3751645c5bb3a2c09a62eeda74ea8c442c94a93c53c25`

- debug2.log: `350882b59fe92db00ef7c743664a7433915b683196ef7b2b3ce4bd8f5d8f187e`

- release1.log: `c6cb5024abb354e23a1375199f33038203b09305a89fb1352f60ae20b2515f87`

- release2.log: `ec4f6efdc7bd8210b565e8cbe9cb4007b7cbde559d25e5bdfca3f24f18b6e975`

- clippy.log: `d47f40a950cf3add1187a3c07f8b1127360005b8cb35d241c708c19d6a88de72`

- controls.log: `d5acd772f862cfc084f5ea74402e308c05b6d234805115044bc7885eaa012c8e`

- restored.log: `04a4e8576898dcaaf398f5d69f3023d009bc6764fbbca286ff6de0d3436626b7`

- normalized.txt: `f018bb079b167e41fcff7b40a8afa77507f8eba332c86b50064506e1cb318d19`
