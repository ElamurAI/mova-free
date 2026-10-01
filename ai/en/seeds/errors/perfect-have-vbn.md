# Perfect: have + participle (VBN)

**Gist.** The perfect auxiliary *have* requires a past participle: *has gone*, *have seen*. The forms ✗ *has went*, ✗ *have saw*, ✗ *has go* are a VBD/VBN confusion, typical for irregular verbs. In ERRANT this is VERB:FORM and VERB:INFL. If another auxiliary follows *have* (*has been going*), that one determines the form.

**Conditions and exceptions.**
- *have to* ("must") and lexical *have* are VERB, not `aux`.
- Regular verbs have identical VBD and VBN (*missed*). Here the rule catches a tag error in the annotation, not in the text.

**Examples.**
- ✗ *He has went home.*
- ✓ *He has been working.*

**Check against gold.** EWT 2.18: 77/1 (*have missed* tagged VBD — a tag error); GUM — 68/4.

**Sources.** `P17-1074` (VERB:FORM, VERB:INFL: *getted → got*); `2020.tacl-1.25` (BLiMP: IRREGULAR FORMS — participles of irregular verbs); UD `_en/dep/aux_.md`.

```rule
rule: en.errors.have-vbn
what: after perfect have the verb is not VBN and there is no other auxiliary (has went)
match: v[upos=VERB, xpos=VB|VBD|VBZ|VBP|VBG]; h[lemma=have, rel=aux, head=v]
require: exists a[rel=aux|aux:pass, head=v, after=h]
severity: warn
source: P17-1074 (VERB:FORM, VERB:INFL); 2020.tacl-1.25 (BLiMP irregular forms)
```
