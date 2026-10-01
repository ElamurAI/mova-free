# One central determiner per noun

**Gist.** A noun has at most one "central" determiner: an article, demonstrative, possessive or quantifying word. *The my book*, *a this car* are impossible. To combine an article with possession one says *a friend of mine*, *this idea of yours*.
**Conditions and exceptions.** The predeterminers *all, both, half, such, what, quite* stand before the central one (*all the, both my, such a*) — this is a separate relation `det:predet`, not a second `det`. *Many, few, other, same* after a determiner are adjectives (*the many, a few, the other*). Archaic *these my children* (Poutsma) is a rare bookish exception.
**Examples.** ✓ *my old car*; ✓ *all my friends*; ✓ *a friend of mine*; ✗ *the my car*; ✗ *a his friend*.
**In UD.** A noun has at most one `det` child and does not have both `det` and a pronominal `nmod:poss`. EWT 2.18: two `det` on one noun — 2 times out of 19 371 (*the all blacks*, *a another*), `det` together with PRP$ — once (*any his reasons*).
**Sources.** Poutsma GLME vol. 4, Ch. XXXIII §11, §23 (pp. 123, 140 = book pp. 803, 820); Poutsma GLME vol. 1, Ch. VIII §155–156 (pp. 557–558); Curme 1931 §10 II 1 b (double genitive; p. 75).

```rule
rule: en.nominal.single-det
what: a noun has at most one det (a predeterminer is det:predet)
match: n[]; d[rel=det, head=n]
require: none c[rel=det, head=n]
severity: warn
source: Poutsma GLME I Ch. VIII §155–156; UD en det, det:predet
```

```rule
rule: en.nominal.det-with-poss-pron
what: a possessive pronoun (my, his…) does not combine with det on the same noun (a friend of mine)
match: n[]; p[upos=PRON, rel=nmod:poss, head=n]
require: none c[rel=det, head=n]
severity: warn
source: Poutsma GLME IV Ch. XXXIII §11, §23
```
