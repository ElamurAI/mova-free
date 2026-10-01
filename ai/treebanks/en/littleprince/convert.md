# LittlePrince ↔ mova — converter draft

The rule language is `train/dialects-and-converters.md`. Conventions beyond the specification are in `../seeds-overview.md`. Matches were counted by `en expert-check`: `littleprince → mova` on LittlePrince, `mova → littleprince` on EWT.

LittlePrince is test only, and it mixes two norms (`seeds/stanza-legacy.md`). On the way there the converter reduces everything to EWT. On the way back it reproduces the majority, so divergences are inevitable. We count them rather than hide them.

## littleprince → mova

```convert
rule: littleprince.obl-tmod
what: obl:tmod → obl:unmarked + TemporalNPAdjunct
from: littleprince
to: mova
match: t[rel=obl:tmod]
set: t[rel=obl:unmarked]; t[misc+=TemporalNPAdjunct=Yes]
source: UD 2.15, docs#1028
```

```convert
rule: littleprince.obl-npmod
what: obl:npmod → obl:unmarked
from: littleprince
to: mova
match: t[rel=obl:npmod]
set: t[rel=obl:unmarked]
source: UD 2.15, docs#1028
```

Next, agreement — the same rules as in `../atis/convert.md`:


```convert
rule: littleprince.agree-pron
what: finite verb without Number — number and person from a pronoun subject
from: littleprince
to: mova
match: v[feats.VerbForm=Fin, feats.Tense, !feats.Number]; s[rel=nsubj|nsubj:pass, head=v, upos=PRON, feats.Person]
set: v[feats+=Number=@s]; v[feats+=Person=@s]
source: EWT 2.18 (Number/Person from the subject); UD docs/changes.md No. 18 (2.19)
```

```convert
rule: littleprince.agree-noun
what: finite verb without Number — number from a noun subject, 3rd person
from: littleprince
to: mova
match: v[feats.VerbForm=Fin, feats.Tense, !feats.Number]; s[rel=nsubj|nsubj:pass, head=v, upos=NOUN|PROPN|NUM]
set: v[feats+=Number=@s]; v[feats+=Person=3]
source: EWT 2.18
```

```convert
rule: littleprince.agree-aux-pron
what: aux/cop without Number — from the head's subject (pronoun)
from: littleprince
to: mova
match: h[]; s[rel=nsubj|nsubj:pass, head=h, upos=PRON, feats.Person]; a[rel=aux|cop|aux:pass, head=h, feats.VerbForm=Fin, feats.Tense, !feats.Number]
set: a[feats+=Number=@s]; a[feats+=Person=@s]
source: EWT 2.18 («for aux and cop — the head's subject»)
```

```convert
rule: littleprince.agree-aux-noun
what: aux/cop without Number — from the head's subject (noun)
from: littleprince
to: mova
match: h[]; s[rel=nsubj|nsubj:pass, head=h, upos=NOUN|PROPN|NUM]; a[rel=aux|cop|aux:pass, head=h, feats.VerbForm=Fin, feats.Tense, !feats.Number]
set: a[feats+=Number=@s]; a[feats+=Person=3]
source: EWT 2.18
```

```convert
rule: littleprince.agree-conj
what: conjoined predicate without its own subject — features of the first conjunct
from: littleprince
to: mova
match: h[feats.Number, feats.Person]; v[rel=conj, head=h, feats.VerbForm=Fin, feats.Tense, !feats.Number]
require: none c[rel~nsubj, head=v]
set: v[feats+=Number=@h]; v[feats+=Person=@h]
source: («for conj — the subject of the first conjunct»)
```

```convert
rule: littleprince.agree-relcl
what: predicate of a relative clause with subject that/which — number of the antecedent
from: littleprince
to: mova
match: n[upos=NOUN|PROPN]; v[rel=acl:relcl, head=n, feats.VerbForm=Fin, feats.Tense, !feats.Number]; t[rel=nsubj|nsubj:pass, head=v, feats.PronType=Rel]
set: v[feats+=Number=@n]; v[feats+=Person=3]
source: («for a relative pronoun — the antecedent»)
```

