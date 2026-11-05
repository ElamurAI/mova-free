# Phonology and orthography of Targumic Aramaic

The script, the consonant inventory and the regular correspondences with Hebrew are those of [../arc/script-phonology.md](../arc/script-phonology.md). This file gives what is particular to Onkelos and Jonathan: the spelling of the printed and manuscript texts, the reading traditions, and the sound changes since Biblical Aramaic.

References: Dalman 1905, *Grammatik des jüdisch-palästinischen Aramäisch* (which treats Onkelos and Jonathan as its "Judean" dialect, with paradigms); Berliner 1884, *Targum Onkelos*, vol. 2; Morag 1988; Fassberg 2012; Sperber 1959–1973. Text: Sefaria PD editions of Onkelos ("Onkelos Genesis" …) and Jonathan ("Mikraot Gedolot"), quoted as printed.

## 1. Spelling

**Matres lectionis are used more than in Biblical Aramaic, less than in the Talmuds.**

| feature | Biblical Aramaic | Onkelos (printed) | example |
|---|---|---|---|
| final -ā of the determined state | ־א or ־ה | ־א almost always | שְׁמַיָּא, אַרְעָא (Gen 1:1) |
| internal ī, ē | often defective | often י | רֵיקַנְיָא "empty" (Gen 1:2); מֵימְרָא |
| internal ū, ō | often defective | often ו | חֲשׁוֹכָא "darkness" (Gen 1:2); נְהוֹרָא "light" (Gen 1:3) |
| the divine name | — | יְיָ (two yods with qameṣ) | Gen 1:1 |
| the relative | דִּי | דִּי and ד־ | דִּי רְחַמְתָּ (Gen 22:2); דְּרָחֵשׁ (Gen 1:26) |

The writing of the tetragrammaton as **יְיָ** is a scribal substitute for יהוה; it is read as the Hebrew name is read (*Adonai*). It is not an Aramaic word, and a tokenizer should map it to the same lemma as Hebrew יהוה ([mova.md](mova.md) §3).

## 2. Vocalisation

Three layers exist (see [../arc/script-phonology.md](../arc/script-phonology.md) §1.3):

1. **Babylonian supralinear** vocalisation in Yemenite and Babylonian manuscripts, which reflects the Babylonian reading of Onkelos. It distinguishes fewer vowel qualities than Tiberian: one sign for a/ă, one for e/ē (Morag 1988; Fassberg 2012).
2. **Yemenite Tiberian** (the *Taj*), a transfer of the Babylonian tradition into Tiberian signs, still read aloud.
3. **European Tiberian** in the printed editions, often influenced by Biblical Aramaic and by Hebrew, and inconsistent.

Typical differences between the printed text and the Babylonian-Yemenite tradition (Dalman 1905; Berliner 1884):

