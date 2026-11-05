# Prepositions, prefixes, conjunctions and particles

This chapter covers the small words of Modern Hebrew: the prepositions and how they take pronoun suffixes (*li* 'to me', *alav* 'on him'); the one-letter prefixes ו ה ש ב כ ל מ, which are written as part of the next word, their order and their vowels; the accusative marker את and its inflected forms (*oti*, *otkha* …); compound prepositions; the conjunctions; and the adverbs and particles that carry focus and attitude (*gam*, *rak*, *kvar*, *dávka*, *bikhlal*, *pashut*, *mamash*, *harey*). The discourse particles of speech (*kaze*, *yaani*, *áha*, *nu*, *yála*) are only listed here and treated fully in [09-pragmatics-politeness.md](09-pragmatics-politeness.md). Nouns, pronouns and possessive suffixes are in [04-nouns-adjectives-pronouns.md](04-nouns-adjectives-pronouns.md); clause syntax (object marking, relatives, questions, negation) in [07-syntax.md](07-syntax.md); how UD splits these words into tokens in [11-ud-analysis.md](11-ud-analysis.md). Index: [README.md](README.md).

Transliteration and glossing follow [02-script-and-spelling.md](02-script-and-spelling.md): *kh* = /χ/, *ts* = צ, *sh* = ש, stress marked only when not final (*lánu*, *otánu*), prefixes joined by a hyphen (*be-bayit*, *ba-báyit*). Leipzig abbreviations: ACC, DEF, GEN, CS, 1SG, 3PL.M etc.

---

## 1. Prepositions take pronoun suffixes

Hebrew has no free object pronouns after prepositions. Where English says *to him*, *with me*, *about them*, Hebrew fuses the preposition and the pronoun into one word: *lo*, *iti*, *aleyhem*. These "inflected prepositions" are among the most frequent words of the language (Wikipedia, *Modern Hebrew grammar*, "Prepositions"; Glinert 2017).

```
נתתי לו את הספר.
natáti lo et ha-séfer
give.PST.1SG to.3SG.M ACC DEF-book
'I gave him the book.'
```

```
דיברנו עליהם כל הערב.
dibárnu aleyhem kol ha-érev
speak.PST.1PL about.3PL.M all DEF-evening
'We talked about them all evening.'
```

Prepositions fall into two inflection series, by the shape of their suffixes. The **singular-type** series uses the suffixes of a singular noun (*-i*, *-kha*, *-o* …; compare *sifri* 'my book'). The **plural-type** series uses the suffixes of a plural noun (*-ay*, *-ékha*, *-av* …; compare *sfaray* 'my books'), because these prepositions were historically plural nouns or took a *-y-* linking element. The series is a lexical property of each preposition and must be listed.

In the full spelling (*ktiv male*) the two series are often told apart only by a yod: עליו *alav* 'on him' has a yod before the *-v*, while לו *lo* 'to him' does not.

---

## 2. Singular-type prepositions

### 2.1 ל- 'to, for' and ב- 'in, at, with'

The two most frequent prepositions are single-letter prefixes before nouns but full words with suffixes.

| Person | ל- 'to' | | ב- 'in' | |
|---|---|---|---|---|
| 1SG | לי | *li* | בי | *bi* |
| 2SG.M | לך | *lekha* | בך | *bekha* |
| 2SG.F | לך | *lakh* | בך | *bakh* |
| 3SG.M | לו | *lo* | בו | *bo* |
| 3SG.F | לה | *la* | בה | *ba* |
| 1PL | לנו | *lánu* | בנו | *bánu* |
| 2PL.M | לכם | *lakhem* | בכם | *bakhem* |
| 2PL.F | לכן | *lakhen* | בכן | *bakhen* |
| 3PL.M | להם | *lahem* | בהם | *bahem* |
| 3PL.F | להן | *lahen* | בהן | *bahen* |

Notes:

- לו *lo* 'to him' and לא *lo* 'not' sound the same and are a classic spelling error (*lo* 'not' is written with aleph).
- לך is *lekha* (M) or *lakh* (F); the spelling does not tell (pointing does: לְךָ / לָךְ). The same holds for בך and every 2SG suffix (-ך).
- In speech, *bahem* is often levelled for feminine reference, as with all feminine plural forms (see [04-nouns-adjectives-pronouns.md](04-nouns-adjectives-pronouns.md), §8.1).

