# ParTUT — known annotation errors

**Gist.** ParTUT is an automatic conversion from TUT via USD with manual corrections. The last corrections were in 2.5. So alongside the Italian customs there are traces of the conversion: lemmas, labels and features that match neither EWT nor ParTUT itself. The converter does not fix them.

**Examples and numbers.**
- **Pronoun lemmas in lowercase and in the oblique case:** *me* → *i* (9, lowercase), *us* → *us* (28), *them* → *them* (1), *themselves* → *they* (5), *our/ours* → *us* (60 DET + 2 PRON). EWT: *I*, *we*, *they*, *themselves*, *our*.
- **An adjacent noun as `obj`** of the head noun — 15 (`nmod-compound.md`): a noun does not take `obj`.
- **Possessive PRON among DET** — 2 of 617: an inconsistency in the custom (`structures.md`).
- **`Number` on PROPN** — only 82 of 2230: some proper nouns have Number, the rest do not.
- **Passive subject as `nsubj`** (`tb.en.pass-subj`) — 1 of 453.
- **`obl` on a noun without a copula** — 0 of 11. Here ParTUT already agrees with UD 2.18.

**In UD.** For merging, ParTUT's main problem is not errors but customs: `nmod` in compound nouns (862), features without `Case` and `Voice`, possessive DET. The errors (≈ 120) are within the noise.

**Sources.** README `UD_English-ParTUT` (changelog: last corrections — 2.5); rules in `../../seeds-overview.md`.
