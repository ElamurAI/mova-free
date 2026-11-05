# Checks

16 tests: measured-time selection, deficit rule versus two-lane oracle, key/lane validation and cursor, pinned queue discovery with tamper, uncertain recovery, agent child run with branch prefix, override limits, STOP, short versus lifetime leases, pinned start, inherited lease, capacity, large history, unmeasured estimates, base rebinding, config gates.

Command: cargo test --manifest-path tools/whale/Cargo.toml --offline --locked (74 tests, fake teachers/executables; two identical runs on 2026-11-03).
These are finite fixtures; no live run, model call or deployment was exercised.
