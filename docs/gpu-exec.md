# Execution modes: CPU only and GPU enhanced

The rule since 2026-10-15: Mova always has two modes.

- **CPU only** is the default. It builds without CUDA, runs anywhere, and is exact. This is the reference.
- **GPU enhanced** needs a build with `--features gpu` and `MOVA_EXEC=gpu` at run time. Batch stages run on the GPU (RTX 4000 SFF Ada, 20 GB). This mode is for executing pipelines, not for training.

Each GPU stage is one of three kinds:

| kind | what it means | gate |
|---|---|---|
| **exact** | the same result as on the CPU | byte-identical output on a full run |
| **approximant** | a GPU model stands in for a CPU one (e.g. the GPU parser for the perceptron) | agreement and task score measured against the CPU mode, and reported |
| **hybrid** | the GPU does the bulk and the CPU does the rest (tokens, lemmas, decoding), in threads | as for its GPU part |

Approximants fit tasks that tolerate small differences: bulk runs, training pipelines (drafts, silver data), and debugging. Releases and measured claims are made in CPU mode, or in GPU mode with its delta stated.

If you ask for `MOVA_EXEC=gpu` from a CPU-only build, the program stops with an error (fail-fast). It does not quietly fall back to the CPU.

## Build and run

```
# CPU only (default), at the repo root
cargo build --release
# GPU enhanced: its own target directory, so the CPU build is not rebuilt each time
cargo build --release -p world --features gpu --target-dir target/gpu
MOVA_EXEC=gpu MOVA_GPARSE_MODEL=$MOVA_DATA/en/models/gparse target/gpu/release/world knowledge course biology
```

`MOVA_TIMING=1` shows where a course's time goes. `GP_TIMING=1` splits the GPU parser's time into forward pass, Eisner decoding and relations.

## Stages (2026-10-15)

### world: parse ahead (approximant + hybrid)

`exec::parse_many` serves the student's parse-ahead step (`PragStudent::preparse`).

- CPU mode runs the `en` perceptron on 12 threads, exactly as before.
- GPU mode is a hybrid. `gparse` (now also a library, `gparse::load` / `parse_forms`) predicts XPOS and the tree. `en` supplies tokens, UPOS, FEATS and lemmas (`Annotator::complete`), with tokenizing, completion and Eisner decoding spread over threads.

| | CPU mode | GPU mode |
|---|---|---|
| 25 primary courses, test questions right | 11 730 / 26 992 (43.46%) | **11 817** (43.78%) |
| biology, parse-ahead time (68 512 units) | 2.8 s (12 threads) | 6.4 s (forward + copy 3.6 s) |
| deterministic | yes | yes: two runs are identical, and batch size does not change the output |

The GPU parser is the more accurate one (school LAS 88 against 84), and the student gains +87 questions from it. 13 courses gain and 7 lose (everyday −35, books −12). For short units the 20-core CPU parses faster. The GPU starts to pay off on bulk input with long sentences: 266 k words in 1.5 s of prediction (`gparse annotate`, 10 k school sentences), about 180 k words/s.

### world: the student's frame matching (CPU, exact; applies in both modes)

This was measured first, because a GPU only helps where the time actually goes. On biology, 34 s per course broke down as: align 13 s (31 M calls), re-reading the same questions and options 11 s, parsing 3 s. Two exact CPU fixes, now in both modes:

- **frames as numbers** (`MCode` for remembered frames, `QCode` for question frames, `align_mask` returning bits): 13 → 3.8 s;
- **readings remembered** by (what was read before, text, question?): 12 → 7 s.

All 25 courses give byte-identical results to the string path (`MOVA_ALIGN=str`, `MOVA_NOREADMEMO=1`, which also serve as the negative control).

On numbers, align is a set of integer comparisons. An exact GPU kernel is possible (question frames × the whole memory, bit masks), but at 31 M pairs per course the CPU does it in about 3 s. A kernel pays only on bulk sets, such as many students or a large memory.

### prag: the pragmatic witnesses (approximant)

`prag/witness/witness.py` runs the cycle's open witnesses: two zero-shot NLI DeBERTas, a certainty model, politeness and irony. `WITNESS_FAST=1` or `MOVA_EXEC=gpu` turns on three changes: inputs sorted by length (less padding), fp16 weights, and batches of 256 instead of 64. The slow path stays byte-identical to the earlier rounds.

| cycle round 27, 2 200 sentences | fp32 (slow) | fast |
|---|---|---|
| time | 3 min 16 s | **59 s** |
| label agreement with fp32 | — | act 99.91 / 99.55, polarity 99.95 / 99.82, uncertainty 99.95 / 100, polite 99.95, irony 100%; certainty within 0.01 |
| agreed pragmatics (`prag agree`) differing | — | 3 of 2 200 (P2 fired 619 times instead of 621) |

The witnesses need a GPU in either mode. The slow path is the reference.

### world induce / selfplay: the SLM finding its own rules (exact)

`world induce` is transformation-based learning (Brill 1995). Every 1–3 conjunction of a mislabelled word's feature atoms becomes a candidate label rule. Each candidate's fixes and breaks are counted over all words, the best one is applied, and the loop repeats. `world selfplay` tests deltas over a rule set: remove, relax or add a rule.

- **CPU (both modes):** atoms as 21-bit codes and conjunctions as one u64 key, with a fast deterministic hasher. Atoms are cached across iterations and recomputed only for sentences a rule changed. The fix table is updated incrementally: only the changed sentences are counted again. Parsing, atoms and counting run on threads.
- **GPU (`MOVA_EXEC=gpu`):** the break counting runs as one CUDA kernel (`world/src/induce_gpu.rs`, nvrtc through candle's cudarc). Each thread takes one correctly labelled word and probes every 1–3 subset of its atoms in an open-addressing table of candidate groups, using `atomicAdd` of ones. These are integer counts, so the result matches the CPU number for number.

| run (train EWT + ESLSpok + school-agreed train; parser en-school.bin) | original | CPU now | GPU |
|---|---|---|---|
| 54 k sentences, 3 iterations | ~20 min per iteration (one thread) | 34 s (break counting 19.1 s) | **18 s** (break counting 1.5 s) |
| 54 k sentences, full run (stops after 3 rules) | — | 46 s (break counting 28 s) | **19 s** (1.7 s) |
| + parsedb history, 152 k sentences, 10 iterations | — | 5 min 39 s (break counting 287 s; fix counting 15 → 3.3 s incrementally) | **33 s** (break counting 7.6 s) |
| selfplay (EWT dev / school test, 14 rules, 61 deltas) | 16.7 s | 6.9 s | 6.9 s |

`rules.tsv`, `report.txt` and `rules.machine.tsv` are identical between CPU and GPU, as are `selfplay.jsonl` and the log against the original binary. In GPU mode, break counting runs 34× faster at scale. At 152 k sentences, the remaining time in GPU mode goes mostly to parsing (13 s on the CPU threads).

## Next candidates

Ordered by how much time each saves:

1. **Cycle drafts**: `gparse` already drafts on the GPU. Stanza and Trankit are Python on the GPU.
2. **`prag world` (61 s / 48 k sentences)** and **`prag label` (27 s / 2.2 k)**: profile first. They are CPU code, so threads and numbers come before any GPU work.
3. **Exact integer kernels on the GPU** for bulk scoring: align over a whole memory, absurdity-matrix lookups over many pairs, index scoring. Integers keep the GPU result exact, so these can be gated byte-for-byte.
