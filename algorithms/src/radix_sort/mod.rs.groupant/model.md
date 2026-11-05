# algorithm-radix-sort

## descriptant

### radix_sort mod.rs epoch memory

Four tests pass in debug and release. All 21,845 arrays of lengths 0..7 over {0,255,256,u64::MAX} match std slice sorting. An 8,192-value fixed-seed full-width array and individual-byte/extreme fixtures match. An internal pass test compares each byte against stable std sort_by_key. Allocator exhaustion is not injected; allocation error atomicity follows source ordering only. A compiled mutant skipping the highest byte pass is rejected by full_width_boundaries_and_idempotence.

The combiner relates raw source, partial syntax, forward relation, typed obernants, invariant, cost and finite evidence. Preserve predictions and rejected mutants across future epochs. Internal radix byte stability is observable via distinct values sharing a digit; whole-sort payload stability is not exposed. Missing universal proof, allocator-exhaustion injection and downstream wiring remain explicit. Exact algorithms require no invented approximant profiles. Future executable approximants must honor 1%, 2%, 5%, 10% caps and source/evidence binding.

## model

# radix_sort mod.rs model

## Forward

For length below two return Ok unchanged. Otherwise reserve n scratch values before writes; reserve failure returns TryReserveError with unchanged input. Eight stable byte passes in increasing significance yield the unique ascending permutation, preserving all multiplicities.

## Invariant

At each byte pass counts sum to n; prefix cursors partition destination into disjoint buckets. Traversing source left-to-right preserves relative order within a digit. After k passes values are sorted by their k lowest bytes; after eight this is unsigned u64 order. Explicit little-endian byte extraction is host independent.

## Cost

O(8(n+256)) time and O(n+256) auxiliary elements; eight copies of n values. Trivial inputs O(1). No timing measurements.

## nonbehavior

Type: behavior_complement. For observations (input, allocator outcome, Result, output), exclude nonascending success, changed length or multiset, mutation on reserve error, or failure after any byte pass. Internal pass observations exclude decreasing digit order or reversal within equal-digit source subsequences.

## preimages

Type: reverse_reconstruction. Given successful sorted output y, compatible original inputs are all permutations of its multiset; allocation must succeed unless len<2. Err admits the unchanged input and failed reserve. For an internal digit-pass result, predecessors are shuffles of digit subsequences preserving order within each digit. Sorting is not injective; no unique original ordering is claimed.

## idempotence

Type: metamorphic_relation. S(S(x))=S(x). Each byte pass preserves every digit subsequence. Tests use distinct scalar values sharing selected byte keys to observe internal pass stability; scalar full-sort equality cannot demonstrate attached payload stability.

## pass-shuffle-fiber

Type: reverse_reconstruction. Domain: Private byte_pass with byte 0..7, equal source/destination lengths, arbitrary u64 values.

For fixed byte b and digit-sorted y, P_b(x)=y iff for every digit d the subsequence of x with digit d equals that of y in order. Thus predecessors are interleavings of fixed digit subsequences, not arbitrary multiset permutations. Distinct values [0x300,0x100,0x201,0x001] give six accepted shuffles at byte zero; reversing a same-digit pair is rejected.

Source-derived conditional relation; finite correspondence is not a universal proof. See model.md#evidence.

## pass-composition

Type: composition_relation. Domain: Eight internal passes, k in 1..8, successful scratch allocation and distinct or repeated u64 scalars.

After k stable passes in increasing byte significance, output is the stable ordering of original input by its low 8k bits. At k=8 this is full unsigned order; using a full-width mask avoids shifting by 64. Reversing pass order or reversing equal-digit source subsequences invalidates the inductive step. All eight stages are compared against an independent stable key sort.

Source-derived conditional relation; finite correspondence is not a universal proof. See model.md#evidence.

## monotone-relabel

Type: metamorphic_relation. Domain: Finite u64 slices, bounded strictly increasing transformations, successful whole-sort calls.

S(f(x))=f(S(x)) for f strictly increasing on the finite support with representable u64 images and successful allocations. The checked affine map 257v+19 is observed; wrapping addition on [0,MAX] is a countermodel when representability is dropped. This is a full-sort law, not a law that byte partitions commute with arbitrary relabeling.

Source-derived conditional relation; finite correspondence is not a universal proof. See model.md#evidence.

## scratch-boundary

Type: resource_preimage. Domain: Whole-call Result and input/output observations including allocator outcomes; valid allocatable Rust slices.

Lengths zero and one succeed without reservation. For n>=2 the sole fallible reservation precedes every pass and copy; Err therefore has unchanged input and a failed reservation as compatible antecedent. Scratch length and requested capacity are n u64 elements (the allocator may reserve more) and counts use 256 usize slots per pass. No public budget parameter or allocator failure injection exists; exact allocator feasibility cannot be inferred from n alone.

Source-derived conditional relation; finite correspondence is not a universal proof. See model.md#evidence.

## bucket-partition

Type: conservation_relation. Domain: Internal valid byte passes; byte index 0..7 and destination length n.

