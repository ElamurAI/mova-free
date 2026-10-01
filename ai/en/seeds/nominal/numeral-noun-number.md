# Numeral and noun number: one book, two books

**Gist.** After *one* (1) the noun is singular, after *two* and larger numbers plural: *one book, two books, forty dollars*.
**Conditions and exceptions.** A measure used as a modifier stays singular: *a two-hour delay*, *a 5 year old* (Poutsma XXV §31). *Percent/per cent* is invariable: *80 percent*. Abbreviated units (*5 lb., 8 GB*) have no number. *Dozen, hundred, thousand* after a numeral take no *-s*: *two dozen eggs*. In *one or two days* the noun agrees with the nearer *two*.
**Examples.** ✓ *one day*; ✓ *three days*; ✓ *a three-day trip*; ✓ *20 percent*; ✗ *two day*.
**In UD.** `nummod` (NUM) → NOUN head with `Number`. EWT 2.18: *one* + singular 205, + plural 5; *two…ten, dozen, hundred, thousand* on a noun head serving as subject, object or adverbial — almost always plural; singular — *percent*, *cent* and a measure as modifier (*2 year old*: `obl:unmarked`).
**Sources.** Poutsma GLME vol. 4, Ch. XLII §1–3 (pp. 545–546 = book pp. 1225–1226); Poutsma GLME vol. 3, Ch. XXV §28, §31 (pp. 270, 287).

```rule
rule: en.nominal.one-singular
what: after the numeral one the noun is singular
match: n[upos=NOUN]; x[rel=nummod, form=one|1, head=n]
require: n[feats.Number=Sing]
severity: warn
source: Poutsma GLME IV Ch. XLII §1; Poutsma GLME III Ch. XXV §28
```

```rule
rule: en.nominal.numeral-plural
what: after two…ten, twelve, twenty, dozen, hundred, thousand a noun subject, object or adverbial is plural
match: n[upos=NOUN, rel=nsubj|nsubj:pass|obj|iobj|obl|nmod|root|conj, !feats.Abbr]; x[rel=nummod, form=two|three|four|five|six|seven|eight|nine|ten|twelve|twenty|dozen|hundred|thousand, head=n]
require: n[feats.Number=Plur|Ptan]
severity: warn
source: Poutsma GLME IV Ch. XLII §1–3; Poutsma GLME III Ch. XXV §31 (modifier — singular)
```
