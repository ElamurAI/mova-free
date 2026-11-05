# algorithm-frank-wolfe

## descriptant

### frank_wolfe implementation

Minimize .5*x'Q*x-b'x on the unit simplex, using the shared conservative symmetric diagonally dominant PSD domain. Choose earliest argmin_i gradient_i, vertex s=e_i and direction d=s-x. Compute gap=-g'd and curvature=d'Qd. If curvature>0 use alpha=min(1,gap/curvature), otherwise alpha=1. Update by the convex combination (1-alpha)*x+alpha*s.

#### Prospective epoch

The prospective model was saved before source creation in the external wave20 optimization run and retained in model.md#prospective. Original standard-library Rust, no downloaded solution code.

#### Observations retained

Nine two-coordinate analytic optimizers; zero-curvature linear problem with tied best vertices; dense quadratic against 101 simplex-grid objectives; budget zero and malformed simplex rejection.

#### Limits

Computed max(gap,0)<=tolerance precedes budget exhaustion. Negative roundoff gap is clamped, not treated as a rigorous certificate. If no coordinate changes at an attempted step return Stagnated with iterations incremented; value/gap still describe the unchanged point. No timing or universal machine-accuracy claim.

Reference: Frank and Wolfe, An algorithm for quadratic programming, Naval Research Logistics Quarterly 3 (1956), 95-110.

## model

### Mathematical combiner

Inputs are the authoritative mod.rs, source-bound partial syntax, forward relation, typed complement first and reverse reconstruction second, specialized relations and named finite evidence. Intersect the accepted-domain relation with each update relation, then partition termination by convergence, budget, stagnation or error. Minimize .5*x'Q*x-b'x on the unit simplex, using the shared conservative symmetric diagonally dominant PSD domain. Choose earliest argmin_i gradient_i, vertex s=e_i and direction d=s-x. Compute gap=-g'd and curvature=d'Qd. If curvature>0 use alpha=min(1,gap/curvature), otherwise alpha=1. Update by the convex combination (1-alpha)*x+alpha*s.

Tests are sibling source nodes; manifest children are absent because these compact files have no independent nested fragment contracts. Syntax does not expand macros or prove semantics.

Retained observation: Nine two-coordinate analytic optimizers; zero-curvature linear problem with tied best vertices; dense quadratic against 101 simplex-grid objectives; budget zero and malformed simplex rejection.

Composition limits: Three checked length-n vectors, shared allocation failures retained. No callback. Fixed matrix/coordinate traversal and earliest gradient ties. No simplex_projection allocation: the linear simplex oracle and convex combination are the method's substantive updates. Computed max(gap,0)<=tolerance precedes budget exhaustion. Negative roundoff gap is clamped, not treated as a rigorous certificate. If no coordinate changes at an attempted step return Stagnated with iterations incremented; value/gap still describe the unchanged point.

## approximant

### Forward approximant and invariant

Minimize .5*x'Q*x-b'x on the unit simplex, using the shared conservative symmetric diagonally dominant PSD domain. Choose earliest argmin_i gradient_i, vertex s=e_i and direction d=s-x. Compute gap=-g'd and curvature=d'Qd. If curvature>0 use alpha=min(1,gap/curvature), otherwise alpha=1. Update by the convex combination (1-alpha)*x+alpha*s.

#### Domain and invariant

Shared validation checks n<=64, matrix shape, finite bounds, symmetry and conservative dominance before simplex feasibility. Initial coordinates nonnegative, mass within 64*n*EPSILON of one. Reject nonfinite value/gradient/gap/curvature/update, negative computed curvature, or excessive updated mass residual. Near-feasible initial input remains near-feasible rather than exactly normalized.

The linear minimization oracle compares gradient entries with strict less-than, preserving earliest ties. The real quadratic along d is f(x)+alpha*g'd+.5*alpha²*d'Qd; the chosen clipped minimizer is exact for that real model. Convex combinations preserve nonnegativity; machine simplex mass is checked after updates.

#### Cost

O(n²) validation and O(k*n²) arithmetic, O(n) scratch (x,gradient,direction), n<=64. Three checked length-n vectors, shared allocation failures retained. No callback. Fixed matrix/coordinate traversal and earliest gradient ties. No simplex_projection allocation: the linear simplex oracle and convex combination are the method's substantive updates.

#### Executable approximation gap

