# algorithm-lz77

## descriptant

### Memory

Original bounded byte lz77. Reference: Ziv and Lempel (1977), IEEE Transactions on Information Theory 23(3), A Universal Algorithm for Sequential Data Compression. Prospective equations preceded implementation. Exact semantics and observed checks are distinct from syntax binding. Round-trip cannot alone validate two mutually wrong codecs; retain hand-derived streams and malformed checks. No timing claim or percentage approximant.

## model

### Mathematical combiner

For source S, partial syntax P(S), forward relation F and typed relations O, the sidecar records (hash(S),P(S),F,O,I). Source identity binds descriptions to bytes but does not prove F. Tests intersect compatible implementations with finite observations. Composition with callers requires domain caps and exact model identity; no global Mova accuracy claim follows. aaaa has distinct valid encodings four literals or Literal(a),Match(1,3); original stream is not uniquely recoverable from decoded bytes. Canonical encoding depends additionally on window and match cap.

## approximant

### Forward, invariant, and cost

At position i search distances 1..min(i,window), maximize matching prefix <=max_match. Strict improvement retains smallest-distance ties. Emit match for length>=3 else literal. Decoder reads each byte from current output length-distance, permitting overlap.

O(n*window*max_match) encode, O(tokens+output) decode, O(n) output storage.

Input/token cap 65536 first; encode window/max_match must be 1..4096. Decode validates match before remaining-budget subtraction. Even usize::MAX length is rejected without overflow; cap min(user,65536). Allocations may panic/abort.

Invariant: A decoded output whose Match bytes fail the recursive backward-copy relation is excluded. Zero distance, distance beyond prefix, or zero length cannot succeed.

Original implementation informed by Ziv and Lempel (1977), IEEE Transactions on Information Theory 23(3), A Universal Algorithm for Sequential Data Compression; no copied implementation. Exact categorical semantics; no useful certified percentage surrogate is attached. Patience minimality is explicitly unsupported. Finite fixtures do not constitute universal proof.

## models

# Typed relations

## behavior_complement
A decoded output whose Match bytes fail the recursive backward-copy relation is excluded. Zero distance, distance beyond prefix, or zero length cannot succeed.

## reverse_reconstruction
aaaa has distinct valid encodings four literals or Literal(a),Match(1,3); original stream is not uniquely recoverable from decoded bytes. Canonical encoding depends additionally on window and match cap.

## transition_relation
At position i search distances 1..min(i,window), maximize matching prefix <=max_match. Strict improvement retains smallest-distance ties. Emit match for length>=3 else literal. Decoder reads each byte from current output length-distance, permitting overlap.

## finite_arithmetic
Input/token cap 65536 first; encode window/max_match must be 1..4096. Decode validates match before remaining-budget subtraction. Even usize::MAX length is rejected without overflow; cap min(user,65536). Allocations may panic/abort.

## resource_observation
O(n*window*max_match) encode, O(tokens+output) decode, O(n) output storage. No callbacks (count zero), no IO, no randomized state. Input errors are values; allocator failures and stack exhaustion are environmental and not caught.

## deterministic_ties
Traversal is increasing index; maps are ordered. Myers and decoders have no optimization ties; LZW freezes dictionary rather than clearing it; LZ77 keeps nearest distance; BPE keeps lexicographically smallest pair; patience LIS keeps earliest predecessor and endpoint. These are per-algorithm rules, not arbitrary hash iteration.

## tests

Run cargo test -p mova-algorithms lz77::tests in debug/release twice. Preserve normalized transcripts, compiled negative control, and exact restoration under (see History). No benchmark evidence yet.

### Checks

## prospective

### Prospective model
Original bounded byte lz77 implementation. Before implementation: exact transform, deterministic traversal, no callbacks; input cap 65536 (diff 4096), dictionary 4096, output cap min(caller,65536). Allocation may panic/abort; resource checks prevent size-driven overflow but do not certify allocator success. Myers uses bit-vector edit row recurrence with wrapping addition; LZW extends previous phrase by next first byte, freezing at dictionary capacity; LZ77 greedy longest previous match permits overlap; BPE repeatedly merges most frequent pair, lexicographically smallest tie, applying learned merges in order; patience diff recursively anchors unique shared bytes by increasing subsequence and uses delete/insert fallback. Expected costs respectively O(n), O(n * phrase length), O(n*window*match), O(merges*n*log n), O(n^3) conservative. Correctness is exact distance or lossless reconstruction; patience edit count is not guaranteed minimal. No percentage approximant justified. Independent DP/exhaustive and round-trip/malformed checks planned; no timing claim.

## validation

### Observed validation

The five-module batch adds 14 native tests. All pass twice debug and twice release; sorted named test records match across all four runs. Initial integrated library run: 632 passed. Strict library Clippy passed. Each production file has a compiled mutant rejected by a named assertion, followed by byte-for-byte restoration and a passing module rerun: Myers score+1/exhaustive_dp; LZW flipped decoded byte/known_and_malformed; LZ77 match length-1/greedy_overlap_and_invalid; BPE tie comparison >=/ties_and_model_identity; patience anchor Insert instead of Equal/unique_anchor_and_nonminimal_witness.

## History

### 2026-10-30 · descriptant

Heading `### Epoch memory — 2026-10-30` stays in descriptant as `### Memory`.

### 2026-10-30 · tests

Run cargo test -p mova-algorithms lz77::tests in debug/release twice. Preserve normalized transcripts, compiled negative control, and exact restoration under $MOVA_DATA/runs/octopus-wave25-sequences-2026-10-30. No benchmark evidence yet.

### 2026-10-30 · prospective

Heading `### Prospective model — 2026-10-30` stays in prospective as `### Prospective model`.

### 2026-10-30 · validation

Heading `### Observed validation — 2026-10-30` stays in validation as `### Observed validation`.

Each of ten source/test files has paired identical partial syntax projections, refreshed after all mutants were restored. Receipts: $MOVA_DATA/runs/octopus-wave25-sequences-2026-10-30/{focused-debug-1.log,focused-debug-2.log,focused-release-1.log,focused-release-2.log,normalized-*,mutations.log,mutant-*,restored-*,syntax.log,clippy.log}. No timing or speedup evidence; allocator failures untested. These finite observations and lexical source bindings do not establish a semantic proof or global Mova quality improvement.
