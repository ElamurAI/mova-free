# GUMReddit ↔ mova — converter draft

GUMReddit was annotated with the GUM pipeline. The rules are the same as in `../gum/convert.md`, with `from: gumreddit`. The rule language is `train/dialects-and-converters.md`, conventions are in `../seeds-overview.md`.

**Text first.** FORM and LEMMA in the release are «_» (`seeds/data.md`). Rules with `form=` and `lemma=` (*you*, *such*, *what about*, `ADD`, `NFP`, `AFX`) work only after `get_text.py`. Without text only the feature-based rules apply: `you-feat`, `you-verb-feat`, `obl-fragment`. Without lemmas `adv-degree` would remove `Degree=Pos` from all adverbs, including *well*, *far*, because the condition `lemma!=…` holds on «_». So do not run it without text.

Matches were counted by `en expert-check` on the release without text: `gumreddit → mova` on GUMReddit, `mova → gumreddit` on EWT.

## gumreddit → mova

```convert
rule: gumreddit.you-feat
what: 2nd person pronoun without Number — by features, when there is no text
from: gumreddit
to: mova
match: p[upos=PRON, feats.Person=2, feats.PronType=Prs, !feats.Poss, !feats.Reflex, feats.Number]
set: p[feats-=Number]
source: EWT 2.18; seeds/gum-family.md
```

```convert
rule: gumreddit.you-verb-feat
what: 2nd person plural verb → Sing (EWT), by features
from: gumreddit
to: mova
match: v[feats.VerbForm=Fin, feats.Person=2, feats.Number=Plur]
set: v[feats+=Number=Sing]
source: EWT 2.18
```

```convert-todo
rule: gumreddit.you-guys
what: you (dep) ← guys → you — head, guys — nmod:unmarked
from: gumreddit
to: mova
match: n[upos=NOUN|ADJ|PROPN]; y[form=you, rel=dep, head=n]
set: y[head=@n]; y[rel=@n]; n[head=y]; n[rel=nmod:unmarked]
source: EWT README v2.17 (#436)
```

```convert
rule: gumreddit.obl-fragment
what: obl on a noun without a copula → nmod
from: gumreddit
to: mova
match: h[upos=NOUN|PROPN|PRON|NUM]; o[rel=obl, head=h]
require: none c[rel=cop, head=h]
set: o[rel=nmod]
source: UD 2.18 nmod/obl (obl-should-be-nmod); EWT: What about X — nmod
```

```convert
rule: gumreddit.obl-unmarked-fragment
what: obl:unmarked on a noun without a copula → nmod:unmarked
from: gumreddit
to: mova
match: h[upos=NOUN|PROPN|PRON|NUM]; o[rel=obl:unmarked, head=h]
require: none c[rel=cop, head=h]
set: o[rel=nmod:unmarked]
source: UD 2.18 nmod/obl
```

```convert
rule: gumreddit.you-verb
what: verb with subject you — Number=Sing (EWT)
from: gumreddit
to: mova
match: v[feats.Number=Plur, feats.Person=2]; s[form=you, rel=nsubj|nsubj:pass, head=v]
set: v[feats+=Number=Sing]
source: EWT 2.18
```

```convert
rule: gumreddit.you-aux
what: aux/cop on a predicate with subject you — Number=Sing
from: gumreddit
to: mova
match: h[]; s[form=you, rel=nsubj|nsubj:pass, head=h]; a[rel=aux|cop|aux:pass, head=h, feats.Number=Plur, feats.Person=2]
set: a[feats+=Number=Sing]
source: EWT 2.18
```

```convert
rule: gumreddit.you
what: you without Number
from: gumreddit
to: mova
match: p[form=you|yourself, upos=PRON, feats.Number]
require: p[feats.Person=2]
set: p[feats-=Number]
source: EWT 2.18
```

```convert
rule: gumreddit.adv-degree
what: Degree=Pos only on adverbs with degrees (EWT list)
from: gumreddit
to: mova
match: a[upos=ADV, feats.Degree=Pos, lemma!=well|far|soon|long|hard|early|late|little|close|high|fast|badly|low]
set: a[feats-=Degree]
source: EWT 2.18
```

```convert
rule: gumreddit.such
what: such ADJ — Degree=Pos
from: gumreddit
to: mova
match: a[upos=ADJ, lemma=such, !feats.Degree]
set: a[feats+=Degree=Pos]
source: EWT 2.18 (88 of 88)
```

```convert
rule: gumreddit.add
what: URL — ADD
from: gumreddit
to: mova
match: u[xpos=NNP, prefix=http|www.]
set: u[xpos=ADD]
source: Webtext addendum (EWT: ADD)
```

