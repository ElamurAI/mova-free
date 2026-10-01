# Bagging Predictors

**Authors:** Leo Breiman · **Year:** 1996 · **Venue:** Machine Learning 24(2)
**Link:** https://doi.org/10.1007/BF00058655 (DOI 10.1007/BF00058655)
**License of the paper:** unknown (publisher copyright)

## Summary
Bagging ("bootstrap aggregating") builds several versions of a predictor by training each on a bootstrap resample of the training set and then combines them, by voting for classification or averaging for regression. The paper argues that the gain comes from reducing variance: learners whose output changes a lot with small perturbations of the data, such as decision trees or subset selection in regression, benefit most, whereas stable learners such as nearest neighbours gain little or nothing. Experiments on real and simulated datasets show substantial error reductions for unstable methods. The idea became a foundation of later ensemble methods such as random forests.

## How Mova uses it
- `ai/math/src/bin/mathsolve.rs` (`--bag`) — for the arithmetic word-problem solver, two extra beam-search perceptron models are trained with reversed and with shuffled training order (instead of bootstrap resamples), and their top-K candidate solutions are pooled for the reranking judge.
- `--bag-judge` — a variant where the judge is trained on the merged candidates of the three models with a "votes" feature (how many models proposed the answer).

## Effectiveness in Mova
Measured on SVAMP on the idea test bench (architecture notes, table of ideas from papers):
- `--bag`: the correct answer appears among the candidates more often (258 → 265), but accuracy is unchanged (57,3%).
- `--bag-judge`: 57,3% → 56,0%, worse. Models with the same features make the same mistakes, so agreement says little; an ensemble needs genuinely different models (tree, world model, DAG), not one perceptron trained in different orders. Not adopted by default.

---

👨‍🔬💥
