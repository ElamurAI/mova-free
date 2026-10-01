# A demonstrative agrees with its noun in number

**Gist.** *This/that* go with the singular, *these/those* with the plural: *this book, these books*. These are the only English determiners that inflect for number.
**Conditions and exceptions.** Colloquial *these kind of men* is agreement by meaning: *kind of* is felt as a modifier (Curme: colloquial in Britain, substandard in America). A measure as one whole: *that ten years*, *this three weeks* (cf. *a three months*, Poutsma XXVI §17). Pluralia tantum go with *these/those*: *these trousers*.
**Examples.** ✓ *this book*, *these books*; ~ *these kind of things*; ✗ *this books*; ✗ *those car*.
**In UD.** DET, lemma *this* (forms this/these) or *that* (that/those), `PronType=Dem`, `Number=Sing|Plur`. The number of the head is its `Number` (`Plur` or `Ptan` for plural). EWT 2.18: singular with singular 1 090, plural with plural 279, mismatches 6.
**Sources.** Poutsma GLME vol. 4, Ch. XXXVI §1 (p. 211 = book p. 891); Curme 1931, Ch. XXVI §59 7 "Plural of Kind, Sort" (pp. 544–545); Poutsma GLME vol. 3, Ch. XXVI §17 (p. 324 = book p. 304).

```rule
rule: en.nominal.dem-number
what: a demonstrative determiner and its noun have the same number (this book, these books; these scissors — Ptan); these kind of is colloquial
match: n[upos=NOUN]; d[rel=det, feats.PronType=Dem, feats.Number, head=n]
require: n[feats.Number=@d] or d[feats.Number=Plur]; n[feats.Number=@d] or n[feats.Number=Ptan]
severity: warn
source: Poutsma GLME IV Ch. XXXVI §1; Curme 1931 §59 7; 2020.tacl-1.25 (BLiMP DET-NOUN AGR); P17-1074 (NOUN:NUM)
```
