# GENTLE ↔ mova — converter draft

GENTLE was annotated with the GUM pipeline, so the converter is the same too: the rules of `../gum/convert.md` with `from: gentle`. Seeds — `seeds/gum-family.md` and `../gum/seeds/`. The rule language is `train/dialects-and-converters.md`, conventions are in `../seeds-overview.md`. Matches were counted by `en expert-check`: `gentle → mova` on GENTLE, `mova → gentle` on EWT, as for GUM.

GENTLE is test only. The main direction is `mova → gentle`: bring the `en` output to GUM customs, so that UFeats and LAS measure quality rather than the difference between dialects.

## gentle → mova

```convert-todo
rule: gentle.you-guys
what: you (dep) ← guys → you — head, guys — nmod:unmarked
from: gentle
to: mova
match: n[upos=NOUN|ADJ|PROPN]; y[form=you, rel=dep, head=n]
set: y[head=@n]; y[rel=@n]; n[head=y]; n[rel=nmod:unmarked]
source: EWT README v2.17 (#436)
```

```convert
rule: gentle.obl-fragment
what: obl on a noun without a copula → nmod
from: gentle
to: mova
match: h[upos=NOUN|PROPN|PRON|NUM]; o[rel=obl, head=h]
require: none c[rel=cop, head=h]
set: o[rel=nmod]
source: UD 2.18 nmod/obl (obl-should-be-nmod); EWT: What about X — nmod
```

```convert
rule: gentle.obl-unmarked-fragment
what: obl:unmarked on a noun without a copula → nmod:unmarked
from: gentle
to: mova
match: h[upos=NOUN|PROPN|PRON|NUM]; o[rel=obl:unmarked, head=h]
require: none c[rel=cop, head=h]
set: o[rel=nmod:unmarked]
source: UD 2.18 nmod/obl
```

```convert
rule: gentle.you-verb
what: verb with subject you — Number=Sing (EWT)
from: gentle
to: mova
match: v[feats.Number=Plur, feats.Person=2]; s[form=you, rel=nsubj|nsubj:pass, head=v]
set: v[feats+=Number=Sing]
source: EWT 2.18
```

```convert
rule: gentle.you-aux
what: aux/cop on a predicate with subject you — Number=Sing
from: gentle
to: mova
match: h[]; s[form=you, rel=nsubj|nsubj:pass, head=h]; a[rel=aux|cop|aux:pass, head=h, feats.Number=Plur, feats.Person=2]
set: a[feats+=Number=Sing]
source: EWT 2.18
```

```convert
rule: gentle.you
what: you without Number
from: gentle
to: mova
match: p[form=you|yourself, upos=PRON, feats.Number]
require: p[feats.Person=2]
set: p[feats-=Number]
source: EWT 2.18
```

```convert
rule: gentle.adv-degree
what: Degree=Pos only on adverbs with degrees (EWT list)
from: gentle
to: mova
match: a[upos=ADV, feats.Degree=Pos, lemma!=well|far|soon|long|hard|early|late|little|close|high|fast|badly|low]
set: a[feats-=Degree]
source: EWT 2.18
```

```convert
rule: gentle.such
what: such ADJ — Degree=Pos
from: gentle
to: mova
match: a[upos=ADJ, lemma=such, !feats.Degree]
set: a[feats+=Degree=Pos]
source: EWT 2.18 (88 of 88)
```

```convert
rule: gentle.add
what: URL — ADD
from: gentle
to: mova
match: u[xpos=NNP, prefix=http|www.]
set: u[xpos=ADD]
source: Webtext addendum (EWT: ADD)
```

```convert
rule: gentle.nfp
what: emoticon — NFP
from: gentle
to: mova
match: e[xpos=SYM, form=:)|:(|;)|:-)|:-(|:D|:P|;-)]
set: e[xpos=NFP]
source: Webtext addendum (EWT: NFP)
```

