# The copula is only be; seem, become, get — xcomp

**Gist.** In English UD `cop` is only *be* in predicative use. The head is the nominal or adjectival predicate: *Bill is honest* → cop(honest, is). Similar verbs *become, seem, get, remain, look* are ordinary VERBs with `xcomp`. When the predicate is a verb, *be* is `aux` or `aux:pass`, not `cop`. The only exception is a predicate clause (*The problem is that…*): there is `nsubj:outer`, and *be* is `cop` of the subordinate verb.

**Conditions and exceptions.**
- Existential *there is* is a VERB root (see `existential-there.md`).
- Former participles that became adjectives in hyphenated spelling (*mouth watering*, *self-cleaning*, *dull looking*) sometimes have VERB with cop in EWT (8 cases).
- ESLSpok has no lemmas, so the first rule gives 651 false alarms there. This is a treebank convention, not errors.

**Examples.**
- *Bill got rich* → xcomp(got, rich).
- *Bill is speaking* → aux(speaking, is).
- *The important thing is to keep calm* → nsubj:outer(keep, thing), cop(keep, is).

**Check against gold.** EWT 2.18: `cop` not on *be* — 0 of 5939; `cop` on a VERB without `:outer` — 8 of 219.

**Sources.** UD `_en/dep/cop.md`; english-banks §2 (table: copula in PTB, SD, UD); `2025.law-1.14` (perfect confused with copula) (registry gate: cop is only be).

```rule
rule: en.errors.cop-be
what: cop not on be — become/seem/get are VERBs with xcomp
match: c[rel=cop]
require: c[lemma=be]
severity: error
source: UD _en/dep/cop.md (Predicative be is the only verb recognized as a copula)
```

```rule
rule: en.errors.cop-on-verb
what: cop on a verb without nsubj:outer — be with a verb is aux or aux:pass
match: h[upos=VERB]; c[rel=cop, head=h]
require: exists s[rel=nsubj:outer|csubj:outer, head=h]
severity: warn
source: UD _en/dep/cop.md (predicate clauses, nsubj:outer)
```