Counts sum to n and prefix starts partition destination into digit intervals [sum_{e<d}H[e],sum_{e<=d}H[e]). Left-to-right source traversal advances each cursor once per occurrence, preserving its subsequence. Disjoint intervals imply no dropped or duplicated values. Whole-sort scalar equality cannot establish stability of attached records absent from this API.

Source-derived conditional relation; finite correspondence is not a universal proof. See model.md#evidence.

## evidence

### Evidence

Four tests pass in debug and release. All 21,845 arrays of lengths 0..7 over {0,255,256,u64::MAX} match std slice sorting. An 8,192-value fixed-seed full-width array and individual-byte/extreme fixtures match. An internal pass test compares each byte against stable std sort_by_key. Allocator exhaustion is not injected; allocation error atomicity follows source ordering only. A compiled mutant skipping the highest byte pass is rejected by full_width_boundaries_and_idempotence.

Strict crate Clippy and owned rustfmt checks pass. Two deterministic debug executions have matching normalized test outcomes. Source-derived syntax is generated twice and compared. No timing, downstream wiring or universal verification claim.

## prospective

### Prospective

Before implementation: repository search in algorithms/src and math/src found no counting_sort or radix_sort. Implement original exact integer sorting, with comparison sorting used only as an independent test oracle. Counting histogram spans min..=max with checked arithmetic and caller bucket cap; allocation finishes before mutation. Radix uses eight stable least-significant-byte passes. Predict O(n+span) time/O(span) memory and O(8(n+256)) time/O(n+256) memory respectively. Preserve length and multiplicities. Empty input succeeds without allocation. No timing gain or universal verification claim. Validate exhaustive bounded arrays, extremes, deterministic larger inputs, debug/release and strict Clippy; reject a compiled implementation mutant.

## History

### 2026-10-30 · descriptant

2026-10-30. Repository search found no matching implementation in algorithms/src or math/src. Original English std-only code; no external solution imports. Prospective mathematics preceded implementation and remains in model.md#prospective.

#### Relation refinement 2026-10-30

Added histogram and digit-subsequence fibers, bounded monotone transforms and explicit resource or pass-composition boundaries. The model distinguishes whole-sort order loss from pass-local subsequence retention. Three new Rust relation tests per algorithm reject explicit false premises. The implementation and public API remain unchanged. Models retain allocator uncertainty and scalar stability limits. No timing claim or approximate execution profile is introduced; supported caps remain at most 10%.

Finite confirmation: debug/release twice and strict Clippy passed; all six targeted compiled mutant checks showed red. The monotone relation alone admits a minimum-only implementation, so multiplicity-fiber checks remain independently necessary. See model.md#evidence for the preserved surviving countermodel and log hashes.

### 2026-10-30 · evidence

Heading `### Evidence 2026-10-30` stays in evidence as `### Evidence`.

#### Relation refinement checks 2026-10-30

Focused debug and release suites each ran twice with identical normalized per-test outcomes (durations excluded): six counting tests and seven radix tests. Strict crate Clippy, all targets with warnings denied, passed. Three new production-linked relation tests per module include explicit false-relation countermodels. The counting histogram test enumerates 256 inputs and admits 12 predecessors; the radix pass test enumerates 256 inputs and admits six.

Each new test separately rejected a compiled production mutant. Counting mutants: fill every bucket with minimum (histogram fiber), omit minimum from output value (monotone relabel), reject budget equality (budget fiber). Radix mutants: reverse source traversal in scatter (pass fiber and pass composition), omit low-byte pass (full-sort fiber and monotone relabel). Six compiled mutant runs each failed the selected test. An initial minimum-fill mutant survived the monotone-only test because taking minima commutes with increasing maps; this is retained as evidence that the metamorphic law alone does not prove sorting. The histogram test rejects it.

Production implementations and APIs are unchanged. Four source-bound syntax projections reproduced identically twice. Allocation exhaustion remains untested; counting deterministic capacity overflow has existing coverage. No timing gain or approximation claim.

External evidence SHA-256 (logs stored outside the repository):

- radix_sort-debug-1.log: 3e1a87a2176d0625c5ab5b74390269e9f2d66372a44ffbd767fb95d36e0d3892
- radix_sort-debug-2.log: 3e1a87a2176d0625c5ab5b74390269e9f2d66372a44ffbd767fb95d36e0d3892
- radix_sort-release-1.log: ee0f25d86b6fc7bfc89fd8f4de99e3049ceef91f34f26d18e78817251e95e0de
- radix_sort-release-2.log: 4de3fc40af29762e67d9d15727de071eaf0abe4930e36bdf11faf2b52c868e1c
- clippy.log: ee1d3fffa2022e962e63e0dbe4d35c9c198450a20c29fc09030f635dbb38bfcd
- mutant-3.log: 96087631e7a2ed9572be0819c0071d3b78d6c95e4889d4bc02a7e8f4956deede
- mutant-4.log: 018e7d7bb13827e2125e3851ebc81232e98cd86fb52c6aba35e2f21124df36d7
- mutant-5.log: b34ad4f7aaed0f71231a7b63105f82f9feb4b975022f394724f3343d30cae665

### 2026-10-30 · prospective

Heading `### Prospective epoch 2026-10-30` stays in prospective as `### Prospective`.
