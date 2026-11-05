# Targumic Aramaic (`arc-x-targum`)

The literary Aramaic of the two "official" targums of the Babylonian Jewish tradition: **Targum Onkelos** to the Torah and **Targum Jonathan** to the Prophets. It is the Aramaic most Jews have read most often, because Onkelos is printed beside the Hebrew in every rabbinic Bible (*Miqraot Gedolot*) and the Talmud requires that the weekly portion be read "twice in Scripture and once in targum" (b. Berakhot 8a–b, *shnayim miqra ve-eḥad targum*). It is still read aloud in the synagogue in Yemen and by Yemenite communities.

The code `arc-x-targum` is Mova's private-use extension of `arc` ([../arc/README.md](../arc/README.md)). In the corpus registry it also covers the late targums to the Writings (Psalms, Job, Proverbs, the five Scrolls, Chronicles), which are linguistically mixed and described here only briefly (§5). The **Palestinian** targums (Neofiti, the Fragment targums, the Geniza targums) and **Pseudo-Jonathan** are filed under `jpa` ([../jpa/](../jpa/)).

Conventions (transliteration, glossing, citing) are those of [../arc/README.md](../arc/README.md). This folder describes how Targumic Aramaic differs from Biblical Aramaic ([../arc/](../arc/)) and from the other varieties.

## 1. What it is

- **A translation language.** Onkelos renders the Hebrew Torah verse by verse and mostly word by word, with Aramaic words in the Hebrew order, and departs from the Hebrew where theology, law, clarity or the poetry require. Jonathan to the Prophets is freer and often expands into midrash ([translation-syntax.md](translation-syntax.md)).
- **A standard literary dialect, not a vernacular.** Its grammar continues the "Standard Literary Aramaic" of the late Second Temple period, the language of the Aramaic Dead Sea Scrolls (Genesis Apocryphon, 11Q Targum of Job), with Western (Palestinian) roots and an Eastern (Babylonian) final editing (Kutscher 1958; Tal 1975; Kaufman 1985; Cook 1986; Fassberg 2012). It is therefore close to Biblical Aramaic in morphology, and different from both spoken dialects of the Talmudic period (JPA and JBA).
- **Dates.** The consensus places the formation of Onkelos and Jonathan in Palestine in the 1st–3rd c. CE and their final redaction, and canonisation, in Babylonia in the 3rd–5th c. (Alexander 1992; Flesher and Chilton 2011, ch. 4–5). Both were provided with a Babylonian Masorah and a Babylonian vocalisation (Berliner 1877, *Die Massorah zum Targum Onkelos*).
- **Names and tradition.** The Bavli attributes the targum of the Torah to Onkelos the proselyte "from the mouth of R. Eliezer and R. Joshua" and the targum of the Prophets to Jonathan ben Uzziel "from the mouth of Haggai, Zechariah and Malachi" (b. Megillah 3a). The Yerushalmi tells a parallel story of the Greek translation of Aquila (עֲקִילַס) (y. Megillah 1:9, 71c; numbered 1:11 in some editions), and most scholars think the name "Onkelos" is a Babylonian form of "Aquila" transferred to the Aramaic version (Alexander 1992). The Bavli calls Onkelos "our targum" (תַּרְגּוּם דִּידַן, b. Kiddushin 49a).

## 2. What marks it

Against **Biblical Aramaic** (`arc`):

| feature | Biblical Aramaic | Targumic |
|---|---|---|
| causative stem | haphel הַ־ (mostly) | **aphel** אַ־: אַפְרֵשׁ "he separated" (Gen 1:4), אַפֵּיקְתָּךְ "I brought you out" (Exod 20:2) |
| internal passives (peil, hophal) | alive | gone except the passive participle; passives are **ithpeel / ithpaal / ittaphal** |
| 1 sg. perfect | ־ֵת (עַבְדֵת) | ־ִית (עֲבַדִית, יְהַבִית) |
| object marker | ל־, יָת once | **יָת** on most definite objects (as Hebrew אֶת): 3 238 יָת / וְיָת in Onkelos against 3 450 אֶת / וְאֶת in the Hebrew Torah (our count, 2026-10-17) |
| relative | דִּי separate | דִּי (still frequent) and the prefix ד־ |
| "because, that" | כָּל־קֳבֵל דִּי, דִּי | **אֲרֵי** (936× in Onkelos for Hebrew כִּי, 988×) |
| "this" | דְּנָה, דָּא | דֵּין, דָּא; with ה־: **הָדֵין**, **הָדָא**, **הָאִלֵּין** |
| 3 m. pl. pronoun | הִמּוֹ, אִנּוּן | אִנּוּן only |
| 3 m. pl. suffix | ־הוֹן, ־הֹם | ־הוֹן only |
| "now" | כְּעַן | כְּעַן, also for Hebrew נָא "please" (Gen 22:2) |
| the divine name | — | יְיָ (written as two yods), אֱלָהָא |

