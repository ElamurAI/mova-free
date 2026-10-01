# Noun suffixes: -ness, -ity, -ism, -hood, -tion, -ment, -er

**Gist.** A derivational suffix, unlike a prefix, usually **changes** the part of speech and itself determines it: *kind → kindness, real → reality, social → socialism, child → childhood, create → creation, govern → government, teach → teacher*. So the end of a word is a strong hint for the tag: a word in *-ness, -ity, -ism, -hood* is almost always a noun, and *-nesses, -ities, -isms, -tions, -ments* is a plural noun.

**Conditions and exceptions.**
- *-ness* is the most productive: from any adjective and participle (*kindness, readiness, preparedness*), even from phrases (*up-to-dateness*); it is not formed where a noun already exists (*possible → possibility*). Special cases: *business* (≠ *busyness*), *wilderness*, *witness* (also a verb), *harness* (also a verb).
- *-ity* — with Latin stems (*reality, ability*); *pity* can be a verb; *quality* before a noun (*quality goods*) is sometimes tagged JJ in EWT.
- *-ism/-ist* go in a pair with *-ize* (*social → socialism, socialist, socialize*); *-ist* can also be an adjective (*racist*).
- *-hood, -ship, -dom*: *childhood, likelihood; friendship, membership* (*worship* — also a verb); *freedom, kingdom* (but *random* is an adjective, *seldom* an adverb: there *-dom* is not a suffix).
- *-tion/-sion/-ation, -ment, -ance/-ence, -al* — names of actions and results (*creation, government, arrival*); many of them have also become verbs by conversion (*to question, to mention, to function, to comment*).
- Persons: *-er/-or* (*teacher, actor*), *-ee* (*employee, referee*), *-ess* (*actress, hostess*), *-ist* (*cyclist*), *-ant/-ent* (*assistant, student*); diminutives *-let, -ling, -ette* (*booklet, duckling*).
- *-th* from adjectives: *warm → warmth, long → length, strong → strength, deep → depth, wide → width, true → truth*.

**Examples.** *his **kindness**; the **reality**; **racism**; in my **childhood**; two **creations***.

**In UD.** NOUN (NN/NNS) with `Number`; the lemma is the singular of the derived word (*kindnesses → kindness*), not the base stem.

**Sources.** Jespersen MEG VI 19.3₁–19.3₄ (text-5, pp. 331–333: *-ness*, *business/busyness, witness*), 19.1 (pp. 328–330: *-ess*), 19.4, 19.9 (pp. 334–335, 349–351: *-ize/-ism/-ist*), 21.8₃ (p. 393: *-ment*), 14.1₅ (pp. 243–244: *-er*), 13.6₁ (p. 236: *-ee*), 24.4₅–24.4₆ (pp. 457–458: *-th*), 25.3 (pp. 474–477: *-hood, -ship, -dom*), 23.4–23.5 (pp. 437–441: *-let, -ling*), 22.2₂ (p. 401: *-al*); Sweet NEG I §1597–1605 (text-1, pp. 489–493), §1682–1718, §1733 (pp. 509–522); Kruisinga II.3 §§1634–1681 (vol. 4 = text-4, pp. 45–65: living suffixes; *-ness* not where a noun exists); Mätzner I, p. 450 (p. 468: *-ness*); Whitney §90–96, §118 (text-1, pp. 57–59, 69).

```rule
rule: en.morph.suffix-ness-noun
what: word in -ness(es) — noun (only witness, harness can be verbs)
match: w[suffix=ness|nesses, !feats.Typo]
require: w[upos=NOUN|PROPN|VERB|X]
severity: warn
source: Jespersen MEG VI 19.3₁–19.3₄; Sweet NEG I §1597; Kruisinga II.3 §§1634–1648
```

```rule
rule: en.morph.suffix-ity-ism-hood-noun
what: word in -ity, -ism, -hood (and their plurals) — noun
match: w[suffix=ity|ities|ism|isms|hood|hoods, !feats.Typo]
require: w[upos=NOUN|PROPN|X]
severity: warn
source: Jespersen MEG VI 19.4, 25.3; Sweet NEG I §1601–1605, §1689–1718
```

```rule
rule: en.morph.suffix-tion-noun
what: word in -tion/-sion — noun (or a verb formed by conversion: to question, to mention)
match: w[suffix=tion|tions|sion|sions, !feats.Typo]
require: w[upos=NOUN|PROPN|VERB|X]
severity: warn
source: Sweet NEG I §1689–1718; Jespersen MEG VI 21.6–21.8
```

```rule
rule: en.morph.derived-plural-noun
what: noun in -nesses, -ities, -isms, -tions, -ments, -ships, -hoods — plural
match: n[upos=NOUN, suffix=nesses|ities|isms|tions|sions|ments|ships|hoods, !feats.Typo]
require: n[xpos=NNS, feats.Number=Plur|Ptan]
severity: warn
source: Jespersen MEG VI 16.1₁; Sweet NEG I §1000
```
