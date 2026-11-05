# accelerated-optimization

## model

# accelerated-optimization

## forward

Run three distinct positive-diagonal quadratic recurrences and two LASSO proximal recurrences with recomputed residuals, explicit finite budgets, and tagged reports/errors. These are alternatives sharing mathematical targets, not a serial optimizer pipeline.

## invariant

Quadratics minimize .5*x'Dx-b'x and report Euclidean gradient norm; LASSO minimizes .5||Ax-b||²+lambda||x||1 and reports objective plus KKT infinity violation. Returned metrics belong to returned x. Convergence takes precedence over exhaustion, including zero budget; no one-step stagnation inference.

## cost

Each diagonal method O(n(k+1)) time/O(n) memory, two initial checked buffers and no per-step allocations. ISTA O(m*n*(k+1))/O(n) working memory, two initial vectors and one soft_threshold output per update. FISTA has three initial vectors, one prox output per update and two gradient/objective evaluations per update. k<=100000; caller matrix storage is additional. No timing gain claimed.

## composition

Diagonal methods reuse gradient_descent_quadratic Error/Report/Status, not its iteration. ISTA reuses projected_gradient_box buffer and report types plus soft_threshold; FISTA additionally calls ISTA validate/evaluate/proximal_step. Diagonal optimum b_i/D_i supplies a shared oracle; identity-design LASSO supplies soft-threshold oracle. None of these joins equates trajectories, stopping iterations, or residual units.

## epoch-memory

Wave25. Models retain real-arithmetic assumptions separately from native finite checks. Kernels are library tools, not automatically adopted into training or inference.

## checks

Named joins: diagonal_stationarity_join, lasso_soft_threshold_join. See ../algorithm-library.groupant/model.md#wave25-evidence and wave25-validation.md there for integrated receipts. These separately tagged tests do not imply a serial runtime pipeline.

## limitations

Real strong-convexity/acceleration identities do not certify f64 error, monotonic objective or convergence by a fixed budget. Frobenius scale is a real Lipschitz bound; cancellation/underflow and checked nonfinite states remain observable. Checked allocation errors are distinct from invalid input. No callbacks or executable percentage profiles.

## catalog

# Typed family relations

Each relation keeps its own domain and assurance.

## behavior-complement

Type: behavior_complement.

Exclude wrong dimension, nonfinite successful metrics, updates beyond budget, Converged when the recomputed metric exceeds tolerance, or relabeling the quadratic Euclidean metric as the LASSO infinity metric. For zero budget diagonal x must equal x0 and proximal x must be zero. A report satisfying real KKT conditions but not the specified rounded recurrence/status is not automatically a permitted native result.

## reverse-reconstruction

Type: reverse_reconstruction.

For any admissible diagonal D and z with b=Dz and x0=z in bounds, zero-update solution z has many compatible objectives. Identity LASSO optimum z_i!=0 implies response_i=z_i+lambda*sign(z_i), while z_i=0 admits response_i in [-lambda,lambda] in reals. General designs admit null(A') response perturbations; retaining objective still permits different equal-norm orthogonal components. Native fibers additionally reproduce rounded history and status; x alone does not recover momentum or t.

## diagonal-common-target

Type: stationary_target_transport.

For positive diagonal D, the unique real minimizer is x*=D^-1 b and gradient g=D(x-x*). BB, heavy ball and Nesterov target this same vector; their finite trajectories differ. If mu=min D, real ||x-x*||2<=||g||2/mu; applying this to native reports requires accounting for residual roundoff.

## momentum-state

Type: history_sensitive_recurrence.

Heavy ball uses alpha=4/(sqrt(L)+sqrt(mu))² and beta=((sqrt(L)-sqrt(mu))/(sqrt(L)+sqrt(mu)))² at x. Nesterov uses y=x+beta(x-previous), beta without squaring, then gradient at y and step1/L. Moving the gradient evaluation to x or interchanging beta values is a rejected join.

## spectral-step

Type: safeguarded_secant_relation.

BB1 uses s=x-previous and alpha=(s.s)/(s.Ds) clipped to [1/L,1/mu]; first step and zero-denominator fallback use1/L. This is not heavy-ball momentum. In real arithmetic positive D places the quotient in that interval; rounded zero sy invokes the specified fallback.

## proximal-shared-kernel

Type: dependency_contract.

ISTA/FISTA share validation, objective/KKT evaluation and soft_threshold(point-gradient/L,lambda/L). Zero A has zero KKT violation at zero and must stop before dividing by L=0. A tiny nonzero design whose squared norm is below1e-12 is rejected, so a zero-design shortcut must not admit it.

## acceleration-observation

Type: rejected_trajectory_equivalence.

FISTA state (x,y,t) starts (0,0,1); t'=(1+sqrt(1+4t²))/2 and y'=x'+((t-1)/t')*(x'-x). Its first two actual x updates match ISTA for identical input, while later updates can differ. Objective nonmonotonicity is permitted; an unchanged x does not imply unchanged momentum.

## termination-partition

Type: status_transport.

Validation precedes buffers. At each committed iterate recompute metric, return Converged for residual<=tol before checking budget, otherwise return BudgetExhausted at budget. Reports must retain their original metric and error type across shared helpers; finite observed convergence is not a universal accuracy certificate.

## wave25-evidence

# accelerated-optimization: wave25 evidence

## checks (wave25-evidence)

Named joins: diagonal_stationarity_join, lasso_soft_threshold_join. See ../algorithm-library.groupant/model.md#wave25-evidence and wave25-validation.md there for integrated receipts. These separately tagged tests do not imply a serial runtime pipeline.

## History

### 2026-10-30 · epoch-memory

Wave25, 2026-10-30. Models retain real-arithmetic assumptions separately from native finite checks. Kernels are library tools, not automatically adopted into training or inference.

### 2026-10-30 · checks

2026-10-30: this family adds 25 source-local tests and 2 independent composition observations. The full 175-module library passed 782 checks twice in debug and twice in release with identical normalized outcomes. All five production mutants in this family failed named assertions; exact restoration and positive checks followed. Root inspected the failure receipts. Tests are finite observations, not universal semantic or floating-point proofs.

### 2026-10-30 · checks (wave25-evidence)

2026-10-30: this family adds 25 source-local tests and 2 independent composition observations. The full 175-module library passed 782 checks twice in debug and twice in release with identical normalized outcomes. All five production mutants in this family failed named assertions; exact restoration and positive checks followed. Root inspected the failure receipts. Tests are finite observations, not universal semantic or floating-point proofs.