Against the **spoken dialects** of the Talmudic period:

| feature | Targumic | JPA (`jpa`) | JBA (`tmr`) |
|---|---|---|---|
| "see" | חֲזָא | חֲמָא | חֲזָא |
| "there is not" | לֵית | לֵית | לֵיכָּא |
| 3 m. sg. imperfect | יְ־ | יְ־ | לְ־ / נְ־ |
| m. pl. determined | ־ַיָּא | ־ַיָּא | ־ֵי |
| object | יָת | יָת and ל־ | ל־ |
| "because" | אֲרֵי | אֲרוּם, בְּגִין ד־ | מִשּׁוּם ד־ |
| 3 m. pl. perfect | ־וּ | ־וּן | ־וּ / ־וּן |
| "this" | דֵּין, הָדֵין | הָדֵין | הַאי |

Against **Hebrew**: the whole Aramaic grammar ([../arc/family.md](../arc/family.md) §4), and the translational features: מֵימְרָא, יְקָרָא, שְׁכִינְתָּא and קֳדָם for God's presence and action ([translation-syntax.md](translation-syntax.md) §3).

## 3. Editions and reading traditions

| edition / tradition | what | status |
|---|---|---|
| Yemenite manuscripts with **Babylonian (supralinear) vocalisation** | the best witnesses of the Babylonian reading of Onkelos and Jonathan | basis of Sperber 1959–1973, *The Bible in Aramaic* (Brill; in copyright) |
| Yemenite *Taj* (Torah with Onkelos, Tiberian points in the Yemenite tradition) | living oral tradition | on Sefaria as "Targum Onkelos, vocalized according to the Yemenite Taj" (CC BY-SA) |
| Sabbioneta 1557 and other early prints | Tiberian vocalisation | basis of Berliner 1884 (public domain) |
| *Miqraot Gedolot* (rabbinic Bibles) | the printed text with late Tiberian points | Sefaria "Onkelos Genesis" etc. and "Mikraot Gedolot" for Jonathan (public domain) |

The printed Tiberian vocalisation is late and not consistent; for grammar, the Babylonian-Yemenite tradition is the authority (Dalman 1905, introduction; Morag 1988; Fassberg 2012). Our quotations use the public-domain Sefaria texts as they are printed, and say so.

## 4. The files of this folder

| file | what |
|---|---|
| [README.md](README.md) | this overview |
| [phonology-orthography.md](phonology-orthography.md) | spelling, vocalisation traditions, sound changes against BA |
| [nouns-pronouns.md](nouns-pronouns.md) | the noun and its states, pronouns, suffixes, demonstratives |
| [verbs.md](verbs.md) | stems, paradigms, weak roots, the rendering of Hebrew tenses |
| [translation-syntax.md](translation-syntax.md) | translation technique, theological renderings, word order, יָת, די, the syntax of a translation |
| [lexicon.md](lexicon.md) | core vocabulary, Hebrew–Aramaic equivalents, loanwords, theological terms |
| [texts.md](texts.md) | annotated verses of Onkelos and Jonathan beside the Hebrew |
| [mova.md](mova.md) | what Mova would need: data, counts, alignment with Hebrew trees |
| [sources.md](sources.md) | sources and wanted works |

## 5. The targums to the Writings

The Aramaic versions of Psalms, Job, Proverbs, Ruth, Song of Songs, Lamentations, Ecclesiastes, Esther (two: Rishon and Sheni) and Chronicles are later (perhaps 5th–8th c. CE, some later), Palestinian in origin, and **mixed** in language: their base is Targumic (Onkelos-like), with many JPA forms, and in Proverbs even Syriac features (Targum Proverbs is close to the Peshitta). They are full of midrashic expansion: Targum Ruth 1:1 lists "ten famines" that came upon the world; Targum Song of Songs reads the book as the history of Israel (Flesher and Chilton 2011, ch. 12–14; Alexander 1992). In the registry they are `arc-x-targum`; a Mova classifier should expect JPA features in them ([../jpa/README.md](../jpa/README.md)).

```
Aramaic:   וַאֲמָרַת רוּת … אֲרוּם תְּאִיבָא אֲנָא לְאִתְגַּיָּרָא
Translit.: wa-ʾămāraṯ rūṯ … ʾărūm təʾīḇā ʾănā lə-ʾiṯgayyārā
Gloss:     and-say.PEAL.PF.3FS Ruth … because desiring.F I to-convert.ITHPA.INF
English:   And Ruth said: "… for I want to become a proselyte"
Source:    Targum Ruth 1:16 (Mikraot Gedolot, Sefaria, public domain)
```

Note the JPA/late conjunction אֲרוּם "because" (Onkelos has אֲרֵי) and the rabbinic term "to convert" (ithpaal of גיר).

The Elamur team
