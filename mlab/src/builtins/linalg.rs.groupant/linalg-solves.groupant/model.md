# mlab-numerics-linalg-solves

## descriptant

### mlab-numerics-linalg-solves

lstsq_svd computes X=V*diag(1/s retained)*U^T B, ignoring s<=max(m,n)*smax*eps. lstsq_qr pivots largest remaining column norm (strict greater, earliest tie), reflects RHS, triangular-solves and unpermutes; returns None when m<n,n=0 or pivot below tolerance.

## approximant

### Forward relation

lstsq_svd computes X=V*diag(1/s retained)*U^T B, ignoring s<=max(m,n)*smax*eps. lstsq_qr pivots largest remaining column norm (strict greater, earliest tie), reflects RHS, triangular-solves and unpermutes; returns None when m<n,n=0 or pivot below tolerance.

Executable Rust surrogate gap: no calibrated replacement is attached. Exact test calls are evidence for the reference, not a fast approximant. Numerical approximants would need a named domain, zero convention and metric at supported caps 1/2/5/10%; categorical errors/effects have no percent surrogate here.

## invariant

### Invariants and cost

Output rows=n and columns=b.cols; QR preserves original unknown order via perm. Valid b.rows=m and finite nonpathological data assumed.

#### Cost

O(m*n*min(m,n)+RHS*m*n); QR keeps column vectors and RHS copies, SVD retains thin factors.

Allocation failure, unchecked usize dimension arithmetic and IEEE nonfinite propagation are outside finite fixture guarantees. No measured throughput or global accuracy claim.

## models

# Typed source models: mlab-numerics-linalg-solves

## behavior_complement

Zero-row/column SVD excludes backend invocation and returns zero-shaped solution. QR rejects rank-deficient pivots; it cannot certify arbitrary rank-deficient X. Neither helper emits interpreter warnings itself.

## reverse_reconstruction

For fixed full-column-rank A, X is compatible with B=A X+R where A^T R≈0; SVD additionally requires no nullspace component beyond numerical tolerance. Underdetermined input fibers are broad.

## pivot_ties

Strict nj>bn leaves earliest current column on equal norms; columns and permutation swap together.

## svd_zero

m==0 or n==0 returns zeros(n,b.cols) before decomposition.

## qr_rank

Zero3x2 returns None; tall [[1,0],[0,1],[1,1]] and B[2,3,5] yield [2,3].

## rhs_projection

Each RHS projects against U then divides retained singular values; incompatible B shape can index panic, so wrapper validation matters.


## tests

### Evidence and limits

Selected wave30_tests exercise this source; fragment-specific limits are stated in its operators.

Native command specifications, numeric exits, profile-paired logs, compiled controls and exact-restoration receipts are retained externally in the wave30 numerics run. Tests exercise actual source, not a copied model. The selected finite fixtures do not prove full convergence, backend accuracy on all matrices, resource exhaustion or side-effectful callback behavior.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source reading preceded inline tests. This node records implementation behavior, not a complete MATLAB compatibility contract. Models retain limits discovered in source reading, including real-only projections and warning-only result gates. Synthetic CPU fixtures are independent of frozen suites. No external effects, teacher, corpora or benchmark main are executed.
