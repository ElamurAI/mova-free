# CTeTex ↔ mova — converter draft

The rule language is `train/dialects-and-converters.md`. Conventions beyond the specification are in `../seeds-overview.md`. Matches were counted by `en expert-check`: `ctetex → mova` on CTeTex, `mova → ctetex` on EWT.

CTeTex is test only, so the main direction is `mova → ctetex`: the `en` output must be brought to CTeTex customs so that quality is measured rather than the difference between dialects. `ctetex → mova` is needed for the round trip and for checking the two roads to the goal.

CTeTex FEATS are incomplete (`seeds/feats-minimal.md`). On the way there the converter does not fill them in: a test does not need training. On the way back it reduces the full `mova` FEATS to the CTeTex set.

## ctetex → mova

```convert
rule: ctetex.nn-compound
what: noun before a noun as nmod without a preposition → compound
from: ctetex
to: mova
match: h[upos=NOUN|PROPN]; n[upos=NOUN|PROPN, rel=nmod, head=h, before=h]
require: none c[rel=case, head=n]
set: n[rel=compound]
source: UD en compound; seeds/nmod-compound.md
```

```convert
rule: ctetex.prt
what: particle on a verb as compound → compound:prt
from: ctetex
to: mova
match: v[upos=VERB]; p[upos=ADP|ADV, rel=compound, head=v, after=v]
set: p[rel=compound:prt]
source: UD en compound:prt; seeds/old-conventions.md
```

```convert
rule: ctetex.poss
what: possessive DET nmod → PRON nmod:poss, lemma = form
from: ctetex
to: mova
match: p[form=my|your|his|its|our|their, upos=DET, rel=nmod]
set: p[upos=PRON]; p[rel=nmod:poss]; p[lemma=@form]
source: UD en nmod:poss, pos/PRON; EWT README v2.11
```

```convert
rule: ctetex.wh-advmod
what: subordinate when/where as SCONJ mark → ADV advmod
from: ctetex
to: mova
match: w[form=when|where|how|why|whenever, rel=mark]
set: w[upos=ADV]; w[rel=advmod]
source: EWT README v2.11 (#88)
```

```convert
rule: ctetex.an
what: lemma an → a
from: ctetex
to: mova
match: d[form=an, upos=DET]
set: d[lemma=a]
source: UD en pos/DET
```

```convert
rule: ctetex.enum
what: list item number as nummod on the predicate → discourse
from: ctetex
to: mova
match: h[upos=VERB|ADJ|ADV|AUX]; n[upos=NUM|X, rel=nummod, head=h, before=h]
set: n[rel=discourse]
source: UD docs/changes.md No. 14; EWT README v2.14 (#518)
```

Matches: `nn-compound` 472 (of 487 `nmod` nouns before a head noun, 15 have a preposition), `prt` 14, `poss` 16, `wh-advmod` 36, `an` 41, `enum` 25.

`nn-compound` takes not only adjacent pairs (307, `seeds/nmod-compound.md`) but also chains: in *Speed Dump Lag Frames* several `nmod` attach to *Frames*. In EWT such chains are also `compound`.

## mova → ctetex

```convert
rule: mova.ctetex.prt
what: compound:prt → compound
from: mova
to: ctetex
match: p[rel=compound:prt]
set: p[rel=compound]
source: seeds/old-conventions.md
```

```convert
rule: mova.ctetex.poss
what: possessive nmod:poss → DET nmod
from: mova
to: ctetex
match: p[form=my|your|his|its|our|their, upos=PRON, rel=nmod:poss]
set: p[upos=DET]; p[rel=nmod]
source: seeds/old-conventions.md
```

```convert
rule: mova.ctetex.when
what: subordinate when/where on advcl → SCONJ mark
from: mova
to: ctetex
match: v[rel=advcl]; w[form=when|where, rel=advmod, head=v, before=v]
set: w[upos=SCONJ]; w[rel=mark]
source: seeds/old-conventions.md
```

