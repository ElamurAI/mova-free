# advcl under a noun — should be acl

**Gist.** A subordinate clause attached to a noun has the relation `acl` or `acl:relcl` (relative). `advcl` is an adverbial clause attached to a predicate: a verb, an adjective or a nominal predicate with a copula. `advcl` under a noun without a copula usually means the clause was attached to the nearest noun instead of the verb (a short arc) or was given a wrong label.

**Conditions and exceptions.**
- A nominal predicate with a copula legitimately takes `advcl`: *It was a mess when we arrived*.
- In verbless fragments (headlines, captions) a noun with `advcl` may be the head.

**Examples.**
- *a decision to leave* → acl(decision, leave).
- *I called when I arrived* → advcl(called, arrived).
- *It was a mess when we arrived* → advcl(mess, arrived), because there is cop(mess, was).

**In UD.** `advcl` modifies a predicate, `acl` a noun. Choosing between them is choosing the head of the clause.

**Check against gold.** EWT 2.18: fired 201, violations 15 (fragments and headlines).

**Sources.** UD `_en/dep/advcl.md`, `_en/dep/acl.md`; `2025.law-1.14` (LLMs produce shorter arcs: mean distance 3.05 vs 3.41 in the gold); `2023.udw-1.7`.

```rule
rule: en.errors.advcl-on-noun
what: advcl under a noun without a copula — probably acl or a wrong head of the clause
match: h[upos=NOUN|PROPN]; a[rel=advcl, head=h]
require: exists c[rel=cop, head=h]
severity: warn
source: UD _en/dep/advcl.md, _en/dep/acl.md; 2025.law-1.14
```
