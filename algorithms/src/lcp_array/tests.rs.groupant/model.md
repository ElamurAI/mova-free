# algorithm-lcp-array-tests

## descriptant

### algorithm-lcp-array-tests epoch memory

For finite byte text T of length n, return (A,L): A sorts the nonempty suffixes, L[0]=0 if n>0, and L[r]=max k with T[A[r-1]..A[r-1]+k]=T[A[r]..A[r]+k]. Empty input returns two empty vectors.

Existing suffix_array costs O(n log² n); the Kasai phase costs O(n) time and O(n) auxiliary space. Each successful byte comparison increases common and each outer iteration decreases it by at most one. This is not an overall linear suffix constructor. Allocation exhaustion is outside the terminating domain.

The prospective design retained comparison-sort construction costs and declined an unsupported linear total-time claim. Canonical BWT reachability is stricter than merely applying LF; byte wildcards deliberately have no escape grammar. Models distinguish information retained in full outputs from information discarded by projections. Test runs and compiled controls are recorded in model.md#evidence. No earlier modeling epoch is claimed. Partial Rust syntax is source-bound, with macro bodies opaque; it is neither a semantic AST nor proof. Future work may improve bounds only after prospective derivation and measurements.

## model

# algorithm-lcp-array-tests

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

Direct whole-suffix comparison plus zipped byte-prefix counting supplies a reference independent of Kasai carry. Enumerate ternary words through length eight; interval-minimum and distinct-substring oracles inspect actual substrings.

Domain: Finite declared fixture executions with successful compilation and allocation. Observation: finite test outcomes. Assumptions: fixed source revision, unsigned byte ordering, successful allocation and each stated premise. Status: source-derived relation supported by bounded checks, not universal proof.

## fiber-observation

Across two-byte ternary words, target ([1,0],[0,1]) has exactly the three constant pairs; label ambiguity is retained.

Domain: Finite declared fixture executions with successful compilation and allocation. Observation: finite test outcomes. Assumptions: fixed source revision, unsigned byte ordering, successful allocation and each stated premise. Status: source-derived relation supported by bounded checks, not universal proof.

## negative-control

A compiled production mutation and a separate false test expectation each fail a named assertion; exact original bytes are restored and the selected tests pass. This establishes these controls are detectable, not complete mutation coverage.

Domain: Finite declared fixture executions with successful compilation and allocation. Observation: finite test outcomes. Assumptions: fixed source revision, unsigned byte ordering, successful allocation and each stated premise. Status: source-derived relation supported by bounded checks, not universal proof.

## evidence

### Evidence

False oracle control: expected compatible two-byte constant-pair count four instead of three; inverse_fiber_and_relabel fails.

Syntax is generated twice and byte-compared. Strict algorithm Clippy and narrow rustfmt run separately. Structural validators check declarations, not semantic truth. Exhaustion, Unicode scalar matching, downstream application behavior and universal correctness remain outside the evidence.

Structural validation uses an external scaffold preserving all base registry owners and appending these six sources. A first minimal scaffold exposed existing owner checks; the corrected scaffold passes with no diagnostics and reports all six changed sources registered. The temporary missing scaffold-root sidecar and 473 preexisting missing sidecars are explicit backlog, not failures in these leaves.

## History

### 2026-10-30 · descriptant

2026-10-30: original implementation and fixtures; exact byte semantics and independent bounded oracles. Source SHA-256: 2f696f045dc74f4c32e313bddf071fc3992b367b92d586804808c34ec2fc34b3.

### 2026-10-30 · evidence

2026-10-30. Source SHA-256: 2f696f045dc74f4c32e313bddf071fc3992b367b92d586804808c34ec2fc34b3. Five module tests pass twice in debug and twice in release with identical normalized test identities/results. Raw logs: $MOVA_DATA/runs/octopus-wave10-text-2026-10-30/.

- lcp_array-debug-1.log: SHA-256 21d7e4bdc9b42c16e789ef9ab9012f786d26b0b18239be4e5ce2a18b85e97d88

- lcp_array-debug-2.log: SHA-256 0b8df33a6fee80e288458354d9f76bbfed1cfedf1ce2241ae8d4bcb5de8d0f05

- lcp_array-release-1.log: SHA-256 4c251490e30659849cc12b1a8159835a1f8ae209fa8ff53286c8bc12d3cf0639

- lcp_array-release-2.log: SHA-256 6d4bc8e0409c5df5e527130a704576b5028376e841013fd35a59fea73c4727e2

Compiled mutation log lcp_array-tests-rs-mutant.log: SHA-256 20610ab74fcb747fa1ea432fac9633a6c96264110c9a0415e5f5f23f772469df. Named assertion failure is required, not compilation failure; original source bytes restored and focused test passes.

Strict Clippy log SHA-256: ee26a23cab9e52eddcd50fa854be2183d0a2329a03f7c8cb922475aaf27ec62f. Source syntax was generated twice identically; syntax SHA-256: 2cc359017484fd5d060fa7bedec5f2850168bfda355b2c87e0c212124c1bb7fc.
