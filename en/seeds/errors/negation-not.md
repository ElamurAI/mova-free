# not and n't: PART, advmod, Polarity=Neg

**Gist.** The negative particle *not/n't* in English UD is PART with XPOS RB, the relation `advmod` and the feature Polarity=Neg. Negative pronouns and determiners (*no, nothing, nobody, none*) have PronType=Neg. The `neg` label from SD and UD v1 no longer exists. EWT 2.18 has Polarity=Neg on all 2077 *not/n't*. However, Findlay et al. wrote for UD 2.15 that EWT lacked this feature. So this is a new alignment, and old models or data do not know about it. The features are put on the negative word, not on the predicate, i.e. they reflect form, not meaning.

**Conditions and exceptions.** The relation of *not* can also be different: `cc` with ExtPos=CCONJ in *not only … but*, `conj` in *whether or not*, `fixed`. So the rule checks only UPOS, XPOS and the feature.

**Examples.**
- *He did n't go* → advmod(go, n't); n't: PART, RB, Polarity=Neg.
- *no problem* → det(problem, no); no: DET, PronType=Neg.

**Check against gold.** EWT 2.18 — 2077, violations 0. ESLSpok — 277 of 277 (no lemmas or features, a treebank convention).

**Sources.** `2025.udw-1.8` (Findlay et al.: Polarity=Neg only in 224 of 296 UD 2.15 treebanks, absent in EWT; proposal Negated=+ on the predicate); UD `_en/feat/Polarity.md` (not gets Polarity=Neg); english-banks §2 (not/n't → PART).

```rule
rule: en.errors.not-part
what: not/n't is not PART RB with Polarity=Neg (there is no neg relation in UD v2)
match: n[form=not|n't]
require: n[upos=PART, xpos=RB, feats.Polarity=Neg]
severity: error
source: UD _en/feat/Polarity.md; 2025.udw-1.8
```
