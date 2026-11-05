# mlab-ops-finite-range

## descriptant

### Finite range count and endpoint

Zero step, any NaN input, or wrong sign direction yields n=0; zero endpoints are valid. Otherwise span=(b-a)/s; n=floor(span+3eps*max(abs(span),1))+1; last=a+(n-1)s snaps to b if close. at(k) uses start for first half and last for second half.

## approximant

### Finite range count and endpoint forward relation

Zero step, any NaN input, or wrong sign direction yields n=0; zero endpoints are valid. Otherwise span=(b-a)/s; n=floor(span+3eps*max(abs(span),1))+1; last=a+(n-1)s snaps to b if close. at(k) uses start for first half and last for second half.

Domain: Finite span safely below usize::MAX, allocatable n, representable arithmetic; no percentage accuracy promise.

No executable Rust surrogate is supplied: categorical decisions require exact observations; numerical kernels have no measured approximation metric here. Future supported caps are 1%, 2%, 5%, 10% only.

## invariant

### Invariant and cost

at requires k<n; last is exactly b when snap criterion holds, not universally. range uses first element of arguments; empty supplies NaN and default step is 1.

Cost: range_spec O(1), at O(1), range_values O(n) time/storage.

Failure ordering: range converts first a then b then optional s; NaN/zero/direction gate before span, endpoint snap after count.

## models

## behavior_complement

Bounded valid at excludes overshooting by naive cumulative addition; zero-step range excludes elements. Infinite span is not guaranteed to return Result/error: saturating cast plus addition can overflow.

## reverse_reconstruction

Observed nonempty vector constrains endpoints/step only through rounded formulas; singleton vector permits many steps. Empty fiber includes zero step, NaN inputs and wrong-direction triples.

## representation_transport

at requires k<n; last is exactly b when snap criterion holds, not universally. range uses first element of arguments; empty supplies NaN and default step is 1.

## failure_precedence

range converts first a then b then optional s; NaN/zero/direction gate before span, endpoint snap after count.

## resource_domain

Finite span safely below usize::MAX, allocatable n, representable arithmetic; no percentage accuracy promise.

## interaction_relation

VM can use RangeSpec lazily while core colon/materialized expressions allocate Vec. Char class preserved only both endpoints char and implicit step.


## tests

### Bounded evidence

Selected actual-code Rust test: `ops::wave30::arithmetic` with `cargo test --offline --locked -p mlab --lib`. CPU synthetic fixtures only. Two debug and two release runs, sorted named outcomes, numeric statuses, compiled assertion controls and exact source restoration are retained externally under the wave30 expressions receipt directory. The source-to-test map distinguishes runtime semantics from AST representation checks. This fragment shares the parent test: only the exercised branches are observed; its whole prose is not runtime coverage. No corpus, GPU, live filesystem inspection, callbacks, RNG shuffle or wall-clock execution.

Syntax is generated twice from exact source and compared. Lexical projection has no expansion/type/name resolution. No strict Clippy claim; existing dependency/style findings are not re-run merely for repetition.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source reading precedes test additions. Retained observations: VM can use RangeSpec lazily while core colon/materialized expressions allocate Vec. Char class preserved only both endpoints char and implicit step.

This is partial source-derived modeling, not proof, full semantic coverage, or a performance result. Public malformed representations, unbounded allocations and exhaustive numerical edge cases remain outside the checked fixtures.
