# school-vidtvorant

The group of the student's vidtvorant: the reference observer (`world/examples/student_probe.rs`), the executable
vidtvorant (`world/src/knowledge.rs.groupant/student-vidtvorant.groupant/rust_approximant.rs`) and its runner
(`tools/approx/examples/student_vidtvorant.rs`). The subject they model is `school-runtime` (`world/src/knowledge.rs`),
whose groupant holds the vidtvorant as a child fragment.

## Forward

probe(course) → traces (choice, understood, scores, answer ns) and stage times; repetitions → medians; runner(traces,
lesson files) → fitted vidtvorant per fold → out-of-fold predictions → agreement, disagreement classes, time
errors, own speed. The wiring is checked by two equalities: probe choices = CLI trace choices (three courses), and
repetition choices identical (ten courses × five).

## Composition

The probe's observation (choice, understood, ns) is exactly the runner's label set; the vidtvorant reads only lesson
text and the shown options, never the student's internal frames, scores or anchors (those in the trace are used
only to classify disagreements). Errors do not compose into a bound: choice agreement 67.24% says nothing about
correctness on other courses or switches.

## Invariant

Labels come from the student, not from the right answers; folds never share a lesson (parity) or a course (course
split); semantic reports are deterministic; timing is reported apart.

## Cost

Student: ≈ 101 s for ten courses on one core (median of five, under load); vidtvorant replay 0.48–0.56 s; fit and
report ≈ 170 s.

## nonbehavior

The group does not substitute the vidtvorant for the student anywhere, does not train Mova, call a teacher, read a
held-out set, or claim a 1/2/5/10% cap (none is met).

## compatible-evidence

The reports are compatible with any student revision that gives the same choices and similar times on the ten
courses; a new revision of `knowledge.rs` requires new traces (the fragment's source hash binds it).

## Descriptant

First epoch. Speed of the pupil and the fidelity of its word-bag shadow, measured on one P-core thread.

## History

### 2026-11-04 · nonbehavior

The group does not substitute the vidtvorant for the student anywhere, does not train Mova, call a teacher, read a
held-out set, or claim a 1/2/5/10% cap (none is met, 2026-11-04).

### 2026-11-04 · Descriptant

2026-11-04: first epoch. Speed of the pupil and the fidelity of its word-bag shadow, measured on one P-core thread.
