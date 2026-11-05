# 2. Script and spelling

This chapter covers how Modern Hebrew is written. It describes the 22 letters and their five final forms, the vowel points (*niqqud*) and the other diacritics, and the two spelling systems: pointed spelling, and the full spelling without points (*ktiv male*) used in almost every everyday text. It covers the Academy of the Hebrew Language's rules for full spelling, the geresh and gershayim, abbreviations, numbers written with letters, punctuation, Unicode and right-to-left layout, and how ambiguous unpointed text is in practice. It closes with the romanization standards and **the transliteration used throughout this manual**.

Index: [README.md](README.md) · previous: [01-overview.md](01-overview.md) · next: [03-phonology.md](03-phonology.md) · related: [04-nouns-adjectives-pronouns.md](04-nouns-adjectives-pronouns.md), [11-ud-analysis.md](11-ud-analysis.md)

---

## 2.1 The alphabet

Modern Hebrew uses the Hebrew "square script" (*ktav ashuri*, כְּתָב אַשּׁוּרִי), which descends from the Imperial Aramaic script. It is an **abjad**: the letters write consonants, and vowels are either left out or shown by a few letters doing double duty (*matres lectionis*, אִמּוֹת קְרִיאָה, *imot kri'a*), or by the optional points. It is written **right to left**. There is no upper or lower case. Words are separated by spaces, but the article, some prepositions and conjunctions, and the relative particle are written joined to the next word (see §2.10 and [11-ud-analysis.md](11-ud-analysis.md)).

The table gives each letter's name as it is said today (the Academy's spelling of the name in brackets), its value in standard Israeli speech, its traditional numerical value, and its final form where it has one. "Dagesh" means the letter written with the inner dot (§2.3.1).

| # | Letter | Final | Name (Israeli) | Modern value(s) | Historical value (Tiberian, simplified) | Number |
|---:|:---:|:---:|---|---|---|---:|
| 1 | א | | *álef* (אָלֶף) | glottal stop [ʔ] or nothing; vowel carrier | /ʔ/ | 1 |
| 2 | ב | | *bet* / *vet* (בֵּית) | בּ [b], ב [v] | /b/, spirant [β] | 2 |
| 3 | ג | | *gímel* (גִּימֶל) | [g]; ג׳ = [dʒ] | /g/, spirant [ɣ] | 3 |
| 4 | ד | | *dálet* (דָּלֶת) | [d]; ד׳ = [ð] in transcription | /d/, spirant [ð] | 4 |
| 5 | ה | | *he* (הֵא) | [h] (often dropped); silent at word end | /h/ | 5 |
| 6 | ו | | *vav* (וָו) | consonant [v]; vowel letter for /o/, /u/ | /w/ | 6 |
| 7 | ז | | *záyin* (זַיִן) | [z]; ז׳ = [ʒ] | /z/ | 7 |
| 8 | ח | | *khet* (חֵית) | [χ]; [ħ] for some Mizrahi speakers | /ħ/ | 8 |
| 9 | ט | | *tet* (טֵית) | [t] | emphatic /tˤ/ | 9 |
| 10 | י | | *yod* / *yud* (יוֹד) | consonant [j]; vowel letter for /i/, /e/, /ej/ | /j/ | 10 |
| 11 | כ | ך | *kaf* / *khaf* (כַּף) | כּ [k], כ ך [χ] | /k/, spirant [x] | 20 |
| 12 | ל | | *lámed* (לָמֶד) | [l] | /l/ | 30 |
| 13 | מ | ם | *mem* (מֵם) | [m] | /m/ | 40 |
| 14 | נ | ן | *nun* (נוּן) | [n] | /n/ | 50 |
| 15 | ס | | *sámekh* (סָמֶךְ) | [s] | /s/ | 60 |
| 16 | ע | | *áyin* (עַיִן) | [ʔ] or nothing; [ʕ] for some Mizrahi speakers | /ʕ/ | 70 |
| 17 | פ | ף | *pe* / *fe* (פֵּא) | פּ [p], פ ף [f] | /p/, spirant [ɸ~f] | 80 |
| 18 | צ | ץ | *tsádi* (צָדִי) | [ts]; צ׳ = [tʃ] | emphatic /sˤ/ | 90 |
| 19 | ק | | *kuf* (קוֹף) | [k] | /q/ (uvular or emphatic) | 100 |
| 20 | ר | | *resh* (רֵישׁ) | [ʁ] (uvular); alveolar [r] for some speakers | /r/ | 200 |
| 21 | ש | | *shin* (שִׁין) / *sin* (שִׂין) | שׁ [ʃ], שׂ [s] | /ʃ/, /ɬ/~/s/ | 300 |
| 22 | ת | | *tav* (תָּו) | [t]; ת׳ = [θ] in transcription | /t/, spirant [θ] | 400 |

Sources for the modern values: Laufer (1999), Bolozky (1997), Asherov & Cohen (2019); the overview in Wikipedia "Hebrew alphabet" and "Modern Hebrew phonology". The historical column is a simplification of the Tiberian reading tradition as described by Khan (2020). [03-phonology.md](03-phonology.md) gives the sound system in full.

### 2.1.1 Final forms

Five letters have a separate shape at the end of a word: כ→ך, מ→ם, נ→ן, פ→ף, צ→ץ (mnemonic *mantsepakh*, מנצפ״ך). The final form is a matter of position, not of sound. ך is [χ] (*khaf sofit*) and is written with a dagesh, ךּ [k], only in a few words and in pointed text. ף is [f] in native words. Loanwords and slang ending in [p] or [ts]/[tʃ] are a known problem area:

- **[p] at the end.** Some writers use the ordinary final ף, and the reader has to know it is [p]. Others keep a non-final פ at the end of the word (שופ or שוף "shop", צ׳יפ or צ׳יף "chip"), and this is common in brand names. Pointed text can write ףּ (final pe with dagesh). Expect both spellings in data.
- **[tʃ] at the end.** Final [tʃ] is written either as ץ׳ or as a non-final צ׳ (e.g. סנדוויץ׳ / סנדוויצ׳ "sandwich"). Usage varies.

The variation is real and should be expected in data. A tokenizer or spell checker must not "correct" a non-final פ or צ at the end of a word in loanwords.

### 2.1.2 Letters with two values and letters with one value

