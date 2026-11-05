# mlab-numerics-poly-convolution

## descriptant

### mlab-numerics-poly-convolution

conv flattens nonempty inputs, accumulates complex products into length P+Q-1 and uses first-input orientation. deconv requires nonempty divisor with nonzero first re; long-divides real coefficients, explicitly zeroing eliminated remainder positions.

## approximant

### Forward relation

conv flattens nonempty inputs, accumulates complex products into length P+Q-1 and uses first-input orientation. deconv requires nonempty divisor with nonzero first re; long-divides real coefficients, explicitly zeroing eliminated remainder positions.

Executable Rust surrogate gap: no calibrated replacement is attached. Exact test calls are evidence for the reference, not a fast approximant. Numerical approximants would need a named domain, zero convention and metric at supported caps 1/2/5/10%; categorical errors/effects have no percent surrogate here.

## invariant

### Invariants and cost

Output orientation follows first input for conv, dividend for deconv. Remainder remains dividend length in long-division branch.

#### Cost

O(PQ) conv; O((Y-D+1)D) division; linear output and remainder storage.

Allocation failure, unchecked usize dimension arithmetic and IEEE nonfinite propagation are outside finite fixture guarantees. No measured throughput or global accuracy claim.

## models

# Typed source models: mlab-numerics-poly-convolution

## behavior_complement

conv excludes empty success but does not validate vector-only shapes despite error text. deconv excludes imaginary participation except short-dividend clone; neither guarantees no overflow.

## reverse_reconstruction

For real divisor d with leading nonzero, (q,r) reconstructs y=conv(d,q)+r with leading padding to dividend length; floating rounding prevents exact general identity. Different divisor/quotient factorizations share convolution.

## product_order

conv [1,-1],[1,-2] gives [1,-3,2].

## division_fixture

[1,-3,2]/[1,-1] yields q[1,-2] and r[0,0,0].

## short_dividend

Y shorter than D returns scalar0 quotient and original cloned Y if remainder requested.

## zero_divisor

D empty or D.re[0]==0 fails even if imaginary first coefficient is nonzero.


## tests

### Evidence and limits

Selected wave30_tests exercise this source; fragment-specific limits are stated in its operators.

Native command specifications, numeric exits, profile-paired logs, compiled controls and exact-restoration receipts are retained externally in the wave30 numerics run. Tests exercise actual source, not a copied model. The selected finite fixtures do not prove full convergence, backend accuracy on all matrices, resource exhaustion or side-effectful callback behavior.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source reading preceded inline tests. This node records implementation behavior, not a complete MATLAB compatibility contract. Models retain limits discovered in source reading, including real-only projections and warning-only result gates. Synthetic CPU fixtures are independent of frozen suites. No external effects, teacher, corpora or benchmark main are executed.
