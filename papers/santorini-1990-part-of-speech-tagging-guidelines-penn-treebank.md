# Part-of-Speech Tagging Guidelines for the Penn Treebank Project (3rd Revision)

**Authors:** Beatrice Santorini · **Year:** 1990 · **Venue:** Technical Report MS-CIS-90-47, Department of Computer and Information Science, University of Pennsylvania
**Link:** unknown (technical report MS-CIS-90-47)
**License of the paper:** unknown

## Summary
These are the annotator guidelines behind the Penn Treebank part-of-speech tag set (NN, NNS, JJ, VBD, VBN and the rest). The report defines each tag and, more importantly, gives decision procedures for the hard cases that make tagging inconsistent: adjective versus noun, past tense versus past participle, particle versus preposition, and similar confusions. The tests are mostly substitution and modification tests, for example whether a word can be intensified by "very" or can take a plural ending. Many short examples illustrate each decision. The document became the de facto reference for English PTB-style tagging and is still the authority behind the XPOS column of English UD treebanks.

## How Mova uses it
- `ai/en/seeds/` — the English grammar seeds (short declarative rule notes with conditions, exceptions, examples and sources) cite the guidelines as the authority for PTB tag decisions; about 43 seed files reference it, mostly under `ai/en/seeds/morph/` (noun number, verb forms, adjective degree, pronouns, word formation) plus a few in `nominal/` and `verbal/`.
- Example: `ai/en/seeds/morph/adj-as-noun.md` uses the "(very) rich" test from §4.1 to separate partial conversion ("the rich" stays JJ/ADJ) from full conversion ("natives" becomes NNS/NOUN).
- The seeds combine the guidelines with classic descriptive grammars, so a tagging rule always carries both a linguistic rationale and the PTB convention it must produce.

## Effectiveness in Mova
Reference material for rule writing: it defines the target conventions rather than a technique. Not measured separately.

---

👨‍🔬💥
