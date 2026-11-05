# algorithm-nesterov-quadratic

## descriptant

### nesterov_quadratic — mod.rs

D positive diagonal, 1..256 entries in [1e-6,1e6]; b and x0 bounded by 1e6.

Report.x with Euclidean gradient residual ||Dx-b||2, update count and Converged/BudgetExhausted. Error is InvalidInput, AllocationFailed or NonFiniteArithmetic.

Original implementation and tests. Identifiable mathematical sources are listed in model.md#prospective and production source; no copied code. Retain the prospective predictions as hypotheses until measured. This epoch explicitly distinguishes residual and solution accuracy.

## model

### Mathematical combiner

Raw source plus partial syntax identifies declarations; forward equations and the ordered typed family constrain admissible observations; tests challenge selected equations and boundaries. Neither syntax nor the model proves Rust semantics.

Nesterov state is (x,previous). Extrapolate before evaluating gradient: y=x+beta*(x-previous), beta=(sqrt(L)-sqrt(mu))/(sqrt(L)+sqrt(mu)); then x'=y-(Dy-b)/L. Evaluating gradient at old x would produce a different second iterate.

Dependencies: gradient_descent_quadratic public Error/Report/Status only; original iteration and checked buffers local.

History: prospective model saved before source; first implementation uses original Rust, existing report types and checked buffers, and the existing soft threshold for LASSO. No measured performance gain or executable surrogate has been established.

## approximant

### Forward model and invariant

D positive diagonal, 1..256 entries in [1e-6,1e6]; b and x0 bounded by 1e6.

Nesterov strongly convex scheme: beta=(sqrt(L)-sqrt(mu))/(sqrt(L)+sqrt(mu)); y=x+beta*(x-x_previous); x_next=y-(Dy-b)/L.

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
Nesterov state is (x,previous). Extrapolate before evaluating gradient: y=x+beta*(x-previous), beta=(sqrt(L)-sqrt(mu))/(sqrt(L)+sqrt(mu)); then x'=y-(Dy-b)/L. Evaluating gradient at old x would produce a different second iterate.

## termination_partition
Validation error precedes allocation and iteration. Recompute residual at iteration k; r<=tol returns Converged even at k=budget, otherwise k=budget returns BudgetExhausted. No Stagnated result is emitted. Small residual does not bound coordinate error without conditioning and rounding analysis.

## finite_arithmetic
All public operands are finite and bounded. Quadratic gradients, norm, step statistics and candidate coordinates are checked. LASSO checks objective, gradients, coordinates, KKT violation and extrapolation; shared soft_threshold rejects nonfinite pre-prox points. Inputs below squared-scale threshold are rejected, including nonzero designs whose squares underflow to zero. Underflow/cancellation inside allowed arithmetic may change branch decisions; no analytical f64 guarantee is claimed.

## resource_observation
O(n*(k+1)) time and O(n) storage. Two checked vector allocations, no per-iteration allocation. Checked reserve failures map to AllocationFailed, and callers cannot infer deterministic memory availability. No callbacks, I/O, random state or mutation of input. Allocation-failure branch is specified but not fault-injected.

## tests

### Executable checks

cargo test -p mova-algorithms nesterov_quadratic::tests

Five named tests per algorithm: hand-derived recurrence; independent optimum grid; validation precedence; termination/shape boundaries; small-gradient ambiguity or correlated/nonunique KKT fixture. Validation receipts record repeated debug/release and mutation results. Allocation-failure injection and universal floating accuracy are absent.

## prospective

### Prospective model
Saved before implementation.

Nesterov strongly convex scheme: beta=(sqrt(L)-sqrt(mu))/(sqrt(L)+sqrt(mu)); y=x+beta*(x-x_previous); x_next=y-(Dy-b)/L.

Minimize .5*x'Dx-b'x. Dimension 1..256, diagonal [1e-6,1e6], initial and linear magnitudes <=1e6. Gradient Euclidean residual is recomputed at returned x.
Finite positive tolerance; iteration cap <=100000, zero permitted. Validation precedes allocation. Checked reserve failure is separate from invalid inputs and nonfinite arithmetic. No callbacks. Deterministic index order, exact tie at residual=tolerance counts converged, and convergence precedes budget exhaustion. History is algorithm-specific; stationarity of one iterate alone does not establish stagnation with momentum.

Cost O(n) per step and O(n) auxiliary storage; no dense inverse. Expected benefit: momentum on moderately conditioned quadratics. This is a hypothesis, not timing evidence. Residual convergence is not solution-error certification, especially under poor conditioning or nonuniqueness. f64 rounding, cancellation and underflow are not covered by real-number convergence identities. No 1/2/5/10% executable profile is justified yet.

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

Source SHA256: f1b09a73e2ec8bd963c22180d448f248029102ed7b1d22d301da8a78cdf25981.

Receipts below use $MOVA_DATA/runs/octopus-wave25-optimization-2026-10-30/. SHA256 binds the observed logs; logs remain external to the source repository.

- nesterov_quadratic-debug-1.log: b1f767a559e4990fab5a34370af705740baefdb95a7882b86e722af06617a349

- nesterov_quadratic-debug-2.log: 563d544b576f83be8b5180be69c25e85f87728a9887a396407c7e6bcf665829e

- nesterov_quadratic-release-1.log: 64c62ed9255d9b8c3acb624be75a0e6973341939554f380276bdf6c0c4f0fa50

- nesterov_quadratic-release-2.log: 105a8ffffbc95c988b5c299740ac44a28f33718c94c1d9c3ced5ae8c11640914

- nesterov_quadratic-mutant.log: b9441b64ba1801e71b346bb5aa16154bb5594ede1b8028383a139026e33469a3

- nesterov_quadratic-restored.log: a323d0f6adbc6164a258539e4c614176baa18e9c1e8f1013d42b084d6ce83854

- nesterov_quadratic-normalized.txt: 577b1039cdfb275dcc313912e10266b7cafed2534aede5a8231dadd15f60aa10
