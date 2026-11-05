# twin-forecast

## catalog

# Prediction and observed-support projection

Every relation below is scoped to the named observation, holding other input/state coordinates fixed unless quantified. U is its stated input/output (or finite trace) Cartesian universe, R the described reachable relation. Complement targets U \ R; converse retains every compatible antecedent, never an invented unique cause. Source-derived formulas are partial hypotheses about Rust; floating-point expressions use evaluation order where stated, ideal real identities are conditional. Checks marked proposed were not executed.

## forward

validate M, require finite x of matching length; compute px/h/seconds, reject unstable projections and return n/RMSE/box summary.

## invariant

Validation requires finite vectors/SSE>=0 and approximate symmetry plus successful finite positive Cholesky pivots. It does not certify an exact symmetric matrix, box ordering, n/SSE consistency or observational reachability. Constructor does not call validate: subnormal positive ridge may yield infinite P. Predict may return negative seconds. n+1 overflow is not guarded: debug panic or release wrapping is outside the n<u64::MAX model.

## cost

At dimension d<=32, validation uses O(d^3) arithmetic and O(d^2) allocation. Predict adds O(d^2); observe calls predict and another validate, hence O(d^3), not merely the O(d^2) rank-one kernel. State memory O(d^2). No wall-clock estimate is calibrated.

## composition

Leaf fragment. Its validation dependency lies in the sibling initial-state fragment; cost includes that call.

## epoch-memory

Initial source review. Default Rust source remains unchanged. These models record mechanisms and gaps rather than claiming statistical calibration, semantic proof or a complete AST. CLI evidence is trusted local data, not authenticated provenance. Preserve these limits when refreshing bindings.

## checks

Existing `cargo test -p mova-twin --locked`: three library tests compare online updates with independent 2D ridge, reject invalid information and preserve prospective residuals; three ledger integration tests exercise forecast-before-execution, parent replay/tamper rejection, rejected runs, timeout, input mutation and predict without launch. Integration commands are bounded local fixtures, not live benchmarks or teacher calls. See ../model.md#checks at the file root for execution evidence. Other oracles below are proposals. Structural validation checks hashes, references and bounds, not equations.

## behavior-complement

U is (M,x,Result Prediction). Source-derived U\R excludes successful nonfinite seconds/leverage or negative leverage and mismatched feature length. Negative finite seconds and zero feature vectors are allowed by library predict. Proposed oracle distinguishes this library domain from CLI nonzero-feature validation.

## reverse-reconstruction

At fixed M and prediction z, B(z)={x:dot(x,theta)=z.seconds, dot(x,Px)=z.leverage, box predicate=z.outside, all Rust guards pass}; n and RMSE are fixed state projections. In real SPD geometry this intersects a hyperplane with an ellipsoid plus box inequalities; rounding modifies boundaries. Proposed oracle groups a small feature grid and preserves all ties.

## rmse-observation

Observation n=0 maps any validate-accepted SSE to historical_rmse=None; n>0 maps to sqrt(SSE/(n as f64)). Thus None loses SSE, and floating casts/sqrt can merge distinct history values. Proposed oracle contrasts two n=0 loaded states with unequal SSE; validate permits both.

## box-boundary

For n>0, outside is OR_i(x_i<min_i or x_i>max_i), using strict inequalities. Boundary equality is inside; n=0 always outside. Observed box coverage is not density, hull membership or uncertainty. Existing ridge test exercises x5 outside and x2 inside after maximum4.

## History

### 2026-10-29 · document

Subject: `tools/twin/src/lib.rs`, exact SHA-256 in manifest; modeling epoch 2026-10-29.

### 2026-10-29 · epoch-memory

Initial source review, 2026-10-29. Default Rust source remains unchanged. These models record mechanisms and gaps rather than claiming statistical calibration, semantic proof or a complete AST. CLI evidence is trusted local data, not authenticated provenance. Preserve these limits when refreshing bindings.
