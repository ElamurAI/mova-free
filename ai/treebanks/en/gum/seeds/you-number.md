# GUM — *you* has Number=Sing or Plur, and the verb agrees with it

**Gist.** GUM distinguishes singular and plural *you*: `Number=Sing` or `Number=Plur` by meaning. A verb with subject *you* takes the same number: *you guys are* — `Number=Plur`. In EWT *you* has no Number, and a verb with *you* always has `Number=Sing|Person=2`. `mova` currently follows EWT.

**Conditions and exceptions.**
- *you* in GUM:
  - `Number=Sing` — 2203 (Nom 1857, Acc 346);
  - `Number=Plur` — 256 (Nom 181, Acc 75).

  In total with Number — 2461 of 2461.
- A verb with Tense with subject *you*:
  - *you*:Sing — verb Sing 819;
  - *you*:Plur — verb Plur 63.

  In EWT: Sing 759, Plur only 6.
- Likewise in the GUM family: GENTLE (*you*: Sing 134, Plur 17), GUMReddit (Sing 203, Plur 6). In ATIS *you* also has Number, but only Sing.
- LittlePrince has Number on *you* 11 times of 85, LinES and ParTUT never.

**Examples.** `GUM_conversation_grounded-5` *You guys are always in trouble.* — *You*: `Number=Plur`, *are*: `Number=Plur|Person=2`. In EWT *are* would have `Number=Sing|Person=2`, and *You* no Number.

**In UD.**

| rule | GUM | EWT |
|---|---:|---:|
| `tb.atis.you-number`: *you* with Number | 2461 / 2461 | 0 / 2768 |
| `tb.gum.you-plur`: verb with subject *you* has `Number=Plur` | 63 / 882 | 6 / 765 |

**Conclusion for `mova`.** The GUM custom is more informative: it distinguishes singular and plural *you*. But it cannot be derived from EWT: EWT does not say how many addressees are being spoken to. By the dialect selection rule (`train/dialects-and-converters.md`: "more consistent and more informative and convertible to the standard without loss") this is a candidate for borrowing in `dialects/mova/seeds/`. In `mova` → EWT it converts without loss: remove Number from *you*, give the verb Sing. Mova decides. For now `mova` = EWT, and the converter removes Number (`../convert.md`).

**Sources.** https://universaldependencies.org/en/feat/Number.html, https://universaldependencies.org/en/pos/PRON.html; README `UD_English-GUM`, changelog 2022-10-21 («Many updates to UPOS and FEATS consistency with EWT» — *you* kept Number); Stanovsky & Tamari 2019, `D19-5549` «Y'all should read this! Identifying Plurality in Second-Person Personal Pronouns in English Texts» (papers/s/st/stanovsky-2019-y-all-should-read-this) — distinguishing singular and plural *you* as a separate task.

```rule
rule: tb.gum.you-plur
what: verb with subject you in the plural (GUM agrees with the number of you; EWT — always Sing)
match: v[feats.VerbForm=Fin, feats.Tense]; s[form=you, rel=nsubj|nsubj:pass, head=v]
require: not v[feats.Number=Plur]
severity: warn
source: EWT 2.18; README UD_English-GUM; counter
```
