# Architecture

This page is the map of Mova: what the pieces are, how a text flows through them, and how the model learns. The details live in each crate's README and in the pages linked below. The design note on the three levels of logic, which much of this grew from, is in [design-three-levels.md](design-three-levels.md).

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

1. **`en` parses.** A sentence becomes a Universal Dependencies tree (UPOS, XPOS, FEATS, lemma, head, relation). The tagger and the arc-standard transition parser are averaged perceptrons with beam search. The lexicon is compiled into the binary. On the GPU, [`gparse`](../gparse) gives the same tree with a transformer trained from scratch.
2. **`prag` reads the tree into frames.** A frame is one event or fact: its verb (with negation), agent, patient, recipient, place, attributes, obliques, and links to its cause, to what came next, and to what was said. Roles come from VerbNet classes where the verb is known, and from tree heuristics otherwise. The same crate labels speech acts, implicatures and irony candidates. See [pragmatics.md](pragmatics.md).
3. **`world` keeps the world of the text.** It holds entities with their states, who has what, and the order of events. The readers and students live here: the course pupil, the maths student, and the readers of fables and tales.
4. **`global` is the knowledge underneath.** It holds verb classes (give, get, lose, have), measures (a week is seven days), values as principles, shortcuts between concepts, and the [absurdity matrix](absurdity.md). All of it is compiled from seed files at build time: no disk reads at run time, and a link to an undeclared concept breaks the build.

No component works from a bag of words. A reader matches the roles of frames, never the raw words of whole lines.

## How Mova learns: the teacher loop

Mova's parser and pragmatics learn from silver data that several witnesses agree on, and that a large model has checked. The large model never answers in Mova's place.

```text
school sentences ──► drafts: en (CPU) + gparse (GPU)
                 ──► witnesses: Stanza, Trankit, spaCy-trf; open NLI models for pragmatics
                 ──► treeagree: written rules R1–R8 agree the trees, gates keep them valid UD
                 ──► prag agree: our pragmatic labels stand unless agreeing witnesses and rules say otherwise
                 ──► teacher, in tiers: Haiku checks every sentence,
                                        Sonnet a tenth of Haiku's, Opus a tenth of Sonnet's
                 ──► parse database: every version kept, agreed by weighted vote (Opus 8, Sonnet 4, Haiku 2, seed 1)
                 ──► trainer: the GPU parser fine-tuned on the fresh agreed trees, asynchronously
```

- [`tools/treeagree`](../tools/treeagree) agrees trees by rules, each with its evidence.
- [`tools/cycle`](../tools/cycle) runs the loop as a daemon, round after round, and keeps the parse database.
- [`annot`](../annot) holds the teacher prompts. The teacher gets the agreed draft, the tree, and a short summary of the temporary world of the document, and it answers with corrections only where it disagrees.

Measured on 2026-10-15:
- with the agreed draft, Haiku keeps about 98.5–98.7 LAS of it, against 86.7 when it parses blind;
- Sonnet keeps 95–96.7% of Haiku's arcs, and Opus keeps 92–95% of Sonnet's;
- the GPU parser trained on the agreed school reaches school LAS 88.5 and EWT 85.7, against 84.5 and 80.7 for the perceptron trained on the same data.

## The students

Mova is measured by what its students can do, never by the teacher.

- **The course pupil** (`world knowledge course <name>`) lives through 25 primary-school courses: biology, everyday life, history, maths in words and others. For every lesson it reads the explanation and facts into frames, practises on the exercises (learning weights for its features), then answers the tests, four options each, and an exam in formal language. On 2026-10-15 it answered 11 730 of 26 992 test questions in CPU mode (43.5%), and 11 817 with the GPU parser. Chance is a quarter. How the courses are written: [school-making.md](school-making.md); the format: [school-format.md](school-format.md).
- **The maths student** (`world school actors …`) solves word problems. It takes the told quantities, chooses the operation with a learned perceptron, and reads the story's world: who has what, who gives what to whom. A newer reader takes the numbers off the dependency tree (`world treeread`). The headline measure is the two-fold check on SVAMP's train part, `cv`: 2 100 checks, the problems held out in turn, 1 116 right. The SVAMP test is opened once per release.
- **The readers** answer questions on fables and fairy tales (FairytaleQA, bAbI, StepGame, SpartQA). Their measurements are in [design-three-levels.md](design-three-levels.md).

## Self-directed learning

Besides the teacher loop, Mova can find its own patterns:
- [`world induce`](induce.md) learns tree-repair rules from its own parsing errors, in the style of transformation-based learning (Brill 1995). Every rule reads as a sentence with its evidence.
- [`world selfplay`](selfplay.md) tries changes to its own rule set, tests them on held-out data in parallel, and keeps only those that help significantly. It also keeps a journal of every experiment.

## Execution modes

Every pipeline runs in **CPU only** mode (the default, exact) or **GPU enhanced** mode (`--features gpu`, `MOVA_EXEC=gpu`). A GPU stage is either exact or a measured approximant. See [gpu-exec.md](gpu-exec.md).

## The crates

| group | crate | what it is |
|---|---|---|
| language core | [`en`](../en) | English UD: tokenizer, tagger, parser, morphology, lemmas; the compressor text → graph → text |
| | [`global`](../global) | compiled knowledge: values, shortcuts, verb classes, measures, the absurdity matrix |
| | [`prag`](../prag) | frames, speech acts, implicatures, irony, expressions read by their meaning, the pragmatics labeller and agreer |
| | [`coref`](../coref) | coreference: a multi-pass sieve with a reason for every link; CorefUD scoring |
| | [`onto`](../onto) | the world model: concepts from WordNet, FrameNet and Wikidata, kept apart from the narrative |
| | [`annot`](../annot) | the teacher side of annotation: prompts, batches, gates, drafts |
| | [`dialects`](../dialects), [`treebanks`](../treebanks) | the Mova UD dialect and converters between treebank conventions ([dialect.md](dialect.md)) |
| world and students | [`world`](../world) | worlds of texts, schools, the course pupil, the maths student, readers, induce and self-play |
| | [`layers`](../layers) | the three levels of logic and the routing between them |
| | [`math`](../math) | exact rationals, symbolic expressions, units with dimensions, step-by-step solving |
| | [`qagen`](../qagen) | training questions on tale paragraphs, written by the teacher |
| | [`kg`](../kg) | a compact knowledge graph of facts |
| GPU | [`gparse`](../gparse) | the GPU parser: transformer encoder, biaffine heads, Eisner decoding; a library and a CLI |
| other domains | [`weather`](../weather), [`mlab`](../mlab), [`latex`](../latex) | weather forecasts, a MATLAB-like language, LaTeX formulas |
| sandbox | [`vfs`](../vfs), [`wasi-host`](../wasi-host) | an in-memory file system and a WASI host for running untrusted modules |
| tools | [`tools/*`](../tools) | the teacher loop (`cycle`, `treeagree`), parser comparison (`parsecmp`), benchmarks (`bench`), jobs with memory limits (`jobs`), the free-release exporter (`freesync`), and data tools |

## Where things live

- **The repo** holds code, seeds, lessons and compiled-in tables.
- **The data directory** (`$MOVA_DATA`) holds models, raw corpora, runs and the parse database. Results of runs never go into the repo.
- **`tera/`** holds the little that is not part of the free release.
