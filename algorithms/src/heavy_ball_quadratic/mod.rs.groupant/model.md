# algorithm-heavy-ball-quadratic

## descriptant

### heavy_ball_quadratic — mod.rs

D positive diagonal, 1..256 entries in [1e-6,1e6]; b and x0 bounded by 1e6.

Report.x with Euclidean gradient residual ||Dx-b||2, update count and Converged/BudgetExhausted. Error is InvalidInput, AllocationFailed or NonFiniteArithmetic.

Original implementation and tests. Identifiable mathematical sources are listed in model.md#prospective and production source; no copied code. Retain the prospective predictions as hypotheses until measured. This epoch explicitly distinguishes residual and solution accuracy.

## model

### Mathematical combiner

Raw source plus partial syntax identifies declarations; forward equations and the ordered typed family constrain admissible observations; tests challenge selected equations and boundaries. Neither syntax nor the model proves Rust semantics.

Heavy-ball state is (x,previous), initialized equal. Alpha=4/(sqrt(L)+sqrt(mu))² and beta=((sqrt(L)-sqrt(mu))/(sqrt(L)+sqrt(mu)))². Exact two-step tests distinguish both coefficients and the sign of gradient.

Dependencies: gradient_descent_quadratic public Error/Report/Status only; original iteration and checked buffers local.

History: prospective model saved before source; first implementation uses original Rust, existing report types and checked buffers, and the existing soft threshold for LASSO. No measured performance gain or executable surrogate has been established.

## approximant

### Forward model and invariant

D positive diagonal, 1..256 entries in [1e-6,1e6]; b and x0 bounded by 1e6.

Polyak heavy ball: alpha=4/(sqrt(L)+sqrt(mu))^2; beta=((sqrt(L)-sqrt(mu))/(sqrt(L)+sqrt(mu)))^2. x_next=x-alpha*(Dx-b)+beta*(x-x_previous), x_previous=x_initial.

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
Heavy-ball state is (x,previous), initialized equal. Alpha=4/(sqrt(L)+sqrt(mu))² and beta=((sqrt(L)-sqrt(mu))/(sqrt(L)+sqrt(mu)))². Exact two-step tests distinguish both coefficients and the sign of gradient.

## termination_partition
Validation error precedes allocation and iteration. Recompute residual at iteration k; r<=tol returns Converged even at k=budget, otherwise k=budget returns BudgetExhausted. No Stagnated result is emitted. Small residual does not bound coordinate error without conditioning and rounding analysis.

## finite_arithmetic
All public operands are finite and bounded. Quadratic gradients, norm, step statistics and candidate coordinates are checked. LASSO checks objective, gradients, coordinates, KKT violation and extrapolation; shared soft_threshold rejects nonfinite pre-prox points. Inputs below squared-scale threshold are rejected, including nonzero designs whose squares underflow to zero. Underflow/cancellation inside allowed arithmetic may change branch decisions; no analytical f64 guarantee is claimed.

## resource_observation
O(n*(k+1)) time and O(n) storage. Two checked vector allocations, no per-iteration allocation. Checked reserve failures map to AllocationFailed, and callers cannot infer deterministic memory availability. No callbacks, I/O, random state or mutation of input. Allocation-failure branch is specified but not fault-injected.

## tests

### Executable checks

cargo test -p mova-algorithms heavy_ball_quadratic::tests

Five named tests per algorithm: hand-derived recurrence; independent optimum grid; validation precedence; termination/shape boundaries; small-gradient ambiguity or correlated/nonunique KKT fixture. Validation receipts record repeated debug/release and mutation results. Allocation-failure injection and universal floating accuracy are absent.

## prospective

### Prospective model
Saved before implementation.

Polyak heavy ball: alpha=4/(sqrt(L)+sqrt(mu))^2; beta=((sqrt(L)-sqrt(mu))/(sqrt(L)+sqrt(mu)))^2. x_next=x-alpha*(Dx-b)+beta*(x-x_previous), x_previous=x_initial.

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

Source SHA256: 3fc54b6993a47537fd35f2c6997d841097fce6678774872a84d7699f72a40327.

Receipts below use $MOVA_DATA/runs/octopus-wave25-optimization-2026-10-30/. SHA256 binds the observed logs; logs remain external to the source repository.

- heavy_ball_quadratic-debug-1.log: 025b169dc70267cbd704db99640ebcd31f6d01c8a1ffaf5cd165c0ac24bf729b

- heavy_ball_quadratic-debug-2.log: 78e188f5b6524e885b13b86c9428123b872368fe0e60ed9916ab8da1858edf5e

- heavy_ball_quadratic-release-1.log: 87bf3433abf46db534585f7e618d92d7b5e8ed5d2c8f62ab691cc732b1cbe259

- heavy_ball_quadratic-release-2.log: 73f598d4cee5c891cdec46ccf6a35d0181da7a917bdae8779257c0f31cd0cec4

- heavy_ball_quadratic-mutant.log: f2d561690085ae0ce0e8cdc339661b9fc332ea0943cb0d3e70bdfc6b4a95a29c

- heavy_ball_quadratic-restored.log: b9f7b49d7f5f4b420b42fa10dd85f56ccf6e927a97259189946d17b1d54fcb08

- heavy_ball_quadratic-normalized.txt: 49edad85f860c83176c61d82dc531f71bfb3cf2738e67d1248c5dc2c16f5cd8b
