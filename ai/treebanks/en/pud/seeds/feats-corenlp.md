# PUD — FEATS from CoreNLP: agreement by form, no `Voice`, no `Case=Gen`, no `Mood=Imp`; *by* without `obl:agent`

**Gist.** Lemmas and FEATS in PUD were assigned by CoreNLP (≈ 2017) and have not been checked since. So they reflect EWT of those years, not 2.18:
- **Number/Person only where visible in the form:** VBZ, *am*, *was/were*. 940 of 1569 finite verbs with Tense lack them.
- **No `Voice=Pass`:** 272 of 272 participles with `aux:pass` lack it. EWT introduced it more widely in 2.13.
- **No `Case=Gen` on possessives:** 260 of 260 in `nmod:poss`. EWT added it in 2.11.
- **Not a single `Mood=Imp`:** 74 infinitive roots have `VerbForm=Inf`. Only 4 of them are real imperatives without a subject (*Fast forward to 2016…*). News and Wiki have few imperatives.
- **`Polarity=Pos`, `Style`, `Abbr=Yes`** — isolated or 0.

The syntax was corrected manually in 2017, so the old norm is there too: the ***by* agent** is `obl`, not `obl:agent`, in 66 of 75.

**Conditions and exceptions.**
- Mass edits by the maintainers (`data.md`) added `ExtPos`, `PronType=Dem` on *there*, `:unmarked`, `nmod:desc`, `Polarity=Neg` on *not* (74 of 74). But not agreement, `Voice` and `Case=Gen`.
- Subordinate *when* are mostly already ADV `advmod` (25). `mark` — only 4 (`tb.en.wh-advmod` 4 of 69).

**Examples.**
- `n01002042` *The new spending is fueled by Clinton’s large bank account.* — *fueled*: without `Voice=Pass`; *account*: `obl`, not `obl:agent`.
- `n01002017` *…in a break from his past rhetoric…* — *his*: without `Case=Gen`.
- `n01001011` *…wrote…* — `Mood=Ind|Tense=Past|VerbForm=Fin`, without Number/Person.

**In UD.**

| rule | PUD | EWT |
|---|---:|---:|
| `tb.en.fin-agree` | 940 / 1569 | 0 / 19 126 |
| `tb.en.part-voice` | 272 / 272 | 0 / 1643 |
| `tb.en.poss-gen` | 260 / 260 | 16 / 3689 |
| `tb.en.obl-agent` | 66 / 75 | 16 / 411 |
| `tb.en.imp-mood` | 4 / 74 | 38 / 3066 |
| `tb.en.wh-advmod` | 4 / 69 | 0 / 1049 |

**Sources.** README `UD_English-PUD` («Morphological features and lemmata were added automatically using Stanford CoreNLP»); EWT README v2.11 (Case), v2.13 (`Voice`, `obl:agent`); UD docs/changes.md` No. 18.
