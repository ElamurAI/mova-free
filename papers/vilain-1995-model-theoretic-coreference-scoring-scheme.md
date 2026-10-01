# A Model-Theoretic Coreference Scoring Scheme

**Authors:** Marc Vilain, John Burger, John Aberdeen, Dennis Connolly, Lynette Hirschman · **Year:** 1995 · **Venue:** Sixth Message Understanding Conference (MUC-6), Columbia, Maryland
**Link:** https://aclanthology.org/M95-1005/ (ACL Anthology M95-1005)
**License of the paper:** CC BY-NC-SA 3.0 (ACL Anthology, before 2016) — https://creativecommons.org/licenses/by-nc-sa/3.0/

## Summary
The paper defines the scoring metric used for the coreference task of MUC-6, now known as the MUC metric. Coreference chains are viewed as equivalence classes, and scoring counts links rather than individual pairwise decisions. Recall for a key chain is the number of links needed to connect it minus the number of pieces the response splits it into, normalized by the minimal number of links; precision is computed symmetrically with key and response swapped. This makes the score independent of which particular links a system outputs, as long as the resulting partition is the same. The metric is simple and intuitive, but it is known to ignore singleton entities and to reward over-merging, which later metrics (B³, CEAF) were designed to address.

## How Mova uses it
- `ai/coref/src/score.rs`: the MUC metric is implemented in Rust as one of the coreference scores, alongside B³, CEAF-e/CEAF-m, and CoNLL F1 (the average of MUC, B³ and CEAF-e).
- Mentions are matched by head word (as in the CRAC shared tasks), with and without singletons.
- The scorer is checked against the worked example of Pradhan et al. 2014 (MUC 0.40 / 0.40) and by negative controls: one giant cluster gives MUC recall 1 but CoNLL < 0.7; all-singletons gives MUC 0.

## Effectiveness in Mova
It is an evaluation metric, not a modelling idea, so there is no gain to measure. It is a component of the reported results, e.g. on GUM test (UD 2.18, gold trees, with singletons) the default sieve system scores CoNLL F1 73.04 with MUC F1 69.18, B³ 75.26, CEAF-e 74.68.

---

👨‍🔬💥
