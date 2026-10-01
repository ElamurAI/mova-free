# The Grammar of English Grammars

**Authors:** Goold Brown · **Year:** 1851 (10th ed. 1880 used, Project Gutenberg #11615) · **Venue:** book (New York; prescriptive grammar)
**Link:** https://www.gutenberg.org/ebooks/11615
**License of the paper:** Public domain (author died 1857; published 1851; Project Gutenberg text is PD in the US) — https://www.gutenberg.org/policy/license.html

## Summary
A massive nineteenth-century compendium of school grammar that both states the rules of English and argues at length with earlier grammarians such as Murray, Lowth and Webster. Its distinctive feature is the "False Syntax" material: thousands of deliberately faulty sentences to be corrected, each tied to a numbered rule, which makes it an unusually rich source of negative examples. The etymology part covers the parts of speech, with full verb paradigms (simple, progressive, passive, negative and interrogative forms) and lists of irregular and defective verbs. Syntax is condensed into 24 rules with Observations and Notes; the verb-related ones cover subject-verb agreement (collective nouns, "and", "or/nor", "each/either"), the infinitive with and without "to", participles, the nominative absolute, and objects and predicate nominals. The language is normative and in places dated ("thou" forms, strict subjunctive), so it is read as a record of rules and their exceptions rather than as a description of modern usage.

## How Mova uses it
- Cited in about 35 grammar "seeds" (curated rule notes) under ai/en/seeds/verbal/ and ai/en/seeds/lexicon/, and named as a source in about 30 executable rules of the English grammar expert system (ai/en/src/expert.rs reads the `rule` blocks from these seeds).
- Rule XIX (bare infinitive after bid, feel, hear, let, make, see) and its note that "to" returns in the passive became two rules in ai/en/seeds/lexicon/verb-bare-infinitive.md: an active causative/perception verb must not have a "to"-marked infinitive complement, a passive one must.
- Agreement rules (XIV–XVII) feed the subject-verb agreement and "finite verb has a subject" checks (ai/en/seeds/verbal/finite-verb-subject.md, agreement-*.md), and the nominative absolute (Rule VIII) informs ai/en/seeds/verbal/absolute-construction.md.
- Also used for verb lexicon lists: verbs without a passive and raising/control verbs (ai/en/seeds/lexicon/verb-no-passive.md, verb-raising-control.md).
- Prescriptive statements are translated into checks on Universal Dependencies trees (relations, XPOS tags, features), not taken over verbatim.

## Effectiveness in Mova
Source for one of the larger groups of executable rules. Its own contribution is not measured separately. The rule engine as a whole (rules from all sources together) has been measured, not per book: on UD EWT train only 16 of 213 error-level rules fire at all, and almost all of their hits are errors in EWT itself; on a 292-sentence Opus annotation, violations of error-level rules were real errors in 36 of 36 cases, but covered only 6.4% of the erroneous tokens (warn-level rules another 3.9%). In short: precise, low recall, useful as hints rather than full control.

---

👨‍🔬💥
