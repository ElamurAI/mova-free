# ParTUT ↔ mova — converter draft

The rule language is `train/dialects-and-converters.md`. Conventions beyond the specification are in `../seeds-overview.md`. Matches were counted by `en expert-check`: `partut → mova` on all parts of ParTUT, `mova → partut` on EWT.

ParTUT is NC, so for `mova` it is test and research. The main direction is `mova → partut`, for honest evaluation. `partut → mova` is needed for the round trip and for the two roads to the goal.

## partut → mova

### 1. Compound nouns (`seeds/nmod-compound.md`)

```convert
rule: partut.nn-compound
what: noun modifier before a noun as nmod without a preposition → compound
from: partut
to: mova
match: h[upos=NOUN|PROPN]; n[upos=NOUN, rel=nmod, head=h, before=h]
require: none c[rel=case, head=n]; none c[rel=case|det|nmod:poss, head=h, after=n, before=h]
set: n[rel=compound]
source: UD en compound
```


### 2. Modals and agreement (`seeds/feats.md`)

Modals first: otherwise their `Person=3` would pass as agreement.


```convert
rule: partut.modal
what: modal — only VerbForm=Fin
from: partut
to: mova
match: m[upos=AUX, lemma=will|would|can|could|may|might|shall|should|must]
set: m[feats-=Person]; m[feats-=Tense]; m[feats-=Mood]; m[feats-=Number]
source: UD en pos/AUX
```

The agreement rules are as in `../atis/convert.md`, but with the condition `!feats.Person` rather than `!feats.Number`: *are*, *have* in ParTUT have `Number=Plur` without `Person`. `feats+=Number=@s` also replaces an existing Number.


```convert
rule: partut.agree-pron
what: finite verb without Number — number and person from a pronoun subject
from: partut
to: mova
match: v[feats.VerbForm=Fin, feats.Tense, !feats.Person]; s[rel=nsubj|nsubj:pass, head=v, upos=PRON, feats.Person]
set: v[feats+=Number=@s]; v[feats+=Person=@s]
source: EWT 2.18 (Number/Person from the subject); UD docs/changes.md No. 18 (2.19)
```

```convert
rule: partut.agree-noun
what: finite verb without Number — number from a noun subject, 3rd person
from: partut
to: mova
match: v[feats.VerbForm=Fin, feats.Tense, !feats.Person]; s[rel=nsubj|nsubj:pass, head=v, upos=NOUN|PROPN|NUM]
set: v[feats+=Number=@s]; v[feats+=Person=3]
source: EWT 2.18
```

```convert
rule: partut.agree-aux-pron
what: aux/cop without Number — from the head's subject (pronoun)
from: partut
to: mova
match: h[]; s[rel=nsubj|nsubj:pass, head=h, upos=PRON, feats.Person]; a[rel=aux|cop|aux:pass, head=h, feats.VerbForm=Fin, feats.Tense, !feats.Person]
set: a[feats+=Number=@s]; a[feats+=Person=@s]
source: EWT 2.18 («for aux and cop — the head's subject»)
```

```convert
rule: partut.agree-aux-noun
what: aux/cop without Number — from the head's subject (noun)
from: partut
to: mova
match: h[]; s[rel=nsubj|nsubj:pass, head=h, upos=NOUN|PROPN|NUM]; a[rel=aux|cop|aux:pass, head=h, feats.VerbForm=Fin, feats.Tense, !feats.Person]
set: a[feats+=Number=@s]; a[feats+=Person=3]
source: EWT 2.18
```

```convert
rule: partut.agree-conj
what: conjoined predicate without its own subject — features of the first conjunct
from: partut
to: mova
match: h[feats.Number, feats.Person]; v[rel=conj, head=h, feats.VerbForm=Fin, feats.Tense, !feats.Person]
require: none c[rel~nsubj, head=v]
set: v[feats+=Number=@h]; v[feats+=Person=@h]
source: («for conj — the subject of the first conjunct»)
```

```convert
rule: partut.agree-relcl
what: predicate of a relative clause with subject that/which — number of the antecedent
from: partut
to: mova
match: n[upos=NOUN|PROPN]; v[rel=acl:relcl, head=n, feats.VerbForm=Fin, feats.Tense, !feats.Person]; t[rel=nsubj|nsubj:pass, head=v, feats.PronType=Rel]
set: v[feats+=Number=@n]; v[feats+=Person=3]
source: («for a relative pronoun — the antecedent»)
```


### 3. Case, voice, numbers (`seeds/feats.md`)


```convert
rule: partut.case-nom
what: personal pronouns in the nominative — Case=Nom
from: partut
to: mova
match: p[upos=PRON, form=i|he|she|we|they, !feats.Case]
set: p[feats+=Case=Nom]
source: EWT 2.18; UD en feat/Case
```

