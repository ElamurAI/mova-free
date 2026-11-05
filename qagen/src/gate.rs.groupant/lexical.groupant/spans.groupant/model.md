# Gate spans model

## forward

For nonempty a, B(a,s)=exists scalar interval [i,j): s[i..j]=a and each adjacent scalar, when present, is nonalphanumeric. Implementation scans matches including overlapping occurrences.

## behavior-complement

Excluded relation is empty a or every equal interval having an alphanumeric neighbor. One invalid early occurrence cannot exclude a later valid occurrence.

## reverse-reconstruction

Given true, compatible inputs are unions of all interval witnesses; given false all intervals fail equality or boundary. Exact original spelling is not reconstructed.

## specialization

interval_witness: (String,String,Range<ScalarIndex>) with equality and two optional-neighbor constraints.

## invariant

All accepted anchors are sorted unique local indices; accepted paragraph and question indices are 1-based. span_reason requires equal word/normalized sentence lengths and prevalidated in-range anchors; arbitrary direct callers can violate that precondition. Integer multiplication assumes representable counts; memory exhaustion and usize overflow are outside this model.

## composition

Lexical children produce distinct W and N views. Row checks consume both plus parsed anchors; batch classification precedes row checks and paragraph accumulation adds deduplication/cap. Accepted tuples cannot be fed back as full rejection logs: that projection loses information. The lexical node composes normalization followed by boundary matching; it does not identify word equality with span equality.

## cost

Let B be supplied bytes, R rows, S total paragraph sentence bytes and A answer length. Lexical normalization is O(B) scalar work/storage (lowercase expansion bounded by Unicode tables). Verbatim window comparisons cost O(total sentence tokens * answer tokens). Span matching with repeated suffix checks is conservatively O(S*A); anchors O(K log K). Batch indexing O(R log N), stored raw/column rows O(B); each row rebuilds answer views and scans relevant sentence views. No measured speed or approximation-error claim.

## epoch-memory

Prospective ratio/fold model predicted strict malformed-anchor and single-prefix repairs. Original production accepted 1,2 and PP1. These are repaired without changing signatures; valid local numbering unchanged. Unicode interval search passed independent finite enumeration, including invalid first occurrence followed by valid occurrence. A test expectation counted six rejects for eleven rows capped at six; corrected to five before final evidence, not a production bug.

## checks

Eight gate::wave17_models tests: Unicode interval oracle 341 squared = 116281 comparisons plus two overlap witnesses; batch integer-ratio oracle 220 cases; literal normalization; empty anchors; single prefix; cap/dense numbering; span/implicit composition; accepted-projection reconstruction. Synthetic Para only, no live teacher/parser/corpus/database. Debug/release twice and compiled mutations are recorded in root ../../model.md#evidence. Finite evidence does not establish all Unicode/grapheme semantics or full source correctness.

## History

### 2026-10-30 · epoch-memory

2026-10-30: prospective ratio/fold model predicted strict malformed-anchor and single-prefix repairs. Original production accepted 1,,2 and PP1. These are repaired without changing signatures; valid local numbering unchanged. Unicode interval search passed independent finite enumeration, including invalid first occurrence followed by valid occurrence. A test expectation counted six rejects for eleven rows capped at six; corrected to five before final evidence, not a production bug.

### Undated · document

Source SHA-256: 09328f711abf4b6453f95bed3d873deb205c8801d939841974416fe74e3e5806. Domain: finite supplied UTF-8 strings and synthetic Para values; rule V1/V2. Observation is the operation result specified below. Mathematical descriptions are source-level exact relations under stated assumptions, not a certified implementation theorem or numerical approximant.
