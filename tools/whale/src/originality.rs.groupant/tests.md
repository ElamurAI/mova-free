# Checks

8 originality tests: exact/case, near copy, configured thresholds incl. invalid sizes, aggregate union with negative control, exempt fields, incomplete scope, four concurrent cold cache writers, cache tamper.

Command: cargo test --manifest-path tools/whale/Cargo.toml --offline --locked (74 tests, fake teachers/executables; two identical runs on 2026-11-03).
These are finite fixtures; no live run, model call or deployment was exercised.
