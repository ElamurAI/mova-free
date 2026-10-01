# Essentials of English Grammar for the Use of Schools

**Authors:** William Dwight Whitney · **Year:** 1877 · **Venue:** book (Boston: Ginn; school grammar)
**Link:** https://archive.org/details/essentialsofengl01whit
**License of the paper:** Public domain (author died 1894; published before 1931) — https://en.wikipedia.org/wiki/William_Dwight_Whitney

## Summary
A school grammar by one of the leading general linguists of the nineteenth century, a Sanskrit scholar at Yale. Whitney states explicitly that the grammarian records and orders usage rather than making laws, and where the language is inconsistent he accepts that custom decides. He deliberately departs from the Latin model: gender only for nouns denoting sex, two cases for nouns (an objective case only by analogy with pronouns), a clear line between true verb forms (two tenses, three moods) and analytic verb phrases, and infinitives and participles treated as verbal nouns and adjectives. Parts of speech are defined by sentence function, and conversion is treated as a kind of word formation. The book's value lies in short but precise lists (irregular verbs by class with an index, foreign and zero plurals, irregular comparison, suffixes) and in diagnostic remarks such as gerund vs noun and participle vs adjective ("very" vs "very much").

## How Mova uses it
- Cited in about 49 seeds in ai/en/seeds/morph/ and named in about 28 executable rules of the grammar expert system (ai/en/src/expert.rs).
- Verb inventories: strong verbs (ablaut) and their participles, mixed weak verbs, invariable verbs, homograph pasts, -s in the 3rd person, archaic -eth endings and archaic modals give tag/form checks such as `strong-participle-vbn`, `strong-preterite-form-tag`, `vbz-form-s`, `archaic-eth-vbz`.
- Nouns: mutation plurals ("men", "feet") with their number and lemma, zero plurals with plural determiners, plural-only nouns (`mutation-plural-number`, `zero-plural-with-plural-det`, `ptan-lemma`).
- Comparison and adverbs: periphrastic and suppletive comparison, false comparatives, flat and -ly adverbs, cited together with Sweet, Kruisinga and Mätzner (ai/en/seeds/morph/adj-*.md, adv-*.md).

## Effectiveness in Mova
A core reference for verb and noun inventories in the morphology rules; its own contribution is not measured separately. The rule engine as a whole (rules from all sources together) has been measured, not per book: on UD EWT train only 16 of 213 error-level rules fire at all, and almost all of their hits are errors in EWT itself; on a 292-sentence Opus annotation, violations of error-level rules were real errors in 36 of 36 cases, but covered only 6.4% of the erroneous tokens (warn-level rules another 3.9%). In short: precise, low recall, useful as hints rather than full control.

---

👨‍🔬💥
