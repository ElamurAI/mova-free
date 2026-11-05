# 13. Late and Christian Latin; the Vulgate

From the third century on, written Latin splits into registers that classical grammar keeps apart:
- the school language of the rhetoricians, which goes on imitating Cicero;
- the language of the Church, built on translations of the Greek and Hebrew Scriptures;
- the spoken Latin of the provinces, which the documents show only where their writers' education fails, and which becomes the Romance languages.

This chapter describes the Latin of the Christian authors and of Jerome's Vulgate (late fourth century), the Latin Mova reads beside the Hebrew and the Greek, and the changes that lead from it to the medieval language ([chapter 14](14-medieval-latin.md)) and to French ([fr README](../fr/README.md)).

## Contents

1. Sources and registers
2. Spelling and sounds
3. Nouns and cases
4. The verb: periphrases and new forms
5. Syntax: parataxis, *quod/quia*, word order
6. Hebraisms and Grecisms of the Bible
7. The Christian vocabulary
8. How Mova handles it

---

## 1. Sources and registers

**The Bible.** The first Latin translations of the Greek Old Testament (the Septuagint) and of the New Testament, the *Vetus Latina*, were made in Africa and Italy in the second and third centuries, word for word and in a popular style. Jerome revised the Gospels (383) and translated most of the Old Testament from the Hebrew (390–405). His translation, with the older versions he did not replace (most of the deuterocanonical books, the Psalter *iuxta Septuaginta*), became the **Vulgate**. Mova reads it in the Clementine edition of 1592 (public domain), the text the Catholic Church used until the twentieth century.

**The Fathers.** Tertullian, Cyprian, Ambrose, Augustine, Jerome himself and Gregory the Great write a Latin that is classical in its grammar and Christian in its vocabulary, and lower in register when they preach.

**The documents.** Inscriptions, letters, legal and practical texts show the spoken language: the *Peregrinatio Egeriae* (an account of a pilgrimage, late fourth century), the *Appendix Probi* (a list of words *not* to say: *speculum non speclum*, *auris non oricla*), the medical and veterinary manuals.

## 2. Spelling and sounds

The sound changes that make the Romance languages are visible in the spellings of the period ([chapter 1](01-sounds-spelling.md)):
- **Vowel length is lost.** Quantity gives way to quality: short *i* and long *ē* merge as a close *e*, short *u* and long *ō* as a close *o*. Verse becomes accentual, not quantitative.
- **Diphthongs become vowels:** *ae* is written *e* (*celum* for *caelum*), *oe* becomes *e* (*pena*). The Clementine Vulgate prints *æ* and *œ* (*cælum, cœlum*) to mark the old diphthongs.
- **Final *-m* is lost**, and final *-s* weakens in Italy, which is why the accusative and the ablative merge (*rosam*, *rosā* → *rosa*).
- **Consonants change:**
  - *b* and consonantal *v* merge (*bixit* for *vixit*);
  - *c* and *g* before *e, i* palatalize (Italian *cento*, French *cent*);
  - *ti, ci* before a vowel merge (*tertius / tercius*);
  - *h* is silent.
- **Syncope:** unstressed vowels drop: *oculus → oclus → occhio, œil*; *calidus → caldus → chaud*.

## 3. Nouns and cases

- **The cases shrink.** In speech the nominative, the accusative and the ablative fall together after the loss of final *-m* and of vowel length. Old French keeps two cases (subject and object), Romanian three, Italian, Spanish and modern French none. The genitive and dative are replaced by prepositions: *dē* + noun for the genitive, *ad* + noun for the dative ([chapter 8](08-prepositions-particles.md), §10).
- **The neuter disappears** into the masculine. The neuter plural in *-a* becomes a feminine singular collective: *folia* "leaves" → French *la feuille*, Italian *la foglia*.
- **Articles are born.** The demonstratives *ille* and *ipse* weaken into definite articles (*ille* → French *le, la*, Italian *il*), and *ūnus* into the indefinite. In the Vulgate they are still demonstratives, though often weak: *in illo tempore* "at that time".

## 4. The verb: periphrases and new forms

- **The perfect with *habēre*:** *habeō litterās scrīptās* "I have letters written" → "I have written letters" → French *j'ai écrit*.
- **The future with *habēre*:** *cantāre habeō* "I have to sing" → *chanterai*. The conditional with *habēbam* → *chanterais* ([chapter 10](10-subordination.md), §8).
- **The passive with *esse*:** *amātus est* "he is loved" (present), where the classical form meant "he has been loved". The synthetic passive (*amātur*) disappears.
- **Deponent verbs** become active (*mīrāre* for *mīrārī*).
- **The infinitive with *dē* or *ad*** replaces the supine and many subjunctive clauses (French *de* + infinitive).

