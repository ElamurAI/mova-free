# world-primers-reachability

## forward

Normalize number words, collect unsigned digit/comma runs, deduplicate and combine with constants 12,60,100,7,24. Enumerate sum/product/absolute difference/exact quotient for one or two operations; checked sum/product omit results outside i64.

## invariant

All generated candidates are nonnegative representable i64 values. Equal source numbers are deduplicated and same-value base pairs are skipped unless the second is an appended constant. Signs/decimals are not faithfully parsed as numbers.

## cost

With U distinct numeric runs, one-step vector O(U(U+5)); second step O(U(U+5)^2) operations; O(U^2) vector storage. Temporary operator vectors remain allocated.

## composition

The real crate::numbers::normalize is called on supplied number words. The finite oracle uses i128 expression enumeration for 36 numeric pairs and 41 answers each; a large-value witness excludes wrapped negative output.

## epoch-memory

Wave11. Synthetic old-code witnesses failed before narrow repairs: frame display lost attrs/cond; marker spelling priority selected an earlier answer; unchecked arithmetic panicked; dotted page answer 57 was mistaken for an item. Current tests preserve these counterexamples. No optimization, percentage-error surrogate or general correctness claim.

## checks

cargo test -p world --lib primers::wave11_models -- --test-threads=1. See file-root ../model.md#evidence for repeated debug/release identities and compiled mutants. Syntax is a source-bound partial projection; macro bodies are opaque.

## behavior-complement

Subject/revision: manifest source hash and this scope. Universe: bounded supplied inputs and terminating observations under fixed dependencies and successful allocation. Inner absent-behavior model excludes any transition violating: All generated candidates are nonnegative representable i64 values. Equal source numbers are deduplicated and same-value base pairs are skipped unless the second is an appended constant. Signs/decimals are not faithfully parsed as numbers. External effects and unmodeled malformed states are unknown, not excluded. Finite tests support only named fixtures.

## reverse-reconstruction

Subject/revision: manifest source hash. Domain: the scoped forward input/output relation, fixed dependencies and successful allocation. Operator is the full compatible preimage: For answer a retain every normalized source-number set admitting a permitted expression tree of depth at most two, plus singleton equality. Many texts normalize to each set; multiplicities are lost and operands may be reused. This is a partial source model, not an implemented complete inverse search; no unique causal explanation is inferred.

## coupling

Subject/revision: manifest source hash. Constraint relation on shared identities/intermediate states: The real crate::numbers::normalize is called on supplied number words. The finite oracle uses i128 expression enumeration for 36 numeric pairs and 41 answers each; a large-value witness excludes wrapped negative output. Source inspection and actual finite joins are distinguished.

## observation-fiber

Subject/revision: manifest source hash. Information-loss relation holds inputs in the same fiber when their declared observation agrees: This heuristic can accept using implicit constants and repeated operands, or reject valid problems needing more operations. Passing is an OCR plausibility signal, not proof of the story's answer. Finite correspondence only; unrestricted soundness/completeness is not established.

## History

### 2026-10-30 · epoch-memory

2026-10-30 wave11. Synthetic old-code witnesses failed before narrow repairs: frame display lost attrs/cond; marker spelling priority selected an earlier answer; unchecked arithmetic panicked; dotted page answer 57 was mistaken for an item. Current tests preserve these counterexamples. No optimization, percentage-error surrogate or general correctness claim.
