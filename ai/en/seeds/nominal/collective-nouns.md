# Collective nouns: singular form, plural meaning

**Gist.** Collective nouns of the first type (*family, team, government, committee, army, party*) are singular in form but name a group. When its members are meant, the verb and pronoun can be plural, especially in British English: *The committee are divided.* Collectives of the second type (*people, cattle, police, clergy, vermin*) are always plural in meaning: *these people, a hundred people*.
**Conditions and exceptions.** `Number` in UD follows the word form, not the meaning: *family* has `Sing` even with *are*. So subject–predicate agreement cannot be checked strictly for collectives. *People* "nation" has the plural *peoples*.
**Examples.** *My family is/are here.*; *The police have arrived.*; *Many people were there.*
**In UD.** *people* — NOUN NNS `Number=Plur` (EWT 2.18: 315 of 316). *Police* is annotated inconsistently (15 times NNS Plur, 11 — NN Sing). Collectives of the first type — NN Sing.
**Sources.** Poutsma GLME vol. 3, Ch. XXVI §6–7 (pp. 300–301 = book pp. 280–281); Poutsma GLME vol. 3, Ch. XXV §27–28 (*people, cattle*; pp. 266–270); Curme 1931 §8 1 d and Ch. XXVI "Collective Nouns" (pp. 50, 539).

```rule
rule: en.nominal.people-plural
what: people meaning "persons" is plural (NNS, Number=Plur)
match: n[upos=NOUN, form=people, !feats.Typo]
require: n[xpos=NNS, feats.Number=Plur]
severity: warn
source: Poutsma GLME III Ch. XXV §28 (people — plural in all respects)
```
