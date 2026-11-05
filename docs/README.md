# Documentation

Mova is a Hybrid Combinatorial–Operational System with a Combinatorial Small Language Model (SLM) at its core. These
pages describe how it is built and how it works. Start with [the hybrid system](hybrid-system.md), then
[the architecture](architecture.md). Each crate has a README next to its code with the full command reference.

## Overview

- [hybrid-system.md](hybrid-system.md) — what Mova is: the three layers (language, combinatorial, operational), what
  is implemented and what is direction; the octopus mascot.
- [economics.md](economics.md) — why Mova changes where intelligence is spent: a persistent orchestrator, the SLM at the margin, large models as consultants, and what to measure.
- [architecture.md](architecture.md) — the pipeline from text to answer, the three levels of logic, the teacher loop,
  the students, the crates and where things live.
- [context.md](context.md) — context, layers and memory: the tale world, the SLM subsystems and what is inferred from
  context, with the gaps named.

## The language layer

- [english.md](english.md) — the English parser and compressor step by step, the grammar expert system, trees by
  derivation, and the round trip through the layers.
- [parsers.md](parsers.md) — the open parsers Mova uses as witnesses and tests, their licences and roles.
- [dialect.md](dialect.md) — the Mova UD dialect, treebanks and converters between conventions.
- [pragmatics.md](pragmatics.md) — frames (who did what to whom, why, and what came next), speech acts, implicature,
  expressions read by their meaning, the external pragmatics tools.
- [seeds/](seeds/) — the grammar seeds explained, one page per folder.

## Worlds, knowledge and the school

- [knowledge.md](knowledge.md) — the knowledge contract (skeleton, important graph, global graph), the absurdity
  matrix, the nerve graph, RAG.
- [school.md](school.md) — the primary school: how courses are made, their format, how the pupil reads and answers,
  controls, diagnosis, tools, retention in a fresh process, Mova-driven improvement, and the school results.
- [training.md](training.md) — how Mova is trained: the teacher transport, the training pipeline, measuring and gates,
  self-directed learning (induce, self-play), the annotation and storage contract, reviewed promotion, re-annotation.

## Workers

- [animals.md](animals.md) — the Octopus (an LLM that develops Mova, not part of its runtime) and Mova's Rust
  animals: Crab, Eel, Fish, Whales, Nautilus, Turtle, Sea Spider (and MovaBot), terminals, the Dolphin consultants;
  their protocols, evidence boundaries and custom modules.

## Groupants and mathematical modeling

- [groupants.md](groupants.md) — the groupant: the combinator node of a piece of code, its components, sidecars,
  tree, registry, coverage, the external reference library and the publication scope.
- [mathematical-modeling.md](mathematical-modeling.md) — the models every file and group needs: vidtvorants, typed
  obernants, reverse reconstruction, the mathematical combiner, executable Rust approximants.
- [correspondences.md](correspondences.md) — typed mathematical correspondences between algorithm groupants and Mova
  sources, and algorithm transfers through explicit adapters.

## Execution, data and releases

- [gpu-exec.md](gpu-exec.md) — the two execution modes, CPU only and GPU enhanced: stages, checks and reference switches.
- [data.md](data.md) — where data lives, licences, the maths and science libraries, Wiktionary, paper cards,
  competitions.
- [modules.md](modules.md) — add-on modules (for example dictionaries under their own licences): install from a URL or a torrent, verify, and load into Mova.
- [../CHANGELOG.md](../CHANGELOG.md) — what changed in each release; release notes:
  [0.5](../releases/0.5.md), [0.4](../releases/0.4.md), [0.3](../releases/0.3.md), all in `releases/`.
- [../LICENSE.md](../LICENSE.md) and [../licence/DATA.md](../licence/DATA.md) — the Elamur Cosmic License and the
  licences of data and models.

## Elsewhere in the repository

- [../tests/README.md](../tests/README.md) — unit tests and benchmark commands.
- [../corpus/open/](../corpus/open/) — the open corpus. [../papers/README.md](../papers/README.md) lists the works
  Mova uses, each linked to its card on Elamur Research; [usage.tsv](usage.tsv) logs where each is applied.
- [../train/](../train/) — how to retrain Mova for your own domain.
- [eval/](eval) — evaluation lists. Ablation reports of [tools/value](../tools/value/README.md) are run records, not pages.
- [assets/](assets) — the octopus artwork. [mysticism/](mysticism) — background texts and
  [Gali](mysticism/02-gali.md), the working mode for AI instances that build Mova: Mova's own working philosophy,
  not tooling; nothing in the build or the checks depends on it.

## Writing docs

New documentation extends one of the pages above. A new page needs a new subsystem; per-feature, per-experiment and
per-review notes do not become pages. Docs describe how things are and work; the only measurements kept are school
results and comparisons with earlier releases. Every relative link must resolve:
`cargo test -p mova-modelcheck --test doc_links` checks files and heading anchors in all public Markdown.
