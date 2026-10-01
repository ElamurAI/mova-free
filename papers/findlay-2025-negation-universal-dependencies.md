# Negation in Universal Dependencies

**Authors:** Jamie Yates Findlay, Dag Trygve Truslew Haug · **Year:** 2025 · **Venue:** Proceedings of the Eighth Workshop on Universal Dependencies (UDW, SyntaxFest 2025)
**Link:** https://aclanthology.org/2025.udw-1.8/ (ACL Anthology 2025.udw-1.8)
**License of the paper:** CC BY 4.0 — https://creativecommons.org/licenses/by/4.0/

## Summary
The authors study how negation is encoded in Universal Dependencies, with semantic parsing from UD trees as the goal. A survey of treebanks shows that the two existing negation features, Polarity=Neg and PronType=Neg, are applied inconsistently across treebanks and even within one language, and often against the guidelines. In UD 2.15, Polarity=Neg appears in only 224 of 296 treebanks; English-EWT did not mark clausal negation at all. The authors argue that perfect consistency would still not solve the problem: these features sit on individual words and describe form, so the meaning of negation cannot be composed from them easily. They propose marking the predicate itself with two new features, Negated=+ and DoubleNegated=+, which say directly that the predicate is semantically under negation. For English they provide GREW rewrite rules that add these features to existing UD annotation automatically, covering plain clausal negation, negation affecting adjuncts, and negative quantifiers. The proposal fits the UniDive morphosyntactic parsing shared task, where function words pass their features up to their heads.

## How Mova uses it
- `ai/en/seeds/errors/negation-not.md`: grounds the error rule `en.errors.not-part` (not/n't must be PART, XPOS RB, Polarity=Neg; the old `neg` relation no longer exists). The paper's observation that EWT lacked Polarity=Neg in UD 2.15 is set against Mova's own check that EWT 2.18 has it on all 2077 tokens. The seed reads this as a recent realignment that older models may not reflect.
- `ai/en/seeds/errors/double-negation.md`: cited for English being a language without negative concord (unlike Spanish). This backs two warning rules, `en.errors.double-negation-pronoun` ("don't know nothing") and `en.errors.double-negation-det` ("didn't take no reason"). Both are written as tree patterns over the Polarity=Neg and PronType=Neg features.
- The proposed predicate-level Negated=+ feature is noted in the seed as background; Mova does not implement it.

## Effectiveness in Mova
Not measured separately. The paper is one of two or three sources behind three small rules in the English error-detection rule set. Gold-data checks reported in the seeds: `en.errors.not-part` on EWT 2.18, 2077 checks with 0 violations; the double-negation rules fire 1 + 1 times on EWT 2.18, both in colloquial text.

---

👨‍🔬💥
