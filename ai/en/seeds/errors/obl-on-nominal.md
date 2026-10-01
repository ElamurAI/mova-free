# obl under a noun — suspected nmod

**Gist.** A prepositional phrase gets its relation by what it is attached to: to a verb, adjective or adverb — `obl`, to a noun — `nmod`. So `obl` with a noun head means one of two things: the phrase actually hangs on the verb (wrong head), or the label should be changed to `nmod`. This is the main error of English parsers: in the EWT and GUM confusion matrices the obl↔nmod pair is first in both directions, and it is almost always a prepositional-phrase attachment error (*eat a pizza with a fork* vs *with anchovies*).

**Conditions and exceptions.**
- A nominal predicate with a copula is a legitimate head for `obl`: the guidelines allow `obl` with "nominal predication".
- In verbless fragments GUM hangs the phrase on the nominal root as `obl` (*Good morning to all*, *What about you?*), while EWT uses `nmod`. For EWT style this is suspicious, for GUM it is the convention.
- Email headers in EWT (*X on 01/25/2002*) — 15 cases of `obl` without a copula.

**Examples.**
- *a preference for lilies* → nmod(preference, lilies).
- *we prefer lilies to daisies* → obl(prefer, daisies).
- *eat a pizza with a fork* → obl(eat, fork); *with anchovies* → nmod(pizza, anchovies).
- *He was a teacher in Boston* → obl(teacher, Boston), because there is cop(teacher, was).

**In UD.** The preposition is `case` inside the phrase. `obl` is from VERB, ADJ, ADV or from a nominal predicate with `cop`; `nmod` is from a noun.

**Check against gold.** EWT 2.18: fired 243, violations 15 (email headers). GUM 2.18: 281/85, of which 47 on a fragment root.

**Sources.** UD `_en/dep/obl.md`, `_en/dep/nmod.md` (`data/raw/ud-docs/docs/`); `2023.udw-1.7` (Zeldes, Schneider — appendix, fig. 2: obl/nmod is the most frequent confusion on both tests); `W18-4918` (Peng, Zeldes — obl/nmod confusion among the main causes of 10 % of errors in conversion from constituency trees); `2025.law-1.14` (LLMs tend toward shorter arcs, i.e. toward the nearer head).

```rule
rule: en.errors.obl-on-nominal
what: obl under a noun or pronoun without a copula — probably nmod or a wrong head of the prepositional phrase
match: h[upos=NOUN|PROPN|PRON]; o[rel=obl, head=h]
require: exists c[rel=cop, head=h]
severity: warn
source: UD _en/dep/obl.md, _en/dep/nmod.md; 2023.udw-1.7 (fig. 2); W18-4918
```
