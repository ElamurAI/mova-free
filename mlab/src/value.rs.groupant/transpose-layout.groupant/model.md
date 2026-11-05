# mlab-value-transpose-layout

## descriptant

### Matrix transpose permutation

For r×c matrix, T.re[i*c+j]=M.re[j*r+i], same for optional im; output shape c×r and identical class.

## approximant

### Matrix transpose permutation forward relation

For r×c matrix, T.re[i*c+j]=M.re[j*r+i], same for optional im; output shape c×r and identical class.

Domain: Valid buffers and bounded dimension product; no checked allocation interface.

No executable Rust surrogate is supplied: categorical decisions require exact observations; numerical kernels have no measured approximation metric here. Future supported caps are 1%, 2%, 5%, 10% only.

## invariant

### Invariant and cost

Transpose twice restores dimensions, values, class and optional im. Empty shapes swap even without cells.

Cost: O(r*c) time/new storage for each present component.

Failure ordering: Real buffer allocated/permuted before optional imaginary buffer; no narrow call.

## models

## behavior_complement

For consistent lengths T excludes conjugation, dropping im, or unchanged flattened order on nonsymmetric rectangular input.

## reverse_reconstruction

T^-1(Y)={Y.transpose()} for well-formed buffers and recorded class/im representation. No uniqueness of original allocation identity.

## representation_transport

Transpose twice restores dimensions, values, class and optional im. Empty shapes swap even without cells.

## failure_precedence

Real buffer allocated/permuted before optional imaginary buffer; no narrow call.

## resource_domain

Valid buffers and bounded dimension product; no checked allocation interface.

## interaction_relation

ops::transpose applies optional conjugation after this permutation; core reshape instead changes shape only.


## tests

### Bounded evidence

Selected actual-code Rust test: `value::wave30::layout` with `cargo test --offline --locked -p mlab --lib`. CPU synthetic fixtures only. Two debug and two release runs, sorted named outcomes, numeric statuses, compiled assertion controls and exact source restoration are retained externally under the wave30 expressions receipt directory. The source-to-test map distinguishes runtime semantics from AST representation checks. This fragment shares the parent test: only the exercised branches are observed; its whole prose is not runtime coverage. No corpus, GPU, live filesystem inspection, callbacks, RNG shuffle or wall-clock execution.

Syntax is generated twice from exact source and compared. Lexical projection has no expansion/type/name resolution. No strict Clippy claim; existing dependency/style findings are not re-run merely for repetition.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source reading precedes test additions. Retained observations: ops::transpose applies optional conjugation after this permutation; core reshape instead changes shape only.

This is partial source-derived modeling, not proof, full semantic coverage, or a performance result. Public malformed representations, unbounded allocations and exhaustive numerical edge cases remain outside the checked fixtures.
