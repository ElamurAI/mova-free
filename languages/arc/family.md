# The Aramaic family and the Jewish varieties

This file places the five Jewish varieties Mova distinguishes (`arc`, `arc-x-targum`, `tmr`, `jpa`, `arc-x-zohar`) inside the history of Aramaic, and ends with a table of the features that tell them apart from each other and from Hebrew. Conventions (transliteration, glossing, citing) are in [README.md](README.md).

## 1. Aramaic in one page

Aramaic is a **Northwest Semitic** language, a sister of Canaanite (Hebrew, Phoenician, Moabite) and of Ugaritic. It is attested for about three thousand years without a break, from the inscriptions of the Aramaean kingdoms of Syria in the 10th–9th c. BCE to the Neo-Aramaic languages still spoken today (Fitzmyer 1979, "The Phases of the Aramaic Language"; Gzella 2015, *A Cultural History of Aramaic*; Creason 2004, "Aramaic", in Woodard ed., *The Cambridge Encyclopedia of the World's Ancient Languages*).

The usual division into phases goes back to Fitzmyer (1979) and, with some changes, is still used (Gzella 2015, ch. 1; Kaufman 1992, "Languages (Aramaic)", *Anchor Bible Dictionary* 4: 173–178):

| phase | dates (approx.) | what | ISO 639-3 |
|---|---|---|---|
| Old Aramaic | 10th c.–c. 700 BCE | royal inscriptions of the Aramaean city states: Tel Dan, Sefire, Zakkur, Bar-Rakib; Tell Fekheriye (Akkadian–Aramaic bilingual) | `oar` |
| Imperial (Official) Aramaic | c. 700–200 BCE | the chancery language of the Neo-Assyrian, Neo-Babylonian and above all the Achaemenid Persian empire, from Egypt to Afghanistan: the Elephantine papyri, the Arshama letters, the Bactrian documents; **Biblical Aramaic** of Ezra and (in a later form) Daniel | `arc` |
| Middle Aramaic | c. 200 BCE–200 CE | the literary Aramaic of the Dead Sea Scrolls (Genesis Apocryphon, 11Q Targum of Job), Nabataean, Palmyrene, Hatran inscriptions; the formation of the **targumic literary language** | `arc` (no separate code) |
| Late Aramaic | c. 200–1200 CE | the dialects split into **West**: Jewish Palestinian (`jpa`), Samaritan (`sam`), Christian Palestinian; and **East**: Syriac (`syc`), Jewish Babylonian (`tmr`), Mandaic (`myz`) | `jpa`, `tmr`, `syc`, `myz`, `sam` |
| Modern (Neo-) Aramaic | from c. 1200 CE | Western Neo-Aramaic of Maaloula; Ṭuroyo; North-Eastern Neo-Aramaic (Christian and Jewish dialects of Kurdistan, Urmia, Zakho …); Neo-Mandaic | many (`amw`, `tru`, `aii`, `cld`, `trg`, `huy`, `bjf`, `mid` …) |

The **Zohar's Aramaic** (`arc-x-zohar`) stands outside this sequence: it is a **learned literary language** of 13th-century Castile, built from the Aramaic its authors read (Onkelos, the Bavli, the targums) and shaped by their medieval Hebrew (Scholem 1941, *Major Trends in Jewish Mysticism*, lecture 5; Liebes 1993, *Studies in the Zohar*; see [../arc-x-zohar/](../arc-x-zohar/)).

## 2. Aramaic in Jewish history

- **The Persian period.** After the return from Babylon, Aramaic was the language of the imperial administration in Judah too. The book of Ezra quotes official correspondence in Aramaic (Ezra 4:8–6:18; 7:12–26), and Nehemiah 8:8 has been read since antiquity as describing an Aramaic rendering of the Torah reading (b. Megillah 3a takes מְפֹרָשׁ there as "targum"). The square "Assyrian" script (כְּתָב אַשּׁוּרִי) in which Hebrew has been written ever since is the Aramaic chancery script (b. Sanhedrin 21b–22a; Naveh 1982, *Early History of the Alphabet*).
- **Daniel.** Daniel 2:4b–7:28 is in Aramaic, and the switch happens in mid-verse: "the Chaldeans spoke to the king in Aramaic (אֲרָמִית):" — and the rest is Aramaic. The court tales (ch. 2–6) are usually dated to the late Persian or early Hellenistic period, the vision of ch. 7 to the 160s BCE (Collins 1993, *Daniel*, Hermeneia, introduction; Rosenthal 1961 §§1–4 on the language).
- **Bilingual Judaism.** From the Second Temple period to the early Islamic period, Jews in Palestine and Babylonia spoke Aramaic and studied in both Hebrew and Aramaic. The liturgy kept Aramaic pieces that are still recited: the Qaddish, Kol Nidre, *Ha laḥma ʿanya* and *Ḥad gadya* in the Passover Haggadah, *Yequm purqan*, *Berikh shemeh* (from the Zohar).
- **Targum.** The synagogue reading was accompanied by an oral Aramaic rendering, verse by verse (m. Megillah 4:4, 4:10). The written targums that survive are Onkelos (Torah) and Jonathan (Prophets), which became authoritative in Babylonia, and the Palestinian targums (Neofiti, the Fragment targums, the Geniza fragments, Pseudo-Jonathan) (Alexander 1992, "Targum, Targumim", *Anchor Bible Dictionary* 6: 320–331; Flesher and Chilton 2011, *The Targums: A Critical Introduction*).
- **The Talmuds.** The Mishnah (c. 200 CE) is in Hebrew. The two Talmuds that comment on it are mostly Aramaic: the Jerusalem Talmud in Jewish Palestinian Aramaic (`jpa`, edited c. 400 CE), the Babylonian Talmud in Jewish Babylonian Aramaic (`tmr`, edited 6th–7th c.), both mixed with Hebrew quotations and Hebrew traditions (baraitot).
- **After the Talmud.** Aramaic stayed a written language of learning: geonic responsa, legal formulas (the *ketubbah* and the *get* are still written in Aramaic), the Zohar (late 13th c.), and Aramaic poems of the Kabbalists (e.g. Isaac Luria's Sabbath hymns). Spoken Jewish Neo-Aramaic survived in Kurdistan until the 1950s and is still spoken by some elderly people in Israel (Khan and Noorlander eds. 2021, *Studies in the Grammar and Lexicon of Neo-Aramaic*, open access; research library `hbobook-obp0209-khan-studies-grammar-lexicon-neo-aramaic`).

## 3. Where each Jewish variety sits

```
Northwest Semitic
├── Canaanite ── Hebrew (hbo, hbo-x-mishnaic, he)
└── Aramaic
    ├── Old Aramaic (oar)
    └── Imperial Aramaic (arc) ───────────── Biblical Aramaic (arc)
         │
         ├── Middle: Qumran Aramaic; Jewish Literary Aramaic ── Targum Onkelos, Jonathan (arc-x-targum)
         │
         ├── Late West ─── Jewish Palestinian Aramaic (jpa): Yerushalmi, midrash, Palestinian targums
         │            ├── Samaritan Aramaic (sam)
         │            └── Christian Palestinian Aramaic
         │
         └── Late East ─── Jewish Babylonian Aramaic (tmr): Bavli, geonim, incantation bowls
                      ├── Syriac (syc)
                      └── Mandaic (myz)

Medieval literary: Zoharic Aramaic (arc-x-zohar) ← Onkelos + Bavli + medieval Hebrew
```

- **Biblical Aramaic** is a late form of the Imperial standard. Ezra's documents keep older spellings (for example ז for etymological *ḏ* in some words, the 3mpl suffix ־הֹם beside ־הוֹן) and Daniel shows later features (Rosenthal 1961 §§1–4; Kutscher 1970, "Aramaic", *Current Trends in Linguistics* 6).
- **The targumic literary language** of Onkelos and Jonathan continues a standard literary Aramaic of the Hasmonean–Roman period with Western roots (it is close to Qumran Aramaic) and shows Eastern (Babylonian) features from its final editing (Kutscher 1958, "The Language of the Genesis Apocryphon: A Preliminary Study", *Scripta Hierosolymitana* 4; Tal 1975, *The Language of the Targum of the Former Prophets and its Position within the Aramaic Dialects*; Kaufman 1985, "On methodology in the study of the Targums and their chronology", *JSNT* 23). Its exact home is debated; the conventional view is "Palestinian origin, Babylonian redaction" (Cook 1986, *Rewriting the Bible: The Text and Language of the Pseudo-Jonathan Targum*, PhD UCLA; Fassberg 2012, in Lim and Collins eds.).
- **Jewish Palestinian Aramaic** is the spoken Western Aramaic of Galilee in the Talmudic period, written as a literary dialect (Dalman 1905; Kutscher 1976, *Studies in Galilean Aramaic*; Sokoloff 2002 [2nd ed.], *A Dictionary of Jewish Palestinian Aramaic*).
- **Jewish Babylonian Aramaic** is the Eastern Aramaic of the Jews of Sasanian Mesopotamia, closely related to Mandaic and to Syriac (Margolis 1910; Epstein 1960, *A Grammar of Babylonian Aramaic* (Hebrew); Sokoloff 2002, *A Dictionary of Jewish Babylonian Aramaic*; Bar-Asher Siegal 2013, *Introduction to the Grammar of Jewish Babylonian Aramaic*).
- **Zoharic Aramaic** is not anyone's mother tongue. Its vocabulary and forms come mostly from Onkelos and the Bavli, its syntax and semantics often from medieval Hebrew, and it has its own invented words and its own regularised "errors" (Scholem 1941, lecture 5; Liebes 1976, *Peraqim be-millon sefer ha-Zohar*).

## 4. What all Aramaic shares, against Hebrew

These traits hold for every variety in this manual, with the exceptions named in the variety folders.

1. **No prefixed definite article.** Definiteness is a suffix: the **determined (emphatic) state** in ־ָא *-ā* (מַלְכָּא *malkā* "the king"), against Hebrew הַמֶּלֶךְ *ham-meleḵ*. In the Eastern dialects (Syriac, JBA) the suffix lost its definite meaning and became the plain form of the noun ([nouns.md](nouns.md); [../tmr/](../tmr/)).
2. **The relative and genitive particle די / ד־** "who, which, that; of" for Hebrew אֲשֶׁר, שֶׁ־ and for many construct chains: מַלְכוּתָא דִּי בָבֶל ~ Hebrew מַלְכוּת בָּבֶל.
3. **Proto-Semitic interdentals and *ḍ*:** where Hebrew has ז ש צ for the interdentals *ḏ ṯ ṯ̣*, Aramaic has ד ת ט; where Hebrew has צ for *ḍ*, (older) Aramaic has ק and later ע: Hebrew זָהָב / Aramaic דְּהַב "gold", Hebrew שָׁלֹשׁ / Aramaic תְּלָת "three", Hebrew אֶרֶץ / Aramaic אֲרַע (older אֲרַק, Jer 10:11) "earth" ([script-phonology.md](script-phonology.md) §4).
4. **Verb stems:** the basic stem is called **peal** (פְּעַל), not *qal*; the doubled stem **pael**; the causative is **haphel/aphel/shaphel** (Hebrew *hiphil*); the passives and reflexives are formed with a prefix **hit-/it-** (*hithpeel*, *ithpaal*) and, in older Aramaic, with internal vowel change (*peil*, *hophal*). There is no *niphal* ([verbs.md](verbs.md)).
5. **The participle as a present tense.** In Late Aramaic the active participle with a pronoun is the normal present (and often the narrative past): JBA קָאָמֵינָא "I am saying". Biblical Aramaic already leans this way ([verbs.md](verbs.md) §8).
6. **No *waw*-consecutive.** Aramaic narrative uses the perfect, or the participle; it never has the Hebrew *wayyiqtol*.
7. **Vocabulary:** Aramaic אֲזַל "go" (Hebrew הָלַךְ), חֲזָא "see" (Hebrew רָאָה; JPA חֲמָא), עֲבַד "do, make" (Hebrew עָשָׂה), בְּרַם "but", בַּר "son" (Hebrew בֵּן, though בְּנִין "sons" keeps the n), אֱנָשׁ "man" (Hebrew אָדָם, אִישׁ), טָב "good", סַגִּי "much", חַד "one", דְּנָא "this" (Hebrew זֶה).
8. **Pronouns:** 1sg אֲנָה/אֲנָא (Hebrew אֲנִי, אָנֹכִי), 2sg m. אַנְתְּ/אַנְתָּה, 1pl אֲנַחְנָא / אֲנַן (Hebrew אֲנַחְנוּ), 2pl אַנְתּוּן, suffix 3ms ־ֵהּ *-ēh* (Hebrew ־וֹ), 2mpl ־כוֹן *-ḵōn* (Hebrew ־כֶם).

## 5. Telling the five apart: the comparison table

The table is a working tool for a classifier (see [mova.md](mova.md) §5). Forms are typical, written consonantally with the usual vocalisation; each line is discussed in the variety's own folder. Hebrew is given for contrast.

| feature | `arc` Biblical | `arc-x-targum` Onkelos/Jonathan | `jpa` Palestinian | `tmr` Babylonian | `arc-x-zohar` | Hebrew (`hbo`) |
|---|---|---|---|---|---|---|
| "the king" | מַלְכָּא | מַלְכָּא | מלכא / מלכה | מלכא (also "a king") | מלכא | הַמֶּלֶךְ |
| m. pl. determined | ־ַיָּא *-ayyā* (מַלְכַיָּא) | ־ַיָּא | ־ייא / ־יה *-ayyā* | ־ֵי *-ē* (מלכי, רבנן is ־ָן) | ־ייא and ־י mixed | הַ־…־ִים |
| m. pl. absolute | ־ִין *-īn* | ־ִין | ־ין, ־ים in Hebraisms | ־י / ־ין (often ־י) | ־ין | ־ִים |
| relative | דִּי (separate word) | דִּי and ד־ | ד־ (also די) | ד־ | ד־ | אֲשֶׁר, שֶׁ־ |
| object marker | ל־; יָת once (Dan 3:12) | יָת very frequent | ית (Neofiti) and ל־ | ל־; ית rare | ית and ל־ | אֶת |
| "this" (m.) | דְּנָה, דֵּךְ | דֵּין, הָדֵין | הדין, דין | האי, הַאי | דא, דין, האי | זֶה |
| "this" (f.) | דָּא | דָּא, הָדָא | הדא | הא, הָא | דא | זֹאת |
| "these" | אִלֵּין, אֵל | אִלֵּין | אילין, הלין | הני | אלין | אֵלֶּה |
| "he" | הוּא | הוּא | הוא | הוּא, אִיהוּ, ניהו (copula) | איהו (very frequent), הוא | הוּא |
| "they" (m.) | הִמּוֹ, אִנּוּן | אִנּוּן | אינון, הינון | אִינְהוּ | אינון | הֵם |
| "we" | אֲנַחְנָא | אֲנַחְנָא | אנן (Palestinian targums also אנחנא) | אֲנַן | אנן, אנחנא | אֲנַחְנוּ |
| 3mpl suffix "their" | ־הוֹן / ־הֹם | ־הוֹן | ־הון | ־הוּ, ־יְהוּ | ־הון | ־ָם, ־הֶם |
| 3ms imperfect prefix | י־ (but לֶהֱוֵא "let it be") | י־ | י־ | ל־ (and נ־): לֵימָא, נֵימָא | י־ (mostly) | י־ |
| 1pl imperfect | נ־ | נ־ | נ־ | נ־ | נ־ | נ־ |
| 3mpl perfect | ־וּ | ־וּ | ־ון / ־ו (אזלון) | קְטוּל type (סְבוּר); ־וּ archaic | ־ו | ־וּ |
| 1sg perfect | ־ֵת (עַבְדֵת) | ־ִית (עֲבַדִית) | ־ית | ־ִי / ־ִית (חֲזַאי, אֲמַרִי) | ־ית | ־ְתִּי |
| causative stem | haphel (הַ־) mostly, aphel (אַ־) some | aphel | aphel | aphel | aphel | hiphil |
| reflexive stems | hithpeel, hithpaal (also ith-) | ithpeel, ithpaal, ittaphal | ithpeel, ithpaal (it-, at-) | ithpeel, ithpaal, ittaphal (ם assimilated: אִיקְּרִי) | ithpeel | niphal, hithpael |
| "see" | חֲזָה | חֲזָא | חֲמָא (חמי) | חֲזָא (חזי) | חמא, חזא (ת"ח = תָּא חֲזֵי) | רָאָה |
| "now" | כְּעַן, כְּעֶנֶת | כְּעַן | כְּדוֹן | הָשְׁתָּא | השתא, כען | עַתָּה |
| "there is / is not" | אִיתַי / לָא אִיתַי | אִית / לֵית | אית / לית | אִיכָּא / לֵיכָּא, לָאו | אית / לית | יֵשׁ / אֵין |
| "what?" | מָה | מָה | מה, מאן | מַאי | מאי, מה | מָה |
| "why?" | לְמָה | לְמָא | למה | אַמַּאי, מַאי טַעְמָא | אמאי, מאי טעמא | לָמָּה |
| "because" | כָּל־קֳבֵל דִּי | אֲרֵי | ארום, בגין ד־ | מִשּׁוּם ד־, דְּהָא | בגין ד־, דהא | כִּי |
| "but" | בְּרַם | בְּרַם, אֱלָהֵן | ברם, אלא | אֶלָּא, מִיהוּ | אבל, אלא | אֲבָל, אַךְ |
| "like" | כְּ־, כְּדִי | כְּ־, כְּמָא | כמה, הֵיךְ | כִּי, כְּ־, הֵיכִי | כגוונא ד־ | כְּ־, כְּמוֹ |
| durative particle | — | — | — | קָא / קְ־ | קא (rare) | — |
| copula (3ms) | הוּא | הוּא | הוא | ניהו, הוּא | איהו | הוּא |

Checked 2026-10-18 for the four classical columns: Biblical Aramaic against Kautzsch 1884 and Strack 1901, Onkelos and Galilean against Dalman 1905 (§§ 16–19 and the *Anhang* tables) and Stevenson 1924, JBA against Margolis 1910 (Tab. II, VI, XI, XII); details and citations are in each folder. Corrected: the JPA "we" (Dalman p. 106 and Stevenson §3 give only אנן for Galilean / PTM) and the JBA 3 m. pl. perfect (Margolis §28b: the common form is the קְטוּל type, ־וּ is archaic). The Zohar column is measured, not taken from a grammar.

Two rules of thumb fall out of the table:

- A text with **ל־ prefixed imperfects** (לימא, ליהוי, לישקול), **איכא/ליכא**, **מאי**, **האי/הני**, **קא** and the plural in **־ֵי** is Babylonian (`tmr`).
- A text with **חמי** "see", **כדון** "now", **הדין/הדא**, **3mpl perfect in ־ון** and **ארום** is Palestinian (`jpa`).
- A text with **יָת** on almost every definite object, **אֲרֵי** "because", **קֳדָם יְיָ** "before the LORD" where Hebrew has a direct reference to God, and with the **word order of the Hebrew** verse by verse, is targumic (`arc-x-targum`).
- A text with **איהו** as a copula in every other sentence, **ת"ח**, **רזא ד־**, **סטרא** and the names of the sefirot is Zoharic (`arc-x-zohar`).
- A text with **haphel** causatives in ה־, **הִמּוֹ** "they", **דִּי** written separately before every genitive, and Persian administrative words (דָּת, פִּתְגָם, גִּזְבָּר) is Biblical (`arc`).

## 6. Measures

Counted from MACULA Hebrew (WLC text, OSHB morphology, `lang` = `A`; CC BY 4.0), 2026-10-17:

| | words (morph units) |
|---|---:|
| Daniel | 5 687 |
| Ezra | 1 835 |
| Jeremiah 10:11 | 25 |
| Genesis 31:47 | 2 |
| **all Biblical Aramaic** | **7 549** |

These are MACULA's units, where prefixes, the article ־א and suffixes are words of their own. The targums, the Talmuds and the Zohar are many times larger: see each folder's `mova.md`.

## Sources for this file

Fitzmyer 1979; Gzella 2015; Kaufman 1992; Creason 2004; Rosenthal 1961; Kutscher 1970; Tal 1975; Dalman 1905; Margolis 1910; Sokoloff 2002 (both dictionaries); Bar-Asher Siegal 2013; Scholem 1941; Liebes 1976, 1993; Alexander 1992; Flesher and Chilton 2011; Collins 1993; Naveh 1982; Khan and Noorlander 2021; MACULA Hebrew. Full references in [sources.md](sources.md).

The Elamur team