The dative ל- marks the recipient (*natáti lo*), the experiencer of states (*kar li* 'I'm cold', lit. 'cold to me'; *nim'as li* 'I'm fed up'), the possessor in existential sentences (*yesh li* 'I have', lit. 'there is to me'), and the "ethical dative" of interest (*hu halakh lo* 'off he went', lit. 'he went to him'). See [07-syntax.md](07-syntax.md) on *yesh*/*eyn* and possession.

```
קר לי.
kar li
cold.M.SG to.1SG
'I'm cold.'
```

### 2.2 של 'of' (possession)

| Person | Form | | Person | Form |
|---|---|---|---|---|
| 1SG | שלי *sheli* | | 1PL | שלנו *shelánu* |
| 2SG.M | שלך *shelkha* | | 2PL.M | שלכם *shelakhem* |
| 2SG.F | שלך *shelakh* | | 2PL.F | שלכן *shelakhen* |
| 3SG.M | שלו *shelo* | | 3PL.M | שלהם *shelahem* |
| 3SG.F | שלה *shela* | | 3PL.F | שלהן *shelahen* |

של is historically ש- 'that' + ל- 'to', and its inflection is that of ל- with *she-* in front. It is the everyday possessive: *ha-báyit sheli* 'my house'. It also predicates possession: *ha-séfer ha-ze shelkha?* 'is this book yours?'. The bound possessive suffixes and the double genitive are described in [04-nouns-adjectives-pronouns.md](04-nouns-adjectives-pronouns.md), §4.6 and §6.

Ambiguity: the written form שלו is usually *shelo* 'his', but can also be *she-lu* 'that if only' or *she-lo* 'that not' (written שלא, so only in errors) and *she-lo* 'that to him'. The IAHLT treebanks (UD 2.18) contain all three analyses: של + ו (*shelo*, 85 times as `nmod:poss` or `nmod`), ש + לו (*she-lu*, as two `mark`), and ש + ל + ו (*she-lo* 'that to him'); counted 2026-10-17.

### 2.3 את: the accusative marker

The object marker את *et* comes before a definite direct object (see [07-syntax.md](07-syntax.md) and [04-nouns-adjectives-pronouns.md](04-nouns-adjectives-pronouns.md), §5.3). With a pronoun it changes its stem to *ot-*:

| Person | Form | | Person | Form |
|---|---|---|---|---|
| 1SG | אותי *oti* | | 1PL | אותנו *otánu* |
| 2SG.M | אותך *otkha* | | 2PL.M | אתכם *etkhem*, coll. אותכם *otkhem* |
| 2SG.F | אותך *otakh* | | 2PL.F | אתכן *etkhen*, coll. אותכן *otkhen* |
| 3SG.M | אותו *oto* | | 3PL.M | אותם *otam* |
| 3SG.F | אותה *ota* | | 3PL.F | אותן *otan* |

```
ראיתי אותה אתמול.
ra'íti ota etmol
see.PST.1SG ACC.3SG.F yesterday
'I saw her yesterday.'
```

```
אני אוהב אתכם.
ani ohev etkhem
I love.PRS.M.SG ACC.2PL.M
'I love you (all).'
```

Notes:

- The 2PL *etkhem*/*etkhen* is the normative form; *otkhem* is common in speech and is accepted by many.
- **אותו *oto* also means 'the same'**: *oto davar* 'the same thing', *ota ha-isha* 'the same woman', *otam ha-anashim* 'the same people'. Here it is a determiner before a noun, not an object pronoun. In UD this use is `DET` (`PronType=Dem`), while the object *oto* is a multiword token ADP + PRON (§9).
- The suffixed forms of את are full pronouns: they can be stressed, coordinated and fronted (*oti hu lo ra'a* 'me he didn't see').
- Classical Hebrew also had את *et* 'with' (*iti* 'with me' comes from it; §2.4). Modern Hebrew uses את as 'with' only in fixed Biblical phrases.
- **Homographs.** Unpointed את is also *at* 'you (F.SG)'. A tagger must decide from the next word: *et* is followed by a definite noun phrase, *at* by a predicate or verb.

### 2.4 עם 'with' → איתי

עם *im* 'with' takes suffixes on a different stem, *it-* (historically from את 'with'):

| Person | Form | Formal | | Person | Form | Formal |
|---|---|---|---|---|---|---|
| 1SG | איתי *iti* | עמי *imi* | | 1PL | איתנו *itánu* | עמנו *imánu* |
| 2SG.M | איתך *itkha* | עמך *imkha* | | 2PL.M | איתכם *itkhem* | עמכם *imakhem* |
| 2SG.F | איתך *itakh* | עמך *imakh* | | 2PL.F | איתכן *itkhen* | עמכן *imakhen* |
| 3SG.M | איתו *ito* | עמו *imo* | | 3PL.M | איתם *itam* | עמם *imam* |
| 3SG.F | איתה *ita* | עמה *ima* | | 3PL.F | איתן *itan* | עמן *iman* |

The forms with *it-* are the everyday ones; the forms on *im-* are literary and liturgical (*ha-shem imkha* 'the Lord is with you'). *Iti* is spelled with or without yod (איתי / אתי); the full spelling with yod is the Academy's rule for unpointed text (Academy 2017).

```
אתה בא איתנו?
ata ba itánu?
you come.PRS.M.SG with.1PL
'Are you coming with us?'
```

### 2.5 מן / מ- 'from'

מן *min* (before the article, and in fixed phrases) and the prefix מ- *mi-*/*me-* 'from, than' have irregular suffixed forms with a doubled stem:

| Person | Form | | Person | Form |
|---|---|---|---|---|
| 1SG | ממני *miméni* | | 1PL | מאיתנו *me-itánu* (also ממנו *miménu*) |
| 2SG.M | ממך *mimkha* | | 2PL.M | מכם *mikem* |
| 2SG.F | ממך *mimekh* | | 2PL.F | מכן *mikhen* |
| 3SG.M | ממנו *miménu* | | 3PL.M | מהם *mehem* |
| 3SG.F | ממנה *miména* | | 3PL.F | מהן *mehen* |

ממנו *miménu* is both 'from him' and 'from us'; to avoid the ambiguity modern speakers say *me-itánu* 'from (with) us' or *mi-ménu* only for 'him'.

מ- also means 'than' in comparisons: *hu gavóha miméni* 'he is taller than me'. With the article it does not fuse: *me-ha-báyit* 'from the house' (contrast *ba-báyit*, §5.3).

### 2.6 כמו 'like, as'

| Person | Form | | Person | Form |
|---|---|---|---|---|
| 1SG | כמוני *kamóni* | | 1PL | כמונו *kamónu* |
| 2SG.M | כמוך *kamókha* | | 2PL.M | כמוכם *kmokhem* |
| 2SG.F | כמוך *kamokh* | | 2PL.F | כמוכן *kmokhen* |
| 3SG.M | כמוהו *kamóhu* | | 3PL.M | כמוהם *kmohem* |
| 3SG.F | כמוה *kamóha* | | 3PL.F | כמוהן *kmohen* |

The prefix כ- *ke-* 'as, like' takes no suffixes in Modern Hebrew; כמו *kmo* replaces it with pronouns. Colloquially *kmo* also combines with a free pronoun (*kmo hu* 'like him') or with *she-*: *kmo she-amárti* 'as I said'. *Kamókha* survives in the fixed *ve-ahavta le-re'akha kamókha* 'love your neighbour as yourself' and *kamókha?* 'someone like you'. See also the slang *kaze* 'like this, kind of' in §7.4.

### 2.7 Other singular-type prepositions

| Preposition | Meaning | 1SG | 3SG.M | 3PL.M | Notes |
|---|---|---|---|---|---|
| אצל *étsel* | at (someone's place), with | אצלי *etsli* | אצלו *etslo* | אצלם *etslam* | *etsli ba-báyit* 'at my place' |
| בשביל *bishvil* | for | בשבילי *bishvili* | בשבילו *bishvilo* | בשבילם *bishvilam* | everyday 'for' |
| בגלל *biglal* | because of | בגללי *biglali* | בגללו *biglalo* | בגללם *biglalam* | |
| ליד *leyad* | near, next to | לידי *leyadi* | לידו *leyado* | לידם *leyadam* | |
| נגד *néged* | against | נגדי *negdi* | נגדו *negdo* | נגדם *negdam* | |
| בעד *be'ad* | for, in favour of | בעדי *ba'adi* | בעדו *ba'ado* | בעדם *ba'adam* | |
| מול *mul* | opposite | מולי *muli* | מולו *mulo* | מולם *mulam* | |
| לבד *levad* | alone | לבדי *levadi* | לבדו *levado* | לבדם *levadam* | adverb-like; speech: invariant *levad* |
| עצם *étsem* | self | עצמי *atsmi* | עצמו *atsmo* | עצמם *atsmam* | reflexive (noun) |
| בלעדי *bil'adey* | without | בלעדיי *bil'aday* | בלעדיו *bil'adav* | בלעדיהם | **plural-type**; free form בלי *bli* takes no suffix |
| כלפי *klapey* | towards (attitude) | כלפיי *klapay* | כלפיו *klapav* | כלפיהם | **plural-type** |
| לגבי *legabey* | concerning | לגביי *legabay* | לגביו *legabav* | לגביהם | **plural-type** |
| אודות *odot* | about | אודותיי *odotay* | אודותיו *odotav* | אודותיהם | formal, **plural-type** |

The full 2nd person and 1PL forms follow the pattern of ל- above: *etslekha*, *etslekh*, *etslénu*, *etslekhem*, *etslekhen*; *bishvilkha*, *bishvilekh*, *bishvilénu*, *bishvilkhem*.

---

## 3. Plural-type prepositions

### 3.1 על 'on, about', אל 'to(wards)'

| Person | על 'on' | | אל 'to' | |
|---|---|---|---|---|
| 1SG | עליי | *alay* | אליי | *elay* |
| 2SG.M | עליך | *alékha* | אליך | *elékha* |
| 2SG.F | עלייך | *aláyikh* | אלייך | *eláyikh* |
| 3SG.M | עליו | *alav* | אליו | *elav* |
| 3SG.F | עליה | *aléha* | אליה | *eléha* |
| 1PL | עלינו | *alénu* | אלינו | *elénu* |
| 2PL.M | עליכם | *aleykhem* | אליכם | *aleykhem* |
| 2PL.F | עליכן | *aleykhen* | אליכן | *aleykhen* |
| 3PL.M | עליהם | *aleyhem* | אליהם | *aleyhem* |
| 3PL.F | עליהן | *aleyhen* | אליהן | *aleyhen* |

The 2PL and 3PL of אל are pronounced with *a-* (*aleykhem*) in normal speech, like those of על, so they merge in sound; in pointed text they differ (אֲלֵיכֶם / עֲלֵיכֶם). The greeting *shalom aleykhem* is על.

על is the general 'on' and 'about' (*sipur al ahava* 'a story about love'), marks the object of many verbs (*hishpía al* 'influenced', *shamar al* 'guarded', *vitér al* 'gave up'), and forms a host of idioms (*al yad* 'next to', *al-pi* 'according to', *al menat* 'in order to'). אל 'to, towards' is mostly formal; speech uses ל- (*halákhti el ha-rofe* (formal) / *la-rofe* 'I went to the doctor'), but *elay* 'to me' is common with verbs of motion and speaking: *hu pana elay* 'he turned to me'.

### 3.2 אחרי 'after', לפני 'before'

| Person | אחרי 'after' | | לפני 'before' | |
|---|---|---|---|---|
| 1SG | אחריי | *akharay* | לפניי | *lefanay* |
| 2SG.M | אחריך | *akharékha* | לפניך | *lefanékha* |
| 2SG.F | אחרייך | *akharáyikh* | לפנייך | *lefanáyikh* |
| 3SG.M | אחריו | *akharav* | לפניו | *lefanav* |
| 3SG.F | אחריה | *akharéha* | לפניה | *lefanéha* |
| 1PL | אחרינו | *akharénu* | לפנינו | *lefanénu* |
| 2PL.M | אחריכם | *akhareykhem* | לפניכם | *lifneykhem* |
| 2PL.F | אחריכן | *akhareykhen* | לפניכן | *lifneykhen* |
| 3PL.M | אחריהם | *akhareyhem* | לפניהם | *lifneyhem* |
| 3PL.F | אחריהן | *akhareyhen* | לפניהן | *lifneyhen* |

Both are also conjunctions with ש-: *akharey she-*, *lifney she-* 'after/before (doing)'. *Lifney* is temporal and spatial ('in front of', formal); speech uses *mi-lifney* or *mul* for 'in front of'. 'Ago' is *lifney*: *lifney shalosh shanim* 'three years ago'.

### 3.3 תחת 'under' and בין 'between'

| Person | תחת 'under' | | בין 'between' | |
|---|---|---|---|---|
| 1SG | תחתיי | *takhtay* | ביני | *beyni* |
| 2SG.M | תחתיך | *takhtékha* | בינך | *beynkha* |
| 2SG.F | תחתייך | *takhtáyikh* | בינך | *beynekh* |
| 3SG.M | תחתיו | *takhtav* | בינו | *beyno* |
| 3SG.F | תחתיה | *takhtéha* | בינה | *beyna* |
| 1PL | תחתינו | *takhténu* | בינינו | *beynéynu* |
| 2PL.M | תחתיכם | *takhteykhem* | ביניכם | *beyneykhem* |
| 2PL.F | תחתיכן | *takhteykhen* | ביניכן | *beyneykhen* |
| 3PL.M | תחתיהם | *takhteyhem* | ביניהם / בינם | *beyneyhem* / *beynam* |
| 3PL.F | תחתיהן | *takhteyhen* | ביניהן | *beyneyhen* |

בין is mixed: singular-type in the singular, plural-type in the plural (the forms above match the Wiktionary table in kaikki.org's Hebrew dictionary). 'Between X and Y' is *beyn X le-Y* or *beyn X u-veyn Y* (formal, with the preposition repeated): *beyni le-veynekha* 'between me and you'. *Beynéynu* is also 'between us' = 'frankly' in speech.

תחת is formal; speech uses מתחת ל- *mitákhat le-* 'below', which takes the suffixes of ל-: *mitákhat lo* 'under him'. תחת also means 'instead of' in formal style.

### 3.4 Other plural-type prepositions

| Preposition | Meaning | 1SG | 3SG.M | 3PL.M |
|---|---|---|---|---|
| סביב *saviv* | around | סביבי *svivi* | סביבו *svivo* | סביבם *svivam* / סביביהם *sviveyhem* (forms vary) |
| עד *ad* | until, up to | עדיי *aday* | עדיו *adav* | עדיהם *adeyhem* (rare; formal) |
| מעל *me'al* | above | מעליי *me'alay* | מעליו *me'alav* | מעליהם |
| אחר (lit.) | after | (see אחרי) | | |
| כלפי, לגבי, בלעדי, אודות | towards, concerning, without, about | see §2.7 | | |

---

## 4. The inseparable prefixes ו ה ש ב כ ל מ

### 4.1 The seven letters

Seven one-letter function words are written together with the next word, with no space and no hyphen. Hebrew grammar teaches them with the mnemonic *moshe ve-khalev* (משה וכלב, 'Moses and Caleb'), whose letters are מ ש ה ו כ ל ב:

| Prefix | Normal reading | Meaning | Category (UD) |
|---|---|---|---|
| ו | *ve-*, *u-* | and | CCONJ, `cc` |
| ה | *ha-* (*he-*) | the; also question particle (lit.) and relative (lit.) | DET, `det` |
| ש | *she-* | that, who, which | SCONJ, `mark` |
| ב | *be-* (*ba-*, *bi-*) | in, at, with (instrument) | ADP, `case` |
| כ | *ke-* (*ka-*, *ki-*) | as, like; about (approximation) | ADP, `case` |
| ל | *le-* (*la-*, *li-*) | to, for | ADP, `case` |
| מ | *mi-*, *me-* | from, than | ADP, `case` |

Also prefixed in writing, as a combination of these: כש- *kshe-* 'when' (כ + ש), מש- *mi-she-* 'from (the time) that', לכש- *likhshe-* 'when (in future)' (formal), and the Aramaic-origin conjunction ד- in a few fixed words.

### 4.2 Order

When several prefixes stack, their order is fixed (Glinert 2017; Tsarfaty 2006 uses this as a constraint on segmentation):

**ו → ש / כש → ב / כ / ל / מ → ה → stem**

| Written | Segmented | Reading | Meaning |
|---|---|---|---|
| והבית | ו + ה + בית | *ve-ha-báyit* | and the house |
| שבבית | ש + ב + (ה) + בית | *she-ba-báyit* | that (is) in the house |
| וכשהגעתי | ו + כש + הגעתי | *u-khshe-higáti* | and when I arrived |
| ומהבית | ו + מ + ה + בית | *u-me-ha-báyit* | and from the house |
| שלבית | ש + ל + בית | *she-le-báyit* / *she-la-báyit* | that to a/the house |
| וכשבבית | ו + כש + ב + בית | *u-khshe-ba-báyit* | and when in the house |

Up to four or five prefix letters can stack in formal writing (וכשמהבית is possible but strained). Each one adds ambiguity: a word starting with ו, ה, ש, ב, כ, ל or מ may be a prefix + a word, or a word whose first letter is that letter. A classic example is the word-form בצל, which is *batsal* 'onion' (one word) or *be-tsel* / *ba-tsel* 'in (the) shadow' (ב + צל). Tsarfaty et al. (2019) and Seker et al. (2022) call this the core problem of Hebrew processing: the input tokens are not the syntactic words, so segmentation and parsing must be done jointly or with a lattice (Goldberg and Tsarfaty 2008; More et al. 2019).

### 4.3 The vowel of ו 'and'

The conjunction ו is normally *ve-*. Normative rules, still followed in careful reading and the media but only partly in speech, change it:

| Context | Form | Example |
|---|---|---|
| default | *ve-* | *ve-yéled* 'and a boy' |
| before ב, מ, פ (labials) | *u-* | *u-bayit* 'and a house', *u-mekhonit*, *u-panim* |
| before a consonant with shva (no full vowel) | *u-* | *u-sfarim* 'and books', *u-tmunot* 'and pictures' |
| before the article | *ve-ha-* | *ve-ha-yéled* |
| before *y-* with shva (יְ) | *vi-* | *vi-ladim* וִילָדִים 'and children' (*ve-yeladim* in speech) |
| in pairs and fixed phrases | *va-* | *yom va-láyla* 'day and night', *shtáyim va-khétsi* 'two and a half' |

In everyday speech *ve-* is used almost everywhere, and *u-* survives mostly before *b-*, *m-*, *p-* and in set phrases (*u-ma itkha?*). The written form is always ו; pointing alone shows the vowel.

### 4.4 The vowel of ש, ב, כ, ל, מ

- **ש** is always *she-* (*she-ani*, *she-ha-*).
- **ב, כ, ל** are *be-*, *ke-*, *le-* before most words; *bi-*, *ki-*, *li-* before a consonant with shva (*bi-tmuna* 'in a picture', *li-yladim* > speech *le-yeladim*); before a guttural with a *khataf* vowel they copy that vowel, so that indefinite בעבודה is normatively *ba-avoda* 'in work' (בַּעֲבוֹדָה), sounding like the definite; and fused with the article as *ba-*, *ka-*, *la-* (§4.5).
- **מ** is *mi-* before most consonants, with the next consonant doubled in pointing (*mi-bayit* מִבַּיִת), and *me-* before gutturals and *r* (*me-ha-*, *me-ir* 'from a city', *me-rekhov*). In speech *mi-* and *me-* vary freely.

### 4.5 The article swallowed by ב, כ, ל

When ב, כ or ל comes before the article ה, the ה disappears from writing and speech, and its vowel *a* moves to the preposition:

| Without article | With article | Not |
|---|---|---|
| בבית *be-báyit* 'in a house' | בבית *ba-báyit* 'in the house' | \*בהבית |
| לבית *le-báyit* 'to a house' | לבית *la-báyit* 'to the house' (and 'home') | \*להבית |
| כמורה *ke-more* 'as a teacher' | כמורה *ka-more* 'as the teacher' | \*כהמורה |
| בעבודה *be-avoda* 'in a job' | בעבודה *ba-avoda* 'at the work' | |

In pointed text the vowel shows the article (בְּבַיִת / בַּבַּיִת, with the *dagesh* of the article in the next letter); in unpointed text the two are written exactly the same, and definiteness must be inferred. This hidden article is one of the main sources of ambiguity in Hebrew text processing; UD treebanks handle it by putting `Definite=Def|PronType=Art` on the preposition word itself rather than inventing an invisible ה token (§9).

מ does **not** absorb the article: מהבית *me-ha-báyit* 'from the house', with ה written and pronounced. The prefix ש does not absorb it either: שהבית *she-ha-báyit*.

In speech the *be-*/*ba-* contrast is audible. Where ה is part of a proper name (הפועל *ha-po'el*, a sports club) it is kept in writing after the prefix: בהפועל *be-ha-po'el*.

### 4.6 Prefixes on numbers, Latin script and acronyms

Prefixes attach also to digits, foreign words and abbreviations, often with a hyphen or a geresh-like mark: ב־2026 *be-alpáyim ve-esrim ve-shesh*, ל־Google, ב-WhatsApp, וצה"ל *ve-tsáhal*. The Academy's punctuation rules prescribe the maqaf (Hebrew hyphen ־) between a prefix and a digit or foreign-script word (Academy 2017; [02-script-and-spelling.md](02-script-and-spelling.md)).

---

## 5. Compound and complex prepositions

Many prepositions are built from a simple preposition and a noun (often in the construct), or from two prepositions:

| Preposition | Literal | Meaning | Example |
|---|---|---|---|
| על יד *al yad* | on the hand of | next to | *al yad ha-báyit* 'next to the house' |
| על ידי *al yedey* | on the hands of | by (agent of a passive) | *nikhtav al yedey dani* 'written by Dani' |
| לפי *lefi* | to the mouth of | according to | *lefi ha-khok* 'according to the law' |
| על פי *al pi* | on the mouth of | according to (formal) | |
| בתוך *betokh* | in the middle of | inside | *betokh ha-kufsa* 'inside the box' |
| מתוך *mitokh* | from the middle of | out of, from among | *mitokh ha-shloshim* 'out of the thirty' |
| בזכות *bizkhut* | in the merit of | thanks to | |
| בעקבות *be-ikvot* | in the footsteps of | following, as a result of | |
| במשך *be-méshekh* | in the extent of | during, for (time) | *be-méshekh shana* 'for a year' |
| במהלך *be-mahalakh* | in the course of | during | |
| לקראת *likrat* | towards meeting | towards, ahead of | *likrat ha-bkhirot* 'ahead of the elections' |
| מאחורי *me-akhorey* | from behind | behind | |
| מתחת ל- *mitákhat le-* | from under to | under | |
| מעל ל- *me'al le-* | from on to | above | |
| ליד *leyad* | to the hand | next to | |
| בלי / ללא *bli* / *lelo* | without | without | *bli kesef* 'without money'; *lelo* formal |
| חוץ מ- *khuts mi-* | outside from | except | *khuts miméni* 'except me' |
| בנוסף ל- *be-nosaf le-* | in addition to | in addition to | |
| כדי *kdey* | in the amount of | in order to (+ INF) | *kdey lilmod* 'in order to learn' |
| בגלל *biglal* | in the rolling of | because of | |
| בשביל *bishvil* | in the path of | for | |
| עד *ad* | until | until, up to | *ad mákhar* 'until tomorrow' |
| מאז *me'az* | from then | since | *me'az ha-milkhama* 'since the war' |
| בשל *bishel* | in that of | because of (formal) | |
| אצל *étsel* | beside | at (someone's) | |
| כלפי *klapey* | towards the face of | towards (attitude) | |

Compound prepositions written as two words (על ידי, על פי, בנוסף ל־) are a problem for tokenization: UD treats many of them as `fixed` multiword expressions (head word `case`, the rest `fixed`), as in the IAHLTknesset card's *כפי ש* 'as' (כפי + ש `fixed`). Suffixes go on the last noun: *al yadi* 'next to me', *al yadav* 'by him', *lefiv* 'according to him', *betokho* 'inside it', *bizkhuto* 'thanks to him'.

---

## 6. Conjunctions

### 6.1 Coordinating conjunctions

| Word | Meaning | Notes |
|---|---|---|
| ו- *ve-* | and | prefix; see §4.3 for its vowel |
| או *o* | or | *o ... o* 'either ... or' |
| אבל *aval* | but | the everyday 'but' |
| אך *akh* | but, yet | formal |
| אולם *ulam* | however | formal, clause-initial |
| אלא *éla* | but rather (after a negation) | *lo ani éla hu* 'not me but him' |
| גם ... וגם *gam ... ve-gam* | both ... and | |
| לא ... ולא *lo ... ve-lo* | neither ... nor | also *af lo* |
| כלומר *klomar* | that is | |
| לכן *lakhen* | therefore | |
| ולכן *ve-lakhen* | and so | |

```
לא הלכתי לעבודה אלא נשארתי בבית.
lo halákhti la-avoda éla nish'árti ba-báyit
NEG go.PST.1SG to.DEF-work but.rather stay.PST.1SG in.DEF-house
'I didn't go to work, but stayed home instead.'
```

*Aval* and *éla* differ like German *aber* and *sondern*: *éla* replaces a negated alternative, *aval* adds a contrast.

### 6.2 Subordinating conjunctions

The general subordinator is the prefix ש- *she-* 'that'; most other subordinators are a preposition or adverb + ש-.

| Word | Meaning | Example / note |
|---|---|---|
| ש- *she-* | that (complement and relative) | *amar she-hu ba* 'he said that he's coming'; *ha-ish she-ra'íti* 'the man (that) I saw' |
| כי *ki* | that (formal complement); because | *hodía ki* 'announced that' (news); *ki* 'because' in speech too |
| אשר *ashér* | that, which (formal relative) | *ha-khok ashér nikbá* 'the law which was set' |
| כש- *kshe-*, כאשר *ka'ashér* | when | *kshe-higáti* 'when I arrived'; *ka'ashér* formal |
| אם *im* | if (real condition); whether | *im yered géshem, nisha'er ba-báyit* 'if it rains, we'll stay home' |
| לו *lu*, אילו *ilu* | if (counterfactual) | *ilu yadáti* 'had I known' |
| אילולא *ilule* | if not for | formal |
| כי / מפני ש- / בגלל ש- / כיוון ש- | because | *biglal she-* is the everyday one; *mipney she-*, *keyvan she-* formal |
| למרות ש- *lamrot she-* | although | also *af al pi she-* (formal) |
| כדי ש- *kdey she-* | so that | + future tense |
| עד ש- *ad she-* | until | |
| לפני ש- / אחרי ש- | before / after | |
| מאז ש- *me'az she-* | since | |
| בזמן ש- *bi-zman she-* | while | also *be-od* 'while, whereas' |
| כמו ש- *kmo she-* | as | |
| כפי ש- *kfi she-* | as (formal) | |
| כאילו *ke'ilu* | as if; (slang) 'like' | |
| האם *ha-im* | whether | formal; speech uses *im* |

```
אם תבוא מוקדם, נלך ביחד.
im tavo mukdam, nelekh be-yákhad
if come.FUT.2SG.M early go.FUT.1PL together
'If you come early, we'll go together.'
```

```
אמרתי לו שאני לא יכול.
amárti lo she-ani lo yakhol
say.PST.1SG to.3SG.M that-I NEG can.M.SG
'I told him that I can't.'
```

The relative clause with ש- and its resumptive pronouns (*ha-ish she-dibárti ito* 'the man I spoke with', lit. 'the man that I spoke with him') is in [07-syntax.md](07-syntax.md).

---

## 7. Adverbs and focus particles

Hebrew has a set of short particles that mark focus, scope and the speaker's stance. They are frequent, often unstressed, and hard to translate; a frame reader must at least know their scope and polarity.

### 7.1 Focus and additive particles

| Word | Meaning | Notes and examples |
|---|---|---|
| גם *gam* | also, too; even | before the focus: *gam ani* 'me too'; *gam im* 'even if'; negative 'neither' = *gam ... lo*: *gam ani lo* 'me neither' |
| רק *rak* | only, just | *rak rega* 'just a moment'; *rak she-* 'except that' |
| אפילו *afílu* | even | *afílu lo* 'not even' |
| עוד *od* | still, more, another | *od lo* 'not yet'; *od kafe* 'another coffee'; *od pá'am* 'again'; *od* + participle 'still': *hu od yashen* 'he's still asleep' |
| כבר *kvar* | already | *kvar higáti* 'I've already arrived'; *kvar lo* 'no longer'; impatient *nu, kvar!* 'come on!' |
| עדיין *adáyin* | still, yet | *adáyin lo* 'not yet' |
| שוב *shuv* | again | also *od pá'am* |
| בדיוק *bediyuk* | exactly; just (now) | *bediyuk yatsáti* 'I was just leaving' |
| ממש *mamash* | really, literally, very | intensifier: *mamash yafe* 'really nice'; *mamash lo* 'not at all' |
| פשוט *pashut* | simply | *pashut lo ba li* 'I just don't feel like it' |
| בכלל *bikhlal* | at all; in general; actually | *lo bikhlal* 'not at all'; *bikhlal lo yadáti* 'I had no idea at all'; *ma bikhlal kara?* 'what actually happened?' |
| דווקא *dávka* | precisely, of all things; actually (contrary to expectation) | *dávka ha-yom?* 'today of all days?'; *dávka hayá nekhmad* 'it was actually nice'; *lo dávka* 'not necessarily'. A famously untranslatable word, borrowed into Yiddish-influenced English |
| הרי *harey* | after all, surely (shared knowledge) | *harey amárti lekha* 'I told you, after all'; *harey at* 'behold you are' in the wedding formula |
| הלא *halo* | isn't it (rhetorical) | formal |
| בטח *betakh* | surely, of course | *betakh!* 'sure!'; *betakh she-* 'of course that' |
| אולי *ulay* | maybe | *ulay ken, ulay lo* |
| כנראה *kanir'e* | apparently, probably | |
| כמעט *kim'at* | almost | |
| בערך *be-érekh* | approximately | also prefix כ- *ke-*: *ke-shloshim* 'about thirty' (formal) |
| ככה *kákha* | like this, so | *kákha-kákha* 'so-so' |
| כך *kakh* | so, thus | formal; *kol kakh* 'so (much)' |
| כל כך *kol kakh* | so, so much | *kol kakh yafe* 'so beautiful' |
| יותר מדי *yoter midáy* | too (much) | |
| מספיק *maspik* | enough | |
| בסך הכול *be-sakh ha-kol* | all in all, only | |
| סתם *stam* | just (for no reason), merely; just kidding | *stam!* 'just kidding!'; *stam kakh* 'for no particular reason'; *stam ish* 'just some guy' |

```
דווקא היום הוא החליט לבוא.
dávka ha-yom hu hekhlít lavo
precisely DEF-day he decide.PST.3SG.M come.INF
'He decided to come today, of all days.'
```

```
גם אני לא הבנתי כלום.
gam ani lo hevánti klum
also I NEG understand.PST.1SG nothing
'I didn't understand anything either.'
```

### 7.2 Negation words

| Word | Use |
|---|---|
| לא *lo* | general negation of verbs, adjectives and constituents; 'no' |
| אין *eyn* | negative existential ('there is no'); formal negation of the participle (*eyni yodéa* 'I don't know', with suffixes *eyni*, *eynkha*, *eyno* …) |
| אל *al* | negative imperative + future: *al tid'ag* 'don't worry' |
| בל *bal* | literary negation, in fixed compounds: *bal yimakhe* 'indelible' |
| בלי *bli* | without |
| אי- *i-* | prefix 'non-, in-': *i-éfshar* 'impossible', *i-havana* 'misunderstanding' |
| לא- *lo-* | prefix 'non-' on nouns and adjectives: *lo-khuki* 'illegal' |

The full treatment of negation (word order, negative concord, *eyn* with suffixes) is in [07-syntax.md](07-syntax.md).

### 7.3 Time, place and manner adverbs

A short list of very common adverbs: *akhshav* 'now', *az* 'then, so', *etmol* 'yesterday', *ha-yom* 'today', *mákhar* 'tomorrow', *shilshom* 'the day before yesterday', *mokhrotáyim* 'the day after tomorrow', *tamid* 'always', *af pá'am* 'never', *le-itim* 'sometimes', *lif'amim* 'sometimes', *bekarov* 'soon', *pit'om* 'suddenly', *kvar* 'already', *po* / *kan* 'here', *sham* 'there', *hena* 'hither', *le-mála* 'up(stairs)', *le-máta* 'down', *bakhuts* 'outside', *bifnim* 'inside', *maher* 'fast', *le'at* 'slowly', *be-yákhad* 'together', *levad* 'alone', *hárbe* 'a lot', *ktsat* 'a little', *me'od* 'very', *nora* 'terribly' (intensifier: *nora yafe* 'terribly nice'), *sababa* 'fine, great' (slang).

Many adverbs are prepositional phrases written as one word (*bekarov* 'soon' = ב + קרוב, *be-yákhad* 'together', *be-emet* 'really', *be-sédér* 'OK'), and UD often splits them into ADP + NOUN even when they are lexicalised; see [11-ud-analysis.md](11-ud-analysis.md). Manner adverbs are also formed with *be-ofen* + adjective (*be-ofen khofshi* 'freely') or *be-* + abstract noun (*bi-mhirut* 'quickly', *be-zehirut* 'carefully').

### 7.4 Discourse particles (overview)

Spoken Hebrew is full of particles that organise talk and mark attitude. They are only listed here; meanings, functions and examples are in [09-pragmatics-politeness.md](09-pragmatics-politeness.md).

| Particle | Origin | Rough function |
|---|---|---|
| כאילו *ke'ilu* | 'as if' | hedge, quotative, 'like' |
| כזה *kaze* | 'like this' | hedge, approximation, quotative |
| יעני *yá'ani* | Arabic 'it means' | 'I mean', 'sort of' |
| נו *nu* | Yiddish | impatience, 'come on', 'well?' |
| יאללה *yála* | Arabic | 'let's go', 'come on', 'OK, bye' |
| אחלה *áhla* | Arabic | 'great' |
| סבבה *sababa* | Arabic | 'cool, fine' |
| בקיצור *bekitsur* | 'in short' | topic close, 'anyway' |
| בעצם *be-étsem* | 'in essence' | 'actually' |
| תכל'ס *tákhles* | Yiddish < Hebrew *takhlit* 'purpose' | 'bottom line, in practice' |
| אז *az* | 'then' | 'so' (turn-initial) |
| טוב *tov* | 'good' | 'well, OK' (closing) |
| בסדר *be-séder* | 'in order' | 'OK' |
| ואללה *wála* | Arabic 'by God' | 'really?!', 'I swear' |
| אה, אהה *a*, *áha* | | backchannel, 'I see' |
| אוקיי *okey* | English | 'OK' |
| חבל *khaval* | 'pity' | 'what a pity'; slang 'awesome' (*khaval al ha-zman*) |

UD tags most of these `INTJ` or `ADV` with `discourse` or `advmod`; the spoken IAHLTknesset treebank is the place to look for them in annotated form.

---

## 8. Prepositions and verbs: governed prepositions

Many verbs take their object with a fixed preposition instead of את. The preposition is part of the lexical entry and does not translate literally. A frame reader must map these to roles:

| Verb | Preposition | Meaning |
|---|---|---|
| עזר *azar* | ל- | help (someone) |
| חיכה *khika* | ל- | wait for |
| הקשיב *hikshiv* | ל- | listen to |
| טלפן *tilfen* | ל- | phone |
| הודה *hoda* | ל- (person), על (thing) | thank |
| שאל *sha'al* | את (person), על (topic) | ask |
| דיבר *diber* | עם (person), על (topic) | speak |
| פגש *pagash* | את / ב- (formal) | meet |
| השפיע *hishpía* | על | influence |
| שמר *shamar* | על | guard, keep |
| ויתר *viter* | על | give up |
| התחיל *hitkhil* | ב- / את / ל- + INF | begin |
| השתמש *hishtamesh* | ב- | use |
| בחר *bakhar* | ב- | choose |
| נגע *naga* | ב- | touch |
| פחד *pakhad* | מ- | fear |
| נהנה *nehene* | מ- | enjoy |
| התרשם *hitrashem* | מ- | be impressed by |
| התגעגע *hitga'agéa* | ל- | miss (someone) |
| כעס *ka'as* | על | be angry at |
| חשב *khashav* | על | think about |
| האמין *he'emin* | ב- (in), ל- (someone) | believe |
| שכח *shakhakh* | את / מ- / על | forget |
| התחתן *hitkhaten* | עם | marry |

```
אני מתגעגע אליך.
ani mitga'agéa elékha
I miss.PRS.M.SG to.2SG.M
'I miss you.'
```

(*mitga'agéa le-* + noun, but *elékha* with a pronoun: with this verb speakers prefer אל in the suffixed forms.)

In UD, the object of such verbs is `obl` (or `obl` with a subtype in some treebanks), not `obj`, because it has a case marker; only את-marked and bare objects are `obj`. This makes the Hebrew argument structure in UD look very different from the English one, and a role mapper must consult the verb's governed preposition list.

---

## 9. How UD tokenizes these words

The details are in [11-ud-analysis.md](11-ud-analysis.md); here are the facts that bear on this chapter, checked against the IAHLT treebanks of UD 2.18 (CC BY-SA 4.0), on 2026-10-17:

- **Prefixes are separate syntactic words** inside a multiword token: והבית → ו (CCONJ, `cc`) + ה (DET, `det`) + בית.
- **The hidden article** after ב, כ, ל is not restored as a token; the preposition carries `Definite=Def|PronType=Art`. The IAHLTknesset example in the treebank card shows this: בעולם *ba-olam* 'in the world' is ב (ADP, `Definite=Def|PronType=Art`) + עולם. With מ, the article is a real token: מהבית → מ + ה + בית.
- **Inflected prepositions are split** into ADP + PRON, with the surface pieces as FORM and the lemmas restored: לי → ל + י (lemma אני), שלו → של + ו (lemma הוא; relation `nmod:poss`), אותו → אות + ו (lemma את + הוא; relation `obj`), עליו → עלי + ו, ממנו → ממנ + ו, ביניהם → ביני + הם. The pronoun gets the relation (`obl`, `obj`, `nmod:poss`) and the preposition is `case` on it.
- **כש- is one SCONJ** (*kshe-*), not כ + ש.
- **אותו 'the same'** is a DET, not a split ADP + PRON.
- **Compound prepositions** are usually head + `fixed`: כפי ש-, על ידי, בגלל ש- vary by treebank.

Counts in the IAHLT data (train+dev+test, both treebanks together): בו split as ב + ו 127 times, לי as ל + י 113, אותו as אות + ו 109, שלו as של + ו 85, אותם as אות + ם 76, עליו as עלי + ו 36, ממנו as ממנ + ו 16.

---

## 10. Summary for a parser

| Problem | What helps |
|---|---|
| Which first letters are prefixes (ו ה ש ב כ ל מ) | the fixed order ו → ש → ב/כ/ל/מ → ה; a lexicon of stems; a lattice of segmentations |
| Hidden article after ב, כ, ל | agreement of following adjectives (*ba-báyit ha-gadol*), context, pointing if present |
| את: *et* (ACC) or *at* 'you (F)' | what follows: a definite NP vs a predicate |
| לו *lo* 'to him' vs לא *lo* 'not' | spelling (except in errors) |
| ממנו 'from him' vs 'from us' | context, person agreement elsewhere |
| -ך: 2SG.M or 2SG.F | gender agreement in the clause; often unresolvable |
| Plural-type vs singular-type suffixes (עליו vs לו) | lexical class of the preposition |
| Governed prepositions | verb lexicon with its preposition; UD `obl` |
| שלו *shelo* vs *she-lu* vs *she-lo* | context; *she-lu* only in counterfactual wishes |

---

## References

- Academy of the Hebrew Language. 2017. *כללי הכתיב חסר הניקוד* (Rules for unpointed spelling) and punctuation rules. https://hebrew-academy.org.il/topic/hahlatot/missingvocalizationspelling/
- Glinert, Lewis. 2017. *Modern Hebrew: An Essential Grammar.* 4th ed. Routledge. (Not held; cited for inflected prepositions, the prefix order and the vowel of ו.)
- Goldberg, Yoav and Reut Tsarfaty. 2008. "A Single Generative Model for Joint Morphological Segmentation and Syntactic Parsing." ACL 2008. local: $RESEARCH_RAW/aclbulk-2008-goldberg-single-generative-model-joint-morphological/.
- Hacohen, Aviya, Olga Kagan and Dana Plaut. 2021. "Differential Object Marking in Modern Hebrew: Definiteness and Partitivity." *Glossa* 6(1): 148. https://doi.org/10.16995/glossa.5729 — local: $RESEARCH_RAW/lingbulk-2021-hacohen-differential-object-marking-modern-hebrew/.
- More, Amir, Amit Seker, Victoria Basmova and Reut Tsarfaty. 2019. "Joint Transition-Based Models for Morpho-Syntactic Parsing: Parsing Strategies for MRLs and a Case Study from Modern Hebrew." *TACL* 7. https://aclanthology.org/Q19-1003/ (not held locally).
- Seker, Amit et al. 2022. "AlephBERT: Language Model Pre-training and Evaluation from Sub-Word to Sentence Level." ACL 2022. https://aclanthology.org/2022.acl-long.4/ — local: $RESEARCH_RAW/acl-2022-seker-alephbert-language-model-pre-training/.
- Tsarfaty, Reut. 2006. "Integrated Morphological and Syntactic Disambiguation for Modern Hebrew." COLING/ACL 2006 Student Research Workshop. local: $RESEARCH_RAW/aclbulk-2006-tsarfaty-integrated-morphological-syntactic-disambiguation-modern/.
- Tsarfaty, Reut, Amit Seker, Shoval Sadde and Stav Klein. 2019. "What's Wrong with Hebrew NLP? And How to Make it Right." EMNLP-IJCNLP 2019 demos. https://aclanthology.org/D19-3044/ — local: $RESEARCH_RAW/aclbulk-2019-tsarfaty-what-s-wrong-hebrew-nlp/.
- Universal Dependencies 2.18: UD_Hebrew-IAHLTwiki and UD_Hebrew-IAHLTknesset (CC BY-SA 4.0). https://universaldependencies.org/treebanks/he_iahltknesset/ — local: $RESEARCH_RAW/lexres/treebanks/ud-2.18/. Segmentations and counts in §2.2 and §9 were read from the `.conllu` files on 2026-10-17.
- Universal Dependencies, Hebrew guidelines (`_he/feat/Prefix.md`, `_he/dep/case-acc.md`, `_he/dep/case-gen.md`). https://universaldependencies.org/he/ — local: $RESEARCH_RAW/hebgram/sa/ud-docs-he-hbo/_he/.
- Wikipedia contributors. "Modern Hebrew grammar," sections on prepositions and prefixes. https://en.wikipedia.org/wiki/Modern_Hebrew_grammar (consulted 2026-10-17; CC BY-SA; paraphrased).
- Wiktionary (via kaikki.org). English Wiktionary Hebrew entries with inflection tables (checked: בין). local: $RESEARCH_RAW/hebgram/sa/kaikki-enwiktionary-hebrew-aramaic/kaikki.org-dictionary-Hebrew.jsonl (CC BY-SA).
