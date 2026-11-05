# math-comb

## descriptant

### comb

factorial uses balanced range products with empty product1; binomial applies negative-k zero and C(n,k)=(-1)^k C(k-n-1,k) for negative n, then product ratios. Legendre checks use nt sieve. Stirling II descends j with j*S(i-1,j)+S(i-1,j-1), I uses (i-1)*c(i-1,j)+c(i-1,j-1). Bell triangle, signed pentagonal partitions, coin DP, descending distinct/ascending odd-part counts, derangements seeds(1,0), and Fibonacci doubling (a(2b-a),a²+b²) produce Z values with alternate checks.

## approximant

### Forward and cost

factorial uses balanced range products with empty product1; binomial applies negative-k zero and C(n,k)=(-1)^k C(k-n-1,k) for negative n, then product ratios. Legendre checks use nt sieve. Stirling II descends j with j*S(i-1,j)+S(i-1,j-1), I uses (i-1)*c(i-1,j)+c(i-1,j-1). Bell triangle, signed pentagonal partitions, coin DP, descending distinct/ascending odd-part counts, derangements seeds(1,0), and Fibonacci doubling (a(2b-a),a²+b²) produce Z values with alternate checks.

Product trees O(n) big multiplies/O(log n) recursion; binomial O(min(k,n-k)) nonnegative case after guards. StirlingII O(nk)/O(k) storage, I/Bell O(n²)/O(n). Pentagonal O(n sqrt n)/O(n); coin/distinct O(n²)/O(n), partitions_k O(nk) time/space. Fibonacci O(log n) growing-width steps, derangements O(n). Legendre sieve O(n log log n) plus exponentiation, bit costs additional.

No new executable Rust surrogate: exact values, branching, domains and effects require an observation-specific metric before supported 1/2/5/10% profiles. Existing floating diagnostics are not such a profile; no speed claim.

## invariant

### Domain and invariant

Factorial and selected product work cap1e6; Stirling n<=3000, Bell<=2000/check<=400, partitions<=200000/DP<=3000, partitions_k/distinct<=5000, derangements<=100000/sum<=3000. binomial converts k before k>n/symmetry; Stirling n gate precedes k>n. Catalan unchecked2*n,n+1 and direct helper dimensions lack universal machine-overflow protection. Big values still require allocation.

factorial uses balanced range products with empty product1; binomial applies negative-k zero and C(n,k)=(-1)^k C(k-n-1,k) for negative n, then product ratios. Legendre checks use nt sieve. Stirling II descends j with j*S(i-1,j)+S(i-1,j-1), I uses (i-1)*c(i-1,j)+c(i-1,j-1). Bell triangle, signed pentagonal partitions, coin DP, descending distinct/ascending odd-part counts, derangements seeds(1,0), and Fibonacci doubling (a(2b-a),a²+b²) produce Z values with alternate checks.

A Z count has many compatible function/index antecedents: 0!=1!=1, D0=D2=1. Known function and admitted domain constrain recurrence fibers but generally do not invert uniquely. Cassini alone admits more pairs than intended Fibonacci values; unavailable checks impose no value constraint.

## composition

### Composition

factorial uses balanced range products with empty product1; binomial applies negative-k zero and C(n,k)=(-1)^k C(k-n-1,k) for negative n, then product ratios. Legendre checks use nt sieve. Stirling II descends j with j*S(i-1,j)+S(i-1,j-1), I uses (i-1)*c(i-1,j)+c(i-1,j-1). Bell triangle, signed pentagonal partitions, coin DP, descending distinct/ascending odd-part counts, derangements seeds(1,0), and Fibonacci doubling (a(2b-a),a²+b²) produce Z values with alternate checks.

calc dispatches to comb/nt. poly/linsys consume calc E and stateful Env for constants; deriv consumes E and calc::approx. fcheck consumes separate legacy expr::Expr/units, with no implicit lossless E adapter. Q/Z joins use big. Parent symbolic-computation models these edges; local checks do not certify application wiring. Nested scopes reuse parent syntax, and remaining helpers/types/tests are parent-modeled.

## models

# Typed models

## behavior_complement

For fixed admitted input/state, exclude outputs and effect traces contradicting this actual relation: factorial uses balanced range products with empty product1; binomial applies negative-k zero and C(n,k)=(-1)^k C(k-n-1,k) for negative n, then product ratios. Legendre checks use nt sieve. Stirling II descends j with j*S(i-1,j)+S(i-1,j-1), I uses (i-1)*c(i-1,j)+c(i-1,j-1). Bell triangle, signed pentagonal partitions, coin DP, descending distinct/ascending odd-part counts, derangements seeds(1,0), and Fibonacci doubling (a(2b-a),a²+b²) produce Z values with alternate checks. These exclusions are conditional on the stated domain. Unavailable checks, passing residuals and resource limits do not establish global correctness.

## reverse_reconstruction

A Z count has many compatible function/index antecedents: 0!=1!=1, D0=D2=1. Known function and admitted domain constrain recurrence fibers but generally do not invert uniquely. Cassini alone admits more pairs than intended Fibonacci values; unavailable checks impose no value constraint.

## product_identity

lo>hi gives1; negative k gives0 before conversion. Generalized negative n uses parity sign.

## recurrence_direction

Descending Stirling indices preserve previous row. Distinct sum descends to forbid reuse; odd-parts sum ascends to allow reuse.

## limit_precedence

stirling2 rejects n>3000 before k>n. Oversized k conversion may fail before useful symmetry. factorial_legendre has no factorial cap.

## certificate_availability

check_bell beyond400 and binomial outside independent domain return ok=true with unavailability text, not independent proof.

## tests

### Actual-code evidence

Each selected named test passed twice debug/twice release, sorted outcomes identical:

- `test comb::tests::classic_values ... ok`
- `test comb::wave28::zero_seeds_and_limit_precedence ... ok`

Finite synthetic fixtures only; no teacher/API/corpus/GPU/heldout. No allocation/stack faults, huge sizes, deadline proof, or universal numerical bound.

Final validation: eight compiled mutants total (seven module boundaries plus linsys width guard), all compile 0 / test 101 and exact restoration / positive 0. Old-code count mismatch witness compile 0 / test 101 separately. Full-registry structural run reports only ancestor integration bindings; isolated source snapshots validate each owned sidecar. Strict Clippy: dependency invocation blocked in existing global build script; --no-deps reaches math and reports 113 existing library findings, retained externally, no clean lint claim. Added test blocks formatted independently; production layout preserved.

## History

### 2026-10-31 · descriptant

Epoch 2026-10-31: complete source read before edits, bounded actual-code tests and source controls. Linsys shape defect was witnessed on old code and repaired; other semantics retained. External receipt name octopus-wave28-mathcore-2026-10-31 includes prospective.md. Registration is partial, not verification.

### 2026-10-31 · tests

Heading `### Actual-code evidence 2026-10-31` stays in tests as `### Actual-code evidence`.

Compiled source control: control-comb.patch.txt; compiler exit0, test nonzero, exact source restoration and focused positive exit0. Full logs and numeric exits retained externally in octopus-wave28-mathcore-2026-10-31. Linsys also retains shape-old compile 0 / test 101 witness before shape repair. Seven module controls plus the additional linsys width-guard control; fragment declarations are not extra tests.
