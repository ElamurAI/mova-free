# Agreement: the form without -s — not with he/she/it

**Gist.** The present tense without *-s* (tag `VBP`: *go, have, are, do*) is used with *I, you, we, they* and with a plural subject. Third person singular pronouns (*he, she, it, this, that*) require the *-s* form. *Am* — only with *I*.

**Conditions and exceptions.** Coordinated subjects (*He and I **are***) take the plural (see `agreement-compound-subject.md`). The subjunctive (*that he **go***) is tagged `VB`, not `VBP` — so `VBP` with *he* often means the subjunctive was mistakenly tagged `VBP`, or else there is a real error in the text (*it sound like*). Relative pronouns (*who, that*) have no person.

**Examples.** *They **are** here.* — *I **am** here.* — *\*He **see** someone* (text error). — *I insist that he **go*** (VB, subjunctive).

**In UD.** A pronoun subject with `Person=3|Number=Sing` (without conjuncts) does not go with `VBP` — neither on the head nor via `cop`/`aux`. With *am* the pronoun subject has `Person=1|Number=Sing`.

**Sources.** Brown 1851, Rule XIV and Note IV (each, one, either, neither — third person singular); Brown 1851, Part II, Ch. VI "Persons and Numbers"; Reed & Kellogg, Higher Lessons, Lesson 142; https://universaldependencies.org/en/feat/Mood.html (subjunctive); Santorini 1990 (PTB), VBP.

```rule
rule: en.verbal.vbp-3sg-pronoun
what: a VBP predicate with subject he/she/it — suspicious (the subjunctive should be VB)
match: v[xpos=VBP]; s[rel=nsubj|nsubj:pass, head=v, upos=PRON, feats.Person=3, feats.Number=Sing]
require: exists c[rel=conj, head=s]
severity: warn
source: Brown 1851 Rule XIV; Santorini 1990, VBP
```

```rule
rule: en.verbal.vbp-aux-3sg-pronoun
what: a VBP copula or auxiliary with subject he/she/it — suspicious
match: h[]; a[xpos=VBP, rel=cop|aux|aux:pass, head=h]; s[rel=nsubj|nsubj:pass, head=h, upos=PRON, feats.Person=3, feats.Number=Sing]
require: exists c[rel=conj, head=s]
severity: warn
source: Brown 1851 Rule XIV
```

```rule
rule: en.verbal.am-first-person
what: am — only with subject I
match: h[]; a[form=am, rel=cop|aux|aux:pass, head=h]; s[rel=nsubj|nsubj:pass, head=h, upos=PRON]
require: s[feats.Person=1, feats.Number=Sing]
severity: warn
source: Brown 1851 Part II Ch. VI, conjugation of BE
```
