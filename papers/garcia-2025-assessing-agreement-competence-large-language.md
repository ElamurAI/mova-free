# Assessing the Agreement Competence of Large Language Models

**Authors:** Alba Táboas García, Leo Wanner · **Year:** 2025 · **Venue:** Proceedings of the Eighth International Conference on Dependency Linguistics (Depling, SyntaxFest 2025)
**Link:** https://aclanthology.org/2025.depling-1.4/ (ACL Anthology 2025.depling-1.4)
**License of the paper:** CC BY 4.0 — https://creativecommons.org/licenses/by/4.0/

## Summary
Agreement is a common probe for whether a language model has learned hierarchical sentence structure rather than surface word order. Most such probes have targeted English, where agreement is essentially subject-verb number. This paper extends the evaluation to four morphologically richer languages: Italian, Portuguese, Spanish and Russian. For each language, fluent linguists hand-built ten test suites covering agreement inside noun phrases and within the clause, in gender, number and person, plus case for Russian. Each item pairs a grammatical sentence with one or more deliberately corrupted variants, and the harder items put distance and an "attractor" with different features between controller and target. A model passes an item if it prefers the grammatical form. Twenty-five monolingual and multilingual models were tested on more than 5,000 handmade items. Models do reasonably well overall but fail on the complex constructions with attractors. Monolingual models beat multilingual ones, and model size or training data volume matter little.

## How Mova uses it
- `ai/en/seeds/errors/attraction-of-phrase.md`: cited as evidence that attractor constructions are the hardest for LLMs. This motivates the warning rules `en.errors.attraction-of` and `en.errors.attraction-of-auxcop`, which flag a singular subject with a plural of-phrase followed by a plural (VBP) verb ("A pattern of arrests indicate"), with an exception list of quantity nouns (lot, number, majority...).
- `ai/en/seeds/errors/relative-clause-agreement.md`: cited for relative clauses with attractors being the hardest case. It backs `en.errors.relcl-plural-antecedent` and `en.errors.relcl-singular-antecedent`, which compare the number of the antecedent noun with the tag of the relative-clause verb. A mismatch is a text error or a wrong attachment of the relative clause.
- In Mova the paper's evaluation idea is reused in reverse: instead of probing a model, these rules check annotations (from Mova's own tagger or an LLM annotator) for exactly the construction where agreement breaks down.

## Effectiveness in Mova
Not measured separately. The paper is one supporting source, alongside Fowler and BLiMP, for four rules in the English error-detection rule set. Gold-data checks reported in the seeds: attraction rule on EWT 2.18, 7 checks / 1 hit for verbs (exactly "pattern ... indicate") and 13/5 for copula or auxiliary; GUM 16/3 and 27/8. Relative-clause rules on EWT 2.18: plural antecedent + VBZ 338/1, singular antecedent + VBP 601/17.

---

👨‍🔬💥
