# The pronoun subject of a finite verb is nominative

**Gist.** The subject of a finite verb is in the nominative case: *I, he, she, we, they*. The forms ✗ *Me and my family are moving*, ✗ *them came up with*, ✗ *her has a place* are colloquial or learner forms. However, in non-finite constructions the subject is accusative, and that is the norm: *for me to sit in*, *him stuttering*, *them suggesting a fund*. So the rule checks only predicates with VerbForm=Fin: on the verb itself, on the copula or on the auxiliary.

**Conditions and exceptions.**
- Reflexive pronouns as subject (*Myself and Credit were calling*) are also non-standard.
- The feature Case=Acc on *it* in EWT is sometimes wrong (a feature error in the gold). The rule catches that too.

**Examples.**
- ✗ *Me and my family are moving.*
- ✓ *It would be OK for me to sit in.*
- ✓ *I remember him stuttering.*

**In UD.** nsubj on a PRON with Case=Acc with a finite predicate. `_en/feat/Case.md`: Nom is the subject of a finite verb.

**Check against gold.** EWT 2.18: verb — 3; copula or auxiliary — 8. All are colloquial or non-standard forms or a wrong Case on *it*.

**Sources.** UD `_en/feat/Case.md`; `P17-1074` (ERRANT: PRON); `W19-4406` (PRON — 2.64 % of W&I edits).

The rule is `en.nominal.finite-subject-nom` in `nominal/pron-case-function.md`.

```rule
rule: en.errors.finite-subject-nominative-auxcop
what: accusative subject before a finite copula or auxiliary (Me and my family are…)
match: h[]; a[rel=cop|aux|aux:pass, feats.VerbForm=Fin, head=h]; s[rel=nsubj|nsubj:pass, head=h, upos=PRON, feats.Case=Acc, before=a]
require: not s[feats.Case=Acc]
severity: warn
source: UD _en/feat/Case.md; P17-1074 (PRON); Poutsma GLME IV Ch. XXXII §8
```
