# Checks

Command: cargo test --manifest-path tools/whale/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin (paper/terminal/reader tests: fixture paper with a fake terminal, pod config gates, lease exclusivity, licence admissibility; Mova-only dry run on five CC BY 4.0 papers, two byte-identical runs; 2026-11-04)

Negative controls 2026-11-04: appending answers to the description, asking on after a usage limit, and a lease that does not lock each turn the tests red; restored afterwards.
These are finite fixtures and a Mova-only dry run; no live terminal or teacher; not a proof.

2026-11-04: cargo test --manifest-path tools/whale/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin; tests/terminal_adapter.rs (3 tests: answer, usage limit, a full Whale run) against a fake bash LLM in a real tmux server on a private socket, tmux required, never a model (2026-11-04)
Negative control 2026-11-04: ignoring the limit text makes the limit test wait out the timeout and fail; restored afterwards.

2026-11-04: tests/terminal_adapter.rs an_answer_counts_only_when_the_terminal_is_idle_again (fake LLM writes the answer, works 1.5 s, then idles); negative control: returning on the answer file alone turns it red; restored.
