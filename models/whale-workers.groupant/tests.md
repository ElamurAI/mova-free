# Checks

cargo test --manifest-path tools/whale/Cargo.toml --offline --locked (74 tests, fake teachers/executables; two identical runs on 2026-11-03).
Negative control: disabling the pin hash comparison in util::pinned turns four tests red (spec, whale, agent template and swarm queue pins); restored afterwards.
