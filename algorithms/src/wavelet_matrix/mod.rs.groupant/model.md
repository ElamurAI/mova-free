# algorithm-wavelet-matrix

## model

# Static unsigned wavelet matrix

## domain
Finite u64 slices; usize endpoints and zero-based order k. Index intervals
and range_count value intervals are half-open. Allocation success assumed.
All 64 bits participate, including the unsigned high bit. No configurable
resource budget; callers must limit input length to their memory budget.

## forward
Build 64 stable partitions from most significant bit to least. Row d stores
P_d[i], number of zero bits before i, and Z_d=P_d[n]. A zero branch maps
[a,b) to [P_d[a],P_d[b]); a one branch maps to
[Z_d+a-P_d[a],Z_d+b-P_d[b]). rank follows all bits of its exact value.
count_less adds P_d[b]-P_d[a] when upper has bit one, then follows that
bit. range_count subtracts two less counts. kth follows zeros if k is
less than zero multiplicity; otherwise subtracts that multiplicity and
sets the answer bit. Empty counts are zero; kth is absent for empty ranges.
Reversed values or invalid indices return None; no saturation/clamping.

## invariant
Rows begin at zero and each increment is zero or one. Each mapped interval
is the stable subsequence matching the already selected bit prefix. Counts
are bounded by original interval length. Stable partition order is required:
global sorting would lose the mapping from original subintervals. Query
subtractions follow validation. Bit shifts are 0..63. All sums of prefix
counts are bounded by stored length; feasible allocations bound usize work.

## cost
Exactly 64(n+1) stored usize prefixes and 64 usize zero totals, plus Vec
headers; O(64n) build scans, O(n) temporary u64 storage (two arrays), O(64)
rank/kth/less queries and O(128) range_count. For n=1,000,000 on 64-bit,
prefix payload is 512,000,512 bytes excluding allocator/Vec overhead.
Not a succinct bitvector implementation. Query allocates nothing; cloning
copies all prefix storage. Allocation failure and panic are outside Option.
No measured speedup or general timing guarantee is claimed.

## behavior-complement
Type behavior_complement. Universe U pairs snapshots, queries and candidate
observations. R is the exact forward relation. An inner excluded subset of
U minus R includes Some for invalid ranges, absent counts for valid empty
ranges, wrong duplicate multiplicity and signed rather than unsigned order.
The implementation model is conditional on successful allocation; finite
checks do not prove every n or allocator behavior.

## reverse-reconstruction
Type reverse_reconstruction. For fixed interval and count c at value x,
retain every snapshot with exactly c copies of x inside, arbitrary other
values and arbitrary outside values. For kth output x, retain snapshots
with count(v<x)<=k<count(v<=x). Stable original order remains ambiguous.
Combined rank observations for every value reconstruct a multiset, not an
ordered sequence. None restricts endpoints/k to the invalid class for known n.

## prefix-conservation
Type partition_conservation. For each row, zero multiplicity plus one
multiplicity equals b-a. Branch intervals never overlap within a partition
and remain within 0..n. Stable filtering preserves original subinterval
membership after prefix selection. Prefix increment tests inspect stored rows.

## unsigned-order
Type lexicographic_bit_order. Compare all 64 bits from high to low. First
differing bit determines u64 ordering, including 2^63 and MAX. No signed cast
or max+1 sentinel occurs. MAX cannot be an inclusive upper bound in the
half-open value API; exact MAX rank remains available.

## quantile-fiber
Type order_statistic_bounds. kth x implies less(x)<=k<less(x)+rank(x).
Duplicates occupy repeated positions, without selecting a stable original
index. Switching the zero branch from k<z to k<=z violates this relation
at [0,1], k=1; the compiled control must reject it.

## immutable-snapshot
Type snapshot_frame. Construction copies values, retaining only derived
prefix rows. Later caller mutation cannot change answers; queries borrow
immutably and do not allocate. Clone owns an independent prefix structure.

## tests
Four tests cover exhaustive ternary arrays of lengths 0..5, all intervals,
all kth positions plus missing kth, selected thresholds including unsigned
extremes, reversed values, invalid/extreme usize endpoints, duplicates and
caller mutation. See model.md#evidence for repeated profiles and mutation logs.

## descriptant
Composition: construction's stable partition invariant is the premise of
all three traversal loops; range_count composes two count_less calls under
ordered thresholds. Numeric equality of global multisets alone cannot
establish range mapping, so tests enumerate interior subranges. syntax.json
is a source-bound partial syntax projection, not semantic AST/proof.
No executable numerical approximant is provided: exact discrete rank and
selection observations are the useful contract; approximate replacements
and error-bound composition remain explicit gaps.