```convert
rule: mova.ctetex.an
what: lemma an
from: mova
to: ctetex
match: d[form=an, upos=DET]
set: d[lemma=an]
source: seeds/old-conventions.md
```

```convert
rule: mova.ctetex.enum
what: discourse numbering marker → nummod
from: mova
to: ctetex
match: h[upos=VERB|ADJ|ADV|AUX]; n[upos=NUM|X, rel=discourse, head=h, before=h]
set: n[rel=nummod]
source: seeds/lists.md
```

```convert
rule: mova.ctetex.feats
what: CTeTex FEATS — only Number, Tense, Typo, ExtPos
from: mova
to: ctetex
match: t[]
set: t[feats-=Person]; t[feats-=VerbForm]; t[feats-=Mood]; t[feats-=Voice]; t[feats-=Case]; t[feats-=Gender]; t[feats-=PronType]; t[feats-=Poss]; t[feats-=Reflex]; t[feats-=Definite]; t[feats-=Degree]; t[feats-=NumType]; t[feats-=NumForm]; t[feats-=Polarity]; t[feats-=Foreign]; t[feats-=Abbr]; t[feats-=Style]
source: seeds/feats-minimal.md
```

```convert
rule: mova.ctetex.number-nonoun
what: Number only on NOUN, AUX and VERB
from: mova
to: ctetex
match: t[upos=PROPN|PRON|DET|ADJ|NUM|ADV|ADP|X|SYM]
set: t[feats-=Number]
source: seeds/feats-minimal.md (PROPN without Number — 293 of 293)
```

```convert
rule: mova.ctetex.verb-pres
what: VERB in the present — without Tense; Number only in 3sg
from: mova
to: ctetex
match: v[upos=VERB, feats.Tense=Pres]
set: v[feats-=Tense]
source: seeds/feats-minimal.md (VERB Tense=Pres — 0)
```

```convert
rule: mova.ctetex.verb-past
what: VERB in the past — without Number
from: mova
to: ctetex
match: v[upos=VERB, feats.Tense=Past]
set: v[feats-=Number]
source: seeds/feats-minimal.md (VERB Tense=Past — 286, without Number)
```

```convert
rule: mova.ctetex.verb-plur
what: VERB with Number=Plur — without Number (VBP)
from: mova
to: ctetex
match: v[upos=VERB, feats.Number=Plur]
set: v[feats-=Number]
source: seeds/feats-minimal.md
```

**Needs string XPOS:** CTeTex has no XPOS. A `xpos=_` action is needed — clear XPOS. If `xpos=` accepts only PTB tags, this is `convert-todo`. The rule itself is `t[]` → `t[xpos=_]`, as in `../atis/convert.md`.

Matches on EWT: `prt` 907, `poss` 3467, `when` 414, `an` 619, `enum` 75, `feats` 254 820, `number-nonoun` 117 378, `verb-pres` 8030, `verb-past` 8571, `verb-plur` 2398.

## Round trip ctetex → mova → ctetex

**Lossless:**
- `compound:prt` (14);
- possessives (16);
- *an* (41);
- numbering markers (25);
- *when* on `advcl`.

FEATS do not change on the way there, and the reduction on the way back does not touch them. The exception is 3 VERB with `Number=Plur`, which `verb-plur` removes.

**Lossy:**
- **A noun before a noun.** On the way there 472 `nmod` → `compound`. On the way back the CTeTex custom cannot be predicted: CTeTex itself splits adjacent pairs in half, 324 versus 307. So 472 DEPREL divergences, 5.1% of words.
- ***when* not on `advcl`.** 36 `mark` become `advmod` on the way there, and on the way back only the 32 that are on `advcl` and before it get `mark`. 4 UPOS and DEPREL divergences.
- **3 VERB `Number=Plur`** in FEATS.

Total: DEPREL ≈ 94.9% (the limit is noun pairs), UPOS ≥ 99.9%, FEATS ≥ 99.9%, lemmas 100%.
