# school-student-vidtvorant

The student's vidtvorant: an executable, typed fast model of the course pupil's test answers
(`PragStudent::answer` after `read` and the exercise passes, `world/src/knowledge.rs` at the manifest SHA-256). It
reproduces selected observations of the student, right or wrong; it is not a solver and never answers a test for
Mova. Code: [rust_approximant.rs](rust_approximant.rs) (standard library only). Runner:
`tools/approx/examples/student_vidtvorant.rs`. Reference traces: `world/examples/student_probe.rs`.

## vidtvorant

**Admissible inputs and states.** A course as lesson files (teaching lines: `explain:` then `fact:`), read in lesson
order; for each question, the question text and the four options in the order shown (`knowledge::shuffled`). The
state is the set of teaching lines read so far, as bags of stemmed content words (`knowledge::words`). Questions of
the maths student, the dictionary course and adapted courses are outside the domain.

**Observable results.**

| observation | student | vidtvorant |
|---|---|---|
| chosen option | `Answer::choice` | `Prediction::choice` |
| understood | `Answer::understood` (support decided, else the first option shown) | support found above the gate |
| correctness | `choice == right` | predicted choice `== right` |
| answer time | wall time of `answer` (median of repetitions) | `TimeModel` on ln(ns) |

**Decision structure (read from the code, mirrored).** Question → tail cut (`tail_question`) → anchors (lines sharing
≥ `anchor_min` words; current lesson first; strongest only; ten at most) → targets (anchor, linked neighbour lines,
next line for «what next») → own support (lines about the question's things that share ≥ `own_min` words with the
option) → structured support only (an asked word, or `own_min` overlap on the own route) above a support gate →
lesson-first choice → ties to the current lesson, then the first shown → the subset/extra tie rule → no support:
first option, not understood → a unique counted number decides. Frames become word bags; role matches become word
overlaps outside / inside the question; the student's per-ask, per-route weight table becomes nine integer
weights fitted by an averaged perceptron to the student's own choices (the student's update rule aimed at its
choices); the strength gates become two fitted score gates (with and without a readable wh-gap).

**Assumptions (unverified).** The student's lesson-order state is captured by which lines were read; parse ahead and
reading are deterministic (two runs give identical traces); option order is the shown order; answer time on one
core is dominated by candidate alignment, which grows with question and option length and only weakly with memory.

**Preserved properties (by construction, checked by unit tests).** Not understood ⇒ first option shown; a question
with no content word is not understood; current-lesson support decides when present; equal scores go to the
earlier option; the fit is deterministic (ordered maps, fixed epochs, integer weights) and agreement teaches nothing.