```convert
rule: littleprince.poss-case
what: possessive modifier without Case=Gen — Case=Gen, lemma = form
from: littleprince
to: mova
match: p[upos=PRON, rel=nmod:poss, form=my|your|his|her|its|our|their, feats.Case!=Gen]
set: p[feats+=Case=Gen]; p[lemma=@form]
source: EWT README v2.11 (docs#517)
```

```convert
rule: littleprince.expl-there
what: expletive there — PronType=Dem
from: littleprince
to: mova
match: t[form=there, rel=expl, !feats.PronType]
set: t[feats+=PronType=Dem]
source: EWT README v2.17 (docs#517)
```

```convert
rule: littleprince.not
what: not/n't — Polarity=Neg
from: littleprince
to: mova
match: n[form=not|n't, upos=PART, !feats.Polarity]
set: n[feats+=Polarity=Neg]
source: EWT README v2.15
```

```convert
rule: littleprince.numform-digit
what: number in digits — NumForm=Digit in FEATS
from: littleprince
to: mova
match: n[upos=NUM, suffix=0|1|2|3|4|5|6|7|8|9]
set: n[feats+=NumForm=Digit]; n[misc-=NumForm]
source: EWT README v2.13
```

```convert
rule: littleprince.numform-word
what: number in words — NumForm=Word in FEATS
from: littleprince
to: mova
match: n[upos=NUM, !feats.NumForm]
set: n[feats+=NumForm=Word]; n[misc-=NumForm]
source: EWT README v2.13
```

```convert
rule: littleprince.wh-advmod
what: subordinate when/where/how/why as mark → ADV advmod
from: littleprince
to: mova
match: w[form=when|where|how|why, rel=mark]
set: w[upos=ADV]; w[rel=advmod]
source: EWT README v2.11 (#88)
```

```convert
rule: littleprince.you
what: you without Number
from: littleprince
to: mova
match: p[form=you, upos=PRON, feats.Number]
set: p[feats-=Number]
source: EWT 2.18
```


Matches on LittlePrince: `obl-tmod` 6, `obl-npmod` 3, `agree-pron` 119, `agree-noun` 89, `agree-aux-pron` 57, `agree-aux-noun` 25, `agree-relcl` 11, `agree-conj` 0, `poss-case` 111, `expl-there` 15, `not` 65, `numform-digit` 15, `numform-word` 127 (142 − 15), `wh-advmod` 17, `you` 10.

## mova → littleprince


```convert
rule: mova.littleprince.agree-pres
what: present without -s, except am — without Number/Person
from: mova
to: littleprince
match: v[feats.VerbForm=Fin, feats.Tense=Pres, suffix!=s, form!=am|'m]
set: v[feats-=Number]; v[feats-=Person]
source: seeds/stanza-legacy.md
```

```convert
rule: mova.littleprince.agree-past
what: past, except be — without Number/Person
from: mova
to: littleprince
match: v[feats.VerbForm=Fin, feats.Tense=Past, lemma!=be]
set: v[feats-=Number]; v[feats-=Person]
source: seeds/stanza-legacy.md
```

```convert
rule: mova.littleprince.poss
what: possessive — without Case=Gen
from: mova
to: littleprince
match: p[upos=PRON, rel=nmod:poss, feats.Case=Gen]
set: p[feats-=Case]
source: seeds/stanza-legacy.md
```

```convert
rule: mova.littleprince.lemma-his
what: his → lemma he (old norm, majority)
from: mova
to: littleprince
match: p[form=his, upos=PRON, rel=nmod:poss]
set: p[lemma=he]
source: seeds/stanza-legacy.md
```

```convert
rule: mova.littleprince.lemma-your
what: your → lemma you (old norm, majority)
from: mova
to: littleprince
match: p[form=your, upos=PRON, rel=nmod:poss]
set: p[lemma=you]
source: seeds/stanza-legacy.md
```

