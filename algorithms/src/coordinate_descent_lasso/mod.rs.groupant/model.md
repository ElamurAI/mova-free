# algorithm-coordinate-descent-lasso

## descriptant

### coordinate_descent_lasso implementation

Minimize .5||Ax-y||²+lambda||x||1 from x=0. For each cyclic coordinate j, q_j=sum_i A_ij², correlation=A_j'r+q_j*x_j, x_new=soft_threshold(correlation,lambda)/q_j, and r<-r-A_j*(x_new-x_j). Zero columns remain zero. Recompute r=y-Ax each sweep before objective and KKT stopping, limiting cumulative cached-residual drift.

#### Prospective epoch

The prospective model was saved before source creation in the external wave20 optimization run and retained in model.md#prospective. Original standard-library Rust, no downloaded solution code.

#### Observations retained

17 orthogonal designs against scalar analytic shrinkage; correlated design against 1681 grid objectives and KKT tolerance; true zero column; zero budget with/without optimal zero; negative penalty and NaN; deterministic repeat; nonzero underflowing column rejection.

#### Limits

Maximum recomputed KKT violation<=tolerance precedes budget exhaustion. Budget counts full sweeps. A full sweep without any representable coefficient change returns Stagnated, retaining the already recomputed objective and residual at the unchanged vector. No timing or universal machine-accuracy claim.

Reference: Friedman, Hastie and Tibshirani, Regularization Paths for Generalized Linear Models via Coordinate Descent, Journal of Statistical Software 33(1), 2010; Parikh and Boyd (2014), section 6.5.2.

## model

### Mathematical combiner

Inputs are the authoritative mod.rs, source-bound partial syntax, forward relation, typed complement first and reverse reconstruction second, specialized relations and named finite evidence. Intersect the accepted-domain relation with each update relation, then partition termination by convergence, budget, stagnation or error. Minimize .5||Ax-y||²+lambda||x||1 from x=0. For each cyclic coordinate j, q_j=sum_i A_ij², correlation=A_j'r+q_j*x_j, x_new=soft_threshold(correlation,lambda)/q_j, and r<-r-A_j*(x_new-x_j). Zero columns remain zero. Recompute r=y-Ax each sweep before objective and KKT stopping, limiting cumulative cached-residual drift.

Tests are sibling source nodes; manifest children are absent because these compact files have no independent nested fragment contracts. Syntax does not expand macros or prove semantics.

Retained observation: 17 orthogonal designs against scalar analytic shrinkage; correlated design against 1681 grid objectives and KKT tolerance; true zero column; zero budget with/without optimal zero; negative penalty and NaN; deterministic repeat; nonzero underflowing column rejection.

Composition limits: Validate all inputs before checked x/residual/norm allocations. Each threshold call can separately fail allocation; map that case to AllocationFailed and other threshold errors to NonFiniteArithmetic. Fixed ascending cyclic order, no callbacks. Shared bounded/buffer/report types compose with the standalone threshold kernel. Maximum recomputed KKT violation<=tolerance precedes budget exhaustion. Budget counts full sweeps. A full sweep without any representable coefficient change returns Stagnated, retaining the already recomputed objective and residual at the unchanged vector.

## approximant

### Forward approximant and invariant

Minimize .5||Ax-y||²+lambda||x||1 from x=0. For each cyclic coordinate j, q_j=sum_i A_ij², correlation=A_j'r+q_j*x_j, x_new=soft_threshold(correlation,lambda)/q_j, and r<-r-A_j*(x_new-x_j). Zero columns remain zero. Recompute r=y-Ax each sweep before objective and KKT stopping, limiting cumulative cached-residual drift.

#### Domain and invariant

Rectangular 1..256 by 1..64 design; finite data and lambda abs<=1e50, lambda>=0; positive finite tolerance and budget<=100000. Nonfinite correlation, coefficient, residual, objective or violation returns NonFiniteArithmetic. Underflowed nonzero column norm is explicitly rejected. Remaining rounding/cancellation is not certified by real KKT identities.

