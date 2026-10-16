# Documentation

Start with [architecture.md](architecture.md): the map of Mova, its pipeline, its teacher loop and its crates. Each crate also has a README next to its code.

## Overview

- [architecture.md](architecture.md) — the pipeline from text to answer, the teacher loop, the students, the crates.
- [design-three-levels.md](design-three-levels.md) — the design note on three levels of logic (global knowledge, domains, context), with the measurements of its first days.
- [gpu-exec.md](gpu-exec.md) — the two execution modes, CPU only and GPU enhanced, and what each GPU stage measures.
- [../CHANGELOG.md](../CHANGELOG.md) — what changed in each release; [release-0.4.md](release-0.4.md) and [release-0.3.md](release-0.3.md) — notes and metrics of releases 0.4 and 0.3.
- [mysticism/](mysticism) — texts on the background ideas behind Mova, and [Gali](mysticism/gali.md), the working mode for AI instances that build Mova with a human operator.

## Language: trees and the dialect

- [en-explained.md](en-explained.md) — the English parser and compressor step by step, in plain terms, with real `en explain` output.
- [dialect.md](dialect.md) — the Mova UD dialect, treebanks, and converters between conventions.
- [expert.md](expert.md) — an expert system for English grammar: seeds and rules that check the annotator.
- [parsers.md](parsers.md) — the collection of open English parsers, which run, and how they compare on the school.
- [derive.md](derive.md) — trees by logical derivation: attachment rules with reasons.
- [induce.md](induce.md) — the model derives its own tree-repair rules from its errors.
- [selfplay.md](selfplay.md) — self-play: the model changes its own rules cheaply, tests, and keeps what helps.
- [wiktionary.md](wiktionary.md) — dictionaries built from raw Wiktionary dumps.
- [seeds/](seeds/) — the seeds explained, one page per folder: English morphology, the noun phrase, the clause, the lexicon, typical errors, the global level, Mova's UD dialect.

## Pragmatics and knowledge

- [pragmatics.md](pragmatics.md) — frames: who did what to whom, why, and what came next; the reader, its yardstick, its lexicons, its rules.
- [pragmatics-algorithms.md](pragmatics-algorithms.md) — algorithms from the literature and how each fits a deterministic Rust model.
- [pragmatics-expressions.md](pragmatics-expressions.md) — idioms, proverbs, phrasal verbs and slang read by their real meaning.
- [pragmatics-tools.md](pragmatics-tools.md) — the collection of open pragmatics software and models (tests and measurement).
- [absurdity.md](absurdity.md) — the absurdity matrix: how plausible an event is, the central common-sense tool.
- [nerves.md](nerves.md) — the graph that ties words to code, worlds and the self.
- [events.md](events.md) — event statistics of books: who did what to whom.
- [rag.md](rag.md) — documents expanded into layers for retrieval, and what Mova provides without AI.

## Schools and students

- [school-making.md](school-making.md) — how the courses are made: the simulation, the pipeline, the voice, the limits.
- [school-format.md](school-format.md) — the school corpus format (CoNLL-U blocks).
- [school-tools.md](school-tools.md) — the checks and safe edits of lessons and the pupils' dictionary.
- [school.md](school.md) — principles of Mova's own annotation for all languages (a draft).

## Measurements and data

- [../tests/README.md](../tests/README.md) — unit tests and the benchmark commands, with Mova's results.
- [../corpus/open/](../corpus/open/) — the open corpus: tales, questions, summaries, treebanks, maths scripts, contrast sets.
- [../papers/](../papers/) — one page per scientific work Mova uses.
- [eval/](eval) — evaluation lists, e.g. the StepGame questions left out of the clean number, with reasons.
- [value/](value) — ablation reports: what a component is worth.
- [usage.tsv](usage.tsv) — the manual log of where each paper and book is applied in the code.
- [math-library.md](math-library.md), [science-library.md](science-library.md) — how the maths and science libraries were collected.

## Guides

- [../train/](../train/) — how to retrain Mova and rebuild it for your own domain, for people and AI agents.

## Licences

- [../LICENSE.md](../LICENSE.md) and [../licence/](../licence) — the Elamur Cosmic License and its annexes; [../licence/DATA.md](../licence/DATA.md) — the licences of data and models file by file.

## Plans and notes

- [context.md](context.md) — the long design notebook: layers, context, memory, domains (history included).
- [local-models.md](local-models.md) — local LLMs tried on the GPU (2026-10-15) and why they were set aside.
- [competitions.md](competitions.md) — where to submit the model.
