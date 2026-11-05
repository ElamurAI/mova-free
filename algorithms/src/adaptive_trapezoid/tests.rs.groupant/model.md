# algorithm-adaptive-trapezoid-tests

## descriptant

### adaptive_trapezoid: tests.rs

Original std-only Rust. References: Neville/Newton: Stoer and Bulirsch, Introduction to Numerical Analysis, interpolation chapter; spline: de Boor, A Practical Guide to Splines; trapezoid: Davis and Rabinowitz, Methods of Numerical Integration; tanh-sinh: Takahasi and Mori (1974), Double exponential formulas for numerical integration. No copied implementation.

#### Prospective epoch (before implementation)

Before implementation: finite strictly increasing nonempty interpolation nodes, matched finite ordinates; spline at least two. Neville triangular O(n^2) time/O(n) memory, P_i,j=((x_j-x)P_i,j-1+(x-x_i)P_i+1,j)/(x_j-x_i). Newton backward divided differences O(n^2)/O(n), nested product evaluation O(n). Natural spline Thomas solve for second derivatives, zero endpoint curvatures, O(n) construction and binary interval evaluation. Reject nonfinite intermediate states and undefined spacing; no conditioning promise. Quadrature endpoints strictly ordered finite, finite positive tolerance, finite span; callbacks finite, deterministic and resource bounded by caller. Adaptive trapezoid dyadically doubles grid reusing samples, /3 consecutive difference heuristic; bounded evaluations and stagnation. Tanh sinh fixed transformed truncation [-3,3], dyadic h, samples strictly inside endpoints in ascending transformed order per level, no reuse; error is consecutive difference only and excludes truncation error. Both no arbitrary-function accuracy bound: zero and unsampled bump share sample trace. Expected gain: Neville O(n) storage versus full tableau, Newton O(n) repeated evaluation, spline linear solve; no measured speed claim or executable approximation percentage. Validation independent polynomial/analytic fixtures, domain/overflow/status/count and alias witnesses; compiled production mutants and repeats.


#### Mathematical combiner

Raw source and partial syntax anchor the f64 transition F; forward model predicts F, typed views describe its excluded pairs and preimage fibers; independent tests evaluate finite projections of F. Their conjunction narrows compatible implementations but is not a proof. No semantic AST resolution or macro expansion. Leaf children=[]; crate wiring is parent-owned.

## model

## behavior_complement

Observed Pass excludes implementations violating any asserted result on these fixtures, including compiled mutant: Change retained coarse contribution from 0.5 to 0.25; analytic_oracle assertion fails. Pass does not exclude bugs outside tested cases; it is not a universal theorem.

## reverse_reconstruction

Pass has many preimages: exact implementations and implementations agreeing only on this finite fixture set. Untested q=12345 behavior can change while all assertions pass. Zero and indicator bump in (0.1,0.2) agree at 0,1,0.5 and return identical zero reports after three calls although true integrals differ by 0.1. Finite callback trace identifies only sampled values, not smoothness or integral.

## oracle_independence

Expected values use analytic monomials, antiderivatives or explicit spline cubic, independently of production recurrence. analytic_oracle compares powers 0..4 and sine to exact integrals; budget_and_order checks exact [0,1,.5,.25,.75], count 5, value .34375 and budget=2; failure_and_alias checks no invalid calls, early nonfinite abort, overflow, adjacent-float stagnation and hidden bump.

## mutation_sensitivity

Change retained coarse contribution from 0.5 to 0.25; analytic_oracle assertion fails.

## finite_scope

Pass records only finite fixtures, not all f64 values or callbacks. Compile errors do not count as killed mutants; production mutation must compile and fail named assertion, then exact original bytes restored and rerun.

## approximant

### Forward model and invariant

Finite test transition maps fixture implementations to pass or assertion panic. analytic_oracle compares powers 0..4 and sine to exact integrals; budget_and_order checks exact [0,1,.5,.25,.75], count 5, value .34375 and budget=2; failure_and_alias checks no invalid calls, early nonfinite abort, overflow, adjacent-float stagnation and hidden bump.

Cost: O(evaluations) scalar work plus two geometry loops, O(1) storage. Callback resources and panics excluded; no calls beyond usize budget. Refinement doubling checked.

Invariants: successful reported scalars are finite; invalid-domain cases do not become approximate answers. Callback budgets and order are observable semantics. Real polynomial reproduction and quadrature convergence statements do not certify f64 accuracy. Determinism presumes deterministic callback. No executable surrogate profile: cancellation/conditioning and unobserved samples prevent a useful general 1/2/5/10% certificate. No gain or percentage promise.

## tests

analytic_oracle compares powers 0..4 and sine to exact integrals; budget_and_order checks exact [0,1,.5,.25,.75], count 5, value .34375 and budget=2; failure_and_alias checks no invalid calls, early nonfinite abort, overflow, adjacent-float stagnation and hidden bump.

Control: Change retained coarse contribution from 0.5 to 0.25; analytic_oracle assertion fails.

## History

### 2026-10-30 · descriptant

#### Observed epoch 2026-10-30

15 focused tests pass twice debug and twice optimized release with identical normalized outcomes. Five compiled production mutants fail oracle assertions; original bytes restored and positives rerun. No benchmark or speed claim. Finite arithmetic, allocations and callback limitations remain.

### 2026-10-30 · tests

External evidence: $MOVA_DATA/runs/octopus-wave25-numerical-2026-10-30; repeated focused harness and strict Clippy.

### 2026-11-01 · descriptant

#### Observed epoch 2026-11-01

A source-linked probe confirmed the hidden-bump report fiber and a sampled-bump negative control. The test itself asserts equality of reports, not their exact trace. This separates the test's observation from the stronger probe observation and from the analytic integral. The finite passing fiber and untested behaviors remain open.

### 2026-11-01 · document

#### Observed report fiber, 2026-11-01

For this fixture let Z(x)=0 and H(x)=1 on (0.1,0.2), zero elsewhere. With [0,1], tolerance 1e-10 and budget 10, the actual API samples 0, 1, 0.5 and returns the same zero report for both. Yet their Riemann integrals are 0 and 0.1. The test asserts report equality only; the source-linked probe independently checks the trace. The report fiber therefore contains different integrals, without implying that all callbacks with the same integral have the same report. As a falsifying control, the indicator on (0.4,0.6) is sampled at 0.5 and produces a different report. This is a finite callback observation, not a guaranteed error bound or a full reconstruction of the test suite's passing implementations.

### 2026-11-01 · tests

2026-11-01: A source-linked external Rust probe checks the hidden-bump report and exact three-sample trace, and rejects equality for an indicator containing the sampled midpoint. Its two runs match. This supplements, rather than expands, the assertions in the source test.
