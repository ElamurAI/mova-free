# Checks

hardcases::tests::found_once_open_until_learned (v2 records with licence and document pointer; red control: another source's case is not rechecked; bundle CoNLL-U shape). Live 2026-11-04: 309 cases (all no_skeleton) from the hot neighbourhood of bryant-2017.

cargo test --manifest-path tools/topic-corpus/Cargo.toml --offline with WORLD_EN_MODEL=$MOVA_DATA/download/0/0.4/en.bin (18 unit tests; fixture library, fake `papers` binary, fake terminal, local Sea Spider store; two identical runs on 2026-11-04). Without the model the model-dependent tests report a skip.

Negative controls 2026-11-04 (each red, then restored): public admission without the origin condition; assert_public without the Sea Spider/draft/note refusals; seeding without the learner lock; shadow-library filter off; public citing that ignores refusals; cache eviction ignoring the hot pin.

These are finite fixtures and live samples; not a proof.

2026-11-04 (branch hard-loop): 17 of 18 pass; seed::tests::seeds_terms_from_neighbour_papers_and_respects_the_learner_lock fails on main as well (unrelated to this change).
