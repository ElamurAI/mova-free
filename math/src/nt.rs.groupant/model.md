# math-nt

## descriptant

### nt

gcd returns0 for (0,0), otherwise uses dashu, ext_gcd iterates independent Euclid and lcm handles zero. mod_floor normalizes modulo|m|; modpow uses inverse for negative exponents, rejects m=0 and short-circuits m=1 to0. CRT joins noncoprime congruences with difference divisibility and lcm modulus. Primality uses12 fixed bases for u64,30 plus trial/Pocklington for bigger values. factor strips sign, trial-divides and Pollard-splits, then sorts/merges; probable factors count complete. Multiplicative functions consume factors; digits emits |n| digits, from_digits unrestricted Horner sum.

Wave39 reviews integer digit/Horner transport and its own typed model sections. Finite checks, dependent formula derivations and algebraic claims retain separate scopes; no production speed or universal source proof is inferred.

## approximant

### Forward and cost

gcd returns0 for (0,0), otherwise uses dashu, ext_gcd iterates independent Euclid and lcm handles zero. mod_floor normalizes modulo|m|; modpow uses inverse for negative exponents, rejects m=0 and short-circuits m=1 to0. CRT joins noncoprime congruences with difference divisibility and lcm modulus. Primality uses12 fixed bases for u64,30 plus trial/Pocklington for bigger values. factor strips sign, trial-divides and Pollard-splits, then sorts/merges; probable factors count complete. Multiplicative functions consume factors; digits emits |n| digits, from_digits unrestricted Horner sum.

Euclid O(log magnitude) big divisions, modpow O(log|e|) ring operations; CRT one gcd/inverse per pair. Sieve O(n log log n)/O(n), cached small primes to1e6. Factor cost includes trial, recursive primality and Pollard without strict total time cap. Divisors materializes tau(n) values and sorts O(tau log tau), unchecked u128 count product. Digit conversion/repeated division and Horner have operand-width costs.

No new executable Rust surrogate: exact values, branching, domains and effects require an observation-specific metric before supported 1/2/5/10% profiles. Existing floating diagnostics are not such a profile; no speed claim.

## invariant

### Domain and invariant

mod_floor/check_crt require nonzero moduli; check_crt zips mismatched arrays and accepts empty zip. Fixed MR bases lack the randomized premise for textual4^-30 bound. complete permits probable primes. factor_limited ignores Limits.max_steps, uses a separate partial Pollard budget; trial/primality/recovery bypass it and u64 ignores deadline. Sieve dimensions can overflow/allocate; public prime_pi caps2e8 and nth_prime caps1e7. Clock and OnceLock cache are internal effects.

gcd returns0 for (0,0), otherwise uses dashu, ext_gcd iterates independent Euclid and lcm handles zero. mod_floor normalizes modulo|m|; modpow uses inverse for negative exponents, rejects m=0 and short-circuits m=1 to0. CRT joins noncoprime congruences with difference divisibility and lcm modulus. Primality uses12 fixed bases for u64,30 plus trial/Pocklington for bigger values. factor strips sign, trial-divides and Pollard-splits, then sorts/merges; probable factors count complete. Multiplicative functions consume factors; digits emits |n| digits, from_digits unrestricted Horner sum.

CRT (x,M) admits all nonempty matched congruence arrays whose lcm is M and residues agree modulo|mi|, including signs/order/redundancies. Factor list reconstructs magnitude, losing sign; empty list admits0 and±1 under producer conventions. Canonical digits recover magnitude but lose sign; unrestricted from_digits has many noncanonical antecedents. Probable verdict does not recover an integer or proof.

## composition

### Composition

gcd returns0 for (0,0), otherwise uses dashu, ext_gcd iterates independent Euclid and lcm handles zero. mod_floor normalizes modulo|m|; modpow uses inverse for negative exponents, rejects m=0 and short-circuits m=1 to0. CRT joins noncoprime congruences with difference divisibility and lcm modulus. Primality uses12 fixed bases for u64,30 plus trial/Pocklington for bigger values. factor strips sign, trial-divides and Pollard-splits, then sorts/merges; probable factors count complete. Multiplicative functions consume factors; digits emits |n| digits, from_digits unrestricted Horner sum.

calc dispatches to comb/nt. poly/linsys consume calc E and stateful Env for constants; deriv consumes E and calc::approx. fcheck consumes separate legacy expr::Expr/units, with no implicit lossless E adapter. Q/Z joins use big. Parent symbolic-computation models these edges; local checks do not certify application wiring. Nested scopes reuse parent syntax, and remaining helpers/types/tests are parent-modeled.

## models

# Typed models

## behavior_complement

