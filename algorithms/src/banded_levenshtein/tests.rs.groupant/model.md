# algorithm-banded-levenshtein-tests

## descriptant

### Description and epoch memory

Prospective cost: O(n*min(m+1,2k+1)) time, O(min(m+1,2k+1)) space, two rows capped at 4096 cells each. Empty and difference-pruned cases are constant time. No full matrix. Standard allocator failure is not a typed error. No timings or speed claim. Retained observations: band width budgets differ from semantic thresholds; unsigned borrow must wrap. Empty and arbitrary byte inputs remain lawful. Classical references in mod.rs: Ukkonen (1985) for diagonal thresholding; Allison and Dix (1986) for bit-string LCS. No copied implementation.

Test-level observation: four named assertion outcomes. Each module enumerates 3969 ordered pairs of binary words through length five against independently allocated full DP matrices. Distance checks eight thresholds per pair, LCS also swaps inputs. Oracle cost O(n*m) time and space per pair. The finite transcript cannot identify an implementation: untested inputs may differ. Excluded passes include disagreement with fixed oracle and literal boundary fixtures; failed assertions do not identify a unique defect.

Partial syntax excludes macro expansion and semantic resolution. Structural checks and finite fixtures do not establish a general proof. No approximant-error cap or automatic substitution is claimed.

## approximant

### Forward

Let d be unit-cost byte Levenshtein distance, n=max lengths, m=min lengths. If n-m>t return None; if m=0 return Some(n). Otherwise k=min(t,n), w=min(m+1,2k+1) using saturation. If w>4096 return BandTooWide{cells:w}; else return Some(d) iff d<=t, None otherwise.

### Invariant

Retained cells equal min(k+1,D(i,j)) for j=max(0,i-k)..min(m,i+k). Every path costing at most k stays in this band. Length pruning guarantees nonempty bands and final-column membership. A row uses at most min(m+1,2k+1) cells.

### Cost

O(n*min(m+1,2k+1)) time, O(min(m+1,2k+1)) space, two rows capped at 4096 cells each. Empty and difference-pruned cases are constant time. No full matrix. Standard allocator failure is not a typed error.

Test-level observation: four named assertion outcomes. Each module enumerates 3969 ordered pairs of binary words through length five against independently allocated full DP matrices. Distance checks eight thresholds per pair, LCS also swaps inputs. Oracle cost O(n*m) time and space per pair. The finite transcript cannot identify an implementation: untested inputs may differ. Excluded passes include disagreement with fixed oracle and literal boundary fixtures; failed assertions do not identify a unique defect.

## models

# Typed models

Every view binds the source revision in manifest.json. Assumption: ordinary completed calls. Test-level observation: four named assertion outcomes. Each module enumerates 3969 ordered pairs of binary words through length five against independently allocated full DP matrices. Distance checks eight thresholds per pair, LCS also swaps inputs. Oracle cost O(n*m) time and space per pair. The finite transcript cannot identify an implementation: untested inputs may differ. Excluded passes include disagreement with fixed oracle and literal boundary fixtures; failed assertions do not identify a unique defect.

## behavior_complement

Universe: finite byte pairs, usize thresholds and Result<Option<usize>,BandTooWide>. Exclude Some(v) unless v=d<=t, exclude None when d<=t and no resource error applies. Exclude resource errors unless m>0, n-m<=t and w>4096; exclude successful outcomes under that precise resource predicate. Resource rejection is not evidence that d>t. Status: concrete source-derived complement specification, with finite correspondence evidence only.

## reverse_reconstruction

Some(v) retains every pair whose distance is v<=t and whose call is not a budget error. None retains all pairs with d>t outside the resource-error predicate. Error w retains all inputs satisfying the exact width predicate. ab/ac and xy/xz at t=1 both map to Some(1); None merges distinct distances. Lengths, bytes and edit scripts cannot be reconstructed uniquely. No complete executable reconstructor exists beyond finite oracle enumeration.

## boundary_partition

At fixed identical 4096-byte inputs, t=0 yields Some(0), but usize::MAX yields BandTooWide{4097}: resource errors break unconditional threshold monotonicity. A deleted prefix abc/bc at t=1 touches the band edge. Empty inputs bypass allocation; difference pruning precedes the cap. Finite oracle: the named boundary, resource and exhaustive tests. This specialization is a partial source-derived relation.

## threshold_oracle_discrimination

For each of 3969 binary-word pairs let d be the independent matrix distance. The assertion vector over t=0..6 and usize::MAX must equal [Some(d) if d<=t else None]. Since both lengths are at most five, every case fits the resource budget and every d is at most five. This finite vector uniquely identifies d for that pair: its first Some occurs at t=d; two distinct candidate distances cannot share the vector. This completeness applies only to the enumerated domain. A false strict threshold d<t fails the t=d coordinate, while a saturated result min(d,t) fails every coordinate t<d. Source-derived finite discrimination relation; named oracle is exhaustive_distance_and_thresholds.


## edit_objective_counterfactual

Hold the literal fixture abc/bc fixed: unit insertion/deletion/substitution distance is one, so a Hamming-only objective that requires equal lengths cannot satisfy its asserted Some(1). Hold ab/ba fixed: allowing an adjacent transposition for cost one changes the expected distance from two to one. The boundary and fiber tests therefore distinguish these changed objectives from unit Levenshtein on these witnesses; a pass is not evidence that all alternative edit objectives are excluded. Domain is these named supplied byte pairs, observation is their assertion outcomes. Falsifiable oracles: unit_cost_boundaries_and_byte_semantics and fibers_and_false_oracle.


## tests

### Native evidence

Four named tests per module, twice debug and twice release with identical normalized outcomes. Independent full DP oracle over 3969 pairs per algorithm, plus byte, empty, threshold, cap and bit63 fixtures. Two compiled production mutants invert substitution equality and remove the bitset complement; named boundary assertions fail. Exact restoration precedes positive runs. Strict all-target Clippy and narrow formatting pass. receipt.json binds source and raw evidence hashes. Partial syntax and structural checks run twice. No live parser, teacher, corpus, GPU or performance run.

## History

### 2026-10-30 · descriptant

2026-10-30: original implementation and independent matrix oracle. Let d be unit-cost byte Levenshtein distance, n=max lengths, m=min lengths. If n-m>t return None; if m=0 return Some(n). Otherwise k=min(t,n), w=min(m+1,2k+1) using saturation. If w>4096 return BandTooWide{cells:w}; else return Some(d) iff d<=t, None otherwise.

Model refinement, 2026-10-30: two additional scoped relations distinguish observation loss, validation or objective changes using the existing fixtures. This file now declares five typed views. Source, native checks and partial syntax are unchanged; the added declarations are not new executed tests or a proof.

### 2026-11-01 · document

#### Passing transcript is a partial exclusion (review 2026-11-01)

The behavior complement above describes the intended distance API, not what this test file alone excludes. For the observation consisting of all four test outcomes, let D be the real API and M agree with D except M([42],[43],1)=Ok(Some(0)). The binary exhaustive calls use only bytes 0 and 255; none of the literal calls is that triple. The actual test source passes against M, while D([42],[43],1)=Ok(Some(1)). Thus the passing-transcript fiber contains at least these two distinct functions. This is a finite intervention, not a general characterization of the fiber; a failure similarly need not identify the faulty operation. The converse is a set of compatible implementations, not a unique reconstruction, and this observation gives no approximation error bound.
