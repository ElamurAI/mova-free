# Higher Lessons in English (1877/1886) and Graded Lessons in English (1877)

**Authors:** Alonzo Reed, Brainerd Kellogg · **Year:** 1877 · **Venue:** books (school grammars; Project Gutenberg #7188, #7010)
**Link:** https://www.gutenberg.org/ebooks/7188 (also https://www.gutenberg.org/ebooks/7010)
**License of the paper:** Public domain (authors died 1899 and 1920; published before 1931) — https://www.gutenberg.org/policy/license.html

## Summary
Two American school textbooks that introduced the Reed–Kellogg sentence diagram, the standard way of parsing sentences in American schools for more than a century. The junior course has 97 lessons, the senior one 168. Both move from the sentence down to the word: subject and predicate first, then modifiers, prepositional phrases and compound members, then three kinds of complement (direct object, predicate attribute after "be/become", and objective complement as in "made Victoria queen"). Later lessons treat participles and infinitives, independent elements (address, absolute phrases, expletive "there"), word order and inversion, ellipsis, and complex and compound sentences, followed by the parts of speech with their forms, including a "by"-agent test for the passive. Each lesson has analysis and parsing exercises, and diagrams are reproduced in text form, so the books amount to a small school-style treebank.

## How Mova uses it
- Cited in about 30 seeds in ai/en/seeds/verbal/ and ai/en/seeds/lexicon/ (agreement with compound, indefinite and third-person subjects, absolute constructions, copula-like verbs, retained objects in the passive, objective complements, existential "there", auxiliary heads, ellipsis and gapping).
- Its clause analyses are used to decide how such constructions should look as Universal Dependencies trees (which word is the head, what relation the element gets), e.g. the absolute phrase in ai/en/seeds/verbal/absolute-construction.md.
- Named directly in one executable rule, `parataxis-has-subject` (Lesson 57, coordinated predicates; ai/en/seeds/verbal/finite-verb-subject.md).

## Effectiveness in Mova
Mostly background for how constructions are analysed, with one rule citing it directly; not measured separately.

---

👨‍🔬💥
