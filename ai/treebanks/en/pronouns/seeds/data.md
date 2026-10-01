# Pronouns — what these data are and which layers exist

**Gist.** Constructed examples for the independent possessive pronouns *hers, his, mine, yours, theirs*. The author is Robert Munro, for the book "Human-in-the-Loop Machine Learning". 57 sentences, each with the pronoun in a different syntactic role: subject, object, in coordination, after a preposition, in a relative clause… Each sentence is repeated with all five pronouns. In total 285 sentences and 1705 words. The goal is to fix the wrong tag of *hers* (NOUN or ADJ) in common parsers and to balance *hers* with *his*, and singular *theirs* with plural.

**Conditions and exceptions.**
- Test only. The README advises, if training, to split by even and odd `sent_id`.
- `# comment` describes the construction (*copular subject*, *extraction/raising via "tough extraction"*). `# previous` gives a possible preceding sentence: it makes *theirs* and *yours* unambiguously singular.
- No proper nouns (PROPN — 0).
- License CC BY-SA 4.0.

**Examples.** `1` *It is hers.* (`# previous = Which person owns this?`, `# comment = copular subject`); `166` *Hers was cleaned.*; `81` *The car is at hers.*

**In UD.**

| layer | state |
|---|---|
| XPOS | PTB, 21 tags |
| FEATS | present; `Case` removed in 2.8 (`conventions.md`) |
| lemmas | present: *hers* → *her*, *mine* → *my* — as in EWT since 2.11 |
| DEPS | absent |
| MISC | «_» in 1375 of 1705 |
| MWT | 65 |

Our annotator on Pronouns: UPOS 95.72, UFeats 69.56, LAS 70.50. UFeats is second from the bottom after CTeTex (58.07), because of the customs from `conventions.md`. LAS is low for such short sentences, because the constructions are deliberately rare.

**Sources.** README `UD_English-Pronouns` (Introduction, Data Statement, Changelog); `data/raw/ud-docs/docs/treebanks/en_pronouns/index.md`.
