# LinES ↔ mova — converter draft

The rule language is `train/dialects-and-converters.md`. Conventions beyond the specification are in `../seeds-overview.md`. Matches were counted by `en expert-check`: `lines → mova` on all parts of LinES, `mova → lines` on EWT.

## lines → mova

### 1. Relative pronouns and *when* (`seeds/nominal-feats.md`, `seeds/old-structures.md`)

Relatives first: the agreement of the relative-clause predicate depends on them.

```convert
rule: lines.rel-prontype
what: relative pronoun in acl:relcl — PronType=Rel
from: lines
to: mova
match: v[rel=acl:relcl]; t[form=that|which|who|whom, upos=PRON, rel=nsubj|nsubj:pass|obj|obl|iobj, head=v, before=v]
set: t[feats+=PronType=Rel]
source: UD en feat/PronType; EWT 2.18
```

```convert
rule: lines.wh-advmod
what: subordinate when/where/how/why as SCONJ mark → ADV advmod
from: lines
to: mova
match: w[form=when|where|how|why, rel=mark]
set: w[upos=ADV]; w[rel=advmod]
source: EWT README v2.11 (#88)
```


### 2. Agreement (`seeds/verb-feats.md`)

The same rules as in `../atis/convert.md`, with `from: lines`.


```convert
rule: lines.agree-pron
what: finite verb without Number — number and person from a pronoun subject
from: lines
to: mova
match: v[feats.VerbForm=Fin, feats.Tense, !feats.Number]; s[rel=nsubj|nsubj:pass, head=v, upos=PRON, feats.Person]
set: v[feats+=Number=@s]; v[feats+=Person=@s]
source: EWT 2.18 (Number/Person from the subject); UD docs/changes.md No. 18 (2.19)
```

```convert
rule: lines.agree-noun
what: finite verb without Number — number from a noun subject, 3rd person
from: lines
to: mova
match: v[feats.VerbForm=Fin, feats.Tense, !feats.Number]; s[rel=nsubj|nsubj:pass, head=v, upos=NOUN|PROPN|NUM]
set: v[feats+=Number=@s]; v[feats+=Person=3]
source: EWT 2.18
```

```convert
rule: lines.agree-aux-pron
what: aux/cop without Number — from the head's subject (pronoun)
from: lines
to: mova
match: h[]; s[rel=nsubj|nsubj:pass, head=h, upos=PRON, feats.Person]; a[rel=aux|cop|aux:pass, head=h, feats.VerbForm=Fin, feats.Tense, !feats.Number]
set: a[feats+=Number=@s]; a[feats+=Person=@s]
source: EWT 2.18 («for aux and cop — the head's subject»)
```

```convert
rule: lines.agree-aux-noun
what: aux/cop without Number — from the head's subject (noun)
from: lines
to: mova
match: h[]; s[rel=nsubj|nsubj:pass, head=h, upos=NOUN|PROPN|NUM]; a[rel=aux|cop|aux:pass, head=h, feats.VerbForm=Fin, feats.Tense, !feats.Number]
set: a[feats+=Number=@s]; a[feats+=Person=3]
source: EWT 2.18
```

```convert
rule: lines.agree-conj
what: conjoined predicate without its own subject — features of the first conjunct
from: lines
to: mova
match: h[feats.Number, feats.Person]; v[rel=conj, head=h, feats.VerbForm=Fin, feats.Tense, !feats.Number]
require: none c[rel~nsubj, head=v]
set: v[feats+=Number=@h]; v[feats+=Person=@h]
source: («for conj — the subject of the first conjunct»)
```

```convert
rule: lines.agree-relcl
what: predicate of a relative clause with subject that/which — number of the antecedent
from: lines
to: mova
match: n[upos=NOUN|PROPN]; v[rel=acl:relcl, head=n, feats.VerbForm=Fin, feats.Tense, !feats.Number]; t[rel=nsubj|nsubj:pass, head=v, feats.PronType=Rel]
set: v[feats+=Number=@n]; v[feats+=Person=3]
source: («for a relative pronoun — the antecedent»)
```

