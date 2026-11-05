# Gate rows model

## forward

Validated anchors map comma tokens to an ascending set, reject empty components/duplicates/out-of-range. check rejects first failing column count, enum, anchors, question/answer words, implicit verbatim, V2 explicit span predicate. span_reason priority: anchored span, nonanchor span, anchored word-only match, generic absence.

## behavior-complement

Implicit answer copied from any paragraph sentence is excluded even when that sentence is not an anchor. V2 explicit answer from only a nonanchor is excluded. V1 explicit spans are unconstrained.

## reverse-reconstruction

Row success fiber consists of six-column rows whose parsed type/explicitness/anchor set and trimmed question/answer equal the tuple. Anchor permutations and outer whitespace give alternate witnesses. Reason fibers must also satisfy absence of every earlier rejection condition.

## specialization

anchor_domain: Result<SortedUniqueLocalSentenceIndices,Reason>; local indices refer to Para.sents order, never Sent.sent_id or Para.para.

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
