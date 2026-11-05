# world-primers-inline-answers

## forward

Find the rightmost occurrence among six supported OCR marker spellings; consume optional dollar sign and digit/comma prefix. Reject missing digits, immediate decimal digits, slash/pound/caret suffix or i64 parse overflow.

## invariant

A later recognized marker determines the candidate even if malformed. Prefix parsing is conservative only for listed suffixes: comma grouping and whitespace-separated mixed fractions are not comprehensively validated.

## cost

Six reverse string searches plus prefix scan: O(B) time and O(B) transient digits.

## composition

word_problem truncates chunk at returned byte offset, then story_and_question processes only preceding text. UTF-8 byte positions come from string search; no character-index slicing is used.

## epoch-memory

Wave11. Synthetic old-code witnesses failed before narrow repairs: frame display lost attrs/cond; marker spelling priority selected an earlier answer; unchecked arithmetic panicked; dotted page answer 57 was mistaken for an item. Current tests preserve these counterexamples. No optimization, percentage-error surrogate or general correctness claim.

## checks

cargo test -p world --lib primers::wave11_models -- --test-threads=1. See file-root ../model.md#evidence for repeated debug/release identities and compiled mutants. Syntax is a source-bound partial projection; macro bodies are opaque.

## behavior-complement

Subject/revision: manifest source hash and this scope. Universe: bounded supplied inputs and terminating observations under fixed dependencies and successful allocation. Inner absent-behavior model excludes any transition violating: A later recognized marker determines the candidate even if malformed. Prefix parsing is conservative only for listed suffixes: comma grouping and whitespace-separated mixed fractions are not comprehensively validated. External effects and unmodeled malformed states are unknown, not excluded. Finite tests support only named fixtures.

## reverse-reconstruction

Subject/revision: manifest source hash. Domain: the scoped forward input/output relation, fixed dependencies and successful allocation. Operator is the full compatible preimage: Given (offset,value), retain all prefixes with that rightmost marker offset and all allowed numeric spellings/suffixes parsing to value. Different markers, commas and dollar formatting preserve the same integer. This is a partial source model, not an implemented complete inverse search; no unique causal explanation is inferred.

## coupling

Subject/revision: manifest source hash. Constraint relation on shared identities/intermediate states: word_problem truncates chunk at returned byte offset, then story_and_question processes only preceding text. UTF-8 byte positions come from string search; no character-index slicing is used. Source inspection and actual finite joins are distinguished.

## observation-fiber

Subject/revision: manifest source hash. Information-loss relation holds inputs in the same fiber when their declared observation agrees: For all 36 ordered marker pairs the second marker owns answer 7. Historical spelling-priority behavior returned 3 for some pairs; changed order is a local OCR semantic correction. Finite correspondence only; unrestricted soundness/completeness is not established.

## History

### 2026-10-30 · epoch-memory

2026-10-30 wave11. Synthetic old-code witnesses failed before narrow repairs: frame display lost attrs/cond; marker spelling priority selected an earlier answer; unchecked arithmetic panicked; dotted page answer 57 was mistaken for an item. Current tests preserve these counterexamples. No optimization, percentage-error surrogate or general correctness claim.
