# A Modern English Grammar on Historical Principles, Parts II–VI

**Authors:** Otto Jespersen · **Year:** 1914–1942 (Part II 1914) · **Venue:** book (Heidelberg: C. Winter / London: Allen & Unwin)
**Link:** https://archive.org/details/modernenglishgra03jesp
**License of the paper:** Public domain in UA/EU (author died 1943); in the US Parts II–III (before 1931) are PD, Parts IV–VI may be under restored copyright — https://en.wikipedia.org/wiki/Otto_Jespersen

## Summary
Jespersen's multi-volume descriptive grammar of English, written over four decades, covers the language from Shakespeare to the 1930s and explains each present-day form through its history. Every phenomenon is illustrated by many dated quotations from literature, labelled as British or American, vulgar, archaic or poetic, which makes the work a large corpus of usage in its own right. Jespersen splits grammar into morphology (from form to meaning, the hearer's view) and syntax (from meaning to form, the speaker's view), and uses his own terminology: ranks (primary, secondary, tertiary words), nexus versus junction, conversion as a zero suffix, and "metanalysis" for re-segmentation of words. Mova uses five parts: four on syntax and the morphology volume. For annotation checking it is most valuable for its inventories of forms (irregular verbs, suffixes), its spelling rules for inflection, and its systematic account of doublet forms and which one is used attributively or predicatively.

## How Mova uses it
- The most widely cited grammar in the seeds: about 70 files across ai/en/seeds/morph/, ai/en/seeds/verbal/ and ai/en/seeds/lexicon/, and about 44 executable rules of the grammar expert system (ai/en/src/expert.rs) name it as a source.
- Verb lexicon lists: copula-like verbs that take no object, verbs taking an infinitive vs a gerund object, verbs licensing an indirect object, verbs without a passive, retained objects in the passive (ai/en/seeds/lexicon/verb-*.md).
- Morphology: lemma checks for homograph past forms and "lay" as past of "lie", -ness nouns, conversion and word-formation suffixes, plural mutation, pronoun paradigms (ai/en/seeds/morph/).
- Syntax: subject inversion in questions, do-support, negation, passive subjects, indirect object before direct object, open complements (ai/en/seeds/verbal/).
- Jespersen's descriptions are turned into conditions on Universal Dependencies trees; his rank and nexus ideas also inform the language-independent annotation design in the annot crate (background).

## Effectiveness in Mova
A core reference for the English rule seeds; its own contribution is not measured separately. The rule engine as a whole (rules from all sources together) has been measured, not per book: on UD EWT train only 16 of 213 error-level rules fire at all, and almost all of their hits are errors in EWT itself; on a 292-sentence Opus annotation, violations of error-level rules were real errors in 36 of 36 cases, but covered only 6.4% of the erroneous tokens (warn-level rules another 3.9%). In short: precise, low recall, useful as hints rather than full control.

---

👨‍🔬💥
