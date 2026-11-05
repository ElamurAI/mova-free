# mova-hard-cases-main

## Descriptant

CLI of mova-hard-cases: list, wave, lane, guard, harvest, witness, eel, sample, measure, recheck, watch. Source: `tools/hard-cases/src/main.rs`.

## Forward approximant

args -> Config::load -> command -> pretty JSON on stdout or error with nonzero exit.

## Invariant

Every command reads one configuration; no command starts a teacher call (wave, harvest and judge read and write files; eel runs the engine once; guard and watch loop until STOP).

## Cost

That of the chosen command.

## Composition

Binary entry; parent hard-cases.

## behavior_complement

No shell, no arbitrary executable arguments: executables come from the pinned configuration. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

The printed JSON is the command's report; inputs are recoverable from the configuration and lane files.

## Tests

cargo test --manifest-path tools/hard-cases/Cargo.toml --offline (9 unit tests, two identical runs) Finite fixtures and live samples; not a proof.

## History

### 2026-11-04 · Tests

cargo test --manifest-path tools/hard-cases/Cargo.toml --offline (2026-11-04: 9 unit tests, two identical runs) Finite fixtures and live samples; not a proof.
