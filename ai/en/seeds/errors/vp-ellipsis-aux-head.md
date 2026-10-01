# VP ellipsis: the auxiliary becomes the head

**Gist.** In *John will win gold and Mary will too* the second *will* is left without its verb. There are no empty nodes in the basic UD layer, so the auxiliary itself becomes the head of the second clause: conj(win, will₂), nsubj(will₂, Mary). A wrong parse attaches *will₂* as `aux` to the first *win*. This gives a long backward arc, and the auxiliary stands AFTER its head. In ordinary sentences this does not happen, because the auxiliary comes before the verb.

**Conditions and exceptions.** Inversion with a fronted participle or gerund: *Attached is the file* (this is `aux:pass`, the rule does not take it), *Sailing with the Roosevelt is…*, *Compounding this is…*.

**Examples.**
- *He can swim and I can too* → conj(swim, can₂), nsubj(can₂, I).
- ✗ aux(swim, can₂).

**In UD.** The `orphan` guideline: in VP ellipsis the auxiliary remains the head; `orphan` is not used here. Predicate ellipsis in basic UD is only worked around by promotion, and the gap itself is visible only in Enhanced UD.

**Check against gold.** EWT 2.18: `aux` after a VERB head — 2 (inversion); GUM — 1.

**Sources.** UD `_en/dep/orphan.md` ("In VP-ellipsis, we keep the auxiliary as the head"); `2025.tlt-1.6` (Corbetta et al.: predicate ellipsis — only promotion or orphan, the empty node is in Enhanced); `2025.findings-emnlp.863` (LLMs are inconsistent on ellipsis); `2025.law-1.14` (shorter arcs in LLMs).

```rule
rule: en.errors.aux-before-verb
what: aux stands after its verb — probably VP ellipsis, where the auxiliary should be the head (Mary will too)
match: h[upos=VERB]; a[rel=aux, head=h, after=h]
require: not a[rel=aux]
severity: warn
source: UD _en/dep/orphan.md (VP-ellipsis: auxiliary as head); 2025.tlt-1.6
```
