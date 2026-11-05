# algorithm-subset-zeta

## descriptant

### subset_zeta mod.rs

Retained observation: Tie orientation, residue normalization and first-error order are part of the public behavior, not incidental implementation choices.

The reference attribution is in production module documentation. No copied solution text or implementation is used. No approximate surrogate is useful for the exact API here, and no performance gain is claimed. Partial syntax is generated twice identically; macro bodies remain opaque and semantic AST/proofs are absent. These small files have no nested fragments.

## model

# subset_zeta mod.rs model

## Forward

For n=2^k nonempty, modulus m>0, output y[S]=sum_{T subset S} x[T] mod m. Normalize then for each bit add the bit-cleared entry to bit-set entries. After j stages all submask choices in the processed j coordinates have been summed.

## Invariant

Length is unchanged and entries remain in [0,m). Each stage reads the bit-cleared half which that stage never writes. u128 sum of two normalized u64 residues fits; modulus need not be prime. Invalid calls return before normalization.

## Cost

k*n mask visits, k*n/2 modular additions, n normalizations; O(n log n) time and O(1) extra space. n<=2^20 bounds stage shifts and indices. No allocation inside transform.

## behavior_complement

Universe: (raw vector, modulus, result, final vector). For admitted calls any final coefficient incongruent to the literal submask sum, changed length, or residue >=m is absent. Invalid calls paired with Ok or a changed vector are absent. Ordered errors are ZeroModulus, Empty, NotPowerOfTwo, TooLong.

## reverse_reconstruction

For fixed admitted n,m, normalized y has exactly one normalized preimage given by subset Mobius inversion. Raw u64 preimages are all vectors congruent coordinatewise to that preimage; [0] and [m] share output when representable. Unknown modulus or errors enlarge fibers; m=1 sends all raw vectors of fixed length to zero.

## stage_coordinate

At processed-bit set B, output at S sums x[S minus U] for U subset (S intersect B). The bit-set update must use mask xor bit, not another coordinate; a literal [1,2,4,8] fixture gives [1,3,5,15] modulo100.

## wide_modulus

Normalized a,b<m<=u64::MAX imply a+b<2^65. Converting each to u128 before addition prevents debug panic/release wrap disagreement. Wide raw values are reduced at entry; m=1 is well defined.

## validation_partition

m=0 dominates all lengths; then n=0, then non-power-of-two, then n>2^20. Therefore cap+1 returns NotPowerOfTwo, and 2*cap returns TooLong. Every error preserves exact original u64 representatives.

## evidence

### Evidence

Cartesian and inversion enumerate sum(3^n,n=0..8)=9841 vectors each; subset files enumerate 3+9+81+6561=6654 vectors at five moduli, 33270 comparisons each. Cap fixtures dominate memory/time; no benchmark claim.

Raw logs are external. Bound evidence hashes:

No whole-repository test, allocator injection, general proof, live parser/teacher/GPU, approximation-error or runtime improvement claim.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: prospective model selected bounded exact computation before implementation. k*n mask visits, k*n/2 modular additions, n normalizations; O(n log n) time and O(1) extra space. n<=2^20 bounds stage shifts and indices. No allocation inside transform.

### 2026-10-30 · evidence

Heading `### Evidence, 2026-10-30` stays in evidence as `### Evidence`.

Source SHA-256: `778eead6c097bec1fea2d06a8457379dc205473e2d92375df870efd163a17231`. Twelve focused named tests (three per module) pass twice in debug and twice in release with identical normalized outcomes. Strict all-target Clippy and narrow rustfmt pass. Each production module has a compiled negative control rejected by its exhaustive assertion; test false-oracle controls are separately rejected. Exact source bytes are restored and positives rerun.

- debug1.log: `021da34b29ae9f5cc1d3751645c5bb3a2c09a62eeda74ea8c442c94a93c53c25`

- debug2.log: `350882b59fe92db00ef7c743664a7433915b683196ef7b2b3ce4bd8f5d8f187e`

- release1.log: `c6cb5024abb354e23a1375199f33038203b09305a89fb1352f60ae20b2515f87`

- release2.log: `ec4f6efdc7bd8210b565e8cbe9cb4007b7cbde559d25e5bdfca3f24f18b6e975`

- clippy.log: `d47f40a950cf3add1187a3c07f8b1127360005b8cb35d241c708c19d6a88de72`

- controls.log: `d5acd772f862cfc084f5ea74402e308c05b6d234805115044bc7885eaa012c8e`

- restored.log: `04a4e8576898dcaaf398f5d69f3023d009bc6764fbbca286ff6de0d3436626b7`

- normalized.txt: `f018bb079b167e41fcff7b40a8afa77507f8eba332c86b50064506e1cb318d19`
