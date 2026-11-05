# Checks

Exercised by the live runs of 2026-11-04 on bryant-2017 (graph twice byte-identical, missing --dry twice identical, build, explain, world twice identical, seed, cite, web-plan, sol-import, sol-claims).

cargo test --manifest-path tools/topic-corpus/Cargo.toml --offline with WORLD_EN_MODEL=$MOVA_DATA/download/0/0.4/en.bin (18 unit tests; fixture library, fake `papers` binary, fake terminal, local Sea Spider store; two identical runs on 2026-11-04). Without the model the model-dependent tests report a skip.

Negative controls 2026-11-04 (each red, then restored): public admission without the origin condition; assert_public without the Sea Spider/draft/note refusals; seeding without the learner lock; shadow-library filter off; public citing that ignores refusals; cache eviction ignoring the hot pin.

These are finite fixtures and live samples; not a proof.
