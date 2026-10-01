# Applying Occam's Razor to Transformer-Based Dependency Parsing: What Works, What Doesn't, and What is Really Necessary

**Authors:** Stefan Grünewald, Annemarie Friedrich, Jonas Kuhn · **Year:** 2021 · **Venue:** Proceedings of the 17th International Conference on Parsing Technologies and the IWPT 2021 Shared Task on Parsing into Enhanced Universal Dependencies (IWPT 2021)
**Link:** https://aclanthology.org/2021.iwpt-1.13/ (ACL Anthology 2021.iwpt-1.13; DOI 10.18653/v1/2021.iwpt-1.13)
**License of the paper:** CC BY 4.0 — https://creativecommons.org/licenses/by/4.0/

## Summary
Modern graph-based dependency parsers for Universal Dependencies rely on transformer embeddings but differ in several design choices: which pretrained model to use, whether to add LSTM layers, and whether to train jointly on POS and morphology. Following Occam's razor, the authors ask which of these components are actually needed. They build STEPS, a modular graph-based parser with biaffine classifiers, and run systematic experiments on UD treebanks of twelve typologically diverse languages. They compare mBERT, language-specific BERTs and XLM-R, with and without a BiLSTM, and with and without multi-task training. The choice of pretrained model matters most, and XLM-R is the most reliable. The LSTM adds nothing, and multi-task training can hurt and can distort comparisons between systems. A simple setup, fine-tuned XLM-R with factorised arc and label scoring, sets a new LAS state of the art for most languages and carries over well to Enhanced UD. The practical lesson is that architectural add-ons are often unnecessary; what matters is a strong language model that covers the target language and domain.

## How Mova uses it
- the development notes of `ai/en`: the STEPS (XLM-R) result on English-EWT test, 91.91 LAS as reported in the paper, is one of the reference points in the benchmark table that Mova's own tagger and parser are measured against.
- Mova does not reuse STEPS's architecture: it has no transformer and no neural parser. The paper serves as an external yardstick for what a strong neural parser reaches on the same test set.

## Effectiveness in Mova
Used only as a benchmark reference, so there is nothing to measure for the paper itself. The comparison as recorded in the development notes of `ai/en` (EWT test, gold tokens): STEPS (XLM-R) 91.91 LAS; UDPipe 2 (EWT 2.17 model, Mova's own measurement) 92.37 LAS; Mova small model plus large-model edits 91.90 LAS on 300 random sentences, where UDPipe scores 92.11; Mova's small model alone 80.24 LAS, and 81.15 LAS when also trained on GUM converted to Mova's annotation dialect.

---

👨‍🔬💥
