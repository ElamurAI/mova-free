# Ordinal numerals are adjectives

**Gist.** *First, second, third, 21st* name a place in a series, not a quantity. Syntactically they behave like adjectives: they stand after the article (*the second day*) and can take an intensifier (*the very first*).
**Conditions and exceptions.** *First* can be an adverb (*First, we…* — ADV RB). *Second* as a noun means "a second (of time)". Ordinals and *last, next* can stand before a cardinal: *the first three chapters* (more usual) and *the three first* (Poutsma I VIII §157).
**Examples.** *the second day*; *his 21st birthday*; *the first three weeks*.
**In UD.** ADJ, XPOS JJ, `Degree=Pos|NumForm=Word|NumType=Ord` (digit forms `NumForm=Combi`: *2nd*), relation `amod`, not `nummod`. EWT 2.18: *first* ADJ 129, *second* 46, *third* 17 — all `amod`.
**Sources.** Poutsma GLME vol. 4, Ch. XLII §11–13 (pp. 575–576 = book pp. 1255–1256); Poutsma GLME vol. 1, Ch. VIII §157 (p. 558); https://universaldependencies.org/en/feat/NumType.html.

```rule
rule: en.nominal.ordinal-not-nummod
what: an ordinal (NumType=Ord) is never nummod — it is amod
match: o[feats.NumType=Ord]
require: not o[rel=nummod]
severity: error
source: Poutsma GLME IV Ch. XLII §11–12; UD en NumType
```

```rule
rule: en.nominal.ordinal-adj-feats
what: the adjectives first/second/third… have NumType=Ord
match: o[upos=ADJ, lemma=first|second|third|fourth|fifth|sixth|seventh|eighth|ninth|tenth]
require: o[feats.NumType=Ord]
severity: warn
source: Poutsma GLME IV Ch. XLII §11; UD en NumType
```
