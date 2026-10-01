# CHILDES ↔ mova — converter draft

The rule language is `train/dialects-and-converters.md`. Conventions beyond the specification are in `../seeds-overview.md`. Matches were counted by `en expert-check`: `childes → mova` on all parts of CHILDES, `mova → childes` on EWT.

CHILDES has no FEATS (`seeds/data.md`). The converter does not invent them: training skips the layer (`Sentence::has_feats`). The converter does not treat the 11,446 sentences with `gold_annotation = False` separately. Training should give them a weight, as silver data.

## childes → mova

### 1. `:npmod` → `:unmarked` (`seeds/npmod.md`)

```convert
rule: childes.unmarked-time
what: existing :unmarked in CHILDES — former :tmod, temporal
from: childes
to: mova
match: t[rel=obl:unmarked|nmod:unmarked]
set: t[misc+=TemporalNPAdjunct=Yes]
source: README UD_English-CHILDES v2.17 («Replace all :tmod with :unmarked»); EWT README v2.15
```

```convert
rule: childes.obl-npmod
what: obl:npmod → obl:unmarked (after the previous rule: without a time trace)
from: childes
to: mova
match: t[rel=obl:npmod]
set: t[rel=obl:unmarked]
source: UD 2.15, docs#1028
```

```convert
rule: childes.nmod-npmod
what: nmod:npmod → nmod:unmarked
from: childes
to: mova
match: t[rel=nmod:npmod]
set: t[rel=nmod:unmarked]
source: UD 2.15, docs#1028
```

### 2. Punctuation (`seeds/punct.md`)

```convert
rule: childes.punct-xpos
what: ? and ! have XPOS . (PTB)
from: childes
to: mova
match: p[upos=PUNCT, form=?|!]
set: p[xpos=.]
source: Santorini 1990, `.`
```

### 3. Pronoun lemmas (`seeds/pron-lemma.md`)

```convert
rule: childes.lemma-me
what: me → lemma I
from: childes
to: mova
match: p[form=me, upos=PRON]
set: p[lemma=I]
source: EWT README v2.11, docs#517
```

```convert
rule: childes.lemma-him
what: him → lemma he
from: childes
to: mova
match: p[form=him, upos=PRON]
set: p[lemma=he]
source: EWT README v2.11
```

```convert
rule: childes.lemma-them
what: them → lemma they
from: childes
to: mova
match: p[form=them, upos=PRON]
set: p[lemma=they]
source: EWT README v2.11
```

```convert
rule: childes.lemma-us
what: us → lemma we
from: childes
to: mova
match: p[form=us, upos=PRON]
set: p[lemma=we]
source: EWT README v2.11
```

```convert
rule: childes.lemma-poss
what: your/his/our/their (possessives) — lemma = form
from: childes
to: mova
match: p[form=your|his|our|their, upos=PRON, rel=nmod:poss]
set: p[lemma=@form]
source: EWT README v2.11
```

```convert
rule: childes.lemma-i
what: i → I
from: childes
to: mova
match: p[form=i, upos=PRON]
set: p[lemma=I]
source: EWT 2.18
```

### 4. Old customs (`seeds/old-ud.md`)

```convert
rule: childes.wh-advmod
what: subordinate when/where/how/why as mark → ADV advmod
from: childes
to: mova
match: w[form=when|where|how|why, rel=mark]
set: w[upos=ADV]; w[xpos=WRB]; w[rel=advmod]
source: EWT README v2.11 (#88)
```

```convert
rule: childes.relcl-mark-subj
what: relative that as mark → PRON subject, if there is no subject
from: childes
to: mova
match: v[rel=acl:relcl]; t[form=that|which, rel=mark, head=v, before=v]
require: none c[rel~nsubj, head=v]
set: t[rel=nsubj]; t[upos=PRON]; t[xpos=WDT]
source: UD en acl:relcl
```

```convert
rule: childes.relcl-mark-obj
what: relative that as mark on a verb with a subject → obj
from: childes
to: mova
match: v[rel=acl:relcl]; t[form=that|which, rel=mark, head=v, before=v]
require: exists c[rel~nsubj, head=v]; none c[rel=obj, head=v]
set: t[rel=obj]; t[upos=PRON]; t[xpos=WDT]
source: UD en acl:relcl
```

```convert
rule: childes.dem-pron
what: demonstrative DET as a noun — PRON
from: childes
to: mova
match: t[form=this|that|these|those, upos=DET, rel=nsubj|nsubj:pass|obj|iobj|obl|root|conj]
set: t[upos=PRON]
source: EWT README v1.2
```

```convert
rule: childes.to-case
what: to in case with UPOS PART — ADP/IN
from: childes
to: mova
match: t[form=to, rel=case, upos=PART]
set: t[upos=ADP]; t[xpos=IN]
source: UD en pos/ADP
```

```convert
rule: childes.pass-subj
what: subject with aux:pass — nsubj:pass
from: childes
to: mova
match: v[]; a[rel=aux:pass, head=v]; s[rel=nsubj, head=v]
set: s[rel=nsubj:pass]
source: UD en nsubj:pass
```

```convert
rule: childes.s-vbz
what: 's = is/has with XPOS POS → VBZ
from: childes
to: mova
match: s[form='s, upos=AUX, xpos=POS]
set: s[xpos=VBZ]
source: seeds/errors.md
```

