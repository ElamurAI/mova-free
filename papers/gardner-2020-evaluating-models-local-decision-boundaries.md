# Evaluating Models' Local Decision Boundaries via Contrast Sets

**Authors:** Matt Gardner, Yoav Artzi, Victoria Basmov, Jonathan Berant, Ben Bogin, Sihao Chen, Pradeep Dasigi, Dheeru Dua, Yanai Elazar, Ananth Gottumukkala, Nitish Gupta, Hannaneh Hajishirzi, Gabriel Ilharco, Daniel Khashabi, Kevin Lin, Jiangming Liu, Nelson F. Liu, Phoebe Mulcaire, Qiang Ning, Sameer Singh, Noah A. Smith, Sanjay Subramanian, Reut Tsarfaty, Eric Wallace, Ally Zhang, Ben Zhou · **Year:** 2020 · **Venue:** Findings of the Association for Computational Linguistics: EMNLP 2020
**Link:** https://aclanthology.org/2020.findings-emnlp.117/ (ACL Anthology 2020.findings-emnlp.117; DOI 10.18653/v1/2020.findings-emnlp.117)
**License of the paper:** CC BY 4.0 — https://creativecommons.org/licenses/by/4.0/

## Summary
Standard test sets come from the same distribution as the training data, so a dataset with systematic gaps or annotation artifacts lets models score well with shallow heuristics. The authors propose that dataset creators add a step after building a dataset: take test instances and manually make small, meaningful edits that usually change the gold label. The group of perturbed variants around one original instance is a "contrast set". It probes whether the model's decision boundary is correct in the immediate neighbourhood of real test data. Edits are written without consulting any model, so they are not tuned to one system's weaknesses. Besides accuracy, a consistency metric records whether a model gets every variant of an instance right. Contrast sets were built for ten NLP datasets, among them NLVR2, DROP, QUOREF, MC-TACO, IMDb, MATRES, PERSPECTRUM and UD English parsing. Although not designed to be adversarial, they caused large drops for strong models, in some cases up to about 25%. The approach is a fairly cheap way to check what a model actually learned.

## How Mova uses it
- `ai/world/src/babi.rs` (`CONTRAST` table and `contrast()`; command `world babi-contrast`): the bAbI story reader is tested on contrast variants of the bAbI tasks. These keep the same stories but swap in new names, places and objects, plus synonym verbs. Some synonyms are already in Mova's first-level verb classes, and some (hurried, abandoned) deliberately are not.
- The goal, as stated in the code, is to show that the reader applies a principle rather than a template, and to expose gaps honestly. In Mova the method serves as a robustness test for a rule-based reader, not as an evaluation of a trained neural model.

## Effectiveness in Mova
Measured (from the architecture status table): on the contrast variants the bAbI reader first scored 74.6%. This exposed parser defects ("attic" tagged ADJ, "Taras" lemmatised as "tara", "Following" and "is" read as verbs) and two vocabulary gaps (hurry, abandon). After these were fixed, the contrast set reached 99.7% with all 20 tasks at 95% or higher, and the original test did not drop (99.8%). The method found real bugs that the in-distribution test (already 99.8%) could not show.

---

👨‍🔬💥
