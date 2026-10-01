# PUD ↔ mova — converter draft

The rule language is `train/dialects-and-converters.md`. Conventions beyond the specification are in `../seeds-overview.md`. Matches were counted by `en expert-check`: `pud → mova` on PUD, `mova → pud` on EWT.

PUD is test only. The trees and UPOS here are close to EWT, so the converter touches almost only the CoreNLP FEATS and `obl:agent` (`seeds/feats-corenlp.md`).

## pud → mova

```convert
rule: pud.imperative
what: infinitive root without subject, aux and mark — imperative
from: pud
to: mova
match: v[upos=VERB, head=0, feats.VerbForm=Inf]
require: none c[rel~nsubj, head=v]; none c[rel~csubj, head=v]; none c[rel=expl, head=v]; none c[rel~aux, head=v]; none c[rel=mark, head=v]
set: v[feats+=VerbForm=Fin]; v[feats+=Mood=Imp]
source: EWT 2.18
```

Agreement uses the same rules as in `../atis/convert.md`:

```convert
rule: pud.agree-pron
what: finite verb without Number — number and person from a pronoun subject
from: pud
to: mova
match: v[feats.VerbForm=Fin, feats.Tense, !feats.Number]; s[rel=nsubj|nsubj:pass, head=v, upos=PRON, feats.Person]
set: v[feats+=Number=@s]; v[feats+=Person=@s]
source: EWT 2.18 (Number/Person from the subject); UD docs/changes.md No. 18 (2.19)
```

```convert
rule: pud.agree-noun
what: finite verb without Number — number from a noun subject, 3rd person
from: pud
to: mova
match: v[feats.VerbForm=Fin, feats.Tense, !feats.Number]; s[rel=nsubj|nsubj:pass, head=v, upos=NOUN|PROPN|NUM]
set: v[feats+=Number=@s]; v[feats+=Person=3]
source: EWT 2.18
```

```convert
rule: pud.agree-aux-pron
what: aux/cop without Number — from the head's subject (pronoun)
from: pud
to: mova
match: h[]; s[rel=nsubj|nsubj:pass, head=h, upos=PRON, feats.Person]; a[rel=aux|cop|aux:pass, head=h, feats.VerbForm=Fin, feats.Tense, !feats.Number]
set: a[feats+=Number=@s]; a[feats+=Person=@s]
source: EWT 2.18 («for aux and cop — the head's subject»)
```

```convert
rule: pud.agree-aux-noun
what: aux/cop without Number — from the head's subject (noun)
from: pud
to: mova
match: h[]; s[rel=nsubj|nsubj:pass, head=h, upos=NOUN|PROPN|NUM]; a[rel=aux|cop|aux:pass, head=h, feats.VerbForm=Fin, feats.Tense, !feats.Number]
set: a[feats+=Number=@s]; a[feats+=Person=3]
source: EWT 2.18
```

```convert
rule: pud.agree-conj
what: conjoined predicate without its own subject — features of the first conjunct
from: pud
to: mova
match: h[feats.Number, feats.Person]; v[rel=conj, head=h, feats.VerbForm=Fin, feats.Tense, !feats.Number]
require: none c[rel~nsubj, head=v]
set: v[feats+=Number=@h]; v[feats+=Person=@h]
source: («for conj — the subject of the first conjunct»)
```

```convert
rule: pud.agree-relcl
what: predicate of a relative clause with subject that/which — number of the antecedent
from: pud
to: mova
match: n[upos=NOUN|PROPN]; v[rel=acl:relcl, head=n, feats.VerbForm=Fin, feats.Tense, !feats.Number]; t[rel=nsubj|nsubj:pass, head=v, feats.PronType=Rel]
set: v[feats+=Number=@n]; v[feats+=Person=3]
source: («for a relative pronoun — the antecedent»)
```

