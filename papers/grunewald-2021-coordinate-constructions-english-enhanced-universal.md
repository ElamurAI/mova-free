# Coordinate Constructions in English Enhanced Universal Dependencies: Analysis and Computational Modeling

**Authors:** Stefan Grünewald, Prisca Piccirilli, Annemarie Friedrich · **Year:** 2021 · **Venue:** Proceedings of the 16th Conference of the European Chapter of the Association for Computational Linguistics: Main Volume (EACL 2021)
**Link:** https://aclanthology.org/2021.eacl-main.67/ (ACL Anthology 2021.eacl-main.67; DOI 10.18653/v1/2021.eacl-main.67)
**License of the paper:** CC BY 4.0 — https://creativecommons.org/licenses/by/4.0/

## Summary
In Enhanced UD, coordination is represented by propagating the first conjunct's relations (its head and its dependents) to the other conjuncts. The English enhanced treebanks were produced automatically, by applying the heuristic converter of Schuster and Manning (2016) to manual basic trees, and that converter only propagates core arguments. The authors manually checked and corrected 1,417 EWT sentences with coordinated verbs. They decided by meaning, so they also propagated adjunct dependents such as `obl` where the context required it. During annotation they found systematic converter errors, for example when conjuncts differ in voice or mood, or when several coordinations interact. With the new data they compare rules, SVM classifiers and a new neural model with tree features and RoBERTa, for the first time systematically. On gold trees the data-driven models beat the rules. For automatic parsing, a graph parser that predicts enhanced edges directly clearly beats "tree parser plus converter" pipelines.

## How Mova uses it
- `ai/en/seeds/errors/coordination-structure.md`: cited in the seed on UD v2 coordination (the first conjunct is the head, and `cc` attaches to the following conjunct) for the observation that the Enhanced UD converter fails systematically when conjuncts differ in voice or mood.
- The seed's actual rules, `en.errors.conj-rightward` and `en.errors.cc-before-conjunct`, come from the UD guidelines and validator. This paper only adds a caveat about enhanced-graph conversion, which Mova does not produce yet.

## Effectiveness in Mova
Background reading, a single supporting citation in one seed. Not measured separately.

---

👨‍🔬💥
