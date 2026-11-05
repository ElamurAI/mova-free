# mlab-numerics-elem-gamma

## descriptant

### mlab-numerics-elem-gamma

gamma(x) returns Inf when x==trunc(x) and x<=0; otherwise returns libm::tgamma(x).

## approximant

### Forward relation

gamma(x) returns Inf when x==trunc(x) and x<=0; otherwise returns libm::tgamma(x).

Executable Rust surrogate gap: no calibrated replacement is attached. Exact test calls are evidence for the reference, not a fast approximant. Numerical approximants would need a named domain, zero convention and metric at supported caps 1/2/5/10%; categorical errors/effects have no percent surrogate here.

## invariant

### Invariants and cost

Exact pole policy precedes library evaluation; no rejection Result.

#### Cost

Constant wrapper work plus libm implementation.

Allocation failure, unchecked usize dimension arithmetic and IEEE nonfinite propagation are outside finite fixture guarantees. No measured throughput or global accuracy claim.

## models

# Typed source models: mlab-numerics-elem-gamma

## behavior_complement

Nonpositive integer poles exclude a finite result. No reflected approximation is implemented here.

## reverse_reconstruction

All nonpositive integers belong to the positive-Inf output fiber, along with possible overflow inputs; a gamma value generally has multiple real antecedents.

## pole_boundary

gamma(-2)=Inf and gamma(0)=Inf.

## positive_factorial

For fixture x=5, gamma=4!=24; this does not certify every integer.

## nan_branch

NaN fails equality and reaches libm; no custom NaN error.

## library_dependency

Accuracy outside poles is delegated to libm; no Boost held-out suite executed.


## tests

### Evidence and limits

Selected wave30_tests exercise this source; fragment-specific limits are stated in its operators.

Native command specifications, numeric exits, profile-paired logs, compiled controls and exact-restoration receipts are retained externally in the wave30 numerics run. Tests exercise actual source, not a copied model. The selected finite fixtures do not prove full convergence, backend accuracy on all matrices, resource exhaustion or side-effectful callback behavior.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source reading preceded inline tests. This node records implementation behavior, not a complete MATLAB compatibility contract. Models retain limits discovered in source reading, including real-only projections and warning-only result gates. Synthetic CPU fixtures are independent of frozen suites. No external effects, teacher, corpora or benchmark main are executed.