Because historical sounds have merged (see [03-phonology.md §3.2.2](03-phonology.md#322-mergers-the-letters-that-sound-alike)), several **letters share a sound**, and three **letters have two sounds**:

| Sound | Letters | Example pair |
|---|---|---|
| [t] | ת, ט | תָּעָה *ta'a* "went astray" / טָעָה *ta'a* "erred" |
| [k] | כּ, ק | כָּרָה *kara* "dug" / קָרָה *kara* "happened" |
| [χ] | ח, כ | סָחַר *sakhar* "traded" / שָׂכַר *sakhar* "hired" (also ס/שׂ) |
| [v] | ב, ו | אָב *av* "father" (ב) / תָּו *tav* "note, sign" (ו): no common homophone pair, but the spelling of [v] must be learned word by word |
| [s] | ס, שׂ | סָם *sam* "drug" / שָׂם *sam* "put" |
| [ʔ] or zero | א, ע | אִם *im* "if" / עִם *im* "with" |

| Letter | Values |
|---|---|
| ב | [b] / [v] |
| כ | [k] / [χ] |
| פ | [p] / [f] |
| ש | [ʃ] (shin, dot on the right) / [s] (sin, dot on the left) |

In unpointed text, these letters cannot be told apart, so the reader must know the word. This is one main source of ambiguity (§2.9).

## 2.2 The vowel points (*niqqud*)

The points are a system of dots and dashes set below, inside or above the letters. The Tiberian Masoretes created them in the 8th–10th centuries CE to record a reading tradition of the Bible that had more vowel distinctions than Modern Hebrew (Khan 2020). Modern speech has five vowels (/a e i o u/, [03-phonology.md](03-phonology.md)), so several signs are now pronounced the same.

Today pointed text is used in:

- poetry and song lyrics;
- books for small children and early readers, and materials for learners (*ulpan*);
- the Bible and the prayer book;
- dictionaries, and occasionally to disambiguate a single word in otherwise unpointed text (*niqqud ezer*, "helping points");
- some official notices and signs aimed at new immigrants.

Everyday prose (newspapers, books for adults, the web, messages) is unpointed.

### 2.2.1 Vowel signs and their modern values

The table shows each sign on the base letter ב (shown in the "Shape" column), its traditional name, its traditional class, and its value in standard Israeli speech.

| Shape | Name | Traditional class | Modern value | Note |
|:---:|---|---|---|---|
| בַ | *patakh* (פַּתָּח) | short a | [a] | |
| בָ | *kamats gadol* (קָמָץ גָּדוֹל) | long a | [a] | |
| בָ | *kamats katan* (קָמָץ קָטָן) | short o | [o] | same shape as kamats gadol; only context (or a special sign in some editions) tells them apart, e.g. כָּל *kol* "all", חָכְמָה *khokhma* "wisdom" |
| בֵ | *tsere* (צֵירֵי) | long e | [e], often [ej] | with yod: בֵי |
| בֶ | *segol* (סֶגּוֹל) | short e | [e] | |
| בִ | *khirik* (חִירִיק) | i | [i] | "long" khirik = with yod: בִי |
| בֹ / בוֹ | *kholam khaser* / *kholam male* (חוֹלָם) | o | [o] | a dot above the left of the letter, or a dot over a vav: וֹ |
| בֻ | *kubuts* (קֻבּוּץ) | short u | [u] | |
| בוּ | *shuruk* (שׁוּרוּק) | long u | [u] | vav with a dot in it, in the same position as dagesh (§2.3.1) |
| בְ | *shva* (שְׁוָא) | none or reduced | [e] or nothing | see §2.2.2 |
| חֲ | *khataf patakh* (חֲטַף פַּתָּח) | reduced a | [a] | mainly under the gutturals א ה ח ע |
| אֱ | *khataf segol* (חֲטַף סֶגּוֹל) | reduced e | [e] | |
| חֳ | *khataf kamats* (חֲטַף קָמָץ) | reduced o | [o] | |

*Under modern pronunciation*, then, the vowel signs collapse into five groups: patakh + kamats gadol + khataf patakh = [a]; tsere + segol + khataf segol (+ some shva) = [e]; khirik = [i]; kholam + kamats katan + khataf kamats = [o]; kubuts + shuruk = [u]. The difference between kamats and patakh, or tsere and segol, survives only in spelling, and in the spelling rules that depend on it (e.g. tsere is a reason to write yod in some words; §2.4.4). Sources: Laufer (1999); Wikibooks *Dikduk ha-Ivrit ha-Hadasha*, "Vowels" page (Hebrew Wikibooks); Coffin & Bolozky (2005, ch. 1).

### 2.2.2 Shva: *na* and *nakh*

A shva under a letter means that the letter has no full vowel. The grammars distinguish:

- **shva nakh** (שְׁוָא נָח, "resting"): silent, ending a syllable, e.g. the shva under ס in יִסְגֹּר *yisgor* "he will close";
- **shva na** (שְׁוָא נָע, "moving"): in the Tiberian tradition a short vowel, e.g. under the first letter of a word, under a letter after another shva, under a doubled (dagesh forte) letter.

Modern speech does **not follow the traditional rules** consistently. A shva is said as [e] where the speaker needs a vowel to make the word pronounceable or where morphology requires it, and is silent elsewhere. Some typical outcomes (Bolozky 1997; Laufer 1999):

| Pointed | Traditional | Modern | Gloss |
|---|---|---|---|
| זְמַן | *zeman* | *zman* | "time" |
| שְׁמוֹנֶה | *shemone* | *shmone* | "eight" |
| בְּבַיִת | *bevayit* | *be-váyit* | "in a house" (prefix keeps [e]) |
| רְחוֹב | *reḥov* | *rekhov* | "street" (no [rχ] cluster at word start) |
| לְמַדְתֶּם | *lemadtem* | *lemadtem* | "you (pl.) learned" |
| כָּתְבוּ | *kotvu*/*katevu* | *katvu* | "they wrote" (Modern regularly *katvu*) |
| מְדַבֶּרֶת | *medaberet* | *medabéret* | "speaking (F)" (shva in a prefix, kept) |

Rules of thumb for Modern Hebrew:

1. A shva in a **prefix** (בְּ, כְּ, לְ, וְ, the participle prefix מְ) is said [e]: *be-*, *ke-*, *le-*, *ve-*, *me-*.
2. A shva on the **first of two consonants that cannot form an onset** is said [e] (§3.6): *lerakhem*, not **lrakhem*. The forbidden initial clusters include a first sonorant such as ר, ל, נ, מ or י in many words, and two consonants that are identical or differ only in voicing.
3. Elsewhere at the start of a word, a shva is usually **silent**: *zman*, *shmone*, *kfar*, *sfarim*, *tmuna*.
4. Inside a word, a shva after a closed syllable is silent: *mikhtav*, *yikhtov*.

The UD treebanks of Modern Hebrew are unpointed, so none of this is visible in their FORM fields. A speech or text-to-speech system, or a model that uses the points, has to deal with it.

## 2.3 Other diacritics

### 2.3.1 Dagesh

The **dagesh** (דָּגֵשׁ) is a dot inside a letter. In the Tiberian system it marked two things:

- **dagesh kal** ("light"): the stop value of ב ג ד כ פ ת (the *begadkefat* letters) at the start of a word or after a closed syllable;
- **dagesh khazak** ("strong"): a doubled (geminate) consonant, e.g. in the pi'el pattern (דִּבֵּר *diber* "spoke") or from an assimilated נ (יִפֹּל *yipol* "will fall").

In modern pronunciation, **gemination is gone**, and the stop/spirant split is audible only for בּ/ב, כּ/כ, פּ/פ. For the other letters, the dagesh is a spelling sign. In ordinary unpointed text, even the letters with two values carry no dagesh, so כ can be [k] or [χ] and פ can be [p] or [f] (but see §2.4.3: final ף is always [f] in native words).

Two other dot signs take the same place:

- **shuruk** וּ = [u] (a vav with a dot, which is a vowel, not dagesh);
- **mappiq** (מַפִּיק) in a final ה: הּ marks the ה as a consonant, not a vowel letter. Today it appears mainly in the 3rd person feminine possessive suffix: סִפְרָהּ *sifra(h)* "her book" vs. סִפְרָה *sifra* "digit, numeral"; לָהּ *la(h)* "to her" vs. לָה (no such word). The [h] is said in careful or formal speech and usually not in casual speech (Asherov & Cohen 2019), so the two forms are often homophones.

### 2.3.2 Shin dot and sin dot

שׁ (dot top right) is *shin* [ʃ]; שׂ (dot top left) is *sin* [s]. In unpointed text both are ש. Most ש in Modern Hebrew are shin. Common words with sin include שָׂם *sam* "put", שָׂמֵחַ *saméakh* "happy", יִשְׂרָאֵל *Yisra'el* "Israel", שָׂפָה *safa* "language", עֶשֶׂר *éser* "ten", שָׂדֶה *sade* "field" and מַעֲשֶׂה *ma'ase* "deed".

### 2.3.3 Rafe, meteg and the accents

The **rafe** (רָפֶה), a bar above a letter, marks the spirant value, and appears in some older printing and in Yiddish. The **meteg** (מֶתֶג), a small vertical bar under a letter, marks secondary stress and some vowel lengths. The cantillation **accents** (*te'amim*, טְעָמִים), which give stress and musical phrasing in the Bible, play no role in Modern Hebrew writing. They are encoded in Unicode (U+0591–U+05AF) and appear in Bible text in the same corpora that hold Modern Hebrew, so a pipeline should strip them. The same applies to the special characters U+05C0 (paseq), U+05C3 (sof pasuq) and U+05C6 (nun hafukha).

## 2.4 Spelling without points: *ktiv male*

### 2.4.1 Two systems

Hebrew has two spelling traditions:

- **ktiv menukad** (כְּתִיב מְנֻקָּד, "pointed spelling"), also called *ktiv khaser* (כְּתִיב חָסֵר, "defective spelling") when it is unpointed. Words are spelled with the minimum of vowel letters, as in the Bible and pointed texts, and the points carry the vowels. Example: שֻׁלְחָן "table", דִּבֵּר "spoke", חָכְמָה "wisdom".
- **ktiv male** (כְּתִיב מָלֵא, "full spelling"), the official name being *ha-ktiv she-eino menukad*, "spelling without points". Extra ו and י are added to help the reader where there are no points: שולחן, דיבר, חוכמה. Consonantal ו and י are doubled to tell them apart from vowel letters: תקווה *tikva* "hope", עניין *inyan* "matter".

All modern unpointed text is in some version of ktiv male. In practice, it is a **mixed system**: the Academy's rules are recommendations, newspapers and publishers have house styles, and individual writers vary. Both לחשוב and לחשב *lakhshov* "to think", מילה and מלה *mila* "word", and אשה and אישה *isha* "woman" are frequent. A lexicon or tokenizer for Modern Hebrew therefore needs **spelling variants** linked to one lemma. Hspell, for example, follows the Academy's rules strictly and accepts only the spellings they allow (Har'El & Kenigsberg, Hspell documentation), while the UD treebanks keep the spelling of the source text.

### 2.4.2 History of the rules

- **1947 (Tishrei 5708):** the Hebrew Language Committee (*Va'ad ha-Lashon*), the Academy's forerunner, publishes "rules for spelling without points".
- **1968–1993:** the Academy of the Hebrew Language, founded in 1953, revises and republishes the rules, and they become standard in education.
- **2002:** the Academy publishes the rules together with the punctuation rules (*Klalei ha-pisuk ve-klalei ha-ktiv she-eino menukad*).
- **2017 (Sivan 5777):** the Academy publishes revised **rules of full spelling** (*Klalei ha-ktiv ha-male*). The best-known change is the end of the 1947 exception that wrote אשה *isha* "woman" without yod: the standard spelling is now **אישה**. Other details changed as well.

Sources: Academy of the Hebrew Language, "Klalei ha-ktiv she-eino menukad / ha-ktiv ha-male" (hebrew-academy.org.il, the decisions section, and the PDF of the 2017 rules); Hebrew Wikipedia, "כתיב חסר ניקוד" (accessed 2026-10-17). The Academy's site refused automatic fetching on 2026-10-17 (HTTP 403), so the summary below follows the published rules as they are summarized in the Hebrew Wikipedia article and the grammars. Check the Academy PDF before relying on a fine detail.

### 2.4.3 The rules in outline

The rules are given by the **vowel in pointed spelling**, so a writer has to know the pointed form. That is why native writers also vary.

**(1) /u/ (kubuts or shuruk) is always written with ו.**

| Pointed | Full | Reading | Gloss |
|---|---|---|---|
| שֻׁלְחָן | שולחן | *shulkhan* | table |
| קֻפְסָה | קופסה | *kufsa* | box |
| כֻּלָּם | כולם | *kulam* | all of them |
| סֻכָּר | סוכר | *sukar* | sugar |
| הֻפַּל | הופל | *hupal* | was dropped |

**(2) /o/ (kholam, and kamats katan) is written with ו**, with a few exceptions.

| Pointed | Full | Reading | Gloss |
|---|---|---|---|
| בֹּקֶר | בוקר | *bóker* | morning |
| יִשְׁמֹר | ישמור | *yishmor* | he will guard |
| חָכְמָה | חוכמה | *khokhma* | wisdom |
| תָּכְנִית | תוכנית | *tokhnit* | plan, program |
| צָהֳרַיִם | צוהריים | *tsohoráyim* | noon |

The main exceptions are כָּל *kol* "all", written **כל**, and words where /o/ is already shown by א, such as רֹאשׁ → **ראש** *rosh* "head", לֹא → **לא** *lo* "no, not", זֹאת → **זאת** *zot* "this (F)" and צֹאן → **צאן** *tson* "sheep".

**(3) /i/ (khirik) is written with י**, except mainly where a **shva follows** (a closed syllable) or in certain fixed small words and prefixes.

| Pointed | Full | Reading | Gloss | Why |
|---|---|---|---|---|
| דִּבֵּר | דיבר | *diber* | spoke | khirik, open syllable → י |
| סִפּוּר | סיפור | *sipur* | story | → י |
| מִלָּה | מילה | *mila* | word | → י (מלה also seen) |
| יִשּׁוּב | יישוב | *yishuv* | settlement | → י, so יי at the start |
| מִכְתָּב | מכתב | *mikhtav* | letter | khirik before shva nakh → no י |
| שִׂמְחָה | שמחה | *simkha* | joy | before shva → no י |
| הִכְתִּיב | הכתיב | *hikhtiv* | dictated | hif'il prefix before shva → no י |
| אִם, עִם, מִן | אם, עם, מן | *im, im, min* | if, with, from | fixed small words |
| מִבַּיִת | מבית | *mi-báyit* | from a house | prefix מִ־ → no י |

**(4) /e/ (tsere, segol) is normally written without י.** The common exceptions are a group of words that traditionally have a yod (תֵּימָן *teman* "Yemen" → **תימן**, מֵיתָר *meitar* "string" → **מיתר**), and forms where the **tsere comes from compensatory lengthening** before a guttural or ר (pi'el forms whose middle root letter cannot double): תֵּאֵר *te'er* "described" → **תיאר**, תֵּאוּר *te'ur* "description" → **תיאור**, פֵּרוּשׁ *perush* "commentary" → **פירוש**, בֵּרֵךְ *berekh* "blessed" → **בירך**. Here the full spelling treats the tsere as if it were the pi'el khirik.

| Pointed | Full | Reading | Gloss |
|---|---|---|---|
| סֵפֶר | ספר | *séfer* | book |
| מֵמַד | ממד | *memad* | dimension |
| אֵזוֹר | אזור | *ezor* | region |
| תֵּאֵר | תיאר | *te'er* | described |
| בֵּרוּר | בירור | *berur* | clarification |

**(5) Consonantal ו [v] is doubled (וו) inside a word**, but not at the start or end of the word, and not when the result would have three ו in a row.

| Pointed | Full | Reading | Gloss |
|---|---|---|---|
| תִּקְוָה | תקווה | *tikva* | hope |
| מִצְוָה | מצווה | *mitsva* | commandment |
| זָוִית | זווית | *zavit* | angle |
| שָׁוֶה | שווה | *shave* | equal, worth |
| וֶרֶד | ורד | *véred* | rose (start: single) |
| קַו | קו | *kav* | line (end: single) |
| עַכְשָׁו | עכשיו | *akhshav* | now (end: single) |
| תִּוּוּךְ | תיווך | *tivukh* | mediation (no third ו) |

After a prefix, a word-initial ו is doubled (הַוֶּרֶד → **הוורד** *ha-véred* "the rose"), **except after the conjunction ו**: וְוֶרֶד → **וורד** *ve-véred* "and a rose", not *ווורד.

**(6) Consonantal י [j] is doubled (יי) inside a word and often at the end**, but not at the start of a word, not after a prefix, and not next to a vowel letter ו or י.

| Pointed | Full | Reading | Gloss |
|---|---|---|---|
| בִּנְיָן | בניין | *binyan* | building; verb pattern |
| עִנְיָן | עניין | *inyan* | matter |
| הָיְתָה | הייתה | *hayta* | she was |
| בָּנַי | בניי | *banay* | my sons |
| שָׁמַיִם | שמיים | *shamáyim* | sky |
| יֶלֶד | ילד | *yéled* | boy (start: single) |
| הַיֶּלֶד | הילד | *ha-yéled* | the boy (after prefix: single) |
| בַּיִת | בית | *báyit* | house (diphthong [aj] in segolates: single) |
| פְּרוֹיֶקְט | פרויקט | *proyekt* | project (next to ו: single) |
| בְּעָיָה | בעיה | *be'aya* | problem (next to vowel-letter י: single) |

The rule for the end of a word has some exceptions, in short frequent words: חַי *khay* "alive" → **חי**, דַּי *day* "enough" → **די**, שַׁי *shay* "gift" → **שי**, and מַיִם *máyim* "water" → **מים**. But שָׁמַיִם → **שמיים** "sky".

**(7) No more than two identical vowel letters in a row.** When the rules would give three, as in קִיֵּם *kiyem* "fulfilled", which would be *קייים, the writer keeps two: **קיים**. So the written form קיים covers *kayam* "exists" (adjective), *kiyem* "fulfilled", and *kayem* "fulfil!".

### 2.4.4 What full spelling still leaves open

Even with the rules applied, unpointed full spelling does **not** show:

- /a/ and most /e/ (no letter at all);
- whether ו is [v], [o] or [u], and whether י is [j], [i] or [e]/[ej];
- whether ב כ פ are stops or spirants (except final ף, ך, which are spirants in native words);
- shin vs. sin;
- gemination, and stress;
- whether an initial ו ה ב כ ל מ ש is a prefix or part of the stem.

§2.9 counts the resulting ambiguity.

## 2.5 The geresh: sounds that Hebrew letters lack

The **geresh** (גֵּרֵשׁ, ׳, U+05F3; often typed as the apostrophe ') after a letter changes its value. It marks consonants found in loanwords, slang and names, and in transcriptions of Arabic (Academy of the Hebrew Language, rules for transcribing foreign words; Wikipedia "Geresh").

| Written | Sound | Example | Reading | Gloss |
|---|---|---|---|---|
| ג׳ | [dʒ] | ג׳ינס | *djins* | jeans |
| ג׳ | [dʒ] | ג׳וק | *djuk* | cockroach (slang, from Arabic/Turkish) |
| ז׳ | [ʒ] | ז׳ורנל | *zhurnal* | (fashion) magazine |
| ז׳ | [ʒ] | בז׳ | *bezh* | beige |
| צ׳ | [tʃ] | צ׳יפס | *chips* | French fries |
| צ׳ | [tʃ] | צ׳ופר | *chúper* | treat, bonus |
| ת׳ | [θ] (transcription; said [t]) | ת׳צ׳ר | *Tácher* | Thatcher |
| ד׳ | [ð] (transcription; said [d]) | ד׳ה | *de* | "the" in a foreign name |
| ח׳ | [x] (Arabic خ, when it must be kept apart from ח [ħ]) | ח׳ליל | *Khalil* | (Arabic name) |
| ע׳ | [ɣ] (Arabic غ) | ע׳זה | *Ghaza* | Gaza (in Arabic transcription; Hebrew עזה *Aza*) |
| ט׳ / צ׳ | Arabic emphatics ظ / ض | | | scientific transcription only |

Speakers have no [θ] or [ð], so ת׳ and ד׳ are pronounced [t] and [d] (or sometimes [s], [z]). They serve in transcription only.

**[w].** Hebrew has no standard letter for [w]. Loanwords and names write it as a single **ו** at the start of a word, or as **וו** inside it: ווטסאפ *wótsap* "WhatsApp", וויסקי *víski*/*wíski* "whisky", ווינדוס *wíndos* "Windows". The Academy's transcription rules recommend וו at the start of a word too, and ו׳ is non-standard. In speech, [w] is often replaced by [v].

**[p] and [f] at word end, and final צ׳.** See §2.1.1.

## 2.6 Gershayim, abbreviations and acronyms

The **gershayim** (גֵּרְשַׁיִם, ״, U+05F4, often typed as the ASCII quotation mark ") go **before the last letter** of a word that is:

1. an **acronym** (*rashei tevot*, רָאשֵׁי תֵּבוֹת), made of the first letters of several words; or
2. a **number written with letters** of two or more letters (§2.7).

Acronyms are very common in Modern Hebrew, and many are said as words:

| Acronym | Full form | Said | Gloss |
|---|---|---|---|
| צה״ל | צְבָא הֲגַנָּה לְיִשְׂרָאֵל | *Tsáhal* | Israel Defense Forces |
| ת״א | תֵּל אָבִיב | *Tel Aviv* | Tel Aviv |
| ארה״ב | אַרְצוֹת הַבְּרִית | *Artsot ha-Brit* | United States |
| ביה״ס | בֵּית הַסֵּפֶר | *beit ha-séfer* | the school |
| דו״ח | דִּין וְחֶשְׁבּוֹן | *dóakh* (also *du'akh*) | report |
| מנכ״ל | מְנַהֵל כְּלָלִי | *mankal* | director general, CEO |
| רמטכ״ל | רֹאשׁ הַמַּטֶּה הַכְּלָלִי | *ramatkal* | Chief of General Staff |
| ש״ח | שֶׁקֶל חָדָשׁ | *shékel* (read out: *shékel khadash*) | new shekel |
| בע״מ | בְּעֵרָבוֹן מֻגְבָּל | *be'eravon mugbal* | Ltd. |
| חו״ל | חוּץ לָאָרֶץ | *khul* | abroad |
| עו״ד | עוֹרֵךְ דִּין | *orekh din* | lawyer |
| ד״ר | דּוֹקְטוֹר | *dóktor* | Dr. |
| בי״ח | בֵּית חוֹלִים | *beit kholim* | hospital |
| או״ם | הָאֻמּוֹת הַמְּאֻחָדוֹת | *um* | United Nations |
| תנ״ך | תּוֹרָה נְבִיאִים כְּתוּבִים | *Tanakh* | the Hebrew Bible |
| רמב״ם | רַבִּי מֹשֶׁה בֶּן מַיְמוֹן | *Rambam* | Maimonides |
| בס״ד | בְּסִיַּעְתָּא דִּשְׁמַיָּא | (not read) | "with the help of Heaven", at the head of a letter |
| ז״ל | זִכְרוֹנוֹ לִבְרָכָה | *zal* / *zikhrono li-vrakha* | "of blessed memory" |

The **geresh** alone marks an abbreviation cut off after one or more letters (*kitsur*):

| Abbreviation | Full | Gloss |
|---|---|---|
| גב׳ | גְּבֶרֶת | Mrs., Ms. |
| מס׳ | מִסְפָּר | no. (number) |
| עמ׳ | עַמּוּד | p. (page) |
| וכו׳ | וְכוּלֵּי | etc. |
| רח׳ | רְחוֹב | St. (street) |
| פרופ׳ | פְּרוֹפֶסוֹר | Prof. |
| א׳ | (Sunday; or "1st") | |

**Prefixes on acronyms.** A prefix attaches to the acronym, and the gershayim stay before its last letter: בצה״ל *be-Tsáhal* "in the IDF", לארה״ב *le-Artsot ha-Brit* "to the US", מהאו״ם *me-ha-um* "from the UN". Tokenization must not split such a word at the gershayim. In some UD treebanks, these are multiword tokens like any other word with a prefix ([11-ud-analysis.md](11-ud-analysis.md)).

**Typing.** In practice, ״ (U+05F4) and ׳ (U+05F3) are rare in user-generated text. Writers use ASCII " (U+0022) and ' (U+0027), and sometimes typographic quotes or ’. A normalizer should map all of these to one form, and must also handle the use of " and ' as real quotation marks (§2.8).

## 2.7 Numbers written with letters (gematria)

Each letter has a numerical value (table in §2.1). Numbers are written by adding the values, largest first. This "Hebrew numbering" is used for:

- Hebrew calendar **dates** and years: ה׳תשפ״ז = 5787;
- **ordinals and section numbers** in lists, laws, chapters and verses (סעיף ג׳ "section 3", פרק י״ב "chapter 12");
- school **grades** (כיתה ה׳ "5th grade") and days of the week (יום א׳ "Sunday");
- house numbers in some old addresses and religious texts.

Everyday arithmetic, prices and quantities use Arabic digits.

| Number | Letters | Remark |
|---:|---|---|
| 1–9 | א׳ … ט׳ | |
| 10 | י׳ | |
| 11 | י״א | 10 + 1 |
| 15 | **ט״ו** | 9 + 6, **not** י״ה (10+5), which would spell part of the divine name יה |
| 16 | **ט״ז** | 9 + 7, **not** י״ו (10+6), for the same reason |
| 17 | י״ז | |
| 20 | כ׳ | |
| 99 | צ״ט | |
| 100 | ק׳ | |
| 270 | ר״ע | sometimes written ע״ר, to avoid the word רע "evil" |
| 304 | ש״ד | sometimes written ד״ש, to avoid שד "demon" |
| 400 | ת׳ | the largest single letter |
| 500 | ת״ק | 400 + 100 (final letters ך ם ן ף ץ = 500–900 in an older system) |
| 1000 | א׳ (with a following geresh, written before the hundreds) | e.g. ה׳תשפ״ז: ה׳ = 5000 |
| 5787 | ה׳תשפ״ז | 5000 + 400 + 300 + 80 + 7 (the Hebrew year 2026/27) |

Years are usually written **without** the thousands: תשפ״ז = (5)787. Rule for punctuation: one letter takes a geresh (ה׳), several take gershayim before the last letter (תשפ״ז). Sources: Wikipedia "Hebrew numerals"; Academy rules for punctuation (2002).

The same convention gives rise to **gematria**, the interpretive practice of comparing words with equal numerical value (e.g. יין *yáyin* "wine" = 70 = סוד *sod* "secret"). It belongs to traditional exegesis and wordplay, not to grammar.

## 2.8 Punctuation

Hebrew uses the European set of punctuation marks: period, comma, colon, semicolon, question mark, exclamation mark, quotation marks and parentheses. The Academy's punctuation rules of 2002 (updated 2017) describe the standard use (Academy of the Hebrew Language, *Klalei ha-pisuk*). Points specific to Hebrew:

- **Maqaf** (מַקָּף, ־, U+05BE): the traditional Hebrew hyphen, set at the top of the line. It joins words in compounds and in some fixed phrases, such as בית־ספר "school", דו־שיח "dialogue" and אי־אפשר "impossible". In modern text it is often replaced by the ASCII hyphen - or by a plain space. Both spellings of the same compound must be expected.
- **Quotation marks**: "…" (ASCII) is the most common choice. The Academy recommends „…“ or ״…״-style marks, but these are rare in data. Because ASCII " also stands for the gershayim (§2.6), a text such as `אמר "צה"ל מוכן"` is ambiguous for a naive tokenizer.
- **Ellipsis**, **dash** (– or a spaced hyphen) as in English.
- **Sentence-final punctuation** sits at the left end of the line (the end in RTL), as the bidi algorithm places it (§2.11).
- The **sof pasuq** ׃ (U+05C3) and **paseq** ׀ (U+05C0) belong to Biblical text only.

## 2.9 How ambiguous is unpointed text?

Unpointed Modern Hebrew is highly ambiguous at the level of the written token. One string can be:

- different **lexemes** with different vowels: ספר = *séfer* "book", *sapar* "barber", *safar* "counted", *siper* "told", *sfar* "border region", *supar* "was told";
- one lexeme in different **forms**, or homophones across parts of speech: אהבה = *ahava* "love (N)" and *ahava* "she loved"; כתבה = *katva* "she wrote" and *katava* "(newspaper) article";
- a **prefix + stem** split in several ways: בצל = *batsal* "onion" / *be-tsel* "in a shadow" / *ba-tsel* "in the shadow"; שמן = *shémen* "oil" / *shamen* "fat (ADJ)"; מכה = *maka* "blow, plague" / *make* "hits (PRS.MS)"; הרכבת = *ha-rakévet* "the train" / *harkavat* "assembly of (CS)" / *hirkavt* "you (F) assembled";
- a **stem + pronoun suffix**: ספרה = *sifra* "digit" / *sifra(h)* "her book" / *safra* "she counted";
- **unclear spirantization and shin/sin**: בשר = *basar* "meat" / *biser* "announced" (ב = [b]/[v] in other forms of the same root); שבר = *shéver* "fracture" / *shavar* "broke".

Example of a sentence-level ambiguity with the prefix ה *ha-* "the":

```
הספר נמצא בחדר
ha-séfer nimtsa ba-khéder
DEF-book be_found.PST.3MS in.DEF-room
'The book was found / is located in the room.'
```

```
הספר נמצא בחדר
ha-sapar nimtsa ba-khéder
DEF-barber be_found.PST.3MS in.DEF-room
'The barber is in the room.'
```

Both readings are spelled exactly alike, and only the context decides.

### 2.9.1 Counted figures

From the literature on Hebrew morphological analysis:

- A study of a Modern Hebrew corpus found an **average of 2.1 analyses per word form**, with **55% of the word forms** morphologically ambiguous (Sima'an et al. 2001, as reported by Tsarfaty 2006).
- On a 6M-word newspaper corpus (178,580 distinct word forms, 64,541 lemmas), the analyzer gave **2.4 analyses per word** on average (Adler & Elhadad 2006).
- Goldberg & Elhadad (2009), citing Adler (2007): **2.7 analyses per token**, compared with **1.4 in English**. Some tokens have **up to 15** readings.

**Our own count (2026-10-17).** This count uses the two CC BY-SA UD treebanks, IAHLTwiki and IAHLTknesset (UD 2.18; 129,208 non-punctuation tokens). For each surface token, we listed the distinct analyses that the gold annotation gives it, an analysis being the sequence of (lemma, UPOS) of its syntactic words. This counts only readings **attested in the gold data**, so it is a lower bound on the ambiguity a reader faces.

| Measure | Value |
|---|---:|
| surface token types | 30 238 |
| types with ≥ 2 attested analyses | 2 634 (8.7%) |
| tokens | 129 208 |
| tokens whose type has ≥ 2 attested analyses | 50 741 (**39.3%**) |

The most ambiguous types are mostly frequent function words, which is why few types cover many tokens:

| Token | Attested analyses (count in data) |
|---|---|
| היום | *ha-yom* "today" ADV (113) · ה+יום "the day" (45) |
| מספר | *mispar* "a number of" DET (57) · "number" NOUN (42) · *mesaper* "tells" (from סיפר) VERB (4) |
| שני | *shney* "two (CS)" NUM (from שתיים, 92) · *sheni* "second" ADJ (30) … |
| מאז | *me-az* מ+אז "since then" (30) · *me'az* "since" ADP (10), ADV (8), SCONJ (4) |
| שכן | *shekén* "since, for" SCONJ (14) · ש+כן "that yes" (5) · *shakhan* "dwelt" VERB (2) · *shakhen* "neighbour" NOUN |
| בעלות | *be-alut* ב+עלות "at a cost" (8) · *ba'alut* "ownership" (5) · *ba'alot* "owners (F)" (from בעל) … |
| חולים | *kholim* "sick people" (lemma חולה, 35) · the same word with lemma חלה (3), VERB or ADJ readings — partly a lemma-convention difference between annotators |
| בנוגע | *be-noge'a* "regarding" (one word ADP, or ב + נגע/נוגע) — 6 analyses, mostly an annotation-level split |

(The count was made with a short script over `UD_Hebrew-IAHLTwiki/*.conllu` and `UD_Hebrew-IAHLTknesset/*.conllu`. Some "ambiguity" comes from the annotation, not from the language, such as בנוגע analysed both as one word and as two.)

Consequences for processing:

- tokens must be **segmented** into prefixes, stem and suffix, and the segmentation is itself ambiguous, so it has to be decided jointly with tagging and parsing (Tsarfaty 2006; Goldberg & Tsarfaty 2008; More et al. 2019);
- a lexicon of **all forms with all analyses** (Hspell, the MILA lexicon of Itai & Wintner 2008, Dicta's word list) is the usual first step;
- **diacritization** (restoring the points) is a separate, hard task in its own right (Shmidman et al. 2020, Nakdan; Gershuni & Pinter 2022).

See [11-ud-analysis.md](11-ud-analysis.md) and [12-mova.md](12-mova.md).

## 2.10 Prefixes written together with the word

Seven one-letter function words (traditionally *mosh ve-khalev*, מש״ה וכל״ב) are written **attached** to the following word, and can stack:

| Letter | Function | Example | Reading | Gloss |
|---|---|---|---|---|
| ו | "and" | ובית | *u-váyit* | and a house |
| ה | definite article "the" | הבית | *ha-báyit* | the house |
| ה | (interrogative *ha-*, Biblical; in Modern Hebrew only in fixed or high-register uses) | | | |
| ש | "that, which, who" | שבא | *she-ba* | who came |
| ב | "in, at, with" | בבית | *ba-báyit* | in the house (ב + ה fuse) |
| כ | "as, like" | כבית | *ke-váyit* | like a house |
| ל | "to, for" | לבית | *la-báyit* | to the house (ל + ה fuse) |
| מ | "from" | מהבית | *me-ha-báyit* | from the house |
| (stacked) | | וכשבבית | *u-kshe-ba-báyit* | "and when in the house …" (ו + כש + ב + ה) |

When ב, כ or ל comes before the article ה, the ה is **not written** and its vowel goes onto the preposition: ל + הבית → **לבית** *la-báyit* "to the house". So the unpointed לבית is both *le-váyit* "to a house" and *la-báyit* "to the house". The UD treebanks handle the hidden article in two ways. HTB restores it as a syntactic word of its own, with FORM `ה_`, LEMMA ה, `DET` and `PronType=Art` (למשטרה = ל + ה_ + משטרה; 2 547 such words in HTB train). IAHLTwiki and IAHLTknesset write no extra word and put `Definite=Def|PronType=Art` on the preposition (לבית = ל[Definite=Def] + בית). Counted in UD 2.18 on 2026-10-17. This is one of the main points in [11-ud-analysis.md](11-ud-analysis.md).

The order of stacked prefixes is fixed: ו, then ש/כש, then ב/כ/ל/מ, then ה.

## 2.11 Unicode, right-to-left layout and normalization

### 2.11.1 Code points

| Range | Contents |
|---|---|
| U+0591–U+05AF | cantillation accents (Bible only) |
| U+05B0–U+05BD, U+05BF, U+05C1–U+05C2, U+05C4–U+05C5, U+05C7 | points: shva (05B0), khataf (05B1–05B3), khirik (05B4), tsere (05B5), segol (05B6), patakh (05B7), kamats (05B8), kholam (05B9), kholam for vav (05BA), kubuts (05BB), dagesh/mappiq/shuruk dot (05BC), meteg (05BD), rafe (05BF), shin dot (05C1), sin dot (05C2), kamats katan (05C7) |
| U+05BE | maqaf ־ |
| U+05C0, U+05C3, U+05C6 | paseq, sof pasuq, nun hafukha |
| U+05D0–U+05EA | the 27 letters (22 + 5 final forms) |
| U+05EF–U+05F2 | Yiddish ligatures and the yod triangle (װ ױ ײ) |
| U+05F3, U+05F4 | geresh ׳, gershayim ״ |
| U+FB1D–U+FB4F | Alphabetic Presentation Forms: precomposed pointed letters (שׁ U+FB2A, בּ U+FB31, …) and the ligature ﭏ |

Source: The Unicode Standard, chapter on Hebrew and the code charts for U+0590–U+05FF (https://www.unicode.org/charts/PDF/U0590.pdf).

### 2.11.2 Normalization

- **Point order.** Several points can sit on one letter (dagesh + vowel + shin dot). Canonical ordering under Unicode NFC/NFD sorts combining marks by their *canonical combining class*. For Hebrew, these classes were assigned in a way that does **not** follow the traditional order of writing, and NFC can reorder points, for example putting a vowel before the dagesh. Two sources that look identical can then differ byte for byte. Compare pointed text **after normalization**, or compare the consonants only. (Mova's `tools/macula` compares lemmas by consonants for this reason.)
- **Presentation forms.** U+FB1D–U+FB4F decompose under NFKC/NFD into base letters + points. Apply NFD or NFKC before comparing.
- **Lookalikes.** Writers type ׳ ״ as ' ", the maqaf as -, and the final letters are sometimes wrong in OCR or in older encodings (e.g. ISO-8859-8 "visual" Hebrew, stored in reverse order).
- **Stripping points.** For matching unpointed text, remove U+0591–U+05C7 except the letters, maqaf and geresh/gershayim. Removing the points without the ktiv male changes gives a *defective* spelling (שֻׁלְחָן → שלחן), which differs from the everyday full spelling (שולחן). A diacritized corpus therefore cannot be turned into everyday text by simple stripping.

### 2.11.3 Bidirectional text

Hebrew is laid out right to left by the **Unicode Bidirectional Algorithm** (UAX #9). Hebrew letters are strong RTL characters. Digits and Latin letters keep their left-to-right order inside a Hebrew line. Practical consequences:

- **Logical order.** Text is stored in logical (reading) order and displayed reversed. Old files in "visual" order (ISO-8859-8 visual, some PDFs) must be reversed. PDF text extraction of Hebrew often yields reversed or scrambled words.
- **Mixed lines.** A Hebrew sentence with a number, an English word or a URL renders in a mixed order, and neutral characters (punctuation, spaces, parentheses) take their direction from their neighbours. Parentheses are *mirrored*: ")" is displayed as "(" in RTL.
- **Control characters** (U+200E LRM, U+200F RLM, U+202A–U+202E embeddings, U+2066–U+2069 isolates) are often present in web text and must be stripped or ignored by a tokenizer.
- **Code blocks in this manual** show Hebrew in logical order. Viewers that do not apply bidi to code blocks may show the words of a Hebrew line left to right.

## 2.12 Romanization standards

Several standards exist for writing Hebrew in Latin letters. They differ in what they try to show: the spelling, the Tiberian pronunciation, or modern speech.

| Standard | Year | Basis | Sample: שָׁלוֹם עֲלֵיכֶם |
|---|---|---|---|
| Academy of the Hebrew Language | 1953; simplified 2000; **new rules 2006** (UN standard 2007) | modern pronunciation with some historical distinctions | *shalom ʿalekhem* |
| ISO 259 | 1984 | full transliteration of the pointed Tiberian text, letter by letter | (each point and dagesh encoded, e.g. doubled letters for dagesh forte) |
| ISO 259-2 | 1994 | simplified transliteration | *šalwm ʿalykm* style (*šalŵm ʿaleykem*) |
| ISO/FDIS 259-3 | 1999 (draft, not adopted) | phonemic, modern | *šalom ˁaleikem* |
| SBL Handbook of Style, academic | 2nd ed. 2014 | Tiberian, scholarly | *šālôm ʿălêkem* |
| SBL general-purpose | 2nd ed. 2014 | readable, no diacritics | *shalom alekhem* |
| UNGEGN / BGN-PCGN | 1977 / 1962 | place names, based on the Academy rules | |

Sources: Wikipedia "Romanization of Hebrew" (accessed 2026-10-17); Academy of the Hebrew Language, *Klalei ha-ta'atik mi-ivrit le-otiyot latiniyot* (2006); SBL Handbook of Style, 2nd ed. (2014), §5.1.

The **Academy 2006** rules map letters as follows. Consonants: א and ע → ʾ / ʿ (or nothing), ב b/v, ג g, ד d, ה h, ו v, ז z, ח ẖ (h with a line below; plain h in simplified use), ט t, י y, כּ k, כ kh, ל l, מ m, נ n, ס s, פּ p, פ f, צ ts, ק k, ר r, ש sh, שׂ s, ת t. Vowels: a e i o u, with no length marks, and shva na written e. Unlike everyday practice, the rules keep ח (ẖ) apart from כ (kh) and do not write the diphthong [ej] of tsere-yod, but they do follow modern speech in dropping many shva na (*zman*, not *zeman*). Gemination is shown only in some cases. (Values from Wikipedia's comparison table; the Academy's own PDF should be consulted for details.)

### 2.12.1 The transliteration used in this manual

This manual uses a **broad Israeli transliteration**, meant to show how a standard speaker says the word, not how it is spelled. It is close to the Academy's 2006 rules and to SBL general style, with these choices:

| Sound | Written | Letters | Example |
|---|---|---|---|
| glottal stop [ʔ] | **'** only where it is actually pronounced (between vowels, or after a prefix before a vowel) | א, ע | *ma'amar* "article", *ha-'ir* is written *ha-ir* (not pronounced separately in normal speech), *be'aya* "problem" |
| [χ] | **kh** | both ח and כ | *khaver* "friend" (ח), *mikhtav* "letter" (כ) |
| [ts] | **ts** | צ | *tsipor* "bird" |
| [ʃ] | **sh** | ש | *shalom* |
| [ʁ] | **r** | ר | *rakevet* "train" |
| [j] | **y** | י | *yéled* |
| [v] | **v** | ב, ו | *véred*, *kav* |
| [dʒ] [ʒ] [tʃ] | **dj**, **zh**, **ch** | ג׳ ז׳ צ׳ | *djins*, *zhurnal*, *chips* |
| vowels | **a e i o u**; the diphthongs **ay**, **ey**, **oy** | | *báyit*, *beyt* (CS) |

- **Stress** is marked with an acute accent **only when it is not on the final syllable**: *yéled* "boy", *báyit* "house", *séfer* "book", *khanut* "shop" (final, unmarked). Final stress is the morphological default for native words, and penultimate stress is common in segolate nouns, some suffixes and loanwords. A two-syllable word without an accent is stressed on the last syllable. Monosyllables carry no accent.
- **Prefixes** (the article, the prepositions ב כ ל מ, ו, ש) are separated by a **hyphen**: *ha-yéled* "the boy", *be-váyit* "in a house", *la-yéled* "to the boy", *ve-ha-yéled* "and the boy", *she-ra'iti* "that I saw". This mirrors the UD segmentation into syntactic words ([11-ud-analysis.md](11-ud-analysis.md)). Pronoun suffixes are **not** hyphenated: *sifri* "my book".
- **Spirantization** is shown as it is pronounced: *be-váyit* (ב after the prefix = [v]).
- **Shva** is written *e* only where it is pronounced in standard speech (*zman*, but *be-*, *le-*, *medabéret*).
- **ח and ע as pharyngeals** [ħ ʕ] are not shown. Where the Mizrahi realization matters, [03-phonology.md](03-phonology.md) uses IPA.
- **Final ה** of a feminine noun is not written (*yalda* "girl"). The pronounced [h] of the mappiq suffix is written in parentheses: *sifra(h)* "her book".
- **Names** of people and places use their usual English spellings (Tel Aviv, Ben-Yehuda).
- **IPA** in square brackets [ ] is used where phonetic detail matters, and slashes / / for phonemes.

### 2.12.2 Example layout

Every example in this manual is a four-line block: the Hebrew as normally written (unpointed unless points matter), the transliteration, a Leipzig-style gloss, and a translation.

```
הילדה קראה ספר בגן
ha-yalda kar'a séfer ba-gan
DEF-girl read.PST.3FS book in.DEF-garden
'The girl read a book in the garden.'
```

Gloss abbreviations: 1, 2, 3 person; M masculine; F feminine; SG/S singular; PL/P plural; DEF definite; ACC accusative marker את; CS construct state; POSS possessive; PST past; PRS present (participle); FUT future; IMP imperative; INF infinitive; NEG negation; COP copula; EXIST existential; REL relative ש; Q question particle; PASS passive; DU dual. A combined person-gender-number tag is written 3MS (3rd person masculine singular), 2FP (2nd person feminine plural), and so on. Abbreviations specific to one chapter are explained there.

---

## References

- Academy of the Hebrew Language (1947; revised; 2017). *Klalei ha-ktiv she-eino menukad* / *Klalei ha-ktiv ha-male* [Rules of spelling without points / of full spelling]. https://hebrew-academy.org.il/topic/hahlatot/missingvocalizationspelling/ (2017 PDF linked there). Automatic fetching refused (HTTP 403) on 2026-10-17; content checked via the Hebrew Wikipedia summary below.
- Academy of the Hebrew Language (2002, updated 2017). *Klalei ha-pisuk* [Punctuation rules]. https://hebrew-academy.org.il/topic/hahlatot/punctuation/
- Academy of the Hebrew Language (2006). *Klalei ha-ta'atik mi-ivrit le-otiyot latiniyot* [Rules for transliteration from Hebrew into Latin letters]. https://hebrew-academy.org.il/topic/hahlatot/transliteration/
- Adler, Meni & Michael Elhadad (2006). An Unsupervised Morpheme-Based HMM for Hebrew Morphological Disambiguation. COLING-ACL 2006. https://aclanthology.org/P06-1084/ ; local: $RESEARCH_RAW/aclbulk-2006-adler-unsupervised-morpheme-based-hmm-hebrew/
- Asherov, Daniel & Evan-Gary Cohen (2019). A phonetic description of Modern Hebrew consonants and vowels. *Brill's Annual of Afroasiatic Languages and Linguistics* 11(1). (cited via Wikipedia; not held locally)
- Bolozky, Shmuel (1997). Israeli Hebrew phonology. In A. S. Kaye (ed.), *Phonologies of Asia and Africa*, vol. 1, 287–311. Eisenbrauns. (not held locally)
- Coffin, Edna Amir & Shmuel Bolozky (2005). *A Reference Grammar of Modern Hebrew*. Cambridge University Press. (not held locally; listed in $RESEARCH_RAW/hebgram/grammars.tsv)
- Gershuni, Elazar & Yuval Pinter (2022). Restoring Hebrew Diacritics Without a Dictionary. Findings of NAACL 2022. https://aclanthology.org/2022.findings-naacl.75/ ; local: $RESEARCH_RAW/findings-2022-gershuni-restoring-hebrew-diacritics-without-dictionary/
- Goldberg, Yoav & Michael Elhadad (2009). Hebrew Dependency Parsing: Initial Results. IWPT 2009. https://aclanthology.org/W09-3819/ ; local: $RESEARCH_RAW/lingbulk-2009-goldberg-hebrew-dependency-parsing-initial-results/
- Goldberg, Yoav & Reut Tsarfaty (2008). A Single Generative Model for Joint Morphological Segmentation and Syntactic Parsing. ACL 2008. https://aclanthology.org/P08-1043/
- Har'El, Nadav & Dan Kenigsberg. *Hspell — the free Hebrew spell checker and morphological analyzer*, v1.4. http://hspell.ivrix.org.il/ ; local: $RESEARCH_RAW/lexres/morphology/sa/hspell/ (AGPL-3.0)
- Hebrew Wikipedia. כתיב חסר ניקוד [Spelling without points]. https://he.wikipedia.org/wiki/כתיב_חסר_ניקוד (accessed 2026-10-17)
- Itai, Alon & Shuly Wintner (2008). Language resources for Hebrew. *Language Resources and Evaluation* 42(1), 75–98. (see also $RESEARCH_RAW/lingbulk-2006-itai-computational-lexicon-contemporary-hebrew/)
- Khan, Geoffrey (2020). *The Tiberian Pronunciation Tradition of Biblical Hebrew*, 2 vols. Open Book Publishers. https://doi.org/10.11647/OBP.0163 ; local: $RESEARCH_RAW/hebgram/open/khan-tiberian-pronunciation-v1-2020/
- Laufer, Asher (1999). Hebrew. In *Handbook of the International Phonetic Association*, 96–99. Cambridge University Press.
- More, Amir, Amit Seker, Victoria Basmova & Reut Tsarfaty (2019). Joint Transition-Based Models for Morpho-Syntactic Parsing: Parsing Strategies for MRLs and a Case Study from Modern Hebrew. *TACL* 7. https://aclanthology.org/Q19-1003/
- SBL (2014). *The SBL Handbook of Style*, 2nd ed., §5.1 (Hebrew transliteration). SBL Press.
- Shmidman, Avi, Shaltiel Shmidman, Moshe Koppel & Yoav Goldberg (2020). Nakdan: Professional Hebrew Diacritizer. ACL 2020 System Demonstrations. https://aclanthology.org/2020.acl-demos.23/ ; local: $RESEARCH_RAW/aclbulk-2020-shmidman-nakdan-professional-hebrew-diacritizer/
- Sima'an, Khalil, Alon Itai, Yoad Winter, Alon Altman & Noa Nativ (2001). Building a tree-bank of Modern Hebrew text. *Traitement Automatique des Langues* 42(2). (cited via Tsarfaty 2006)
- Tsarfaty, Reut (2006). Integrated Morphological and Syntactic Disambiguation for Modern Hebrew. COLING-ACL 2006 Student Research Workshop. https://aclanthology.org/P06-3009/ ; local: $RESEARCH_RAW/aclbulk-2006-tsarfaty-integrated-morphological-syntactic-disambiguation-modern/
- Unicode Consortium. *The Unicode Standard*, Hebrew block U+0590–U+05FF code chart. https://www.unicode.org/charts/PDF/U0590.pdf ; and UAX #9, Unicode Bidirectional Algorithm. https://www.unicode.org/reports/tr9/
- Universal Dependencies 2.18: UD_Hebrew-IAHLTwiki and UD_Hebrew-IAHLTknesset (CC BY-SA 4.0). https://universaldependencies.org/treebanks/he_iahltwiki/ ; local: $RESEARCH_RAW/lexres/treebanks/ud-2.18/
- Wikibooks (Hebrew). *Dikduk ha-Ivrit ha-Hadasha* [Grammar of Modern Hebrew], pages "Consonants" and "Vowels" (CC BY-SA). https://he.wikibooks.org/wiki/דקדוק_העברית_החדשה ; local: $RESEARCH_RAW/hebgram/sa/wikibooks-he-dikduk-haivrit-hahadasha/
- Wikipedia. Hebrew alphabet; Geresh; Gershayim; Hebrew numerals; Romanization of Hebrew; Modern Hebrew phonology. https://en.wikipedia.org/wiki/Hebrew_alphabet etc. (accessed 2026-10-17)
