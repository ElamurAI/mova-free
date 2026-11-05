# Nouns, adjectives, pronouns and numerals

This chapter describes the nominal system of Modern (Israeli) Hebrew. It covers gender and number, the noun patterns (*mishkalim*) and what they mean, the construct state (*smixut*), definiteness, the possessive suffixes, adjectives and their comparison, the pronouns (personal, demonstrative, reflexive, reciprocal, interrogative, indefinite), and the numerals with their gender "polarity". Verbs are in [05-verbs.md](05-verbs.md), prepositions and their inflection in [06-prepositions-particles.md](06-prepositions-particles.md), and clause syntax (copula, את, relatives) in [07-syntax.md](07-syntax.md). How UD tokenizes and annotates all this is in [11-ud-analysis.md](11-ud-analysis.md). Index: [README.md](README.md).

Transliteration follows [02-script-and-spelling.md](02-script-and-spelling.md): broad Israeli pronunciation, *kh* = /χ/, *ts* = צ, *sh* = ש, *'* = a pronounced glottal stop, stress marked with an acute accent only when it is not on the last syllable (*yéled*, *báyit*), a hyphen after a prefix (*ha-yéled*). Glosses use the Leipzig abbreviations: M, F, SG, PL, DU, DEF, CS (construct state), POSS, ACC, GEN, 1SG etc.

---

## 1. Gender

Every Hebrew noun is masculine or feminine; a handful are used with both genders. Gender is lexical for things and ideas, and mostly natural for people and many animals. It controls agreement in adjectives, verbs (past, future, participle), demonstratives, pronouns and numerals, so a parser must know it for every noun, including those whose form does not show it.

### 1.1 Marked feminines

Most feminine nouns end in one of four suffixes. In unpointed writing the first ends in ה, the others in ת.

| Suffix | Spelling | Examples | Notes |
|---|---|---|---|
| *-a* (stressed) | ־ה | ילדה *yalda* 'girl', מנורה *menora* 'lamp', תמונה *tmuna* 'picture' | the commonest; construct *-at* |
| *-et* (unstressed, after a stressed *e* or *a*) | ־ת | מחברת *makhbéret* 'notebook', כתובת *któvet* 'address', מזכירה/מזכרת | segolate feminine; construct unchanged |
| *-it* | ־ית | מכונית *mekhonit* 'car', חנוכיה (*-iya*, see below), עברית *ivrit* 'Hebrew' | languages, diminutives, many artefacts |
| *-ut* | ־ות | ילדות *yaldut* 'childhood', חברות *khaverut* 'friendship', מציאות *metsi'ut* 'reality' | abstract nouns; all feminine |
| *-at* | ־ת | (construct form of *-a*); free in a few words: קשת *késhet* 'bow', דלת *délet* 'door' | |

