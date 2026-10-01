# GUM ↔ mova — converter draft

The rule language is `train/dialects-and-converters.md`. Conventions beyond the specification are in `../seeds-overview.md`. Matches were counted by `en expert-check`: `gum → mova` on all parts of GUM, `mova → gum` on EWT.

GUM is the large treebank closest to EWT: since 2021 it has deliberately been converging with EWT. So the converter is short. GUMReddit and GENTLE were annotated with the same pipeline; their converters (`../gumreddit/convert.md`, `../gentle/convert.md`) repeat these rules with a different `from`.

## gum → mova

### 1. *You guys* (`seeds/dep.md`)

```convert-todo
rule: gum.you-guys
what: you (dep) ← guys → you — head, guys — nmod:unmarked
from: gum
to: mova
match: n[upos=NOUN|ADJ|PROPN]; y[form=you, rel=dep, head=n]
set: y[head=@n]; y[rel=@n]; n[head=y]; n[rel=nmod:unmarked]
source: EWT README v2.17 (#436)
```

Needs a `rel=@v` action — copy another node's relation. The workaround for now is one rule per role of *guys*: `nsubj`, `obj`, `vocative`, `root`, `conj`. Dependents of *guys* (*all you guys*) stay with it, except those standing before *you*: they also have to be reattached to *you*. 56 cases.

### 2. `obl` in fragments (`seeds/obl-fragment.md`)

```convert
rule: gum.obl-fragment
what: obl on a noun without a copula → nmod
from: gum
to: mova
match: h[upos=NOUN|PROPN|PRON|NUM]; o[rel=obl, head=h]
require: none c[rel=cop, head=h]
set: o[rel=nmod]
source: UD 2.18 nmod/obl (obl-should-be-nmod); EWT: What about X — nmod
```

```convert
rule: gum.obl-unmarked-fragment
what: obl:unmarked on a noun without a copula → nmod:unmarked
from: gum
to: mova
match: h[upos=NOUN|PROPN|PRON|NUM]; o[rel=obl:unmarked, head=h]
require: none c[rel=cop, head=h]
set: o[rel=nmod:unmarked]
source: UD 2.18 nmod/obl
```

### 3. The number of *you* (`seeds/you-number.md`)

```convert
rule: gum.you-verb
what: verb with subject you — Number=Sing (EWT)
from: gum
to: mova
match: v[feats.Number=Plur, feats.Person=2]; s[form=you, rel=nsubj|nsubj:pass, head=v]
set: v[feats+=Number=Sing]
source: EWT 2.18
```

```convert
rule: gum.you-aux
what: aux/cop on a predicate with subject you — Number=Sing
from: gum
to: mova
match: h[]; s[form=you, rel=nsubj|nsubj:pass, head=h]; a[rel=aux|cop|aux:pass, head=h, feats.Number=Plur, feats.Person=2]
set: a[feats+=Number=Sing]
source: EWT 2.18
```

```convert
rule: gum.you
what: you without Number
from: gum
to: mova
match: p[form=you|yourself, upos=PRON, feats.Number]
require: p[feats.Person=2]
set: p[feats-=Number]
source: EWT 2.18
```

*yourselves* keeps `Number=Plur`, as in EWT: there it comes from the form.

### 4. `Degree` (`seeds/adv-degree.md`)

```convert
rule: gum.adv-degree
what: Degree=Pos only on adverbs with degrees (EWT list)
from: gum
to: mova
match: a[upos=ADV, feats.Degree=Pos, lemma!=well|far|soon|long|hard|early|late|little|close|high|fast|badly|low]
set: a[feats-=Degree]
source: EWT 2.18
```

```convert
rule: gum.such
what: such ADJ — Degree=Pos
from: gum
to: mova
match: a[upos=ADJ, lemma=such, !feats.Degree]
set: a[feats+=Degree=Pos]
source: EWT 2.18 (88 of 88)
```

### 5. Web-text XPOS (`seeds/xpos-web.md`)

```convert
rule: gum.add
what: URL — ADD
from: gum
to: mova
match: u[xpos=NNP, prefix=http|www.]
set: u[xpos=ADD]
source: Webtext addendum (EWT: ADD)
```

```convert
rule: gum.nfp
what: emoticon — NFP
from: gum
to: mova
match: e[xpos=SYM, form=:)|:(|;)|:-)|:-(|:D|:P|;-)]
set: e[xpos=NFP]
source: Webtext addendum (EWT: NFP)
```

```convert
rule: gum.afx
what: prefix before a hyphen — AFX
from: gum
to: mova
match: p[xpos=NN|NNP|JJ|IN|VB|RB, form=post|non|pre|mid|over|anti|multi|co|re|inter|intra|sub|semi]; h[xpos=HYPH, next=p]
set: p[xpos=AFX]
source: BioMedical addendum (AFX); EWT 2.18
```

