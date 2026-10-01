# Discriminative Training Methods for Hidden Markov Models: Theory and Experiments with Perceptron Algorithms

**Authors:** Michael Collins · **Year:** 2002 · **Venue:** Proceedings of the 2002 Conference on Empirical Methods in Natural Language Processing (EMNLP 2002)
**Link:** https://aclanthology.org/W02-1001/ (ACL Anthology W02-1001; DOI 10.3115/1118693.1118694)
**License of the paper:** CC BY-NC-SA 3.0 (ACL Anthology, pre-2016) — https://creativecommons.org/licenses/by-nc-sa/3.0/

## Summary
The paper proposes training sequence-labelling models, such as part-of-speech taggers and base-NP chunkers, with a perceptron instead of maximum-entropy models or CRFs. The classic perceptron is extended from classification to structured outputs: for each training sentence the current weights are used to decode the best tag sequence with Viterbi, and if it is wrong, the features of the correct sequence are rewarded and those of the predicted one penalised. Features are the same as in maximum-entropy taggers, but there is no local probability normalisation. The author adapts the perceptron convergence proof to give mistake bounds for separable data and a bound for the non-separable case. In practice the averaged-parameters variant works best, and it outperforms a maximum-entropy tagger on both tasks. The method is simple, fast and applies to any model with Viterbi-style decoding; it became the basis of the structured perceptron in NLP.

## How Mova uses it
- ai/en/src/ptag.rs: the English PTB part-of-speech tagger is an averaged perceptron in the spirit of this paper, run as a left-to-right greedy/beam tagger (following Honnibal 2013) with hashed integer features and per-tag weights.
- Adaptations: a dictionary hint (mask of tags the built-in lexicon allows for a word form) is just another feature, so the model learns how much to trust it; for unknown words the prior from the older TnT tagger is added to the perceptron score with a weight; a small beam (width 2) with state merging on the last two tags replaces full Viterbi.
- A noise-robust variant stops updating on words that are wrong in every epoch, which also yields a list of suspected gold-label errors.
- The same averaged structured perceptron recipe is reused elsewhere in Mova (word-problem solver, reader models).

## Effectiveness in Mova
Measured on UD English EWT test (recorded in the module header of ai/en/src/ptag.rs): the TnT HMM tagger gives 93.12%; the greedy averaged perceptron 93.74%; plus the TnT prior for unknown words 94.13%; plus neighbouring-word shape features and beam 94.35%. The dependency parser on these tags improved from LAS 77.27 to 79.00. With 5% corrupted training tags, the robust variant raised accuracy from 94.00 to 94.10 and found the corrupted labels with 95.8% precision and 63% recall.

Note: the code cites "Collins 2002"; this refers to the EMNLP 2002 perceptron paper above, not Collins & Duffy 2002 (kernel-based reranking, ACL 2002).

---

👨‍🔬💥
