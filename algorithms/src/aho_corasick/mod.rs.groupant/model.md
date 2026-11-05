# algorithm-aho-corasick

## descriptant

### aho_corasick mod.rs epoch memory

Four tests: 225 ordered pattern pairs, each with duplicate first pattern, across 127 binary texts (28,575 oracle comparisons); full byte range; failure-output/empty/duplicate boundaries; injective relabel and ID permutation; five compatible predecessors and prefix extension. Compiled production mutant removing inherited outputs fails boundaries_failure_outputs_and_duplicates.

The combiner relates authoritative source and its SHA-256, partial syntax, forward relation, typed obernants, invariant, cost and finite evidence. Models preserve ambiguity in compatible predecessors. Missing semantic AST analysis, universal proof, allocator exhaustion and downstream composition remain explicit. Exact production algorithms need no fabricated executable approximation. Any future approximate profile must honor supported 1%, 2%, 5%, 10% caps and bind source and evidence.

## model

# aho_corasick mod.rs model

## Forward

Build an owned trie with breadth-first failure links and inherited sorted output IDs. Scan arbitrary bytes and emit every exact occurrence, including overlaps, in (end,pattern) order. Duplicates preserve IDs; empty patterns match all boundaries.

## Invariant

Trie labels are distinct prefixes; each nonroot failure points to a shorter proper suffix prefix. BFS initializes failure ancestors first. Sorted output IDs exactly match suffix patterns, including empty IDs once. State is longest prefix suffix of consumed text.

## Cost

Let M=sum pattern lengths, p=pattern count, L=max length, H=total stored output IDs and o_s=outputs at state s. Construction O(p+M L log 256+H+sum_s o_s log o_s), storage O(M+H+p). Scan O(n log 256+z) time, O(z) returned output; copied inherited outputs can be superlinear in M. No timing measurement.

## nonbehavior

Type: behavior_complement. Domain: Finite byte slices and ordinary terminating executions with successful allocation.

Exclude missing or spurious occurrences, duplicate reports of the same (end,pattern), out-of-range offsets, wrong byte spans, changed inputs, and an output not ordered by (end,pattern). Empty-pattern omissions and deduplication of equal pattern values are outside the contract.

Source-derived partial relation with finite evidence, not universal verification.

## preimages

Type: reverse_reconstruction. Domain: Finite byte slices and ordinary terminating executions with successful allocation.

For fixed patterns P and length n, restrict R to valid pattern IDs and ranges 0<=s<=e<=n, unique (end,pattern) pairs, and canonical increasing (end,pattern) order. Such an output R admits exactly byte texts T of length n for which each (i,s,e) in R has e-s=|P_i| and T[s..e]=P_i, and every matching span is in R. These positive and negative span constraints generally admit several texts; no unique reconstruction is claimed.

Source-derived partial relation with finite evidence, not universal verification.

## state-suffix

Type: state_preimage. Domain: Finite byte slices and ordinary terminating executions with successful allocation.

After consuming T[..e], state represents the longest suffix of T[..e] which is a trie prefix. Compatible histories are precisely histories with this longest-prefix-suffix property, not histories with only the same final byte.

Source-derived partial relation with finite evidence, not universal verification.

## output-fiber

Type: conservation_relation. Domain: Finite byte slices and ordinary terminating executions with successful allocation.

Outputs at a state are exactly IDs whose patterns are suffixes of that state's trie label. Each pattern ID occurs once; distinct IDs with equal bytes coexist. Root outputs are exactly empty IDs.

Source-derived partial relation with finite evidence, not universal verification.

## relabel

Type: metamorphic_relation. Domain: Finite byte slices and ordinary terminating executions with successful allocation.

Every injective byte map applied jointly to patterns and text preserves the entire match list. Noninjective maps can introduce occurrences and are excluded.

Source-derived partial relation with finite evidence, not universal verification.

## pattern-permutation

Type: equivariance_relation. Domain: Finite byte slices and ordinary terminating executions with successful allocation.

Permuting pattern IDs transports each report's pattern field by the same permutation, then sorting by (end,pattern) restores required order. Repeated byte patterns remain separate IDs.

