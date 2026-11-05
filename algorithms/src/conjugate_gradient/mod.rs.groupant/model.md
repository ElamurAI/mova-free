# algorithm-conjugate-gradient

## descriptant

### conjugate_gradient mod.rs

Original standard-library numerical implementation and finite tests. Prospective predictions were saved before code in model.md#prospective.

r=b-Ax, p initially r; alpha=(r'r)/(p'Ap), x_next=x+alpha*p; recompute fresh r and beta=(r_new'r_new)/(r_old'r_old), p_next=r_new+beta*p. Reject encountered nonpositive curvature. Exact arithmetic gives classical CG; recomputation and rounding lose exact conjugacy.

12 dense SPD matrices diag(1..n)+11' with independently constructed known solutions, directly recomputed residual, zero budget, zero residual, squared-norm underflow stagnation, asymmetry, negative curvature and arithmetic overflow.

Literature: Burden and Faires, Numerical Analysis (bisection); Kiefer (1953), Sequential minimax search for a maximum (golden section); Nocedal and Wright, Numerical Optimization, second edition, chapter 3 (gradient descent); Hestenes and Stiefel (1952), Methods of conjugate gradients for solving linear systems (CG). No source code was copied.

## model

### Mathematical combiner

Inputs: authoritative mod.rs, partial syntax projection, forward model, typed complement and reconstruction relations, and named finite observations. The combiner intersects accepted-domain constraints with update invariants, then partitions outputs by convergence, budget, stagnation or error. Cost is O(n²*k) arithmetic, O(n) auxiliary storage beyond caller matrix, checked vector reservations; symmetry precheck O(n²). Tests are sibling sources, never implementation children. Source hashes bind this descriptive epoch; syntax is not semantic AST or proof.

Observation retained: 12 dense SPD matrices diag(1..n)+11' with independently constructed known solutions, directly recomputed residual, zero budget, zero residual, squared-norm underflow stagnation, asymmetry, negative curvature and arithmetic overflow.

Limits: callback determinism/continuity/unimodality and global SPD are assumptions; residual stopping alone says nothing universal about solution error. Conditioning, underflow, finite rounding and callback cost remain material. No speed measurements or universal accuracy certificate are claimed.

## approximant

### Forward approximant and invariant

Domain: Dimension 1..256, square exactly symmetric finite matrix, RHS and initial coordinates bounded by 1e100; caller assumes SPD; positive finite residual tolerance and budget <=100000.

r=b-Ax, p initially r; alpha=(r'r)/(p'Ap), x_next=x+alpha*p; recompute fresh r and beta=(r_new'r_new)/(r_old'r_old), p_next=r_new+beta*p. Reject encountered nonpositive curvature. Exact arithmetic gives classical CG; recomputation and rounding lose exact conjugacy.

Cost: O(n²*k) arithmetic, O(n) auxiliary storage beyond caller matrix, checked vector reservations; symmetry precheck O(n²).

Invariant: accepted reports contain finite values; successful scalar intervals stay nested and vector residuals correspond to returned coordinates. Global mathematical assumptions remain caller obligations.

Numerical claims concern additive interval widths or absolute gradient/residual norms. A relative coordinate error has undefined behavior at zero without a separate convention and is not claimed. No supported 1/2/5/10% executable surrogate is attached: this exact API is already iterative, and no independently measured faster transformation with a useful certified metric is supplied.

## models

# Typed obernant family

## behavior_complement

On the scoped input/report universe, the complement excludes fabricated convergence when neither declared stopping observation holds, invalid accepted shapes or nonfinite arithmetic results, and reports whose estimate leaves the retained bracket or whose vector residual is inconsistent. Domain: Dimension 1..256, square exactly symmetric finite matrix, RHS and initial coordinates bounded by 1e100; caller assumes SPD; positive finite residual tolerance and budget <=100000. This relation is broader than enumerated rejection tests. It does not claim the solver detects violated global callback/SPD assumptions.

## reverse_reconstruction

For residual zero, any SPD A paired with b=Ax gives a compatible antecedent; infinitely many A share x. For nonzero residual <=tau, b belongs to the Euclidean ball centered at Ax. Initial state and iteration count restrict this fiber but do not generally make it unique.

## curvature_guard

SPD implies positive p'Ap for nonzero p in exact arithmetic. Positive visited curvature is necessary but insufficient to certify global SPD. Squared residual underflow returns stagnation; other nonfinite arithmetic returns an error.

## termination_partition

The bounded loop checks convergence first, then budget, then representational stagnation. Zero budget can still converge on the initial stopping observation. Iteration count is completed updates (including an attempted unchanged vector update). No unbounded search or implicit retry.

## finite_arithmetic

