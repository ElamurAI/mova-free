# algorithm-gradient-descent-quadratic-tests

## descriptant

### gradient_descent_quadratic tests.rs

Original standard-library numerical implementation and finite tests. Prospective predictions were saved before code in ../mod.rs.groupant/model.md#prospective.

For q(x)=half x'Dx-b'x, use g=Dx-b and x_next=x-g/L with L=max D. In exact arithmetic coordinate errors contract by 1-D_i/L in [0,1). Fresh Euclidean gradient norm is the stopping observation, not coordinate error.

19 diagonal spectra compared to analytic coordinate divisions b_i/D_i; fresh residual check, zero budget, already stationary input, ill-conditioned representational stagnation, shapes and invalid coefficients.

Literature: Burden and Faires, Numerical Analysis (bisection); Kiefer (1953), Sequential minimax search for a maximum (golden section); Nocedal and Wright, Numerical Optimization, second edition, chapter 3 (gradient descent); Hestenes and Stiefel (1952), Methods of conjugate gradients for solving linear systems (CG). No source code was copied.

## model

### Mathematical combiner

Inputs: authoritative tests.rs, partial syntax projection, forward model, typed complement and reconstruction relations, and named finite observations. The combiner intersects accepted-domain constraints with update invariants, then partitions outputs by convergence, budget, stagnation or error. Cost is O(n*k) arithmetic, O(n) auxiliary memory through checked vector reservations; allocation failures are explicit. Tests are sibling sources, never implementation children. Source hashes bind this descriptive epoch; syntax is not semantic AST or proof.

Observation retained: 19 diagonal spectra compared to analytic coordinate divisions b_i/D_i; fresh residual check, zero budget, already stationary input, ill-conditioned representational stagnation, shapes and invalid coefficients.

Limits: callback determinism/continuity/unimodality and global SPD are assumptions; residual stopping alone says nothing universal about solution error. Conditioning, underflow, finite rounding and callback cost remain material. No speed measurements or universal accuracy certificate are claimed.

## approximant

### Forward approximant and invariant

Domain: Dimension 1..256, diagonal in [1e-100,1e100], finite linear and initial coordinates bounded by 1e100; positive finite gradient norm tolerance; budget <=100000.

For q(x)=half x'Dx-b'x, use g=Dx-b and x_next=x-g/L with L=max D. In exact arithmetic coordinate errors contract by 1-D_i/L in [0,1). Fresh Euclidean gradient norm is the stopping observation, not coordinate error.

Cost: O(n*k) arithmetic, O(n) auxiliary memory through checked vector reservations; allocation failures are explicit.

Test source maps fixed fixtures to assertion success/failure. 19 diagonal spectra compared to analytic coordinate divisions b_i/D_i; fresh residual check, zero budget, already stationary input, ill-conditioned representational stagnation, shapes and invalid coefficients.

Numerical claims concern additive interval widths or absolute gradient/residual norms. A relative coordinate error has undefined behavior at zero without a separate convention and is not claimed. No supported 1/2/5/10% executable surrogate is attached: this exact API is already iterative, and no independently measured faster transformation with a useful certified metric is supplied.

## models

# Typed obernant family

## behavior_complement

Let U be all possible fixture outcomes and R the conjunction of the explicit assertions. U\R includes a wrong coordinate even if status looks plausible, false convergence, accepted invalid fixtures or exceeded callback count. Passing tests establishes membership only for these finite inputs, not all inputs.

## reverse_reconstruction

A passing assertion report maps back to all implementations agreeing on the finite fixtures. This is a large nonunique fiber: unsampled callbacks, matrix spectra and resource failures remain free. Domain: Dimension 1..256, diagonal in [1e-100,1e100], finite linear and initial coordinates bounded by 1e100; positive finite gradient norm tolerance; budget <=100000.

## independent_oracle

19 diagonal spectra compared to analytic coordinate divisions b_i/D_i; fresh residual check, zero budget, already stationary input, ill-conditioned representational stagnation, shapes and invalid coefficients.

## mutation_sensitivity

A compiled production mutation must fail the named assertion `diagonal coordinate oracle`; restoring exact source bytes must recover a passing focused suite. This shows local detection power only.

## boundary_partition

Partition observations by valid convergence, zero/exhausted budget, representational stagnation, and invalid inputs/nonfinite values. Allocation failure is modeled but not injected. Strict continuity/unimodality/SPD are not established by finite tests.


## tests

### Checks

Run `cargo test -p mova-algorithms gradient_descent_quadratic::tests` and its release equivalent. 19 diagonal spectra compared to analytic coordinate divisions b_i/D_i; fresh residual check, zero budget, already stationary input, ill-conditioned representational stagnation, shapes and invalid coefficients.

Compiled negative control must hit `diagonal coordinate oracle`. Full algorithms suite runs twice per profile; normalized ordered test outcomes are compared. Strict all-target Clippy and focused rustfmt check complete validation. See model.md#validation for observed evidence.

## validation

### Validation evidence

Partial syntax projection generated independently twice and compared byte-for-byte. These are finite checks; allocation exhaustion and arbitrary caller functions/matrices are not exhaustively verified.

## History

### 2026-10-30 · descriptant

2026-10-30 epoch: scoped safe bounds and explicit failure statuses were retained; finite observations do not establish universal convergence.

### 2026-10-30 · validation

2026-10-30. External records under `$MOVA_DATA/runs/octopus-wave19-solvers-2026-10-30/`. Two debug and two release runs each passed 501 library tests (12 new focused tests); sorted named outcomes agree across four runs. Strict algorithms all-target Clippy and focused rustfmt passed.

Source SHA-256: `fb1292d57709475d0d75146a7db40bd27c2c1ec44d8ffe2ca3705c7b45391e22`. Negative production mutant failed `diagonal coordinate oracle`; restored bytes passed the focused suite.

- `negative-gradient_descent_quadratic.log` SHA-256 `155d6eb36448f4ab36eb6b7d2924c5083e14e4d858d3277e976f9f065b35bc73`

- `restored-gradient_descent_quadratic.log` SHA-256 `de6de777c098e634fee1a7039f978a4798a261239f2fa86882312fdf8abf349c`

- `debug1.log` SHA-256 `61428a2e0c6dbc43cde2fa1cf9a4b021a6cebd75c04de1ecb3cf761fa95eccbf`

- `debug2.log` SHA-256 `73ffe6afebfbf497bd448639d77d254de8142e4c170e346e976eff376e95e496`

- `release1.log` SHA-256 `9a28e6d7f1af6cfbf056323b4ec824aca1899d9ed52b1635d30ed01ed8785afb`

- `release2.log` SHA-256 `17cdc616d33e1ca8add5062ebed8e5303eebab267338c981dbc2aecba57afd43`

- `clippy.log` SHA-256 `71ef179d48cc2d1e07e85d2e77009ed79e6e543be1e409886afdb92b50363422`
