# school-course

## forward

Load/preparse lessons; read kinds; optionally fork two controls; for each lesson optionally forget, read it, test-before, learn twice, test-after, the two question controls (`question_controls`: the next test's question of the same lesson; a question of `other_lesson`, half the course away) and exam; join controls and write optional results/trace.

## invariant

Controls use separate students and shared cloned parse data, not the primary mutable weights. Reporting a better after-score is not a retained-learning or causal improvement proof.

## cost

Preparse + main learner + concurrent controls; latency is not the sum of thread CPU costs. Skip-before removes an assessment pass; trace memory scales with emitted rows.

## checks

knowledge::control_tests and trace_tests validate local reporting; full course execution intentionally not run. See ../model.md#tests for executed scope. A proposed oracle below is not a completed check.

## epoch-memory

The course loop prints `gap_diag` lines to stderr for each test not understood when MOVA_GAPDIAG=1; the trace TSV and the report are unchanged (byte-identical traces, ten courses, two runs).

The fourth control «questions from another lesson» (`question_controls`, `other_lesson`). Prediction: C_cross ≤ C_same, both red; a topic-only switch raises C_same, not C_cross. Measured on the ten courses: trace rows byte-identical to the base, C_cross 2 178 against C_same 2 354 of 7 283 (student 3 204), red in all ten; lower than C_same in seven. `student-qents-mods` then: student +30, C_same +20, C_cross +2 (kept). Test `the_cross_lesson_control_tells_topic_from_question`: a topic reader is red only against the cross-lesson control, an options-only reader against neither; the mutant `other_lesson = li` fails it.

## behavior-complement

Type: behavior_complement. Domain and observation: Diagnostic course orchestration from preprocessing through trace/results persistence, as narrowed below. Operator/target: U is terminating configured course traces, R is run_course_lessons. Inner-bound U\R excludes before-test calls when before_tests=false, control forks when controls=false, and school results writes when save_school=false. Optional explicit trace writes remain independent of save_school.

Approximation: source-derived scoped relation, conditional on valid internal state and fixed flags/resources; unmodeled behavior remains unknown. Evidence/oracle: Source inspection; no filesystem side-effect oracle executed.

## reverse-reconstruction

Type: reverse_reconstruction. Domain and observation: Diagnostic course orchestration from preprocessing through trace/results persistence, as narrowed below. Operator/target: For aggregate (right,total,understood,exam), compatible histories include all per-row outcomes satisfying sums and per-lesson order. Totals cannot recover labels, exact weights, lesson histories or parse cache contents. Trace rows refine but do not collapse that preimage to one state.

Approximation: source-derived scoped relation, conditional on valid internal state and fixed flags/resources; unmodeled behavior remains unknown. Evidence/oracle: Existing trace test validates selected row projections; aggregate inverse enumeration is proposed.

## control-partition

Type: experimental_relation. Domain and observation: Diagnostic course orchestration from preprocessing through trace/results persistence, as narrowed below. Operator/target: Control students share initial skeleton/kinds and cloned parse trees; no-lesson and key-removed tracks alter lesson reading. Shuffled-question control keeps options but rotates questions only when there are >1 tests. The cross-lesson control keeps options and takes the question at the same place (cyclically) of lesson (l + n/2) mod n, never l, when n > 1 and that lesson has tests. Therefore both denominators can differ from the main total and control_red compares ratios. The same-lesson shuffle removes question-specific evidence only; the cross-lesson one removes it and the lesson's topic, leaving an option prior.

Approximation: source-derived scoped relation, conditional on valid internal state and fixed flags/resources; unmodeled behavior remains unknown. Evidence/oracle: Existing control-share unequal-denominator negative case; no empirical causal claim.

## output-failure-prefix

Type: failure_projection. Domain and observation: Diagnostic course orchestration from preprocessing through trace/results persistence, as narrowed below. Operator/target: After inference and joining controls, optional school results write precedes optional trace directory/write. A late trace error can coexist with already written results; operations are not transactional. Conversely preparse failure prevents lesson loop and those writes. Reconstruct failures as compatible prefixes, not rolled-back state.

Approximation: source-derived scoped relation, conditional on valid internal state and fixed flags/resources; unmodeled behavior remains unknown. Evidence/oracle: Proposed injected filesystem failure oracle; no mutations to runtime source.

## History

### 2026-10-29 · document

Source-bound partial models, epoch 2026-10-29. Subject: world/src/knowledge.rs at the manifest SHA-256. Observation: Diagnostic course orchestration from preprocessing through trace/results persistence. Every relation below concerns this frozen source/configuration. Structural validation does not establish semantic correctness.

### 2026-10-29 · epoch-memory

2026-10-29: inspected the existing implementation before modeling. Preserved current behavior, including lossy decoding, index-sensitive ties and conditional updates. Existing witness tests distinguish active-scope learning from a wrong global witness. Counts represent distinct observations, not independent proofs. No source changes, performance gain, full-course result or autonomous combiner is claimed. AST remains explicitly absent; ranges bind source bytes only. Parser resources, panic/allocation behavior and complete inference semantics remain gaps.

### 2026-11-04 · epoch-memory

2026-11-04: the course loop prints `gap_diag` lines to stderr for each test not understood when MOVA_GAPDIAG=1; the trace TSV and the report are unchanged (byte-identical traces, ten courses, two runs).

2026-11-04 (student-control): the fourth control «questions from another lesson» (`question_controls`, `other_lesson`). Prediction: C_cross ≤ C_same, both red; a topic-only switch raises C_same, not C_cross. Measured on the ten courses: trace rows byte-identical to the base, C_cross 2 178 against C_same 2 354 of 7 283 (student 3 204), red in all ten; lower than C_same in seven. `student-qents-mods` then: student +30, C_same +20, C_cross +2 (kept). Test `the_cross_lesson_control_tells_topic_from_question`: a topic reader is red only against the cross-lesson control, an options-only reader against neither; the mutant `other_lesson = li` fails it.
