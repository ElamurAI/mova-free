# algorithm-triangular-solve

## descriptant

### Descriptant and retained epoch

Original std-only triangular_solve implementation. Reference: Golub and Van Loan, Matrix Computations, fourth edition (2013), section 3.1. No source code or fixtures were copied from the citation.

All arithmetic is deterministic f64 in index order, with explicit nonfinite intermediate checks. Underflow and rounding can change real identities. No callbacks or random state. Input slices are not mutated. Size checks precede allocation; try_reserve_exact failure becomes AllocationFailed. Finite reports contain finite coordinates/residuals. No allocator-failure injection or universal conditioning analysis is supplied.

Retained observation: 4,000 exact two-dimensional integer manufactured systems across both directions, diagonals 1..4, off-diagonal and coordinates -2..2; structural-error precedence; subnormal-pivot overflow; negative diagonal at n=64.

## model

### Mathematical combiner

Intersect the accepted domain, recurrence invariants and the typed relations in model.md#models with partial Rust syntax and named finite observations. Lower order i=0..n-1 or upper order i=n-1..0: xi=(bi-sum(dependencies j) Tij*xj)/Tii. Compute rho=||Tx-b||_2 freshly.

Cost composition: O(n^2) validation/substitution/residual work; O(n) output; one checked reservation. Test modules are sibling source nodes, not implementation children. Raw source remains authoritative; source hashes and syntax projections bind the epoch but do not provide macro expansion or semantic AST.

All arithmetic is deterministic f64 in index order, with explicit nonfinite intermediate checks. Underflow and rounding can change real identities. No callbacks or random state. Input slices are not mutated. Size checks precede allocation; try_reserve_exact failure becomes AllocationFailed. Finite reports contain finite coordinates/residuals. No allocator-failure injection or universal conditioning analysis is supplied.

## approximant

### Forward approximant and invariant

Domain: Square 1<=n<=64, matching rhs; finite magnitudes <=1e100; requested opposite triangle exactly zero; every diagonal nonzero, either sign accepted.

Lower order i=0..n-1 or upper order i=n-1..0: xi=(bi-sum(dependencies j) Tij*xj)/Tii. Compute rho=||Tx-b||_2 freshly.

Cost: O(n^2) validation/substitution/residual work; O(n) output; one checked reservation.

All arithmetic is deterministic f64 in index order, with explicit nonfinite intermediate checks. Underflow and rounding can change real identities. No callbacks or random state. Input slices are not mutated. Size checks precede allocation; try_reserve_exact failure becomes AllocationFailed. Finite reports contain finite coordinates/residuals. No allocator-failure injection or universal conditioning analysis is supplied.

Real equations model the intended transformation; they do not certify machine accuracy.

No executable 1/2/5/10% surrogate is attached. A residual is not a conditioning-free relative solution-error metric, whose zero-reference case would need separate treatment. No faster calibrated surrogate or universal floating error cap is claimed.

## models

# Typed obernant family

## behavior_complement

The accepted dependency relation excludes use of future unsolved coordinates and nonzero entries in the forbidden triangle. It excludes interpreting a lower matrix as upper without transposition; it does not invert or factor a general dense matrix. Zero diagonal is structural failure even for zero rhs.

## reverse_reconstruction

For any finite returned x, every admissible nonsingular triangular T paired with b=T x is a real antecedent with zero residual. Thus x or (x,rho) cannot recover T or rhs uniquely. When T is fixed, b=T x is unique in exact arithmetic; floating rounding enlarges fibers.

## dependency_order

Lower order consumes j<i, upper order consumes j>i. Transposing a lower matrix changes the solve direction; preserving the flag would violate the dependency relation.

## diagonal_scaling

For real nonzero c, replacing (T,b) with (cT,cb) preserves x, provided input bounds hold. Floating rounding/residual magnitudes need not preserve bitwise output.

## residual_reconstruction

After solving, rho is a fresh Euclidean norm of Tx-b. It is not the norm of an internal recursively updated residual and does not bound coordinate error without ||T^-1||.

## validation_partition

Shapes and finite bounds are checked before triangle structure, which is checked before all diagonals, followed by allocation. A zero diagonal plus forbidden entry is NotTriangular; a nonfinite entry dominates both.


## tests

### Checks

`cargo test -p mova-algorithms triangular_solve::tests`

4,000 exact two-dimensional integer manufactured systems across both directions, diagonals 1..4, off-diagonal and coordinates -2..2; structural-error precedence; subnormal-pivot overflow; negative diagonal at n=64.

Compiled production mutant must fail `exhaustive_both_directions` by assertion, then byte restoration must pass the focused suite. Source fixtures are original and deterministic.

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

Compiled triangular_solve production mutant fails named assertion `exhaustive_both_directions`; exact source restored and focused positive suite passes. One mutation shows finite local sensitivity, not comprehensive mutation coverage.

## History

### 2026-10-30 · descriptant

Prospective model was saved before implementation in $MOVA_DATA/runs/octopus-wave20-linear-2026-10-30/prospective.md and retained beside this source as model.md#prospective.

2026-10-30: bounded dense storage and fixed dependency order were selected to make costs and termination inspectable. Optimization hypothesis: reuse compact scratch buffers rather than allocate each iteration; no timing gain was measured or claimed.

### 2026-10-30 · validation

Heading `### Validation epoch 2026-10-30` stays in validation as `### Validation`.

Receipts: $MOVA_DATA/runs/octopus-wave20-linear-2026-10-30/{debug1,debug2,release1,release2,mutant-triangular_solve,restored-triangular_solve}.log. Partial syntax projections are generated twice, compared byte-for-byte, then installed. Allocation failures remain unforced. No performance or universal numerical accuracy result is claimed.
