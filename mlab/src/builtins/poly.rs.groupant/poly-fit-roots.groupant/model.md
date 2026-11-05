# mlab-numerics-poly-fit-roots

## descriptant

### mlab-numerics-poly-fit-roots

polyfit parses x,y,n then rejects extra outputs, unequal lengths, invalid degree; constructs Vandermonde and calls SVD. roots rejects complex/nonfinite, removes leading zeros, records trailing zeros, constructs companion first row -c[j+1]/c0 and subdiagonal1 then calls eig.

## approximant

### Forward relation

polyfit parses x,y,n then rejects extra outputs, unequal lengths, invalid degree; constructs Vandermonde and calls SVD. roots rejects complex/nonfinite, removes leading zeros, records trailing zeros, constructs companion first row -c[j+1]/c0 and subdiagonal1 then calls eig.

Executable Rust surrogate gap: no calibrated replacement is attached. Exact test calls are evidence for the reference, not a fast approximant. Numerical approximants would need a named domain, zero convention and metric at supported caps 1/2/5/10%; categorical errors/effects have no percent surrogate here.

## invariant

### Invariants and cost

Highest-degree coefficient first. Degree cast to usize/powi i32 not range-certified; eig result ordering is backend-dependent.

#### Cost

Vandermonde O(Md), SVD O(Md min(M,d)); companion O(d²) storage/O(d³) eig.

Allocation failure, unchecked usize dimension arithmetic and IEEE nonfinite propagation are outside finite fixture guarantees. No measured throughput or global accuracy claim.

## models

# Typed source models: mlab-numerics-poly-fit-roots

## behavior_complement

No roots eig call for constant/all-zero polynomial. polyfit's nargout error excludes further length/degree decision after initial argument parsing. Imaginary x/y cannot affect fitted Vandermonde in current code.

## reverse_reconstruction

Given monic roots, coefficient families differ by nonzero scale and leading zeros; trailing root zeros remain visible. Least-squares coefficients are compatible with residuals orthogonal to retained SVD column space, not a unique y.

## leading_trim

[0,1,-3,2] and [1,-3,2] yield same companion.

## trailing_roots

[1,-2,0] evaluates eig for residual degree1 then appends zero root.

## finite_gate

Any NaN/Inf coefficient errors before trimming, including nominal leading zeros around it.

## caller_join

roots uses it.call_function("eig",...,1); overridden/missing dispatch and non-Mat result are not backend proofs.


## tests

### Evidence and limits

Selected wave30_tests exercise this source; fragment-specific limits are stated in its operators.

Native command specifications, numeric exits, profile-paired logs, compiled controls and exact-restoration receipts are retained externally in the wave30 numerics run. Tests exercise actual source, not a copied model. The selected finite fixtures do not prove full convergence, backend accuracy on all matrices, resource exhaustion or side-effectful callback behavior.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source reading preceded inline tests. This node records implementation behavior, not a complete MATLAB compatibility contract. Models retain limits discovered in source reading, including real-only projections and warning-only result gates. Synthetic CPU fixtures are independent of frozen suites. No external effects, teacher, corpora or benchmark main are executed.
