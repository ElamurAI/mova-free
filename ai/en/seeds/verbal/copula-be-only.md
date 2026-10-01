# The copula is only be

**Gist.** When the predicate is a noun, adjective or prepositional phrase (*Bill is honest*, *Sue is a patriot*, *We are in the barn*), the verb *be* only "links" the subject to the predicative — it is a copula. In English UD only *be* counts as a copula. The semantically close *become, seem, remain, look, get, grow, turn* are ordinary verbs, and their predicative is `xcomp`.

**Conditions and exceptions.** *Be* is not a copula when it means "exist" (*There is a cow in the field*; `VERB`), in progressive forms (*is speaking* — `aux`), in the passive (*was given* — `aux:pass`), in colloquial quotative *be like* (*I was like, "What?"* — `VERB` with `compound:prt`). Logic (and Reed & Kellogg) call any "asserting word" a copula; UD grammar is narrower.

**Examples.** *Bill is honest* — `cop(honest, is)`. — *Bill got rich* — `xcomp(got, rich)`. — *I became very upset* — `xcomp`.

**In UD.** `cop`: lemma *be*, UPOS `AUX`. The head of the clause is the predicative; the subject attaches to it too.

**Sources.** https://universaldependencies.org/en/dep/cop.html ("“be” is the only verb recognized as a copula"); https://universaldependencies.org/en/specific-syntax.html, Copular verbs, Functional control; Reed & Kellogg, Higher Lessons, Lesson 29 (attribute complement; copula in logic); Jespersen, MEG III, ch. XVII–XVIII "Predicatives" (predicatives of being and becoming, §17.0x; text-3, p. 369).

```rule
rule: en.verbal.cop-be-only
what: the copula (cop) is only be with UPOS AUX
match: c[rel=cop]
require: c[lemma=be, upos=AUX]
severity: error
source: https://universaldependencies.org/en/dep/cop.html; https://universaldependencies.org/en/specific-syntax.html, Copular verbs
```
