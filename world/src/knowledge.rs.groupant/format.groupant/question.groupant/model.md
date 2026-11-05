# school-question-label

## forward

Split on literal space-pipe-space, trim parts, require at least question plus two options; choose sole starred option or default first; preserve remaining option order.

## invariant

Successful question output contains at least one wrong option; no uniqueness or nonempty string guarantee exists.

## cost

O(input bytes + output strings) for split/trim and copied options.

## checks

knowledge::tail_tests::the_starred_option_is_the_right_one. See ../../model.md#tests for executed scope. A proposed oracle below is not a completed check.

## epoch-memory

Inspected the existing implementation before modeling. Preserved current behavior, including lossy decoding, index-sensitive ties and conditional updates. Existing witness tests distinguish active-scope learning from a wrong global witness. Counts represent distinct observations, not independent proofs. No source changes, performance gain, full-course result or autonomous combiner is claimed. AST remains explicitly absent; ranges bind source bytes only. Parser resources, panic/allocation behavior and complete inference semantics remain gaps.

## behavior-complement

Type: behavior_complement. Domain and observation: Question separator and marked-answer decoding, as narrowed below. Operator/target: U is question(value,file) Result transitions; R is its delimiter/marker decoder. Inner-bound U\R excludes success with fewer than three split parts and success with two or more marked option positions. A star in the question position is not an answer marker.

Approximation: source-derived scoped relation, conditional on valid internal state and fixed flags/resources; unmodeled behavior remains unknown. Evidence/oracle: Existing two-star failure is a negative control; short-line cases are proposed.

## reverse-reconstruction

Type: reverse_reconstruction. Domain and observation: Question separator and marked-answer decoding, as narrowed below. Operator/target: For (q,r,wrongs), compatible encodings include unmarked r-first and every insertion position of starred r among ordered wrongs, with whitespace and repeated leading stars normalized. Restrict encodings so other wrong options do not start with star. Do not infer original answer position.

Approximation: source-derived scoped relation, conditional on valid internal state and fixed flags/resources; unmodeled behavior remains unknown. Evidence/oracle: Proposed exhaustive insertion-position oracle.

## marked-position

Type: label_projection. Domain and observation: Question separator and marked-answer decoding, as narrowed below. Operator/target: Let M={i>=1 | part_i starts with '*'}. |M|=0 selects part1 unchanged; |M|=1 selects trim(trim_start_matches('*',part_i)); |M|>1 errors. Wrong option order is the original index order with chosen index deleted.

Approximation: source-derived scoped relation, conditional on valid internal state and fixed flags/resources; unmodeled behavior remains unknown. Evidence/oracle: Existing test covers sole marker away from first and legacy fallback.

## delimiter-boundary

Type: grammar_boundary. Domain and observation: Question separator and marked-answer decoding, as narrowed below. Operator/target: Only literal ' | ' splits fields; bare pipe or tab-pipe-tab remains within a part before trimming. Empty trimmed answer strings and duplicate strings are admitted if cardinality/marker guards pass. Format validation therefore does not imply semantically distinct alternatives.

Approximation: source-derived scoped relation, conditional on valid internal state and fixed flags/resources; unmodeled behavior remains unknown. Evidence/oracle: Prospective separator/empty/duplicate fixture; no new executed claim.

## History

### 2026-10-29 · document

Source-bound partial models, epoch 2026-10-29. Subject: world/src/knowledge.rs at the manifest SHA-256. Observation: Question separator and marked-answer decoding. Every relation below concerns this frozen source/configuration. Structural validation does not establish semantic correctness.

### 2026-10-29 · epoch-memory

2026-10-29: inspected the existing implementation before modeling. Preserved current behavior, including lossy decoding, index-sensitive ties and conditional updates. Existing witness tests distinguish active-scope learning from a wrong global witness. Counts represent distinct observations, not independent proofs. No source changes, performance gain, full-course result or autonomous combiner is claimed. AST remains explicitly absent; ranges bind source bytes only. Parser resources, panic/allocation behavior and complete inference semantics remain gaps.
