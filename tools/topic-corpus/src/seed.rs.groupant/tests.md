# Checks

seed::tests (learner.lock red control, GEC seeded from the reference, not seeded twice). Live 2026-11-04 on a fresh Whale for bryant-2017: 1 term seeded (NTHU); dry learn round 3,124 words with and without it (the term occurs only in tables).

cargo test --manifest-path tools/topic-corpus/Cargo.toml --offline with WORLD_EN_MODEL=$MOVA_DATA/download/0/0.4/en.bin (18 unit tests; fixture library, fake `papers` binary, fake terminal, local Sea Spider store; two identical runs on 2026-11-04). Without the model the model-dependent tests report a skip.

Negative controls 2026-11-04 (each red, then restored): public admission without the origin condition; assert_public without the Sea Spider/draft/note refusals; seeding without the learner lock; shadow-library filter off; public citing that ignores refusals; cache eviction ignoring the hot pin.

These are finite fixtures and live samples; not a proof.

2026-11-05 (release model 2483516a): the seed test failed at `known_terms().contains("GEC")`. The Whale store keeps known terms normalized (`knowledge::norm_term`, «GEC» → «gec»), so the raw-string check could never hold, and `seed` compared an explicit term with the normalized set unnormalized, so a second seed of an explicit acronym was not recognised as known. Fix: `seed` normalizes the explicit term before the known check; the test asserts the normalized key. 18/18 pass with the release model; the «not seeded twice» assertion is the control for the code change.
