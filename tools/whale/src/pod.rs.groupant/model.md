# mova-whale-pod

## Descriptant

A pod's shared knowledge: one store for all Whales of a batch pod, keyed by normalized term (`knowledge::norm_term`), with provenance per Whale, paper, item and answer. A term one Whale asked is not asked by another; what one Whale learned, every Whale explains with. Bare-category facts go to review instead of knowledge. Source: `tools/whale/src/pod.rs`.

## Forward approximant

open(dir): terms.jsonl (entries), claims.jsonl (first claim per term wins), review.jsonl, answers/. publish(whale, store): under pod.lock, every term item of the store not yet published -> non-bare facts as an entry, bare facts to review, the answer file copied; every asked term key -> a claim. claim(term, whale, paper): under pod.lock, reload; true and recorded only when no claim and no facts exist for the normalized term. items(exclude): entries of other Whales as knowledge items `pod:<whale>:<item>`, ordered by (term, whale, item). answers(): every copied answer text (for the overlap gate).

## Invariant

Appends only, fsynced, under a blocking flock; a (whale, item) pair is published at most once; a normalized term has at most one effective claim; items() depends on the file's content, not its order; answer texts never leave the pod except into the overlap gate.

## Cost

Each claim/publish rereads the three files (linear in pod size: hundreds of lines per batch); items() sorts O(n log n).

## Composition

Parent: whale-workers. Written and read by learn.rs (publish at start and after each round, claim before asking a term, items for Known::of and the gates' item lookup, answers for the Guard); configured by batch.rs (`pod`).

## behavior_complement

Not done: retracting or correcting a fact, choosing between conflicting facts of two Whales beyond «first per relation in the reader's order», sharing sentence or section readings (paper-specific), locking across machines (flock is local), deduplicating identical facts from different Whales. Parallel learners make append order timing-dependent; a Whale's view in a given round therefore depends on what the others had published by then. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

Every entry names its Whale, paper, item id and answer hash; with the Whale's own store and the copied answer the source answer is recovered. A claim names who asked first; the facts do not determine the answer text.

## History

### 2026-11-04 · document

#### Epoch 2026-11-04 (created)

Measured on a fresh pod with today's real answers replayed (10 papers, 5 Whales, sequential, two identical runs): 81 term entries, 118 claims, 26 review records; no term asked twice across the pod.
