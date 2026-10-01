# Universal Dependencies

**Authors:** Marie-Catherine de Marneffe, Christopher D. Manning, Joakim Nivre, Daniel Zeman · **Year:** 2021 · **Venue:** Computational Linguistics 47(2): 255–308
**Link:** https://aclanthology.org/2021.cl-2.11/ (ACL Anthology 2021.cl-2.11; DOI 10.1162/coli_a_00402)
**License of the paper:** CC BY-NC-ND 4.0 — https://creativecommons.org/licenses/by-nc-nd/4.0/

## Summary
This article sets out the linguistic theory behind Universal Dependencies (UD), which is at once a cross-linguistically consistent morphosyntactic annotation scheme, an open research community and a growing collection of treebanks. The authors argue that UD is not a loose set of practical compromises but a coherent theory grounded in typologically oriented grammatical traditions. The paper presents no new data or models; instead it explains the principles of UD version 2: the word as the basic unit, a dependency tree with typed grammatical relations, three kinds of phrasal units (nominals, clauses, modifiers), and content words as heads with function words attached to them. Many constructions are discussed with examples from diverse languages, with special attention to core arguments in typologically distant languages. The authors show how the design balances linguistic adequacy, simplicity and usability by non-specialists, and conclude that it suits parallel annotation across languages, multilingual parsing and typological research.

## How Mova uses it
- ai/en/seeds/errors/numbered-entities-nummod.md: cited in the history of conventions for numbers after a noun ("page 394", "Route 66"): the article recommended `nmod`, while the current UD guideline uses `flat`; the rule card implements the current convention — `nummod` only for quantities and only before a noun.
- More broadly, its content-head principle (function words as leaves) underlies the UD-based tree conventions Mova's annotator follows; the explicit rule checks cite the UD guidelines directly.

## Effectiveness in Mova
Background reading; not measured separately. The nummod rule it is cited in holds on UD English EWT 2.18 without violations (`nummod` before NOUN/PROPN — 1,433 of 1,433; `nummod` on NUM — 1,794 of 1,794; GUM: 3 and 2 violations).

---

👨‍🔬💥
