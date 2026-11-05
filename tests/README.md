# Tests

Mova's animals and the Octopus, the large language model that develops Mova, are described in [the sea animals of Mova](../docs/animals.md).

Run commands from the repository root. The scores these commands give for each release are in the release notes ([0.3](../releases/0.3.md), [0.4](../releases/0.4.md), [0.5](../releases/0.5.md)); school results are in [docs/school.md](../docs/school.md#results).

## Unit tests

```
CARGO_BUILD_JOBS=2 CARGO_NET_OFFLINE=true tests/run.sh            # default eleven-crate suite
CARGO_BUILD_JOBS=2 CARGO_NET_OFFLINE=true tests/run.sh math world # selected suites
```

The script builds and tests each selected crate with `cargo test --release --locked`. Its default list is en, global, layers, prag, coref, qagen, math, latex, world, mlab and vfs. Other workspace suites, including algorithms and mathematical tools, require explicit candidate checks. Crab and both Fish workers are separate public workspaces at tools/crab, tools/fish and tools/fish-terminal. A root build does not establish that those tests ran.

Selected root crates share `target/` or `CARGO_TARGET_DIR`. Offline resolution requires dependencies already present locally; report unavailable dependencies rather than silently changing the check. The final release matrix must record ignored tests and model/data requirements.

The additional CI lanes are:

```sh
CARGO_BUILD_JOBS=2 CARGO_NET_OFFLINE=true tests/run.sh algorithms tools/modelcheck tools/correspondence teacher
CARGO_BUILD_JOBS=2 cargo test --manifest-path tools/crab/Cargo.toml --release --offline --locked -j2 -- --test-threads=2
CARGO_BUILD_JOBS=2 cargo test --manifest-path tools/fish/Cargo.toml --release --offline --locked -j2 -- --test-threads=2
CARGO_BUILD_JOBS=2 cargo test --manifest-path tools/fish-terminal/Cargo.toml --release --offline --locked -j2 -- --test-threads=2
```

The authenticated smoke test in teacher/tests/live.rs is ignored by default. Explicitly running ignored tests makes one Codex call; ordinary suites above do not. Sidecar harnesses, other root suites and CUDA still need their own declared checks; this matrix is not exhaustive.

`tests/secret-guard-control.sh` is the negative control of `secret-guard.sh`: in a throwaway repo with a fake
`local.md` it checks that a staged machine path or secret is refused and that a variable, or a path inside `peta/`, passes.

## Data folder

Commands that read datasets, write runs or databases look under `$MOVA_DATA` (default: `~/mova-data`, the same as `install.sh`): `raw/` for downloaded datasets, `runs/` for outputs, `db/` for databases, `ud/` for Universal Dependencies treebanks (or set `EN_UD_DIR`). Current training-teacher calls use subscription-authenticated Codex CLI through the shared mova-teacher crate, with no Claude API fallback. Install/authenticate Codex only for an explicitly requested teacher run. The shared default model identifier is gpt-6.1-terra; component-specific configuration and existing receipts retain their exact identifiers. Ordinary local unit/benchmark checks must not implicitly launch a teacher. Keep credentials outside tracked files.

## Benchmarks

Use the model `en.bin` from `install.sh`. Build once at the root (`cargo build --release`); `W=target/release/world`.

The benchmark data is not in this repository (each set has its own license);
download it only under its applicable terms and point the command to it. A reproduced score needs the same
source, model, data and configuration as the release it is compared with; GPU numerical or task differences are
reported separately. Keep held-out sets held out: make decisions on development or cross-validation runs, and run
the final release assessment once, under its frozen protocol.

| Benchmark | What it tests | Command |
|---|---|---|
| [bAbI](https://research.facebook.com/downloads/babi/) tasks 1–20, v1.2 (CC BY 3.0) | keeping the world of a story: who is where, who has what, relations; no training on the questions | `$W babi <path>/tasks_1-20_v1-2/en test` |
| bAbI contrast set | the same stories with new names, places, things and synonym verbs | `$W babi-contrast <bAbI en dir> <out dir>`, then `$W babi <out dir> test` |
| [StepGame](https://github.com/ShiZhengyan/StepGame) clean (MIT) | relative positions on a grid over 1–10 steps | `$W stepgame <path to the json files>` |
| [StepGame corrected](https://doi.org/10.5518/1468) (Li, Hogg & Cohn 2024, MIT) | the same task with the wrong templates removed; clean and noise variants | `$W stepgame <dir with qaK_{train,test}.txt>` (unpack `data.zip`; one dir per variant) |
| StepGame contrast set | phrasings the StepGame generator never produced (incl. the reverse view "B has A on its left"), zero overlap with training | `$W stepgame-contrast <StepGame json dir> 500 --test`; `STEPGAME_NO_LEXICAL=1` turns the lexical reader off |
| [SpartQA-Human](https://github.com/HLR/SpartQA_generation) (data: non-commercial) | spatial questions about human-written scene descriptions | `$W spartqa <path>/human_test.json` |
| [SVAMP](https://github.com/arkilpatel/SVAMP) (MIT) | arithmetic word problems | `cd math && MATH_NOLEX=1 ../target/release/mathsolve steps --set svamp --exact --epochs 20 --beam 32 --rerank 5 --qread` (expects `$MOVA_DATA/raw/mwpdata-svamp/svamp-{train,test}.json`) |
| SVAMP, the school's maths student | the same test read into a world of actors (once per release) | `$W school actors held <path>/svamp-test.json 0`; decisions are made on `$W school actors cv` (train only) |
| Parsing, tales test and UD English EWT test | LAS of `en.bin` with the shipped repair rules | `$W state las origin <tales-odd.conllu> tale`, `$W state las origin <en_ewt-ud-test.conllu> real` |
| Primary-school courses | the knowledge courses listed in `world/src/knowledge.rs` (`$MOVA_DATA/school/courses/<course>/`) | `$W knowledge course <course>` for each course; `MOVA_EXEC=gpu` uses the GPU parser |

Negative control for the corrected StepGame set: run it with the test labels shifted by one class.

`world babi-probe "sentence" "question?"` shows how Mova reads single
sentences (set `BABI_DEBUG=1` to print the dependency trees).
