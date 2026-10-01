# EWT ↔ mova — converter

`mova` today is UD 2.18 with EWT conventions plus three normalizations: `det:poss` → `nmod:poss`, `:tmod`/`:npmod` → `:unmarked`, Number/Person of verbs from the subject. On the EWT 2.18 data all three are empty (`seeds/mova-diff.md`). So the converter is identity in each direction. The rules below stand as safeguards for other EWT versions (≤ 2.14) and for Opus annotation in the EWT style. On EWT 2.18 they never fire.

## ewt → mova

```convert
rule: ewt.det-poss
what: det:poss → nmod:poss (mova's own normalization)
from: ewt
to: mova
match: t[rel=det]
require: t[feats.Poss=Yes]
set: t[rel=nmod:poss]
source: train/dialects-and-converters.md (annot::ann::ewt_rel)
```

The seed engine does not know the `det:poss` subtype: an unknown subtype is read as the base `det`. So `match` here is `det` with `Poss=Yes`. On EWT 2.18 there are 0 such: possessives are `nmod:poss`. When the converter engine reads subtypes as strings, the rule will become `t[rel=det:poss]`.

```convert
rule: ewt.obl-tmod
what: obl:tmod (EWT ≤ 2.14) → obl:unmarked + TemporalNPAdjunct
from: ewt
to: mova
match: t[rel=obl:tmod]
set: t[rel=obl:unmarked]; t[misc+=TemporalNPAdjunct=Yes]
source: EWT README v2.15
```

```convert
rule: ewt.nmod-tmod
what: nmod:tmod (EWT ≤ 2.14) → nmod:unmarked + TemporalNPAdjunct
from: ewt
to: mova
match: t[rel=nmod:tmod]
set: t[rel=nmod:unmarked]; t[misc+=TemporalNPAdjunct=Yes]
source: EWT README v2.15
```

```convert
rule: ewt.npmod
what: obl:npmod (EWT ≤ 2.14) → obl:unmarked
from: ewt
to: mova
match: t[rel=obl:npmod]
set: t[rel=obl:unmarked]
source: EWT README v2.15
```

```convert
rule: ewt.nmod-npmod
what: nmod:npmod (EWT ≤ 2.14) → nmod:unmarked
from: ewt
to: mova
match: t[rel=nmod:npmod]
set: t[rel=nmod:unmarked]
source: EWT README v2.15
```

Matches on EWT 2.18: `det-poss` finds 19,371 `det`, but none has `Poss=Yes`, so there are 0 applications. The other rules have no matches.

## mova → ewt

Identity. `mova` MISC must keep the EWT attributes (`TemporalNPAdjunct`, `Superlocation`, `FlatType`, `Cxn`, STREUSLE) — without them the reverse converters to old versions and other treebanks have to guess (`seeds/mova-diff.md`).

## Round trip ewt → mova → ewt

**0 divergences** in all columns. This is also a negative control of the engine: if the identity converter gives even one divergence on EWT, the engine is at fault (reading or writing CoNLL-U, MWT, empty nodes, DEPS, MISC), not the rules.

The second control is a broken rule, for example `t[rel=nmod:unmarked]` → `nmod`. It must give 1360 DEPREL divergences.
