# algorithm-pollard-rho

## descriptant

### pollard_rho mod.rs

Reject n=0, then excessive budget. Traverse pending divisors: skip one; emit deterministic Miller-Rabin primes; split even composites for free; otherwise run x<-f(x),y<-f(f(y)), f(t)=t*t+c mod value, gcd(|x-y|,value). Start c=1 and x=y=2; restart with next c when gcd equals value. Consume one global budget unit before each Floyd iteration. Proper factors split pending value; finally sort.

For pollard_rho the initial test incorrectly assumed n=9 needs more than one iteration; actual first iteration finds divisor three. The exhaustion witness was corrected to n=25,budget=1. Production logic was unchanged by that correction.

## model

# pollard_rho mod.rs model

## Domain

All u64 n and budgets, with zero invalid and maximum allowed budget 10000000. Result<Vec<u64>,FactorError> on normal execution.

## Forward

Reject n=0, then excessive budget. Traverse pending divisors: skip one; emit deterministic Miller-Rabin primes; split even composites for free; otherwise run x<-f(x),y<-f(f(y)), f(t)=t*t+c mod value, gcd(|x-y|,value). Start c=1 and x=y=2; restart with next c when gcd equals value. Consume one global budget unit before each Floyd iteration. Proper factors split pending value; finally sort.

## Invariant

Product of emitted primes and pending/current unresolved factors equals input. Every proper split strictly reduces values and adds at most one stack item. A u64 integer has <=63 factors for n>1. c<=budget+1<=10000001 so it cannot wrap. Iterates x<value<=u64::MAX; x*x+c fits u128 (including initial x=2). Zero budget never enters a rho iteration.

## Cost

At most budget Floyd iterations across splits and restarts, each three modular squarings and one Euclidean gcd. At most 127 pending-node visits for a u64 factor tree, each Miller-Rabin O(log(n+1)) word work. Conservative O((budget+64)*log(n+1)) word operations, O(log(n+1)) pending/output words. This is a hard work cap, not an expected success claim.

## nonbehavior

Type: behavior_complement. Excluded successes are unsorted lists, composite factors, missing multiplicity, or product unequal to n. Success on zero/excessive budget, unbounded rho execution, and partial factor vectors labeled successful are excluded.

## factor_fibers

Type: reverse_reconstruction. A successful prime multiset reconstructs n uniquely by its product (empty reconstructs one), but does not reconstruct the budget: any accepted larger budget than a successful one follows the same deterministic path and succeeds. BudgetExhausted covers many composite n and budgets and reveals no unique factor.

## product_certificate

Type: certificate_relation. Sorted primes with u128 product equal to input certify complete factorization by unique factorization. Checking factors with the production primality routine is only an invariant, so small fixtures additionally use independent trial factorization.

## budget_extension

Type: metamorphic_relation. If budget b succeeds and b<=b'<=MAX_STEPS, b' returns identical factors. Budget affects only stopping, never seeds or arithmetic. Failure at b makes no claim about success at b'.

## prime_power

Type: algebraic_relation. For n=p^k within u64 with prime p, successful output contains exactly k copies of p. Powers of two need no rho budget; odd powers may exhaust it. Prime inputs use no rho budget.

## collision_restart

Type: resource_relation. gcd=1 continues, gcd=value restarts with c+1, and 1<gcd<value splits. Every path through a collision or continuation consumes an iteration; all restarts share one decreasing counter, ensuring deterministic termination. No randomized success probability is claimed.

## Composition and gaps

The source is a small leaf; its validation, core arithmetic and result assembly compose under the invariant above. Tests observe the public operation plus any private helper explicitly named by tests. Partial syntax does not expand macros or provide a semantic AST. No useful certified approximate metric is supplied for exact discrete outputs; executable numerical approximants are an explicit gap. No performance benchmark or formal proof is claimed.

## evidence

### Evidence

Five tests: independent trial factorization n=1..4999; zero/cap/exhaustion, free prime/even cases; u64::MAX known factors, 3^40, 1000000007*1000000009, largest known prime below u64::MAX; seven deterministic replay/budget-extension cases; additive modular-product oracle for n=3..29,x<n,c=1..9.

