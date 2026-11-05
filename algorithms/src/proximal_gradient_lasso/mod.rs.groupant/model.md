# algorithm-proximal-gradient-lasso

## descriptant

### proximal_gradient_lasso — mod.rs

A rectangular 1..256 by 1..64, entries/response/lambda magnitudes <=1e6, lambda>=0. A nonzero requires ||A||F²>=1e-12; zero A accepted. The solution may be nonunique.

Report.x with objective .5||Ax-b||²+lambda||x||1, KKT infinity residual, update count and Converged/BudgetExhausted. Error is InvalidInput, AllocationFailed or NonFiniteArithmetic.

Original implementation and tests. Identifiable mathematical sources are listed in model.md#prospective and production source; no copied code. Retain the prospective predictions as hypotheses until measured. This epoch explicitly distinguishes residual and solution accuracy.

## model

### Mathematical combiner

Raw source plus partial syntax identifies declarations; forward equations and the ordered typed family constrain admissible observations; tests challenge selected equations and boundaries. Neither syntax nor the model proves Rust semantics.

ISTA state contains actual x. Gradient is recomputed there and all coordinates pass together through the shared L1 prox with lambda/L. Frobenius scale bounds spectral squared norm in real arithmetic; zero A must terminate before division.

Dependencies: projected_gradient_box report/buffer and soft_threshold. FISTA additionally composes proximal_gradient_lasso validation/evaluation/prox; all helper preconditions are established by shared validation, with scale=0 handled by converged zero-design stopping.

History: prospective model saved before source; first implementation uses original Rust, existing report types and checked buffers, and the existing soft threshold for LASSO. No measured performance gain or executable surrogate has been established.

## approximant

### Forward model and invariant

A rectangular 1..256 by 1..64, entries/response/lambda magnitudes <=1e6, lambda>=0. A nonzero requires ||A||F²>=1e-12; zero A accepted. The solution may be nonunique.

