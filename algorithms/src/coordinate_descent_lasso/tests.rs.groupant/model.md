# algorithm-coordinate-descent-lasso-tests

## descriptant

### coordinate_descent_lasso finite checks

The test harness observes finite assertions on coordinate_descent_lasso. Analytic formulas and finite grid competitors are distinct forms of evidence; the grid alone proves no optimizer bound.

#### Prospective epoch

The prospective model was saved before source creation in the external wave20 optimization run and retained in model.md#prospective. Original standard-library Rust, no downloaded solution code.

#### Observations retained

17 orthogonal designs against scalar analytic shrinkage; correlated design against 1681 grid objectives and KKT tolerance; true zero column; zero budget with/without optimal zero; negative penalty and NaN; deterministic repeat; nonzero underflowing column rejection.

#### Limits

Maximum recomputed KKT violation<=tolerance precedes budget exhaustion. Budget counts full sweeps. A full sweep without any representable coefficient change returns Stagnated, retaining the already recomputed objective and residual at the unchanged vector. No timing or universal machine-accuracy claim.

Reference: Friedman, Hastie and Tibshirani, Regularization Paths for Generalized Linear Models via Coordinate Descent, Journal of Statistical Software 33(1), 2010; Parikh and Boyd (2014), section 6.5.2.

## model

### Mathematical combiner

Inputs are the authoritative tests.rs, source-bound partial syntax, forward relation, typed complement first and reverse reconstruction second, specialized relations and named finite evidence. Intersect the accepted-domain relation with each update relation, then partition termination by convergence, budget, stagnation or error. Minimize .5||Ax-y||²+lambda||x||1 from x=0. For each cyclic coordinate j, q_j=sum_i A_ij², correlation=A_j'r+q_j*x_j, x_new=soft_threshold(correlation,lambda)/q_j, and r<-r-A_j*(x_new-x_j). Zero columns remain zero. Recompute r=y-Ax each sweep before objective and KKT stopping, limiting cumulative cached-residual drift.

Tests are sibling source nodes; manifest children are absent because these compact files have no independent nested fragment contracts. Syntax does not expand macros or prove semantics.

Retained observation: 17 orthogonal designs against scalar analytic shrinkage; correlated design against 1681 grid objectives and KKT tolerance; true zero column; zero budget with/without optimal zero; negative penalty and NaN; deterministic repeat; nonzero underflowing column rejection.

Composition limits: Validate all inputs before checked x/residual/norm allocations. Each threshold call can separately fail allocation; map that case to AllocationFailed and other threshold errors to NonFiniteArithmetic. Fixed ascending cyclic order, no callbacks. Shared bounded/buffer/report types compose with the standalone threshold kernel. Maximum recomputed KKT violation<=tolerance precedes budget exhaustion. Budget counts full sweeps. A full sweep without any representable coefficient change returns Stagnated, retaining the already recomputed objective and residual at the unchanged vector.

## approximant

### Forward approximant and invariant

The test harness observes finite assertions on coordinate_descent_lasso. Analytic formulas and finite grid competitors are distinct forms of evidence; the grid alone proves no optimizer bound.

#### Domain and invariant

Rectangular 1..256 by 1..64 design; finite data and lambda abs<=1e50, lambda>=0; positive finite tolerance and budget<=100000. Nonfinite correlation, coefficient, residual, objective or violation returns NonFiniteArithmetic. Underflowed nonzero column norm is explicitly rejected. Remaining rounding/cancellation is not certified by real KKT identities.

Orthogonal columns decouple, giving x_j=sign(A_j'y)*max(|A_j'y|-lambda,0)/q_j. The independent scalar formula verifies coefficient values. The dense grid is only a finite objective comparison, while reported KKT checks validate its chosen stopping metric.

#### Cost

O(k*m*n) arithmetic, O(m+n) scratch; m<=256,n<=64. Existing soft_threshold allocates one scalar vector per nonzero-coordinate update; up to k*n tiny allocations, explicitly not an allocation-free optimization. Harness repeats only the named bounded fixture set; no production throughput claim.

#### Executable approximation gap

No useful percentage-bounded cheaper surrogate is attached. A sampled decrease, projected step, bracket width or KKT/dual residual is not a universal relative objective/solution certificate. No 1/2/5/10% claim is inferred from finite success; zero-reference relative error is undefined without a separately specified metric. This gap is explicit rather than wrapping the exact implementation.

Reference: Friedman, Hastie and Tibshirani, Regularization Paths for Generalized Linear Models via Coordinate Descent, Journal of Statistical Software 33(1), 2010; Parikh and Boyd (2014), section 6.5.2.

## models

# Typed obernant family

These are scoped partial relations, not executable inverses or semantic proofs.

## behavior_complement

The observation suite excludes the named wrong coefficients, vertices, brackets and statuses on these fixtures: 17 orthogonal designs against scalar analytic shrinkage; correlated design against 1681 grid objectives and KKT tolerance; true zero column; zero budget with/without optimal zero; negative penalty and NaN; deterministic repeat; nonzero underflowing column rejection.. It says nothing about callbacks or matrices outside these finite cases; arbitrary unseen bugs remain compatible with all passes.

## reverse_reconstruction

A passing transcript reconstructs only a family of implementations agreeing on these assertions. The analytic oracle is Orthogonal columns decouple, giving x_j=sign(A_j'y)*max(|A_j'y|-lambda,0)/q_j. The independent scalar formula verifies coefficient values. The dense grid is only a finite objective comparison, while reported KKT checks validate its chosen stopping metric.. Many different update orders or untested behaviors have the same pass transcript, so source recovery is not unique.

## independent_oracle

Orthogonal columns decouple, giving x_j=sign(A_j'y)*max(|A_j'y|-lambda,0)/q_j. The independent scalar formula verifies coefficient values. The dense grid is only a finite objective comparison, while reported KKT checks validate its chosen stopping metric.

## mutation_sensitivity

Pass zero lambda to production soft_threshold; orthogonal_oracle must reject the compiled wrong shrinkage coefficients.

## boundary_partition

A true zero column stays zero, an underflowing nonzero column fails arithmetic, a high lambda can converge at zero even with zero budget, but low lambda with zero budget exhausts it.


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

## validation

### Validation

Five independently compiled production mutants are rejected by named runtime assertions; source bytes restored before the positive rerun. This source's control: Pass zero lambda to production soft_threshold; orthogonal_oracle must reject the compiled wrong shrinkage coefficients.

Fresh syntax projections are generated twice and compared before installing syntax.json. These are partial structural observations, not macro-expanded AST or semantic proofs. No performance or universal <=10% result follows.

## History

### 2026-10-30 · validation

Heading `### Validation epoch 2026-10-30` stays in validation as `### Validation`.

External receipts: `$MOVA_DATA/runs/octopus-wave20-optimization-2026-10-30`. Two debug and two release library runs each pass 568 tests, including 21 new native tests, with identical sorted test-name/outcome transcripts. Strict all-target algorithms Clippy and focused rustfmt pass.
