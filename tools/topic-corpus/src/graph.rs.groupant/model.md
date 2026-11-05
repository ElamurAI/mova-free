# mova-topic-corpus-graph

## Descriptant

Stage 1: the neighbourhood of one paper: topics (taxonomy names), concepts, tags, authors (homepage, h-index), references (bibliography entries resolved to library works or kept external with ids), library citers, similar works, works of the same authors; every node with ids, licence class, public admissibility and the text we hold. Source: `tools/topic-corpus/src/graph.rs`.

## Forward approximant

build: find paper -> bibliography + resolved refs (deduplicated) -> citers sorted by cited_by -> similar = idf-weighted keyword cosine + 0.5 * shared topic share + 0.2 shared concept (>=0.05 or a shared topic), top k, ties by id -> same author by OpenAlex author ids. members(): paper, references, similar, cited_by, same_author, each id once with its roles.

## Invariant

Deterministic: same library -> byte-identical JSON (checked twice on bryant-2017). Scores rounded to 4 decimals.

## Cost

O(N * keywords) for similarity over all works plus one pass each over citations and refs.

## Composition

Parent: `topic-corpus`.

## behavior_complement

Does not include citers outside the library (OpenAlex citing works are not in the slim snapshot); does not read texts. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

Reference entries are kept verbatim; similarity scores do not identify which keywords matched. No unique inverse is claimed.

## History

### 2026-11-04 · Invariant

Deterministic: same library -> byte-identical JSON (checked twice on bryant-2017, 2026-11-04). Scores rounded to 4 decimals.
