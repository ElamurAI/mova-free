# A Tale of Two Parsers: Investigating and Combining Graph-based and Transition-based Dependency Parsing

**Authors:** Yue Zhang, Stephen Clark · **Year:** 2008 · **Venue:** Proceedings of the 2008 Conference on Empirical Methods in Natural Language Processing (EMNLP)
**Link:** https://aclanthology.org/D08-1059/ (ACL Anthology D08-1059)
**License of the paper:** CC BY-NC-SA 3.0 (ACL Anthology, before 2016) — https://creativecommons.org/licenses/by-nc-sa/3.0/

## Summary
The paper compares graph-based and transition-based dependency parsing within one framework based on beam search and a discriminatively trained perceptron. It shows that a transition-based parser does not have to be greedy: decoding with a beam of partial action sequences, combined with perceptron training with early update (stop and update as soon as the gold sequence falls out of the beam), gives a strong transition-based parser. A graph-based parser is formulated with the same beam-search decoder. Because the two approaches use different kinds of features, the authors combine them into a single model whose score sums both feature sets, and report that the combination improves over each parser alone on English and Chinese.

## How Mova uses it
- `ai/en/src/parse.rs`: the English UD parser is an arc-standard transition system (SHIFT / LEFT(label) / RIGHT(label)) with weight tables trained by an averaged perceptron. Decoding uses a beam of 8 hypotheses, and training uses beam search with early update, as in this paper.
- Only the transition-based half is used; the graph-based parser and the combined model are not implemented.

## Effectiveness in Mova
Measured (beam with early update vs greedy, same features): EWT test LAS 79.00 → 79.77, dev 77.33 → 79.17, PUD 74.48 → 76.33. In the `en` crate summary with Mova's own tags: EWT UAS/LAS 84.2/79.8 with the beam vs 83.4/79.0 greedy.

---

👨‍🔬💥