```convert
rule: pud.voice
what: participle with aux:pass or in acl — Voice=Pass
from: pud
to: mova
match: v[feats.VerbForm=Part, feats.Tense=Past]
require: exists c[rel=aux:pass, head=v] or v[rel=acl]
set: v[feats+=Voice=Pass]
source: EWT README v2.13
```

```convert
rule: pud.obl-agent
what: by phrase with a passive — obl:agent
from: pud
to: mova
match: v[feats.Voice=Pass]; o[rel=obl, head=v]; b[form=by, rel=case, head=o]
set: o[rel=obl:agent]
source: EWT README v2.13
```

```convert
rule: pud.poss-gen
what: possessive modifier — Case=Gen
from: pud
to: mova
match: p[upos=PRON, rel=nmod:poss, feats.Poss=Yes, !feats.Case]
set: p[feats+=Case=Gen]
source: EWT README v2.11
```

```convert
rule: pud.wh-advmod
what: subordinate when/where/how/why as mark → ADV advmod
from: pud
to: mova
match: w[form=when|where|how|why, rel=mark]
set: w[upos=ADV]; w[rel=advmod]
source: EWT README v2.11 (#88)
```


Matches on PUD: `imperative` 4 (of 74 infinitive roots), agreement — `agree-pron` 150, `agree-noun` 395, `agree-aux-pron` 51, `agree-aux-noun` 163, `agree-relcl` 53, `agree-conj` 0 on the source data; `voice` 349 (588 − 239), `obl-agent` 0 on the source → ≈ 66 after `voice`; `poss-gen` 260; `wh-advmod` 4.

## mova → pud


```convert
rule: mova.pud.imperative
what: imperative → VerbForm=Inf without Mood
from: mova
to: pud
match: v[feats.Mood=Imp]
set: v[feats-=Mood]; v[feats+=VerbForm=Inf]
source: seeds/feats-corenlp.md
```

```convert
rule: mova.pud.agree-pres
what: present without -s, except am — without Number/Person
from: mova
to: pud
match: v[feats.VerbForm=Fin, feats.Tense=Pres, suffix!=s, form!=am|'m]
set: v[feats-=Number]; v[feats-=Person]
source: seeds/feats-corenlp.md
```

```convert
rule: mova.pud.agree-past
what: past, except be — without Number/Person
from: mova
to: pud
match: v[feats.VerbForm=Fin, feats.Tense=Past, lemma!=be]
set: v[feats-=Number]; v[feats-=Person]
source: seeds/feats-corenlp.md
```

```convert
rule: mova.pud.voice
what: without Voice
from: mova
to: pud
match: v[feats.Voice]
set: v[feats-=Voice]
source: seeds/feats-corenlp.md
```

```convert
rule: mova.pud.obl-agent
what: obl:agent → obl
from: mova
to: pud
match: o[rel=obl:agent]
set: o[rel=obl]
source: seeds/feats-corenlp.md
```

```convert
rule: mova.pud.poss
what: possessive — without Case
from: mova
to: pud
match: p[upos=PRON, rel=nmod:poss, feats.Case=Gen]
set: p[feats-=Case]
source: seeds/feats-corenlp.md
```


Matches on EWT: `imperative` 1663, `agree-pres` 6308, `agree-past` 4493, `voice` 3255, `obl-agent` 376, `poss` 3673.

## Round trip pud → mova → pud

**Lossless** (CoreNLP is consistent, so the reverse reproduces exactly):
- agreement by form — ≈ 810 verbs (PUD: Number only on VBZ, *am*, *was/were*);
- `Voice` — 349;
- `Case=Gen` on possessives — 260;
- `obl:agent` — ≈ 66;
- imperatives — 4.

**Lossy:**
- **`when` as `mark`** — the 4 become `advmod` on the way there. There is no reverse rule: most *when* in PUD are already `advmod` (25). 4 divergences.
- **Agreement without a subject** — verbs without their own subject, aux heads and the first conjunct remain without Number. On the way back this is a match, not a loss, but in `mova` they are incomplete.

Total: ≥ 99.9% across all columns.
