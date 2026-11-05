# school-student-vidtvorant-runner

`student_vidtvorant SCHOOL_DIR PROBE_ROOT OUT_DIR COURSE...`: fits and measures the student's vidtvorant
([world/src/knowledge.rs.groupant/student-vidtvorant.groupant](../../../../world/src/knowledge.rs.groupant/student-vidtvorant.groupant/model.md))
against `student_probe` traces.

## forward

Load each course's traces from every repetition directory; refuse a course whose choices or understanding differ
between repetitions; take each question's median answer time. Read the lesson files' teaching lines. Parity folds
(fit on even lessons, test odd; the reverse): grid (anchor_min, own_min) ∈ {2,3}², perceptron five epochs, gap gate
then no-gap gate from twenty score quantiles, best training agreement. Predict every question out of fold; fit the
time model per fold on the other fold's questions. Report agreement (choice, understood, correctness), per-course
scores, disagreement classes by the student's route (guess, support, support tie, rule or count) with examples, and
a course split (fit on courses 1, 3, 5…, test the others). Then time `predict` per question with the first fold's
parameters.

## invariant

`semantic.json` depends only on the traces' choices, scores and median times and on the lesson files: two runs are
byte-identical (checked four times). `timing.json` holds the nondeterministic own-speed numbers apart,
together with the deterministic time-model errors. No held-out set is read: the school's tests and exams are the
pupil's diagnostic score, and folds are by lesson parity or by course.

## cost

≈ 4 × (5 epochs + 40 gate replays) replays per fold, three folds; ≈ 170 s on one core for ten courses (7 412
questions).

## behavior-complement

Does not run the student, the parser or any teacher; does not write outside OUT_DIR; does not register or claim a
cap; does not select courses after seeing results (the course list is the command line).

## reverse-reconstruction

A report is compatible with every set of traces giving the same per-question choices, scores and median times; the
examples listed are a deterministic subset, not all disagreements.

## tests

The included vidtvorant's six unit tests run with `cargo test --release -p mova-approx --example
student_vidtvorant`; the repetition-consistency refusal and the semantic determinism are exercised by every run.

## epoch-memory

First version; results in (see History).

## History

### 2026-11-04 · invariant

`semantic.json` depends only on the traces' choices, scores and median times and on the lesson files: two runs are
byte-identical (checked four times, 2026-11-04). `timing.json` holds the nondeterministic own-speed numbers apart,
together with the deterministic time-model errors. No held-out set is read: the school's tests and exams are the
pupil's diagnostic score, and folds are by lesson parity or by course.

### 2026-11-04 · epoch-memory

2026-11-04: first version; results in `$MOVA_DATA/runs/mova-speed-2026-11-04/vid-a..d`.
