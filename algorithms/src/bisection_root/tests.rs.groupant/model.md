# algorithm-bisection-root-tests

## descriptant

### bisection_root tests.rs

Original standard-library numerical implementation and finite tests. Prospective predictions were saved before code in ../mod.rs.groupant/model.md#prospective.

Keep [l,r] with opposite endpoint signs; evaluate m=l+(r-l)/2 and replace the same-sign endpoint. Exact computed zero collapses the bracket. Width stopping retains the last evaluated endpoint, whose distance to any retained exact-real root is at most the width under exact signs and continuity.

sqrt(k), k=1..39, independently checks coordinates and brackets; endpoint zero, no bracket, nonfinite callbacks, budget count and adjacent-float stagnation.

Literature: Burden and Faires, Numerical Analysis (bisection); Kiefer (1953), Sequential minimax search for a maximum (golden section); Nocedal and Wright, Numerical Optimization, second edition, chapter 3 (gradient descent); Hestenes and Stiefel (1952), Methods of conjugate gradients for solving linear systems (CG). No source code was copied.

## model

### Mathematical combiner

Inputs: authoritative tests.rs, partial syntax projection, forward model, typed complement and reconstruction relations, and named finite observations. The combiner intersects accepted-domain constraints with update invariants, then partitions outputs by convergence, budget, stagnation or error. Cost is O(k) callback invocations (at most k+2), O(1) internal storage; callback cost and allocation excluded. Tests are sibling sources, never implementation children. Source hashes bind this descriptive epoch; syntax is not semantic AST or proof.

Observation retained: sqrt(k), k=1..39, independently checks coordinates and brackets; endpoint zero, no bracket, nonfinite callbacks, budget count and adjacent-float stagnation.

Limits: callback determinism/continuity/unimodality and global SPD are assumptions; residual stopping alone says nothing universal about solution error. Conditioning, underflow, finite rounding and callback cost remain material. No speed measurements or universal accuracy certificate are claimed.

## approximant

### Forward approximant and invariant

Domain: Continuous deterministic callback, opposite finite endpoint signs; endpoints bounded by 1e100; positive finite absolute width tolerance and iteration budget <=100000.

Keep [l,r] with opposite endpoint signs; evaluate m=l+(r-l)/2 and replace the same-sign endpoint. Exact computed zero collapses the bracket. Width stopping retains the last evaluated endpoint, whose distance to any retained exact-real root is at most the width under exact signs and continuity.

Cost: O(k) callback invocations (at most k+2), O(1) internal storage; callback cost and allocation excluded.

Test source maps fixed fixtures to assertion success/failure. sqrt(k), k=1..39, independently checks coordinates and brackets; endpoint zero, no bracket, nonfinite callbacks, budget count and adjacent-float stagnation.

Numerical claims concern additive interval widths or absolute gradient/residual norms. A relative coordinate error has undefined behavior at zero without a separate convention and is not claimed. No supported 1/2/5/10% executable surrogate is attached: this exact API is already iterative, and no independently measured faster transformation with a useful certified metric is supplied.

## models

# Typed obernant family

## behavior_complement

Let U be all possible fixture outcomes and R the conjunction of the explicit assertions. U\R includes a wrong coordinate even if status looks plausible, false convergence, accepted invalid fixtures or exceeded callback count. Passing tests establishes membership only for these finite inputs, not all inputs.

## reverse_reconstruction

A passing assertion report maps back to all implementations agreeing on the finite fixtures. This is a large nonunique fiber: unsampled callbacks, matrix spectra and resource failures remain free. Domain: Continuous deterministic callback, opposite finite endpoint signs; endpoints bounded by 1e100; positive finite absolute width tolerance and iteration budget <=100000.

## independent_oracle

sqrt(k), k=1..39, independently checks coordinates and brackets; endpoint zero, no bracket, nonfinite callbacks, budget count and adjacent-float stagnation.

## mutation_sensitivity

A compiled production mutation must fail the named assertion `root bracket oracle`; restoring exact source bytes must recover a passing focused suite. This shows local detection power only.

## boundary_partition

Partition observations by valid convergence, zero/exhausted budget, representational stagnation, and invalid inputs/nonfinite values. Allocation failure is modeled but not injected. Strict continuity/unimodality/SPD are not established by finite tests.


## tests

### Checks

Run `cargo test -p mova-algorithms bisection_root::tests` and its release equivalent. sqrt(k), k=1..39, independently checks coordinates and brackets; endpoint zero, no bracket, nonfinite callbacks, budget count and adjacent-float stagnation.

Compiled negative control must hit `root bracket oracle`. Full algorithms suite runs twice per profile; normalized ordered test outcomes are compared. Strict all-target Clippy and focused rustfmt check complete validation. See model.md#validation for observed evidence.

## validation

### Validation evidence

Partial syntax projection generated independently twice and compared byte-for-byte. These are finite checks; allocation exhaustion and arbitrary caller functions/matrices are not exhaustively verified.

## History

### 2026-10-30 · descriptant

2026-10-30 epoch: scoped safe bounds and explicit failure statuses were retained; finite observations do not establish universal convergence.

### 2026-10-30 · validation

2026-10-30. External records under `$MOVA_DATA/runs/octopus-wave19-solvers-2026-10-30/`. Two debug and two release runs each passed 501 library tests (12 new focused tests); sorted named outcomes agree across four runs. Strict algorithms all-target Clippy and focused rustfmt passed.

Source SHA-256: `ccb0af8ae7a9ab6a579b4e88ec6abf72b51cf47f946a10d4fdae7a97aef0ddfd`. Negative production mutant failed `root bracket oracle`; restored bytes passed the focused suite.

- `negative-bisection_root.log` SHA-256 `f58f8e999cebbb85fb2a182ae2550e7cb6f1ffa0ea820dec898cacac123fa569`

- `restored-bisection_root.log` SHA-256 `a430b193dcffec2efdabab7169b108eb8807ea59c4ef23f57289cb88e2c66523`

- `debug1.log` SHA-256 `61428a2e0c6dbc43cde2fa1cf9a4b021a6cebd75c04de1ecb3cf761fa95eccbf`

- `debug2.log` SHA-256 `73ffe6afebfbf497bd448639d77d254de8142e4c170e346e976eff376e95e496`

- `release1.log` SHA-256 `9a28e6d7f1af6cfbf056323b4ec824aca1899d9ed52b1635d30ed01ed8785afb`

- `release2.log` SHA-256 `17cdc616d33e1ca8add5062ebed8e5303eebab267338c981dbc2aecba57afd43`

- `clippy.log` SHA-256 `71ef179d48cc2d1e07e85d2e77009ed79e6e543be1e409886afdb92b50363422`
