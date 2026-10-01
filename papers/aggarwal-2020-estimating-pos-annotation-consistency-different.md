# Estimating POS Annotation Consistency of Different Treebanks in a Language

**Authors:** Akshay Aggarwal, Daniel Zeman · **Year:** 2020 · **Venue:** Proceedings of the 19th International Workshop on Treebanks and Linguistic Theories (TLT 2020)
**Link:** https://aclanthology.org/2020.tlt-1.9/ (ACL Anthology 2020.tlt-1.9; DOI 10.18653/v1/2020.tlt-1.9)
**License of the paper:** CC BY 4.0 (ACL Anthology) — https://creativecommons.org/licenses/by/4.0/

## Summary
The paper asks whether several treebanks of the same language, all nominally following the Universal Dependencies guidelines, actually tag parts of speech in the same way. It builds on an asymmetric KL-divergence measure over UPOS trigram distributions (KLcpos3, from Rosa and Žabokrtský 2015), computes it in both directions and adds the two to get a symmetric score, θpos. To make the score interpretable, the authors derive thresholds empirically, separating the effect of data size (cross-validation on Czech-PDT and Estonian-EDT news text) from the effect of genre mix. Roughly, θpos at or below 0.5 indicates consistent annotation and θpos at or above 4.0 inconsistent annotation, with intermediate thresholds depending on whether genres differ. They apply the test to treebank pairs in UD 2.5 and show it can localise a problem, for example to the grammar-examples section of a Finnish treebank. The method does not depend on the quality of any tagger or parser, unlike cross-training approaches, but it only covers POS tags, not syntax.

## How Mova uses it
- Summarised as one of five methods for detecting annotation errors automatically in the seed ai/en/seeds/errors/consistency-detection-methods.md, which guides how the grammar expert system (ai/en/src/expert.rs) and its rules are interpreted.
- The idea adopted is the principle that treebank inconsistency leaves statistical traces that can be detected without a trained model; the θpos measure itself is not implemented in code.

## Effectiveness in Mova
Background reading only; not implemented and not measured separately.

---

👨‍🔬💥
