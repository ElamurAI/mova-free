# Discriminative Reranking for Natural Language Parsing

**Authors:** Michael Collins · **Year:** 2000 · **Venue:** Proceedings of the 17th International Conference on Machine Learning (ICML 2000)
**Link:** unknown for the ICML version; extended journal version: Collins & Koo 2005, Computational Linguistics 31(1), https://aclanthology.org/J05-1003/ (ACL Anthology J05-1003; DOI 10.1162/0891201053630273)
**License of the paper:** unknown (ICML version); the 2005 journal version is CC BY-NC-SA 3.0 (ACL Anthology, before 2016) — https://creativecommons.org/licenses/by-nc-sa/3.0/

## Summary
The work splits parsing into two stages. A base probabilistic parser proposes a short list of candidate trees for each sentence together with their probabilities, and a second, discriminatively trained model reorders the list. Because the second model only scores complete candidates, it can use arbitrary, overlapping features of whole trees without changing the generative model or its decision sequence. Collins trains the reranker with a boosting-style method that minimises an exponential ranking loss and effectively selects useful features one at a time. On the Wall Street Journal treebank, reranking clearly improves over the base parser (the 2005 journal version reports F-measure rising from 88.2% to 89.75%). The approach generalises to any NLP task that can be cast as choosing among candidates.

## How Mova uses it
- `ai/math/src/steps.rs` ("top-K reranking") — the arithmetic word-problem solver's beam-search perceptron produces the top-K distinct final expression trees (`Model::topk`, candidates with tree, score, value and actions), and a separate judge reranks them.
- `ai/math/src/bin/mathsolve.rs` (`--rerank K`) — the judge is trained on candidates produced by models that did not see the corresponding fold (4 folds), so it does not learn to over-trust the base model's training-set answers.
- The judge later became the place where extra evidence is plugged in as features (world-model answer, ensemble votes, shapes), as in the original two-stage design.

## Effectiveness in Mova
Measured on SVAMP (architecture notes, table of ideas from papers): reranking with a judge over top-K raised accuracy from 47,7% to 54,3% — one of the confirmed gains.

---

👨‍🔬💥
