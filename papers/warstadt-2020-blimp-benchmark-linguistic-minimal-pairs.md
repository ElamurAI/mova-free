# BLiMP: The Benchmark of Linguistic Minimal Pairs for English

**Authors:** Alex Warstadt, Alicia Parrish, Haokun Liu, Anhad Mohananey, Wei Peng, Sheng-Fu Wang, Samuel R. Bowman · **Year:** 2020 · **Venue:** Transactions of the Association for Computational Linguistics (TACL), vol. 8
**Link:** https://aclanthology.org/2020.tacl-1.25/ (ACL Anthology 2020.tacl-1.25; DOI 10.1162/tacl_a_00321)
**License of the paper:** CC BY 4.0 (ACL Anthology) — https://creativecommons.org/licenses/by/4.0/

## Summary
BLiMP is a challenge set for measuring which parts of English grammar language models have learned. It contains 67 paradigms of 1,000 sentence pairs each; within a pair the two sentences differ minimally and only one is acceptable. The pairs are generated automatically from templates designed by linguists and cover syntax, morphology and semantics in 12 phenomenon categories, including subject–verb agreement, determiner–noun agreement, argument structure, binding, filler–gap dependencies, island effects, negative polarity items and quantifiers. Evaluation needs no fine-tuning: a model is counted correct when it assigns higher probability to the acceptable sentence. Crowdworkers agreed with the generated labels about 96% of the time. Among the tested models (n-gram, LSTM, Transformer-XL, GPT-2) agreement phenomena were handled well, while islands, NPI licensing and quantifiers remained hard, and the best model still trailed humans by several points.

## How Mova uses it
- `ai/en/seeds/errors/attraction-of-phrase.md` — the rules `en.errors.attraction-of` and `en.errors.attraction-of-auxcop` flag agreement attraction through an *of*-phrase (*A pattern of arrests indicate…*), excluding quantity nouns such as *lot*, *number*, *majority*; BLiMP's attractor paradigms (relational nouns, relative clauses) are cited as motivation alongside Fowler 1926 and later work on attraction.
- About a dozen other seeds in `ai/en/seeds/errors/` and `ai/en/seeds/nominal/` (determiner–noun number, relative-clause agreement, double negation, overregularised verbs and others) cite BLiMP for the phenomenon definitions and minimal-pair examples.
- The benchmark's phenomenon inventory is used as a checklist for hand-written, explainable error rules over UD trees; BLiMP is not run as a probability-based benchmark.

## Effectiveness in Mova
Not measured separately; Mova does not report a BLiMP score. The derived rules are checked against gold treebanks for false alarms (the attraction seed records its hit counts on EWT and GUM), but no accuracy figure attributable to BLiMP is available.

---

👨‍🔬💥
