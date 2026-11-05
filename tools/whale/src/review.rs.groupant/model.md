# mova-whale-review

## Descriptant

Review hints: the Whale's own terminal reads the Whale's current description next to the paper parts it describes and answers with typed hint lines (MISSING, WRONG, UNCLEAR, REPHRASE, ORDER). Hints are per-paper working materials, cached by (description hash, paper hash); they guide which questions come first; their text never enters a description. Source: `tools/whale/src/review.rs`.

## Forward approximant

requests(reading, description, parts, max_bytes): frame + description + as many described parts (paper order) as fit; further parts in further requests; each <= max_bytes (100,000 by default, 20,000..100,000); None when the description alone does not fit. parse(answer): `KIND: part | text` lines (ORDER: text), deduplicated, at most 40. cached/store/latest: hints/index.jsonl + round-NNN.json + round-NNN.md. focus(hints): content words of missing/wrong/unclear hints.

## Invariant

A (description, paper) pair is reviewed at most once; every hint answer is kept with the Whale's answers, so the answer-overlap gate refuses any 6-gram copy of it in a description; no request exceeds max_bytes.

## Cost

One request per <= max_bytes chunk (usually one); parsing linear in the answer; learn.rs asks only when a round gained fewer than min_gain_words words or the draft passes.

## Composition

Parent: whale-workers. Called by learn.rs (when to review, ask, store, journal `review`/`review_cached`/`hints`; `gaps` orders terms and sentences by `focus`); configured by batch.rs/learn.rs (`review`).

## behavior_complement

Not done: rewriting or reordering the description from hints (the writer restates in paper order and keeps every restatable clause; rephrase/order hints are journaled as ignored), checking that a hint is right, reviewing parts the description does not cover, a hint vocabulary beyond five kinds. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

round-NNN.json names the description and paper hashes, request hashes and sizes and answer hashes; with knowledge/answers/SHA.txt the answers are recovered. Hints do not determine the answer text.

## History

### 2026-11-04 · document

#### Epoch 2026-11-04 (created)

Fake-terminal tests only; no live review was run.
