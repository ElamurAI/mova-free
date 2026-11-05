# algorithm-brent-minimize-tests

## descriptant

### brent_minimize finite checks

The test harness observes finite assertions on brent_minimize. Analytic formulas and finite grid competitors are distinct forms of evidence; the grid alone proves no optimizer bound.

#### Prospective epoch

The prospective model was saved before source creation in the external wave20 optimization run and retained in model.md#prospective. Original standard-library Rust, no downloaded solution code.

#### Observations retained

17 translated quadratics; absolute-value cusp; monotone endpoint; zero budget and exact callback counts; invalid interval callback not invoked; NaN callback; adjacent floats; repeated exponential fixture.

#### Limits

Width<=tolerance wins over budget exhaustion; budget wins over inability to generate a distinct interior u. A representationally unchanged or exterior candidate yields Stagnated. Endpoint minima are approached by interior samples; value is always one actual sampled value. No timing or universal machine-accuracy claim.

Reference: Brent, Algorithms for Minimization without Derivatives (1973), chapter 5.

## model

### Mathematical combiner

Inputs are the authoritative tests.rs, source-bound partial syntax, forward relation, typed complement first and reverse reconstruction second, specialized relations and named finite evidence. Intersect the accepted-domain relation with each update relation, then partition termination by convergence, budget, stagnation or error. For a deterministic strictly unimodal f on [a,b], retain the sampled best x and two historical samples w,v. A safeguarded interpolating-parabola displacement is accepted only within the bracket and below half the previous displacement; otherwise choose the golden displacement toward the longer side. Comparing f(u) to f(x) removes the opposite interval. Equality promotes u. Strict unimodality is a caller premise, not inferred from samples.

Tests are sibling source nodes; manifest children are absent because these compact files have no independent nested fragment contracts. Syntax does not expand macros or prove semantics.

Retained observation: 17 translated quadratics; absolute-value cusp; monotone endpoint; zero budget and exact callback counts; invalid interval callback not invoked; NaN callback; adjacent floats; repeated exponential fixture.

Composition limits: No internal heap allocation. Validate all scalar configuration before the first callback; each completed iteration adds exactly one evaluation. Stateful callbacks are accepted syntactically but deterministic semantics require equal values at equal arguments. Panics and callback allocations are not caught. Width<=tolerance wins over budget exhaustion; budget wins over inability to generate a distinct interior u. A representationally unchanged or exterior candidate yields Stagnated. Endpoint minima are approached by interior samples; value is always one actual sampled value.

## approximant

### Forward approximant and invariant

The test harness observes finite assertions on brent_minimize. Analytic formulas and finite grid competitors are distinct forms of evidence; the grid alone proves no optimizer bound.

#### Domain and invariant

Endpoints are finite, abs<=1e50, left<right; tolerance positive finite, budget<=100000. Reject callback NaN/infinity separately from nonfinite interpolation arithmetic. Minimum step max(tol/8,EPSILON*abs(x)) prevents most repeated samples; tiny tolerance may still stagnate.

The analytic quadratic optimizer c and abs(x-.137) cusp are independent of interpolation logic. Their tolerances constrain returned location and bracket; the monotone linear oracle constrains the endpoint limit.

#### Cost

O(k) arithmetic and at most k+1 callback evaluations; O(1) internal storage, callback work excluded. Harness repeats only the named bounded fixture set; no production throughput claim.

#### Executable approximation gap

No useful percentage-bounded cheaper surrogate is attached. A sampled decrease, projected step, bracket width or KKT/dual residual is not a universal relative objective/solution certificate. No 1/2/5/10% claim is inferred from finite success; zero-reference relative error is undefined without a separately specified metric. This gap is explicit rather than wrapping the exact implementation.

Reference: Brent, Algorithms for Minimization without Derivatives (1973), chapter 5.

## models

# Typed obernant family

These are scoped partial relations, not executable inverses or semantic proofs.

## behavior_complement

The observation suite excludes the named wrong coefficients, vertices, brackets and statuses on these fixtures: 17 translated quadratics; absolute-value cusp; monotone endpoint; zero budget and exact callback counts; invalid interval callback not invoked; NaN callback; adjacent floats; repeated exponential fixture.. It says nothing about callbacks or matrices outside these finite cases; arbitrary unseen bugs remain compatible with all passes.

## reverse_reconstruction

