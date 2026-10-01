# Classifying genitive: a children's story, a day's work

**Gist.** A genitive can be individual — "whose exactly" (*John's car*) — or classifying — "what kind, for whom, of what measure" (*a children's story, a women's college, a fool's errand, a day's work, two weeks' notice*). The classifying one describes a kind, like an adjective (Poutsma).
**Conditions and exceptions.** Hence an article with a classifying genitive belongs to the head, not the possessor: in *a children's story* the article *a* agrees with *story* (singular), not with *children*. The genitive of measure belongs here too: *a stone's throw, an hour's drive*.
**Examples.** *a children's book*; *a bachelor's degree*; *a day's journey*; *two hours' drive*.
**In UD.** `det(story, a)`, `nmod:poss(story, children)`, `case(children, 's)`. So *a/an* is never `det` of a plural possessor. EWT 2.18: such a `det` — 1 time (suspicious).
**Sources.** Poutsma GLME vol. 3, Ch. XXIV §7, §22–23, §40–44 (pp. 57, 83–84, 109–116 = book pp. 37, 63–64, 89–96); Curme 1931 §10 II "Descriptive genitive", "Genitive of measure" (p. 83); https://universaldependencies.org/en/dep/nmod-poss.html.

```rule
rule: en.nominal.classifying-genitive-det
what: a/an is never det of a plural possessor (a children's story — a goes with story)
match: p[rel=nmod:poss, feats.Number=Plur]
require: none c[lemma=a, rel=det, head=p]
severity: error
source: Poutsma GLME III Ch. XXIV §40–41; UD en nmod:poss (a children's story)
```
