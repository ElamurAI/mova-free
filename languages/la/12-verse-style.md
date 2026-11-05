# 12. Verse and the poetic language

Classical Latin verse is built on the length of syllables, not on stress. A line is a fixed pattern of long and short syllables, and the words are fitted into it. The fit explains much of what a parser meets in poetry: the free order, the separated adjectives, the old and Greek forms, the shortened words. Medieval hymns and songs count syllables and stress instead, and rhyme. This chapter gives the quantity of syllables, the main metres, the licences of verse, and the poetic language that a reader of Virgil, Ovid, Horace or Seneca's tragedies has to expect.

## Contents

1. Syllable quantity
2. Elision
3. The dactylic hexameter
4. The elegiac couplet
5. Lyric and dramatic metres
6. The poetic language
7. Accentual verse and rhyme
8. How Mova handles it

---

## 1. Syllable quantity

A syllable is **long** (Bennett 1918 §§360–365):
- **by nature** when its vowel is long or a diphthong: *rō-ma*, *cae-lum*;
- **by position** when its vowel, long or short, is followed by two consonants (in the same word or across words) or by *x* or *z*: *ar-ma*, *est* before a consonant.

Otherwise it is **short**. A short vowel before a stop with *l* or *r* (*pa-tris*) may count either way (*muta cum liquida*). A vowel before another vowel is short (*dĕ-us*); *h* does not count as a consonant. The quantities of vowels are in [chapter 1](01-sounds-spelling.md); the dictionaries mark them with macrons, as this manual does.

## 2. Elision

A word that ends in a vowel, a diphthong or a vowel with *-m* loses that ending before a word that begins with a vowel or *h*: *mōnstr(um) horrend(um) īnfōrm(e) ingēns* is read *mōnstrorrendinfōrmingēns*. Before *es* and *est* the second vowel goes instead (*prodig(i)ost* for *prōdigium est*, prodelision). Elision is a matter of the metre: the text keeps the full spelling, and a tokenizer meets ordinary words (Bennett 1918 §367).

## 3. The dactylic hexameter

The metre of epic (Ennius, Lucretius, Virgil's *Aeneid*, Ovid's *Metamorphoses*), of didactic poetry and of satire. Six feet (Bennett 1918 §368):
- the first four are dactyls (– ⏑ ⏑) or spondees (– –);
- the fifth is almost always a dactyl;
- the sixth is two syllables, the last long or short (– ×).

A word end, the **caesura**, falls in the third foot, after its first syllable (the strong caesura) or between its shorts (the weak).

> *Arma virumque canō, Trōiae quī prīmus ab ōrīs* (Verg. *Aen.* 1.1)
> – ⏑ ⏑ | – ⏑ ⏑ | – ‖ – | – – | – ⏑ ⏑ | – –
> "Arms and the man I sing, who first from the shores of Troy …"

## 4. The elegiac couplet

A hexameter followed by a **pentameter**, two halves of two and a half dactyls each, with a fixed word end between them (– ⏑ ⏑ | – ⏑ ⏑ | – ‖ – ⏑ ⏑ | – ⏑ ⏑ | ×). The metre of love elegy (Catullus, Tibullus, Propertius, Ovid's *Amores*), of epigram (Martial) and of Ovid's poems from exile. A sentence usually closes at the end of the couplet, which makes the couplet a natural unit for a reader (Bennett 1918 §369).

## 5. Lyric and dramatic metres

- **Lyric** (Catullus, Horace's *Odes*): stanzas taken from Greek models, the Sapphic (three lines of – ⏑ – × – ⏑ ⏑ – ⏑ – – and a short close) and the Alcaic above all.
- **Drama**: Plautus and Terence use the iambic senarius (six iambs, ⏑ –, with many resolutions) and the long lines of song; Seneca's tragedies use a stricter iambic trimeter for dialogue and lyric metres for the choruses.

The verse of the CIRCSE treebank is Seneca's (*Hercules Furens*, *Agamemnon*, *Oedipus*, *Phoenissae*) beside Tacitus' *Germania* in prose (Bennett 1918 §370).

## 6. The poetic language

What sets poetry apart from prose, and what a parser trained on prose must expect:
- **Free order and hyperbaton** much beyond prose: an adjective and its noun at the two ends of a line, two noun phrases interlaced ([chapter 9](09-simple-clause.md), §7). On our counts, Seneca's verse uses the six orders of subject, verb and object nearly equally ([chapter 9](09-simple-clause.md), §6).
- **Old and short forms:**
  - *-ēre* for *-ērunt* in the perfect (*dīxēre*);
  - *-um* for *-ōrum* in the genitive plural (*deum, virum*);
  - *-ī* for *-iī*;
  - syncopated perfects (*amāsse* for *amāvisse*);
  - *ollī* for *illī*.
- **Greek forms and constructions:**
  - Greek case endings in names (*Aenēān, Dīdūs*);
  - the accusative of respect (*nūda pedem* "bare as to the foot");
  - the infinitive of purpose.
- **Poetic plurals** for singulars (*ōra* "face", *silentia*), and the singular for the plural (*mīles* "the soldiers").
- **Words used by poets only:** *ēnsis* for *gladius* "sword", *aequor* "the sea", *amnis* "river".
- **The dative of direction** without a preposition (*it clāmor caelō* "the shout goes to the sky").

## 7. Accentual verse and rhyme

From late antiquity, as vowel length was lost in speech ([chapter 13](13-late-latin.md), §2), Christian hymns came to count syllables and place stresses instead of quantities. Ambrose's hymns still scan by quantity and stress alike. The medieval sequences and songs use stress and rhyme alone:

> *Diēs īrae, diēs illa, / solvet saeclum in favīllā* (13th century) — eight syllables a line, stress on the odd syllables, rhyme.

The *Carmina Burana* and the goliardic songs, the hymns of the breviary and the liturgical sequences are in this verse ([chapter 14](14-medieval-latin.md)).

## 8. How Mova handles it

**In the UD trees.**
- Verse is annotated like prose, line breaks aside. The sentence is the unit, not the line.
- Elided words are written in full.
- Old forms carry their usual `FEATS` (*dīxēre* is `Number=Plur|Person=3|Tense=Past`).

**In Mova.** The CIRCSE treebank (Seneca's verse) is the measure of how Mova's Latin parser does on poetry, as LLCT is on the documents. The CIRCSE score is still to be measured. A Latin reader needs:
- the old and short forms of §6 in its morphology;
- the hyperbaton read through agreement (gender, number, case), not adjacency: the case of the noun, again, is what Mova's Latin side lacks first ([chapter 9](09-simple-clause.md), §9);
- the metre as a check. A scansion of a line confirms or refutes the quantities and so the case of an ambiguous *-a* (nominative short *-a* against ablative long *-ā*): a deterministic test of the kind Mova likes, and one that needs no model.
