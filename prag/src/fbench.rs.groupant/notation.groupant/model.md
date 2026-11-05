# prag-fbench-notation

## model

# prag-fbench-notation

## forward
Trim initial numbering and normalize two dash forms; split spaced bar; recognize Event/Bare/Fact(/Say( prefix. Parse agent, pre-key verb with NOT removed, key/value spans and numeric links. norm_id lowercases and keeps final space/underscore word. Entities split '+' and preserve pronoun annotation.

## invariant
Not strict validation: repeated singular keys overwrite; unknown keys are obliques; invalid numeric links drop; unterminated recognized kind prefixes may survive. Pronoun-only entity falls back to pronoun ID.

## cost
O(B) scanning/copying, O(B) strings for bounded valid UTF-8 bytes B.

## behavior-complement
Canonical fixture excludes retaining the first multiword-name component or interpreting malformed #x as a valid target. Unknown kind lines return None; malformed recognized prefixes are not certified syntax. Universe: bounded supplied inputs and candidate observable outputs at this source revision. Partial source-derived inner exclusions with finite witness evidence; unsampled semantics remain unknown.

## reverse-reconstruction
Case, numbering, spacing, malformed links and name prefixes may vary in the same parsed-frame fiber. Original bytes cannot be uniquely recovered. Target is all x in the stated domain with F(x)=y, without unique recovery or an implemented complete solver.

## specialization
Field transport: attrs/negation/quoted/condition survive parsing although role metrics omit them.

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
