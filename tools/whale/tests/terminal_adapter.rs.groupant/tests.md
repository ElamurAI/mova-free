# Checks

Command: cargo test --manifest-path tools/whale/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin; tests/terminal_adapter.rs (3 tests: answer, usage limit, a full Whale run) against a fake bash LLM in a real tmux server on a private socket, tmux required, never a model (2026-11-04)

Negative control 2026-11-04: ignoring the limit text makes the limit test wait out the timeout and fail; restored afterwards.
Finite fixtures; no live terminal; not a proof.

2026-11-04: 4 tests pass; negative control (adapter returns on the answer file alone) red, restored.
