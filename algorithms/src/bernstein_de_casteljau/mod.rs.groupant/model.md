# algorithm-bernstein-de-casteljau

## descriptant

### Description and epoch memory

bernstein_de_casteljau mod.rs. Original standard-library-only implementation with bounded work/storage.

Citation: Farin, Curves and Surfaces for CAGD, 5th ed., chapter4.

### Combiner

Raw source selects recurrence and validation; forward equations describe real observation; complement excludes concrete outcomes; reverse relation retains ambiguous antecedents. Typed specialized relations describe polynomial/shape/sample structure. Partial syntax binds source bytes but has no semantic resolution. Sibling tests falsify selected claims. Composition transports absolute perturbations only under stated premises; no automatic error-tier inheritance.

## model

### Invariant and scope

1..128 finite controls |c|<=1e100; t in[0,1]. Repeated triangle c_i^(r)=(1-t)c_i^(r-1)+t*c_(i+1)^(r-1) yields sum binomial(n,i)*(1-t)^(n-i)*t^i*c_i. Size, nonfinite, domain, arithmetic precedence. t=0 and1 shortcuts return exact stored controls.

n(n-1)/2 blends, O(n²) time; stack128 f64, no allocation/callback.

Validation order follows source. Ties at exact knots/endpoints take direct stored-value return after validation. Deterministic iteration, no heap allocation. Callback assumptions are explicit only for integration; interpolators have no callbacks. Real identities do not certify machine accuracy. Every successful result is finite. Arithmetic rejection does not imply invalid mathematical input. No unconditional relative-error percentage is claimed.

## approximant

### Forward approximant

1..128 finite controls |c|<=1e100; t in[0,1]. Repeated triangle c_i^(r)=(1-t)c_i^(r-1)+t*c_(i+1)^(r-1) yields sum binomial(n,i)*(1-t)^(n-i)*t^i*c_i. Size, nonfinite, domain, arithmetic precedence. t=0 and1 shortcuts return exact stored controls.

n(n-1)/2 blends, O(n²) time; stack128 f64, no allocation/callback.

Citation: Farin, Curves and Surfaces for CAGD, 5th ed., chapter4. Equations implemented independently, no copied solution code. Real exactness requires exact arithmetic; stored tables and arithmetic are rounded. No certified useful cheaper executable Rust surrogate is attached: error certification near cancellation/zero and unseen callbacks remains a gap. Tests are checks, not a surrogate.

## models

## behavior_complement

For valid t=0 output unequal to first control is excluded. In the real model values outside the control convex hull are excluded. No universal f64 percentage claim.

## reverse_reconstruction

At t=.5, controls [0,2] and [-1,3] both yield1. At t=0 all tails with the same first control are compatible antecedents.

## convex_hull_transport

Every intermediate real control is a convex combination; induction preserves min(c)<=row<=max(c). Fixed finite tests inspect129-or-fewer point sweeps; floating rounding is separate.

## bernstein_equivalence

Triangle equals the Bernstein polynomial of degree n-1 by Pascal identity. Test oracle computes binomial coefficients and powers, not a second triangle.

## control_reversal

Reverse controls and replace t with1-t gives same real value; endpoints swap. Roundoff may differ because addition order differs.

## affine_control_transport

For real alpha,beta, replacing all controls c with alpha*c+beta gives alpha*value+beta since weights sum1. Absolute upstream perturbations <=e propagate <=e in the real model; arithmetic rounding not included.


## tests

Run cargo test -p mova-algorithms bernstein_de_casteljau::tests. Independent analytic polynomial or Bernstein oracle, shape constraints where applicable, validation boundaries and ambiguous fibers. Two debug and two release executions normalize named test outcomes before comparison. Compiled production arithmetic mutant fails named oracle; original bytes restored and positive test rerun. Evidence: (see History). Receipt records concrete result and hashes. No performance benchmark, semantic proof, or generic floating error cap.

### Tests and evidence

## prospective

## History

### 2026-10-30 · descriptant

Prospective prediction preserved in production model.md#prospective before source changes: fixed stack and recurrence reuse reduce repeated work without a benchmark speed claim. 2026-10-30 finite independent oracles are the evidence, not universal proofs.

### 2026-10-30 · tests

Run cargo test -p mova-algorithms bernstein_de_casteljau::tests. Independent analytic polynomial or Bernstein oracle, shape constraints where applicable, validation boundaries and ambiguous fibers. Two debug and two release executions normalize named test outcomes before comparison. Compiled production arithmetic mutant fails named oracle; original bytes restored and positive test rerun. Evidence: $MOVA_DATA/runs/octopus-wave20-interpolation-2026-10-30/. Receipt records concrete result and hashes. No performance benchmark, semantic proof, or generic floating error cap.

### 2026-10-30 · prospective

Prospective 2026-10-30 before production writes. All kernels std-only f64, finite validated data magnitude <=1e100, <=64 interpolation nodes or <=128 Bezier controls; interpolation spacings >=1e-8 on [-1,1]. Arithmetic failures explicit; no machine relative-error cap. Finite polynomial fixtures measure absolute error, zero reference uses absolute error too. Barycentric weights reciprocal node products, O(n²) work O(n) fixed stack; exact node shortcut prevents singularity. PCHIP uses weighted harmonic slopes and clipped one-sided endpoint slopes, O(n) validation/lookup O(1) stack; real monotonicity/affine reproduction. De Casteljau convex combinations O(n²) work O(n) stack; Bernstein polynomial and convex hull in reals. Gauss Legendre orders 1..5 fixed table, O(order) calls and constant storage, exact real polynomial degree <=2order-1; finite table rounding and callback error separate. Romberg nested trapezoids and Richardson (4^j-1) extrapolation, <=20 levels and explicit evaluation budget; O(2^levels+levels²) work O(levels) stack. Diagonal difference is heuristic, never certified; explicit budget, level and coordinate stagnation statuses. Smoothness needed for asymptotic error model. Callback pure deterministic finite bounded outputs assumed; panics not caught. Expected benefits are weight-free query shortcut, in-place Bezier triangle, symmetric Gaussian constants and reused Romberg samples; no timing gain promised. No useful certified cheaper surrogate is supplied; gap explicit.

### 2026-11-01 · descriptant

On 2026-11-01 a bounded independent integer Bernstein oracle checked the actual API at t=1/4, including a rejected reversed-weight candidate and an ambiguous output fiber. Validation of every control precedes endpoint shortcuts. These observations do not prove the floating implementation or supply an error cap; see model.md#models.

### 2026-11-01 · document

#### Bounded dyadic observation and validation boundary (2026-11-01)

For 1..=6 integer controls in [-2,2] and t=1/4, an independent integer Bernstein sum agreed exactly with the actual API on all 19,530 vectors. Its numerator is sum_i binom(n-1,i) 3^(n-1-i) c_i, with denominator 4^(n-1). This finite dyadic observation does not establish machine equality for arbitrary inputs. Reversing the weights is false: [0,4] yields 1, not 3. The output 1 at t=1/4 admits both [0,4] and [2,-2]; reconstruction is a fiber, not an inverse function.

The endpoint shortcut occurs only after all controls are validated: [1, NaN] at zero returns NonFinite and [1, 1e101] at zero returns Domain. Size precedes NonFinite, which precedes Domain. This is a partial error partition, separate from the real convex-hull exclusion and from the converse fiber. The Arithmetic branch and general floating error remain uncharacterized by this observation.
