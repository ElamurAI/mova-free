# Noun number: tags NN/NNS/NNP/NNPS and the Number feature

**Gist.** The number of an English noun is visible from its form: singular is tagged NN (proper nouns NNP), plural NNS (NNPS). The `Number` feature must agree with the tag.
**Conditions and exceptions.** Pluralia tantum (*trousers, goods, clothes*) are NNS with `Number=Ptan`. Invariable *sheep, fish, species, aircraft* take their number from context (EWT: *sheep* 2 times NN and 2 times NNS). Number goes by form, not meaning: *family, police* in the sense of a group of people stay as their form says.
**Examples.** *book* NN Sing; *books* NNS Plur; *the Alps* NNPS Plur; *clothes* NNS Ptan.
**In UD.** NOUN/PROPN with `Number=Sing|Plur|Ptan`. EWT 2.18: NN → Sing 32 706 without exceptions, NNP → Sing 15 355, NNS → Plur or Ptan 10 287.
**Sources.** Poutsma GLME vol. 3, Ch. XXV §1–14 (plural formation; pp. 132–161 = book pp. 112–141); Santorini 1990 (PTB): NN, NNS, NNP, NNPS; https://universaldependencies.org/en/feat/Number.html.

```rule
rule: en.nominal.nn-sing
what: NN and NNP — a singular noun
match: n[upos=NOUN|PROPN, xpos=NN|NNP]
require: n[feats.Number=Sing]
severity: error
source: Santorini 1990 (PTB) NN, NNP; UD en Number
```

```rule
rule: en.nominal.nns-plur
what: NNS and NNPS — a plural noun (Plur or Ptan)
match: n[upos=NOUN|PROPN, xpos=NNS|NNPS]
require: n[feats.Number=Plur|Ptan]
severity: error
source: Santorini 1990 (PTB) NNS, NNPS; UD en Number (Ptan)
```