## 5. Syntax: parataxis, *quod/quia*, word order

**Parataxis.** Biblical narrative strings its sentences with *et*:

> **Et** vidit Deus lucem quod esset bona: **et** divisit lucem a tenebris. Appellavit**que** lucem Diem, **et** tenebras Noctem: factum**que** est vespere **et** mane, dies unus. (Vulgate, Gen. 1:4–5)

On our data (UD Latin PROIEL training set, NC, counted only, 2026-10-22), *et* is 75 words in a thousand in the Vulgate New Testament and 21 in a thousand in Caesar's *Gallic War*.

**Reported speech with *quod, quia, quoniam*** instead of the accusative and infinitive: in the Vulgate the *quod/quia* clause outnumbers the AcI (340 to 241 in the same treebank), where Caesar has 11 to 295 ([chapter 10](10-subordination.md), §11).

**The verb moves forward.** The Vulgate New Testament is SVO in 44% of its main clauses with a subject and an object, Caesar SOV in 53% ([chapter 9](09-simple-clause.md), §6).

## 6. Hebraisms and Grecisms of the Bible

The Latin Bible follows its sources word for word in many places, and its idiom became the idiom of Christian Latin and, through it, of the European languages:
- ***Factum est*** "it came to pass" opening a narrative, from the Hebrew *wayehî* through the Greek *kai egeneto*: *Factum est autem in diebus illis …*;
- **the genitive of quality** for an adjective: *mons sanctitatis* "the holy mountain", *filius perditionis* "the son of perdition", from the Hebrew construct state;
- ***Ecce*** "behold" to introduce a new event (191 times in the Vulgate New Testament of PROIEL);
- **the infinitive absolute** rendered by a noun or participle of the same root: *morte morieris* "you shall surely die" (Gen. 2:17), *benedicens benedicam* (Gen. 22:17);
- **comparative and superlative by repetition** and the genitive: *canticum canticorum* "the Song of Songs", *vanitas vanitatum* "vanity of vanities";
- ***in*** + ablative for the instrument (*in gladio* "with the sword"), from the Hebrew *bə-*;
- **Greek constructions** in the New Testament: the genitive absolute rendered as an ablative absolute, the articular infinitive as *in* + gerund. The same Hebraisms in the Greek of the Septuagint are in [grc chapter 17](../grc/17-koine.md), §5.

Mova sees these through its parallel Bibles: the Hebrew, the Greek and the Latin of the same verse are aligned by verse, and the meta-world of `translate` links them ([Holy Wars](https://mova.elamur.ai/free/en/holy-wars/)).

## 7. The Christian vocabulary

- **Greek loans:** *ecclēsia* (church), *episcopus* (bishop), *presbyter* (priest), *diaconus*, *baptizāre*, *angelus*, *apostolus*, *evangelium*, *martyr*, *eucharistia*.
- **Hebrew words,** through the Greek: *amen, alleluia, sabbatum, pascha, satanas, cherubim, seraphim, hosanna*.
- **Old Latin words with new senses:**
  - *fides* faith; *gratia* grace; *salus* salvation;
  - *caritas* (Greek *agapē*) love; *saeculum* the world (as against God); *gentes* the heathen;
  - *paganus* "villager" → heathen; *confessio* confession of faith; *sacramentum* (oath → sacrament); *verbum* the Word (Greek *logos*).
- **New words:** *salvator, sanctificare, glorificare, carnalis, spiritualis, incarnatio, trinitas*.

## 8. How Mova handles it

**The texts.** Mova holds:
- the Clementine Vulgate (eBible `latVUC`, public domain, with the *Glossa ordinaria* in its notes) and the CrossWire Clementine;
- the open UD treebanks LLCT and CIRCSE (CC BY-SA);
- the PROIEL treebank, whose Latin part is mostly the Vulgate New Testament. It is CC BY-NC-SA, so it is used for tests and counts only, never quoted.

**In Mova.** Latin is parsed by Mova's English parser through `en xpos-fill`. LLCT dev LAS is 87.35 (2026-10-22); the Vulgate itself is not measured, since its open treebank is NC. Its closeness to the Romance word order (§5) makes it easier for an order-driven parser than Caesar or poetry. In the many-translations meta-world ([Holy Wars](https://mova.elamur.ai/free/en/holy-wars/)) the Vulgate is the next text to plug in after the English. Two things are needed first:
- **the versification:** the Vulgate's Psalms and some books follow the Septuagint's numbering. The STEPBible mapping tables in the library give the conversion;
- **a Latin reader into the meta-world:** lemmas from a Latin morphology (Mova has none yet; the open candidates are LatinCy's lemmatizer, Lemlat and Whitaker's Words, with their licences in [sources.md](sources.md)).
