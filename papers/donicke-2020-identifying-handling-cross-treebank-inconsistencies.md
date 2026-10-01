# Identifying and Handling Cross-Treebank Inconsistencies in UD: A Pilot Study

**Authors:** Tillmann Dönicke, Xiang Yu, Jonas Kuhn · **Year:** 2020 · **Venue:** Proceedings of the Fourth Workshop on Universal Dependencies (UDW 2020)
**Link:** https://aclanthology.org/2020.udw-1.8/ (ACL Anthology 2020.udw-1.8)
**License of the paper:** CC BY 4.0 — https://creativecommons.org/licenses/by/4.0/

## Summary
Treebanks of the same language in Universal Dependencies share one label set, but individual relations are often annotated differently from treebank to treebank, which hurts merging corpora and transferring parsers. The authors propose a simple, language-independent signal: for each relation in each treebank, compute the share of dependents that appear to the right of their head, then take the largest gap in that share between treebanks of the same language (called MBD). On UD 2.5 they manually inspect the 20 language-relation pairs with the highest MBD and separate genuine inconsistencies from false alarms. Two clear cases, Chinese classifiers and Korean auxiliaries, are then fixed in the training treebanks with simple conversion rules that follow the UD guidelines. A UDPipe model retrained on the harmonised data gains roughly 1 to 3.4 LAS points on guideline-conforming test treebanks. The broader point is that even crude arc-direction statistics expose hidden annotation drift, and fixing it measurably improves cross-treebank parsing.

## How Mova uses it
- `ai/en/seeds/errors/consistency-detection-methods.md`: the arc-direction (MBD) method is listed as one of five ways to find annotation errors automatically. Mova adopted its logic as a family of direction rules: `flat` and `conj` go rightward, `compound` and `nummod` leftward, `cc` precedes its conjunct.
- `ai/en/seeds/errors/clausal-subject-csubj.md`: cited for English `csubj` being among the most inconsistent relations across treebanks. This supports the warning rule `en.errors.verb-subject-csubj` (an `nsubj` on a VERB should be `csubj`).
- `ai/en/seeds/errors/flat-compound-direction.md` and `ai/en/seeds/nominal/compound.md`: cited for English `compound` (rightward in ParTUT because of `compound:prt` and names annotated as compounds). This backs the rule `en.nominal.compound-head-final` (a compound dependent precedes its nominal head).
- `ai/en/seeds/errors/english-treebank-conventions.md`: background for treating a higher violation count on GUM than on EWT as a treebank convention, not a rule error.

## Effectiveness in Mova
Not measured separately. Its idea is the basis of several rules in the rule engine. The seeds report how those rules behave on gold data, as written: on EWT 2.18, `flat` to the right in 2501 of 2501 cases; `compound` between nouns to the left 8416 times with 16 violations (GUM 6713/36); `compound` to the right 23 times out of 8690, mostly errors in the gold annotation itself; `nsubj` on VERB 4 on EWT and 8 on GUM. These numbers validate the rules. They do not measure what the paper adds over other sources.

---

👨‍🔬💥
