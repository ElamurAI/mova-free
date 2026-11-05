# algorithm-conjugate-gradient-tests

## descriptant

### conjugate_gradient tests.rs

Original standard-library numerical implementation and finite tests. Prospective predictions were saved before code in ../mod.rs.groupant/model.md#prospective.

r=b-Ax, p initially r; alpha=(r'r)/(p'Ap), x_next=x+alpha*p; recompute fresh r and beta=(r_new'r_new)/(r_old'r_old), p_next=r_new+beta*p. Reject encountered nonpositive curvature. Exact arithmetic gives classical CG; recomputation and rounding lose exact conjugacy.

12 dense SPD matrices diag(1..n)+11' with independently constructed known solutions, directly recomputed residual, zero budget, zero residual, squared-norm underflow stagnation, asymmetry, negative curvature and arithmetic overflow.

Literature: Burden and Faires, Numerical Analysis (bisection); Kiefer (1953), Sequential minimax search for a maximum (golden section); Nocedal and Wright, Numerical Optimization, second edition, chapter 3 (gradient descent); Hestenes and Stiefel (1952), Methods of conjugate gradients for solving linear systems (CG). No source code was copied.

## model

### Mathematical combiner

Inputs: authoritative tests.rs, partial syntax projection, forward model, typed complement and reconstruction relations, and named finite observations. The combiner intersects accepted-domain constraints with update invariants, then partitions outputs by convergence, budget, stagnation or error. Cost is O(n²*k) arithmetic, O(n) auxiliary storage beyond caller matrix, checked vector reservations; symmetry precheck O(n²). Tests are sibling sources, never implementation children. Source hashes bind this descriptive epoch; syntax is not semantic AST or proof.

Observation retained: 12 dense SPD matrices diag(1..n)+11' with independently constructed known solutions, directly recomputed residual, zero budget, zero residual, squared-norm underflow stagnation, asymmetry, negative curvature and arithmetic overflow.

Limits: callback determinism/continuity/unimodality and global SPD are assumptions; residual stopping alone says nothing universal about solution error. Conditioning, underflow, finite rounding and callback cost remain material. No speed measurements or universal accuracy certificate are claimed.

## approximant

### Forward approximant and invariant

Domain: Dimension 1..256, square exactly symmetric finite matrix, RHS and initial coordinates bounded by 1e100; caller assumes SPD; positive finite residual tolerance and budget <=100000.

r=b-Ax, p initially r; alpha=(r'r)/(p'Ap), x_next=x+alpha*p; recompute fresh r and beta=(r_new'r_new)/(r_old'r_old), p_next=r_new+beta*p. Reject encountered nonpositive curvature. Exact arithmetic gives classical CG; recomputation and rounding lose exact conjugacy.

Cost: O(n²*k) arithmetic, O(n) auxiliary storage beyond caller matrix, checked vector reservations; symmetry precheck O(n²).

Test source maps fixed fixtures to assertion success/failure. 12 dense SPD matrices diag(1..n)+11' with independently constructed known solutions, directly recomputed residual, zero budget, zero residual, squared-norm underflow stagnation, asymmetry, negative curvature and arithmetic overflow.

Numerical claims concern additive interval widths or absolute gradient/residual norms. A relative coordinate error has undefined behavior at zero without a separate convention and is not claimed. No supported 1/2/5/10% executable surrogate is attached: this exact API is already iterative, and no independently measured faster transformation with a useful certified metric is supplied.

## models

# Typed obernant family

## behavior_complement

Let U be all possible fixture outcomes and R the conjunction of the explicit assertions. U\R includes a wrong coordinate even if status looks plausible, false convergence, accepted invalid fixtures or exceeded callback count. Passing tests establishes membership only for these finite inputs, not all inputs.

## reverse_reconstruction

A passing assertion report maps back to all implementations agreeing on the finite fixtures. This is a large nonunique fiber: unsampled callbacks, matrix spectra and resource failures remain free. Domain: Dimension 1..256, square exactly symmetric finite matrix, RHS and initial coordinates bounded by 1e100; caller assumes SPD; positive finite residual tolerance and budget <=100000.

## independent_oracle

12 dense SPD matrices diag(1..n)+11' with independently constructed known solutions, directly recomputed residual, zero budget, zero residual, squared-norm underflow stagnation, asymmetry, negative curvature and arithmetic overflow.

## mutation_sensitivity

A compiled production mutation must fail the named assertion `dense coordinate oracle`; restoring exact source bytes must recover a passing focused suite. This shows local detection power only.

## boundary_partition

Partition observations by valid convergence, zero/exhausted budget, representational stagnation, and invalid inputs/nonfinite values. Allocation failure is modeled but not injected. Strict continuity/unimodality/SPD are not established by finite tests.


## tests

### Checks

Run `cargo test -p mova-algorithms conjugate_gradient::tests` and its release equivalent. 12 dense SPD matrices diag(1..n)+11' with independently constructed known solutions, directly recomputed residual, zero budget, zero residual, squared-norm underflow stagnation, asymmetry, negative curvature and arithmetic overflow.

Compiled negative control must hit `dense coordinate oracle`. Full algorithms suite runs twice per profile; normalized ordered test outcomes are compared. Strict all-target Clippy and focused rustfmt check complete validation. See model.md#validation for observed evidence.

## validation

### Validation evidence

Partial syntax projection generated independently twice and compared byte-for-byte. These are finite checks; allocation exhaustion and arbitrary caller functions/matrices are not exhaustively verified.

## History

### 2026-10-30 · descriptant

2026-10-30 epoch: scoped safe bounds and explicit failure statuses were retained; finite observations do not establish universal convergence.

### 2026-10-30 · validation

2026-10-30. External records under `$MOVA_DATA/runs/octopus-wave19-solvers-2026-10-30/`. Two debug and two release runs each passed 501 library tests (12 new focused tests); sorted named outcomes agree across four runs. Strict algorithms all-target Clippy and focused rustfmt passed.

Source SHA-256: `f5461c2081df0fa44dd47ff3e840f71e9dc0705f14b52baa3c1e7e1833ae4c5e`. Negative production mutant failed `dense coordinate oracle`; restored bytes passed the focused suite.

- `negative-conjugate_gradient.log` SHA-256 `23b4b2c34030bee1b468aa16497e2f539bbb1d047f125d5641291cdc4ea333ca`

- `restored-conjugate_gradient.log` SHA-256 `eadca419529e42fb1088f6bb5ee6760ae151cb4f8b51e7a9fa85ab6d769ff617`

- `debug1.log` SHA-256 `61428a2e0c6dbc43cde2fa1cf9a4b021a6cebd75c04de1ecb3cf761fa95eccbf`

- `debug2.log` SHA-256 `73ffe6afebfbf497bd448639d77d254de8142e4c170e346e976eff376e95e496`

- `release1.log` SHA-256 `9a28e6d7f1af6cfbf056323b4ec824aca1899d9ed52b1635d30ed01ed8785afb`

- `release2.log` SHA-256 `17cdc616d33e1ca8add5062ebed8e5303eebab267338c981dbc2aecba57afd43`

- `clippy.log` SHA-256 `71ef179d48cc2d1e07e85d2e77009ed79e6e543be1e409886afdb92b50363422`
