# Personal pronouns: person, number, gender, case

**Gist.** Personal pronouns are the only English words with a full case paradigm: nominative (*I, he, she, we, they*), objective (*me, him, her, us, them*), possessive (*my, his…*). In *you* and *it* the nominative and objective have merged. Gender is distinguished only in the 3rd person singular: *he* — masculine, *she* — feminine, *it* — neuter.
**Conditions and exceptions.** *They* is plural in form even when it refers to one person ("singular they"): UD keeps `Number=Plur`. *You* has no number (singular and plural have merged), but *yourself/yourselves* do. *Thou, thee, thy* are archaic (`Style=Arch`). The *'s* in *let's* is *us* (lemma *we*, `Case=Acc`).
**Examples.** *I saw him.*; *She met us.*; *They know it.*
**In UD.** PRON, XPOS PRP (possessives PRP$); `PronType=Prs`, `Person`, `Number` (except *you*), `Gender` (only 3rd person singular), `Case=Nom|Acc|Gen`. The lemma is the nominative form: *me* → *I*, *him* → *he*, *us* → *we*, *them* → *they*. EWT 2.18: all rules below hold without exceptions, apart from isolated typos.
**Sources.** Poutsma GLME vol. 4, Ch. XXXII §1–2 (declension; p. 25 = book p. 705); Poutsma GLME vol. 4, Ch. XXXV §11 (you instead of thou; p. 204 = book p. 884); Curme 1931 §3 (p. 3: only pronouns have a separate nominative); https://universaldependencies.org/en/feat/Case.html, `Person.md`, `Gender.md`, `Number.md`.

```rule
rule: en.nominal.pron-nom-forms
what: I, he, she, we, they — personal pronouns in the nominative
match: p[upos=PRON, !feats.Typo, form=i|he|she|we|they]
require: p[xpos=PRP, feats.Case=Nom, feats.PronType=Prs]
severity: error
source: Poutsma GLME IV Ch. XXXII §1; UD en Case
```

```rule
rule: en.nominal.pron-acc-forms
what: me, him, us, them — personal pronouns in the objective
match: p[upos=PRON, !feats.Typo, form=me|him|us|them]
require: p[xpos=PRP, feats.Case=Acc, feats.PronType=Prs]
severity: error
source: Poutsma GLME IV Ch. XXXII §1; UD en Case
```

```rule
rule: en.nominal.pron-masc
what: he, him, his, himself — 3rd person singular masculine
match: p[upos=PRON, !feats.Typo, form=he|him|his|himself]
require: p[feats.Gender=Masc, feats.Person=3, feats.Number=Sing]
severity: error
source: Poutsma GLME IV Ch. XXXII §1; UD en Gender
```

```rule
rule: en.nominal.pron-fem
what: she, her, hers, herself — 3rd person singular feminine
match: p[upos=PRON, !feats.Typo, form=she|her|hers|herself]
require: p[feats.Gender=Fem, feats.Person=3, feats.Number=Sing]
severity: error
source: Poutsma GLME IV Ch. XXXII §1; UD en Gender
```

```rule
rule: en.nominal.pron-neut
what: it, its, itself — 3rd person singular neuter
match: p[upos=PRON, !feats.Typo, form=it|its|itself]
require: p[feats.Gender=Neut, feats.Person=3, feats.Number=Sing]
severity: error
source: Poutsma GLME IV Ch. XXXII §1, §20; UD en Gender
```

```rule
rule: en.nominal.pron-plur
what: we/us/our/ours/ourselves and they/them/their/theirs/themselves — plural (including singular they)
match: p[upos=PRON, !feats.Typo, form=we|us|our|ours|ourselves|they|them|their|theirs|themselves]
require: p[feats.Number=Plur]
severity: error
source: Poutsma GLME IV Ch. XXXII §1; UD en Number
```

```rule
rule: en.nominal.pron-first-sing
what: I/me/my/mine/myself — 1st person singular
match: p[upos=PRON, !feats.Typo, form=i|me|my|mine|myself]
require: p[feats.Person=1, feats.Number=Sing]
severity: error
source: Poutsma GLME IV Ch. XXXII §1; UD en Person, Number
```

```rule
rule: en.nominal.you-no-number
what: you/your/yours — 2nd person without Number (singular and plural have merged)
match: p[upos=PRON, !feats.Typo, form=you|your|yours]
require: p[feats.Person=2, !feats.Number]
severity: error
source: Poutsma GLME IV Ch. XXXII §10, Ch. XXXV §11; UD en Number
```