```convert
rule: partut.case-acc
what: personal pronouns in the oblique — Case=Acc
from: partut
to: mova
match: p[upos=PRON, form=me|him|us|them, !feats.Case]
set: p[feats+=Case=Acc]
source: EWT 2.18
```

```convert
rule: partut.case-it-you-nom
what: it/you as subject — Case=Nom
from: partut
to: mova
match: p[upos=PRON, form=it|you, !feats.Case, rel=nsubj|nsubj:pass|expl|nsubj:outer]
set: p[feats+=Case=Nom]
source: EWT 2.17 (#589)
```

```convert
rule: partut.case-it-you-acc
what: it/you not as subject — Case=Acc
from: partut
to: mova
match: p[upos=PRON, form=it|you|her, !feats.Case, feats.PronType=Prs, rel!=nmod:poss]
set: p[feats+=Case=Acc]
source: EWT 2.17 (#589)
```

```convert
rule: partut.voice
what: participle with aux:pass or in acl — Voice=Pass
from: partut
to: mova
match: v[feats.VerbForm=Part, feats.Tense=Past]
require: exists c[rel=aux:pass, head=v] or v[rel=acl]
set: v[feats+=Voice=Pass]
source: EWT README v2.13
```

```convert
rule: partut.pass-subj
what: subject with aux:pass — nsubj:pass
from: partut
to: mova
match: v[]; a[rel=aux:pass, head=v]; s[rel=nsubj, head=v]
set: s[rel=nsubj:pass]
source: UD en nsubj:pass
```

```convert
rule: partut.numform-digit
what: number in digits — NumForm=Digit
from: partut
to: mova
match: n[upos=NUM, suffix=0|1|2|3|4|5|6|7|8|9]
set: n[feats+=NumForm=Digit]
source: EWT README v2.13
```

```convert
rule: partut.numform-word
what: number in words — NumForm=Word
from: partut
to: mova
match: n[upos=NUM, !feats.NumForm]
set: n[feats+=NumForm=Word]
source: EWT README v2.13
```

```convert
rule: partut.propn-number
what: proper noun without Number — Number=Sing
from: partut
to: mova
match: n[upos=PROPN, !feats.Number]
set: n[feats+=Number=Sing]
source: EWT 2.18 (PROPN with Number — 16 087 of 16 562)
```


### 4. Possessives, lemmas, structures (`seeds/structures.md`, `seeds/errors.md`)


```convert
rule: partut.poss-my
what: my — PRON, lemma my, Case=Gen
from: partut
to: mova
match: p[form=my, upos=DET, rel=nmod:poss]
set: p[upos=PRON]; p[lemma=@form]; p[feats+=Case=Gen]; p[feats+=Person=1]; p[feats+=Number=Sing]
source: EWT README v2.11 (docs#517)
```

```convert
rule: partut.poss-your
what: your — PRON, lemma your, Case=Gen
from: partut
to: mova
match: p[form=your, upos=DET, rel=nmod:poss]
set: p[upos=PRON]; p[lemma=@form]; p[feats+=Case=Gen]; p[feats+=Person=2]
source: EWT README v2.11 (docs#517)
```

```convert
rule: partut.poss-his
what: his — PRON, lemma his, Case=Gen
from: partut
to: mova
match: p[form=his, upos=DET, rel=nmod:poss]
set: p[upos=PRON]; p[lemma=@form]; p[feats+=Case=Gen]; p[feats+=Person=3]; p[feats+=Number=Sing]
source: EWT README v2.11 (docs#517)
```

```convert
rule: partut.poss-her
what: her — PRON, lemma her, Case=Gen
from: partut
to: mova
match: p[form=her, upos=DET, rel=nmod:poss]
set: p[upos=PRON]; p[lemma=@form]; p[feats+=Case=Gen]; p[feats+=Person=3]; p[feats+=Number=Sing]
source: EWT README v2.11 (docs#517)
```

```convert
rule: partut.poss-its
what: its — PRON, lemma its, Case=Gen
from: partut
to: mova
match: p[form=its, upos=DET, rel=nmod:poss]
set: p[upos=PRON]; p[lemma=@form]; p[feats+=Case=Gen]; p[feats+=Person=3]; p[feats+=Number=Sing]
source: EWT README v2.11 (docs#517)
```

```convert
rule: partut.poss-our
what: our — PRON, lemma our, Case=Gen
from: partut
to: mova
match: p[form=our, upos=DET, rel=nmod:poss]
set: p[upos=PRON]; p[lemma=@form]; p[feats+=Case=Gen]; p[feats+=Person=1]; p[feats+=Number=Plur]
source: EWT README v2.11 (docs#517)
```

