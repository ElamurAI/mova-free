# Checks

Exercised through spec::tests (pin refusal), whale/agent/swarm tests (leases, private dirs, storage floor bounds 0 and u64::MAX).

Command: cargo test --manifest-path tools/whale/Cargo.toml --offline --locked (74 tests, fake teachers/executables; two identical runs on 2026-11-03).
These are finite fixtures; no live run, model call or deployment was exercised.
