# wikt-extract-entry

## descriptant

### wikt-extract-entry

#### Descriptant

Written with the code (tools/wikt-extract, first version). A page's wikitext → its ==English== section (to the next level-2 heading or ----) → entries: each part-of-speech heading (fixed list) under the current etymology number is one entry; its first template line is the head (forms from en-noun/en-verb/en-adj/en-adv/en-proper noun/head parameters, head labels from tlb); # and ## lines are senses rendered by wikitext::render; #: lines add examples and in-sense nyms to the last sense; #* lines count quotations; Synonyms/Antonyms subsections add section nyms from l/col/syn templates and plain links. Logical lines join lines while templates are open. Entries without senses are dropped. row() formats 13 tab-separated cells: word, pos, numbered meaning with bracketed labels, empty lesson sense, numbered examples, etymology note, labels, synonyms, antonyms, forms, form-of, gaps, provenance.

The forward model below is source-derived; the tests in model.md#tests exercise the actual functions on small synthetic pages, and the full English pass (docs/data.md, «Wiktionary») is the corpus-scale observation. Executable percentage approximants are not supported: the outputs are categorical text records with no useful error metric.

## approximant

### Forward relation

A page's wikitext → its ==English== section (to the next level-2 heading or ----) → entries: each part-of-speech heading (fixed list) under the current etymology number is one entry; its first template line is the head (forms from en-noun/en-verb/en-adj/en-adv/en-proper noun/head parameters, head labels from tlb); # and ## lines are senses rendered by wikitext::render; #: lines add examples and in-sense nyms to the last sense; #* lines count quotations; Synonyms/Antonyms subsections add section nyms from l/col/syn templates and plain links. Logical lines join lines while templates are open. Entries without senses are dropped. row() formats 13 tab-separated cells: word, pos, numbered meaning with bracketed labels, empty lesson sense, numbered examples, etymology note, labels, synonyms, antonyms, forms, form-of, gaps, provenance.

#### Domain

Well-formed UTF-8 wikitext/XML of the English Wiktionary dump export format (2026-09-01 run); bounded resources.

## models

# Typed source models

Partial source-derived relations at this manifest snapshot; one call under the stated domain; no universal semantic proof.

## behavior_complement

Does not read other language sections, translation subpages, Thesaurus pages, pronunciation, etymology text, quotation text, Usage notes, Derived/Related terms, Alternative forms or Translations; forms are not computed by the Lua module (irregular forms only when written as parameters; inline specifications kept raw); a POS heading outside the fixed list is not an entry.

Domain: Well-formed UTF-8 wikitext/XML of the English Wiktionary dump export format (2026-09-01 run); bounded resources.

## reverse_reconstruction

From a row one recovers the page title, part of speech, sense texts and counts, labels and nyms per sense, the head forms and the exact source location (page id, revision, file, stream offset) — so the original wikitext is recoverable from the dump, not from the row; the row itself is a lossy projection (template names, quotation texts and dropped sections are not recoverable).

Domain: Well-formed UTF-8 wikitext/XML of the English Wiktionary dump export format (2026-09-01 run); bounded resources. Preimages are sets, not unique inverses: the rendering is not injective.

## forms_rules

plural: +es after s/x/z/ch/sh, consonant+y → ies, else +s; verb: 3sg as plural with -o → -oes, -ing drops a final e (not ee/ye/oe) and turns ie → ying, past adds d after e, ied after consonant+y, ed otherwise; one-syllable CVC words (not ending in w, x, y) double the final consonant when no parameters or ++ are given; adjective er adds er/est with the same e/y/CVC rules.

## section_ownership

Synonyms/Antonyms deeper than the POS heading, or at its level, belong to the last entry; other subsections switch reading off until the next POS heading.

## tests

### Native evidence

Package/bin: `wikt-extract`. Selected tests (`cargo test --release -p wikt-extract`), all passing:

- `entry::tests::telly_entries`
- `entry::tests::default_forms`
- `entry::tests::no_english_section`

Negative control: telly_entries asserts that the French section is excluded, the translation lines are not read and an unknown template appears in the row's gaps cell; no_english_section asserts None for a page without English.

Scope: actual functions on synthetic in-memory inputs; the corpus-scale pass is an observation, not a proof. Lua module semantics, full template coverage and the correctness of every printed sense are not established.

## invariant-and-cost

### Invariant and cost

#### Invariant

Each row has exactly 13 cells, no tab or newline inside a cell; sense numbers are 1..k for top-level senses and k.j for subsenses; an entry has at least one sense; gaps of a row are the sum of its lines' gaps.

#### Cost

O(L) per page section of L bytes plus render costs per line; memory O(L) for logical lines and the entries.

## History

### 2026-11-04 · descriptant

Heading `#### Epoch 2026-11-04` stays in descriptant as `#### Descriptant`.

### 2026-11-04 · tests

Heading `### Native evidence — 2026-11-04` stays in tests as `### Native evidence`.
