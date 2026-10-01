# C4.5: Programs for Machine Learning

**Authors:** J. Ross Quinlan · **Year:** 1993 · **Venue:** Morgan Kaufmann (book)
**Link:** unknown
**License of the paper:** unknown (commercially published book, not openly licensed)

## Summary
The book presents C4.5, a widely used algorithm and program for learning decision trees and rule sets from labelled examples. Trees are grown top-down by repeatedly choosing the attribute that best separates the classes. Because plain information gain favours attributes with many values, the book uses the gain ratio: information gain divided by the split information (the entropy of the attribute's value distribution). It also covers handling of continuous attributes, missing values, pruning to avoid overfitting, and conversion of trees into production rules. The accompanying source code made C4.5 a reference baseline in machine learning for many years.

## How Mova uses it
- `ai/prag/src/model.rs` — the function `gain_ratio` computes (H(Y) − Σ p(v) H(Y|v)) / SplitInfo for each feature slot, following this book (and the same weighting as in the TiMBL memory-based learning package).
- Gain ratio orders feature slots in IGTree (most informative slot at the root) and weights slots in the overlap distance of the k-NN (IB1) classifier.
- Only the gain-ratio criterion is used; C4.5's tree growing, pruning and rule extraction are not implemented.

## Effectiveness in Mova
Not measured separately (no comparison of gain ratio against other slot weightings). The classifiers that depend on it are measured in the `prag` README, e.g. five-fold cross-validation on 2,986 sentences, accuracy / macro-F1 %: speech act — IGTree 64.1 / 23.9, k-NN 68.1 / 26.0 (perceptron 70.3 / 38.0, baseline 28.9 / 3.9); hedge — IGTree 98.0 / 75.5, k-NN 97.8 / 73.4. IGTree is the smallest and fastest model (72 KB, 0.008 ms per sentence).

---

👨‍🔬💥
