# nmod under a verb or adjective — should be obl

**Gist.** The mirror case of `obl` under a noun. `nmod` is only for dependents of a noun. A prepositional phrase on a verb is always `obl`, on an adjective also `obl` (*afraid of sharks*). `nmod` under a verb is a legacy of old schemes: SD had the pair `prep` + `pobj`, and UD v1 had a single `nmod` for all prepositional phrases. UD v2 split it into `nmod` and `obl` by the part of speech of the head. An annotator trained on old data repeats the old label.

**Conditions and exceptions.** Quantity adjectives used as pronouns take `nmod`: *most of them*, *many of the people*, *much of the time*, *few of us*. The adjective here works as a nominal head.

**Examples.**
- *people afraid of sharks* → obl(afraid, sharks).
- *He talked about it* → obl(talked, it), not nmod.
- *most of them* → nmod(most, them).

**In UD.** The relation of a prepositional phrase is determined by the part of speech of the head: NOUN, PROPN or PRON → `nmod`; VERB, ADJ or ADV → `obl`.

**Check against gold.** EWT 2.18: `nmod` under VERB — 0. Under ADJ — 92, of which 89 with most/many/much/few/more/little, the other 3 are real violations. GUM: under VERB — 2.

**Sources.** UD `_en/dep/obl.md` (examples *afraid of sharks*, *Unfortunately for you*), `_en/dep/nmod.md`, `_en/migration-guidelines.md` §1.3 (SD prep+pobj → UD case+nmod/obl); `2023.udw-1.7`.

```rule
rule: en.errors.nmod-on-verb
what: nmod under a verb — a prepositional phrase on a verb should be obl
match: h[upos=VERB]; o[rel=nmod, head=h]
require: not o[rel=nmod]
severity: error
source: UD _en/dep/nmod.md, _en/dep/obl.md; english-banks §1.3 (UD v1 nmod → v2 nmod/obl)
```

```rule
rule: en.errors.nmod-on-adj
what: nmod under an adjective — should be obl, except quantity most/many/much/few (most of them)
match: h[upos=ADJ]; o[rel=nmod, head=h]
require: h[lemma=most|many|much|few|more|little|less|least|several|enough]
severity: warn
source: UD _en/dep/obl.md (people afraid of sharks), _en/dep/nmod.md
```