```convert
rule: partut.poss-their
what: their — PRON, lemma their, Case=Gen
from: partut
to: mova
match: p[form=their, upos=DET, rel=nmod:poss]
set: p[upos=PRON]; p[lemma=@form]; p[feats+=Case=Gen]; p[feats+=Person=3]; p[feats+=Number=Plur]
source: EWT README v2.11 (docs#517)
```

```convert
rule: partut.lemma-me
what: me/i → lemma I
from: partut
to: mova
match: p[form=me|i, upos=PRON]
set: p[lemma=I]
source: EWT README v2.11
```

```convert
rule: partut.lemma-us
what: us → lemma we
from: partut
to: mova
match: p[form=us, upos=PRON]
set: p[lemma=we]
source: EWT README v2.11
```

```convert
rule: partut.lemma-them
what: them → lemma they
from: partut
to: mova
match: p[form=them, upos=PRON]
set: p[lemma=they]
source: EWT README v2.11
```

```convert
rule: partut.lemma-self
what: themselves — lemma themselves
from: partut
to: mova
match: p[form=themselves|ourselves|yourselves, upos=PRON]
set: p[lemma=@form]
source: EWT README v2.11
```

```convert
rule: partut.obl-agent
what: by phrase with a passive — obl:agent
from: partut
to: mova
match: v[feats.Voice=Pass]; o[rel=obl, head=v]; b[form=by, rel=case, head=o]
set: o[rel=obl:agent]
source: EWT README v2.13
```

```convert
rule: partut.wh-advmod
what: subordinate when/where/how/why as mark → ADV advmod
from: partut
to: mova
match: w[form=when|where|how|why, rel=mark]
set: w[upos=ADV]; w[rel=advmod]
source: EWT README v2.11 (#88)
```

```convert
rule: partut.flat-foreign
what: flat:foreign → flat + Foreign=Yes
from: partut
to: mova
match: f[rel=flat:foreign]
set: f[rel=flat]; f[feats+=Foreign=Yes]
source: EWT README v2.13 (#459)
```


Matches on ParTUT (on the source data, before the effect of previous rules):
- `nn-compound` 915 (of 948 `nmod` nouns before a noun, 33 have a preposition or determiner between them);
- `modal` 690;
- agreement: `agree-pron` 152, `agree-noun` 165, `agree-aux-pron` 102, `agree-aux-noun` 200, `agree-relcl` 37, `agree-conj` 1;
- `case-nom` 652, `case-acc` 121, `case-it-you-nom` 290, `case-it-you-acc` 380 (of them ≈ 90 remain after `nom`);
- `voice` 787, `pass-subj` 1, `obl-agent` 0 → ≈ 141 after `voice`;
- `numform-digit` 591, `numform-word` 226 (817 − 591), `propn-number` 2148;
- `poss-*` 639 (*his* 249, *their* 116, *its* 101, *our* 60, *your* 50, *my* 39, *her* 24);
- lemmas: `lemma-me` 200, `lemma-us` 28, `lemma-them` 46, `lemma-self` 12;
- `wh-advmod` 70, `flat-foreign` 88.

**Not converted:** TUT XPOS (`seeds/xpos-tut.md`) — the engine neither reads nor writes tags outside PTB. `VerbForm=Ger`. Errors from `seeds/errors.md`, except the systematic pronoun lemmas.

## mova → partut


```convert
rule: mova.partut.nn-nmod
what: compound between adjacent nouns → nmod
from: mova
to: partut
match: h[upos=NOUN|PROPN]; n[upos=NOUN, rel=compound, head=h]
set: n[rel=nmod]
source: seeds/nmod-compound.md
```

```convert
rule: mova.partut.feats
what: without Case, Voice, NumForm
from: mova
to: partut
match: t[]
set: t[feats-=Case]; t[feats-=Voice]; t[feats-=NumForm]
source: seeds/feats.md
```

```convert
rule: mova.partut.propn
what: PROPN without Number
from: mova
to: partut
match: n[upos=PROPN]
set: n[feats-=Number]
source: seeds/feats.md
```

```convert
rule: mova.partut.modal-pres
what: present modal — Mood=Ind|Person=3|Tense=Pres
from: mova
to: partut
match: m[upos=AUX, lemma=will|can|shall|may|must]
set: m[feats+=Mood=Ind]; m[feats+=Person=3]; m[feats+=Tense=Pres]
source: seeds/feats.md
```

```convert
rule: mova.partut.modal-past
what: past modal — Mood=Ind|Person=3|Tense=Past
from: mova
to: partut
match: m[upos=AUX, lemma=would|could|should|might]
set: m[feats+=Mood=Ind]; m[feats+=Person=3]; m[feats+=Tense=Past]
source: seeds/feats.md
```

