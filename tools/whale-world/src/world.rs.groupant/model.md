# mova-whale-world-store

## Descriptant

The global world store (mova-whale-world-v1): every text of every dossier and the cached parses of topic-corpus neighbours, parsed once by Mova (with a heading fallback), as zstd shards; 256 word buckets and 64 entity buckets (FNV-1a), per-document entity counts, library and corpus-role edges, docs/texts tables; world(e) = texts of epochs < e; incremental adds under a lock with a pending marker; an in-process LRU for shards and buckets. Source: `tools/whale-world/src/world.rs`.

## Forward approximant

add(cfg, E, docs): lock -> refuse pending or another reader -> collect new texts (dossier rows epoch < E, hash-checked; cached corpus neighbours) -> library graph edges for new dossier docs -> pending.json -> per text: annotate_text -> sentences with statement kind and entities (acronyms, names, terms, compounds; acronym expansions) -> shard -> postings per bucket -> merge into touched buckets -> doc-entities -> append texts.jsonl, rewrite docs/edges/world.json -> pending -> last-build.

## Invariant

Text indices are dense and append-only; a text is parsed once; existing shards are never rewritten; world(e) views only texts of earlier epochs; the same dossiers and corpora built in the same order give byte-identical stores (checked twice live and in tests).

## Cost

Dominated by parsing new texts (about 25-32 s for six papers' 14 texts plus 79 cached neighbours, 19,260 sentences); index merge rewrites each touched bucket, so it grows with the store; queries read three small tables and the buckets/shards they touch.

## Composition

Parent: whale-world; uses topic_corpus::corpus::{annotate, Corpus, write_private, file_of, READER}, topic_corpus::world::{acronym, kind}, topic_corpus::graph::build, mova_whale::knowledge::{norm_term, expansion}; read by query.

## behavior_complement

Does not drop, rank away or summarize content; does not repair an interrupted build, re-read an indexed text, parse uncached corpus neighbours, fetch, or decide truth; statement kinds are lexical. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

Every sentence's text, words and frames are kept in its shard and every posting names (text, sentence), so a citation recovers its sentence and the text row recovers the dossier text id and hash; the original wording of a dossier text is recovered from the dossier, not from the world.
