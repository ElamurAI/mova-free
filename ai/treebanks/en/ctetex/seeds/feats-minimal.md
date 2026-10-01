# CTeTex — FEATS reduced to `Number` and `Tense`

**Gist.** Of all English UD features, CTeTex sets only:
- `Number` — on nouns and 3sg verbs;
- `Tense` — on verbs;
- isolated `Typo` and `ExtPos`.

There is no `VerbForm`, `Mood`, `Person`, `PronType`, `Definite`, `Degree`, `Case`, `NumType`, `NumForm`, `Polarity`, `Voice`. EWT and `mova` have the full set. So for CTeTex the FEATS layer is not "absent" but incomplete. Such gold data can neither be trained on as complete nor used to measure UFeats directly.

**Conditions and exceptions.**
- The most frequent FEATS:
  - NOUN `Number=Sing` 2222, `Number=Plur` 424;
  - VERB `Tense=Past` 286 — participles and past alike;
  - AUX `Number=Sing|Tense=Pres` 183;
  - VERB `Number=Sing` 47.
- VERB without any feature — 385, AUX — 280. These are infinitives and modals: *shall* has «_».
- PROPN have no `Number` (293 of 293), all NOUN have it.

**Examples.** `199` *…The BE shall receive…* — *shall*: AUX, FEATS «_»; *receive*: VERB, «_». In EWT *shall* — `VerbForm=Fin`, *receive* — `VerbForm=Inf`.

**In UD.** General rules from `../../seeds-overview.md` (CTeTex / EWT):

| rule | CTeTex | EWT |
|---|---:|---:|
| `tb.en.art-feats` | 811 / 811 | 0 / 16 365 |
| `tb.en.adj-degree` | 647 / 647 | 0 / 16 789 |
| `tb.en.modal-fin` | 246 / 246 | 0 / 4048 |
| `tb.en.num-feats` | 317 / 317 | 15 / 5051 |
| `tb.en.propn-number` | 293 / 293 | 475 / 16 562 |
| `tb.en.not-polarity` | 52 / 52 | 0 / 2077 |
| `tb.en.fixed-extpos` | 51 / 52 | 0 / 586 |
| `tb.en.expl-there` | 8 / 8 | 0 / 458 |

**Sources.** README `UD_English-CTeTex` («Features: manual native»); `data/raw/ud-docs/docs/treebanks/en_ctetex/index.md` (feature list: ExtPos, Number, Tense, Typo); Hassert et al. 2021, `2021.udw-1.5`.