```convert
rule: mova.partut.agree-pres
what: present without -s (except am) — Number=Plur without Person
from: mova
to: partut
match: v[feats.VerbForm=Fin, feats.Tense=Pres, suffix!=s, form!=am|'m]
set: v[feats+=Number=Plur]; v[feats-=Person]
source: seeds/feats.md (are, have — Number=Plur)
```

```convert
rule: mova.partut.agree-past
what: past, except be — without Number/Person
from: mova
to: partut
match: v[feats.VerbForm=Fin, feats.Tense=Past, lemma!=be]
set: v[feats-=Number]; v[feats-=Person]
source: seeds/feats.md
```

```convert
rule: mova.partut.poss
what: possessive modifier — DET, without Case/Person/Number
from: mova
to: partut
match: p[upos=PRON, rel=nmod:poss, form=my|your|his|her|its|our|their]
set: p[upos=DET]; p[feats-=Case]; p[feats-=Person]; p[feats-=Number]
source: seeds/structures.md
```

```convert
rule: mova.partut.poss-lemma-our
what: our → lemma us
from: mova
to: partut
match: p[form=our, rel=nmod:poss]
set: p[lemma=us]
source: seeds/structures.md
```

```convert
rule: mova.partut.poss-lemma-your
what: your → lemma you
from: mova
to: partut
match: p[form=your, rel=nmod:poss]
set: p[lemma=you]
source: seeds/structures.md
```

```convert
rule: mova.partut.obl-agent
what: obl:agent → obl
from: mova
to: partut
match: o[rel=obl:agent]
set: o[rel=obl]
source: seeds/structures.md
```

```convert
rule: mova.partut.when
what: subordinate when on advcl — SCONJ mark
from: mova
to: partut
match: v[rel=advcl]; w[form=when, rel=advmod, head=v, before=v]
set: w[upos=SCONJ]; w[rel=mark]
source: seeds/structures.md
```

```convert
rule: mova.partut.flat-foreign
what: flat with Foreign=Yes → flat:foreign
from: mova
to: partut
match: f[rel=flat, feats.Foreign=Yes]
set: f[rel=flat:foreign]; f[feats-=Foreign]
source: seeds/structures.md
```

```convert-todo
rule: mova.partut.xpos
what: TUT XPOS from UPOS and FEATS: NOUN → S, VERB → V, ADP → E…
from: mova
to: partut
match: n[upos=NOUN]
set: n[xpos=S]
source: seeds/xpos-tut.md
```

Needs string XPOS. The table UPOS (+ `PronType`, `Definite`) → TUT tag is unambiguous for most classes: NOUN → `S`, ADJ → `A`, ADP → `E`, CCONJ → `CC`, SCONJ → `CS`, PROPN → `SP`, NUM → `N`, DET+Art+Def → `RD`, DET+Art+Ind → `RI`, possessive → `AP`, copula → `V`, other AUX → `VA`/`VM`.

Matches on EWT: `nn-nmod` 5466, `feats` 254 820, `propn` 16 562, `modal-pres` 2484, `modal-past` 1564, `agree-pres` 6308, `agree-past` 4493, `poss` 3594, `poss-lemma-our` 429, `poss-lemma-your` 813, `obl-agent` 376, `when` 401, `flat-foreign` 39.

## Round trip partut → mova → partut

**Lossless:**
- modals: `Tense` restored by lemma (*would/could/should/might* — Past). ParTUT writes exactly that: Pres 403, Past 279;
- `Case`, `Voice`, `NumForm` — ParTUT does not have them; set on the way there, removed on the way back;
- `obl:agent`, `flat:foreign` (88 → `flat` + `Foreign=Yes` → back), *when* on `advcl`;
- possessive DET: 639 there, all back.

**Lossy:**
- **Compound nouns.** On the way there 915 `nmod` → `compound`; on the way back all `compound` between nouns become `nmod`. ParTUT also has real `compound` — 61 adjacent and 124 in total. They will become `nmod` on the way back: ≈ 124 DEPREL divergences.
- **Agreement.** ParTUT puts `Number=Plur` on *are/have/do* even with an *I/you* subject, and Person only on 3sg. The reverse rule repeats this by form. Verbs where ParTUT is inconsistent will diverge: past with Number, except *be* — 2 (*put*, *found*). Together ≈ 10–30 FEATS divergences.
- **PROPN `Number`.** 82 PROPN in ParTUT had Number; on the way back we remove it from all: 82 FEATS divergences.
- **Possessive lemmas.** *his, their, its* stay as the form on the way back, as in ParTUT. *our* → *us*, *your* → *you* are restored by rules. *my* in ParTUT → *my*, a match.
- **Pronoun lemmas** *me* → *i* (9, lowercase) will become *I* on the way back: 9 divergences.
- **XPOS** outside the round trip until there is string XPOS.

Total: DEPREL ≈ 99.7% (compound nouns), FEATS ≈ 99.7%, lemmas ≈ 99.9%.
