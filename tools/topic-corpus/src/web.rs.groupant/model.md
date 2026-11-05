# mova-topic-corpus-web

## Descriptant

Stage 4: pages through Sea Spider only: plan (authors' homepages, claim pages, given URLs; no shadow hosts), queue file, English prompts of at most 16 URLs asked as caller whale, import of stored pages as web documents referencing Sea Spider's store, with the licence the page states and the site's terms link. Source: `tools/topic-corpus/src/web.rs`.

## Forward approximant

plan -> queue/prompts -> ask_animal per batch -> Reader::open(store).get(url) -> 2xx html/text only -> page_markdown (drop script/style/nav/header/footer/form/aside, headings, entities) -> page_licence (rel=license, CC link, rights meta, copyright line) -> register(kind web, origin Seaspider, source_kind seaspider).

## Invariant

Every page is internal and never public-admissible, whatever licence it states.

## Cost

Linear in page bytes; asking is one socket round trip per batch.

## Composition

Parent: `topic-corpus`.

## behavior_complement

Does not fetch anything itself, does not follow links, does not run JavaScript. Evidence status: source-derived model with finite checks, not a proof.

## reverse_reconstruction

The store keeps the raw body; the Markdown is a lossy projection of it. No unique inverse is claimed.