Matches: `obl-fragment` 88, `obl-unmarked-fragment` 17 (together 105, as `tb.en.obl-fragment` counts via `rel~obl`), `you-verb` 63, `you-aux` 48, `you` 2498 (with *yourself*), `adv-degree` 4933, `such` 145, `add` 16, `nfp` 1, `afx` 6.

`adv-degree` removes `Degree=Pos` from 4933 adverbs. This is more than the 1146 in the seed: there the list has only 24 words, while here it is everything except 13 EWT lemmas. For comparison: in EWT there are only 482 ADV tokens with `Degree=Pos`.

**Not converted:**
- **`dep` in captions** (*Image: David Titley*) — 58. EWT annotates such captions in different ways (*AP Photo/Nasser Nasser*, *Photo from …*); there is no stable counterpart.
- **`dep` on numbers** — 24: heterogeneous.
- **Ellipsis** `:` → `,`/`.`: a value with a comma cannot be written in a rule. Needs a comma in the value.
- ***excuse me*** with `iobj`: an error, not dialect.

## mova → gum

```convert-todo
rule: mova.gum.you-guys
what: you ← guys (nmod:unmarked) → guys — head, you — dep
from: mova
to: gum
match: y[form=you]; n[rel=nmod:unmarked, head=y, next=y]
set: n[head=@y]; n[rel=@y]; y[head=n]; y[rel=dep]
source: seeds/dep.md
```

```convert
rule: mova.gum.what-about
what: What about X — X as obl
from: mova
to: gum
match: h[form=what, head=0]; o[rel=nmod, head=h, after=h]; c[form=about, rel=case, head=o]
set: o[rel=obl]
source: seeds/obl-fragment.md
```

```convert
rule: mova.gum.you
what: you — Number=Sing (plural cannot be recovered from mova)
from: mova
to: gum
match: p[form=you|yourself, upos=PRON, feats.Person=2]
set: p[feats+=Number=Sing]
source: seeds/you-number.md
```

```convert
rule: mova.gum.adv-degree
what: Degree=Pos on all adverbs except pronominal ones and the GUM list without degrees
from: mova
to: gum
match: a[upos=ADV, !feats.Degree, !feats.PronType, lemma!=so|just|also|very|even|as|only|about|however|all|already|over|instead|maybe|out|rather|pretty|once|yet|quite|thus|twice|e.g.|anyway|somewhat|i.e.|c.|anyways|namely|merely]
set: a[feats+=Degree=Pos]
source: seeds/adv-degree.md (GUM: ADV without Degree mostly — only these lemmas)
```

```convert
rule: mova.gum.such
what: such ADJ without Degree
from: mova
to: gum
match: a[upos=ADJ, lemma=such]
set: a[feats-=Degree]
source: seeds/adv-degree.md
```

```convert
rule: mova.gum.add
what: ADD → NNP
from: mova
to: gum
match: u[xpos=ADD]
set: u[xpos=NNP]
source: seeds/xpos-web.md
```

```convert
rule: mova.gum.nfp
what: NFP → SYM
from: mova
to: gum
match: e[xpos=NFP]
set: e[xpos=SYM]
source: seeds/xpos-web.md
```

```convert
rule: mova.gum.afx
what: AFX → NN
from: mova
to: gum
match: p[xpos=AFX]
set: p[xpos=NN]
source: seeds/xpos-web.md
```

Matches on EWT: `what-about` 7, `you` 2806, `adv-degree` 5151, `such` 88, `add` 475, `nfp` 499, `afx` 75.

## Round trip gum → mova → gum

**Lossless:**
- `such` — 145;
- *What about X* — 13 of 13;
- `ADD`, `NFP` — isolated (16 and 1);
- ***you guys*** — until there is `rel=@v`, the rule is in `convert-todo`, and `dep` stays in both directions. The round trip is lossless, but `mova` here is not yet like EWT.

**Lossy:**
- **The number of *you*** — 256 *you*:Plur and 111 verbs (63 predicates and 48 aux/cop). `mova` = EWT has no Number on *you*; on the way back we set Sing. If Mova borrows the GUM custom (`seeds/you-number.md`), the losses will disappear.
- **`obl` in fragments.** On the way there — 105 `obl`/`obl:unmarked` → `nmod`/`nmod:unmarked`. On the way back the GUM custom is restored only for *What about X* (13). The remaining ≈ 90 stay `nmod`. GUM itself is inconsistent in fragments: on roots without a copula 56 `obl` versus 151 `nmod`, so there is no rule that would restore exactly these 90.
- **`Degree` on adverbs.** On the way there we remove it from 4933, on the way back we set it according to the GUM list. Adverbs that are not in the list and that GUM left without Degree will produce a divergence. Expect 100–300 per 12,206 ADV.
- **`AFX`** → `NN`: 6 on the way there, on the way back only NN, while GUM also has NNP, VB, IN — up to 6 XPOS divergences.

Total: UPOS and DEPREL ≥ 99.9%; FEATS ≈ 99.8%, the rest — *you* (≈ 370 tokens) and `Degree`; XPOS ≥ 99.99%.
