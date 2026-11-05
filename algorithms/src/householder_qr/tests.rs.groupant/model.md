# algorithm-householder-qr-tests

## descriptant

### Descriptant and retained epoch

Original std-only householder_qr test harness. Reference: Householder (1958), Unitary triangularization of a nonsymmetric matrix, Journal of the ACM 5(4). No source code or fixtures were copied from the citation.

All arithmetic is deterministic f64 in index order, with explicit nonfinite intermediate checks. Underflow and rounding can change real identities. No callbacks or random state. Input slices are not mutated. Size checks precede allocation; try_reserve_exact failure becomes AllocationFailed. Finite reports contain finite coordinates/residuals. No allocator-failure injection or universal conditioning analysis is supplied.

Retained observation: 245 tall 4x2 least-squares problems checked by independent 2x2 normal-equation formulas; positive/negative leading signs; square solutions; exact zero columns; shape/nonfinite/overflow errors; maximum 128x64 orthonormal-column fixture.

## model

### Mathematical combiner

Intersect the accepted domain, independent oracle assertions and the typed relations in model.md#models with partial Rust syntax and named finite observations. For column k choose alpha=-sign(Rkk)||R[k..,k]|| (zero sign positive), v=R[k..,k]-alpha*e1, normalize v. R<-R-2v(v^T R), c<-c-2v(v^T c). Back substitute leading triangular R. Report ||Ax-b||_2 and ||A^T(Ax-b)||_2 freshly.

Cost composition: O(m*n^2) reflector work plus O(n^2) substitution and O(m*n) fresh residuals; O(m*n+m+n) storage, four checked reservations (R,c,v,x). Test modules are sibling source nodes, not implementation children. Raw source remains authoritative; source hashes and syntax projections bind the epoch but do not provide macro expansion or semantic AST.

All arithmetic is deterministic f64 in index order, with explicit nonfinite intermediate checks. Underflow and rounding can change real identities. No callbacks or random state. Input slices are not mutated. Size checks precede allocation; try_reserve_exact failure becomes AllocationFailed. Finite reports contain finite coordinates/residuals. No allocator-failure injection or universal conditioning analysis is supplied.

## approximant

### Forward approximant and invariant

Domain: Rectangular 1<=n<=64 and n<=m<=128, rhs length m, finite magnitudes <=1e100. Exact-zero encountered column norm or pivot rejected; full numerical rank is not certified.

245 tall 4x2 least-squares problems checked by independent 2x2 normal-equation formulas; positive/negative leading signs; square solutions; exact zero columns; shape/nonfinite/overflow errors; maximum 128x64 orthonormal-column fixture.

Cost: O(m*n^2) reflector work plus O(n^2) substitution and O(m*n) fresh residuals; O(m*n+m+n) storage, four checked reservations (R,c,v,x).

All arithmetic is deterministic f64 in index order, with explicit nonfinite intermediate checks. Underflow and rounding can change real identities. No callbacks or random state. Input slices are not mutated. Size checks precede allocation; try_reserve_exact failure becomes AllocationFailed. Finite reports contain finite coordinates/residuals. No allocator-failure injection or universal conditioning analysis is supplied.

Harness runtime sums the finite fixture invocations; assertions give finite observations only.

No executable 1/2/5/10% surrogate is attached. A residual is not a conditioning-free relative solution-error metric, whose zero-reference case would need separate treatment. No faster calibrated surrogate or universal floating error cap is claimed.

## models

# Typed obernant family

## behavior_complement

Let R be the conjunction of the assertions on the named finite fixtures. The complementary observation relation contains disagreeing coordinates, accepted rejected-domain cases, incorrect residual/status, or an unexpected successful mutant. The suite does not observe every bounded input or allocator state; passing is not universal membership in the production relation.

## reverse_reconstruction

A passing test transcript reconstructs the fiber of all implementations agreeing on these finite fixture observations. An implementation differing only on an untested matrix remains in that fiber; the source hash identifies this implementation but pass/fail does not identify its semantics.

## independent_oracle

