# Mova RAG

Retrieval-augmented generation needs documents that answer more than their own text. This page describes how Mova expands a document into layers, what it can already provide without any AI, and how each piece is measured. For now this covers English only, with standard annotation.

## How it should work: an example

Take a real text document, "Weather in Vinnytsia on 2026-09-25". It is then expanded with everything relevant to it:
- neighboring towns and villages;
- the weather on this date in past years;
- historical background, from archives and from our own modeling;
- the forecast ahead;
- a description of the fronts.

So the document is a core wrapped in layers. Each layer answers questions the core itself does not answer. The same template on a text document, for example a developer diary:

| layer | weather in Vinnytsia | text document |
| --- | --- | --- |
| core | today's weather | the text itself |
| neighbors | towns and villages around | related entities and documents with shared terms |
| past | this date in past years, normals, records | earlier mentions, previous revisions, how it changed |
| future | forecast | what is planned, consequences |
| mechanism | fronts, pressure centers | why: causal links |
| names | Vinnytsia, Vinnitsa, Winnica, UKWW, WMO 33562 | names, abbreviations, alternative spellings |

## What Mova can provide without AI

In English, on standard annotation (`en`: tagger, in-code dictionary, parser):

1. **POS-aware lemmas** as a separate zone. For example, built → build, markets → market. The dictionary has 315k entries. Character n-grams catch a shared root with noise; a lemma does it without noise.
2. **Phrases from the tree.** Noun groups with compound and amod relations become terms: trade route, estate privileges. A phrase is more precise than a bag of words.
3. **Neighbors.** Documents that share key lemmas and phrases, as a separate zone. It must read as an "index map", not as facts.
4. **Rule-based name shadow.** Spelling variants, hyphens, abbreviations: TT-2 / TT2 / Tinto Talks 2. This is only the part of the shadow that can be derived without AI.
5. **Query analysis.** Lemmas, phrases, and parts of speech for English queries, without an AI rewriter.

Each item is a separate zone or feature. We enable it only when measurement shows a gain.

## How to measure

- **The same bench** on the same documents. A new zone must not worsen quote queries.
- **Paraphrasing without AI.** In a quote query, words are changed by our morphology: markets → market, built → build, construction → construct. The correct answer is known in advance, so it shows what lemmas contribute.
- **Negative control.** A lemma zone shuffled across documents must give a worse result. If it does not, the measurement measures nothing.
- **Live bot queries** (`rewrites.tsv`): only 25 so far. Good as examples, too few for statistics.

## Limits

- The developer diaries used for the bench belong to a third party: the measurements on them are internal and not published.
