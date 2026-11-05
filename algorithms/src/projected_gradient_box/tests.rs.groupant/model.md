# algorithm-projected-gradient-box-tests

## descriptant

### projected_gradient_box finite checks

The test harness observes finite assertions on projected_gradient_box. Analytic formulas and finite grid competitors are distinct forms of evidence; the grid alone proves no optimizer bound.

#### Prospective epoch

The prospective model was saved before source creation in the external wave20 optimization run and retained in model.md#prospective. Original standard-library Rust, no downloaded solution code.

#### Observations retained

13 diagonal cases compared to clipped analytic minimizers; dense 2x2 stationary optimizer [1,2] and objective -7.5; linear objective with zero Q; zero budget; negative diagonal, asymmetric and empty matrix rejection.

#### Limits

Projected-step residual<=tolerance precedes budget exhaustion. Each update swaps buffers, preserving coordinate order. The equality stagnation guard is presently dominated by the residual-zero convergence test for finite values and positive tolerance; Status::Stagnated is shared but this branch is unreachable in this scoped arithmetic model. No unsupported stagnation coverage is claimed. No timing or universal machine-accuracy claim.

Reference: Bertsekas, Nonlinear Programming (1999); Nocedal and Wright, Numerical Optimization (2006).

## model

### Mathematical combiner

Inputs are the authoritative tests.rs, source-bound partial syntax, forward relation, typed complement first and reverse reconstruction second, specialized relations and named finite evidence. Intersect the accepted-domain relation with each update relation, then partition termination by convergence, budget, stagnation or error. Minimize f(x)=.5*x'Q*x-b'x on lower<=x<=upper. Validate exact symmetry and nonnegative diagonal; off-diagonal row absolute sum must be zero or below diagonal/(1+128*EPSILON), a conservative sufficient PSD domain. L=max(max row absolute sum,1e-100). Clip initial point, then x_next=clip(x-(Qx-b)/L). Residual=max_i|x_i-x_next_i| is a projected-step observation, not gradient norm or solution distance.

Tests are sibling source nodes; manifest children are absent because these compact files have no independent nested fragment contracts. Syntax does not expand macros or prove semantics.

Retained observation: 13 diagonal cases compared to clipped analytic minimizers; dense 2x2 stationary optimizer [1,2] and objective -7.5; linear objective with zero Q; zero budget; negative diagonal, asymmetric and empty matrix rejection.

Composition limits: Dimension/value/matrix validation precedes bound validation, both precede checked allocation of x, gradient, next. Failed try_reserve maps to AllocationFailed. Iteration order ascending, no callbacks. Caller matrix memory O(n²) excluded from scratch cost. Zero Q uses tiny positive scale and often jumps to an active box endpoint. Projected-step residual<=tolerance precedes budget exhaustion. Each update swaps buffers, preserving coordinate order. The equality stagnation guard is presently dominated by the residual-zero convergence test for finite values and positive tolerance; Status::Stagnated is shared but this branch is unreachable in this scoped arithmetic model. No unsupported stagnation coverage is claimed.

## approximant

### Forward approximant and invariant

The test harness observes finite assertions on projected_gradient_box. Analytic formulas and finite grid competitors are distinct forms of evidence; the grid alone proves no optimizer bound.

#### Domain and invariant

n in 1..64, finite operands abs<=1e50, exact rectangular shape and symmetry, finite positive tolerance, budget<=100000; then finite ordered bounds. Nonfinite objective, gradient or raw step is NonFiniteArithmetic. Summed diagonal dominance includes a safety margin but is not a general floating PSD certification.

Diagonal Q has exact optimizer clip(b_i/Q_ii); dense example directly satisfies Q*[1,2]=[5,5]. These independent identities verify location and objective without replaying projected iterations.

#### Cost

O(n²) validation and O(k*n²) arithmetic; O(n) additional storage (three length-n vectors), n<=64. Harness repeats only the named bounded fixture set; no production throughput claim.

#### Executable approximation gap

No useful percentage-bounded cheaper surrogate is attached. A sampled decrease, projected step, bracket width or KKT/dual residual is not a universal relative objective/solution certificate. No 1/2/5/10% claim is inferred from finite success; zero-reference relative error is undefined without a separately specified metric. This gap is explicit rather than wrapping the exact implementation.

Reference: Bertsekas, Nonlinear Programming (1999); Nocedal and Wright, Numerical Optimization (2006).

## models

# Typed obernant family

These are scoped partial relations, not executable inverses or semantic proofs.

## behavior_complement

The observation suite excludes the named wrong coefficients, vertices, brackets and statuses on these fixtures: 13 diagonal cases compared to clipped analytic minimizers; dense 2x2 stationary optimizer [1,2] and objective -7.5; linear objective with zero Q; zero budget; negative diagonal, asymmetric and empty matrix rejection.. It says nothing about callbacks or matrices outside these finite cases; arbitrary unseen bugs remain compatible with all passes.