No useful percentage-bounded cheaper surrogate is attached. A sampled decrease, projected step, bracket width or KKT/dual residual is not a universal relative objective/solution certificate. No 1/2/5/10% claim is inferred from finite success; zero-reference relative error is undefined without a separately specified metric. This gap is explicit rather than wrapping the exact implementation.

Reference: Frank and Wolfe, An algorithm for quadratic programming, Naval Research Logistics Quarterly 3 (1956), 95-110.

## models

# Typed obernant family

These are scoped partial relations, not executable inverses or semantic proofs.

## behavior_complement

A successful vector with a negative coordinate or a checked simplex-mass residual above 64*n*EPSILON is outside the update relation. No projection or general constrained solve is performed. An observed small floating dual gap is not a rigorous objective-error or relative-solution certificate.

## reverse_reconstruction

At a nondegenerate step with known previous x and selected vertex s, changed coordinates can recover a compatible alpha from x_next=(1-alpha)x+alpha*s. At alpha=1 the prior x is lost; equal minimum gradient coordinates have ambiguous mathematical vertices, resolved by earliest index in code. Many PSD objectives share the same simplex optimizer and terminal report.

## update_relation

The linear minimization oracle compares gradient entries with strict less-than, preserving earliest ties. The real quadratic along d is f(x)+alpha*g'd+.5*alpha²*d'Qd; the chosen clipped minimizer is exact for that real model. Convex combinations preserve nonnegativity; machine simplex mass is checked after updates.

## termination_partition

Computed max(gap,0)<=tolerance precedes budget exhaustion. Negative roundoff gap is clamped, not treated as a rigorous certificate. If no coordinate changes at an attempted step return Stagnated with iterations incremented; value/gap still describe the unchanged point.

## finite_arithmetic

Shared validation checks n<=64, matrix shape, finite bounds, symmetry and conservative dominance before simplex feasibility. Initial coordinates nonnegative, mass within 64*n*EPSILON of one. Reject nonfinite value/gradient/gap/curvature/update, negative computed curvature, or excessive updated mass residual. Near-feasible initial input remains near-feasible rather than exactly normalized.

## resource_observation

Three checked length-n vectors, shared allocation failures retained. No callback. Fixed matrix/coordinate traversal and earliest gradient ties. No simplex_projection allocation: the linear simplex oracle and convex combination are the method's substantive updates.


## tests

### Executable checks

Run `cargo test -p mova-algorithms frank_wolfe::tests`.

Nine two-coordinate analytic optimizers; zero-curvature linear problem with tied best vertices; dense quadratic against 101 simplex-grid objectives; budget zero and malformed simplex rejection.

Independent oracle: For Q=2I,b=[b0,0], write x=[t,1-t]; independent scalar minimizer t=clip((2+b0)/4,0,1). A linear objective selects earliest best vertex. Dense grid comparisons are finite lower-quality competitors, not a global accuracy proof.

Compiled negative control: Reverse gradient argmin to argmax; linear_vertex_and_tie must fail the compiled vertex assertion.

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

Five independently compiled production mutants are rejected by named runtime assertions; source bytes restored before the positive rerun. This source's control: Reverse gradient argmin to argmax; linear_vertex_and_tie must fail the compiled vertex assertion.

Fresh syntax projections are generated twice and compared before installing syntax.json. These are partial structural observations, not macro-expanded AST or semantic proofs. No performance or universal <=10% result follows.

## History

### 2026-10-30 · format-epoch

Heading `### Integration formatting epoch — 2026-10-30` stays in format-epoch as `### Integration formatting`.

Only the adjacent use and pub use import lines were reordered by Rust 2024 formatting. A sorted-line comparison confirms all lines are retained. Worker validation remains bound to historical source SHA-256 3c9d0c14fb9643bf2457dba6d20a6893a7ae20fc4ba70703827548882cd094bb; current source SHA-256 0ff02f917b940441ab5b1b9eced3917ffa33e9d94dbd536e1e57c5d065440719. Paired syntax was regenerated for the current source. The integrated wave20 native suite supplies final-source evidence; mathematical contracts and algorithm arithmetic are unchanged.

### 2026-10-30 · validation

Heading `### Validation epoch 2026-10-30` stays in validation as `### Validation`.

External receipts: `$MOVA_DATA/runs/octopus-wave20-optimization-2026-10-30`. Two debug and two release library runs each pass 568 tests, including 21 new native tests, with identical sorted test-name/outcome transcripts. Strict all-target algorithms Clippy and focused rustfmt pass.