```convert
rule: gentle.afx
what: prefix before a hyphen — AFX
from: gentle
to: mova
match: p[xpos=NN|NNP|JJ|IN|VB|RB, form=post|non|pre|mid|over|anti|multi|co|re|inter|intra|sub|semi]; h[xpos=HYPH, next=p]
set: p[xpos=AFX]
source: BioMedical addendum (AFX); EWT 2.18
```

Matches on GENTLE: `obl-fragment` 16, `obl-unmarked-fragment` 5, `you-verb` 7, `you-aux` 4, `you` 156, `adv-degree` 345, `such` 11, `add` 9, `nfp` 0, `afx` 3.

*you guys* (`convert-todo`) in GENTLE — 2 cases. `dep` on numbers and dictionary marks (*t*, *s*, *ipa*) is not converted, as in GUM: EWT has no stable counterpart.

## mova → gentle

```convert-todo
rule: mova.gentle.you-guys
what: you ← guys (nmod:unmarked) → guys — head, you — dep
from: mova
to: gentle
match: y[form=you]; n[rel=nmod:unmarked, head=y, next=y]
set: n[head=@y]; n[rel=@y]; y[head=n]; y[rel=dep]
source: ../gum/seeds/dep.md
```

```convert
rule: mova.gentle.what-about
what: What about X — X as obl
from: mova
to: gentle
match: h[form=what, head=0]; o[rel=nmod, head=h, after=h]; c[form=about, rel=case, head=o]
set: o[rel=obl]
source: ../gum/seeds/obl-fragment.md
```

```convert
rule: mova.gentle.you
what: you — Number=Sing (plural cannot be recovered from mova)
from: mova
to: gentle
match: p[form=you|yourself, upos=PRON, feats.Person=2]
set: p[feats+=Number=Sing]
source: ../gum/seeds/you-number.md
```

```convert
rule: mova.gentle.adv-degree
what: Degree=Pos on all adverbs except pronominal ones and the GUM list without degrees
from: mova
to: gentle
match: a[upos=ADV, !feats.Degree, !feats.PronType, lemma!=so|just|also|very|even|as|only|about|however|all|already|over|instead|maybe|out|rather|pretty|once|yet|quite|thus|twice|e.g.|anyway|somewhat|i.e.|c.|anyways|namely|merely]
set: a[feats+=Degree=Pos]
source: ../gum/seeds/adv-degree.md (GUM: ADV without Degree mostly — only these lemmas)
```

```convert
rule: mova.gentle.such
what: such ADJ without Degree
from: mova
to: gentle
match: a[upos=ADJ, lemma=such]
set: a[feats-=Degree]
source: ../gum/seeds/adv-degree.md
```

```convert
rule: mova.gentle.add
what: ADD → NNP
from: mova
to: gentle
match: u[xpos=ADD]
set: u[xpos=NNP]
source: ../gum/seeds/xpos-web.md
```

```convert
rule: mova.gentle.nfp
what: NFP → SYM
from: mova
to: gentle
match: e[xpos=NFP]
set: e[xpos=SYM]
source: ../gum/seeds/xpos-web.md
```

```convert
rule: mova.gentle.afx
what: AFX → NN
from: mova
to: gentle
match: p[xpos=AFX]
set: p[xpos=NN]
source: ../gum/seeds/xpos-web.md
```

Matches on EWT — as for GUM: `what-about` 7, `you` 2806, `adv-degree` 5151, `such` 88, `add` 475, `nfp` 499, `afx` 75.

## Round trip gentle → mova → gentle

As for GUM (`../gum/convert.md`), with smaller numbers:
- **lossless:** *such* (11), `ADD` (9), *What about*;
- **lossy:**
  - the number of *you* — 17 *you*:Plur and 11 verbs;
  - `obl` in fragments — 21 there, back only *What about*;
  - `Degree` on adverbs — ≈ 10–30 of 736 ADV;
  - `AFX` — up to 3.

Total ≈ 99.8% FEATS and ≈ 99.9% DEPREL.
