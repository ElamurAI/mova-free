# A Handbook of Present-Day English

**Authors:** Etsko Kruisinga · **Year:** 1925 (Part I, 4th ed.); Part II, 5th ed. 1931–1932 · **Venue:** book (Groningen: Noordhoff)
**Link:** https://archive.org/details/handbookofpresen12krui
**License of the paper:** Public domain in UA/EU (author died 1944); in the US Part I (1925) is PD, Part II (1931–1932) only UA/EU-PD — https://en.wikipedia.org/wiki/Etsko_Kruisinga

## Summary
A large synchronic grammar of English as spoken and written in roughly 1900–1930, from the Dutch school of English studies; the final edition consists of a volume on sounds and three volumes on accidence and syntax. Kruisinga describes living usage rather than school norms, backing each point with quotations from contemporary novels, newspapers and colloquial-language manuals, and uses history only to explain. He defines parts of speech by function and treats form and use together, and he openly criticises school rules about "whom", genitive plurals or "an hotel". Its strongest parts are full lists of irregular verbs grouped by vowel alternation, plurals and genitives with their exceptions, and comparison of adjectives, where he argues that stress, not syllable count, decides between -er/-est and more/most. He also covers living word-formation suffixes, clipping, and a careful analysis of full versus partial conversion.

## How Mova uses it
- Cited in about 40 seeds in ai/en/seeds/morph/ (comparison with -er/-est and more/most, suppletive and false comparatives, flat adverbs and -ly adverbs, articles, plurals, genitives, verb forms, clitics, spelling changes) and named in 6 executable rules of the grammar expert system (ai/en/src/expert.rs).
- Its spelling rules for inflection give lemma checks: consonant doubling, y-to-i, silent -e, "ie" to "ying", and the lemma of clipped forms (`doubling-verb-lemma`, `y-to-i-verb-lemma`, `silent-e-verb-lemma`, `ie-ying-lemma`, `clipping-lemma`).
- The stress-based account of comparison (ai/en/seeds/morph/adj-comparison-er-est.md) is cited next to Sweet, Whitney and Mätzner as supporting evidence for the form checks on JJR/JJS tags.

## Effectiveness in Mova
A supporting reference for morphology seeds, mostly as cross-checked evidence; its own contribution is not measured separately. The rule engine as a whole (rules from all sources together) has been measured, not per book: on UD EWT train only 16 of 213 error-level rules fire at all, and almost all of their hits are errors in EWT itself; on a 292-sentence Opus annotation, violations of error-level rules were real errors in 36 of 36 cases, but covered only 6.4% of the erroneous tokens (warn-level rules another 3.9%). In short: precise, low recall, useful as hints rather than full control.

---

👨‍🔬💥
