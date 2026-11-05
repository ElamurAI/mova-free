# algorithm-armijo-line-search-tests

## descriptant

### armijo_line_search finite checks

The test harness observes finite assertions on armijo_line_search. Analytic formulas and finite grid competitors are distinct forms of evidence; the grid alone proves no optimizer bound.

#### Prospective epoch

The prospective model was saved before source creation in the external wave20 optimization run and retained in model.md#prospective. Original standard-library Rust, no downloaded solution code.

#### Observations retained

Eight power-of-two initial steps against an independent scalar backtracking loop; always-rejected callback budget/count; invalid slope; callback infinity; overflowing Armijo bound; minimum-subnormal contraction stagnation.

#### Limits

Converged precedes BudgetExhausted, which precedes Stagnated. Budget counts contractions, so zero still evaluates the initial step. If the next step is zero or equal to the current step, terminate Stagnated without another callback. No timing or universal machine-accuracy claim.

Reference: Armijo, Minimization of functions having Lipschitz continuous first partial derivatives, Pacific Journal of Mathematics 16 (1966).

## model

### Mathematical combiner

Inputs are the authoritative tests.rs, source-bound partial syntax, forward relation, typed complement first and reverse reconstruction second, specialized relations and named finite evidence. Intersect the accepted-domain relation with each update relation, then partition termination by convergence, budget, stagnation or error. Given f0 and negative ray derivative d supplied by the caller, form alpha_j=alpha0*rho^j with 0<rho<1. Return the first tested step satisfying f(alpha)<=f0+c*alpha*d and strict representable decrease f(alpha)<f0. The latter excludes equality caused by rounding. No gradient evaluation or verification of f0/d is attempted.

Tests are sibling source nodes; manifest children are absent because these compact files have no independent nested fragment contracts. Syntax does not expand macros or prove semantics.

Retained observation: Eight power-of-two initial steps against an independent scalar backtracking loop; always-rejected callback budget/count; invalid slope; callback infinity; overflowing Armijo bound; minimum-subnormal contraction stagnation.

Composition limits: No internal allocation; validate scalar inputs before callback. Caller supplies deterministic ray function, true f0 and directional derivative. Callback cost, side effects, allocation and panics are outside the model; exactly iterations+1 evaluations on successful reports. Converged precedes BudgetExhausted, which precedes Stagnated. Budget counts contractions, so zero still evaluates the initial step. If the next step is zero or equal to the current step, terminate Stagnated without another callback.

## approximant

### Forward approximant and invariant

The test harness observes finite assertions on armijo_line_search. Analytic formulas and finite grid competitors are distinct forms of evidence; the grid alone proves no optimizer bound.

#### Domain and invariant

All scalars finite; slope<0, initial step>0, rho and c strictly between zero and one, budget<=100000. Compute and validate the bound before callback; overflow is NonFiniteArithmetic, callback NaN/infinity is NonFiniteEvaluation. Underflow and rounded bound are observable.

For f(alpha)=(1-alpha)^2, f0=1,d=-2,c=.25, solve/check the scalar inequality directly. The independent loop finds the first member of the halving sequence satisfying both Armijo and strict decrease.

#### Cost

O(k) scalar arithmetic and at most k+1 callback evaluations; O(1) internal storage. Harness repeats only the named bounded fixture set; no production throughput claim.

#### Executable approximation gap

No useful percentage-bounded cheaper surrogate is attached. A sampled decrease, projected step, bracket width or KKT/dual residual is not a universal relative objective/solution certificate. No 1/2/5/10% claim is inferred from finite success; zero-reference relative error is undefined without a separately specified metric. This gap is explicit rather than wrapping the exact implementation.

Reference: Armijo, Minimization of functions having Lipschitz continuous first partial derivatives, Pacific Journal of Mathematics 16 (1966).

## models

# Typed obernant family

These are scoped partial relations, not executable inverses or semantic proofs.

## behavior_complement

The observation suite excludes the named wrong coefficients, vertices, brackets and statuses on these fixtures: Eight power-of-two initial steps against an independent scalar backtracking loop; always-rejected callback budget/count; invalid slope; callback infinity; overflowing Armijo bound; minimum-subnormal contraction stagnation.. It says nothing about callbacks or matrices outside these finite cases; arbitrary unseen bugs remain compatible with all passes.

## reverse_reconstruction

A passing transcript reconstructs only a family of implementations agreeing on these assertions. The analytic oracle is For f(alpha)=(1-alpha)^2, f0=1,d=-2,c=.25, solve/check the scalar inequality directly. The independent loop finds the first member of the halving sequence satisfying both Armijo and strict decrease.. Many different update orders or untested behaviors have the same pass transcript, so source recovery is not unique.

## independent_oracle

For f(alpha)=(1-alpha)^2, f0=1,d=-2,c=.25, solve/check the scalar inequality directly. The independent loop finds the first member of the halving sequence satisfying both Armijo and strict decrease.

## mutation_sensitivity

Replace the acceptance inequality by value>=bound; backtracking_oracle must fail the expected step/status assertion after compiling.

## boundary_partition

A minimum-subnormal step with unchanged value is Stagnated, never Converged. Invalid slope suppresses callbacks, overflow precedes callback evaluation, finite always-failing callback consumes budget+1 calls.


## tests

### Executable checks

Run `cargo test -p mova-algorithms armijo_line_search::tests`.

