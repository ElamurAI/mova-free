# algorithm-huffman-tests

## descriptant

### Combiner and epoch memory

Prospective prediction: deterministic exhaustive agreement, boundary correctness and a compiled semantic mutant rejected. Initial scope deliberately excludes throughput and downstream language quality. Preserve observations when later epochs revise these models. Full semantic AST, macro expansion, executable approximate substitutes and universal proof remain absent.

## model

# huffman tests.rs model

## Forward

Run two test functions for huffman: independent exhaustive oracle and explicit boundary/relational fixtures. Pass iff all assertions hold.

## Invariant

Fixture enumeration is deterministic, oracle observations are compared to production outputs, and tests use no random or external state.

## Cost

Exhaustive finite verification; exponential oracle work deliberately bounded by fixture lengths. No production throughput conclusions.

## nonbehavior

Type: behavior_complement. Domain: Deterministic successful Rust test runs on named bounded fixtures; no claims for unexecuted cases.

Within deterministic terminating test executions, a pass is excluded when any executed assertion is false; failure is excluded when all assertions hold. Untested inputs are outside this observation.

## compatible_antecedents

Type: reverse_reconstruction. Domain: Deterministic successful Rust test runs on named bounded fixtures; no claims for unexecuted cases.

A pass admits every implementation agreeing with all tested observations; many disagree elsewhere. A failure trace restricts compatible executions to those violating its named assertion and cannot identify a unique source bug.

## oracle_independence

Type: oracle_relation. Domain: Deterministic successful Rust test runs on named bounded fixtures; no claims for unexecuted cases.

Enumerate every anchored bipartition recursively, minimizing over all labeled full binary trees; do not greedily choose two least weights. Prefix checks independently enforce valid codes.

## finite_scope

Type: evidence_boundary. Domain: Deterministic successful Rust test runs on named bounded fixtures; no claims for unexecuted cases.

1093 weight vectors over {0,1,2} through length 6; explicit max-u64, zero, singleton, tie and scaling fixtures.

## mutant_discrimination

Type: negative_control. Domain: Deterministic successful Rust test runs on named bounded fixtures; no claims for unexecuted cases.

A separately compiled semantic source mutant must fail a real production assertion; a compiler error does not count. This demonstrates one detected error, not mutation completeness.

## Limits and reference

Conceptual reference: D. A. Huffman, A Method for the Construction of Minimum-Redundancy Codes, Proceedings of the IRE 40(9), 1952, 1098–1101. Original code, no imported solution. Finite evidence is not a proof; partial syntax omits macro expansion and semantic resolution. No executable approximation profile is declared for these exact routines.

## evidence

### Evidence

Passed: two debug and two release executions of the 220-test library, including two new tests for this algorithm. Ordered successful test records are byte-identical across all four runs after removing timing and harness-order noise. Strict cargo clippy -p mova-algorithms --offline --all-targets -- -D warnings and rustfmt --check on all six owned Rust files pass. One initial Clippy finding in the Huffman oracle was corrected to isolate_lowest_one before the final four-run verification.

Scope: 1,093 vectors of weights {0,1,2} through length 6 against exhaustive anchored partitions of labeled full binary trees, plus pairwise prefix exclusion. Explicit singleton zero, equal ties, zero-weight leaves, u64::MAX, positive scaling and code collisions.

Negative control: Changing the singleton bit from false to true in a separate copy compiles with rustc --edition 2024 --test; tests::boundaries_ties_and_scaling rejects it with an assertion failure and exit 101. The production source is unchanged by mutation checks. Compile failure does not count as rejection.

Finite evidence is not a universal proof or throughput measurement. Exact routines have no approximate executable profile, and no downstream school gain is claimed.

## prospective

## History

### 2026-10-30 · descriptant

2026-10-30: Original std-only implementation and independent oracle. The combiner jointly relates raw Rust, source-bound partial syntax, forward target, ordered typed obernants, invariant, cost and measured assertions. Tests and implementation remain separate leaves; passing leaves do not certify integration wiring. Conceptual reference: D. A. Huffman, A Method for the Construction of Minimum-Redundancy Codes, Proceedings of the IRE 40(9), 1952, 1098–1101. Original code, no imported solution.

### 2026-10-30 · evidence

Heading `### Evidence 2026-10-30` stays in evidence as `### Evidence`.

Both Rust files have syntax projections generated twice by installed mova-modelcheck syntax and compared byte-for-byte; manifests bind exact source SHA-256. No macro expansion or semantic resolution is claimed. Logs, normalized records and Rust gate helper are under $MOVA_DATA/runs/octopus-wave5-strings-2026-10-30. Temporary lib.rs exports were used for local checks and restored before commit; parent registry/wiring validation belongs to integration.

### 2026-10-30 · prospective

#### Prediction recorded before final gates

2026-10-30. Predict huffman agrees with its independent oracle over the bounded domain; debug/release result records repeat exactly. Predict semantic mutant fails a real assertion. Cost prediction: O(n log n + L) time, O(n+L) space where L is total emitted code length, up to O(n²). Sums in u128 cannot overflow for allocated u64 slices on <=64-bit usize. Weighted cost is measured in tests with u128.
