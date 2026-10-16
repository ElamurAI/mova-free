# iobj — a bare noun before obj; "to X" — obl

**Gist.** An indirect object in English UD is only a preposition-less noun in a double-object construction: *She gave me a raise* → iobj(gave, me), obj(gave, raise). A semantic recipient with *to* or *for* (*gave it to me*) is `obl`, because prepositional phrases in English are not core arguments. The indirect object stands between the verb and the direct object. LLMs learn `iobj` late and unstably, and often wrongly label a semantic recipient with *to* as iobj.

**Conditions and exceptions.**
- The direct object may come earlier if it is fronted: *How much money does the USA give NASA?*. So the order rule takes only an obj after the verb.
- Relative and interrogative object pronouns (*the book that I gave him*) are not covered by the rule: the obj there is a PRON.

**Examples.**
- ✓ *give the children the toys* → iobj(give, children), obj(give, toys).
- ✓ *give the toys to the children* → obl(give, children), case(children, to).
- ✗ iobj(give, children) together with case(children, to).

**In UD.** `iobj` has no `case` dependent. `obl` for the prepositional dative is stated directly in the `obl` guideline.

**Check against gold.** EWT 2.18: `iobj` with case — 0 of 795; iobj after a nominal obj — 0 of 338. GUM — 0 of 482 and 0 of 174.

**Sources.** UD `_en/dep/iobj.md`, `_en/dep/obl.md` (give the toys to the children); `2026.udw-1.1` (Matsuda et al.: iobj is learned late and unstably in 30 of 33 languages).

```rule
rule: en.errors.iobj-no-case
what: iobj with a preposition — a prepositional phrase with to/for is obl, not iobj
match: i[rel=iobj]
require: none c[rel=case, head=i]
severity: error
source: UD _en/dep/iobj.md, _en/dep/obl.md (prepositional dative)
```

```rule
rule: en.errors.iobj-before-obj
what: iobj after a nominal direct object — in a double object the order is verb, iobj, obj
match: v[]; i[rel=iobj, head=v]; o[rel=obj, head=v, upos=NOUN|PROPN, after=v]
require: i[before=o]
severity: warn
source: UD _en/dep/iobj.md (She gave me a raise)
```
