# Changelog

What changed in each release of Mova. Measurements are in each release's notes ([releases/](releases/)) and the school results in [docs/school.md](docs/school.md#results).

## Unreleased

Nothing yet.

## 0.5 (2026-11-05)

Notes: [releases/0.5.md](releases/0.5.md).

: the courses, lessons and school dictionaries left the repository for `school-0.5.tar.gz` (`mova fetch 0.5 school`, installed into `$MOVA_DATA/school`; `MOVA_SCHOOL_DATA` picks another folder).
### Licences

- **Code and data have separate licences** (2026-11-05): the Elamur Cosmic License covers only the code; Mova's data (the in-code English dictionary, the school's lessons and dictionaries, the corpora, the models) is under CC BY-SA 4.0, or its sources' stricter terms. Each data module has its own `LICENSE.md` with a machine-readable header and full citations ([licence/DATA.md](licence/DATA.md)); the in-code dictionary's ([en/data/LICENSE.md](en/data/LICENSE.md)) is compiled in and printed by `en licence`, outside the fingerprinted blob. `mova fetch` now requires the dictionaries archive to be CC BY-SA 4.0.

### Added

- **🐚 Nautilus** ([tools/nautilus](tools/nautilus/README.md), `mova-nautilus`): the dictionary maker, one of Mova's animals. Native dictionary layers from a course or corpus (Mova's own lemmas, UPOS, counts and examples; meanings from the pupils' dictionary), English through the deputy planner, a bounded standing loop with review-only proposals, and its teacher through its own `mova-terminal` (the operator's word required). First draft: the commands dictionary (`$MOVA_DATA/school/dictionaries/commands.tsv`, in `school-0.5.tar.gz`); a Wikipedia layer is CC BY-SA and stays outside the repository.
- **🐢 Sea Turtle** ([tools/turtle](tools/turtle/README.md), `mova-turtle`): the school maker, one of Mova's animals. `turtle check` (the school's own rule, shared with `world knowledge check` through `world::knowledge::lesson_problem`, and the answer marks), native gap questions from Nautilus layers, English through the deputy planner, and the same standing loop over the courses.
- **The Judaism dictionary** (`$MOVA_DATA/school/dictionaries/judaism.tsv`, in `school-0.5.tar.gz`, built by a dictionary tool outside this repository): a layer on the primary school dictionary. It covers every word of the English Tanakh (JPS 1917: the Torah, the Prophets, the Writings) and the English Sefer Yetzirah. For each word, the sense in the Tanakh and Jewish tradition and the tradition's interpretation come first (written by the teacher), then the corpus sentences with their verse, then the primary examples. `requests --update` grows it: new words are written, and an entry is reviewed only against sentences it has not seen (the teacher keeps it or rewrites it). A Tanakh round added the missing words and rewrote the entries that needed it.
- **Ancient Hebrew Torah** (`tools/macula`): MACULA Hebrew trees into dependency trees, measured against the UD PTNK test. It also builds an English dictionary of the Torah's words (BDB 1906, OSHB) with examples in layers: Hebrew, word for word, JPS 1917.
- **Translation and ancient languages**: `translate` and `translate many` (an interlingua translator measured on the parallel Tanakh), the Ancient Hebrew parser `hbo`, `langseg`, `ocrfix`, `macula prag`, `targums` and `tanakh3` ([tools/translate](tools/translate/README.md), [tools/hbo](tools/hbo/README.md), [tools/macula](tools/macula/README.md)).
- **`yom-hadin`**: a prototype that extracts laws (Torah, Jerusalem Talmud, samples of Hammurabi, Ukrainian and US federal law) into layered verdicts with lenses, checked on hand-checked sentences.
- **The autoencoder diagnostic** ([tools/roundtrip](tools/roundtrip/README.md), [docs/english.md](docs/english.md)): text → tree → frames → world and back (`run`, `blocks`, `mixed`, `site-learn`, `compare`), with an exact byte-for-byte gate and damage controls. Frames keep tense, aspect and modality (`tam`), every word and clause link, and punctuation from clause structure; the "no word left out" steps (`frame-fragment`, `frame-rest`, `ent-rest`, `rt-nominal`, `world-bare`).
- **Frames and the reader**: frame anchors (`Ctx::anchors`); corpus invariant checks (`prag::invariants`); reader steps `time-more`, `acl-own`, `imp-do`, `vn-gap`, `vn-stable`; implication signatures for verbs (`prag/data/signatures.tsv`, `factuality`) and *if* clauses as irrealis (`cond-irrealis`). Every step can be switched off with `PRAG_OFF`.
- **Coreference**: an *it* classifier (pleonastic and event uses) and *they / them / their* for a singular collective noun (`coref-notional`).
- **The school student** ([docs/school.md](docs/school.md)): `world knowledge diagnose` sorts failed course questions by the failing step; the student reads stranded prepositions (`q-strand`), *to* as the recipient (`student-to`), a quote as the patient of *ask/say* (`student-quote`), numbers as quantities (`student-qty`), irrealis like negation (`student-irrealis`), and breaks ties against an unsupported longer option (`student-extra`).
- **Course controls** (`PRAG_OFF=course-controls`): four controls that must go red in every course report: no lesson, questions shuffled, key lines removed, and options asked under a question from the lesson half the course away.
- **School tools**: `world knowledge bench` (quick, medium and full tiers), `MOVA_STRENGTH=mask` (opt-in), retained evaluation from a saved state in a fresh process (`cargo run --release -p world --example retention_probe`, [docs/school.md](docs/school.md)), and `mova-enrich improve`, a bounded study/propose/review/learn/assess step ([docs/school.md](docs/school.md)).
- **Biology lessons 406–415** on tales from *Where Animals Talk* (causal transfer, compatible causes) ([docs/school.md](docs/school.md)).
- **Parser diagnostics** ([en/README.md](en/README.md)): `en parse-diag` (errors by depth, beam oracle, cascading errors, PP attachment), `en construct-eval` (sentences grouped by construction, with floor files and a negative control), the share of complete trees in `parse-eval`, `EN_BEAM_CURVE=1`, and the conjunct, subcategorisation and distance feature groups (`EN_PX`; `en.bin` is not retrained with them).
- **`gparse`**: `gparse plan` (the work of a run without loading the model), periodic dev evaluation (`GP_DEV_EVERY`, `GP_DEV_QUICK`) and the experimental `GP_GELU=tanh`.
- **Open parsers for fr, la, grc, hbo and he** as witnesses and test references ([docs/parsers.md](docs/parsers.md)).
- **Groupants** ([docs/groupants.md](docs/groupants.md)): a `<file>.groupant/` sidecar with models and typed obernant families for every Rust source, checked by `mova-modelcheck` (`groupants`, `inventory --require-all`, `changed`, `syntax`, `sidecar-links`, `relink-evidence`, `extract-table`, `compose-registry`); module overlays ([docs/animals.md](docs/animals.md)); nonexclusive `global::placement` decisions ([docs/knowledge.md](docs/knowledge.md)).
- **`mova-algorithms`** ([algorithms/](algorithms/README.md)): deterministic algorithms with tests, groupants and composition harnesses, with adapters into the annotation and conversion gates, coreference mention matching, the assignment score and spatial inference.
- **`mova-correspondence`**: typed correspondences (`models/correspondences.json`) that bind algorithm groupants to Mova sources.
- **`mova-approx`**: executable Rust approximants with 1%, 2%, 5% and 10% caps ([docs/mathematical-modeling.md](docs/mathematical-modeling.md)).
- **The sea animals** ([docs/animals.md](docs/animals.md)): Fish annotation workers and the Fish terminal adapter, the Crab deputy (with its native deputy planner from the `mova-octopus` crate: `crab serve`, `crab ask`); the Fish crates `fish-swarm` (prepared queue supervisor), `fish-priority` (cooperative priority dispatch) and `fish-batch` (bounded batch preparation), the Eel learner, the Whales, the Sea Spider, a polite link downloader running as MovaBot ([docs/animals.md](docs/animals.md)), and the agent runner.
- **Reviewed annotation** ([docs/training.md](docs/training.md)): `cycle promote-reviewed` and `rollback-reviewed`.
- **`mova-twin`** ([tools/twin](tools/twin)): prospective cost predictions, measured runs and online calibration, with Lean checks (`formal/Twin.lean`) and GPU calibration.
- **`agent-mova`** ([tools/agent-mova](tools/agent-mova/README.md), [docs/animals.md](docs/animals.md)): bounded Mova-native agent capabilities (source-window location inference, retained location recall, bounded report collection) with retained episode evidence.
- **The command school** (`courses/commands` of the school, `school-0.5.tar.gz`): a course on English commands, with an anonymized command corpus and exam used by Crab (`crab exam`, `crab audit`).
- **Lean relation laws**: scoped relation composition ([formal/RelationComposition.md](formal/RelationComposition.md)) and the Schröder equivalences with the right residual ([formal/RelationResidual.md](formal/RelationResidual.md)).
- **Parser and translation comparisons**: `parsecmp agree` (pairwise agreement of parsers per file on the same tokens) and `translate many --parallel-only on` (only the verses every measured translation has).
- **Dolphins** ([docs/animals.md](docs/animals.md#the-dolphin-pod), [tools/crab](tools/crab/README.md#dolphins)): a configurable pod of named LLM terminals held by Crab through `crab-tmux-worker` (one host-wide lock per Dolphin, adoption of a running session, graceful stop, a pin for self-updating CLIs), the public consultant and documenter prompts, a durable consult queue that delivers only to an idle Dolphin, optional Crab-initiated consultations kept as review records, and English commands that always name the Dolphin (a quoted `text` slot, `group_aliases`, collective capabilities). `mova-crab check` validates a configuration.

### Changed

- **The 🐋 Whales** (`tools/whale`, crate and binary `mova-whale`) write big articles — for example an in-depth description of a scientific paper — on special missions given by a deployment's own prompt and specification; dictionaries move to 🐚 Nautilus and the school's questions, lessons and exams to 🐢 Sea Turtle ([docs/animals.md](docs/animals.md)).
- **The training teacher** is Codex CLI through the shared `teacher` crate (`mova-teacher`, [docs/training.md](docs/training.md)); there is no fallback to the Claude API. The cycle runs only with `CYCLE_TEACHER=codex` and `CYCLE_TIERS=0`; the tiered teacher cascade is gone.
- **`gparse`** uses fused attention softmax on CUDA by default, in prediction and in training with an analytical backward pass (an approximant; `GP_FUSED_SOFTMAX=0` / `GP_TRAIN_FUSED_SOFTMAX=0` select the reference); ordinary prediction skips unused scores (`predict_scored` keeps them).
- **Lesson training** reuses evaluations (`MOVA_LEARN_REUSE=0` selects the reference); course lessons are read with references taken off (`strip_citations`).
- `exec::parse_many` returns errors instead of silently dropping sentences.
- Approximant profiles are 1/2/5/10% (were 1/5/15/25%).
- Comparison receipts use schema 3; exams carry `assessment_sha256`.
- `en::text::lcs` keeps two rolling rows (same results, fewer allocations).
- The `translate` interlingua lexicon takes no phrase that holds a modal ("may Yahweh").
- Crab embeds the deputy planner (`crab serve`); the standalone `mova-octopus serve|angel` modes remain available.
- The docs describe the current state only; changes are in this file and the release notes.
- `install.sh` also fetches Open English WordNet 2025 (JSON, SHA-256 checked) into `$MOVA_DATA/raw/en-oewn/json`;
  a missing WordNet now stops the world model with a message that says how to install it (`MOVA_OEWN=optional`
  runs without it on purpose).
- CI builds and tests every standalone workspace under `tools/`, and `tests/ci-standalone-list.sh` fails when one
  is missing from the CI matrix.

### Fixed

- `gparse`: empty tokenized sentences no longer crash CUDA batches or the Eisner decoder.
- Reader: no apposition across a quote (`appos-quote`); a *wh*-copula inside a longer sentence (`wh-copula`).
- `math::nt::gcd(0, 0)` returns 0 instead of panicking.
- `school-sents usfm` keeps the punctuation after Strong-tagged `\w …\w*` marks (KJV, WEB, ASV).
- Fish preparation accepts the six pragmatic labels only in their canonical spelling; the Fish terminal adapter refuses the Codex update dialog; the agent-runner watcher logs health changes after its queue is exhausted.
- Bugs found through the groupant models: a foreign assessment accepted as the comparison baseline, duplicates admitted by the lexical-priority gate, world values depending on `HashMap` order, world-store deltas that dropped tags and lemmas, an unbounded delta cursor and cyclic ancestry, a sign-tail NaN, a nondeterministic pragmatic gain ratio, duplicate-link credit inflation, `qagen` overwriting artifacts on malformed JSONL, silent shape truncation in the linear-system checker, overflows in registry allocation and ontology snapshot slicing, and LF/CR framing in registry labels and partial Codex JSONL results ([docs/groupants.md](docs/groupants.md#lessons-from-model-reviews)).
- Biology lessons 033, 049, 212, 287 and 342 no longer overgeneralise about eggs; lesson 067 says correctly where water is absorbed.
- Sea Spider: an IPv6 address is public only inside global unicast `2000::/3`, minus Teredo and the other
  `2001::/23` blocks and the documentation ranges; IPv4-mapped, NAT64 and 6to4 addresses take the verdict of their
  embedded IPv4, so a hostile DNS answer can no longer reach a private or loopback IPv4 through them.
- Octopus: a prompt over the 8192-byte budget whose cut falls inside a multi-byte character is refused for its length,
  not with a UTF-8 error.
- World: a missing Open English WordNet is an error, not a silent empty WordNet.
- The in-code dictionary no longer holds personal data from the EWT e-mail and web genres: 415 lemmas that were e-mail
  addresses, links, `@handles` or telephone numbers are `<redacted-N>` placeholders (lemma numbers kept) and their 321
  form rows are gone (`en lexicon-redact`). The dictionary fingerprint changed, so `en.bin` must be retrained.
  Organisation domain names seen as words (such as `Newsday.com`) are not personal data and stay.

### Removed

- The 15% and 25% approximant profiles.

## 0.4 (2026-10-16)

Notes: [releases/0.4.md](releases/0.4.md).

### Added

- **The tree reader for word problems** (`world treeread`, `school_tree::read_tree`): told numbers, actors, verbs, receivers and comparisons are read off the dependency tree into the maths student's world.
- **Two execution modes** ([docs/gpu-exec.md](docs/gpu-exec.md)): CPU only (the default, exact) and GPU enhanced (`--features gpu`, `MOVA_EXEC=gpu`), where each GPU stage is exact or a measured approximant.
- **`induce` and `selfplay` on the GPU**: break counting as an exact integer CUDA kernel, with rules and reports identical to the CPU mode.
- **The GPU parser** ([`gparse`](gparse)): a transformer trained from scratch on the GPU, also usable as a library.
- **The schools**: primary-school knowledge courses with lessons, exercises, tests and exams, and the pupils' explanatory dictionary.
- **`install.sh`**: downloads the model `en.bin` into `$MOVA_DATA/en/models/` with a SHA-256 check.
- **Corpus 0.4** (<https://mova.elamur.ai/free/en/releases/0.4/>), under CC BY-SA 4.0 as a whole: the school's books, the agreed school treebank, silver and bronze trees, the pupils' dictionary, maths scripts, contrast sets.
- This changelog, a Limits section and a licences table in the README.
- **One Cargo workspace** at the root: `cargo build --release` builds every CPU crate into `target/release/` with no CUDA (the GPU parser on request), with one `Cargo.lock` (the same dependency versions as the per-crate locks of 0.3).

### Changed

- **One model, `en.bin`**: trained on UD English EWT, ESL Spoken and the agreed school trees; every default points at `$MOVA_DATA/en/models/en.bin`. The model is not in git.
- **Licences**: the code moved from MIT OR Apache-2.0 to the Elamur Cosmic License ([LICENSE.md](LICENSE.md), [licence/](licence)); the downloadable models are under CC BY-SA 4.0.
- **Docs, comments and READMEs in English** throughout; private paths replaced by `$MOVA_DATA`.
- Internal tooling and notes (the site server, the sync and paper tools, the coder, the old story worlds) are no longer in this repository.

- **Measured** (2026-10-16): LAS on the tales test 77.38 → 79.69 and on EWT test 79.86 → 80.68; SVAMP (`math` solver) 64.0% → 64.3%; bAbI 99.8% → 99.4% and its contrast set 99.7% → 99.4% (see Known issues); StepGame and SpartQA unchanged.
- The two unfinished courses (programming, philosophy) are no longer part of the release.

### Known issues

- `en.bin` misreads *do*-support questions with a long subject ("Does the box of chocolates fit in the box?"): bAbI task 18 92.3%, task 16 95.3%. The same code with the 0.3 model gives 99.8% on bAbI.
- The maths reader has no verb class for *give away*; the unit test `question_numbers_must_be_used` is ignored (still open; see the 0.5 notes).

### Removed

- The 0.3 model `ud-ewt-eslspok.bin` is not carried on.
- The maths student's old window-of-words reader; the tree reader is the only reader.
- The story worlds without a teacher's check moved out of the default build (the schools replace them).

## 0.3 (2026-10-03)

Notes and metrics: [releases/0.3.md](releases/0.3.md).

### Added

- **Self-retraining with states** ([docs/training.md](docs/training.md)): a tree of delta states with rollback, a brain and decision letters per state, and the family training loop on a Unix socket with live control.
- **Learning on big texts**: samples from the tree store judged by the absurdity matrix, UD grammar constraints and a teacher check; a negative list of rejected rules.
- **The absurdity matrix** ([docs/knowledge.md](docs/knowledge.md)): verbs × nouns scored 0–4, plus a fairy-tale layer.
- **Induced tree-repair rules** ([docs/training.md](docs/training.md)) and **trees by logical derivation** ([docs/english.md](docs/english.md)).
- **StepGame corrected and contrast sets**: a reader of new phrasings next to the learned perceptron.
- **Expressions with their real meanings** ([docs/pragmatics.md](docs/pragmatics.md)) and the **tree store** (`world store`).
- Corpus 0.3 on research.elamur.ai.

### Changed

- SpartQA-Human yes/no 60.1% → 75.5%, relation questions 50.6% → 63.6%; SVAMP 59.7% → 64.0%; LAS on EWT test 79.82 → 79.86.
- The first verified self-training gain shipped: `nsubj/obj → obl` when the word has a preposition.

### Removed

- Nothing of note.

## 0.2 (2026-10-01)

The first free release.

### Added

- The English UD parser (`en`), pragmatics (`prag`), story worlds (`world`), the maths solver (`math`) and the compiled global knowledge (`global`).
- bAbI 99.8%, StepGame 96.3%, SpartQA-Human yes/no 60.1%, SVAMP 59.7%, LAS on EWT test 79.82.
- Corpus 0.2 on research.elamur.ai.
- Licence: MIT OR Apache-2.0.
