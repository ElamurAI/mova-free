# Verse and metre

Greek verse, like Latin, is built on the length of syllables. The Latin metres were taken from it ([la chapter 12](../la/12-verse-style.md)). Much of what survives of early Greek is verse:
- Homer and Hesiod;
- the lyric poets;
- the tragedies and comedies.

Verse is also the largest part of the classical treebank (Perseus: Homer, Hesiod, Sophocles, Aeschylus). This chapter gives the quantity of syllables, the dactylic hexameter of epic, the iambic trimeter of drama, the lyric metres in brief, and what the metre does to the language a parser meets.

## Contents

1. Quantity
2. The dactylic hexameter
3. The elegiac couplet
4. The iambic trimeter
5. Lyric and choral metres
6. What metre does to the language
7. From quantity to stress
8. How Mova handles it
9. Sources

---

## 1. Quantity

A syllable is long by nature (a long vowel η, ω, a long α, ι, υ, or a diphthong) or by position (a short vowel before two consonants or a double consonant ζ, ξ, ψ), otherwise short (smyth-1920 §§144–148). A short vowel before a stop with λ, ρ, μ or ν may count either way. The script shows η and ω as long and ε and ο as short, but α, ι, υ may be either: the dictionaries mark them ([chapter 1](01-script-sounds.md)). A long vowel or diphthong at the end of a word may be shortened before a word beginning with a vowel (epic correption), and a final short vowel is elided before one: δ᾽ ἐγώ for δὲ ἐγώ.

## 2. The dactylic hexameter

The metre of Homer, Hesiod and all later epic: six feet, the first five dactyls (– ⏑ ⏑) or spondees (– –), the fifth almost always a dactyl, the sixth – ×:

> μῆνιν ἄειδε θεὰ Πηληϊάδεω Ἀχιλῆος (Il. 1.1)
> – ⏑ ⏑ | – ⏑ ⏑ | – ‖ ⏑ ⏑ | – – | – ⏑ ⏑ | – –
> "Sing, goddess, the wrath of Peleus' son Achilles"

The line has a caesura in the third foot. Its rhythm made the formulae of oral poetry: noun and epithet in fixed shapes that fill the end of a line (πόδας ὠκὺς Ἀχιλλεύς, γλαυκῶπις Ἀθήνη, ῥοδοδάκτυλος Ἠώς), chosen by metrical shape, not by sense.

## 3. The elegiac couplet

A hexameter and a pentameter (– ⏑ ⏑ – ⏑ ⏑ – ‖ – ⏑ ⏑ – ⏑ ⏑ –): the metre of elegy, epigram and epitaph, from Archilochus and Solon to the Greek Anthology. The couplet is a unit of sense, as in Latin.

## 4. The iambic trimeter

The metre of the spoken parts of tragedy and comedy: three pairs of iambs (× – ⏑ –), with resolutions of a long syllable into two shorts, which comedy allows freely and tragedy sparingly. It is the metre closest to speech: Aristotle says that iambic is what people fall into in conversation (*Poetics* 1449a).

## 5. Lyric and choral metres

- **Lyric** (Sappho, Alcaeus, Anacreon): short stanzas in fixed patterns, the Sapphic and the Alcaic, later copied by Horace.
- **Choral** (Pindar, Bacchylides, the choruses of tragedy): long strophes answered by antistrophes of the same shape, in complex metres of dactyls, iambs, choriambs and others.

The choral language is a literary Doric, whatever the poet's dialect ([chapter 16](16-dialects.md)).

## 6. What metre does to the language

What sets verse apart, and what a parser trained on prose must expect:
- **Homeric and poetic forms**, chosen by their metrical shape:
  - the genitive -οιο for -ου;
  - the dative plural -οισι, -ῃσι;
  - the infinitive -μεν, -μεναι;
  - the absent augment;
  - the uncontracted vowels ([chapter 16](16-dialects.md));
- **a free order** with hyperbaton, often across a line, and the object first;
- **tmesis**: a prefix separated from its verb (κατὰ … ἔδουσιν for κατέδουσιν);
- **the article as a demonstrative** in Homer ([chapter 5](05-article.md), §2);
- **poetic vocabulary**: words that prose does not use, and old words kept by the formulae.

## 7. From quantity to stress

In the Koine, between the third century BC and the third century AD, vowel length was lost in speech and the pitch accent became a stress accent ([chapter 2](02-accent.md)). Byzantine verse, Christian hymns (Romanos the Melode, 6th century) and the "political verse" of fifteen syllables count syllables and stress, not quantity: the same change as in Latin ([la chapter 12](../la/12-verse-style.md), §7).

## 8. How Mova handles it

**In the UD trees.** The Perseus treebank is mostly verse: Homer, Hesiod, Sophocles, Aeschylus. Verse is annotated by the sentence, not the line. Elided forms keep the apostrophe (δ᾽). Homeric forms have their usual lemma (θεοῖο → θεός).

**In Mova.** Mova has not measured its Greek parser on verse; Perseus is NC, and the open alternative is still to be found (the Scientist's queue). A Greek reader needs:
- the Homeric endings of §6 in its morphology, as alternatives to the Attic ones;
- elision restored before lookup (δ᾽ → δέ, ἀλλ᾽ → ἀλλά);
- tmesis rejoined: the prefix found and attached to its verb.

**Scansion as a check.** As for Latin ([la chapter 12](../la/12-verse-style.md), §8), the scansion of a line confirms or refutes the quantities. It can settle an ambiguous α, ι, υ: a nominative in short -α against a genitive in long -ᾱς. This is a deterministic test that needs no model.

## 9. Sources

- Smyth, H. W. 1920 (smyth-1920): §§144–148 (quantity), paraphrased from the Perseus text (CC BY-SA). The metres are the standard account; Smyth's appendix on versification is not in our text of it.
- Aristotle, *Poetics* 1449a (public domain).
- Homer, *Iliad* 1.1 (public domain).
