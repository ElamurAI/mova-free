# Solving General Arithmetic Word Problems

**Authors:** Subhro Roy, Dan Roth · **Year:** 2015 · **Venue:** Proceedings of the 2015 Conference on Empirical Methods in Natural Language Processing (EMNLP)
**Link:** https://aclanthology.org/D15-1202/ (ACL Anthology D15-1202; DOI 10.18653/v1/D15-1202)
**License of the paper:** CC BY-NC-SA 3.0 (ACL Anthology, before 2016) — https://creativecommons.org/licenses/by-nc-sa/3.0/

## Summary
The paper solves multi-step arithmetic word problems without predefined equation templates by representing the answer as an expression tree in which each number from the text is used at most once. The key result is that for "monotonic" trees (subtraction placed above addition, division above multiplication) the operation at the lowest common ancestor of any pair of numbers is uniquely determined. Building the tree therefore reduces to simple classification: one classifier decides whether each number is relevant, another predicts the LCA operation for each pair (including reversed subtraction and division). A bottom-up beam search then assembles the tree that maximises the sum of these scores, subject to constraints such as a positive, integer answer. Features come from a "quantity schema": the governing verb, its subject, the unit, related noun phrases and whether the quantity is a rate. Later work by the same authors refers to this system as LCA++.

## How Mova uses it
- `ai/math/src/arith.rs` — the core of the arithmetic tree model: a relevance judgement for each number and an operation judgement for each pair of numbers at their lowest common ancestor, both as averaged perceptrons over word features so the deciding words and weights are visible.
- The solver core enumerates expression trees bottom-up with dynamic programming over subsets of numbers and a beam per subset; a tree's score decomposes into the scores of its subtrees plus pairwise operation weights across the split, which keeps the beam search exact with respect to the model. Final answers are recomputed exactly in rational numbers.
- Training uses weak supervision from the gold answer only: for each training problem the smallest tree that yields the answer becomes the "gold" tree for a structured perceptron.

## Effectiveness in Mova
The arith model has been measured as an extra signal for the main SVAMP solver's judge (its answer as a feature, computed fairly across folds): SVAMP 56.3% versus 57.3% without it, i.e. no gain. An earlier record of 58.3% turned out to be a lucky run caused by non-deterministic map ordering in arith (results varied between 55.3% and 58.3%); after making it deterministic the result is a stable 56.3%. The standalone accuracy of the arith model is not reported separately.


---

👨‍🔬💥
