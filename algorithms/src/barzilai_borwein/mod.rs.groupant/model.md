# algorithm-barzilai-borwein

## descriptant

### barzilai_borwein — mod.rs

D positive diagonal, 1..256 entries in [1e-6,1e6]; b and x0 bounded by 1e6.

Report.x with Euclidean gradient residual ||Dx-b||2, update count and Converged/BudgetExhausted. Error is InvalidInput, AllocationFailed or NonFiniteArithmetic.

Original implementation and tests. Identifiable mathematical sources are listed in model.md#prospective and production source; no copied code. Retain the prospective predictions as hypotheses until measured. This epoch explicitly distinguishes residual and solution accuracy.

## model

### Mathematical combiner

Raw source plus partial syntax identifies declarations; forward equations and the ordered typed family constrain admissible observations; tests challenge selected equations and boundaries. Neither syntax nor the model proves Rust semantics.

BB history retains previous x. Step uses s.s/(s.Ds), clipped to [1/L,1/mu]; sy=0 falls back to 1/L. Floating underflow can trigger fallback. The clipped spectral step is distinct from fixed gradient descent.

Dependencies: gradient_descent_quadratic public Error/Report/Status only; original iteration and checked buffers local.

History: prospective model saved before source; first implementation uses original Rust, existing report types and checked buffers, and the existing soft threshold for LASSO. No measured performance gain or executable surrogate has been established.

## approximant

### Forward model and invariant

D positive diagonal, 1..256 entries in [1e-6,1e6]; b and x0 bounded by 1e6.

BB1: s=x_k-x_(k-1), y=D*s, alpha=(s.s)/(s.y), safeguarded to [1/L,1/mu]. Initial alpha=1/L. x_next=x-alpha*(Dx-b).

Report.x with Euclidean gradient residual ||Dx-b||2, update count and Converged/BudgetExhausted. Error is InvalidInput, AllocationFailed or NonFiniteArithmetic.

O(n*(k+1)) time and O(n) storage. Two checked vector allocations, no per-iteration allocation.

Invariant: reported residual belongs to returned x; convergence precedes exhaustion including equality; iterations counts committed updates. Validation is complete before vector allocation. No callback assumptions or evaluations. No Stagnated return is inferred from one unchanged point. Input slices remain unchanged.

Tolerance is absolute, finite and positive; budget 0..100000. All index reductions use deterministic ascending order. Real equations are models, not f64 accuracy proofs. No generic percentage guarantee. Executable 1/2/5/10% surrogates are unsupported pending a useful metric and evidence.

## models

# Typed obernant family

## behavior_complement
For fixed valid input, a successful report with x length different from D length, nonfinite x/residual, iterations>budget, or Converged while its recomputed gradient norm>tolerance is excluded. For zero budget the only compatible x is x0; for invalid shape any Ok result is excluded.

## reverse_reconstruction
At budget zero, output x uniquely recovers x0, but not D,b,tolerance. For any positive diagonal D and chosen z, b=Dz with x0=z gives converged z in zero updates, so infinitely many compatible objectives yield one solution. Recovering previous momentum states from only x is ambiguous; even with D,b known a converged status erases trajectory length choices. A single BB gradient step on scalar D at 1/D maps every initial point to b/D in exact arithmetic and is noninjective.

## iteration_state
BB history retains previous x. Step uses s.s/(s.Ds), clipped to [1/L,1/mu]; sy=0 falls back to 1/L. Floating underflow can trigger fallback. The clipped spectral step is distinct from fixed gradient descent.

For D=[1,4], b=0 and x0=[t,t], the first update is [3t/4,0]. In real arithmetic the next BB1 step is 17/65 and the next first coordinate is 36t/65. At t=1e-200 the computed squares underflow to zero, so the implementation instead falls back to 1/4 and produces 9t/16. This is a scale-dependent floating-point branch, not an error bound or a universal homogeneity law.

## termination_partition
Validation error precedes allocation and iteration. Recompute residual at iteration k; r<=tol returns Converged even at k=budget, otherwise k=budget returns BudgetExhausted. No Stagnated result is emitted. Small residual does not bound coordinate error without conditioning and rounding analysis.

## finite_arithmetic
All public operands are finite and bounded. Quadratic gradients, norm, step statistics and candidate coordinates are checked. Small nonzero iterates are admitted; their squared displacements can underflow. Underflow and cancellation inside allowed arithmetic may change branch decisions; no analytical f64 guarantee is claimed. These checks do not evaluate an objective value or certify forward error.

## resource_observation
O(n*(k+1)) time and O(n) storage. Two checked vector allocations, no per-iteration allocation. Checked reserve failures map to AllocationFailed, and callers cannot infer deterministic memory availability. No callbacks, I/O, random state or mutation of input. Allocation-failure branch is specified but not fault-injected.

## tests

### Executable checks

cargo test -p mova-algorithms barzilai_borwein::tests

