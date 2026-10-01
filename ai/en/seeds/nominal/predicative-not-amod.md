# A predicative adjective is not amod

**Gist.** An adjective can be attributive — on a noun without a verb (*a sick man*) — or predicative — via a copula (*The man is sick*) or as a predicative complement (*I found him sick*). Poutsma distinguishes these uses as the main functions of the adjective.
**Conditions and exceptions.** In UD a predicative adjective with *be* becomes the head of the clause: the subject and copula depend on it. With *find, make, consider* it is `xcomp`. So an adjective that has a `cop` or a subject cannot be `amod`.
**Examples.** *The house is old.* → `nsubj(old, house)`, `cop(old, is)`; *an old house* → `amod(house, old)`.
**In UD.** `amod` has no `cop` or `nsubj` children (EWT 2.18: 0 out of over 12 000).
**Sources.** Poutsma GLME vol. 3, Ch. XXVIII §6 (p. 379 = book p. 359); https://universaldependencies.org/en/dep/amod.html, https://universaldependencies.org/u/dep/cop.html.

```rule
rule: en.nominal.amod-no-cop
what: amod has no copula or subject — such an adjective is predicative (head of the clause)
match: a[rel=amod]
require: none c[rel=cop, head=a]; none c[rel~nsubj, head=a]
severity: error
source: Poutsma GLME III Ch. XXVIII §6; UD en amod, cop
```
