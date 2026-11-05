# algorithm-fista-lasso

## descriptant

### fista_lasso — mod.rs

A rectangular 1..256 by 1..64, entries/response/lambda magnitudes <=1e6, lambda>=0. A nonzero requires ||A||F²>=1e-12; zero A accepted. The solution may be nonunique.

Report.x with objective .5||Ax-b||²+lambda||x||1, KKT infinity residual, update count and Converged/BudgetExhausted. Error is InvalidInput, AllocationFailed or NonFiniteArithmetic.

Original implementation and tests. Identifiable mathematical sources are listed in model.md#prospective and production source; no copied code. Retain the prospective predictions as hypotheses until measured. This epoch explicitly distinguishes residual and solution accuracy.

## model

### Mathematical combiner

Raw source plus partial syntax identifies declarations; forward equations and the ordered typed family constrain admissible observations; tests challenge selected equations and boundaries. Neither syntax nor the model proves Rust semantics.

FISTA state (x,y,t) starts (0,0,1). Gradient evaluated at y; reports evaluate x. t'=(1+sqrt(1+4t²))/2, y'=x'+(t-1)/t'*(x'-x). First two x iterates equal ISTA; the third distinguishes acceleration. Objective need not decrease at each update.

Dependencies: projected_gradient_box report/buffer and soft_threshold. FISTA additionally composes proximal_gradient_lasso validation/evaluation/prox; all helper preconditions are established by shared validation, with scale=0 handled by converged zero-design stopping.

History: prospective model saved before source; first implementation uses original Rust, existing report types and checked buffers, and the existing soft threshold for LASSO. No measured performance gain or executable surrogate has been established.

## approximant

### Forward model and invariant

A rectangular 1..256 by 1..64, entries/response/lambda magnitudes <=1e6, lambda>=0. A nonzero requires ||A||F²>=1e-12; zero A accepted. The solution may be nonunique.

FISTA: t_0=1,y_0=x_0=0; x_next=soft_threshold(y-A'(Ay-b)/L,lambda/L); t_next=(1+sqrt(1+4t²))/2; y_next=x_next+((t-1)/t_next)*(x_next-x). L=||A||_F². Zero design returns zero.

Report.x with objective .5||Ax-b||²+lambda||x||1, KKT infinity residual, update count and Converged/BudgetExhausted. Error is InvalidInput, AllocationFailed or NonFiniteArithmetic.

O(m*n*(k+1)) time with two gradient/objective evaluations per update, O(n) storage. Three initial checked vectors and one soft_threshold output per update. Shares validation/evaluation/prox with ISTA; distinct t,y recurrence.

Invariant: reported residual belongs to returned x; convergence precedes exhaustion including equality; iterations counts committed updates. Validation is complete before vector allocation. No callback assumptions or evaluations. No Stagnated return is inferred from one unchanged point. Input slices remain unchanged.

Tolerance is absolute, finite and positive; budget 0..100000. All index reductions use deterministic ascending order. Real equations are models, not f64 accuracy proofs. No generic percentage guarantee. Executable 1/2/5/10% surrogates are unsupported pending a useful metric and evidence.

## models

# Typed obernant family

## behavior_complement
For fixed valid input, success with wrong column dimension, nonfinite x/objective/KKT residual, iterations>budget, or Converged with KKT violation>tolerance is excluded. With zero budget only x=0 is compatible. For all-zero A every valid report is zero at iteration 0, value=.5||b||². Invalid shape cannot map to success.

## reverse_reconstruction
At zero budget the zero vector has preimages from all valid A,b,lambda with status determined by their KKT residual. At stationarity, gradients belong to -lambda*sign(x) on nonzeros and [-lambda,lambda] on zeros; this relation recovers compatible responses, not a unique one. For rank-deficient A, adding a response component in null(A') preserves gradient but changes objective. Even retaining objective permits sign choices of such components. Threshold output 0 has a whole interval [-lambda/L,lambda/L] of pre-proximal coordinates. FISTA t and extrapolated y cannot be recovered from x alone.

## iteration_state
FISTA state (x,y,t) starts (0,0,1). Gradient evaluated at y; reports evaluate x. t'=(1+sqrt(1+4t²))/2, y'=x'+(t-1)/t'*(x'-x). First two x iterates equal ISTA; the third distinguishes acceleration. Objective need not decrease at each update.

## termination_partition
Validation error precedes allocation and iteration. Recompute residual at iteration k; r<=tol returns Converged even at k=budget, otherwise k=budget returns BudgetExhausted. No Stagnated result is emitted. Small residual does not bound coordinate error without conditioning and rounding analysis.

## finite_arithmetic
All public operands are finite and bounded. Quadratic gradients, norm, step statistics and candidate coordinates are checked. LASSO checks objective, gradients, coordinates, KKT violation and extrapolation; shared soft_threshold rejects nonfinite pre-prox points. Inputs below squared-scale threshold are rejected, including nonzero designs whose squares underflow to zero. Underflow/cancellation inside allowed arithmetic may change branch decisions; no analytical f64 guarantee is claimed.

## resource_observation
O(m*n*(k+1)) time with two gradient/objective evaluations per update, O(n) storage. Three initial checked vectors and one soft_threshold output per update. Shares validation/evaluation/prox with ISTA; distinct t,y recurrence. Checked reserve failures map to AllocationFailed, and callers cannot infer deterministic memory availability. No callbacks, I/O, random state or mutation of input. Allocation-failure branch is specified but not fault-injected.

## tests

### Executable checks

cargo test -p mova-algorithms fista_lasso::tests

Five named tests per algorithm: hand-derived recurrence; independent optimum grid; validation precedence; termination/shape boundaries; small-gradient ambiguity or correlated/nonunique KKT fixture. Validation receipts record repeated debug/release and mutation results. Allocation-failure injection and universal floating accuracy are absent.

## prospective

### Prospective model
Saved before implementation.

FISTA: t_0=1,y_0=x_0=0; x_next=soft_threshold(y-A'(Ay-b)/L,lambda/L); t_next=(1+sqrt(1+4t²))/2; y_next=x_next+((t-1)/t_next)*(x_next-x). L=||A||_F². Zero design returns zero.

Minimize .5||Ax-b||²+lambda||x||1. Rows 1..256, columns 1..64; finite input magnitudes <=1e6; lambda nonnegative <=1e6. KKT infinity violation and objective recomputed at returned x. Nonzero design with Frobenius squared norm below 1e-12 is rejected to avoid an unusable tiny-scale step.
Finite positive tolerance; iteration cap <=100000, zero permitted. Validation precedes allocation. Checked reserve failure is separate from invalid inputs and nonfinite arithmetic. No callbacks. Deterministic index order, exact tie at residual=tolerance counts converged, and convergence precedes budget exhaustion. No premature stagnation claim for accelerated history.

Cost O(rows*columns) per step, O(rows+columns) auxiliary storage; no dense inverse. Expected benefit: acceleration of convex proximal iteration. This is a hypothesis, not timing evidence. Residual convergence is not solution-error certification, especially under poor conditioning or nonuniqueness. f64 rounding, cancellation and underflow are not covered by real-number convergence identities. No 1/2/5/10% executable profile is justified yet.

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

Source SHA256: 9b9856eab9c48dd6af4179d8e5c2fcc76da59268a1c0466d79920144079e8829.

Receipts below use $MOVA_DATA/runs/octopus-wave25-optimization-2026-10-30/. SHA256 binds the observed logs; logs remain external to the source repository.

- fista_lasso-debug-1.log: 9169929ae35506c207790e8bc858ffa3aa5448f4209c7ad3817ad874c3aea05c

- fista_lasso-debug-2.log: 76b00c4833da1db81736a2563f6e792ed159a4cd3f2b6216610a019dd27d29dc

- fista_lasso-release-1.log: 6978e7c5fab3a18a504af254904d0fd5b4a9df8ecbfe14cce150d51919fc9670

- fista_lasso-release-2.log: 343c9c73154c3540db8ada2d6a45779c26a296e9864e4ed48a00715ab9becca7

- fista_lasso-mutant.log: dbf4d04739c0793db20442cf43a709cd89e1d4a12a0c184d13fafc61fc30dabd

- fista_lasso-restored.log: ec15b309edeb5b4e10b750349c33c8389b776f7726f948099268b8e25b9d614f

- fista_lasso-normalized.txt: b936691836da64f8ce919a1c7209460a397b81427d41f3d40360ed6e16d64e4a
