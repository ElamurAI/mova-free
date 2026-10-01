# Indirect object (iobj)

**Gist.** The indirect object is the recipient or addressee of the action without a preposition: *She gave **me** a raise*, *Tell **them** a story*. It stands before the direct object. The same role with a preposition (*gave it **to me***) is no longer an object but an `obl` adverbial.

**Conditions and exceptions.** Since UD 2.12 `iobj` can also be the only object if the verb allows another one: *tell **them*** (since *tell them a story* is possible), *tell them that the party is canceled* (with `ccomp`), *teach **her students** to write well* (with `xcomp`). In questions and relative clauses the direct object is fronted (*What did you give him?*). Which verbs take `iobj` — see `lexicon/verb-iobj-licensors.md`.

**Examples.** *They gave me the trip as a gift.* — *I told them that I'm coming* (iobj + ccomp). — *How much notification would you give the customers?* (obj fronted).

**In UD.** `iobj` stands to the left of a nominal `obj` of the same head.

**Sources.** UD docs/changes.md` "Sole iobj" (v2.12: the "only with obj" restriction lifted); https://universaldependencies.org/u/dep/iobj.html (teach: obj, iobj or both); https://universaldependencies.org/en/dep/iobj.html (outdated: "only when there is a obj or ccomp"); https://universaldependencies.org/en/dep/obl.html (give the toys to the children — obl); Jespersen, MEG III, ch. XIV "Two Objects", §14.11 (*he gave the boy a shilling*; text-3, p. 292); PropBank 3.1 `give.xml` (give.01: ARG2 "entity given to").

```rule
rule: en.verbal.iobj-before-obj
what: the indirect object stands before a nominal direct object
match: h[]; i[rel=iobj, head=h]; o[rel=obj, upos=NOUN|PROPN, head=h]
require: i[before=o]
severity: warn
source: Jespersen MEG III §14.11; https://universaldependencies.org/en/dep/iobj.html
```
