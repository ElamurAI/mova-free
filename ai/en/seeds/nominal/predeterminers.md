# Predeterminers: all the, both my, half an, such a, what a

**Gist.** A few words stand **before** the article or possessive pronoun: *all, both, half* before *the/my/these*; *such, what, quite, rather, many* before the indefinite article.
**Conditions and exceptions.** Without a central determiner *all, both* are ordinary `det` (*all people*, *both sides*). *Such* without an article is an adjective (*such people*: ADJ `amod`). *Many a* + singular is a bookish distributive (*many a man*). *Half* can also be a noun (*the first half*). Poutsma: after *all/both* the definite article often drops (*all day*, *both hands*).
**Examples.** *all the time*; *both my parents*; *half an hour*; *such a pity*; *what a mess*; *quite a few*.
**In UD.** Relation `det:predet`, UPOS DET, XPOS PDT (*all, both, half, such, quite, many*) or WDT (*what*). EWT 2.18: `det:predet` always precedes the `det`/`nmod:poss` of the same noun (217 pairs of 217) and almost always co-occurs with it (10 exceptions like *all those*, where the head is the pronoun itself).
**Sources.** Poutsma GLME vol. 1, Ch. VIII §155–156 (pp. 557–558); Poutsma GLME vol. 3, Ch. XXXI §18 (p. 574 = book p. 554); Poutsma GLME vol. 4, Ch. XL §88 (*many a*; p. 429 = book p. 1109); https://universaldependencies.org/en/dep/det-predet.html; Santorini 1990 (PTB), PDT.

```rule
rule: en.nominal.predet-before-det
what: a predeterminer stands before the central determiner or possessive
match: n[]; p[rel=det:predet, head=n]; d[rel=det|nmod:poss, head=n]
require: p[before=d]
severity: error
source: Poutsma GLME I Ch. VIII §155–156; UD en det:predet
```

```rule
rule: en.nominal.predet-needs-det
what: det:predet requires a central determiner or possessive on the same noun
match: n[]; p[rel=det:predet, head=n]
require: exists c[rel=det|nmod:poss, head=n]
severity: warn
source: Poutsma GLME I Ch. VIII §155–156; UD en det:predet
```

```rule
rule: en.nominal.predet-xpos
what: det:predet is DET tagged PDT (all, both, half, such, quite, many) or WDT (what)
match: p[rel=det:predet]
require: p[upos=DET, xpos=PDT|WDT]
severity: error
source: UD en det:predet; Santorini 1990 (PTB) PDT
```
