# many, several, few, both — with plural; much — with singular

**Gist.** *Many, several, few, both* count separate items, so the noun is plural: *many people, few chances, both hands*. *Much* (and *little* in the sense "not much") measures mass, so it goes with an uncountable singular noun: *much time, little money*.
**Conditions and exceptions.** *Many a man* is the bookish distributive *many a* with a singular (`det:predet`, see predeterminers). *Little* in the sense "small" is an ordinary adjective: *little kids*. Colloquial *much more people* occurs, but it is a deviation.
**Examples.** ✓ *many people*; ✓ *several times*; ✓ *much time*; ✗ *much books*; ✓ *many a day* (bookish).
**In UD.** *many, several, few, much, little* are ADJ JJ with `amod` (not DET); *both* is DET with `det`. EWT 2.18: *many* + plural 160, + singular 1; *few* + plural 128; *much* + singular 50, + plural 5.
**Sources.** Poutsma GLME vol. 4, Ch. XL §26–30 (*both*), §57–62 (*few*), §64–68 (*little*), §85–90 (*many*, *many a*), §91–93 (*much*) (pp. 381–432 = book pp. 1061–1112); Poutsma GLME vol. 3, Ch. XXV §22 (p. 260).

```rule
rule: en.nominal.count-quantifier-plural
what: many, several, few, both on a noun — the noun is plural (many a is det:predet, not here)
match: n[upos=NOUN]; q[lemma=many|several|few|both, rel=amod|det, head=n]
require: n[feats.Number=Plur|Ptan]
severity: warn
source: Poutsma GLME IV Ch. XL §26–30, §57–62, §85–87
```

```rule
rule: en.nominal.much-singular
what: much on a noun — an uncountable singular noun
match: n[upos=NOUN]; q[lemma=much, rel=amod, head=n]
require: n[feats.Number=Sing]
severity: warn
source: Poutsma GLME IV Ch. XL §91–93; Poutsma GLME III Ch. XXV §22
```
