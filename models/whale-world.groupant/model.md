# whale-world

## Descriptant

Document dossiers and one global world for Whale epochs: per document an append-only set of texts (original, Mova drafts, consultant descriptions, earlier descriptions, web pages) with licence, provenance, inputs and citation counts; one incremental store that reads every text once with Mova into entities, statements, a word → sentence → document index and document edges; queries that give a writer a document's nearest neighbourhood with all its content, explanations and context packs with citations. A texts export writes the finished descriptions, their shadows (dossier drafts and earlier descriptions) and their cited sources into an editorial texts repository, idempotently and without overwriting edits. Crate `tools/whale-world` (`mova-whale-world`), a standalone workspace over `mova-topic-corpus` (library, graph, licence, annotate, explain_score, acronym, statement kinds, corpora) and `mova-whale` (knowledge norm_term/expansion, private_dir).

## Forward

library / site roots / topic corpora / learner states -> import -> dossier rows (append-only) ; epoch driver -> dossier add ; dossiers (epoch < E) + cached corpus neighbours -> world add (parse once, shards, buckets, doc entities, edges) -> world(E) -> queries (entity, explain, related, neighbourhood [+content], sentences, context). Batch log + passed learner rounds + dossiers -> export-texts -> texts repository (main article with footnotes, seo.json, shadows, index, manifest).

## Invariant

Dossier rows and files are never rewritten; changed files are refused everywhere; only library originals under open licences, not local-only, and only with public_originals, can be public_ok; web pages, consultant and Mova drafts and earlier descriptions never. The world keeps all content; world(e) shows only texts of earlier epochs; the same inputs give byte-identical dossiers and stores (checked twice live).

## Cost

Import 11.3 s for six papers (library load included). World build 25–32 s for 93 texts / 19,260 sentences (parsing dominates; cached neighbour parses are reused). Queries 10 ms – 1.1 s cold. 6.8 MB on disk.

## Composition

Leaves are file groupants under `tools/whale-world/src/**/*.rs.groupant`. The export reads dossiers (never creating one) and learner rounds and writes only into the given repository directory. Correct leaves do not prove the wiring: the interaction checks are the end-to-end tests (`tests.rs`: dossier -> world -> query, epochs, refusals) and the live six-paper build. The parent is `crab-orchestration`, beside `whale-workers` and `topic-corpus`; an epoch driver in `tools/whale` codes against the documented library API.

## behavior_complement

Domain: all configured runs. The crate does not: fetch pages, ask a terminal or a consultant, write descriptions, publish anything, commit or overwrite an edited exported file, delete or edit dossier texts, repair an interrupted world build, parse uncached corpus neighbours, or judge statements beyond lexical kinds. Counterexample: a CC0 web page and a CC BY-SA original stay public_ok false (tests).

## reverse_reconstruction

From a dossier row the exact text is recovered (hash-checked) with its source, licence and inputs; from a world citation {doc, text, s} the sentence is recovered from its shard and the text row names the dossier text. Parses are not mapped back to character offsets. Evidence status: source-derived with finite checks.

## tests

cargo test --manifest-path tools/whale-world/Cargo.toml --offline with WORLD_EN_MODEL=(see History) (9 unit tests). Negative controls (each red, then restored): public_ok without the original-kind condition; Dossier::read without the hash check; context statements citing a wrong sentence. Live: six papers imported and built twice into separate roots, byte-identical.

### Checks

## History

### 2026-11-05 · Invariant

Dossier rows and files are never rewritten; changed files are refused everywhere; only library originals under open licences, not local-only, and only with public_originals, can be public_ok; web pages, consultant and Mova drafts and earlier descriptions never. The world keeps all content; world(e) shows only texts of earlier epochs; the same inputs give byte-identical dossiers and stores (checked twice live on 2026-11-05).

### 2026-11-05 · tests

cargo test --manifest-path tools/whale-world/Cargo.toml --offline with WORLD_EN_MODEL=$MOVA_DATA/download/0/0.4/en.bin (9 unit tests, 2026-11-05). Negative controls 2026-11-05 (each red, then restored): public_ok without the original-kind condition; Dossier::read without the hash check; context statements citing a wrong sentence. Live: six papers imported and built twice into separate roots, byte-identical.

### 2026-11-05 · texts export

export::tests (5, no model) green; the whole crate 14 tests green with WORLD_EN_MODEL=en.bin.2026-11-05-redact (the 0.4 en.bin is refused: lexicon changed). Negative controls (each red, then restored): edit gate disabled; held original and page written as shadows; a clock value in the front matter. Live: 83 papers of batches 13-32 exported into a texts-repository worktree; second run wrote nothing; a fresh export identical by diff -r.