245 tall 4x2 least-squares problems checked by independent 2x2 normal-equation formulas; positive/negative leading signs; square solutions; exact zero columns; shape/nonfinite/overflow errors; maximum 128x64 orthonormal-column fixture.

## mutation_sensitivity

tall_least_squares_oracle

## boundary_partition

Finite fixtures partition arithmetic success, structural/input rejection and resource-size boundaries. Stationary tests additionally separate convergence, budget and representational stagnation. Allocator failure is modeled but not injected; these observations cannot determine its frequency.


## tests

### Checks

`cargo test -p mova-algorithms householder_qr::tests`

245 tall 4x2 least-squares problems checked by independent 2x2 normal-equation formulas; positive/negative leading signs; square solutions; exact zero columns; shape/nonfinite/overflow errors; maximum 128x64 orthonormal-column fixture.

Compiled production mutant must fail `tall_least_squares_oracle` by assertion, then byte restoration must pass the focused suite. Source fixtures are original and deterministic.

## prospective

### Prospective model before source changes
Five original standard-library dense algorithms. n=1..64, QR n<=m<=128, finite inputs <=1e100; stationary budget <=10000. Validate shapes/value/options before structure then allocation. Checked try_reserve buffers; no callbacks. No approximate profiles because residual is not conditioning-free solution error. All sums deterministic increasing index.
Cholesky: Lij=(Aij-sum Lik Ljk)/Ljj and Lii=sqrt(Aii-sum Lik²); exact symmetry and positive encountered pivots; O(n³) work/O(n²) output; reconstruct LLᵀ to measure Frobenius residual. No claim SPD certification under rounding.
Triangular solve: xi=(bi-sum Tij xj)/Tii in dependency order; reject nonzero opposite triangle and zero diagonal; O(n²)/O(n). Fresh Euclidean residual. No iteration budget applies to fixed finite loops.
Householder QR: sign avoiding cancellation, normalized reflector v, R<-R-2v(vᵀR), c<-c-2v(vᵀc), back substitute. Tall least squares O(mn²), O(mn) memory. Reject exact zero encountered norm/diagonal; report fresh primal and normal-equation residual. Rank threshold deliberately absent; ill conditioning not certified.
Jacobi uses old iterate for every component; Gauss-Seidel uses newest prefix, old suffix. O(kn²)/O(n), no convergence assumption; check fresh residual then budget then exact unchanged iterate; divergence overflow is explicit arithmetic error. Finite stopping never promises solution accuracy.
Expected optimization is row-major bounded scratch reuse (Jacobi 2 vectors, Gauss-Seidel 1 update vector), no empirical speed claim. Tests will include exact manufactured solutions, independent products, rejection precedence, rank/pivot failures, zero budget and one-step distinctions. Mutants alter recurrence/diagonal/reflector and must fail named assertions.

## validation

### Validation

Two debug and two release library runs passed 573 tests each, including 26 native tests in this five-algorithm batch. Sorted named outcomes match in all four executions; elapsed time is intentionally excluded. Strict all-target algorithm Clippy and focused rustfmt pass.

Compiled householder_qr production mutant fails named assertion `tall_least_squares_oracle`; exact source restored and focused positive suite passes. One mutation shows finite local sensitivity, not comprehensive mutation coverage.

## History

### 2026-10-30 · descriptant

Prospective model was saved before implementation in $MOVA_DATA/runs/octopus-wave20-linear-2026-10-30/prospective.md and retained beside this source as model.md#prospective.

2026-10-30: bounded dense storage and fixed dependency order were selected to make costs and termination inspectable. Optimization hypothesis: reuse compact scratch buffers rather than allocate each iteration; no timing gain was measured or claimed.

### 2026-10-30 · validation

Heading `### Validation epoch 2026-10-30` stays in validation as `### Validation`.

Receipts: $MOVA_DATA/runs/octopus-wave20-linear-2026-10-30/{debug1,debug2,release1,release2,mutant-householder_qr,restored-householder_qr}.log. Partial syntax projections are generated twice, compared byte-for-byte, then installed. Allocation failures remain unforced. No performance or universal numerical accuracy result is claimed.
