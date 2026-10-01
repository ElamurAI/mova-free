# Determiner types: demonstrative, total, indefinite, negative

**Gist.** English determiners are a closed list, and each has a fixed type of meaning: demonstrative (*this, that*), total — "all, every" (*all, every, each, both*), indefinite (*some, any, another, either, such*), negative (*no, neither*), interrogative and relative (*what, which, whatever*).
**Conditions and exceptions.** *Many, much, few, little, several, enough* are counted as determiners by grammars (and CGEL), but in English UD they are ADJ with `amod`. Cardinal numerals are NUM with `nummod`. *Each* in *each other* is reciprocal (`PronType=Rcp`), see the seed on reciprocal pronouns.
**Examples.** *this car* (Dem); *every day* (Tot); *some water* (Ind); *no idea* (Neg); *what time* (Int).
**In UD.** DET; XPOS DT, PDT for predeterminers, WDT for *what/which*. `PronType` by lemma; demonstratives also have `Number`. EWT 2.18: *all* 564 Tot, *every* 113 Tot, *some* 429 Ind, *any* 403 Ind, *no* 323 Neg — no exceptions.
**Sources.** Poutsma GLME vol. 4, Ch. XXXVI §1 (demonstratives; p. 211 = book p. 891), Ch. XXXVII §7–8 (*such*; pp. 252–253), Ch. XL (*all* §1–7, *any* §16, *both* §26–28, *each* §35–36, *every* §51, *neither* §109–111, *no* §114–115; pp. 331–447); CGELBank (`data/raw/en-gram-cgelbank`, datasets/ewt.cgel): *many, several* and numerals are category D; https://universaldependencies.org/en/feat/PronType.html.

```rule
rule: en.nominal.det-prontype-tot
what: all, every, both are total determiners (PronType=Tot)
match: d[upos=DET, lemma=all|every|both]
require: d[feats.PronType=Tot]
severity: error
source: Poutsma GLME IV Ch. XL §1, §26–28, §51; UD en PronType
```

```rule
rule: en.nominal.det-prontype-each
what: each is total (Tot), and in each other reciprocal (Rcp)
match: d[upos=DET, lemma=each]
require: d[feats.PronType=Tot|Rcp]
severity: error
source: Poutsma GLME IV Ch. XL §35–37; UD en PronType
```

```rule
rule: en.nominal.det-prontype-ind
what: some, any, another, either, such, quite, half are indefinite determiners (PronType=Ind)
match: d[upos=DET, lemma=some|any|another|either|such|quite|half]
require: d[feats.PronType=Ind]
severity: error
source: Poutsma GLME IV Ch. XL §16; Ch. XXXVII §7; UD en PronType
```

```rule
rule: en.nominal.det-prontype-neg
what: no, neither as determiners are negative (PronType=Neg)
match: d[upos=DET, lemma=no|neither]
require: d[feats.PronType=Neg]
severity: error
source: Poutsma GLME IV Ch. XL §109–115; UD en PronType
```

```rule
rule: en.nominal.det-prontype-dem
what: this/these, that/those as determiners are demonstratives with a number feature
match: d[upos=DET, lemma=this|that]
require: d[feats.PronType=Dem, feats.Number]
severity: error
source: Poutsma GLME IV Ch. XXXVI §1; UD en PronType, Number
```