## reverse_reconstruction

A passing transcript reconstructs only a family of implementations agreeing on these assertions. The analytic oracle is Diagonal Q has exact optimizer clip(b_i/Q_ii); dense example directly satisfies Q*[1,2]=[5,5]. These independent identities verify location and objective without replaying projected iterations.. Many different update orders or untested behaviors have the same pass transcript, so source recovery is not unique.

## independent_oracle

Diagonal Q has exact optimizer clip(b_i/Q_ii); dense example directly satisfies Q*[1,2]=[5,5]. These independent identities verify location and objective without replaying projected iterations.

## mutation_sensitivity

Change shared gradient dot-b to dot+b; diagonal_clipping_oracle must reject the compiled incorrect optimization direction.

## boundary_partition

A zero matrix is valid, while a negative diagonal or asymmetric matrix fails before allocation. Linear box optimum is an endpoint; a zero budget returns the clipped initial point and its observed projected residual.


## tests

### Executable checks

Run `cargo test -p mova-algorithms projected_gradient_box::tests`.

13 diagonal cases compared to clipped analytic minimizers; dense 2x2 stationary optimizer [1,2] and objective -7.5; linear objective with zero Q; zero budget; negative diagonal, asymmetric and empty matrix rejection.

Independent oracle: Diagonal Q has exact optimizer clip(b_i/Q_ii); dense example directly satisfies Q*[1,2]=[5,5]. These independent identities verify location and objective without replaying projected iterations.

Compiled negative control: Change shared gradient dot-b to dot+b; diagonal_clipping_oracle must reject the compiled incorrect optimization direction.

No allocator failure injection, all-input proof or benchmark is claimed.

## prospective

### Prospective optimization model
Before source creation. Brent maintains unimodal bracket and three sampled points; accepts safeguarded parabolic proposals, otherwise golden contraction. O(k) callbacks, O(1) storage. Deterministic callback is assumed; strict unimodality cannot be checked. Armijo shrinks positive alpha by rho until f(alpha)<=f0+c*alpha*slope, slope<0. O(k) callbacks, O(1) space; supplied f0 and slope are caller premises.
Dense quadratics f=.5*x'Qx-b'x use symmetric diagonally dominant nonnegative Q, dimension<=64, finite magnitudes<=1e50. This is a sufficient restricted PSD domain, not arbitrary convex quadratics. PG clips x-g/L into finite bounds with L=max absolute row sum; mapping residual is max |x-clip(x-g/L)|. FW on unit simplex selects earliest minimum gradient vertex, gap=g'(x-s), step=min(1,gap/(d'Qd)); linear curvature uses step 1. O(kn²) time O(n) scratch. These real identities are not floating error guarantees.
Lasso minimizes .5||Ax-y||²+lambda||x||1 by cyclic exact coordinate minimization and residual maintenance, using existing soft threshold kernel. Up to 256 rows and 64 columns. O(kmn) time O(m+n) space. Stop on recomputed KKT max violation. Zero columns set zero. Deterministic ties and iteration orders. All validation precedes work; checked allocation and nonfinite arithmetic explicit; budget<=100000, tolerance positive finite. Each routine distinguishes convergence, budget exhaustion and representational stagnation. No percent objective/solution bound. Expected algorithmic advantage: interpolation avoids golden-only work on smooth functions; cached residual updates avoid recomputing predictions per coordinate. No measured speed claim. Executable approximate surrogate deferred: residual and finite samples lack a justified universal supported percentage metric.
References: Brent, Algorithms for Minimization without Derivatives (1973); Armijo (1966); Nocedal and Wright (2006); Friedman et al., Regularization Paths for Generalized Linear Models via Coordinate Descent (2010); Frank and Wolfe (1956).

## validation

### Validation

Five independently compiled production mutants are rejected by named runtime assertions; source bytes restored before the positive rerun. This source's control: Change shared gradient dot-b to dot+b; diagonal_clipping_oracle must reject the compiled incorrect optimization direction.

Fresh syntax projections are generated twice and compared before installing syntax.json. These are partial structural observations, not macro-expanded AST or semantic proofs. No performance or universal <=10% result follows.

## History

### 2026-10-30 · validation

Heading `### Validation epoch 2026-10-30` stays in validation as `### Validation`.

External receipts: `$MOVA_DATA/runs/octopus-wave20-optimization-2026-10-30`. Two debug and two release library runs each pass 568 tests, including 21 new native tests, with identical sorted test-name/outcome transcripts. Strict all-target algorithms Clippy and focused rustfmt pass.