The scalar threshold is reused from soft_threshold, not copied. Cached residual changes only by the updated column times delta; in real arithmetic this preserves r=y-Ax and coordinate minimization does not increase the objective. Full residual recomputation determines the next reported objective/KKT norm. Zero norm with any nonzero column entry is an underflow failure, not a genuine zero column.

#### Cost

O(k*m*n) arithmetic, O(m+n) scratch; m<=256,n<=64. Existing soft_threshold allocates one scalar vector per nonzero-coordinate update; up to k*n tiny allocations, explicitly not an allocation-free optimization. Validate all inputs before checked x/residual/norm allocations. Each threshold call can separately fail allocation; map that case to AllocationFailed and other threshold errors to NonFiniteArithmetic. Fixed ascending cyclic order, no callbacks. Shared bounded/buffer/report types compose with the standalone threshold kernel.

#### Executable approximation gap

No useful percentage-bounded cheaper surrogate is attached. A sampled decrease, projected step, bracket width or KKT/dual residual is not a universal relative objective/solution certificate. No 1/2/5/10% claim is inferred from finite success; zero-reference relative error is undefined without a separately specified metric. This gap is explicit rather than wrapping the exact implementation.

Reference: Friedman, Hastie and Tibshirani, Regularization Paths for Generalized Linear Models via Coordinate Descent, Journal of Statistical Software 33(1), 2010; Parikh and Boyd (2014), section 6.5.2.

## models

# Typed obernant family

These are scoped partial relations, not executable inverses or semantic proofs.

## behavior_complement

A terminal nonzero coordinate whose computed |g_j+lambda*sign(x_j)| exceeds tolerance cannot be reported Converged. Zero coordinates instead require |g_j|<=lambda+tolerance. A small KKT residual alone cannot prove coefficient error, especially with correlated or rank-deficient columns. Nonunique minimizers are allowed.

## reverse_reconstruction

For an observed nonzero coordinate update in exact arithmetic, correlation=q_j*x_new+lambda*sign(x_new). Zero output admits the whole correlation interval [-lambda,lambda]. Residual updates recover the previous residual only if the previous coefficient and column are supplied; the report alone cannot reconstruct y or the coordinate history. Nullspace-equivalent predictions may have multiple Lasso minimizers.

## update_relation

The scalar threshold is reused from soft_threshold, not copied. Cached residual changes only by the updated column times delta; in real arithmetic this preserves r=y-Ax and coordinate minimization does not increase the objective. Full residual recomputation determines the next reported objective/KKT norm. Zero norm with any nonzero column entry is an underflow failure, not a genuine zero column.

## termination_partition

Maximum recomputed KKT violation<=tolerance precedes budget exhaustion. Budget counts full sweeps. A full sweep without any representable coefficient change returns Stagnated, retaining the already recomputed objective and residual at the unchanged vector.

## finite_arithmetic

Rectangular 1..256 by 1..64 design; finite data and lambda abs<=1e50, lambda>=0; positive finite tolerance and budget<=100000. Nonfinite correlation, coefficient, residual, objective or violation returns NonFiniteArithmetic. Underflowed nonzero column norm is explicitly rejected. Remaining rounding/cancellation is not certified by real KKT identities.

## resource_observation

Validate all inputs before checked x/residual/norm allocations. Each threshold call can separately fail allocation; map that case to AllocationFailed and other threshold errors to NonFiniteArithmetic. Fixed ascending cyclic order, no callbacks. Shared bounded/buffer/report types compose with the standalone threshold kernel.


## tests

### Executable checks

Run `cargo test -p mova-algorithms coordinate_descent_lasso::tests`.

17 orthogonal designs against scalar analytic shrinkage; correlated design against 1681 grid objectives and KKT tolerance; true zero column; zero budget with/without optimal zero; negative penalty and NaN; deterministic repeat; nonzero underflowing column rejection.

