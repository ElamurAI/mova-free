# world-fbench-sampling

## forward

Hash course/lesson/line identity with wrapping FNV and mixer, sort tuples, rotate six categories selecting the earliest unused row. Stop at 40 picks or six empty categories. Global IDs are append positions; every fourth local pick is held.

## invariant

pick_indices returns distinct input positions of length min(40,N), in deterministic order for fixed sorted rows. Equal text at different positions remains distinct; course/lesson/line uniqueness is not validated.

## cost

O(N log N) tuple sorting; capped picks and at most six misses between picks give O(N L) selection for fixed cap 40, O(N) pool and bounded picked set.

## composition

The helper consumes already sorted tuples. Caller sorts using hash, lesson, line and text and derives split from local pick rank, not the original line index. The finite oracle supplies independently specified category masks.

## epoch-memory

Wave11. Synthetic old-code witnesses failed before narrow repairs: frame display lost attrs/cond; marker spelling priority selected an earlier answer; unchecked arithmetic panicked; dotted page answer 57 was mistaken for an item. Current tests preserve these counterexamples. No optimization, percentage-error surrogate or general correctness claim.

## checks

cargo test -p world --lib fbench::wave11_models -- --test-threads=1. See file-root ../model.md#evidence for repeated debug/release identities and compiled mutants. Syntax is a source-bound partial projection; macro bodies are opaque.

## behavior-complement

Subject/revision: manifest source hash and this scope. Universe: bounded supplied inputs and terminating observations under fixed dependencies and successful allocation. Inner absent-behavior model excludes any transition violating: pick_indices returns distinct input positions of length min(40,N), in deterministic order for fixed sorted rows. Equal text at different positions remains distinct; course/lesson/line uniqueness is not validated. External effects and unmodeled malformed states are unknown, not excluded. Finite tests support only named fixtures.

## reverse-reconstruction

Subject/revision: manifest source hash. Domain: the scoped forward input/output relation, fixed dependencies and successful allocation. Operator is the full compatible preimage: For a picked sequence, retain all sorted pools whose category-first available choices reproduce it. Unpicked rows and hash collisions preclude unique reconstruction; rank and text both constrain candidates. This is a partial source model, not an implemented complete inverse search; no unique causal explanation is inferred.

## coupling

Subject/revision: manifest source hash. Constraint relation on shared identities/intermediate states: The helper consumes already sorted tuples. Caller sorts using hash, lesson, line and text and derives split from local pick rank, not the original line index. The finite oracle supplies independently specified category masks. Source inspection and actual finite joins are distinguished.

## observation-fiber

Subject/revision: manifest source hash. Information-loss relation holds inputs in the same fiber when their declared observation agrees: Text category observation has six Boolean coordinates; punctuation/case changes can retain the same mask and selection while changing source text. Finite correspondence only; unrestricted soundness/completeness is not established.

## History

### 2026-10-30 · epoch-memory

2026-10-30 wave11. Synthetic old-code witnesses failed before narrow repairs: frame display lost attrs/cond; marker spelling priority selected an earlier answer; unchecked arithmetic panicked; dotted page answer 57 was mistaken for an item. Current tests preserve these counterexamples. No optimization, percentage-error surrogate or general correctness claim.
