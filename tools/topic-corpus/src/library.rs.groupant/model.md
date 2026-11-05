# mova-topic-corpus-library

## Descriptant

Read-only view of the paper library: export rows, citation edges, resolved reference lists, per-paper refs.json bibliographies, OpenAlex topic taxonomy, author positions; id/DOI/arXiv/ACL/title resolution and text discovery. Source: `tools/topic-corpus/src/library.rs`.

## Forward approximant

load: stream export JSONL -> Work rows with keyword ids (title+abstract words, stop words out) and document frequencies -> indices by id, DOI (+ACL DOI), arXiv, ACL, normalized title (>=3 words). resolve(entry): DOI > arXiv > ACL > title with year +-1. text(w): text.md (open unless local-only) > text.local.md > eprint.md.

## Invariant

First row of an id wins; stale layout paths are matched by their last component; no network.

## Cost

One pass over the export (~186k rows) plus one pass over citations.tsv (2.57M lines) or refs.jsonl per query.

## Composition

Parent: `topic-corpus`.

## behavior_complement

Does not write anything; does not fetch; title matches need three words and the year within one. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

From a Work the export row's graph fields are recoverable; abstracts are reduced to keyword ids and not recoverable. No unique inverse is claimed.
