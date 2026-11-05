# optimization-solvers

## model

# optimization-solvers

## forward

Bracket roots and deterministic strictly unimodal minima; descend diagonal quadratics and solve dense symmetric positive definite linear systems with explicit stopping observations.

## invariant

Preserve finite reports, bracket order, dimensional agreement and distinct converged, exhausted and stagnated states. True residual is recomputed for stopping.

## cost

Sum O(k_b+k_g+n*k_d+n^2*k_c) arithmetic plus callback costs; serial auxiliary O(n), input matrix O(n^2). Hard dimension and iteration limits remain leaf-specific.

## composition

On q(x)=d*x^2/2-b*x with d>0, derivative root, scalar minimizer, diagonal descent and one-dimensional CG share b/d. For a common diagonal system, GD and CG target identical stationarity but need not share iteration counts or statuses.

## epoch-memory

Wave19. Models retain real-arithmetic assumptions separately from native finite checks. Kernels are library tools, not automatically adopted into training or inference.

## checks

["quadratic_solvers_join_scalar_roots_and_polynomial_values"]

## limitations

Continuity, deterministic strict unimodality and global SPD are not established by sampling or visited curvature. Residual magnitude alone does not bound coordinate error without conditioning. Floating-point underflow and conservative arithmetic rejection remain possible.

## catalog

# Typed family relations

Each relation keeps its own domain and assurance.

## behavior-complement

Type: behavior_complement.

Exclude input/report pairs outside each tagged solver relation: exhausted or stagnant reports relabeled as convergence, bracket-width bounds promoted to objective bounds, residual norms promoted to unconditional coordinate error, or visited positive curvature promoted to global SPD.

## reverse-reconstruction

Type: reverse_reconstruction.

A returned estimate and status admit many callbacks, brackets, initial states and budgets. For zero residual x, any positive diagonal D with b=D*x is compatible. Common roots/minima survive positive scaling; output does not uniquely recover the problem.

## scalar-stationarity

Type: stationarity_transport.

For q(x)=d*x^2/2-b*x,d>0 with valid enclosing intervals, q'=d*x-b links bisection, golden-section, GD and CG to the same b/d. Native finite fixtures compare additive coordinates with explicit tolerance, not status equality.

## diagonal-equations

Type: residual_transport.

For the same positive diagonal D and b, gradient D*x-b and linear residual b-D*x differ only by sign. Their Euclidean norms agree in real arithmetic; rounding and solver trajectories remain separate.

## conditioning

Type: conditional_error_bound.

For SPD A with known lambda_min>0, ||x-x*||2 <= ||A*x-b||2/lambda_min in real arithmetic. This group supplies no eigenvalue certificate; without that premise the residual-to-solution bound is absent.

## wave19-evidence

### Family evidence

This family binds eight implementation/test sources through its manifest; views are partial source-derived models. Selected named joins are ["quadratic_solvers_join_scalar_roots_and_polynomial_values"] in algorithms/tests/composition_wave19.rs. Those checks pass four times as part of the 616-check suite. Leaf oracles and compiled production controls remain separately bound in adjacent evidence.

[Integrated evidence](../algorithm-library.groupant/model.md#wave19-evidence) records receipts; conditional laws outside these fixture scopes are not promoted to executed checks. No joint universal percentage bound follows from family membership.

## History

### 2026-10-30 · epoch-memory

Wave19, 2026-10-30. Models retain real-arithmetic assumptions separately from native finite checks. Kernels are library tools, not automatically adopted into training or inference.

### 2026-10-30 · wave19-evidence

Heading `### Family evidence — 2026-10-30` stays in wave19-evidence as `### Family evidence`.
