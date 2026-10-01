# Are UD Treebanks Getting More Consistent? A Report Card for English UD

**Authors:** Amir Zeldes, Nathan Schneider · **Year:** 2023 · **Venue:** Proceedings of the Sixth Workshop on Universal Dependencies (UDW, GURT/SyntaxFest 2023)
**Link:** https://aclanthology.org/2023.udw-1.7/ (ACL Anthology 2023.udw-1.7)
**License of the paper:** CC BY 4.0 (ACL Anthology) — https://creativecommons.org/licenses/by/4.0/

## Summary
The paper checks whether the two largest English UD treebanks, EWT and GUM, which have different origins and conversion histories, are converging, and whether it makes sense to train joint models on them. First, each pair of consecutive releases from v2.6 to v2.11 is compared with the official CoNLL 2018 scorer, treating the newer release as gold, to locate the changes in tokenization, tags, lemmas and dependencies that caused the largest shifts. Then a parser is trained on every version of each corpus and on both combined, and tested across corpora. Consistency grows with each release both within and between the treebanks, and v2.11 is best on all metrics. Joint training still lags slightly behind in-domain training, but the gap has become very small, so a combined model is reasonable for new genres. The method applies to any language with several treebanks.

## How Mova uses it
Background for the English grammar expert system (`ai/en/src/expert.rs`), whose rules live in `ai/en/seeds/errors/`:
- `english-treebank-conventions.md`: divergences between EWT and GUM are treated as conventions, not rule errors; a much higher violation count on GUM than on EWT signals a convention difference.
- `adjectives-in-names.md` (rule `en.errors.adj-not-compound`): since v2.8, adjectives in proper names are ADJ with `amod`, not `compound`.
- `dep-last-resort.md` (rule `en.errors.dep-last-resort`): `dep` should be rare, following the v2.11 clean-up of `dep` uses.
- `advcl-on-noun.md`, `flat-compound-direction.md`: supporting examples from the paper's appendix.
- `consistency-detection-methods.md`: one of several sources on detecting annotation errors automatically (release-to-release comparison).

## Effectiveness in Mova
Not measured separately. Background reading behind several annotation-checking rules. Rule statistics on the reference treebanks: ADJ as `compound` of a noun — EWT 2.18: 14, GUM: 57; `dep` — EWT 2.18: 5, GUM 2.18: 184; `advcl` under a noun — EWT 2.18: 201 hits, 15 violations.

---

👨‍🔬💥
