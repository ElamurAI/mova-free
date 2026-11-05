# algorithm-banded-levenshtein

## descriptant

### Description and epoch memory

Prospective cost: O(n*min(m+1,2k+1)) time, O(min(m+1,2k+1)) space, two rows capped at 4096 cells each. Empty and difference-pruned cases are constant time. No full matrix. Standard allocator failure is not a typed error. No timings or speed claim. Retained observations: band width budgets differ from semantic thresholds; unsigned borrow must wrap. Empty and arbitrary byte inputs remain lawful. Classical references in mod.rs: Ukkonen (1985) for diagonal thresholding; Allison and Dix (1986) for bit-string LCS. No copied implementation.

Call-level observation is an exact scalar/typed error. This source-derived specification is checked finitely, not formally proved. Allocator failure, process interruption and timing are outside the model.

Partial syntax excludes macro expansion and semantic resolution. Structural checks and finite fixtures do not establish a general proof. No approximant-error cap or automatic substitution is claimed.

## approximant

### Forward

Let d be unit-cost byte Levenshtein distance, n=max lengths, m=min lengths. If n-m>t return None; if m=0 return Some(n). Otherwise k=min(t,n), w=min(m+1,2k+1) using saturation. If w>4096 return BandTooWide{cells:w}; else return Some(d) iff d<=t, None otherwise.

### Invariant

Retained cells equal min(k+1,D(i,j)) for j=max(0,i-k)..min(m,i+k). Every path costing at most k stays in this band. Length pruning guarantees nonempty bands and final-column membership. A row uses at most min(m+1,2k+1) cells.

### Cost

O(n*min(m+1,2k+1)) time, O(min(m+1,2k+1)) space, two rows capped at 4096 cells each. Empty and difference-pruned cases are constant time. No full matrix. Standard allocator failure is not a typed error.

Call-level observation is an exact scalar/typed error. This source-derived specification is checked finitely, not formally proved. Allocator failure, process interruption and timing are outside the model.

## models

# Typed models

Every view binds the source revision in manifest.json. Assumption: ordinary completed calls. Call-level observation is an exact scalar/typed error. This source-derived specification is checked finitely, not formally proved. Allocator failure, process interruption and timing are outside the model.

## behavior_complement

Universe: finite byte pairs, usize thresholds and Result<Option<usize>,BandTooWide>. Exclude Some(v) unless v=d<=t, exclude None when d<=t and no resource error applies. Exclude resource errors unless m>0, n-m<=t and w>4096; exclude successful outcomes under that precise resource predicate. Resource rejection is not evidence that d>t. Status: concrete source-derived complement specification, with finite correspondence evidence only.

## reverse_reconstruction

Some(v) retains every pair whose distance is v<=t and whose call is not a budget error. None retains all pairs with d>t outside the resource-error predicate. Error w retains all inputs satisfying the exact width predicate. ab/ac and xy/xz at t=1 both map to Some(1); None merges distinct distances. Lengths, bytes and edit scripts cannot be reconstructed uniquely. No complete executable reconstructor exists beyond finite oracle enumeration.

## boundary_partition

At fixed identical 4096-byte inputs, t=0 yields Some(0), but usize::MAX yields BandTooWide{4097}: resource errors break unconditional threshold monotonicity. A deleted prefix abc/bc at t=1 touches the band edge. Empty inputs bypass allocation; difference pruning precedes the cap. Finite oracle: the named boundary, resource and exhaustive tests. This specialization is a partial source-derived relation.

## resource_refusal_fiber

Fix threshold t and observe Err(BandTooWide{cells:w}). Its preimage is exactly pairs with m>0, n-m<=t and min(m+1,2min(t,n)+1)=w>4096, with arithmetic saturated as in the source. Byte equality is unrestricted: at t=usize::MAX, two identical 4096-byte strings and two disjoint constant strings both return cells=4097 despite distances 0 and 4096. Thus this error fiber carries a storage-width constraint but no equality or threshold-failure evidence. Assumption: finite valid slices. Source-derived exact relation, not an executable reconstructor. Falsifiable oracle: narrow_band_resource_boundary_and_pruning fixes the identical-string error and verifies the same input succeeds at t=0.


## length_pruning_preimage

Hold lengths n>=m and t fixed. If n-m>t every byte assignment belongs to the Ok(None) fiber before allocation; this follows from the insertion/deletion lower bound. The converse is false: equal-length ab/cd at t=0 also returns None. Observing None therefore cannot identify whether length pruning or a computed distance caused refusal. On m=0, n>t gives None and n<=t gives Some(n), so the length bound fully determines this subdomain. Source-derived partition; exhaustive_distance_and_thresholds and empty fixtures compare this partition against an independent full matrix.


## tests

### Native evidence

Four named tests per module, twice debug and twice release with identical normalized outcomes. Independent full DP oracle over 3969 pairs per algorithm, plus byte, empty, threshold, cap and bit63 fixtures. Two compiled production mutants invert substitution equality and remove the bitset complement; named boundary assertions fail. Exact restoration precedes positive runs. Strict all-target Clippy and narrow formatting pass. receipt.json binds source and raw evidence hashes. Partial syntax and structural checks run twice. No live parser, teacher, corpus, GPU or performance run.

#### Wave16 integrated CPU timing

The [bounded timing receipt](../../../../models/algorithm-library.groupant/model.md#wave16-string-timings) records two external executions against the existing exact DP reference, source/binary/log hashes and unchanged outputs. Its fixed synthetic domains do not establish training or whole-system throughput.

## History

### 2026-10-30 · descriptant

2026-10-30: original implementation and independent matrix oracle. Let d be unit-cost byte Levenshtein distance, n=max lengths, m=min lengths. If n-m>t return None; if m=0 return Some(n). Otherwise k=min(t,n), w=min(m+1,2k+1) using saturation. If w>4096 return BandTooWide{cells:w}; else return Some(d) iff d<=t, None otherwise.

Model refinement, 2026-10-30: two additional scoped relations distinguish observation loss, validation or objective changes using the existing fixtures. This file now declares five typed views. Source, native checks and partial syntax are unchanged; the added declarations are not new executed tests or a proof.

### 2026-11-01 · length_pruning_preimage

#### Threshold transition within the resource fiber (2026-11-01)

For finite slices with n>=m>=4096 and t>=n-m, the error predicate simplifies to t>=2048: n is at least 4096, so min(t,n)>=2048 exactly when t>=2048, and both arguments of min(m+1, 2k+1) then exceed 4096. Saturation does not change this comparison. If m<=4095, the width cannot exceed the cap. This is a statement about the resource error only, not a converse from `None` or a claim about computed distances. At n=m=4096, identical bytes return Some(0) at 2047 and error 4097 at 2048; disjoint constant bytes return None then the same error. Thus increasing a threshold can replace either successful variant with an error. An external actual-source Rust probe checked these cases twice on 2026-11-01; it also checked the m=4095 boundary. The algebra is source-derived and has not been formally verified. Allocation failure and interrupted calls are excluded.
