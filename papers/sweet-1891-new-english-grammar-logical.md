# A New English Grammar, Logical and Historical

**Authors:** Henry Sweet · **Year:** 1891 (Part I: Introduction, Phonology and Accidence); Part II: Syntax, 1898 · **Venue:** book (Oxford: Clarendon Press)
**Link:** https://archive.org/details/newenglishgramma01sweeuoft
**License of the paper:** Public domain (author died 1912; published 1891–1898) — https://en.wikipedia.org/wiki/Henry_Sweet

## Summary
The first large scholarly grammar of English to combine a description of the contemporary language with its history. Sweet keeps form and meaning apart and distinguishes "logical" from "grammatical" categories; for him a word's part of speech is decided by its form, so a converted word takes on all the formal marks of its new class. This leads to clear distinctions between full and partial conversion ("goods" vs "the good"), between gerund, participle and verbal noun, and between compound and phrase (decided by stress). The description is based on spoken standard southern British English of the 1890s, with literary, obsolete and vulgar forms marked and pronunciation given in his phonetic notation. For Mova its value lies in its lists: plurals (mutation, foreign plurals, "fathers-in-law"), comparison, irregular verbs in a new classification, anomalous verbs with spoken contractions, affixes marked for productivity, and noun/verb stress pairs. Some usage is dated (e.g. "court-martials", "data" only plural).

## How Mova uses it
- One of the main morphology sources: cited in about 54 seeds in ai/en/seeds/morph/ and named in about 53 executable rules of the grammar expert system (ai/en/src/expert.rs).
- Comparison: which adjectives take -er/-est and which take more/most, suppletive forms, and false comparatives (words ending in -er/-est that are not comparatives) give the checks `comparative-form`, `superlative-form`, `false-comparative-er`, `false-superlative-est` (ai/en/seeds/morph/adj-comparison-*.md, adj-false-comparatives.md).
- Plurals and lemmas: -ves plurals and homographs ("lives", "leaves"), y-to-i, mutation plurals, plural-only nouns, foreign plurals (ai/en/seeds/morph/noun-*.md).
- Verbs: homograph past forms ("lay" as past of "lie"), invariable verbs (put, cut) in 3rd-person checks, modals and archaic endings; also pronoun paradigms and word-formation affixes.
- The rules are written as checks on Universal Dependencies lemmas, XPOS tags and features.

## Effectiveness in Mova
A core reference for morphology rules; its own contribution is not measured separately. The rule engine as a whole (rules from all sources together) has been measured, not per book: on UD EWT train only 16 of 213 error-level rules fire at all, and almost all of their hits are errors in EWT itself; on a 292-sentence Opus annotation, violations of error-level rules were real errors in 36 of 36 cases, but covered only 6.4% of the erroneous tokens (warn-level rules another 3.9%). In short: precise, low recall, useful as hints rather than full control.

---

👨‍🔬💥
