# Checks

missing::tests (ranking, routes incl. a title-keyed arXiv record, request dir, never rewritten, disk guard refusal; a shadow-library copy never queued; civil date). Negative control 2026-11-04: shadow() always false turns two tests red. Live 2026-11-04 bryant-2017: 201 candidates, 23 queued, 174 list-only, two identical dry runs.

cargo test --manifest-path tools/topic-corpus/Cargo.toml --offline with WORLD_EN_MODEL=$MOVA_DATA/download/0/0.4/en.bin (18 unit tests; fixture library, fake `papers` binary, fake terminal, local Sea Spider store; two identical runs on 2026-11-04). Without the model the model-dependent tests report a skip.

Negative controls 2026-11-04 (each red, then restored): public admission without the origin condition; assert_public without the Sea Spider/draft/note refusals; seeding without the learner lock; shadow-library filter off; public citing that ignores refusals; cache eviction ignoring the hot pin.

These are finite fixtures and live samples; not a proof.
