# The BEA-2019 Shared Task on Grammatical Error Correction

**Authors:** Christopher Bryant, Mariano Felice, Øistein E. Andersen, Ted Briscoe · **Year:** 2019 · **Venue:** Proceedings of the Fourteenth Workshop on Innovative Use of NLP for Building Educational Applications (BEA)
**Link:** https://aclanthology.org/W19-4406/ (ACL Anthology W19-4406; DOI 10.18653/v1/W19-4406)
**License of the paper:** CC BY 4.0 — https://creativecommons.org/licenses/by/4.0/

## Summary
The paper reports on the BEA-2019 shared task on grammatical error correction, organised to restore comparable evaluation after five years in which systems had been reported on different corpora and metrics. A new corpus, Write & Improve + LOCNESS, was released, covering learner texts across proficiency levels together with native-speaker essays; its test portion was annotated independently several times. Existing corpora (FCE, Lang-8, NUCLE) were converted to a common format and re-typed with ERRANT, which for the first time allowed their error distributions to be compared directly. Three tracks — restricted, unrestricted and low-resource — controlled how much annotated data participants could use, and systems were scored with ERRANT F0.5 on a public leaderboard. Top systems mostly combined Transformer-based sequence-to-sequence models with synthetic training data and clearly surpassed the 2014 state of the art. The low-resource track showed that strong systems can be built with little hand-annotated data.

## How Mova uses it
- ai/en/seeds/errors/errant-taxonomy.md: the table of the most frequent edit types in W&I+LOCNESS, FCE and NUCLE comes from this paper, and is used to prioritise which error types get deterministic rules; its finding that GEC systems struggle most with content words is noted there.
- Error-type frequencies are cited as justification in individual rule cards: missing article / DET share of edits (ai/en/seeds/errors/missing-article.md), determiner–noun number (determiner-noun-number.md), word order (adverb-placement.md), ADJ:FORM (double-comparative.md) and `a`/`an` (a-an-choice.md).

## Effectiveness in Mova
Not measured separately. The paper serves as background statistics for choosing and prioritising error rules; the rules themselves are checked on gold treebanks (e.g. missing article on UD English EWT 2.18: 953 hits / 42 flags; double comparative: 2 hits, both real text errors).

---

👨‍🔬💥
