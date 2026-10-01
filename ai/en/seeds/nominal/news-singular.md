# News and names of sciences: form in -s, but singular number

**Gist.** Some nouns end in *-s* but are grammatically singular: *news*, names of sciences in *-ics* in the sense of a discipline (*mathematics, physics, linguistics*), of diseases (*measles*) and games (*billiards*). The verb with them is singular: *The news is good.*
**Conditions and exceptions.** *Politics, economics* can also be plural (*His politics are dubious*); in EWT *politics, economics* are annotated as NNS with `Number=Ptan`. *News* is always singular (there is no *a news*; people say *a piece of news*).
**Examples.** *No news is good news.*; *Mathematics is hard.*; *Measles is contagious.*
**In UD.** *news* — NOUN NN `Number=Sing` (EWT 2.18: 45 of 45). Names of sciences — NN Sing or NNS Ptan, depending on use.
**Sources.** Poutsma GLME vol. 3, Ch. XXVI §12–13 (plural nouns construed as singulars; p. 319 = book p. 299); Curme 1931, Ch. XXVI "Plural Used as Singular" (p. 540).

```rule
rule: en.nominal.news-sing
what: news is a singular noun (NN, Number=Sing)
match: n[upos=NOUN, lemma=news]
require: n[xpos=NN, feats.Number=Sing]
severity: error
source: Poutsma GLME III Ch. XXVI §12–13; Curme 1931 Ch. XXVI (Plural Used as Singular)
```