ISTA: L=||A||_F^2 (safe upper bound on spectral squared norm). x_next=soft_threshold(x-A'(Ax-b)/L,lambda/L), from zero. Zero design returns zero.

Report.x with objective .5||Ax-b||²+lambda||x||1, KKT infinity residual, update count and Converged/BudgetExhausted. Error is InvalidInput, AllocationFailed or NonFiniteArithmetic.

O(m*n*(k+1)) time, O(n) working storage beyond caller A. Two initial checked vectors and one soft_threshold output allocation per update.

Invariant: reported residual belongs to returned x; convergence precedes exhaustion including equality; iterations counts committed updates. Validation is complete before vector allocation. No callback assumptions or evaluations. No Stagnated return is inferred from one unchanged point. Input slices remain unchanged.

Tolerance is absolute, finite and positive; budget 0..100000. All index reductions use deterministic ascending order. Real equations are models, not f64 accuracy proofs. No generic percentage guarantee. Executable 1/2/5/10% surrogates are unsupported pending a useful metric and evidence.

## models

# Typed obernant family

## behavior_complement
For fixed valid input, success with wrong column dimension, nonfinite x/objective/KKT residual, iterations>budget, or Converged with KKT violation>tolerance is excluded. With zero budget only x=0 is compatible. For all-zero A every valid report is zero at iteration 0, value=.5||b||². Invalid shape cannot map to success.

## reverse_reconstruction
At zero budget the zero vector has preimages from all valid A,b,lambda with status determined by their KKT residual. At stationarity, gradients belong to -lambda*sign(x) on nonzeros and [-lambda,lambda] on zeros; this relation recovers compatible responses, not a unique one. For rank-deficient A, adding a response component in null(A') preserves gradient but changes objective. Even retaining objective permits sign choices of such components. Threshold output 0 has a whole interval [-lambda/L,lambda/L] of pre-proximal coordinates. FISTA t and extrapolated y cannot be recovered from x alone.

## iteration_state
ISTA state contains actual x. Gradient is recomputed there and all coordinates pass together through the shared L1 prox with lambda/L. Frobenius scale bounds spectral squared norm in real arithmetic; zero A must terminate before division.

## termination_partition
Validation error precedes allocation and iteration. Recompute residual at iteration k; r<=tol returns Converged even at k=budget, otherwise k=budget returns BudgetExhausted. No Stagnated result is emitted. Small residual does not bound coordinate error without conditioning and rounding analysis.

## finite_arithmetic
All public operands are finite and bounded. Quadratic gradients, norm, step statistics and candidate coordinates are checked. LASSO checks objective, gradients, coordinates, KKT violation and extrapolation; shared soft_threshold rejects nonfinite pre-prox points. Inputs below squared-scale threshold are rejected, including nonzero designs whose squares underflow to zero. Underflow/cancellation inside allowed arithmetic may change branch decisions; no analytical f64 guarantee is claimed.

## resource_observation
O(m*n*(k+1)) time, O(n) working storage beyond caller A. Two initial checked vectors and one soft_threshold output allocation per update. Checked reserve failures map to AllocationFailed, and callers cannot infer deterministic memory availability. No callbacks, I/O, random state or mutation of input. Allocation-failure branch is specified but not fault-injected.

## tests

### Executable checks

cargo test -p mova-algorithms proximal_gradient_lasso::tests

Five named tests per algorithm: hand-derived recurrence; independent optimum grid; validation precedence; termination/shape boundaries; small-gradient ambiguity or correlated/nonunique KKT fixture. Validation receipts record repeated debug/release and mutation results. Allocation-failure injection and universal floating accuracy are absent.

## prospective

### Prospective model
Saved before implementation.

ISTA: L=||A||_F^2 (safe upper bound on spectral squared norm). x_next=soft_threshold(x-A'(Ax-b)/L,lambda/L), from zero. Zero design returns zero.

Minimize .5||Ax-b||²+lambda||x||1. Rows 1..256, columns 1..64; finite input magnitudes <=1e6; lambda nonnegative <=1e6. KKT infinity violation and objective recomputed at returned x. Nonzero design with Frobenius squared norm below 1e-12 is rejected to avoid an unusable tiny-scale step.
Finite positive tolerance; iteration cap <=100000, zero permitted. Validation precedes allocation. Checked reserve failure is separate from invalid inputs and nonfinite arithmetic. No callbacks. Deterministic index order, exact tie at residual=tolerance counts converged, and convergence precedes budget exhaustion. No premature stagnation claim for accelerated history.

Cost O(rows*columns) per step, O(rows+columns) auxiliary storage; no dense inverse. Expected benefit: simultaneous proximal steps with a safe deterministic step size. This is a hypothesis, not timing evidence. Residual convergence is not solution-error certification, especially under poor conditioning or nonuniqueness. f64 rounding, cancellation and underflow are not covered by real-number convergence identities. No 1/2/5/10% executable profile is justified yet.

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

Source SHA256: 4d6bdf740802406cf7f6031d9e03e028fca4d769688cded0dd0ccaf8e76cc2e3.

Receipts below use $MOVA_DATA/runs/octopus-wave25-optimization-2026-10-30/. SHA256 binds the observed logs; logs remain external to the source repository.

- proximal_gradient_lasso-debug-1.log: 4ba1c877308e90e4dda8c9f67e5c1813328624c255bfab16fb7e830baf174119

- proximal_gradient_lasso-debug-2.log: 00ae39948e6e9d1ef60c89e71e3414568982ddc85f18c97a35c8a3d9cb580417

- proximal_gradient_lasso-release-1.log: 9098c4080ebb4835ff2529c3b808b9bc82931774e6c6b720268af7f7bec1e9cd

- proximal_gradient_lasso-release-2.log: 2049f998282f1cd72bc06c3ac8153cc749222113bb86b94782bdbe83e7bd9b93

- proximal_gradient_lasso-mutant.log: 9d6a52ab057a0a14cf8b1d232d874eddba9bfd8f66f534752a167288bcb621ad

- proximal_gradient_lasso-restored.log: f0d2da86a31f6d8e67d4939d5ad097e605d532a1a39bce8b8f8711b05b435c08

- proximal_gradient_lasso-normalized.txt: aa94d79590789050aed6f7066c8cf17dc69311c1a401b4cc6a626653b900d111
