# mova-topic-corpus-corpus

## Descriptant

Stage 3: the closed topic corpus, lazy: a registry of documents (paper, author note, web page, consultant draft) whose originals stay where they are and stay compressed (library texts, Sea Spider's zstd store, zstd originals/), parsed on request whole (one discourse context per document) into UD words, frames and issues; the hot neighbourhood parsed at build and pinned; a bounded LRU parse cache; explanations of terms; the public gate and export. Source: `tools/topic-corpus/src/corpus.rs`.

## Forward approximant

register(doc,text): internal_only := true, public_admissible := licence rule, sha256, stored originals compressed. parsed(id): cache hit (sha, reader) -> decode; else text -> sha check -> annotate (whale reader parts/admitted sentences -> frames::read with one Ctx -> words with original forms, frames, issues incl. no_skeleton) -> zstd cache -> evict LRU non-hot over cap -> hard cases recorded. explain: text prefilter, then parse only documents containing the term; score definitional patterns and roles through the public `explain_score(forms, term, roles)` (also used by `mova-whale-world` over its global world).

## Invariant

Every document is internal; only open-licensed library texts are public-admissible; a changed original is refused, not silently re-read; hot documents are never evicted; files are 0600 in 0700 directories.

## Cost

Build: ~16 s for 80 registered and 12 hot documents (2,508 sentences) on bryant-2017; explain parses documents lazily (first ERRANT query 49 s, cached afterwards).

## Composition

Parent: `topic-corpus`.

## behavior_complement

Does not copy library texts, does not parse documents nobody asks for, does not export web pages, drafts or notes, does not publish anything itself. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

Originals are recoverable through the registry (path, Sea Spider URL+store, zstd original) and their hash; parses are recoverable from the cache only while cached. No unique inverse is claimed.
