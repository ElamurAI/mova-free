# mova-hard-cases-config

## Descriptant

The mova-hard-cases-v1 configuration: list, lane root, source bundle commands, policy, held-out pins, en/prag models, gparse, Fish lane, witnesses, measurement, Eel track configurations, control; unknown fields refused; pinned files checked by SHA-256. Source: `tools/hard-cases/src/config.rs`.

## Forward approximant

load(path) -> Config after schema, absolute-path, bound (pagers 1..8, page 1..256, targets 1..8, context 0..8, timeout 1..900, poll 1..60) and track-name checks; Pin::check -> path when the file's SHA-256 matches.

## Invariant

A loaded configuration has absolute paths and bounded lane parameters; eel tracks are only shipped/internal.

## Cost

One file read; pins hash their files on use.

## Composition

Read by every command (main.rs).

## behavior_complement

Does not create directories, start workers or verify that executables are the right programs beyond their pinned hashes. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

The parsed Config serializes back to an equivalent JSON object; key order and formatting are not preserved.

## Tests

cargo test --manifest-path tools/hard-cases/Cargo.toml --offline (9 unit tests, two identical runs) Finite fixtures and live samples; not a proof.

## History

### 2026-11-04 · Tests

cargo test --manifest-path tools/hard-cases/Cargo.toml --offline (2026-11-04: 9 unit tests, two identical runs) Finite fixtures and live samples; not a proof.
