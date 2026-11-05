# algorithm-inversion-count-tests

## descriptant

### inversion_count tests.rs

Retained observation: Independent literal oracles observe direction and ties without relying only on roundtrip agreement.

The reference attribution is in production module documentation. No copied solution text or implementation is used. No approximate surrogate is useful for the exact API here, and no performance gain is claimed. Partial syntax is generated twice identically; macro bodies remain opaque and semantic AST/proofs are absent. These small files have no nested fragments.

## model

# inversion_count tests.rs model

## Forward

Execute independent finite oracle for inversion_count, literal direction/tie cases, and accepted/rejected cap witnesses. Each assertion compares concrete API observations against independently constructed expected outputs. Return normally iff all scheduled assertions hold; panic is the observed rejection channel.

## Invariant

Enumeration order and expected arithmetic are deterministic. Oracle domain is finite; passing this file does not quantify over all lawful values, allocators, or implementations.

## Cost

Cartesian and inversion enumerate sum(3^n,n=0..8)=9841 vectors each; subset files enumerate 3+9+81+6561=6654 vectors at five moduli, 33270 comparisons each. Cap fixtures dominate memory/time; no benchmark claim.

## behavior_complement

Universe: candidate implementation behaviors on this file's exact fixtures paired with normal/panic outcomes. Normal completion is absent when inversion_count's independently computed tree/pair/submask expectation differs from actual, when a literal differs, or when error precedence/state preservation differs. Inputs outside these fixtures impose no test constraint; this complements the explicit assertion relation only.

## reverse_reconstruction

A normal test result retains every implementation agreeing on all observed fixtures, including implementations differing elsewhere. Failure identifies an assertion and witness but not a unique bug. A same-output optimized implementation and the current algorithm have identical test observations; this ambiguity cannot reconstruct production source.

## oracle_independence

Tree expectations recursively scan interval minima; inversion expectations enumerate all strict pairs; subset expectations enumerate all subsets and signed parity. None invokes another production algorithm as its sole oracle. Mutation runs establish rejection of one selected wrong implementation, not completeness.

## fixture_cardinality

Cartesian and inversion enumerate sum(3^n,n=0..8)=9841 vectors each; subset files enumerate 3+9+81+6561=6654 vectors at five moduli, 33270 comparisons each. Cap fixtures dominate memory/time; no benchmark claim.

## control_scope

A compiled production mutation changes one arithmetic or tie decision; a test-only false expectation changes the oracle independently. Named assertion failure, compilation success, exact source restoration and positive rerun are required observations. Unsampled faults remain outside this evidence.

## evidence

### Evidence

Cartesian and inversion enumerate sum(3^n,n=0..8)=9841 vectors each; subset files enumerate 3+9+81+6561=6654 vectors at five moduli, 33270 comparisons each. Cap fixtures dominate memory/time; no benchmark claim.

Raw logs are external. Bound evidence hashes:

No whole-repository test, allocator injection, general proof, live parser/teacher/GPU, approximation-error or runtime improvement claim.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: prospective model selected bounded exact computation before implementation. Cartesian and inversion enumerate sum(3^n,n=0..8)=9841 vectors each; subset files enumerate 3+9+81+6561=6654 vectors at five moduli, 33270 comparisons each. Cap fixtures dominate memory/time; no benchmark claim.

### 2026-10-30 · evidence

Heading `### Evidence, 2026-10-30` stays in evidence as `### Evidence`.

Source SHA-256: `3ac72ee6f9cb82afd692bfa3a67aaa51581d85791f73910ad6147c811dfe0a59`. Twelve focused named tests (three per module) pass twice in debug and twice in release with identical normalized outcomes. Strict all-target Clippy and narrow rustfmt pass. Each production module has a compiled negative control rejected by its exhaustive assertion; test false-oracle controls are separately rejected. Exact source bytes are restored and positives rerun.

- debug1.log: `021da34b29ae9f5cc1d3751645c5bb3a2c09a62eeda74ea8c442c94a93c53c25`

- debug2.log: `350882b59fe92db00ef7c743664a7433915b683196ef7b2b3ce4bd8f5d8f187e`

- release1.log: `c6cb5024abb354e23a1375199f33038203b09305a89fb1352f60ae20b2515f87`

- release2.log: `ec4f6efdc7bd8210b565e8cbe9cb4007b7cbde559d25e5bdfca3f24f18b6e975`

- clippy.log: `d47f40a950cf3add1187a3c07f8b1127360005b8cb35d241c708c19d6a88de72`

- controls.log: `d5acd772f862cfc084f5ea74402e308c05b6d234805115044bc7885eaa012c8e`

- restored.log: `04a4e8576898dcaaf398f5d69f3023d009bc6764fbbca286ff6de0d3436626b7`

- normalized.txt: `f018bb079b167e41fcff7b40a8afa77507f8eba332c86b50064506e1cb318d19`
