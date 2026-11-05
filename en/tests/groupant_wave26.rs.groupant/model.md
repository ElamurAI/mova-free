# en-groupant-wave26

## descriptant

Original independent fixtures connecting lossless CoNLL-U representation and raw tree validation to functional graph basin coordinates. Preserve the distinction between representation refresh and graph validity. Prospective model: (see History). No parser or gate production replacement is proposed.

### Composition

## model

# en-groupant-wave26

## Domain

Finite CoNLL-U strings; exhaustive 3413 head vectors on 1..5 tokens with exactly one zero head and matching root/dep labels, plus explicit empty, malformed, multiword/empty-node and 64/65/66-token cases. Original word IDs are one-based; virtual graph vertex zero maps to itself.

## Forward

Doc::parse checks representation and refreshes typed tokens. tree_gate checks raw-head syntax, range, root labels/count, then nontermination. Independently adapt preflight-valid head vectors by successor[0]=0 and successor[i]=head_i. Functional cycle membership predicts acceptance and the least original failing word; independent bounded walks corroborate the relation.

## Invariant

The adapter preserves every one-based word/head coordinate and adds only virtual root zero. Representation acceptance is not tree validity; refreshed malformed heads can be zero while their raw strings remain invalid. Multiword and empty-node rows persist byte-for-byte but are absent from syntactic head vectors.

## Cost

Finite sum of parsing/string allocation, gate work and O(n) functional decomposition; exhaustive 3413 cases n<=5 plus fixed fixtures n<=66. Independent per-start walks use O(n^2) work; distance witnesses add at most O(n^3) small-fixture work due to adapter reconstruction. No measured throughput claim.

## nonbehavior

Type: behavior_complement.

Within admitted preflight-valid vectors, exclude a passing gate with a nonzero eventual cycle, an incorrect first failure coordinate, or a parser serialization that loses retained rows. Exclude treating refreshed numeric head zero as proof that raw HEAD was valid. Assertions constrain only finite fixtures.

## preimages

Type: reverse_reconstruction.

A pass reconstructs implementations agreeing on all finite representation, error, coordinate and graph observations; it does not uniquely recover source behavior elsewhere. A gate Ok maps to many rooted trees and many preserved comments/MWT rows. A cycle-through-word-k error maps to all preflight-valid graphs whose least nonroot-basin word is k; it does not identify the actual cycle entry or edge.

## root_basin

Type: composition_relation.

After root-count and raw-head preflight, tree_gate accepts iff every syntactic vertex shares cycle_id[0]. Since successor[0]=0, this cycle is exactly [0]. Exact first cycle error is min i>=1 with cycle_id[i]!=cycle_id[0]; tails into a nonroot cycle are included.

## independent_walk

Type: oracle_relation.

For each start, at most n head steps reach zero iff it terminates; otherwise a nonzero vertex repeats. This independent bounded-walk predicate corroborates cycle membership without using production decomposition state or tree-gate internals.

## error_order

Type: error_partition.

Doc rejects malformed field count, nonconsecutive IDs and unknown relations before a gate call. Accepted representation can still contain malformed HEAD, range errors, relation/root mismatch, zero/multiple roots or cycles. Gate preflight scans all raw rows before cycle detection; a later malformed head therefore precedes an earlier self-loop.

## discarded_rows

Type: coordinate_relation.

Doc retains MWT range and decimal empty-node rows. words indexes only syntactic rows, while head coordinates refer to one-based syntactic words, never raw row offsets. Adding ignored rows preserves gate result and decomposition while changing raw row indices.

## negative_control

Type: counterfactual_relation.

Compile a harness adapter mutant replacing extend_from_slice(heads) by extending heads mapped with saturating_sub(1). The named coordinates_first_failure_and_threshold exact adapter assertion must fail; restore byte-identical source and rerun positively.

## finite_coverage

Type: coverage_relation.

Exhaustive small vectors cover all self-loops, multiple cycles and tails compatible with one root. Fixed 64/65/66 cases cross the production gate implementation threshold. Passing does not establish large-input correctness, arbitrary parser equivalence, allocation safety or formal proof.

## Limits

No callbacks, external mutation, corpus/live evaluation or timing. Allocator failure remains process-level and untested. No numerical metric supports a 1/2/5/10% surrogate for representation/error/cycle observations; no profile is fabricated. Partial syntax omits macro expansion and semantic resolution.

## evidence

#### Evidence

Five focused tests, 3413 exhaustive head vectors plus boundary fixtures; twice debug and release with identical normalized outcomes. Compiled shifted-head adapter mutant rejected by coordinates_first_failure_and_threshold; exact restoration positive. Source SHA and twice-identical partial syntax recorded. Receipts: (see History). No corpus or timing results.

## History

### 2026-10-30 · descriptant

Heading `### Composition epoch 2026-10-30` stays in descriptant as `### Composition`.

Original independent fixtures connecting lossless CoNLL-U representation and raw tree validation to functional graph basin coordinates. Preserve the distinction between representation refresh and graph validity. Prospective model: $MOVA_DATA/runs/octopus-wave26-combinatorial-2026-10-30/en-prospective.md. No parser or gate production replacement is proposed.

### 2026-10-30 · evidence

#### Evidence

Five focused tests, 3413 exhaustive head vectors plus boundary fixtures; twice debug and release with identical normalized outcomes. Compiled shifted-head adapter mutant rejected by coordinates_first_failure_and_threshold; exact restoration positive. Source SHA and twice-identical partial syntax recorded. Receipts: $MOVA_DATA/runs/octopus-wave26-combinatorial-2026-10-30/en-*. No corpus or timing results.
