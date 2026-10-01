# Tests

## Unit tests

```
tests/run.sh            # all crates
tests/run.sh math world # some crates
```

Each crate in `ai/` is built and tested with `cargo test --release`. All
crates share one build directory (`target/` at the repository root).

## Data folder

Commands that read datasets, write runs or databases look under `$MOVA_DATA` (default: `data` in the current folder): `raw/` for downloaded datasets, `runs/` for outputs, `db/` for databases, `ud/` for Universal Dependencies treebanks (or set `EN_UD_DIR`). Commands that call a large language model read `ANTHROPIC_API_KEY=…` from `local.md` in the current folder (or the file named by `MOVA_LOCAL`); none of them is needed for the tests and benchmarks above.

## Benchmarks

The benchmark data is not in this repository (each set has its own license);
download it and point the command to it. Results are deterministic: the same
command gives the same number on every run.

| Benchmark | What it tests | Command | Mova's result |
|---|---|---|---|
| [bAbI](https://research.facebook.com/downloads/babi/) tasks 1–20, v1.2 (CC BY 3.0) | keeping the world of a story: who is where, who has what, relations | `cd ai/world && cargo run --release -- babi <path>/tasks_1-20_v1-2/en test` | 99.8% on test, 20 of 20 tasks ≥ 95%; train 99.7%; no training on the questions |
| bAbI contrast set | the same stories with new names, places, things and synonym verbs | `cargo run --release -- babi-contrast <bAbI en dir> <out dir>`, then `babi <out dir> test` | 99.7%, 20 of 20 |
| [StepGame](https://github.com/ShiZhengyan/StepGame) clean (MIT) | relative positions on a grid over 1–10 steps | `cd ai/world && cargo run --release -- stepgame <path to the json files>` | 96.3% on the full test (k=1 98.2% … k=10 94.7%); 98.0–99.6% on questions without the two expressions that the dataset labels inconsistently |
| [SpartQA-Human](https://github.com/HLR/SpartQA_generation) (data: non-commercial) | spatial questions about human-written scene descriptions | `cd ai/world && cargo run --release -- spartqa <path>/human_test.json` | YN 60.1%, FB 67.1%, CO 58.2%, FR 50.6% |
| [SVAMP](https://github.com/arkilpatel/SVAMP) (MIT) | arithmetic word problems | `cd ai/math && MATH_NOLEX=1 cargo run --release --bin mathsolve -- steps --set svamp --exact --epochs 20 --beam 32 --rerank 5 --qread` (expects `$MOVA_DATA/raw/mwpdata-svamp/svamp-{train,test}.json`; `MOVA_DATA` defaults to `data` in the current folder) | 59.7% (179 of 300) |

`world babi-probe "sentence" "question?"` shows how Mova reads single
sentences (set `BABI_DEBUG=1` to print the dependency trees).
