# Checks

6 tests: word/scalar units, equality and fence refusal, object rules, verbatim runs, projection boundaries, status rewrite and artifact order.

Command: cargo test --manifest-path tools/whale/Cargo.toml --offline --locked (74 tests, fake teachers/executables; two identical runs on 2026-11-03).
These are finite fixtures; no live run, model call or deployment was exercised.

2026-11-04: cargo test --manifest-path tools/whale/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin (unit tests incl. 5 service tests, 2 realizer tests, 3 length-gate tests; 2 English tests: 18 prompts, 10 refusals, a served end-to-end run with the real mova-whale child and a fake Codex executable; two identical runs on 2026-11-04)
