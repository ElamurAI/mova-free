# Syntax of referents of relative markers: Evidence from a corpus of learner English

**Authors:** Izabela Czerniak, Debopam Das · **Year:** 2025 · **Venue:** Proceedings of the 23rd International Workshop on Treebanks and Linguistic Theories (TLT, SyntaxFest 2025)
**Link:** https://aclanthology.org/2025.tlt-1.12/ (ACL Anthology 2025.tlt-1.12)
**License of the paper:** CC BY 4.0 — https://creativecommons.org/licenses/by/4.0/

## Summary
The paper studies the antecedents (referents) of relative markers in English relative clauses, focusing on the syntactic role those antecedents play in the matrix clause — an aspect much less studied than the markers themselves. It is part of the ICLE-RC project, a corpus of academic essays by learners of English drawn from the International Corpus of Learner English and covering six first languages from different families. The annotation scheme records the marker type, its function inside the relative clause, the type and grammatical function of the antecedent, and whether the clause is restrictive (integrated) or non-restrictive (supplementary); related constructions such as clefts and existential relatives are annotated separately. The analysis finds that how often a referent is relativised depends on its syntactic function, and that referent properties interact with other relative-clause features in ways that vary systematically by the writer's first language. Some of these differences are linked to typological properties of the L1s. The corpus is useful for research on cross-linguistic influence, second-language acquisition and World Englishes.

## How Mova uses it
- ai/en/seeds/errors/relative-pronoun-in-clause.md: cited as background on antecedent functions for rule `en.errors.relpron-inside-clause` — a relative pronoun must be a member of the relative clause (`nsubj`, `obj`, `obl`…) under its predicate, not attached directly to the antecedent; the clause itself is `acl:relcl` of the antecedent.

## Effectiveness in Mova
Background reading for one rule; not measured separately. The rule itself on UD English EWT 2.18 flags 1 pronoun attached to the antecedent and 1 of 619 cases of *that* in `acl:relcl` not tagged PRON (GUM: 0 and 6).

---

👨‍🔬💥
