# optimization-methods

## model

# optimization-methods

## forward

Bounded scalar minimization and sufficient-decrease search, box/simplex quadratic optimization, and coordinate Lasso descent.

## invariant

Quadratic objective is .5*xQx-linear*x; Lasso is .5||Ax-y||²+lambda||x||1. Scalar bracket, feasible set, stationarity measure and termination status remain tagged.

## cost

Scalar O(k) callbacks; dense quadratic O(k*n²), Lasso O(k*m*n) plus objective/residual checks; leaf dimensions and iteration caps apply.

## composition

Directional quadratic restrictions connect Armijo to Brent; QR normal equations connect least squares to constrained optimization on fixtures with feasible minimizer; orthogonal Lasso reduces to soft threshold.

## epoch-memory

Wave20. Models retain real-arithmetic assumptions separately from native finite checks. Kernels are library tools, not automatically adopted into training or inference.

## checks

["qr_and_constrained_optimizers_join_known_quadratic","brent_and_armijo_join_scalar_quadratic","lasso_and_triangular_identity_join_soft_threshold_solution"]

## limitations

Scalar unimodality/differentiability are caller premises. Projected step residual and FW floating gap are not unconditional distance/certified objective bounds. Tightened PSD admission rejects some valid PSD matrices.

## catalog

# Typed family relations

Each relation keeps its own domain and assurance.

## behavior-complement

Type: behavior_complement.

For real convex quadratic minimization on C, exclude infeasible x or b-Q*x outside N_C(x); for Lasso exclude Aᵀ(A*x-y)+lambda*s!=0 for every s in the L1 subgradient. For a supplied scalar restriction, exclude an Armijo step violating its sufficient-decrease inequality and a claimed global Brent minimizer outside the stipulated unimodal target. Native finite reports instead obey rounded transition/stopping relations: box projected-step residual, simplex FW gap and Lasso KKT residual are distinct. Exclude switching the minus-linear convention, using objective error as coordinate error, exceeding callback budget or relabeling exhausted/stagnated reports. Native Converged does not assert the exact real KKT equalities.

## reverse-reconstruction

Type: reverse_reconstruction.

For fixed real Q,C and feasible optimizer x, compatible linear terms lie in Q*x+N_C(x); thus a boundary optimizer generally does not identify b. Identity-design Lasso with lambda fixed gives y_i=x_i+lambda*sign(x_i) for nonzero x_i, while x_i=0 admits every y_i in [-lambda,lambda]. General designs admit y with Aᵀ(y-A*x) in lambda*partial||x||1, including null(Aᵀ) perturbations. Brent and Armijo observations retain many callbacks agreeing at the sampled points but differing elsewhere; a step does not recover its direction or original objective. These are real target fibers; matching a native report additionally requires matching its rounded path, validation and status.

## line-restriction

Type: directional_derivative_transport.

For q(x)=.5*xQx-b*x and p, derivative at t0 is pᵀ(Qx-b); Armijo requires negative supplied slope and evaluates q(x+t*p). Brent shares the same restricted objective under a valid unimodal bracket.

## normal-cone

Type: constrained_stationarity_relation.

Real box optimum obeys 0 in Qx-b+N_box(x); simplex FW gap=max_vertex gradient·(x-vertex) vanishes at optimum for convex q. Different residuals cannot be equated as numerical error.

## lasso-threshold

Type: orthogonal_proximal_relation.

Identity design yields x_i=sign(y_i)max(|y_i|-lambda,0). Nonzero outputs reconstruct y by adding lambda sign; zero output retains interval[-lambda,lambda]. General design couples coordinates.

## wave20-evidence

### Family evidence

This family binds ten implementation/test sources. Named joins ["qr_and_constrained_optimizers_join_known_quadratic","brent_and_armijo_join_scalar_quadratic","lasso_and_triangular_identity_join_soft_threshold_solution"] in algorithms/tests/composition_wave20.rs pass four times within the 695-check algorithm suite.

Forward: Bounded scalar minimization and sufficient-decrease search, box/simplex quadratic optimization, and coordinate Lasso descent.

Invariant: Quadratic objective is .5*xQx-linear*x; Lasso is .5||Ax-y||²+lambda||x||1. Scalar bracket, feasible set, stationarity measure and termination status remain tagged.

Cost: Scalar O(k) callbacks; dense quadratic O(k*n²), Lasso O(k*m*n) plus objective/residual checks; leaf dimensions and iteration caps apply.

Assumptions: Scalar unimodality/differentiability are caller premises. Projected step residual and FW floating gap are not unconditional distance/certified objective bounds. Tightened PSD admission rejects some valid PSD matrices.

[Integrated evidence](../algorithm-library.groupant/model.md#wave20-evidence) records actual receipts. Leaf oracles remain separate from cross-implementation agreement; family membership cannot verify unchecked callback premises or a shared error budget.

## History

### 2026-10-30 · epoch-memory

Wave20, 2026-10-30. Models retain real-arithmetic assumptions separately from native finite checks. Kernels are library tools, not automatically adopted into training or inference.

### 2026-10-30 · wave20-evidence

Heading `### Family evidence — 2026-10-30` stays in wave20-evidence as `### Family evidence`.
