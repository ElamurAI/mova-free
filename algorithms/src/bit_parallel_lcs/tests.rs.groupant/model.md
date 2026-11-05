# algorithm-bit-parallel-lcs-tests

## descriptant

### Description and epoch memory

Prospective cost: O(256+n+m) time and 256 u64 masks plus constant state, no heap allocation. Shorter input is at most 64; longer side has no arbitrary cap. No timings or speed claim. Retained observations: band width budgets differ from semantic thresholds; unsigned borrow must wrap. Empty and arbitrary byte inputs remain lawful. Classical references in mod.rs: Ukkonen (1985) for diagonal thresholding; Allison and Dix (1986) for bit-string LCS. No copied implementation.

Test-level observation: four named assertion outcomes. Each module enumerates 3969 ordered pairs of binary words through length five against independently allocated full DP matrices. Distance checks eight thresholds per pair, LCS also swaps inputs. Oracle cost O(n*m) time and space per pair. The finite transcript cannot identify an implementation: untested inputs may differ. Excluded passes include disagreement with fixed oracle and literal boundary fixtures; failed assertions do not identify a unique defect.

Partial syntax excludes macro expansion and semantic resolution. Structural checks and finite fixtures do not establish a general proof. No approximant-error cap or automatic substitution is claimed.

## approximant

### Forward

Select shorter s and longer l, choosing first on ties. If |s|>64 return ShortSideTooLong{|s|}; otherwise return the exact maximum length of any shared byte subsequence. Empty slices return zero.

### Invariant

After a prefix of l, bit j is LCS(s[..j+1],prefix)-LCS(s[..j],prefix). Popcount gives the final row score. For byte mask M, x=M|state, y=(state<<1)|1; update state=x & !(x-y) with unsigned modulo-2^64 subtraction. x excludes excess high bits. Masks shift by 0..63; state shifts by one.

### Cost

O(256+n+m) time and 256 u64 masks plus constant state, no heap allocation. Shorter input is at most 64; longer side has no arbitrary cap.

Test-level observation: four named assertion outcomes. Each module enumerates 3969 ordered pairs of binary words through length five against independently allocated full DP matrices. Distance checks eight thresholds per pair, LCS also swaps inputs. Oracle cost O(n*m) time and space per pair. The finite transcript cannot identify an implementation: untested inputs may differ. Excluded passes include disagreement with fixed oracle and literal boundary fixtures; failed assertions do not identify a unique defect.

## models

# Typed models

Every view binds the source revision in manifest.json. Assumption: ordinary completed calls. Test-level observation: four named assertion outcomes. Each module enumerates 3969 ordered pairs of binary words through length five against independently allocated full DP matrices. Distance checks eight thresholds per pair, LCS also swaps inputs. Oracle cost O(n*m) time and space per pair. The finite transcript cannot identify an implementation: untested inputs may differ. Excluded passes include disagreement with fixed oracle and literal boundary fixtures; failed assertions do not identify a unique defect.

## behavior_complement

Universe: finite byte pairs and Result<usize,ShortSideTooLong>. Exclude success iff shorter length>64; otherwise exclude errors and every returned q differing from the maximum shared-subsequence length. In particular q cannot exceed either length. API supplies no subsequence witness. Status: concrete source-derived complement specification, with finite correspondence evidence only.

## reverse_reconstruction

Ok(q) retains every pair with min length<=64 and LCS=q. ab/ac and xy/xz both map to one. Zero admits empty slices and disjoint nonempty alphabets. Error r retains every pair with min length=r>64. Neither bytes, matching positions, lengths nor the choice of an optimal subsequence is uniquely recoverable. No complete executable reconstructor exists beyond finite oracle enumeration.

## boundary_partition

At x=0,y=1 wrapping borrow produces u64::MAX and the update restores zero. A full matching word sets bit63 safely. abc/axbyc scores3, while ab/ba scores1, distinguishing subsequence from substring or unordered matching. UTF-8 é/è share one byte and score1. Finite oracle: the named boundary, resource and exhaustive tests. This specialization is a partial source-derived relation.

## sequence_objective_discrimination

The literal pair abc/axbyc has LCS three but longest common substring one. The pair ab/ba has LCS one but unordered multiset intersection size two. Together their assertion vector (3,1) distinguishes ordered subsequence length from both changed objectives on these fixtures. The UTF-8 é/è witness expects one shared byte, separating byte equality from Unicode scalar equality, which gives zero. Domain: these exact supplied byte encodings; observation: literal assertion vector. This is finite objective discrimination, not a universal rejection of every alternative algorithm. Falsifiable oracle: subsequence_not_substring_and_fibers.


## short_side_cap_discrimination

The domain partition is min(|a|,|b|)<=64 versus >64. The test vector includes (65,66) and (66,65), which must both error with length65; (64,100), which must return64 for all-one slices; and (0,100)/(100,0), which must return zero. A counterfactual cap on the first argument or the longer side disagrees with at least one coordinate. These assertions also distinguish rejecting at >=64 from the inclusive <=64 boundary. The finite transcript constrains cap placement and symmetry only at the listed witnesses; it does not reconstruct all validation code. Falsifiable oracle: length_cap_and_empty_domain.


## tests

### Native evidence

Four named tests per module, twice debug and twice release with identical normalized outcomes. Independent full DP oracle over 3969 pairs per algorithm, plus byte, empty, threshold, cap and bit63 fixtures. Two compiled production mutants invert substitution equality and remove the bitset complement; named boundary assertions fail. Exact restoration precedes positive runs. Strict all-target Clippy and narrow formatting pass. receipt.json binds source and raw evidence hashes. Partial syntax and structural checks run twice. No live parser, teacher, corpus, GPU or performance run.

## History

### 2026-10-30 · descriptant

2026-10-30: original implementation and independent matrix oracle. Select shorter s and longer l, choosing first on ties. If |s|>64 return ShortSideTooLong{|s|}; otherwise return the exact maximum length of any shared byte subsequence. Empty slices return zero.

Model refinement, 2026-10-30: two additional scoped relations distinguish observation loss, validation or objective changes using the existing fixtures. This file now declares five typed views. Source, native checks and partial syntax are unchanged; the added declarations are not new executed tests or a proof.
