# world-primers-segmentation

## forward

Split on line-start one-to-three digit labels followed by dot-space, concatenate continuation lines, then reject short/drill/garbled/no-question text. Drop a leading numeric label, clean spacing, choose last question mark and preceding dot/semicolon, capitalize first scalar.

## invariant

Original chunk order survives; alphabetic-word minimum is six. Prefix material before the first label can form its own chunk. Decimal dots and multiple question marks can yield imperfect segmentation; no general sentence parser is claimed.

## cost

O(B) scans and allocations per chunk, ignoring repeated string concatenation growth implementation details; total stored text O(B).

## composition

problems outputs chunks; inline extraction calls word_problem while paged/chapter extraction calls story_and_question with separately associated numeric labels. Finite fixture checks the actual inline helper chain.

## epoch-memory

Wave11. Synthetic old-code witnesses failed before narrow repairs: frame display lost attrs/cond; marker spelling priority selected an earlier answer; unchecked arithmetic panicked; dotted page answer 57 was mistaken for an item. Current tests preserve these counterexamples. No optimization, percentage-error surrogate or general correctness claim.

## checks

cargo test -p world --lib primers::wave11_models -- --test-threads=1. See file-root ../model.md#evidence for repeated debug/release identities and compiled mutants. Syntax is a source-bound partial projection; macro bodies are opaque.

## behavior-complement

Subject/revision: manifest source hash and this scope. Universe: bounded supplied inputs and terminating observations under fixed dependencies and successful allocation. Inner absent-behavior model excludes any transition violating: Original chunk order survives; alphabetic-word minimum is six. Prefix material before the first label can form its own chunk. Decimal dots and multiple question marks can yield imperfect segmentation; no general sentence parser is claimed. External effects and unmodeled malformed states are unknown, not excluded. Finite tests support only named fixtures.

## reverse-reconstruction

Subject/revision: manifest source hash. Domain: the scoped forward input/output relation, fixed dependencies and successful allocation. Operator is the full compatible preimage: All line layouts and problem labels that collapse to the same story/question pair are compatible. Removed numbering, whitespace and post-question text cannot be reconstructed uniquely. This is a partial source model, not an implemented complete inverse search; no unique causal explanation is inferred.

## coupling

Subject/revision: manifest source hash. Constraint relation on shared identities/intermediate states: problems outputs chunks; inline extraction calls word_problem while paged/chapter extraction calls story_and_question with separately associated numeric labels. Finite fixture checks the actual inline helper chain. Source inspection and actual finite joins are distinguished.

## observation-fiber

Subject/revision: manifest source hash. Information-loss relation holds inputs in the same fiber when their declared observation agrees: Drill/fraction exclusion partitions a supplied chunk universe using character flags, word counts and punctuation. A rejected chunk does not reveal which discarded OCR text was intended. Finite correspondence only; unrestricted soundness/completeness is not established.

## History

### 2026-10-30 · epoch-memory

2026-10-30 wave11. Synthetic old-code witnesses failed before narrow repairs: frame display lost attrs/cond; marker spelling priority selected an earlier answer; unchecked arithmetic panicked; dotted page answer 57 was mistaken for an item. Current tests preserve these counterexamples. No optimization, percentage-error surrogate or general correctness claim.
