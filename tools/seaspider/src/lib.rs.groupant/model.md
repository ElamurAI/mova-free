# mova-seaspider-lib

## model

# mova-seaspider-lib

## Descriptant

Crate root of mova-seaspider: declares the seven modules. Source: `tools/seaspider/src/lib.rs`.

## Forward approximant

Expose config, crawl, fetch, links, polite, robots, store.

## Invariant

Only module declarations; no state or effects.

## Cost

O(1).

## Composition

Root of the library used by main and the tests.

## behavior_complement

Holds no behaviour of its own; everything is delegated to the modules. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

The module list is recoverable from the source; nothing else. No unique inverse is claimed.

## tests

### Checks

Indirect through all tests.

## History

### 2026-11-04 · document

#### Epoch 2026-11-04 (English)

2026-11-04: adds the `animal` and `answer` modules.

#### Epoch 2026-11-04 (Sea Spider live: singleton and callers)

Epoch 2026-11-04: Exports the new `singleton` module.

### 2026-11-04 · tests

Command: cargo test --manifest-path tools/seaspider/Cargo.toml --offline --locked (27 unit + 9 end-to-end tests against a local 127.0.0.1 HTTP server; two identical runs on 2026-11-04).

These are finite fixtures; the only live run was the 2026-11-04 smoke against the project's own site.

2026-11-04 (English): cargo test --manifest-path tools/seaspider/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin (31 unit + 9 end-to-end + 2 English tests on a local 127.0.0.1 server; two identical runs on 2026-11-04). Negative controls 2026-11-04: binding arguments to the first action instead of through the parse, allowing every robots decision, and ordering jobs by arrival only each turn the English tests red; restored afterwards.

2026-11-04: cargo test --manifest-path tools/seaspider/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin (34 unit + 9 end-to-end + 1 singleton CLI + 2 English tests; two identical runs on 2026-11-04)
