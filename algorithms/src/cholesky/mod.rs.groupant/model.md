# algorithm-cholesky

## descriptant

### Descriptant and retained epoch

Original std-only cholesky implementation. Reference: Golub and Van Loan, Matrix Computations, fourth edition (2013), section 4.2. No source code or fixtures were copied from the citation.

All arithmetic is deterministic f64 in index order, with explicit nonfinite intermediate checks. Underflow and rounding can change real identities. No callbacks or random state. Input slices are not mutated. Size checks precede allocation; try_reserve_exact failure becomes AllocationFailed. Finite reports contain finite coordinates/residuals. No allocator-failure injection or universal conditioning analysis is supplied.

Retained observation: 320 integer 3x3 Gram matrices independently constructed from positive-diagonal lower factors; irrational 2x2 residual; asymmetric, zero and negative pivots; input precedence; subnormal and 1e100 scalars; dimension 64 identity.

## model

### Mathematical combiner

Intersect the accepted domain, recurrence invariants and the typed relations in model.md#models with partial Rust syntax and named finite observations. For j<=i, s=Aij-sum(k<j) Lik*Ljk; Lii=sqrt(s), Lij=s/Ljj when i>j. Upper entries remain zero. Compute rho=||LL^T-A||_F by fresh deterministic products.

Cost composition: O(n^3) arithmetic including fresh reconstruction; O(n^2) owned output; n+1 checked reservations (outer rows and n row buffers). Test modules are sibling source nodes, not implementation children. Raw source remains authoritative; source hashes and syntax projections bind the epoch but do not provide macro expansion or semantic AST.

All arithmetic is deterministic f64 in index order, with explicit nonfinite intermediate checks. Underflow and rounding can change real identities. No callbacks or random state. Input slices are not mutated. Size checks precede allocation; try_reserve_exact failure becomes AllocationFailed. Finite reports contain finite coordinates/residuals. No allocator-failure injection or universal conditioning analysis is supplied.

## approximant

### Forward approximant and invariant

Domain: Square 1<=n<=64; entries finite with |a|<=1e100; exact symmetry; every encountered Schur pivot positive.

For j<=i, s=Aij-sum(k<j) Lik*Ljk; Lii=sqrt(s), Lij=s/Ljj when i>j. Upper entries remain zero. Compute rho=||LL^T-A||_F by fresh deterministic products.

Cost: O(n^3) arithmetic including fresh reconstruction; O(n^2) owned output; n+1 checked reservations (outer rows and n row buffers).

All arithmetic is deterministic f64 in index order, with explicit nonfinite intermediate checks. Underflow and rounding can change real identities. No callbacks or random state. Input slices are not mutated. Size checks precede allocation; try_reserve_exact failure becomes AllocationFailed. Finite reports contain finite coordinates/residuals. No allocator-failure injection or universal conditioning analysis is supplied.

Real equations model the intended transformation; they do not certify machine accuracy.

No executable 1/2/5/10% surrogate is attached. A residual is not a conditioning-free relative solution-error metric, whose zero-reference case would need separate treatment. No faster calibrated surrogate or universal floating error cap is claimed.

## models

# Typed obernant family

## behavior_complement

Within the accepted matrix domain, the factor relation excludes an upper-triangle output entry other than zero and excludes negative factor diagonals. It does not map nonsymmetric matrices to an approximate symmetric factor or produce eigenvalues. A finite positive pivot sequence under rounding is not a general SPD proof.

## reverse_reconstruction

In real arithmetic a lower triangular positive-diagonal L uniquely reconstructs A=LL^T. The floating output observation (L,rho) instead admits all bounded exactly symmetric matrices producing that rounded L and that measured residual; distinct nearby input values can collapse. rho alone leaves infinitely many factor/matrix antecedents in the real model.

## schur_dependency

At step (i,j), only columns k<j contribute. In reals s is the Schur-complement entry. A nonpositive diagonal s prevents a real positive-diagonal continuation; this encountered-pivot condition is the actual floating error branch.

## triangular_positivity

Output rows are initialized to zero, written only at j<=i, and each diagonal is sqrt(s)>0. This structural relation is stronger than a small reconstruction residual.

## reconstruction_observation

rho accumulates hypot of each fresh dot(Li,Lj)-Aij. The real identity A=LL^T motivates the measurement; finite hypot and rounded products do not certify a relative solution-error cap.

## factor_solve_composition

In exact arithmetic forward solve Ly=b followed by backward solve L^T x=y gives Ax=b for A=LL^T. The standalone factor does not perform solves or promise the composed floating error. Reusing factors costs two O(n^2) substitutions per rhs.


## tests

### Checks

`cargo test -p mova-algorithms cholesky::tests`

320 integer 3x3 Gram matrices independently constructed from positive-diagonal lower factors; irrational 2x2 residual; asymmetric, zero and negative pivots; input precedence; subnormal and 1e100 scalars; dimension 64 identity.

Compiled production mutant must fail `exhaustive_gram_reconstruction` by assertion, then byte restoration must pass the focused suite. Source fixtures are original and deterministic.

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

Compiled cholesky production mutant fails named assertion `exhaustive_gram_reconstruction`; exact source restored and focused positive suite passes. One mutation shows finite local sensitivity, not comprehensive mutation coverage.

## History

### 2026-10-30 · descriptant

Prospective model was saved before implementation in $MOVA_DATA/runs/octopus-wave20-linear-2026-10-30/prospective.md and retained beside this source as model.md#prospective.

2026-10-30: bounded dense storage and fixed dependency order were selected to make costs and termination inspectable. Optimization hypothesis: reuse compact scratch buffers rather than allocate each iteration; no timing gain was measured or claimed.

### 2026-10-30 · validation

Heading `### Validation epoch 2026-10-30` stays in validation as `### Validation`.

Receipts: $MOVA_DATA/runs/octopus-wave20-linear-2026-10-30/{debug1,debug2,release1,release2,mutant-cholesky,restored-cholesky}.log. Partial syntax projections are generated twice, compared byte-for-byte, then installed. Allocation failures remain unforced. No performance or universal numerical accuracy result is claimed.
