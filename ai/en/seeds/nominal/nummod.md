# Cardinal numeral on a noun: nummod on NUM, before the noun

**Gist.** A numeral that counts things (*three books, forty dollars*) is a separate kind of modifier. It stands before the noun, together with other determiner-like words (Poutsma: numerals come before adjectives).
**Conditions and exceptions.** After a currency sign the number goes to the right: *$ 40* — `nummod($, 40)`. A number as identifier after a noun is not a quantity: *page 394, Route 66, World War II* — `flat`. Dates and addresses (*October 8, 1963*) are `nmod:unmarked`. Ordinals (*first, third*) are ADJ with `amod`. In a compound number the smaller member is `compound` of the larger (see the seed about *hundred*).
**Examples.** ✓ *three books* — `nummod(books, three)`; ✓ *room 101* — `flat(room, 101)`; ✗ `nummod(page, 394)`.
**In UD.** `nummod` → NUM (EWT 2.18: 1 794 of 1 794); to the left of NOUN/PROPN (to the right only with SYM *$*). NUM as `amod` — 10 times, all suspicious.
**Sources.** https://universaldependencies.org/en/dep/nummod.html, `nmod-desc.md` (Numbered Entities), `nmod-unmarked.md` (Dates); Poutsma GLME vol. 4, Ch. XLII §3, §7 (pp. 546–566); Poutsma GLME vol. 1, Ch. VIII §156 (p. 558).

```rule
rule: en.nominal.nummod-num
what: a nummod dependent is a cardinal numeral NUM
match: x[rel=nummod]
require: x[upos=NUM]
severity: error
source: UD en nummod; Poutsma GLME IV Ch. XLII §3
```

```rule
rule: en.nominal.nummod-before-noun
what: nummod stands before the noun; a number after the noun is an identifier (flat) or a date (nmod:unmarked)
match: n[upos=NOUN|PROPN]; x[rel=nummod, head=n]
require: x[before=n]
severity: error
source: UD en nummod, nmod:desc (Numbered Entities); 2023.udw-1.7
```

```rule
rule: en.nominal.num-not-amod
what: a cardinal numeral NUM is never amod (ordinals are ADJ)
match: x[upos=NUM]
require: not x[rel=amod]
severity: warn
source: UD en nummod, amod; Poutsma GLME IV Ch. XLII §11–12
```
