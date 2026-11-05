# mova-topic-corpus-cite

## Descriptant

Stage 5: cited descriptions: every written sentence of a Whale round (draft.json) gets the numbers of its sources — the paper for restated sentences and structure, the corpus document for topic-corpus knowledge, the Whale's consultation for terminal knowledge — and a bibliography with stable ids; public mode refuses any sentence based on a non-admissible source. Source: `tools/topic-corpus/src/cite.rs`.

## Forward approximant

items + knowledge provenance -> source ids -> numbering in first-use order -> «text [n, m].» -> paragraphs by part -> ## Bibliography (entry + id).

## Invariant

Every sentence cites at least one source; in public mode only admissible sources (open-licensed library texts, the consultation notes) may be cited.

## Cost

Linear in draft items plus knowledge items.

## Composition

Parent: `topic-corpus`.

## behavior_complement

Does not change the Whale's description; does not judge whether a source supports the sentence. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

The citation map per sentence is returned; the draft text is recoverable by removing the markers. No unique inverse is claimed.
