# algorithm-lanczos

## descriptant

### Symmetric Lanczos projection with twice full reorthogonalization

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

Domain: Square 1..64, exact symmetry, finite matrix/start magnitude <=1e100 and nonzero start. Absolute beta threshold [0,1e100]; max_steps<=4096 capped by n.

Start q normalized with canonical sign. At j: alpha=q^T Aq; w=Aq-alpha*q-beta_prev*q_prev. Append q/alpha, perform two MGS passes against all current basis columns, beta=||w||2. Complete at n first; else beta<=tol gives Breakdown; else budget returns; otherwise append nonnegative beta and q=w/beta. Report true ||AQ-QT||F and ||Q^TQ-I||F.

Cost: O(k*n^2+n*k^2) including reorthogonalization and fresh observations; O(n*k) storage, checked outer basis and scalar buffers, one checked matvec per step and residual column.

Validation precedence: shape/value bounds first, then control parameters, symmetry where applicable, initial vector where applicable, checked allocations, then arithmetic/encountered-pivot failures in recurrence order. No callbacks, random state, caller-input mutation or persistent effects. Checked reservations map errors to AllocationFailed; Rust panic/OOM abort outside reserved bounded production allocations are not modeled. Finite helper checks reject nonfinite intermediates; rounded underflow remains permitted. Ties and signs follow model.md#models.

No executable 1/2/5/10% surrogate is attached. Residual and orthogonality norms are finite observations, not generic relative solution-error certificates. A meaningful percentage profile needs a separate domain, metric, zero-reference rule and analytical or finite evidence. No benchmark, production speedup or end-to-end Mova quality gain is claimed.

## models

# Typed obernant family

## behavior_complement

Outputs with k-1 offdiagonals violated, negative betas, nonsymmetric T, or more than min(max_steps,n) columns are excluded. Budget does not imply convergence. Breakdown only says encountered beta<=threshold; it need not mean zero projection residual.

## reverse_reconstruction

Exact (Q,T) constrains the projected action. With full orthogonal Q and zero residual, A=QTQ^T; with k<n, A on the orthogonal complement is free and terminal coupling is unrecorded except its norm. Multiple starts, scalings, and ambient matrices share a projection. Zero budget has an empty projection and loses all matrix information.

## full_reorthogonalization

Two ordered passes subtract dot(qi,w)*qi against all basis vectors. In exact Lanczos arithmetic these corrections vanish; finite corrections are not inserted into T and their effect is retained in fresh AQ-QT residual.

## nonnegative_beta_sign

Only the initial vector is independently canonicalized. Later vectors inherit w/beta with nonnegative beta so recurrence signs remain consistent. Arbitrary later sign flips would invalidate offdiagonals.

## terminal_status_order

At each completed step: full dimension Complete first, beta<=tol Breakdown second, exhausted budget third. Zero budget validates input and start but returns empty Q/T, zero observations and Budget.

## terminal_residual_observation

Fresh AQ-QT includes terminal coupling and finite recurrence/reorthogonalization errors. It is not just beta or an eigenvalue-error estimator. Complete means dimension reached, never a promised small residual.

## tests

### Checks

2x2 diagonal analytic alpha=2,beta=1; exact coordinate-start tridiagonal recovery; invariant-subspace breakdown; zero/short budgets; scalar completion and threshold priority; independent 3x3 Q^T A Q and orthogonality checks.

Run cargo test -p mova-algorithms lanczos::tests. Repeat debug and release outcomes. A compiled production mutant exercises a named assertion; exact source restoration must pass. Strict all-target Clippy and focused rustfmt complete static checks.

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

Compiled mutant: alpha incremented by one; diagonal_full_span_oracle must fail. Source restored byte-for-byte and named positive rerun passes. First exploratory LU mutation omitted the U row swap and failed by unwrap of an arithmetic error; it was not accepted as assertion evidence. The accepted LU mutation omits the permutation swap and fails assert_eq.

Integration boundary: checking the full augmented temporary registry reports 21 diagnostics, all confined to parent algorithm-library/mova child lists and transitive source bindings that the integration owner will refresh. No new leaf diagnostics occur in that report. The full hierarchy is not yet declared valid. A single-node registry cannot validate this whole checkout because the checker discovers all sidecars.

## History

### 2026-10-30 · descriptant

Epoch 2026-10-30: 31 new named tests; complete library 724 tests passes twice debug and twice release with identical nonempty normalized outcomes. Five production mutants fail named assertions after compilation; restoration reruns pass. Strict algorithm all-target Clippy passes. No timing was performed.

### 2026-10-30 · validation

Heading `### Validation epoch 2026-10-30` stays in validation as `### Validation`.

Receipts: $MOVA_DATA/runs/octopus-wave26-linear-2026-10-30/ contains prospective.md, debug-1/2.log, release-1/2.log, normalized records, controls.log, mutant-*.log, restored-*.log, clippy-verified.log, paired syntax JSON and generator log. Validation log paths are observations from this finite run, not universal certificates. No timing or allocator-exhaustion injection performed. Partial syntax is generated twice, compared exactly, then installed and SHA-bound.
