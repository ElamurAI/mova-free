# Double genitive: a friend of John's

**Gist.** When a noun already has an article, demonstrative or numeral, the possessor is placed after it — with *of* and in the genitive at the same time: *a friend of my father's, that remark of Tom's, a play of Shakespeare's*. For pronouns the independent form is used: *a friend of mine*. Poutsma calls this the "pleonastic genitive": the meaning of *of* is repeated by the ending.
**Conditions and exceptions.** Most natural after *a, this/that, any, some, no* or a numeral (Poutsma). The meaning can differ: *a picture of John* is an image of John, *a picture of John's* is a picture belonging to John.
**Examples.** *a cousin of Mary's*; *this idea of yours*; *two books of his*.
**In UD.** The possessor after *of* is `nmod` (not `nmod:poss`: that one is only before the head), with two `case`: *of* to the left and *'s* to the right; the pronoun *mine* is `nmod` with `case(of)`. EWT 2.18: a possessor with *of* and *'s* — 3 times, all `nmod`.
**Sources.** Poutsma GLME vol. 3, Ch. XXIV §33–34 (pp. 97–98 = book pp. 77–78); Poutsma GLME vol. 4, Ch. XXXIII §23 (p. 140); Curme 1931 §10 II 1 b "Double Genitive" (pp. 75–77).

```rule
rule: en.nominal.double-genitive-nmod
what: a possessor with of and 's (a friend of John's) is nmod after the noun, not nmod:poss
match: p[]; s[xpos=POS, head=p]; o[lemma=of, rel=case, head=p]
require: not p[rel=nmod:poss]
severity: error
source: Poutsma GLME III Ch. XXIV §33; Curme 1931 §10 II 1 b
```
