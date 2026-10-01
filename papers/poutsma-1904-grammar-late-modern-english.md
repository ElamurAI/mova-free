# A Grammar of Late Modern English

**Authors:** Hendrik Poutsma · **Year:** 1904 (first edition; the 5 volumes used are 1914–1929) · **Venue:** book (Groningen: P. Noordhoff)
**Link:** https://archive.org/details/grammaroflatemod01poutuoft
**License of the paper:** Public domain (author died 1937; all volumes published before 1931) — https://www.dbnl.org/auteurs/auteur.php?id=pout001

## Summary
A five-volume descriptive grammar of literary English of the nineteenth and early twentieth centuries, written by a Dutch teacher for Continental students. Every claim is supported by many sourced quotations from writers such as Dickens, Thackeray and Trollope and from newspapers, often with cross-references to Sweet, Jespersen, Mätzner and the OED. The method is to show usage with its fluctuations rather than to prescribe: what is ordinary, colloquial, vulgar or archaic, and where "usage is divided". It draws fine distinctions between functions of the same word: conjoint and absolute possessives (my/mine), individual and classifying genitives, partial and full conversion of adjectives into nouns, reflexive and emphatic -self. It gives much attention to word order, notional agreement with collective nouns, the choice between 's and of, who/which/that, and the use and omission of the article. Its categories are close to modern ones (relative "that" as a pronoun, "one" as a prop-word), so they map well onto Universal Dependencies.

## How Mova uses it
- One of the two largest sources of the rule seeds: about 60 files, mostly in ai/en/seeds/nominal/ and ai/en/seeds/errors/, and about 88 executable rules of the grammar expert system (ai/en/src/expert.rs) name it.
- Noun-phrase rules: articles and "a/an" by sound, predeterminers, adjective before article ("so harsh an answer"), ordinals vs numerals, demonstrative number, collective and mass nouns, absolute and double genitives, prop-word "one", relative "that", reflexive agreement, "a-" adjectives (afraid, asleep) not used attributively (ai/en/seeds/nominal/).
- Error rules: determiner-noun number mismatch, singular subject with plural verb, nominative case of subject pronouns (ai/en/seeds/errors/determiner-noun-number.md, singular-subject-plural-verb.md, subject-pronoun-case.md).
- Verb lexicon: verbs taking a gerund rather than a "to"-infinitive object and raising/control verbs (ai/en/seeds/lexicon/verb-gerund-object.md, verb-infinitive-object.md).

## Effectiveness in Mova
A core reference for noun-phrase and agreement rules; its own contribution is not measured separately. The rule engine as a whole (rules from all sources together) has been measured, not per book: on UD EWT train only 16 of 213 error-level rules fire at all, and almost all of their hits are errors in EWT itself; on a 292-sentence Opus annotation, violations of error-level rules were real errors in 36 of 36 cases, but covered only 6.4% of the erroneous tokens (warn-level rules another 3.9%). In short: precise, low recall, useful as hints rather than full control.

---

👨‍🔬💥
