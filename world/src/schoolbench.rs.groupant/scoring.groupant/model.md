# world-schoolbench-scoring

## forward
Skip first line and # comments. Require exactly 15 tab cells. Accumulate right literal in exam bucket or fallback test bucket. Return four counts only when expected test/exam cardinalities agree.

## invariant
right<=total in each bucket; empty text with expected zero succeeds. First line is skipped without header validation.

## cost
Linear trace bytes; one row's split vector at a time; counters bounded by representable in-memory row count.

## composition
Leaf projections share exact lesson identities, selected indices, text bytes and option ordering. Compose only with those shared values retained; isolated score/length observations cannot recover them. Source inspection identifies knowledge::main_args dispatch, schoolbench selected-lessons -> counts -> run_course_lessons -> trace scoring, and schooltools option_line -> canon -> tally/lint/edit. Tests execute native pure helpers and isolated snapshot/restore; no live course/parser/teacher call.

## epoch-memory
Initial coverage records tolerant parser/score semantics and per-file effect boundaries. Model review exposed the direct zero restore index panic, repaired after old-code failure. Benchmark score loop was extracted unchanged to a pure private helper to test actual classification and width/count gates. Lock constructor failure may leave a stale directory; argument failures can consume flags. These remain explicit source-inspected limitations.

## checks

Native module filters schoolbench:: and schooltools::, including eight new wave9 tests; independent finite rank/cadence/length/fraction oracles and a restore no-effect fixture. Raw evidence is external under (see History); root ../model.md#evidence binds outcomes. Macro bodies in syntax projection remain opaque; finite agreement does not certify unbounded input space.

## behavior-complement
Evidence/status: source-inspected relation with the bounded tests named under checks; unenumerated behavior remains unknown. Complement claims are inner constraints, reverse claims retain all compatible candidates rather than identifying a historical cause.

## reverse-reconstruction
Evidence/status: source-inspected relation with the bounded tests named under checks; unenumerated behavior remains unknown. Complement claims are inner constraints, reverse claims retain all compatible candidates rather than identifying a historical cause.

## classification-boundary
Evidence/status: source-inspected relation with the bounded tests named under checks; unenumerated behavior remains unknown. Complement claims are inner constraints, reverse claims retain all compatible candidates rather than identifying a historical cause.

## course-join
Evidence/status: source-inspected relation with the bounded tests named under checks; unenumerated behavior remains unknown. Complement claims are inner constraints, reverse claims retain all compatible candidates rather than identifying a historical cause.

## History

### 2026-10-30 · document

Source-bound epoch 2026-10-30. Every view below uses this exact source revision and finite inputs with sufficient allocation resources. Forward/absent/reverse relations observe only the stated projection; filesystem models assume fixed supplied dependencies and distinguish successful returns from partial effects. Status: partial source model, with bounded native evidence in ../model.md#evidence at the root sidecar. No approximation percentage, universal proof or school accuracy claim.

### 2026-10-30 · checks

Native module filters schoolbench:: and schooltools::, including eight new wave9 tests; independent finite rank/cadence/length/fraction oracles and a restore no-effect fixture. Raw evidence is external under $MOVA_DATA/runs/octopus-wave9-school-2026-10-30; root ../model.md#evidence binds outcomes. Macro bodies in syntax projection remain opaque; finite agreement does not certify unbounded input space.

### 2026-10-30 · behavior-complement

Type: behavior_complement. Subject: world-schoolbench-scoring, epoch 2026-10-30. Domain and observation: scoped inputs and projected outputs defined here. Operator/assumptions: Successful score cannot have wrong cell width or mismatching expected counts; semantic identifiers, header content and right labels other than exact right are not validated.

### 2026-10-30 · reverse-reconstruction

Type: reverse_reconstruction. Subject: world-schoolbench-scoring, epoch 2026-10-30. Domain and observation: scoped inputs and projected outputs defined here. Operator/assumptions: Fix expected counts; all permutations of rows and substitutions in ignored cells preserve totals. For zero totals, empty text and header-only text are both preimages.

### 2026-10-30 · classification-boundary

Type: encoding_relation. Subject: world-schoolbench-scoring, epoch 2026-10-30. Domain and observation: scoped inputs and projected outputs defined here. Operator/assumptions: All non-exam strings, including before and unknown, become test. Right comparison is case sensitive. Recorded-score test explicitly covers these otherwise easy-to-misstate cases.

### 2026-10-30 · course-join

Type: composition_relation. Subject: world-schoolbench-scoring, epoch 2026-10-30. Domain and observation: scoped inputs and projected outputs defined here. Operator/assumptions: main_args obtains expected cardinalities from selected Lesson tests/exams, passes same selected sequence to run_course_lessons, then checks trace cardinality. This is an inspected actual join plus native scorer tests, not execution of parser/teacher/course.
