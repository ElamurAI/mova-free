# Classifying Syntactic Errors in Learner Language

**Authors:** Leshem Choshen, Dmitry Nikolaev, Yevgeni Berzak, Omri Abend · **Year:** 2020 · **Venue:** Proceedings of the 24th Conference on Computational Natural Language Learning (CoNLL 2020)
**Link:** https://aclanthology.org/2020.conll-1.7/ (ACL Anthology 2020.conll-1.7; DOI 10.18653/v1/2020.conll-1.7)
**License of the paper:** CC BY 4.0 — https://creativecommons.org/licenses/by/4.0/

## Summary
The authors present SERCL, an automatic taxonomy and classifier for syntactic errors in learner language, where a syntactic error is one whose correction changes the part of speech, dependency relation or morphological features involved. For each erroneous span and its correction, the method picks the node closest to the root of the dependency tree, and the error type is the ordered pair of the two nodes' labels. Because labels come from Universal Dependencies, no hand-built category set is needed and the approach carries over to other languages. Reliability is checked by comparing results on automatic versus manual parses of a learner treebank and by relating the types to NUCLE's manual categories and to ERRANT. The paper describes syntactic error distributions in learner English and learner Russian and shows that leading GEC systems correct syntactic errors less well than other errors. It offers a multilingual, fine-grained error analysis compatible with standard NLP tooling.

## How Mova uses it
- ai/en/seeds/errors/errant-taxonomy.md: cited as the complement to ERRANT — SERCL classifies syntactically a large share of what ERRANT leaves as OTHER, and supports Mova's view that error types with a trace in the UD tree can be detected by tree rules.
- It is background reading for the error-rule section; no SERCL classifier is implemented.

## Effectiveness in Mova
Background reading only; not measured separately.

---

👨‍🔬💥
