# Gapping: orphan links the remnants of an omitted predicate

**Gist.** In *Marie went to Paris and Miriam to Prague* the verb of the second clause is omitted (gapping). UD promotes one remnant (*Miriam*) to the predicate's place, conj(went, Miriam), and attaches the other remnants to it with `orphan`: orphan(Miriam, Prague). So the head of `orphan` is always a promoted remnant acting as a conjunct, and in fragments as the root, `advcl` or `parataxis`. `orphan` is not used when verbal material remains: with a shared right element (RNR) and with VP ellipsis.

**Conditions and exceptions.**
- Since 2.10 GUM annotates ellipsis more aggressively, so it has `orphan` in wider contexts (6 violations).
- In learner text, the choice between ellipsis and asyndetic coordination depends on whether the annotator considers the sentence grammatical.

**Examples.**
- *Marie went to Paris and Miriam to Prague* → orphan(Miriam, Prague).
- *John bought and ate an apple* → conj(bought, ate), obj(bought, apple), no orphan.

**Check against gold.** EWT 2.18: 29 `orphan`, violations 0; GUM: 128/6.

**Sources.** UD `_en/dep/orphan.md`; `2025.tlt-1.6` (Corbetta et al.); `2023.udw-1.7` (GUM v2.10: «more aggressive identification of ellipsis»); `W18-4918` (orphan cannot be reliably derived from the tree by conversion); `dickinson-2015-grammaticality-syntactic-annotation-learner-language` (ellipsis and listing in learner text).

```rule
rule: en.errors.orphan-head
what: orphan hangs on something other than a promoted remnant (conj, root, advcl, parataxis) — gapping parsed wrongly
match: h[]; o[rel=orphan, head=h]
require: h[rel=conj|root|advcl|parataxis]
severity: warn
source: UD _en/dep/orphan.md; 2025.tlt-1.6
```
