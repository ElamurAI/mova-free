# Checks

7 tests with a fake executable: two disjoint workers once, uncertain launch, owned markers, identity changes, short versus lifetime lease semantics, descendant lease uncertainty and manifest tamper, cohort separation.

Command: cargo test --manifest-path tools/whale/Cargo.toml --offline --locked (74 tests, fake teachers/executables; two identical runs on 2026-11-03).
These are finite fixtures; no live run, model call or deployment was exercised.
