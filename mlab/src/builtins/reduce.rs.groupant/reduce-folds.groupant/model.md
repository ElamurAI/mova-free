# mlab-numerics-reduce-folds

## descriptant

### mlab-numerics-reduce-folds

slices(dim1) lists column indices j*rows+i, dim2 row indices, else singleton. reduce_c folds (init,0) with complex operation and returns shaped result; absent dim on0x0 yields scalar init. cumulative uses same slices but real-only running accumulation.

## approximant

### Forward relation

slices(dim1) lists column indices j*rows+i, dim2 row indices, else singleton. reduce_c folds (init,0) with complex operation and returns shaped result; absent dim on0x0 yields scalar init. cumulative uses same slices but real-only running accumulation.

Executable Rust surrogate gap: no calibrated replacement is attached. Exact test calls are evidence for the reference, not a fast approximant. Numerical approximants would need a named domain, zero convention and metric at supported caps 1/2/5/10%; categorical errors/effects have no percent surrogate here.

## invariant

### Invariants and cost

Output shape dim1=1xcols,dim2=rowsx1,else original. Complex storage narrows via Mat::complex only for reduce_c.

#### Cost

O(N+S) fold, slice-index/vector-header allocation and output storage; cumulative O(N+S) plus cloned real cells. For0xM at dim1, N0 but S=M identity outputs.

Allocation failure, unchecked usize dimension arithmetic and IEEE nonfinite propagation are outside finite fixture guarantees. No measured throughput or global accuracy claim.

## models

# Typed source models: mlab-numerics-reduce-folds

## behavior_complement

No default empty fold calls f. Singleton higher dimensions exclude cross-cell accumulation. cumulative excludes imaginary dependence.

## reverse_reconstruction

Given sum result, compatible inputs have the requested shape and per-slice sum; product fibers depend on zeros and scaling pairs. Cumulative real sum can be differenced along same dimension absent rounding, but cannot recover imaginary input.

## column_fixture

2x2 re[1,3,2,4] summed dim1→1x2[4,6].

## identity

0x0 with no dim and init1 returns scalar1 before slicing.

## explicit_empty

Explicit dim changes empty handling, producing shape derived from slices rather than scalar shortcut.

## real_scan

cumulative output is new real Mat even for input with im; no complex prefix-product claim.


## tests

### Evidence and limits

Selected wave30_tests exercise this source; fragment-specific limits are stated in its operators.

Native command specifications, numeric exits, profile-paired logs, compiled controls and exact-restoration receipts are retained externally in the wave30 numerics run. Tests exercise actual source, not a copied model. The selected finite fixtures do not prove full convergence, backend accuracy on all matrices, resource exhaustion or side-effectful callback behavior.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source reading preceded inline tests. This node records implementation behavior, not a complete MATLAB compatibility contract. Models retain limits discovered in source reading, including real-only projections and warning-only result gates. Synthetic CPU fixtures are independent of frozen suites. No external effects, teacher, corpora or benchmark main are executed.
