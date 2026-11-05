# mlab-numerics-linalg

## descriptant

### mlab-numerics-linalg

Public builtins adapt column-major Mat to faer. Real square mldivide uses LU and reciprocal condition estimate, switching to minimum-norm SVD when rc<eps; tall full-rank uses pivoted QR, otherwise SVD. Complex square solve is separate elimination. Factor/eigen/SVD outputs vary by nargout; gates warn on residuals rather than reject.

## approximant

### Forward relation

Public builtins adapt column-major Mat to faer. Real square mldivide uses LU and reciprocal condition estimate, switching to minimum-norm SVD when rc<eps; tall full-rank uses pivoted QR, otherwise SVD. Complex square solve is separate elimination. Factor/eigen/SVD outputs vary by nargout; gates warn on residuals rather than reject.

Executable Rust surrogate gap: no calibrated replacement is attached. Exact test calls are evidence for the reference, not a fast approximant. Numerical approximants would need a named domain, zero convention and metric at supported caps 1/2/5/10%; categorical errors/effects have no percent surrogate here.

## invariant

### Invariants and cost

Mat/faer transport uses column-major order; decomposition gates compare observations with 1000*max(n,1)*eps. Warnings are interpreter effects. Direct low-level helpers assume compatible shapes; not all validate B.

#### Cost

Dense decomposition roughly O(mn min(m,n)), square O(n^3), O(mn+n^2) workspaces; residual products add matrix multiplication. QR custom reflectors and SVD solves scale with RHS count. Backend convergence and allocations remain delegated.

Allocation failure, unchecked usize dimension arithmetic and IEEE nonfinite propagation are outside finite fixture guarantees. No measured throughput or global accuracy claim.

## composition

### Composition

polyfit directly uses lstsq_svd; roots/poly call eig through Interp. ops routes matrix division to mldivide. Transport, solve and factor fragments connect at shared Mat shape/order; warning gates are observations, not hard feasibility filters.

Fragments refer to disjoint useful source scopes in the single parent lexical projection. Registration and inline tests remain parent-modeled. Source callbacks and external backend implementations are dependencies, not duplicated ownership. Local tolerances cannot be composed into a subsystem percent guarantee.

## models

# Typed source models: mlab-numerics-linalg

## behavior_complement

Gates exclude silent acceptance only when their finite conditions and tolerance comparisons trigger; warnings do not exclude returned results. Complex nonsquare solve excludes success. eig/svd reject nonfinite input, unlike a universal wrapper policy. Singular inv yields Inf matrix plus warning.

## reverse_reconstruction

Given A and exact full-rank X solving AX=B, B=AX is reconstructible; given B and X alone many A fit. Rank-deficient solves choose a minimum-norm representative only under finite numerical thresholds. LU factors with P imply A=P^T LU; eigenvalues alone define a broad similarity fiber.

## representation_transport

view uses from_column_major_slice; from_faer emits columns. P row i copies original row p[i], so P*A=L*U.

## rank_thresholds

lstsq_svd retains s>max(m,n)*smax*eps. rank default uses max(m,n)*2^(floor(log2(smax))-52), not the identical threshold.

## warning_gate

gate_solve returns early for nonfinite inputs; finite inputs/nonfinite solution warn. Fault modes deliberately perturb x/eig/SVD and retain outputs.

## empty_and_complex

Empty real solve returns zeros(n,b.cols); inv empty returns empty, det empty returns1. complex_solve requires square and rejects exact zero pivots.


## tests

### Executed evidence

builtins::linalg::wave30_tests::transport_and_rectangular_solve plus existing builtins::linalg::tests::lu_perm_convention: representation, QR/SVD agreement, rank-deficient fallback boundary, empty solve and LU permutation.

#### Native selection

    cargo test --offline --locked -p mlab --lib builtins::linalg::wave30_tests::transport_and_rectangular_solve -- --exact

Each selected test passed twice in debug and twice in release with numeric exit 0. Existing lu_perm_convention also ran in every profile/pass. Eight aggregate named outcomes were extracted from actual logs, sorted, and identical across all four runs. Post-control exact-restoration runs passed both profiles with separately retained numeric exits.

#### Compiled negative control

In this source, the expression

    sign = -sign;

was temporarily replaced by

    sign = sign;

Cargo compiled successfully and the selected test reached an assertion failure with exit 101. The complete original source bytes were restored and compared before positive reruns. This control validates the selected observation, not all branches.

#### Scope and limits

Parent and fragment models distinguish executed fixtures from source-only branches. No benchmark main, held-out datasets, network, teacher or GPU run. No numerical approximation surrogate, exhaustive convergence proof, allocation-failure injection or speed claim. Strict Clippy was not rerun for this test/model-only change; added test blocks were individually rustfmt-formatted without changing production formatting. External receipts retain native commands, test maps, logs, exits, mutations and structural checks.

## History

### 2026-10-31 · descriptant

#### Modeling epoch 2026-10-31

Prospective source reading preceded inline tests. This node records implementation behavior, not a complete MATLAB compatibility contract. Models retain limits discovered in source reading, including real-only projections and warning-only result gates. Synthetic CPU fixtures are independent of frozen suites. No external effects, teacher, corpora or benchmark main are executed.

### 2026-10-31 · tests

Heading `### Executed evidence — 2026-10-31` stays in tests as `### Executed evidence`.
