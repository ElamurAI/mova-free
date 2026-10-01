# Algorithms for Scoring Coreference Chains

**Authors:** Amit Bagga, Breck Baldwin · **Year:** 1998 · **Venue:** Proceedings of the LREC 1998 Workshop on Linguistic Coreference
**Link:** unknown
**License of the paper:** unknown

## Summary
The paper proposes the B-cubed (B³) metric for evaluating coreference resolution. Instead of counting links between mentions, as the earlier MUC score does, B³ computes precision and recall for every individual mention: how much of the predicted cluster containing that mention overlaps with its gold cluster, and vice versa. The per-mention scores are then averaged, optionally with weights. This mention-centric view fixes known blind spots of link-based scoring, for example that merging everything into one large cluster is not punished enough and that singleton entities are ignored. B³ became one of the three components of the standard CoNLL coreference score, together with MUC and CEAF.

## How Mova uses it
- `ai/coref/src/score.rs` — the coreference scorer implements B³ alongside MUC, CEAF-e and CEAF-m; the CoNLL F1 reported for Mova's coreference resolver is the mean of MUC, B³ and CEAF-e.
- Formulas for predicted (non-gold) mentions follow the later reference treatment by Pradhan et al. 2014; mentions are matched by syntactic head, as in the CRAC shared tasks, with and without singletons.
- The scorer is self-checking: before every evaluation it reproduces the worked example from Pradhan et al. 2014 (B³ 35/84 and 0.50) and the "gold against itself = 100" check, failing fast otherwise; negative controls (one giant cluster, everything separate) must score low.

## Effectiveness in Mova
This is an evaluation metric, not a modelling idea, so it has no gain of its own. It is the measuring instrument for all coreference results, e.g. the sieve resolver on GUM test with gold trees: CoNLL F1 73.04 with singletons (MUC / B³ / CEAF-e = 69.18 / 75.26 / 74.68), 60.31 without singletons (B³ 58.79), per the development notes of `ai/coref`.

---

👨‍🔬💥
