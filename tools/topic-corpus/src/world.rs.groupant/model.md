# mova-topic-corpus-world

## Descriptant

The small world of one paper against the cross corpus: entities (acronyms, proper names, frequent nouns) with mentions, the paper's own expansions, the best neighbour explanation and the corpus documents that speak of them; statements sorted into claim, method, result, statement with frames and entities. Source: `tools/topic-corpus/src/world.rs`.

## Forward approximant

parsed(paper) -> entity counts (the public `acronym` test, shared with `mova-whale-world`) -> ranking (acronyms first) -> definitions/links/explanations (lazy) -> statements with kinds -> world.json.

## Invariant

Deterministic for the same corpus state (checked twice on bryant-2017).

## Cost

Linear in paper sentences times entities plus text scans of the registered documents.

## Composition

Parent: `topic-corpus`.

## behavior_complement

Does not decide truth or importance; the kind of a statement is a lexical heuristic. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

Statements keep their sentence index and text; links keep document ids and counts only. No unique inverse is claimed.
