# Architecture

This page is the map of Mova: how a text flows through the language core, how
the knowledge is organised in three levels of logic, how the model learns, how
its students are measured, and which crate does what. What Mova is, its mascot
and the contract of its three layers (linguistic, combinatorial, operational)
are in [the hybrid-system page](hybrid-system.md). Details live in each crate's
README and in the pages linked below.

## The pipeline: from text to an answer

```text
text ──► en ──► UD tree ──► prag ──► frames ──► world ──► answer + reason
          │                   │                  │
          │  tokens, tags,    │  who did what to │  entities, holdings,
          │  heads, relations,│  whom, where,    │  states, what changed;
          │  lemmas, FEATS    │  why, what next  │  the students read here
          ▼                   ▼                  ▼
        global: compiled knowledge — verb classes, measures, values, absurdity matrix
```

1. **`en` parses.** A sentence becomes a Universal Dependencies tree (UPOS,
   XPOS, FEATS, lemma, head, relation). The tagger and the arc-standard
   transition parser are averaged perceptrons with beam search. The lexicon is
   compiled into the binary. On the GPU, [`gparse`](../gparse) gives the same
   kind of tree with a transformer trained from scratch. See
   [English](english.md).
2. **`prag` reads the tree into frames.** A frame is one event or fact: its verb
   (with negation), agent, patient, recipient, place, attributes, obliques, and
   links to its cause, to what came next and to what was said. Roles come from
   VerbNet classes where the verb is known, and from tree heuristics otherwise.
   The same crate labels speech acts, implicatures and irony candidates. See
   [pragmatics](pragmatics.md).
3. **`world` keeps the world of the text:** entities with their states, who has
   what, and the order of events. The readers and students live here.
4. **`global` is the compiled knowledge underneath:** verb classes (give, get,
   lose, have), measures (a week is seven days), values as principles,
   shortcuts between concepts, and the absurdity matrix
   ([knowledge](knowledge.md)). It is compiled from seed files at build time:
   no disk reads at run time, and a link to an undeclared concept breaks the
   build.

No component works from a bag of words. A reader matches the roles of frames,
never the raw words of whole lines.

## Three levels of logic

Mova's logic has three levels. They are an axis of computation; where
knowledge is stored (skeleton, important graph in RAM, durable global graph) is
a separate axis described in [knowledge](knowledge.md).

| level | what it holds | when it runs | how it is built |
|---|---|---|---|
| **1. Global** | principles of existence and values; exact mathematics (rationals, expressions, units, number theory); the world model (concepts, parts, typical actions and consequences); shortcuts and links between all of these | always, on every sentence | compiled into the binary: Rust types and tables from seeds (`build.rs`), no disk reads |
| **2. Domain modules** | one field each, such as customs classification (`onto hs`), research-level mathematics with Lean, tables and statistics (`mlab`); many, each separate | only when the context calls it | a compiled module with its own version, licence and access level |
| **3. Context** | the current document, task or conversation: the story world, pragmatics, working memory, the quantities of a task, the justification of each step | the whole time the text runs | working state with access to the persistent graphs: `world`, `prag`, `coref`, `math::steps` |

**Global shortcuts** join knowledge in one hop: "the apple falls → accelerates
→ hits the head → hurts"; "gave away half → divide by 2 → the quantity
decreased"; "lying harms trust". Every shortcut carries an explanation and the
law or principle it comes from, so every judgment shows why.

**Values are part of the global level, not a filter on the side.** Humanism,
basic Christian values, friendship, peace and harmony are the principles by
which the SLM judges a story (who harmed whom, who helped, what is fair), its
motivation (what to strive for, what to avoid) and its answers (do not advise
harm, explain respectfully). A principle is a declarative seed: essence,
conditions and exceptions, examples, source. A conclusion always prints which
principle fired. The SLM judges its own work by the same principles
(`global::self_judge`): many branches tried is perseverance, an independent
check is diligence, asking for help when unsure is humility, a confident error
violates humility.

**Context judgments are predicates over the world state** (who owns what, who
is in what state, who caused what); concepts and principles come from level 1.
Whatever the context cannot explain with level 1 links it prints as a **gap**
(`world values`: "action gnaw — no class"). Gaps go into a learning queue: for
durable graph knowledge, RAM activation, or a slower, tested revision of seeds
and parameters. Level 1 covers general human knowledge at the level of a
beginning scientist or a strong linguist; where it is too thin, level 1 changes
through a tested build. Cultural or trick logic (a culture where a death is a
joy) is not level 1: it is a domain or a viewpoint module that gives its answer
next to the classic one, with its source.

