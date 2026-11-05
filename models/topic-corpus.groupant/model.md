# topic-corpus

## model

# topic-corpus

## Descriptant

Topic graphs and closed topic corpora for the paper-describing Whales (our Owls): a paper's neighbourhood from the library (stage 1), download requests for the most-cited missing open copies through the library's own driver (2), a lazy cross corpus annotated by Mova with a bounded parse cache and a shared hard-cases list (3), the Whale seed hook, pages through Sea Spider kept internal (4), cited descriptions (5), consultant drafts turned into claims followed to originals (6), and the paper's small world. Crate `tools/topic-corpus` (`mova-topic-corpus`), a standalone workspace depending on `mova-whale` (reader, knowledge store, terminal adapter), `mova-seaspider` (store reader), `mova-octopus` (relay), `world` and `en`.

## Forward

library -> graph (neighbourhood) -> missing (ranked, routed, request dir; nothing started) | corpus (register cross corpus, parse hot, parse others on request) -> world (paper's small world) / seed (Whale knowledge, under learner.lock) / web (Sea Spider queue and import) / sol (terminal draft -> claims -> download and page requests feeding missing and web) -> cite (draft.json + knowledge provenance + corpus -> cited description).

## Invariant

Every corpus document is internal; only open-licensed library texts are public-admissible; Sea Spider pages and their trees, consultant drafts and our notes never go public (operator rule enforced in licence, corpus::assert_public, public_export and cite --public). No stage starts a download stream, a terminal session or a fetch by itself.

## Cost

Library load about 10 s (186k export rows); one OpenAlex snapshot scan per request (10-20 s); hot build about 16 s for 12 documents (2,508 sentences); further parses lazy and cached under a byte cap.

## Composition

Leaves are file groupants under `tools/topic-corpus/src/*.rs.groupant`. Correct leaves do not prove the wiring: the interaction checks are the fixture tests that run graph -> missing -> corpus -> seed/web/cite/sol/world on one fixture library, and the live sample of on bryant-2017. The Whale hook writes the Whale's knowledge store through `mova_whale::knowledge::Store` (no Whale code change); the parent is `crab-orchestration`, beside `whale-workers` and `seaspider`.

## behavior_complement

Domain: all configured runs. The crate does not: download (the library driver does), fetch pages (Sea Spider does), run a consultant without an explicit command and an untaken terminal lease, write a Whale's store while it learns, publish anything, admit a web page, draft or note to a public output, or judge claims beyond word overlap. Counterexample: a web record forged as public-admissible is refused by assert_public.

## reverse_reconstruction

From the corpus registry every original is recoverable (path, Sea Spider URL and store, zstd original) with its hash; from a request directory every candidate with its route; from claims every citation's resolution. Parses are recoverable only while cached. Evidence status: source-derived with finite checks.

## tests

cargo test --manifest-path tools/topic-corpus/Cargo.toml --offline with WORLD_EN_MODEL=(see History) (18 unit tests; fixture library, fake `papers` binary, fake terminal, local Sea Spider store; two identical runs). Without the model the model-dependent tests report a skip.

Negative controls (each red, then restored): public admission without the origin condition; assert_public without the Sea Spider/draft/note refusals; seeding without the learner lock; shadow-library filter off; public citing that ignores refusals; cache eviction ignoring the hot pin.

Live on bryant-2017-automatic-annotation-evaluation-error-types: graph twice byte-identical (10 s); missing --dry twice identical (201 candidates, 23 queued); build (80 registered, 12 hot, 2,508 sentences, 309 hard cases); world twice identical; cite of owl-1 round 28 (247 sentences); sol claims from a stand-in draft (6 claims).

### Checks

## History

### 2026-11-04 · Invariant

Every corpus document is internal; only open-licensed library texts are public-admissible; Sea Spider pages and their trees, consultant drafts and our notes never go public (operator rule 2026-11-04, enforced in licence, corpus::assert_public, public_export and cite --public). No stage starts a download stream, a terminal session or a fetch by itself.

### 2026-11-04 · Composition

Leaves are file groupants under `tools/topic-corpus/src/*.rs.groupant`. Correct leaves do not prove the wiring: the interaction checks are the fixture tests that run graph -> missing -> corpus -> seed/web/cite/sol/world on one fixture library, and the live sample of 2026-11-04 on bryant-2017. The Whale hook writes the Whale's knowledge store through `mova_whale::knowledge::Store` (no Whale code change); the parent is `crab-orchestration`, beside `whale-workers` and `seaspider`.

### 2026-11-04 · tests

cargo test --manifest-path tools/topic-corpus/Cargo.toml --offline with WORLD_EN_MODEL=$MOVA_DATA/download/0/0.4/en.bin (18 unit tests; fixture library, fake `papers` binary, fake terminal, local Sea Spider store; two identical runs on 2026-11-04). Without the model the model-dependent tests report a skip.

Negative controls 2026-11-04 (each red, then restored): public admission without the origin condition; assert_public without the Sea Spider/draft/note refusals; seeding without the learner lock; shadow-library filter off; public citing that ignores refusals; cache eviction ignoring the hot pin.

Live 2026-11-04 on bryant-2017-automatic-annotation-evaluation-error-types: graph twice byte-identical (10 s); missing --dry twice identical (201 candidates, 23 queued); build (80 registered, 12 hot, 2,508 sentences, 309 hard cases); world twice identical; cite of owl-1 round 28 (247 sentences); sol claims from a stand-in draft (6 claims).
