# algorithm-tonelli-shanks-tests

## descriptant

### algorithm-tonelli-shanks-tests

Enumerate primes p<200 by naive integer divisibility independently of production primality; enumerate every root x in0..p for radicands a in0..3p. The first matching root is the canonical oracle. Check typed errors, p=2, normalization fibers and large-prime squares.

Prospective model: Sum over prime p<200 of O(p²) simple products for the exhaustive root search, plus bounded large-prime production calls. Fixed finite fixture domain, no claim about global timing.
No optimization or speed claim is made. The small-prime enumeration and full-width boundary fixtures are complementary: the former is independent but finite, the latter stresses arithmetic without exhausting the domain. Primality depends on the cited deterministic witness-set result. Root search has a finite but potentially large worst-case bound; no randomized efficiency inference is made.

Partial syntax is source-bound and generated twice identically. Macro bodies are opaque, semantic AST and general mathematical proof remain absent. No executable approximate surrogate is introduced because these exact scalar APIs require exact outputs.

## model

# algorithm-tonelli-shanks-tests

## Forward

Enumerate primes p<200 by naive integer divisibility independently of production primality; enumerate every root x in0..p for radicands a in0..3p. The first matching root is the canonical oracle. Check typed errors, p=2, normalization fibers and large-prime squares.

## Invariant

Small root products x*x fit u64 because p<200. Large fixture products explicitly widen to u128. Independent prime enumeration never calls production is_prime.

## Cost

Sum over prime p<200 of O(p²) simple products for the exhaustive root search, plus bounded large-prime production calls. Fixed finite fixture domain, no claim about global timing.

## behavior_complement

For enumerated prime p and a<3p, successful comparison excludes any result other than the least enumerated modular root, or None if the enumeration is empty. Error fixtures forbid acceptance of invalid or composite fields even for a=0.

## reverse_reconstruction

An all-pass observation retains every implementation matching the finite fixture outputs; untested calls can differ. For fixed p, multiple representatives a share the same observed root, as explicitly checked for4,11,18,25 modulo7.

## root_oracle

Naive ordered root enumeration finds min{x in0..p : x² mod p=a mod p}; no Euler criterion, subgroup recurrence or production primality contributes to that oracle.

## large_square

For selected large prime p and x, generate a=x² mod p with u128 and require min(x,p-x). The field property supplies the two-root characterization. These fixtures exercise maximal residues and deep powers of two.

## error_precedence

Cartesian fixtures a in {0,1,u64::MAX} with p in {0,1} expect InvalidModulus and selected composite p expect CompositeModulus. Zero radicands cannot bypass field validation.

## oracle_control

Changing the exhaustive root predicate from equality to inequality builds successfully and fails exhaustive_root_enumeration. This demonstrates sensitivity to a wrong independent oracle rather than a compilation-only control.

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

Source SHA-256: e0b76fab115c6c619c57873b52b0833ca590b60323fb2dcc2a58a592c27c9310.

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
