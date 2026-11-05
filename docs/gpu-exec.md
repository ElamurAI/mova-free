# Execution modes: CPU only and GPU enhanced

This page describes Mova's two execution modes, how to build and run each, what every GPU stage does and how it is checked against the CPU, and the reference switches that select the exact path.

## The two modes

- **CPU only** is the default. It builds without CUDA, runs anywhere, and is exact. It is the reference.
- **GPU enhanced** needs a build with `--features gpu` and `MOVA_EXEC=gpu` at run time. Batch stages run on a CUDA GPU. The mode is for executing pipelines; GPU training is configured separately (see [training.md](training.md)).

Each GPU stage is one of three kinds:

| kind | what it means | gate |
|---|---|---|
| **exact** | the same result as on the CPU | byte-identical output on a full run |
| **approximant** | a GPU model or kernel stands in for a CPU one | agreement and task score measured against the CPU mode, and reported |
| **hybrid** | the GPU does the bulk and the CPU threads do the rest (tokens, lemmas, decoding) | as for its GPU part |

Approximants fit tasks that tolerate small differences: bulk runs, training pipelines (drafts, silver data) and debugging. Training speed and test-pass speed take priority over byte-identical GPU results; small numerical or task-score deltas are acceptable when they are reported. Releases and measured claims are made in CPU mode, or in GPU mode with its delta stated.

Asking for `MOVA_EXEC=gpu` from a CPU-only build stops the program with an error (fail-fast). It never falls back to the CPU quietly.

## Build and run

```
# CPU only (default), at the repo root
cargo build --release
# GPU enhanced: its own target directory, so the CPU build is not rebuilt each time
cargo build --release -p world --features gpu --target-dir target/gpu
MOVA_EXEC=gpu MOVA_GPARSE_MODEL=$MOVA_DATA/en/models/gparse target/gpu/release/world knowledge course biology
```

`MOVA_TIMING=1` shows where a course's (or `world induce`'s) time goes. `GP_TIMING=1` splits the GPU parser's time into forward pass, Eisner decoding and relations.

## Stages

### world: parse ahead (hybrid approximant)

`exec::parse_many` serves the student's parse-ahead step (`PragStudent::preparse`).

- CPU mode runs the `en` perceptron on threads, exactly.
- GPU mode is a hybrid. `gparse` (also a library: `gparse::load`, `parse_forms`) predicts XPOS and the tree. `en` supplies tokens, UPOS, FEATS and lemmas (`Annotator::complete`); tokenizing, completion and Eisner decoding are spread over threads.

Both paths preserve input order and return `Result`: a missing or corrupt model, a failed worker or a malformed GPU batch stops the course, trace or diagnosis before it writes a score or report. Workers preserve input order and return the whole batch, so a worker failure cannot silently drop sentences. Regression tests are in `world/tests/parser_failures.rs`; model-dependent agreement and repeatability checks run explicitly with `--ignored`. These finite checks do not establish batch-size or cross-device identity in general.

CPU and GPU mode use different parsing models, so they can give different school answers. School result, 2026-10-15, the 25 primary courses' test questions: CPU mode 43.46%, GPU mode 43.78%.

### gparse: prediction without unused scores (exact)

Ordinary prediction (`gparse::predict`, `parse_forms`) does not compute the tag and relation log-probability reductions or copy those unused scores to the host. `predict_scored` still computes them. Both paths share batching and decoding, and give the same trees and tags. The check is `gparse/tests/predict_paths.rs`: set `GP_TEST_MODEL` to the model directory and `GP_TEST_CORPUS` to a training CoNLL-U file (optionally `GP_TEST_SENTENCES`, default 1024), then run `cargo test --release -p gparse --test predict_paths -- --ignored --nocapture`. Empty batches and the overlength fallback are covered too.

### gparse: fused attention softmax (approximant)

