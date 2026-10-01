# An English Grammar: Methodical, Analytical, and Historical

**Authors:** Eduard Mätzner (translated by C. J. Grece) · **Year:** 1874 · **Venue:** book, 3 vols (London; translation of "Englische Grammatik")
**Link:** https://archive.org/details/englishgrammarme01maetuoft
**License of the paper:** Public domain (author died 1892, translator died 1905; UA/EU and US) — https://archive.org/details/englishgrammarme01maetuoft

## Summary
The English translation of Mätzner's large German grammar of English, historical and descriptive in approach: each phenomenon is traced from Anglo-Saxon through Middle English to the modern language and illustrated with many quotations from Chaucer and Shakespeare to Byron, Macaulay and Dickens. Mätzner records variants rather than prescribing, and sometimes argues directly against narrow rules of earlier grammarians, for example on comparison of adjectives. This makes it strong as an inventory of forms: exceptional plurals (-ves, -oes/-os, foreign plurals, plurals of compounds), plural-only nouns, strong verbs by class with obsolete forms marked, irregular weak verbs, comparison and -most forms, pronouns, and long lists of suffixes and prefixes. Its limits are nineteenth-century and mainly literary usage with many archaisms, no statistics, and little on pronunciation.

## How Mova uses it
- Cited in about 24 seeds in ai/en/seeds/morph/ (comparison, adjectives used as nouns, -ly and flat adverbs, articles, genitive -s, plurals, pronouns, word-formation suffixes, spelling changes) and named in 6 executable rules of the grammar expert system (ai/en/src/expert.rs).
- Its spelling examples ("bigger, hotter", "politer, abler, truer", "happier" vs "gayer") support lemma checks for consonant doubling, y-to-i and silent -e on verbs, nouns and adjectives (e.g. `doubling-adj-lemma`, `y-to-i-noun-lemma`, `y-to-i-adj-lemma`).
- Used as cross-evidence next to Sweet, Whitney and Kruisinga rather than as a sole source.

## Effectiveness in Mova
A supporting reference for morphology seeds; its own contribution is not measured separately. The rule engine as a whole (rules from all sources together) has been measured, not per book: on UD EWT train only 16 of 213 error-level rules fire at all, and almost all of their hits are errors in EWT itself; on a 292-sentence Opus annotation, violations of error-level rules were real errors in 36 of 36 cases, but covered only 6.4% of the erroneous tokens (warn-level rules another 3.9%). In short: precise, low recall, useful as hints rather than full control.

---

👨‍🔬💥
