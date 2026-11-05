# algorithm-adaptive-trapezoid

## descriptant

### adaptive_trapezoid: mod.rs

Original std-only Rust. References: Neville/Newton: Stoer and Bulirsch, Introduction to Numerical Analysis, interpolation chapter; spline: de Boor, A Practical Guide to Splines; trapezoid: Davis and Rabinowitz, Methods of Numerical Integration; tanh-sinh: Takahasi and Mori (1974), Double exponential formulas for numerical integration. No copied implementation.

#### Prospective epoch (before implementation)

Before implementation: finite strictly increasing nonempty interpolation nodes, matched finite ordinates; spline at least two. Neville triangular O(n^2) time/O(n) memory, P_i,j=((x_j-x)P_i,j-1+(x-x_i)P_i+1,j)/(x_j-x_i). Newton backward divided differences O(n^2)/O(n), nested product evaluation O(n). Natural spline Thomas solve for second derivatives, zero endpoint curvatures, O(n) construction and binary interval evaluation. Reject nonfinite intermediate states and undefined spacing; no conditioning promise. Quadrature endpoints strictly ordered finite, finite positive tolerance, finite span; callbacks finite, deterministic and resource bounded by caller. Adaptive trapezoid dyadically doubles grid reusing samples, /3 consecutive difference heuristic; bounded evaluations and stagnation. Tanh sinh fixed transformed truncation [-3,3], dyadic h, samples strictly inside endpoints in ascending transformed order per level, no reuse; error is consecutive difference only and excludes truncation error. Both no arbitrary-function accuracy bound: zero and unsampled bump share sample trace. Expected gain: Neville O(n) storage versus full tableau, Newton O(n) repeated evaluation, spline linear solve; no measured speed claim or executable approximation percentage. Validation independent polynomial/analytic fixtures, domain/overflow/status/count and alias witnesses; compiled production mutants and repeats.


#### Mathematical combiner

Raw source and partial syntax anchor the f64 transition F; forward model predicts F, typed views describe its excluded pairs and preimage fibers; independent tests evaluate finite projections of F. Their conjunction narrows compatible implementations but is not a proof. No semantic AST resolution or macro expansion. Leaf children=[]; crate wiring is parent-owned.

## model

## behavior_complement

For each input s, let F(s) be this source's tagged f64 outcome and callback trace. Complement contains (s,o) with o!=F(s), including successful NaN/Inf, invalid-input success and wrong stated recurrence. For node algorithms, invalid shape/order excludes any Ok result; for quadrature, invalid budget excludes any callback, and calls>budget are excluded even when the integral estimate is accurate. Finite a<b with finite span, positive finite absolute tolerance and budget>=2. Evaluate a then b; initial T=(fa/2+fb/2)(b-a). Each complete level evaluates old-panel midpoints ascending, Tnew=T/2+step*sum/2, estimator |Tnew-T|/3. Report EstimatedTolerance when estimator<=tolerance; it is never a guaranteed integral error. Whole-level budget checked before geometry/callbacks; geometry checked for all abscissae before any level callback.

## reverse_reconstruction

Zero and indicator bump in (0.1,0.2) agree at 0,1,0.5 and return identical zero reports after three calls although true integrals differ by 0.1. Finite callback trace identifies only sampled values, not smoothness or integral.

## callback_trace

Initial endpoints called sequentially; nonfinite first endpoint prevents second call. Each new dyadic level calls left to right exactly once per new midpoint; no old samples recalled. FnMut permitted; analytic meaning assumes deterministic bounded callback.

## budget_partition

With 2 available calls only initial trapezoid returned, estimated_error=None. Budget=5 permits exactly 2+1+2 calls. If next entire level will not fit, Budget returns last complete estimate and its previous estimator.

## stagnation_relation

Geometry requires a<first<...<last<b; duplicate rounded abscissae stop before any callback on that level. Adjacent floats return initial estimate and Stagnation. No progress is inferred from tolerance alone.

## estimator_scope

Division by 3 follows second-order asymptotic trapezoid error for smooth functions, not arbitrary inputs. Aliasing can produce delta=0 with positive missed integral. Finite nonnegative estimator accompanies complete refinements only.

### Budget masks initial geometry

For adjacent representable finite endpoints with finite positive span, valid tolerance,
finite endpoint samples and finite initial arithmetic, there is no representable
interior abscissa. With budget 2 the initial report is `Budget`; with budget at
least 3 it is `Stagnation`. Both have two calls, the same initial value and no
estimator. Thus status is not determined by geometry alone: the whole-level
budget test precedes geometry. This is a conditional observation, not a converse
for either status; other intervals can stagnate at later levels, and callback or
arithmetic errors preempt these reports.

## approximant

### Forward model and invariant

Finite a<b with finite span, positive finite absolute tolerance and budget>=2. Evaluate a then b; initial T=(fa/2+fb/2)(b-a). Each complete level evaluates old-panel midpoints ascending, Tnew=T/2+step*sum/2, estimator |Tnew-T|/3. Report EstimatedTolerance when estimator<=tolerance; it is never a guaranteed integral error. Whole-level budget checked before geometry/callbacks; geometry checked for all abscissae before any level callback.

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

The initial budget gate can mask representational stagnation: adjacent finite
endpoints and successful initial arithmetic return Budget at budget 2, but
Stagnation at budget 3 or above. A source-linked Rust probe checked budgets
2, 3 and 4, plus a non-stagnating midpoint control, twice. The conditional
claim leaves later-level behavior and full status preimages open.

### 2026-11-01 · tests

2026-11-01 independent source-linked probe: adjacent floats with zero callback
and budgets 2, 3, 4 give Budget, Stagnation, Stagnation respectively, with
identical two endpoint calls and no estimator. The [0,1] control with budget 3
reaches EstimatedTolerance after three calls. Two probe runs matched. This is
finite evidence for control precedence, not an integration error bound.
