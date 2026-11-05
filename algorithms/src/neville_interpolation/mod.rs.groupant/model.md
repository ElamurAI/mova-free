# algorithm-neville-interpolation

## descriptant

### neville_interpolation: mod.rs

Original std-only Rust. References: Neville/Newton: Stoer and Bulirsch, Introduction to Numerical Analysis, interpolation chapter; spline: de Boor, A Practical Guide to Splines; trapezoid: Davis and Rabinowitz, Methods of Numerical Integration; tanh-sinh: Takahasi and Mori (1974), Double exponential formulas for numerical integration. No copied implementation.

#### Prospective epoch (before implementation)

Before implementation: finite strictly increasing nonempty interpolation nodes, matched finite ordinates; spline at least two. Neville triangular O(n^2) time/O(n) memory, P_i,j=((x_j-x)P_i,j-1+(x-x_i)P_i+1,j)/(x_j-x_i). Newton backward divided differences O(n^2)/O(n), nested product evaluation O(n). Natural spline Thomas solve for second derivatives, zero endpoint curvatures, O(n) construction and binary interval evaluation. Reject nonfinite intermediate states and undefined spacing; no conditioning promise. Quadrature endpoints strictly ordered finite, finite positive tolerance, finite span; callbacks finite, deterministic and resource bounded by caller. Adaptive trapezoid dyadically doubles grid reusing samples, /3 consecutive difference heuristic; bounded evaluations and stagnation. Tanh sinh fixed transformed truncation [-3,3], dyadic h, samples strictly inside endpoints in ascending transformed order per level, no reuse; error is consecutive difference only and excludes truncation error. Both no arbitrary-function accuracy bound: zero and unsampled bump share sample trace. Expected gain: Neville O(n) storage versus full tableau, Newton O(n) repeated evaluation, spline linear solve; no measured speed claim or executable approximation percentage. Validation independent polynomial/analytic fixtures, domain/overflow/status/count and alias witnesses; compiled production mutants and repeats.


#### Mathematical combiner

Raw source and partial syntax anchor the f64 transition F; forward model predicts F, typed views describe its excluded pairs and preimage fibers; independent tests evaluate finite projections of F. Their conjunction narrows compatible implementations but is not a proof. No semantic AST resolution or macro expansion. Leaf children=[]; crate wiring is parent-owned.

## model

## behavior_complement

For each input s, let F(s) be this source's tagged f64 outcome and callback trace. Complement contains (s,o) with o!=F(s), including successful NaN/Inf, invalid-input success and wrong stated recurrence. For node algorithms, invalid shape/order excludes any Ok result; for quadrature, invalid budget excludes any callback, and calls>budget are excluded even when the integral estimate is accurate. Neville triangular recurrence P(i,w)=((x[i+w]-q)P(i,w-1)+(q-x[i])P(i+1,w-1))/(x[i+w]-x[i]). Updates i ascending preserve previous-width right neighbors. Nonempty matched finite slices, strictly increasing nodes, finite query; extrapolation allowed. Shape/nonfinite/order rejection precedes arithmetic. One node returns its ordinate even for distant queries.

## reverse_reconstruction

At q=0, nodes [-1,1], ordinates [-1,1] and [0,0] both yield 0. A scalar query cannot recover ordinates or nodes. At known distinct nodes, real exact interpolation is unique for degree<n; rounded evaluation is not injective.

## tableau_projection

After width w, slot i represents polynomial on nodes i..i+w in real algebra; ascending i prevents overwriting p[i+1] too soon. Floating results need not obey exact reproduction.

## finite_failure

Check span, query offsets, products and sum before division, then quotient. Finite inputs with [-MAX,MAX] nodes are excluded from successful arithmetic; repeated/descending nodes reject before arithmetic.

## query_fiber

Single query is linear in ys in real arithmetic, with kernel dimension n-1 generically; at a knot, all other ordinates are invisible. Finite arithmetic can additionally collapse distinct ordinates.

## storage_refinement

Full n by n triangle and n-element rolling frontier describe the same real recurrence. The implementation discards old widths, so full intermediate history is not reconstructible from the output.

## approximant

### Forward model and invariant

Neville triangular recurrence P(i,w)=((x[i+w]-q)P(i,w-1)+(q-x[i])P(i+1,w-1))/(x[i+w]-x[i]). Updates i ascending preserve previous-width right neighbors. Nonempty matched finite slices, strictly increasing nodes, finite query; extrapolation allowed. Shape/nonfinite/order rejection precedes arithmetic. One node returns its ordinate even for distant queries.

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
