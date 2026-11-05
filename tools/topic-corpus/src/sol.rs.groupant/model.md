# mova-topic-corpus-sol

## Descriptant

Stage 6: draft articles by an Owl's own terminal on its paper's topics with as many citations as possible (internal, low quality), registered in the corpus; split into claims with citation numbers; reference entries parsed (DOI, arXiv, ACL, URL, year, title) and resolved: library text, library without text, download request (id or title), Sea Spider page, unresolved; a claim is sourced, pending or kept «from Sol»; support from the best-overlapping sentence of a held original. Source: `tools/topic-corpus/src/sol.rs`.

## Forward approximant

prompt -> PrivateTerminal::ask once -> store(sol doc) -> split(body markers, numbered references) -> parse_entry -> resolve -> claims + requests -> save under <corpus>/sol/.

## Invariant

A draft is never public-admissible and never enters a description; a terminal error registers nothing.

## Cost

One terminal question; linear text work.

## Composition

Parent: `topic-corpus`.

## behavior_complement

Does not retry a terminal question, does not judge claim truth beyond word overlap, does not fetch. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

Claims keep their text and resolved citations; the draft stays compressed in originals/. No unique inverse is claimed.

## History

### 2026-11-04 · document

#### Epoch 2026-11-04

`Question` gained `items` (batched questions in the Whale crate); the article request sets it empty.

### 2026-11-05 · document

#### Epoch 2026-11-05 (split moved)

`split` now re-exports `mova_whale::claims::split` (moved unchanged), shared with the Whale epochs' citation check; sol::tests pass.
