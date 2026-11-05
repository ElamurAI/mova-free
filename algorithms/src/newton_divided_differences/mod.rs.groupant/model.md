# algorithm-newton-divided-differences

## descriptant

### newton_divided_differences: mod.rs

Original std-only Rust. References: Neville/Newton: Stoer and Bulirsch, Introduction to Numerical Analysis, interpolation chapter; spline: de Boor, A Practical Guide to Splines; trapezoid: Davis and Rabinowitz, Methods of Numerical Integration; tanh-sinh: Takahasi and Mori (1974), Double exponential formulas for numerical integration. No copied implementation.

#### Prospective epoch (before implementation)

Before implementation: finite strictly increasing nonempty interpolation nodes, matched finite ordinates; spline at least two. Neville triangular O(n^2) time/O(n) memory, P_i,j=((x_j-x)P_i,j-1+(x-x_i)P_i+1,j)/(x_j-x_i). Newton backward divided differences O(n^2)/O(n), nested product evaluation O(n). Natural spline Thomas solve for second derivatives, zero endpoint curvatures, O(n) construction and binary interval evaluation. Reject nonfinite intermediate states and undefined spacing; no conditioning promise. Quadrature endpoints strictly ordered finite, finite positive tolerance, finite span; callbacks finite, deterministic and resource bounded by caller. Adaptive trapezoid dyadically doubles grid reusing samples, /3 consecutive difference heuristic; bounded evaluations and stagnation. Tanh sinh fixed transformed truncation [-3,3], dyadic h, samples strictly inside endpoints in ascending transformed order per level, no reuse; error is consecutive difference only and excludes truncation error. Both no arbitrary-function accuracy bound: zero and unsampled bump share sample trace. Expected gain: Neville O(n) storage versus full tableau, Newton O(n) repeated evaluation, spline linear solve; no measured speed claim or executable approximation percentage. Validation independent polynomial/analytic fixtures, domain/overflow/status/count and alias witnesses; compiled production mutants and repeats.


#### Mathematical combiner

Raw source and partial syntax anchor the f64 transition F; forward model predicts F, typed views describe its excluded pairs and preimage fibers; independent tests evaluate finite projections of F. Their conjunction narrows compatible implementations but is not a proof. No semantic AST resolution or macro expansion. Leaf children=[]; crate wiring is parent-owned.

## model

## behavior_complement

For each input s, let F(s) be this source's tagged f64 outcome and callback trace. Complement contains (s,o) with o!=F(s), including successful NaN/Inf, invalid-input success and wrong stated recurrence. For node algorithms, invalid shape/order excludes any Ok result; for quadrature, invalid budget excludes any callback, and calls>budget are excluded even when the integral estimate is accurate. Nonempty finite matched slices with strictly increasing nodes. Backward c[i]=(c[i]-c[i-1])/(x[i]-x[i-j]) for j=1..n-1; descending i preserves old column. Evaluate v=c[last], then v=v*(q-node[i])+c[i] in reverse. Constructor validates shape/finiteness/order first; query validates finite q first. Coefficients accessor is immutable.

## reverse_reconstruction

At q=0, polynomials x and 0 give identical 0; query does not identify coefficients. Fixed distinct nodes plus complete real coefficients uniquely recover node values through nested evaluation, but coefficients without nodes do not identify original samples and f64 rounding loses information.

## column_preservation

Descending i leaves c[i-1] from the previous divided-difference column. Changing traversal order changes the polynomial for degree>=2.

## finite_failure

Finite span and numerator checked separately, then quotient; evaluate checks offset, product, sum. For ordinates [-MAX,MAX] numerator overflow returns NonFiniteArithmetic even with finite nodes.

## basis_reconstruction

c=[1,3,1], nodes [0,1,2] represents 1+3q+q(q-1)=(q+1)^2. Recover values [1,4,9] in reals; changing nodes changes the represented polynomial.

## query_cost

Stored n nodes/coefficient pairs permit repeated O(n) evaluations without rebuilding O(n^2) differences; no empirical speed ratio claimed.

## approximant

### Forward model and invariant

Nonempty finite matched slices with strictly increasing nodes. Backward c[i]=(c[i]-c[i-1])/(x[i]-x[i-j]) for j=1..n-1; descending i preserves old column. Evaluate v=c[last], then v=v*(q-node[i])+c[i] in reverse. Constructor validates shape/finiteness/order first; query validates finite q first. Coefficients accessor is immutable.

Cost: O(n^2) build, O(n) evaluation, O(n) storage; Rust allocation failure/panic excluded from Result.

Invariants: successful reported scalars are finite; invalid-domain cases do not become approximate answers. Callback budgets and order are observable semantics. Real polynomial reproduction and quadrature convergence statements do not certify f64 accuracy. Determinism presumes deterministic callback. No executable surrogate profile: cancellation/conditioning and unobserved samples prevent a useful general 1/2/5/10% certificate. No gain or percentage promise.

## tests

polynomial_oracle checks degrees 0..5, 23 rational queries and coefficients [1,3,1]; boundaries checks empty, order, difference overflow, evaluation overflow and NaN; ambiguous_query witnesses nonunique scalar output.

Control: Change coefficient difference subtraction to addition; polynomial_oracle assertion fails.

## History

### 2026-10-30 · descriptant

#### Observed epoch 2026-10-30

15 focused tests pass twice debug and twice optimized release with identical normalized outcomes. Five compiled production mutants fail oracle assertions; original bytes restored and positives rerun. No benchmark or speed claim. Finite arithmetic, allocations and callback limitations remain.

### 2026-10-30 · tests

External evidence: $MOVA_DATA/runs/octopus-wave25-numerical-2026-10-30; repeated focused harness and strict Clippy.
