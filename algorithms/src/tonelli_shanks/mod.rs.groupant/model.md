# algorithm-tonelli-shanks

## descriptant

### algorithm-tonelli-shanks

Validate p>=2, then deterministic is_prime(p), before reducing a. p=2 or residue zero returns a%p. Euler criterion rejects nonresidues. For p-1=q*2^s, find the first quadratic nonresidue z by ascending search, set c=z^q, x=a^((q+1)/2), t=a^q, m=s. While t!=1, choose least i with t^(2^i)=1; multiply x by c^(2^(m-i-1)), square that correction into c, update t and m=i. Return min(x,p-x). Tonelli-Shanks follows the method described by Shanks, Five Number-Theoretic Algorithms (1972).

Prospective model: Primality O(log p), ascending search at most p-2 Euler exponentiations, and at most63 reducing rounds with O(log² p) total modular work. O(p log p + log² p) worst-case modular operations, O(1) space. No random expected-time or small worst-case search claim.
No optimization or speed claim is made. The small-prime enumeration and full-width boundary fixtures are complementary: the former is independent but finite, the latter stresses arithmetic without exhausting the domain. Primality depends on the cited deterministic witness-set result. Root search has a finite but potentially large worst-case bound; no randomized efficiency inference is made.

Partial syntax is source-bound and generated twice identically. Macro bodies are opaque, semantic AST and general mathematical proof remain absent. No executable approximate surrogate is introduced because these exact scalar APIs require exact outputs.

## model

# algorithm-tonelli-shanks

## Forward

Validate p>=2, then deterministic is_prime(p), before reducing a. p=2 or residue zero returns a%p. Euler criterion rejects nonresidues. For p-1=q*2^s, find the first quadratic nonresidue z by ascending search, set c=z^q, x=a^((q+1)/2), t=a^q, m=s. While t!=1, choose least i with t^(2^i)=1; multiply x by c^(2^(m-i-1)), square that correction into c, update t and m=i. Return min(x,p-x). Tonelli-Shanks follows the method described by Shanks, Five Number-Theoretic Algorithms (1972).

## Invariant

For nonzero residue: x²=a*t mod p, c has order 2^m, t has order dividing 2^(m-1); each round reduces m, so i<m and shifts are below64. p prime guarantees nonresidue search termination before p. Residues and products fit u64 and u128 respectively.

## Cost

Primality O(log p), ascending search at most p-2 Euler exponentiations, and at most63 reducing rounds with O(log² p) total modular work. O(p log p + log² p) worst-case modular operations, O(1) space. No random expected-time or small worst-case search claim.

## behavior_complement

Universe U is (u64 a,u64 p) x Result<Option<u64>,ModulusError>. For p<2 every output except InvalidModulus is absent; for composite p>=2 every output except CompositeModulus is absent. For prime p, errors, roots outside [0,p/2], wrong squares and None on a quadratic residue are absent; Some on a nonresidue is absent.

## reverse_reconstruction

Fix prime p and observed Some(r). Its fiber contains all u64 a congruent to r² mod p provided 0<=r<=p/2; the fiber is empty otherwise. None has all nonresidue classes. Without fixed p, retain the union over prime p. Errors discard a entirely, so every radicand is compatible with the indicated modulus class.

## validation_partition

Validation occurs before zero or modulus-two shortcuts. InvalidModulus means p in {0,1}; CompositeModulus means p>=2 composite. Inputs (0,9) must not be accepted merely because zero squares to zero.

## canonical_sign

For nonzero square a over odd prime p, the two field roots are x and p-x. Taking the minimum removes sign ambiguity in output but cannot recover which root generated a. p=2 is handled separately; a=0 returns0.

## normalization_fiber

Fix p=7. a in {4,11,18,25} all map to Some(2). More generally representatives differing by a multiple of p share the same output, but addition must remain within u64. Large radicands are reduced before exponentiation.

## subgroup_descent

The correction forces t into a strictly smaller power-of-two subgroup. With i minimal, 0<i<m when t!=1; assigning m=i decreases the rank. Ascending z search is finite because the prime field has nonresidues, not because of any imposed small search cap.

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

Source SHA-256: f3bcec6953a9fc4054133028ce47ced339bf39e578e6c3d98b6c7919ded8c5e0.

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
