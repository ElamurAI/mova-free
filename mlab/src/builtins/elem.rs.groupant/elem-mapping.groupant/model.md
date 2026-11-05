# mlab-numerics-elem-mapping

## descriptant

### mlab-numerics-elem-mapping

map(m,f)=(rows,cols,f(re)); cmap applies f to (re,im_or_zeros) and narrows; map_parts separately transforms stored parts and narrows. csqrt(x,0) returns real sqrt if x>=0 else positive imaginary root; nonzero y uses hypot half-angle formula.

## approximant

### Forward relation

map(m,f)=(rows,cols,f(re)); cmap applies f to (re,im_or_zeros) and narrows; map_parts separately transforms stored parts and narrows. csqrt(x,0) returns real sqrt if x>=0 else positive imaginary root; nonzero y uses hypot half-angle formula.

Executable Rust surrogate gap: no calibrated replacement is attached. Exact test calls are evidence for the reference, not a fast approximant. Numerical approximants would need a named domain, zero convention and metric at supported caps 1/2/5/10%; categorical errors/effects have no percent surrogate here.

## invariant

### Invariants and cost

Same rows/cols and one result per valid real cell; malformed public vector lengths outside contract.

#### Cost

O(N) maps and temporary imaginary vector; csqrt O(1).

Allocation failure, unchecked usize dimension arithmetic and IEEE nonfinite propagation are outside finite fixture guarantees. No measured throughput or global accuracy claim.

## models

# Typed source models: mlab-numerics-elem-mapping

## behavior_complement

map excludes imaginary dependence; cmap excludes index reordering. For finite x and y<0, csqrt excludes positive imaginary branch. A zero-imaginary result may narrow, so complex representation identity is not guaranteed.

## reverse_reconstruction

map preimage includes arbitrary imaginary arrays when re maps to output. cmap preimages are products of scalar f fibers subject to shape; principal csqrt reconstructs z by squaring only on admitted branch.

## branch_selection

(-9,0)→(0,3); (3,-4)→(2,-1); nonzero imaginary sign follows y.

## narrowing

map_parts invokes narrow after transforming im; a rounded-to-zero im array becomes real.

## shape_transport

Column-major index k is unchanged for every mapped cell.

## nonfinite_gap

hypot avoids some overflow but (r+x)/2 can still overflow; NaN and infinities are not certified.


## tests

### Evidence and limits

Selected wave30_tests exercise this source; fragment-specific limits are stated in its operators.

Native command specifications, numeric exits, profile-paired logs, compiled controls and exact-restoration receipts are retained externally in the wave30 numerics run. Tests exercise actual source, not a copied model. The selected finite fixtures do not prove full convergence, backend accuracy on all matrices, resource exhaustion or side-effectful callback behavior.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source reading preceded inline tests. This node records implementation behavior, not a complete MATLAB compatibility contract. Models retain limits discovered in source reading, including real-only projections and warning-only result gates. Synthetic CPU fixtures are independent of frozen suites. No external effects, teacher, corpora or benchmark main are executed.