A passing transcript reconstructs only a family of implementations agreeing on these assertions. The analytic oracle is The analytic quadratic optimizer c and abs(x-.137) cusp are independent of interpolation logic. Their tolerances constrain returned location and bracket; the monotone linear oracle constrains the endpoint limit.. Many different update orders or untested behaviors have the same pass transcript, so source recovery is not unique.

## independent_oracle

The analytic quadratic optimizer c and abs(x-.137) cusp are independent of interpolation logic. Their tolerances constrain returned location and bracket; the monotone linear oracle constrains the endpoint limit.

## mutation_sensitivity

Reverse fu<=fx to fu>=fx; quadratic_oracle must fail its status/location assertion after successful compilation.

## boundary_partition

Adjacent representable endpoints with MIN_POSITIVE tolerance produce Stagnated; a valid zero budget returns BudgetExhausted after one callback, while invalid endpoints invoke none.


## tests

### Executable checks

Run `cargo test -p mova-algorithms brent_minimize::tests`.

17 translated quadratics; absolute-value cusp; monotone endpoint; zero budget and exact callback counts; invalid interval callback not invoked; NaN callback; adjacent floats; repeated exponential fixture.

Independent oracle: The analytic quadratic optimizer c and abs(x-.137) cusp are independent of interpolation logic. Their tolerances constrain returned location and bracket; the monotone linear oracle constrains the endpoint limit.

Compiled negative control: Reverse fu<=fx to fu>=fx; quadratic_oracle must fail its status/location assertion after successful compilation.

No allocator failure injection, all-input proof or benchmark is claimed.

## prospective

### Prospective optimization model
Before source creation. Brent maintains unimodal bracket and three sampled points; accepts safeguarded parabolic proposals, otherwise golden contraction. O(k) callbacks, O(1) storage. Deterministic callback is assumed; strict unimodality cannot be checked. Armijo shrinks positive alpha by rho until f(alpha)<=f0+c*alpha*slope, slope<0. O(k) callbacks, O(1) space; supplied f0 and slope are caller premises.
Dense quadratics f=.5*x'Qx-b'x use symmetric diagonally dominant nonnegative Q, dimension<=64, finite magnitudes<=1e50. This is a sufficient restricted PSD domain, not arbitrary convex quadratics. PG clips x-g/L into finite bounds with L=max absolute row sum; mapping residual is max |x-clip(x-g/L)|. FW on unit simplex selects earliest minimum gradient vertex, gap=g'(x-s), step=min(1,gap/(d'Qd)); linear curvature uses step 1. O(kn²) time O(n) scratch. These real identities are not floating error guarantees.
Lasso minimizes .5||Ax-y||²+lambda||x||1 by cyclic exact coordinate minimization and residual maintenance, using existing soft threshold kernel. Up to 256 rows and 64 columns. O(kmn) time O(m+n) space. Stop on recomputed KKT max violation. Zero columns set zero. Deterministic ties and iteration orders. All validation precedes work; checked allocation and nonfinite arithmetic explicit; budget<=100000, tolerance positive finite. Each routine distinguishes convergence, budget exhaustion and representational stagnation. No percent objective/solution bound. Expected algorithmic advantage: interpolation avoids golden-only work on smooth functions; cached residual updates avoid recomputing predictions per coordinate. No measured speed claim. Executable approximate surrogate deferred: residual and finite samples lack a justified universal supported percentage metric.
References: Brent, Algorithms for Minimization without Derivatives (1973); Armijo (1966); Nocedal and Wright (2006); Friedman et al., Regularization Paths for Generalized Linear Models via Coordinate Descent (2010); Frank and Wolfe (1956).

## validation

### Validation

Five independently compiled production mutants are rejected by named runtime assertions; source bytes restored before the positive rerun. This source's control: Reverse fu<=fx to fu>=fx; quadratic_oracle must fail its status/location assertion after successful compilation.

Fresh syntax projections are generated twice and compared before installing syntax.json. These are partial structural observations, not macro-expanded AST or semantic proofs. No performance or universal <=10% result follows.

## History

### 2026-10-30 · validation

Heading `### Validation epoch 2026-10-30` stays in validation as `### Validation`.

External receipts: `$MOVA_DATA/runs/octopus-wave20-optimization-2026-10-30`. Two debug and two release library runs each pass 568 tests, including 21 new native tests, with identical sorted test-name/outcome transcripts. Strict all-target algorithms Clippy and focused rustfmt pass.
