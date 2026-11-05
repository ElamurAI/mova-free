# algorithm-jacobi-iteration-tests

## descriptant

### Descriptant and retained epoch

Original std-only jacobi_iteration test harness. Reference: Varga (1962), Matrix Iterative Analysis. No source code or fixtures were copied from the citation.

All arithmetic is deterministic f64 in index order, with explicit nonfinite intermediate checks. Underflow and rounding can change real identities. No callbacks or random state. Input slices are not mutated. Size checks precede allocation; try_reserve_exact failure becomes AllocationFailed. Finite reports contain finite coordinates/residuals. No allocator-failure injection or universal conditioning analysis is supplied.

Retained observation: 405 strictly diagonally dominant manufactured 2x2 systems; exact one-sweep dependency witness; initial convergence and zero budget; scalar denominators 2..200 exposing nonzero-residual stagnation; invalid precedence; overflow; bounded divergence and n=64 diagonal solve.

## model

### Mathematical combiner

Intersect the accepted domain, independent oracle assertions and the typed relations in model.md#models with partial Rust syntax and named finite observations. With A=D+L+U, x_new=D^-1(b-(L+U)*x_old). All coordinates read the old vector; swap buffers only after the complete sweep.

Cost composition: O(k*n^2) for k sweeps plus k+1 fresh residual evaluations; O(n) storage; two checked n-vector reservations; swap-buffer simultaneous sweeps. Initial residual is evaluated even at budget zero. Test modules are sibling source nodes, not implementation children. Raw source remains authoritative; source hashes and syntax projections bind the epoch but do not provide macro expansion or semantic AST.

All arithmetic is deterministic f64 in index order, with explicit nonfinite intermediate checks. Underflow and rounding can change real identities. No callbacks or random state. Input slices are not mutated. Size checks precede allocation; try_reserve_exact failure becomes AllocationFailed. Finite reports contain finite coordinates/residuals. No allocator-failure injection or universal conditioning analysis is supplied.

## approximant

### Forward approximant and invariant

Domain: Square 1<=n<=64 with matching rhs/initial; finite input magnitudes <=1e100, nonzero diagonal, positive finite tolerance, max_iterations<=10000. No contraction or SPD assumption is required for admission.

405 strictly diagonally dominant manufactured 2x2 systems; exact one-sweep dependency witness; initial convergence and zero budget; scalar denominators 2..200 exposing nonzero-residual stagnation; invalid precedence; overflow; bounded divergence and n=64 diagonal solve.

Cost: O(k*n^2) for k sweeps plus k+1 fresh residual evaluations; O(n) storage; two checked n-vector reservations; swap-buffer simultaneous sweeps. Initial residual is evaluated even at budget zero.

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

405 strictly diagonally dominant manufactured 2x2 systems; exact one-sweep dependency witness; initial convergence and zero budget; scalar denominators 2..200 exposing nonzero-residual stagnation; invalid precedence; overflow; bounded divergence and n=64 diagonal solve.

## mutation_sensitivity

one_sweep_dependency_order

## boundary_partition

Finite fixtures partition arithmetic success, structural/input rejection and resource-size boundaries. Stationary tests additionally separate convergence, budget and representational stagnation. Allocator failure is modeled but not injected; these observations cannot determine its frequency.


## tests

### Checks

`cargo test -p mova-algorithms jacobi_iteration::tests`

405 strictly diagonally dominant manufactured 2x2 systems; exact one-sweep dependency witness; initial convergence and zero budget; scalar denominators 2..200 exposing nonzero-residual stagnation; invalid precedence; overflow; bounded divergence and n=64 diagonal solve.

Compiled production mutant must fail `one_sweep_dependency_order` by assertion, then byte restoration must pass the focused suite. Source fixtures are original and deterministic.

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

Compiled jacobi_iteration production mutant fails named assertion `one_sweep_dependency_order`; exact source restored and focused positive suite passes. One mutation shows finite local sensitivity, not comprehensive mutation coverage.

## History

### 2026-10-30 · descriptant

Prospective model was saved before implementation in $MOVA_DATA/runs/octopus-wave20-linear-2026-10-30/prospective.md and retained beside this source as model.md#prospective.

2026-10-30: bounded dense storage and fixed dependency order were selected to make costs and termination inspectable. Optimization hypothesis: reuse compact scratch buffers rather than allocate each iteration; no timing gain was measured or claimed.

### 2026-10-30 · validation

Heading `### Validation epoch 2026-10-30` stays in validation as `### Validation`.

Receipts: $MOVA_DATA/runs/octopus-wave20-linear-2026-10-30/{debug1,debug2,release1,release2,mutant-jacobi_iteration,restored-jacobi_iteration}.log. Partial syntax projections are generated twice, compared byte-for-byte, then installed. Allocation failures remain unforced. No performance or universal numerical accuracy result is claimed.
