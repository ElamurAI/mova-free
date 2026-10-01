# Learning to Reason Deductively: Math Word Problem Solving as Complex Relation Extraction

**Authors:** Zhanming Jie, Jierui Li, Wei Lu · **Year:** 2022 · **Venue:** Proceedings of the 60th Annual Meeting of the Association for Computational Linguistics (Volume 1: Long Papers), ACL 2022
**Link:** https://aclanthology.org/2022.acl-long.410/ (ACL Anthology 2022.acl-long.410; DOI 10.18653/v1/2022.acl-long.410)
**License of the paper:** CC BY 4.0 — https://creativecommons.org/licenses/by/4.0/

## Summary
The paper treats solving a math word problem as multi-step relation extraction between quantities, instead of generating an expression tree top-down as seq2tree and graph2tree models do. At each step the model, DeductReasoner, picks a pair of known quantities and an operation (including reversed subtraction and division), and adds the result to the pool of quantities. A result can be used again, so shared subexpressions are not duplicated. Quantity representations come from a pretrained encoder (BERT/RoBERTa). A pair is scored from the concatenation and element-wise product of its two vectors, with a separate scorer per operation and a binary terminator that decides when to stop. After each step a GRU-based "rationalizer" updates all quantity representations so the model does not repeat the same first step. On MAWPS, Math23k, MathQA and SVAMP it outperforms strong tree-generation baselines, especially on multi-step problems. Every step is an explicit binary operation over known values, which makes the reasoning chain inspectable.

## How Mova uses it
- `ai/math/src/deduct.rs` (math solver v5, `mathsolve steps --dag`): a transparent reimplementation of the DeductReasoner idea as a graph of subproblems (DAG). The pool holds the numbers from the text plus all intermediate results, nothing is ever removed, and each step combines two pool items with an operation. Stopping means the last created value is the answer, and the exact arithmetic core computes every step.
- Adaptation: the neural encoder, rationalizer and terminator are replaced by an averaged structured perceptron with beam search and early update. It uses the same non-lexical features as the tree solver (`ai/math/src/steps.rs`: verb classes, rates, units, subject and question words), computed for each pooled value through a representative leaf. Weights are a dense feature-hashed array so that scanning all pool pairs stays fast. A dynamic oracle supplies training targets.

## Effectiveness in Mova
Measured in the commit that introduced the module: SVAMP 49.7% with the reranking judge (45.7% without), and GSM8K 7.0%, both with canonical step order and with the dynamic oracle. The model did not learn long problems: the space of pool pairs is too large for the beam. The tree-based solver therefore stayed the working default, and the DAG solver is kept as an experiment. For reference, project notes list the neural DeductReasoner at 47.3 on SVAMP.

---

👨‍🔬💥
