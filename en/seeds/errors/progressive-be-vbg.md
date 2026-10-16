# Progressive aspect: be + -ing

**Gist.** *be* as a non-passive auxiliary (`aux`) requires the -ing form: *is working*, *were running*. A base form after *be* (✗ *they are find*, ✗ *he is hate*, ✗ *if you're get*) is a text error (VERB:FORM) or a wrong annotation. The only legitimate "be + infinitive" is the *be to* construction with the particle *to*: *He was to leave at noon*. EWT annotates *was* here as `aux` of the infinitive.

**Conditions and exceptions.**
- The passive *be* + VBN is `aux:pass` (see `passive-structure.md`).
- *is being done* is a progressive passive: *is* — aux, *being* — aux:pass, the head is VBN.

**Examples.**
- ✗ *they are find more interest in oil drilling*
- ✓ *The United States was to cut its level.*

**Check against gold.** EWT 2.18: *be* + VB, VBD, VBZ or VBP without *to* — 16/5, all five are text errors.

**Sources.** `P17-1074` (VERB:FORM); UD `_en/dep/aux_.md`; `2025.law-1.14` (confusion of the perfect with a copula in LLM annotation).

```rule
rule: en.errors.be-progressive
what: after the auxiliary be, a base or finite form without to (they are find) — should be -ing
match: v[upos=VERB, xpos=VB|VBD|VBZ|VBP]; b[lemma=be, rel=aux, head=v]
require: exists t[form=to, rel=mark, head=v]
severity: warn
source: P17-1074 (VERB:FORM); UD _en/dep/aux_.md
```
