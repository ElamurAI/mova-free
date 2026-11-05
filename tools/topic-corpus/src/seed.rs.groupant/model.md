# mova-topic-corpus-seed

## Descriptant

The Whale hook: before a Whale asks its consultant, the closed corpus explains the terms it would ask about (whale reader gaps not yet known) from neighbour papers only; defining sentences only (score >= 3), defining facts only (stands_for, is_a, means), an expansion from a farther document only when all its words occur in the paper; the facts are retained in the Whale's knowledge store with provenance topic-corpus under its learner.lock. Source: `tools/topic-corpus/src/seed.rs`.

## Forward approximant

lease learner.lock (refuse when held) -> Store::open -> reader::read(paper) -> gaps minus known minus self-defined -> explain(k=5) -> term_facts -> filters -> retain Item{source: topic-corpus, corpus, doc, sentence}.

## Invariant

No write while the Whale learns; no web page or draft is a seed source; a known term is never seeded again.

## Cost

One paper read plus lazy explains.

## Composition

Parent: `topic-corpus`.

## behavior_complement

Does not change Whale code, does not mark questions asked, does not write descriptions. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

Each seeded item names its document and sentence; the explanation text is not stored in the item. No unique inverse is claimed.