```convert
rule: gumreddit.nfp
what: emoticon — NFP
from: gumreddit
to: mova
match: e[xpos=SYM, form=:)|:(|;)|:-)|:-(|:D|:P|;-)]
set: e[xpos=NFP]
source: Webtext addendum (EWT: NFP)
```

```convert
rule: gumreddit.afx
what: prefix before a hyphen — AFX
from: gumreddit
to: mova
match: p[xpos=NN|NNP|JJ|IN|VB|RB, form=post|non|pre|mid|over|anti|multi|co|re|inter|intra|sub|semi]; h[xpos=HYPH, next=p]
set: p[xpos=AFX]
source: BioMedical addendum (AFX); EWT 2.18
```

Matches on the release without text: `you-feat` 210, `you-verb-feat` 0, `obl-fragment` 5, `obl-unmarked-fragment` 0. Rules with `form=`/`lemma=` give 0, except `adv-degree`: 502, because on the lemma «_» the condition `lemma!=…` holds. After `get_text.py` the numbers will be comparable to GUM per 16k words.

## mova → gumreddit

```convert
rule: mova.gumreddit.you-feat
what: 2nd person pronoun — Number=Sing, by features, when there is no text
from: mova
to: gumreddit
match: p[upos=PRON, feats.Person=2, feats.PronType=Prs, !feats.Poss, !feats.Reflex, !feats.Number]
set: p[feats+=Number=Sing]
source: seeds/gum-family.md
```

```convert-todo
rule: mova.gumreddit.you-guys
what: you ← guys (nmod:unmarked) → guys — head, you — dep
from: mova
to: gumreddit
match: y[form=you]; n[rel=nmod:unmarked, head=y, next=y]
set: n[head=@y]; n[rel=@y]; y[head=n]; y[rel=dep]
source: ../gum/seeds/dep.md
```

```convert
rule: mova.gumreddit.what-about
what: What about X — X as obl
from: mova
to: gumreddit
match: h[form=what, head=0]; o[rel=nmod, head=h, after=h]; c[form=about, rel=case, head=o]
set: o[rel=obl]
source: ../gum/seeds/obl-fragment.md
```

```convert
rule: mova.gumreddit.you
what: you — Number=Sing (plural cannot be recovered from mova)
from: mova
to: gumreddit
match: p[form=you|yourself, upos=PRON, feats.Person=2]
set: p[feats+=Number=Sing]
source: ../gum/seeds/you-number.md
```

```convert
rule: mova.gumreddit.adv-degree
what: Degree=Pos on all adverbs except pronominal ones and the GUM list without degrees
from: mova
to: gumreddit
match: a[upos=ADV, !feats.Degree, !feats.PronType, lemma!=so|just|also|very|even|as|only|about|however|all|already|over|instead|maybe|out|rather|pretty|once|yet|quite|thus|twice|e.g.|anyway|somewhat|i.e.|c.|anyways|namely|merely]
set: a[feats+=Degree=Pos]
source: ../gum/seeds/adv-degree.md (GUM: ADV without Degree mostly — only these lemmas)
```

```convert
rule: mova.gumreddit.such
what: such ADJ without Degree
from: mova
to: gumreddit
match: a[upos=ADJ, lemma=such]
set: a[feats-=Degree]
source: ../gum/seeds/adv-degree.md
```

```convert
rule: mova.gumreddit.add
what: ADD → NNP
from: mova
to: gumreddit
match: u[xpos=ADD]
set: u[xpos=NNP]
source: ../gum/seeds/xpos-web.md
```

```convert
rule: mova.gumreddit.nfp
what: NFP → SYM
from: mova
to: gumreddit
match: e[xpos=NFP]
set: e[xpos=SYM]
source: ../gum/seeds/xpos-web.md
```

```convert
rule: mova.gumreddit.afx
what: AFX → NN
from: mova
to: gumreddit
match: p[xpos=AFX]
set: p[xpos=NN]
source: ../gum/seeds/xpos-web.md
```

Matches on EWT — as for GUM, plus `you-feat` 2763: `what-about` 7, `you` 2806, `adv-degree` 5151, `such` 88, `add` 475, `nfp` 499, `afx` 75.

## Round trip gumreddit → mova → gumreddit

- **Without text.** Only the feature-based rules apply. `you-feat` removes Number from 210 pronouns on the way there, `mova.gumreddit.you-feat` sets Sing on the way back. So only the 6 *you*:Plur will diverge. `obl-fragment` — 5 there, not restored on the way back. Together ≈ 11 divergences per 16,364 words.
- **With text** — as in GUM (`../gum/convert.md`): losses in the number of *you* (6 plural), in `Degree` on adverbs (dozens of 1105 ADV) and in `obl` in fragments (5).