Source-derived partial relation with finite evidence, not universal verification.

## extension

Type: prefix_relation. Domain: Finite byte slices and ordinary terminating executions with successful allocation.

For text U followed by V, filtering matches on UV to end<=|U| gives exactly matches on U, including empty matches at the shared boundary only once.

Source-derived partial relation with finite evidence, not universal verification.

## empty-boundaries

Type: cardinality_relation. Domain: Finite byte slices and ordinary terminating executions with successful allocation.

With e empty pattern IDs and text length n, empty reports number e(n+1); all other reports have start<end. No patterns yields an empty result on every text.

Source-derived partial relation with finite evidence, not universal verification.

## evidence

### Evidence

Four tests: 225 ordered pattern pairs, each with duplicate first pattern, across 127 binary texts (28,575 oracle comparisons); full byte range; failure-output/empty/duplicate boundaries; injective relabel and ID permutation; five compatible predecessors and prefix extension. Compiled production mutant removing inherited outputs fails boundaries_failure_outputs_and_duplicates.

Debug and release each ran twice with identical normalized per-test outcomes (durations excluded). Strict crate Clippy with all targets and denied warnings, owned rustfmt, and four twice-generated identical source-bound syntax projections passed. All 175 crate tests also passed once with temporary module declarations. Mutants restored before hashing; no shared lib or registry edits committed. No timing, allocator-exhaustion or universal proof claim.

## prospective

### Prospective

Repository and family QA source search found no existing matching API. Original std-only implementation from classical algorithm ideas, no copied source. Before implementation: Build a byte trie with failure links and inherited output IDs. Empty patterns match all n+1 boundaries, duplicates retain distinct IDs. Return matches in (end, pattern) order. Invariant: state is the longest trie-prefix suffix of consumed text. Sparse BTreeMap edges; construction bounded by O(M L log 256 + H + sum_s o_s log o_s), storage O(M+H), scan O(n log 256 + z); H counts inherited output entries, L maximum pattern length. Exhaustive independent byte oracle, boundaries, metamorphic and compatible-preimage checks; debug/release twice, strict Clippy, source-bound syntax twice and a compiled mutant. No timing or universal proof claim.

## History

### 2026-10-30 · descriptant

2026-10-30. Original std-only code, no copied source. Prospective model preceded code in model.md#prospective. Classical citations appear in implementation documentation; variant complexity is explicit.

### 2026-10-30 · evidence

Heading `### Evidence 2026-10-30` stays in evidence as `### Evidence`.

### 2026-10-30 · prospective

Heading `### Prospective epoch 2026-10-30` stays in prospective as `### Prospective`.

### 2026-11-01 · descriptant

2026-11-01 independent review: the prefix projection survives a split, but independently concatenated match lists lose cross-cut occurrences and duplicate empty matches at the shared boundary. The source-linked finite oracle and negative control are described in model.md#evidence; a full streaming composition and universal proof remain open.

### 2026-11-01 · empty-boundaries

#### Split-boundary observation (2026-11-01)

For finite byte slices and successful allocation, the existing `extension` relation retains the reports ending at or before the cut. It does not say that independently scanning both parts and concatenating their offset-adjusted reports reconstructs the full list. With patterns `[ba, empty, empty]`, split `b | a` loses the `ba` occurrence and repeats each empty-pattern report at the shared boundary. A correct composition needs cross-boundary state and a boundary convention. This is a distinction between a prefix projection and a full-list composition, not a new inverse or an approximation error bound.

### 2026-11-01 · evidence

#### Independent split review 2026-11-01

A source-linked external Rust probe compared the public API with direct byte-slice enumeration for 961 pairs of binary texts of length at most four, using five pattern IDs including duplicates and two empty patterns. It also checked the prefix projection for each split. Two executions produced identical output. The separate `b | a` control rejects independent-list concatenation: it omits the cross-cut `ba` and duplicates both empty IDs at the cut. Four existing focused tests passed. This is bounded evidence, not a universal proof of suffix-state behavior or a complete converse reconstruction.