Inputs reject NaN/infinity and restricted magnitude violations. Scalar callback results must remain finite; bounded endpoints keep differences and convex combinations finite. Vector norms use hypot; inner products and updates check finite arithmetic. Finite precision is not an exact-real proof; CG squared norm underflow is stagnation.

## resource_observation

O(n²*k) arithmetic, O(n) auxiliary storage beyond caller matrix, checked vector reservations; symmetry precheck O(n²).


## tests

### Checks

Run `cargo test -p mova-algorithms conjugate_gradient::tests` and its release equivalent. 12 dense SPD matrices diag(1..n)+11' with independently constructed known solutions, directly recomputed residual, zero budget, zero residual, squared-norm underflow stagnation, asymmetry, negative curvature and arithmetic overflow.

Compiled negative control must hit `dense coordinate oracle`. Full algorithms suite runs twice per profile; normalized ordered test outcomes are compared. Strict all-target Clippy and focused rustfmt check complete validation. See model.md#validation for observed evidence.

## prospective

### Prospective numerical solver model

Scalar routines accept finite endpoints with magnitude at most 1e100 and positive finite absolute x tolerance, up to 100000 iterations. Bisection requires a continuous deterministic caller function and opposite endpoint signs; golden-section requires a deterministic strictly unimodal function (possibly with a unique boundary minimum). Function evaluations must be finite. Neither routine can validate those global caller assumptions. Nested intervals are the invariant. Width stopping is an additive coordinate observation, not relative error. Golden-section ties deterministically discard the right portion; bisection exact computed zero reports convergence but does not certify the exact-real function.

Vector routines accept 1..256 coordinates bounded by 1e100, positive finite tolerance, and up to 100000 iterations. Diagonal gradient descent requires coefficients in [1e-100,1e100], uses step 1/max diagonal, and minimizes half x'Dx - b'x. Dense conjugate gradients require exact symmetry and caller-assumed positive definiteness, with runtime positive-curvature checks. Every arithmetic vector and scalar is checked for finiteness; fresh residuals determine convergence. No residual-to-solution-error assertion without conditioning. Budget exhaustion and representational stagnation are distinct statuses. Checked reservations report allocation failure. Cost: scalar O(iterations) evaluations/O(1) storage; diagonal O(n*iterations)/O(n); dense O(n²*iterations)/O(n) auxiliary storage. Rounding, underflow and ill-conditioning can cause stagnation; no universal accuracy or speed claim. No useful separately measured executable surrogate is claimed.

## validation

### Validation evidence

Partial syntax projection generated independently twice and compared byte-for-byte. These are finite checks; allocation exhaustion and arbitrary caller functions/matrices are not exhaustively verified.

## History

### 2026-10-30 · descriptant

2026-10-30 epoch: scoped safe bounds and explicit failure statuses were retained; finite observations do not establish universal convergence.

### 2026-10-30 · prospective

2026-10-30, before implementation. Original standard-library implementation, following the classical interval and quadratic optimization recurrences (Burden and Faires, Numerical Analysis; Nocedal and Wright, Numerical Optimization, second edition, chapters 3 and 5).

### 2026-10-30 · validation

2026-10-30. External records under `$MOVA_DATA/runs/octopus-wave19-solvers-2026-10-30/`. Two debug and two release runs each passed 501 library tests (12 new focused tests); sorted named outcomes agree across four runs. Strict algorithms all-target Clippy and focused rustfmt passed.

Source SHA-256: `db3811f6fab44ae073eb03834a83530439095894ee1d839a3fb88f30c8d61fdb`. Negative production mutant failed `dense coordinate oracle`; restored bytes passed the focused suite.

- `negative-conjugate_gradient.log` SHA-256 `23b4b2c34030bee1b468aa16497e2f539bbb1d047f125d5641291cdc4ea333ca`

- `restored-conjugate_gradient.log` SHA-256 `eadca419529e42fb1088f6bb5ee6760ae151cb4f8b51e7a9fa85ab6d769ff617`

- `debug1.log` SHA-256 `61428a2e0c6dbc43cde2fa1cf9a4b021a6cebd75c04de1ecb3cf761fa95eccbf`

- `debug2.log` SHA-256 `73ffe6afebfbf497bd448639d77d254de8142e4c170e346e976eff376e95e496`

- `release1.log` SHA-256 `9a28e6d7f1af6cfbf056323b4ec824aca1899d9ed52b1635d30ed01ed8785afb`

- `release2.log` SHA-256 `17cdc616d33e1ca8add5062ebed8e5303eebab267338c981dbc2aecba57afd43`

- `clippy.log` SHA-256 `71ef179d48cc2d1e07e85d2e77009ed79e6e543be1e409886afdb92b50363422`
