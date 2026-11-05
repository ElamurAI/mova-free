# twin-initial-state

## catalog

# Prior construction and loaded-state validation

Every relation below is scoped to the named observation, holding other input/state coordinates fixed unless quantified. U is its stated input/output (or finite trace) Cartesian universe, R the described reachable relation. Complement targets U \ R; converse retains every compatible antecedent, never an invented unique cause. Source-derived formulas are partial hypotheses about Rust; floating-point expressions use evaluation order where stated, ideal real identities are conditional. Checks marked proposed were not executed.

## forward

new establishes prior/diagonal state; validate checks the loaded-state predicate independently.

## invariant

Validation requires finite vectors/SSE>=0 and approximate symmetry plus successful finite positive Cholesky pivots. It does not certify an exact symmetric matrix, box ordering, n/SSE consistency or observational reachability. Constructor does not call validate: subnormal positive ridge may yield infinite P. Predict may return negative seconds. n+1 overflow is not guarded: debug panic or release wrapping is outside the n<u64::MAX model.

## cost

At dimension d<=32, validation uses O(d^3) arithmetic and O(d^2) allocation. Predict adds O(d^2); observe calls predict and another validate, hence O(d^3), not merely the O(d^2) rank-one kernel. State memory O(d^2). No wall-clock estimate is calibrated.

## composition

Leaf fragment; no child composition. predict and observe invoke validate externally; constructor does not.

## epoch-memory

Initial source review. Default Rust source remains unchanged. These models record mechanisms and gaps rather than claiming statistical calibration, semantic proof or a complete AST. CLI evidence is trusted local data, not authenticated provenance. Preserve these limits when refreshing bindings.

## checks

Existing `cargo test -p mova-twin --locked`: three library tests compare online updates with independent 2D ridge, reject invalid information and preserve prospective residuals; three ledger integration tests exercise forecast-before-execution, parent replay/tamper rejection, rejected runs, timeout, input mutation and predict without launch. Integration commands are bounded local fixtures, not live benchmarks or teacher calls. See ../model.md#checks at the file root for execution evidence. Other oracles below are proposals. Structural validation checks hashes, references and bounds, not equations.

## behavior-complement

U is constructor/validator input to Result observations. For the source-derived guard model, U\R includes new Ok with empty prior, d>32, nonfinite prior or nonpositive/nonfinite ridge, and validate Ok with malformed row lengths. It does not include new Ok with reciprocal overflow. Proposed oracle pairs tiny ridge constructor success with later validation failure.

## reverse-reconstruction

At fixed constructor output M, candidate priors equal M.theta and candidate positive finite ridge values satisfy fl(1/ridge)=each diagonal and off-diagonal zeros, n=SSE=0, zero bounds. Rounded reciprocals can have several antecedents; validation success alone admits many unreachable constructed/history states. Proposed finite oracle groups ridge candidates by output bits.

## cholesky-prefix

Validation first checks dimensions, vector finiteness, SSE, all row dimensions/finiteness, then ordered lower-triangle symmetry and pivots. At each diagonal a=P_ii-sum L_ik² must be finite positive. First failing prefix determines error; near-symmetric matrices can pass without exact SPD semantics. Proposed oracle isolates each failure stage, including a finite indefinite 2x2 matrix.

## initial-cost

Constructor O(d²) allocation/arithmetic and validation O(d³) plus O(d²) temporary L. Thus construction success is cheaper than validated usable state. Proposed oracle checks allocation dimensions d=1,32 and rejection at0,33; no latency calibration executed.

## History

### 2026-10-29 · document

Subject: `tools/twin/src/lib.rs`, exact SHA-256 in manifest; modeling epoch 2026-10-29.

### 2026-10-29 · epoch-memory

Initial source review, 2026-10-29. Default Rust source remains unchanged. These models record mechanisms and gaps rather than claiming statistical calibration, semantic proof or a complete AST. CLI evidence is trusted local data, not authenticated provenance. Preserve these limits when refreshing bindings.
