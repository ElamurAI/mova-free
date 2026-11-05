# algorithm-wildcard-matching-tests

## descriptant

### algorithm-wildcard-matching-tests epoch memory

Return true iff the complete pattern consumes the complete text. A literal byte consumes the same byte, '?' consumes one arbitrary byte, and '*' consumes k>=0 arbitrary bytes. There is no escaping or Unicode decoding. The frontier stores exact prefix-language membership.

For text length n and pattern length m, O((n+1)(m+1)) time and O(m+1) storage; this is exact bounded-frontier DP rather than an unproved linear greedy claim. Valid byte slice lengths fit isize so m+1 fits usize; allocation failure may panic/abort. No resource cap or approximate mode.

The prospective design retained comparison-sort construction costs and declined an unsupported linear total-time claim. Canonical BWT reachability is stricter than merely applying LF; byte wildcards deliberately have no escape grammar. Models distinguish information retained in full outputs from information discarded by projections. Test runs and compiled controls are recorded in model.md#evidence. No earlier modeling epoch is claimed. Partial Rust syntax is source-bound, with macro bodies opaque; it is neither a semantic AST nor proof. Future work may improve bounds only after prospective derivation and measurements.

## model

# algorithm-wildcard-matching-tests

## Forward

Execute finite oracle, boundary, fiber and conditional-law assertions. Success reports agreement only on their enumerated observations.

## Invariant

Fixtures and expected results are deterministic. Independent reference construction avoids reusing the production state transition being tested.

## Cost

Finite exhaustive enumeration; exponential in fixture word length, with explicit caps in source. BWT oracle materializes n rotations; wildcard recursive oracle enumerates star partitions. No production cost or broad performance claim follows.

## nonbehavior

Within finite tested observations, exclude suite acceptance if any expected value disagrees with its independently specified oracle or asserted conditional law. Unvisited cases are not excluded by finite testing.

Domain: Finite declared fixture executions with successful compilation and allocation. Observation: finite test outcomes. Assumptions: fixed source revision, unsigned byte ordering, successful allocation and each stated premise. Status: source-derived relation supported by bounded checks, not universal proof.

## preimages

The preimage of a passing test report includes every implementation agreeing on the observed fixtures, even implementations differing outside this domain. Failing reports identify a violated assertion, not a unique faulty program.

Domain: Finite declared fixture executions with successful compilation and allocation. Observation: finite test outcomes. Assumptions: fixed source revision, unsigned byte ordering, successful allocation and each stated premise. Status: source-derived relation supported by bounded checks, not universal proof.

## oracle-independence

Recursively enumerate every possible consumed length of each star; compare to DP on binary text through length five and four-symbol patterns through length five. This oracle shares language semantics but no frontier machinery.

Domain: Finite declared fixture executions with successful compilation and allocation. Observation: finite test outcomes. Assumptions: fixed source revision, unsigned byte ordering, successful allocation and each stated premise. Status: source-derived relation supported by bounded checks, not universal proof.

## fiber-observation

Pattern a*b on length-four binary texts accepts exactly those with first byte a and final byte b; ?*b* requires b after the first position. Several accepted texts witness nonunique Boolean preimages.

Domain: Finite declared fixture executions with successful compilation and allocation. Observation: finite test outcomes. Assumptions: fixed source revision, unsigned byte ordering, successful allocation and each stated premise. Status: source-derived relation supported by bounded checks, not universal proof.

## negative-control

A compiled production mutation and a separate false test expectation each fail a named assertion; exact original bytes are restored and the selected tests pass. This establishes these controls are detectable, not complete mutation coverage.

Domain: Finite declared fixture executions with successful compilation and allocation. Observation: finite test outcomes. Assumptions: fixed source revision, unsigned byte ordering, successful allocation and each stated premise. Status: source-derived relation supported by bounded checks, not universal proof.

## evidence

### Evidence

False oracle control: reject ab against *; boolean_fiber_preserves_multiple_texts fails.

Syntax is generated twice and byte-compared. Strict algorithm Clippy and narrow rustfmt run separately. Structural validators check declarations, not semantic truth. Exhaustion, Unicode scalar matching, downstream application behavior and universal correctness remain outside the evidence.

Structural validation uses an external scaffold preserving all base registry owners and appending these six sources. A first minimal scaffold exposed existing owner checks; the corrected scaffold passes with no diagnostics and reports all six changed sources registered. The temporary missing scaffold-root sidecar and 473 preexisting missing sidecars are explicit backlog, not failures in these leaves.

## History

### 2026-10-30 · descriptant

2026-10-30: original implementation and fixtures; exact byte semantics and independent bounded oracles. Source SHA-256: 792a86a0b777bfd9a3bd6f37583f30fbc12b1136b0136efc1a4468b88f67d423.

### 2026-10-30 · evidence

2026-10-30. Source SHA-256: 792a86a0b777bfd9a3bd6f37583f30fbc12b1136b0136efc1a4468b88f67d423. Five module tests pass twice in debug and twice in release with identical normalized test identities/results. Raw logs: $MOVA_DATA/runs/octopus-wave10-text-2026-10-30/.

- wildcard_matching-debug-1.log: SHA-256 ddba4c7d5c7148e800af0bb8704b61b6b950680a2eda9ae34ce5c42ea031bd74

- wildcard_matching-debug-2.log: SHA-256 52927abb4c41a9fce418ec62ac5ef3e7c07dc4b50b2fde703f6f49707f578af6

- wildcard_matching-release-1.log: SHA-256 dc1b06b4b2917d9e8afbcf1ff1cf30e4f97d81e1a5e408ce60720baa13b04c69

- wildcard_matching-release-2.log: SHA-256 d64ba49b1f531d63e32877c53d2dca179bb5878411d4759e9db825d9fc290a9c

Compiled mutation log wildcard_matching-tests-rs-mutant.log: SHA-256 42e5e54aa5ac820d224b104d444834a89b4bd37097ec3b00af644947f652ba52. Named assertion failure is required, not compilation failure; original source bytes restored and focused test passes.

Strict Clippy log SHA-256: ee26a23cab9e52eddcd50fa854be2183d0a2329a03f7c8cb922475aaf27ec62f. Source syntax was generated twice identically; syntax SHA-256: 5fba5f1931b8c9c91a720b03cad29908c2f4f56fffdd1d1c0e652decba1cf899.
