# mlab-numerics-numeric

## descriptant

### mlab-numerics-numeric

Registry maps integral/quad/quadgk to adaptive transformed GK7-15, fzero to bracket search and Brent, fminsearch to Nelder-Mead, ode45 to adaptive Dormand-Prince and trapz to sampled trapezoids. Callback results mutate Interp as normal; vectorized quadrature trial may be followed by repeated scalar calls. Tolerances and step budgets are algorithm stopping rules, not verified global bounds.

## approximant

### Forward relation

Registry maps integral/quad/quadgk to adaptive transformed GK7-15, fzero to bracket search and Brent, fminsearch to Nelder-Mead, ode45 to adaptive Dormand-Prince and trapz to sampled trapezoids. Callback results mutate Interp as normal; vectorized quadrature trial may be followed by repeated scalar calls. Tolerances and step budgets are algorithm stopping rules, not verified global bounds.

Executable Rust surrogate gap: no calibrated replacement is attached. Exact test calls are evidence for the reference, not a fast approximant. Numerical approximants would need a named domain, zero convention and metric at supported caps 1/2/5/10%; categorical errors/effects have no percent surrogate here.

## invariant

### Invariants and cost

Callbacks are not assumed pure by code. Matrix storage is column-major; scalar callback accepts first cell of any nonempty Mat, RHS requires exact real vector length. Bounds/options are not uniformly finite-validated.

#### Cost

GK starts10 intervals then caps650, 15 nodes each, each refinement two rules and O(intervals) selection/sum; callback cost dominates. Brent1000 iterations after up to200 bracket expansions. Nelder-Mead O(n^2) storage and200*n iteration/evaluation thresholds (batch may overshoot). ODE at most1e6 attempted steps, O(neq*saved outputs) storage.

Allocation failure, unchecked usize dimension arithmetic and IEEE nonfinite propagation are outside finite fixture guarantees. No measured throughput or global accuracy claim.

## composition

### Composition

Numeric functions consume Interp call1 and Mat values. eval_nodes adapter retries vectorized callback scalarwise without rollback; this is a real interaction with interpreter state. trapz has no callbacks. The group cannot treat every numerical callback as a pure mathematical function.

Fragments refer to disjoint useful source scopes in the single parent lexical projection. Registration and inline tests remain parent-modeled. Source callbacks and external backend implementations are dependencies, not duplicated ownership. Local tolerances cannot be composed into a subsystem percent guarantee.

## models

# Typed source models: mlab-numerics-numeric

## behavior_complement

An empty fminsearch initial state excludes optimization. ode45 nargout<2 excludes even argument parsing; unsupported options exclude RHS evaluation. Integral equal endpoints exclude callback evaluation only after option validation. No contract excludes all warnings or nonfinite results.

## reverse_reconstruction

Integral value determines a fiber of integrands differing by zero-integral components. fzero outputs constrain the computed bracket/Brent stopping relation, not a zero fiber: bracket half-width tolerance or1000-iteration exhaustion can return nonzero residual and still success flag1. Discontinuous bracketed functions with no true zero are compatible; only observed fb==0 branches establish a sampled zero. A sampled ODE trajectory is compatible with many RHS fields off visited points. Trapz sums constrain weighted adjacent ordinates but do not reconstruct unique samples.

## callback_effects

eval_nodes first calls vectorized; wrong shape/complex/error triggers scalar retry, so effects from failed vector call are not rolled back.

## quadrature_transform

Finite x=(b-a)t(3-t²)/4+(a+b)/2; Jacobian3(b-a)(1-t²)/4. Infinite-domain rational transforms use positive integration weights.

## stopping_observation

GK compares summed embedded estimate to max(abstol,reltol*abs(q));650 limit warns and returns estimate, not an error certificate. Brent may return on bracket tolerance or1000-iteration exhaustion with nonzero residual; fzero still emits requested success flag1.

## ode_precedence

nargout then unsupported option count then f/tspan/y0 parsing then tspan length then RHS. Initial RHS occurs even at equal times.


## tests

### Executed evidence

builtins::numeric::wave30_tests::maps_and_trapezoid_contract: four transforms/Jacobians, scalar-spacing trapezoid, mismatched X and early ode45 nargout error. Adaptive convergence and callback side effects remain unexecuted.

#### Native selection

    cargo test --offline --locked -p mlab --lib builtins::numeric::wave30_tests::maps_and_trapezoid_contract -- --exact

Each selected test passed twice in debug and twice in release with numeric exit 0. Eight aggregate named outcomes were extracted from actual logs, sorted, and identical across all four runs. Post-control exact-restoration runs passed both profiles with separately retained numeric exits.

#### Compiled negative control

In this source, the expression

    acc += dx *

was temporarily replaced by

    acc += 2.0 * dx *

Cargo compiled successfully and the selected test reached an assertion failure with exit 101. The complete original source bytes were restored and compared before positive reruns. This control validates the selected observation, not all branches.

#### Scope and limits

Parent and fragment models distinguish executed fixtures from source-only branches. No benchmark main, held-out datasets, network, teacher or GPU run. No numerical approximation surrogate, exhaustive convergence proof, allocation-failure injection or speed claim. Strict Clippy was not rerun for this test/model-only change; added test blocks were individually rustfmt-formatted without changing production formatting. External receipts retain native commands, test maps, logs, exits, mutations and structural checks.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source reading preceded inline tests. This node records implementation behavior, not a complete MATLAB compatibility contract. Models retain limits discovered in source reading, including real-only projections and warning-only result gates. Synthetic CPU fixtures are independent of frozen suites. No external effects, teacher, corpora or benchmark main are executed.

### 2026-10-31 · tests

Heading `### Executed evidence — 2026-10-31` stays in tests as `### Executed evidence`.
