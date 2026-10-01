# On Coreference Resolution Performance Metrics

**Authors:** Xiaoqiang Luo · **Year:** 2005 · **Venue:** Proceedings of Human Language Technology Conference and Conference on Empirical Methods in Natural Language Processing (HLT/EMNLP 2005)
**Link:** https://aclanthology.org/H05-1004/ (ACL Anthology H05-1004)
**License of the paper:** CC BY-NC-SA 3.0 (ACL Anthology, pre-2016) — https://creativecommons.org/licenses/by-nc-sa/3.0/

## Summary
The paper criticises existing coreference metrics, notably MUC (which ignores singletons and rewards over-merging) and the ACE value, and proposes CEAF (Constrained Entity-Alignment F-measure). CEAF finds the best one-to-one alignment between key and response entities, computed with the Kuhn-Munkres (Hungarian) algorithm, under a chosen entity-similarity function. Recall and precision are the total similarity of the alignment normalised by the self-similarity of the key and of the response respectively. Two similarity functions give the mention-based variant (number of shared mentions) and the entity-based variant (a Dice-like overlap per entity pair). The paper shows that CEAF behaves more sensibly than MUC on degenerate outputs such as putting all mentions in one cluster. CEAF later became one of the three components of the CoNLL coreference score.

## How Mova uses it
- `ai/coref/src/score.rs` — Mova's own Rust scorer implements CEAF-e (φ4, entity-based) and CEAF-m (φ3, mention-based), with the optimal entity alignment found by Kuhn-Munkres, computed separately per connected component of the overlap graph.
- CEAF-e is one of the three terms of the CoNLL F1 reported for the coref crate (average of MUC, B³ and CEAF-e).
- Unit tests check hand-computed CEAF values and the degenerate cases (one giant cluster, all singletons).

## Effectiveness in Mova
It is an evaluation metric, not a modelling idea, so it has no "gain" of its own. It is part of every coref measurement, e.g. the default system's CEAF-e F1 on GUM test is 74.68 with singletons and 52.89 without on gold trees (the development notes of `ai/coref`).

---

👨‍🔬💥
