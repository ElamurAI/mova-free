# Scoring Coreference Partitions of Predicted Mentions: A Reference Implementation

**Authors:** Sameer Pradhan, Xiaoqiang Luo, Marta Recasens, Eduard Hovy, Vincent Ng, Michael Strube · **Year:** 2014 · **Venue:** Proceedings of the 52nd Annual Meeting of the Association for Computational Linguistics (Volume 2: Short Papers)
**Link:** https://aclanthology.org/P14-2006/ (ACL Anthology P14-2006; DOI 10.3115/v1/P14-2006)
**License of the paper:** CC BY-NC-SA 3.0 (ACL Anthology, before 2016) — https://creativecommons.org/licenses/by-nc-sa/3.0/

## Summary
Coreference metrics such as MUC, B-cubed and CEAF were originally defined for gold mentions, and different research groups adapted them to predicted mentions in incompatible ways, often by adding or removing mentions artificially so that the key and response partitions line up. This made published scores hard to compare. The paper states how each metric should be computed directly on partitions with predicted mentions, without such manipulations: a mention present in only one partition simply contributes nothing where it has no counterpart. It walks through worked examples and describes a reference implementation of the scorer that became the standard tool for CoNLL-style evaluation. The aim is a single, transparent definition so that results across systems and papers are comparable.

## How Mova uses it
- `ai/coref/src/score.rs` — the MUC, B-cubed, CEAF-e and CEAF-m formulas for predicted mentions follow this paper: no mention manipulation, unmatched mentions just do not contribute; CoNLL F1 is the mean of MUC, B-cubed and CEAF-e, and multi-document totals sum numerators and denominators.
- The worked example from the paper's Section 4 (key {a,b,c}{d,e,f,g}, response {a,b}{c,d}{f,g,h,i}) is a unit test of the Rust scorer, checking MUC, B-cubed, CEAF-e and CEAF-m values. The test documents that the paper's printed B-cubed F1 of 0.46 comes from rounded precision and recall; the exact value is 0.4545.

## Effectiveness in Mova
Not an idea that changes scores, so not measured as a gain. Its role is correctness of the measuring instrument: the scorer reproduces the paper's worked example, and every reported coreference result (for example CoNLL F1 73.04 with singletons and 60.31 without on GUM test, gold trees) depends on it.

---

👨‍🔬💥