| point | printed Tiberian | Babylonian-Yemenite |
|---|---|---|
| peal perfect 3ms | קְטַל | קְטַל (same) |
| 1 sg. perfect | קְטָלִית (Sefaria) / קְטַלִית (Dalman 1905, p. 402) | *qəṭalīṯ* (pataḥ: יְהַבִית Gen 9:3 in Berliner's list of "old targum" readings) |
| determined plural | ־ַיָּא | ־ַיָּא, often *-ayyā* with a single y written |
| shewa after the article-like ה of הָדֵין | הָדֵין | *hāḏēn* |
| 1 sg. imperfect prefix | אֶ־ (Sefaria: segol) | אִ־ (no segol in the Babylonian system) |

The exact vowels matter little for a model of the consonantal text; they matter for a dictionary and for reading aloud. Mova should keep whatever pointing the source has in FORM and match by consonants.

## 3. Sound changes since Biblical Aramaic

| change | Biblical Aramaic | Targumic | example |
|---|---|---|---|
| *ḍ > ʿ complete | אֲרַק beside אֲרַע | אֲרַע only | אַרְעָא (Gen 1:1) |
| שׂ → ס in spelling | שַׂגִּיא, עֲשַׂר | סַגִּי, עֲסַר (both spellings occur in prints) | יִסְגֵי "will grow" (Tg Isa 52:13) |
| haphel → aphel | הַ־ | אַ־ | אַפְרֵשׁ (Gen 1:4), אַסְקֵהּ (Gen 22:2) |
| נ assimilation | written (nasalisation): מַנְדַּע, יִנְתֵּן | assimilated, except before ה and ע (and a few roots before ז, צ): יִתֵּן (Gen 28:20) against יִנְהוֹם, יִנְזֵיק (Gen 26:11) | מַנְדְּעָא "knowledge" (Sefaria מַנְדְעֵי Deut 4:42) is from ידע, not an I-n root |
| t of the t-stems | הִתְ־, also with sibilant metathesis | אִתְ־, assimilated to ת, ט, ד, metathesis with ש, ס, צ, ז (צ: ת → ט; ז: ת → ד): אִיטַּמַּר (Gen 3:8, < *ʾiṯṭammar*) | |
| initial ʾ of the reflexive | ה | א | אִתְאֲמַר; יִתְאֲמַר (Gen 22:14) |
| 2 m. sg. pronoun | אַנְתְּ | אַתְּ (assimilated) | אָן אָתְּ "where are you?" (Gen 3:9) |
| final ־ם of suffixes | ־הֹם, ־כֹם | ־הוֹן, ־כוֹן only | לְוָתְכוֹן (Exod 3:14) |
| diphthong ay | בַּיִת, בַּיְתָא | בֵּית, בֵּיתָא; בַּיְתָא in some forms | בְּבֵיתָךְ (Deut 6:7) |

Checked 2026-10-18 in part: Dalman 1905 §16.1, p. 107 (Onkelos אַתְּ), §16b, p. 109 (only ־הוֹן, ־כוֹן) and *Anhang* VI.1, p. 403 (aphel with א in all forms). Second pass 2026-10-18: Dalman 1905 §9, p. 53 (the Yemenite Onkelos writes ס for every שׂ except in some proper names; the non-Yemenite targum texts keep שׂ much more: hence both spellings in the prints; the Sefaria Onkelos has סגי 42× but עשר 66× against עסר 17×, counted 2026-10-18), §13.5a, p. 87 (*ai* survives less in the targum than in Galilean; *au* is almost always ō), §15.4b α, p. 103 (assimilation and metathesis of the t-prefix, for all the dialects and Onkelos), §66.1, p. 291 (Onkelos leaves נ unassimilated always before ה, ע and sometimes before ז, צ: יִנְהוֹם, יִנְעֵי, יִנְזֵיק Gen 26:11, יִנְצוֹן Exod 21:18), §69.2, p. 308 (the n-insertion in מַנְדַּע belongs to ידע); Berliner 1884, vol. 2, p. 135 (the Babylonian system, the Sabbioneta edition and cod. de Rossi 7 have no segol, so the 1 sg. imperfect is pointed with ḥireq: Gen 6:7, 9:5, 18:26) and p. 147 (words pointed with pataḥ in the "old targum" and with qameṣ later, among them יְהַבִית Gen 9:3). Corrected: the נ-assimilation row (מַנְדְּעָא is not an example) and the t-stem row (assimilation before ת, ט, ד; the sibilants metathesise). Added: the 1 sg. imperfect prefix row. Disagreement: Sefaria points the prefix with segol (196 alef-initial words before shewa, counted 2026-10-18), Dalman 1905 (p. 402: אִכְתּוֹב) and Berliner with ḥireq, Dalman 1894 (p. 330: אַכְתּוֹב) with pataḥ. Still open: the §1 spelling table and the rows on the determined plural and הָדֵין (no grammar statement found); the diphthong row is confirmed only in general (Dalman gives no Onkelos count).

## 4. Gutturals

Unlike JPA and JBA, Targumic Aramaic keeps the gutturals apart in spelling: ע is not confused with א, ח not with ה. Where the printed Onkelos shows guttural confusion, it is usually a copyist's error from a later dialect (Dalman 1905). This is one of the reasons the targumic text is a reasonable training base for a model that later must cope with the "weak-guttural" spellings of the Talmuds ([../tmr/phonology-orthography.md](../tmr/phonology-orthography.md); [../jpa/phonology-orthography.md](../jpa/phonology-orthography.md)).

## 5. Variants in the printed editions

The printed texts in the Sefaria PD files carry occasional bracketed or parenthesised variants, e.g. Gen 28:20 "לַחֲמָא (נ''י לְחֵם)": "lḥmʾ (another reading: lḥm)". These are editorial notes marking a variant reading, not text. A tokenizer must remove them from the sentence and keep them in MISC.

The Elamur team