The feminine of a masculine person or animal noun is usually made with *-a* or *-et*/*-it*:

| Masculine | Feminine | Meaning |
|---|---|---|
| ילד *yéled* | ילדה *yalda* | child |
| מורה *more* | מורה *mora* | teacher (same unpointed spelling!) |
| תלמיד *talmid* | תלמידה *talmida* | pupil |
| סטודנט *student* | סטודנטית *studéntit* | student |
| מזכיר *mazkir* | מזכירה *mazkira* | secretary |
| רופא *rofe* | רופאה *rof'a* | doctor |
| כלב *kélev* | כלבה *kalba* | dog / bitch |
| מלך *mélekh* | מלכה *malka* | king / queen |
| שחקן *sakhkan* | שחקנית *sakhkanit* | player, actor |
| ישראלי *yisre'eli* | ישראלית *yisre'elit* | Israeli |

The pair מורה *more* / מורה *mora* shows a general problem of unpointed text: the masculine *-e* and the feminine *-a* of words from roots ending in a weak letter are both written ה. Without context or pointing, a reader (or a tagger) cannot tell them apart; agreement on a verb or adjective usually decides (Itai, Wintner and Yona 2006 stress that the lexicon must store gender for exactly this reason).

### 1.2 Unmarked feminines

A closed but frequent set of feminine nouns has no feminine ending. The main groups are (Glinert 2017 §5; Wikipedia, *Modern Hebrew grammar*):

- **paired body parts**: יד *yad* 'hand', רגל *régel* 'leg, foot', עין *áyin* 'eye', אוזן *ózen* 'ear', ברך *bérekh* 'knee', כתף *katef* 'shoulder', שן *shen* 'tooth', בטן *béten* 'belly';
- **cities and countries** as places: ירושלים *yerushaláyim*, צרפת *tsarfat*, and the noun עיר *ir* 'city' and ארץ *érets* 'land';
- **some old everyday nouns**: אבן *éven* 'stone', כוס *kos* 'glass, cup', חרב *khérev* 'sword', ציפור *tsipor* 'bird', נעל *náal* 'shoe', כף *kaf* 'spoon', אם *em* 'mother', בת *bat* 'daughter';
- **words of both genders**, where speakers vary or the literary and colloquial gender differ: דרך *dérekh* 'way', שמש *shémesh* 'sun', רוח *rúakh* 'wind, spirit', סכין *sakin* 'knife', נפש *néfesh* 'soul', לשון *lashon* 'tongue, language'.

### 1.3 Masculines that look feminine

Some masculine nouns end in *-a*, *-et* or *-ut*, because the ending is part of the root or the word is a loan:

| Noun | Meaning | Why |
|---|---|---|
| לילה *láyla* | night | old ending, stressed on the first syllable |
| בית *báyit* | house | ת is a root letter |
| מוות *mávet* | death | root letter |
| שירות *sherut* | service | root ש-ר-ת, not the *-ut* suffix |
| זית *záyit* | olive | root letter |
| ערב *érev* | evening | (no ending; listed for contrast with *láyla*) |
| אבא *ába* | father | Aramaic *-a* |

### 1.4 Gender of loanwords and acronyms

Loans default to masculine (*ha-telefon*, *ha-internet*, *ha-imeyl*) unless they end in stressed *-a*, which attracts the feminine (*ha-pitsa*, *ha-banana*, *ha-sigarya* 'cigarette'). Acronyms take the gender of their head word: צה"ל *tsáhal* (Israel Defense Forces, head צבא *tsava* 'army', masculine), אוניברסיטה → *universita* is feminine.

### 1.5 Mixed groups

A group of mixed gender is grammatically masculine plural: *ha-yeladim* can mean 'the boys' or 'the children (boys and girls)'. This is the rule of the standard and of speech alike. Gender-inclusive writing in recent years doubles forms (*talmidim ve-talmidot*), uses a slash (תלמידים/ות) or a dot (תלמידים.ות); it is spreading in academic and activist registers, but it is not part of the grammar a parser should expect in news or Wikipedia text.

---

## 2. Number

Hebrew distinguishes singular and plural in all nouns, and a dual in a small set.

### 2.1 Regular plurals

- Masculine: *-im* (־ים): ספר *séfer* → ספרים *sfarim* 'books', תלמיד *talmid* → תלמידים *talmidim*.
- Feminine: *-ot* (־ות): ילדה *yalda* → ילדות *yeladot*, מכונית *mekhonit* → מכוניות *mekhoniyot*, מחברת *makhbéret* → מחברות *makhbarot*, חנות *khanut* 'shop' → חנויות *khanuyot*.

Adding the plural ending usually moves the stress to the ending and reduces vowels of the stem that are now two syllables before the stress:

| Singular | Plural | Meaning | Change |
|---|---|---|---|
| דבר *davar* | דברים *dvarim* | thing, word | first *a* drops |
| מקום *makom* | מקומות *mekomot* | place | *a* → *e*; plural in *-ot* |
| ילד *yéled* | ילדים *yeladim* | child | segolate: plural stem *CCaC* |
| ספר *séfer* | ספרים *sfarim* | book | segolate |
| בוקר *bóker* | בקרים *bkarim* | morning | segolate |
| חדר *khéder* | חדרים *khadarim* | room | guttural: *khatef* vowel kept as *a* |
| עט *et* | עטים *etim* | pen | monosyllable |
| בית *báyit* | בתים *batim* | house | irregular |
| שולחן *shulkhan* | שולחנות *shulkhanot* | table | masculine, plural in *-ot* |

The segolate nouns (stress on the first syllable, two *e* or *a* vowels, the *CéCeC* pattern) are the main source of stem change: their plural is *CCaCim* (*kélev* → *klavim* 'dogs', *mélekh* → *mlakhim* 'kings'), and their construct plural is *CaCCey* (*kalvey*, *malkhey*; see §4).

### 2.2 Plural "mismatches"

The ending does not always match the gender. Agreement always follows the gender of the noun, never the shape of the ending.

**Masculine nouns with *-ot*** (agreement stays masculine):

| Singular | Plural | Meaning | Agreement |
|---|---|---|---|
| אב *av* | אבות *avot* | father, forefather | *avot gdolim* |
| שולחן *shulkhan* | שולחנות *shulkhanot* | table | *shulkhanot gdolim* |
| מקום *makom* | מקומות *mekomot* | place | |
| חלום *khalom* | חלומות *khalomot* | dream | |
| כיסא *kise* | כיסאות *kis'ot* | chair | |
| לילה *láyla* | לילות *leylot* | night | |
| קול *kol* | קולות *kolot* | voice, vote | |
| רחוב *rekhov* | רחובות *rekhovot* | street | |
| ארון *aron* | ארונות *aronot* | cupboard | |
| אור *or* | אורות *orot* | light | |
| שם *shem* | שמות *shemot* | name | |

**Feminine nouns with *-im*** (agreement stays feminine):

| Singular | Plural | Meaning | Agreement |
|---|---|---|---|
| אישה *isha* | נשים *nashim* | woman | *nashim yafot* 'beautiful women' |
| מילה *mila* | מילים *milim* | word | *milim kashot* |
| שנה *shana* | שנים *shanim* | year | *shanim rabot* |
| ביצה *beytsa* | ביצים *beytsim* | egg | |
| יונה *yona* | יונים *yonim* | dove | |
| נמלה *nemala* | נמלים *nemalim* | ant | |
| עיר *ir* | ערים *arim* | city | |
| אבן *éven* | אבנים *avanim* | stone | |
| שעורה *se'ora* | שעורים *se'orim* | barley | |

```
יש פה הרבה מילים קשות.
yesh po harbe milim kashot
EXIST here many word.PL(F) hard.F.PL
'There are many hard words here.'
```

```
השולחנות החדשים כבר הגיעו.
ha-shulkhanot ha-khadashim kvar higi'u
DEF-table.PL(M) DEF-new.M.PL already arrive.PST.3PL
'The new tables have already arrived.'
```

**Suppletive and irregular plurals**: איש *ish* → אנשים *anashim* 'man → people', בת *bat* → בנות *banot* 'daughter, girl', בן *ben* → בנים *banim* 'son', יום *yom* → ימים *yamim* 'day', ראש *rosh* → ראשים *rashim* 'head', אח *akh* → אחים *akhim* 'brother', אחות *akhot* → אחיות *akhayot* 'sister, nurse', עיר *ir* → ערים *arim*.

**Loanwords** take *-im* (masculine) or *-ot*/*-iyot* (feminine): טלפון → טלפונים *telefonim*, אימייל → אימיילים *imeylim*, פיצה → פיצות *pitsot*, סיגריה → סיגריות *sigaryot*. Some loans ending in *-o* or *-i* keep the singular stem (*radyo* → *radyo'im*, *taksi* → *taksiyot*/*monitim*).

### 2.3 The dual

The dual ending *-áyim* (־יים, stressed on the *a*) survives in three kinds of word (Wikipedia, *Modern Hebrew grammar*, "Dual"):

**(a) Real duals of measure and time.** Here *-áyim* means exactly 'two', and contrasts with both singular and plural:

| Singular | Dual | Plural (3 and more) |
|---|---|---|
| יום *yom* 'day' | יומיים *yomáyim* 'two days' | ימים *yamim* |
| שבוע *shavúa* 'week' | שבועיים *shvu'áyim* | שבועות *shavu'ot* |
| חודש *khódesh* 'month' | חודשיים *khodsháyim* | חודשים *khodashim* |
| שנה *shana* 'year' | שנתיים *shnatáyim* | שנים *shanim* |
| שעה *sha'a* 'hour' | שעתיים *sha'atáyim* | שעות *sha'ot* |
| פעם *pá'am* 'time, once' | פעמיים *pa'amáyim* 'twice' | פעמים *pe'amim* |
| מאה *me'a* 'hundred' | מאתיים *matáyim* '200' | מאות *me'ot* |
| אלף *élef* 'thousand' | אלפיים *alpáyim* '2000' | אלפים *alafim* |

A speaker says *yomáyim*, not \**shney yamim*; the analytic form is rare and marked.

**(b) Paired body parts and paired things.** The dual form is the ordinary plural, used for any number above one:

| Singular | Dual/plural | Meaning |
|---|---|---|
| יד *yad* | ידיים *yadáyim* | hands |
| רגל *régel* | רגליים *ragláyim* | legs, feet |
| עין *áyin* | עיניים *eynáyim* | eyes |
| אוזן *ózen* | אוזניים *oznáyim* | ears |
| ברך *bérekh* | ברכיים *birkáyim* | knees |
| נעל *náal* | נעליים *naaláyim* | shoes |
| גרב *gérev* | גרביים *garbáyim* | socks |

```
לשולחן הזה יש שלוש רגליים.
la-shulkhan ha-ze yesh shalosh ragláyim
to.DEF-table DEF-this EXIST three.F leg.DU
'This table has three legs.'
```

**(c) Pseudo-duals.** Nouns that have only the dual form and name one object made of two parts. They are grammatically plural, and to count them one uses זוג *zug* 'pair':

| Noun | Meaning |
|---|---|
| משקפיים *mishkafáyim* | glasses |
| מכנסיים *mikhnasáyim* | trousers |
| מספריים *misparáyim* | scissors |
| אופניים *ofanáyim* | bicycle |
| מלקחיים *melkakháyim* | tongs |

```
קניתי שני זוגות מכנסיים.
kaníti shney zugot mikhnasáyim
buy.PST.1SG two.M.CS pair.PL trousers
'I bought two pairs of trousers.'
```

Some place names and other nouns have a frozen *-áyim* that is not a dual at all: מים *máyim* 'water' (plural noun), שמיים *shamáyim* 'sky' (plural), צהריים *tsohoráyim* 'noon', ירושלים *yerushaláyim*, מצרים *mitsráyim* 'Egypt' (feminine singular as a country).

### 2.4 Collectives, mass nouns, pluralia tantum

- **Collectives** are singular in form and agreement: צאן *tson* 'flock (sheep and goats)', בקר *bakar* 'cattle', עוף *of* 'poultry, chicken (as food)', דגה *daga* 'fish (collective)'. The UD feature is just `Number=Sing`.
- **Mass nouns** can still pluralize with a kind reading: *máyim* is always plural; יין *yáyin* 'wine' → יינות *yeynot* 'wines'.
- **Pluralia tantum** (no singular in use): *máyim* 'water', *shamáyim* 'sky', פנים *panim* 'face' (also 'inside'), חיים *khayim* 'life', רחמים *rakhamim* 'mercy', the dual-only items above, and the honorific אלוהים *elohim* 'God' which takes singular agreement when it means the God of Israel.

```
המים קרים.
ha-máyim karim
DEF-water(PL) cold.M.PL
'The water is cold.'
```

```
הפנים שלו היו חיוורים.
ha-panim shelo hayu khivarim
DEF-face(PL) of.3SG.M be.PST.3PL pale.M.PL
'His face was pale.'
```

---

## 3. Noun patterns (*mishkalim*)

Most native nouns are built by putting a consonantal root (*shóresh*, usually three consonants) into a pattern (*mishkal*) of vowels, prefixes, suffixes and doubling. A pattern often carries a general meaning (action, agent, place, tool, quality), though meanings have drifted and many words fit the pattern only in form (Wikibooks, *Dikduk ha-ivrit ha-khadasha*, page on שם, makes the same caution). The pattern names below use C for a root consonant, as in English grammars; Hebrew grammars use the dummy root ק-ט-ל (*miktal*, *katlan* …).

| Pattern | Typical meaning | Examples (root) |
|---|---|---|
| *CaCaC* (historically *CaCCaC*) | profession | טבח *tabakh* 'cook' (ט-ב-ח), נגר *nagar* 'carpenter', ספר *sapar* 'barber', טייס *tayas* 'pilot', צלם *tsalam* 'photographer', חייל *khayal* 'soldier' |
| *CaCCan* | agent with a habit or trait | רקדן *rakdan* 'dancer', שקרן *shakran* 'liar', קמצן *kamtsan* 'miser', בכיין *bakhyan* 'crybaby', פחדן *pakhdan* 'coward' |
| *CoCeC* (participle) | agent, occupation | שומר *shomer* 'guard', שוטר *shoter* 'policeman', סופר *sofer* 'writer', רופא *rofe* 'doctor' |
| *meCaCeC*, *maCCiC*, *meCaCCeC* (participles of *pi'el*, *hif'il*, *hitpa'el*) | agent, device | מנהל *menahel* 'manager', מדריך *madrikh* 'guide, instructor', מקרר *mekarer* 'refrigerator', מתנדב *mitnadev* 'volunteer' |
| *miCCaC* | place, result, object | משרד *misrad* 'office', מפעל *mif'al* 'factory', מכתב *mikhtav* 'letter', מגדל *migdal* 'tower', מטבח *mitbakh* 'kitchen' |
| *miCCaCa* | place | מרפאה *mirpa'a* 'clinic', מכללה *mikhlala* 'college', מסעדה *mis'ada* 'restaurant', מספרה *mispara* 'barbershop', מאפייה *ma'afiya* 'bakery' |
| *maCCeC*, *maCCeCa* | tool | מפתח *mafte'akh* 'key', מסרק *masrek* 'comb', מזלג *mazleg* 'fork', מחשב *makhshev* 'computer', מברג *mavreg* 'screwdriver', מצלמה *matslema* 'camera', משאבה *masheva* 'pump' |
| *CiCuC* | action noun of *pi'el* | דיבור *dibur* 'speech' (דיבר), ביקור *bikur* 'visit', תיקון *tikun* 'repair', שיפור *shipur* 'improvement', סידור *sidur* 'arrangement; prayer book' |
| *CCiCa* | action noun of *pa'al* | כתיבה *ktiva* 'writing', שמירה *shmira* 'guarding', אכילה *akhila* 'eating', נסיעה *nesi'a* 'trip', הליכה *halikha* 'walking' |
| *hiCaCCut* | action noun of *nif'al* | היכנסות *hikansut* 'entering', הישארות *hisha'arut* 'staying', הירדמות *hiradmut* 'falling asleep' |
| *haCCaCa* | action noun of *hif'il* | החלטה *hakhlata* 'decision', הזמנה *hazmana* 'invitation, order', הצגה *hatsaga* 'show', הרגשה *hargasha* 'feeling', הסבר *hesber* (*heCCeC* variant) 'explanation' |
| *hitCaCCut* | action noun of *hitpa'el* | התפתחות *hitpatkhut* 'development', התנגדות *hitnagdut* 'opposition', התרגשות *hitragshut* 'excitement', התנהגות *hitnahagut* 'behaviour' |
| *CéCeC* (segolate) | basic nouns | ילד *yéled*, ספר *séfer*, כלב *kélev*, דרך *dérekh*, בוקר *bóker* |
| *CaCéCet* | illness | דלקת *daléket* 'inflammation', צהבת *tsahévet* 'jaundice', אדמת *adémet* 'rubella', שפעת *shapá'at* 'flu', סוכרת *sukéret* 'diabetes' |
| *CiCaCon* | abstract, collection | זיכרון *zikaron* 'memory', פיקדון *pikadon* 'deposit', חיסכון *khisakhon* 'saving' |
| *-ut* | abstract quality | ילדות *yaldut* 'childhood', אחריות *akhrayut* 'responsibility', אפשרות *efsharut* 'possibility', יהדות *yahadut* 'Judaism' |
| *-iya* | place of work or sale, collection | ספרייה *sifriya* 'library', נגרייה *nagariya* 'carpentry shop', פיצרייה *pitseriya* 'pizzeria', גלידרייה *glideriya* 'ice-cream parlour' |
| *-on* | diminutive, also books/periodicals | ספרון *sifron* 'booklet', דובון *dubon* 'teddy bear' (דוב 'bear'), סרטון *sirton* 'clip, short video' (סרט 'film'), עיתון *iton* 'newspaper' (עת 'time'), מילון *milon* 'dictionary' (מילה) |
| *-it* | diminutive, small artefact | כפית *kapit* 'teaspoon' (כף 'spoon'), מפית *mapit* 'napkin' (מפה 'tablecloth'), שקית *sakit* 'bag' (שק 'sack'), כוסית *kosit* 'small glass' |
| reduplication *CCaCCaC* | diminutive | כלבלב *klavlav* 'puppy', חתלתול *khataltul* 'kitten', זקנקן *zkankan* 'little beard' |
| *-an* | agent, also from nouns | עיתונאי? (*-ay*) → see below; פסלן *paslan* 'sculptor', מדען *mad'an* 'scientist', חקלאי *khakla'i* (*-ay*) 'farmer' |
| *-ay*, *-ai* | profession | עיתונאי *itona'i* 'journalist', חשמלאי *khashmala'i* 'electrician'; מוזיקאי *muzika'i* 'musician' |

Linear suffixes (*-ut*, *-iya*, *-on*, *-it*, *-ay*, *-an*, *-i*) attach to whole words, not to roots, and are the main way Modern Hebrew builds words from loans: *tikshóret* → *tikshurti*, *politika* → *politika'i* 'politician', *sport* → *sporta'i* 'athlete' (Bolozky 1999; Ravid on linear vs non-linear word formation — not held here, see [08-lexicon-register.md](08-lexicon-register.md)).

Patterns matter for two practical reasons. First, a morphological analyzer such as Hspell or the Dicta word list maps an unpointed form to a set of (root, pattern) analyses, and the pattern often decides the lemma and the part of speech (מכתב is 'letter', not a verb). Second, the action-noun patterns (*CiCuC*, *haCCaCa*, *hitCaCCut*, *CCiCa*) are productive: every verb in a *binyan* has a predictable action noun, which a frame reader can map back to the event (see [05-verbs.md](05-verbs.md), and the UD analysis of nominalizations in [11-ud-analysis.md](11-ud-analysis.md)).

---

## 4. The construct state (*smixut*)

### 4.1 What it is

The construct state (Hebrew סמיכות *smikhut*, 'leaning') joins two nouns into one phrase: the first noun (*nismakh*, the head) is in a special bound form and leans on the second noun (*somekh*, the dependent), which is in its ordinary form. The pair is stressed as one phonological word; the main stress falls on the second noun, and the first noun often loses vowels.

```
בית ספר
beyt séfer
house.CS book
'school' (lit. house-of-book)
```

```
ממשלת ישראל
memshélet yisra'el
government.CS Israel
'the government of Israel'
```

The construct expresses possession, part-whole, material, purpose, a characterizing property, and the arguments of action nouns: *nitskhon ha-miflaga al yeriveha* 'the party's victory over its rivals', where the external argument (the party) follows the head noun directly in the construct (Bruening 2022, ex. 1, citing Preminger 2020).

### 4.2 The forms of the construct

Masculine singular nouns often keep their form, or lose a vowel; the other forms change in regular ways (Wikipedia, *Modern Hebrew grammar*, "Construct state"):

| Absolute | Construct | Example | Meaning |
|---|---|---|---|
| *-a* (F.SG) ילדה *yalda* | *-at* ילדת *yaldat* | ילדת פלא *yaldat péle* | child prodigy (F) |
| משפחה *mishpakha* | משפחת *mishpákhat* | משפחת כהן *mishpákhat kohen* | the Cohen family |
| שנה *shana* | שנת *shnat* | שנת לימודים *shnat limudim* | school year |
| *-et*, *-it*, *-ut* (F.SG) | unchanged | מכונית ספורט *mekhonit sport* | sports car |
| *-ot* (F.PL) | unchanged | בנות הכיתה *bnot ha-kita* | the girls of the class |
| *-im* (M.PL) | *-ey* (־י) | ילדים → ילדי *yaldey*; ספרים → ספרי *sifrey* | ספרי ילדים *sifrey yeladim* 'children's books' |
| *-áyim* (DU) | *-ey* | עיניים → עיני *eyney* | עיני הכלב 'the dog's eyes' |
| M.SG with *a* in the first syllable | first vowel drops | דבר *davar* → דבר *dvar* | דבר המלך *dvar ha-mélekh* 'the king's word' |
| | | מקום *makom* → מקום *mekom* | מקום עבודה *mekom avoda* 'workplace' |
| M.SG with final *-e* | *-e* | שדה *sade* → שדה *sde* | שדה תעופה *sde te'ufa* 'airport' |
| irregular | | בית *báyit* → בית *beyt*; בתים → בתי *batey* | בתי ספר *batey séfer* 'schools' |
| | | אישה *isha* → אשת *éshet* | אשת עסקים *éshet asakim* 'businesswoman' |
| | | אב *av* → אבי *avi*; אח *akh* → אחי *akhi* | אבי הכלה *avi ha-kala* 'the bride's father' (formal) |
| segolate plurals | *CaCCey* / *CiCCey* | ילדים → ילדי *yaldey*, מלכים → מלכי *malkhey*, ספרים → ספרי *sifrey* | |
| *CaCaC* plural | *CiCCey* | דברים *dvarim* → דברי *divrey* | דברי הימים 'Chronicles' |

In unpointed writing the construct is usually visible only on feminine *-a* nouns (ה → ת) and masculine plurals (ים → י). For a masculine singular noun (ספר, בית, מקום) the construct and absolute forms are spelled alike, so the parser must recognise the construct from the following noun and the absence of the article on the first noun.

### 4.3 Definiteness sits on the last member

The article ה- never goes on the head of a construct; it goes on the last noun, and the whole phrase becomes definite:

```
בית ספר            בית הספר
beyt séfer          beyt ha-séfer
house.CS book       house.CS DEF-book
'a school'          'the school'
```

A definite construct phrase behaves as definite everywhere: it takes את as an object, and its adjectives take the article.

```
ראיתי את בית הספר החדש.
ra'íti et beyt ha-séfer he-khadash
see.PST.1SG ACC house.CS DEF-book DEF-new.M.SG
'I saw the new school.'
```

The dependent can also be definite because it is a proper name or carries a possessive suffix: *memshélet yisra'el* 'the government of Israel', *beyt avi* 'my father's house'.

**Colloquial article on the head.** In lexicalised compounds speakers often put the article on the first member: *ha-beyt séfer*, *ha-yom huledet* 'the birthday', *ha-orekh din* 'the lawyer'. Normative grammar rejects it; it is common in speech and informal writing, and is a sign that the compound has become one word (Glinert 2017; Wikipedia, *Modern Hebrew grammar*). A tagger trained on HTB news text will rarely see it; one trained on IAHLTknesset (spoken parliament) may.

### 4.4 Chains of several nouns

A construct can contain another construct; every noun except the last is in the construct form, and only the last can be definite:

```
דלת בית הספר
délet beyt ha-séfer
door.CS house.CS DEF-book
'the door of the school'
```

```
יו"ר ועדת החוץ והביטחון של הכנסת
yoshev rosh va'adat ha-khuts ve-ha-bitakhon shel ha-knéset
chair.CS head.CS committee.CS DEF-foreign and-DEF-security of DEF-Knesset
'the chairman of the Knesset Foreign Affairs and Defence Committee'
```

Long chains are formal; the colloquial language prefers to break them with של (§4.6). The IAHLTwiki example quoted in the treebank card, *נשיא בית המשפט העליון* 'the President of the Supreme Court' (sent_id `iahltwiki_piskat-hitgabrut-27`), is a three-noun chain with an adjective at the end.

### 4.5 Adjectives after a construct

An adjective cannot come between the two nouns; it follows the whole chain. It agrees in gender, number and definiteness with the noun it modifies, which may be the head or the dependent. Agreement is often the only clue:

```
מכונית המנהל החדשה
mekhonit ha-menahel ha-khadasha
car(F).CS DEF-manager(M) DEF-new.F.SG
'the manager's new car'
```

```
מכונית המנהל החדש
mekhonit ha-menahel he-khadash
car(F).CS DEF-manager(M) DEF-new.M.SG
'the new manager's car'
```

When the two nouns have the same gender and number, the phrase is ambiguous (*beyt ha-more ha-gadol* 'the big house of the teacher' or 'the house of the big teacher'). Speakers then switch to the free genitive with של.

### 4.6 Three ways to say "X of Y"

Modern Hebrew has three genitive constructions (Netzer and Elhadad 1999 give a contrastive analysis for generation; Wikipedia, *Modern Hebrew grammar*, "Possession"):

| Construction | Example | Register |
|---|---|---|
| construct state | בית המורה *beyt ha-more* | formal or lexicalised; neutral for compounds |
| free genitive with של *shel* | הבית של המורה *ha-báyit shel ha-more* | the everyday form |
| double genitive | ביתו של המורה *beyto shel ha-more* | formal, journalistic; the head carries a possessive suffix agreeing with the possessor |

```
הבית של המורה
ha-báyit shel ha-more
DEF-house of DEF-teacher
'the teacher's house'
```

```
ביתו של המורה
beyt-o shel ha-more
house-POSS.3SG.M of DEF-teacher
'the teacher's house' (formal)
```

In the free genitive the possessed noun takes the article itself (*ha-báyit*), unlike the construct. The double genitive is common in news (*mot-o shel...* 'the death of ...', *bsofo shel davar* 'in the end', lit. 'in its end of thing'), and appears in the IAHLTknesset sentence *הם רוצים גם את ירושלים בסופו של דבר* 'they want Jerusalem too, in the end' (sent_id `13_ptm_532410-11`).

### 4.7 Compounds and syntactic constructs

Not every construct is the same. Two ends of a scale:

- **Lexical compounds** have a non-compositional meaning, cannot be split, and pluralise as a unit, usually on the head: *beyt séfer* 'school' → *batey séfer*; *yom huledet* 'birthday' → *yemey huledet*; *orekh din* 'lawyer' → *orkhey din*; *kadur regel* 'football' (sometimes written as one word, *kaduregel*). Some have fused into single words: רמקול *ramkol* 'loudspeaker' (*ram* 'loud' + *kol* 'voice'), כדורגל *kaduregel*, קולנוע *kolnóa* 'cinema' (*kol* + *nóa*).
- **Syntactic constructs** are free combinations: *beyt ha-more* 'the teacher's house', *memshélet yisra'el*. The dependent can be modified, coordinated and replaced by a suffix (*beyt-o*).

The distinction is central for UD: both kinds are written with the relation `compound:smixut` in the UD Hebrew guidelines and `compound` with `Definite=Cons` on the head in the IAHLT treebanks; see [11-ud-analysis.md](11-ud-analysis.md). A measure of how frequent the construct is: in the IAHLT treebanks (UD 2.18), 11 014 words in IAHLTwiki and 3 789 in IAHLTknesset carry `Definite=Cons`, against 10 381 and 3 137 `compound` relations (counted by streaming the `.conllu` files on 2026-10-17).

### 4.8 Adjectival and quantifier constructs

The construct is not limited to nouns:

- **Adjectival construct** ('X of Y' = 'X with respect to Y'): יפה עיניים *yefe eynáyim* 'beautiful-eyed', כבד שמיעה *kvad shmi'a* 'hard of hearing', קל דעת *kal dá'at* 'frivolous', רחב לב *rekhav lev* 'generous'.
- **Quantifiers** in the construct: כל *kol* 'all, every' (*kol ha-yeladim* 'all the children'), רוב *rov* 'most' (*rov ha-anashim*), חלק *khélek* 'part' (*khélek me-ha-...* or *khélek ha-...*), שום *shum* 'any (negative)', and the numeral construct forms (*shlóshet ha-yeladim* 'the three children', §9.3). UD gives שום the feature `Definite=Cons`, as in the IAHLTknesset sentence quoted in the treebank card (*בשום סקטור בעולם*).
- **Superlative construct**: גדול המשוררים *gdol ha-meshorerim* 'the greatest of poets' (§7.4).

---

## 5. Definiteness

### 5.1 The article

The definite article is the prefix ה- *ha-*, written together with the word. It has no gender or number. Before a guttural or *r* it is *ha-* or *he-* by the old rules (*he-khaver* 'the friend', *he-arim* 'the cities', but *ha-ir* 'the city'), but in normal speech most speakers say *ha-* everywhere except before a few words with *e* (*he-khadash* 'the new one') (see [03-phonology.md](03-phonology.md)). After the prepositions ב, כ, ל the article fuses with them: *ba-báyit* 'in the house', not \**be-ha-báyit* (detail in [06-prepositions-particles.md](06-prepositions-particles.md)).

The article marks:

- a noun known to the hearer: *ha-yéled* 'the boy';
- a generic or class noun: *ha-adam* 'man(kind)', *ha-kélev hu khaver ha-adam* 'the dog is man's friend';
- every adjective that modifies a definite noun: *ha-yéled ha-katan* 'the little boy';
- the demonstrative after a noun: *ha-yéled ha-ze* 'this boy';
- the relative ה- in literary language before a participle (*ha-yoshvim* 'those who sit'), which UD analyses separately (see [07-syntax.md](07-syntax.md)).

### 5.2 Inherently definite nouns

Proper names, nouns with a possessive suffix, and construct phrases whose last member is definite are definite without ה-:

| Type | Example |
|---|---|
| proper name | דני *dani*, ירושלים *yerushaláyim*, ישראל *yisra'el* |
| possessive suffix | ספרי *sifri* 'my book', ביתו *beyto* 'his house' |
| definite construct | בית הספר *beyt ha-séfer* |

Agreement shows their definiteness: *yerushaláyim ha-atika* 'old Jerusalem' (the adjective has the article), *sifri he-khadash* 'my new book'.

### 5.3 Definite objects take את

A definite direct object must be preceded by את *et*; an indefinite object must not:

```
ראיתי ילד.              ראיתי את הילד.
ra'íti yéled              ra'íti et ha-yéled
see.PST.1SG boy           see.PST.1SG ACC DEF-boy
'I saw a boy.'            'I saw the boy.'
```

```
פגשתי את דני.
pagáshti et dani
meet.PST.1SG ACC Dani
'I met Dani.'
```

This is a case of differential object marking. The standard view is that et goes with definite objects only; Hacohen, Kagan and Plaut (2021) show in a judgment study (41 speakers) that et with partitive objects such as *et khélek me-ha-sfarim* 'some of the books' is also widely accepted (average 3.6 of 5), more so for preposed objects and proportional quantifiers. *et ze* 'it, this' is normal, because the pronoun *ze* counts as definite; et before an indefinite human object (*ra'íti et míshehu*) is heard but is not standard. The syntax of et is in [07-syntax.md](07-syntax.md); its inflection (*oti*, *otkha* …) in [06-prepositions-particles.md](06-prepositions-particles.md).

In the IAHLT treebanks et is the word את with relation `case` on the object; the 2.18 data have 1 514 such words in IAHLTwiki and 1 224 in the smaller spoken IAHLTknesset, a sign of how much more often speech uses definite objects (counted 2026-10-17).

### 5.4 Indefiniteness: no article, אחד and איזה

Hebrew has no indefinite article: *yéled* is 'a boy' or 'boy'. Two words come close to one:

- **אחד/אחת** *ekhad*/*akhat* 'one' after the noun means 'a certain, some': *yesh li khaver ekhad she-...* 'I have a friend who ...'. It introduces a new specific referent, like English *this* in "there was this guy".
- **איזה/איזו/איזשהו** *éyze*/*éyzo*/*éyzeshehu* before the noun means 'some (unknown), any': *éyze bakhur halakh* 'some guy left'; *yesh lekha éyzeshehu ra'ayon?* 'do you have any idea?'. It is also the interrogative 'which' (§8.5).

```
פגשתי בחור אחד במסיבה.
pagáshti bakhur ekhad ba-mesiba
meet.PST.1SG guy one.M in.DEF-party
'I met a (certain) guy at the party.'
```

---

## 6. Possessive suffixes

### 6.1 The two series

Possessive pronouns can be attached to a noun as suffixes. There are two series: one for singular nouns (and feminine plurals in *-ot*, which add their own *-ey-*), and one for masculine plural and dual nouns, whose suffixes contain *-ay-*/*-ey-*. The suffix attaches to the construct stem: *séfer* → *sifr-*, *tmuna* → *tmunat-*.

**Singular noun: ספר *séfer* 'book', stem *sifr-*; תמונה *tmuna* 'picture', stem *tmunat-***

| Person | Suffix | ספר 'book' | תמונה 'picture' |
|---|---|---|---|
| 1SG | *-i* | ספרי *sifri* | תמונתי *tmunati* |
| 2SG.M | *-kha* | ספרך *sifrekha* | תמונתך *tmunatkha* |
| 2SG.F | *-ekh* | ספרך *sifrekh* | תמונתך *tmunatekh* |
| 3SG.M | *-o* | ספרו *sifro* | תמונתו *tmunato* |
| 3SG.F | *-a* (written ־ה, mappiq in pointing: ־הּ) | ספרה *sifra* | תמונתה *tmunata* |
| 1PL | *-énu* | ספרנו *sifrénu* | תמונתנו *tmunaténu* |
| 2PL.M | *-khem* | ספרכם *sifrekhem* | תמונתכם *tmunatkhem* |
| 2PL.F | *-khen* | ספרכן *sifrekhen* | תמונתכן *tmunatkhen* |
| 3PL.M | *-am* | ספרם *sifram* | תמונתם *tmunatam* |
| 3PL.F | *-an* | ספרן *sifran* | תמונתן *tmunatan* |

**Plural noun: ספרים *sfarim* 'books', stem *sfar-*/*sifrey-*; תמונות *tmunot* 'pictures', stem *tmunot-***

| Person | Suffix | ספרים 'books' | תמונות 'pictures' |
|---|---|---|---|
| 1SG | *-ay* | ספריי *sfaray* | תמונותיי *tmunotay* |
| 2SG.M | *-ékha* | ספריך *sfarékha* | תמונותיך *tmunotékha* |
| 2SG.F | *-áyikh* | ספרייך *sfaráyikh* | תמונותייך *tmunotáyikh* |
| 3SG.M | *-av* | ספריו *sfarav* | תמונותיו *tmunotav* |
| 3SG.F | *-éha* | ספריה *sfaréha* | תמונותיה *tmunotéha* |
| 1PL | *-énu* | ספרינו *sfarénu* | תמונותינו *tmunoténu* |
| 2PL.M | *-eykhem* | ספריכם *sifreykhem* | תמונותיכם *tmunoteykhem* |
| 2PL.F | *-eykhen* | ספריכן *sifreykhen* | תמונותיכן *tmunoteykhen* |
| 3PL.M | *-eyhem* | ספריהם *sifreyhem* | תמונותיהם *tmunoteyhem* |
| 3PL.F | *-eyhen* | ספריהן *sifreyhen* | תמונותיהן *tmunoteyhen* |

Notes on spelling. In full spelling (*ktiv male*) the 1SG plural suffix is written with a double yod (ספריי) to distinguish it from the singular ספרי *sifri* 'my book' (Academy of the Hebrew Language rules for unpointed spelling, 2017; see [02-script-and-spelling.md](02-script-and-spelling.md)). The 3SG.F singular *-a* (ספרה) and the absolute feminine noun (ספרה *sifra* 'digit') are homographs; the 1PL *sfarénu* 'our books' and *sifrénu* 'our book' differ only by a yod.

A word-form like ספרו is thus ambiguous at least between *sifro* 'his book', *safru* 'they counted' and *sipru* 'they told' (verbs of the root ס-פ-ר in *pa'al* and *pi'el*), while ספריו *sfarav* 'his books' differs by one letter; this ambiguity of suffixes is one of the main reasons Hebrew morphological analysis and segmentation are hard (Tsarfaty et al. 2019; Seker et al. 2022).

### 6.2 Register: bound possessive vs של

In everyday speech the bound possessive is restricted to:

- **kinship and close personal terms**: אבי *avi* 'my father' (formal) but *aba sheli* in speech, בני *bni* 'my son', בתי *biti* 'my daughter', אשתי *ishti* 'my wife', בעלי *ba'ali* 'my husband', אחי *akhi* 'my brother' (also slang 'bro'), חברי *khaveri* 'my friend' (formal);
- **fixed phrases**: *ma shlomkha?* 'how are you (M)?' (lit. 'what is your peace'), *le-da'ati* 'in my opinion', *be-atsmi* 'by myself', *lefi da'ato* 'in his view', *be-khol libi* 'with all my heart', *be-sofo shel davar*;
- **body parts in idioms**: *al rosho* 'on his head' (= on his responsibility).

Elsewhere, speech uses the noun with the article plus של and a pronoun: *ha-séfer sheli* 'my book', *ha-bayit shelahem* 'their house'. The written standard, especially news and Wikipedia, still uses the bound forms widely. A quick count in UD 2.18 (2026-10-17) shows the split by genre: in the IAHLTwiki treebank there are 2 132 bound possessive pronouns (`PRON` with `Poss=Yes`) against 132 של + pronoun; in the spoken IAHLTknesset, 803 against 215 — still mostly bound (parliamentary speech is formal), but with *shel* + pronoun about four times as common relative to the written texts. An example of the colloquial pattern from the Knesset data:

```
לא ירשתי מסבא שלי תלמוד.
lo yarášti mi-sába sheli talmud
NEG inherit.PST.1SG from-grandpa of.1SG Talmud
'I didn't inherit Talmud from my grandfather.'
```

(IAHLTknesset, sent_id `19_ptm_221681-89`, CC BY-SA 4.0.) The same corpus has the formal bound plural in *bi-mhalakh dvaray* 'during my words (= my speech)' (sent_id `15_ptv_490855-42`).

### 6.3 Analysis in UD

UD Hebrew splits a noun with a possessive suffix into a multiword token: the noun (in the IAHLT data it carries gender and number but no `Definite` feature) and a `PRON` with `Poss=Yes` and `Case=Gen`, attached as `nmod:poss`. The double genitive *beyto shel ha-more* is then two `nmod:poss` dependents of one noun (the pronoun and *ha-more*), as in the IAHLTwiki example in the treebank card (*עמדתו של נשיא בית המשפט*). See [11-ud-analysis.md](11-ud-analysis.md).

---

## 7. Adjectives

### 7.1 Agreement

Adjectives follow the noun and agree with it in gender, number and definiteness:

| | Indefinite | Definite |
|---|---|---|
| M.SG | ילד גדול *yéled gadol* | הילד הגדול *ha-yéled ha-gadol* |
| F.SG | ילדה גדולה *yalda gdola* | הילדה הגדולה *ha-yalda ha-gdola* |
| M.PL | ילדים גדולים *yeladim gdolim* | הילדים הגדולים *ha-yeladim ha-gdolim* |
| F.PL | ילדות גדולות *yeladot gdolot* | הילדות הגדולות *ha-yeladot ha-gdolot* |

Agreement follows the noun's gender, not its ending: *shulkhanot gdolim* 'big tables' (M), *milim ktanot* 'small words' (F). A noun with two adjectives repeats the article on each: *ha-báyit ha-gadol ha-yashan* 'the big old house'.

Predicate adjectives agree in gender and number but take no article; the article would turn the phrase back into a noun phrase:

```
הילד גדול.            הילד הגדול
ha-yéled gadol         ha-yéled ha-gadol
DEF-boy big.M.SG       DEF-boy DEF-big.M.SG
'The boy is big.'      'the big boy'
```

This contrast (article or no article on the adjective) is the main cue that tells a verbless clause from a noun phrase; see [07-syntax.md](07-syntax.md).

### 7.2 Inflection classes

Adjectives inflect like nouns, with four forms:

| Pattern | M.SG | F.SG | M.PL | F.PL | Meaning |
|---|---|---|---|---|---|
| *CaCoC* | גדול *gadol* | גדולה *gdola* | גדולים *gdolim* | גדולות *gdolot* | big |
| | ירוק *yarok* | ירוקה *yeruka* | ירוקים *yerukim* | ירוקות *yerukot* | green |
| *CaCaC* | חדש *khadash* | חדשה *khadasha* | חדשים *khadashim* | חדשות *khadashot* | new |
| *CaCeC* | זקן *zaken* | זקנה *zkena* | זקנים *zkenim* | זקנות *zkenot* | old (person) |
| *CaCiC* | נעים *na'im* | נעימה *ne'ima* | נעימים *ne'imim* | נעימות *ne'imot* | pleasant |
| *CaCuC* (passive) | סגור *sagur* | סגורה *sgura* | סגורים *sgurim* | סגורות *sgurot* | closed |
| *-e* | יפה *yafe* | יפה *yafa* | יפים *yafim* | יפות *yafot* | beautiful |
| | קשה *kashe* | קשה *kasha* | קשים *kashim* | קשות *kashot* | hard |
| *-i* | ישראלי *yisre'eli* | ישראלית *yisre'elit* | ישראלים *yisre'elim* | ישראליות *yisre'eliyot* | Israeli |
| | מדעי *mada'i* | מדעית *mada'it* | מדעיים *mada'iyim* | מדעיות *mada'iyot* | scientific |
| *CiCeC* (defects) | עיוור *iver* | עיוורת *ivéret* | עיוורים *ivrim* | עיוורות *ivrot* | blind |
| *CaCaCCaC* (diminutive) | אדמדם *adamdam* | אדמדמה *adamdama* | אדמדמים | אדמדמות | reddish |
| loan | נורמלי *normáli* | נורמלית *normálit* | נורמליים | נורמליות | normal |
| invariable loan | סבבה *sababa* | סבבה | סבבה | סבבה | cool, fine (slang, Arabic) |

Note the ambiguity of יפה: *yafe* (M) and *yafa* (F) are spelled alike, like מורה above.

### 7.3 Denominal adjectives in *-i*

The suffix *-i* (F *-it*, M.PL *-iyim*, F.PL *-iyot*) makes an adjective from almost any noun, including loans and proper names: *bayit* → *beyti* 'homely, domestic', *olam* → *olami* 'worldly, global', *mada* → *mada'i* 'scientific', *politika* → *politi* 'political', *amerika* → *amerika'i* 'American', *sport* → *sportívi* 'sporty'. It is the most productive adjective-forming process (Bolozky 1999). Ethnic and national adjectives also serve as nouns (*yisre'eli* 'an Israeli').

### 7.4 Comparison

| Degree | Construction | Example |
|---|---|---|
| comparative | יותר *yoter* 'more' + ADJ (+ מ- / מאשר) | *dani yoter gavóha mi-yosi* 'Dani is taller than Yossi' |
| | ADJ + יותר (formal order) | *gavóha yoter* |
| inferior | פחות *pakhot* 'less' | *ze pakhot yakar* 'this is less expensive' |
| equal | כמו *kmo*, -כ *ke-*, באותה מידה | *hu gavóha kmo aba shelo* 'he is as tall as his father' |
| superlative (colloquial) | הכי *ha-khi* + ADJ | *ha-yéled ha-khi gavóha ba-kita* 'the tallest boy in the class' |
| superlative (formal) | ADJ + ביותר *be-yoter* | *ha-yéled ha-gavóha be-yoter* |
| superlative (literary) | construct ADJ + DEF-PL noun | *gdol ha-meshorerim* 'the greatest of poets' |
| elative | מאוד *me'od* 'very', ממש *mamash*, נורא *nora*, הכי (absolute) | *ze me'od yafe* / *yafe me'od* 'very beautiful'; *ze ha-khi!* (slang) 'it's the best!' |

```
המבחן היה יותר קשה ממה שחשבתי.
ha-mivkhan haya yoter kashe mi-ma she-khasháavti
DEF-exam be.PST.3SG.M more hard.M from-what that-think.PST.1SG
'The exam was harder than I thought.'
```

```
זה הסרט הכי טוב שראיתי השנה.
ze ha-séret ha-khi tov she-ra'íti ha-shana
this DEF-film DEF-most good that-see.PST.1SG DEF-year
'This is the best film I've seen this year.'
```

UD: in the IAHLT treebanks (2.18) הכי is one word, `ADV`, attached as `advmod` (30 tokens), and ביותר likewise (`ADV`, `advmod`, 103 tokens); counted 2026-10-17.

### 7.5 Adjectives as nouns and adverbs

- With the article an adjective is a noun phrase: *ha-gadol* 'the big one', *ha-khadashim* 'the new ones', *ha-tov ve-ha-ra* 'good and evil'.
- Without a noun in a definite context: *ani rotse et ha-adom* 'I want the red one'.
- The masculine singular adjective serves as a manner adverb: *hi medabéret yafe* 'she speaks nicely', *hu ratz maher* 'he ran fast', *tov* 'well'. Formal Hebrew prefers a prepositional phrase: *be-ofen yafe*, *bi-mhirut* 'quickly'.
- Participles are adjectives, nouns and present-tense verbs at once (*shomer* 'guarding / guard'; *mevugar* 'adult'): see [05-verbs.md](05-verbs.md).

---

## 8. Pronouns

### 8.1 Personal pronouns

| Person | Pronoun | Transliteration | Notes |
|---|---|---|---|
| 1SG | אני | *ani* | literary אנוכי *anokhi* |
| 2SG.M | אתה | *ata* | |
| 2SG.F | את | *at* | homograph of the accusative marker את *et* |
| 3SG.M | הוא | *hu* | |
| 3SG.F | היא | *hi* | |
| 1PL | אנחנו | *anákhnu* | formal אנו *ánu* |
| 2PL.M | אתם | *atem* | |
| 2PL.F | אתן | *aten* | colloquially often replaced by *atem* |
| 3PL.M | הם | *hem* | literary המה *héma* |
| 3PL.F | הן | *hen* | colloquially often replaced by *hem*; literary הנה *héna* |

**Feminine plural.** The feminine plural pronouns *aten*, *hen*, the 2PL.F and 3PL.F verb forms of the future (*tikhtóvna*) and imperative (*któvna*), and the feminine plural suffixes (*-khen*, *-an*) are in retreat in speech. Most speakers use the masculine forms for female groups: *atem*, *hem*, *tikhtevu*, *shelakhem* (Glinert 2017; Wikipedia, *Modern Hebrew grammar*). The future and imperative feminine plurals are effectively gone from speech and survive only in high literary and religious registers; *aten* and *hen* are still used by many speakers, especially in careful speech and writing (the Academy treats them as standard). For a parser: expect *hem* with feminine reference in speech transcripts, and *hen* in news.

**Use.** Subject pronouns are needed in the present tense (participle has no person) and in the 3rd person past and future; in the 1st and 2nd person past they are optional, since the verb ending shows the person (*katávti* 'I wrote'), and are often dropped in formal writing and kept in speech for emphasis or habit (*ani katávti*). See [07-syntax.md](07-syntax.md) on pro-drop.

**Pronouns as copula.** The 3rd person pronouns *hu*, *hi*, *hem*, *hen* serve as a present-tense copula in identity and classificatory sentences: *dani hu ha-menahel* 'Dani is the manager'. UD tags them `AUX` with relation `cop` (see [07-syntax.md](07-syntax.md) and [11-ud-analysis.md](11-ud-analysis.md)).

### 8.2 Demonstratives

| | Near ('this, these') | Far ('that, those') |
|---|---|---|
| M.SG | זה *ze* | ההוא *ha-hu* |
| F.SG | זאת *zot*, זו *zo* | ההיא *ha-hi* |
| PL | אלה *éle*, אלו *élu* | ההם *ha-hem* (M), ההן *ha-hen* (F) |

As **determiners** they follow the noun, and take the article like an adjective when the noun is definite:

```
הספר הזה        הבחורה הזאת        האנשים האלה        הבית ההוא
ha-séfer ha-ze   ha-bakhura ha-zot   ha-anashim ha-éle   ha-báyit ha-hu
'this book'      'this girl'         'these people'      'that house'
```

Formal and bureaucratic Hebrew can drop the articles: *séfer ze* 'this book', *be-mikre ze* 'in this case', *ba-shana zo* 'this year'. *zo* is more formal than *zot*; *élu* and *éle* are both standard, *élu* slightly more colloquial, and *éle* the Academy's preference in writing.

As **pronouns** they stand alone and come first in presentational sentences:

```
זה הבית שלי.
ze ha-báyit sheli
this DEF-house of.1SG
'This is my house.'
```

```
זאת אחותי.
zot akhoti
this.F sister.POSS.1SG
'This is my sister.'
```

**Invariant *ze*.** In speech *ze* is a neuter pronoun for situations, clauses and things not yet named: *ma ze?* 'what is it?', *ze lo meshane* 'it doesn't matter', *ze kashe* 'it's hard'. In speech it also works as a copula between a subject and a predicate that is a clause, an infinitive or an adjective with generic meaning: *ha-be'aya ze she-ein li zman* 'the problem is that I don't have time', *lishon ze kef* 'sleeping is fun'. The normative copula there would be *hi* (*ha-be'aya hi she-...*). See [07-syntax.md](07-syntax.md).

**Slang and discourse uses**: *ze ha-khayim* 'that's life'; *kaze* (*ke-ze*, 'like this') as a filler 'like' (*hu kaze lo hevin* 'he, like, didn't get it'); *ze lo ze* 'it's not it'. See [09-pragmatics-politeness.md](09-pragmatics-politeness.md).

### 8.3 Reflexive and emphatic עצם

The reflexive is the noun עצם *étsem* 'bone, essence' with a possessive suffix:

| Person | Form | | Person | Form |
|---|---|---|---|---|
| 1SG | עצמי *atsmi* | | 1PL | עצמנו *atsménu* |
| 2SG.M | עצמך *atsmekha* | | 2PL.M | עצמכם *atsmekhem* |
| 2SG.F | עצמך *atsmekh* | | 2PL.F | עצמכן *atsmekhen* |
| 3SG.M | עצמו *atsmo* | | 3PL.M | עצמם *atsmam* |
| 3SG.F | עצמה *atsma* | | 3PL.F | עצמן *atsman* |

Uses:

```
הוא ראה את עצמו במראה.
hu ra'a et atsmo ba-mar'a
he see.PST.3SG.M ACC self.POSS.3SG.M in.DEF-mirror
'He saw himself in the mirror.'
```

```
המנהל עצמו הגיע.
ha-menahel atsmo higía
DEF-manager self.POSS.3SG.M arrive.PST.3SG.M
'The manager himself came.'
```

- *be-atsmi* 'by myself, personally'; *me-atsmo* 'by itself, spontaneously';
- the plural is often colloquially levelled to the masculine (*hen atsmam*).
- Many reflexive meanings are expressed by the verb pattern *hitpa'el* instead (*hitlabesh* 'he dressed (himself)'); see [05-verbs.md](05-verbs.md) and Kastner (2017) on reflexive verbs.

**Alone**: לבד *levad* with suffixes, *levadi*, *levadkha*, *levado* …, means 'alone'; in speech the bare *levad* is normal for all persons (*ani levad* 'I'm alone').

### 8.4 Reciprocals

The reciprocal is a doubled demonstrative, with the preposition the verb needs between the two parts:

```
הם עוזרים זה לזה.
hem ozrim ze la-ze
they help.PRS.M.PL this to-this
'They help each other.'
```

| Form | Use |
|---|---|
| זה את זה *ze et ze* | M (and mixed): *hem ohavim ze et ze* 'they love each other' |
| זו את זו *zo et zo* | F: *hen makirot zo et zo* |
| אלה את אלה *éle et éle* | groups of groups |
| זה עם זה, זה על זה, זה מזה | with other prepositions |
| אחד את השני *ekhad et ha-sheni* | colloquial: 'one the other' |
| איש את רעהו *ish et re'ehu* | literary, Biblical |

The *hitpa'el* also forms reciprocal verbs: *hitkatvu* 'they corresponded', *hitnashku* 'they kissed (each other)'.

### 8.5 Interrogatives

| Word | Meaning | Note |
|---|---|---|
| מי *mi* | who | *shel mi* 'whose', *le-mi* 'to whom', *et mi* 'whom' |
| מה *ma* | what | also exclamatory *ma yafe!* 'how beautiful!'; colloquial *ma* = 'why' (*ma ata tsoek?* 'why are you shouting?') |
| איפה *éyfo* | where | formal היכן *heykhan* |
| לאן *le'an* | where to | |
| מאיפה *me-éyfo* | where from | formal מאין *me-áyin* |
| מתי *matay* | when | |
| איך *eykh* | how | formal כיצד *keytsad* |
| למה *láma* | why | formal מדוע *madúa* |
| כמה *káma* | how much, how many | also 'some, a few' (*káma anashim* 'a few people') |
| איזה, איזו, אילו *éyze*, *éyzo*, *éylu* | which | agreement M.SG, F.SG, PL; in speech *éyze* for all |
| האם *ha-im* | yes/no question particle | formal; speech uses intonation |

Interrogatives normally come first; in echo questions and speech they can stay in place (*halakhta le'an?*). See [07-syntax.md](07-syntax.md).

### 8.6 Indefinite and negative pronouns

Indefinites are formed with the suffix ־שהו *-shehu* (from *she-* + *hu*):

| Word | Meaning |
|---|---|
| מישהו *míshehu*, מישהי *míshehi* (F) | someone |
| משהו *máshehu* | something; colloquially also 'a bit, very' (*hi máshehu máshehu* 'she's something else') |
| איפשהו *éyfoshehu* | somewhere |
| מתישהו *matáyshehu* | sometime |
| איכשהו *éykhshehu* | somehow |
| איזשהו, איזושהי, אילושהם *éyzeshehu*, *éyzoshehi*, *éylushehem* | some, any |

Negative words require the verbal negation *lo* or *eyn* in the same clause — Hebrew has negative concord:

| Word | Meaning | Example |
|---|---|---|
| אף אחד *af ekhad*, אף אחת *af akhat* | nobody | *af ekhad lo ba* 'nobody came' |
| כלום *klum* | nothing | *lo ra'íti klum* 'I didn't see anything' |
| שום דבר *shum davar* | nothing | *shum davar lo kara* 'nothing happened' |
| אף פעם *af pá'am* | never | *af pá'am lo hayíti sham* 'I've never been there' |
| בשום מקום *be-shum makom* | nowhere | |
| שום *shum* + noun | no | *eyn li shum ra'ayon* 'I have no idea' |

*klum* alone answers a question: *ma kara? — klum* 'what happened? — nothing'. *af* is 'not even' before other nouns: *af mila* 'not a word'.

### 8.7 Universal quantifier כל

כל *kol* 'all, every' is a construct-like quantifier:

- *kol yéled* 'every child' (singular indefinite);
- *kol ha-yeladim* 'all the children';
- *kol ha-yom* 'all day', *kol yom* 'every day';
- *ha-kol* 'everything';
- with suffixes, 'all of': *kulánu* 'all of us', *kulkhem*, *kulam* 'everyone, all of them', *kulo* 'all of it (M)', *kula* (F).

| Person | Form | | Person | Form |
|---|---|---|---|---|
| 1SG | כולי *kuli* | | 1PL | כולנו *kulánu* |
| 2SG.M | כולך *kulkha* | | 2PL.M | כולכם *kulkhem* |
| 2SG.F | כולך *kulekh* | | 2PL.F | כולכן *kulkhen* |
| 3SG.M | כולו *kulo* | | 3PL.M | כולם *kulam* |
| 3SG.F | כולה *kula* | | 3PL.F | כולן *kulan* |

Netzer and Elhadad (1998) describe how such quantifiers, numerals and the article combine and order in the Hebrew noun phrase (*kol shlóshet ha-yeladim ha-éle* 'all these three children'), noting that definiteness is marked in several places at once.

---

## 9. Numerals

### 9.1 Two sets and the "polarity"

Hebrew cardinal numbers 1–19 have two forms. Confusingly for learners, the forms ending in *-a* go with **masculine** nouns, and the shorter forms without *-a* go with **feminine** nouns. This is the Semitic gender "polarity" (Wikipedia, *Hebrew numerals*; Glinert 2017).

| # | With F nouns | With M nouns | F construct | M construct |
|---|---|---|---|---|
| 1 | אחת *akhat* | אחד *ekhad* | אחת *akhat* | אחד *akhad* |
| 2 | שתיים *shtáyim* | שניים *shnáyim* | שתי *shtey* | שני *shney* |
| 3 | שלוש *shalosh* | שלושה *shlosha* | שלוש *shlosh* | שלושת *shlóshet* |
| 4 | ארבע *árba* | ארבעה *arba'a* | ארבע *árba* | ארבעת *arbá'at* |
| 5 | חמש *khamesh* | חמישה *khamisha* | חמש *khamesh* | חמשת *khaméshet* |
| 6 | שש *shesh* | שישה *shisha* | שש *shesh* | ששת *shéshet* |
| 7 | שבע *shéva* | שבעה *shiv'a* | שבע *shva* | שבעת *shiv'at* |
| 8 | שמונה *shmóne* | שמונה *shmona* | שמונה *shmóne* | שמונת *shmonat* |
| 9 | תשע *tésha* | תשעה *tish'a* | תשע *tsha* | תשעת *tish'at* |
| 10 | עשר *éser* | עשרה *asara* | עשר *éser* | עשרת *aséret* |

- 'One' follows the noun as an adjective: *yéled ekhad* 'one boy', *yalda akhat*.
- 2 has a special form before nouns (*shney yeladim*, *shtey banot*); *shnáyim*/*shtáyim* stand alone (*ha-shnáyim*, 'the two of them').
- 3 and up come before the noun: *shlosha yeladim* 'three boys', *shalosh banot* 'three girls'.
- 8: the two forms are spelled the same (שמונה) and differ only in the final vowel.
- The feminine forms are the ones used for **counting and arithmetic**: *ákhat, shtáyim, shalosh, árba* …; *shtáyim ve-shalosh ze khamesh* '2 + 3 = 5'.

```
יש לי שלושה אחים ושתי אחיות.
yesh li shlosha akhim u-shtey akhayot
EXIST to.1SG three.M brother.PL and-two.F.CS sister.PL
'I have three brothers and two sisters.'
```

### 9.2 Colloquial levelling

In speech many speakers use the feminine (short) forms with masculine nouns as well: *shalosh shkalim* 'three shekels', *árba yeladim* for normative *arba'a yeladim*. The levelling is strongest with numbers above 2 and in quick, informal speech; the masculine *-a* forms remain in writing, the media and careful speech (Glinert 2017; Wikipedia, *Modern Hebrew grammar*). For UD annotation this matters little (the numeral is `NUM` with `Gender` matching its form, not its noun), but for agreement checking and generation it does.

### 9.3 Construct numerals: 'the three children'

A definite counted noun takes the construct form of the numeral:

```
שלושת הילדים        שלוש הבנות        שני הספרים        שתי האחיות
shlóshet ha-yeladim  shlosh ha-banot    shney ha-sfarim    shtey ha-akhayot
'the three boys'     'the three girls'  'the two books'    'the two sisters'
```

The construct numeral can also stand with a possessive suffix: *shlosht-am* 'the three of them (M)', *shnéyhem* 'both of them (M)', *shtéyhen* 'both of them (F)'; colloquially *shlóshtam*, *shnéyhem*, or *ha-shlosha*.

### 9.4 Teens and tens

| # | With F nouns | With M nouns |
|---|---|---|
| 11 | אחת־עשרה *akhat-esre* | אחד־עשר *akhad-asar* |
| 12 | שתים־עשרה *shteym-esre* | שנים־עשר *shneym-asar* |
| 13 | שלוש־עשרה *shlosh-esre* | שלושה־עשר *shlosha-asar* |
| 14 | ארבע־עשרה *arba-esre* | ארבעה־עשר *arba'a-asar* |
| 15 | חמש־עשרה *khamesh-esre* | חמישה־עשר *khamisha-asar* |
| 16 | שש־עשרה *shesh-esre* | שישה־עשר *shisha-asar* |
| 17 | שבע־עשרה *shva-esre* | שבעה־עשר *shiv'a-asar* |
| 18 | שמונה־עשרה *shmone-esre* | שמונה־עשר *shmona-asar* |
| 19 | תשע־עשרה *tsha-esre* | תשעה־עשר *tish'a-asar* |

In speech the feminine teen forms (*shlosh-esre*, *arba-esre*) are common with all nouns; the masculine *-asar* forms survive mostly in careful use.

| Tens | | Hundreds | | Thousands | |
|---|---|---|---|---|---|
| 20 | עשרים *esrim* | 100 | מאה *me'a* | 1 000 | אלף *élef* |
| 30 | שלושים *shloshim* | 200 | מאתיים *matáyim* | 2 000 | אלפיים *alpáyim* |
| 40 | ארבעים *arba'im* | 300 | שלוש מאות *shlosh me'ot* | 3 000 | שלושת אלפים *shlóshet alafim* |
| 50 | חמישים *khamishim* | 400 | ארבע מאות *arba me'ot* | 10 000 | עשרת אלפים *aséret alafim* |
| 60 | שישים *shishim* | 500 | חמש מאות *khamesh me'ot* | 100 000 | מאה אלף *me'a élef* |
| 70 | שבעים *shiv'im* | 600 | שש מאות | 1 000 000 | מיליון *milyon* |
| 80 | שמונים *shmonim* | 700 | שבע מאות | 10⁹ | מיליארד *milyard* |
| 90 | תשעים *tish'im* | 800 | שמונה מאות | | |
| | | 900 | תשע מאות | | |

Tens have one form for both genders. Compound numbers join the last element with ו *ve-* (*u-* before labials and shva): *esrim ve-khamesh* (F) / *esrim ve-khamisha* (M) '25', *me'a ve-esrim* '120', *alpáyim ve-esrim ve-shesh* '2026'. *Me'a* is feminine (hence *shlosh me'ot*), *élef* masculine (hence *shlóshet alafim*).

**Singular after large numbers.** After numbers above ten, some common nouns can stay singular: *esrim shana* '20 years', *me'a ish* '100 men', *khamishim shékel*, *élef yom*. This is a literary pattern that survives with a fixed set of nouns (*shana*, *yom*, *ish*, *shékel*, *dolar*, *élef*); with most nouns the plural is used (Glinert 2017; not checked against a corpus here).

### 9.5 Ordinals

| # | M | F |
|---|---|---|
| 1st | ראשון *rishon* | ראשונה *rishona* |
| 2nd | שני *sheni* | שנייה *shniya* |
| 3rd | שלישי *shlishi* | שלישית *shlishit* |
| 4th | רביעי *revi'i* | רביעית *revi'it* |
| 5th | חמישי *khamishi* | חמישית *khamishit* |
| 6th | שישי *shishi* | שישית *shishit* |
| 7th | שביעי *shvi'i* | שביעית *shvi'it* |
| 8th | שמיני *shmini* | שמינית *shminit* |
| 9th | תשיעי *tshi'i* | תשיעית *tshi'it* |
| 10th | עשירי *asiri* | עשירית *asirit* |

Ordinals are adjectives and follow the noun: *ha-yom ha-rishon* 'the first day', *ha-pá'am ha-shniya* 'the second time'. Above ten, the cardinal is used after the noun with the article: *ha-me'a ha-esrim* 'the twentieth century' (lit. 'the hundred the twenty'), *ha-koma ha-shlosh-esre* 'the thirteenth floor'. The feminine ordinals 3–10 double as fractions: *shlishit* 'a third', *revi'it* 'a quarter', *asirit* 'a tenth'; 'half' is חצי *khétsi* (formal מחצית *makhatsit*).

The days of the week are ordinals: *yom rishon* 'Sunday', *yom sheni* 'Monday', … *yom shishi* 'Friday', and *shabat* 'Saturday'.

### 9.6 Dates, money, time, age

**Dates.** The day of the month is a masculine cardinal (agreeing with an implied *yom* 'day') with the article, then *be-* + month: *ha-khamisha be-yuni* 'the fifth of June', *ha-esrim ve-arba'a be-mars* or colloquially *esrim ve-árba le-mars*. Years are read in full (*alpáyim ve-esrim ve-shesh*) or, in speech, abbreviated (*shnat esrim ve-shesh* '(20)26'). The Hebrew calendar uses letters as numbers: *ha-tet-vav be-shvat* 'the 15th of Shvat' (ט"ו בשבט), year *tashpa"v* (תשפ"ו); see [02-script-and-spelling.md](02-script-and-spelling.md) on gematria.

**Money.** שקל *shékel* (M), plural שקלים *shkalim*; אגורה *agora* (F), plural אגורות *agorot*: *shney shkalim*, *khamisha shkalim* (normative) or *khamesh shkalim*/*khamesh shékel* (speech), *esrim shékel*. The new shekel is written ש"ח (*shékel khadash*), read *shékel* or *shkalim*.

**Time.** Hours are feminine (agreeing with *sha'a* 'hour'): *ha-sha'a shalosh* 'it's three o'clock', *be-árba* 'at four', *shalosh va-khétsi* 'half past three', *árba va-réva* 'quarter past four', *réva le-khamesh* 'quarter to five', *éser (dakot) le-shesh* 'ten to six', *shmóne ve-esrim* '8:20'. Twelve o'clock is *shteym-esre*. *Ba-bóker*, *ba-tsohoráyim*, *ba-érev*, *ba-láyla* mark the part of the day.

**Age.** *ben* 'son' / *bat* 'daughter' + number: *hu ben shlosh-esre* 'he is 13', *hi bat esrim ve-árba* 'she is 24' (feminine forms with *ben* are usual because *shana* 'year' is understood); *ben káma ata?* 'how old are you (M)?'.

```
הפגישה בשעה שלוש וחצי, ביום שלישי.
ha-pgisha be-sha'a shalosh va-khétsi be-yom shlishi
DEF-meeting in-hour three.F and-half in-day third.M
'The meeting is at half past three, on Tuesday.'
```

### 9.7 Numerals in UD

Cardinal numerals are `NUM` with `NumType=Card` and a `Gender` matching their form (not their noun), attached to the noun as `nummod`. Construct numerals carry `Definite=Cons`. Ordinals are `ADJ` with `NumType=Ord` (or `NUM`, depending on the treebank). Compound numbers (*esrim ve-khamesh*) are joined by `compound` or `conj` in the IAHLT data; the Mova converter for Biblical Hebrew uses `flat` for the parts of a compound number (tools/macula/README.md). See [11-ud-analysis.md](11-ud-analysis.md).

---

## 10. Summary for a parser

| Problem | What decides it |
|---|---|
| gender of a noun with no ending | the lexicon |
| *-a* vs *-e* (מורה, יפה) in unpointed text | agreement with verb or adjective |
| construct vs absolute masculine singular (בית, ספר) | a following noun without the article on the first noun |
| which noun an adjective after a construct modifies | gender/number agreement; else ambiguous |
| definite object | את before it; ה, possessive suffix, proper name, definite construct |
| noun + suffix vs other readings (ספרו) | context; morphological analysis (Hspell, Dicta) |
| bound possessive vs של | register; both map to `nmod:poss` in UD |
| masculine vs feminine numerals | the noun's gender (normative); speech often levels to the short forms |

---

## References

- Bolozky, Shmuel. 1999. *Measuring Productivity in Word Formation: The Case of Israeli Hebrew.* Leiden: Brill. (Not held; cited for the productivity of *-i* and linear suffixes.)
- Bruening, Benjamin. 2022. "Hebrew nominals do not require functional structure above the NP." *Glossa* 7(1). https://doi.org/10.16995/glossa.5763 — local: $RESEARCH_RAW/lingbulk-2022-bruening-hebrew-nominals-do-not-require/ (CC BY 4.0).
- Coffin, Edna Amir and Shmuel Bolozky. 2005. *A Reference Grammar of Modern Hebrew.* Cambridge University Press. (Not held; listed in $RESEARCH_RAW/hebgram/grammars.tsv as "list only".)
- Dahan Netzer, Yael and Michael Elhadad. 1998. "Generating Determiners and Quantifiers in Hebrew." Workshop on Computational Approaches to Semitic Languages. https://aclanthology.org/ — local: $RESEARCH_RAW/aclbulk-1998-netzer-generating-determiners-quantifiers-hebrew/.
- Dahan Netzer, Yael and Michael Elhadad. 1999. "Bilingual Hebrew-English Generation of Possessives and Partitives: Raising the Input Abstraction Level." ACL 1999. local: $RESEARCH_RAW/aclbulk-1999-netzer-bilingual-hebrew-english-generation-possessives/.
- Glinert, Lewis. 2017. *Modern Hebrew: An Essential Grammar.* 4th ed. Routledge. (Not held; cited for standard facts of gender, numerals and register.)
- Hacohen, Aviya, Olga Kagan and Dana Plaut. 2021. "Differential Object Marking in Modern Hebrew: Definiteness and Partitivity." *Glossa* 6(1): 148. https://doi.org/10.16995/glossa.5729 — local: $RESEARCH_RAW/lingbulk-2021-hacohen-differential-object-marking-modern-hebrew/.
- Itai, Alon, Shuly Wintner and Shlomo Yona. 2006. "A Computational Lexicon of Contemporary Hebrew." LREC 2006. local: $RESEARCH_RAW/lingbulk-2006-itai-computational-lexicon-contemporary-hebrew/.
- Kastner, Itamar. 2017. "Reflexive verbs in Hebrew: Deep unaccusativity meets lexical semantics." *Glossa* 2(1). local: $RESEARCH_RAW/lingbulk-2017-kastner-reflexive-verbs-hebrew-deep-unaccusativity/.
- Preminger, Omer. 2020. (Cited via Bruening 2022 for the construct example *nitskhon ha-miflaga al yeriveha*.)
- Seker, Amit et al. 2022. "AlephBERT: Language Model Pre-training and Evaluation from Sub-Word to Sentence Level." ACL 2022. https://aclanthology.org/2022.acl-long.4/ — local: $RESEARCH_RAW/acl-2022-seker-alephbert-language-model-pre-training/.
- Tsarfaty, Reut, Amit Seker, Shoval Sadde and Stav Klein. 2019. "What's Wrong with Hebrew NLP? And How to Make it Right." EMNLP-IJCNLP 2019 demos. https://aclanthology.org/D19-3044/ — local: $RESEARCH_RAW/aclbulk-2019-tsarfaty-what-s-wrong-hebrew-nlp/.
- Universal Dependencies 2.18: UD_Hebrew-IAHLTwiki and UD_Hebrew-IAHLTknesset (CC BY-SA 4.0). https://universaldependencies.org/treebanks/he_iahltwiki/ — local: $RESEARCH_RAW/lexres/treebanks/ud-2.18/. Counts in this chapter: streamed over train+dev+test on 2026-10-17. Example sentences quoted with their sent_id.
- Universal Dependencies, Hebrew guidelines (`_he/dep/compound-smixut.md`, `_he/dep/case-gen.md`). https://universaldependencies.org/he/ — local: $RESEARCH_RAW/hebgram/sa/ud-docs-he-hbo/_he/.
- Wikibooks contributors. *דקדוק העברית החדשה* (A grammar of Modern Hebrew), page "שם" (noun). https://he.wikibooks.org/wiki/דקדוק_העברית_החדשה — local: $RESEARCH_RAW/hebgram/sa/wikibooks-he-dikduk-haivrit-hahadasha/ (CC BY-SA; paraphrased, not copied).
- Wikipedia contributors. "Modern Hebrew grammar." https://en.wikipedia.org/wiki/Modern_Hebrew_grammar (consulted 2026-10-17; CC BY-SA; paraphrased).
- Wikipedia contributors. "Hebrew numerals." https://en.wikipedia.org/wiki/Hebrew_numerals.
- Wiktionary (via kaikki.org). English Wiktionary Hebrew entries with inflection tables. local: $RESEARCH_RAW/hebgram/sa/kaikki-enwiktionary-hebrew-aramaic/kaikki.org-dictionary-Hebrew.jsonl (CC BY-SA; used to check construct and suffixed forms).
- Academy of the Hebrew Language. 2017. *כללי הכתיב חסר הניקוד* (Rules for unpointed spelling). https://hebrew-academy.org.il/topic/hahlatot/missingvocalizationspelling/
