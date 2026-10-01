# Infinitive of purpose — an adverbial (advcl), not a complement

**Gist.** An infinitive can be a complement of the verb (*I want **to help***) or an adverbial of purpose (*I came **to see** you* = in order to see). A complement is required by the verb itself; a purpose adverbial can be added to almost any action and replaced with *in order to*. A phrase with *in order to* is always an adverbial.

**Conditions and exceptions.** An infinitive of purpose after verbs of motion (*come, go, sit down*) is `advcl`; *go to see* ≠ *want to see*. An infinitive on a noun (*a house to live in*) is `acl`.

**Examples.** *He talked to him **in order to secure** the account* — `advcl`. — *I came to tell you* — `advcl`. — *I tried to finish it* — `xcomp`.

**In UD.** A clause with `mark(in)` + `fixed(order)` is not `xcomp`, not `ccomp`, not `csubj` (usually `advcl`).

**Sources.** https://universaldependencies.org/en/dep/advcl.html (purpose clause: *in order to secure the account*); https://universaldependencies.org/en/dep/xcomp.html (complements, not purpose clauses); Reed & Kellogg, Higher Lessons, Lesson 40 ("Frequently the infinitive phrase expresses purpose"); Poutsma 1923, *The Infinitive…*, §3 (to has expressed purpose from of old: *I came to tell you*; vol. 2, p. 15).

```rule
rule: en.verbal.in-order-to-advcl
what: a clause with in order to is an adverbial, not a complement
match: x[]; m[form=in, rel=mark, head=x]; f[form=order, rel=fixed, head=m]
require: not x[rel=xcomp|ccomp|csubj]
severity: warn
source: https://universaldependencies.org/en/dep/advcl.html; https://universaldependencies.org/en/dep/xcomp.html
```
