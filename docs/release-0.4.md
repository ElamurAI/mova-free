# Release 0.4 — notes (published 2026-10-16)

0.4 is the release of the **school**: one parser model trained with the school's own agreed trees, a maths
student that reads word problems through the dependency tree, primary-school courses, and a GPU mode for the
heavy batch stages. The short list of changes is in [CHANGELOG.md](../CHANGELOG.md).

## What is new

- **One model, `en.bin`.** The `en` tagger and parser are trained on UD English EWT, ESL Spoken and the
  agreed school treebank (42 759 sentences that several parsers and a teacher check agreed on). The model is not
  in git: `install.sh` downloads it from <https://research.elamur.ai/download/0/0.4/> and checks its SHA-256.
  It is under CC BY-SA 4.0, as the treebanks it learns from.
- **The tree reader for word problems** (`world/src/school_tree.rs`, `world treeread`). Told numbers, actors,
  verbs, receivers and comparisons are read off the UD tree into the maths student's world. It replaced the old
  reader of word windows. On the course lessons (2026-10-15): lessons 1–38 and 49–64 2367 → 2380, the gate
  2443 → 2733, lessons read as data 1123 → 1338, the exam 627 → 635; the two-fold check `cv` on SVAMP train,
  world reader 297 → 513.
- **Primary-school courses** ([school-making.md](school-making.md), [school.md](school.md)): 25 knowledge
  courses with lessons, exercises, tests and exams, 26 992 test questions, and the pupils' dictionary. The level
  is a clever child of 9–10; half of every conversation is plain English and half is slang and regional speech.
- **Two execution modes** ([gpu-exec.md](gpu-exec.md)). *CPU only* is the default and is exact. *GPU
  enhanced* (`--features gpu`, `MOVA_EXEC=gpu`) runs the batch stages on the GPU; each stage is either exact
  (gated byte for byte against the CPU) or an approximant whose agreement is measured.
- **`induce` and `selfplay` on the GPU** ([induce.md](induce.md), [selfplay.md](selfplay.md)): break
  counting is an exact integer kernel, so the rules and reports are the same as on the CPU.
- **One Cargo workspace.** `cargo build --release` at the root builds everything into `target/release/`.
- **Corpus 0.4** (<https://research.elamur.ai/corpus/0.4/>): the school's books, the agreed school treebank,
  silver and bronze trees, the pupils' dictionary, maths scripts and the contrast sets, all under CC BY-SA 4.0.
- **Licences.** The code is under the Elamur Cosmic License ([LICENSE.md](../LICENSE.md),
  [licence/](../licence)); the model and the corpus are under CC BY-SA 4.0. To be reviewed in 0.5.

## Metrics 0.3 → 0.4

The same commands as in [tests/README.md](../tests/README.md), run on a CPU with the workspace build and
`en.bin` (sha256 `31a93401fe7135b17e1eacccf608d1a42e94ebc8951f011d7f80ff9db144eb86`). Held-out tests were run once.

| measure | 0.3 (2026-10-03) | 0.4 (2026-10-16) |
|---|---|---|
| bAbI test, 20 tasks | 99.8%, 20 of 20 ≥ 95% | 99.4%, 19 of 20 (task 18 92.3%, task 16 95.3%) |
| bAbI contrast set | 99.7%, 20 of 20 | 99.4%, 19 of 20 |
| StepGame, original labels | 96.3% | 96.3% |
| StepGame corrected, clean and noise | 100% | 100% |
| StepGame contrast set (no lexical reader) | 100% (17.1%) | 100% (17.1%) |
| SpartQA-Human YN / FR / total | 75.5 / 63.6 / 74.1 | 75.5 / 63.6 / 74.1 |
| SVAMP test, `math` solver | 64.0% (192) | 64.3% (193) |
| SVAMP test, the school's maths student | 169 (2026-10-08, between releases) | 160 |
| maths student `cv` on SVAMP train: world reader / book's way | 297 / 1116 (before the tree reader) | 513 / 1116 |
| LAS, tales test (with the shipped repair rules) | 77.38 | 79.69 |
| LAS, UD English EWT test | 79.86 | 80.68 |
| primary-school courses, 26 992 test questions | — | 11 730 right (43.5%) |

Notes:

- **bAbI fell because of the parser model, not the code.** With the 0.3 model `ud-ewt-eslspok.bin` the same
  0.4 code gives 99.8% and 20 of 20 (task 18 96.4%, task 16 99.5%). `en.bin` takes *Does* for the main verb in a
  question with a long subject ("Does the box of chocolates fit in the box?"). This is the main known issue of 0.4.
- **The school's SVAMP test fell by 9** (Subtraction 95 → 87, Common-Division 39 → 36, Addition 24 → 26,
  Multiplication 11 → 11), while `cv` on train did not fall. The probable cause is the parser change; not checked.
- **LAS rose on both tests.** No sentence of the tales test or of the EWT test is in the model's training data
  (checked by text against the agreed school treebank); the school text is close in genre to the tales.
- **Determinism.** The bAbI test was run twice; the outputs are identical.

## GPU mode (2026-10-15)

| stage | CPU | GPU | agreement |
|---|---|---|---|
| parsing the 25 courses (`gparse` instead of `en`) | 11 730 right | 11 817 right | an approximant: other trees, deterministic run to run |
| pragmatic witnesses (`WITNESS_FAST=1`) | 3 min 16 s | 59 s | labels agree on 99.5–100%; 3 of 2 200 differ after agreement |
| `induce`: counting broken sentences, 152 000 sentences | 1× | 34× faster | exact: the same rules byte for byte |
| `selfplay` round | 16.7 s | 6.9 s | exact |

## Known issues

- *Do*-support questions with a long subject (above).
- The maths reader has no verb class for *give away*: `en.bin` now reads it as a phrasal verb (`compound:prt`,
  which is right), and "she gives away 64 games" is not read. The unit test
  `question_numbers_must_be_used` is marked ignored until 0.5.
- The school's maths student is far below the `math` solver on the SVAMP test (160 against 193): it reads
  through a world of actors, which is slower to learn but explains every step.

## Not carried on

- The 0.3 model `ud-ewt-eslspok.bin` (it stays in the 0.3 download).
- The old maths reader of word windows.
- The story worlds without a teacher's check, and the unfinished courses (programming, philosophy).
