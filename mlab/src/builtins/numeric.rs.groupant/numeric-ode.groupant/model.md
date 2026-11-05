# mlab-numerics-numeric-ode

## descriptant

### mlab-numerics-numeric-ode

ode45 validates output arity/options, gets real tspan/y0 storage, evaluates initial RHS, then adapts Dormand-Prince step using scaled embedded error. Accepted steps append all times when two tspan points, otherwise only requested output targets; RHS must return real vector with neq cells.

## approximant

### Forward relation

ode45 validates output arity/options, gets real tspan/y0 storage, evaluates initial RHS, then adapts Dormand-Prince step using scaled embedded error. Accepted steps append all times when two tspan points, otherwise only requested output targets; RHS must return real vector with neq cells.

Executable Rust surrogate gap: no calibrated replacement is attached. Exact test calls are evidence for the reference, not a fast approximant. Numerical approximants would need a named domain, zero convention and metric at supported caps 1/2/5/10%; categorical errors/effects have no percent surrogate here.

## invariant

### Invariants and cost

t is directed toward final endpoint on admitted monotone tspan; stored state matrix uses rows=time, columns=equations. No monotonicity validation for arbitrary tspan.

#### Cost

At most1e6 attempted steps; each executes stages plus extra final RHS, O(neq) stage work and O(neq*outputs) storage; callback work external.

Allocation failure, unchecked usize dimension arithmetic and IEEE nonfinite propagation are outside finite fixture guarantees. No measured throughput or global accuracy claim.

## models

# Typed source models: mlab-numerics-numeric-ode

## behavior_complement

nargout1 excludes successful struct output. RHS length mismatch excludes stepping. Rejected steps exclude trajectory append but not callback effects. Nonfinite error or too-small step errors; no dense-output interpolation.

## reverse_reconstruction

A returned table fixes sampled states at times; compatible RHS values must produce accepted stage updates within code's local estimates, but off-stage fields remain unconstrained. Equal-time output can come from any RHS passing initial shape validation.

## arity_failure

Empty args with nargout1 fails struct-output policy before missing-function argument.

## rhs_shape

eval_rhs accepts any real Mat with neq cells, then interprets vector order; complex storage rejects.

## step_acceptance

err<=1 appends accepted y; next factor is5 if zero else clamp(.9*err^-0.2,.2,5).

## zero_interval

Initial RHS still evaluates when tf==t0; while loop skips and returns initial row.


## tests

### Evidence and limits

Selected wave30_tests exercise this source; fragment-specific limits are stated in its operators.

Native command specifications, numeric exits, profile-paired logs, compiled controls and exact-restoration receipts are retained externally in the wave30 numerics run. Tests exercise actual source, not a copied model. The selected finite fixtures do not prove full convergence, backend accuracy on all matrices, resource exhaustion or side-effectful callback behavior.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source reading preceded inline tests. This node records implementation behavior, not a complete MATLAB compatibility contract. Models retain limits discovered in source reading, including real-only projections and warning-only result gates. Synthetic CPU fixtures are independent of frozen suites. No external effects, teacher, corpora or benchmark main are executed.
