# Simple Coreference Resolution with Rich Syntactic and Semantic Features

**Authors:** Aria Haghighi, Dan Klein · **Year:** 2009 · **Venue:** Proceedings of the 2009 Conference on Empirical Methods in Natural Language Processing (EMNLP 2009)
**Link:** https://aclanthology.org/D09-1120/ (ACL Anthology D09-1120)
**License of the paper:** CC BY-NC-SA 3.0 — https://creativecommons.org/licenses/by-nc-sa/3.0/

## Summary
The paper shows that a deliberately simple, deterministic coreference system can compete with much more complex learned models if it uses good syntactic and semantic information. Each mention is linked to an antecedent chosen with a small set of modular components rather than a trained pairwise classifier. On the syntactic side, the system uses constraints read off the parse tree, such as appositives and predicate nominatives, and ranks candidate antecedents by their position in the tree instead of by flat surface distance. On the semantic side, it filters candidate pairs for compatibility using head-word relations mined in an unsupervised way from a large unlabeled corpus. The authors report that this simple system was competitive with, and in places better than, the state-of-the-art supervised and unsupervised systems of the time on standard coreference benchmarks. The paper was influential in showing that much of the work in coreference is done by syntax and lexical semantics rather than by elaborate learning.

## How Mova uses it
- the development notes of `ai/coref` and `ai/coref/src/sieve.rs`: Mova's coreference module is a deterministic multi-pass sieve (architecture after Raghunathan et al. 2010 and Lee et al. 2013). From this paper it takes the principle that candidates higher in the parse tree come first. Candidate antecedents are sorted by the depth of their head (root = 0), then by position, in the Hobbs-style traversal order.
- Tree depth is precomputed per sentence (`ai/coref/src/doc.rs`), so the ordering is an integer sort, which keeps every linking decision explainable.

## Effectiveness in Mova
One ordering principle inside the coreference sieve. Not measured separately; there is no ablation of the depth ordering on its own.

---

👨‍🔬💥
