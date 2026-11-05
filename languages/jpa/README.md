# Jewish Palestinian Aramaic (`jpa`)

Jewish Palestinian Aramaic (JPA), also called **Galilean Aramaic**, is the Western Aramaic of the Jews of the Land of Israel in the Roman and Byzantine periods (3rd–8th c. CE). It is the language of the **Jerusalem Talmud** (the Yerushalmi), of the Aramaic parts of the classical **Palestinian midrashim** (Genesis Rabbah, Leviticus Rabbah, Lamentations Rabbah, Pesiqta de-Rav Kahana), of the **Palestinian targums** (Neofiti, the Fragment targums, the Cairo Geniza fragments, and in a mixed form Pseudo-Jonathan), of **synagogue inscriptions** (Rehov, En Gedi, Hammat Gader) and of Aramaic **liturgical poetry** (piyyut).

Its code is ISO 639-3 `jpa`. Conventions: [../arc/README.md](../arc/README.md). Vowels are given after the grammars (Dalman 1905; Fassberg 1991; Sokoloff 2002b) and, for the Yerushalmi, after the vocalisation of the Guggenheimer edition on Sefaria (a modern editorial vocalisation, CC BY), **as reconstructions**.

## 1. What it is

- **A spoken Western dialect, written.** JPA is the Aramaic Jews spoke in Galilee when the Mishnah was being studied there (Tiberias, Sepphoris, Caesarea). It belongs with Samaritan Aramaic and Christian Palestinian Aramaic to the western branch of Late Aramaic (Kutscher 1976; Sokoloff 2002b, introduction).
- **The sources and their reliability.** The printed Yerushalmi (Venice 1523, from the Leiden manuscript of 1289) was copied by scribes who knew the Bavli better than the Yerushalmi and often "corrected" JPA forms into Babylonian ones. The most reliable JPA texts are therefore the **Geniza fragments** (of the Yerushalmi, the midrashim, the Palestinian targums), the **Leiden manuscript** itself, and **inscriptions** (Kutscher 1976; Sokoloff 2002b). Studies of JPA grammar rest on these (Fassberg 1991 on the Geniza targum fragments; Golomb 1985 on Neofiti).
- **Galilean and Judean.** Rabbinic sources themselves note that Galileans pronounced gutturals differently (b. Eruvin 53b, the anecdotes of the Galilean who asked for "אמר" and could not be understood whether he meant ḥamar "ass", ḥamr "wine", ʿamar "wool" or ʾimmar "lamb"). JPA as we have it is mostly Galilean.
- **The Palestinian targums** translate the Torah with many more expansions than Onkelos; Neofiti (a Vatican manuscript of the 16th c. copying an older text) is the only complete one; the Fragment targums have selected verses; Geniza fragments preserve older copies. **Pseudo-Jonathan** (the "Targum Jonathan on the Torah" of the printed editions) combines Onkelos with Palestinian material and late additions; its language is mixed (Cook 1986).

## 2. What marks it

Against **JBA** (`tmr`) — the clearest contrasts, with counts on 2026-10-17 in the Guggenheimer Yerushalmi (39 tractates, CC BY, 811 794 tokens) and the Wikisource Bavli (CC BY-SA, measurement only, 1 858 077 tokens), unpointed word tokens:

| feature | JPA | Yerushalmi | Bavli | JBA |
|---|---|---:|---:|---|
| "now" | **כְּדוֹן** | 419 | 0 | הַשְׁתָּא (Bavli 974; Yerushalmi 1) |
| "to them" | **לוֹן** | 731 | 0 | לְהוּ (Bavli 4 024; Yerushalmi 9) |
| "there" (also = Babylonia) | **תַּמָּן** | 1 667 | 11 | הָתָם |
| "this" (f.); "this implies" | **הָדָא**, הָדָא אָמְרָה | 2 117 | 23 | הָא |
| "there is" | **אִית** | 880 | 772 (in fixed phrases) | אִיכָּא (Bavli 2 488; Yerushalmi 6) |
| "this" (m.) | הָדֵין | 244 | (396, mostly the Hebrew הַדִּין "the law") | הַאי (Bavli 2 246; Yerushalmi 6) |
| durative particle | — | קָא: 0 | 2 828 | קָא |
| "how" | **הֵיךְ** | 218 | 0 | הֵיכִי |
| "also" | **אוּף** | 188 | 0 | נַמִי |
| "because, in order to" | **בְּגִין** | 250 | 2 | מִשּׁוּם, כִּי הֵיכִי |
| "himself" | **גַּרְמֵיהּ** | 60 | 1 | נַפְשֵׁיהּ |
| "see" | **חֲמָא, חָמֵי** | 319 (חמי + חמא) | (חמא is also a name) | חֲזָא |
| 3 m. pl. perfect | **־וֹן**: אַזְלוֹן, אֲתוֹן, זְבַנּוֹן | אזלון 28 | 0 | ־וּ / *qəṭūl* |
| 3 m. sg. imperfect | **יְ־**: יֵיזִיל | | | לְ־ / נְ־: לֵיזִיל |
| m. pl. determined | ־ַיָּא | | | ־ֵי |

The cue words are so cleanly divided that a **classifier counting a dozen words** separates Yerushalmi from Bavli Aramaic almost perfectly ([mova.md](mova.md) §3).

Against the **targumic** language (`arc-x-targum`): JPA has חֲמָא "see" (targum חֲזָא), כְּדוֹן "now" (targum כְּעַן), the 3 m. pl. perfect in ־וֹן, the frequent "ethical dative" (נָחַת לֵיהּ "he went down (for himself)"), אֲרוּם / בְּגִין "because" (targum אֲרֵי), many Greek loanwords, and a looser translation style in the Palestinian targums.

Against **Hebrew**: the whole Aramaic grammar; and in the Yerushalmi, constant switching to Mishnaic Hebrew, more frequent than in the Bavli (many amoraic statements of the Palestinian sages are in Hebrew).

## 3. The files of this folder

| file | what |
|---|---|
| [README.md](README.md) | this overview |
| [phonology-orthography.md](phonology-orthography.md) | weak gutturals, plene spelling, final ־ה / ־א, ־ן / ־ם, inscriptions |
| [nouns-pronouns.md](nouns-pronouns.md) | states, plural, pronouns, suffixes (לוֹן), demonstratives (הָדֵין, הָדָא), interrogatives |
| [verbs.md](verbs.md) | stems, the perfect in ־ון and ־ית, the imperfect in יְ־, participles, ethical dative, paradigms |
| [particles-syntax.md](particles-syntax.md) | ד־, objects (ית in Neofiti, ל־), negation, existentials, narrative style |
| [lexicon.md](lexicon.md) | core vocabulary, 40+ JPA–JBA word pairs, Greek and Latin loanwords, Hebrew elements |
| [texts.md](texts.md) | annotated passages: the Palestinian targums on Gen 4:8 and Exod 15:18, Yerushalmi stories (Bava Metzia 2:5, Maaser Sheni 4:6, Berakhot 2:8), and the language of synagogue inscriptions |
| [mova.md](mova.md) | what Mova would need |
| [sources.md](sources.md) | sources and wanted works |

The Elamur team
