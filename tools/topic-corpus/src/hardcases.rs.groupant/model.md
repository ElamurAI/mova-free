# mova-topic-corpus-hardcases

## Descriptant

The topic corpora's side of the one shared hard-cases list (`mova-hard-case-list`, format mova-hard-case-v2, the same file as the Wikipedia corpus's cases): judges a parse (no words, a tree without exactly one root, a head outside the sentence, no frame for six words or more), records found sentences with the document's licence and a pointer to the document, re-judges only its own open cases, and writes a parsed document as a mova-hard-bundle-v1 bundle for the Fish priority lane. Source: `tools/topic-corpus/src/hardcases.rs`.

## Forward approximant

issues(words, roots per tree, frames) -> reasons. record(found, licence, reader): build v2 Found records (source topic-corpus, id <doc>:<s>, document {doc, s}) -> hard_case_list::record for parser en. recheck: hard_case_list::recheck(source = topic-corpus, parser en) with the sentence check. bundle(corpus, doc, sentences) -> JSON with each sentence's CoNLL-U (forms, lemmas, UPOS, heads, relations; XPOS/FEATS unknown) and empty pragmatics rows.

## Invariant

A key is open from its first found until a learned event; found again after learned reopens it; appends are flocked and fsynced.

## Cost

O(list) per record/recheck (the whole file is read); bundle O(sentences).

## Composition

Parent: `topic-corpus`.

## behavior_complement

Does not delete cases, does not decide what the parser learns, never learns a case another source found (source-scoped recheck, tested). Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

The list keeps text, place and reasons of each case; the parse that failed is not kept. No unique inverse is claimed.
