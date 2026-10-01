# ICLE-RC: International Corpus of Learner English for Relative Clauses

**Authors:** Debopam Das, Izabela Czerniak, Peter Bourgonje · **Year:** 2025 · **Venue:** Proceedings of the 19th Linguistic Annotation Workshop (LAW-XIX-2025)
**Link:** https://aclanthology.org/2025.law-1.16/ (ACL Anthology 2025.law-1.16; DOI 10.18653/v1/2025.law-1.16)
**License of the paper:** CC BY 4.0 — https://creativecommons.org/licenses/by/4.0/

## Summary
The paper introduces ICLE-RC, a corpus of learner English annotated for relative clauses and related constructions. Earlier corpus work on English relative clauses relied mostly on small samples collected for narrow questions, and no large resource with detailed multi-level annotation existed. The corpus is built from academic essays in the International Corpus of Learner English written by students with six first languages from six different families. Each relative clause is described by lexical, syntactic, semantic and discourse features, and constructions that use the same markers — it-clefts, pseudo-clefts, existential relatives — are annotated separately. The paper also reports an inter-annotator agreement study, experiments on semi-automatic classification with a small pretrained transformer, and first distributional results. Because the L1s are typologically diverse, the corpus supports research on L1 influence, second-language acquisition, typology, World Englishes and discourse.

## How Mova uses it
- ai/en/seeds/errors/relative-pronoun-in-clause.md: cited for the inventory of relative markers (*that*, wh-forms, zero) and for the observation that learners also use *that* in non-restrictive relatives; supports rule `en.errors.relpron-inside-clause`.
- ai/en/seeds/errors/relative-clause-agreement.md: cited as a learner-English resource on relative clauses for rules `en.errors.relcl-plural-antecedent` / `en.errors.relcl-singular-antecedent` (when the relative pronoun is the subject, the relative-clause verb agrees in number with the antecedent; a mismatch means a text error or a wrong attachment).

## Effectiveness in Mova
Background reading for two rule cards; not measured separately. The agreement rules on UD English EWT 2.18: plural antecedent + VBZ — 338 hits / 1 flag; singular antecedent + VBP — 601 / 17.

---

👨‍🔬💥
