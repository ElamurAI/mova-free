# mlab-builtins-core-reshape

## descriptant

### Reshape dimension inference

Collect Some(cast dimension) or None for empty positional dimensions. known=product(Some); holes>1 errors. One hole fills n/known only if known!=0 and divides n. Then require at least two dimensions and trailing dimensions all one, require r*c=n, clone and replace shape.

## approximant

### Reshape dimension inference forward relation

Collect Some(cast dimension) or None for empty positional dimensions. known=product(Some); holes>1 errors. One hole fills n/known only if known!=0 and divides n. Then require at least two dimensions and trailing dimensions all one, require r*c=n, clone and replace shape.

Domain: Valid input Mat and bounded dimensions/products; empty dimension-vector versus positional empty are different syntaxes.

No executable Rust surrogate is supplied: categorical decisions require exact observations; numerical kernels have no measured approximation metric here. Future supported caps are 1%, 2%, 5%, 10% only.

## invariant

### Invariant and cost

numel preserved, flattened re/im exact. Casts use Rust f64→usize semantics; this code does not validate integral positive dimensions uniformly.

Cost: O(argument elements+input cells clone), O(input cells) output memory.

Failure ordering: The unchecked known-dimension product is computed before hole counting; after successful bounded arithmetic, validation checks hole count, known divisibility, rank/trailing dimensions and final product in that order. Overflow can precede those Result errors.

## models

## behavior_complement

Two holes exclude success before dimension-rank checks. Nondivisible known product excludes guessing rounded inferred size. Success excludes permutation or class/im change.

## reverse_reconstruction

Result plus prior rows/cols uniquely recovers prior Mat contents/shape on valid product; argument list remains many-valued through dimension-vector versus positional, trailing ones and one inferred hole.

## representation_transport

numel preserved, flattened re/im exact. Casts use Rust f64→usize semantics; this code does not validate integral positive dimensions uniformly.

## failure_precedence

The unchecked known-dimension product is computed before hole counting; after successful bounded arithmetic, validation checks hole count, known divisibility, rank/trailing dimensions and final product in that order. Overflow can precede those Result errors.

## resource_domain

Valid input Mat and bounded dimensions/products; empty dimension-vector versus positional empty are different syntaxes.

## interaction_relation

Consumes value::Mat and returns same column-major data, distinct from transpose permutation; parser/interpreter evaluate [] before this helper.


## tests

### Bounded evidence

Selected actual-code Rust test: `builtins::core::wave30::constructors` with `cargo test --offline --locked -p mlab --lib`. CPU synthetic fixtures only. Two debug and two release runs, sorted named outcomes, numeric statuses, compiled assertion controls and exact source restoration are retained externally under the wave30 expressions receipt directory. The source-to-test map distinguishes runtime semantics from AST representation checks. This fragment shares the parent test: only the exercised branches are observed; its whole prose is not runtime coverage. No corpus, GPU, live filesystem inspection, callbacks, RNG shuffle or wall-clock execution.

Syntax is generated twice from exact source and compared. Lexical projection has no expansion/type/name resolution. No strict Clippy claim; existing dependency/style findings are not re-run merely for repetition.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source reading precedes test additions. Retained observations: Consumes value::Mat and returns same column-major data, distinct from transpose permutation; parser/interpreter evaluate [] before this helper.

This is partial source-derived modeling, not proof, full semantic coverage, or a performance result. Public malformed representations, unbounded allocations and exhaustive numerical edge cases remain outside the checked fixtures.
