# mlab-numerics-linalg-factors

## descriptant

### mlab-numerics-linalg-factors

lu returns packed L/U for <=1 output, P^T L and U for2, or L,U,P for3+. qr computes full/thin Q/R and zeros below R diagonal. chol symmetrizes upper input triangle, returning R with R^T R=upper-symmetric A or empty,status1 on two-output failure. eig dispatches exact symmetry then returns values or V,D and warning residuals.

## approximant

### Forward relation

lu returns packed L/U for <=1 output, P^T L and U for2, or L,U,P for3+. qr computes full/thin Q/R and zeros below R diagonal. chol symmetrizes upper input triangle, returning R with R^T R=upper-symmetric A or empty,status1 on two-output failure. eig dispatches exact symmetry then returns values or V,D and warning residuals.

Executable Rust surrogate gap: no calibrated replacement is attached. Exact test calls are evidence for the reference, not a fast approximant. Numerical approximants would need a named domain, zero convention and metric at supported caps 1/2/5/10%; categorical errors/effects have no percent surrogate here.

## invariant

### Invariants and cost

Factor shapes depend on nargout/economy. Gates use finite input and relative residual with explicit zero-reference behavior; no rejection on gate violation.

#### Cost

Decompositions O(n^3) square with O(n^2) storage; reconstruction gates add O(n^3); diagonal eigen scaling O(n^2).

Allocation failure, unchecked usize dimension arithmetic and IEEE nonfinite propagation are outside finite fixture guarantees. No measured throughput or global accuracy claim.

## models

# Typed source models: mlab-numerics-linalg-factors

## behavior_complement

LU's two-output L excludes interpreting it as unit lower triangular without permutation. chol excludes lower-triangle influence. eig nonsquare/complex/nonfinite checks precede empty/backend evaluation.

## reverse_reconstruction

Three-output LU reconstructs A=P^T L U; two-output factors reconstruct A=L2 U. chol R reconstructs symmetric upper-selected A but arbitrary lower-triangle input shares output. V,D reconstruct A only if V invertible; values-only fibers include similarity transforms.

## lu_output_arity

nargout2 uses P^T L; nargout3 emits permutation matrix.

## chol_failure

Positive-definite failure returns status1 if requested, otherwise MError; upper triangle defines the factorized symmetric matrix.

## eigen_symmetry

Symmetric branch uses exact mirrored-cell equality; nonsymmetric vectors are normalized by Euclidean column norm.

## residual_limits

Value-only eigen gate checks sum eigenvalues versus trace, which cannot detect trace-preserving errors.


## tests

### Evidence and limits

Selected wave30_tests exercise this source; fragment-specific limits are stated in its operators.

Native command specifications, numeric exits, profile-paired logs, compiled controls and exact-restoration receipts are retained externally in the wave30 numerics run. Tests exercise actual source, not a copied model. The selected finite fixtures do not prove full convergence, backend accuracy on all matrices, resource exhaustion or side-effectful callback behavior.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source reading preceded inline tests. This node records implementation behavior, not a complete MATLAB compatibility contract. Models retain limits discovered in source reading, including real-only projections and warning-only result gates. Synthetic CPU fixtures are independent of frozen suites. No external effects, teacher, corpora or benchmark main are executed.
