# algorithm-integer-sqrt-rust-approximant

## descriptant

### Executable integer_sqrt approximant

Earlier exact observations remain in the parent. Expanded AST, formal proof and production evaluation remain absent.

## model

# Executable integer_sqrt model

## Forward

Let t=floor(log2(n)/2), k=7,6,5,4 for the four profiles, s=max(0,t-k), q=floor(sqrt(n >> 2s)), a=q*2^s. Integer checks certify q. Reduced n <=65535, so certification products fit u64. For R=floor(sqrt(n)), a<=R<(q+1)*2^s. If s=0, a=R. Otherwise q>=2^k and (R-a)/R<1/q<=2^-k, hence <=0.78125%,1.5625%,3.125%,6.25%. Zero returns zero. No minimum error is required.

## Invariant

All u64 inputs; zero is exact. Metric: abs(result-floor(sqrt(n)))/floor(sqrt(n)); zero input has zero error. Caps are upper bounds. Production source is unchanged; explicit standalone harness only.

## behavior_complement

For fixed b, exclude all (n,a) with a != F_b(n), where F is the deterministic certified-root expression above. In particular a>floor(sqrt(n)) and error exceeding b are impossible.

## reverse_reconstruction

For a,b, compatible inputs are the union over s in 0..31 with a divisible by 2^s, q=a/2^s, of [q^2*2^(2s),(q+1)^2*2^(2s)-1] intersect u64 and the shift-choice region s=max(0,floor(log2(n)/2)-k). Include n=0 when a=0. Set-valued preimage, not unique recovery.

## Cost

Constant space; one floating square root seed on integer <=65535 then integer certification. Prospective expectation: faster than unchanged binary search. Integration measured about 33x versus that reference and 3.36-3.38x versus std on the fixed batch; see docs/mathematical-modeling.md#executable-rust-approximants.

## Epoch notes

## evidence

Compile standalone with rustc --edition 2024 --test rust_approximant.rs --extern mova_approx=PATH_TO_RLIB, then execute. Runtime contract: tools/approx. Release timing belongs to the integrated harness; no speed factor claimed here.

The same tests passed on two executions. External compiled negative-control mutants were rejected: replacing the sqrt result with zero fails its differential bound test; unconditionally accepting sampled rank candidates fails both quickselect tests. Mutants were outside the repository and never changed production sources.

## History

### 2026-10-29 · descriptant

2026-10-29: real Rust companion with four budgets. The mathematical combiner connects source, domain, metric, runtime certificate and independent oracle. Source hashes alone establish no accuracy or speed claim.

2026-10-29, rustc 1.99.0: two tests pass. Four budgets on 0..100000 plus 100000 fixed-seed values each, square neighbors and extremes; independent u64::isqrt oracle; exact integer ratio comparison. u64::MAX witnesses nonexact execution.

### 2026-10-29 · evidence

2026-10-29, rustc 1.99.0: two tests pass. Four budgets on 0..100000 plus 100000 fixed-seed values each, square neighbors and extremes; independent u64::isqrt oracle; exact integer ratio comparison. u64::MAX witnesses nonexact execution.

### 2026-10-30 · evidence

#### Current cap epoch, 2026-10-30

Supported profiles are 1%, 2%, 5%, 10%; prior 15%/25% profiles are retired.
The two optimized standalone Rust tests passed twice with the updated shared
contract. Independent isqrt and integer loss checks cover every profile;
u64::MAX remains a nonexact witness for all four. A compiled zero-result mutant
failed the differential test. The analytical bounds are 0.78125%, 1.5625%,
3.125%, 6.25%; finite tests are supporting evidence, not their derivation.
Classification: scalar_relative / quantized_numeric / analytical.
Earlier timing observations belong to the retired profile configuration and
are not fresh measurements of the current profiles.

### 2026-11-04 · document

Migration: consolidated into model.md, 2026-11-04, operator decision; fresh hash-bound references to the former files were rebound to the sections below.

### 2026-11-04 · Epoch notes

- Comments repointed to consolidated docs, 2026-11-04.
