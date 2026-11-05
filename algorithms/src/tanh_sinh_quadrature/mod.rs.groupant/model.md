# algorithm-tanh-sinh-quadrature

## descriptant

### tanh_sinh_quadrature: mod.rs

Original std-only Rust. References: Neville/Newton: Stoer and Bulirsch, Introduction to Numerical Analysis, interpolation chapter; spline: de Boor, A Practical Guide to Splines; trapezoid: Davis and Rabinowitz, Methods of Numerical Integration; tanh-sinh: Takahasi and Mori (1974), Double exponential formulas for numerical integration. No copied implementation.

#### Prospective epoch (before implementation)

Before implementation: finite strictly increasing nonempty interpolation nodes, matched finite ordinates; spline at least two. Neville triangular O(n^2) time/O(n) memory, P_i,j=((x_j-x)P_i,j-1+(x-x_i)P_i+1,j)/(x_j-x_i). Newton backward divided differences O(n^2)/O(n), nested product evaluation O(n). Natural spline Thomas solve for second derivatives, zero endpoint curvatures, O(n) construction and binary interval evaluation. Reject nonfinite intermediate states and undefined spacing; no conditioning promise. Quadrature endpoints strictly ordered finite, finite positive tolerance, finite span; callbacks finite, deterministic and resource bounded by caller. Adaptive trapezoid dyadically doubles grid reusing samples, /3 consecutive difference heuristic; bounded evaluations and stagnation. Tanh sinh fixed transformed truncation [-3,3], dyadic h, samples strictly inside endpoints in ascending transformed order per level, no reuse; error is consecutive difference only and excludes truncation error. Both no arbitrary-function accuracy bound: zero and unsampled bump share sample trace. Expected gain: Neville O(n) storage versus full tableau, Newton O(n) repeated evaluation, spline linear solve; no measured speed claim or executable approximation percentage. Validation independent polynomial/analytic fixtures, domain/overflow/status/count and alias witnesses; compiled production mutants and repeats.


#### Mathematical combiner

Raw source and partial syntax anchor the f64 transition F; forward model predicts F, typed views describe its excluded pairs and preimage fibers; independent tests evaluate finite projections of F. Their conjunction narrows compatible implementations but is not a proof. No semantic AST resolution or macro expansion. Leaf children=[]; crate wiring is parent-owned.

## model

## behavior_complement

For each input s, let F(s) be this source's tagged f64 outcome and callback trace. Complement contains (s,o) with o!=F(s), including successful NaN/Inf, invalid-input success and wrong stated recurrence. For node algorithms, invalid shape/order excludes any Ok result; for quadrature, invalid budget excludes any callback, and calls>budget are excluded even when the integral estimate is accurate. Finite a<b and finite span, positive finite tolerance, budget>=7. Set mid=a+(b-a)/2, half=(b-a)/2; x(t)=mid+half*tanh(pi/2*sinh t), w(t)=half*pi/2*cosh(t)/cosh(pi/2*sinh t)^2. Composite trapezoid on fixed t window [-3,3] with 6,12,24,... panels, endpoint half weights, ascending t. Recompute all samples each level. Consecutive estimate difference excludes truncation tails and is not an accuracy guarantee.

## reverse_reconstruction

Zero function and indicator in (0.3,0.31) agree on the first 7+13 samples and yield identical zero report although integrals differ by .01. Sampled values and report cannot recover unvisited function values, tail contributions or smoothness.

## transform_geometry

All points strictly interior and ascending; fixed window endpoints are transformed interior nodes, not original endpoints. Rounded collisions, endpoint hits or nonfinite weights produce Stagnation before callbacks at that level. Before first valid level value=0 and estimator=None is a sentinel, not an integral estimate.

## budget_partition

First level costs 7 calls, second 13, third 25. Budget 20 stops after exactly two levels if difference exceeds tolerance. Old sample values are intentionally recalled, so stateful callbacks need not represent any fixed integrand.

## tail_complement

Truncation omits |t|>3. A level difference of zero places no bound on tails or missed interior spikes; EstimatedTolerance can be returned for functions with nonzero integral and zero samples.

## finite_arithmetic

Every callback value, weighted term, accumulated sum, refined value and difference must remain finite or an error returns. Transform geometry rejects unusable weights as Stagnation. Smooth real double-exponential decay does not imply machine-relative accuracy.

## approximant

### Forward model and invariant

Finite a<b and finite span, positive finite tolerance, budget>=7. Set mid=a+(b-a)/2, half=(b-a)/2; x(t)=mid+half*tanh(pi/2*sinh t), w(t)=half*pi/2*cosh(t)/cosh(pi/2*sinh t)^2. Composite trapezoid on fixed t window [-3,3] with 6,12,24,... panels, endpoint half weights, ascending t. Recompute all samples each level. Consecutive estimate difference excludes truncation tails and is not an accuracy guarantee.

Cost: O(evaluations) transcendental work and O(1) storage; each level has geometry and sampling pass. Budget checked for whole panels+1 calls; checked doubling. Callback costs/panics outside model.

Invariants: successful reported scalars are finite; invalid-domain cases do not become approximate answers. Callback budgets and order are observable semantics. Real polynomial reproduction and quadrature convergence statements do not certify f64 accuracy. Determinism presumes deterministic callback. No executable surrogate profile: cancellation/conditioning and unobserved samples prevent a useful general 1/2/5/10% certificate. No gain or percentage promise.

## tests

analytic_oracle compares powers 0..4 and sine to exact integrals; budget_and_order checks 7+13 counts, ascending interior abscissae, repeated samples, midpoint and budget=7 estimator None; failure_and_alias checks invalid/no calls, first-call abort, adjacent floats and hidden bump.

Control: Multiply transformed sum by 0.5; analytic_oracle assertion fails.

## History

### 2026-10-30 · descriptant

#### Observed epoch 2026-10-30

15 focused tests pass twice debug and twice optimized release with identical normalized outcomes. Five compiled production mutants fail oracle assertions; original bytes restored and positives rerun. No benchmark or speed claim. Finite arithmetic, allocations and callback limitations remain.

### 2026-10-30 · tests

External evidence: $MOVA_DATA/runs/octopus-wave25-numerical-2026-10-30; repeated focused harness and strict Clippy.
