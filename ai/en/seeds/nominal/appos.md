# Apposition (appos): a second name for the same thing, to the right

**Gist.** An appositive is a second noun phrase that names the same thing as the first and specifies it: *Sam, my brother, arrived*; *the Australian Broadcasting Corporation (ABC)*. Poutsma distinguishes three types: identity (*Mr. Lloyd George, the late Prime Minister*), species with genus (*the planet Mars*), quantity with substance (*a dozen collars, a little wine*). Curme distinguishes loose apposition (with a comma) and close apposition (*my brother John*).
**Conditions and exceptions.** In UD `appos` is only for identity of two full noun phrases. Quantity with substance takes other relations (*a couple cookies* — `nmod:unmarked`). A noun modifier (*iron bedstead*) is `compound`. A clause on a noun (*the fact that…*) is `acl`, not `appos`. *The city of Rome* is `nmod` with *of* (Poutsma: "specializing of").
**Examples.** *Sam, my brother,* → `appos(Sam, brother)`; *Bill (John's cousin)* → `appos(Bill, cousin)`.
**In UD.** `appos` is always to the right of its anchor word (EWT 2.18: 1 691 of 1 691); the dependent is mostly NOUN, PROPN, NUM. `appos` on a clause with the conjunction *that* is suspicious (should be `acl`).
**Sources.** Poutsma GLME vol. 1, Ch. IV §4–8 (pp. 289–292); Curme 1931 §10 III "Apposition" (pp. 88–92); https://universaldependencies.org/en/dep/appos.html, `nmod-unmarked.md` (vi).

```rule
rule: en.nominal.appos-right
what: an appositive stands to the right of the word it specifies
match: h[]; a[rel=appos, head=h]
require: a[after=h]
severity: error
source: UD en appos; Poutsma GLME I Ch. IV §6
```

```rule
rule: en.nominal.appos-not-content-clause
what: a clause with that/whether on a noun is acl, not appos
match: v[rel=appos]
require: none m[rel=mark, lemma=that|whether, head=v]
severity: warn
source: Curme 1931 §23 I; UD en acl (content clauses)
```
