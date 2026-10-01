# ATIS — function words: UPOS and FEATS from the PTB tag without context

**Gist.** The ATIS rules convert the PTB tag to UPOS and FEATS regardless of the word's role:
- DT → `PronType=Art` for all determiners, even demonstrative *this, that*;
- WDT, WP, WRB → `PronType=Int,Rel`, without splitting into interrogative and relative;
- PRP$ → DET;
- IN → ADP, even when the word is a subordinating conjunction (*if, that*).

In EWT and `mova` UPOS and features depend on role: *that* as a conjunction is SCONJ, *which* in a relative clause is `PronType=Rel`, *my* is PRON with `Poss=Yes|Case=Gen`.

**Conditions and exceptions.**
- Articles have `PronType=Art` but no `Definite`: 3341 of 3341.
- *not* — ADV without `Polarity=Neg` (4 of 4). In EWT it is PART with `Polarity=Neg`.
- NUM has only `NumType=Card`; there is no `NumForm` anywhere (1191 of 1191).
- Adverbs have `Degree=Pos` where EWT does not put it: *now, back, then, here* — 132 of 132 from the list of invariable ones.
- `fixed` heads have no `ExtPos`: 22 of 22.
- *what* as a determiner — DET with `PronType=Int,Rel`, 532 times.

**Examples.**
- `0083.dev` *a flight that goes from tampa…*: *that* — ADP, `mark` (on relatives — `relative-that-mark.md`).
- `0057.test` *please list me the flights and their cost…*: *their* — DET.
- *this/that/these/those* with `PronType=Art` — 67 times: *that* 40, *this* 18, *these* 5, *those* 4.

**In UD.** Counters — the general rules from `../../seeds-overview.md`, violations / matches:

| rule | ATIS | EWT |
|---|---:|---:|
| `tb.en.art-feats`: article without `PronType=Art` or `Definite` | 3341 / 3341 | 0 / 16 365 |
| `tb.en.sconj-mark`: *that, if, because…* in `mark` not SCONJ | 284 / 284 | 0 / 2419 |
| `tb.en.poss-pron`: *my, your, their…* in `nmod:poss` not PRON | 28 / 28 | 1 / 3468 |
| `tb.en.dem-pron`: *this, that…* as a noun not PRON | 90 / 125 | 0 / 1531 |
| `tb.en.num-feats`: NUM without `NumType` or `NumForm` | 1191 / 1191 | 15 / 5051 |
| `tb.en.adv-degree`: invariable adverb with `Degree` | 132 / 132 | 0 / 5076 |
| `tb.en.fixed-extpos`: `fixed` head without `ExtPos` | 22 / 22 | 0 / 586 |

`PronType=Int,Rel` is on 1793 words (`grep -c 'PronType=Int,Rel'`); EWT has no such pair. The rule `tb.atis.int-rel` below catches them without a comma in the value, because the rule language does not accept a comma.

**Sources.** Cesur et al. 2024, `2024.bucc-1.11`, §3 («the Penn POS-tags were automatically converted into UD-style tags… by a rule-based algorithm»); https://universaldependencies.org/en/pos/SCONJ.html, `pos/DET.md`, `pos/PRON.md`, `feat/PronType.md`, `feat/Definite.md`; EWT README v2.15 (`Polarity=Neg` on *not*, `ExtPos`), v2.13 (DET and NUM features).

```rule
rule: tb.atis.int-rel
what: ATIS — PronType=Int,Rel (the feature is present, but is none of the simple values)
match: w[feats.PronType, feats.PronType!=Int|Rel|Prs|Art|Dem|Ind|Neg|Tot|Rcp|Emp]
require: w[feats.PronType=Int|Rel]
severity: warn
source: README UD_English-Atis; counter
```

```rule
rule: tb.atis.dem-art
what: ATIS — demonstrative this/that/these/those with PronType=Art (DT → Art)
match: d[form=this|that|these|those, feats.PronType=Art]
require: d[feats.PronType=Dem]
severity: warn
source: UD en feat/PronType (Dem); counter
```
