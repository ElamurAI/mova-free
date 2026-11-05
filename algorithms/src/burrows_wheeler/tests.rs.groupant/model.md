# algorithm-burrows-wheeler-tests

## descriptant

### algorithm-burrows-wheeler-tests epoch memory

Sort all n cyclic rotations lexicographically with equal rotations ordered by ascending source offset. Return their last bytes C and the row p whose source offset is zero. Empty T returns ([],0). Inverse checks p then computes an LF candidate and accepts it iff its canonical transform equals (C,p).

Comparison-sort cyclic doubling and final sort take O(n log² n) time and O(n) storage. LF candidate decoding takes O(n+256); inverse's canonical retransformation adds O(n log² n), so complete inverse is O(n log² n+256), not linear. Allocation failure may panic/abort. No benchmark gain is claimed.

The prospective design retained comparison-sort construction costs and declined an unsupported linear total-time claim. Canonical BWT reachability is stricter than merely applying LF; byte wildcards deliberately have no escape grammar. Models distinguish information retained in full outputs from information discarded by projections. Test runs and compiled controls are recorded in model.md#evidence. No earlier modeling epoch is claimed. Partial Rust syntax is source-bound, with macro bodies opaque; it is neither a semantic AST nor proof. Future work may improve bounds only after prospective derivation and measurements.

## model

# algorithm-burrows-wheeler-tests

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

Materialize all rotations and sort (rotation,offset), independently of doubling. Enumerate ternary words through length eight; enumerate every column through length five and every primary 0..=n, comparing inverse to a complete independently constructed reachable map.

Domain: Finite declared fixture executions with successful compilation and allocation. Observation: finite test outcomes. Assumptions: fixed source revision, unsigned byte ordering, successful allocation and each stated premise. Status: source-derived relation supported by bounded checks, not universal proof.

## fiber-observation

The column bbaa has at least the two texts abab and baba; full canonical primary distinguishes them. Exhaustive reachability search detects empty inverse fibers for invalid pairs.

Domain: Finite declared fixture executions with successful compilation and allocation. Observation: finite test outcomes. Assumptions: fixed source revision, unsigned byte ordering, successful allocation and each stated premise. Status: source-derived relation supported by bounded checks, not universal proof.

## negative-control

A compiled production mutation and a separate false test expectation each fail a named assertion; exact original bytes are restored and the selected tests pass. This establishes these controls are detectable, not complete mutation coverage.

Domain: Finite declared fixture executions with successful compilation and allocation. Observation: finite test outcomes. Assumptions: fixed source revision, unsigned byte ordering, successful allocation and each stated premise. Status: source-derived relation supported by bounded checks, not universal proof.

## evidence

### Evidence

False oracle control: require abab and baba primary rows to be equal; column_projection_has_rotation_fiber fails.

Syntax is generated twice and byte-compared. Strict algorithm Clippy and narrow rustfmt run separately. Structural validators check declarations, not semantic truth. Exhaustion, Unicode scalar matching, downstream application behavior and universal correctness remain outside the evidence.

Structural validation uses an external scaffold preserving all base registry owners and appending these six sources. A first minimal scaffold exposed existing owner checks; the corrected scaffold passes with no diagnostics and reports all six changed sources registered. The temporary missing scaffold-root sidecar and 473 preexisting missing sidecars are explicit backlog, not failures in these leaves.

## History

### 2026-10-30 · descriptant

2026-10-30: original implementation and fixtures; exact byte semantics and independent bounded oracles. Source SHA-256: ca1d45dcc711efe6b16e2fef8f8471fb19673c14ab386257dcb1d634355906cb.

### 2026-10-30 · evidence

2026-10-30. Source SHA-256: ca1d45dcc711efe6b16e2fef8f8471fb19673c14ab386257dcb1d634355906cb. Five module tests pass twice in debug and twice in release with identical normalized test identities/results. Raw logs: $MOVA_DATA/runs/octopus-wave10-text-2026-10-30/.

- burrows_wheeler-debug-1.log: SHA-256 4819eaea7c81fac155dbaa8d05a34495c63d94d4161da2f028ecea6eb90e5f31

- burrows_wheeler-debug-2.log: SHA-256 fd104938fbabc76cc45744b20b2d00afaeb1fe651a7cdf4ae30d07eadb2ced29

- burrows_wheeler-release-1.log: SHA-256 6eb3830dc86658a05077b1768431f65a104df9bd21028158d686a7efdcbf399f

- burrows_wheeler-release-2.log: SHA-256 5cdea5dd1b9749e2d1bf0f331164586c47cd0962b88c701e2b8cf4e58ee43e94

Compiled mutation log burrows_wheeler-tests-rs-mutant.log: SHA-256 9f60b7116f3db7c5e8efadacf44fd578764615c1519b9a9f90ddb5ac98e957ad. Named assertion failure is required, not compilation failure; original source bytes restored and focused test passes.

Strict Clippy log SHA-256: ee26a23cab9e52eddcd50fa854be2183d0a2329a03f7c8cb922475aaf27ec62f. Source syntax was generated twice identically; syntax SHA-256: e8d887d865e4bd5e8f6939286cd61e090d401b3ea9fb15eb7582911d0013f107.