Emit three on the even-composite split instead of two; boundaries_and_budget must fail on 2^63.

Syntax extraction is run twice and compared exactly; syntax.json is a partial Rust syntax projection, not semantic AST analysis. Root integration supplies registry validation; this branch does not modify the shared registry.

## prospective

### Groupant prospective

#### Prospective model

Domain n:u64 and budget<=10^7 Floyd iterations; zero invalid (before budget validation), one yields empty. Prime inputs require no rho budget. Reuse deterministic Miller-Rabin and Euclidean gcd, exact u128 square plus c modular arithmetic. Deterministic c restarts on collision, with global consumed iteration count; no guarantee of success within any budget. Each iteration performs three polynomial evaluations plus gcd; pending split stack and output have <=64 entries. Invariant product(pending)*product(primes)=original and each emitted factor is prime. O((budget+64)*log(n)) word-operation conservative bound including primality; O(log n) words. Complement excludes incomplete successful factors and hangs. Successful output reconstructs n uniquely by multiplication but not budget; exhausted output cannot identify n. Prime-power, product reconstruction, budget monotonic success, and deterministic replay specialize model.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: prospective model retained before implementation. Independent bounded oracles, extreme arithmetic fixtures and compiled intentional mutations test the claim. Five owned tests pass twice each in debug and release, with identical normalized records. Finite results do not prove all inputs.

### 2026-10-30 · evidence

Heading `### Evidence — 2026-10-30` stays in evidence as `### Evidence`.

Source SHA-256: `aa991534784f3474e089c09e7e75292922c5aa7472503158577b6c3f57f5ac28`. Five tests per algorithm pass twice in debug and twice in release; normalized records compared by a Rust driver. Strict library Clippy and owned-file formatting pass. One compiled production mutation per algorithm fails a named assertion and restored bytes pass the positive suite.

External evidence under `$MOVA_DATA/runs/octopus-wave18-numbers-2026-10-30/`:

- `pollard_rho-debug-1.log`: SHA-256 `9758cfd6078528c523f63a8f6c72a6b7ec1ebdec37387a29d1ead040046a2c14`.

- `pollard_rho-debug-2.log`: SHA-256 `598d3a802e75de03bb0c4041d4fdc2ab2e3665fd621138f0e54afe5979cce6fd`.

- `pollard_rho-release-1.log`: SHA-256 `64ef2510ce388035d3c0c867ba97cd62b37873a00126afebc5044d395ecba5b5`.

- `pollard_rho-release-2.log`: SHA-256 `c2cfed586585fe3197a7de263072894817e4c6775abe3c5662850e64d6f9c0dc`.

- `pollard_rho-mutant.log`: SHA-256 `f161e35f0867062da3e0f6365170332fe3505bd58bdc1f560d3cf5d0e89721c5`.

- `pollard_rho-restored.log`: SHA-256 `a9a907a93f31a6cd5e17d3f4303f37a20f05f256c1102ade91f760144b632baf`.

### 2026-10-30 · prospective

Heading `#### Prospective model — 2026-10-30` stays in prospective as `#### Prospective model`.

#### Module prospective

##### Prospective model — 2026-10-30

Domain n:u64 and budget<=10^7 Floyd iterations; zero invalid (before budget validation), one yields empty. Prime inputs require no rho budget. Reuse deterministic Miller-Rabin and Euclidean gcd, exact u128 square plus c modular arithmetic. Deterministic c restarts on collision, with global consumed iteration count; no guarantee of success within any budget. Each iteration performs three polynomial evaluations plus gcd; pending split stack and output have <=64 entries. Invariant product(pending)*product(primes)=original and each emitted factor is prime. O((budget+64)*log(n)) word-operation conservative bound including primality; O(log n) words. Complement excludes incomplete successful factors and hangs. Successful output reconstructs n uniquely by multiplication but not budget; exhausted output cannot identify n. Prime-power, product reconstruction, budget monotonic success, and deterministic replay specialize model.

### Undated · descriptant

Source SHA-256: `aa991534784f3474e089c09e7e75292922c5aa7472503158577b6c3f57f5ac28`.
