# mlab-numerics-numeric-quadrature

## descriptant

### mlab-numerics-numeric-quadrature

map_point supplies x(t) and positive quadrature weight; gk15 samples symmetric15 nodes, forms K15 and embedded G7, returns h*K and abs(h*(K-G)). integral orders endpoints, sets sign, starts10 intervals and splits maximum estimated error until tolerance or650.

## approximant

### Forward relation

map_point supplies x(t) and positive quadrature weight; gk15 samples symmetric15 nodes, forms K15 and embedded G7, returns h*K and abs(h*(K-G)). integral orders endpoints, sets sign, starts10 intervals and splits maximum estimated error until tolerance or650.

Executable Rust surrogate gap: no calibrated replacement is attached. Exact test calls are evidence for the reference, not a fast approximant. Numerical approximants would need a named domain, zero convention and metric at supported caps 1/2/5/10%; categorical errors/effects have no percent surrogate here.

## invariant

### Invariants and cost

Finite map endpoints x(-1)=a,x(1)=b, zero derivative endpoints. Error estimate is not an analytical bound; finite nonpathological parameters assumed.

#### Cost

O(I²+15*I*callback cost) worst interval scans for I<=650, O(I) interval storage plus15 nodes.

Allocation failure, unchecked usize dimension arithmetic and IEEE nonfinite propagation are outside finite fixture guarantees. No measured throughput or global accuracy claim.

## models

# Typed source models: mlab-numerics-numeric-quadrature

## behavior_complement

No endpoint callback in the embedded rule; equal integral bounds exclude all samples after options. Exhaustion does not exclude returned value: it emits warning. Nonfinite q returns immediately.

## reverse_reconstruction

A finite quadrature result constrains weighted sample sums; integrands agreeing at sampled nodes or differing in nullspace of those weights can share result. Mapping antecedent t is unique only on interior monotone finite/right domains; Left reverses orientation with positive weight.

## finite_map

a2,b6,t0→x4,w3; Jacobian is cubic map derivative.

## infinite_map

Right2,t.5→3,4; Left2,t.5→1,4; Both0→0,1.

## retry_effect

Vectorized nonnumeric/wrong-shape/complex result or error falls back to scalar calls after its effects.

## option_order

Unknown paired option errors before lo==hi shortcut; unpaired trailing option is ignored by loop condition.


## tests

### Evidence and limits

Selected wave30_tests exercise this source; fragment-specific limits are stated in its operators.

Native command specifications, numeric exits, profile-paired logs, compiled controls and exact-restoration receipts are retained externally in the wave30 numerics run. Tests exercise actual source, not a copied model. The selected finite fixtures do not prove full convergence, backend accuracy on all matrices, resource exhaustion or side-effectful callback behavior.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source reading preceded inline tests. This node records implementation behavior, not a complete MATLAB compatibility contract. Models retain limits discovered in source reading, including real-only projections and warning-only result gates. Synthetic CPU fixtures are independent of frozen suites. No external effects, teacher, corpora or benchmark main are executed.