| metric | value |
|---|---|
| choice agreement (the student's option) | 67.24% (per course 62.27–71.74%) |
| understood agreement | 66.14% |
| correctness agreement (right/wrong as the student) | 73.39% |
| course test score: student vs vidtvorant | 45.94% vs 52.63% overall; relative error per course 3.38–32% |
| answer time, per question relative error | median 20.5%, p90 55.9% |
| course answer time, relative error | per course 0.64–20.2% |
| course split (fit on five courses, test the other five) | choice agreement 66.06%, correctness 72.25% |
| hand start without any fit | 37.37% joint choice+understood agreement |

**Performance profile.** Same core (P-core thread 0, taskset), host load average 6–15 during the vidtvorant runs and
15–34 during the student's: the student answers in 1.22 ms median (p95 2.37 ms) per question after parse ahead; the
vidtvorant predicts in 40–48 µs median (p95 146–170 µs), a 26–30× speed-up per answer (four runs). Replaying all ten
courses (reading every teaching line and predicting every question) takes 0.48–0.56 s against the student's
≈ 101 s of course wall time (parse ahead, reading, practice and answers), about 200×.

**Conformance criteria and caps.** A defect is a violation of the preserved properties above, a nondeterministic
semantic report (two runs must give byte-identical `semantic.json`), or a student trace whose choices differ between
repetitions (the runner refuses it). Supported caps are 1%, 2%, 5% and 10%. **No cap is met** on the declared
domain: choice disagreement 32.76%, correctness disagreement 26.61%, worst course-score relative error 32%, worst
course answer-time relative error 20.2%. Individual courses fall below 5% or 10% on single metrics (commands score
3.38%, physics 5.18%; history, medicine and physics answer time under 3%), but a cap chosen per course after
measurement would be selection, not a profile. Therefore `components.rust_approximants` stays empty.

**Classification (for when a cap is met).** metric: categorical decision disagreement (fraction of questions where the
chosen option differs; correctness disagreement as a second categorical metric; scalar relative error for course
score and course answer time); method: trace-fitted text surrogate (word-bag features over the student's decision
structure, perceptron and least squares); assurance: finite_observation (bound kind empirical). The first two labels
are not yet in the `mova-approx` and validator vocabularies; adding them is deferred until a profile meets a cap.

## forward

For each course, lesson by lesson: read the lesson's teaching lines into the word index; for each traced question,
predict (choice, understood, supports, anchors, targets); time features → ln ns. Fit: grid over (anchor_min,
own_min) ∈ {2,3}², perceptron five epochs on the training fold's lessons, then the gap gate and the no-gap gate from
twenty score quantiles of the training fold, best training agreement first.

## invariant

`choice < options.len()` when options exist; `!understood ⇒ choice == 0` unless a unique counted number decided;
lesson-first ⇒ the chosen support is in the current lesson; weights move only on a disagreement with supported
witnesses on both sides; the memory at a question holds exactly the lessons up to and including the current one.

## cost

Reading a line: O(w log V) for w content words into a word index of V words. A prediction: O(Σ postings of the
question's words) for anchors, plus per option O(Σ postings of its words) and O(targets · |option words|) features;
no parse, no WordNet, no frames. Fitting: (grid 4) × (5 epochs + 40 gate replays) × replay. Measured: 40–48 µs median
per prediction, 65–76 µs per question including reading; a full two-fold fit and report ≈ 170 s on one core.

## behavior-complement

Within questions of the declared domain, the vidtvorant does not: read syntax, roles, polarity scope, quotes or
clause links (so conversational courses' «what does she want?» questions that the student reads through a quote are
missed, 1 033 of its 2 428 disagreements are the student decided by support and the vidtvorant found none); use
WordNet synonyms, hypernyms or the kind base (paraphrased options stay unsupported); apply the verb gate, compiled
rules, the arithmetic tree reader or the kind base's sums (31 rule-or-count decisions, 9 agreed); learn per ask and
route; model the exercise passes' effect on the student's weights lesson by lesson (one weight vector for all
lessons); observe anything about exercises, the maths school, other switches, GPU parsing or other revisions. It
never predicts the right answer as a goal: a vidtvorant that scores above the student is a worse vidtvorant.

## reverse-reconstruction

From a prediction (choice, understood) the compatible inputs are all lesson prefixes and option sets whose word
overlaps give the same argmax under the fitted weights and gates: many, not unique (any rephrasing with the same word
bags, any extra lines that add no larger overlap). From the student's observed choice and the vidtvorant's
features, the weights compatible with that choice form a polyhedral cone (the perceptron constraints); the fitted
vector is one point of it, not the student's weight table. From a predicted time only the feature hyperplane
β·x = ln t is recovered.

## disagreement-insight

Where the shadow differs from the student (out of fold, 2 428 questions): the student guessed where
word overlap found support (578): its frames demand role alignment (verb and entity in the same roles), not shared
words, so overlapping but differently structured lines give it nothing; the student decided where overlap found
nothing (1 033 + 107 ties + 22 rules/counts): synonyms, kinds, quotes and the asked role carry its support; both
decided on different options (688): its per-role weights and verb gate order supports in ways a word bag cannot.
The student understands 54% of the 7 412 questions. Without the fitted gates the same word-overlap reader answered
68.2% right against the student's 45.9% while agreeing with it on only 55.8% of choices; with the gates it agrees on
67.2% and scores 52.6%. Mova is stricter than lexical matching: it answers from what the lesson states in the asked
role, not from shared words, which is what its controls (shuffled questions, questions from another lesson) demand.

## tests

`cargo test --release -p mova-approx --example student_vidtvorant`: six unit tests (stems and stop words; support
wins and nothing read guesses first; the support gates turn support into a guess, the no-gap gate only without a
gap; current lesson first; fit moves on disagreement only and only inside the training fold; the time model
recovers a planted law). The runner refuses student traces whose choices differ between repetitions. The student
probe's choices equal `world knowledge course --trace` (with `PRAG_OFF=course-controls MOVA_COURSE_BEFORE=0`) on
commands, myths and geography, every row. Runs a–d of the runner gave byte-identical `semantic.json`.

## epoch-memory

Built after reading `answer_with`, `current_support`, `learn_answer` and `tail_question`.
Prediction before measurement: word overlap would over-read (claim support the student lacks) and so out-score it;
confirmed (hand start 37% agreement, vidtvorant 68% right). Adding fitted support gates (the student's strength
gates) raised choice agreement from 55.75% to 67.09%; a separate gate for questions without a wh-gap (the student
understands 32% of those against 62% of «wh-…?» questions) added 0.15 points. The plateau near 67% is the cost of
word bags against frames; the next lever would be options' and questions' roles, which need the parser and would
remove most of the speed-up.

## History

### 2026-11-04 · vidtvorant

**Error profile, measured 2026-11-04** (ten primary courses: commands, myths, money, geography, history, physics,
medicine, chemistry, space, technology; 7 412 test and exam questions; fitted on even lessons and tested on odd, and
the reverse; every question predicted once out of fold; finite observation, no guarantee on other courses, switches
or revisions):

**Proven, measured, assumed.** Proven: nothing beyond the unit tests' finite cases. Measured: the numbers above,
dated, with run directory `$MOVA_DATA/runs/mova-speed-2026-11-04` (`probe/1..5`, `vid-a..d`). Assumed: everything
under Assumptions.

### 2026-11-04 · disagreement-insight

Where the shadow differs from the student (2026-11-04, out of fold, 2 428 questions): the student guessed where
word overlap found support (578): its frames demand role alignment (verb and entity in the same roles), not shared
words, so overlapping but differently structured lines give it nothing; the student decided where overlap found
nothing (1 033 + 107 ties + 22 rules/counts): synonyms, kinds, quotes and the asked role carry its support; both
decided on different options (688): its per-role weights and verb gate order supports in ways a word bag cannot.
The student understands 54% of the 7 412 questions. Without the fitted gates the same word-overlap reader answered
68.2% right against the student's 45.9% while agreeing with it on only 55.8% of choices; with the gates it agrees on
67.2% and scores 52.6%. Mova is stricter than lexical matching: it answers from what the lesson states in the asked
role, not from shared words, which is what its controls (shuffled questions, questions from another lesson) demand.

### 2026-11-04 · epoch-memory

2026-11-04 (first epoch). Built after reading `answer_with`, `current_support`, `learn_answer` and `tail_question`.
Prediction before measurement: word overlap would over-read (claim support the student lacks) and so out-score it;
confirmed (hand start 37% agreement, vidtvorant 68% right). Adding fitted support gates (the student's strength
gates) raised choice agreement from 55.75% to 67.09%; a separate gate for questions without a wh-gap (the student
understands 32% of those against 62% of «wh-…?» questions) added 0.15 points. The plateau near 67% is the cost of
word bags against frames; the next lever would be options' and questions' roles, which need the parser and would
remove most of the speed-up.

### Undated · vidtvorant

**Subject.** The course pupil with default switches (`PRAG_OFF`, `PRAG_VN` unset), CPU mode, `en.bin` 0.4
(SHA-256 31a93401…eb86), answering a primary course's test and exam questions in lesson order, each after reading the
lesson and practising its exercises twice.