```convert
rule: mova.littleprince.lemma-their
what: their → lemma they (old norm, majority)
from: mova
to: littleprince
match: p[form=their, upos=PRON, rel=nmod:poss]
set: p[lemma=they]
source: seeds/stanza-legacy.md
```

```convert
rule: mova.littleprince.lemma-our
what: our → lemma we (old norm, majority)
from: mova
to: littleprince
match: p[form=our, upos=PRON, rel=nmod:poss]
set: p[lemma=we]
source: seeds/stanza-legacy.md
```

```convert
rule: mova.littleprince.lemma-her
what: her → lemma she (old norm, majority)
from: mova
to: littleprince
match: p[form=her, upos=PRON, rel=nmod:poss]
set: p[lemma=she]
source: seeds/stanza-legacy.md
```

```convert
rule: mova.littleprince.there
what: expletive there — without PronType
from: mova
to: littleprince
match: t[form=there, rel=expl]
set: t[feats-=PronType]
source: seeds/stanza-legacy.md
```

```convert
rule: mova.littleprince.not
what: not — without Polarity (65 of 69)
from: mova
to: littleprince
match: n[form=not|n't, upos=PART]
set: n[feats-=Polarity]
source: seeds/stanza-legacy.md
```

```convert
rule: mova.littleprince.when
what: subordinate when on advcl — SCONJ mark
from: mova
to: littleprince
match: v[rel=advcl]; w[form=when, rel=advmod, head=v, before=v]
set: w[upos=SCONJ]; w[rel=mark]
source: seeds/stanza-legacy.md
```


```convert-todo
rule: mova.littleprince.numform
what: NumForm from FEATS into MISC
from: mova
to: littleprince
match: n[feats.NumForm]
set: n[misc+=NumForm=@feats.NumForm]; n[feats-=NumForm]
source: seeds/data.md
```

Needs a "copy a feature value into MISC" action (`misc+=K=@feats.X`). For now it can be done with two rules with constant values — `feats.NumForm=Digit` → `misc+=NumForm=Digit` and likewise `Word`. Kept as a marker that the general action is missing.

Matches on EWT: `agree-pres` 6308, `agree-past` 4493, `poss` 3673, `lemma-his` 450, `lemma-your` 813, `lemma-their` 476, `lemma-our` 429, `lemma-her` 127, `there` 458, `not` 2077, `when` 401.

## Round trip littleprince → mova → littleprince

The treebank mixes two norms, so on the way there everything becomes the EWT norm, and on the way back the majority norm. The losses equal the minority of each pair:
- **`:tmod`/`:npmod`** — 9. On the way back `:unmarked` remains (19 versus 9), so 9 DEPREL divergences. With a condition on MISC (`TemporalNPAdjunct`) `obl:tmod` can be restored: then 3 (`npmod`).
- **Agreement.** On the way back we remove Number from everything except *-s*, *am* and past *be*. The students added Number on some verbs: past non-*be* — 75 (*say* 6, *do* 5, *have* 4…), present non-3sg except *am* — 33 (*have* 8, *are* 4, *do* 3…). Together 108 FEATS divergences, versus 314 verbs without Number, so on the way back the majority custom.
- **Possessive lemmas.** On the way back — the old norm *his* → *he*. The minority *his* → *his* (5), *your* → *your* (2) and *my* → *my* (all 41: here the old norm coincides with the form) will diverge: ≈ 7 lemmas.
- **`Case=Gen`.** On the way back we remove it from all, but 12 had it: 12 FEATS divergences.
- **`Polarity=Neg`.** On the way back we remove it from all, but 4 had it: 4.
- **`NumForm`** — until there is `misc+=…=@feats`, it stays in FEATS on the way back: 157 FEATS and MISC divergences. With two constant rules — 0.
- ***when*** on `advcl` — lossless. *how* SCONJ `mark` (4) becomes `advmod` on the way back.
- ***you* with Number** (10) is not restored on the way back: 10.

Total ≈ 310 divergences per 6852 words: FEATS ≈ 96%, DEPREL and UPOS ≥ 99.8%. For a test this is acceptable, because the divergences are the minority within the treebank's own mixed norms.
