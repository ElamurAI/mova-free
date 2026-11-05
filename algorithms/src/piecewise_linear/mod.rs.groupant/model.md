# algorithm-piecewise-linear

## descriptant

### piecewise_linear: mod.rs

At least two strictly increasing i64 abscissae with i64 ordinates and an integer in-domain query. Exact rational chord value N/h, where N=y_left*(x_right-x)+y_right*(x-x_left). Exact knot queries return ordinate/1. Under caller-supplied exact samples of a C² function on full domain with |f''|<=M, absolute error<=M*h²/8.

## model

### Mathematical model

At least two strictly increasing i64 abscissae with i64 ordinates and an integer in-domain query. Exact rational chord value N/h, where N=y_left*(x_right-x)+y_right*(x-x_left). Exact knot queries return ordinate/1. Under caller-supplied exact samples of a C² function on full domain with |f''|<=M, absolute error<=M*h²/8.

#### Invariant

Validation returns the first failing check in order: WorkBudget when n exceeds the budget; TooFewKnots when n<2; Unordered for non-increasing abscissae; OutsideDomain for an out-of-range query. Binary lookup follows these gates; only interior checked arithmetic can then return Overflow. h is positive and fits u64; checked i128 products/sum form an unreduced rational. Checked u128 M*h² bounds the conditional interpolation remainder. Endpoints return exact sample and zero error even when a generic interval certificate would overflow.

#### Cost and resources

O(n) validation and O(log n) binary search, O(1) storage, no allocation. Caller budget bounds validated knots; overflow rejects rather than wrapping. Work budget is not a wall-clock deadline. No approximate floating evaluation or relative error tier is advertised.

#### Observation limits

Integer certificates are scoped to successful calls and stated premises, with categorical validation, work-budget and arithmetic-overflow errors. The kernel allocates no storage and exposes no allocation-error variant. usize targets up to64 bits are modeled. No floating approximation, timing benefit or universal error theorem inferred from tests. No retained selected subset witness.

#### Literature

Atkinson, An Introduction to Numerical Analysis, interpolation remainder theorem for degree1 polynomials.

#### Executable companion gap

The production kernel already realizes the scoped approximation. No separate cheaper Rust surrogate has been measured; no fake exact wrapper or speed claim is attached.

## approximant

### Mathematical model

At least two strictly increasing i64 abscissae with i64 ordinates and an integer in-domain query. Exact rational chord value N/h, where N=y_left*(x_right-x)+y_right*(x-x_left). Exact knot queries return ordinate/1. Under caller-supplied exact samples of a C² function on full domain with |f''|<=M, absolute error<=M*h²/8.

#### Invariant

Validation returns the first failing check in order: WorkBudget when n exceeds the budget; TooFewKnots when n<2; Unordered for non-increasing abscissae; OutsideDomain for an out-of-range query. Binary lookup follows these gates; only interior checked arithmetic can then return Overflow. h is positive and fits u64; checked i128 products/sum form an unreduced rational. Checked u128 M*h² bounds the conditional interpolation remainder. Endpoints return exact sample and zero error even when a generic interval certificate would overflow.

#### Cost and resources

O(n) validation and O(log n) binary search, O(1) storage, no allocation. Caller budget bounds validated knots; overflow rejects rather than wrapping. Work budget is not a wall-clock deadline. No approximate floating evaluation or relative error tier is advertised.

#### Observation limits

Integer certificates are scoped to successful calls and stated premises, with categorical validation, work-budget and arithmetic-overflow errors. The kernel allocates no storage and exposes no allocation-error variant. usize targets up to64 bits are modeled. No floating approximation, timing benefit or universal error theorem inferred from tests. No retained selected subset witness.

#### Literature

Atkinson, An Introduction to Numerical Analysis, interpolation remainder theorem for degree1 polynomials.

#### Executable companion gap

The production kernel already realizes the scoped approximation. No separate cheaper Rust surrogate has been measured; no fake exact wrapper or speed claim is attached.

## models

# Typed obernants: piecewise_linear mod.rs

## behavior_complement

For validated representable inputs the forward relation is constrained by: Validation returns the first failing check in order: WorkBudget when n exceeds the budget; TooFewKnots when n<2; Unordered for non-increasing abscissae; OutsideDomain for an out-of-range query. Binary lookup follows these gates; only interior checked arithmetic can then return Overflow. h is positive and fits u64; checked i128 products/sum form an unreduced rational. Checked u128 M*h² bounds the conditional interpolation remainder. Endpoints return exact sample and zero error even when a generic interval certificate would overflow. Its complement includes output pairs violating these bounds, unsupported acceptance, out-of-domain values and untyped overflow. Categorical errors are outside numerical approximation, with validation precedence part of the relation.

## reverse_reconstruction

A rational output N/h constrains a weighted sum of neighboring ordinates. Infinitely many underlying C² functions have the same samples and midpoint while respecting sufficiently large curvature. Querying exact knot0,y0 admits different neighboring knots. Even with fixed samples, adding c*(x-a)*(x-b) changes interior values while preserving endpoint data.

## curvature_assumption_fiber

For an interval[a,b], the family f=chord+c*(x-a)*(x-b) shares endpoint samples and has |f''|=2|c|. At midpoint its error is |c|h²/4=M*h²/8 when M=2|c|, proving sharpness under that premise. Samples alone cannot bound c.

## affine_reproduction

For y=mx+b with integer samples, curvature0 gives exact N/h=mx+b at every integer query. The affine fixture spans negative and positive x; rational1/4 checks that results are not rounded to integers.

## domain_partition

WorkBudget, TooFewKnots, Unordered and OutsideDomain partition first validation failures in that order; Overflow can arise only from subsequent interior arithmetic. No allocation failure is part of this API's error enum. Extrapolation is excluded. Exact-knot fiber bypasses interior arithmetic after full validation, retaining exact endpoints at extreme i64 values.

## absolute_metric_transport

If a downstream map is L-Lipschitz on the relevant range, interpolation absolute error e contributes <=Le. Relative error near f(x)=0 is unrestricted; this certificate never promises1/2/5/10% scalar relative accuracy.
## budget_masked_validation

Domain: a fixed finite knot slice K, integer query x and curvature M, with only the caller's usize work budget B varied. Hold source revision and all input values fixed. Let n=|K|. For B<n, the result is WorkBudget before any shape, ordering, domain or arithmetic check. For B>=n, let V(K,x,M) be the result of the remaining source decision tree; the result equals V and is independent of further increases in B.

This counterfactual budget relation distinguishes refusal to inspect from evidence of invalid interpolation data. For K=[(0,0)], x=0 and M=0, B=0 selects WorkBudget while B=1 selects TooFewKnots. Raising the budget can expose a different error without changing the data. It does not imply successful interpolation or a wall-clock bound. Assumptions: ordinary Rust execution and the declared integer domain; no callback or allocation branch exists. This is a source-derived branch relation. Existing named tests cover individual errors; no new combined-failure test or runtime verification is claimed for this witness.

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

### 2026-10-31 · descriptant

#### Source review 2026-10-31

Corrected validation precedence and removed the nonexistent allocation-error branch from current models. The budget-masked validation model records a source-derived intervention, not newly executed evidence. Historical prospective notes and test receipts remain unchanged.

### Undated · document

Source SHA256: 83e21dc6ede1e8cfb9c397eb4e3eea7050646fbac380a74a5f9932f703adbe5c. Partial source-derived relations; finite checks are not proofs.
