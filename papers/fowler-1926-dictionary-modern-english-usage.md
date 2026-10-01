# A Dictionary of Modern English Usage

**Authors:** H. W. Fowler · **Year:** 1926 (1st edition) · **Venue:** book (Oxford: Clarendon Press; usage guide)
**Link:** https://archive.org/details/bwb_T5-BCG-591
**License of the paper:** Public domain (author died 1933; published 1926) — https://en.wikipedia.org/wiki/Henry_Watson_Fowler (later revisions by Gowers, Burchfield and Butterfield remain under copyright and are not used)

## Summary
A classic alphabetical usage guide of roughly four thousand entries on disputed forms, agreement, word choice, idiom and style in English. Entries range from short notes on a single word to long essays on topics such as number agreement, the split infinitive and the choice between "a" and "an". Fowler judges usage by clarity and by the habits of good writers rather than by Latin-based rules, and he often names the typical slip and explains why it happens. For Mova it serves as a public-domain base for rules about common errors, and as a reference point against later commercial usage guides.

## How Mova uses it
- Cited in 9 seeds in ai/en/seeds/errors/ and named in about 15 executable error rules of the grammar expert system (ai/en/src/expert.rs).
- The "A, AN" entry gives the rules in ai/en/seeds/errors/a-an-choice.md: flag "a" directly before a noun starting with a vowel letter and "an" before a consonant letter, with exceptions for pronunciation-driven cases noted in the seed.
- The "NUMBER" entry (agreement "red herrings") underlies attraction errors ("the quality of the apples are") in ai/en/seeds/errors/attraction-of-phrase.md, together with recent papers on agreement attraction in language models.
- Also used for coordinated-subject agreement, "there is/are" agreement, relative-clause agreement, determiner-noun number and the pronoun case after prepositions (ai/en/seeds/errors/coordinated-subject-agreement.md and neighbours).

## Effectiveness in Mova
Source for a small set of error rules. The seed for a/an records a check against the EWT 2.18 reference: "a" before a vowel letter 363/6 (all six flagged cases are errors in the text); "an" before a consonant letter 198/3 (abbreviations). Contribution to overall annotation quality is not measured separately. The rule engine as a whole (rules from all sources together) has been measured, not per book: on UD EWT train only 16 of 213 error-level rules fire at all, and almost all of their hits are errors in EWT itself; on a 292-sentence Opus annotation, violations of error-level rules were real errors in 36 of 36 cases, but covered only 6.4% of the erroneous tokens (warn-level rules another 3.9%). In short: precise, low recall, useful as hints rather than full control.

---

👨‍🔬💥
