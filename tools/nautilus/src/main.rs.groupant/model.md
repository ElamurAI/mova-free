# mova-nautilus-main

## model

# mova-nautilus-main

## Descriptant

The nautilus CLI: make, stats, teacher-task, work, work-summary, work-resume, plan, serve (singleton), ask. Source: `tools/nautilus/src/main.rs`.

## Forward approximant

Parse argv -> the matching library call; serve and work take their singleton locks first (exit 5 when held); work plugs the profile's own mova-terminal (Unplugged when its profile is unreadable); no question without the operator's word.

## Invariant

Exit status 0 only on success; 5 for a held singleton.

## Cost

As the called functions.

## Composition

Parent: `nautilus`. Parent: nautilus. State: None of its own.

## behavior_complement

No command calls a model; unknown argv prints the usage. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

Argv and the printed JSON recover what was asked. No unique inverse is claimed.

## tests

### Checks

These are finite fixtures; no teacher or model was called; not a proof.

## History

### 2026-11-04 · document

#### Epoch 2026-11-04

First model, written with the source; no prospective prediction is claimed beyond the checks.

#### Epoch 2026-11-04 (pool)

`pool-work` (a pool member's loop), `backlog` (open questions per layer, writes backlog.json), `job-task` (print the next question of a job; nothing sent), `pool-seed` (seed a ledger once); `work` drafts only.

#### Epoch 2026-11-04 (pool)

`pool-work` (a pool member's loop), `backlog` (open questions per layer, writes backlog.json), `job-task` (print the next question of a job; nothing sent), `pool-seed` (seed a ledger once); `work` drafts only.

#### Epoch 2026-11-04 (senses)

`replay CONFIG OUT TERMINAL_ROOT...`; the loops' pacing from Config::pacing; the standing loop's DictJob takes the configured batch and store.

### 2026-11-04 · tests

Command: cargo test --manifest-path tools/nautilus/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin (15 unit + 2 English tests; two identical runs on 2026-11-04).

Negative controls 2026-11-04 (each red, restored): no operator-word check; no terminal-id check; a limit not stopping the loop; stats drafting instead of reading.

Pool epoch (2026-11-04): cargo test --release with the 0.4 en.bin, 26 unit + 2 English tests, two identical runs; red controls in the pool, improve and sources tests (each red, restored). Finite fixtures, not a proof.

2026-11-04 (senses): cargo test --release (0.4 en.bin), tools/nautilus 37 unit + 2 English tests, two identical runs; tools/turtle 9 + 2 pass.

### 2026-11-05 · document

#### Epoch 2026-11-05 (sampled Wikipedia)

`nautilus dry CONFIG LAYER [ITEMS]` prints `work::dry` of a layer's job (nothing sent or written); `work` sets the
whole job's `ask_all` from the configuration.

### 2026-11-05 · tests

2026-11-05 (sampled Wikipedia): cargo test --release --offline --locked (0.4 en.bin), tools/nautilus 46 unit + 2 English tests, two identical runs.
