# Degrees of comparison in -er/-est

**Gist.** Short adjectives form the comparative with the ending *-er* and the superlative with *-est*: *big → bigger → biggest*, *happy → happier → happiest*. Long ones are compared periphrastically: *more difficult, most difficult* (see `adj-comparison-periphrastic`). The choice depends on the length and sound shape of the word, not on a "that's how it is" rule: there is a transition zone where both ways are possible.

**Conditions and exceptions.**
- *-er/-est* is taken by: monosyllables (*big, high, young, sad*); disyllables stressed on the end (*polite, severe, complete*); many disyllables stressed on the first syllable, especially in *-y, -ow, -le, -er* (*happy, easy, narrow, simple, tender, clever*).
- *More/most* is taken by: words in *-ful, -ish, -ive, -ous, -st* (*useful, childish, active, famous, honest*), participial adjectives in *-ed, -ing* (*tired, charming*), almost all words of three or more syllables (*difficult, comfortable*).
- Spelling when adding the ending: doubling of the final consonant after a short stressed vowel (*big → bigger, hot → hottest*), dropping of silent *-e* (*nice → nicer*), *-y* after a consonant → *-i-* (*happy → happier*; see `spell-consonant-doubling`, `spell-silent-e`, `spell-y-to-i`).
- Adverbs without *-ly* are compared the same way (*hard → harder, soon → sooner*; see `adv-flat-degree`).
- The lemma of an *-er/-est* form is the positive degree: *bigger → big, happiest → happy*.

**Examples.** *a **bigger** house; the **easiest** way; **nicer** weather; the **latest** news.*

**In UD.** ADJ, XPOS JJR + `Degree=Cmp` or JJS + `Degree=Sup`; ADV — RBR/RBS. A form tagged JJR/RBR ends in *-er*, one tagged JJS/RBS ends in *-est*; the only exceptions are the irregular *more, less, worse* and *most, least, worst* (see `adj-comparison-suppletive`).

**Sources.** Sweet NEG I §1038–1039 (vol. 1 = text-1, p. 356–357: which adjectives take *-er/-est*, which *more/most*); Whitney §199–200 (p. 105–106); Santorini 1990, §2, p. 1 (JJR — an *-er* form with comparative meaning; JJS — in *-est*, and also *worst*); https://universaldependencies.org/en/feat/Degree.html; Kruisinga II.3 §§1723–1729 (text-4, p. 82–86: stress decides, not the number of syllables; *tenderer, narrower, happier, abler, handsomer*); Mätzner I, p. 273–275 (text-1, p. 291–293: *politer, abler, truer; happier*, but *gayer*; *bigger, hotter*).

```rule
rule: en.morph.comparative-form
what: a form tagged JJR or RBR ends in -er (or is more, less, worse)
match: a[xpos=JJR|RBR, !feats.Typo]
require: a[suffix=er|more|less|worse]
severity: warn
source: Santorini 1990, §2, p. 1 (JJR: -er form, comparative meaning); Sweet NEG I §1038, §1044–1052
```

```rule
rule: en.morph.superlative-form
what: a form tagged JJS or RBS ends in -est (or is most, least, worst)
match: a[xpos=JJS|RBS, !feats.Typo]
require: a[suffix=est|most|least|worst]
severity: warn
source: Santorini 1990, §2, p. 1 (JJS: -est form, plus worst); Sweet NEG I §1043–1052
```
