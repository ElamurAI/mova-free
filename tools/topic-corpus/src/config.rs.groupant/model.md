# mova-topic-corpus-config

## Descriptant

Run configuration mova-topic-corpus-v1: library files, downloads, corpora root, limits (cited_by, similar, same_author, max_sentences, max_docs, hot, cache_max_bytes), Sea Spider socket/store, hard-cases list. Source: `tools/topic-corpus/src/config.rs`.

## Forward approximant

serde with deny_unknown_fields -> validate: schema, every path absolute, limits within bounds, hot <= max_docs, cache >= 1 MiB.

## Invariant

A loaded Config has only absolute paths and bounded limits.

## Cost

O(size of the JSON).

## Composition

Parent: `topic-corpus`.

## behavior_complement

No defaults for paths; no relative paths; unknown fields refused. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

The JSON is the whole state; defaults are filled for missing limits only. No unique inverse is claimed.
