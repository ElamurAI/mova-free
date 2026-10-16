# A missing copula: "He very happy"

**Gist.** The verb *be* cannot be omitted in an English sentence: *He is very happy*. Learners whose language has a zero present copula (Ukrainian, Russian, Chinese, Arabic) write ✗ *He very happy*, ✗ *this a problem*. Under literal annotation such a sentence has a nominal or adjectival predicate with `nsubj` but without `cop`. The same tree arises when the annotator attached an existing copula in the wrong place.

**Conditions and exceptions.**
- Telegraphic style of reviews and headlines: *Rooms clean.*, *Beer a bit expensive.*
- Absolute constructions (*with the kids asleep*) are `advcl` and are not covered: the rule takes only root, conj, ccomp and parataxis.

**Examples.**
- ✗ *Evidently, this a problem that…* → nsubj(problem, this) without cop.
- ✓ *Rooms very clean* (review style).

**Check against gold.** EWT 2.18: 3727/44 — mostly telegraphic reviews and a missing *is*. GUM — 3011/37.

**Sources.** `P16-1070` (Berzak et al.: literal reading in TLE); `P17-1074` (M:VERB); `dickinson-2015-grammaticality-syntactic-annotation-learner-language` (missing heads and ellipsis in learner text); UD `_en/dep/cop.md`.

```rule
rule: en.errors.missing-copula
what: a nominal or adjectival predicate with a subject but without a copula — a missing is or a wrong attachment of cop
match: r[upos=NOUN|ADJ|PROPN, rel=root|conj|ccomp|parataxis]; s[rel=nsubj, head=r, before=r]
require: exists c[rel=cop|aux, head=r]
unless: none f[feats.VerbForm=Fin]
severity: warn
source: P16-1070 (literal reading); P17-1074 (M:VERB); UD _en/dep/cop.md; exception from the seed: telegraphic style of reviews and headlines — the sentence has no finite verb at all (Rooms clean.)
```
