# algorithm-armijo-line-search

## descriptant

### armijo_line_search implementation

Given f0 and negative ray derivative d supplied by the caller, form alpha_j=alpha0*rho^j with 0<rho<1. Return the first tested step satisfying f(alpha)<=f0+c*alpha*d and strict representable decrease f(alpha)<f0. The latter excludes equality caused by rounding. No gradient evaluation or verification of f0/d is attempted.

#### Prospective epoch

The prospective model was saved before source creation in the external wave20 optimization run and retained in model.md#prospective. Original standard-library Rust, no downloaded solution code.

#### Observations retained

Eight power-of-two initial steps against an independent scalar backtracking loop; always-rejected callback budget/count; invalid slope; callback infinity; overflowing Armijo bound; minimum-subnormal contraction stagnation.

#### Limits

Converged precedes BudgetExhausted, which precedes Stagnated. Budget counts contractions, so zero still evaluates the initial step. If the next step is zero or equal to the current step, terminate Stagnated without another callback. No timing or universal machine-accuracy claim.

Reference: Armijo, Minimization of functions having Lipschitz continuous first partial derivatives, Pacific Journal of Mathematics 16 (1966).

## model

### Mathematical combiner

Inputs are the authoritative mod.rs, source-bound partial syntax, forward relation, typed complement first and reverse reconstruction second, specialized relations and named finite evidence. Intersect the accepted-domain relation with each update relation, then partition termination by convergence, budget, stagnation or error. Given f0 and negative ray derivative d supplied by the caller, form alpha_j=alpha0*rho^j with 0<rho<1. Return the first tested step satisfying f(alpha)<=f0+c*alpha*d and strict representable decrease f(alpha)<f0. The latter excludes equality caused by rounding. No gradient evaluation or verification of f0/d is attempted.

Tests are sibling source nodes; manifest children are absent because these compact files have no independent nested fragment contracts. Syntax does not expand macros or prove semantics.

Retained observation: Eight power-of-two initial steps against an independent scalar backtracking loop; always-rejected callback budget/count; invalid slope; callback infinity; overflowing Armijo bound; minimum-subnormal contraction stagnation.

Composition limits: No internal allocation; validate scalar inputs before callback. Caller supplies deterministic ray function, true f0 and directional derivative. Callback cost, side effects, allocation and panics are outside the model; exactly iterations+1 evaluations on successful reports. Converged precedes BudgetExhausted, which precedes Stagnated. Budget counts contractions, so zero still evaluates the initial step. If the next step is zero or equal to the current step, terminate Stagnated without another callback.

## approximant

### Forward approximant and invariant

Given f0 and negative ray derivative d supplied by the caller, form alpha_j=alpha0*rho^j with 0<rho<1. Return the first tested step satisfying f(alpha)<=f0+c*alpha*d and strict representable decrease f(alpha)<f0. The latter excludes equality caused by rounding. No gradient evaluation or verification of f0/d is attempted.

#### Domain and invariant

All scalars finite; slope<0, initial step>0, rho and c strictly between zero and one, budget<=100000. Compute and validate the bound before callback; overflow is NonFiniteArithmetic, callback NaN/infinity is NonFiniteEvaluation. Underflow and rounded bound are observable.

Each failed trial contracts alpha by rho. Evaluate the current step before deciding whether the contraction budget is exhausted. Earliest sufficient-decrease trial wins, giving a deterministic finite prefix under a deterministic callback.

#### Cost

O(k) scalar arithmetic and at most k+1 callback evaluations; O(1) internal storage. No internal allocation; validate scalar inputs before callback. Caller supplies deterministic ray function, true f0 and directional derivative. Callback cost, side effects, allocation and panics are outside the model; exactly iterations+1 evaluations on successful reports.

#### Executable approximation gap

