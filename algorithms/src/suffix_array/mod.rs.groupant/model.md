# algorithm-suffix-array

## descriptant

### suffix_array mod.rs epoch memory

Four tests: 9,841 ternary inputs of length 0..8 against independent direct suffix comparison; empty/singleton/banana, 1,025-byte equal and periodic strings, all bytes; increasing relabel and minimum-prefix laws; ten compatible predecessor triples. Compiled production mutant reversing returned order fails boundaries_and_long_common_prefixes.

The combiner relates authoritative source and its SHA-256, partial syntax, forward relation, typed obernants, invariant, cost and finite evidence. Models preserve ambiguity in compatible predecessors. Missing semantic AST analysis, universal proof, allocator exhaustion and downstream composition remain explicit. Exact production algorithms need no fabricated executable approximation. Any future approximate profile must honor supported 1%, 2%, 5%, 10% caps and bind source and evidence.

## model

# suffix_array mod.rs model

## Forward

Return the unique permutation of offsets 0..n whose nonempty suffixes increase lexicographically by unsigned byte order. Prefix-doubling pairs refine ranks until distinct; exclude the empty suffix.

## Invariant

Ranks encode equal truncated prefixes and their lexicographic order. Each offset appears once; absent second ranks sort before actual ranks. Unique ranks imply complete suffix order. Index addition is checked.

## Cost

Comparison-sort doubling takes O(n log² n) time and O(n) auxiliary storage. No timing measurement or optimized linear-sort construction claim.

## nonbehavior

Type: behavior_complement. Domain: Finite byte slices and ordinary terminating executions with successful allocation.

Exclude omitted, repeated or out-of-range offsets, the empty suffix n, or adjacent offsets whose suffixes are in descending order. Input bytes remain unchanged.

Source-derived partial relation with finite evidence, not universal verification.

## preimages

Type: reverse_reconstruction. Domain: Finite byte slices and ordinary terminating executions with successful allocation.

For a fixed permutation A of 0..n, compatible byte texts are precisely length-n texts whose adjacent suffixes T[A[j]..] are strictly lexicographically increasing. Several texts satisfy these constraints; A alone does not recover bytes.

Source-derived partial relation with finite evidence, not universal verification.

## rank-fiber

Type: state_preimage. Domain: Finite byte slices and ordinary terminating executions with successful allocation.

Observe rank ordering and equality, not the initial literal numeric byte ranks. At width k, equal ranks mean equal length-k truncated suffix prefixes and rank order matches their lexicographic order. Any byte text with the same ordered prefix equivalence classes is compatible with this rank observation.

Source-derived partial relation with finite evidence, not universal verification.

## pair-refinement

Type: refinement_relation. Domain: Finite byte slices and ordinary terminating executions with successful allocation.

Sorting pairs (rank_i, optional rank_{i+k}) orders length-2k truncated prefixes; None is less than every present rank. Assigning consecutive ranks preserves pair equality and strict ordering.

Source-derived partial relation with finite evidence, not universal verification.

## monotone-relabel

Type: metamorphic_relation. Domain: Finite byte slices and ordinary terminating executions with successful allocation.

A strictly increasing map on the input byte support preserves the suffix array. Injectivity without monotonicity is insufficient, and wrapping maps violate the premise.

Source-derived partial relation with finite evidence, not universal verification.

## minimum-prefix

Type: metamorphic_relation. Domain: Finite byte slices and ordinary terminating executions with successful allocation.

Prepending a byte strictly below every input byte produces array [0] followed by every original offset increased by one. The empty input gives [0].

Source-derived partial relation with finite evidence, not universal verification.

## constant-fiber

Type: reverse_reconstruction. Domain: Finite byte slices and ordinary terminating executions with successful allocation.

For n=3, output [2,1,0] admits exactly nonincreasing triples; over three byte values there are ten compatible triples. Equal-value and strictly decreasing texts can yield the same suffix order.

Source-derived partial relation with finite evidence, not universal verification.

## termination

Type: progress_relation. Domain: Finite byte slices and ordinary terminating executions with successful allocation.

Each round doubles represented prefix length unless all n ranks are unique and returns. Width>=n distinguishes suffix lengths even for constant bytes. checked_add handles absent tails; saturating doubling cannot prevent termination for valid byte slices.

Source-derived partial relation with finite evidence, not universal verification.

## evidence

### Evidence

Four tests: 9,841 ternary inputs of length 0..8 against independent direct suffix comparison; empty/singleton/banana, 1,025-byte equal and periodic strings, all bytes; increasing relabel and minimum-prefix laws; ten compatible predecessor triples. Compiled production mutant reversing returned order fails boundaries_and_long_common_prefixes.

Debug and release each ran twice with identical normalized per-test outcomes (durations excluded). Strict crate Clippy with all targets and denied warnings, owned rustfmt, and four twice-generated identical source-bound syntax projections passed. All 175 crate tests also passed once with temporary module declarations. Mutants restored before hashing; no shared lib or registry edits committed. No timing, allocator-exhaustion or universal proof claim.

## prospective

### Prospective

Repository and family QA source search found no existing matching API. Original std-only implementation from classical algorithm ideas, no copied source. Before implementation: Prefix-doubling ranks encode lexicographic order of length-k suffix prefixes; sort rank pairs and double k until all ranks differ. Exclude empty suffix. Predict O(n log² n) time and O(n) space; absent second rank sorts before every present rank. Exhaustive independent byte oracle, boundaries, metamorphic and compatible-preimage checks; debug/release twice, strict Clippy, source-bound syntax twice and a compiled mutant. No timing or universal proof claim.

## History

### 2026-10-30 · descriptant

2026-10-30. Original std-only code, no copied source. Prospective model preceded code in model.md#prospective. Classical citations appear in implementation documentation; variant complexity is explicit.

### 2026-10-30 · evidence

Heading `### Evidence 2026-10-30` stays in evidence as `### Evidence`.

### 2026-10-30 · prospective

Heading `### Prospective epoch 2026-10-30` stays in prospective as `### Prospective`.
