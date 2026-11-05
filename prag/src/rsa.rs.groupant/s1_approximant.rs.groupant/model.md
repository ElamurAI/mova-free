# rsa-s1-rust-approximant

## descriptant

# Bounded pragmatic-speaker S1 approximant

## Forward

Domain: 1..=8 worlds and utterances, rectangular truth [utterance][world], priors 1..=8, costs 0..=4, alpha 1..=2. The API infers dimensions from prior/truth and requires one cost per utterance. Output is [world][utterance]. Unsupported inputs return typed errors before arithmetic.

Classification is probability_relative / floating_normalization / analytical. Metric: maximum positive-cell relative error, fractional cap 1e-12 (1e-10 percent); reference-zero cells must remain exactly zero. This describes numerical S1 cells only.

## Exact reference arithmetic

For a supported utterance d[u] is an integer at most 64. A reduced L0 numerator is at most 8. L0 normalization adds at most eight integers and divides by a sum at most 64; its intermediates are safe. Powers use alpha<=2 and the exact cost divisor at most 2^8. Each unnormalized S1 term has reduced numerator <=64 and denominator <=64^2*2^8=2^20.

Let D=(product over utterances of d[u]^alpha)*2^max(alpha*cost[u]), assigning d=1 to empty support. Then D<=64^16*2^8=2^104 and every reduced term denominator divides D. At every partial positive row sum, the reduced denominator divides D and its numerator is at most 8D<=2^107: each term is <=1. In Ratio.add the intermediate denominator is the LCM, hence <=D; each scaled numerator and their sum are <=8D. This controls intermediates, not merely the final reduced fraction.

For final term/sum division, even BEFORE cross-cancellation, numerator <=64*D<=2^110 and denominator <=2^20*8D<=2^127. Both are strictly below 2^128; gcd cancellation only reduces them. Zero terms also satisfy these bounds, and zero rows bypass division. Therefore the actual Game::s1 reference cannot overflow on the entire declared domain. No claim applies to L1, best_world, explain, percentage formatting, or arbitrary Ratio comparisons.

## Floating error derivation

Assume IEEE binary64 round-to-nearest, ordinary operations without fast-math. Exact integer conversions cover all support sums and priors. Write u=2^-53. The first division has relative factor in [1-u,1+u]; squaring when alpha=2 duplicates that factor and adds at most one rounding, so each positive term has factor in [(1-u)^3,(1+u)^3]. Division by a power of two is exact here. Exact positive terms are between 2^-20 and 1, so none of these operations underflows or overflows.

The positive row sum includes at most seven nontrivial rounding additions. Its error factor is bounded by [(1-u)^10,(1+u)^10], by positivity and the term bounds. The final division therefore has factor between (1-u)^4/(1+u)^10 and (1+u)^4/(1-u)^10. Each differs from one by less than 2e-15, well below 1e-12. Normalized positive cells are at least 2^-23, so the normal-number error model remains applicable. Exact zeros stay zero through explicit false-cell handling and zero-row bypass; dividing zero by a positive sum is exact.

Converting reference u128 numerator and denominator to binary64 and dividing introduces at most three additional unit-roundoff factors. Their sizes remain normal. The measured comparison still lies far inside the conservative 1e-12 cap. Finite differential observations validate code against the reference; they are not the source of this domain-wide analytical argument.

## Invariant

Dimensions transpose from truth to world rows. Output is finite, nonnegative, and zero exactly where truth is false; unsupported world rows remain all zero. Every supported row sums to one within 1e-12. Normalization and positive sums ensure cells <=1. Each active profile meets its configured cap. Production sources and callers remain unchanged.

## Behavior complement

Accepted inputs cannot produce a negative or nonfinite cell, a nonzero false cell, positive mass on an unsupported row, a wrong matrix shape, or positive-cell relative error above the scoped cap under the stated arithmetic assumptions. Invalid domains cannot produce successful approximations. Categorical decisions, parameter training and end-task accuracy are not modeled.

## Reverse reconstruction

For fixed truth, supported world w, alpha and costs, compatible exact outputs obey S1[u|w]/S1[v|w]=(d[v]/d[u])^alpha * 2^(-alpha*(cost[u]-cost[v])) for true u,v: the world prior cancels in this within-world ratio. Denominators d are subset sums of a shared bounded positive integer prior vector. Compatible inputs are the intersection of these constraints across worlds, the bounds and the observed zero pattern. For approximate output, replace exact cell equalities with intervals induced by the 1e-12 forward cap. This defines an actual preimage relation; common prior scaling and common cost shifts within bounds may preserve the output, so unique recovery is not claimed.

## Cost

Validation, support sums, term construction and normalization take O(UW) time with O(UW+U) allocated storage. The exact kernel repeatedly reduces u128 fractions; the surrogate replaces GCD work with bounded binary64 operations. A lower constant factor is a prospective hypothesis; timing in the common runner is finite evidence, not a universal speed guarantee. No exact fallback is needed within this domain.

## Composition

The parent source groupant now attaches separate L0 and S1 models. S1 computes its own bounded L0 values and propagates their error through the power and normalization above. It does not call or silently widen the earlier L0 companion. The source is compiled directly into differential tests, including the original reference tests. L1 and all downstream decisions remain explicit gaps. The sidecar models the whole new Rust file; syntax projection is partial, with no macro expansion or semantic AST.

## Epoch memory

## measurements

### S1 focused validation

The new differential checks cover all 512 truth matrices at 3x3 for both alpha values with priors [1,3,8] and costs [0,2,4], all 64 world/utterance shapes with 16 deterministic mixed fixtures each, and six extreme/zero fixtures. Each fixture checks all four active profiles against actual Game::s1, checks positive-cell relative error <=1e-12 with exact zeros, and verifies supported row sums. These are finite observations, separate from the analytical domain proof.

Negative controls reject NaN, infinity, a negative cell, a 50% changed positive cell, nonzero mass in a false cell, and wrong output shape for every profile. Invalid-domain checks cover each typed error. The embedded source comparison uses no reimplementation of the exact kernel.

The common runner supplies separate timing evidence; no timing or end-task improvement is claimed by these tests.

## History

### 2026-10-30 · Forward

Prospective model, 2026-10-30: actual Game::s1 computes L0[u,w]=truth[u,w]*prior[w]/d[u], then t[w,u]=L0[u,w]^alpha/2^(alpha*cost[u]), and divides each world row by its sum. Empty support remains zero. The surrogate performs these same operations with exact integer support sums, binary64 division, an optional square, exact binary scaling, and positive row normalization. Four profiles (1%, 2%, 5%, 10%) share this tighter implementation.

### 2026-10-30 · Epoch memory

2026-10-30: direct bounded S1 selected after inspecting production Ratio intermediates. The common-denominator argument and raw final-division bounds establish reference safety without an empirical overflow assumption. Existing L0 implementation and evidence remain intact. No training, GPU or whole-system quality claim follows.

### 2026-10-30 · measurements

2026-10-30: direct Rust test compilation of s1_approximant.rs passed all 11 tests in debug and optimized (-O) builds. Seven are included original production RSA tests, four are new S1 checks.

### 2026-11-04 · document

Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.