## CPU measurement epoch wave18

For 256 long-range kth queries on 8192 u64 values, repeated four times, query-only execution is 47.54–49.34x faster than cloning and sorting every range. Including a fresh index build and drop for each repetition reduces this to 1.989–1.994x. The O(64n) build term materially changes the conclusion; one short query and dynamic workloads are not measured. See model.md#benchmark-wave18. These measurements calibrate constants on fixed fixtures; they do not prove an asymptotic bound or justify automatic production substitution.

## evidence

### Evidence

4 tests pass twice debug and twice release. Normalized named outcomes match within each profile. Compiled control kth_zero_branch_control: k<z changed to k<=z fails by named assertion (exit 101), then source is restored byte-for-byte and all focused tests pass. Syntax projections run twice and compare byte-for-byte.

## prospective

### Prospective model
Finite u64 snapshots and half-open index/value ranges. 64 stable binary partitions with zero-prefix counts; mapped intervals preserve prefix-class multiplicities. Rank descends exact bits; less-than accumulates zero branches; kth selects by zero count. Invalid bounds return None. Exactly 64(n+1) usize prefixes plus 64 zero totals; O(64n) construction, O(64) queries, O(n) temporary values. About 512 MB prefix payload for one million entries on 64-bit; no configured cap, allocation exhaustion outside contract. Complement excludes invalid Some and wrong unsigned order/multiplicity. Count and kth fibers admit many arrays and permutations. Exact discrete outputs: no certified numerical surrogate proposed.

## benchmark-wave18

### Bounded CPU cost observation

For 256 long-range kth queries on 8192 u64 values, repeated four times, query-only execution is 47.54–49.34x faster than cloning and sorting every range. Including a fresh index build and drop for each repetition reduces this to 1.989–1.994x. The O(64n) build term materially changes the conclusion; one short query and dynamic workloads are not measured.

## History

### 2026-10-30 · document

2026-10-30. Original implementation. Origin: Claude, Navarro and Ordóñez,
"The wavelet matrix" (2015). Prospective commitments remain in model.md#prospective.

### 2026-10-30 · evidence

Heading `### Evidence, 2026-10-30` stays in evidence as `### Evidence`.

External logs: $MOVA_DATA/runs/octopus-wave18-structures-2026-10-30/. Finite tests provide no universal proof or timing guarantee.

Source algorithms/src/wavelet_matrix/mod.rs: sha256 390d954b78becb6d0e0aab3d5a56f0395db3a18c55b5a4c185c6a129ea289481

- wavelet_matrix-debug-1.log: sha256 53b85b3440b903f66f5307cfa8d7339a541ce890b8f608f9b1ed3cfb6c0ccaa9

- wavelet_matrix-debug-2.log: sha256 8291cc72e554a5aa78186d53861a09c77640cb453110e199571024e9c94cfc99

- wavelet_matrix-release-1.log: sha256 3a0556f4fbee33f3b8a0e80fd5c701f53f8c5d47ef335509feae080673095687

- wavelet_matrix-release-2.log: sha256 2af3c5a12f51752acdf9edeaa5f3d3dcce9e432556d910896397c69db2c8edce

- wavelet_matrix-mutant.log: sha256 8c4e9579811733c20ca39e237e048bf400cef4cfa495ee789fbb99a88cf9db13

- wavelet_matrix-restored.log: sha256 ef4f497ad91b5d661bd7bca2b612dde1a46e9ebb96bce66a8bac24db0cb9309f

- syntax.json: sha256 0986dd4b720b574c6c6837a885caf7d6c830fcaa7c6d46881923768bbcebe3d1

### 2026-10-30 · prospective

Heading `### Prospective model, 2026-10-30` stays in prospective as `### Prospective model`.

### 2026-10-30 · benchmark-wave18

Heading `### Bounded CPU cost observation — 2026-10-30` stays in benchmark-wave18 as `### Bounded CPU cost observation`.

Source SHA-256 390d954b78becb6d0e0aab3d5a56f0395db3a18c55b5a4c185c6a129ea289481. Harness SHA-256 20b57c991f61192b2b05e085edf8303d56e43290fe10b7e1dfd36f10ffc9cccf. See [full fixtures, timings, limitations and prospective model](../../../../models/algorithm-library.groupant/model.md#wave18-benchmark). Two executions retain exact outputs/checksums and all seven samples per method. This is an exact-API CPU comparison, not a Rust approximation profile or Mova training claim.
