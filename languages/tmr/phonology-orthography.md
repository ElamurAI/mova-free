# Phonology and orthography of Jewish Babylonian Aramaic

The script and the inherited consonants are described in [../arc/script-phonology.md](../arc/script-phonology.md). JBA differs in three ways that matter for reading and for a tokenizer: it is written **plene** (with many vowel letters), it has **lost or weakened** many sounds (gutturals, final consonants, unstressed vowels), and its texts are full of **abbreviations**. The vowels given here are reconstructions after Margolis 1910, Epstein 1960, Morag 1988 and Bar-Asher Siegal 2013, and are marked as such where it matters.

## 1. Spelling

### 1.1 Plene writing

Manuscripts and prints of the Bavli write vowels with א, ו, י much more than Biblical Aramaic or the targums:

| vowel | letter | example | reading (reconstr.) | meaning |
|---|---|---|---|---|
| a in an open syllable | א | קָאָמַר, קאמר | *qāʾamar* | "he is saying" (קָא + אָמַר) |
| ā final (determined) | א | גַּבְרָא, מִילְּתָא | *gaḇrā*, *milltā* | "man", "word, matter" |
| i, e | י | אִיתְּמַר, לֵימָא, מִילֵּי | *ʾittəmar*, *lēmā*, *millē* | "it was said", "let him say", "words" |
| u, o | ו | אוּשְׁפִּיזָא, הוּא | *ʾušpīzā*, *hū* | "lodging", "he" |
| consonantal w, y | וו, יי | אַיְיתֵי, אִיתְיְלִיד | *ʾaytē*, *ʾiṯyəlīḏ* | "he brought", "was born" |

This makes the unpointed Bavli easier to read than unpointed Biblical Aramaic, but it also means the **same word is spelled in several ways** across manuscripts (אמינא / אמינה; היכי / הכי in some prints). A model must normalise spelling before counting.

### 1.2 Final ־א and ־ה

The determined ending is written ־א (גַּבְרָא, מִילְּתָא). The 3 f. sg. perfect and some feminine forms end in ־ה or ־א by manuscript. In the printed text, ־ה at the end of a noun is often the 3 m. sg. suffix spelled short (בְּרֵיהּ "his son" is written בריה): the context decides.

### 1.3 Abbreviations

