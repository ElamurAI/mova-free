# Pronouns ↔ mova — converter draft

The rule language is `train/dialects-and-converters.md`. Conventions beyond the specification are in `../seeds-overview.md`. Matches were counted by `en expert-check`: `pronouns → mova` on Pronouns, `mova → pronouns` on EWT.

Pronouns is a test of independent possessives. The main thing for us is `mova → pronouns`: so that UFeats on it measures quality rather than the difference in norms (`seeds/conventions.md`).

## pronouns → mova

```convert
rule: pronouns.pass-subj
what: subject with aux:pass — nsubj:pass
from: pronouns
to: mova
match: v[]; a[rel=aux:pass, head=v]; s[rel=nsubj, head=v]
set: s[rel=nsubj:pass]
source: UD en nsubj:pass
```

```convert
rule: pronouns.voice
what: participle with aux:pass — Voice=Pass
from: pronouns
to: mova
match: v[feats.VerbForm=Part, feats.Tense=Past]; a[rel=aux:pass, head=v]
set: v[feats+=Voice=Pass]
source: EWT README v2.13
```

```convert
rule: pronouns.it-nom
what: it as subject — Case=Nom, Gender=Neut
from: pronouns
to: mova
match: p[form=it, upos=PRON, rel=nsubj|nsubj:pass|expl]
set: p[feats+=Case=Nom]; p[feats+=Gender=Neut]
source: EWT 2.17 (#589)
```

```convert
rule: pronouns.it-acc
what: it not as subject — Case=Acc, Gender=Neut
from: pronouns
to: mova
match: p[form=it, upos=PRON, !feats.Case]
set: p[feats+=Case=Acc]; p[feats+=Gender=Neut]
source: EWT 2.17 (#589)
```

```convert
rule: pronouns.indep-gender
what: mine/yours/theirs without Gender
from: pronouns
to: mova
match: p[form=mine|yours|theirs, upos=PRON, feats.Poss=Yes]
set: p[feats-=Gender]
source: EWT 2.18
```

```convert
rule: pronouns.theirs-plur
what: theirs — Number=Plur (UD en: by form)
from: pronouns
to: mova
match: p[form=theirs, upos=PRON]
set: p[feats+=Number=Plur]
source: UD en feat/Number (theirs — Plur)
```

```convert
rule: pronouns.is-person
what: is without Person — Person=3
from: pronouns
to: mova
match: v[form=is, feats.Number=Sing, !feats.Person]
set: v[feats+=Person=3]
source: seeds/errors.md
```

Next, agreement — the same rules as in `../atis/convert.md`:

```convert
rule: pronouns.agree-pron
what: finite verb without Number — number and person from a pronoun subject
from: pronouns
to: mova
match: v[feats.VerbForm=Fin, feats.Tense, !feats.Number]; s[rel=nsubj|nsubj:pass, head=v, upos=PRON, feats.Person]
set: v[feats+=Number=@s]; v[feats+=Person=@s]
source: EWT 2.18 (Number/Person from the subject); UD docs/changes.md No. 18 (2.19)
```

```convert
rule: pronouns.agree-noun
what: finite verb without Number — number from a noun subject, 3rd person
from: pronouns
to: mova
match: v[feats.VerbForm=Fin, feats.Tense, !feats.Number]; s[rel=nsubj|nsubj:pass, head=v, upos=NOUN|PROPN|NUM]
set: v[feats+=Number=@s]; v[feats+=Person=3]
source: EWT 2.18
```

```convert
rule: pronouns.agree-aux-pron
what: aux/cop without Number — from the head's subject (pronoun)
from: pronouns
to: mova
match: h[]; s[rel=nsubj|nsubj:pass, head=h, upos=PRON, feats.Person]; a[rel=aux|cop|aux:pass, head=h, feats.VerbForm=Fin, feats.Tense, !feats.Number]
set: a[feats+=Number=@s]; a[feats+=Person=@s]
source: EWT 2.18 («for aux and cop — the head's subject»)
```

```convert
rule: pronouns.agree-aux-noun
what: aux/cop without Number — from the head's subject (noun)
from: pronouns
to: mova
match: h[]; s[rel=nsubj|nsubj:pass, head=h, upos=NOUN|PROPN|NUM]; a[rel=aux|cop|aux:pass, head=h, feats.VerbForm=Fin, feats.Tense, !feats.Number]
set: a[feats+=Number=@s]; a[feats+=Person=3]
source: EWT 2.18
```

```convert
rule: pronouns.agree-conj
what: conjoined predicate without its own subject — features of the first conjunct
from: pronouns
to: mova
match: h[feats.Number, feats.Person]; v[rel=conj, head=h, feats.VerbForm=Fin, feats.Tense, !feats.Number]
require: none c[rel~nsubj, head=v]
set: v[feats+=Number=@h]; v[feats+=Person=@h]
source: («for conj — the subject of the first conjunct»)
```

