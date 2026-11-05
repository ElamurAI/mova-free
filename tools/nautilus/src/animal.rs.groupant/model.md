# mova-nautilus-animal

## model

# mova-nautilus-animal

## Descriptant

Nautilus as one of Mova's animals: layer configuration, make/stats, English answers realized from results, deputy capability check, the nautilus handler (draft, stats), answer through Deputy::run_with, and a host-wide singleton lock. Source: `tools/nautilus/src/animal.rs`.

## Forward approximant

Config::load validates sources, paths and top; make: read base, read and annotate the source, layer::build, render, write out, stats; stats: read out or not made; Hands: operation from trusted parameters -> make/stats + English sentence; answer: Deputy::run_with with Hands, report(); singleton: try_lock on LOCK, pid written, else 'already running as pid N'.

## Invariant

One handler per configured operation and layer; a second serve on the same lock never starts.

## Cost

make: as layer and sources; stats: O(layer bytes).

## Composition

Parent: `nautilus`. Parent: nautilus. State: Layer files, deputy ledger, lock file.

## behavior_complement

No slots, no teacher calls, no file other than the configured layer is written; a refused prompt does nothing. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

From the deputy ledger: prompt, plan and outputs per request id; from the layer file its counts. No unique inverse is claimed.

## tests

### Checks

These are finite fixtures; no teacher or model was called; not a proof.

## History

### 2026-11-04 · document

#### Epoch 2026-11-04

First model, written with the source; no prospective prediction is claimed beyond the checks.

#### Epoch 2026-11-04 (pool)

sources `course` (course folders) and `layer` (an existing layer improved through proposals; no `out`); `licence_class` and `shipped` enforced by LayerConfig::check (CC BY-SA, NC, research-only never in a shipped layer; Wikipedia is always cc-by-sa) and check_inputs (a CC BY-SA header in a lesson or the layer file refuses a shipped layer); Config::job builds a LayerJob or a DictJob for part of parts; stats of an improved layer report weak, askable, thin rows and teacher questions. Red controls: licence check removed; header check removed; layer folder not frozen.

#### Epoch 2026-11-04 (pool)

sources `course` (course folders) and `layer` (an existing layer improved through proposals; no `out`); `licence_class` and `shipped` enforced by LayerConfig::check (CC BY-SA, NC, research-only never in a shipped layer; Wikipedia is always cc-by-sa) and check_inputs (a CC BY-SA header in a lesson or the layer file refuses a shipped layer); Config::job builds a LayerJob or a DictJob for part of parts; stats of an improved layer report weak, askable, thin rows and teacher questions. Red controls: licence check removed; header check removed; layer folder not frozen.

#### Epoch 2026-11-04 (senses)

Config gains `senses` (the shared sense store path), `batch` (1..40 words per question, default work::BATCH 25) and `pacing_secs` (>= 30, default 600); Config::job passes batch and the store to LayerJob and DictJob. Unknown fields are still refused (an old binary refuses a configuration with the new fields).

### 2026-11-04 · tests

Command: cargo test --manifest-path tools/nautilus/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin (15 unit + 2 English tests; two identical runs on 2026-11-04).

Negative controls 2026-11-04 (each red, restored): no operator-word check; no terminal-id check; a limit not stopping the loop; stats drafting instead of reading.

Pool epoch (2026-11-04): cargo test --release with the 0.4 en.bin, 26 unit + 2 English tests, two identical runs; red controls in the pool, improve and sources tests (each red, restored). Finite fixtures, not a proof.

2026-11-04 (senses): cargo test --release (0.4 en.bin), tools/nautilus 37 unit + 2 English tests, two identical runs; tools/turtle 9 + 2 pass.

### 2026-11-05 · document

#### Epoch 2026-11-05 (sampled Wikipedia)

A fifth source, `wikipedia-examples` (`sampling`): `make` orders tier 1's words, fetches the missing words' examples
through the configured `wikicorpus` binary, streams the examples file and writes the rows of the first `top` (up to
50 000) words with two chosen examples, growing the window past thin words. `WIKIPEDIA` sources are always
`cc-by-sa`; any `cc-by-sa` layer's `out` and examples file are refused inside a Git work tree (`git_tree`: an
ancestor holding `.git`). `native_picks: false` puts the layer in `Config::ask_all`, so every word with examples is
asked. Red controls (each red, restored): the Git-tree gate off; the sampled source not forced to CC BY-SA.

#### Epoch 2026-11-05 (questions only observe)

`check_capabilities` refuses a deputy configuration that declares a `draft` capability `observes`; only `stats` answers questions ("Is the Wikipedia dictionary drafted?" no longer builds it).

### 2026-11-05 · tests

2026-11-05 (sampled Wikipedia): cargo test --release --offline --locked (0.4 en.bin), tools/nautilus 46 unit + 2 English tests, two identical runs. licences_are_enforced_by_configuration covers the sampled source, the Git-tree gate and native_picks defaults.

### 2026-11-05 · school data outside the repository

Doc comment: course folders are courses/<course> of the school data.
