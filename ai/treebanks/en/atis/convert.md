# ATIS ↔ mova — converter draft

The rule language is `train/dialects-and-converters.md`, section "Converter rule language". There is no engine yet, so the rules are written to the specification. What is assumed beyond it (`require:` as an application condition, `feats+=` replaces a value, ```` ```convert-todo ```` blocks) is in `../seeds-overview.md`, section "Conventions for convert.md".

"Matches" — how many times `match` + `require` fires. Counted by the seed engine `en expert-check`:
- `atis → mova` rules — on all parts of ATIS;
- `mova → atis` rules — on EWT as a stand-in for `mova`.

The seeds the rules grew from are in `seeds/`.

## atis → mova

### 1. Temporal modifiers (`seeds/tmod.md`)

```convert
rule: atis.tmod-np
what: nmod:tmod without a preposition → nmod:unmarked + TemporalNPAdjunct (EWT 2.15)
from: atis
to: mova
match: t[rel=nmod:tmod]
require: none c[rel=case, head=t]
set: t[rel=nmod:unmarked]; t[misc+=TemporalNPAdjunct=Yes]
source: UD 2.15, docs#1028; EWT README v2.15
```

```convert
rule: atis.tmod-pp
what: nmod:tmod with a preposition — plain nmod
from: atis
to: mova
match: t[rel=nmod:tmod]
require: exists c[rel=case, head=t]
set: t[rel=nmod]
source: UD en nmod:unmarked (only NP without case)
```

```convert
rule: atis.unmarked-pp
what: obl:unmarked with a preposition — plain obl
from: atis
to: mova
match: t[rel=obl:unmarked]
require: exists c[rel=case, head=t]
set: t[rel=obl]
source: UD en obl:unmarked
```

```convert
rule: atis.unmarked-np
what: obl:unmarked without a preposition is always temporal in ATIS
from: atis
to: mova
match: t[rel=obl:unmarked]
require: none c[rel=case, head=t]
set: t[misc+=TemporalNPAdjunct=Yes]
source: EWT README v2.15
```

Matches: `tmod-np` 241, `tmod-pp` 1022, `unmarked-pp` 688, `unmarked-np` 163.

### 2. Imperative, agreement, modals, passive (`seeds/verb-feats.md`, `seeds/errors.md`)

```convert
rule: atis.imperative
what: infinitive root without subject, aux and mark — imperative
from: atis
to: mova
match: v[upos=VERB, head=0, feats.VerbForm=Inf]
require: none c[rel~nsubj, head=v]; none c[rel~csubj, head=v]; none c[rel=expl, head=v]; none c[rel~aux, head=v]; none c[rel=mark, head=v]
set: v[feats+=VerbForm=Fin]; v[feats+=Mood=Imp]
source: EWT 2.18; UD en feat/Mood
```

```convert
rule: atis.imperative-conj
what: conjunct of an imperative without its own subject and aux — also imperative
from: atis
to: mova
match: v[head=0, feats.Mood=Imp]; w[upos=VERB, rel=conj, head=v, feats.VerbForm=Inf]
require: none c[rel~nsubj, head=w]; none c[rel~aux, head=w]; none c[rel=mark, head=w]
set: w[feats+=VerbForm=Fin]; w[feats+=Mood=Imp]
source: EWT 2.18
```

```convert
rule: atis.agree-pron
what: finite verb without Number — number and person from a pronoun subject
from: atis
to: mova
match: v[feats.VerbForm=Fin, feats.Tense, !feats.Number]; s[rel=nsubj|nsubj:pass, head=v, upos=PRON, feats.Person]
set: v[feats+=Number=@s]; v[feats+=Person=@s]
source: EWT 2.18 (Number/Person from the subject); UD docs/changes.md No. 18 (2.19)
```

```convert
rule: atis.agree-noun
what: finite verb without Number — number from a noun subject, 3rd person
from: atis
to: mova
match: v[feats.VerbForm=Fin, feats.Tense, !feats.Number]; s[rel=nsubj|nsubj:pass, head=v, upos=NOUN|PROPN|NUM]
set: v[feats+=Number=@s]; v[feats+=Person=3]
source: EWT 2.18
```

```convert
rule: atis.agree-aux-pron
what: aux/cop without Number — from the head's subject (pronoun)
from: atis
to: mova
match: h[]; s[rel=nsubj|nsubj:pass, head=h, upos=PRON, feats.Person]; a[rel=aux|cop|aux:pass, head=h, feats.VerbForm=Fin, feats.Tense, !feats.Number]
set: a[feats+=Number=@s]; a[feats+=Person=@s]
source: EWT 2.18 («for aux and cop — the head's subject»)
```

```convert
rule: atis.agree-aux-noun
what: aux/cop without Number — from the head's subject (noun)
from: atis
to: mova
match: h[]; s[rel=nsubj|nsubj:pass, head=h, upos=NOUN|PROPN|NUM]; a[rel=aux|cop|aux:pass, head=h, feats.VerbForm=Fin, feats.Tense, !feats.Number]
set: a[feats+=Number=@s]; a[feats+=Person=3]
source: EWT 2.18
```

```convert
rule: atis.agree-conj
what: conjoined predicate without its own subject — features of the first conjunct
from: atis
to: mova
match: h[feats.Number, feats.Person]; v[rel=conj, head=h, feats.VerbForm=Fin, feats.Tense, !feats.Number]
require: none c[rel~nsubj, head=v]
set: v[feats+=Number=@h]; v[feats+=Person=@h]
source: («for conj — the subject of the first conjunct»)
```

```convert
rule: atis.modal
what: modal without features — VerbForm=Fin
from: atis
to: mova
match: m[upos=AUX, lemma=will|would|can|could|may|might|shall|should|must, !feats.VerbForm]
set: m[feats+=VerbForm=Fin]
source: UD en pos/AUX (MD: only VerbForm=Fin)
```

```convert
rule: atis.voice
what: participle with aux:pass or in acl — Voice=Pass
from: atis
to: mova
match: v[feats.VerbForm=Part, feats.Tense=Past]
require: exists c[rel=aux:pass, head=v] or v[rel=acl]
set: v[feats+=Voice=Pass]
source: EWT README v2.13 (#290); EWT: 558 of 567 VBN in acl have Voice=Pass
```

```convert
rule: atis.pass-subj
what: subject with aux:pass — nsubj:pass
from: atis
to: mova
match: v[]; a[rel=aux:pass, head=v]; s[rel=nsubj, head=v]
set: s[rel=nsubj:pass]
source: UD en nsubj:pass
```

```convert
rule: atis.obl-agent
what: by phrase with a passive — obl:agent
from: atis
to: mova
match: v[feats.Voice=Pass]; o[rel=obl, head=v]; b[form=by, rel=case, head=o]
set: o[rel=obl:agent]
source: EWT README v2.13
```

Matches on the source ATIS: `imperative` 1709, `agree-pron` 504, `agree-noun` 256, `agree-aux-pron` 142, `agree-aux-noun` 327, `modal` 782, `voice` 11, `pass-subj` 10. `imperative-conj`, `agree-conj` and `obl-agent` give 0 on the source data: they rely on features set by previous rules (`Mood=Imp`, Number of the first conjunct, `Voice=Pass`). After them — up to 36 conjuncts without their own subject (on a head with a subject) and 12 *by* phrases.

### 3. Relative clauses (`seeds/relative-that-mark.md`, `seeds/acl-relcl-participle.md`)

```convert
rule: atis.relcl-participle
what: participle in acl:relcl without subject and mark — acl
from: atis
to: mova
match: v[rel=acl:relcl, feats.VerbForm=Part]
require: none c[rel~nsubj, head=v]; none c[rel=mark, head=v]
set: v[rel=acl]
source: UD en acl
```

```convert
rule: atis.relcl-mark-subj
what: relative that/which as mark → pronoun subject, if there is no subject
from: atis
to: mova
match: v[rel=acl:relcl]; t[form=that|which, rel=mark, head=v, before=v]
require: none c[rel~nsubj, head=v]
set: t[rel=nsubj]; t[upos=PRON]; t[feats+=PronType=Rel]
source: UD en acl:relcl; EWT 2.18
```

```convert
rule: atis.relcl-mark-obj
what: relative that/which as mark on a verb with a subject → obj
from: atis
to: mova
match: v[rel=acl:relcl]; t[form=that|which, rel=mark, head=v, before=v]
require: exists c[rel~nsubj, head=v]; none c[rel=obj, head=v]
set: t[rel=obj]; t[upos=PRON]; t[feats+=PronType=Rel]
source: UD en acl:relcl
```

```convert
rule: atis.relcl-pron
what: relative that/which argument with UPOS ADP/DET — PRON, PronType=Rel
from: atis
to: mova
match: v[rel=acl:relcl]; t[form=that|which, rel=nsubj|nsubj:pass|obj|obl, head=v, before=v, upos=ADP|DET]
set: t[upos=PRON]; t[feats+=PronType=Rel]
source: UD en pos/PRON
```

```convert
rule: atis.agree-relcl
what: predicate of a relative clause with subject that/which — number of the antecedent
from: atis
to: mova
match: n[upos=NOUN|PROPN]; v[rel=acl:relcl, head=n, feats.VerbForm=Fin, feats.Tense, !feats.Number]; t[rel=nsubj|nsubj:pass, head=v, feats.PronType=Rel]
set: v[feats+=Number=@n]; v[feats+=Person=3]
source: («for a relative pronoun — the antecedent»)
```

Matches: `relcl-participle` 662, `relcl-mark-subj` 278, `relcl-mark-obj` 13 (2 of the 293 `mark` fall under neither: the verb has both a subject and an `obj`), `relcl-pron` 102. `agree-relcl` on the source data — 0, after `relcl-mark-subj` — up to 56 relative-clause predicates without a subject.

### 4. Function words (`seeds/pos-feats-from-ptb.md`)

```convert
rule: atis.def
what: the — Definite=Def
from: atis
to: mova
match: d[form=the, upos=DET]
set: d[feats+=Definite=Def]
source: UD en feat/Definite
```

```convert
rule: atis.indef
what: a/an — Definite=Ind
from: atis
to: mova
match: d[form=a|an, upos=DET]
set: d[feats+=Definite=Ind]
source: UD en feat/Definite
```

```convert
rule: atis.dem-sing
what: this/that with PronType=Art — singular demonstrative
from: atis
to: mova
match: d[form=this|that, feats.PronType=Art]
set: d[feats+=PronType=Dem]; d[feats+=Number=Sing]
source: UD en feat/PronType; EWT 2.18
```

```convert
rule: atis.dem-plur
what: these/those with PronType=Art — plural demonstrative
from: atis
to: mova
match: d[form=these|those, feats.PronType=Art]
set: d[feats+=PronType=Dem]; d[feats+=Number=Plur]
source: UD en feat/PronType
```

```convert
rule: atis.dem-pron
what: demonstrative DET as a noun — PRON
from: atis
to: mova
match: t[form=this|that|these|those, upos=DET, rel=nsubj|nsubj:pass|obj|iobj|obl|root|conj]
set: t[upos=PRON]
source: EWT README v1.2 (this/that/these/those → PRON)
```

```convert
rule: atis.sconj
what: subordinating conjunction with UPOS ADP — SCONJ
from: atis
to: mova
match: m[rel=mark, upos=ADP, form=that|if|because|whether|although|though|unless|while|since|as]
set: m[upos=SCONJ]
source: EWT README v1.2 (ADP → SCONJ); UD en pos/SCONJ
```

```convert
rule: atis.poss-my
what: my — PRON, lemma my, Case=Gen, Person=1
from: atis
to: mova
match: p[form=my, upos=DET]
set: p[upos=PRON]; p[lemma=@form]; p[feats+=Case=Gen]; p[feats+=Person=1]
source: EWT README v2.11 (lemmas and features of possessives, docs#517)
```

```convert
rule: atis.poss-your
what: your — PRON, lemma your, Case=Gen, Person=2, without Number
from: atis
to: mova
match: p[form=your, upos=DET]
set: p[upos=PRON]; p[lemma=@form]; p[feats+=Case=Gen]; p[feats+=Person=2]; p[feats-=Number]
source: EWT README v2.11
```

```convert
rule: atis.poss-their
what: their/our/his/her/its — PRON, lemma = form, Case=Gen, Person=3 (our — 1)
from: atis
to: mova
match: p[form=their|his|her|its, upos=DET]
set: p[upos=PRON]; p[lemma=@form]; p[feats+=Case=Gen]; p[feats+=Person=3]
source: EWT README v2.11
```

```convert
rule: atis.not
what: not/n't as ADV — PART with Polarity=Neg
from: atis
to: mova
match: n[form=not|n't, upos=ADV]
set: n[upos=PART]; n[feats+=Polarity=Neg]
source: EWT README v2.15
```

```convert
rule: atis.numform-digit
what: number in digits — NumForm=Digit
from: atis
to: mova
match: n[upos=NUM, suffix=0|1|2|3|4|5|6|7|8|9]
set: n[feats+=NumForm=Digit]
source: EWT README v2.13
```

```convert
rule: atis.numform-word
what: number in words — NumForm=Word
from: atis
to: mova
match: n[upos=NUM, !feats.NumForm]
set: n[feats+=NumForm=Word]
source: EWT README v2.13
```

```convert
rule: atis.adv-degree
what: Degree=Pos only on adverbs that have it in EWT
from: atis
to: mova
match: a[upos=ADV, feats.Degree=Pos, lemma!=well|far|soon|long|hard|early|late|little|close|high|fast|badly|low]
set: a[feats-=Degree]
source: EWT 2.18 (ADV with Degree=Pos — only these lemmas)
```

```convert
rule: atis.you
what: you without Number (after the agreement rules, which take Sing from it)
from: atis
to: mova
match: p[form=you, upos=PRON]
set: p[feats-=Number]
source: EWT 2.18
```

Matches: `def` + `indef` 3341, `dem-sing` + `dem-plur` 67, `dem-pron` 37, `sconj` 286, `poss-*` 35, `not` 4, `numform-digit` 830 and `numform-word` 361 (together 1191), `adv-degree` 325, `you` 223.

```convert-todo
rule: atis.int-rel
what: PronType=Int,Rel → Rel in a relative clause, otherwise Int
from: atis
to: mova
match: w[feats.PronType=Int,Rel]
set: w[feats+=PronType=Int]
source: EWT 2.18
```

Needs a comma in a feature value: currently `,` separates node conditions. There is a workaround for `match` — `w[feats.PronType, feats.PronType!=Int|Rel|Prs|Art|Dem|Ind|Neg|Tot|Rcp|Emp]`, as in `seeds/pos-feats-from-ptb.md`. For the reverse action `feats+=PronType=Int,Rel` there is no workaround. The relative variant is the same with `v[rel=acl:relcl]` and `w[head=v]` → `PronType=Rel`. 1793 words.

```convert-todo
rule: atis.extpos
what: ExtPos on fixed heads — by expression
from: atis
to: mova
match: h[form=out|instead|so|at|as|all]; f[rel=fixed, head=h, next=h]
set: h[feats+=ExtPos=@expr]
source: UD 2.16, ExtPos
```

Needs an "expression → ExtPos" dictionary: *out of*, *instead of* → ADP; *so that* → SCONJ; *at least*, *as well*, *all right* → ADV. These are six rules with `form=` on both nodes, they can be written even now. But *stand for*, *or as*, *from which* in `fixed` in ATIS are errors; there will be no rules for them. 22 heads.

**Not converted:**
- **Names** (`seeds/names-flat.md`). *new york* with head *new* → *York* with `amod` *New* requires moving the head together with its dependents, i.e. a `rel=@v` action — copy another node's relation. We keep `flat`: since 2.17 EWT also puts `flat` on place names.
- **XPOS.** ATIS does not have it, so the sentences do not train the PTB tagger, as `en::ud` already does.
- **`VerbForm=Ger`.** A gerund cannot be distinguished from a participle by form.
- **Errors from `seeds/errors.md`**, except the systematic `nsubj` in the passive.

## mova → atis

The rules predict the ATIS custom by form and role, without a trace in MISC.

```convert
rule: mova.atis.imperative
what: imperative → VerbForm=Inf without Mood
from: mova
to: atis
match: v[feats.Mood=Imp]
set: v[feats-=Mood]; v[feats+=VerbForm=Inf]
source: seeds/verb-feats.md
```

```convert
rule: mova.atis.agree-pres
what: present without -s — without Number and Person
from: mova
to: atis
match: v[feats.VerbForm=Fin, feats.Tense=Pres, suffix!=s]
set: v[feats-=Number]; v[feats-=Person]
source: seeds/verb-feats.md (ATIS: Number/Person only on VBZ)
```

```convert
rule: mova.atis.agree-past
what: past — without Number and Person
from: mova
to: atis
match: v[feats.VerbForm=Fin, feats.Tense=Past]
set: v[feats-=Number]; v[feats-=Person]
source: seeds/verb-feats.md
```

```convert
rule: mova.atis.modal
what: modal — without features
from: mova
to: atis
match: m[upos=AUX, feats.VerbForm=Fin, !feats.Tense, !feats.Mood]
set: m[feats-=VerbForm]
source: seeds/verb-feats.md
```

```convert
rule: mova.atis.voice
what: without Voice, nsubj:pass → nsubj, obl:agent → obl
from: mova
to: atis
match: v[feats.Voice=Pass]
set: v[feats-=Voice]
source: seeds/errors.md
```

```convert
rule: mova.atis.pass-subj
what: nsubj:pass → nsubj
from: mova
to: atis
match: s[rel=nsubj:pass]
set: s[rel=nsubj]
source: seeds/errors.md
```

```convert
rule: mova.atis.obl-agent
what: obl:agent → obl
from: mova
to: atis
match: o[rel=obl:agent]
set: o[rel=obl]
source: seeds/errors.md
```

```convert
rule: mova.atis.tmod-np
what: nmod:unmarked → nmod:tmod
from: mova
to: atis
match: t[rel=nmod:unmarked]
set: t[rel=nmod:tmod]
source: seeds/tmod.md
```

```convert
rule: mova.atis.tmod-pp
what: nmod with a preposition on a time word → nmod:tmod
from: mova
to: atis
match: t[rel=nmod, lemma=morning|afternoon|evening|night|noon|pm|am|monday|tuesday|wednesday|thursday|friday|saturday|sunday|january|february|march|april|may|june|july|august|september|october|november|december|today|tomorrow|day|week|time]
require: exists c[rel=case, head=t]
set: t[rel=nmod:tmod]
source: seeds/tmod.md
```

```convert
rule: mova.atis.unmarked-pp
what: obl with a preposition on a time word → obl:unmarked
from: mova
to: atis
match: t[rel=obl, lemma=morning|afternoon|evening|night|noon|pm|am|monday|tuesday|wednesday|thursday|friday|saturday|sunday|january|february|march|april|may|june|july|august|september|october|november|december|today|tomorrow|day|week|time]
require: exists c[rel=case, head=t]
set: t[rel=obl:unmarked]
source: seeds/tmod.md
```

```convert
rule: mova.atis.def
what: article without Definite
from: mova
to: atis
match: d[form=a|an|the, upos=DET]
set: d[feats-=Definite]
source: seeds/pos-feats-from-ptb.md
```

```convert
rule: mova.atis.dem
what: demonstrative DET — PronType=Art without Number
from: mova
to: atis
match: d[upos=DET, feats.PronType=Dem, form=this|that|these|those]
set: d[feats+=PronType=Art]; d[feats-=Number]
source: seeds/pos-feats-from-ptb.md
```

```convert
rule: mova.atis.dem-pron
what: demonstrative PRON — DET with PronType=Art
from: mova
to: atis
match: t[upos=PRON, form=this|that|these|those, feats.PronType=Dem]
set: t[upos=DET]; t[feats+=PronType=Art]; t[feats-=Number]
source: seeds/pos-feats-from-ptb.md (90 of 125 — DET)
```

```convert
rule: mova.atis.relcl-that
what: relative that argument → mark ADP (as in 293 of 463)
from: mova
to: atis
match: v[rel=acl:relcl]; t[form=that, rel=nsubj|nsubj:pass|obj, head=v, before=v]
set: t[rel=mark]; t[upos=ADP]; t[feats-=PronType]
source: seeds/relative-that-mark.md
```

```convert
rule: mova.atis.sconj
what: SCONJ → ADP
from: mova
to: atis
match: m[upos=SCONJ]
set: m[upos=ADP]
source: seeds/pos-feats-from-ptb.md
```

```convert
rule: mova.atis.poss
what: possessive modifier — DET, lemma = personal pronoun, without Case and Person
from: mova
to: atis
match: p[upos=PRON, rel=nmod:poss, feats.Poss=Yes, form=my|your|their|our|his|her|its]
set: p[upos=DET]; p[feats-=Case]; p[feats-=Person]
source: seeds/pos-feats-from-ptb.md
```

```convert
rule: mova.atis.poss-lemma-my
what: my — lemma I, as in ATIS
from: mova
to: atis
match: p[form=my, rel=nmod:poss]
set: p[lemma=I]
source: seeds/pos-feats-from-ptb.md
```

```convert
rule: mova.atis.poss-lemma-your
what: your — lemma you, as in ATIS
from: mova
to: atis
match: p[form=your, rel=nmod:poss]
set: p[lemma=you]
source: seeds/pos-feats-from-ptb.md
```

```convert
rule: mova.atis.poss-lemma-their
what: their — lemma they, as in ATIS
from: mova
to: atis
match: p[form=their, rel=nmod:poss]
set: p[lemma=they]
source: seeds/pos-feats-from-ptb.md
```

```convert
rule: mova.atis.not
what: not — ADV without Polarity
from: mova
to: atis
match: n[form=not|n't, upos=PART]
set: n[upos=ADV]; n[feats-=Polarity]
source: seeds/pos-feats-from-ptb.md
```

```convert
rule: mova.atis.numform
what: without NumForm
from: mova
to: atis
match: n[feats.NumForm]
set: n[feats-=NumForm]
source: seeds/pos-feats-from-ptb.md
```

```convert
rule: mova.atis.extpos
what: without ExtPos
from: mova
to: atis
match: h[feats.ExtPos]
set: h[feats-=ExtPos]
source: seeds/pos-feats-from-ptb.md
```

```convert
rule: mova.atis.adv-degree
what: adverb, except interrogatives and not — Degree=Pos
from: mova
to: atis
match: a[upos=ADV, !feats.Degree, !feats.PronType]
set: a[feats+=Degree=Pos]
source: seeds/pos-feats-from-ptb.md
```

```convert
rule: mova.atis.you
what: you — Number=Sing
from: mova
to: atis
match: p[form=you, upos=PRON]
set: p[feats+=Number=Sing]
source: seeds/verb-feats.md
```

```convert
rule: mova.atis.relcl-participle
what: participle in acl → acl:relcl
from: mova
to: atis
match: v[rel=acl, feats.VerbForm=Part]
set: v[rel=acl:relcl]
source: seeds/acl-relcl-participle.md
```

```convert
rule: mova.atis.xpos
what: ATIS has no XPOS
from: mova
to: atis
match: t[]
set: t[xpos=_]
source: seeds/data.md
```

```convert-todo
rule: mova.atis.int-rel
what: PronType=Int and Rel → Int,Rel
from: mova
to: atis
match: w[feats.PronType=Int|Rel]
set: w[feats+=PronType=Int,Rel]
source: seeds/pos-feats-from-ptb.md
```

Matches on EWT (`mova → atis`): `imperative` 1663, `agree-pres` 6851, `agree-past` 6456, `modal` 4050, `voice` 3255, `pass-subj` 1445, `obl-agent` 376, `tmod-np` 1360, `tmod-pp` 199, `unmarked-pp` 546, `def` 16 365, `dem` 1397, `dem-pron` 994, `relcl-that` 576, `sconj` 4600, `poss` 3594, `not` 2077, `numform` 5608, `extpos` 869, `adv-degree` 8907, `relcl-participle` 584, `you` 2768. This is for estimating volume: on our annotator's output for ATIS text the numbers will differ.

**Needs actions that do not exist:**
- a comma in a feature value: `PronType=Int,Rel`;
- `xpos=_` — clear the column (if `xpos=` accepts only PTB tags);

## Round trip atis → mova → atis

**Lossless** (each forward rule has an exact reverse, and ATIS keeps its custom consistently):
- imperative — 1709 roots and their conjuncts;
- agreement. On the way there Number/Person are set from the subject, on the way back they are removed from forms without *-s* and from the past. This restores ATIS exactly: it has Number only on VBZ. 59 present-tense verbs without Number have a singular noun subject. They will get `Sing|3`, but on the way back will lose Number again, because their form has no *-s*;
- modals, `Voice`, `nsubj:pass`, `obl:agent`, `Definite`, `NumForm`, `ExtPos`, `Polarity`, *you*;
- `tmod-np` / `unmarked-np` (NP without a preposition);
- demonstrative DET (67), conjunctions (286), possessives with lemmas (35: `poss-lemma-*` restore *I*, *you*, *they*).

**Lossy:**
- **Temporal phrases with a preposition.** ATIS itself is inconsistent: 1710 PPs with the subtype and 210 without. Prediction by lemma will give the subtype to those 210 as well, and lemmas outside the list (*o'clock*, *hours*…) will remain without it. Expect ≈ 200–300 DEPREL divergences.
- **Relative *that/which*.** On the way back we apply the majority custom: *that* — `mark` ADP. DEPREL and UPOS will diverge for *that* that were `nsubj` with ADP/DET/PRON in ATIS (94), and for *which*-`mark` (18): on the way there it becomes `nsubj`, and there is no reverse rule for *which*. Together ≈ 110.
- **Demonstrative pronouns.** 35 of 125 in ATIS were PRON; on the way back they become DET.
- **`Degree` on adverbs.** ATIS puts `Degree=Pos` not on all ADV. The reverse rule will give it to all except interrogatives, so expect a few dozen FEATS divergences.
- **`PronType=Int,Rel`** — 1793 words, until the language accepts a comma in a value. Then lossless.
- **Agreement without a subject.** About 260 finite verbs will get Number/Person neither from their own subject, nor from the head, nor from the first conjunct. The round trip does not lose from this: on the way back they are without Number anyway. But in `mova` these sentences will be incomplete with respect to agreement.

Total: DEPREL and UPOS should agree on ≥ 99% of tokens, FEATS on ≥ 96%, until the comma rule.
