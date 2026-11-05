# algorithm-miller-rabin-tests

## descriptant

### algorithm-miller-rabin-tests

Run independent trial division on n=0..99999; compare the Boolean classifier. Check literal pseudoprime and product composites, large prime fixtures, and literal prime fiber below50. Test assertions report exact disagreement input.

Prospective model: O(N sqrt N) upper bound for trial enumeration with N=100000; constant-size boundary arrays and O(50) fiber vector. This is bounded test work, not an exhaustive-u64 proof.
No optimization or speed claim is made. The small-prime enumeration and full-width boundary fixtures are complementary: the former is independent but finite, the latter stresses arithmetic without exhausting the domain. Primality depends on the cited deterministic witness-set result. Root search has a finite but potentially large worst-case bound; no randomized efficiency inference is made.

Partial syntax is source-bound and generated twice identically. Macro bodies are opaque, semantic AST and general mathematical proof remain absent. No executable approximate surrogate is introduced because these exact scalar APIs require exact outputs.

## model

# algorithm-miller-rabin-tests

## Forward

Run independent trial division on n=0..99999; compare the Boolean classifier. Check literal pseudoprime and product composites, large prime fixtures, and literal prime fiber below50. Test assertions report exact disagreement input.

## Invariant

Trial oracle divides integers directly; d<=n/d avoids squared-divisor overflow. Product fixtures are selected to fit u64. Expectations do not reuse modular exponentiation.

## Cost

O(N sqrt N) upper bound for trial enumeration with N=100000; constant-size boundary arrays and O(50) fiber vector. This is bounded test work, not an exhaustive-u64 proof.

## behavior_complement

For each enumerated n, acceptance of an is_prime(n) value opposite to the trial-divisor truth is forbidden. Literal fiber checks also forbid omitting a prime or inserting a composite below50. Failure does not by itself localize implementation versus fixture defects.

## reverse_reconstruction

A passing test summary is compatible with any implementation agreeing on all exercised inputs, including implementations wrong on an unexercised u64. Thus success cannot recover source or establish universal correctness; failing n narrows the compatible disagreement location.

## divisor_oracle

For n>=2 trial returns false iff an integer d in [2,floor(sqrt n)] divides n. This elementary characterization is independent of strong probable-prime witnesses.

## pseudoprime_strength

Boundary fixtures include 2047,1373653,3215031751,341550071728321 and3825123056546413051: composite values that expose insufficient fixed-witness testing. The array is finite and not a complete pseudoprime catalog.

## prime_fiber

The explicit below50 prime list is a second observation of the Boolean output fiber. All other integers below50 map false; no inverse identification of a unique prime follows.

## oracle_control

Replacing the trial oracle initial n<2 branch with n<3 yields the concrete disagreement at2. This compiled test-only control must fail exhaustive_trial_division while restored bytes pass.

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

Source SHA-256: 2d8f0f017ba230eb9ac41a7020598a4c711cddbfc0d3b169857f99f8a09ae1c8.

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
