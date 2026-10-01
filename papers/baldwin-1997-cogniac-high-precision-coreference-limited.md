# CogNIAC: high precision coreference with limited knowledge and linguistic resources

**Authors:** Breck Baldwin · **Year:** 1997 · **Venue:** Operational Factors in Practical, Robust Anaphora Resolution for Unrestricted Texts (ACL/EACL 1997 workshop)
**Link:** https://aclanthology.org/W97-1306/ (ACL Anthology W97-1306)
**License of the paper:** CC BY-NC-SA 3.0 (ACL Anthology, pre-2016) — https://creativecommons.org/licenses/by-nc-sa/3.0/

## Summary
CogNIAC is a rule-based pronoun resolution system designed for settings where deep linguistic knowledge and rich resources are not available. Its guiding idea is to favour precision over recall: a pronoun is resolved only when one of a small, ordered set of high-confidence rules applies, and is otherwise left unresolved rather than guessed. The rules rely on shallow cues such as uniqueness of a compatible antecedent in the current and preceding sentences, reflexives, possessives and grammatical role. Because each rule is simple and its firing is explicit, every resolution decision can be traced back to the rule that produced it. The paper argues that such a conservative resolver is useful as a component in larger systems, where a wrong link costs more than a missing one. It is an early, influential example of the "precise rules first" design later generalised by multi-pass sieve coreference systems.

## How Mova uses it
- the development notes of `ai/coref`, sieve `speaker`: the rules for first- and second-person pronouns within one voice ("I" of one speaker is one person; "we"/"you" of one voice are linked; "you" addressed to someone links to that addressee's earlier "I"; "I" inside a quotation refers to the subject of the speech verb outside it) are taken from Lee et al. 2013 (§3.3.1) together with Baldwin 1997.
- The broader CogNIAC principle — resolve only on a confident rule, otherwise abstain, and record which rule fired — matches the design of Mova's sieve coreference, where every link carries the id of the sieve that created it.

## Effectiveness in Mova
The `speaker` sieve, whose rules are credited jointly to Lee et al. 2013 and this paper, is measured on the GUM test set (UD 2.18): 771 links at 90.7% precision, contributing +21.88 CoNLL F1 (without singletons, cumulative, on gold trees) and +21.99 on Mova's own parses. It is the first and most precise sieve. The share attributable to Baldwin 1997 alone is not measured separately.

---

👨‍🔬💥
