# twin-update

## catalog

# Prospective RLS state transition

Every relation below is scoped to the named observation, holding other input/state coordinates fixed unless quantified. U is its stated input/output (or finite trace) Cartesian universe, R the described reachable relation. Complement targets U \ R; converse retains every compatible antecedent, never an invented unique cause. Source-derived formulas are partial hypotheses about Rust; floating-point expressions use evaluation order where stated, ideal real identities are conditional. Checks marked proposed were not executed.

## forward

observe predicts before correcting, constructs candidate, checks it and commits only on success.

## invariant

Validation requires finite vectors/SSE>=0 and approximate symmetry plus successful finite positive Cholesky pivots. It does not certify an exact symmetric matrix, box ordering, n/SSE consistency or observational reachability. Constructor does not call validate: subnormal positive ridge may yield infinite P. Predict may return negative seconds. n+1 overflow is not guarded: debug panic or release wrapping is outside the n<u64::MAX model.

## cost

At dimension d<=32, validation uses O(d^3) arithmetic and O(d^2) allocation. Predict adds O(d^2); observe calls predict and another validate, hence O(d^3), not merely the O(d^2) rank-one kernel. State memory O(d^2). No wall-clock estimate is calibrated.

## composition

Leaf fragment; prediction and validation are external sibling relations sharing the same pre-state. No assertion that independently valid projected children prove the entire update.

## epoch-memory

Initial source review. Default Rust source remains unchanged. These models record mechanisms and gaps rather than claiming statistical calibration, semantic proof or a complete AST. CLI evidence is trusted local data, not authenticated provenance. Preserve these limits when refreshing bindings.

## checks

Existing `cargo test -p mova-twin --locked`: three library tests compare online updates with independent 2D ridge, reject invalid information and preserve prospective residuals; three ledger integration tests exercise forecast-before-execution, parent replay/tamper rejection, rejected runs, timeout, input mutation and predict without launch. Integration commands are bounded local fixtures, not live benchmarks or teacher calls. See ../model.md#checks at the file root for execution evidence. Other oracles below are proposals. Structural validation checks hashes, references and bounds, not equations.

## behavior-complement

U is (M,x,y,Result,Mafter) with n<u64::MAX. Source-derived U\R excludes successful updates for nonpositive/nonfinite y and Err returns with Mafter different from M. Successful n increment without changes to any theta entry is possible for x=0 or zero residual, so excluded behaviors must not assume every sample changes coefficients. Proposed oracle uses zero x and finite positive y.

## reverse-reconstruction

For a successor, candidate antecedents satisfy the full rank-one equations, n'=n+1, SSE'=SSE+e² and coordinate min/max expansion. First observation n=0 discards previous box values; later updates can hide prior extrema. Retain all prior boxes satisfying these equations and validate, rather than deriving one. Real parameter inverse in the file catalog is conditional and does not uniquely invert f64 histories.

## residual-shrinkage

With symmetric SPD P and exact arithmetic at the same x, y-x^Ttheta'=(y-x^Ttheta)/(1+x^TPx). Since h>=0, magnitude does not increase; this does not imply other-feature errors improve or universal finite-precision monotonicity. Existing 2D ridge test checks this identity at tolerance1e-12 for four observations.

## rollback-boundary

next is a deep clone; theta/P/box/count/SSE writes precede next.validate. Only *self=next commits. Overflow of SSE can force Err without mutation; counter overflow may panic before Result in debug or wrap in release. Proposed oracle uses finite huge y, verifies Err and original state equality; counter-max behavior remains a documented gap.

## History

### 2026-10-29 · document

Subject: `tools/twin/src/lib.rs`, exact SHA-256 in manifest; modeling epoch 2026-10-29.

### 2026-10-29 · epoch-memory

Initial source review, 2026-10-29. Default Rust source remains unchanged. These models record mechanisms and gaps rather than claiming statistical calibration, semantic proof or a complete AST. CLI evidence is trusted local data, not authenticated provenance. Preserve these limits when refreshing bindings.
