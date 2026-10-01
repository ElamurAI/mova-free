# A Grammar of the English Language, Vol. III: Syntax

**Authors:** George O. Curme · **Year:** 1931 · **Venue:** book (Boston/London: D. C. Heath)
**Link:** https://archive.org/details/dli.ernet.8033
**License of the paper:** Public domain in UA/EU (author died 1948); US status depends on copyright renewal, not verified — https://en.wikipedia.org/wiki/George_O._Curme

## Summary
The most complete American descriptive syntax of English from the interwar period, built on a large body of examples from Old English texts through to contemporary American prose and speech. Curme explains each construction historically (for example, where the group genitive and relative "who" come from) and then divides current usage into literary, colloquial, popular, British and American layers. Its strongest chapters deal with modifiers of the noun: the order and stress of adjectives (attributive "adherent" versus postposed "appositive" use), the genitive in all its senses, apposition, restrictive and descriptive relative clauses, and subject-verb agreement with collective nouns, "each/everyone" and "these kind of". Some terminology is dated (he treats "one" in "the red one" as a suffix), so the concepts have to be mapped onto modern categories rather than copied. It complements the British and Continental grammars of Jespersen, Poutsma and Kruisinga with American norms.

## How Mova uses it
- Cited in about 34 seeds, mostly in ai/en/seeds/nominal/ (adjective position, adjective before article as in "too costly a sacrifice", absolute genitives and possessives, adjectives used as nouns, collective and mass nouns, relative clauses, who/whom/whose, prop-word "one"), and named in about 24 executable rules of the grammar expert system (ai/en/src/expert.rs).
- Agreement material feeds the error rules in ai/en/seeds/errors/singular-subject-plural-verb.md.
- Exceptions from Curme (e.g. subjectless "Thank you", "Hope to see you again") are used as `unless` clauses in ai/en/seeds/verbal/finite-verb-subject.md, and fixed postposed adjectives shape the rule `postposed-adj-has-deps`.
- Descriptions are converted into conditions on Universal Dependencies trees (UPOS, XPOS, features, relations).

## Effectiveness in Mova
A major source for noun-phrase rules; its own contribution is not measured separately. The rule engine as a whole (rules from all sources together) has been measured, not per book: on UD EWT train only 16 of 213 error-level rules fire at all, and almost all of their hits are errors in EWT itself; on a 292-sentence Opus annotation, violations of error-level rules were real errors in 36 of 36 cases, but covered only 6.4% of the erroneous tokens (warn-level rules another 3.9%). In short: precise, low recall, useful as hints rather than full control.

---

👨‍🔬💥
