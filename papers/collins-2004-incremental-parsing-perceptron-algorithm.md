# Incremental Parsing with the Perceptron Algorithm

**Authors:** Michael Collins, Brian Roark · **Year:** 2004 · **Venue:** Proceedings of the 42nd Annual Meeting of the ACL (ACL-04)
**Link:** https://aclanthology.org/P04-1015/ (ACL Anthology P04-1015; DOI 10.3115/1218955.1218970)
**License of the paper:** CC BY-NC-SA 3.0 (ACL Anthology, pre-2016) — https://creativecommons.org/licenses/by-nc-sa/3.0/

## Summary
The paper asks whether an unnormalised discriminative model can work well together with heuristic search in parsing. Instead of reranking candidates from another parser, the authors train the parameters of an incremental left-to-right parser directly with a perceptron variant, using the same beam search during training and decoding. The key device is early update: as soon as the correct partial analysis falls out of the beam, decoding stops and the weights are updated against the best item at that point, so the model is not penalised for errors that search alone would cause later. Features are borrowed from Roark's generative incremental parser, which makes the two approaches directly comparable, and some training refinements and punctuation features are added. On the Penn Treebank the perceptron alone matches the generative model, and a hybrid that adds the generative model's log-probability as a feature improves F-measure further. The approach offers a simple way to add arbitrary features to a parser without exhaustive candidate enumeration or expensive dynamic programming.

## How Mova uses it
- ai/math/src/steps.rs: word problems are solved as a transition system — the model walks through the numbers in text order and at each step chooses SHIFT (take the number), SKIP (ignore it) or REDUCE (combine the top two with an operation and direction), mirroring Mova's arc-standard dependency parser; an exact arithmetic core computes each subproblem.
- Training is a structured perceptron with a beam and early update exactly in the sense of this paper: the gold action sequence comes from the gold expression tree, and when the gold prefix leaves the beam the update is made at that step against the best state there. Parameters are averaged.
- Features come from the UD parse of the problem (units, verbs, subjects, cue words, question words) rather than lexical bags.

## Effectiveness in Mova
Early update is the default training scheme of the solver and is not ablated on its own. The alternative max-violation update (Huang, Fayong, Guo 2012; `MATH_MAXVIOL=1`) was measured against it: SVAMP 57.3% → 56.0%, i.e. worse, so early update was kept.

---

👨‍🔬💥
