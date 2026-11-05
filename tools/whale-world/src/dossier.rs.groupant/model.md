# mova-whale-world-dossier

## Descriptant

The dossier of one document (mova-whale-dossier-v1): an append-only texts.jsonl index and texts/<epoch>-<kind>-<n>.md files (originals zstd), with licence, licence class, public_ok, hash, words, created time, inputs and citation counts; selection by epoch and kind from the index alone; hash-checked reads. Source: `tools/whale-world/src/dossier.rs`.

## Forward approximant

add: lock -> n = 1 + rows of (epoch, kind) -> refuse existing id -> create_new file (zstd for originals) -> fsync -> row (public_ok rule, citations scan) -> append + fsync. read: file -> decompress -> sha256 == row or refuse. citations: numeric and author-year markers against [n] lines and reference-section lines.

## Invariant

Rows are never rewritten and files never replaced; public_ok implies kind original, an open library licence, not local-only and public_originals; a changed file is refused by every reader.

## Cost

add and select are linear in the number of rows (the index is read whole); read is linear in the text; citations linear in text length times reference lines.

## Composition

Parent: whale-world; child module import; used by world::add and main.

## behavior_complement

Does not delete or edit texts, decide quality, resolve citations to documents, or publish anything; an author-year marker is matched only by surname and year. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

From a row the stored text is recovered exactly (hash-checked); the source path or URL and inputs name where it came from; an original is recovered after zstd decompression. Citation counts do not recover the markers.
