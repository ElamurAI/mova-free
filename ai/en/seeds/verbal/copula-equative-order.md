# Equating two nouns: the subject is the first one

**Gist.** In equative sentences (*Rolfe's wife was Pocahontas*, *The title is Green Eggs and Ham*) both nouns are on equal footing, and deciding which is the subject is hard. Reed & Kellogg advised looking at meaning: the subject is what the reader already knows. UD chooses a formal criterion: the subject is the one that comes first, the predicative is the second.

**Conditions and exceptions.** The subject comes after the predicative in inversion: *Among the weapons intercepted were two launchers*, *In the inner circle are the terrorists*, in questions (*Who is he?*). Brown: a verb between two nouns agrees with the preceding one, "except when the terms are transposed" (*The wages of sin is death*).

**Examples.** *Words are wind.* — *Lizards are reptiles.* — *Among the strongest proponents of war were Sharon and his supporters* (inversion: nsubj after).

**In UD.** For a `NOUN`/`PROPN` head with `cop`, the noun subject (`nsubj`) stands to the left of the head; otherwise it is inversion (to check) or swapped roles.

**Sources.** https://universaldependencies.org/en/specific-syntax.html, Copulas ("the subject is taken to appear first"); Reed & Kellogg, Higher Lessons, Lesson 29 (Explanation: *A dainty plant is the ivy green*); Brown 1851, Rule XIV, Note V.

```rule
rule: en.verbal.equative-subject-first
what: in an equative sentence the noun subject stands before the predicative noun
match: h[upos=NOUN|PROPN]; c[rel=cop, head=h]; s[rel=nsubj, head=h, upos=NOUN|PROPN]
require: s[before=h]
severity: warn
source: https://universaldependencies.org/en/specific-syntax.html, Copulas; Brown 1851 Rule XIV, Note V
```
