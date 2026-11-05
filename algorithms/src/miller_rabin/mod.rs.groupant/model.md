# algorithm-miller-rabin

## descriptant

### algorithm-miller-rabin

Reject n<2; handle even n; write n-1=d*2^s for odd n. For each fixed reduced base a!=0, require a^d=1 or a^(d*2^j)=-1 for some 0<=j<s. Return true iff every witness passes. The full-u64 seven-witness theorem is an external finite-range premise, attributed to Jim Sinclair by the deterministic variants table at https://miller-rabin.appspot.com/. Rabin (1980) supplies the strong probable-prime background, not this witness-set provenance.

Prospective model: Seven witnesses, at most 64 exponent bits and 62 follow-up squarings each: O(log n) modular operations and O(1) space on fixed u64 words.
No optimization or speed claim is made. The small-prime enumeration and full-width boundary fixtures are complementary: the former is independent but finite, the latter stresses arithmetic without exhausting the domain. Primality depends on the cited deterministic witness-set result. Root search has a finite but potentially large worst-case bound; no randomized efficiency inference is made.

Partial syntax is source-bound and generated twice identically. Macro bodies are opaque, semantic AST and general mathematical proof remain absent. No executable approximate surrogate is introduced because these exact scalar APIs require exact outputs.

## model

# algorithm-miller-rabin

## Forward

Reject n<2; handle even n; write n-1=d*2^s for odd n. For each fixed reduced base a!=0, require a^d=1 or a^(d*2^j)=-1 for some 0<=j<s. Return true iff every witness passes. The full-u64 seven-witness theorem is an external finite-range premise, attributed to Jim Sinclair by the deterministic variants table at https://miller-rabin.appspot.com/. Rabin (1980) supplies the strong probable-prime background, not this witness-set provenance.

## Invariant

Odd tested n>=3; d odd; 1<=s<=63; every residue lies in [0,n); u128 holds (n-1)^2 exactly. Input is immutable and all loops have fixed bit/witness bounds.

## Cost

Seven witnesses, at most 64 exponent bits and 62 follow-up squarings each: O(log n) modular operations and O(1) space on fixed u64 words.

## behavior_complement

Universe U is u64 x Bool. Pairs (n,true) with n<2 or composite n and (n,false) with prime n are excluded by the deterministic specification. This is a source-derived target conditional on the cited witness result; finite tests alone do not prove the whole domain.

## reverse_reconstruction

True has fiber all u64 primes; false has every composite plus 0 and 1. Both fibers contain many inputs; no numerical input can be uniquely recovered from its Boolean observation. In the 0..50 restriction the true fiber is enumerated literally.

## witness_reduction

A base divisible by candidate n contributes no rejection: a=0 must be skipped. For prime n dividing a fixed witness, rejection would violate primality. Small-prime exhaustive trials exercise such divisors.

## strong_chain

For odd composite n, the complement includes any tested witness with initial x neither 1 nor -1 and no later -1. The witness is a rejection certificate; passing one witness is not primality. Named pseudoprimes expose weakened base sets.

## product_boundary

Multiplication embeds both residues into u128 before their product. Boundary composites include (4294967291)^2 and u64::MAX; replacing wide multiplication with wrapping u64 has no correctness premise.

## finite_oracle

Trial division independently checks 100000 consecutive integers using d<=n/d, without modular exponentiation or the witness set. Literal large primes and known pseudoprimes supplement, but do not exhaust, the full-u64 theorem premise.

## evidence

### Evidence

Eight named tests total: four per algorithm. Each focused set passes twice in debug and twice in release with equal normalized named outcomes. Primality checks100000 consecutive integers by independent trial division plus finite pseudoprime and full-width fixtures. Roots exhaust every a<3p and root candidate for every prime p<200, plus large-field square identities.

Compiled production controls: odd-prime acceptance changed to false in Miller-Rabin; canonical min changed to max in Tonelli-Shanks. Named assertions reject both. Test-only false trial/root oracles also fail their named exhaustive assertions. Exact source bytes are restored before positive rechecks. Raw logs and normalized receipts remain in the external wave14 numeric run bundle.

Strict all-target algorithm Clippy and narrow rustfmt checks executed. Finite evidence does not establish all-u64 correctness, worst-case practical runtime, or full-system behavior.

#### Evidence bindings

12681 exhaustive root cases across46 primes below200;100000 independent trial-division cases. Production control witnesses: n=3 and (a=1,p=3). All four restored sources passed the repeated profiles.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: original exact implementation and independent fixtures.

### Undated · evidence

Source SHA-256: 0e6f4c3f59e549dc83106dbc98834b9eac28366202b094d60f67c288996f270b.

- checks.log: a488b6b18b14829f74ad4c8ad3b795bdb4aba9d663d9efa249aac72e6c7c4f4e

- normalized-debug-1.txt: 406299e9aef489625b11aaf04a81f6308836e3f53ca151a8ffae946479f76bdd

- normalized-debug-2.txt: 406299e9aef489625b11aaf04a81f6308836e3f53ca151a8ffae946479f76bdd

- normalized-release-1.txt: 406299e9aef489625b11aaf04a81f6308836e3f53ca151a8ffae946479f76bdd

- normalized-release-2.txt: 406299e9aef489625b11aaf04a81f6308836e3f53ca151a8ffae946479f76bdd

- clippy-final.log: bed8a7567193417c4dcfd75194a4c30434db100e1dd2b4518bd92b42b0537d6f

- mutant-primality.log: 8b845fbf9f323c0cd1c0c1ea98918e7447cf2cd6289892d8feb9e29cc5d43a8a

- mutant-root.log: aece9c16d4d78a8b585a2a155733be68ffa362d152cd22bf84f44f2adbb719ca

- mutant-trial-oracle.log: 87a1e69f1e1915cb3205602ee4c09ca2b51c8141992c702d55a614a5153f4eb6

- mutant-root-oracle.log: 465be9f24fd24a75bcda68251efd7f3dc0f8fdd85ae0a52b2fdc9b02755e1b23