On CUDA, `gparse` uses a fused row softmax by default, for all callers including `world`. CUDA training uses a separate wrapper with an analytical backward pass. It is an approximant because floating-point reductions differ, so output agreement is checked on the intended model and inputs. Implementation, citations and the reproducible probe: [gparse README](../gparse/README.md#fused-inference-softmax).

Empty tokenized sentences are left out of GPU work; their empty results and zero scores keep their original positions. CUDA regression tests cover entirely empty and mixed batches.

### world: the student's frame matching (CPU, exact, both modes)

Frame matching runs on the CPU in both modes, with two exact optimisations:

- **frames as numbers**: `MCode` for remembered frames, `QCode` for question frames, `align_mask` returning bits, so align is a set of integer comparisons;
- **readings remembered** by (what was read before, text, question?).

All courses give byte-identical results to the string path, which is the reference and the negative control. An exact GPU kernel for align (question frames × the whole memory, bit masks) is possible, but pays only on bulk sets, such as many students or a large memory.

The exact bit-mask strength path (`MOVA_STRENGTH=mask`) avoids temporary feature vectors for strength-only queries. An exhaustive mask test and matched course traces establish equivalence. It is opt-in; `vec` is the default.

### world: lesson training reuse (exact, both modes)

Lesson training reuses evaluations under exact dependency checks. Reuse is on by default; traces, reports and post-lesson weights are identical to the reference.

### prag: the pragmatic witnesses (approximant)

`prag/witness/witness.py` runs the open witnesses: two zero-shot NLI DeBERTa models, a certainty model, politeness and irony. `WITNESS_FAST=1` or `MOVA_EXEC=gpu` turns on three changes: inputs sorted by length (less padding), fp16 weights, and batches of 256 instead of 64. The slow path (fp32, batches of 64, unsorted) is the reference. Compare labels, scores and the agreed pragmatics (`prag agree`) before using the fast path; no general agreement or score-error bound is established. The witnesses need a GPU in either mode.

### world induce and selfplay (exact)

`world induce` is transformation-based learning (Brill 1995). Every conjunction of one to three feature atoms of a mislabelled word becomes a candidate label rule. Each candidate's fixes and breaks are counted over all words, the best one is applied, and the loop repeats. `world selfplay` tests deltas over a rule set: remove, relax or add a rule.

- **CPU (both modes):** atoms as 21-bit codes and conjunctions as one u64 key, with a fast deterministic hasher. Atoms are cached across iterations and recomputed only for sentences a rule changed; the fix table is updated incrementally. Parsing, atoms and counting run on threads.
- **GPU (`MOVA_EXEC=gpu`):** break counting runs as one CUDA kernel (`world/src/induce_gpu.rs`, nvrtc through candle's cudarc). Each thread takes one correctly labelled word and probes every one-to-three subset of its atoms in an open-addressing table of candidate groups with `atomicAdd`. These are integer counts, so the result matches the CPU number for number.

`rules.tsv`, `report.txt`, `rules.machine.tsv` and `selfplay.jsonl` are identical between CPU and GPU.

### Not on the GPU

- Cycle drafts: `gparse` drafts on the GPU; Stanza and Trankit run as Python on the GPU, outside these modes.
- `prag world` and `prag label` are CPU code.
- Align over a whole memory, absurdity-matrix lookups over many pairs and index scoring have no GPU kernels. They are integer work, so a kernel could be gated byte for byte.

## Reference switches

Every faster path keeps a switch to its reference, for comparisons and as a negative control.

| switch | reference it selects |
|---|---|
| (no `MOVA_EXEC`) | CPU-only execution |
| `GP_FUSED_SOFTMAX=0` | unfused inference softmax (CPU always uses it) |
| `GP_TRAIN_FUSED_SOFTMAX=0` | unfused training softmax |
| `MOVA_ALIGN=str` | string frame matching |
| `MOVA_NOREADMEMO=1` | no remembered readings |
| `MOVA_STRENGTH=vec` (default) | vector strength path; `mask` is the opt-in exact alternative |
| `MOVA_LEARN_REUSE=0` | no evaluation reuse in lesson training |
| `WITNESS_FAST` unset, CPU mode | fp32 witnesses, batches of 64, unsorted |
| `GP_DEV_EVERY=1` (default) | full development check every epoch |
| `GP_SUBWORD` unset or `0` | word, prefix-3, suffix-3 and shape input without character n-grams |
| `GP_PA` unset or `0` | every word in the arc and relation losses |
| `GP_GELU` unset (erf) | exact GELU; `GP_GELU=tanh` is an experimental approximation, persisted in newly saved models |

The [Lean neural specifications](../formal/README.md#neural-mathematics) derive softmax's derivative and factor its reverse-mode Jacobian product. CUDA gradient tests remain the check of the implementation and its floating-point behaviour.

## Fast iterative measurements

Quick probes serve iterative work; larger and full checks run periodically and before any claim. School sampling
(`world knowledge bench`, its tiers and parse cache) is in [training probes](training.md#fast-probes-and-full-runs);
parser checkpoint selection with `GP_DEV_EVERY` and `GP_DEV_QUICK` is in
[gparse development evaluation](../gparse/README.md#periodic-development-evaluation). Both are separate from the
execution mode.
