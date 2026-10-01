# The possessor stands before the noun and is marked with 's

**Gist.** A possessive modifier (*John's, my, the company's*) stands before what is possessed. After the noun, possession is expressed with the preposition *of*: *the office of the president*. Curme: the *s*-genitive prevails with names of living beings, *of* with inanimates.
**Conditions and exceptions.** A possessor noun without *'s* is a sign of a missing apostrophe in the text (*the players wives*). The double genitive *a friend of John's* stands after the noun, but that is already `nmod` with *of* (a separate seed). The pronouns *my, your, his* are `nmod:poss` without the clitic.
**Examples.** ✓ *Marie's book* — `nmod:poss(book, Marie)`; ✓ *my book*; ✓ *the office of the president* — `nmod(office, president)`.
**In UD.** `nmod:poss` is always to the left of the head (EWT 2.18: 4 466 of 4 466). NOUN/PROPN/NUM in `nmod:poss` has a POS child (738 of 760; the rest are missing apostrophes in the texts).
**Sources.** https://universaldependencies.org/en/dep/nmod-poss.html, `nmod.md`; Poutsma GLME vol. 3, Ch. XXIV §7–11 (pp. 57–63); Curme 1931 §10 II 1 (p. 73: the *s*-genitive and living beings).

```rule
rule: en.nominal.poss-before-head
what: nmod:poss stands before its head
match: n[]; p[rel=nmod:poss, head=n]
require: p[before=n]
severity: error
source: UD en nmod:poss; Poutsma GLME III Ch. XXIV §7
```

```rule
rule: en.nominal.poss-noun-needs-s
what: a noun possessor (nmod:poss) has the clitic 's / ' (without it — a missing apostrophe)
match: p[rel=nmod:poss, upos=NOUN|PROPN|NUM]
require: exists s[xpos=POS, head=p]
severity: warn
source: Poutsma GLME III Ch. XXIV §1; UD en nmod:poss
```
