# algorithm-gauss-legendre

## descriptant

### Description and epoch memory

gauss_legendre mod.rs. Original standard-library-only implementation with bounded work/storage.

Citation: Davis and Rabinowitz, Methods of Numerical Integration, 2nd ed., Gaussian quadrature.

### Combiner

Raw source selects recurrence and validation; forward equations describe real observation; complement excludes concrete outcomes; reverse relation retains ambiguous antecedents. Typed specialized relations describe polynomial/shape/sample structure. Partial syntax binds source bytes but has no semantic resolution. Sibling tests falsify selected claims. Composition transports absolute perturbations only under stated premises; no automatic error-tier inheritance.

## model

### Invariant and scope

Order1..5; finite -1e100<=a<=b<=1e100. Fixed rounded Legendre nodes and positive weights; value=(b-a)/2*sum w_i*f((a+b)/2+(b-a)/2*x_i). Order, nonfinite, domain, then callback validity precedence. Equal endpoints return0 without callback. FnMut assumed deterministic with finite |f|<=1e100; errors abort at first bad sample, panic propagates.

Exactly order callback evaluations on nondegenerate success, increasing tabulated nodes, O(order) arithmetic and constant stack. Caller callback cost/panics external, no allocations.

Validation order follows source. Ties at exact knots/endpoints take direct stored-value return after validation. Deterministic iteration, no heap allocation. Callback assumptions are explicit only for integration; interpolators have no callbacks. Real identities do not certify machine accuracy. Every successful result is finite. Arithmetic rejection does not imply invalid mathematical input. No unconditional relative-error percentage is claimed.

## approximant

### Forward approximant

Order1..5; finite -1e100<=a<=b<=1e100. Fixed rounded Legendre nodes and positive weights; value=(b-a)/2*sum w_i*f((a+b)/2+(b-a)/2*x_i). Order, nonfinite, domain, then callback validity precedence. Equal endpoints return0 without callback. FnMut assumed deterministic with finite |f|<=1e100; errors abort at first bad sample, panic propagates.

Exactly order callback evaluations on nondegenerate success, increasing tabulated nodes, O(order) arithmetic and constant stack. Caller callback cost/panics external, no allocations.

Citation: Davis and Rabinowitz, Methods of Numerical Integration, 2nd ed., Gaussian quadrature. Equations implemented independently, no copied solution code. Real exactness requires exact arithmetic; stored tables and arithmetic are rounded. No certified useful cheaper executable Rust surrogate is attached: error certification near cancellation/zero and unseen callbacks remains a gap. Tests are checks, not a surrogate.

## models

## behavior_complement

No call on valid zero-width interval. Successful callback result NaN or magnitude>1e100 is excluded. No adaptive refinement or error estimate supplied.

## reverse_reconstruction

Every odd integrand on [-1,1] has real quadrature sum0 by symmetry. x and zero function are distinct compatible antecedents; finite samples do not reconstruct an integrand.

## polynomial_exactness

With exact algebraic nodes/weights and real arithmetic, degree<=2order-1 polynomials integrate exactly. Stored f64 constants and arithmetic break literal exactness; tests compare analytic antiderivatives with absolute tolerance1e-12.

## symmetric_cancellation

Nodes occur in +/-pairs with equal weights; odd functions cancel in reals. Iteration is ascending, hence finite cancellation depends on addition order.

## interval_affine_map

Substitution x=m+h*t maps integral to h*integral[-1,1] f(m+h*t)dt. h=0 returns0 without inspecting f; reversed intervals reject instead of negating.

## positive_weight_stability

Positive weights summing2 imply real quadrature perturbation <=(b-a)*max_i|delta f_i|. This bounds propagation of supplied sample errors, not quadrature truncation or machine rounding.


## tests

Run cargo test -p mova-algorithms gauss_legendre::tests. Independent analytic polynomial or Bernstein oracle, shape constraints where applicable, validation boundaries and ambiguous fibers. Two debug and two release executions normalize named test outcomes before comparison. Compiled production arithmetic mutant fails named oracle; original bytes restored and positive test rerun. Evidence: (see History). Receipt records concrete result and hashes. No performance benchmark, semantic proof, or generic floating error cap.

### Tests and evidence

## prospective

## History

### 2026-10-30 · descriptant

Prospective prediction preserved in production model.md#prospective before source changes: fixed stack and recurrence reuse reduce repeated work without a benchmark speed claim. 2026-10-30 finite independent oracles are the evidence, not universal proofs.

### 2026-10-30 · tests

Run cargo test -p mova-algorithms gauss_legendre::tests. Independent analytic polynomial or Bernstein oracle, shape constraints where applicable, validation boundaries and ambiguous fibers. Two debug and two release executions normalize named test outcomes before comparison. Compiled production arithmetic mutant fails named oracle; original bytes restored and positive test rerun. Evidence: $MOVA_DATA/runs/octopus-wave20-interpolation-2026-10-30/. Receipt records concrete result and hashes. No performance benchmark, semantic proof, or generic floating error cap.

### 2026-10-30 · prospective

Prospective 2026-10-30 before production writes. All kernels std-only f64, finite validated data magnitude <=1e100, <=64 interpolation nodes or <=128 Bezier controls; interpolation spacings >=1e-8 on [-1,1]. Arithmetic failures explicit; no machine relative-error cap. Finite polynomial fixtures measure absolute error, zero reference uses absolute error too. Barycentric weights reciprocal node products, O(n²) work O(n) fixed stack; exact node shortcut prevents singularity. PCHIP uses weighted harmonic slopes and clipped one-sided endpoint slopes, O(n) validation/lookup O(1) stack; real monotonicity/affine reproduction. De Casteljau convex combinations O(n²) work O(n) stack; Bernstein polynomial and convex hull in reals. Gauss Legendre orders 1..5 fixed table, O(order) calls and constant storage, exact real polynomial degree <=2order-1; finite table rounding and callback error separate. Romberg nested trapezoids and Richardson (4^j-1) extrapolation, <=20 levels and explicit evaluation budget; O(2^levels+levels²) work O(levels) stack. Diagonal difference is heuristic, never certified; explicit budget, level and coordinate stagnation statuses. Smoothness needed for asymptotic error model. Callback pure deterministic finite bounded outputs assumed; panics not caught. Expected benefits are weight-free query shortcut, in-place Bezier triangle, symmetric Gaussian constants and reused Romberg samples; no timing gain promised. No useful certified cheaper surrogate is supplied; gap explicit.
