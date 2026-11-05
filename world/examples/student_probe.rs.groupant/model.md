# world-student-probe

`student_probe OUT_DIR COURSE...`: the course pupil run through `world::knowledge`'s public API with a clock on every
stage and every answer. It is the reference observer for the speed measurements and the source of traces for the
student's vidtvorant ([../../src/knowledge.rs.groupant/student-vidtvorant.groupant](../../src/knowledge.rs.groupant/student-vidtvorant.groupant/model.md)).

## forward

Per course: collect every text the course run parses (teaching lines; questions, right and wrong options of
exercises, tests and exams) as `course_texts` does; count the distinct parse units (`prag::frames::units`) and their
whitespace words; `PragStudent::default()`; `preparse` (timed, includes the model load on the first call);
`read_kinds`; then per lesson `read`, two exercise passes of `learn` with `shuffled(n<<20 | i<<1)` (the recomputation
reference of `learn_lesson`), then each test (`seed | 1`) and exam (`seed | 1 ^ 0xE8A3`) answered with `answer` and
timed. Writes `<course>.trace.tsv` (shown options, choice, right, understood, scores, anchors, targets, frames
remembered, answer ns) and `<course>.stages.json` (stage seconds, wall, process CPU seconds, counts, VmHWM).

## invariant

The answers equal `world knowledge course <course> --trace` with `PRAG_OFF=course-controls MOVA_COURSE_BEFORE=0`:
the same seeds, the same lesson order and, by the exactness of exercise reuse (docs/school.md, «Exact reuse inside
lesson training»), the same weights after each lesson (reuse on in the CLI, off here). Nothing is written to the shared state (`state::root()` is not touched).
Timing never feeds a decision.

## cost

Parse ahead + Σ lessons (read + 2·exercises·answer + (tests + exams)·answer). Measured (one P-core
thread, host load average 9–34): 6.2–18.1 s per course (median of five), parse ahead 40–70%, peak resident memory
0.95–1.09 GiB. Without exercise reuse it is slower than the CLI pupil alone (geography whole course: 7.0 s CLI
against 7.8 s at load 9 and 11.7 s median here).

## behavior-complement

Does not run the controls, the score before the exercises, the GPU path (unless the environment selects it), rule
or gap diagnostics, nor save results; does not use the parse cache of `bench`; does not decide anything from times.

## reverse-reconstruction

A trace row is compatible with every student state that gives the same choice and scores; times are compatible
with many schedules and machine loads, so only medians over repetitions are used.

## tests

Equality of choices with the CLI trace on commands (472 rows), myths (537) and geography (955). Five
repetitions per course gave identical choices (the vidtvorant runner refuses otherwise).

## epoch-memory

Added to measure the student's speed per stage and per answer without changing `knowledge.rs`; the
trace format of the CLI stays as it is.

## History

### 2026-11-04 · cost

Parse ahead + Σ lessons (read + 2·exercises·answer + (tests + exams)·answer). Measured 2026-11-04 (one P-core
thread, host load average 9–34): 6.2–18.1 s per course (median of five), parse ahead 40–70%, peak resident memory
0.95–1.09 GiB. Without exercise reuse it is slower than the CLI pupil alone (geography whole course: 7.0 s CLI
against 7.8 s at load 9 and 11.7 s median here).

### 2026-11-04 · tests

Equality of choices with the CLI trace on commands (472 rows), myths (537) and geography (955), 2026-11-04. Five
repetitions per course gave identical choices (the vidtvorant runner refuses otherwise).

### 2026-11-04 · epoch-memory

2026-11-04: added to measure the student's speed per stage and per answer without changing `knowledge.rs`; the
trace format of the CLI stays as it is.
