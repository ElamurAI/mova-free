# mova-whale-world-import

## Descriptant

Importers of existing material into a dossier: the library original, earlier site descriptions, topic-corpus web pages (read through the corpus from Sea Spider's store) and consultant drafts, and Mova's learner drafts (last or every round); idempotent by kind, hash and source; nothing fetched, no terminal asked. Source: `tools/whale-world/src/dossier/import.rs`.

## Forward approximant

library.get(doc) -> text path (open or local) -> add original(epoch 0, licence) ; site roots -> <a>/<ab>/<abc>/<doc> or <a>/<ab>/<doc> -> files -> add site-old ; corpus docs kind web/sol -> Corpus::text -> add web(0, URL)/sol(1) ; state roots -> papers/<doc>/round-* sorted -> last|all -> add mova(1, inputs: original).

## Invariant

Importing twice adds nothing; imported web, sol, mova and site-old texts are never public_ok.

## Cost

Library load about 10 s (done once per command by main); then linear in the imported texts.

## Composition

Parent: mova-whale-world-dossier; reuses topic_corpus::corpus::Corpus and topic_corpus::library::Library.

## behavior_complement

Does not fetch pages, ask a terminal, parse texts, or import a learner round that has no description.txt; earlier-description layouts other than the two prefix forms are not found. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

Each imported row keeps its source path or URL and the source file's mtime, so the material can be found again; corpus-held texts keep the corpus key and document id.
