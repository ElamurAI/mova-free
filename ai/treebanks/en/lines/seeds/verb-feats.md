# LinES — verb features by form: Number/Person only where visible; modals with `Mood`; no gerund

**Gist.** LinES verb FEATS are derived from the morphological XPOS (`xpos-lines.md`). So they describe the form, not agreement:
- Number and Person are present only where the form shows them: *-s* in 3sg (`Pres` + `Sing|3`), *am*, *was/were*;
- VBP (*they go*) and the past tense of ordinary verbs (*went*) — without Number and Person;
- modals — `Mood=Ind|VerbForm=Fin`, not only `VerbForm=Fin` as in EWT;
- *-ing* forms — always `Tense=Pres|VerbForm=Part`; there is not a single `VerbForm=Ger`.

EWT and `mova` set Number/Person on every finite verb from the subject. A modal there has only `VerbForm=Fin`, and since 2.14 the gerund is separated from the participle.

**Conditions and exceptions.**
- Finite verbs with Tense — 9608, of which 6607 without Number:
  - present without Number — 1609 (all VBP);
  - past without Number — 4996.
- Past with Number — 1360, and all of them are *be* (*was/were*).
- Present non-3sg with Number — 52: *am* 41, *'m* 9 and 2 more.
- 1295 modals, all with `Mood=Ind`. 42 of them also have `Tense=Past` (*could, would*), 5 — `Tense=Pres`.
- *-ing* with `Tense=Pres|VerbForm=Part` — 1973:
  - `advcl` 558;
  - `acl` 303;
  - `root` 299;
  - `conj` 276;
  - `xcomp` 187;
  - `ccomp` 96.

  Some of them would be `VerbForm=Ger` in EWT (EWT: `acl`, `xcomp`, `csubj`…).

**Examples.**
- `en_lines-ud-dev-doc1-3178` *…the account name and password were validated…* — *were*: `Number=Plur|Person=3`. And *verifies* in the same sentence — `Sing|3`: a form with *-s*.
- `en_lines-ud-dev-doc1-3179` *…the user must…* — *must*: `Mood=Ind|VerbForm=Fin`.

**In UD.**

| rule | LinES | EWT |
|---|---:|---:|
| `tb.en.fin-agree`: finite with Tense without Number/Person | 6607 / 9608 | 0 / 19 126 |
| `tb.atis.vbp-bare`: present without Number | 1609 / 1609 | 0 / 0 |
| `tb.en.modal-fin`: modal not only `VerbForm=Fin` | 1295 / 1295 | 0 / 4048 |
| `VerbForm=Ger` (`grep`) | 0 | 1230 |
| `tb.en.part-voice`: participle with `aux:pass` without `Voice=Pass` | 5 / 842 | 0 / 1643 |

LinES shares this "feature only from form" custom with PUD, LittlePrince, ATIS and partly ParTUT (`../../seeds-overview.md`). UD 2.19 (UD docs/changes.md` No. 18) clarifies: a feature underspecified by the form is taken from context. So the EWT custom is normative, and the LinES custom is incomplete.

**Sources.** `data/raw/ud-docs/docs/treebanks/en_lines/index.md`; UD docs/changes.md` No. 18; https://universaldependencies.org/en/feat/VerbForm.html, `feat/Mood.md`; EWT README v2.14 (Ger versus Part, #305).
