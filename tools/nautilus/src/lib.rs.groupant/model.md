# mova-nautilus-lib

## model

# mova-nautilus-lib

## Descriptant

Crate root of mova-nautilus: declares animal, layer, sources, teacher and work. Source: `tools/nautilus/src/lib.rs`.

## Forward approximant

Expose the modules.

## Invariant

Only module declarations.

## Cost

O(1).

## Composition

Parent: `nautilus`. Root of the library used by main, the tests and mova-turtle. State: None.

## behavior_complement

Holds no behaviour of its own. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

The module list is recoverable from the source; nothing else. No unique inverse is claimed.

## tests

### Checks

These are finite fixtures; no teacher or model was called; not a proof.

## History

### 2026-11-04 · document

#### Epoch 2026-11-04

First model, written with the source; no prospective prediction is claimed beyond the checks.

#### Epoch 2026-11-04 (pool)

modules improve (the `layer` source) and pool (a Crab pool member).

#### Epoch 2026-11-04 (pool)

modules improve (the `layer` source) and pool (a Crab pool member).

#### Epoch 2026-11-04 (senses)

Declares `senses` and `replay`.

### 2026-11-04 · tests

Command: cargo test --manifest-path tools/nautilus/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin (15 unit + 2 English tests; two identical runs on 2026-11-04).

Negative controls 2026-11-04 (each red, restored): no operator-word check; no terminal-id check; a limit not stopping the loop; stats drafting instead of reading.

Pool epoch (2026-11-04): cargo test --release with the 0.4 en.bin, 26 unit + 2 English tests, two identical runs; red controls in the pool, improve and sources tests (each red, restored). Finite fixtures, not a proof.

2026-11-04 (senses): cargo test --release (0.4 en.bin), tools/nautilus 37 unit + 2 English tests, two identical runs; tools/turtle 9 + 2 pass.

### 2026-11-05 · document

#### Epoch 2026-11-05 (sampled Wikipedia)

`pub mod sampling` joins the library: the sampled Wikipedia layer's word order and example choice.

### 2026-11-05 · tests

2026-11-05 (sampled Wikipedia): cargo test --release --offline --locked (0.4 en.bin), tools/nautilus 46 unit + 2 English tests, two identical runs.
