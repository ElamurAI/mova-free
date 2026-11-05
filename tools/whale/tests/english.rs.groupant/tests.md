# Checks

Command: cargo test --manifest-path tools/whale/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin (unit tests incl. 5 service tests, 2 realizer tests, 3 length-gate tests; 2 English tests: 18 prompts, 10 refusals, a served end-to-end run with the real mova-whale child and a fake Codex executable; two identical runs on 2026-11-04)

Negative controls 2026-11-04: arrival-only ordering, recovery that ignores started lines, length bounds widened by one word, and dropping not/if from the deputy's scope check each turn the corresponding tests red; restored afterwards.
These are finite fixtures; no live teacher or Whale run; not a proof.
