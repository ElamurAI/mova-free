# Tests

## Unit tests

```
tests/run.sh            # all crates
tests/run.sh math world # some crates
```

Each crate is built and tested with `cargo test --release`. All
crates share one build directory (`target/` at the repository root, or `CARGO_TARGET_DIR`).

## Data folder

Commands that read datasets, write runs or databases look under `$MOVA_DATA` (default: `~/mova-data`, the same as `install.sh`): `raw/` for downloaded datasets, `runs/` for outputs, `db/` for databases, `ud/` for Universal Dependencies treebanks (or set `EN_UD_DIR`). Commands that call a large language model read `ANTHROPIC_API_KEY=…` from `local.md` in the current folder (or the file named by `MOVA_LOCAL`), a local file that `.gitignore` keeps out of git; none of them is needed for the tests and benchmarks above.

## Benchmarks

The numbers below were measured for release 0.4 (2026-10-16), with the model `en.bin` from `install.sh`.
Build once at the root (`cargo build --release`); `W=target/release/world`.

The benchmark data is not in this repository (each set has its own license);
download it and point the command to it. Results are deterministic: the same
command gives the same number on every run.

| Benchmark | What it tests | Command | Mova's result |
|---|---|---|---|
| [bAbI](https://research.facebook.com/downloads/babi/) tasks 1–20, v1.2 (CC BY 3.0) | keeping the world of a story: who is where, who has what, relations | `$W babi <path>/tasks_1-20_v1-2/en test` | 99.4% on test, 19 of 20 tasks ≥ 95% (task 18 92.3%; with the 0.3 model 99.8%, 20 of 20); no training on the questions |
| bAbI contrast set | the same stories with new names, places, things and synonym verbs | `$W babi-contrast <bAbI en dir> <out dir>`, then `$W babi <out dir> test` | 99.4%, 19 of 20 |
| [StepGame](https://github.com/ShiZhengyan/StepGame) clean (MIT) | relative positions on a grid over 1–10 steps | `$W stepgame <path to the json files>` | 96.3% on the full test (k=1 98.2% … k=10 94.7%); 98.0–99.6% on questions without the two expressions that the dataset labels inconsistently |
| [StepGame corrected](https://doi.org/10.5518/1468) (Li, Hogg & Cohn 2024, MIT) | the same task with the wrong templates removed; clean and noise variants | `$W stepgame <dir with qaK_{train,test}.txt>` (unpack `data.zip`; one dir per variant) | 100.0% at every k=1..10, clean and noise; negative control: test labels shifted by one class → 0.0% |
| StepGame contrast set | phrasings the StepGame generator never produced (incl. the reverse view "B has A on its left"), zero overlap with training | `$W stepgame-contrast <StepGame json dir> 500 --test` | 100.0% at every k; without the lexical reader (`STEPGAME_NO_LEXICAL=1`) 17.1% |
| [SpartQA-Human](https://github.com/HLR/SpartQA_generation) (data: non-commercial) | spatial questions about human-written scene descriptions | `$W spartqa <path>/human_test.json` | YN 75.5%, FB 79.7%, CO 75.9%, FR 63.6%; total 74.1% |
| [SVAMP](https://github.com/arkilpatel/SVAMP) (MIT) | arithmetic word problems | `cd math && MATH_NOLEX=1 ../target/release/mathsolve steps --set svamp --exact --epochs 20 --beam 32 --rerank 5 --qread` (expects `$MOVA_DATA/raw/mwpdata-svamp/svamp-{train,test}.json`) | 64.3% (193 of 300) |
| SVAMP, the school's maths student | the same test read into a world of actors (once per release) | `$W school actors held <path>/svamp-test.json 0`; decisions are made on `$W school actors cv` (train only) | 160 of 300; `cv`: world reader 513, book's way 1116 |
| Parsing, tales test and UD English EWT test | LAS of `en.bin` with the shipped repair rules | `$W state las origin <tales-odd.conllu> tale`, `$W state las origin <en_ewt-ud-test.conllu> real` | tales 79.69, EWT 80.68 |
| Primary-school courses | 25 knowledge courses, 26 992 test questions | `$W knowledge course <course>` for each of the 25 courses listed in `world/src/knowledge.rs` (`world/data/school/<course>/`) | 11 730 right (43.5%); with the GPU parser (`MOVA_EXEC=gpu`) 11 817 |

`world babi-probe "sentence" "question?"` shows how Mova reads single
sentences (set `BABI_DEBUG=1` to print the dependency trees).
