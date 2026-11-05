# The school

This page describes Mova's school: the primary-school courses, how they are written and stored, how the course pupil
(the SLM's school student) reads a lesson and answers its questions, the controls every course report carries, how
failures are diagnosed, the tools that check and edit lessons, persistent learning in a fresh process, Mova-driven
curriculum improvement, and the school's results. The large model writes and reviews lessons; it never answers a test.
Every score on this page is the SLM's own.

## What the school is

The pupil lives a child's school years in a simulation: a child of 9–10 in an English-speaking town, with Ukrainian
and Chinese street friends, who plays games, watches cartoons and TV, goes to church and Sunday school, gets into
fights, does small sins and gets caught or feels guilty. Near the end of the AI course the pupil is told the truth:
it is Mova, a small transparent language model, and the schools are an experiment in learning language, logic and
real life.

- **Level.** Primary school only. Everything is written for a child of 9–10. A plan row too hard for that age gets a
  simple primary version or is moved to a list for later. Programming and philosophy are placeholders, not courses.
- **Where the school lives.** The courses and the school dictionaries are data outside the repository: the release
  archive `school-0.5.tar.gz`, installed by `mova fetch 0.5 school` into `$MOVA_DATA/school` (`courses/`,
  `dictionaries/`; `MOVA_SCHOOL_DATA` names another folder; `world::knowledge::school_data`). The pupils' dictionary
  `world/data/dictionary-improved.tsv` stays in the repository and is compiled in.
- **Courses.** A course is a folder `$MOVA_DATA/school/courses/<course>/` of `.lesson` files: biology, everyday life,
  history, geography, myths, money, commands, the books' chapters and others. `knowledge::course_names` lists the
  primary courses (not philosophy, programming, the maths `lessons` folder or the dictionary course). The maths
  school (word problems, the SVAMP train set and the book course) is a separate student with its own gate; see
  [training.md](training.md).
- **A lesson** has `explain:` and `fact:` lines (what the pupil reads), `exercise:` lines (practice, where it learns
  its weights), `test:` lines (the score) and `exam:` lines (scored apart, in stiff official language).
- **Questions** have four options, so chance is 25%. A question the pupil does not understand gets the first option
  shown.
- **Separation.** Test and exam sentences never go into the parsers' training. Only `explain:` and `fact:` lines go
  into the training pools, and the lessons held out for the parser's own test are left out whole, by id.

## How courses are made

**The pipeline of a course.**

1. A register brief (`<course>-register-brief.md`): domains, columns, what must be true, limits.
2. A register of 300–1 000 rows (TSV) and a plan (lesson, title, domain, 9–12 rows joined by « ; »), written by the
   teacher; big registers are cut into slices of 20 lessons, each written separately.
3. Lesson writers, ten lessons each, all on one shared brief (`course-lessons-brief.md`) plus the voice briefs
   (`casual-talk.md`, `exam-voice.md`, `harsh-brief.md`, `gossip-brief.md`). A lesson has 4–5 explanations, 11–14
   facts, 15–18 exercises, 10–12 tests and 4 exams; three wrong options per question, with myths as wrong options;
   stars 4/3/2/1 per block of ten. The writers put the right answer first; the dictionary builder's `lessons mark`
   (a tool outside this repository) marks it with `*` and moves it to a drawn place, so its position gives nothing away.
4. The gates, all `world knowledge` subcommands: `check` (no ✗), `lint`, `balance` (see [Tools](#tools)).
5. A first run of the pupil on the course with its controls (`world knowledge course <course>`).
6. A full run of the maths school (lessons, data, book course, `cv`): neither the gate nor `cv` may fall below the
   base build; then commit.
7. After a batch of lessons, the dictionary cycle: every word of the courses gets a rich record; the skeleton's
   operation words stay frozen.

Writers work in bounded waves, so an interruption loses only the lessons in flight. Shared briefs are separate files
that every writer reads; only the task line differs. A writer's «done, all green» is a claim: run the gates yourself
and read a sample.

**The voice.** Real speech, not a school book: clipped, wrong grammar, slips and self-corrections (the last number
counts), wrong explanations to put right, gossip in vague words, euphemisms, rough everyday swearing without slurs.
Half the conversational lines are the everyday mix (light general slang, sometimes dry, sometimes formal); half are
hard slang and dialect of the speaker's generation and place: US generations and the words of every English-speaking
country, Britain village by village. Hard consequences are real: dogs bite, children who swim out too far drown,
sins get found out, or don't. The exam is the opposite: so formal the pupil panics.

**Limits.** No sexual content, no slurs, no how-to for anything dangerous (only «never» and what to do). Real
people only as public record («it was reported»). Contested points named as contested, each side in its own words.
Facts true, or marked as reported or argued. Legal open sources only. Nothing about the people, plans, prices or
partners behind Mova goes into a lesson.

**Why.** Saturation: few new words, hundreds and thousands of examples of the common ones, in every register a child
meets. Lower scores and confusion are accepted; more lessons and courses come later.

## Formats

### Lesson lines

```text
course: everyday
lesson: 1
title: Stairs and steps
explain: Hold the handrail, take one step at a time and put your whole foot on each step.
fact: Toys, shoes and bags left on stairs are easy to trip over.
exercise: What should you do with toys left on the stairs? | Step over them quickly in the dark | Leave them for someone else | *Pick them up and put them away | Kick them down to the bottom
```

An option line is `exercise:|test:|exam: question | option | option | option | option`, the right option marked with
a leading `*` anywhere among the four. The tools read it as question, right, wrong (`schooltools::canon`); an
unmarked line is read right first. Lengths are counted in Unicode characters, never bytes.

### The school corpus (CoNLL-U)

One sentence is one CoNLL-U block, so `en`, `annot` and the UD tools read the file unchanged:

```
# source = school lesson (ours)
# course = <course folder, e.g. chemistry>
# lesson = <lesson file, e.g. 013-rust-irons-slow-fire.lesson>
# annotation = <pipeline: draft, patch model, gates, batch id>
# text = <the sentence as it stands in the lesson>
# sent_id = school:<course>:<lesson>:<index in the lesson>
<token rows: ID FORM LEMMA UPOS XPOS FEATS HEAD DEPREL DEPS MISC>
<blank line>
```

- The tree is columns 7–8 (HEAD, DEPREL), UD 2.18 with Mova's registry, always in the same block as its sentence.
- `sent_id` is the key across all layers; nothing is keyed by line number.
- Later layers go into the same block. Token labels go in MISC as `Layer=value` (`Prag=speech-act:request`).
  Sentence-level and structural labels go in comment lines `# <layer> = <json>` after `# sent_id`, referring to
  tokens by ID. A layer that has not run is absent; there are no empty placeholders.
- A layer never changes FORM, the token count or the tree unless its own gate passes; tree changes come only from a
  gated patch step.
- One block per sentence, in lesson order, none without a tree. A sentence the gates reject is not written (rejects
  are logged in the run folder). To re-annotate, replace the block; never append a second one with the same `sent_id`.

### Dictionary lessons

`$MOVA_DATA/school/courses/dictionary/` holds one lesson per word of the primary dictionary
(`$MOVA_DATA/school/dictionaries/primary-school.v2.tsv`): file `NNN-<word>.lesson`, `course: dictionary`, `lesson: NNN`
the word's place in the generation order. Same line format; options shuffled with the lesson number as seed. Built by
the dictionary builder (a tool outside this repository; the shipped lessons cannot be rebuilt without it): `lessons requests` / `lessons requests-empty` → teacher batches →
`lessons post`; words with school sentences get up to five of them (by `sent_id`) as context. Words the teacher
refused (swear words) have no lesson.

### Adapted courses

`$MOVA_DATA/school/courses/<course>-<lang>/` is a course adapted into another language lesson by lesson: the same file names,
the same number of lines of every kind, the same header with `course:` renamed. In `torah-he` (the Torah course in
Modern Hebrew) the old languages are the object of the lessons, never their medium: quoted verses stay in Biblical
Hebrew with their points, byte for byte as in `torah/`, and each layered fact reads
`<reference>: <verse> — מילה במילה: <gloss> — בעברית של היום: <today's Hebrew> — הפירוש: <the commentators' reading>`.

The builder's `lessons translate requests <school-dir> <course> <lang> <model> <out-dir>` → `annot batch-run
<out-dir>` → `lessons translate post <out-dir> <school-dir>`. `post` writes a lesson only if its counts, script,
question form and quoted verses match the source; the rest go to `<out-dir>/redo/`. Then `lessons mark`,
`lessons wrong requests|post` (wrong options in the question's language) and `world knowledge check <course>-<lang>`.
The SLM does not read these languages; adapted courses serve human readers and have no SLM scores.

## How the pupil reads and answers

The pupil is `PragStudent` in `world/src/knowledge.rs`, built on the pragmatics reader (`prag`). For each lesson in
order it reads, practises twice on the exercises, then answers the tests and the exam. Reading features are on by
default and each has a switch: `PRAG_OFF=<name>,…` turns one off (the output is then byte for byte the build without
it); `PRAG_VN=<name>,…` turns on an opt-in variant.

### Reading the lesson

- **Frames.** Each `explain:` and `fact:` line is read into event frames (verb, roles, polarity, time, quotes,
  clause links) and remembered with its course, lesson and line. References such as «(Genesis 12:1)» or «in English
  (JPS 1917)» are stripped first (`strip_citations`), and a layered fact gives two facts: the verse and its idea.
- **Kinds and names.** «X is a Y» and «X is a kind of Y» give kinds (`isa`); appositions and names of one thing give
  `same`. For things the lessons never name, WordNet gives synonyms and up to three hypernyms by meaning, every sense
  in order (`student-synonyms`, `student-kinds`): «husky» reaches dog and mammal; going up by the word would read
  «canine» as a tooth.
- **The kind base** (`student-parts`). (kind, part) → number from every «X has N Y» («insects have six legs»), with a
  count per part («four toes on each foot»). Before any lesson the pupil reads the body plans, `world/data/kinds.txt`
  (plain sentences such as «Mammals have four legs»; `student-body-plans`). A narrower kind is looked up first, so a
  lesson about it wins.
- **Rules** (`student-rules`, `prag::frames::implications`). A rule line is compiled into condition ⇒ conclusion over
  shared placeholders: an if / when / whenever / once clause ⇒ its main clause; the quoted subject of «mean», «ask» or
  «tell» ⇒ the complement («"Don't forget X" means remember to bring X»); a cause link ⇒ its effect
  (`student-rules-because`). In a conclusion «do it / that / so» stands for the condition's action; «want», «need»,
  «mean», «ask», «tell» only carry their complement. Rules are compiled again from the remembered lines; nothing new
  is saved.

### Reading the question

The question is read into frames, one of them holding the asked role (the gap). Reader repairs, each with its switch:

| switch | what it reads |
|---|---|
| `q-strand` | a stranded preposition takes its wh-word: «Who is the serpent talking to?» → «the serpent is talking to whom?»; the asked role is the preposition's |
| `speaker-label` | a label before a quote gets its verb of saying back: «Mum, at the door: "have you seen my keys?"» → «Mum says, …», so the speaker says the quote and «you» or the vocative is spoken to |
| `appos-quote` | no apposition across a quote («Isaac said to his father, "Where is the lamb?"» does not make father = lamb) |
| `wh-copula` | «Where is the lamb?» inside a longer sentence reads as alone |
| `wh-passive` | «When was the college founded (by Y)?» → `Y found patient=college`, the time asked of «found» |
| `wh-event-noun` | a time or place asked of «happen», «occur», «take place» moves to the event of its subject noun (founding → found, death → die) |
| `vn-gerund` | a gerund noun with an «of» or «by» argument names its verb's event («the opening of the stadium» → open the stadium) |
| `q-do-prep` | do-support in statement order takes the verb after the subject, never a preposition inside it |
| `wh-time` | time values (months, seasons, years, days beside a month, era marks) become `when:<value>`; «when» asks that attribute, and a time attribute meets only a time attribute |
| `wh-which` | for «which / what N», an option that is one of the names of the remembered N in that role fills it (`Ent::names`: compounds, appositions, named «of» modifiers) |
| `student-qents-mods` | the question's things include its noun compounds the lessons name, possessors and «of» modifiers («lithium mines» is about lithium) |
| `factuality` | implication signatures (`prag/data/signatures.tsv`): «managed to eat» ate, «failed to eat» did not, «wanted / tried / planned to eat» is irrealis |
| `cond-irrealis` | an «if» clause is not said to have happened |
| `conj-neg` | a predicate joined to a negated one is negated too («never walk, swim or drive through flood water») |
| `hyphen-adj` | the second part of a hyphenated compound adjective that the parser tags as a verb modifier («a short-eared owl»: eared/VERB/amod) is no predicate: no frame «Event _ ear»; the noun keeps it among its attributes |
| `conj-pp` | a prepositional phrase the parser coordinated with an object noun («8 eyes, usually in two or three rows»: rows/conj with its own «in») is the verb's phrase (place=row), not a coordinated entity |
| `student-qty` | a cardinal in an entity's phrase («400 shekels») is its quantity; «how many / how much» counts the entity its wh-word modifies |

### Choosing an option

1. **Anchors.** Remembered frames that share roles with the question are its anchors. The current lesson comes first:
   anchors are taken from it where it has any, and options are judged by their support in it where any option has
   some (`student-anchor-first`, `student-lesson-first`). Earlier lessons decide only where the current one is silent.
2. **Support.** Each option is read and aligned with remembered frames reached from the anchors or by its own route.
   An option's candidates are capped per entity and verb (`MOVA_CAND_CAP`, default 300). The score is a learned
   linear function over role-match features, per asked role and route (17 features; `Feat` in `knowledge.rs`).
   Extra rules: «to» is the recipient (`student-to`); a quote fills the patient of «ask» / «say» (`student-quote`);
   support may be about an entity joined to a frame's (`student-own-and`); a question with nothing to be about
   («Which is true?») lets options find support anywhere (`student-open-claim`); irrealis disagrees like a negation
   (`student-irrealis`). A lesson frame that denies what an option asserts (the same content verb, a shared core
   role, the lesson's frame negated, the option's not) counts against it: a positive weighted sum is negated
   (`student-neg-counter`). The other direction, an option denying what a lesson affirms, only with
   `PRAG_VN=student-neg-both`: the reader often puts an option's «not» on a clause it does not govern.
   Measured 2026-11-05 with `hyphen-adj` and `conj-pp` (26 primary courses, full bench tier, seed 42): tests
   19 813 → 19 817 (odd lessons −6, even +10), exam 5 793 → 5 811 (odd +26, even −8); medium tier 6 811 → 6 799,
   exam 1 932 → 1 944; both directions (`student-neg-both`) 6 785 on the medium tier. Almost every changed answer
   comes from the changed learning path, not from a counted-against witness or a hyphenated word.
3. **The verb gate** (`student-verb`). For a role question whose gap frame has a content verb, a remembered frame
   supports an option only if its verb states the question's verb (`verb_agrees`: the same verb, a WordNet synonym of
   the first two senses, the verb without its particle, or a hypernym within two levels). Not gated: light and
   copular verbs, «happen / occur / take place», carrier verbs (want, need, mean, ask, tell, expect, like, wish),
   verbs of saying, verbs whose preferred VerbNet class takes a clause, the quote route and clause options. A frame
   with a light verb or none counts. Gated-out frames only order options tied on support.
4. **Rules.** A compiled rule fires when its condition matches a question frame other than the asked one: a quoted
   pattern only a quoted frame, a «can» pattern only an asked «can», the same verb or a synonym, the same polarity,
   every named entity present, placeholders bound to the question's entities. A binding alone is never a match. An
   option fits when it has the conclusion's content verb, the same polarity and no core role filled by another
   entity. Exactly one fitting option decides, and the trace names the rule, its line, what it bound and what it
   concluded.
5. **Counting and arithmetic.** For counting questions the kind base answers: each counted group times its part's
   count, a part the question states counting first; it answers only when every group is found and exactly one option
   holds the number, and the trace shows the sum with its sources. A question's own numbers are worked out by the
   maths student's tree reader (`student-arith`).
6. **Ties.** When two options tie and the shorter one's words are a subset of the longer one's, the longer loses if it
   has more unsupported content (`student-extra`).

Opt-in variants, off by default because they did not pass the controls: `PRAG_VN=quant-of` (a word of measure read
as what it measures), `student-dict` (a noun's dictionary kind joins its kinds), `student-verb-strict` (light frames
do not pass the gate), `student-verb-all` (every option frame gated), `student-rules-tie` (rule ties broken by
agreeing roles).

### Learning from exercises

The exercises run in two passes per lesson. The weights are a table per (ask, route, feature) of integers; one ask
reads a block of seven routes × 17 features. Learning uses the answer without the rules and without the verb gate
(`answer_by_weights`), so a rule or the gate never stops an exercise from teaching. Nothing is learned from tests or
exams.

#### Active witnesses

When current-lesson priority chooses an option, the update uses the frames that actually supported it in the current
lesson, not its best support across all lessons; an option with only older support is not updated. A rule-computed
count does not depend on the weights and triggers no correction. Both alternatives need supported witnesses before a
comparison teaches; one-sided updates are skipped. For fixed witnesses, with d = φ_right − φ_chosen, the correction
w' = w + η·d raises the gap w·d by η·Σd_i², so it cannot decrease for η ≥ 0 ([School.lean](../formal/School.lean);
a local model, not a convergence or accuracy proof; Collins 2002). A Rust regression shows the wrong-witness case
going the wrong way. A counted-against witness (above) scored minus its features, so its step goes along their
negation; its stored score differing from the weighted sum marks it.

`MOVA_STRENGTH=mask` computes alignment strength from the seven role-match bits (twice the population count of
non-attribute matches, plus one for an attribute match) instead of the feature vector; it agrees with the vector on
every 16-bit mask. The vector path (`MOVA_STRENGTH=vec`) stays the default.

#### Exact reuse inside lesson training

Within one lesson's exercise passes the remembered world, options and labels are fixed, and an answer reads only its
ask's 119-weight block. A supported answer is reused only when that whole block equals the snapshot taken before its
evaluation; the usual update still runs. Answers that cannot change through weights (no understanding, a
rule-computed count, no structural support for the right option) skip later evaluation. The memo lives inside one
`learn_lesson` call. `MOVA_LEARN_REUSE=0` is the recomputation reference ([ExerciseReuse.lean](../formal/ExerciseReuse.lean);
Michie 1968). Every intermediate weight state equals the reference's.

## Controls in every course report

`world knowledge course <course>` prints, beside the score, two guessers and four controls. A gain counts only
against them.

| line | what it is | what it catches |
|---|---|---|
| the longest option | always picks the longest option | answers given away by length (the bar to beat) |
| the first option shown | always picks the first option | what a not-understood question gets |
| no lesson | a pupil that reads nothing, learns from the exercises and takes the tests | gains that need no lesson |
| questions shuffled | the real pupil answers each test's options under the next test's question of the same lesson | an option prior: the lesson agrees with the option whatever is asked |
| key sentences removed | a pupil that reads each lesson without the key line of its tests (the line sharing most content words with the question and right answer) | gains that do not come from the line that states the answer |
| questions from another lesson | options asked under the question at the same place of lesson `(l + n/2) mod n` | topical reading: the question names the lesson's things but is not read |

Each control must score below the real pupil («all red»; otherwise the line names the control that is not). An
option's evidence can be question-specific (the anchors), topical (the question names the lesson's things) or a prior
(the lesson agrees with the option). The same-lesson shuffle removes only the first; the cross-lesson control removes
the first two. The extra pupils run in their own threads on the real pupil's parses and leave its scores unchanged.
`PRAG_OFF=course-controls` turns them off. The trace carries each as a `#control` line, and `#bar` lines for the
guessers.

A change is kept when:

- two runs give identical traces (deterministic maps everywhere);
- the pupil gains on odd and even lessons, or, for changes about memory or reading order (where lesson 7 is read with
  what lesson 6 left behind), across courses;
- the controls stay red and do not rise with the pupil, the cross-lesson control above all;
- the maths school's `world school actors ablate` and `cv` do not fall (byte-identical for changes outside it);
- the switch off reproduces the old traces byte for byte.

Score sums lean on large courses (the books are about half of all tests); report the mean of the courses beside the
sum. A reading fix can make the reading true and barely move the score: check it on the sentence's frames, not only on
the school. For fast iterations use `world knowledge bench` (quick and medium tiers are subsets, see
[training.md](training.md)); a probe is an approximation, so confirm with full runs periodically.

## Diagnosing failures

`world knowledge diagnose [examples.md] [course…]` (defaults: `diagnose-examples.md` and the primary courses without
the dictionary) splits every test by the step at which the pupil fails and writes examples with the anchors found
and the frame that won:

| step | meaning | where to look |
|---|---|---|
| 1. no frame in the question | the question was not read into a frame | parser, reader |
| 2. nothing remembered matched | no remembered frame shares a role with the question | representation, retrieval |
| 3. no option supported (guess) | anchors found, but no option is supported | representation of options, rules over instances |
| 4. right option unsupported | another option is supported, the right one is not | missing knowledge, paraphrase |
| 5. right option outscored | both supported, the wrong one more | inference, weights |
| 6. right | | |

Other views:

- `world knowledge trace <course> [lesson] [k]`: each test's question frame, the remembered frames it matched and why
  the choice won. `world knowledge course <course> --trace out.tsv` writes every test with its choice and reasons.
- `MOVA_GAPDIAG=1 world knowledge course <course> --trace …` prints, for every option of a not-understood test, why
  no remembered frame supported it: `light-or-noverb`, `verb-unknown`, `verb-known-noent`, `reached-gated`,
  `about-capped`, `about-weak`, or `not-about` (`/noanchor`, `/line`, `/text`), with its distance from the anchors.
  Traces are unchanged with it on.
- `MOVA_STUDENT_FORGET=1` makes the pupil forget all earlier lessons before each new one, keeping only its weights:
  it shows whether memory helps a course.

Known causes, read from these diagnostics:

- **Rules not applied to instances.** Most guesses: lessons state rules about kinds of situation, tests give new
  instances. Variables alone do not help: a binding makes every option about the instance, traps included. Compiled
  rules help where the option uses the conclusion's verb; options that paraphrase the conclusion stay out of reach.
- **The option prior.** An option that agrees with some sentence of the lesson is right more often than chance,
  whatever the question says. Every way of using that directly raised the shuffled controls with the pupil.
- **Carrier verbs.** Decisions carried by «say», «tell», «mean», «know», «ask» with a matching speaker are close to a
  coin toss in conversational courses and good in courses of reported facts.
- **Conditions and operators.** «if you see a snowman, call me», «or else», «not now» are read as plain actions; the
  condition's truth in the scene is not evaluated. Vocatives do not yet decide who a command is for.
- **Elliptical options.** Many right options are bare names, numbers or copular phrases with no content verb; true
  light-verb constructions are rare.
- **Lessons that lack the answer.** Questions about a quote the lesson does not hold; numbers tied in one phrase
  («from fifty down to ten»).
- **Memory as a pile of sentences.** The pupil keeps frames, not a world model; questions needing many lessons at
  once (books, counting) are its weakest. A heavier negation weight is not the remedy for missed negations.

A diagnosis is a hypothesis about where to look, not a proven cause.

## Tools

The checks and safe edits of lessons are `world knowledge` subcommands (`world/src/schooltools.rs`, parser in
`world/src/knowledge.rs`). **Agents use these commands, not scripts**: no Python, awk, `split` or copy-back one-offs
for school data; a need they do not cover becomes a new subcommand in Rust with a test. With no course named, a
command takes every primary course; a folder or a `.lesson` path works too; `--help` prints the usage.

| command | what it does |
|---|---|
| `check <course>` | lesson shape: ≥ 5 facts, ≥ 12 distinct exercises, ≥ 8 tests, exactly three wrong answers per question; ✗ per bad lesson |
| `lint [course…] [--fix-spaces]` | errors: ≠ 5 fields, empty fields, not one space each side of `|`, duplicate options, a wrong option equal to the right one, stray tabs; reported apart: case-only duplicates and wrong options glued from others («X and Y»). `--fix-spaces` only normalises spaces |
| `balance [course…] [--band L_lo,L_hi,S_lo,S_hi] [--list out.tsv] [--chunk N --prefix p]` | how often the right answer is strictly longest (L) and strictly shortest (S); default band L in [1/6, 1/3], S in [1/10, 1/3], S > 0; lists lessons outside it, split into work queues of N lessons |
| `edit <edits.tsv> [--dry-run]` | applies option edits `file⇥line⇥old option⇥new option`; a file with any bad row is untouched (question or right answer changed, `*` lost, duplicates, not three wrong options, `|` or tab in the text); prints L/S before and after |
| `snapshot <course\|file…> <dir>`, `restore-lines <dir> <file> <line…>` | copies lessons; puts back only the given lines (`12`, `10-14`), refusing if line counts differ. Never copy whole folders back |
| `course <course> [--trace out.tsv]` | the pupil reads the course and answers; score, guessers, controls |
| `bench <course> [--round N \| --tier quick\|medium\|full] [--seed N] [--plan] [--no-cache]` | fast probes with a periodic full tier (`MOVA_BENCH_ROOT`); see [training.md](training.md) |
| `diagnose`, `trace`, `show <course> [lesson]` | see [Diagnosing failures](#diagnosing-failures); `show` prints a lesson as the pupil reads it |
| `retain split\|train\|practice\|test\|meanings …` | persistent learning, below |
| `teacher-check <course> [from] [to] [per]` | the hidden teacher (Codex CLI through `mova-teacher`) judges the pupil's frames; verdicts go to `$MOVA_DATA/runs/prag-teacher/` and never feed the score |
| `coverage [course…]` | coverage of the register by the courses; uncovered situations go to `data/school/queue/` as lesson-sized batches |
| `kinds-up <word>…` | a noun's kinds by WordNet meanings, nearer first |

**The pupils' dictionary** (`world/data/dictionary-improved.tsv`) is merged and checked by the dictionary builder:
`merge <records.tsv…> [--check]` validates every record (UTF-8, six TAB fields, none empty, the word lower-case and
trimmed, no dates), skips words already present and repeats, and appends the rest; any bad record means nothing is
merged. `gaps [--split N --prefix p]` lists words of the primary lessons without a record. `world` embeds the records
at build time, so rebuild after a merge. The merger only appends: a known headword has no versioned replacement path,
and old senses are never silently erased.

## Dictionaries in the pupil

The pupils' dictionary is read by the dictionary course, by the maths reader for an unknown verb's meaning, and,
through `world meanings freeze`, for the maths skeleton's operation words (`data/skeleton-ops.tsv`; the live
dictionary only with `MOVA_LIVEOPS=1`). Its counted facts (`MOVA_DICTFACTS`) stay off. The course pupil matches words
through WordNet; reading a noun's dictionary kind (`PRAG_VN=student-dict`) stays opt-in because generic heads (sauce,
town, animal) let another lesson's question find support as readily as the real one. The Wiktionary module's synonyms
and form-of lemmas for a question's thing (`PRAG_VN=student-wikt`) stay opt-in for the same reason. The dictionary
store and its layers are described in [training.md](training.md).

## Persistent learning and retention

A lesson must leave a durable change: knowledge-graph relations, learned parameters, or a general improvement to the
deterministic skeleton. `world knowledge course` reads, practises and tests in one process; that is a reading
diagnostic, not retained learning. Retention is examined in a fresh process from a saved state:

```sh
world knowledge retain split lesson.lesson teaching.lesson examination.lesson
world knowledge retain train teaching.lesson state-v1.json
world knowledge retain test state-v1.json examination.lesson
world knowledge retain train next.lesson state-v2.json state-v1.json
world knowledge retain practice teaching.lesson state-v3.json state-v2.json
world knowledge retain meanings state-v1.json learned-meanings.tsv
```

- **The state** holds frames, aliases, kind and part relations, role weights, source provenance and a teaching-version
  ledger. Lookup indexes and frame codes are rebuilt from it. Parser caches, test questions and labels are never saved.
- **The exam** receives only test and exam questions with metadata; teaching lines and exercises are rejected. It gets
  no lesson path, does not read and does not update weights; a fingerprint verifies the state is unchanged. Gold
  answers are used only to score.
- **No current-lesson hint.** A retained learner disables the latest-lesson preference in practice and exam: an answer
  must find support in accumulated knowledge.
- **Versions.** Training ingests a new lesson once; an identical lesson needs explicit `practice`, which updates
  weights without rereading or duplicating frames. A changed teaching version needs a rebuilt state. Every output is a
  new file; a parent state is never overwritten. The checksum detects accidents, not a party able to rewrite both
  payload and checksum.
- **Dictionary binding.** The state records the dictionary store's fingerprint (`world dict stats`); an exam under
  another store (`MOVA_DICT`) is refused. `retain meanings` writes what lessons taught about words (kinds and names,
  with course, lesson and line) as a dictionary layer of state `derived`, knowledge only where unreviewed layers are
  allowed: the reader's appositions can give wrong pairs.
- **Limits.** Provenance uses lesson numbers, so each course needs its own state. Cross-course namespaces,
  contradictory facts, time and context scope, graph consolidation and selective version replacement are open. A saved
  graph proves retention, not abstraction or generalization. The checkpoint is a first storage mechanism; the target
  placement of knowledge is in [knowledge.md](knowledge.md).

**Failures drive improvements.** An exam reports each wrong answer, its evidence and an initial diagnosis. Missing
support points to parsing, representation and retrieval before weights; a supported selection error points to
inference and feature updates. Review failures, build teaching evidence and remedial exercises, train a new state,
rerun the frozen assessments and the regression controls. Never train on exam labels and report the same exam as
independent.

**Cost model.** For R repeated assessments, replay costs R·(parse + ingest + learn + evaluate); a durable graph costs
parse + ingest + learn + save + R·(load + evaluate). Retention wins on time only when loading is well below replay.
[Retention.lean](../formal/Retention.lean) proves answer preservation under a round-trip law, state preservation of an
abstract exam and the cost identity; it assumes the round-trip law and does not prove the Rust serializer. The
`retention_probe` example compares answers and explanations before saving with a cold restored learner:

```sh
cargo run --release -p world --example retention_probe -- teaching.lesson examination.lesson new-state.json
```

## Mova-driven improvement

`mova-enrich improve` is one bounded study → propose → review → learn → assess step. Mova supplies the proposals and
measures the learner; one Codex teacher reviews.

```sh
export ENRICH_CURRENT_DICTIONARY="$PWD/world/data/dictionary-improved.tsv"   # from the repository root
mova-enrich improve STAGED_PARENT NEW_ROOT [BASE_MODEL]
```

It writes `study/`, `review/`, `learned/` and `cycle-receipt.json` under a new root:

1. Split the parent lesson into teaching and question-only examination files.
2. Load a retained model or create one. Content already learned is recognised by its ledger digest and reused without
   rereading; new material gets a fresh lesson ID.
3. Record exercise feedback, train new material, save, and examine in a separate process that receives only model and
   assessment.
4. Mova ranks dictionary proposals by missing entries and weak exercise support, and proposes literal token clozes
   from agreed subject/object dependencies (no pronouns, ambiguous spans or thin distractor pools).
5. Bind that training-only feedback and its parent versions into a fresh packet; refresh dictionary records so
   published headwords are recognised and old meanings stay protected. Codex reviews once; source, dictionary-format
   and lesson-structure gates follow.
6. Learn the checked lesson on top of the earlier state. Run the old frozen assessment and the new one in separate
   processes, and ask the old learner the same new questions. Report the same-question change, never the difference
   between two question sets.

Results bind `assessment_sha256` to course, lesson seed, questions, labels and option order. A positive same-question
gain with a preserved old score makes a candidate eligible for promotion; that is a recommendation, not a release, and
candidates are kept either way. More frames without a same-question gain is not improvement. The study context holds
exercise feedback only; old test and exam questions and labels are reserved and can never become exercises.

Separate steps, for preparation and recovery: `mova-enrich study`, `prepare-next`, `review`, `stage` and
`finish-improve ROOT`. `study` and `prepare-next` make no teacher calls; a completed `finish-improve` resumes without
another call or learning update; `improve` needs a new root so a paid attempt is never silently repeated. Failures
are kept, not retried. A source eventually runs out of unreserved exercise material and preparation then fails before
any request. `ENRICH_CURRENT_DICTIONARY` overrides `CYCLE_ENRICH_DICTIONARY`; the prompt byte cap applies. The design
borrows candidate generation and filtering from Self-Instruct (Wang et al., ACL 2023) and feedback-driven revision
from Self-Refine (Madaan et al., 2023), in a limited deterministic form.

## Tale-driven everyday-biology lessons

A tale can raise a useful everyday question without supplying the answer. Biology lessons 406–415 take their triggers
from the public-domain [Where Animals Talk](https://www.gutenberg.org/ebooks/58900) (R. H. Nassau) and their
explanations from licensed open textbooks (OpenStax / LibreTexts editions), with source sidecars that keep narrative
trigger, scientific explanation, existing teaching context and support limits apart.

| lesson | trigger | reasoning target |
|---|---|---|
| 406 | a rat dives | swimming does not identify the breathing organ |
| 407 | baskets seem heavier | perceived effort and measured load differ |
| 408 | full belly and thirst | one body state does not determine another |
| 409 | sounds without a visible canoe | observation, inferred source and unseen detail differ |
| 410 | tail and root claims | shape or a speaker's claim does not identify a structure |
| 411 | a crab's regrowth promise | a conditional capacity is not a timing guarantee |
| 412 | a discarded snake skin | a trace is not the organism, its location or age |
| 413 | an egg-origin argument | a shared feature does not identify a group |
| 414 | eggs produce no young | an outcome can have several compatible causes |
| 415 | a plantain stalk and shoots | an organ, a shoot and a plant system are different scopes |

Rules for such lessons: keep a character's utterance apart from narration and from independently supported biology;
preserve conditionals, uncertainty, negation and speaker. A promise in a story is not a universal fact. Carry only the
checked observation, never surrounding commentary. Questions test causal transfer, contrasts, counterexamples and
explicit insufficient-evidence answers rather than animal names; tests and exams use different situations from the
exercises and stay out of teacher prompts and question-only assessment. Format, answer-length balance, provenance and
editorial review are separate checks; none of them certifies semantics or proves learning. Source sentences are
re-annotated in a new bounded batch, review-only, as described in [training.md](training.md); external authoring and
critique produce proposals for review and never rewrite lessons or feed learner feedback by themselves.

## Results

The school's measured scores. Tests right of the test questions counted; four options, chance 25%. Controls in the
order no lesson / questions shuffled / key sentences removed / questions from another lesson.

| date | build | data | exec | score | baselines | meaning |
|---|---|---|---|---|---|---|
| 2026-10-16 | release 0.4 | 25 primary courses, 26 992 tests | CPU | 11 730 (43.5%) | chance 25% | the 0.4 release value |
| 2026-10-16 | release 0.4, `gparse` parses | same | GPU | 11 817 (43.8%) | chance 25% | GPU-enhanced parsing is an approximant: other trees, same pupil |
| 2026-10-23 | main | 25 primary courses with the books' chapters, 49 635 tests | CPU | 19 455 (39.2%) | longest option 25.1%, first option 25.4% | with current-lesson priority, the kind base and counting; the books (half of all tests) pull the sum down |
| 2026-10-29 | main | one tale lesson, 12 questions, retained state examined in a fresh process | CPU | 7 / 12 | untrained state 4 / 12 | retention of what the lesson taught; not generalization |
| 2026-11-04 | main | ten courses (commands and nine primary), 7 283 tests, 2 556 exam questions | CPU | 3 259 (44.7%), exam 1 000 | 1 883 / 2 375 / 2 309 / 2 175 | current pupil; all controls red |
| 2026-11-04 | main | twelve other primary courses, 17 260 tests, 5 450 exam questions | CPU | 8 723 (50.5%), exam 2 466 | 4 484 / 6 039 / 6 023 / 5 594 | current pupil; all controls red |
| 2026-11-04 | main `55f595a3f` | the ten and the twelve courses above, 24 543 tests, 8 006 exam questions | CPU | 11 982 (48.8%), exam 3 466 | 6 367 / 8 414 / 8 332 / 7 769 | rerun after the day's merges: traces byte-identical to the two rows above; two runs of the ten identical; all controls red |
| 2026-11-04 | main `55f595a3f` | 25 primary courses with the books' chapters (the 2026-10-23 set), 49 693 tests, 15 757 exam questions | CPU | 19 640 (39.5%), exam 5 757 | 12 684 / 14 837 / 14 727 / 13 990; longest option 25.1%, first option 25.4% | all 25 courses' controls red; the books: 7 335 of 24 324, exam 2 191 of 7 455 |
| 0.3 → 0.4 | releases | maths student, two-fold `cv` on SVAMP train, 2 100 problems | CPU | world reader 297 → 513; book's way 1 116 → 1 116 | | the tree reader for word problems; the book's way is unchanged through 2026-11-04 |

Further release-to-release values (SVAMP test, LAS) are in [release 0.4 notes](../releases/0.4.md) and
[release 0.5 notes](../releases/0.5.md).

## The student's vidtvorant

`world/src/knowledge.rs.groupant/student-vidtvorant.groupant` is a fast executable shadow of the pupil: it mirrors
the code's decision structure (tail, anchors with the current lesson first, targets, own support, gates, lesson-first
choice, ties to the first shown, the guess) over bags of stemmed words instead of frames, and fits its weights and
gates to the pupil's own choices, never to the right answers. It meets none of the 1/2/5/10% caps, so it is a
diagnostic, not a substitute; its finite fit is recorded in its groupant. Run:

```sh
cargo run --release -p world --example student_probe -- OUT/1 commands geography …   # repeat into OUT/2 … OUT/5
cargo run --release -p mova-approx --example student_vidtvorant -- $MOVA_DATA/school/courses OUT REPORT commands geography …
```

## Limits

- The pupil keeps frames, not a world model: knowledge spread over many lessons, rules applied to new instances and
  conditions evaluated against a scene are its weakest points.
- Retained evaluation is a separate explicit mode; the existing courses are not migrated to it, and each course needs
  its own state.
- Odd and even lessons are not independent for changes about memory or reading order.
- Most measurements are primary-school English. Other languages have manuals, corpora and adapted courses, not pupils.
- The teacher's lesson text and corrections are trusted only after the gates; format checks do not prove semantics.