Five named tests per algorithm: hand-derived recurrence; independent optimum grid; validation precedence; termination/shape boundaries; small-gradient ambiguity or correlated/nonunique KKT fixture. Validation receipts record repeated debug/release and mutation results. Allocation-failure injection and universal floating accuracy are absent.

## prospective

### Prospective model
Saved before implementation.

BB1: s=x_k-x_(k-1), y=D*s, alpha=(s.s)/(s.y), safeguarded to [1/L,1/mu]. Initial alpha=1/L. x_next=x-alpha*(Dx-b).

Minimize .5*x'Dx-b'x. Dimension 1..256, diagonal [1e-6,1e6], initial and linear magnitudes <=1e6. Gradient Euclidean residual is recomputed at returned x.
Finite positive tolerance; iteration cap <=100000, zero permitted. Validation precedes allocation. Checked reserve failure is separate from invalid inputs and nonfinite arithmetic. No callbacks. Deterministic index order, exact tie at residual=tolerance counts converged, and convergence precedes budget exhaustion. History is algorithm-specific; stationarity of one iterate alone does not establish stagnation with momentum.

Cost O(n) per step and O(n) auxiliary storage; no dense inverse. Expected benefit: spectral step adaptation. This is a hypothesis, not timing evidence. Residual convergence is not solution-error certification, especially under poor conditioning or nonuniqueness. f64 rounding, cancellation and underflow are not covered by real-number convergence identities. No 1/2/5/10% executable profile is justified yet.

Independent closed-form diagonal or orthogonal-design optima, exact first/second iterates, zero-budget/status precedence, invalid inputs, correlated-design KKT, and compiled sign/recurrence mutants are planned. Sources: Barzilai and Borwein (1988), Two-point step size gradient methods; Polyak (1964), Some methods of speeding up the convergence of iteration methods; Nesterov (2004), Introductory Lectures on Convex Optimization; Beck and Teboulle (2009), A fast iterative shrinkage-thresholding algorithm for linear inverse problems. Original Rust and fixtures, no imported code.

## validation

### Validation

Five tests passed twice in debug and twice in release, all four normalized named-test result lists identical. Production mutant compiled and failed the named exact_recurrence assertion; restored exact source bytes and reran all five tests successfully. All 643 algorithm library tests passed on the initial integrated source. Strict algorithm library Clippy passed. Ten source syntax projections were generated twice and byte-compared before installation.

No benchmark was run and no speedup is claimed. Tests are finite observations; no universal convergence, allocation-failure, forward-error or generic percentage guarantee. No executable approximation profiles are declared. Source raw code remains outside the sidecar.

## History

### 2026-10-30 · prospective

Heading `### Prospective model — 2026-10-30` stays in prospective as `### Prospective model`.

### 2026-10-30 · validation

Heading `### Validation — 2026-10-30` stays in validation as `### Validation`.

Source SHA256: 86534bbd7f1ca09a45d95b576eb40d42a5a5a80dd436ef7916b8f2877b055686.

Receipts below use $MOVA_DATA/runs/octopus-wave25-optimization-2026-10-30/. SHA256 binds the observed logs; logs remain external to the source repository.

- barzilai_borwein-debug-1.log: 4efc9accc1d0bff2a9598f50cae00b1dc1ef21ed54b5ff87b6980f6689989ace

- barzilai_borwein-debug-2.log: 0c6dd1404baef85dfbb523459a4eb8782190a840fa44535aece91f2dc8ba4933

- barzilai_borwein-release-1.log: 24da6fc861f03d88a0c67e1fcfd99b3dfa2b99910525eb02aaa69283d16c130b

- barzilai_borwein-release-2.log: 507c634af37925921acb8fea277316953d2b3af96ead8ea84a2861a28ad294ca

- barzilai_borwein-mutant.log: 08d1a3979c57188f6ee1fbdfc3d5fc45f6517f0ae5b5afb05f08a0736b47e29a

- barzilai_borwein-restored.log: 16e19fcfe2dddd4a901514f86bc5e50dd8cb90429622bcbc4484672b9d1ec5ae

- barzilai_borwein-normalized.txt: 8c7837b86732faafdae45eb17e5b23be009e23cf12eb963d8d9e036aa4f3d41f

### 2026-11-01 · validation

#### Bounded review — 2026-11-01

An external Rust probe compiled the actual modules and compared two scales of the same two-coordinate problem. At unit scale the second iterate's first coordinate was 0.55384615384615388, consistent with the real-arithmetic 36/65; at 1e-200 its ratio to the scale was 0.5625, consistent with underflow and fallback. Both exhausted a budget of two. The unit-scale case rejects the false claim that every later step uses the initial fixed step; the small-scale case rejects unconditional scale transport of the real BB quotient. A scalar example with D=1, b=0 and distinct initial values 2 and 3 produced identical converged reports after one update, witnessing a non-singleton report fiber. These are finite observations, not a proof of all floating-point branches. Allocation failure, exhaustive error reachability and universal convergence remain open.