**Concept codes.** A concept's code is `lang:word` or `lang:word:sense`:
`en:give_up`, `en:apple:fruit`, `en:apple:company`. Bare names in the seeds are
English concepts. One concept per code is a build gate.
`global::senses(lang, word)` lists a word's senses; `global::sense_of(lang,
word, text)` picks one by the cue words in the text (seeds in
`global/seeds/senses/`).

### Interfaces

```text
Global  ── concept(id) → world-model class
        ── law(name) / eval(expr) → exact core (math; later physics, chemistry)
        ── principle(situation) → judgment + which principle
        ── shortcut(from, to) → chain of links with explanation
        ── needs_domain(query) → which domains and why

Domain  ── id, version, licence, access (public / extended / private)
        ── covers(query) → [0..1], why
        ── solve(query, context) → answer + justification | "not mine"
        ── compile() → classes (at module build time, not at run time)

Context ── document state: world, quantities, pragmatics, memory
        ── route(query): global → domain (covers ≥ threshold) → escalate
        ── justify(answer) → steps, laws, principles, sources
```

The context is the conductor: it tracks what the text is about, passes every
sentence through the global level, decides whether to call a domain, composes
the justification, and, when even the domains cannot cope, escalates with the
task already decomposed. Routing is transparent: every decision prints the
coverage scores and the threshold. A disagreement between the global level and
a domain is an escalation, like a disagreement between models in `prag`.

### Which crates implement which level

| level | crates and modules |
|---|---|
| global | [`global`](../global) (values, shortcuts, verb classes, senses, categories, feelings and object uses from seeds), [`math`](../math) (the exact core: rationals, expressions, units, number theory, polynomials), [`onto`](../onto) (the compiled world-model core and word-sense selection), the `en` lexicon |
| domains | `onto hs` (customs classification), `math` with Lean (research-level mathematics), [`mlab`](../mlab) (tables and statistics) |
| context | [`world`](../world) (tale worlds, the quantitative layer), [`prag`](../prag), [`coref`](../coref), `math::steps` and `deduct` (decomposing a task with justification) |
| conductor | [`layers`](../layers): the `Global`, `Domain` and `Context` types, the domain registry, routing with a threshold and a log, `Mode::{Answer, Explain(Training \| Debug \| UserAsked)}` |

`layers` makes no model call: `Route::Escalate` only marks the query.

### Compilation into classes

- **The global level compiles together with the SLM**, like the `en` lexicon
  and the `onto` core: curated seeds → `build.rs` → enums, tables and `match`.
  Words are constants; rules are integer comparisons.
- **A domain is a module behind the `Domain` trait** of `layers`, with the same
  core types and an access level (`layers::Access`: public, extended, private).
  Separately compiled, versioned and signed domain modules, and a release gate
  over access levels, are not implemented (see Direction).
- **Level 1 extensions are regression-gated.** A verb class that is also a
  feature of a student is extended only when the student's accuracy does not
  drop; otherwise the word enters as a concept for explanations only. A
  `category` block (time units, measures, part and whole) explains steps but
  does not start consequence chains; the negative control "the apple lies on
  the table" checks this.

### The problem solver

The maths student is the first consumer of the three levels:
- **global:** the exact core computes; verb classes (received, gave, lost) and
  "X per Y" rates are world-model shortcuts taken from `global`;
- **context:** the quantitative world of the problem (`math::qworld`,
  `math/src/qread.rs`: every number is a world command — owner, thing, action
  by verb class, rate — and the question is a query over the states),
  decomposition into subtasks (`steps`, `deduct`), a top-K judge, and the
  justification of every step;
- **domain:** research-level mathematics with Lean, separate in `math`.

In explanation mode a step is a chain of level 1 links ("lose → decrease →
subtract"); where a link is missing the step reads "teach me why", and those
reports feed level 1 seeds. During training the student never calls an LLM:
when its first opinion and its judge disagree and the judge is unsure, it
files a report (the branches tried, both opinions, the words unknown to
level 1) instead of answering.

### Direction

- Physics and chemistry as a global core of laws (kinematics, energy,
  momentum).
- Unit rules moved from `math::steps` into `global`, shared by the maths
  student, tale worlds and pragmatics.
- Domains built as hot-pluggable WASM modules through the `Domain` trait, with
  customs classification from `onto` as the first one.
- A "what-if" module in level 1: viewpoint strategies (comparison, state
  tracking, rebinding to the question) and viewpoint triggers (tricky, tale,
  cultural) learned from logged analyses, each behind a regression gate.
- Higher abstraction in level 3: judgments about judgments as a graph, using
  the level 1 vocabulary.
- A job queue with up-front memory and CPU estimates learned from the log;
  recursion as subtasks in the queue rather than the stack.

## How Mova learns: the teacher loop

Mova's parser and pragmatics learn from silver data that several witnesses
agree on and that a teacher has checked. The teacher never answers in Mova's
place.

```text
school sentences ──► drafts: en (CPU) + gparse (GPU)
                 ──► witnesses: Stanza, Trankit, spaCy-trf; open NLI models for pragmatics
                 ──► treeagree: written rules R1–R8 agree the trees, gates keep them valid UD
                 ──► prag agree: Mova's pragmatic labels stand unless agreeing witnesses and rules say otherwise
                 ──► teacher: Codex CLI through mova-teacher patches the agreed draft
                 ──► parse database: every version kept, agreed by weighted vote
                                     (teacher round 2, agreed seed 1; older teacher versions 8 and 4)
                 ──► trainer: the GPU parser fine-tuned on the fresh agreed trees, asynchronously
