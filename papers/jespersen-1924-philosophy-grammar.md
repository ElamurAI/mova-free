# The Philosophy of Grammar

**Authors:** Otto Jespersen · **Year:** 1924 · **Venue:** book (London: G. Allen & Unwin)
**Link:** https://archive.org/details/dli.bengal.10689.9325
**License of the paper:** Public domain (author died 1943; published 1924, PD in the US) — https://archive.org/details/dli.bengal.10689.9325

## Summary
A theoretical book in which Jespersen steps back from describing English to ask what grammatical categories are and how they relate to the categories of thought. He separates syntactic categories (forms a language actually marks) from notional categories (meanings any language may express), and argues that the two should not be confused. He introduces the theory of ranks, where words in a phrase are primary, secondary or tertiary according to their function, and contrasts junction (an attribute joined to a head, "the barking dog") with nexus (a predication, "the dog barks"). The book discusses number, person, case, tense, mood, voice and negation from a cross-linguistic perspective, with examples from many languages. It also discusses ellipsis phenomena such as "prosiopesis", where the opening of an utterance is dropped in speech ("Thank you", "Hope I'm not boring you").

## How Mova uses it
- Cited in 2 seeds and 3 executable rules of the grammar expert system (ai/en/src/expert.rs).
- Prosiopesis is used as a principled exception: subjectless finite verbs such as "Thank you" or "Hope you're well" are exempted in the rules `finite-has-subject` and `parataxis-has-subject` (ai/en/seeds/verbal/finite-verb-subject.md).
- The same notion explains a dropped sentence-initial article ("Room was amazing") as an exception in the missing-article rule (ai/en/seeds/errors/missing-article.md).
- Its separation of notional and syntactic categories is background reading for the language-independent annotation work (annot).

## Effectiveness in Mova
Plays a narrow role: it supplies exceptions for a few rules, which reduce false alarms. The "finite verb has a subject" rule went from 257 violations on EWT train to 9 (plus 17 in `parataxis-has-subject`) after the rule language gained exceptions and the rule was reworked. This change combined several sources, so this book's share is not measured separately.

---

👨‍🔬💥
