# LittlePrince — the Stanza legacy: EWT customs before 2.11–2.17 alongside new ones

**Gist.** The annotation grew out of the output of Stanza trained on older EWT versions. The students corrected mainly trees and UPOS. So FEATS, lemmas and some labels keep the old norm, while what was corrected is already new. The 2017-era treebank mixes two norms:
- **`:tmod`/`:npmod` alongside `:unmarked`:** `obl:unmarked` 19, `obl:tmod` 6, `obl:npmod` 3. Even *day* occurs both as `obl:unmarked` (5) and as `obl:tmod` (2).
- **Agreement by form only:** 317 of 625 finite verbs with Tense lack Number/Person. Past with Number — 151, mostly *was/were* (76), but also *say* 6, *do* 5, *have* 4: the students filled in some.
- **Possessive lemmas by the old rule:** *his* → *he* (39 versus 5 *his* → *his*), *your* → *you* (14 versus 2), *their* → *they* (3), *our* → *we* (2), *mine* → *mine*.
- **`Case=Gen` on possessives** — only 12 of 122 in `nmod:poss` (`tb.en.poss-gen` 110 of 122).
- **Expletive *there* without `PronType=Dem`** — 15 of 15. EWT since 2.17 — `Dem`.
- ***not* without `Polarity=Neg`** — 65 of 69. EWT since 2.15 — everywhere.
- **`NumForm` in MISC**, not in FEATS: `tb.en.num-feats` 142 of 142.
- **Subordinate *when/how*** as SCONJ `mark`: 18 of 43. EWT since 2.11 — ADV `advmod`.
- ***you* with Number** — 11 of 85: traces of the GUM custom that Stanza was also trained on (`../../gum/seeds/you-number.md`).

**Conditions and exceptions.** `advcl:relcl` (6), `nsubj:outer` (2), `obl:agent` (6), `ExtPos` (22) are already in the treebank: newer customs appear where the students corrected the tree.

**Examples.**
- `lpp_1943.284` *" One day , " you said to me…* — *day*: `obl:tmod`.
- `lpp_1943.228` *…there were on the planet…* — *there*: `expl` without `PronType=Dem`.
- `lpp_1943.265` *…the secrets of your sad little life…* — *your*: lemma *you*, without `Case=Gen`.
- `lpp_1943.267` *…on the morning of the fourth day , when you said to me…* — *when*: SCONJ `mark`.

**In UD.**

| rule | LittlePrince | EWT |
|---|---:|---:|
| `tb.en.tmod-npmod` | 9 / 9 | 0 / 0 |
| `tb.en.fin-agree` | 317 / 625 | 0 / 19 126 |
| `tb.en.poss-gen` | 110 / 122 | 16 / 3689 |
| `tb.en.pron-lemma` | 15 / 201 | 9 / 5039 |
| `tb.en.expl-there` | 15 / 15 | 0 / 458 |
| `tb.en.not-polarity` | 65 / 69 | 0 / 2077 |
| `tb.en.num-feats` | 142 / 142 | 15 / 5051 |
| `tb.en.wh-advmod` | 18 / 43 | 0 / 1049 |

**Sources.** README `UD_English-LittlePrince` («silver parses … output of the Stanza parser … manually corrected»); Qi et al. 2020, `2020.acl-demos.14`; EWT README v2.11, v2.15, v2.17; UD docs/changes.md` No. 18.
