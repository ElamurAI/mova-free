# Order of modifiers before the noun

**Gist.** Before a noun, modifiers come in a fixed order: predeterminer (*all, both*) → determiner or possessive (*the, these, my*) → numeral (*two, three*) → adjectives → noun modifier → head: *all my three old college friends*. Poutsma: the article, pronoun and numeral stand before adjectives and noun modifiers; numerals before adjectives (*two great men*).
**Conditions and exceptions.** *First, last, next* and words close in meaning (*other, only, same, past, top, final, additional, further, full, whole*) stand **before** the cardinal: *the first three days, the last two years, the other two, an additional 20 minutes*; Poutsma also allows *the three first months*. An adjective with *so/as/too/how* goes before the article (*so big a house*, a separate seed). The classifying genitive allows an adjective before it (*a new children's book*).
**Examples.** ✓ *the two old men*; ✓ *the next five years*; ✗ *great two men*; ✗ *two the men*.
**In UD.** For one noun: `det:predet` < `det`/`nmod:poss` < `nummod` < `amod` < `compound` < head. EWT 2.18: `det` before `nummod` — 172:0; `det` before `compound` — 3 759:1; `nummod` before `amod` — 169, the reverse — 67, of which 61 are words like *first/last/next* (the other 6 are suspects).
**Sources.** Poutsma GLME vol. 1, Ch. VIII §150, §156–157 (pp. 554, 558); Curme 1931 §10 I 1 (p. 63).

```rule
rule: en.nominal.det-before-nummod
what: the determiner stands before the numeral of the same noun (the two men, not two the men)
match: n[]; d[rel=det, head=n]; x[rel=nummod, head=n, before=n]
require: d[before=x]
severity: error
source: Poutsma GLME I Ch. VIII §150, §156
```

```rule
rule: en.nominal.det-before-compound
what: the determiner stands before the noun modifier (compound) of the same noun
match: n[]; d[rel=det, head=n]; c[rel=compound, head=n]
require: d[before=c]
severity: warn
source: Poutsma GLME I Ch. VIII §150
```

```rule
rule: en.nominal.adj-before-numeral
what: only first/last/next and similar stand before a cardinal numeral; other adjectives go after the numeral
match: n[]; x[rel=nummod, head=n, before=n]; a[upos=ADJ, rel=amod, head=n, before=x]
require: a[lemma=first|last|next|past|top|only|full|other|same|further|final|initial|extra|additional|previous|following|remaining|entire|whole|good|certain|mere|very]
severity: warn
source: Poutsma GLME I Ch. VIII §156–157
```