```

- [`tools/treeagree`](../tools/treeagree) agrees trees by rules, each with its
  evidence.
- [`tools/cycle`](../tools/cycle) runs the loop round after round and keeps the
  parse database.
- [`annot`](../annot) holds the teacher prompts. The teacher gets the agreed
  draft, the tree and a short summary of the document's temporary world, and
  answers with corrections only where it disagrees.
- [`teacher`](../teacher) (`mova-teacher`) is the shared transport: Codex CLI
  authenticated with ChatGPT, the configured Codex model, no fallback to the
  Claude API ([training](training.md)).

Mova also finds its own patterns: `world induce` learns readable tree-repair
rules from its own parsing errors (transformation-based learning, Brill 1995),
and `world selfplay` tries changes to its rule set on held-out data and keeps
only significant gains, with a journal of every experiment. Both, and the
self-study step `mova-enrich improve`, are described in
[training](training.md).

## The students

Mova is measured by what its students can do, never by the teacher.

- **The course pupil** (`world knowledge course <name>`) lives through the
  primary-school courses: biology, everyday life, history, maths in words and
  others. For every lesson it reads the explanation and facts into frames,
  practises on the exercises (learning weights for its features), then answers
  the tests, four options each, and an exam in formal language. Chance is a
  quarter. Every course report carries four controls that must score below the
  real student, and `--trace <file>` writes every counted test with its choice
  and reasons ([school](school.md#controls-in-every-course-report)). How the courses are written, their format
  and retained learning across restarts: [school](school.md).

  The school results, dated and with their data, are in [school](school.md#results).

  The site shows the traces as "How Mova does at school"
  (`/free/en/school-run/`) and every question and option, parsed with trees and
  pragmatics, as a corpus (`/free/en/corpora/questions/`).
- **The maths student** (`world school actors …`) solves word problems: it
  takes the told quantities, chooses the operation with a learned perceptron,
  and reads the story's world (who has what, who gives what to whom). A second
  reader takes the numbers off the dependency tree (`world treeread`).
  Decisions are made on the two-fold check on SVAMP's train part (`cv`); the
  SVAMP test is opened once per release, and its results are in the release
  notes.
- **The readers** answer questions on fables and tales: `world read` (paragraph
  summaries from the tree), `world read-qa` and `world read-events`
  (FairytaleQA, Xu et al. 2022), `world babi` (bAbI), `world stepgame`
  (StepGame, Shi et al. 2022; excluded items in
  [eval/stepgame-excluded.tsv](eval/stepgame-excluded.tsv)) and
  `world spartqa` (SpartQA, Mirzaee et al. 2021; NC data, measurements only).
  See the [world README](../world/README.md).

## Knowledge, durable learning and execution modes

- **Knowledge.** Understanding mechanisms and hard parameters (the skeleton)
  change through code revision and retraining; the important graph keeps
  operationally useful knowledge in RAM; the global graph durably records human
  and AI-derived knowledge with provenance. These are nonexclusive. The crate
  `global` (compiled seeds) is distinct from the global information graph. See
  [knowledge](knowledge.md).
- **Durable school learning.** A course run inside one process does not show
  retention. The retained school state stores a learned graph and role
  parameters and examines them in a fresh process from question-only input,
  without replaying lessons. See [school](school.md).
- **Execution modes.** Every pipeline runs **CPU only** (the default, exact) or
  **GPU enhanced** (`--features gpu`, `MOVA_EXEC=gpu`). A GPU stage is either
  exact or a measured approximant. See [GPU execution](gpu-exec.md).

## The crates

| group | crate | what it is |
|---|---|---|
| language core | [`en`](../en) | English UD: tokenizer, tagger, parser, morphology, lemmas; the compressor text → graph → text |
| | [`global`](../global) | compiled knowledge: values, shortcuts, verb classes, measures, senses, the absurdity matrix |
| | [`prag`](../prag) | frames, speech acts, implicatures, irony, expressions read by their meaning, the pragmatics labeller and agreer |
| | [`coref`](../coref) | coreference: a multi-pass sieve with a reason for every link; CorefUD scoring |
| | [`onto`](../onto) | the world model: concepts from WordNet, FrameNet and Wikidata, kept apart from the narrative |
| | [`annot`](../annot) | the teacher side of annotation: prompts, batches, gates, drafts |
| | [`teacher`](../teacher) | `mova-teacher`, the shared Codex CLI teacher transport |
| | [`tools/treebanks`](../tools/treebanks) | UD treebank facts (`treebanks/cards.tsv`, `INDEX.md`) from the release; converters between treebank conventions live in [`dialects/`](../dialects) and [`treebanks/`](../treebanks) ([dialect](dialect.md)) |
| world and students | [`world`](../world) | worlds of texts, schools, the course pupil, the maths student, readers, induce and self-play |
| | [`layers`](../layers) | the conductor of the three levels of logic |
| | [`math`](../math) | exact rationals, symbolic expressions, units with dimensions, step-by-step solving |
| | [`qagen`](../qagen) | training questions on tale paragraphs, written by the teacher |
| | [`kg`](../kg) | a compact knowledge graph of facts |
| algorithms and models | [`algorithms`](../algorithms) | `mova-algorithms`: deterministic algorithms with tests and groupants |
| | [`tools/modelcheck`](../tools/modelcheck), [`tools/approx`](../tools/approx), [`tools/correspondence`](../tools/correspondence) | groupant and registry checks, executable Rust approximants, typed correspondences ([groupants](groupants.md), [mathematical modeling](mathematical-modeling.md), [correspondences](correspondences.md)) |
| | [`tools/twin`](../tools/twin) | `mova-twin`: prospective cost predictions, measured runs and online calibration |
| | [`tools/enrich`](../tools/enrich) | `mova-enrich`: bounded corpus enrichment and self-study steps ([training](training.md)) |
| GPU | [`gparse`](../gparse) | the GPU parser: transformer encoder, biaffine heads, Eisner decoding; a library and a CLI |
| other domains | [`mlab`](../mlab), [`latex`](../latex) | a MATLAB-like language, LaTeX formulas |
| | [`yom-hadin`](../yom-hadin) | laws read out of the Torah and the Talmud as rules, jurisdictions as data, and a court that enumerates every way a case can be judged, with the why of each path (a study, not legal advice) |
| sandbox | [`vfs`](../vfs), [`wasi-host`](../wasi-host) | an in-memory file system and a WASI host for running untrusted modules |
| tools | [`tools/*`](../tools) | the teacher loop (`cycle`, `treeagree`), parser comparison (`parsecmp`), the round trip of the layers (`roundtrip`), the translator through a meta-world (`translate`), benchmarks (`bench`), jobs with memory and time limits (`jobs`, logging estimate and actual), data tools |
| workers | [`tools/crab`](../tools/crab), [`tools/octopus`](../tools/octopus), `tools/fish*`, [`tools/eel`](../tools/eel), [`tools/whale`](../tools/whale), [`tools/seaspider`](../tools/seaspider), [`tools/nautilus`](../tools/nautilus), [`tools/turtle`](../tools/turtle), [`tools/terminal`](../tools/terminal), [`tools/agent-runner`](../tools/agent-runner), [`tools/agent-mova`](../tools/agent-mova), [`tools/module-overlay`](../tools/module-overlay) | the sea animals of Mova, each a standalone workspace ([animals](animals.md)) |

## Where things live

- **The repo** holds code, seeds, lessons and compiled-in tables.
- **The data directory** (`$MOVA_DATA`) holds models, raw corpora, runs and the
  parse database. Results of runs never go into the repo.
- A **private tree**, not in the public repository, holds the little that is not part of the free release (deployment, internal tools).