For fixed admitted input/state, exclude outputs and effect traces contradicting this actual relation: gcd returns0 for (0,0), otherwise uses dashu, ext_gcd iterates independent Euclid and lcm handles zero. mod_floor normalizes modulo|m|; modpow uses inverse for negative exponents, rejects m=0 and short-circuits m=1 to0. CRT joins noncoprime congruences with difference divisibility and lcm modulus. Primality uses12 fixed bases for u64,30 plus trial/Pocklington for bigger values. factor strips sign, trial-divides and Pollard-splits, then sorts/merges; probable factors count complete. Multiplicative functions consume factors; digits emits |n| digits, from_digits unrestricted Horner sum. These exclusions are conditional on the stated domain. Unavailable checks, passing residuals and resource limits do not establish global correctness.

## reverse_reconstruction

CRT (x,M) admits all nonempty matched congruence arrays whose lcm is M and residues agree modulo|mi|, including signs/order/redundancies. Factor list reconstructs magnitude, losing sign; empty list admits0 and±1 under producer conventions. Canonical digits recover magnitude but lose sign; unrestricted from_digits has many noncanonical antecedents. Probable verdict does not recover an integer or proof.

## congruence_fiber

Normalize modulus sign; m=1 returns0 before negative-exponent inverse. CRT tests residual difference modulo gcd and returns0<=x<M.

## primality_evidence

Prime(u64), Prime(Pocklington), Probable differ; is_prime_like merges latter two, and check_factor cannot upgrade probable evidence.

## resource_boundary

Pollard budget counts blocks, omits trial stripping/primality/recovery; u64 never checks deadline. Not a total work or latency guarantee.

## representation_transport

digits for base>=2 is most-significant-first abs(n), zero=[0]. from_digits([],b)=0 without digit/base validation; canonical roundtrip only recovers magnitude.

## gcd_zero_boundary

For the public signed-integer gcd wrapper, G(0,0)=0; otherwise G(a,b) is dashu's nonnegative gcd. Reverse fiber G^-1(0)={(0,0)}. For g>0, compatible signed pairs satisfy a=g*u,b=g*v with gcd(|u|,|v|)=1. Complement in Z²×Z excludes negative outputs, non-gcd values, and nonzero outputs at zero-zero. This source-derived relation is checked on a bounded signed grid and selected big integers; the general arithmetic theorem is not proved here. The zero guard aligns the wrapper with independent ext_gcd/check_gcd and prevents the dependency's zero-zero panic.

## tests

### Actual-code evidence

Each selected named test passed twice debug/twice release, sorted outcomes identical:

- `test nt::tests::gcd_lcm_bezout ... ok`
- `test nt::tests::modular ... ok`
- `test nt::wave28::signed_moduli_and_digit_fibers ... ok`

Finite synthetic fixtures only; no teacher/API/corpus/GPU/heldout. No allocation/stack faults, huge sizes, deadline proof, or universal numerical bound.

Final validation: eight compiled mutants total (seven module boundaries plus linsys width guard), all compile 0 / test 101 and exact restoration / positive 0. Old-code count mismatch witness compile 0 / test 101 separately. Full-registry structural run reports only ancestor integration bindings; isolated source snapshots validate each owned sidecar. Strict Clippy: dependency invocation blocked in existing global build script; --no-deps reaches math and reports 113 existing library findings, retained externally, no clean lint claim. Added test blocks formatted independently; production layout preserved.

## wave39-digits

# Integer digit relations — wave39

Let H_b(d)=Σ d_i b^(k-1-i), exactly the arbitrary-integer `from_digits` fold in most-significant-first order. For u32 b>=2, D_b(n)=`digits(n,b)` returns the canonical word for |n|. H accepts every finite integer coefficient list and integer base; D has a narrower base type and admission rule. Allocation failure is outside these relations. Statements below are source-derived algebra with bounded actual-API evidence, not machine-checked universal proofs.

## canonical_sign_fiber

Domain: integers n and u32 b>=2. Observation: exact digit vectors and H values. Canonical words C_b are nonempty, have coefficients in [0,b), and have no leading zero except [0]. D_b has image C_b; H_b(D_b(n))=|n|. For c in C_b, D_b^-1(c)={H_b(c),-H_b(c)}, a singleton for [0]. Outside C_b the preimage is empty. This is a relational reconstruction, not a signed inverse. Negative n=-12 and n=12 both emit [1,2] in base10; [] and [0,1] are not emitted. Bounded signed-grid checks cover 15,375 (n,b) pairs, including the decimal text branch and repeated-division branch. Completeness outside the tested grid is a source-derived claim with the stated arithmetic assumptions.

## positional_carry_kernel

