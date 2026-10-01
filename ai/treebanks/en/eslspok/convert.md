# ESLSpok ↔ mova — converter draft

The rule language is `train/dialects-and-converters.md`, "Converter rule language". Conventions beyond the specification (`require:` as a condition, ```` ```convert-todo ````) are in `../seeds-overview.md`. Matches were counted by `en expert-check`: `eslspok → mova` on all parts of ESLSpok, `mova → eslspok` on EWT.

ESLSpok has no lemmas, FEATS, MISC or MWT (`seeds/data.md`). The converter does not invent these layers: in `mova` they stay «_», and training skips them (`Sentence::has_lemmas`, `has_feats`).

## eslspok → mova

### 1. `:tmod`/`:npmod` → `:unmarked` (`seeds/tmod-npmod.md`)

```convert
rule: eslspok.obl-tmod
what: obl:tmod → obl:unmarked + TemporalNPAdjunct
from: eslspok
to: mova
match: t[rel=obl:tmod]
set: t[rel=obl:unmarked]; t[misc+=TemporalNPAdjunct=Yes]
source: UD 2.15, docs#1028; EWT README v2.15
```

```convert
rule: eslspok.nmod-tmod
what: nmod:tmod → nmod:unmarked + TemporalNPAdjunct
from: eslspok
to: mova
match: t[rel=nmod:tmod]
set: t[rel=nmod:unmarked]; t[misc+=TemporalNPAdjunct=Yes]
source: UD 2.15, docs#1028
```

```convert
rule: eslspok.obl-npmod
what: obl:npmod → obl:unmarked
from: eslspok
to: mova
match: t[rel=obl:npmod]
set: t[rel=obl:unmarked]
source: UD 2.15, docs#1028
```

```convert
rule: eslspok.nmod-npmod
what: nmod:npmod → nmod:unmarked
from: eslspok
to: mova
match: t[rel=nmod:npmod]
set: t[rel=nmod:unmarked]
source: UD 2.15, docs#1028
```

Matches: 77 + 20 + 50 + 4 = 151.

### 2. Prepositional *to* (`seeds/to-part.md`)

```convert
rule: eslspok.to-case
what: to as case — ADP/IN
from: eslspok
to: mova
match: t[form=to, rel=case, upos=PART]
set: t[upos=ADP]; t[xpos=IN]
source: UD en pos/ADP; EWT 2.18 (to with a noun — ADP/IN)
```

Matches: 224.

### 3. UD 2.0 customs (`seeds/old-ud.md`)

```convert
rule: eslspok.wh-advmod
what: subordinate when/where/how/why as SCONJ mark → ADV advmod
from: eslspok
to: mova
match: w[form=when|where|how|why, rel=mark]
set: w[upos=ADV]; w[rel=advmod]
source: EWT README v2.11 (#88)
```

```convert
rule: eslspok.so-parataxis
what: «X, so Y»: so as cc on conj → so ADV advmod, Y — parataxis
from: eslspok
to: mova
match: y[rel=conj]; s[form=so, upos=CCONJ, rel=cc, head=y]
set: s[upos=ADV]; s[xpos=RB]; s[rel=advmod]; y[rel=parataxis]
source: EWT README v2.10 (#313)
```

```convert
rule: eslspok.so-adv
what: so as CCONJ outside coordination — ADV
from: eslspok
to: mova
match: s[form=so, upos=CCONJ]
set: s[upos=ADV]; s[xpos=RB]; s[rel=advmod]
source: EWT README v2.10
```

```convert
rule: eslspok.flat-foreign
what: flat:foreign → flat (EWT 2.13 removed the subtype)
from: eslspok
to: mova
match: f[rel=flat:foreign]
set: f[rel=flat]
source: EWT README v2.13 (#459)
```

Matches: `wh-advmod` 41, `so-parataxis` 33, `so-adv` another 27 (7 `cc` not on `conj` and 20 `advmod` with CCONJ), `flat-foreign` 14.

**Not converted:**
- `dep` on Japanese insertions (26): the word's role in the sentence is unknown.
- **MWT and `SpaceAfter`** (`seeds/tokenization.md`). The `convert` language does not create MWT lines. An action like `mwt+=v..w` by the forms *n't, 's, 'm, 're, 'll, 've, 'd* is needed. `SpaceAfter=No` before them can be set by a rule: `t[form=n't|'s|'m|'re|'ll|'ve|'d]; p[next=…]` → `p[misc+=SpaceAfter=No]`. But the tokenized `# text` did not preserve spaces before punctuation, so the text cannot be reconstructed anyway.
- Lemmas and FEATS: the layers do not exist.

## mova → eslspok

```convert
rule: mova.eslspok.nmod-tmod
what: nmod:unmarked on time words → nmod:tmod
from: mova
to: eslspok
match: t[rel=nmod:unmarked, form=week|weeks|day|days|time|today|tomorrow|tonight|yesterday|night|morning|afternoon|evening|weekend|everyday|year|month]
set: t[rel=nmod:tmod]
source: seeds/tmod-npmod.md
```

```convert
rule: mova.eslspok.obl-tmod
what: obl:unmarked on time words → obl:tmod
from: mova
to: eslspok
match: t[rel=obl:unmarked, form=week|weeks|day|days|time|today|tomorrow|tonight|yesterday|night|morning|afternoon|evening|weekend|everyday|sunday|monday|tuesday|wednesday|thursday|friday|saturday|year|month|now]
set: t[rel=obl:tmod]
source: seeds/tmod-npmod.md
```

```convert
rule: mova.eslspok.npmod
what: remaining :unmarked → :npmod
from: mova
to: eslspok
match: t[rel=obl:unmarked]
set: t[rel=obl:npmod]
source: seeds/tmod-npmod.md
```

```convert
rule: mova.eslspok.nmod-npmod
what: remaining nmod:unmarked → nmod:npmod
from: mova
to: eslspok
match: t[rel=nmod:unmarked]
set: t[rel=nmod:npmod]
source: seeds/tmod-npmod.md
```

The remaining `:unmarked` (rules `npmod` and `nmod-npmod`) — `:npmod`, because the temporal ones have already been renamed. More precisely — by the trace in MISC:

```convert-todo
rule: mova.eslspok.tmod-misc
what: :unmarked with TemporalNPAdjunct=Yes → :tmod, without the trace — :npmod
from: mova
to: eslspok
match: t[rel=obl:unmarked, misc.TemporalNPAdjunct=Yes]
set: t[rel=obl:tmod]; t[misc-=TemporalNPAdjunct]
source: EWT README v2.15
```

Needs a condition on MISC (`misc.K=V`), which the rule language does not have yet (`ai/en/src/expert.rs`, "Not done yet").

```convert
rule: mova.eslspok.to-case
what: to with a noun — PART/TO
from: mova
to: eslspok
match: t[form=to, rel=case]
set: t[upos=PART]; t[xpos=TO]
source: seeds/to-part.md
```

```convert
rule: mova.eslspok.when-mark
what: subordinate when (in advcl) — SCONJ mark
from: mova
to: eslspok
match: v[rel=advcl]; w[form=when, rel=advmod, head=v]
set: w[upos=SCONJ]; w[rel=mark]
source: seeds/old-ud.md
```

```convert
rule: mova.eslspok.so-cc
what: parataxis with so → conj with so CCONJ cc
from: mova
to: eslspok
match: y[rel=parataxis]; s[form=so, rel=advmod, head=y, upos=ADV]
require: none c[rel=cc, head=y]
set: s[upos=CCONJ]; s[xpos=CC]; s[rel=cc]; y[rel=conj]
source: seeds/old-ud.md
```

```convert
rule: mova.eslspok.flat-foreign
what: flat on X/FW → flat:foreign
from: mova
to: eslspok
match: f[rel=flat, upos=X, xpos=FW]
set: f[rel=flat:foreign]
source: seeds/old-ud.md
```

```convert
rule: mova.eslspok.new-subtypes
what: subtypes after 2.0: obl:agent → obl
from: mova
to: eslspok
match: t[rel=obl:agent]
set: t[rel=obl]
source: seeds/old-ud.md
```

```convert
rule: mova.eslspok.outer
what: nsubj:outer → nsubj
from: mova
to: eslspok
match: t[rel=nsubj:outer]
set: t[rel=nsubj]
source: seeds/old-ud.md
```

```convert
rule: mova.eslspok.relcl
what: advcl:relcl → advcl
from: mova
to: eslspok
match: t[rel=advcl:relcl]
set: t[rel=advcl]
source: seeds/old-ud.md
```

```convert
rule: mova.eslspok.quotes
what: quotes `` and '' → "
from: mova
to: eslspok
match: q[xpos=``|'']
set: q[xpos="]
source: seeds/data.md
```

```convert-todo
rule: mova.eslspok.layers
what: without lemmas, FEATS, MISC and MWT
from: mova
to: eslspok
match: t[]
set: t[lemma=_]; t[feats=_]; t[misc=_]
source: seeds/data.md
```

Needs the actions `feats=_` and `misc=_`: clear the whole column, not one feature at a time. Removing MWT lines is also needed.

Matches on EWT: `obl-tmod` 604, `nmod-tmod` 91, `npmod` 640 (1244 − 604), `nmod-npmod` 1269 (1360 − 91), `to-case` 2029, `when-mark` 402, `so-cc` 211, `flat-foreign` 36, `new-subtypes` 376, `outer` 254, `relcl` 141, `quotes` 1954.

`nmod:desc` (EWT 2.16) has no unambiguous mapping to UD 2.0. *Mr.*, *Inc.* were then `compound` or `flat`. `nmod:desc` is kept as is: this is a loss when evaluating `en` output on ESLSpok, but ESLSpok itself has no such words.

## Round trip eslspok → mova → eslspok

**Lossless:**
- `to` — 224;
- `flat:foreign` — 13 of 14. One `flat:foreign` is not on X/FW, and it cannot be restored;
- `obl:npmod`/`nmod:npmod` — as long as the form is not in the list of time words;
- everything the converter does not touch: the tree, UPOS, XPOS, the other labels.

**Lossy:**
- **`:tmod` versus `:npmod` by the list of forms.** ESLSpok has `:npmod` on both *years* (10) and *times* (4). Some of them will diverge from the list, some `:tmod` will not be in it. Expect 10–20 DEPREL divergences of 151. With a condition on MISC (`convert-todo` above) — 0.
- ***when*.** 3 subordinate *when* in ESLSpok were `advmod` on `advcl`; on the way back they become `mark`. 2 *when*-`mark` are not on `advcl` and stay `advmod` on the way back. Together 5 UPOS and DEPREL divergences.
- ***so*.** 7 *so*-`cc` on a head that is not `conj`. On the way there they become `advmod`, on the way back they do not.

Total: expect < 30 divergences per 21,312 words, i.e. > 99.8% agreement.
