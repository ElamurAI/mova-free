# algorithm-natural-cubic-spline

## descriptant

### natural_cubic_spline: mod.rs

Original std-only Rust. References: Neville/Newton: Stoer and Bulirsch, Introduction to Numerical Analysis, interpolation chapter; spline: de Boor, A Practical Guide to Splines; trapezoid: Davis and Rabinowitz, Methods of Numerical Integration; tanh-sinh: Takahasi and Mori (1974), Double exponential formulas for numerical integration. No copied implementation.

#### Prospective epoch (before implementation)

Before implementation: finite strictly increasing nonempty interpolation nodes, matched finite ordinates; spline at least two. Neville triangular O(n^2) time/O(n) memory, P_i,j=((x_j-x)P_i,j-1+(x-x_i)P_i+1,j)/(x_j-x_i). Newton backward divided differences O(n^2)/O(n), nested product evaluation O(n). Natural spline Thomas solve for second derivatives, zero endpoint curvatures, O(n) construction and binary interval evaluation. Reject nonfinite intermediate states and undefined spacing; no conditioning promise. Quadrature endpoints strictly ordered finite, finite positive tolerance, finite span; callbacks finite, deterministic and resource bounded by caller. Adaptive trapezoid dyadically doubles grid reusing samples, /3 consecutive difference heuristic; bounded evaluations and stagnation. Tanh sinh fixed transformed truncation [-3,3], dyadic h, samples strictly inside endpoints in ascending transformed order per level, no reuse; error is consecutive difference only and excludes truncation error. Both no arbitrary-function accuracy bound: zero and unsampled bump share sample trace. Expected gain: Neville O(n) storage versus full tableau, Newton O(n) repeated evaluation, spline linear solve; no measured speed claim or executable approximation percentage. Validation independent polynomial/analytic fixtures, domain/overflow/status/count and alias witnesses; compiled production mutants and repeats.


#### Mathematical combiner

Raw source and partial syntax anchor the f64 transition F; forward model predicts F, typed views describe its excluded pairs and preimage fibers; independent tests evaluate finite projections of F. Their conjunction narrows compatible implementations but is not a proof. No semantic AST resolution or macro expansion. Leaf children=[]; crate wiring is parent-owned.

## model

## behavior_complement

For each input s, let F(s) be this source's tagged f64 outcome and callback trace. Complement contains (s,o) with o!=F(s), including successful NaN/Inf, invalid-input success and wrong stated recurrence. For node algorithms, invalid shape/order excludes any Ok result; for quadrature, invalid budget excludes any callback, and calls>budget are excluded even when the integral estimate is accurate. At least two finite matched samples with strictly increasing nodes. h_i=x[i+1]-x[i], s_i=(y[i+1]-y[i])/h_i. Solve h[i-1]M[i-1]+2(h[i-1]+h[i])M[i]+h[i]M[i+1]=6(s[i]-s[i-1]) with M endpoints zero. Thomas elimination/back substitution, then cubic segment via normalized weights. Query finite and in closed domain; exact knots return stored ordinates, no extrapolation.

## reverse_reconstruction

Any two-point affine data produce zero endpoint curvatures; curvature alone cannot recover affine component. At q=0, three-point data [0,1,0] and [0,0,0] on [0,1,2] share value 0. Entire knot values and known nodes determine the real natural spline, but one query does not.

## natural_boundary

M[0]=M[n-1]=0 throughout. The two-knot case performs no interior elimination and is exactly the affine interpolation formula subject to rounding.

## tridiagonal_relation

Positive h gives strictly diagonally dominant symmetric interior system in real arithmetic. Machine nonfinite diagonal/pivot/rhs/solution or nonpositive pivot rejects; algebra alone does not certify rounding accuracy.

## interval_partition

Closed domain; exact binary-search hits return ys directly. Otherwise partition_point identifies unique x_i<q<x_i+1. No arbitrary tie choice or extrapolated segment is used.

## continuity_observation

Real system enforces C2 continuity; finite fixture compares symmetric near-knot values and exact natural curvatures. This finite witness does not establish C2 equality for all f64 inputs.

## approximant

### Forward model and invariant

At least two finite matched samples with strictly increasing nodes. h_i=x[i+1]-x[i], s_i=(y[i+1]-y[i])/h_i. Solve h[i-1]M[i-1]+2(h[i-1]+h[i])M[i]+h[i]M[i+1]=6(s[i]-s[i-1]) with M endpoints zero. Thomas elimination/back substitution, then cubic segment via normalized weights. Query finite and in closed domain; exact knots return stored ordinates, no extrapolation.

Cost: O(n) build/storage and O(log n) interval search plus O(1) query arithmetic; allocations follow Rust allocator policy.

Invariants: successful reported scalars are finite; invalid-domain cases do not become approximate answers. Callback budgets and order are observable semantics. Real polynomial reproduction and quadrature convergence statements do not certify f64 accuracy. Determinism presumes deterministic callback. No executable surrogate profile: cancellation/conditioning and unobserved samples prevent a useful general 1/2/5/10% certificate. No gain or percentage promise.

## tests

analytic_oracle checks explicit 1.5t-0.5t^3 arch and a nonuniform affine example; boundaries checks two knots, singleton, duplicates, overflow, out-of-range and NaN; knots_and_smoothness checks endpoint curvature, interpolation and symmetry about central knot.

Control: Change RHS multiplier 6 to 3; analytic_oracle assertion fails.

## History

### 2026-10-30 · descriptant

#### Observed epoch 2026-10-30

15 focused tests pass twice debug and twice optimized release with identical normalized outcomes. Five compiled production mutants fail oracle assertions; original bytes restored and positives rerun. No benchmark or speed claim. Finite arithmetic, allocations and callback limitations remain.

### 2026-10-30 · tests

External evidence: $MOVA_DATA/runs/octopus-wave25-numerical-2026-10-30; repeated focused harness and strict Clippy.
