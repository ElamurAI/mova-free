# mlab-builtins-core-control

## descriptant

### Messages, warnings and assertion control

format_message treats first string as identifier only if colon-shaped and another argument exists; sprintf invoked for extra args or percent/backslash. error returns MError with id. warning toggles on/off (optionally all), query/error are no-op; otherwise emits via Interp.warn. assert checks truthiness or real numeric elements with exact/absolute/relative tolerance.

## approximant

### Messages, warnings and assertion control forward relation

format_message treats first string as identifier only if colon-shaped and another argument exists; sprintf invoked for extra args or percent/backslash. error returns MError with id. warning toggles on/off (optionally all), query/error are no-op; otherwise emits via Interp.warn. assert checks truthiness or real numeric elements with exact/absolute/relative tolerance.

Domain: Well-formed bounded Value inputs and output sink; no wall-clock or filesystem in this scope. Formatted output and assertion numeric paths not all tested.

No executable Rust surrogate is supplied: categorical decisions require exact observations; numerical kernels have no measured approximation metric here. Future supported caps are 1%, 2%, 5%, 10% only.

## invariant

### Invariant and cost

Positive tolerance absolute; negative tolerance scales |expected|; zero exact equality except paired NaN. Single condition uses ops::truthy nonempty-all semantics.

Cost: O(formatted text+observed numel); warning/error outputs delegated; callbacks absent in this scope.

Failure ordering: warning control strings before format_message; assert arity before string-message mode before numeric dimensions and element checks.

## models

## behavior_complement

warning off excludes message formatting/output on that branch. assert does not exclude imaginary mismatch because loop uses re only. Equal NaN real pairs exclude assertion failure irrespective of tolerance. Nonmatching dimensions excluded unless expected scalar.

## reverse_reconstruction

Error msg/id fibers include all compatible formatting args; original values generally lost. Final warning flag permits arbitrary prior state. Passing tolerance assertion gives a set of real observations in the expected-value interval, not exact inverse.

## representation_transport

Positive tolerance absolute; negative tolerance scales |expected|; zero exact equality except paired NaN. Single condition uses ops::truthy nonempty-all semantics.

## failure_precedence

warning control strings before format_message; assert arity before string-message mode before numeric dimensions and element checks.

## resource_domain

Well-formed bounded Value inputs and output sink; no wall-clock or filesystem in this scope. Formatted output and assertion numeric paths not all tested.

## interaction_relation

ops::truthy and io::sprintf_impl define delegated behavior; Interp carries warnings_on and output capture. Tests toggle only off/on without emitting text.


## tests

### Bounded evidence

Selected actual-code Rust test: `builtins::core::wave30::constructors` with `cargo test --offline --locked -p mlab --lib`. CPU synthetic fixtures only. Two debug and two release runs, sorted named outcomes, numeric statuses, compiled assertion controls and exact source restoration are retained externally under the wave30 expressions receipt directory. The source-to-test map distinguishes runtime semantics from AST representation checks. This fragment shares the parent test: only the exercised branches are observed; its whole prose is not runtime coverage. No corpus, GPU, live filesystem inspection, callbacks, RNG shuffle or wall-clock execution.

Syntax is generated twice from exact source and compared. Lexical projection has no expansion/type/name resolution. No strict Clippy claim; existing dependency/style findings are not re-run merely for repetition.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source reading precedes test additions. Retained observations: ops::truthy and io::sprintf_impl define delegated behavior; Interp carries warnings_on and output capture. Tests toggle only off/on without emitting text.

This is partial source-derived modeling, not proof, full semantic coverage, or a performance result. Public malformed representations, unbounded allocations and exhaustive numerical edge cases remain outside the checked fixtures.