Eight power-of-two initial steps against an independent scalar backtracking loop; always-rejected callback budget/count; invalid slope; callback infinity; overflowing Armijo bound; minimum-subnormal contraction stagnation.

Independent oracle: For f(alpha)=(1-alpha)^2, f0=1,d=-2,c=.25, solve/check the scalar inequality directly. The independent loop finds the first member of the halving sequence satisfying both Armijo and strict decrease.

Compiled negative control: Replace the acceptance inequality by value>=bound; backtracking_oracle must fail the expected step/status assertion after compiling.

No allocator failure injection, all-input proof or benchmark is claimed.

## prospective

### Prospective optimization model
Before source creation. Brent maintains unimodal bracket and three sampled points; accepts safeguarded parabolic proposals, otherwise golden contraction. O(k) callbacks, O(1) storage. Deterministic callback is assumed; strict unimodality cannot be checked. Armijo shrinks positive alpha by rho until f(alpha)<=f0+c*alpha*slope, slope<0. O(k) callbacks, O(1) space; supplied f0 and slope are caller premises.
Dense quadratics f=.5*x'Qx-b'x use symmetric diagonally dominant nonnegative Q, dimension<=64, finite magnitudes<=1e50. This is a sufficient restricted PSD domain, not arbitrary convex quadratics. PG clips x-g/L into finite bounds with L=max absolute row sum; mapping residual is max |x-clip(x-g/L)|. FW on unit simplex selects earliest minimum gradient vertex, gap=g'(x-s), step=min(1,gap/(d'Qd)); linear curvature uses step 1. O(kn²) time O(n) scratch. These real identities are not floating error guarantees.
Lasso minimizes .5||Ax-y||²+lambda||x||1 by cyclic exact coordinate minimization and residual maintenance, using existing soft threshold kernel. Up to 256 rows and 64 columns. O(kmn) time O(m+n) space. Stop on recomputed KKT max violation. Zero columns set zero. Deterministic ties and iteration orders. All validation precedes work; checked allocation and nonfinite arithmetic explicit; budget<=100000, tolerance positive finite. Each routine distinguishes convergence, budget exhaustion and representational stagnation. No percent objective/solution bound. Expected algorithmic advantage: interpolation avoids golden-only work on smooth functions; cached residual updates avoid recomputing predictions per coordinate. No measured speed claim. Executable approximate surrogate deferred: residual and finite samples lack a justified universal supported percentage metric.
References: Brent, Algorithms for Minimization without Derivatives (1973); Armijo (1966); Nocedal and Wright (2006); Friedman et al., Regularization Paths for Generalized Linear Models via Coordinate Descent (2010); Frank and Wolfe (1956).

## validation

### Validation

Five independently compiled production mutants are rejected by named runtime assertions; source bytes restored before the positive rerun. This source's control: Replace the acceptance inequality by value>=bound; backtracking_oracle must fail the expected step/status assertion after compiling.

Fresh syntax projections are generated twice and compared before installing syntax.json. These are partial structural observations, not macro-expanded AST or semantic proofs. No performance or universal <=10% result follows.

## History

### 2026-10-30 · validation

Heading `### Validation epoch 2026-10-30` stays in validation as `### Validation`.

External receipts: `$MOVA_DATA/runs/octopus-wave20-optimization-2026-10-30`. Two debug and two release library runs each pass 568 tests, including 21 new native tests, with identical sorted test-name/outcome transcripts. Strict all-target algorithms Clippy and focused rustfmt pass.

### 2026-11-01 · model

#### Test-to-implementation boundary, 2026-11-01

The status ordering above describes the implementation, not an ordering fully established by this test suite. The budget fixture has a nonzero next step; the subnormal stagnation fixture has remaining budget. Therefore the intersection of their passing observations leaves the simultaneous budget/stagnation case unconstrained. A separate actual-API probe observes exhaustion for budget zero and stagnation for budget one, and convergence first for an accepted value at budget zero. Callback behavior outside these samples and a formal floating-point semantics remain open.

### 2026-11-01 · document

#### precedence_observation (review 2026-11-01)

The existing transcript has no assertion where exhaustion and stagnation coincide. For an unchanged finite callback, minimum positive subnormal step, contraction 0.5 and budget zero, the actual API reports BudgetExhausted after one evaluation; with budget one it reports Stagnated after one evaluation. With the same step and budget zero, callback value 0 instead reports Converged. These witnesses separate the three branches; they do not show that the existing tests detect a swap of budget and stagnation precedence. The transcript's converse contains implementations with such a swap because its budget fixture contracts normally and its stagnation fixture has budget remaining. This is a partial observation fiber, not reconstruction of source or all callbacks. The behavior complement excludes contrary outcomes only on the asserted fixtures; this separate probe does not add assertions to the original suite.

### 2026-11-01 · tests

The existing fixtures do not isolate budget-versus-stagnation precedence when both apply. An external actual-API probe on 2026-11-01 exercises that boundary and acceptance precedence; it is supplementary evidence, not a native suite assertion.

### 2026-11-01 · validation

#### Bounded review 2026-11-01

The four focused native tests pass. An independently compiled actual-API probe checks the simultaneous budget/stagnation boundary and an acceptance control; two outputs are identical. Its hypothesis, source and outputs are retained in the external cycle 12 directory. This does not mutate the original test suite or establish complete mutation coverage. No source or syntax projection changed.