```convert
rule: lines.modal
what: modal — only VerbForm=Fin
from: lines
to: mova
match: m[upos=AUX, lemma=will|would|can|could|may|might|shall|should|must]
set: m[feats-=Mood]; m[feats-=Tense]; m[feats-=Person]; m[feats-=Number]
source: UD en pos/AUX (MD: VerbForm=Fin)
```


### 3. Pronouns, numbers, negation (`seeds/nominal-feats.md`)


```convert
rule: lines.you-nom
what: you as subject without Case — Case=Nom
from: lines
to: mova
match: p[form=you, upos=PRON, !feats.Case, rel=nsubj|nsubj:pass|nsubj:outer]
set: p[feats+=Case=Nom]
source: EWT 2.17 (#589: Case on it/you)
```

```convert
rule: lines.you-acc
what: you not as subject without Case — Case=Acc
from: lines
to: mova
match: p[form=you, upos=PRON, !feats.Case]
set: p[feats+=Case=Acc]
source: EWT 2.17
```

```convert
rule: lines.poss-case
what: possessive modifier without Case=Gen — Case=Gen, lemma = form
from: lines
to: mova
match: p[upos=PRON, rel=nmod:poss, form=my|your|his|her|its|our|their, feats.Case!=Gen]
set: p[feats+=Case=Gen]; p[feats+=Poss=Yes]; p[lemma=@form]
source: EWT README v2.11 (docs#517)
```

```convert
rule: lines.poss-lemma
what: possessive with Case=Gen but an old lemma — lemma = form
from: lines
to: mova
match: p[upos=PRON, rel=nmod:poss, form=my|your|his|our|their, feats.Case=Gen, lemma!=@form]
set: p[lemma=@form]
source: EWT README v2.11
```

```convert
rule: lines.numform-digit
what: number in digits — NumForm=Digit
from: lines
to: mova
match: n[upos=NUM, suffix=0|1|2|3|4|5|6|7|8|9]
set: n[feats+=NumForm=Digit]
source: EWT README v2.13
```

```convert
rule: lines.numform-word
what: number in words — NumForm=Word
from: lines
to: mova
match: n[upos=NUM, !feats.NumForm]
set: n[feats+=NumForm=Word]
source: EWT README v2.13
```

```convert
rule: lines.not
what: not/n't without Polarity — Polarity=Neg
from: lines
to: mova
match: n[form=not|n't, upos=PART, !feats.Polarity]
set: n[feats+=Polarity=Neg]
source: EWT README v2.15
```

```convert
rule: lines.an
what: lemma an → a
from: lines
to: mova
match: d[form=an, upos=DET]
set: d[lemma=a]
source: UD en pos/DET
```


### 4. Structures (`seeds/old-structures.md`)


```convert
rule: lines.obl-agent
what: by phrase on a passive participle — obl:agent
from: lines
to: mova
match: v[feats.VerbForm=Part, feats.Tense=Past]; o[rel=obl, head=v]; b[form=by, rel=case, head=o]
require: none c[rel=aux, head=v, lemma=have]
set: o[rel=obl:agent]
source: EWT README v2.13 (#290)
```

```convert
rule: lines.preconj
what: both/either/neither CCONJ cc → cc:preconj
from: lines
to: mova
match: p[form=both|either|neither, upos=CCONJ, rel=cc]
set: p[rel=cc:preconj]
source: UD en cc:preconj
```

```convert
rule: lines.predet
what: all/both before an article or possessive as det → det:predet
from: lines
to: mova
match: p[form=all|both, rel=det]; d[upos=DET|PRON, rel=det|nmod:poss, next=p]
set: p[rel=det:predet]
source: UD en det:predet
```

