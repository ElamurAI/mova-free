# mlab-numerics-reduce-order

## descriptant

### mlab-numerics-reduce-order

better ignores NaN best when candidate finite and uses strict >/<. minmax reduction scans slices in order; complex keys are modulus but returned cell keeps components. sort computes stable local index permutation by real/modulus keys, with NaNs last ascending and first descending.

## approximant

### Forward relation

better ignores NaN best when candidate finite and uses strict >/<. minmax reduction scans slices in order; complex keys are modulus but returned cell keeps components. sort computes stable local index permutation by real/modulus keys, with NaNs last ascending and first descending.

Executable Rust surrogate gap: no calibrated replacement is attached. Exact test calls are evidence for the reference, not a fast approximant. Numerical approximants would need a named domain, zero convention and metric at supported caps 1/2/5/10%; categorical errors/effects have no percent surrogate here.

## invariant

### Invariants and cost

Coordinates are1-based local slice positions; sort preserves Class and complex storage, minmax casts Char/Logical result class to Double.

#### Cost

O(N+S) extrema; O(N log(max(L,2))+S) sort and O(N+S) index/scratch memory, including headers for empty slices.

Allocation failure, unchecked usize dimension arithmetic and IEEE nonfinite propagation are outside finite fixture guarantees. No measured throughput or global accuracy claim.

## models

# Typed source models: mlab-numerics-reduce-order

## behavior_complement

Ties exclude winner replacement; sort excludes independent real/imag sorting. Empty0x0 minmax returns two empty values regardless nargout, while other empty slice shapes are unchecked.

## reverse_reconstruction

Extremum and position identify selected cell and constrain other keys; earlier equal keys are incompatible with later winner. Sorted values plus 1-based per-slice permutation recover original cells when the permutation is valid.

## strict_tie

[NaN,4,4] max index2, never3.

## nan_sort

Two NaNs compare equal in stable sort; descending moves NaNs before finite keys.

## binary_dispatch

Exactly2 arguments can take elementwise max/min even for empty second Mat; three-argument empty placeholder uses dim reduction.

## complex_key

Equal magnitudes retain first minmax cell without phase tiebreak; returned value may have different phase than another compatible winner.


## tests

### Evidence and limits

Selected wave30_tests exercise this source; fragment-specific limits are stated in its operators.

Native command specifications, numeric exits, profile-paired logs, compiled controls and exact-restoration receipts are retained externally in the wave30 numerics run. Tests exercise actual source, not a copied model. The selected finite fixtures do not prove full convergence, backend accuracy on all matrices, resource exhaustion or side-effectful callback behavior.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source reading preceded inline tests. This node records implementation behavior, not a complete MATLAB compatibility contract. Models retain limits discovered in source reading, including real-only projections and warning-only result gates. Synthetic CPU fixtures are independent of frozen suites. No external effects, teacher, corpora or benchmark main are executed.
