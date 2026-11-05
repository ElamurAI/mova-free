# algorithm-barycentric-lagrange

## descriptant

### Description and epoch memory

barycentric_lagrange mod.rs. Original standard-library-only implementation with bounded work/storage.

Citation: Berrut and Trefethen (2004), SIAM Review 46(3), 501–517.

### Combiner

Raw source selects recurrence and validation; forward equations describe real observation; complement excludes concrete outcomes; reverse relation retains ambiguous antecedents. Typed specialized relations describe polynomial/shape/sample structure. Partial syntax binds source bytes but has no semantic resolution. Sibling tests falsify selected claims. Composition transports absolute perturbations only under stated premises; no automatic error-tier inheritance.

## model

### Invariant and scope

Sorted 1..64 nodes in [-1,1], gaps >=1e-8, finite |y|<=1e100 and query in hull. Reciprocal weights w_i=1/product_{j!=i}(x_i-x_j); output sum(w_i*y_i/(x-x_i))/sum(w_i/(x-x_i)). Exact node returns its stored y after all validation. Scale terms by maximum absolute term before summing. Size precedes nonfinite, domain, spacing, then arithmetic checks. Underflow in node products can produce Arithmetic even for valid bounded inputs.

O(n²) multiplies, O(n) sums; stack 64 f64, no allocation or callbacks.

Validation order follows source. Ties at exact knots/endpoints take direct stored-value return after validation. Deterministic iteration, no heap allocation. Callback assumptions are explicit only for integration; interpolators have no callbacks. Real identities do not certify machine accuracy. Every successful result is finite. Arithmetic rejection does not imply invalid mathematical input. No unconditional relative-error percentage is claimed.

## approximant

### Forward approximant

Sorted 1..64 nodes in [-1,1], gaps >=1e-8, finite |y|<=1e100 and query in hull. Reciprocal weights w_i=1/product_{j!=i}(x_i-x_j); output sum(w_i*y_i/(x-x_i))/sum(w_i/(x-x_i)). Exact node returns its stored y after all validation. Scale terms by maximum absolute term before summing. Size precedes nonfinite, domain, spacing, then arithmetic checks. Underflow in node products can produce Arithmetic even for valid bounded inputs.

O(n²) multiplies, O(n) sums; stack 64 f64, no allocation or callbacks.

Citation: Berrut and Trefethen (2004), SIAM Review 46(3), 501–517. Equations implemented independently, no copied solution code. Real exactness requires exact arithmetic; stored tables and arithmetic are rounded. No certified useful cheaper executable Rust surrogate is attached: error certification near cancellation/zero and unseen callbacks remains a gap. Tests are checks, not a surrogate.

## models

## behavior_complement

No successful extrapolation. For valid exact-node query an output other than its stored ordinate is excluded. Does not certify conditioning or floating accuracy.

## reverse_reconstruction

At x=0, nodes [(-1,2),(1,4)] and [(-1,1),(1,5)] both map to3; a single evaluation cannot reconstruct node ordinates.

## polynomial_reproduction

In exact real arithmetic, every polynomial of degree <n is reproduced at every hull query. Machine tests compare x^d for d=0..5 on seven equally spaced nodes; tolerance is absolute 1e-12, not a universal bound.

## weight_scale_invariance

Multiplying all barycentric terms by any common nonzero real factor preserves the ratio. Production divides by max |term|; this reduces range pressure but cannot recover overflow already incurred in reciprocals.

## node_shortcut_partition

After validation, x=x_i returns y_i without weight computation; otherwise all n weights are computed. A dense 64-node fixture can fail off-node yet succeed at a node.

## ordinate_affine_transport

In real arithmetic replacing y by alpha*y+beta transports output to alpha*p+beta where denominator is nonzero. This is absolute-error transport by |alpha| only under exact transformation; machine arithmetic adds rounding.


## tests