The printed Bavli abbreviates its frequent phrases with a gershayim (״ or ") before the last letter, or a geresh (׳) after a cut word. A small table covers most of the text; counts are in the Wikisource Bavli (CC BY-SA), 2026-10-17:

| abbreviation | full form | meaning | count |
|---|---|---|---:|
| א"ל | אָמַר לֵיהּ | he said to him | 4 398 |
| א"ר | אָמַר רַבִּי / רַב | Rabbi / Rav said | 3 600 |
| ת"ר | תָּנוּ רַבָּנַן | our rabbis taught (a baraita follows) | 1 295 |
| קמ"ל | קָא מַשְׁמַע לַן | it teaches us | 1 263 |
| מ"ט | מַאי טַעְמָא | what is the reason? | 1 052 |
| ת"ש | תָּא שְׁמַע | come and hear | 1 030 |
| ש"מ | שְׁמַע מִינַּהּ | learn from it | 713 |
| ה"נ | הָכִי נַמִי | so too, indeed | 493 |
| מנה"מ | מְנָא הָנֵי מִילֵּי | whence these words? | 35 |
| ר"ל | רֵישׁ לָקִישׁ (also רוֹצֶה לוֹמַר in later Hebrew) | Resh Laqish | |
| אע"ג | אַף עַל גַּב | even though | |
| ע"מ | עַל מְנָת | on condition | |
| ד' / ד׳ | the number 4 (letters as numerals) | | |
| גמ׳, מתני׳ | גְּמָרָא, מַתְנִיתִין | Gemara, Mishnah (section labels) | |

The **same abbreviation can be Hebrew or Aramaic** (א"ר introduces sayings in both languages), and **some are ambiguous** (ר"ל). Expansion is a lookup with context, done before parsing; the abbreviated form stays in MISC ([mova.md](mova.md) §3).

## 2. Lost and weakened sounds

### 2.1 Gutturals

In Babylonia, as in Galilee, the gutturals weakened. Manuscripts confuse ה with ח and א with ע in some words, and some forms show a guttural lost altogether (Epstein 1960; Kutscher 1971; Bar-Asher Siegal 2013, ch. 2):

| process | example | from | meaning |
|---|---|---|---|
| ʿ > ʾ / lost | אַמַּאי | *ʿal mā* (Jastrow s.v. אַמַּאי: "= עַל מַאי") | why? |
| ʿ lost in a compound | הָאִידָּנָא | *hā ʿiddānā* | now (lit. "this time") |
| ʿ lost | הַשְׁתָּא | *hā šaʿtā* "this hour" (Jastrow s.v.: "= הָא שַׁעְתָּא") | now |
| ḥ ~ h in spelling | (manuscript variants) | | |

Checked 2026-10-18: Levias 1900 §159, p. 47 (הָאִידָּנָא = הָא + עִדָּנָא), §121, p. 37 (הַשְׁתָּא from הָא שַׁעְתָּא) and §170, p. 50 (א־ as a short form of עַל, which supports אַמַּאי < *ʿal mā*). In the incantation bowls ח and ה are no longer distinguished, the other gutturals are mostly kept, and one bowl (No. 6) loses ע (Montgomery 1913, pp. 29–30).

The Bavli itself mocks Galileans for confusing gutturals (b. Eruvin 53b), which shows the process was felt in Palestine; Babylonian Aramaic shows its own, partly different, losses.

### 2.2 Final consonants

JBA often drops final *n*, *m* and *t* and final unstressed vowels (Margolis 1910; Epstein 1960; Bar-Asher Siegal 2013):

| BA / targum | JBA | meaning |
|---|---|---|
| תְּרֵין | תְּרֵי | two (m.) |
| ־הוֹן "their" | ־הוּ (־יְהוּ) | 3 m. pl. suffix |
| אִנּוּן "they" | אִינְהוּ | they |
| ־ִין (m. pl. abs.) | ־ִי (often) | plural absolute |
| קְטַלַת "she killed" | קְטַלָה, קְטַלָא | 3 f. sg. perfect |
| אֲמַר לֵהּ | אֲמַר לֵיהּ, written also אמר ליה | "he said to him" |

Checked 2026-10-18: Margolis 1910 §8b (Tab. II, p. 16: אינהו), §23 (p. 32: תרי, archaic תרין), §8b (־הו, archaic ־הון), §19 (Tab. VI, p. 28: m. pl. ־י) and §28a (p. 36: 3 f. sg. perfect ־ה, ־א, archaic ־ת). All rows agree. The derivations in the next table follow Jastrow; they were compared with Levias 1900 in the second pass (below).

### 2.3 Contractions

Many of the most frequent JBA words are contractions of two words (Jastrow s.vv.):

| JBA | from | meaning |
|---|---|---|
| אִיכָּא | *ʾīṯ kā* "there is here" | there is |
| לֵיכָּא | *lā ʾīṯ kā* | there is not |
| הָכָא | *hā kā* | here |
| הָתָם | *hā tām* | there |
| הַשְׁתָּא | *hā šaʿtā* | now |
| אַמַּאי | *ʿal mā* | why? |
| מַאי | *mā* + *hī* (Jastrow s.v. מַאי: "= מא הי"); disputed | what? |
| נִיהוּ | *hū* after a vowel or n (Jastrow s.v.: "= אִיהוּ, mostly after an open syllable") | he (is) |
| תָּא | imperative of אתי "come" | come! |
| קָאָמַר | קָא (Jastrow: from קָאֵם "standing") + אָמַר | he is saying |
| דְּקָאָמַר | ד + קָא + אָמַר | who/that is saying |

Checked 2026-10-18 (second pass): Levias 1900 §167, p. 50 (אִיכָּא = אִית + כָּא), §168 (לֵיכָּא = לֵית + כָּא, and לֵית = לָא + אִית), §158, p. 47 (הָכָא, הָתָם: הָא + כָּא, הָא + תָּם), §121, p. 37 (הַשְׁתָּא), §170, p. 50 (אַמַּאי, with א־ for עַל), §122, p. 37 (מַאי = מָה הִיא; in §129 n. 3 he allows also מָא דֵין or מָא הוּא in some cases) and §160, p. 48 with §66 (קָא, קָאֵי, קְ־ before the participle, from קום with its final consonant dropped). These rows agree. Disagreement on נִיהוּ: Levias §124, p. 37, reports Luzzatto and Nöldeke, who derive it from the personal pronoun with a prefixed נ־ (Nöldeke: a worn-down *hen* / *in*), not from אִיהוּ after an open syllable as Jastrow has it (both given). תָּא and the ד־ compounds were not compared (unchecked).

For tokenisation, a contraction that is one word in writing stays one token (אִיכָּא); a sequence of separate clitics (דְּ־קָא־אָמַר) is split.

### 2.4 Assimilation in the t-stems

The t of the reflexive prefix assimilates fully to a following dental or sibilant, and the prefix vowel is written with י: אִיתְּמַר > אִתְּמַר "it was said" (797× איתמר, 231× אתמר); אִיבַּעְיָא "it was asked" (אִתְבְּעִי > אִיבְּעִי); אִידְּכַר "he remembered"; אִיקְּרִי "he was called" ([verbs.md](verbs.md) §2).

## 3. Reading traditions

| tradition | witness | value |
|---|---|---|
| Yemenite | oral reading, recorded by Morag (1988) | the most conservative living tradition; Morag treats it as continuous with Babylonia |
| Early Eastern manuscripts | Geniza fragments, Yemenite and Persian MSS, some with Babylonian vocalisation | the best evidence for forms (Morgenstern 2011); challenges some features of the Yemenite tradition |
| Incantation bowls | original texts, unvocalised | spelling without medieval copying |
| Printed Vilna (1880–1886) and its predecessors | the standard text, unpointed | influenced by medieval copyists from Europe; normalised spellings |
| Ashkenazi and Sephardi yeshiva pronunciation | oral | carries Hebrew habits; not linguistic evidence |

The digital texts on disk are all derived from printed editions ([mova.md](mova.md) §1). Their spelling is the Vilna-type spelling, and their pointing (where present, e.g. the William Davidson vocalised edition — CC BY-NC, tests only) is modern.

## 4. Stress and vowels (reconstruction)

Stress is mostly on the last syllable of the stem, as in other Aramaic; but the loss of final vowels and consonants left many words stressed on the penultimate in their spoken form, e.g. 3 m. pl. perfect *qəṭál* for *qəṭalū* in forms written קְטוּל (Margolis 1910 §28 describes a 3 m. pl. perfect without an ending and with *u* in the stem). These details are not visible in the unpointed text and do not matter for a model of the written language; they matter for the dictionary's pronunciation guide.

The Elamur team