Independent oracle: Orthogonal columns decouple, giving x_j=sign(A_j'y)*max(|A_j'y|-lambda,0)/q_j. The independent scalar formula verifies coefficient values. The dense grid is only a finite objective comparison, while reported KKT checks validate its chosen stopping metric.

Compiled negative control: Pass zero lambda to production soft_threshold; orthogonal_oracle must reject the compiled wrong shrinkage coefficients.

No allocator failure injection, all-input proof or benchmark is claimed.

## prospective

### Prospective optimization model
Before source creation. Brent maintains unimodal bracket and three sampled points; accepts safeguarded parabolic proposals, otherwise golden contraction. O(k) callbacks, O(1) storage. Deterministic callback is assumed; strict unimodality cannot be checked. Armijo shrinks positive alpha by rho until f(alpha)<=f0+c*alpha*slope, slope<0. O(k) callbacks, O(1) space; supplied f0 and slope are caller premises.
Dense quadratics f=.5*x'Qx-b'x use symmetric diagonally dominant nonnegative Q, dimension<=64, finite magnitudes<=1e50. This is a sufficient restricted PSD domain, not arbitrary convex quadratics. PG clips x-g/L into finite bounds with L=max absolute row sum; mapping residual is max |x-clip(x-g/L)|. FW on unit simplex selects earliest minimum gradient vertex, gap=g'(x-s), step=min(1,gap/(d'Qd)); linear curvature uses step 1. O(kn²) time O(n) scratch. These real identities are not floating error guarantees.
Lasso minimizes .5||Ax-y||²+lambda||x||1 by cyclic exact coordinate minimization and residual maintenance, using existing soft threshold kernel. Up to 256 rows and 64 columns. O(kmn) time O(m+n) space. Stop on recomputed KKT max violation. Zero columns set zero. Deterministic ties and iteration orders. All validation precedes work; checked allocation and nonfinite arithmetic explicit; budget<=100000, tolerance positive finite. Each routine distinguishes convergence, budget exhaustion and representational stagnation. No percent objective/solution bound. Expected algorithmic advantage: interpolation avoids golden-only work on smooth functions; cached residual updates avoid recomputing predictions per coordinate. No measured speed claim. Executable approximate surrogate deferred: residual and finite samples lack a justified universal supported percentage metric.
References: Brent, Algorithms for Minimization without Derivatives (1973); Armijo (1966); Nocedal and Wright (2006); Friedman et al., Regularization Paths for Generalized Linear Models via Coordinate Descent (2010); Frank and Wolfe (1956).

## format-epoch

### Integration formatting

## validation

### Validation

Five independently compiled production mutants are rejected by named runtime assertions; source bytes restored before the positive rerun. This source's control: Pass zero lambda to production soft_threshold; orthogonal_oracle must reject the compiled wrong shrinkage coefficients.

Fresh syntax projections are generated twice and compared before installing syntax.json. These are partial structural observations, not macro-expanded AST or semantic proofs. No performance or universal <=10% result follows.

## History

### 2026-10-30 · format-epoch

Heading `### Integration formatting epoch — 2026-10-30` stays in format-epoch as `### Integration formatting`.

Only the adjacent use and pub use import lines were reordered by Rust 2024 formatting. A sorted-line comparison confirms all lines are retained. Worker validation remains bound to historical source SHA-256 abd0c0202eda8607624e6fe9b3ddcde53f8872274f89c447a160b62a06372d15; current source SHA-256 9dfe57b0107a548468ebd6712150213f54314b7cc85fb773f135ea974cff36e1. Paired syntax was regenerated for the current source. The integrated wave20 native suite supplies final-source evidence; mathematical contracts and algorithm arithmetic are unchanged.

### 2026-10-30 · validation

Heading `### Validation epoch 2026-10-30` stays in validation as `### Validation`.

External receipts: `$MOVA_DATA/runs/octopus-wave20-optimization-2026-10-30`. Two debug and two release library runs each pass 568 tests, including 21 new native tests, with identical sorted test-name/outcome transcripts. Strict all-target algorithms Clippy and focused rustfmt pass.
