# Personal pronouns: person, number, gender

**Gist.** Every personal pronoun form unambiguously states the person (1st — the speaker, 2nd — the addressee, 3rd — everyone else), and most also the number. English distinguishes gender (masculine, feminine, neuter) **only** in the 3rd person singular: *he, she, it* with all their forms. Nouns have no gender, so they get no `Gender` feature in UD.

**Conditions and exceptions.**
- *You, your, yours* — 2nd person without number (singular and plural have merged; *you* was once plural and *thou* singular). Number exists only in the reflexives: *yourself* — singular, *yourselves* — plural.
- *They, them, their* — always `Number=Plur`, even meaning "he or she" (*singular they*): the number is grammatical, by agreement (*they are*).
- Generic *one* (*one must try*) — 3rd person singular without gender.
- *It* as a dummy subject (*it rains*) — the same personal pronoun `PronType=Prs`.

**Examples.** *I/me/my/mine/myself* — `Person=1|Number=Sing`; *we/us/our/ours/ourselves* — `Person=1|Number=Plur`; *he/him/his/himself* — `Person=3|Number=Sing|Gender=Masc`; *it/its/itself* — `Gender=Neut`; *they/them/their/theirs/themselves* — `Person=3|Number=Plur`.

**In UD.** UPOS PRON; XPOS PRP or PRP$; `PronType=Prs` (reflexives in emphatic use — `Emp`); `Person`, `Number`, `Gender` — per the table in `pos/PRON.md`.

**Sources.** https://universaldependencies.org/en/pos/PRON.html (table of personal pronouns), `feat/Person.md`, `feat/Number.md`, `feat/Gender.md` (gender only on 3rd-person personal pronouns); Sweet NEG I §1076–1088 (text-1, p. 368–372: *thou, thee, ye* only in liturgy and poetry; *you* displaced *ye*), §1101 (p. 375: *its* spread only at the end of Early Modern English); Whitney §155, §158, §160 (text-1, p. 86–87); Jespersen MEG II 2.23 (vol. 2 = text-2, p. 50: full paradigm with *-self* forms).

```rule
rule: en.morph.pron-1sg
what: I, me, my, mine, myself — 1st person singular
match: p[upos=PRON, form=i|me|my|mine|myself, !feats.Typo]
require: p[feats.Person=1, feats.Number=Sing]
severity: error
source: UD en pos/PRON, feat/Person
```

```rule
rule: en.morph.pron-1pl
what: we, us, our, ours, ourselves — 1st person plural
match: p[upos=PRON, form=we|us|our|ours|ourselves, !feats.Typo]
require: p[feats.Person=1, feats.Number=Plur]
severity: error
source: UD en pos/PRON, feat/Person
```

```rule
rule: en.morph.pron-2
what: you, your, yours, yourself, yourselves — 2nd person
match: p[upos=PRON, form=you|your|yours|yourself|yourselves, !feats.Typo]
require: p[feats.Person=2]
severity: error
source: UD en pos/PRON, feat/Person
```

```rule
rule: en.morph.pron-3sg-masc
what: he, him, his, himself — 3rd person singular, masculine
match: p[upos=PRON, form=he|him|his|himself, !feats.Typo]
require: p[feats.Person=3, feats.Number=Sing, feats.Gender=Masc]
severity: error
source: UD en pos/PRON, feat/Gender
```

```rule
rule: en.morph.pron-3sg-fem
what: she, her, hers, herself — 3rd person singular, feminine
match: p[upos=PRON, form=she|her|hers|herself, !feats.Typo]
require: p[feats.Person=3, feats.Number=Sing, feats.Gender=Fem]
severity: error
source: UD en pos/PRON, feat/Gender
```

```rule
rule: en.morph.pron-3sg-neut
what: it, its, itself — 3rd person singular, neuter
match: p[upos=PRON, form=it|its|itself, !feats.Typo]
require: p[feats.Person=3, feats.Number=Sing, feats.Gender=Neut]
severity: error
source: UD en pos/PRON, feat/Gender
```

```rule
rule: en.morph.pron-3pl
what: they, them, their, theirs, themselves — 3rd person plural
match: p[upos=PRON, form=they|them|their|theirs|themselves, !feats.Typo]
require: p[feats.Person=3, feats.Number=Plur]
severity: error
source: UD en pos/PRON, feat/Number
```

```rule
rule: en.morph.gender-only-3sg
what: gender occurs only on a 3rd person singular pronoun
match: p[feats.Gender]
require: p[upos=PRON, feats.Person=3, feats.Number=Sing]
severity: error
source: UD en feat/Gender (only 3rd-person personal pronouns)
```
