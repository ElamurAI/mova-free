# mova-whale-world-query

## Descriptant

Queries for writers over world(e): entity (kinds, forms, mentions, docs, grouped expansions), explain (explain_score plus a text-kind bonus, two per document, duplicates dropped), related (cites, cited by, same author, similar, idf shared-entity share), neighbourhood (breadth-first nearest documents with texts; --content streams every sentence), sentences (round-robin diverse examples), context (entities with explanations from the neighbourhood, related documents, every statement of the neighbourhood, optional byte cut); paging by offset and k. Source: `tools/whale-world/src/query.rs`.

## Forward approximant

entity: key -> bucket -> visible postings -> by_doc, grouped definitions. explain: entity postings U rarest-word postings -> visible -> sentence -> explain_score (+ case-folded retry) + kind bonus -> sort (score, doc, text, s) -> two per doc, dedup -> page. related: edges both ways + idf of shared doc entities -> score -> sort. nearest: BFS over related up to radius and max_docs. context: nearest -> doc entities -> explain_in(within neighbourhood, excluding doc) -> statements of texts_of each near doc -> cut an eighth of the larger list until it fits.

## Invariant

Answers are deterministic (ties by doc, text, sentence); every citation names a document of docs.jsonl and a sentence of that text's shard; nothing in the store is dropped to answer.

## Cost

entity 10 ms, explain 0.2-0.3 s, related 70 ms, sentences 18 ms, neighbourhood 73 ms, its content 0.2 s, context 0.9-1.1 s on the six-paper world (cold process); explain and context scan all candidate postings (no cap).

## Composition

Parent: whale-world; reads world::World; uses topic_corpus::corpus::explain_score.

## behavior_complement

Does not write the store, rank by truth, read texts of later epochs than asked, or include documents without visible texts in related; shared-entity strength counts general nouns too. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

Each answer item carries {doc, text, s}, from which the sentence is recovered exactly from the world; scores do not recover their components beyond the listed edges.

## History

### 2026-11-05 · Cost

entity 10 ms, explain 0.2-0.3 s, related 70 ms, sentences 18 ms, neighbourhood 73 ms, its content 0.2 s, context 0.9-1.1 s on the six-paper world (cold process, 2026-11-05); explain and context scan all candidate postings (no cap).
