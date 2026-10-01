# Mischievous nominal constructions in Universal Dependencies

**Authors:** Nathan Schneider, Amir Zeldes · **Year:** 2021 · **Venue:** Proceedings of the Fifth Workshop on Universal Dependencies (UDW, SyntaxFest 2021)
**Link:** https://aclanthology.org/2021.udw-1.14/ (ACL Anthology 2021.udw-1.14)
**License of the paper:** CC BY 4.0 (ACL Anthology) — https://creativecommons.org/licenses/by/4.0/

## Summary
The paper looks at noun phrases that do not fit the usual UD picture: descriptors before names (titles such as Mr., role labels like "French actor"), company names with suffixes, numbered designations, dates, times and similar constructions. UD guidelines had no consistent treatment for them, so annotation diverged even within English. Using the English treebanks EWT, GUM and GUMReddit, the authors compare candidate analyses by syntactic tests: whether an element can be omitted, whether it agrees in number, whether it takes its own determiner. They argue for taking pre-name descriptors out of the headless `flat` structure and attaching them as modifiers, discussing subtypes such as `compound:title`, `appos:title`, `nmod:title` and a broader `nmod:desc`. No new universal relations are introduced, so the proposals are local and compatible with the existing scheme. The issues they describe carry over to other languages with similar name and date constructions.

## How Mova uses it
- `ai/en/seeds/errors/titles-and-company-suffixes.md`: rule `en.errors.title-desc` for the English grammar expert system (`ai/en/src/expert.rs`) — titles before names and company suffixes (Inc., Corp., Ltd.) should be `nmod:desc`; the paper supplies the analysis and frequency estimates.
- `ai/en/seeds/errors/flat-compound-direction.md`: `flat` is head-first, `compound` head-last.
- `ai/en/seeds/errors/numbered-entities-nummod.md`: `nummod` only for quantities before a noun; numbers after a noun (page 394, Route 66) are identifiers, not `nummod`.
- `ai/en/seeds/errors/english-treebank-conventions.md`: background for treating divergences between EWT and GUM as convention differences rather than rule errors.

## Effectiveness in Mova
Not measured separately as an accuracy gain. The rules derived from it are checked against the reference treebanks: titles before PROPN in EWT 2.18 — 128 cases, 0 violations; company suffixes 48/0; GUM 84/1 and 3/0. `flat` head-first in EWT — 2501 of 2501; noun–noun `compound` head-last 8416 with 16 violations (GUM 6713/36). `nummod` before NOUN/PROPN in EWT — 1433 of 1433. Its role is to supply the linguistic analysis behind several annotation-checking rules.

---

👨‍🔬💥
