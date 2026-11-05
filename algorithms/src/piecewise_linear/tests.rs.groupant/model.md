# algorithm-piecewise-linear-tests

## descriptant

### piecewise_linear: tests.rs

Deterministic executable assertions for piecewise_linear; finite fixtures observe production outputs against independent masks or exact arithmetic. Passing is a finite evidence claim, not universal correctness.

## model

### Mathematical model

Deterministic executable assertions for piecewise_linear; finite fixtures observe production outputs against independent masks or exact arithmetic. Passing is a finite evidence claim, not universal correctness.

#### Invariant

Validation precedes binary lookup and arithmetic. h is positive and fits u64; checked i128 products/sum form an unreduced rational. Checked u128 M*h² bounds the conditional interpolation remainder. Endpoints return exact sample and zero error even when a generic interval certificate would overflow.

#### Cost and resources

O(n) validation and O(log n) binary search, O(1) storage, no allocation. Caller budget bounds validated knots; overflow rejects rather than wrapping. Work budget is not a wall-clock deadline. No approximate floating evaluation or relative error tier is advertised.

#### Observation limits

Integer certificates are scoped to successful calls and stated premises, with categorical validation, budget, allocation and overflow errors. usize targets up to64 bits are modeled. No floating approximation, timing benefit or universal error theorem inferred from tests. No retained selected subset witness.

#### Literature

Atkinson, An Introduction to Numerical Analysis, interpolation remainder theorem for degree1 polynomials.

#### Executable companion gap

The production kernel already realizes the scoped approximation. No separate cheaper Rust surrogate has been measured; no fake exact wrapper or speed claim is attached.

## approximant

### Mathematical model

Deterministic executable assertions for piecewise_linear; finite fixtures observe production outputs against independent masks or exact arithmetic. Passing is a finite evidence claim, not universal correctness.

#### Invariant

Validation precedes binary lookup and arithmetic. h is positive and fits u64; checked i128 products/sum form an unreduced rational. Checked u128 M*h² bounds the conditional interpolation remainder. Endpoints return exact sample and zero error even when a generic interval certificate would overflow.

#### Cost and resources

O(n) validation and O(log n) binary search, O(1) storage, no allocation. Caller budget bounds validated knots; overflow rejects rather than wrapping. Work budget is not a wall-clock deadline. No approximate floating evaluation or relative error tier is advertised.

#### Observation limits

Integer certificates are scoped to successful calls and stated premises, with categorical validation, budget, allocation and overflow errors. usize targets up to64 bits are modeled. No floating approximation, timing benefit or universal error theorem inferred from tests. No retained selected subset witness.

#### Literature

Atkinson, An Introduction to Numerical Analysis, interpolation remainder theorem for degree1 polynomials.

#### Executable companion gap

The production kernel already realizes the scoped approximation. No separate cheaper Rust surrogate has been measured; no fake exact wrapper or speed claim is attached.

## models

# Typed obernants: piecewise_linear tests.rs

## behavior_complement

Observation is a vector of named assertion outcomes. The pass fiber contains implementations agreeing on every exercised fixture; its complement contains at least one failing assertion, panic or error mismatch. A surviving mutant outside the tested domain can remain in the pass fiber. This relation explicitly leaves untested behavior unknown.

## reverse_reconstruction

All implementations producing the same finite assertion vector belong to one nonunique fiber. Original production and a mutant differing only on untested data may both pass; the compiled mutant (Increment interior rational denominator; affine reproduction assertion fails.) is separated by affine_exact. A pass vector cannot reconstruct source, runtime or behavior on unseen inputs.

## independent_oracle

Fixtures target the mathematical observation directly: At least two strictly increasing i64 abscissae with i64 ordinates and an integer in-domain query. Exact rational chord value N/h, where N=y_left*(x_right-x)+y_right*(x-x_left). Exact knot queries return ordinate/1. Under caller-supplied exact samples of a C² function on full domain with |f''|<=M, absolute error<=M*h²/8. Small mask enumeration is structurally independent of scaled/trimmed DP; exact integer formulas are independent of interpolation lookup and quantization branches. These finite witnesses falsify implementation claims without promoting sample agreement to proof.

## boundary_refusal_fiber

The validation/budget fixtures distinguish supported success from invalid tiers, dimensions, resource or overflow failures where relevant. Inputs sharing Error variants lose original details. Boundary assertions are executable witnesses of the broader production relation: absolute_metric_transport|If a downstream map is L-Lipschitz on the relevant range, interpolation absolute error e contributes <=Le. Relative error near f(x)=0 is unrestricted; this certificate never promises1/2/5/10% scalar relative accuracy.

## compiled_fault_separation

Increment interior rational denominator; affine reproduction assertion fails. The intentional production edit must compile and fail affine_exact; restored byte hash and positive rerun close the experiment. It only demonstrates sensitivity to this fault, not mutation completeness.

## tests

### Executable evidence

Run cargo test -p mova-algorithms piecewise_linear::tests (also --release). Module has 3 tests. Twice debug and twice release full library502-test runs have equal normalized test names/results. Strict all-target Clippy passed.

Compiled mutant: Increment interior rational denominator; affine reproduction assertion fails. Named assertion test: affine_exact. Production bytes restored and focused positive suite passed.

## prospective

Before implementation: knapsack uses exact rational profit scaling K=epsilon*vmax/n, minimum-weight DP, loss<=nK<=epsilon OPT. Subset sum uses rational trimmed sorted lists delta=epsilon/n and retains feasible lower representatives; (1+delta)^-n>=1-epsilon. Tiers exactly 1/2/5/10%, zeros exact. Checked work/state budgets and fallible allocation, no speed claims. Piecewise interpolation uses integer knots/query and exact rational values, conditional on caller's C² function, exact samples and |f''|<=M; absolute bound M*h²/8 only. Quantized dot uses integer truncation and exact absolute residual bound, normalized by sum|a_i b_i| with exact zero convention, never cancellation-sensitive scalar relative metric. All arithmetic checked. Tests compare exhaustive or independent arithmetic oracles, exercise invalid/overflow/budget boundaries and compiled production mutants.

## History

### 2026-10-30 · descriptant

#### Epoch2026-10-30

Prospective model was saved before implementation. Independent finite fixtures and exact integer arithmetic retain explicit domain boundaries. Initial cancellation test incorrectly expected10% rejection for a1.97% residual; corrected to1%. This was a fixture error, not a reason to weaken the certificate. Four full library runs passed502tests; focused module contains3tests. Compiled production mutation: Increment interior rational denominator; affine reproduction assertion fails. No live workload, end-to-end gain or universal numerical guarantee measured.

### 2026-10-30 · tests

External logs: $MOVA_DATA/runs/octopus-wave19-approx-2026-10-30. receipt.json binds file hashes. Syntax projections are generated independently twice and compared; partial syntax only, not semantic AST. Allocation refusal is not deterministically injected.

### Undated · document

Source SHA256: f084398af5644c3eec94cacb0614ba124e4e83509eb168f540296f3df8ce3fc9. Partial source-derived relations; finite checks are not proofs.
