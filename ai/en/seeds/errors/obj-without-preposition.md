# The direct object and the subject have no preposition

**Gist.** `obj` is a direct object without a preposition. As soon as the phrase has a preposition (`case`), it is `obl` of a verb or `nmod` of a noun. The same goes for the subject. Prepositional verbs (*depend on, look at, listen to, refer to*) semantically have an "object", but syntactically it is `obl`. An annotator who thinks about meaning rather than form puts `obj` here.

**Conditions and exceptions.**
- The possessive *'s* is also `case`, but it hangs on the possessor (`nmod:poss`), not on the obj. So the rule takes only prepositions (ADP).
- Inversion with a fronted prepositional phrase (*Under the bed is a box*) is annotated in EWT as `obl` + `nsubj`.

**Examples.**
- *I depend on you* → obl(depend, you), case(you, on).
- *Refer to our brochure* → obl(Refer, brochure).
- ✗ obj(depend, you) together with case(you, on).

**Check against gold.** EWT 2.18: obj with a preposition — 0 of 12 254; subject with a preposition — 2 of 21 587.

**Sources.** UD `_en/dep/obj.md`, `_en/dep/obl.md` (Refer to our brochure); english-banks §1.3 (SD prep+pobj → UD case+obl/nmod).

```rule
rule: en.errors.obj-no-preposition
what: obj with a preposition — a phrase with a preposition is obl (depend on you), not obj
match: o[rel=obj]
require: none c[rel=case, head=o, upos=ADP]
severity: error
source: UD _en/dep/obj.md, _en/dep/obl.md
```

```rule
rule: en.errors.subject-no-preposition
what: subject with a preposition — a phrase with a preposition is never nsubj
match: s[rel~nsubj]
require: none c[rel=case, head=s, upos=ADP]
severity: warn
source: UD _en/dep/nsubj.md, _en/dep/obl.md
```
