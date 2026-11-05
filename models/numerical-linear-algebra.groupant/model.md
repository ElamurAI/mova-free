# numerical-linear-algebra

## model

# numerical-linear-algebra

## forward

Factor bounded symmetric systems, solve triangular systems, minimize linear least squares and iterate linear solves with fresh residuals.

## invariant

Dimensions and coordinates agree; residual, reconstruction residual and normal residual are different observations; explicit errors/statuses retained.

## cost

Cholesky O(n³), triangular O(n²), QR O(m*n²), stationary O(k*n²); O(m*n+n²) inputs/work; n<=64,m<=128,iterations<=10000.

## composition

Cholesky A=L*Lᵀ transports b through lower and upper solves. QR normal equations transport least squares to a quadratic. Stationary solvers share Ax=b but not trajectories.

## epoch-memory

Wave20. Models retain real-arithmetic assumptions separately from native finite checks. Kernels are library tools, not automatically adopted into training or inference.

## checks

["factor_triangular_and_stationary_solvers_join_spd_system","qr_and_constrained_optimizers_join_known_quadratic"]

## limitations

Positive pivots in floating arithmetic do not certify spectral conditioning. Unpivoted QR rejects exact zero encountered pivots only. Stationary convergence requires matrix properties not inferred from finite success.

## catalog

# Typed family relations

Each relation keeps its own domain and assurance.

## behavior-complement

Type: behavior_complement.

At the real-model level, exclude nontriangular Cholesky factors or A!=L*Lᵀ, triangular solutions with T*x!=b, and least-squares candidates with Aᵀ(A*x-b)!=0. These exact exclusions describe targets, not an exact-equality acceptance test for f64 outputs. The native relation excludes reports differing from rounded factor/solve transitions, incorrect recomputed reconstruction/primal/normal residuals, or a stationary Converged status when its computed residual exceeds tolerance. A small native residual does not exclude large coordinate error on ill-conditioned inputs; numerical rank is not certified by unpivoted QR.

## reverse-reconstruction

Type: reverse_reconstruction.

For fixed real triangular T and observed x, compatible right sides satisfy b=T*x; allowing T to vary gives many systems with the same solution. For fixed full-column-rank A, the real QR least-squares fiber for x is b=A*x+z with Aᵀz=0, so nonzero orthogonal residuals remain compatible. A real positive-diagonal Cholesky factor uniquely determines A=L*Lᵀ, while rounded native factor/report fibers can include neighboring matrices. Stationary output plus residual/status leaves many initial vectors, budgets and systems compatible; native antecedents must reproduce the rounded execution, not merely these real identities.

## factor-transport

Type: factorization_solve_relation.

For real SPD A=L*Lᵀ, solving Ly=b then Lᵀx=y solves Ax=b. Native fixtures test independent [1,2] coordinates, with finite factor and rounding residuals retained.

## normal-equations

Type: least_squares_stationarity_relation.

Aᵀ(Ax-b)=0 characterizes real least squares; unique x additionally requires full column rank. Forming AᵀA can square condition number; the harness's fixed well-conditioned design does not certify arbitrary data.

## conditioning

Type: conditional_residual_error.

For nonsingular A, ||x-x*||<=||A^-1||*||Ax-b|| in real arithmetic. Without certified inverse norm and roundoff accounting, reported residuals give no universal solution-error percentage.

## wave20-evidence

### Family evidence

This family binds ten implementation/test sources. Named joins ["factor_triangular_and_stationary_solvers_join_spd_system","qr_and_constrained_optimizers_join_known_quadratic"] in algorithms/tests/composition_wave20.rs pass four times within the 695-check algorithm suite.

Forward: Factor bounded symmetric systems, solve triangular systems, minimize linear least squares and iterate linear solves with fresh residuals.

Invariant: Dimensions and coordinates agree; residual, reconstruction residual and normal residual are different observations; explicit errors/statuses retained.

Cost: Cholesky O(n³), triangular O(n²), QR O(m*n²), stationary O(k*n²); O(m*n+n²) inputs/work; n<=64,m<=128,iterations<=10000.

Assumptions: Positive pivots in floating arithmetic do not certify spectral conditioning. Unpivoted QR rejects exact zero encountered pivots only. Stationary convergence requires matrix properties not inferred from finite success.

[Integrated evidence](../algorithm-library.groupant/model.md#wave20-evidence) records actual receipts. Leaf oracles remain separate from cross-implementation agreement; family membership cannot verify unchecked callback premises or a shared error budget.

## History

### 2026-10-30 · epoch-memory

Wave20, 2026-10-30. Models retain real-arithmetic assumptions separately from native finite checks. Kernels are library tools, not automatically adopted into training or inference.

### 2026-10-30 · wave20-evidence

Heading `### Family evidence — 2026-10-30` stays in wave20-evidence as `### Family evidence`.