```convert
rule: lines.predet-such
what: such/half before an article as amod → DET det:predet
from: lines
to: mova
match: p[form=such|half, rel=amod]; d[upos=DET, rel=det, next=p]
set: p[rel=det:predet]; p[upos=DET]
source: EWT README v2.16 (docs#1114)
```


Matches on LinES:
- `rel-prontype` 513, `wh-advmod` 253;
- agreement: `agree-pron` 2390, `agree-noun` 1449, `agree-aux-pron` 1010, `agree-aux-noun` 637, `agree-relcl` 56, `agree-conj` 1. `agree-conj` on the source data relies on the Number of the first conjunct, so after the previous rules there will be more;
- `modal` 1295;
- pronouns: `you-nom` 676, `you-acc` 135 (811 − 676), `poss-case` 289, `poss-lemma` 0 (all old lemmas were already caught by `poss-case`);
- `numform-digit` 194, `numform-word` 497 (691 − 194), `not` 98, `an` 357 (the lemma will change in 31);
- structures: `obl-agent` 152, `preconj` 33, `predet` 66, `predet-such` 19.

**Not converted:**
- **XPOS.** LinES tags are not PTB (`seeds/xpos-lines.md`). The rule language cannot read a condition on them: `xpos=` knows only PTB tags. In `mova` LinES stays without PTB XPOS, as now: the PTB tagger does not learn on it. A PTB tag from UPOS + FEATS would need a table of ≈ 25 rules (NOUN+Sing → NN, VERB+Past+Fin → VBD…). But for ADV, ADP, PART (RB/RP/IN/TO/WRB) it would be guessing, so it is better to retag with the `en` tagger than with rules.
- **`VerbForm=Ger`**: cannot be distinguished from the form.
- **Errors** from `seeds/errors.md`, except the systematic `her`/`its` (fixed by `poss-case`).

## mova → lines


```convert
rule: mova.lines.agree
what: present without -s, except am — without Number/Person (LinES: are without Number)
from: mova
to: lines
match: v[feats.VerbForm=Fin, feats.Tense=Pres, suffix!=s, form!=am|'m]
set: v[feats-=Number]; v[feats-=Person]
source: seeds/verb-feats.md
```

```convert
rule: mova.lines.agree-past
what: past, except be — without Number/Person
from: mova
to: lines
match: v[feats.VerbForm=Fin, feats.Tense=Past, lemma!=be]
set: v[feats-=Number]; v[feats-=Person]
source: seeds/verb-feats.md
```

```convert
rule: mova.lines.modal
what: modal — Mood=Ind
from: mova
to: lines
match: m[upos=AUX, lemma=will|would|can|could|may|might|shall|should|must]
set: m[feats+=Mood=Ind]
source: seeds/verb-feats.md
```

```convert
rule: mova.lines.ger
what: gerund — Tense=Pres|VerbForm=Part
from: mova
to: lines
match: v[feats.VerbForm=Ger]
set: v[feats+=VerbForm=Part]; v[feats+=Tense=Pres]
source: seeds/verb-feats.md
```

```convert
rule: mova.lines.you
what: you without Case
from: mova
to: lines
match: p[form=you, upos=PRON]
set: p[feats-=Case]
source: seeds/nominal-feats.md
```

```convert
rule: mova.lines.numform
what: without NumForm
from: mova
to: lines
match: n[feats.NumForm]
set: n[feats-=NumForm]
source: seeds/nominal-feats.md
```

```convert
rule: mova.lines.wh
what: subordinate when on advcl — SCONJ mark
from: mova
to: lines
match: v[rel=advcl]; w[form=when, rel=advmod, head=v, before=v]
set: w[upos=SCONJ]; w[rel=mark]
source: seeds/old-structures.md (when SCONJ mark — 252)
```

