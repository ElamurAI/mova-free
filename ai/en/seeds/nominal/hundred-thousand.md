# hundred, thousand, dozen: part of a number or a noun

**Gist.** After a numeral, *hundred, thousand, million, dozen* stay without *-s*: *two hundred people, three dozen eggs* — this is part of the number. The form with *-s* (*hundreds of people, thousands of years, dozens of times*) is already a noun governing an *of*-phrase.
**Conditions and exceptions.** *A hundred* ≈ *one hundred*: the article replaces *one* (Poutsma XLII §2). In a complex number the smaller member specifies the larger: *four thousand* — "four" on "thousand".
**Examples.** ✓ *five hundred dollars*; ✓ *hundreds of dollars*; ✗ *five hundreds dollars*.
**In UD.** *hundred* in a number is NUM CD with `nummod` to the noun; the multiplier is `compound`: `compound(hundred, two)` (EWT 2.18: NUM → NUM to the left — `compound` 142, `nummod` 17). *hundreds, thousands, millions, billions, dozens* — NOUN NNS `Number=Plur`, the lemma is singular (EWT: 51 of 51).
**Sources.** Poutsma GLME vol. 4, Ch. XLII §1–2 (p. 545 = book p. 1225); Poutsma GLME vol. 3, Ch. XXV §28 (*a hundred people*; p. 270); https://universaldependencies.org/en/dep/compound.html (numbers: *four thousand*).

```rule
rule: en.nominal.hundreds-noun
what: hundreds, thousands, millions, billions, dozens are plural nouns, not NUM
match: h[form=hundreds|thousands|millions|billions|dozens]
require: h[upos=NOUN, feats.Number=Plur]
severity: error
source: Poutsma GLME IV Ch. XLII §1–2; UD en compound (numbers)
```

```rule
rule: en.nominal.complex-number-compound
what: in a complex number (four thousand) the smaller member is compound of the larger, not nummod
match: a[upos=NUM]
require: none c[upos=NUM, rel=nummod, head=a]
severity: warn
source: UD en compound (numbers: four thousand); Poutsma GLME IV Ch. XLII §1
```
