# Uncountable nouns: information, advice, furniture

**Gist.** Names of substances and abstract nouns (Poutsma: "material and abstract nouns") are conceived as unbounded, so they have no plural and take no indefinite article. For some of them this rule is strict: *information, advice, furniture, equipment, luggage, baggage, homework, feedback*. A portion is named with *a piece of*: *a piece of advice*.
**Conditions and exceptions.** Many uncountables become countable in another sense — varieties or portions: *wines, teas, a coffee*, *a knowledge of Latin*, literary *researches*. For the words in the list above there is no such use in the norm; *informations, advices, an equipment* are typical non-native errors.
**Examples.** ✓ *some advice*; ✓ *a piece of furniture*; ✗ *an information*; ✗ *furnitures*.
**In UD.** NOUN NN with `Number=Sing`; `det` *a/an* on them is suspicious. EWT 2.18: *information* 89, *advice* 24, *equipment* 12, *furniture* 11 — all NN Sing and never with *a/an*; the only plural is *luggages* (an error in the text itself).
**Sources.** Poutsma GLME vol. 3, Ch. XXV §22–24 (pp. 260–263 = book pp. 240–243); Curme 1931, Ch. XXVI "Plural of Names of Materials", "Plural of Abstract Nouns" (pp. 542–543).

```rule
rule: en.nominal.mass-no-a
what: uncountable information/advice/furniture/equipment/luggage/baggage/homework/feedback take no a/an
match: n[upos=NOUN, lemma=information|advice|furniture|equipment|luggage|baggage|homework|feedback]
require: none c[lemma=a, rel=det, head=n]
severity: warn
source: Poutsma GLME III Ch. XXV §22–24
```

```rule
rule: en.nominal.mass-no-plural
what: uncountable information/advice/furniture/equipment/luggage/baggage/homework/feedback/knowledge/evidence/software/hardware — singular only: neither Number=Plur nor forms in -s (informations is an error in the text)
match: n[upos=NOUN, lemma=information|advice|furniture|equipment|luggage|baggage|homework|feedback|knowledge|evidence|software|hardware|informations|advices|furnitures|equipments|luggages|baggages|homeworks|feedbacks|knowledges|evidences|softwares|hardwares]
require: n[feats.Number=Sing, suffix!=s]
severity: warn
source: Poutsma GLME III Ch. XXV §22–24; P17-1074 (table 2: NOUN:INFL, informations → information)
```
