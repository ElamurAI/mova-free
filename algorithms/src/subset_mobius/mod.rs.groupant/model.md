# algorithm-subset-mobius

## descriptant

### subset_mobius mod.rs

Retained observation: Tie orientation, residue normalization and first-error order are part of the public behavior, not incidental implementation choices.

The reference attribution is in production module documentation. No copied solution text or implementation is used. No approximate surrogate is useful for the exact API here, and no performance gain is claimed. Partial syntax is generated twice identically; macro bodies remain opaque and semantic AST/proofs are absent. These small files have no nested fragments.

## model

# subset_mobius mod.rs model

## Forward

For n=2^k nonempty,m>0, y[S]=sum_{T subset S} (-1)^(popcount(S xor T)) x[T] mod m. Normalize then subtract each bit-cleared coefficient from bit-set coefficients, once per coordinate. This is subset-lattice Mobius inversion, inverse to subset zeta over every Z/mZ.

## Invariant

Length unchanged, residues in [0,m). If a>=b use a-b, else m-(b-a), with normalized operands; neither branch overflows or returns m. Bit-cleared source remains fixed within a stage. Invalid calls preserve raw vector bytes.

## Cost

k*n mask visits, k*n/2 modular subtractions, n normalizations; O(n log n) time and O(1) extra space, n<=2^20. No primality test, inversions, allocation or speed claim.

## behavior_complement

Universe: (raw vector,modulus,result,final vector). Admitted outputs differing from the signed literal submask sum modulo m are absent, as are residues >=m and changed lengths. Invalid input with Ok or changed raw values is absent. ZeroModulus precedes Empty, NotPowerOfTwo, TooLong.

## reverse_reconstruction

For fixed valid n,m normalized outputs have unique normalized antecedents given by zeta sums. Raw u64 inputs remain ambiguous by adding multiples of m where representable. For m=1 every raw input of fixed shape yields all zeros. Error observations do not recover original entries.

## inclusion_exclusion

Each removed coordinate contributes one negative sign, yielding (-1)^popcount(S xor T). The oracle sums signed i128 raw coefficients directly, independently of stage logic; its n<=8 domain fits i128 easily.

## subtraction_boundary

For a<b the distance b-a is in 1..m, so m-(b-a) is in 1..m. For a>=b the difference is <m. This supports composite moduli and u64::MAX without signed conversion in production.

## validation_partition

Check m=0, empty, non-power-of-two, cap in this exact order before any normalization. Full-cap constant raw input becomes one normalized coefficient at index0 and zeros elsewhere; errors preserve nonnormalized values.

## evidence

### Evidence

Cartesian and inversion enumerate sum(3^n,n=0..8)=9841 vectors each; subset files enumerate 3+9+81+6561=6654 vectors at five moduli, 33270 comparisons each. Cap fixtures dominate memory/time; no benchmark claim.

Raw logs are external. Bound evidence hashes:

No whole-repository test, allocator injection, general proof, live parser/teacher/GPU, approximation-error or runtime improvement claim.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: prospective model selected bounded exact computation before implementation. k*n mask visits, k*n/2 modular subtractions, n normalizations; O(n log n) time and O(1) extra space, n<=2^20. No primality test, inversions, allocation or speed claim.

### 2026-10-30 · evidence

Heading `### Evidence, 2026-10-30` stays in evidence as `### Evidence`.

Source SHA-256: `d63acc0b6b3a974ba487084fafa0311f413941810f56c6d261cb9e2bb5b2d642`. Twelve focused named tests (three per module) pass twice in debug and twice in release with identical normalized outcomes. Strict all-target Clippy and narrow rustfmt pass. Each production module has a compiled negative control rejected by its exhaustive assertion; test false-oracle controls are separately rejected. Exact source bytes are restored and positives rerun.

- debug1.log: `021da34b29ae9f5cc1d3751645c5bb3a2c09a62eeda74ea8c442c94a93c53c25`

- debug2.log: `350882b59fe92db00ef7c743664a7433915b683196ef7b2b3ce4bd8f5d8f187e`

- release1.log: `c6cb5024abb354e23a1375199f33038203b09305a89fb1352f60ae20b2515f87`

- release2.log: `ec4f6efdc7bd8210b565e8cbe9cb4007b7cbde559d25e5bdfca3f24f18b6e975`

- clippy.log: `d47f40a950cf3add1187a3c07f8b1127360005b8cb35d241c708c19d6a88de72`

- controls.log: `d5acd772f862cfc084f5ea74402e308c05b6d234805115044bc7885eaa012c8e`

- restored.log: `04a4e8576898dcaaf398f5d69f3023d009bc6764fbbca286ff6de0d3436626b7`

- normalized.txt: `f018bb079b167e41fcff7b40a8afa77507f8eba332c86b50064506e1cb318d19`
