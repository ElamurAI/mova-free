# 1. Script and pointing

The Hebrew Bible reaches us in two layers of writing. The older layer is the consonantal text: twenty-two letters, written right to left, with a few letters doing double duty as vowel signs. Over it, the Masoretes of Tiberias (roughly the 8th to 10th centuries CE) laid a dense system of dots and strokes that records how the text was read aloud: vowels, the doubling or softening of consonants, the place of the stress, and the chant (GKC §7h–i; Khan 2020, vol. 1, §I.0.4). This chapter covers both layers as they appear in the text Mova reads, the Westminster Leningrad Codex (WLC) as carried by MACULA Hebrew, and shows how the tools treat each sign. The accents (cantillation) have a chapter of their own, [02-cantillation.md](02-cantillation.md). Sounds and syllables are in [03-phonology.md](03-phonology.md).

All counts in this chapter are over the Torah trees in `$MOVA_DATA/corpora/judaism/text/torah-wlc-macula/` (5 853 sentences, the `# text` lines and the FORM column), measured on 2026-10-17 with read-only scripts.

## Contents

1. [The alphabet](#1-the-alphabet)
2. [A short history of the script](#2-a-short-history-of-the-script)
3. [Final forms and letters as numbers](#3-final-forms-and-letters-as-numbers)
4. [Vowel letters (matres lectionis), plene and defective spelling](#4-vowel-letters-matres-lectionis-plene-and-defective-spelling)
5. [The Tiberian vowel points](#5-the-tiberian-vowel-points)
6. [Shewa](#6-shewa)
7. [Dagesh, mappiq and rafe](#7-dagesh-mappiq-and-rafe)
8. [The dots of shin and sin](#8-the-dots-of-shin-and-sin)
9. [Maqqef, meteg, paseq, sof pasuq and the paragraph marks](#9-maqqef-meteg-paseq-sof-pasuq-and-the-paragraph-marks)
10. [Ketiv and qere](#10-ketiv-and-qere)
11. [Other scribal marks: the extraordinary points](#11-other-scribal-marks-the-extraordinary-points)
12. [Unicode: code points, combining order, normalization](#12-unicode-code-points-combining-order-normalization)
13. [The letters in Sefer Yetzirah](#13-the-letters-in-sefer-yetzirah)
14. [In Mova](#14-in-mova)
15. [Sources](#15-sources)

## Conventions

Examples are quoted from our data with their verse and sentence id:

> **GEN.1.1** · `wlc:GEN:0`
> בְּרֵאשִׁית בָּרָא אֱלֹהִים אֵת הַשָּׁמַיִם וְאֵת הָאָרֶץ
> *bərēʾšît bārāʾ ʾĕlōhîm ʾēt haššāmayim wəʾēt hāʾāreṣ* — in-beginning created God (et) the-heavens and-(et) the-earth — "In the beginning God created the heaven and the earth." (JPS 1917)

The Hebrew is shown with vowels but, unless the point is about accents, without the accent marks that FORM keeps. Transliteration follows the SBL academic style: ʾ b/ḇ g/ḡ d/ḏ h w z ḥ ṭ y k/ḵ l m n s ʿ p/p̄ ṣ q r ś š t/ṯ; ə for vocal shewa; ă ĕ ŏ for the hatef vowels; a doubled letter for dagesh forte; â ê î ô û for a vowel written with a vowel letter.

---

## 1. The alphabet

The alphabet has twenty-two letters, all consonants (GKC §5). The order, the names and the numerical values are old: the order is attested in the alphabetic acrostics of the Bible (e.g. Psalm 119, Lamentations 1–4) and passed from the Phoenicians to the Greeks (GKC §5).

The table gives each letter, its final form if it has one, its traditional name, the SBL transliteration, the usual reconstruction of its Tiberian pronunciation, its numerical value, its class in Sefer Yetzirah (§13 below) and its count in the Torah text we hold.

| Letter | Final | Name | Translit. | Tiberian value (Khan 2020) | Value | Sefer Yetzirah | Torah count |
|---|---|---|---|---|---:|---|---:|
| א | | ʾālep̄ (alef) | ʾ | glottal stop [ʔ] | 1 | mother; throat | 27 129 |
| ב | | bêṯ (bet) | b / ḇ | [b] / [v] | 2 | double; lips | 16 444 |
| ג | | gîmel (gimel) | g / ḡ | [g] / uvular fricative [ʁ] | 3 | double; palate | 2 119 |
| ד | | dāleṯ (dalet) | d / ḏ | [d] / [ð] | 4 | double; tongue | 7 071 |
| ה | | hēʾ (he) | h | [h] | 5 | simple; throat | 28 102 |
| ו | | wāw (vav) | w | labio-dental [v] (semivowel [w] in some positions) | 6 | simple; lips | 30 579 |
| ז | | zayin | z | [z] | 7 | simple; teeth | 2 200 |
| ח | | ḥêṯ (het) | ḥ | pharyngeal [ħ] | 8 | simple; throat | 7 212 |
| ט | | ṭêṯ (tet) | ṭ | pharyngealized [tˤ] | 9 | simple; tongue | 1 813 |
| י | | yôḏ (yod) | y | [j] | 10 | simple; palate | 31 656 |
| כ | ך | kap̄ (kaf) | k / ḵ | [k] / [χ] | 20 | double; palate | 8 613 + 3 357 |
| ל | | lāmeḏ (lamed) | l | [l] | 30 | simple; tongue | 21 611 |
| מ | ם | mēm | m | [m] | 40 | mother; lips | 14 487 + 10 645 |
| נ | ן | nûn | n | [n] | 50 | simple; tongue | 9 934 + 4 257 |
| ס | | sāmeḵ (samekh) | s | [s] | 60 | simple; teeth | 1 836 |
| ע | | ʿayin | ʿ | pharyngeal [ʕ] | 70 | simple; throat | 11 313 |
| פ | ף | pēʾ (pe) | p / p̄ | [p] / [f] | 80 | double; lips | 4 017 + 830 |
| צ | ץ | ṣāḏê (tsade) | ṣ | pharyngealized [sˤ] | 90 | simple; teeth | 2 947 + 1 037 |
| ק | | qôp̄ (qof) | q | [q] | 100 | simple; palate | 4 717 |
| ר | | rêš (resh) | r | uvular [ʀ]; alveolar [rˤ] next to alveolars | 200 | double; teeth | 18 233 |
| ש | | śîn / šîn | ś / š | [s] / [ʃ] | 300 | mother; teeth | 15 621 |
| ת | | tāw (tav) | t / ṯ | [t] / [θ] | 400 | double; tongue | 18 003 |

Notes on the table.

- **Pronunciation.** The values are those of the Tiberian reading tradition as reconstructed by Khan (2020, vol. 1, §§I.1.1–I.1.23) from the medieval sources (Hidāyat al-Qāriʾ, Karaite transcriptions, Saadya). Three of them differ from the textbook values. Vav is labio-dental [v], not [w] (§I.1.6). Gimel without dagesh is a uvular fricative [ʁ] (§I.1.3). Resh has two realizations: a basic uvular one and an alveolar one next to the alveolar consonants ד ז צ ת ט ס ל ן (§I.1.20). Older handbooks give the simpler values (GKC §6).
- **The two values of six letters.** ב ג ד כ פ ת (*begadkefat*) have a hard (plosive) and a soft (fricative) value; the dagesh lene marks the hard one (§7 below; GKC §13; §21).
- **Torah counts.** The counts are letter tokens in the 5 853 `# text` lines, minus the 671 paragraph marks פ and ס that the WLC text appends to verses (§9). Final forms are counted apart ("+"). The total is **305 783** letters. The figure traditionally given for a Torah scroll is 304 805 (see e.g. https://en.wikipedia.org/wiki/Sefer_Torah). Part of the difference is that MACULA's surface text follows the qere where the Masora marks one (§10); the rest is not yet explained, a gap.
- **Frequency.** The commonest letters are י, ו, ה, א: the three vowel letters and the alef. ו is high also because the conjunction *wə-* "and" opens most narrative clauses (wayyiqtol, see [09-discourse.md](09-discourse.md)).

**Names.** The names are acrophonic: each names a thing whose word begins with the letter (GKC §5 Rem. 1): ʾālep̄ "ox", bêṯ "house", gîmel "camel", dāleṯ "door", yôḏ "hand", kap̄ "palm", mēm "water", ʿayin "eye", pēʾ "mouth", rêš "head", šîn "tooth". The pictures behind several letters can still be seen in the Old Hebrew shapes (§2).

**Direction and word division.** Hebrew is written right to left. Words are not broken at the end of a line. Manuscripts and printed Bibles stretch some letters to fill a line instead (the *litterae dilatabiles*, GKC §5). Old inscriptions already divide words with dots or strokes (GKC §5, note on Mesha and Siloam).

---

## 2. A short history of the script

- **Old Hebrew (Paleo-Hebrew).** Until the Babylonian exile the Israelites wrote a local form of the Phoenician–Canaanite alphabet. It is the script of the Siloam tunnel inscription (late 8th century BCE), of the Moabite Mesha stele (9th century BCE, a sister language in the same script), of the ostraca of Lachish and Arad, and later of the Hasmonean coins (GKC §5; Stefaniak 2026 on the Arad inscriptions, `$RESEARCH_RAW/hbo-2026-stefaniak-old-hebrew-inscriptions-tel-arad`). GKC's 1910 English edition prints a facsimile of the Siloam inscription and a table of early alphabets by Lidzbarski.
- **The square script.** After the exile, Jews took up the Aramaic chancery script of the Persian empire, which developed into the "square" script (*kəṯāḇ mərubbāʿ*), also called "Assyrian" (*kəṯāḇ ʾaššûrî*) in rabbinic sources (GKC §5). The Dead Sea Scrolls (3rd century BCE–1st century CE) are almost all in square script; some write the divine name in Old Hebrew letters inside a square-script text. The Samaritans kept a descendant of the Old Hebrew script for their Pentateuch to this day. The letter forms have changed little since the 1st century CE, so that the age of a manuscript is hard to tell from its hand (GKC §5).
- **The vowel points.** No vowel signs are known to the Talmud or to Jerome (GKC §7h). Three systems of pointing were made in the early Middle Ages: Babylonian (supralinear), Palestinian (supralinear) and Tiberian (mostly sublinear). The Tiberian one prevailed and is the system of every printed Hebrew Bible. Its model codices are the Aleppo Codex (10th century) and the Leningrad Codex (1008/9 CE), which is the base of the WLC and so of MACULA (Khan 2020, vol. 1, §I.0.4, §I.0.13; Hornkohl and Khan 2020, `$RESEARCH_RAW/hebgram/open/khan-hornkohl-semitic-vocalisation-2020`).

---

## 3. Final forms and letters as numbers

### 3.1 The five final letters

Five letters take a special shape at the end of a word: כ/ך, מ/ם, נ/ן, פ/ף, צ/ץ. The Jewish grammars group them in the mnemonic כַּמְנַפֵּץ *kamnappēṣ* (GKC §5). The final forms of כ נ פ צ keep the old long downstroke; GKC notes that they are the more original shapes (GKC §5, note).

In the Torah (2026-10-17) the final form makes up 28 % of all kaf (3 357 of 11 970), 42 % of all mem (10 645 of 25 132), 30 % of all nun (4 257 of 14 191), 17 % of all pe (830 of 4 847) and 26 % of all tsade (1 037 of 3 984). Final mem is so common because of the masculine plural *-îm* and the suffix *-hem / -ām* "their".

> **GEN.1.1** · `wlc:GEN:0`
> אֱלֹהִים … הַשָּׁמַיִם
> *ʾĕlōhîm … haššāmayim* — two final mems in one verse, the plural ending *-îm* and the dual-looking *-ayim* of "heavens".

A final form is used at the end of a graphic word even when a suffix follows the word in MACULA's segmentation, because the letters are split only in the tokens, not in the writing:

> **GEN.12.1** · `wlc:GEN:299`
> לֶךְ־לְךָ
> *leḵ-ləḵā* — go-for-yourself — "Get thee out." (JPS 1917). Final kaf ך in both words: the first is the bare imperative, the second the suffix *-ḵā* "you" on the preposition.

### 3.2 Letters as numbers (gematria)

The letters also serve as numerals: units א–ט (1–9), tens י–צ (10–90), hundreds ק–ת (100–400). Numbers above 400 are written by adding hundreds to ת (תק = 500); a later convention gives the final letters the values 500–900 (ך ם ן ף ץ). The larger part comes first (on the right): יא = 11, קכא = 121. Fifteen is written טו (9 + 6) and sixteen טז (9 + 7) to avoid spelling parts of the divine name, יה and יו (GKC §5k). The use of letters as numbers appears first on the Hasmonean coins; the biblical text itself always writes numbers out in words (GKC §5k; numbers in the language are covered in [04-noun.md](04-noun.md)).

| Units | | Tens | | Hundreds | |
|---|---:|---|---:|---|---:|
| א | 1 | י | 10 | ק | 100 |
| ב | 2 | כ | 20 | ר | 200 |
| ג | 3 | ל | 30 | ש | 300 |
| ד | 4 | מ | 40 | ת | 400 |
| ה | 5 | נ | 50 | (ך) | (500) |
| ו | 6 | ס | 60 | (ם) | (600) |
| ז | 7 | ע | 70 | (ן) | (700) |
| ח | 8 | פ | 80 | (ף) | (800) |
| ט | 9 | צ | 90 | (ץ) | (900) |

---

## 4. Vowel letters (matres lectionis), plene and defective spelling

The consonantal text writes some vowels with the letters ה, ו, י and, less often, א. These are the *matres lectionis*, "mothers of reading" (GKC §7b–g). They came in step by step:

1. **At the end of a word first.** A final long vowel was marked by keeping a weak final consonant as a sign: ה for final *-ā*, *-e*, *-o*; ו for *-û*; י for *-î* and *-ê* (GKC §7b–d). The Siloam and Mesha inscriptions already write final vowels this way.
2. **Inside the word later.** ו for *ô*, *û* and י for *ê*, *î* began where a diphthong had contracted (*aw > ô*, *ay > ê*), so that the consonant stayed as a written trace, and then spread by analogy to vowels that had never been diphthongs (GKC §7e–f). The Siloam inscription still writes internal long vowels without them, e.g. אִשׁ for later אִישׁ "man" (GKC §7f).
3. **Fixed by the Masoretes.** Once the points were added, the vowel letters became largely a matter of spelling, and their use varies from word to word and book to book. Final vowels are nearly always written; internal ones often are not (GKC §7g; §8k; §9).

**Plene and defective.** A word written with its vowel letter is *plene* (Hebrew *mālēʾ*, "full"); without it, *defective* (*ḥāsēr*, "lacking"). The pointing shows the vowel either way: holem stands over the letter before a missing ו, hireq stands alone where י is missing. The Masoretic notes count such spellings word by word (GKC §17e, the terms מָלֵא and חָסֵר).

The Torah (2026-10-17):

| Vowel | Written with a vowel letter | Written with the point only | Share plene |
|---|---:|---:|---:|
| *ô* (holem) | 7 046 (וֹ, *holem male*) | 13 018 (point ֹ on a consonant, *holem haser*) | 35 % |
| *u* | 7 798 (וּ, *shureq*) | 1 069 (ֻ, *qibbuts*) | 88 % |
| *i* | 11 079 (hireq + י, *hireq male*) | 14 478 (hireq alone) | 43 % |

(The *u* and *i* columns count every hireq and qibbuts, short and long, so they overstate the defective spelling of long vowels; the holem column is clean, since holem is always long.)

The same word is spelled in several ways within the Torah. The noun תּוֹלְדֹת *tôləḏōṯ* "generations, account" (Strong 8435) shows all four combinations of full and defective *ô*:

| Spelling | Count | Where |
|---|---:|---|
| תּוֹלְדֹת (full, defective) | 19 | GEN.5.1, GEN.6.9, GEN.10.1, GEN.11.10, GEN.11.27, … |
| תֹּלְדוֹת (defective, full) | 3 | GEN.36.1, GEN.36.9, GEN.37.2 |
| תּוֹלְדֹת without dagesh | 3 | GEN.10.32, GEN.25.13, EXO.28.10 |
| תֹלְדֹת / תֹּלְדֹת (defective, defective) | 3 | EXO.6.16, EXO.6.19, GEN.25.12 |
| תוֹלְדוֹת (full, full) | 1 | GEN.2.4 |

> **GEN.2.4** · `wlc:GEN:34`
> אֵלֶּה תוֹלְדוֹת הַשָּׁמַיִם וְהָאָרֶץ בְּהִבָּרְאָם
> *ʾēlleh ṯôləḏôṯ haššāmayim wəhāʾāreṣ bəhibbārəʾām* — these generations-of the-heavens and-the-earth in-their-being-created — "These are the generations of the heaven and of the earth when they were created." (JPS 1917). The only fully plene spelling in the Torah; note also that the ת has no dagesh, because the word before ends in a vowel and is joined to it by a conjunctive accent (§7.2).

**Alef as a vowel letter.** א is written in some words where it is no longer pronounced, as a historical spelling (רֹאשׁ *rōʾš* "head", from *raʾš*) or, rarely, as a pure vowel letter (GKC §7b; §23). It is "quiescent" (*nāḥ*) in the Masoretic terms (GKC §17e). The full rules of when א and ה lose their consonant value are phonology ([03-phonology.md](03-phonology.md); GKC §23).

**What this means for a machine.** Plene and defective spellings of one word must meet at one lemma. MACULA gives every token its lemma and Strong number, so all five spellings of *tôləḏōṯ* above have the same Strong 8435 and a lemma תּוֹלֵדוֹת in our trees, and the dictionary `torah-en.tsv` gathers them under one key. A raw-text tool without the lemma would see five different words.

---

## 5. The Tiberian vowel points

The Tiberian system has seven full vowel signs, three hatef (reduced) vowels and shewa (GKC §8–§10; Khan 2020, vol. 1, §I.2). It records *quality* (the colour of the vowel) rather than length: in the Tiberian reading, length came from stress and syllable structure, not from the sign (Khan 2020, vol. 1, §I.2.1 on quality, §I.2.2 on length). The school tradition since the medieval grammarians, and GKC with it, classifies the signs as long and short; that classification is useful for the morphology and is used in the later chapters.

The examples are on the letter ב. Code points are in §12.

| Sign | Name | Tiberian value (Khan) | SBL translit. | School length | Torah count |
|---|---|---|---|---|---:|
| בָ | qāmeṣ | [ɔ] (long or short) | ā (long), o (qamets hatuf, short) | long / short | 39 273 |
| בַ | pataḥ | [a] | a | short | 30 798 |
| בֵ | ṣērê | [e] | ē; with yod ê | long | 15 882 |
| בֶ | səḡôl | [ɛ] | e | short | 22 512 |
| בִ | ḥîreq | [i] | i; with yod î | short / long | 25 557 |
| בֹ | ḥōlem | [o] | ō; with vav ô | long | 20 064 |
| בֻ | qibbûṣ | [u] | u | short / long | 1 069 |
| בוּ | šûreq | [u] | û | long | 7 798 |
| בְ | šəwāʾ | ∅ or short [a] (§6) | ə (vocal) / nothing (silent) | — | 36 788 |
| חֲ | ḥăṭep̄ pataḥ | short [a] | ă | reduced | 6 989 |
| אֱ | ḥăṭep̄ səḡôl | short [ɛ] | ĕ | reduced | 1 116 |
| חֳ | ḥăṭep̄ qāmeṣ | short [ɔ] | ŏ | reduced | 120 |

**Qamets and qamets hatuf.** One sign, ָ, stands for two vowels in the school grammar: long *ā* (qamets gadol) and short *o* (qamets hatuf, in a closed unstressed syllable, e.g. כָּל־ *kol-* "all" before maqqef). In the Tiberian reading both were the same sound, [ɔ], differing only in length (Khan 2020, vol. 1, §I.2.1.2–I.2.1.3). Unicode has a separate code point for qamets hatuf (U+05C7), but the WLC text does not use it: the Torah has 0 occurrences, so a tool that needs the distinction has to work it out from the syllable (GKC §9u; [03-phonology.md](03-phonology.md)).

> **GEN.8.17** · `wlc:GEN:200`
> כָּל־הַחַיָּה אֲשֶׁר־אִתְּךָ
> *kol-haḥayyâ ʾăšer-ʾittəḵā* — all-the-living-thing which-with-you — "every living thing that is with thee." כָּל־ has qamets hatuf (*kol*), being unstressed before maqqef. In הַחַיָּה the same sign is long *ā*, under a stressed open syllable, followed by the vowel letter ה (â).

**Holem and the vav.** Holem stands above and to the left of its consonant. Over a vav it makes the vowel letter וֹ *ô*. A consonantal vav followed by *o* (as in מִצְוֺת *miṣwōṯ* "commandments") gets a separate sign in Unicode, *holam haser for vav* U+05BA, so that the two are not confused (Unicode chapter 9). The Torah text has 100 of these.

> **DEU.4.2** · `wlc:DEU:113`
> אֶת־מִצְוֺת יְהוָה
> *ʾeṯ-miṣwōṯ YHWH* — (et)-commandments-of the-LORD. The vav of מִצְוֺת is a consonant [v] with the vowel *ō*: U+05BA.

**The hatef vowels.** The hatefs are shewa joined to pataḥ, segol or qamets. They appear mostly under the guttural letters א ה ח ע, which cannot carry a plain vocal shewa (GKC §10; §22l). The Torah counts above show how unevenly they are spread: ḥăṭep̄ pataḥ is common (the relative אֲשֶׁר *ʾăšer* "which" alone is 1 908 tokens), ḥăṭep̄ səḡôl mostly in the word אֱלֹהִים *ʾĕlōhîm* "God", and ḥăṭep̄ qāmeṣ rare.

> **GEN.1.1** · `wlc:GEN:0`
> אֱלֹהִים
> *ʾĕlōhîm* — "God": ḥăṭep̄ səḡôl under the alef, then ḥōlem, then ḥîreq with the vowel letter י.

---

## 6. Shewa

Shewa (ְ) does two jobs that look the same in writing (GKC §10; Khan 2020, vol. 1, §I.2.5).

- **Silent shewa** (*šəwāʾ nāḥ*) marks the end of a syllable: the consonant has no vowel after it. It is not written under a final consonant, except under final ך and in a few verb forms with two final consonants (אַתְּ, וַיֵּשְׁתְּ).
- **Vocal shewa** (*šəwāʾ nāʿ*) stands for a very short vowel. In the Tiberian reading its default sound was a short [a]; before a guttural it took the colour of the vowel after the guttural, and before yod it was a short [i] (Khan 2020, vol. 1, §I.2.5, and the summary table in §I.5). The school rendering is ə.

**How to tell them apart.** The rules of thumb, after GKC §10 and the standard textbooks (Davidson 1916 and Harper 1891, the sections on shewa):

| Position | Kind | Example |
|---|---|---|
| under the first consonant of a word | vocal | בְּרֵאשִׁית *bərēʾšîṯ* (GEN.1.1) |
| the second of two shewas in a row inside a word | vocal | יִשְׁמְרוּ *yišmərû* "they will keep" |
| under a consonant with dagesh forte | vocal | הִבָּרְאָם: no; but מְדַבְּרִים *mədabbərîm* |
| after a long vowel in an unstressed open syllable (often with meteg) | vocal | שָֽׁמְרוּ *šāmərû* "they kept" |
| under the second of two identical consonants | vocal | הִנְנִי (hinənî), הַלְלוּ (haləlû) |
| after a short vowel, closing the syllable | silent | מִדְבָּר *miḏbār* "desert" |
| the first of two shewas in a row inside a word | silent | יִשְׁמְרוּ (the shewa under ש) |
| at the end of a word | silent | לֶךְ *leḵ*, אַתְּ *ʾatt* |

The rule "after a long vowel" depends on knowing whether the vowel is long, which the sign does not say (§5). The Masoretes help by marking an open syllable before a vocal shewa with meteg (§9.2): שָֽׁמְרוּ.

**Shewa and begadkefat.** After a vocal shewa the following begadkefat letter is soft, after a silent shewa hard: this is the main external test (GKC §21; [03-phonology.md](03-phonology.md)).

> **GEN.12.1** · `wlc:GEN:299`
> מֵאַרְצְךָ וּמִמּוֹלַדְתְּךָ
> *mēʾarṣəḵā ûmimmôlaḏtəḵā* — from-your-land and-from-your-birthplace. In אַרְצְךָ the shewa under צ is vocal (second of two) and the כ after it is soft (*ḵā*). In מוֹלַדְתְּךָ the shewa under ד is silent, so the ת after it is hard and carries dagesh lene; the shewa under ת is vocal and the כ after it soft again.

Shewa is the single most frequent point after dagesh and qamets: 36 788 in the Torah.

---

## 7. Dagesh, mappiq and rafe

One sign, a dot in the letter (ּ, U+05BC), does four jobs, and the reader has to know which (GKC §12–§14).

### 7.1 Dagesh forte (doubling)

Dagesh forte doubles the consonant: the consonant closes one syllable and opens the next. It comes from assimilation (*yinttēn > yittēn*), from the shape of a word pattern (the piel doubles the middle root letter: דִּבֶּר *dibber* "he spoke") or after the article and the wayyiqtol prefix (GKC §20).

> **GEN.1.1** · `wlc:GEN:0`
> הַשָּׁמַיִם
> *haššāmayim* — the article *ha-* doubles the first letter of the noun.

> **GEN.1.3** · `wlc:GEN:2`
> וַיֹּאמֶר
> *wayyōʾmer* — the wayyiqtol: *wa-* plus doubling of the prefix *y-*.

The gutturals א ה ח ע and, as a rule, ר do not take dagesh forte; the vowel before them is lengthened or the doubling is "virtual" (GKC §22; Khan 2020, vol. 1, on resh with dagesh as an exception). In the Torah the article before a guttural is *hā-* or *he-* instead of *ha-* with doubling: הָאָרֶץ *hāʾāreṣ* (GEN.1.1).

### 7.2 Dagesh lene (hardening)

In the six letters ב ג ד כ פ ת the same dot marks the hard (plosive) value. They are hard at the start of a word and after a closed syllable (a silent shewa), soft after any vowel, even a vocal shewa (GKC §21; §13).

At the start of a word the rule looks across the word boundary: if the previous word ends in a vowel and is closely joined to it by a conjunctive accent or maqqef, the begadkefat letter stays soft. This is one of the clearest places where the cantillation changes the letters ([02-cantillation.md](02-cantillation.md); GKC §21b).

> **GEN.2.4** · `wlc:GEN:34`
> אֵ֣לֶּה תוֹלְד֧וֹת
> *ʾēlleh ṯôləḏôṯ* — the first word ends in the vowel *-eh* and carries the conjunctive accent munach (֣); so the ת of the next word has no dagesh and is soft.

> **GEN.1.1** · `wlc:GEN:0`
> בְּרֵאשִׁ֖ית בָּרָ֣א
> *bərēʾšîṯ bārāʾ* — both bets are hard. The first word starts the verse. The second follows a word ending in a consonant (ת) that carries a disjunctive accent (tipcha ֖).

Of the 60 744 begadkefat letters in the Torah, 21 307 (35 %) carry a dagesh, lene or forte.

### 7.3 Mappiq

In a final ה the dot (*mappîq*, "bringing out") says the ה is a consonant, not a vowel letter. The commonest case is the suffix *-āh* "her" (GKC §14). The Torah has 654 word-final הּ.

> **GEN.3.6** · `wlc:GEN:61`
> וַתִּתֵּן גַּם־לְאִישָׁהּ עִמָּהּ
> *wattittēn gam-ləʾîšāh ʿimmāh* — and-she-gave also-to-her-husband with-her — "and she gave also unto her husband with her." Two mappiqs: *-āh* "her" on a noun and on a preposition. Without the dot, לְאִישָׁה would read *ləʾiššâ* "for a wife".

### 7.4 Shureq

The dot inside a vav with no other vowel, וּ, is the vowel *û* (§5). The Torah has 7 798. At the start of a word, וּ is the conjunction *û-* "and" before a labial or a shewa (GKC §104).

### 7.5 Rafe

*Rāp̄ê* (ֿ, a stroke over the letter) is the opposite of dagesh: it marks a begadkefat letter as soft, or a ה as not consonantal (GKC §14). The model codices use it often; the WLC keeps it only in a handful of places. The Torah text has 7, all in the Decalogue (EXO.20.13–15, DEU.5.13 and 17–19), where the double accentuation (§11; [02-cantillation.md](02-cantillation.md)) makes one reading need the dagesh and the other not, and the rafe records the second.

> **EXO.20.13** · `wlc:EXO:531`
> לֹ֥֖א תִּֿרְצָֽ֖ח
> *lōʾ ṯirṣāḥ* / *lōʾ tirṣāḥ* — "Thou shalt not murder." The ת carries both dagesh and rafe, and each word carries two accents: one reading joins the verse to the next commandment, the other reads it as a verse of its own.

---

## 8. The dots of shin and sin

The letter ש stands for two sounds: *š* [ʃ] and *ś* (in the Tiberian reading [s], historically a lateral). A dot over the right horn (שׁ, U+05C1) marks *šîn*, over the left horn (שׂ, U+05C2) *śîn* (GKC §6). In the Torah: 12 182 shin, 3 420 sin, and 19 with no dot. Eighteen of the 19 undotted ones are the name Issachar, יִשָּׂשכָר (15 + 3 times), whose second ש is not pronounced: the dot is left out because the letter is silent in the qere.

The dot is the only thing that tells some pairs of roots apart: שָׂם *śām* "he put" and שָׁם *šām* "there"; עָשָׂה *ʿāśâ* "he made" and עָשָׁה (no such word). Tools that compare by consonants only (§12) merge them. MACULA's lemmas keep the dots, so the trees keep the words apart.

---

## 9. Maqqef, meteg, paseq, sof pasuq and the paragraph marks

### 9.1 Maqqef

*Maqqēp̄* (־, U+05BE, a high hyphen) joins two or more words into one stress unit: only the last carries the accent, and the first ones lose their stress and often shorten their vowel (GKC §16; Khan 2020, vol. 1, §I.2.11). The Torah has 11 524 maqqefs. Short function words take it most: אֶת־, אֶל־, עַל־, כָּל־, כִּי־, אֲשֶׁר־, בֶּן־. Counted as graphic words (separated by spaces), the Torah has 68 467; counting each maqqef-joined part apart, 79 991, close to the figures of about 80 000 words that tradition gives for the Torah.

> **GEN.1.3** · `wlc:GEN:2`
> וַֽיְהִי־אוֹר
> *wayhî-ʾôr* — and-there-was-light. One stress unit: the verb loses its own accent.

> **GEN.8.17** · `wlc:GEN:200`
> כָּל־הַחַיָּה — *kol-haḥayyâ*: כֹּל "all" becomes *kol* with a short vowel (qamets hatuf) before maqqef.

### 9.2 Meteg (ga'ya)

*Meteg* (ֽ, U+05BD, a small vertical stroke under the letter, to the left of the vowel) marks a secondary stress and, with it, a full vowel kept in an open syllable two or more places before the main stress (GKC §16). It is how the Masoretes tell long *ā* from short *o* under qamets, and vocal from silent shewa after it (§6). The same sign on the last word of a verse is the accent *silluq* (§9.4; [02-cantillation.md](02-cantillation.md)). The Torah has 10 265 of them, of which about 5 850 are silluq.

> **GEN.1.5** · `wlc:GEN:4`
> וַֽיְהִי־עֶרֶב
> *wayhî-ʿereḇ* — meteg on the *wa-* of the wayyiqtol before a shewa: a secondary stress in a long stress unit.

### 9.3 Paseq

*Pāsēq* (׀, U+05C0, a vertical stroke between words) is a small pause. It separates two words where running them together would sound wrong (two like letters, a repeated word, the divine name next to a word of bad sense), and with a conjunctive accent it forms the disjunctive accent *legarmeh* (GKC §15f, I.13; Wickes 1887). The Torah has 387.

> **GEN.1.5** · `wlc:GEN:4`
> וַיִּקְרָא אֱלֹהִים ׀ לָאוֹר יוֹם
> *wayyiqrāʾ ʾĕlōhîm lāʾôr yôm* — and-called God to-the-light day — "And God called the light Day." The paseq after אֱלֹהִים, with the conjunctive accent on it, makes a legarmeh.

### 9.4 Sof pasuq

*Sôp̄ pāsûq* (׃, U+05C3, two dots like a colon) ends a verse, after the last word, which carries silluq (GKC §15f). The Torah text has 5 819. Our 5 853 sentences include 34 without it: where MACULA splits one verse into two sentences, only the last sentence ends with the mark.

### 9.5 The paragraph marks פ and ס

The written Torah is divided into paragraphs (*pārāšiyyôṯ*) by spaces: an "open" paragraph (*pəṯûḥâ*) starts on a new line, a "closed" one (*səṯûmâ*) after a space in the same line. Printed Bibles mark them with the letters פ and ס. The WLC appends them to the verse, and so do MACULA's `# text` lines: 290 פ and 381 ס in the Torah. The paragraph division is an old layer of tradition, found already in the Qumran scrolls, and it does not always agree with the verse division of the accents (Khan 2020, vol. 1, §I.0.6, on פסקה באמצע פסוק "a paragraph break inside a verse", e.g. GEN.35.22).

> **GEN.1.5** ends ־אֶחָֽד׃פ — the first day ends an open paragraph.
> **NUM.6.24** ends וְיִשְׁמְרֶֽךָ׃ס — the first line of the priestly blessing ends a closed one.

---

## 10. Ketiv and qere

Where the Masoretes read something other than what the consonantal text writes, they did not change the consonants. They pointed the written word with the vowels of the word to be read, and put the consonants of that word in the margin. What is written is the *kəṯîḇ* ("written"); what is read is the *qərê* ("read") (GKC §17a; Khan 2020, vol. 1, §I.0.5). Khan argues that the qere is the older reading tradition, transmitted orally alongside the written text, and that it was canonical before the accents were added (Khan 2020, vol. 1, §I.0.5–6).

**Kinds.** Besides a different word or spelling, there are words written but not read (*kəṯîḇ wəlōʾ qərê*) and words read but not written (*qərê wəlōʾ kəṯîḇ*) (GKC §17b).

**Perpetual qere** (*qərê perpetuum*). A few very common words are always read otherwise than written, so the margin note is left out and only the vowels show the reading (GKC §17c):

| Written | Pointed | Read | Meaning | Torah (2026-10-17) |
|---|---|---|---|---|
| יהוה | יְהוָה (also יְהֹוָה, יהוָה) | *ʾăḏōnāy* "the Lord" | the divine name | 1 506 + 287 + 13 + 7 others |
| יהוה after אֲדֹנָי | יֱהוִה / יְהוִה | *ʾĕlōhîm* "God" | the divine name | 4 (GEN.15.2, GEN.15.8, DEU.3.24, DEU.9.26) |
| הוא for "she" | הִוא | *hîʾ* | "she, it (fem.)" | 192 (beside 19 written הִיא) |
| נער for "girl" | נַעֲרָ | *naʿărâ* | "girl" | 8 (GEN.24) |
| יששכר | יִשָּׂשכָר | *yiśśāḵār* | Issachar | 18 |

Notes:

- **The divine name.** יְהוָה carries the vowels of אֲדֹנָי (with simple shewa for the hatef), so it is read *ʾăḏōnāy*; where the text has אֲדֹנָי next to it, it carries the vowels of אֱלֹהִים instead, to avoid saying *ʾăḏōnāy ʾăḏōnāy* (GKC §17c; §102m; §135q, note). The 287 spellings without shewa under the yod all follow a prefix (לְ 252, וְ 21, בְּ 12, כְּ 2 by lemma), pointed לַ, וַ, בַּ, כַּ: the preposition takes the vowel it would take before *ʾăḏōnāy*, so לַיהוָה is read *laʾḏōnāy*.

> **GEN.4.3** · `wlc:GEN:82`
> וַיָּבֵא קַיִן מִפְּרִי הָאֲדָמָה מִנְחָה לַיהוָה
> *wayyāḇēʾ qayin mippərî hāʾăḏāmâ minḥâ laʾḏōnāy* — and-brought Cain from-fruit-of the-ground an-offering to-the-LORD — "Cain brought of the fruit of the ground an offering unto the LORD." The patah of לַ is the vowel of *la-ʾăḏōnāy*; the yod has no shewa.

> **GEN.15.2** · `wlc:GEN:362`
> אֲדֹנָי יֱהוִה מַה־תִּתֶּן־לִי
> *ʾăḏōnāy ʾĕlōhîm mah-titten-lî* — Lord GOD, what will-you-give-me — "O Lord GOD, what wilt Thou give me." The second word carries the vowels of *ʾĕlōhîm*.

- **הִוא.** In the Torah the pronoun "she" is written הוא, the same as "he", 192 times, and pointed הִוא to be read *hîʾ*. Outside the Torah it is spelled הִיא. This is a feature of the Pentateuch's spelling, already noted by the Masora (GKC §17c; §32l). In our trees MACULA gives these tokens the Strong number 1931a and `Gender=Fem`, so the trees read the qere.

> **GEN.3.6** · `wlc:GEN:61`
> וְכִי תַֽאֲוָה־הוּא לָעֵינַיִם — here, *hûʾ* "it (masc.)": the written and read forms agree.

- **Marked qere in our text.** For a qere that the Masora does note in the margin, MACULA's surface form is the qere: in GEN.8.17 the text has הַיְצֵא *hayṣēʾ* "bring out", which is the qere of the written הוצא (the WLC marks it as a qere). In DEU.22 the word "girl" appears as נַּעֲרָה with the final ה of the qere, against the written נער (GKC §17c names Dt 22 as the exception to the perpetual qere). The ketiv is not in our trees.

> **GEN.24.33** · `wlc:GEN:624`
> וַיּוּשַׂם לְפָנָיו לֶאֱכֹל
> *wayyûśam ləp̄ānāyw leʾĕḵōl* — and-was-set before-him to-eat — "And there was set food before him to eat." The written text has ויישם; the qere, which is what our trees carry, is וַיּוּשַׂם, a hophal.

---

## 11. Other scribal marks: the extraordinary points

**Puncta extraordinaria.** In fifteen places of the Bible, ten of them in the Torah, dots stand over letters or words. The tradition from the Sifre onward read them as marks of doubt about the dotted letters (GKC §5n). Unicode encodes them as *upper dot* U+05C4. Our Torah text has 32 of them, in exactly the ten places: GEN.16.5, GEN.18.9, GEN.19.33, GEN.33.4, GEN.37.12, NUM.3.39, NUM.9.10, NUM.21.30, NUM.29.15, DEU.29.28.

> **GEN.33.4** · `wlc:GEN:964`
> וַיִּשָּׁקֵהוּ (each letter dotted: וַׄיִּׄשָּׁׄקֵ֑ׄהׄוּׄ)
> *wayyiššāqēhû* — "and he kissed him" (Esau kissing Jacob). Midrash read the dots as doubting the sincerity of the kiss.

**Not in our text.** Some marks of the scrolls and of other editions are not encoded in the WLC as MACULA carries it: the large and small letters (the large ב of בְּרֵאשִׁית, the large ע and ד of DEU.6.4), the inverted nuns around NUM.10.35–36 (U+05C6, 0 in our text), and the Masoretic circle (U+05AF). A gap, noted; nothing in Mova depends on them.

**Spacing errors in `# text`.** MACULA rebuilds the text line from each word's following space, and a few words lack one, so two words run together in `# text` though the tokens are separate. Three of them: GEN.33.4 צַוָּארָ֖ווַׄיִּׄשָּׁׄקֵ֑ׄהׄוּׄוַיִּבְכּֽוּ, EXO.20.13 לֹ֥֖אתִּֿרְצָֽ֖ח, DEU.6.4 שְׁמַ֖עיִשְׂרָאֵ֑ל. Tools that read the tokens are not affected; anything that splits `# text` by spaces is.

---

## 12. Unicode: code points, combining order, normalization

Hebrew is in the Unicode block U+0590–U+05FF. A pointed word is a sequence: a base letter followed by its combining marks.

### 12.1 The code points used in the Torah text

| Range | What | Notes |
|---|---|---|
| U+05D0–U+05EA | the 27 letter forms (22 + 5 finals) | the only base characters; `consonants()` keeps exactly these |
| U+05B0–U+05BB | shewa, the hatefs, the vowels, qibbuts | U+05BA holam haser for vav (100 in the Torah) |
| U+05BC | dagesh / mappiq / shureq dot | one code point for four functions |
| U+05BD | meteg (also silluq) | |
| U+05BF | rafe | 7 in the Torah |
| U+05C1, U+05C2 | shin dot, sin dot | |
| U+05C7 | qamats qatan | not used by the WLC (0) |
| U+0591–U+05AE | the accents | see [02-cantillation.md](02-cantillation.md) |
| U+05BE | maqaf | a spacing punctuation mark, not combining |
| U+05C0 | paseq | spacing |
| U+05C3 | sof pasuq | spacing |
| U+05C4 | upper dot (puncta extraordinaria) | 32 |
| U+05C6 | nun hafukha | not in our text |

### 12.2 Combining classes and why the order of points varies

Unicode gives each combining mark a *canonical combining class* (ccc), and normalization (NFC or NFD) sorts the marks on one letter by that class. For Hebrew the classes were assigned one per point: shewa 10, hataf segol 11, hataf patah 12, hataf qamats 13, hiriq 14, tsere 15, segol 16, patah 17, qamats 18 (qamats qatan 18), holam 19 (holam haser for vav 19), qubuts 20, dagesh 21, meteg 22, rafe 23, shin dot 24, sin dot 25; the accents are 220, 222, 228 or 230 by position (Unicode Character Database; the values are those Python's `unicodedata.combining` returns).

These classes do not follow the logical order a scribe or typesetter would use (letter, shin/sin dot, dagesh, vowel, accent). Normalizing a pointed word moves the vowel before the dagesh and the shin dot, so בַּ (bet, dagesh, patah) and בַּ (bet, patah, dagesh) are canonically equivalent, and NFC writes the second. Different sources therefore store the same word with the points in different orders, and a byte-for-byte comparison of two pointed strings can fail for words that are the same. This is a known problem of Biblical Hebrew text processing; the SBL Hebrew font manual and the Unicode FAQ on Hebrew discuss it, and the workaround used by the WLC and others is the combining grapheme joiner U+034F to block reordering where it matters (Unicode Standard, ch. 9.1; https://www.unicode.org/faq/char_combmark.html on CGJ).

**Points split across tokens.** MACULA cuts words into morphemes (the prefixes, the suffixes), and the cut can fall between a letter and its point. In GEN.2.4 the infinitive בְּהִבָּֽרְאָם *bəhibbārəʾām* "when they were created" becomes three tokens: בְּ, הִבָּֽרְא and the suffix token ָ֑ם, whose FORM begins with the qamets and the atnach that belong to the alef of the token before. So a token's FORM is not always a well-formed Hebrew string on its own.

### 12.3 What Mova does: compare by consonants

`tools/macula` compares lemmas from different sources (MACULA against UD PTNK in `macula compare`, the BDB headwords against MACULA lemmas in the dictionary builder). It does so with one function:

```rust
/// The consonants of a pointed word: lemmas compare by letters, since the order of points varies between sources.
fn consonants(s: &str) -> String {
    s.chars().filter(|c| ('\u{05D0}'..='\u{05EA}').contains(c)).collect()
}
```

(`tools/macula/src/main.rs`.) Keeping only U+05D0–U+05EA removes every vowel, dagesh, accent, shin/sin dot and maqqef, so that the order of points no longer matters. The same function decides which conjunctions are subordinators (`SUBORDINATORS`: כי, אם, למען, פן, טרם, יען, עקב, לבלתי) and which verbs are verbs of saying (`SAYING`: אמר, דבר, צוה, ענה, שאל, קרא).

What this costs:

- **Shin and sin merge.** שׂ and שׁ become the same ש, so שׂים *śym* "put" and a root *šym* would compare equal. In the Torah's function words and verbs of saying no such pair occurs, so the rules above are not affected; in a general lexical comparison it would matter.
- **Homographs merge.** Words that differ only in vowels compare equal: דָּבָר *dāḇār* "word" and דִּבֶּר *dibber* "he spoke"; that is why lemmas, not forms, are compared, and why a lemma comparison is backed by the Strong number where there is one.
- **Final and medial forms stay distinct.** ך and כ are different code points; lemma comparison is unaffected, since a lemma ends where the word ends, but a consonantal search for a root inside an inflected form must map finals to medials first.

A full Unicode normalization (NFD, then a fixed order) would keep the points and still compare equal; it is not done, since the comparison only needs the letters. A gap only if a future tool needs to compare pointed forms.

---

## 13. The letters in Sefer Yetzirah

Sefer Yetzirah ("the Book of Formation") is a short early work that builds the world from the ten numbers (*səp̄îrôṯ*) and the twenty-two letters. It is also, as Khan notes, a document of the history of the language: it classifies the letters by their place of articulation (Khan 2020, vol. 1, §I.0.13.5, pp. 127–129). Its date is disputed, from the Tannaitic period (Scholem) to the 8th–9th century (Allony), with layers of different ages (Khan, ibid.).

### 13.1 Three classes

| Class | Letters | Count | What the book says of them |
|---|---|---:|---|
| mothers (*ʾimmôṯ*) | א מ שׁ (אמ"ש) | 3 | air, water, fire; "a great secret" (ch. 3) |
| doubles (*kəp̄ûlôṯ*) | ב ג ד כ פ ר ת (בג"ד כפר"ת) | 7 | each has two pronunciations, hard and soft (ch. 4) |
| simple (*pəšûṭôṯ*) | ה ו ז ח ט י ל נ ס ע צ ק | 12 | the twelve signs, months, organs (ch. 5) |

The seven doubles are the six begadkefat letters (§7.2) **plus resh**. A resh with two values is not part of the standard Tiberian pointing, which never gives resh a dagesh lene. Khan cites Morag (1960), who took the inclusion of resh as reflecting a Babylonian rather than a Tiberian pronunciation tradition; yet the Tiberian tradition itself, as Khan reconstructs it from the medieval sources, did have two realizations of resh (uvular and alveolar), and the medieval authors call them "resh dagesh" and "resh rafe", though they confuse the terms (Khan 2020, vol. 1, §I.0.13.5 and §I.1.20).

### 13.2 Five places of articulation (Sefer Yetzirah 2:3)

| Place | Letters | Mnemonic |
|---|---|---|
| throat | א ח ה ע | אחה"ע |
| lips | ב ו מ פ | בומ"ף |
| palate | ג י כ ק | גיכ"ק |
| tongue | ד ט ל נ ת | דטלנ"ת |
| teeth | ז ס ש ר צ | זסשר"ץ |

The same five groups are still taught in Hebrew grammar, and the "throat" group is exactly the four gutturals that refuse dagesh forte and prefer hatef vowels (§5, §7.1; GKC §6 has a modern grouping). Resh is among the teeth (alveolars), not with the gutturals, where the Tiberian grammar puts it for its refusal of doubling (GKC §22).

---

## 14. In Mova

What the tools do with the script, as of 2026-10-17.

| Point | Where in our data | What the tools do | Gap |
|---|---|---|---|
| letters, vowels, dagesh, accents | FORM keeps the full pointed and accented string; LEMMA is pointed without accents; `# text` is the verse | `tools/macula` copies MACULA's `unicode` attribute to FORM unchanged | none |
| segmentation | prefixes ו ה ב ל כ מ and suffix pronouns are tokens of their own (`SpaceAfter=No`) | follows MACULA and the UD rules for Hebrew | a point can fall into the next token (GEN.2.4, §12.2) |
| plene / defective | different FORMs | the same LEMMA and Strong number; the dictionary `torah-en.tsv` keys by Strong | none for lemma-level work |
| qamets / qamets hatuf | one code point | not distinguished | a syllable-based classifier would be needed for transliteration |
| shewa vocal / silent | one code point | not distinguished | same |
| ketiv / qere | surface form is the qere where marked; perpetual qere pointed in the text | the trees read the qere (e.g. הִוא with `Gender=Fem`) | the ketiv is not in our trees |
| maqqef | ends the FORM of the first word (עַל־) | treated as part of the token; the token is followed by a space in CoNLL-U | — |
| sof pasuq, פ / ס | in `# text` only | not tokens | the paragraph division is not used by any tool yet |
| point order | sources differ | `consonants()` compares letters only (§12.3) | shin/sin merge in consonantal comparison |
| letter classes | — | none | nothing yet asks whether, e.g., a root has a guttural |

**Ideas, not done.** (1) A transliterator from FORM to the SBL scheme used in this manual, with the shewa and qamets rules of §5–6; it would make the examples checkable. (2) A guttural test for a weak-root classifier ([07-weak-verbs.md](07-weak-verbs.md)): a guttural is exactly the throat group of §13.2, plus resh for the doubling rules. (3) A begadkefat check: predict dagesh lene from the previous word's last vowel and accent, and measure it against the text (§7.2); a negative control is easy (shuffle the accents).

---

## 15. Sources

- **GKC** — W. Gesenius, ed. E. Kautzsch, tr. A. E. Cowley, *Gesenius' Hebrew Grammar*, 2nd English ed., Oxford 1910. Public domain. §§5–17, 20–23, 32l, 102m, 104e, 135q. Wikisource text: `$RESEARCH_RAW/hebgram/open/gesenius-kautzsch-cowley-1910-wikisource/gesenius-kautzsch-cowley-1910-wikisource.txt`; https://en.wikisource.org/wiki/Gesenius%27_Hebrew_Grammar; scan: https://archive.org/details/geseniushebrewgr00geseuoft.
- **Khan 2020** — Geoffrey Khan, *The Tiberian Pronunciation Tradition of Biblical Hebrew*, vol. 1, Open Book Publishers, Cambridge 2020, CC BY 4.0, https://doi.org/10.11647/OBP.0163. §I.0.4–I.0.6 (Masoretic tradition, qere and ketiv, accents), §I.0.13.5 (Sefer Yetzirah, pp. 127–129), §I.1.1–I.1.23 (consonants; §I.1.3 gimel, §I.1.6 vav, §I.1.20 resh), §I.2 (vowels, §I.2.5 shewa and hatefs), §I.2.11 (maqqef), §I.3 (dagesh and rafe), §I.5 (summary). `$RESEARCH_RAW/hebgram/open/khan-tiberian-pronunciation-v1-2020/`.
- **Khan and Hornkohl 2020** — Aaron D. Hornkohl and Geoffrey Khan (eds.), *Studies in Semitic Vocalisation and Reading Traditions*, Open Book Publishers 2020, CC BY 4.0, https://doi.org/10.11647/OBP.0207. `$RESEARCH_RAW/hebgram/open/khan-hornkohl-semitic-vocalisation-2020/`.
- **Davidson 1916** — A. B. Davidson, *An Introductory Hebrew Grammar*, 19th ed., Edinburgh 1916. Public domain. Shewa and syllables. `$RESEARCH_RAW/hebgram/pd/davidson-introductory-hebrew-grammar-1916/`.
- **Harper 1891** — W. R. Harper, *Elements of Hebrew*, New York 1891. Public domain. Shewa and syllable rules. `$RESEARCH_RAW/hebgram/pd/harper-elements-of-hebrew-1891/`.
- **Wickes 1887** — William Wickes, *A Treatise on the Accentuation of the Twenty-One So-called Prose Books of the Old Testament*, Oxford 1887. Public domain. Paseq and legarmeh. `$RESEARCH_RAW/hebgram/pd/wickes-accentuation-prose-1887/`.
- **Ginsburg 1897** — C. D. Ginsburg, *Introduction to the Massoretico-Critical Edition of the Hebrew Bible*, London 1897. Public domain. Extraordinary points, qere and ketiv (cited by GKC §5n, §17). `$RESEARCH_RAW/hebgram/pd/ginsburg-introduction-masoretic-1897/`.
- **Stefaniak 2026** — on the Old Hebrew inscriptions of Tel Arad. `$RESEARCH_RAW/hbo-2026-stefaniak-old-hebrew-inscriptions-tel-arad/`.
- **The Unicode Standard**, chapter 9.1 (Hebrew); the Unicode Character Database (combining classes). https://www.unicode.org/versions/latest/ch09.pdf.
- **Wikipedia**, "Sefer Torah" (traditional letter count), https://en.wikipedia.org/wiki/Sefer_Torah; "Paleo-Hebrew alphabet", https://en.wikipedia.org/wiki/Paleo-Hebrew_alphabet.
- **Our data.** MACULA Hebrew Linguistic Datasets (Biblica, Clear Bible, the Groves Center, OpenScriptures; CC BY 4.0), https://github.com/Clear-Bible/macula-hebrew/, as converted by `tools/macula` into `$MOVA_DATA/corpora/judaism/text/torah-wlc-macula/`; the English JPS 1917 Torah (public domain) in `$MOVA_DATA/corpora/judaism/text/torah-jps1917/`; the Torah dictionary `$MOVA_DATA/corpora/judaism/dict/torah-en.tsv`.
- **Repository.** `tools/macula/README.md` and `tools/macula/src/main.rs` (`consonants`, `SUBORDINATORS`, `SAYING`).

**Wanted** (not in our holdings, or not open): the Aleppo Codex and the Leningrad Codex in facsimile with their Masora; Israel Yeivin, *Introduction to the Tiberian Masorah* (1980, in copyright); a machine-readable ketiv list for the WLC (the OSHB/WLC source files mark ketiv and qere; MACULA keeps only the qere); E. J. Revell's and Aron Dotan's work on the Masoretic pointing (in copyright).