The last rule fixes an error, not a dialect, because this error is systematic (81). There is no reverse rule.

Matches: `unmarked-time` 604, `obl-npmod` 226, `nmod-npmod` 15, `punct-xpos` 15 152 (XPOS will change in 9652 of them), `lemma-me` 1251, `lemma-him` 378, `lemma-them` 683, `lemma-us` 142, `lemma-poss` 2369, `lemma-i` 7956, `wh-advmod` 676, `relcl-mark-subj` 24, `relcl-mark-obj` 34 (3 of 61 fall under neither), `dem-pron` 204, `to-case` 23, `pass-subj` 22, `s-vbz` 81. The lemma changes only where it differs: *me* — 256, *i* — 53, etc.

## mova → childes

```convert
rule: mova.childes.npmod
what: :unmarked without a time word → :npmod
from: mova
to: childes
match: t[rel=obl:unmarked, lemma!=today|tomorrow|yesterday|tonight|morning|afternoon|evening|night|day|week|time|minute|year|month|now|weekend]
set: t[rel=obl:npmod]
source: seeds/npmod.md
```

```convert
rule: mova.childes.nmod-npmod
what: nmod:unmarked without a time word → nmod:npmod
from: mova
to: childes
match: t[rel=nmod:unmarked, lemma!=today|tomorrow|yesterday|tonight|morning|afternoon|evening|night|day|week|time|minute|year|month|now|weekend]
set: t[rel=nmod:npmod]
source: seeds/npmod.md
```

```convert-todo
rule: mova.childes.npmod-misc
what: :unmarked without TemporalNPAdjunct → :npmod (exact, by the trace)
from: mova
to: childes
match: t[rel=obl:unmarked, misc.TemporalNPAdjunct!=Yes]
set: t[rel=obl:npmod]
source: seeds/npmod.md
```

```convert-todo
rule: mova.childes.punct-xpos
what: ? and ! — XPOS as the sign itself
from: mova
to: childes
match: p[upos=PUNCT, form=?]
set: p[xpos=?]
source: seeds/punct.md
```

Needs string XPOS: `?` and `!` are not PTB tags, and `xpos=` will not accept them.

```convert
rule: mova.childes.feats
what: CHILDES has only ExtPos and Typo from FEATS — remove the rest
from: mova
to: childes
match: t[]
set: t[feats-=Number]; t[feats-=Person]; t[feats-=Tense]; t[feats-=Mood]; t[feats-=VerbForm]; t[feats-=Voice]; t[feats-=Case]; t[feats-=Gender]; t[feats-=PronType]; t[feats-=Poss]; t[feats-=Reflex]; t[feats-=Definite]; t[feats-=Degree]; t[feats-=NumType]; t[feats-=NumForm]; t[feats-=Polarity]; t[feats-=Foreign]; t[feats-=Abbr]; t[feats-=Style]
source: seeds/data.md
```

```convert
rule: mova.childes.lemma-your
what: your → you (the custom of most children, LP23)
from: mova
to: childes
match: p[form=your, upos=PRON]
set: p[lemma=you]
source: seeds/pron-lemma.md (1093 versus 784)
```

```convert
rule: mova.childes.when-mark
what: subordinate when on advcl — SCONJ mark
from: mova
to: childes
match: v[rel=advcl]; w[form=when, rel=advmod, head=v]
set: w[upos=SCONJ]; w[xpos=WRB]; w[rel=mark]
source: seeds/old-ud.md (when SCONJ mark — 420)
```

Matches on EWT: `npmod` 550, `nmod-npmod` 1254, `feats` 254 820, `lemma-your` 815, `when-mark` 402.

## Round trip childes → mova → childes

**Lossless:**
- `:npmod` versus the former `:tmod`, as long as the lemma is in the list or there is a MISC trace (`convert-todo`): 241 + 604;
- `feats` — CHILDES never had them, and `ExtPos` and `Typo` stay;
- `pass-subj`: 22 → `nsubj:pass` → not restored on the way back, because the CHILDES custom is `nsubj:pass` in 241 of 263. This is an error fix, so 22 DEPREL divergences are expected.

**Lossy:**
- **Punctuation `?`/`!`** — 9652 XPOS divergences until there is string XPOS. Then — 0.
- **Pronoun lemmas.** On the way there — one EWT norm, on the way back — the majority norm. Divergences:
  - Adam and Eve: *your* → *your* (784), *me* → *me* (256);
  - *him/them/us* with lemma = form: *them* → *them* 223, *him* → *him* 134, *us* → *us* 14.

  Together ≈ 1400 lemmas. A rule by child (`child_name` in the metadata) would eliminate this, but the rule language does not see sentence comments.
- **WH words.** *how* SCONJ `mark` (132) and *where/why* in `mark` become `advmod` on the way there, and on the way back only *when* on `advcl` gets `mark`. ≈ 250 UPOS and DEPREL divergences.
- **Relative *that*-`mark`** (58), **DET pronouns** (204), ***to*-PART** (23), ***'s*/POS** (81). On the way there this is a correction, on the way back the majority custom, so ≈ 370 divergences.

Total ≈ 99.0% DEPREL and UPOS agreement, ≈ 99.5% lemmas, XPOS — 97% until string XPOS.
