# Double negation: don't know nothing

**Gist.** Standard English has one negation per clause: *I don't know anything* or *I know nothing*. Combining *not/n't* with a negative pronoun or determiner in the same clause is negative concord: *I don't know nothing*, *didn't take no reason*. For many English dialects it is normal, as it is for Ukrainian (literally "nothing not I-know"), but in the standard it is an error. Slavic learners transfer it from their native language. Negation is also tied to negative polarity items (*any, ever*): licensing such words is the hardest thing for language models, alongside islands.

**Conditions and exceptions.**
- Deliberate double negation with an affirmative meaning: *not unhappy*, *I can't not go*.
- Rendering dialect speech in quotations.

**Examples.**
- ✗ *I don't think nothing of it.* → ✓ *anything*.
- ✗ *didn't take no reason*

**In UD.** advmod(v, n't) with Polarity=Neg and, on the same v, a PRON dependent with PronType=Neg or a det with PronType=Neg.

**Check against gold.** EWT 2.18: 1 + 1, both colloquial.

**Sources.** `2025.udw-1.8` (Findlay et al.: English is a language without negative concord, unlike Spanish); `2020.tacl-1.25` (BLiMP: NPI licensing is the hardest category alongside islands).

```rule
rule: en.errors.double-negation-pronoun
what: not/n't and a negative pronoun on the same predicate (don't know nothing)
match: v[]; n[feats.Polarity=Neg, rel=advmod, head=v]; o[feats.PronType=Neg, head=v]
require: not o[feats.PronType=Neg]
severity: warn
source: 2025.udw-1.8 (negative concord); 2020.tacl-1.25 (BLiMP NPI)
```

```rule
rule: en.errors.double-negation-det
what: not/n't and the negative determiner no in a dependent of the same predicate (didn't take no reason)
match: v[]; n[feats.Polarity=Neg, rel=advmod, head=v]; o[rel=obj|obl|nsubj, head=v]; d[rel=det, feats.PronType=Neg, head=o]
require: not d[feats.PronType=Neg]
severity: warn
source: 2025.udw-1.8 (negative concord); 2020.tacl-1.25 (BLiMP NPI)
```
