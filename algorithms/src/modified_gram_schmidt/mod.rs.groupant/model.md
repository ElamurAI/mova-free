# algorithm-modified-gram-schmidt

## descriptant

### One-pass modified Gram-Schmidt with reconstruction and orthogonality

Original std-only bounded Rust. Citations are in the implementation module. This file groupant preserves the prospective model, typed excluded-output and compatible-preimage relations, independent checks and exact source binding.

Model correction: the Jacobi implementation recomputes every original-matrix eigenpair residual each round, so its actual bound is O(b*n^3), stronger work than the prospective O(b*n^2+n^3) sketch. This is retained as a modeling correction, not a speed claim.

Gaps: no semantic AST or macro expansion, allocator exhaustion injection, general floating error proof, or calibrated approximate profiles.

## model

### Mathematical combiner

Inputs: exact mod.rs, source-bound partial syntax, prospective recurrence, current forward/invariant, typed family and assertion outcomes. First validate the domain; compose ordered updates under dimension and finite-arithmetic guards; compare fresh observations with independent oracle relations. No logical implication from a finite pass to universal accuracy is allowed.

The shared helper dependency is algorithms/src/lu_decomposition/mod.rs: checked allocation, finite arithmetic, shape/control validation and vector operations. LU is the owner and the remaining modules import only crate-private helpers plus the shared public Error. Cross-file correctness still requires integration exports and tests; these leaf models do not prove wiring.

Epoch memory retains the original prospective math and its corrected Jacobi cost. Zero residual is an observed rounded equality; ambiguous preimages remain explicit.

## approximant

### Forward approximant and invariant

Domain: 1<=n<=64, n<=m<=128; finite entries magnitude <=1e100; absolute tolerance [0,1e100].

For column j, v=A[:,j]. For k<j take Rkj=qk^T v then v-=Rkj*qk. Rjj=||v||2; norm<=tolerance fails with DependentColumn{index:j}. Else qj=v/Rjj. Return row-major thin Q and R; rank=n on success. Fresh ||QR-A||F and ||Q^TQ-I||F.

Cost: O(m*n^2) arithmetic, O(m*n+n^2) storage; checked outer/row buffers and per-column temporary.

Validation precedence: shape/value bounds first, then control parameters, symmetry where applicable, initial vector where applicable, checked allocations, then arithmetic/encountered-pivot failures in recurrence order. No callbacks, random state, caller-input mutation or persistent effects. Checked reservations map errors to AllocationFailed; Rust panic/OOM abort outside reserved bounded production allocations are not modeled. Finite helper checks reject nonfinite intermediates; rounded underflow remains permitted. Ties and signs follow model.md#models.

No executable 1/2/5/10% surrogate is attached. Residual and orthogonality norms are finite observations, not generic relative solution-error certificates. A meaningful percentage profile needs a separate domain, metric, zero-reference rule and analytical or finite evidence. No benchmark, production speedup or end-to-end Mova quality gain is claimed.

## models

# Typed obernant family

## behavior_complement

The accepted-output relation excludes negative diagonal R, a claimed partial rank after dependency failure, nonzero lower triangular R, and success when encountered column norm<=tolerance. No column pivoting or numerical rank-revelation claim follows.

## reverse_reconstruction

In reals Q,R reconstruct A=QR. Rounded Q,R with measured residual admit nearby input preimages that share rounded projections. Orthogonality residual alone permits many unrelated matrices. Positive diagonal fixes the QR sign convention when full column rank, but rounding and threshold decisions still lose input information.

## projection_order

Each coefficient uses the current residual v after all previous projections; using original column for every coefficient would be classical Gram-Schmidt, a different finite recurrence.

## dependency_threshold

Absolute norm<=tolerance, including equality and zero, fails at earliest column. This diagnoses only this unpivoted encountered residual and is not a numerical rank theorem.

## positive_diagonal_sign

Rjj=norm(v)>tolerance>=0; qj inherits residual sign. There is no independent largest-coordinate sign normalization, which would require compensating R signs.

## factor_observations

QR reconstruction and Q orthogonality are recomputed. Near dependence can degrade orthogonality despite small reconstruction; both independent observations remain visible.

## tests

### Checks

112 upper-triangular integer fixtures reconstructed independently; exact orthogonal and projection oracles; duplicate/zero columns, equality threshold, tiny scalar, dimensions, nonfinite values and near-dependent repeatability.

Run cargo test -p mova-algorithms modified_gram_schmidt::tests. Repeat debug and release outcomes. A compiled production mutant exercises a named assertion; exact source restoration must pass. Strict all-target Clippy and focused rustfmt complete static checks.