Run cargo test -p mova-algorithms barycentric_lagrange::tests. Independent analytic polynomial or Bernstein oracle, shape constraints where applicable, validation boundaries and ambiguous fibers. Two debug and two release executions normalize named test outcomes before comparison. Compiled production arithmetic mutant fails named oracle; original bytes restored and positive test rerun. Evidence: (see History). Receipt records concrete result and hashes. No performance benchmark, semantic proof, or generic floating error cap.

### Tests and evidence

## prospective

## History

### 2026-10-30 · descriptant

Prospective prediction preserved in production model.md#prospective before source changes: fixed stack and recurrence reuse reduce repeated work without a benchmark speed claim. 2026-10-30 finite independent oracles are the evidence, not universal proofs.

### 2026-10-30 · tests

Run cargo test -p mova-algorithms barycentric_lagrange::tests. Independent analytic polynomial or Bernstein oracle, shape constraints where applicable, validation boundaries and ambiguous fibers. Two debug and two release executions normalize named test outcomes before comparison. Compiled production arithmetic mutant fails named oracle; original bytes restored and positive test rerun. Evidence: $MOVA_DATA/runs/octopus-wave20-interpolation-2026-10-30/. Receipt records concrete result and hashes. No performance benchmark, semantic proof, or generic floating error cap.

### 2026-10-30 · prospective

Prospective 2026-10-30 before production writes. All kernels std-only f64, finite validated data magnitude <=1e100, <=64 interpolation nodes or <=128 Bezier controls; interpolation spacings >=1e-8 on [-1,1]. Arithmetic failures explicit; no machine relative-error cap. Finite polynomial fixtures measure absolute error, zero reference uses absolute error too. Barycentric weights reciprocal node products, O(n²) work O(n) fixed stack; exact node shortcut prevents singularity. PCHIP uses weighted harmonic slopes and clipped one-sided endpoint slopes, O(n) validation/lookup O(1) stack; real monotonicity/affine reproduction. De Casteljau convex combinations O(n²) work O(n) stack; Bernstein polynomial and convex hull in reals. Gauss Legendre orders 1..5 fixed table, O(order) calls and constant storage, exact real polynomial degree <=2order-1; finite table rounding and callback error separate. Romberg nested trapezoids and Richardson (4^j-1) extrapolation, <=20 levels and explicit evaluation budget; O(2^levels+levels²) work O(levels) stack. Diagonal difference is heuristic, never certified; explicit budget, level and coordinate stagnation statuses. Smoothness needed for asymptotic error model. Callback pure deterministic finite bounded outputs assumed; panics not caught. Expected benefits are weight-free query shortcut, in-place Bezier triangle, symmetric Gaussian constants and reused Romberg samples; no timing gain promised. No useful certified cheaper surrogate is supplied; gap explicit.

### 2026-11-01 · ordinate_affine_transport

#### Validated knot bit fiber (review 2026-11-01)

For a completed call whose nodes and query pass all validation, if `x == a_k` under Rust `f64` equality, the observation `bits(interpolate(nodes,x))` equals `bits(y_k)`. The return copies the ordinate before arithmetic. In particular the sign bit of a zero ordinate is retained, even though positive and negative zero query coordinates compare equal. With coordinates and the selected ordinate fixed, replacing other ordinates by any finite values of magnitude at most `1e100` stays in this particular output fiber. This is a partial converse, not a unique reconstruction or a complete fiber characterization.

Validation precedes this shortcut: duplicate coordinates at the query give `Spacing`, and an out-of-range ordinate elsewhere gives `Domain`. A valid crowded 64-node fixture returns its ordinate at a knot but `Arithmetic` off-node. Thus neither knot equality alone nor an off-node arithmetic failure determines the outcome without the other inputs. The behavior complement excludes a different returned bit pattern on this validated knot slice; it does not exclude arbitrary outputs off that slice. The 2026-11-01 external Rust probe calls the actual source and checks these cases; finite witnesses and source inspection are not a machine-checked proof or floating error bound.
