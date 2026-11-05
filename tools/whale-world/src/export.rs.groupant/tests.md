# Checks

cargo test --release --manifest-path tools/whale-world/Cargo.toml --offline (export::tests, 5 tests, no model needed; whole crate 14 tests green with WORLD_EN_MODEL=en.bin.2026-11-05-redact, 2026-11-05): selection by batch and gates with a from_batch negative control; sentence-run citations, markers stripped equal to the description, shadows without the duplicate Mova draft, second run writes nothing, a fresh repository byte-identical; an edited article and an edited seo.json kept on two runs, the upstream change let through once the edit is undone, a foreign file kept; a CC BY-SA page and the original cited by URL/file name and licence, their sentences absent from every file (the dossier holds them: control); reference mentions with wrong-year and no-surname controls; seal and intact.

Negative controls 2026-11-05 (each red, then restored): edit gate disabled (edited test red); held original and page texts written as shadows (idempotence and never-copied tests red); a clock value in the front matter (idempotence test red).

Live 2026-11-05: 83 papers into a texts-repository worktree, second run 0 written / 272 unchanged; a fresh export into a scratch directory identical by `diff -r`.

These are finite fixtures and live samples; not a proof.
