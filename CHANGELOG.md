# Changelog

What changed in each release of Mova. Measurements are in the [README](README.md#measured) and in each release's notes.

## 0.4 (2026-10-16)

Notes: [docs/release-0.4.md](docs/release-0.4.md).

### Added

- **The tree reader for word problems** (`world treeread`, `school_tree::read_tree`): told numbers, actors, verbs, receivers and comparisons are read off the dependency tree into the maths student's world.
- **Two execution modes** ([docs/gpu-exec.md](docs/gpu-exec.md)): CPU only (the default, exact) and GPU enhanced (`--features gpu`, `MOVA_EXEC=gpu`), where each GPU stage is exact or a measured approximant.
- **`induce` and `selfplay` on the GPU**: break counting as an exact integer CUDA kernel, with rules and reports identical to the CPU mode.
- **The GPU parser** ([`gparse`](gparse)): a transformer trained from scratch on the GPU, also usable as a library.
- **The schools**: primary-school knowledge courses with lessons, exercises, tests and exams, and the pupils' explanatory dictionary.
- **`install.sh`**: downloads the model `en.bin` into `$MOVA_DATA/en/models/` with a SHA-256 check.
- **Corpus 0.4** (<https://research.elamur.ai/corpus/0.4/>), under CC BY-SA 4.0 as a whole: the school's books, the agreed school treebank, silver and bronze trees, the pupils' dictionary, maths scripts, contrast sets.
- This changelog, a Limits section and a licences table in the README.
- **One Cargo workspace** at the root: `cargo build --release` builds every crate into `target/release/`, with one `Cargo.lock` (the same dependency versions as the per-crate locks of 0.3).

### Changed

- **One model, `en.bin`**: trained on UD English EWT, ESL Spoken and the agreed school trees; every default points at `$MOVA_DATA/en/models/en.bin`. The model is not in git.
- **Licences**: the code moved from MIT OR Apache-2.0 to the Elamur Cosmic License ([LICENSE.md](LICENSE.md), [licence/](licence)); the downloadable models are under CC BY-SA 4.0.
- **Docs, comments and READMEs in English** throughout; private paths replaced by `$MOVA_DATA`.
- Internal tooling and notes (the site server, the sync and paper tools, the coder, the old story worlds) are no longer in this repository.

- **Measured** (2026-10-16): LAS on the tales test 77.38 → 79.69 and on EWT test 79.86 → 80.68; SVAMP (`math` solver) 64.0% → 64.3%; bAbI 99.8% → 99.4% and its contrast set 99.7% → 99.4% (see Known issues); StepGame and SpartQA unchanged.
- The two unfinished courses (programming, philosophy) are no longer part of the release.

### Known issues

- `en.bin` misreads *do*-support questions with a long subject ("Does the box of chocolates fit in the box?"): bAbI task 18 92.3%, task 16 95.3%. The same code with the 0.3 model gives 99.8% on bAbI.
- The maths reader has no verb class for *give away*; the unit test `question_numbers_must_be_used` is ignored until 0.5.

### Removed

- The 0.3 model `ud-ewt-eslspok.bin` is not carried on.
- The maths student's old window-of-words reader; the tree reader is the only reader.
- The story worlds without a teacher's check moved out of the default build (the schools replace them).

## 0.3 (2026-10-03)

Notes and metrics: [docs/release-0.3.md](docs/release-0.3.md).

### Added

- **Self-retraining with states** ([docs/selfplay.md](docs/selfplay.md)): a tree of delta states with rollback, a brain and decision letters per state, and the family training loop on a Unix socket with live control.
- **Learning on big texts**: samples from the tree store judged by the absurdity matrix, UD grammar constraints and a teacher check; a negative list of rejected rules.
- **The absurdity matrix** ([docs/absurdity.md](docs/absurdity.md)): 405 verbs × 601 nouns scored 0–4, plus a fairy-tale layer.
- **Induced tree-repair rules** ([docs/induce.md](docs/induce.md)) and **trees by logical derivation** ([docs/derive.md](docs/derive.md)).
- **StepGame corrected and contrast sets**: a reader of new phrasings next to the learned perceptron.
- **Expressions with their real meanings** ([docs/pragmatics-expressions.md](docs/pragmatics-expressions.md)) and the **tree store** (`world store`).
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