Domain: fixed k and b in Z; observation H_b(d), not digit validity. For adjacent MSB positions, (u,v)->(u+t,v-b*t) preserves H for t in Z. For k>=1 the full fiber of n is {(p,n-b*H_b(p)):p in Z^(k-1)}. For k=1 this is a singleton; for k>=2 it is infinite. For k=0 only n=0 has antecedent []. The adjacent vectors e_i-b*e_(i+1) generate the integer kernel: their first k-1 coordinates are triangular with unit diagonal, and the remaining coordinate is fixed by H. This is an algebraic derivation reviewed by hand, not Lean evidence. Actual API checks perform 75,250 finite carry interventions. Counterexample to omitting the b factor: prefix [4,7], b=10, target123 needs last=-347; last=123-H(prefix)=76 produces546. Canonical digit constraints generally do not admit arbitrary carries, so this fiber must not be described as canonical reconstruction.

## base_admission_partition

Domain: D's u32 base versus H's integer base. Observation: explicit Result rejection versus returned integer. D rejects b=0,1 before reading magnitude, including n=0. H_0(d) is the last coefficient for nonempty lists, H_1 is the sum, H_b([])=0 even for negative b. Negative bases have no D input representation. Witness [1,2]: H_0=2, H_1=3, H_-2=0; D rejects the first two and cannot accept the third base. Thus H's broader admission is not a D error partition or a two-way codec contract.

## magnitude_normalizer

Domain: every integer coefficient list d and fixed u32 b>=2. Observation N_b(d)=D_b(H_b(d)), not preservation of signed H. N maps onto C_b and is idempotent: H(N(d))=|H(d)|, D(|n|)=D(n), hence N(N(d))=N(d). It fixes canonical words. Negative-valued [1,-20] at base10 becomes [1,0], changing H from -10 to10, refuting signed-value preservation. Finite API checks cover 1,562 signed-list/base cases plus leading-zero and negative-value witnesses. Idempotence does not recover sign, length, original coefficients or prior normalization history.

## finite_catalogue

Twelve named observations bind H's actual result to the algorithm Horner value after coefficient reversal. The catalogue uses a shared comparison contract: f64 intermediate and Ok premises constrain the algorithm side only; they are not intrinsic restrictions of the integer H API. Converse and rectangle-complement entries are explicit derived constructions of those forward tables; they add role coverage but no independent API evidence. The complement excludes wrong listed output pairs only, not every unknown output or failure. See the algorithm's wave39 model for floating-point and paired-Result admission. No executable percentage approximant or production substitution is introduced.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: complete source read before edits, bounded actual-code tests and source controls. Linsys shape defect was witnessed on old code and repaired; other semantics retained. External receipt name octopus-wave28-mathcore-2026-10-31 includes prospective.md. Registration is partial, not verification.

Epoch36, 2026-10-31: cross-groupant comparison exposed a real wrapper boundary mismatch. The original public gcd delegated (0,0) to dashu, which panicked; independent ext_gcd/check_gcd and algorithm gcd already represented zero. A new native regression failed on the original body, then passed with a zero-pair guard. All other inputs still delegate to dashu. Signed-grid and big-integer certificate checks constrain the correction; no broad number-theory behavior or speed claim. The earlier description 'gcd uses dashu' omitted this dependency precondition and has been revised. Historical baseline source/hash and failure log remain in external epoch36 evidence.

### 2026-10-31 · tests

Heading `### Actual-code evidence 2026-10-31` stays in tests as `### Actual-code evidence`.

Compiled source control: control-nt.patch.txt; compiler exit0, test nonzero, exact source restoration and focused positive exit0. Full logs and numeric exits retained externally in octopus-wave28-mathcore-2026-10-31. Linsys also retains shape-old compile 0 / test 101 witness before shape repair. Seven module controls plus the additional linsys width-guard control; fragment declarations are not extra tests.

signed_grid_and_large_operands_preserve_euclid compares1088 nonzero pairs in [-16,16]² against independent extended Euclid and checks certificates, plus signed and zero operands beyond128-bit width. Zero-zero agrees with ext_gcd, check_gcd and lcm; false certificate g=1 is rejected. External octopus-wave36-math-2026-10-31 contains gcd-baseline-debug.log, gcd-fixed-debug.log, gcd-fixed-release.log, gcd-bindings.log and nt-baseline.rs. Two focused tests only;95 existing math tests filtered out. No broad factorization/primality or speed claim.

### 2026-11-04 · document

Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.

### Undated · tests

#### Wave36 public gcd zero boundary

Native regression zero_pair_agrees_with_independent_certificate failed on the original wrapper at source SHA502037c74e4cf1b88674de6594fe4a611e92eacc9d13ee96db6aac1b51922164: dashu panicked for zero-zero; compile succeeded, test exit101. The companion signed/large test passed on that baseline. After the explicit guard, both new tests passed debug and release with jobs2. Final source SHA27f4bf91578ab245e6b50f5f880126990b73b93b46882c37127e241031c359fe.
