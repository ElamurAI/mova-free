# algorithm-huffman

## descriptant

### Combiner and epoch memory

Prospective prediction: deterministic exhaustive agreement, boundary correctness and a compiled semantic mutant rejected. Initial scope deliberately excludes throughput and downstream language quality. Preserve observations when later epochs revise these models. Full semantic AST, macro expansion, executable approximate substitutes and universal proof remain absent.

## model

# huffman mod.rs model

## Forward

Return deterministic binary prefix codes minimizing sum_i w_i*len(code_i) among prefix-free nonempty codes. Empty yields []; singleton yields [[false]]. Merge queue orders by (u128 subtree sum, creation index); false labels first pop, true second. New parents follow original indices.

## Invariant

Active trees partition all original symbols. Parent sums equal descendant weights; parent indices increase strictly along links. Each merge preserves disjointness and yields two distinct child bits. Leaf-to-root paths reverse to prefix-free codes.

## Cost

O(n log n + L) time, O(n+L) space where L is total emitted code length, up to O(n²). Sums in u128 cannot overflow for allocated u64 slices on <=64-bit usize. Weighted cost is measured in tests with u128.

## nonbehavior

Type: behavior_complement. Domain: Finite slices of u64 weights; every position is a distinct symbol even when its weight is zero. Codes are vectors of booleans in original order. Successful allocation, ordinary usize<=64-bit platforms.

Exclude wrong symbol count, empty code for any present symbol, duplicate or prefix-related codes, suboptimal weighted cost, and ties/orientations inconsistent with creation-order merging. Empty and singleton conventions are part of the exact behavior.

## compatible_antecedents

Type: reverse_reconstruction. Domain: Finite slices of u64 weights; every position is a distinct symbol even when its weight is zero. Codes are vectors of booleans in original order. Successful allocation, ordinary usize<=64-bit platforms.

For an observed code forest C, compatible weights are all nonnegative u64 vectors whose ordered merge inequalities select its oriented tree under (sum,index) comparisons. Ties permit <= only when creation index wins. Positive scaling within u64 preserves comparisons, so recovery is generally nonunique; [1,1] and [5,9] both yield [false],[true].

## prefix_free

Type: output_constraint. Domain: Finite slices of u64 weights; every position is a distinct symbol even when its weight is zero. Codes are vectors of booleans in original order. Successful allocation, ordinary usize<=64-bit platforms.

For distinct symbols i,j, neither code_i nor code_j prefixes the other. This enables unique decoding of a stream when symbol count or termination convention is supplied; singleton false is not an empty code.

## optimal_cost

Type: optimization_relation. Domain: Finite slices of u64 weights; every position is a distinct symbol even when its weight is zero. Codes are vectors of booleans in original order. Successful allocation, ordinary usize<=64-bit platforms.

For n>=2, weighted path length equals the sum of all merged subtree weights. Choosing two least weights permits an optimal sibling pair by exchange; contraction reduces to the smaller problem.

## positive_scaling

Type: metamorphic_relation. Domain: Finite slices of u64 weights; every position is a distinct symbol even when its weight is zero. Codes are vectors of booleans in original order. Successful allocation, ordinary usize<=64-bit platforms.

Multiplying all weights by the same positive integer, without u64 overflow, preserves all comparisons and exact codes, and multiplies weighted cost.

## zero_weights

Type: boundary_exclusion. Domain: Finite slices of u64 weights; every position is a distinct symbol even when its weight is zero. Codes are vectors of booleans in original order. Successful allocation, ordinary usize<=64-bit platforms.

Zero weights remain leaves and may receive long codes. All-zero inputs still have deterministic nonempty prefix-free codes and weighted cost zero; deleting zero symbols changes the coding problem.

## tie_schedule

Type: determinism. Domain: Finite slices of u64 weights; every position is a distinct symbol even when its weight is zero. Codes are vectors of booleans in original order. Successful allocation, ordinary usize<=64-bit platforms.

Equal weights are resolved by unique creation index; original symbols precede newly created nodes. For [1,1,1] the exact codes are [true,false],[true,true],[false].

## weight_information_loss

Type: information_loss. Domain: Finite slices of u64 weights; every position is a distinct symbol even when its weight is zero. Codes are vectors of booleans in original order. Successful allocation, ordinary usize<=64-bit platforms.

Codes expose relative merge choices, not absolute weights. Different integer weight vectors share a code set; even optimal length vectors do not determine bit orientation.

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
