# prag-fbench-alignment

## model

# prag-fbench-alignment

## forward
Eligibility: nonempty gold verb with matching first word OR at least two matching agent/patient/to/place entities. Weight=4*head equality+full verb equality+2*role matches+kind equality. Sort descending weight then ascending gold/system positions and greedily accept unused endpoints.

## invariant
Partial injection, not maximum-weight matching. Entity same is nonempty ID intersection and is not transitive equivalence.

## cost
O(G*S*C+P log P+P log(G+S)), O(P+G+S) memory.

## behavior-complement
Reuse of either endpoint, eligibility from one role with different verb heads, and reverse positional tie preference are excluded. Universe: bounded supplied inputs and candidate observable outputs at this source revision. Partial source-derived inner exclusions with finite witness evidence; unsampled semantics remain unknown.

## reverse-reconstruction
Mapping alone leaves many possible ranks and frames. A compatible rank trace must have each skipped earlier pair blocked by a used endpoint. Target is all x in the stated domain with F(x)=y, without unique recovery or an implemented complete solver.

## specialization
Eligibility intervention from two roles to three compiles and fails alignment_ties_and_threshold.

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
