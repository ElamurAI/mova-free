# algorithm-neville-interpolation-tests

## descriptant

### neville_interpolation: tests.rs

Original std-only Rust. References: Neville/Newton: Stoer and Bulirsch, Introduction to Numerical Analysis, interpolation chapter; spline: de Boor, A Practical Guide to Splines; trapezoid: Davis and Rabinowitz, Methods of Numerical Integration; tanh-sinh: Takahasi and Mori (1974), Double exponential formulas for numerical integration. No copied implementation.

#### Prospective epoch (before implementation)

Before implementation: finite strictly increasing nonempty interpolation nodes, matched finite ordinates; spline at least two. Neville triangular O(n^2) time/O(n) memory, P_i,j=((x_j-x)P_i,j-1+(x-x_i)P_i+1,j)/(x_j-x_i). Newton backward divided differences O(n^2)/O(n), nested product evaluation O(n). Natural spline Thomas solve for second derivatives, zero endpoint curvatures, O(n) construction and binary interval evaluation. Reject nonfinite intermediate states and undefined spacing; no conditioning promise. Quadrature endpoints strictly ordered finite, finite positive tolerance, finite span; callbacks finite, deterministic and resource bounded by caller. Adaptive trapezoid dyadically doubles grid reusing samples, /3 consecutive difference heuristic; bounded evaluations and stagnation. Tanh sinh fixed transformed truncation [-3,3], dyadic h, samples strictly inside endpoints in ascending transformed order per level, no reuse; error is consecutive difference only and excludes truncation error. Both no arbitrary-function accuracy bound: zero and unsampled bump share sample trace. Expected gain: Neville O(n) storage versus full tableau, Newton O(n) repeated evaluation, spline linear solve; no measured speed claim or executable approximation percentage. Validation independent polynomial/analytic fixtures, domain/overflow/status/count and alias witnesses; compiled production mutants and repeats.


#### Mathematical combiner

Raw source and partial syntax anchor the f64 transition F; forward model predicts F, typed views describe its excluded pairs and preimage fibers; independent tests evaluate finite projections of F. Their conjunction narrows compatible implementations but is not a proof. No semantic AST resolution or macro expansion. Leaf children=[]; crate wiring is parent-owned.

## model

## behavior_complement

Observed Pass excludes implementations violating any asserted result on these fixtures, including compiled mutant: Change left+right to left-right; polynomial_oracle assertion fails. Pass does not exclude bugs outside tested cases; it is not a universal theorem.

## reverse_reconstruction

Pass has many preimages: exact implementations and implementations agreeing only on this finite fixture set. Untested q=12345 behavior can change while all assertions pass. At q=0, nodes [-1,1], ordinates [-1,1] and [0,0] both yield 0. A scalar query cannot recover ordinates or nodes. At known distinct nodes, real exact interpolation is unique for degree<n; rounded evaluation is not injective.

## oracle_independence

Expected values use analytic monomials, antiderivatives or explicit spline cubic, independently of production recurrence. polynomial_oracle checks powers 0..5 at 23 rational queries including extrapolation; boundaries checks singleton, empty, duplicate, NaN and overflowing span; ambiguous_query checks a nonunique preimage.

## mutation_sensitivity

Change left+right to left-right; polynomial_oracle assertion fails.

## finite_scope

Pass records only finite fixtures, not all f64 values or callbacks. Compile errors do not count as killed mutants; production mutation must compile and fail named assertion, then exact original bytes restored and rerun.

## approximant

### Forward model and invariant

Finite test transition maps fixture implementations to pass or assertion panic. polynomial_oracle checks powers 0..5 at 23 rational queries including extrapolation; boundaries checks singleton, empty, duplicate, NaN and overflowing span; ambiguous_query checks a nonunique preimage.

Cost: O(n^2) scalar operations and O(n) allocated storage; allocation failure follows Rust allocator behavior, not Error.

Invariants: successful reported scalars are finite; invalid-domain cases do not become approximate answers. Callback budgets and order are observable semantics. Real polynomial reproduction and quadrature convergence statements do not certify f64 accuracy. Determinism presumes deterministic callback. No executable surrogate profile: cancellation/conditioning and unobserved samples prevent a useful general 1/2/5/10% certificate. No gain or percentage promise.

## tests

polynomial_oracle checks powers 0..5 at 23 rational queries including extrapolation; boundaries checks singleton, empty, duplicate, NaN and overflowing span; ambiguous_query checks a nonunique preimage.

Control: Change left+right to left-right; polynomial_oracle assertion fails.

## History

### 2026-10-30 · descriptant

#### Observed epoch 2026-10-30

15 focused tests pass twice debug and twice optimized release with identical normalized outcomes. Five compiled production mutants fail oracle assertions; original bytes restored and positives rerun. No benchmark or speed claim. Finite arithmetic, allocations and callback limitations remain.

### 2026-10-30 · tests

External evidence: $MOVA_DATA/runs/octopus-wave25-numerical-2026-10-30; repeated focused harness and strict Clippy.