## prospective

### Prospective linear algebra epoch (before implementation)

Domain: finite entries bounded by 1e100, dimensions 1..64 (MGS m>=n up to 128), tolerance finite 0..1e100, iteration budgets 0..4096. Empty matrices rejected. Exact symmetry for Jacobi eigen and Lanczos. Checked reservations and finite intermediate checks; failures precede successful reports. No callbacks, randomness, mutation of caller data, timing or accuracy-cap claims.

LU: partial pivot chooses earliest maximal absolute entry; swap permutation and completed L columns. L unit lower triangular, U upper triangular; PA=LU in reals. Reject exactly zero encountered pivot. Fresh Frobenius reconstruction. O(n^3) work O(n^2) space.
MGS: copy column v; sequential dot and projection against prior normalized columns; positive Rjj=norm(v); reject norm<=tolerance as dependent column. A=QR in reals, Q column orthonormal. Fresh reconstruction and orthogonality. O(m*n^2), O(m*n+n^2) space. Not numerical rank revelation.
Jacobi eigen: repeatedly annihilate earliest largest upper offdiagonal via stable bounded angle rotation, accumulate column eigenvectors; stop by fresh original-matrix residual or budget. Return unsorted diagonal approximations and canonical signs (first largest-magnitude coordinate nonnegative). A V=V D and V^T V=I in exact converged arithmetic. O(b*n^2+n^3), O(n^2). Offdiagonal decisions deterministic.
Power: normalize initial vector; Rayleigh quotient and fresh norm(Ax-lambda*x); check residual then budget; normalize Ax, classify zero image breakdown if encountered. Canonical signs. O(b*n^2), O(n^2+n). Residual only certifies observed approximate eigenpair; no unconditional dominant eigenvalue promise (start may miss dominant eigenspace).
Lanczos: symmetric three-term recurrence alpha=q^T A q, w=Aq-alpha*q-beta_prev*q_prev; two full MGS reorthogonalization passes against every current basis vector. Nonnegative beta=norm(w). Break down if beta<=tol, complete if n columns reached, budget otherwise. Report actual ||AQ-QT||F including terminal residual and dropped finite reorthogonalization corrections; orthogonality observed separately. O(k*n^2+n*k^2), O(n*k). No implicit eigenvalue accuracy guarantee; beta threshold is absolute. No explicit generic 1/2/5/10% surrogates justified.

Oracles planned: exact 2x2 LU with pivot, QR on orthogonal columns and reconstruction, 2x2 symmetric analytic eigenvalues, diagonal power start/missing-component and zero budget, diagonal Lanczos full span/early invariant subspace. Boundary validation and compiled mutants for each algorithm. Repeated debug/release normalized named outcomes. No performance prediction beyond operation bounds until exclusive measurement grant.

## validation

### Validation

31 new tests (6 LU, 6 MGS, 6 Jacobi eigen, 6 power, 7 Lanczos); complete library 724 pass twice debug and twice release. Sorted nonempty test-name/outcome records agree in each mode. Strict cargo clippy -p mova-algorithms --all-targets -- -D warnings passes. Focused rustfmt passes.

Compiled mutant: diagonal normalization doubled; orthogonal_oracle must fail. Source restored byte-for-byte and named positive rerun passes. First exploratory LU mutation omitted the U row swap and failed by unwrap of an arithmetic error; it was not accepted as assertion evidence. The accepted LU mutation omits the permutation swap and fails assert_eq.

Integration boundary: checking the full augmented temporary registry reports 21 diagnostics, all confined to parent algorithm-library/mova child lists and transitive source bindings that the integration owner will refresh. No new leaf diagnostics occur in that report. The full hierarchy is not yet declared valid. A single-node registry cannot validate this whole checkout because the checker discovers all sidecars.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: 31 new named tests; complete library 724 tests passes twice debug and twice release with identical nonempty normalized outcomes. Five production mutants fail named assertions after compilation; restoration reruns pass. Strict algorithm all-target Clippy passes. No timing was performed.

### 2026-10-30 · validation

Heading `### Validation epoch 2026-10-30` stays in validation as `### Validation`.

Receipts: $MOVA_DATA/runs/octopus-wave26-linear-2026-10-30/ contains prospective.md, debug-1/2.log, release-1/2.log, normalized records, controls.log, mutant-*.log, restored-*.log, clippy-verified.log, paired syntax JSON and generator log. Validation log paths are observations from this finite run, not universal certificates. No timing or allocator-exhaustion injection performed. Partial syntax is generated twice, compared exactly, then installed and SHA-bound.
