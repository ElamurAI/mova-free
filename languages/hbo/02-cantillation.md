# 2. Cantillation: the accents (te'amim)

Nearly every word of the Masoretic text carries one more sign besides its vowels: an accent, *ṭaʿam* (plural *ṭəʿāmîm*, in full *ṭaʿămê hammiqrāʾ*, "the senses of the reading"). The accents do three things at once. They mark the stressed syllable of the word; they divide the verse into parts and parts of parts, like a very fine punctuation; and they tell the reader which melodic figure to chant (GKC §15b–d; Khan 2020, vol. 1, §I.0.6). This chapter describes the system of the twenty-one "prose" books, which covers the whole Torah, then the special system of Psalms, Proverbs and Job, then pause, and then measures how the accents line up with the dependency trees of our Torah. For the letters and points see [01-script.md](01-script.md); for stress and syllables, [03-phonology.md](03-phonology.md).

All counts are over the Torah trees in `$MOVA_DATA/corpora/judaism/text/torah-wlc-macula/` (5 853 sentences; FORM keeps every accent), measured on 2026-10-17 with read-only scripts.

## Contents

1. [What the accents are](#1-what-the-accents-are)
2. [Placement: stress, prepositive and postpositive signs](#2-placement-stress-prepositive-and-postpositive-signs)
3. [The prose system: disjunctive accents](#3-the-prose-system-disjunctive-accents)
4. [The prose system: conjunctive accents](#4-the-prose-system-conjunctive-accents)
5. [The continuous dichotomy](#5-the-continuous-dichotomy)
6. [Worked verses](#6-worked-verses)
7. [Pause and pausal forms](#7-pause-and-pausal-forms)
8. [Accents and the letters: begadkefat across words](#8-accents-and-the-letters-begadkefat-across-words)
9. [Double accentuation](#9-double-accentuation)
10. [The poetic system of Psalms, Proverbs and Job](#10-the-poetic-system-of-psalms-proverbs-and-job)
11. [Accents against our dependency trees](#11-accents-against-our-dependency-trees)
12. [Unicode](#12-unicode)
13. [In Mova](#13-in-mova)
14. [Sources](#14-sources)

---

## 1. What the accents are

**Three functions in one sign** (GKC §15b–d):

1. **Stress.** Most accents stand on the stressed syllable of their word. Hebrew stress is usually on the last syllable (*milraʿ*, "below"), less often on the one before (*milʿêl*, "above"), and a few pairs of words differ only by stress: בָּ֫נוּ *bā́nû* "in us" against בָּנ֫וּ *bānû́* "they built"; קָ֫מָה *qā́mâ* "she rose" against קָמָ֫ה *qāmā́* "rising (fem.)" (GKC §15c, k).
2. **Division.** Accents are either *disjunctive* (they end a phrase, like a comma or a semicolon) or *conjunctive* (they bind the word to the next) (GKC §15d). The disjunctives are ranked: a stronger one ends a larger unit. The verse becomes a tree of nested phrases (§5).
3. **Chant.** Each accent names a melodic figure. The melodies differ between communities (Ashkenazi, Sephardi, Yemenite, …); the division they express is the same.

**Age and origin.** The signs were written down in the early Middle Ages, but the division they express is older. Khan reads the cantillation as a layer of oral tradition laid over the qere once the qere was fixed (Khan 2020, vol. 1, §I.0.6). The Talmud already refers to teaching the accents by hand signs (Babylonian Talmud, Berakhot 62a) and uses the term *pisqê ṭəʿāmîm*, "the stops of the accents", for their dividing function; the rabbis read Nehemiah 8:8, "they gave the sense, and caused them to understand the reading", as referring to reading with the accents (Khan 2020, vol. 1, §I.0.6). GKC, following Praetorius and Kahle, traces many of the signs, especially the conjunctives, to the neumes of Greek gospel books (GKC §15b). Some written accent signs seem to have been introduced before the vowel signs (Khan 2020, vol. 1, §I.0.6, n. 47, citing Dotan 1981).

**An interpretation, not only a melody.** Because the division is a reading of the text, it can disagree with other layers of the tradition: with the paragraph division (a paragraph break inside a verse, e.g. GEN.35.22) and, in a number of places, with the vocalization, where a word has a pausal form but a conjunctive accent (Khan 2020, vol. 1, §I.0.6, citing Revell, Dresher 1994, DeCaen and Dresher 2020). De Hoop and Sanders (2022) show with Isaiah 56:9 how much an interpretation can hang on whether a word carries tipcha or a conjunctive.

**Two systems.** The common system is used in twenty-one books ("the 21 books", כ״א). A second system is used in the poetry of Psalms, Proverbs and Job, called by the initials of their Hebrew names תא״ם *ʾemeṯ* (תְּהִלִּים, מִשְׁלֵי, אִיּוֹב) (GKC §15d). The prose frame of Job (1:1–3:1 and 42:7–17) takes the prose accents. The Torah's poems (EXO.15, DEU.32) are accented with the prose system; their poetic layout is a matter of the scroll, not of the accents.

---

## 2. Placement: stress, prepositive and postpositive signs

Most accents sit on the first letter of the stressed syllable, above or below. A few do not (GKC §15e, l):

- **Prepositive** accents stand on the first letter of the word, whatever the stress: yetiv, telisha gedola, and in the poetic books dehi.
- **Postpositive** accents stand on the last letter: pashta, segolta, zarqa, telisha qetana.

For those, the stress has to be found otherwise. The WLC, like Baer's editions and many manuscripts, repeats pashta on the stressed syllable when it is not the last (GKC §15l). That is why the Torah has 6 386 pashta signs on 5 425 words: about one pashta word in six carries it twice.

> **GEN.1.2** · `wlc:GEN:1`
> תֹ֙הוּ֙
> *tṓhû* "formlessness": pashta on the last letter (its proper place) and again on the first syllable, which carries the stress.

**Meteg and the secondary stress.** A second, lighter stress on a long word is marked with meteg (01-script.md §9.2). On the last word of the verse the same sign is the strongest accent, silluq.

**Maqqef.** Words joined by maqqef form one stress unit and carry one accent, on the last word (01-script.md §9.1). For the counts below, a maqqef chain is one "graphic word".

---

## 3. The prose system: disjunctive accents

The table lists the disjunctives of the 21 books in order of strength, with the sign on the letter ב (or on its own), the Unicode code point, its position and the number of graphic words in the Torah that carry it (2026-10-17). The rank column follows the old scheme of emperors, kings, dukes and counts. GKC points out that this scheme comes from a 17th-century Christian work (Bohlius, *Scrutinium S. S. ex accentibus*, Rostock 1636) and calls it a source of confusion; the order of strength itself follows Wickes (GKC §15, note to §15g). It is kept here as a convenient name for the four levels that the Masoretic system does have.

| Rank | Accent | Sign | Code point | Position | Torah words |
|---|---|---|---|---|---:|
| I (emperor) | *silluq* (סִלּוּק, "end") | בֽ ׃ | U+05BD (same as meteg) | below, stressed syllable of the last word | 5 853 (one per sentence) |
| I | *ʾatnaḥ* (אַתְנָח, "rest") | ב֑ | U+0591 | below | 5 507 |
| II (king) | *səḡôltāʾ* | ב֒ | U+0592 | above, postpositive | 366 |
| II | *šalšeleṯ* (with paseq) | ב֓ ׀ | U+0593 + U+05C0 | above | 4 |
| II | *zāqēp̄ qāṭôn* | ב֔ | U+0594 | above | 6 983 |
| II | *zāqēp̄ gāḏôl* | ב֕ | U+0595 | above | 525 |
| II | *ṭip̄ḥāʾ* (*ṭarḥāʾ*) | ב֖ | U+0596 | below | 11 247 |
| III (duke) | *rəḇîaʿ* | ב֗ | U+0597 | above | 2 430 |
| III | *zarqāʾ* (*ṣinnôr*) | ב֮ | U+05AE (mostly) / U+0598 | above, postpositive | 369 |
| III | *pašṭāʾ* | ב֙ | U+0599 | above, postpositive | 5 425 |
| III | *yəṯîḇ* | ב֚ | U+059A | below, prepositive | 355 |
| III | *təḇîr* | ב֛ | U+059B | below | 2 680 |
| IV (count) | *gereš* | ב֜ | U+059C | above | 1 111 |
| IV | *gərāšayim* | ב֞ | U+059E | above | 512 |
| IV | *pāzēr* | ב֡ | U+05A1 | above | 154 |
| IV | *qarnê p̄ārâ* (*pāzēr gāḏôl*) | ב֟ | U+059F | above | 1 |
| IV | *təlîšāʾ gəḏôlâ* | ב֠ | U+05A0 | above, prepositive | 266 |
| IV | *ləḡarmēh* (munach + paseq) | ב֣ ׀ | U+05A3 + U+05C0 | below | 312 |

Notes (GKC §15f; Wickes 1887):

- **Silluq** is always on the last word, before *sôp̄ pāsûq* ׃. In the Unicode encoding it is the same code point as meteg; a tool knows it is silluq by position.
- **Atnach** is the main divider inside the verse. A verse of two or three words, and some longer ones, has no atnach; tipcha then does its job (GKC §15f, 5). In the Torah 367 of 5 853 sentences have no atnach. Typical are the refrains of Genesis 1:

> **GEN.1.13** · `wlc:GEN:12`
> וַֽיְהִי־עֶ֥רֶב וַֽיְהִי־בֹ֖קֶר י֥וֹם שְׁלִישִֽׁי׃
> *wayhî-ʿereḇ wayhî-ḇōqer yôm šəlîšî* — and-was-evening and-was-morning day third — "And there was evening and there was morning, a third day." The main break is tipcha on בֹ֖קֶר, not atnach.

- **Segolta** marks a division before atnach in a long first half: it is the third level of division, counting back from atnach (GKC §15f, 3a). It occurs only in the first half of a verse.
- **Shalshelet** replaces segolta when the segolta clause would be a single word at the head of the verse; it is very rare, seven times in the Bible (GKC §15f, 3b), and in the Torah exactly four times, all in our text: GEN.19.16, GEN.24.12, GEN.39.8, LEV.8.23. Its long, wavering melody has drawn commentary on the hesitation of the character in each (Lot lingering, the servant praying, Joseph refusing).

> **GEN.39.8** · `wlc:GEN:1157`
> וַיְמָאֵ֓ן ׀ וַיֹּ֙אמֶר֙ אֶל־אֵ֣שֶׁת אֲדֹנָ֔יו
> *wayəmāʾēn wayyōʾmer ʾel-ʾēšeṯ ʾăḏōnāyw* — and-he-refused and-said to-the-wife-of his-master — "But he refused, and said unto his master's wife." Shalshelet with paseq on the first word.

- **Zaqef qatan** and **zaqef gadol** divide the half-verses. "Little" zaqef is in fact the stronger; zaqef gadol stands where the zaqef word has no conjunctive before it (GKC §15f, 4).
- **Tipcha** is the last division before atnach and before silluq (GKC §15f, 5). It is the commonest accent in the Torah: 11 247 words.
- **Revia, zarqa, pashta, yetiv, tevir** are the third level. Pashta and yetiv are the same function: yetiv is used instead of pashta on a word stressed on its first syllable with no conjunctive before it (GKC §15f, 8b).
- **Zarqa** is encoded in the WLC mostly with U+05AE (named ZINOR in Unicode, the poetic name of the same postpositive sign): 365 of the Torah's 375 zarqa signs; the other 10 use U+0598 ZARQA.
- **Geresh, gershayim, pazer, telisha gedola, legarmeh** are the fourth level. Gershayim replaces geresh on a word stressed on its last syllable when no qadma precedes; *qarnê p̄ārâ* ("cow's horns") is a heavier pazer, 16 times in the Bible (GKC §15f, 10–11), once in the Torah (NUM.35.5).

---

## 4. The prose system: conjunctive accents

Conjunctives join a word to the next. Each disjunctive has its own set of possible conjunctives before it, its "servants" (*məšārəṯîm*); the rules are strict and were worked out in detail by Wickes (1887), whose chapters treat each disjunctive with its servants (GKC §15o).

| Accent | Sign | Code point | Position | Torah words (graphic, not followed by a disjunctive on the same word) |
|---|---|---|---|---:|
| *mûnaḥ* | ב֣ | U+05A3 | below | ~9 000 |
| *mahpāḵ* (*məhuppāḵ*) | ב֤ | U+05A4 | below | 3 011 |
| *mêrḵāʾ* | ב֥ | U+05A5 | below | ~9 000 |
| *mêrḵāʾ kəp̄ûlâ* | ב֦ | U+05A6 | below | 5 marks |
| *darḡāʾ* | ב֧ | U+05A7 | below | ~1 090 |
| *qaḏmāʾ* (*ʾazlāʾ*) | ב֨ | U+05A8 | above | ~1 900 |
| *təlîšāʾ qəṭannâ* | ב֩ | U+05A9 | above, postpositive | ~450 |
| *yeraḥ ben yômô* (*galgal*) | ב֪ | U+05AA | below | 1 |

The pairs actually found in the Torah show the system at work. For every conjunctive word, the next disjunctive word was counted (2026-10-17, single sentences, maqqef chains as one word):

| Conjunctive | is followed by | count | share of that conjunctive |
|---|---|---:|---:|
| mahpach | pashta | 3 011 | 100 % |
| darga | tevir | 1 013 | 93 % |
| telisha qetana | geresh | 332 | 74 % |
| merkha | tipcha | 5 815 | 64 % |
| merkha | silluq | 2 373 | 26 % |
| qadma | geresh | 950 | 50 % |
| qadma | pashta | 516 | 27 % |
| munach | zaqef qatan | 4 003 | 45 % |
| munach | atnach | 2 296 | 26 % |
| munach | revia | 1 636 | 18 % |

So mahpach serves only pashta, darga almost only tevir, merkha mostly tipcha and silluq, munach mostly zaqef and atnach. The common chant figures *merkha tipcha … silluq* and *munach atnach* (the ends of the two halves) and *mahpach pashta … zaqef* are just these statistics. A model that predicts the next accent from the current one would learn most of Wickes's tables from 25 000 examples; that is a first sanity check for any accent parser (§13).

---

## 5. The continuous dichotomy

Wickes stated the rule that governs the whole prose system: every verse, however short, is divided into two parts; each part, if it is long enough, is divided again into two; and so on down to single words. In his words, it is "the law of the CONTINUOUS DICHOTOMY" (Wickes 1887, ch. III, p. 29). The main division is usually marked by atnach; each half is divided by the next weaker accent that ends a sub-part (zaqef, segolta, tipcha), each of those parts by the next level (pashta, revia, tevir, zarqa), and so on.

Wickes held that the purpose of so fine a division was chiefly musical, though it also marks the "logical and syntactical interpunction" (Wickes 1887, pp. 29–30); he gives a separate chapter to the "syntactical dichotomy" (ch. IV). GKC sums up the same picture with the image of a verse as a province governed by its main divider, with subordinate governors of smaller districts (GKC §15m).

Read as a tree, the accents give a binary bracketing of every verse. Wu and Lowery (2006) coded the rules as a context-free grammar, parsed the whole Hebrew Bible with a CYK parser and found the Masoretic annotation highly consistent; their treebank is an XML tree per verse (the paper is CC BY-NC, cited here, not copied). Prosodic phonology reads the same tree as a prosodic hierarchy: Dresher (1994) and DeCaen and Dresher (2020) map the levels to prosodic domains, and Pitcher (2021) builds an intonation-based model on an Ashkenazi chant tradition and tests it on restrictive and non-restrictive relative clauses.

---

## 6. Worked verses

### 6.1 GEN.1.1: two halves, each split once more

> **GEN.1.1** · `wlc:GEN:0`
> בְּרֵאשִׁ֖ית בָּרָ֣א אֱלֹהִ֑ים אֵ֥ת הַשָּׁמַ֖יִם וְאֵ֥ת הָאָֽרֶץ׃
> *bərēʾšîṯ bārāʾ ʾĕlōhîm ʾēṯ haššāmayim wəʾēṯ hāʾāreṣ* — "In the beginning God created the heaven and the earth." (JPS 1917)

| Word | Accent | Kind |
|---|---|---|
| בְּרֵאשִׁ֖ית | tipcha | disjunctive II |
| בָּרָ֣א | munach | conjunctive |
| אֱלֹהִ֑ים | atnach | disjunctive I |
| אֵ֥ת | merkha | conjunctive |
| הַשָּׁמַ֖יִם | tipcha | disjunctive II |
| וְאֵ֥ת | merkha | conjunctive |
| הָאָֽרֶץ | silluq | disjunctive I |

The dichotomy:

```
[[ bərēʾšîṯ ] [ bārāʾ ʾĕlōhîm ]]  ‖  [[ ʾēṯ haššāmayim ] [ wəʾēṯ hāʾāreṣ ]]
   tipcha       munach  ATNACH         merkha  tipcha      merkha  SILLUQ
```

The atnach puts the verb and its subject in the first half and the objects in the second. In our tree the verb בָּרָא is the root; the only arc crossing the atnach is root → obj (הַשָּׁמַיִם), and the conjoined object hangs on the first object. The tipcha of the first half separates the time phrase (`obl`) from verb and subject; the tipcha of the second half separates the two coordinated objects.

### 6.2 GEN.1.7: segolta in a long first half

> **GEN.1.7** · `wlc:GEN:6`
> וַיַּ֣עַשׂ אֱלֹהִים֮ אֶת־הָרָקִיעַ֒ וַיַּבְדֵּ֗ל בֵּ֤ין הַמַּ֙יִם֙ אֲשֶׁר֙ מִתַּ֣חַת לָרָקִ֔יעַ וּבֵ֣ין הַמַּ֔יִם אֲשֶׁ֖ר מֵעַ֣ל לָרָקִ֑יעַ וַֽיְהִי־כֵֽן׃
> *wayyaʿaś ʾĕlōhîm ʾeṯ-hārāqîaʿ wayyaḇdēl bên hammayim ʾăšer mittaḥaṯ lārāqîaʿ ûḇên hammayim ʾăšer mēʿal lārāqîaʿ wayhî-ḵēn* — "And God made the firmament, and divided the waters which were under the firmament from the waters which were above the firmament; and it was so."

- Atnach on לָרָקִ֑יעַ: the second half is only וַֽיְהִי־כֵֽן "and it was so".
- The long first half is divided by segolta on הָרָקִיעַ֒ (with zarqa on אֱלֹהִים֮ before it): "God made the firmament" ‖ "and divided …".
- The second part of the first half is divided by zaqef qatan on the first לָרָקִ֔יעַ: "between the waters under the firmament" ‖ "and between the waters above".
- Revia on וַיַּבְדֵּ֗ל, pashta on הַמַּ֙יִם֙ and אֲשֶׁר֙ divide further.

Each of the three top breaks falls between clauses or coordinated phrases: wayyiqtol | wayyiqtol | wayyiqtol at the segolta and atnach, and the two `bên … ûḇên` phrases at the zaqef.

### 6.3 GEN.22.1: speech inside narrative

> **GEN.22.1** · `wlc:GEN:548`
> וַיְהִ֗י אַחַר֙ הַדְּבָרִ֣ים הָאֵ֔לֶּה וְהָ֣אֱלֹהִ֔ים נִסָּ֖ה אֶת־אַבְרָהָ֑ם וַיֹּ֣אמֶר אֵלָ֔יו אַבְרָהָ֖ם וַיֹּ֥אמֶר הִנֵּֽנִי׃
> *wayhî ʾaḥar haddəḇārîm hāʾēlleh wəhāʾĕlōhîm nissâ ʾeṯ-ʾaḇrāhām wayyōʾmer ʾēlāyw ʾaḇrāhām wayyōʾmer hinnēnî* — "And it came to pass after these things, that God did prove Abraham, and said unto him: 'Abraham'; and he said: 'Here am I.'"

The atnach closes the narrative frame (the time clause with *wayhî*, then "God tested Abraham"). The second half is the dialogue, split by tipcha after the vocative אַבְרָהָ֖ם: God's call ‖ Abraham's answer. The accents place the boundary of direct speech exactly; see [09-discourse.md](09-discourse.md) on *wayhî* and on speech.

### 6.4 NUM.6.24: a short verse

> **NUM.6.24** · `wlc:NUM:242`
> יְבָרֶכְךָ֥ יְהוָ֖ה וְיִשְׁמְרֶֽךָ׃
> *yəḇāreḵḵā YHWH wəyišmərekā* — "The LORD bless thee, and keep thee." Three words: merkha, tipcha, silluq; no atnach. The tipcha divides "may the LORD bless you" from "and keep you".

---

## 7. Pause and pausal forms

A word under a strong disjunctive, chiefly silluq and atnach (in the poetic books also ʿôleh wəyôrēḏ), is "in pause": its stressed syllable gets extra weight, and its vowels often change (GKC §29i–v). The main changes:

- a short vowel in the stressed syllable becomes long: מַ֫יִם *máyim* → מָ֫יִם *mā́yim*; קָטַל → קָטָ֑ל (GKC §29k);
- a segolate's *e* goes back to *ā*: אֶ֫רֶץ *ʾéreṣ* → אָ֫רֶץ *ʾā́reṣ* (GKC §29k);
- a vowel reduced to shewa before an ending comes back: קָֽטְלָה → קָטָ֑לָה (GKC §29m);
- the stress moves back: אָנֹכִ֫י → אָנֹ֫כִי, אַתָּ֫ה → אָ֫תָּה (GKC §29o);
- in the wayyiqtol, the stress moves forward: וַיָּ֫מָת → וַיָּמֹ֫ת (GKC §29p).

Lesser disjunctives (zaqef, segolta, revia, even tipcha) sometimes bring a pausal form too; GKC calls that the "lesser pause" (GKC §29i).

**Measured in the Torah (2026-10-17).** Two nouns whose pausal form is plain in the letters:

*מַיִם "water"* (Strong 4325), forms with patah (context) or qamets (pause) under the first letter, by the accent on the word:

| Accent on the word | context מַיִם | pausal מָיִם |
|---|---:|---:|
| silluq | 0 | 29 |
| atnach | 0 | 25 |
| zaqef qatan | 19 | 1 |
| tipcha | 44 | 0 |
| pashta | 16 | 0 |
| merkha | 10 | 0 |
| tevir | 9 | 0 |
| revia | 7 | 0 |
| other | 15 | 0 |

*אֶרֶץ "land"* (Strong 776), without the article (after the article the form is always הָאָרֶץ, so it says nothing about pause), segol or qamets under the alef:

| Accent | context אֶרֶץ | pausal אָרֶץ |
|---|---:|---:|
| silluq | 0 | 3 |
| atnach | 0 | 1 |
| zaqef qatan | 2 | 1 |
| munach | 126 | 0 |
| merkha | 97 | 0 |
| tipcha | 17 | 0 |
| other conjunctive and disjunctive | 39 | 0 |

The rule holds without exception for the two emperors: 58 of 58 forms under silluq or atnach are pausal. Under zaqef qatan the pausal form is rare (2 of 23), elsewhere absent. The pause follows the accent, not the syntax: the same noun in the same role is pausal at the verse end and not elsewhere.

> **GEN.1.2** · `wlc:GEN:1`
> … עַל־פְּנֵ֥י הַמָּֽיִם׃
> *ʿal-pənê hammā́yim* — "over the face of the waters": silluq, pausal qamets. Earlier in the chapter, GEN.1.6 בְּת֣וֹךְ הַמָּ֑יִם is pausal under atnach, and בֵּ֥ין מַ֖יִם with tipcha keeps the context form *máyim*.

**Why it matters for the tools.** A pausal form is a different string. A lemmatizer, a morphological analyser or a form-to-gloss table that has not seen pausal forms will miss them; and a generator that writes Hebrew must choose the form from the accent. MACULA gives the same lemma and Strong number to both, so in our trees the difference is only in FORM.

---

## 8. Accents and the letters: begadkefat across words

The accents also decide a detail of the letters. A word-initial ב ג ד כ פ ת is soft (no dagesh lene) when the previous word ends in a vowel *and* is joined to it closely, by a conjunctive accent or maqqef; after a disjunctive it is hard (GKC §21b; 01-script.md §7.2).

> **GEN.2.4** · `wlc:GEN:34`
> אֵ֣לֶּה תוֹלְד֧וֹת — munach on *ʾēlleh*, so the ת of *ṯôləḏôṯ* is soft.

> **GEN.1.1** · `wlc:GEN:0`
> בְּרֵאשִׁ֖ית בָּרָ֣א — tipcha is disjunctive, and the previous word ends in a consonant anyway: hard ב.

This is the one place where the accents change the spelling of another word, and so the one place where a tool can check the accents against the letters without any outside knowledge: predict the dagesh from the previous word's last vowel and accent, and compare with the text. Not done yet (§13).

---

## 9. Double accentuation

In a few passages the text carries two sets of accents, one over the other (GKC §15p):

- **The Decalogue** (EXO.20.2–17 and DEU.5.6–21). The "lower" accentuation divides the text into verses of ordinary length, used for private reading; the "upper" one makes each commandment one verse, used for public reading on the festival. Short commandments then carry two accents on one word, and a letter may carry both dagesh and rafe.

> **EXO.20.13** · `wlc:EXO:531`
> לֹ֥֖א תִּֿרְצָֽ֖ח
> *lōʾ ṯirṣāḥ* — "Thou shalt not murder." Each word has merkha and tipcha: one reading is a two-word verse (merkha, silluq), the other joins the commandment to the next ones (tipcha). The ת has dagesh and rafe for the two readings.

- **GEN.35.22**, where a later accentuation joins verses 22 and 23 so that the reader passes quickly over Reuben's deed (GKC §15p).

In our trees these verses carry both marks in FORM; the counts in this chapter take the strongest disjunctive on each word, which picks one of the two readings.

---

## 10. The poetic system of Psalms, Proverbs and Job

The three books have their own accents, adapted to short, balanced lines (GKC §15h–i; Wickes 1881). They do not occur in the Torah; this section is for completeness.

**Disjunctives**, strongest first:

| Accent | Notes |
|---|---|
| *silluq* | as in prose |
| *ʿôleh wəyôrēḏ* | a two-part sign (above, then below); main divider of a long verse, stronger than atnach |
| *ʾatnaḥ* | main divider of a short verse; after ʿôleh wəyôrēḏ, divider of the second half |
| *rəḇîaʿ gāḏôl* | |
| *rəḇîaʿ mugrāš* | revia with geresh on the same word |
| *šalšeleṯ gəḏôlâ* | with paseq |
| *ṣinnôr* (zarqa) | postpositive |
| *rəḇîaʿ qāṭôn* | just before ʿôleh wəyôrēḏ |
| *dəḥî* (prepositive tipcha) | under the first letter |
| *pāzēr* | |
| *məhuppāḵ ləḡarmēh*, *ʾazlāʾ ləḡarmēh* | with paseq |

**Conjunctives:** merkha, munach, *ʿillûy* (munach above), tarcha, galgal, mahpach, azla, little shalshelet (without paseq), and *ṣinnôrîṯ* as a helper sign (GKC §15i).

Two differences matter for reading. The verse is divided into two *or three* main parts (ʿôleh wəyôrēḏ, atnach, silluq), matching the tripartite lines of poetry. And atnach after ʿôleh wəyôrēḏ is weaker, roughly a zaqef, so pausal forms under it are less regular (GKC §29o). The treatise is Wickes 1881.

---

## 11. Accents against our dependency trees

The accents were added by readers who knew the language; the trees were built from MACULA's syntax by head rules (tools/macula). Both claim to show how the words of a verse group. How well do they agree?

**Method (2026-10-17).** For each sentence (all 5 853 Torah sentences are one verse or part of one), the words were grouped into graphic words (tokens joined by `SpaceAfter=No` or maqqef). Each graphic word was given its strongest accent (silluq for the last word). At each boundary between two graphic words, three things were measured on the dependency tree:

- **crossing arcs:** how many dependency arcs connect a word on the left of the boundary with a word on the right;
- **adjacent link:** whether some arc links a token of the left word directly with a token of the right word;
- **only root arcs:** whether every crossing arc has the root of the sentence at one end, i.e. the boundary separates whole dependents of the root.

**Results** by the rank of the accent before the boundary:

| Accent before the boundary | Boundaries | Mean crossing arcs | Adjacent link | Only root arcs |
|---|---:|---:|---:|---:|
| conjunctive (any) | 24 159 | 3.02 | **93.0 %** | 18.5 % |
| rank I: atnach | 5 507 | **1.41** | 3.9 % | **83.3 %** |
| rank II (segolta, zaqef, tipcha, …) | 19 125 | 1.92 | 34.2 % | 33.6 % |
| rank III (revia, zarqa, pashta, yetiv, tevir) | 11 259 | 2.79 | 43.9 % | 33.0 % |
| rank IV (geresh, pazer, telisha, legarmeh, …) | 2 356 | 3.45 | 53.0 % | 42.4 % |

Selected single accents:

| Accent | Boundaries | Mean crossing | Adjacent link | Only root arcs |
|---|---:|---:|---:|---:|
| atnach | 5 507 | 1.41 | 3.9 % | 83.3 % |
| segolta | 366 | 2.63 | 3.6 % | 82.0 % |
| zaqef qatan | 6 983 | 1.95 | 11.4 % | 48.4 % |
| revia | 2 430 | 2.69 | 25.9 % | 50.9 % |
| tipcha | 11 247 | 1.87 | 48.2 % | 22.7 % |
| pashta | 5 425 | 2.87 | 49.8 % | 27.9 % |
| tevir | 2 680 | 2.67 | 44.7 % | 25.1 % |
| geresh | 1 111 | 3.62 | 29.1 % | 45.0 % |

What it shows:

- **Conjunctives join.** After a conjunctive, the word is linked directly to the next word 93 % of the time; after atnach, 4 %. The binary distinction of the accents is, in nine cases out of ten, a statement about direct dependency.
- **Atnach cuts at the root.** At 83 % of atnach boundaries every crossing arc comes from the root: the atnach separates whole dependents of the main verb. The root verb is in the first half in 97.4 % of verses with atnach (5 343 of 5 486). The arcs that cross the atnach are, per verse: `conj` from the root (a further clause or a coordinated predicate) in 72.3 %, `obl` 4.3 %, `ccomp` 2.6 %, `advcl` 2.6 %, `nsubj` 2.1 %, `obj` 1.3 %. So the atnach most often falls between two coordinated clauses, as in the wayyiqtol chains of narrative.
- **Segolta behaves like atnach** (3.6 % adjacent link, 82 % root-only): it marks a division of the same kind, one level down, in the first half.
- **The ranks are ordered** in the mean number of crossing arcs (1.4 → 1.9 → 2.8 → 3.5), as the dichotomy predicts. The lower ranks also sit deeper in the verse, where more arcs pass over any boundary, so the trend is partly a matter of position; the adjacent-link and root-only columns are less sensitive to that.
- **Tipcha and the other "near" disjunctives are mixed.** Tipcha has an adjacent link 48 % of the time: it is often the last division before atnach or silluq, inside a short phrase such as "the heavens and the earth", where the two sides are a head and its conjunct. The accents make a binary prosodic tree; a dependency tree has no such constraint, so a short prosodic break inside a noun phrase is expected.

The measure is a first one: one corpus, our own trees (not hand-made UD), no significance test. It shows that the accents carry a large part of the syntactic grouping and that atnach is a reliable signal of a main-clause boundary. It says nothing yet about which of the two (accents or trees) is right where they disagree. Wu and Lowery (2006) found the accents internally consistent; whether our trees are consistent with the accents is the question measured here.

---

## 12. Unicode

The accents occupy U+0591–U+05AF. All have combining class 220 (below), 222 (below, left), 228 (above, left) or 230 (above), larger than any vowel point (10–25), so normalization puts them after the points on the same letter (01-script.md §12.2).

| Code point | Unicode name | Accent | Prose / poetic | Torah marks |
|---|---|---|---|---:|
| U+0591 | ETNAHTA | atnach | both | 5 536 |
| U+0592 | SEGOL | segolta | prose | 373 |
| U+0593 | SHALSHELET | shalshelet | both | 4 |
| U+0594 | ZAQEF QATAN | zaqef qatan | prose | 7 018 |
| U+0595 | ZAQEF GADOL | zaqef gadol | prose | 526 |
| U+0596 | TIPEHA | tipcha (tarcha, poetic: conjunctive) | both | 11 317 |
| U+0597 | REVIA | revia | both | 2 443 |
| U+0598 | ZARQA | zarqa | prose | 10 |
| U+0599 | PASHTA | pashta | prose | 6 386 |
| U+059A | YETIV | yetiv | prose | 356 |
| U+059B | TEVIR | tevir | prose | 2 684 |
| U+059C | GERESH | geresh | prose | 1 121 |
| U+059D | GERESH MUQDAM | geresh (variant) | prose | 1 |
| U+059E | GERSHAYIM | gershayim | prose | 512 |
| U+059F | QARNEY PARA | qarne para | prose | 1 |
| U+05A0 | TELISHA GEDOLA | telisha gedola | prose | 269 |
| U+05A1 | PAZER | pazer | both | 159 |
| U+05A2 | ATNAH HAFUKH | (variant sign) | — | 0 |
| U+05A3 | MUNAH | munach | both | 9 515 |
| U+05A4 | MAHAPAKH | mahpach | both | 3 053 |
| U+05A5 | MERKHA | merkha | both | 9 164 |
| U+05A6 | MERKHA KEFULA | merkha kefula | prose | 5 |
| U+05A7 | DARGA | darga | prose | 1 093 |
| U+05A8 | QADMA | qadma / azla | both | 2 074 |
| U+05A9 | TELISHA QETANA | telisha qetana | prose | 460 |
| U+05AA | YERAH BEN YOMO | galgal | both | 1 |
| U+05AB | OLE | ʿôleh (of ʿôleh wəyôrēḏ) | poetic | 0 |
| U+05AC | ILUY | ʿillûy | poetic | 0 |
| U+05AD | DEHI | dəḥî | poetic | 0 |
| U+05AE | ZINOR | ṣinnôr; used by the WLC for prose zarqa | both | 365 |
| U+05AF | MASORA CIRCLE | Masoretic note mark | — | 0 |
| U+05BD | METEG | silluq / meteg | both | 10 265 |
| U+05C0 | PASEQ | paseq; with a conjunctive: legarmeh | both | 387 |
| U+05C3 | SOF PASUQ | end of verse | both | 5 819 |

Two signs serve two functions each: U+05BD is silluq on the last word and meteg elsewhere; U+0596 is the disjunctive tipcha in prose and the conjunctive tarcha in the poetic books (the poetic disjunctive dehi has its own code point, U+05AD). A tool has to decide by position and by book, not by code point alone.

---

## 13. In Mova

**What is there (2026-10-17).**

- **The accents are kept.** `tools/macula` copies MACULA's `unicode` form into FORM with every accent; `# text` holds the accented verse. LEMMA has the vowels but no accents.
- **They are stripped for comparison.** Lemma comparison goes through `consonants()`, which keeps only the letters (01-script.md §12.3); the accents never enter a comparison.
- **No tool reads them.** The head rules of `tools/macula` take the structure from MACULA's word groups only. `macula prag` decides form, act, polarity and voice from the verb forms (`Type=wayyiqtol`, `imperative`, …), the persons and a few particles; it does not look at the accents. The dictionary (the dictionary builder) and its examples carry the accented verse as text.
- **This chapter's measurements** (§4, §7, §11) were made with throw-away read-only scripts, not with a tool in the repository. They are reproducible from the description, but not yet a gate.

**What one could do (not done; listed as ideas, each needing the operator's word before it is built).**

1. **An accent reader in `tools/macula`.** Map each graphic word to its accent (an `enum Accent` with its rank, by code point and position, with the two double-duty signs of §12 resolved), write it to MISC (`Accent=atnach`), and build the dichotomy tree of each verse. Gate: the tree must be strictly binary and nested at every verse (Wu and Lowery 2006 report the Masoretic annotation consistent enough for a CFG); a negative control shuffles the accents of a verse and must fail.
2. **Accents as a feature for the parser.** §11 shows that a conjunctive predicts a direct link with 93 % and atnach a root-level cut with 83 %. A head-rule fallback (the 598 groups that MACULA leaves without a rule) or a learned parser for unpointed Hebrew could use them. Measure on `macula compare` against PTNK, train/dev only.
3. **Pausal forms in the dictionary.** Tag a FORM as pausal when it carries silluq or atnach and differs from the context form of the same lemma (§7); list both in `torah-en.tsv`.
4. **Begadkefat as a check.** Predict dagesh lene from the previous word's ending and accent (§8) and report the agreement; it tests both the accent reader and the transliteration of 01-script.md.
5. **Accent boundaries in `macula prag`.** Direct speech in the narrative is mostly cut off by a major disjunctive before it (§6.3). Using the atnach and zaqef boundaries to split a verse into a narrator's frame and a character's speech would refine `voice`, which today is decided for the whole sentence.

---

## 14. Sources

- **GKC** — W. Gesenius, ed. E. Kautzsch, tr. A. E. Cowley, *Gesenius' Hebrew Grammar*, 2nd English ed., Oxford 1910. Public domain. §15 (the accents: b–d functions, e–g prose accents, h–i poetic accents, k–p remarks, note on the four ranks and Bohlius 1636), §16 (maqqef, meteg), §21 (begadkefat), §29 (tone and pause: i the pause, k–p its effects). Wikisource text: `$RESEARCH_RAW/hebgram/open/gesenius-kautzsch-cowley-1910-wikisource/gesenius-kautzsch-cowley-1910-wikisource.txt`; https://en.wikisource.org/wiki/Gesenius%27_Hebrew_Grammar.
- **Wickes 1887** — William Wickes, *A Treatise on the Accentuation of the Twenty-One So-called Prose Books of the Old Testament* (טעמי כ״א ספרים), Oxford 1887. Public domain. Ch. III "The dichotomy" (p. 29, "the law of the CONTINUOUS DICHOTOMY"), ch. IV "Syntactical dichotomy". `$RESEARCH_RAW/hebgram/pd/wickes-accentuation-prose-1887/treatiseonaccent00wickuoft/`; https://archive.org/details/treatiseonaccent00wickuoft.
- **Wickes 1881** — William Wickes, *A Treatise on the Accentuation of the Three So-called Poetical Books of the Old Testament: Psalms, Proverbs, and Job* (טעמי אמ״ת), Oxford 1881. Public domain. `$RESEARCH_RAW/hebgram/pd/wickes-accentuation-poetical-1881/treatiseonacpoet00wickuoft/`.
- **Praetorius 1901** — Franz Praetorius, *Über die Herkunft der hebräischen Accente*, Berlin 1901. Public domain. On the Greek origin of the signs (cited by GKC §15b). `$RESEARCH_RAW/hebgram/pd/praetorius-herkunft-accente-1901/`.
- **Khan 2020** — Geoffrey Khan, *The Tiberian Pronunciation Tradition of Biblical Hebrew*, vol. 1, Open Book Publishers 2020, CC BY 4.0, https://doi.org/10.11647/OBP.0163. §I.0.5 (qere and ketiv), §I.0.6 (the accents: an oral layer over the qere; Berakhot 62a; *pisqê ṭəʿāmîm*; Neh 8:8; conflicts with paragraphing and with pausal vocalization), §I.2.11 (maqqef). `$RESEARCH_RAW/hebgram/open/khan-tiberian-pronunciation-v1-2020/`.
- **De Hoop and Sanders 2022** — Raymond de Hoop and Paul Sanders, "The System of Masoretic Accentuation: Some Introductory Issues", *Journal of Hebrew Scriptures* 22, art. 2, 2022, CC BY 4.0, https://doi.org/10.5508/jhs29622. `$RESEARCH_RAW/hbo-2022-hoop-system-masoretic-accentuation-some-introductory/`.
- **Pitcher 2021** — Sophia L. Pitcher, "Towards a prosodic model for Tiberian Hebrew: An intonation-based analysis", *Stellenbosch Papers in Linguistics Plus* 63, 2021, 1–27, https://doi.org/10.5842/63-0-934 (CC BY). `$RESEARCH_RAW/hbo-2021-pitcher-towards-prosodic-model-tiberian-hebrew/`.
- **Wu and Lowery 2006** — Andi Wu and Kirk Lowery, "A Hebrew Tree Bank Based on Cantillation Marks", LREC 2006, https://aclanthology.org/L06-1002/ (CC BY-NC 4.0: cited, not copied). `$RESEARCH_RAW/lingbulk-2006-wu-hebrew-tree-bank-based-cantillation/`.
- **Naudé and Miller-Naudé 2017** — Jacobus A. Naudé and Cynthia L. Miller-Naudé, "At the interface of syntax and prosody: differentiating left dislocated and tripartite verbless clauses in Biblical Hebrew", *Stellenbosch Papers in Linguistics* 48, 223–238, https://doi.org/10.5774/48-0-293 (CC BY): accents as evidence for syntactic analysis. `$RESEARCH_RAW/hbo-2018-naude-interface-syntax-prosody-differentiating-left/`.
- **The Unicode Standard**, ch. 9.1 Hebrew, and the Unicode Character Database (names and combining classes of U+0591–U+05C7). https://www.unicode.org/versions/latest/ch09.pdf.
- **Our data.** MACULA Hebrew (CC BY 4.0, https://github.com/Clear-Bible/macula-hebrew/) via `tools/macula` → `$MOVA_DATA/corpora/judaism/text/torah-wlc-macula/`; JPS 1917 (public domain) → `$MOVA_DATA/corpora/judaism/text/torah-jps1917/`. Repository: `tools/macula/README.md`, `tools/macula/src/main.rs` (`consonants`, `hebrew_prag`).

**Wanted** (not in our holdings, or in copyright): James D. Price, *The Syntax of Masoretic Accents in the Hebrew Bible* (1990), the fullest rule system for the prose accents and their syntax; Joshua R. Jacobson, *Chanting the Hebrew Bible* (2002; 2nd ed. 2017), the standard practical guide to the melodies and rules; B. Elan Dresher, "The prosodic basis of the Tiberian Hebrew system of accents", *Language* 70 (1994), and Vincent DeCaen and B. Elan Dresher, "Pausal forms and prosodic structure in Tiberian Hebrew" (2020); Mordechai Breuer, *Ṭaʿămê hammiqrāʾ* (1982); Israel Yeivin, *Introduction to the Tiberian Masorah* (1980); the Wu–Lowery cantillation treebank itself (an XML tree per verse, NC; useful as a measure for idea 1 of §13).
