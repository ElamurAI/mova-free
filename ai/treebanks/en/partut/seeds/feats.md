# ParTUT — FEATS without `Case`, `Voice`, `NumForm`; modals with `Person=3|Tense`; *are* with Number without Person

**Gist.** ParTUT FEATS were converted from TUT's Italian morphology. So the feature set is Italian in spirit:
- **No `Case` anywhere.** The pronouns *I, he, they, them* have only `Number|Person|PronType`: 1153 of 1153 personal pronouns without `Case`.
- **No `Voice`.** 472 of 472 participles with `aux:pass` without `Voice=Pass`.
- **No `NumForm`** (817 of 832 NUM).
- **PROPN without Number** — 2148 of 2230.
- **Modals** — `Mood=Ind|Person=3|Tense=Pres|VerbForm=Fin` (*shall, can, will, may*): 690 of 690. In EWT MD has only `VerbForm=Fin`.
- **Verb agreement — halfway.** *is, has* — `Number=Sing|Person=3`. *are* (222) and *have* (125) — `Number=Plur` without `Person`. The past tense has no Number and Person, except *was/were*.

**Conditions and exceptions.**
- Finite verbs with Tense without Number/Person — 2302 of 3689 (`tb.en.fin-agree`).
- *not* has `Polarity=Neg` (235 of 235) — here ParTUT is already like EWT 2.15.
- `Mood=Sub` — 18: more than in GUM (27 on a five times larger volume). Probably inherited from the Italian `congiuntivo`.

**Examples.**
- `en_partut-ud-72` *You have requested a debate…* — *You*: `Person=2|PronType=Prs`, without `Case`; *have*: without Person.
- `en_partut-ud-7` *The work … is provided under the terms…* — *provided*: `Tense=Past|VerbForm=Part`, without `Voice=Pass`.

**In UD.**

| rule | ParTUT | EWT |
|---|---:|---:|
| `tb.en.pron-case` | 1153 / 1153 | 0 / 14 842 |
| `tb.en.part-voice` | 472 / 472 | 0 / 1643 |
| `tb.en.num-feats` | 817 / 832 | 15 / 5051 |
| `tb.en.propn-number` | 2148 / 2230 | 475 / 16 562 |
| `tb.en.modal-fin` | 690 / 690 | 0 / 4048 |
| `tb.en.fin-agree` | 2302 / 3689 | 0 / 19 126 |

**Sources.** README `UD_English-ParTUT` («Features: converted with corrections»); https://universaldependencies.org/en/feat/Case.html, `feat/Voice.md`, `pos/AUX_.md`; UD docs/changes.md` No. 18 (Number/Person from context).
