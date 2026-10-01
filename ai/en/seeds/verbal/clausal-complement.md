# Complement clause (ccomp)

**Gist.** A subordinate clause can be the complement of a verb or adjective: *He says **that you like to swim***, *I am certain **that he did it***, *I wonder **whether it works***. Such a clause has its own subject (or it is direct speech, or a *to*-infinitive after verbs of saying: *The boss said **to start digging***). The conjunction *that* can be omitted: *He says you like to swim*.

**Conditions and exceptions.** `ccomp` itself takes the place of the direct object, so there is no `obj` alongside it; the addressee alongside it is `iobj`: *I told **them** that I'm coming*. A clause after a noun (*the fact that…*, *no idea what…*) is not `ccomp` but `acl` on the noun: nouns do not take clausal complements.

**Examples.** *He says that you like to swim.* — *I told them that I'm planning to come* (iobj + ccomp). — *He had no idea what he was talking about* (acl on *idea*, not ccomp on *had*).

**In UD.** `ccomp` does not co-occur with an `obj` of the same head.

**Sources.** https://universaldependencies.org/en/dep/ccomp.html; https://universaldependencies.org/en/specific-syntax.html, Core arguments ("the clausal complement label ccomp never cooccurs with obj") and Clausal core arguments ("Nouns never take clausal core arguments"); Reed & Kellogg, Higher Lessons, Lessons 71–72 (noun clause); Jespersen, MEG V, ch. XII "Infinitive as Object" (text-1, p. 210).

```rule
rule: en.verbal.ccomp-no-obj
what: with a complement clause (ccomp) there is no direct object
match: h[]; c[rel=ccomp, head=h]
require: none o[rel=obj, head=h]
severity: warn
source: https://universaldependencies.org/en/specific-syntax.html, Core arguments
```
