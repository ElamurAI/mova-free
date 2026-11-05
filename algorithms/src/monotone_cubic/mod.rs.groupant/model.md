# algorithm-monotone-cubic

## descriptant

### Description and epoch memory

monotone_cubic mod.rs. Original standard-library-only implementation with bounded work/storage.

Citation: Fritsch and Butland (1984), SIAM J. Sci. Stat. Comput. 5, 300–304.

### Combiner

Raw source selects recurrence and validation; forward equations describe real observation; complement excludes concrete outcomes; reverse relation retains ambiguous antecedents. Typed specialized relations describe polynomial/shape/sample structure. Partial syntax binds source bytes but has no semantic resolution. Sibling tests falsify selected claims. Composition transports absolute perturbations only under stated premises; no automatic error-tier inheritance.

## model

### Invariant and scope

Sorted 2..64 nodes in [-1,1], gaps >=1e-8, finite |y|<=1e100, hull query. Interior derivative is weighted harmonic mean of adjacent secants when nonzero and same sign, zero otherwise. Endpoint uses one-sided derivative then sign/3*secant clipping. Cubic Hermite basis evaluates the selected segment. Size, nonfinite, domain, spacing then arithmetic precedence. Two nodes give linear interpolation; exact node returns stored y.

O(n) validation plus partition lookup O(log n); only neighboring secants/slopes evaluated; O(1) stack, no allocation/callback.

Validation order follows source. Ties at exact knots/endpoints take direct stored-value return after validation. Deterministic iteration, no heap allocation. Callback assumptions are explicit only for integration; interpolators have no callbacks. Real identities do not certify machine accuracy. Every successful result is finite. Arithmetic rejection does not imply invalid mathematical input. No unconditional relative-error percentage is claimed.

## approximant

### Forward approximant

Sorted 2..64 nodes in [-1,1], gaps >=1e-8, finite |y|<=1e100, hull query. Interior derivative is weighted harmonic mean of adjacent secants when nonzero and same sign, zero otherwise. Endpoint uses one-sided derivative then sign/3*secant clipping. Cubic Hermite basis evaluates the selected segment. Size, nonfinite, domain, spacing then arithmetic precedence. Two nodes give linear interpolation; exact node returns stored y.

O(n) validation plus partition lookup O(log n); only neighboring secants/slopes evaluated; O(1) stack, no allocation/callback.

Citation: Fritsch and Butland (1984), SIAM J. Sci. Stat. Comput. 5, 300–304. Equations implemented independently, no copied solution code. Real exactness requires exact arithmetic; stored tables and arithmetic are rounded. No certified useful cheaper executable Rust surrogate is attached: error certification near cancellation/zero and unseen callbacks remains a gap. Tests are checks, not a surrogate.

## models

## behavior_complement

No extrapolation; with monotone real data a segment value outside endpoint ordinate interval is excluded in the real model. No machine theorem or relative accuracy promise.

## reverse_reconstruction

For two nodes, midpoint of [0,2] and [-1,3] is1 with identical x=[0,1]; one query cannot recover endpoint values or slopes.

## harmonic_sign_limiter

If adjacent secants have opposite signs or either zero, interior slope is exactly zero. At local maximum [(0,0),(.3,1),(1,0)] the central slope vanishes and segment shape remains bounded in the real model.

## affine_reproduction

All secants of y=a*x+b equal a, so weighted harmonic slopes and one-sided endpoints equal a in reals. Independent tests sweep node counts2..8 and 101 queries.

## endpoint_clip

One-sided slope with sign opposite first secant is replaced by zero; with neighboring sign reversal, magnitude above3|d| is clipped. This local relation prevents endpoint overshoot; it is distinct from interior harmonic averaging.

## locality_relation

For a non-knot query in segment i, only ordinates/abscissae within i-1..i+2 affect its cubic after validation; distant valid-node changes do not affect computed slopes. Validation still scans the complete input.


## tests

Run cargo test -p mova-algorithms monotone_cubic::tests. Independent analytic polynomial or Bernstein oracle, shape constraints where applicable, validation boundaries and ambiguous fibers. Two debug and two release executions normalize named test outcomes before comparison. Compiled production arithmetic mutant fails named oracle; original bytes restored and positive test rerun. Evidence: (see History). Receipt records concrete result and hashes. No performance benchmark, semantic proof, or generic floating error cap.

### Tests and evidence

## prospective

## History

### 2026-10-30 · descriptant

Prospective prediction preserved in production model.md#prospective before source changes: fixed stack and recurrence reuse reduce repeated work without a benchmark speed claim. 2026-10-30 finite independent oracles are the evidence, not universal proofs.

### 2026-10-30 · tests

Run cargo test -p mova-algorithms monotone_cubic::tests. Independent analytic polynomial or Bernstein oracle, shape constraints where applicable, validation boundaries and ambiguous fibers. Two debug and two release executions normalize named test outcomes before comparison. Compiled production arithmetic mutant fails named oracle; original bytes restored and positive test rerun. Evidence: $MOVA_DATA/runs/octopus-wave20-interpolation-2026-10-30/. Receipt records concrete result and hashes. No performance benchmark, semantic proof, or generic floating error cap.

### 2026-10-30 · prospective

Prospective 2026-10-30 before production writes. All kernels std-only f64, finite validated data magnitude <=1e100, <=64 interpolation nodes or <=128 Bezier controls; interpolation spacings >=1e-8 on [-1,1]. Arithmetic failures explicit; no machine relative-error cap. Finite polynomial fixtures measure absolute error, zero reference uses absolute error too. Barycentric weights reciprocal node products, O(n²) work O(n) fixed stack; exact node shortcut prevents singularity. PCHIP uses weighted harmonic slopes and clipped one-sided endpoint slopes, O(n) validation/lookup O(1) stack; real monotonicity/affine reproduction. De Casteljau convex combinations O(n²) work O(n) stack; Bernstein polynomial and convex hull in reals. Gauss Legendre orders 1..5 fixed table, O(order) calls and constant storage, exact real polynomial degree <=2order-1; finite table rounding and callback error separate. Romberg nested trapezoids and Richardson (4^j-1) extrapolation, <=20 levels and explicit evaluation budget; O(2^levels+levels²) work O(levels) stack. Diagonal difference is heuristic, never certified; explicit budget, level and coordinate stagnation statuses. Smoothness needed for asymptotic error model. Callback pure deterministic finite bounded outputs assumed; panics not caught. Expected benefits are weight-free query shortcut, in-place Bezier triangle, symmetric Gaussian constants and reused Romberg samples; no timing gain promised. No useful certified cheaper surrogate is supplied; gap explicit.