```convert
rule: pronouns.agree-relcl
what: predicate of a relative clause with subject that/which — number of the antecedent
from: pronouns
to: mova
match: n[upos=NOUN|PROPN]; v[rel=acl:relcl, head=n, feats.VerbForm=Fin, feats.Tense, !feats.Number]; t[rel=nsubj|nsubj:pass, head=v, feats.PronType=Rel]
set: v[feats+=Number=@n]; v[feats+=Person=3]
source: («for a relative pronoun — the antecedent»)
```

```convert
rule: pronouns.adj-degree
what: ADJ without Degree — Degree=Pos
from: pronouns
to: mova
match: a[upos=ADJ, !feats.Degree]
set: a[feats+=Degree=Pos]
source: EWT 2.18
```


Matches on Pronouns: `pass-subj` 25, `voice` 20, `it-nom` 40 (all *it* here are subjects, so `it-acc` after it — 0), `indep-gender` 171, `theirs-plur` 57, `is-person` 10, `agree-noun` 70, `agree-pron` 10, `adj-degree` 15.

**Singular *they*.** `theirs-plur` erases the very thing Pronouns was created for: `Number=Sing` on *theirs*. The UD guideline for English requires `Plur` by form (https://universaldependencies.org/en/feat/Number.html, `pos/PRON.md`), and `mova` = EWT. As with the number of *you* in GUM (`../gum/seeds/you-number.md`), this is a candidate for borrowing in `dialects/mova/seeds/`. There the singular of *they/theirs* is worth preserving as a feature or in MISC.

## mova → pronouns


```convert
rule: mova.pronouns.pass-subj
what: nsubj:pass → nsubj
from: mova
to: pronouns
match: s[rel=nsubj:pass]
set: s[rel=nsubj]
source: seeds/conventions.md
```

```convert
rule: mova.pronouns.voice
what: without Voice
from: mova
to: pronouns
match: v[feats.Voice]
set: v[feats-=Voice]
source: seeds/conventions.md
```

```convert
rule: mova.pronouns.case
what: personal pronouns without Case
from: mova
to: pronouns
match: p[upos=PRON, feats.PronType=Prs]
set: p[feats-=Case]
source: seeds/conventions.md
```

```convert
rule: mova.pronouns.it
what: it without Gender
from: mova
to: pronouns
match: p[form=it, upos=PRON]
set: p[feats-=Gender]
source: seeds/conventions.md
```

```convert
rule: mova.pronouns.indep-gender
what: mine/yours/theirs — Gender=Neut
from: mova
to: pronouns
match: p[form=mine|yours|theirs, upos=PRON, feats.Poss=Yes]
set: p[feats+=Gender=Neut]
source: seeds/conventions.md
```

```convert
rule: mova.pronouns.theirs
what: theirs — Number=Sing (singular they)
from: mova
to: pronouns
match: p[form=theirs, upos=PRON]
set: p[feats+=Number=Sing]
source: README UD_English-Pronouns
```

```convert
rule: mova.pronouns.adj
what: ADJ with Degree=Pos — without Degree (15 of 20 in Pronouns)
from: mova
to: pronouns
match: a[upos=ADJ, feats.Degree=Pos]
set: a[feats-=Degree]
source: seeds/conventions.md
```


Matches on EWT: `pass-subj` 1445, `voice` 3255, `case` 18 772, `it` 2277, `indep-gender` 28, `theirs` 2, `adj` 15 557.

There are no reverse agreement rules. Pronouns mostly already agrees by subject, like EWT: 225 of 315 finite verbs have Number/Person, including past *cleaned*, *sold*, *drove*. Removing them by form would spoil the majority.

## Round trip pronouns → mova → pronouns

**Lossless:** `nsubj:pass` (25), `Voice` (20), `Case` and `Gender` on *it* (40), `Gender` on *mine/yours/theirs* (171), `Number=Sing` on *theirs* (57).

**Lossy:**
- ***is* without Person** (10) gets `Person=3` on the way there and does not lose it on the way back: *is* ends in *-s*. 10 divergences — an error fix.
- **Agreement.** 90 finite verbs without Number/Person get them from the subject on the way there and keep them on the way back: 90 FEATS divergences. This fixes an inconsistency, not a dialect.
- **`Degree` on ADJ:** on the way there we set it on all 15, on the way back we remove it from all 20. 5 ADJ that had `Degree=Pos` in Pronouns will diverge.

Total ≈ 105 FEATS divergences per 1705 words: 93.8%. DEPREL and UPOS — 100%. All divergences are places where Pronouns is inconsistent with itself.
