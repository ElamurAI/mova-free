# prag-fbench-counts

## model

# prag-fbench-counts

## forward
Count slot presence/nonempty verbs/Say frames and aligned equality. Discard invalid links, map both system endpoints through inverse alignment, intersect occurrence multisets by consuming gold counts. Coreference compares kth occurrence of each allowed pronoun in reading order including obliques, independently of alignment, and counts resolved head!=pronoun. overall sums triples; zero denominator yields zero.

## invariant
Duplicate links remain denominator occurrences, not deduplicated edges. First coordinated head supplies coreference identity. Prf::add retains representable-usize-sum premise; no new assertion API.

## cost
O(L log L+Q log Q) plus alignment/slot cloning; scalar PRF O(1); sums O(10).

## behavior-complement
Three predicted copies of one gold edge cannot earn three tp. Invalid targets do not count. Swapped resolutions of two identical-pronoun occurrences cannot retain both credits. Universe: bounded supplied inputs and candidate observable outputs at this source revision. Partial source-derived inner exclusions with finite witness evidence; unsampled semantics remain unknown.

## reverse-reconstruction
Equal ratios admit scaled triples; equal triples admit different entities/graphs. Zero F1 admits empty sides or disjoint roles. No unique frame or alignment reconstruction. Target is all x in the stated domain with F(x)=y, without unique recovery or an implemented complete solver.

## specialization
Old duplicate_links_are_occurrence_bounded witness yielded (3,3,1) instead of independent (1,3,1). Multiset intersection repairs inflated recall while retaining all unique-link behavior.

## composition
Children share source identity. Preserve frame indices through alignment/link mapping and batch references through prompt/gate numbering. Independent marginal inverses can lose shared correlations; overlap costs are counted once.

## checks
Six fbench and five silver wave16_models tests: finite canonical role oracle, independent ten-role counts, old-code witnesses and compiled controls. See receipts at file root.

## epoch-memory

Prospective inspection predicted predecessor defects and duplicate-edge recall inflation. Three old-code witnesses confirm these. Valid adjacency and unique links retain semantics. No score assertions or deduplication introduced. Historical metrics retain original semantics.

## limitations
Partial syntax is not expanded semantic AST; macros opaque. Scoped models and finite observations, not full-file verification or proof. No live parser, teacher, corpus, filesystem aggregate, GPU, full course, speed or approximation-cap claim. External effects and resource-extreme malformed inputs remain unexecuted.

## evidence

### Evidence scope

See root ../model.md#receipts for source and raw-log hashes. Pure supplied fixtures only; effects are source inspected.

## History

### 2026-10-30 · epoch-memory

2026-10-30: prospective inspection predicted predecessor defects and duplicate-edge recall inflation. Three old-code witnesses confirm these. Valid adjacency and unique links retain semantics. No score assertions or deduplication introduced. Historical metrics retain original semantics.
