# Gate batch model

## forward

Count T in-range numbered rows, J junk, M absent paragraph labels among N. Batch rejects iff T=0 or outside nonempty or 5J>T+J or 4M>N, in that reason precedence. Blank/fence/header lines ignored. A single optional P/p prefix is stripped. Valid batches fold each paragraph in local batch order, keeping at most 6 distinct checked questions with k=accepted length+1.

## behavior-complement

Notes never remove accepted rows. Pair rejects cannot force batch rejection after batch classification. Coverage counts numbered rows even if later row checks reject them; malformed six-column rows may therefore satisfy coverage.

## reverse-reconstruction

Given accepted sequence and reasons, reconstruct compatible numbered row sequences preserving per-paragraph order and first-failure reason; input interleaving across paragraphs is lost. Ignored lines can be inserted freely; junk insertion is constrained by recorded J and threshold.

## specialization

capacity_rank: k is prefix sum of accepted indicators, not input row number; question identity is W(question).join(space).

## invariant

All accepted anchors are sorted unique local indices; accepted paragraph and question indices are 1-based. span_reason requires equal word/normalized sentence lengths and prevalidated in-range anchors; arbitrary direct callers can violate that precondition. Integer multiplication assumes representable counts; memory exhaustion and usize overflow are outside this model.

## composition

Lexical children produce distinct W and N views. Row checks consume both plus parsed anchors; batch classification precedes row checks and paragraph accumulation adds deduplication/cap. Accepted tuples cannot be fed back as full rejection logs: that projection loses information. The lexical node composes normalization followed by boundary matching; it does not identify word equality with span equality.

## cost

Let B be supplied bytes, R rows, S total paragraph sentence bytes and A answer length. Lexical normalization is O(B) scalar work/storage (lowercase expansion bounded by Unicode tables). Verbatim window comparisons cost O(total sentence tokens * answer tokens). Span matching with repeated suffix checks is conservatively O(S*A); anchors O(K log K). Batch indexing O(R log N), stored raw/column rows O(B); each row rebuilds answer views and scans relevant sentence views. No measured speed or approximation-error claim.

## epoch-memory

Prospective ratio/fold model predicted strict malformed-anchor and single-prefix repairs. Original production accepted 1,2 and PP1. These are repaired without changing signatures; valid local numbering unchanged. Unicode interval search passed independent finite enumeration, including invalid first occurrence followed by valid occurrence. A test expectation counted six rejects for eleven rows capped at six; corrected to five before final evidence, not a production bug.

## checks

Eight gate::wave17_models tests: Unicode interval oracle 341 squared = 116281 comparisons plus two overlap witnesses; batch integer-ratio oracle 220 cases; literal normalization; empty anchors; single prefix; cap/dense numbering; span/implicit composition; accepted-projection reconstruction. Synthetic Para only, no live teacher/parser/corpus/database. Debug/release twice and compiled mutations are recorded in root ../model.md#evidence. Finite evidence does not establish all Unicode/grapheme semantics or full source correctness.

## History

### 2026-10-30 · epoch-memory

2026-10-30: prospective ratio/fold model predicted strict malformed-anchor and single-prefix repairs. Original production accepted 1,,2 and PP1. These are repaired without changing signatures; valid local numbering unchanged. Unicode interval search passed independent finite enumeration, including invalid first occurrence followed by valid occurrence. A test expectation counted six rejects for eleven rows capped at six; corrected to five before final evidence, not a production bug.

### Undated · document

Source SHA-256: 09328f711abf4b6453f95bed3d873deb205c8801d939841974416fe74e3e5806. Domain: finite supplied UTF-8 strings and synthetic Para values; rule V1/V2. Observation is the operation result specified below. Mathematical descriptions are source-level exact relations under stated assumptions, not a certified implementation theorem or numerical approximant.