```convert
rule: mova.lines.obl-agent
what: obl:agent → obl
from: mova
to: lines
match: o[rel=obl:agent]
set: o[rel=obl]
source: seeds/old-structures.md
```

```convert
rule: mova.lines.preconj
what: cc:preconj → cc
from: mova
to: lines
match: p[rel=cc:preconj]
set: p[rel=cc]
source: seeds/old-structures.md
```

```convert
rule: mova.lines.predet
what: det:predet → det (all/both) or amod (such/half)
from: mova
to: lines
match: p[rel=det:predet, form=all|both]
set: p[rel=det]
source: seeds/old-structures.md
```

```convert
rule: mova.lines.predet-such
what: such/half det:predet → ADJ amod
from: mova
to: lines
match: p[rel=det:predet, form=such|half]
set: p[rel=amod]; p[upos=ADJ]
source: seeds/old-structures.md
```

```convert
rule: mova.lines.relcl
what: advcl:relcl → advcl
from: mova
to: lines
match: t[rel=advcl:relcl]
set: t[rel=advcl]
source: seeds/old-structures.md
```


```convert-todo
rule: mova.lines.xpos-noun
what: LinES XPOS from UPOS and FEATS: NOUN Sing → SG-NOM
from: mova
to: lines
match: n[upos=NOUN|PROPN, feats.Number=Sing]
set: n[xpos=SG-NOM]
source: seeds/xpos-lines.md
```

Needs string XPOS: `SG-NOM` is not a PTB tag. The whole "UPOS + FEATS → LinES tag" table is ≈ 40 lines: `PL-NOM`, `DEF`, `IND-SG`, `PAST`, `INF`, `ING`, `PRES`, `PASS`, `PERF`, `PAST-AUX`, `PRES-AUX`, `PERS-P3SG-NOM`… For function-word ADP, ADV, CCONJ, SCONJ, PART — `_`.

Matches on EWT: `agree` 6308, `agree-past` 4493, `modal` 4048, `ger` 1230, `you` 2768, `numform` 5608, `wh` 401, `obl-agent` 376, `preconj` 98, `predet` 151, `predet-such` 42, `relcl` 141.

## Round trip lines → mova → lines

**Lossless** (there and back are exact inverses, and LinES is consistent in this):
- agreement: Number/Person are set on the way there and removed on the way back from everything except forms in *-s*, *am/'m* and past *be* (*was/were*). LinES writes exactly that;
- modals (1295), *you* (811), `NumForm` (691), `obl:agent` (152), `cc:preconj` (33), `det:predet` (*all/both* 66, *such/half* 19), *when* on `advcl`.

**Lossy:**
- **Possessives.** On the way there — `Case=Gen` and lemma = form for 289 (among them 54 erroneous *her*/*its*). Nothing is restored on the way back. LinES mixes two norms: *his* → *he* 136, but *his* → *his* 670. So ≈ 290 lemma and FEATS divergences. This is deliberate: we do not reproduce errors on the way back.
- **`Polarity=Neg`** on 98 *not*: not removed on the way back. 98 FEATS divergences — also corrections, not dialect.
- **Relative pronouns.** On the way there `PronType=Rel` on 513. The LinES custom (*that* without PronType, some *which/who* with `Int`) cannot be reproduced on the way back: ≈ 314 FEATS divergences.
- **Subordinate *where/how/why*** in `mark` (≈ 5). On the way back only *when* on `advcl` gets `mark`.
- **Gerund.** On the way back `Ger` → `Part`. LinES has no `Ger` anyway, so for LinES itself there is no loss. The loss appears only on the `en` output.
- ***an*.** On the way there lemma *a* (31), on the way back it stays *a*: LinES itself has *an* → *a* 326 versus *an* → *an* 31.

Total: DEPREL and UPOS ≥ 99.9%, FEATS ≈ 99.3% (possessives, relatives, `Polarity`), lemmas ≈ 99.7%. XPOS is outside the round trip until there is string XPOS.
