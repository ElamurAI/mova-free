# Pronouns — features of independent possessives, no `Case`, passive without `nsubj:pass`

**Gist.** Pronouns was annotated manually in 2019 and has changed little since. So several customs diverge from EWT 2.18:
1. **Independent possessives** *mine, yours, theirs* have `Gender=Neut`, and *theirs* has `Number=Sing` (singular *they*, for which the treebank was created). In EWT *mine, yours, theirs* have no Gender, and *theirs* is `Number=Plur`.
2. **No `Case` at all.** It was removed in 2.8 with the explanation that English treebanks do not use it. Since then EWT has introduced it (2.11–2.17): `Case=Nom/Acc` on personal pronouns, `Case=Gen` on dependent possessives. *it* in Pronouns lacks `Case` 40 times out of 40. Independent *hers, mine* have no `Case` in EWT either; here they agree.
3. **Passive without `nsubj:pass` and `Voice`:** 25 of 25 passives with `aux:pass` have an `nsubj` subject, 20 of 20 participles lack `Voice=Pass`.
4. **Incomplete agreement:** 90 of 315 finite verbs with Tense lack Number/Person. The other 225 agree with the subject, as in EWT, even past ones (*cleaned* 35, *sold* 15, *drove* 10). *is* has `Number=Sing` without `Person` 10 times.
5. **Adjectives without `Degree`:** *fresh* 10, *nice* 5 (15 of 20).

**Conditions and exceptions.** The lemmas of independent possessives already follow the new EWT norm: *hers* → *her*, *mine* → *my*, *yours* → *your*, *theirs* → *their* (57 each).

**Examples.**
- `166` *Hers was cleaned.* — *Hers*: `nsubj`, not `nsubj:pass`; *cleaned*: without `Voice=Pass`.
- `1` *It is hers.* — *It*: `Number=Sing|Person=3|PronType=Prs`, without `Case` and without the `Gender=Neut` that EWT puts on *it*.
- `86` *Hers is nice.* — *nice*: ADJ without `Degree`.

**In UD.**

| rule | Pronouns | EWT |
|---|---:|---:|
| `tb.en.pass-subj` | 25 / 25 | 0 / 1405 |
| `tb.en.part-voice` | 20 / 20 | 0 / 1643 |
| `tb.en.pron-case` | 40 / 40 | 0 / 14 842 |
| `tb.en.fin-agree` | 90 / 315 | 0 / 19 126 |
| `tb.en.adj-degree` | 15 / 20 | 0 / 16 789 |
| `tb.pronouns.indep-gender`: *mine/yours/theirs* with `Gender` | 171 / 285 | 0 / 40 |

**Sources.** README `UD_English-Pronouns`, Changelog v2.8 («Removed Case=Gen…»); EWT README v2.11 (docs#517), v2.13 (`Voice=Pass`), v2.17 (#589, Case on *it/you*); https://universaldependencies.org/en/pos/PRON.html, `feat/Case.md` (Gen — dependent only).

```rule
rule: tb.pronouns.indep-gender
what: independent possessive mine/yours/theirs/hers/his with Gender (EWT: only hers/his)
match: p[upos=PRON, feats.Poss=Yes, form=mine|yours|theirs|ours|hers|his, rel!=nmod:poss]
require: not p[feats.Gender] or p[form=hers|his]
severity: warn
source: EWT 2.18; README UD_English-Pronouns; counter
```
