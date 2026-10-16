# A clausal subject — csubj, not nsubj

**Gist.** If the subject is a whole clause (*That he lied is obvious*, *Taking a nap will relax you*), the relation is `csubj`, and its dependent is the verb of the clause. `nsubj` on a word with UPOS VERB is almost always a mistake: either a clausal subject was wrongly labelled `nsubj`, or a noun was wrongly taken for a verb. LLMs learn `csubj` late and unstably. English treebanks also disagree on `csubj`: EWT and GUM differ in the share and direction of this relation.

**Conditions and exceptions.**
- A verbal noun (*The swimming was fun*, NN) is `nsubj`.
- *the following is…* — EWT has the VERB *following* as nsubj (4 cases).
- Extraposition with *it* — see `tough-vs-extraposition.md`.

**Examples.**
- *Whether he lied is beside the point* → csubj(point, lied).
- *Taking a nap will relax you!* → csubj(relax, Taking).

**Check against gold.** EWT 2.18: `nsubj` on VERB — 4; GUM — 8.

**Sources.** UD `_en/dep/csubj.md`; `2026.udw-1.1` (csubj is late and unstable in LLMs); `2020.udw-1.8` (Dönicke et al.: English csubj is among the most inconsistent relations across treebanks).

```rule
rule: en.errors.verb-subject-csubj
what: nsubj on a verb — a clausal subject should be csubj
match: s[rel=nsubj|nsubj:pass, upos=VERB]
require: not s[upos=VERB]
severity: warn
source: UD _en/dep/csubj.md; 2026.udw-1.1; 2020.udw-1.8
```
