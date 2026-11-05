# Checks

Command: cargo test --manifest-path tools/whale/Cargo.toml --offline --locked with MOVA_OCTOPUS_TEST_MODEL=$MOVA_DATA/download/0/0.4/en.bin (paper/terminal/reader tests: fixture paper with a fake terminal, pod config gates, lease exclusivity, licence admissibility; Mova-only dry run on five CC BY 4.0 papers, two byte-identical runs; 2026-11-04)

Negative controls 2026-11-04: appending answers to the description, asking on after a usage limit, and a lease that does not lock each turn the tests red; restored afterwards.
These are finite fixtures and a Mova-only dry run; no live terminal or teacher; not a proof.

2026-11-04: reader changes covered by learn/gates tests and a five-paper dry run (parts per paper 4/8/10/5/14; sentences without a clause 112/49/27/198/32 of 286/222/253/374/179).

reader::writer_tests::questions_subordinates_and_reference_lists (2026-11-04): a question stays a question (red when the question branch is disabled), a fronted subordinate clause is attached, a coordinated base form keeps its subject, «Note that» is restated, a reference entry is not read.

2026-11-04: tests of the batched/pod/stall/review/grammar/filter/selection changes listed in the crate README «Checks»; negative controls (grammar gate always passing, pod claim ignored, stall off, extraction check off) each turned their tests red, restored.
