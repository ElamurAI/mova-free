# Noun number: tags NN, NNS, NNP, NNPS

**Gist.** The English noun has singular and plural; most form the plural with the ending *-s* (*book → books*). Penn Treebank tags encode number in the name: NN and NNP — singular (common and proper noun), NNS and NNPS — plural. The UD feature `Number` must agree with this. Number is determined by **agreement** (which verb and determiner the word is used with), not just by form.

**Conditions and exceptions.**
- Plurals without *-s* (*men, children, sheep*) are still NNS, `Number=Plur` (see `noun-plural-mutation-en`, `noun-plural-zero`).
- A noun in *-s* with singular agreement (*news, linguistics*) — NN, `Number=Sing` (see `noun-singular-in-s`).
- Collective nouns in the singular (*the committee has voted*) — NN, `Sing`; *police, people, cattle* agree as plurals — NNS, `Plur`.
- Plural-only words without a singular (*clothes, scissors*) — NNS with `Number=Ptan` (see `noun-pluralia-tantum`).
- Measures (*10 minutes is not enough*) — still NNS, `Plur`.

**Examples.** *apple* NN — *apples* NNS; *London* NNP; *the Alps* NNPS; *sheep* NN or NNS depending on context.

**In UD.** UPOS NOUN (NN, NNS) or PROPN (NNP, NNPS). NN, NNP → `Number=Sing`; NNS, NNPS → `Number=Plur` or `Number=Ptan`. `Ptan` is possible only with NNS/NNPS.

**Sources.** https://universaldependencies.org/en/feat/Number.html (Sing: NN, NNP; Plur: NNS, NNPS; Ptan; number by agreement), https://universaldependencies.org/en/pos/NOUN.html, `pos/PROPN.md`; Santorini 1990, §4.1, pp. 17–18 (number by agreement: *Linguistics/NN is*, *The police/NNS have*, *Three years/NNS is*); Jespersen MEG II 2.1 (text-2, p. 48–49: number is marked by the noun, pronoun and verb, not the adjective); Kruisinga II.3 §§2136–2141 (vol. 4 = text-4, p. 321–324: *mathematics is, thirty yards is, the news is*).

```rule
rule: en.morph.nns-number-plur
what: NNS and NNPS — plural (or plurale tantum)
match: n[xpos=NNS|NNPS, upos=NOUN|PROPN]
require: n[feats.Number=Plur|Ptan]
severity: error
source: UD en feat/Number (Plur: every NNS, NNPS); Santorini 1990, NNS
```

```rule
rule: en.morph.nn-number-sing
what: NN and NNP as a noun — singular
match: n[xpos=NN|NNP, upos=NOUN|PROPN]
require: n[feats.Number=Sing]
severity: error
source: UD en feat/Number (Sing: every NN, NNP); Santorini 1990, NN
```

```rule
rule: en.morph.ptan-plural-tag
what: plurale tantum — always with a plural tag
match: n[feats.Number=Ptan]
require: n[xpos=NNS|NNPS]
severity: error
source: UD en feat/Number (Ptan: regular plural suffix and plural agreement)
```
