# algorithm-romberg-integration

## descriptant

### Description and epoch memory

romberg_integration mod.rs. Original standard-library-only implementation with bounded work/storage.

Citation: Romberg (1955), Det Kongelige Norske Videnskabers Selskab Forhandlinger28, 30–36.

### Combiner

Raw source selects recurrence and validation; forward equations describe real observation; complement excludes concrete outcomes; reverse relation retains ambiguous antecedents. Typed specialized relations describe polynomial/shape/sample structure. Partial syntax binds source bytes but has no semantic resolution. Sibling tests falsify selected claims. Composition transports absolute perturbations only under stated premises; no automatic error-tier inheritance.

## model

### Invariant and scope

Finite |a|,|b|<=1e100, a<=b; finite tolerance>0; levels1..20, evaluation budget>=2. Configuration precedes bounds checks. T_k=T_(k-1)/2+h*sum new odd-grid samples. R_(k,j)=R_(k,j-1)+(R_(k,j-1)-R_(k-1,j-1))/(4^j-1). Callback deterministic finite |f|<=1e100, panics propagate. Successive diagonal difference is only heuristic absolute error. Configuration/domain validated before degenerate return.

At complete level L>=1, 2^(L-1)+1 calls; zero interval0 calls. O(2^L+L²) arithmetic, fixed20 f64 stack, no allocation. Refinement budget checked before new level, partial coordinate-stagnated level may consume samples and report previous completed level value.

Validation order follows source. Ties at exact knots/endpoints take direct stored-value return after validation. Deterministic iteration, no heap allocation. Callback assumptions are explicit only for integration; interpolators have no callbacks. Real identities do not certify machine accuracy. Every successful result is finite. Arithmetic rejection does not imply invalid mathematical input. No unconditional relative-error percentage is claimed.

## approximant

### Forward approximant

Finite |a|,|b|<=1e100, a<=b; finite tolerance>0; levels1..20, evaluation budget>=2. Configuration precedes bounds checks. T_k=T_(k-1)/2+h*sum new odd-grid samples. R_(k,j)=R_(k,j-1)+(R_(k,j-1)-R_(k-1,j-1))/(4^j-1). Callback deterministic finite |f|<=1e100, panics propagate. Successive diagonal difference is only heuristic absolute error. Configuration/domain validated before degenerate return.

At complete level L>=1, 2^(L-1)+1 calls; zero interval0 calls. O(2^L+L²) arithmetic, fixed20 f64 stack, no allocation. Refinement budget checked before new level, partial coordinate-stagnated level may consume samples and report previous completed level value.

Citation: Romberg (1955), Det Kongelige Norske Videnskabers Selskab Forhandlinger28, 30–36. Equations implemented independently, no copied solution code. Real exactness requires exact arithmetic; stored tables and arithmetic are rounded. No certified useful cheaper executable Rust surrogate is attached: error certification near cancellation/zero and unseen callbacks remains a gap. Tests are checks, not a surrogate.

## models

## behavior_complement

No callback count above budget and no refinement above20 levels. Converged excludes only a large latest diagonal difference, not large true integration error. Sample aliasing can fool it.

## reverse_reconstruction

All functions agreeing on the finite sampled dyadic grid give the same report under deterministic callbacks. Adding c*product(x-x_j)^2 preserves every sampled value but changes the true integral; no inverse reconstruction of full function or certified error follows.

## nested_sample_reuse

T_k reuses previous trapezoid contribution and samples only2^(k-1) new odd points. Complete-level count is2^k+1, tested independently through report counters.

## richardson_cancellation

For sufficiently smooth integrands, trapezoid expansion in h^(2j) makes denominator4^j-1 cancel successive leading terms. Smoothness/asymptotic regime are premises, not inferred from finite values.

## status_partition

WorkBudget occurs before unaffordable refinement; Stagnated when a generated new coordinate fails strict progress/interiority; LevelLimit after allowed rows; Converged when latest absolute difference<=tolerance or degenerate interval. Previous completed estimate retained on early stop.

## heuristic_error_fiber

Identical finite samples imply identical estimated_error even when unseen function spikes differ arbitrarily. Estimated error is not a true residual, and neither is a solution error certificate. Zero estimate does not mean exact integral.


## tests

Run cargo test -p mova-algorithms romberg_integration::tests. Independent analytic polynomial or Bernstein oracle, shape constraints where applicable, validation boundaries and ambiguous fibers. Two debug and two release executions normalize named test outcomes before comparison. Compiled production arithmetic mutant fails named oracle; original bytes restored and positive test rerun. Evidence: (see History). Receipt records concrete result and hashes. No performance benchmark, semantic proof, or generic floating error cap.

### Tests and evidence

## prospective

## History

### 2026-10-30 · descriptant

Prospective prediction preserved in production model.md#prospective before source changes: fixed stack and recurrence reuse reduce repeated work without a benchmark speed claim. 2026-10-30 finite independent oracles are the evidence, not universal proofs.

### 2026-10-30 · tests

Run cargo test -p mova-algorithms romberg_integration::tests. Independent analytic polynomial or Bernstein oracle, shape constraints where applicable, validation boundaries and ambiguous fibers. Two debug and two release executions normalize named test outcomes before comparison. Compiled production arithmetic mutant fails named oracle; original bytes restored and positive test rerun. Evidence: $MOVA_DATA/runs/octopus-wave20-interpolation-2026-10-30/. Receipt records concrete result and hashes. No performance benchmark, semantic proof, or generic floating error cap.

### 2026-10-30 · prospective

Prospective 2026-10-30 before production writes. All kernels std-only f64, finite validated data magnitude <=1e100, <=64 interpolation nodes or <=128 Bezier controls; interpolation spacings >=1e-8 on [-1,1]. Arithmetic failures explicit; no machine relative-error cap. Finite polynomial fixtures measure absolute error, zero reference uses absolute error too. Barycentric weights reciprocal node products, O(n²) work O(n) fixed stack; exact node shortcut prevents singularity. PCHIP uses weighted harmonic slopes and clipped one-sided endpoint slopes, O(n) validation/lookup O(1) stack; real monotonicity/affine reproduction. De Casteljau convex combinations O(n²) work O(n) stack; Bernstein polynomial and convex hull in reals. Gauss Legendre orders 1..5 fixed table, O(order) calls and constant storage, exact real polynomial degree <=2order-1; finite table rounding and callback error separate. Romberg nested trapezoids and Richardson (4^j-1) extrapolation, <=20 levels and explicit evaluation budget; O(2^levels+levels²) work O(levels) stack. Diagonal difference is heuristic, never certified; explicit budget, level and coordinate stagnation statuses. Smoothness needed for asymptotic error model. Callback pure deterministic finite bounded outputs assumed; panics not caught. Expected benefits are weight-free query shortcut, in-place Bezier triangle, symmetric Gaussian constants and reused Romberg samples; no timing gain promised. No useful certified cheaper surrogate is supplied; gap explicit.
