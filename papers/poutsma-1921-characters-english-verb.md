# The Characters of the English Verb (1921); The Infinitive, the Gerund and the Participles of the English Verb (1923)

**Authors:** Hendrik Poutsma · **Year:** 1921 and 1923 · **Venue:** books (monographs; Groningen: Noordhoff)
**Link:** https://archive.org/details/charactersofengl00pout
**License of the paper:** Public domain (author died 1937; published 1921 and 1923) — https://www.dbnl.org/auteurs/auteur.php?id=pout001

## Summary
Two monographs that expand the verb chapters of Poutsma's larger grammar. The 1921 book contains two essays: one on the "characters" of verbs (Aktionsart) such as momentary, durative, ingressive, continuative and iterative, showing how context shifts a verb's character; the other on the "expanded form" (be + -ing), with its progressive, prospective ("I am leaving tomorrow"), characterising ("you are always grumbling") and other uses, and with a discussion of verbs that resist it (copulas and verbs of perception, feeling and thinking). The 1923 book deals with the non-finite forms: when the infinitive takes "to" and when not (after auxiliaries, need, dare, had better, perception and causative verbs, after but/than), tense and voice of the infinitive, verbal and nominal features of the gerund (his/him coming), and whether participles behave as verbs or adjectives. Each point is illustrated with many literary quotations; the text available is an OCR of scans and somewhat noisy.

## How Mova uses it
- Cited in about 21 seeds in ai/en/seeds/lexicon/ and ai/en/seeds/verbal/, and named in about 13 executable rules of the grammar expert system (ai/en/src/expert.rs).
- Stative verbs and the progressive (§§38–42 of "The Expanded Form") give the rule `stative-progressive` in ai/en/seeds/lexicon/verb-stative-progressive.md.
- The infinitive study gives the bare-infinitive rules: no "to" after active perception/causative verbs, "to" after their passive (`bare-infinitive-active`, `to-infinitive-after-passive` in ai/en/seeds/lexicon/verb-bare-infinitive.md), and the split between gerund-taking and infinitive-taking verbs (verb-gerund-object.md, verb-infinitive-object.md).
- Also used for modal + non-finite, need/dare, semi-modals, "to" before -ing, and participle-or-adjective decisions (ai/en/seeds/verbal/).

## Effectiveness in Mova
A focused source for verb-complement rules; its own contribution is not measured separately. The rule engine as a whole (rules from all sources together) has been measured, not per book: on UD EWT train only 16 of 213 error-level rules fire at all, and almost all of their hits are errors in EWT itself; on a 292-sentence Opus annotation, violations of error-level rules were real errors in 36 of 36 cases, but covered only 6.4% of the erroneous tokens (warn-level rules another 3.9%). In short: precise, low recall, useful as hints rather than full control.

---

👨‍🔬💥
