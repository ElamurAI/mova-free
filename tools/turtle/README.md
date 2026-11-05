# Sea Turtle — `mova-turtle`

🐢 Sea Turtle is one of Mova's animals (Rust on Mova): the school maker — questions, lessons and exams in the school's
lesson format. The long-lived teacher that comes back every year. See [the sea animals of Mova](../../docs/animals.md)
and [how the schools are made](../../docs/school.md).

It works natively first: it checks courses with the school's own rules and makes gap questions from a 🐚 Nautilus
dictionary layer. Written lessons and questions come from a teacher only through the Turtle's own private terminal
and the operator's word (`nautilus::teacher`, purpose `questions`, over [`mova-terminal`](../terminal/README.md)).

## Commands

    turtle check COURSE|DIR                          the school's rule and the marks of every lesson (JSON; fails on ✗)
    turtle gap LAYER_TSV COURSE SOURCE OUT JOURNAL MODEL   gap questions from a Nautilus layer into the new folder OUT
                                                     (MODEL: Mova's en.bin, which reads each gap's slot)
    turtle made JOURNAL                              makings, lessons and questions recorded in the journal
    turtle lesson-task COURSE TOPIC FIRST N          print a teacher question to write N lessons (nothing is sent)
    turtle work CONFIG TEACHERS PROFILE STATE [STEPS|- [SECS [BATCH]]]   the standing loop over the configured courses
    turtle work-summary STATE                        events, throughput, rejection and hold reasons
    turtle work-resume STATE                         continue after a limit has passed
    turtle plan DEPUTY_CONFIG CONFIG ID < text       read English, run it with the Turtle's hands, print the answer
    turtle serve DEPUTY_CONFIG CONFIG SOCKET LOCK    answer English on a private socket (one per LOCK; STOP ends it)
    turtle ask SOCKET ID [urgent] < text             send English to a serving Turtle

Build and test:

    cargo build --release --manifest-path tools/turtle/Cargo.toml --offline --locked
    MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin cargo test --manifest-path tools/turtle/Cargo.toml --offline --locked

## What it does

- **`check`**: every `.lesson` file is parsed by the school itself (`world::knowledge::parse`) and held to the rule of
  `world knowledge check`, the same function (`world::knowledge::lesson_problem`: 5 facts, 12 different exercises, 8
  tests, three wrong answers each). It also reads the marks of `lessons mark`: one leading `*` per question, no lesson
  mixing marked and unmarked questions, and not every right option at one place.
- **`gap`**: for each entry of a layer, each example holding the word as a whole word becomes «… ___ …» Which word
  fills the gap?, with three wrong options drawn by a hash from the layer's other words (its part of speech first),
  the right one starred at a drawn place. Every wrong option passes the fit check (`src/fit.rs`):
  - never a **near-synonym** of the answer: they share a WordNet synset, one is a kind of the other (hypernyms at any
    depth: «mum» → mother → parent), or the meaning of one (the layer's column and the pupils' dictionary) names the
    other;
  - shown in a form that **breaks the gap's slot**, read by Mova's annotator (the answer's tag, its object, the
    determiner before it): no tag of the form in Mova's lexicon (AGID forms attested in UD) can stand there
    («then ___.» takes «go» but not «says»; an adverb or noun slot only a form that is nothing but a finite verb), a
    determiner refuses it («a parents», «these robot», «an robot», «a order»), or a verb slot with an object meets a
    verb WordNet never gives a transitive frame. A meaning-frame rule (a noun of another kind) is not used: it let
    «do you know the teacher?» stand for «… the time?».

  Of the first twelve accepted candidates the three shown least so far are taken; a gap with fewer than three is
  dropped. Lessons keep the school's rule: entries are taken in order, their questions dealt three exercises to two
  tests, and a lesson closes at the first entry where it holds 12 exercises, 8 tests and 5 facts; a tail joins the
  lesson before it, and a layer too small for one lesson makes none. Written to a new folder and journaled
  (`made.jsonl`, with the layer's and the model's hashes, the dropped gaps and the refused candidates by reason).

## In English

`deputy.example.json` (handler `turtle`; operations `check` with the course, `made`; no slots): "Check the command
course.", "Please check the command course.", "Could you check the command course?", "Check the commands lessons.",
"How many questions did you make?", "How many questions have you made?", "Check the commands course." (the 0.5 parse
reads «commands» as its object; the 0.4 parse read it as a verb and refused). Refused with a review candidate:
negation, "Delete the command course.", "Check the course." (which course?), and questions about a check ("Is the
command course checked?"): only `made` (declared `observes`) answers questions. Answers:
"I checked the commands course: N lessons with M questions. Every lesson keeps the school's rules and its right
answers are marked at varied places.", "I have not made any questions yet."

`serve` holds a singleton lock like Nautilus. `crab-workers.example.json` lists the English service, the standing loop
and the Turtle's own terminal holder (Opus through Claude Code, `terminal.example.json`), all `autostart: false`.

## The standing loop

`work` (`src/work.rs` over `nautilus::work` with `make::LessonJob`) works the configured courses in two kinds of
step. A **sweep** reads every open lesson in order (held-out and control sets, a name with `test` or `shuffled`, are
never read) and closes those that pass `check::lesson`, up to BATCH (500) a sweep, with the loop's own `native`
journal line; a sweep that closed something is followed by the next at once, with no pause. Only when a sweep closes
nothing does a **teacher step** (`Loop::step`) take the first open lesson, one with a problem: it goes to the teacher
with its problems under that loop's gates (the operator's word, a question at most every 600 s, limits), and the
answer is kept as a review-only proposal only when it is the same lesson (course and number) and passes the same
check; then the loop pauses SECS (four times that while idle or waiting). The course files are never written.

## Tests

Unit tests, and English and gap tests that need the model in `MOVA_OCTOPUS_TEST_MODEL` (see Build and test above;
the gap tests also read Open English WordNet under `$MOVA_DATA/raw/en-oewn/json`). Red controls: only/just and
mum/parent are refused as near-synonyms, now/out/just and go/say as words that fit the gap, on today's sentences and
on the whole commands layer. Negative controls (each red, restored): `check` without the school's rule; held-out
course names read; the deputy vocabulary without «command»; the fit check switched off (unit and real-data gap tests
red); the sweep reading held-out courses; a teacher step after a sweep that closed lessons.

## Scope

Turtle checks existing courses, makes native gap questions from dictionary layers, prepares lesson tasks and keeps
teacher replies as review-only proposals. The standing loop never writes course files and never admits a proposal.
Course publication and question generators outside this crate are not part of it.