No useful percentage-bounded cheaper surrogate is attached. A sampled decrease, projected step, bracket width or KKT/dual residual is not a universal relative objective/solution certificate. No 1/2/5/10% claim is inferred from finite success; zero-reference relative error is undefined without a separately specified metric. This gap is explicit rather than wrapping the exact implementation.

Reference: Armijo, Minimization of functions having Lipschitz continuous first partial derivatives, Pacific Journal of Mathematics 16 (1966).

## models

# Typed obernant family

These are scoped partial relations, not executable inverses or semantic proofs.

## behavior_complement

A Converged transcript with f(alpha)>=f0 is excluded even when the rounded Armijo bound equals f0. The routine does not establish Wolfe curvature conditions, exact line minimization or that caller-provided d is the actual derivative.

## reverse_reconstruction

A successful alpha determines the contraction count when alpha0 and rho are fixed in exact arithmetic. Callback preimages include all ray functions satisfying failed inequalities at prior tested steps and the accepted inequality at alpha. Unobserved values and derivatives remain ambiguous; different alpha0/rho triples can yield the same final step.

## update_relation

Each failed trial contracts alpha by rho. Evaluate the current step before deciding whether the contraction budget is exhausted. Earliest sufficient-decrease trial wins, giving a deterministic finite prefix under a deterministic callback.

## termination_partition

Converged precedes BudgetExhausted, which precedes Stagnated. Budget counts contractions, so zero still evaluates the initial step. If the next step is zero or equal to the current step, terminate Stagnated without another callback.

## finite_arithmetic

All scalars finite; slope<0, initial step>0, rho and c strictly between zero and one, budget<=100000. Compute and validate the bound before callback; overflow is NonFiniteArithmetic, callback NaN/infinity is NonFiniteEvaluation. Underflow and rounded bound are observable.

## resource_observation

No internal allocation; validate scalar inputs before callback. Caller supplies deterministic ray function, true f0 and directional derivative. Callback cost, side effects, allocation and panics are outside the model; exactly iterations+1 evaluations on successful reports.


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

2026-11-01 review: the powers above describe ideal arithmetic. The actual
sequence repeatedly rounds `step * contraction`; the bound also rounds its
multiplications in source order. The rounded trial and report fiber in
`model.md#models` records status precedence and an API counterexample to recovering
unobserved callback values. The finite probe does not prove the complete model.

### 2026-11-01 · document

#### Rounded trial and report fiber — review 2026-11-01

For a valid input and finite evaluations, let `a_0` be the initial step,
`a_{j+1}=fl(a_j*rho)`, and `b_j=fl(f0+fl(fl(c*a_j)*d))` using the
source's sequential f64 operations. The acceptance predicate is
`v_j <= b_j && v_j < f0`. Acceptance wins over budget, and budget wins over
stagnation. The exact-power expression in the forward approximant is an
idealization, not an equality for this rounded recurrence. A nonfinite bound
returns an arithmetic error before invoking the callback; a nonfinite callback
result gives a distinct error. Callback panics and side effects are outside this
relation.

At the minimum positive subnormal with `rho=c=0.5`, `f0=1`, `d=-1`, and
constant value 1, budgets 0 and 1 respectively report BudgetExhausted and
Stagnated after the same single evaluation. Replacing the sampled value by 0
makes Converged win even at budget 0. These are finite source-API observations,
not a floating-point proof. For fixed scalar inputs and a successful report,
compatible deterministic callbacks must give its reported value at the final
sample and fail acceptance at earlier samples; their values away from the
sampled points are unconstrained. Thus the converse is a fiber, not a unique
callback reconstruction. This is distinct from the behavior complement that
excludes nondecreasing converged values. The error-return fiber and full
floating-point characterization remain open.

### 2026-11-01 · validation

#### Bounded review 2026-11-01

An external Rust probe includes the actual module and checks the minimum
subnormal status partition, acceptance precedence, equal reports from distinct
callbacks and arithmetic failure before callback. Two runs agree. The probe
is finite; full error and callback fibers remain open. No production behavior
or performance claim changes.
