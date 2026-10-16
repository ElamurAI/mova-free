# After a modal, do and to — the base form

**Gist.** A modal verb (*can, will, must, should*), the auxiliary *do* and the particle *to* require the base form (VB) of the next verb: *can go*, *does not know*, *to see*. If another auxiliary stands between them (*could have gone*, *will be going*, *don't get married*, *to be told*), that one determines the form. ERRANT assigns errors like *can goes*, *does not knows*, *to going*, *will meeting* to VERB:FORM: 3.6 % of edits in FCE and W&I.

**Conditions and exceptions.**
- A chain of auxiliaries: the form of the lexical verb is determined by the last auxiliary before it. So the rule requires another auxiliary after the modal, *do* or *to* if the verb is not VB.
- Dialectal *should of took* — see `vernacular-style.md`.

**Examples.**
- ✗ *We will meeting Rod's office.*
- ✗ *it does has one*
- ✓ *don't get married* (*get* is aux:pass)

**In UD.** All auxiliaries hang flat on the lexical verb; their order is visible from their positions in the sentence.

**Check against gold.** EWT 2.18: *to* — 149/0; modal — 480/9 (errors in the text: *will meeting*, *would named*); *do* — 11/1 (*does has*).

**Sources.** `P17-1074` (VERB:FORM; ordered rules for verbs, §3.1); `W19-4406` (table 4: VERB:FORM 3.56 % W&I train); `2025.acl-long.1026` (Koyama et al.: CTSEG — 15 subcategories of verb tense and form); UD `_en/dep/aux_.md`, `_en/dep/mark.md` (to — mark).

```rule
rule: en.errors.to-base
what: after a to-infinitive the verb is not VB and there is no other auxiliary (to going)
match: v[upos=VERB, xpos=VBN|VBG|VBD|VBZ|VBP]; t[form=to, upos=PART, rel=mark, head=v]
require: exists a[rel=aux|aux:pass, head=v, after=t]
severity: warn
source: P17-1074 (VERB:FORM); UD _en/dep/mark.md
```

```rule
rule: en.errors.modal-base
what: after a modal the verb is not VB and there is no other auxiliary (will meeting, can goes)
match: v[upos=VERB, xpos=VBN|VBG|VBD|VBZ|VBP]; m[xpos=MD, rel=aux, head=v]
require: exists a[rel=aux|aux:pass, head=v, after=m]
severity: warn
source: P17-1074 (VERB:FORM); W19-4406
```

```rule
rule: en.errors.do-base
what: after do-support the verb is not VB and there is no other auxiliary (does not knows)
match: v[upos=VERB, xpos=VBN|VBG|VBD|VBZ|VBP]; d[lemma=do, rel=aux, head=v]
require: exists a[rel=aux|aux:pass, head=v, after=d]
severity: warn
source: P17-1074 (VERB:FORM)
```
