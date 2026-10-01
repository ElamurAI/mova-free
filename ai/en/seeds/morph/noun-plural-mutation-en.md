# Irregular plural: vowel change (men, feet) and -en (children, oxen)

**Gist.** A few old English nouns form the plural not with the ending *-s* but with a change of the root vowel: *man → men, woman → women, foot → feet, tooth → teeth, goose → geese, mouse → mice, louse → lice*. Three more have the old ending *-en*: *ox → oxen, child → children, brother → brethren* (the last only "brothers in faith", ordinarily *brothers*). These are closed lists; new words do not change this way.

**Conditions and exceptions.**
- Compounds with *-man, -woman* change the same way: *policeman → policemen, Englishman → Englishmen*; but *German → Germans, human → humans, talisman → talismans* (here *-man* is not the word "person").
- *Mouse* for a computer mouse — both *mice* and *mouses*.
- In compounds the first element is usually singular: *toothbrush, footprint*, but *teeth* and *feet* occur as modifiers (*teeth whitening*); the tag is NNS all the same.
- The lemma is the singular: *men → man, feet → foot, children → child*.

**Examples.** *two **men**; my **feet** hurt; the **children** played; a pair of **oxen**.*

**In UD.** NOUN, XPOS NNS, `Number=Plur`, lemma — the singular form. The tag NN for *feet, teeth, men* is an error: number is determined by agreement, and these forms are always plural.

**Sources.** Sweet NEG I §1002 (text-1, p. 345: *man–men, foot–feet, goose–geese, tooth–teeth, louse–lice, mouse–mice*), §1003 (p. 345: *oxen, children; brethren* only figuratively), §1004 (p. 345: *-man* in compounds); Whitney §125 (text-1, p. 72: *men, women, feet, teeth, geese, lice, mice; oxen, children, brethren*); https://universaldependencies.org/en/feat/Number.html (number by agreement); Jespersen MEG VI 11.1₁–11.1₅ (text-5, p. 199–202: *mice, lice*; *Germans, Normans, talismans* vs *Englishmen*; *dormice, titmice*), 20.2₁ (p. 359: *oxen* — the only unconditional plural in *-en*); Kruisinga II.2 §§760–764 (text-3, p. 28–29: *policemen* [-mən]; *Romans, Germans*).

```rule
rule: en.morph.mutation-plural-number
what: men, women, feet, teeth, geese, mice, lice, children, oxen, brethren — plural, NNS
match: n[upos=NOUN, form=men|women|feet|teeth|geese|mice|lice|children|oxen|brethren, !feats.Typo]
require: n[xpos=NNS, feats.Number=Plur]
severity: warn
source: Sweet NEG I §1002–1003; Whitney §125; UD en feat/Number
```

```rule
rule: en.morph.mutation-plural-lemma
what: the lemma of an irregular plural is the singular (men → man, feet → foot, children → child)
match: n[upos=NOUN, form=men|women|feet|teeth|geese|mice|lice|children|oxen, !feats.Typo]
require: n[lemma=man|woman|foot|tooth|goose|mouse|louse|child|ox]
severity: error
source: Sweet NEG I §1002–1003; Whitney §125
```
