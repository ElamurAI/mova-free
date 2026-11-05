# The lexicon

This chapter describes the vocabulary of Biblical Hebrew as the Torah shows it: how words are built from roots and patterns, how often they occur, which semantic fields they fill, how far their meanings stretch, which of them were borrowed, and how the names work. It ends with the Torah dictionary that Mova builds from the trees (the dictionary builder), with what it holds and where it still goes wrong. Counts are from our MACULA-based trees of the Torah and from the dictionary built from them, both of 2026-10-17. The particles are treated in [05-pronouns-particles.md](05-pronouns-particles.md), the noun and verb morphology in [04-noun.md](04-noun.md), [06-verb.md](06-verb.md) and [07-weak-verbs.md](07-weak-verbs.md), and the later vocabulary in [11-mishnaic.md](11-mishnaic.md) and [12-biblical-mishnaic-modern.md](12-biblical-mishnaic-modern.md).

## Contents

1. [The root](#1-the-root)
2. [Lemmas, Strong's numbers and roots in our data](#2-lemmas-strongs-numbers-and-roots-in-our-data)
3. [Word formation](#3-word-formation)
4. [The frequency profile of the Torah](#4-the-frequency-profile-of-the-torah)
5. [Semantic fields](#5-semantic-fields)
6. [Polysemy and key words](#6-polysemy-and-key-words)
7. [Loanwords](#7-loanwords)
8. [Proper names](#8-proper-names)
9. [The lexicons](#9-the-lexicons)
10. [Mova's Torah dictionary](#10-movas-torah-dictionary)
11. [In Mova](#11-in-mova)
12. [Sources](#12-sources)

Conventions as in [README.md](README.md): SBL academic transliteration; verse, sentence id `wlc:BOOK:n`, pointed text without accents, transliteration, gloss, JPS 1917.

---

## 1. The root

### 1.1 Root and pattern

Most Hebrew words are analysed as a **root** of consonants — usually three — combined with a **pattern** of vowels and affixes. The root carries a broad lexical idea; the pattern carries the word class and a part of the meaning (GKC §30; §§79–86). From the root ד־ב־ר (*d-b-r*) the Torah builds:

| word | pattern | translit. | meaning | Torah |
|---|---|---|---|---:|
| דָּבָר | qāṭāl | dāḇār | word, thing, matter | 256 |
| דִּבֶּר | qiṭṭēl (Piel) | dibber | he spoke | 415 (all forms) |
| מִדְבָּר | miqṭāl | miḏbār | wilderness (different root sense: "driving place" of flocks, BDB) | 105 |
| דְּבִיר | qəṭîl | dəḇîr | inner sanctuary (not in the Torah) | — |

The example also shows the limit of the idea: מִדְבָּר "wilderness" belongs to a homonymous root דבר "to drive (cattle)" in BDB's analysis, so "same consonants" does not mean "same root" (BDB s.v. דבר I, II).

From מ־ל־ך *m-l-k*: מֶלֶךְ *meleḵ* "king" (101), מָלַךְ *mālaḵ* "he reigned" (13), מַמְלָכָה *mamlāḵâ* / מַמְלֶכֶת "kingdom" (12). From ק־ד־שׁ *q-d-š*: קֹדֶשׁ *qōḏeš* "holiness, holy thing" (223), קָדַשׁ *qāḏaš* "be holy; (Piel) consecrate" (75), קָדוֹשׁ *qāḏôš* "holy" (36), מִקְדָּשׁ *miqdāš* "sanctuary" (16).

### 1.2 Biliteral, triliteral, quadriliteral

GKC §30 and §§77–78 discuss the old question whether the triliteral root grew from biliteral bases. Groups of roots sharing two consonants and a related sense suggest so: GKC §30g–h derives from a base קץ "cut off" the stems קצץ, קצה, קצב, קצר and others, lists the developments of רע (רָעַד, רָעַם, רָעַשׁ …) and of בר / פר, and notes in §30m that the two consonants of such a base tend to agree in kind (both emphatic, or both soft). Most "weak" roots (hollow ע״ו/ע״י, geminate ע״ע, ל״ה) behave in inflection as if they had two strong consonants (see [07-weak-verbs.md](07-weak-verbs.md)). A few old nouns have no verb root at all and are better taken as primary: אָב "father", אֵם "mother", יָד "hand", דָּם "blood", שֵׁם "name" (GKC §82).

Quadriliteral roots are rare and mostly nouns: עַקְרָב "scorpion" (DEU.8.15), צְפַרְדֵּעַ "frog" (EXO 7–8), and reduplicated colour adjectives of the pattern *qəṭalṭal*: אֲדַמְדָּם *ʾăḏamdām* "reddish" (6, LEV 13–14), יְרַקְרַק *yəraqraq* "greenish" (2), which weaken the colour (GKC §84b n).

### 1.3 Root families as a reading aid

The Torah plays on roots: וַיִּיצֶר … אֶת־הָאָדָם עָפָר מִן־הָאֲדָמָה "formed the *ʾāḏām* of dust from the *ʾăḏāmâ*" (GEN.2.7); לְזֹאת יִקָּרֵא אִשָּׁה כִּי מֵאִישׁ לֻקֳחָה־זֹּאת "she shall be called *ʾiššâ* because from *ʾîš* she was taken" (GEN.2.23), a pun on sound rather than on etymology (BDB keeps אִישׁ and אִשָּׁה under different roots). Name explanations work the same way (§8.3).

---

## 2. Lemmas, Strong's numbers and roots in our data

### 2.1 What a "lemma" is in the trees

MACULA gives each word a **lemma** (a pointed dictionary form, e.g. אָמַר for any form of "say") and a **Strong's number** (James Strong's 1890 numbering of the Hebrew vocabulary, 1–8674). It does not give the root. Lemma and Strong's number both name a **lexeme**, not a root: the verb דָּבַר (1696) and the noun דָּבָר (1697) are different numbers, as are קָדַשׁ (6942), קֹדֶשׁ (6944) and קָדוֹשׁ (6918). A root view has to be built on top, for example from BDB, which orders its entries by root (§9.1).

Strong's numbers are sometimes too coarse or too fine:

- **Too coarse**: one number covers homonyms. 2617 is both חֶסֶד I "kindness" and חֶסֶד II "shame" (LEV.20.17); 6030 covers עָנָה I "answer" and the homonyms "be bowed down" and "sing". The dictionary then has to choose a sense (§10.4).
- **Extended numbers**: MACULA adds a letter to split what Strong lumped or to number what Strong left out: 2050b is the conjunction ו (not Strong's 2050), 1886a the article, 3807a the preposition לְ, 2050c and others the suffix pronouns; 1121a/b separate senses of בֵּן. Our dictionary keys such words by their lemma (`lemma:וְ`) because the lettered number is not Strong's own (see [tools/macula/README.md](../../tools/macula/README.md)).

### 2.2 Distinct lexemes by part of speech (Torah)

| UPOS | distinct lexemes (Strong) | tokens |
|---|---:|---:|
| NOUN | 1 667 | 29 610 |
| VERB | 852 | 17 922 |
| PROPN | 664 | 8 004 |
| ADJ | 170 | 2 273 |
| ADV | 55 | 2 868 |
| NUM | 34 | 2 422 |
| PRON | 31 | 13 768 |
| ADP | 29 | 21 085 |
| INTJ | 11 | 394 |
| CCONJ, SCONJ, PART, DET, X | 25 | 27 461 |

(Counted on `UPOS` + `Strong` in the five `.conllu` files, 2026-10-17. A few lexemes appear under two UPOS, e.g. a participle as VERB and as NOUN.)

The dictionary built from the trees (§10) has **3 486 keys**: 3 259 Strong's numbers and 227 lemma keys for MACULA's lettered numbers.

---

## 3. Word formation

Hebrew forms nouns and adjectives by internal vowel patterns (Hebrew *mišqālîm*, "weights"), by prefixes, by suffixes, and rarely by compounding (GKC §§81–86; Davidson 1916, the formation of nouns). The patterns below are named with the dummy root ק־ט־ל, as the grammars do.

### 3.1 Patterns of the simple stem

| pattern | translit. | typical meaning | Torah examples (count) |
|---|---|---|---|
| קֶטֶל / קֹטֶל / קֵטֶל (segolates) | qeṭel, qōṭel, qēṭel | concrete and abstract nouns | אֶרֶץ "land" (849), קֹדֶשׁ "holiness" (223), עֶבֶד "servant" (180), זֶרַע "seed" (112), מֶלֶךְ "king" (101), בֹּקֶר "morning" (80), עֶרֶב "evening" (77), חֶסֶד (20), נֶפֶשׁ (205) |
| קָטָל | qāṭāl | nouns, adjectives | דָּבָר (256), זָהָב "gold" (143), בָּשָׂר "flesh" (138), חָכָם "wise" (17) |
| קָטֵל | qāṭēl | stative adjectives | זָקֵן "old; elder" (54), טָמֵא "unclean" (66), כָּבֵד "heavy" |
| קָטוֹל | qāṭôl | adjectives of quality | גָּדוֹל "great" (102), טָהוֹר "pure" (69), קָדוֹשׁ (36), שָׁלוֹם (26) |
| קָטִיל | qāṭîl | adjectives; passive or agent | נָשִׂיא "chief" (71), צָעִיר "young" (8) |
| קְטָלָה, קְטֻלָּה | qəṭālâ, qəṭullâ | feminine abstracts | אֲדָמָה "ground" (96), בְּרָכָה "blessing" (30), צְדָקָה "righteousness" (9), גְּאֻלָּה "redemption right" (9) |

The **segolates** are the largest class: monosyllabic in origin (\*malk, \*qudš, \*sipr), they acquired a helping vowel in the absolute singular and show the old monosyllable again with suffixes: מַלְכּוֹ *malkô* "his king", קָדְשׁוֹ *qoḏšô* "his holiness" (GKC §84a a–e; §93). The noun chapter gives their full inflection ([04-noun.md](04-noun.md)).

### 3.2 Patterns of the intensive stems

| pattern | translit. | typical meaning | Torah examples |
|---|---|---|---|
| קַטָּל | qaṭṭāl | professions, habitual agents | גַּנָּב "thief" (4), טַבָּח "cook, guard" (GEN 37–41) |
| קַטִּיל | qaṭṭîl | intensive adjectives | צַדִּיק "righteous" (17) |
| קִטֵּל | qiṭṭēl | bodily defects | עִוֵּר "blind" (6), פִּסֵּחַ "lame" (2), חֵרֵשׁ "deaf" (2), גִּבֵּחַ "bald" (1), גִּבֵּן "hunchbacked" (LEV.21.20) |
| קִטּוֹל, קִטָּלוֹן | qiṭṭôl, qiṭṭālôn | nouns of action and result | גִּבּוֹר "warrior" (5), זִכָּרוֹן "memorial" (14) |
| קְטַלְטַל | qəṭalṭal | weakened colours | אֲדַמְדָּם "reddish" (6), יְרַקְרַק "greenish" (2) |

(GKC §84b.) The defect pattern is a tight semantic class; Leviticus 21.18–20 lists most of its members in two verses, a good test for a morphological analyser.

### 3.3 Prefixes

| prefix | pattern | meaning | Torah examples (count) |
|---|---|---|---|
| מ־ (מִ, מַ, מְ, מוֹ) | miqṭāl, maqṭēl, maqṭēlâ | place, instrument, abstract of action | מִזְבֵּחַ "altar" (place of slaughter, 198), מִשְׁכָּן "dwelling" (104), מִשְׁפָּט "judgment" (84), מִקְדָּשׁ "sanctuary" (16), מוֹעֵד "appointed time/place" (160), מַטֶּה "staff, tribe" (142), מַעֲשֶׂה "deed" (65), מִשְׁמֶרֶת "charge" (39) |
| ת־ | taqṭîl, təqûlâ, tôqēlâ | abstracts of action, often from weak roots | תּוֹרָה "instruction" (56), תְּרוּמָה "contribution" (44), תּוֹלֵדוֹת "generations" (29), תְּנוּפָה "wave-offering" (28) |
| א־ | ʾaqṭāl | adjectives, nouns | אַכְזָב "deceptive" (not in Torah) |
| י־ | yaqṭūl | names, a few nouns | יִצְהָר "fresh oil" (7), יִשְׂרָאֵל, יִצְחָק (names with verbal prefix, §8) |

(GKC §85.) The prefix מ־ is by far the most productive; in Mishnaic Hebrew it remains so, and the ת־ abstracts multiply (Segal 1927).

### 3.4 Suffixes

| suffix | meaning | Torah examples |
|---|---|---|
| ־ָה, ־ֶת, ־ַת (feminine) | feminine gender, abstracts, collectives, single item | תּוֹרָה, בְּרָכָה, מַמְלֶכֶת; שֵׂעָר "hair" vs שַׂעֲרָה "a single hair" (not in Torah) |
| ־וֹן / ־ָן | abstracts, adjectives, diminutives | זִכָּרוֹן (14), שַׁבָּתוֹן "solemn rest" (11), רִאשׁוֹן "first" (55), עִצָּבוֹן "pain" (GEN.3.16), עֵרָבוֹן "pledge" (3) |
| ־וּת | abstracts (later frequent) | עֵדוּת "testimony" (38), כְּסוּת "covering" (4) |
| ־ִית | abstracts; ordinal-like | רֵאשִׁית "beginning" (18), אַחֲרִית "end" (10) |
| ־ִי (nisbe) | gentilic and relational adjectives, ordinals | לֵוִי "Levite" (87), אֱמֹרִי (40), כְּנַעֲנִי (31), עִבְרִי "Hebrew" (22), מִצְרִי (19); שֵׁנִי "second", שְׁבִיעִי "seventh" |
| ־ָם, ־וֹם | adverbs, some nouns | יוֹמָם "by day", חִנָּם "for nothing" |

(GKC §86.) MACULA marks the gentilics with `Type=gentilic` (436 tokens in the Torah) and the ordinals with `Type=ordinal_number` (235). The abstract suffix ־וּת is rare in the Torah and grows in Late Biblical and Mishnaic Hebrew (מַלְכוּת "kingdom" is late), one of the standard markers of a text's age (see [12-biblical-mishnaic-modern.md](12-biblical-mishnaic-modern.md)).

### 3.5 Denominatives and compounds

Some verbs are made from nouns: from לְבֵנָה "brick", לָבַן "make bricks" (GEN.11.3; EXO.5.7); from שֹׁרֶשׁ "root", שֵׁרֵשׁ "uproot" (not in the Torah); from כֹּפֶר "pitch", כָּפַר "smear with pitch" (GEN.6.14, a hapax sense next to the cultic כִּפֶּר "atone", 78). Benbaji, Doron and Hénot-Mortier 2022 show that word embeddings of Modern Hebrew place denominal verbs nearer their noun than root-derived verbs, a measurable trace of this layer. True compounds are rare outside names: צַלְמָוֶת "deep darkness" (traditionally "shadow of death", not in the Torah), בְּלִיַּעַל "worthlessness" (DEU.13.14), and the construct chain does the work of compounding elsewhere (בֵּית אָב "father's house" = family; see [04-noun.md](04-noun.md)).

---

## 4. The frequency profile of the Torah

### 4.1 Totals

From the Torah dictionary (`torah-en.tsv`, 2026-10-17):

- **125 558** tokens over **3 486** keys.
- **17** keys occur 1 000 times or more; **151** occur 100 or more; **873** occur 10 or more.
- **1 103** keys (31.6 %) occur once (*hapax legomena* within the Torah); **2 255** (64.7 %) occur five times or fewer.
- The **50** commonest keys make up **80 989** tokens (64.5 %) — and the first eleven of them are particles and pronouns.

A reader who knows the 150 words with 100 or more occurrences recognises about three words in four; the remaining quarter is spread over more than three thousand lexemes, a third of them seen only once. This long tail is the main lexical difficulty of the text, and the reason a learner (or a model) needs a dictionary at hand.

### 4.2 The fifty commonest lexemes

| # | lemma | translit. | commonest MACULA glosses | count |
|---:|---|---|---|---:|
| 1 | וְ | wə- | and; so | 13 768 |
| 2 | הוּא (suffixes) | — | your, his, you | 12 092 |
| 3 | הַ | ha- | the | 9 128 |
| 4 | לְ | lə- | to, for, before | 5 729 |
| 5 | אֵת | ʾēṯ | (object marker) | 4 147 |
| 6 | בְּ | bə- | in, on, with | 3 881 |
| 7 | מִן | min | from, of | 2 004 |
| 8 | אֲשֶׁר | ʾăšer | which, who, that | 1 908 |
| 9 | יהוה | YHWH | Yahweh | 1 813 |
| 10 | אֶל | ʾel | to, into | 1 682 |
| 11 | כֹּל | kōl | all, every, any | 1 595 |
| 12 | עַל | ʿal | on, over, with | 1 527 |
| 13 | בֵּן | bēn | son; (pl.) people | 1 496 |
| 14 | אָמַר | ʾāmar | say | 1 373 |
| 15 | לֹא | lōʾ | not | 1 340 |
| 16 | הָיָה | hāyâ | be, become, happen | 1 049 |
| 17 | כִּי | kî | that, because, when | 1 040 |
| 18 | עָשָׂה | ʿāśâ | make, do | 860 |
| 19 | אֶרֶץ | ʾereṣ | land, earth | 849 |
| 20 | אֱלֹהִים | ʾĕlōhîm | God, gods | 812 |
| 21 | יוֹם | yôm | day | 668 |
| 22 | מֹשֶׁה | mōšeh | Moses | 647 |
| 23 | נָתַן | nāṯan | give, put | 645 |
| 24 | פָּנִים (lemma פָּנֶה) | pānîm | face; before | 627 |
| 25 | בּוֹא | bôʾ | come, go in | 617 |
| 26 | יִשְׂרָאֵל | yiśrāʾēl | Israel | 587 |
| 27 | כְּ | kə- | like, according to | 571 |
| 28 | אִישׁ | ʾîš | man, each | 570 |
| 29 | ה (directional) | -â | toward | 460 |
| 30 | הוּא (word) | hûʾ | he, it, that | 433 |
| 31 | דִּבֶּר | dibber | speak | 415 |
| 32 | עַם | ʿam | people | 415 |
| 33 | אָב | ʾāḇ | father | 413 |
| 34 | אֶחָד | ʾeḥāḏ | one | 400 |
| 35 | רָאָה | rāʾâ | see | 396 |
| 36 | לָקַח | lāqaḥ | take | 393 |
| 37 | יָד | yāḏ | hand | 382 |
| 38 | שָׁנָה | šānâ | year | 371 |
| 39 | מִצְרַיִם | miṣrayim | Egypt | 357 |
| 40 | יָצָא | yāṣāʾ | go out | 347 |
| 41 | זֶה | zeh | this | 337 |
| 42 | שְׁנַיִם | šənayim | two | 329 |
| 43 | אָכַל | ʾāḵal | eat | 328 |
| 44 | אִם | ʾim | if | 327 |
| 45 | בַּיִת | bayiṯ | house | 324 |
| 46 | הָלַךְ | hālaḵ | go, walk | 312 |
| 47 | עַד | ʿaḏ | until, as far as | 309 |
| 48 | מוּת | mûṯ | die | 307 |
| 49 | אִשָּׁה | ʾiššâ | woman, wife | 307 |
| 50 | אֵת | ʾēṯ | with | 302 |

(From `torah-en.tsv`, sorted by count; glosses from the `glosses` column, which lists MACULA's three commonest English glosses with their counts. Next come אַהֲרֹן 301, כֹּהֵן "priest" 295, אָח "brother" 285, אֵלֶּה 264, שָׁם 259, דָּבָר 256, אוֹ 256, צִוָּה "command" 252, שֵׁם "name" 252.)

The list is a portrait of the Torah: speech (אָמַר, דִּבֶּר, צִוָּה), giving and taking, coming and going out, family (בֵּן, אָב, אָח, אִשָּׁה, אִישׁ), the land, Egypt, Moses and Aaron, the priest, and the day and year of a calendar-minded law.

### 4.3 Hapaxes

Many Torah hapaxes are technical (cultic, botanical, zoological) or foreign: גֹּפֶר "gopher (wood)" (GEN.6.14), אַבְרֵךְ (GEN.41.43, an Egyptian cry, §7), גֹּמֶא "papyrus" (EXO.2.3), the unclean birds of LEV 11 and DEU 14 whose species are guesses. A hapax is not necessarily rare in the language — it may be a common word that the Torah has no occasion to use — but for a learner and for the dictionary it is a word with a single context, and its gloss can only come from tradition, the ancient versions and comparison.

---

## 5. Semantic fields

The lists give the main Torah words of each field with their counts (dictionary, 2026-10-17) and their usual sense. The fields follow the traditional thematic dictionaries in outline; MACULA's own semantic domains, taken from the Semantic Dictionary of Biblical Hebrew (SDBH), are **not** carried in our trees because of their licence (tools/macula/README.md). Azar, Pahmer and Waxman 2020 build a Biblical Hebrew thesaurus from BDB, Klein and distributional similarity, which is the computational counterpart of such lists.

### 5.1 God and the divine names

| word | translit. | sense | count |
|---|---|---|---:|
| יהוה | YHWH | the proper name of God, read *ʾăḏōnāy* | 1 813 (+7 pointed יֱהֹוִה, read *ʾĕlōhîm*) |
| אֱלֹהִים | ʾĕlōhîm | God; gods (plural form, singular agreement for Israel's God) | 812 |
| אֵל | ʾēl | god, God; in names | 48 |
| אֲדֹנָי | ʾăḏōnāy | (my) Lord | 18 |
| שַׁדַּי | šadday | "Almighty" (traditional), in אֵל שַׁדַּי | 9 |
| עֶלְיוֹן | ʿelyôn | Most High (GEN 14; NUM 24; DEU 32) | 3 |
| יָהּ | yāh | short form of the name (EXO.15.2; 17.16) | 2 |
| אֱלוֹהַּ | ʾĕlôah | God (poetic; DEU.32.15, 17) | 2 |

The name YHWH is pointed in the Masoretic text with the vowels of *ʾăḏōnāy* (as יְהוָה) or, after אֲדֹנָי, with those of *ʾĕlōhîm* (as יֱהֹוִה), so that the reader says the substitute (GKC §17c, §102m; BDB s.v. יהוה). MACULA lemmatises both and glosses "Yahweh"; the JPS 1917 renders "the LORD". The plural form אֱלֹהִים with singular verbs is the plural of majesty or abstraction (GKC §124g; §132h); with plural agreement it is "gods" (51 glosses).

### 5.2 Kinship and society

אָב "father" 413, אֵם "mother" 63, בֵּן "son" 1 496 (with בְּנֵי יִשְׂרָאֵל "the children/people of Israel" accounting for much of it), בַּת "daughter" 204, אָח "brother, kinsman" 285, אָחוֹת "sister" 47, אִישׁ "man, husband, each" 570, אִשָּׁה "woman, wife" 307, חָתָן "son-in-law, bridegroom" 5, כַּלָּה "daughter-in-law, bride" 6, עֶבֶד "servant, slave" 180, אָדוֹן "lord, master" 91, עַם "people, kin" 415, גּוֹי "nation" 89, עֵדָה "congregation" 110, קָהָל "assembly" 34, מַטֶּה / שֵׁבֶט "tribe" 142 / 32, נָשִׂיא "chief" 71, זָקֵן "elder" 54, גֵּר "sojourner, resident alien", מִשְׁפָּחָה "clan". The kin terms are wider than English: אָח covers a nephew (Lot, GEN.13.8 "we are brothers") and a fellow Israelite (DEU 15).

### 5.3 The body

יָד "hand; power; side" 382, עַיִן "eye; spring" 231, פָּנִים "face; presence" 627, רֹאשׁ "head; top; chief" 144, פֶּה "mouth; edge; command" 124, בָּשָׂר "flesh, body" 138, דָּם "blood" 166, קֶרֶב "inward part; midst" 99, לֵב / לֵבָב "heart, mind" 67 / 55, רֶגֶל "foot" 60, אַף "nose; anger" 51, אֹזֶן "ear" 32, כַּף "palm; pan" 58. Body words are the source of many prepositions (לִפְנֵי, בְּיַד, בְּקֶרֶב, עַל־פִּי) and of the vocabulary of emotion: אַף "nose" → "anger" (the burning nose: וַיִּחַר אַף, "and anger burned"), לֵב "heart" = the seat of thought rather than of feeling (BDB s.v. לֵב).

### 5.4 The cult and sacrifice

| word | translit. | sense | count |
|---|---|---|---:|
| מִזְבֵּחַ | mizbēaḥ | altar | 198 |
| קָרַב / קׇרְבָּן | qāraḇ / qorbān | come near, (Hiphil) present / offering | 194 / 78 |
| כֹּהֵן | kōhēn | priest | 295 |
| קֹדֶשׁ / קָדוֹשׁ / קִדֵּשׁ / מִקְדָּשׁ | qōḏeš … | holiness / holy / consecrate / sanctuary | 223 / 36 / 75 / 16 |
| עֹלָה | ʿōlâ | burnt offering ("what goes up") | 148 |
| חַטָּאת | ḥaṭṭāʾṯ | sin offering; sin | 141 |
| מִנְחָה | minḥâ | gift; grain offering | 113 |
| מִשְׁכָּן | miškān | tabernacle | 104 |
| זֶבַח / זָבַח | zeḇaḥ / zāḇaḥ | sacrifice / slaughter | 72 / 41 |
| כִּפֶּר | kipper | make atonement | 78 |
| תְּרוּמָה | tərûmâ | contribution, levy | 44 |
| אֲרוֹן | ʾărôn | ark (chest) | 42 |
| נֶסֶךְ | nesek | drink offering | 41 |
| אֹהֶל מוֹעֵד | ʾōhel môʿēḏ | tent of meeting | אֹהֶל 214, מוֹעֵד 160 |

The field is dense in Leviticus and Numbers, and its words are polysemous in a regular way: חַטָּאת is both the sin and the offering for it, עֹלָה is named from עָלָה "go up". Cult vocabulary is also where the dictionary's English glosses ("a.sin.offering", "the.contribution") show MACULA's translation choices most clearly.

### 5.5 Purity

טָמֵא "be unclean" (verb 114, adjective 66), טֻמְאָה "uncleanness" 20, טָהֵר "be clean" 54, טָהוֹר "clean, pure" 69, צָרַעַת "skin disease" (traditionally "leprosy") 30, נִדָּה "impurity (menstrual)", זוֹב "discharge", כִּבֶּס "wash (clothes)", רָחַץ "bathe". The pair טָמֵא / טָהוֹר is a closed binary of ritual status, not of hygiene; the English "unclean / clean" of JPS keeps it.

### 5.6 Land and agriculture

אֶרֶץ "land, earth" 849, אֲדָמָה "ground, soil" 96, שָׂדֶה "field, open country" 116, זֶרַע "seed; offspring" 112, פְּרִי "fruit" 42, עֵץ "tree; wood" 108, לֶחֶם "bread, food" 84, דָּגָן "grain" 11, תִּירוֹשׁ "new wine" 10, יִצְהָר "fresh oil" 7, גֶּפֶן "vine" 8, קָצִיר "harvest", מִדְבָּר "wilderness, steppe" 105, הַר "mountain", נָהָר "river" 18, יְאֹר "the Nile" 31. The triad דָּגָן, תִּירוֹשׁ, יִצְהָר "grain, wine and oil" is a fixed Deuteronomic formula (DEU.7.13, 11.14 …). זֶרַע shows the regular metaphor seed → descendants: MACULA glosses it "offspring" 68 times.

### 5.7 Animals

| domestic | count | wild and other | count |
|---|---:|---|---:|
| בְּהֵמָה livestock, beast | 103 | חַיָּה living creature, wild animal | (in חַיִּים 50) |
| צֹאן flock (sheep and goats) | 116 | עוֹף birds, flying things | 35 |
| אַיִל ram | 117 | נָחָשׁ snake | 14 |
| בָּקָר herd (cattle) | 98 | שֶׁרֶץ swarming thing | |
| פַּר young bull | 92 | the unclean birds of LEV 11 / DEU 14 | hapaxes |
| כֶּבֶשׂ lamb | 87 | | |
| עֵז she-goat, goat | 56 | | |
| שׁוֹר ox | 52 | | |
| חֲמוֹר donkey | 41 | | |
| שֶׂה one of the flock | 28 | | |
| גָּמָל camel | 28 | | |
| סוּס horse | 12 | | |

The sacrificial animals dominate. The distinction between collectives (צֹאן, בָּקָר) and individuals (שֶׂה, שׁוֹר) matters for agreement and counting (GKC §122r–s).

### 5.8 Time

יוֹם "day" 668, שָׁנָה "year" 371, חֹדֶשׁ "month; new moon" 86, בֹּקֶר "morning" 80, עֶרֶב "evening" 77, לַיְלָה "night" 62, שַׁבָּת "sabbath" 47, מוֹעֵד "appointed time; festival" 160, עוֹלָם "long duration, perpetuity" 73, עֵת "time" 36, מָחָר "tomorrow" 17. Legnaioli 2016 analyses the two words for "month", חֹדֶשׁ and the rare יֶרַח, with the methods of structural lexical semantics and finds them interchangeable variants. עוֹלָם is "remote time" in either direction (מֵעוֹלָם "of old", GEN.6.4; לְעוֹלָם "for ever", GEN.3.22), not "world", which is its Mishnaic sense (see [11-mishnaic.md](11-mishnaic.md)).

### 5.9 Speech

אָמַר "say" 1 373, דִּבֶּר "speak" 415, קָרָא "call, name, read aloud" 192, צִוָּה "command" 252, שָׁמַע "hear, obey" 241, עָנָה "answer" 43, הִגִּיד "tell" 55, בֵּרַךְ "bless" 136, שָׁאַל "ask". Speech verbs introduce direct speech, usually with לֵאמֹר "saying" (see [09-discourse.md](09-discourse.md)), and our tree rules make a clause right after a verb of saying its `ccomp` (tools/macula). שָׁמַע covers "hear", "listen", "obey" and "understand" (GEN.11.7), a span English splits.

### 5.10 Motion

בּוֹא "come, go in" 617, יָצָא "go out" 347, הָלַךְ "go, walk" 312, שׁוּב "return" 180, עָלָה "go up" 201, יָרַד "go down" 88, נָסַע "set out, journey" 120, חָנָה "encamp" 86, עָבַר "pass over" 129, קָרַב "come near" 194, נָגַשׁ "draw near" 45. The pairs עָלָה / יָרַד are tied to the landscape and to ideology: one "goes up" to the land and to the altar and "goes down" to Egypt (GEN.12.10; 46.3–4).

### 5.11 Law and covenant

| word | sense | count |
|---|---|---:|
| מִשְׁפָּט | judgment, ordinance, custom | 84 |
| חֹק / חֻקָּה | statute | 47 / 56 |
| מִצְוָה | commandment | 63 |
| תּוֹרָה | instruction, law | 56 |
| בְּרִית | covenant | 82 |
| עֵדוּת | testimony | 38 |
| מִשְׁמֶרֶת | charge, duty | 39 |
| שָׁמַר | keep, guard | 148 |
| צֶדֶק / צְדָקָה / צַדִּיק | righteousness / right / righteous | 12 / 9 / 17 |

Deuteronomy strings these words together (DEU.6.1 הַמִּצְוָה הַחֻקִּים וְהַמִּשְׁפָּטִים). Vergari 2021 follows מִצְוָה from "a command (of a superior)" in Classical Biblical Hebrew through Late Biblical innovations to the Mishnaic "religious duty, meritorious deed", a clear case of semantic change across the stages of the language.

### 5.12 Numbers and measures

The cardinals (`Type=cardinal_number` 2 253) and ordinals (235) are among the commonest words of Numbers (censuses, offerings): אֶחָד 400, שְׁנַיִם 329, שֶׁבַע 229, חָמֵשׁ 211, מֵאָה 194, שָׁלֹשׁ 183, אַרְבַּע 159, אֶלֶף 118 (the counts are of lemmas and include tens such as עֶשְׂרִים 73). Their syntax (gender polarity of 3–10, the construct, compound numbers as `flat` in our trees) is in [04-noun.md](04-noun.md). Measures: אַמָּה "cubit" 73, שֶׁקֶל "shekel" 63, אֵיפָה "ephah" 9, עֹמֶר "omer", הִין "hin", גֵּרָה "gerah"; metals כֶּסֶף "silver, money" 146, זָהָב "gold" 143, נְחֹשֶׁת "bronze, copper" 48.

---

## 6. Polysemy and key words

The glosses MACULA gives each token show how far a word's meaning stretches inside one corpus. The distributions below count the English gloss of every Torah token after stripping articles and possessives (2026-10-17).

### 6.1 נֶפֶשׁ *nep̄eš* (205)

person 44, self 34, life 33, selves 13, anyone 12, being 11, creature 10, **corpse** 10, persons 9, appetite 4 … BDB's senses run "soul, living being, life, self, person, desire, appetite, emotion, passion". The word is the living person as a whole, not a soul separate from the body: the man "became a living *nep̄eš*" (GEN.2.7), the animals are נֶפֶשׁ חַיָּה (GEN.2.19), "the *nep̄eš* of the flesh is in the blood" (LEV.17.11), and — at the far end — נֶפֶשׁ מֵת "a dead person", a corpse (NUM.6.6).

> **LEV.17.11** · `wlc:LEV:500`
> כִּי נֶפֶשׁ הַבָּשָׂר בַּדָּם הִוא
> *kî nep̄eš habbāśār baddām hîʾ* — for the-life-of the-flesh in-the-blood it — "For the life of the flesh is in the blood." (JPS 1917)

> **NUM.6.6** · `wlc:NUM:224`
> כָּל־יְמֵי הַזִּירוֹ לַיהוָה עַל־נֶפֶשׁ מֵת לֹא יָבֹא
> *kol-yəmê hazzîrô laYHWH ʿal-nep̄eš mēṯ lōʾ yāḇōʾ* — all-days-of his-separating to-YHWH, upon-person dead not he-shall-come — "All the days that he consecrateth himself unto the LORD he shall not come near to a dead body." (JPS 1917)

### 6.2 רוּחַ *rûaḥ* (38)

spirit 24, wind 7, breath 5. The same word is the wind that dries the flood (GEN.8.1), the breath of life, and the spirit of God (GEN.1.2; GEN.41.38). BDB: "breath, wind, spirit".

> **GEN.8.1** · `wlc:GEN:184`
> וַיַּעֲבֵר אֱלֹהִים רוּחַ עַל־הָאָרֶץ וַיָּשֹׁכּוּ הַמָּיִם
> *wayyaʿăḇēr ʾĕlōhîm rûaḥ ʿal-hāʾāreṣ wayyāšōkkû hammāyim* — and-made-pass God wind over-the-earth and-subsided the-waters — "and God made a wind to pass over the earth, and the waters assuaged." (JPS 1917)

### 6.3 דָּבָר *dāḇār* (256)

words 84, word 47, thing 38, matter 37, things 15, anything 7, manner 3 … "Word" and "thing" are one lexeme: הַדְּבָרִים הָאֵלֶּה "these things / these words" (GEN.15.1; 22.1) is ambiguous by design, and הֲיִפָּלֵא מֵיְהוָה דָּבָר "is any *thing* too hard for the LORD?" (GEN.18.14). Compound prepositions grammaticalise it: עַל־דְּבַר "because of" (GEN.20.11).

### 6.4 בַּיִת *bayiṯ* (324)

house 185, household 61, houses 23, inside 11 … The building, the household and the dynasty: בֹּא־אַתָּה וְכָל־בֵּיתְךָ אֶל־הַתֵּבָה "come thou and all thy house into the ark" (GEN.7.1); מִבַּיִת וּמִחוּץ "within and without" (GEN.6.14), where בַּיִת is simply "inside".

### 6.5 יָד *yāḏ* (382)

hand 310, hands 55, side 3 … Beyond the hand: power (הַיָּד הַגְּדֹלָה "the great hand", rendered by JPS "the great work", EXO.14.31), side (עַל־יַד "beside"), agency (בְּיַד "by means of"), share or portion (GEN.43.34 "five times so much", lit. "five hands").

> **EXO.14.31** · `wlc:EXO:387`
> וַיַּרְא יִשְׂרָאֵל אֶת־הַיָּד הַגְּדֹלָה אֲשֶׁר עָשָׂה יְהוָה בְּמִצְרַיִם
> *wayyarʾ yiśrāʾēl ʾeṯ-hayyāḏ haggəḏōlâ ʾăšer ʿāśâ YHWH bəmiṣrayim* — and-saw Israel OM the-hand the-great which did YHWH in-Egypt — "And Israel saw the great work which the LORD did upon the Egyptians." (JPS 1917)

### 6.6 פָּנִים *pānîm* (627)

(no gloss) 335, face 119, before 83, surface 29, faces 18, presence 17 … More than half the tokens have no gloss of their own because the word is part of a compound preposition (לִפְנֵי, מִפְּנֵי, עַל־פְּנֵי). The word is always plural in form (פָּנִים); MACULA's lemma is the singular פָּנֶה, a lexicographic convention (Strong 6440). The span face → presence → before → because of is a textbook path of grammaticalisation (Hardy 2022; Staps 2022 for the threatening sense of לִפְנֵי).

### 6.7 חֶסֶד *ḥeseḏ* (20)

loyalty 8, covenant loyalty 6 … The word for loyal kindness within a relationship (GEN.24.27 חַסְדּוֹ וַאֲמִתּוֹ "his kindness and his faithfulness"; EXO.20.6 עֹשֶׂה חֶסֶד לַאֲלָפִים "showing mercy unto the thousandth generation"). JPS renders "mercy" or "kindness"; MACULA's "covenant loyalty" reflects later scholarship. A homonym חֶסֶד II "shame, disgrace" occurs once (LEV.20.17); Strong's number does not separate them, and the dictionary's OSHB column picks the wrong one (§10.4).

### 6.8 שָׁלוֹם *šālôm* (26)

peace 16, welfare 6, health 3 … Wholeness and well-being more than the absence of war: הֲשָׁלוֹם לוֹ וַיֹּאמְרוּ שָׁלוֹם "Is it well with him? And they said: It is well" (GEN.29.6).

> **GEN.29.6** · `wlc:GEN:801`
> וַיֹּאמֶר לָהֶם הֲשָׁלוֹם לוֹ וַיֹּאמְרוּ שָׁלוֹם
> *wayyōʾmer lāhem hăšālôm lô wayyōʾmərû šālôm* — and-he-said to-them: ?-peace to-him; and-they-said: peace — "And he said unto them: 'Is it well with him?' And they said: 'It is well.'" (JPS 1917)

### 6.9 Other key words

בֵּן "son" (1 496) is glossed "the people" 283 times, because בְּנֵי יִשְׂרָאֵל is rendered "the people of Israel"; לֵב "heart" (67) is the mind (DEU.29.3 "a heart to know"); רַע (72) spans "evil", "bad" and "displeasing"; טוֹב (79) spans "good", "pleasant" and "well". Such words are where an English gloss alone misleads, and where the dictionary's examples layer (§10.2) earns its place.

---

## 7. Loanwords

The Torah's vocabulary is overwhelmingly Northwest Semitic. Its loanwords cluster where the story touches foreign lands: Egypt (the Joseph and Exodus narratives) and Mesopotamia (the flood). Notarius 2025 shows that Akkadian loans are very few in the archaic poems, and unevenly spread, which supports the conservative character of that layer. Persian loans belong to the post-exilic books (פַּרְדֵּס "park", דָּת "law", אֲחַשְׁדַּרְפְּנִים "satraps"), Greek and Latin ones to Mishnaic Hebrew; the Torah has none of either.

| word | translit. | meaning | source (as BDB gives it) | Torah |
|---|---|---|---|---:|
| יְאֹר | yəʾōr | the Nile; a river, canal | Egyptian *ʾitrw* "river" (BDB03752) | 31 |
| תֵּבָה | tēḇâ | ark (Noah's; Moses' basket) | Egyptian *ṯbt* "chest, coffin" (BDB10710) | 28 |
| פַּרְעֹה | parʿōh | Pharaoh | Egyptian *pr-ʿ3* "great house" (BDB08097) | 216 |
| חַרְטֹם | ḥarṭōm | magician, soothsayer-priest | Egyptian; BDB notes it is always applied to Egypt (BDB03457) | 9 |
| אַבְרֵךְ | ʾaḇrēḵ | a cry before Joseph (GEN.41.43) | Egyptian or Coptic, disputed (BDB00094) | 1 |
| שֵׁשׁ | šēš | fine linen, byssus | Egyptian, probably (BDB, שֵׁשׁ⁵) | 34 |
| מֹשֶׁה | mōšeh | Moses | Egyptian *ms(w)* "child, son" (BDB05902); the text gives a Hebrew pun (EXO.2.10) | 647 |
| אָחוּ, גֹּמֶא, סוּף | ʾāḥû, gōmeʾ, sûp̄ | reeds, papyrus, rushes | Egyptian (the Nile vocabulary) | 2, 1, (place name) |
| כֹּפֶר | kōp̄er | pitch (GEN.6.14) | compared with Akkadian *kupru* (BDB s.v. כֹּפֶר) | 1 |
| סָרִיס | sārîs | court official, eunuch | Akkadian *ša rēši* "he of the head" (BDB06886) | 4 |

The Joseph story also gives Egyptian names: צָפְנַת פַּעְנֵחַ *ṣāp̄naṯ paʿnēaḥ* (Joseph's Egyptian name), אָסְנַת *ʾāsnaṯ*, פּוֹטִיפַר / פּוֹטִי פֶרַע *pôṭîp̄ar / pôṭî p̄eraʿ* (GEN.41.45). The flood words (תֵּבָה, כֹּפֶר) and the Egyptian court words together show that the narrators wrote with the local colour of their settings, an argument used in dating debates (Rezetko and Young 2014 survey the methodological problems of such arguments).

> **GEN.41.43** · `wlc:GEN:1238`
> וַיַּרְכֵּב אֹתוֹ בְּמִרְכֶּבֶת הַמִּשְׁנֶה אֲשֶׁר־לוֹ וַיִּקְרְאוּ לְפָנָיו אַבְרֵךְ
> *wayyarkēḇ ʾōṯô bəmirkeḇeṯ hammišneh ʾăšer-lô wayyiqrəʾû ləp̄ānāyw ʾaḇrēḵ* — and-he-made-ride him in-the-chariot-of the-second which-to-him, and-they-cried before-him: Abrech — "And he made him to ride in the second chariot which he had; and they cried before him: 'Abrech.'" (JPS 1917)

---

## 8. Proper names

### 8.1 Frequency

Proper nouns are **8 004** tokens of 664 distinct names in the Torah (UPOS `PROPN`, 2026-10-17). The commonest: יהוה 1 820 (as PROPN, including the seven יֱהֹוִה), מֹשֶׁה 647, יִשְׂרָאֵל 587, מִצְרַיִם 357, אַהֲרֹן 301, פַּרְעֹה 216, יַעֲקֹב 212, יוֹסֵף 175, אַבְרָהָם 151 (+ אַבְרָם 59), יִצְחָק 98, עֵשָׂו 82, כְּנַעַן 65, לָבָן 55, בִּלְעָם 55, יַרְדֵּן 52, יְהוּדָה 49, מוֹאָב 47, רָחֵל 44, אֶלְעָזָר 42, נֹחַ 41, בָּלָק 40, רְאוּבֵן 39, שָׂרָה 37, מְנַשֶּׁה 36.

The genealogies and the census and camp lists of Numbers contribute most of the 664 types; many are hapaxes.

### 8.2 Theophoric names

Names often contain a divine element, as subject or object of a verb or noun:

- **־אֵל / אֱלִי־** "God": 66 distinct names in the Torah, among them יִשְׂרָאֵל, יִשְׁמָעֵאל "God hears" (17), אֶלְעָזָר "God has helped" (42), אֱלִיאָב (10), בְּתוּאֵל (9), רְעוּאֵל (8), בְּצַלְאֵל "in the shadow of God" (6), נְתַנְאֵל "God has given" (5), גַּמְלִיאֵל (5), מִיכָאֵל "who is like God?" (1), and the place name בֵּית־אֵל "house of God" (26).
- **יְהוֹ־ / יוֹ־ / ־יָה** "YHWH": rare in the Torah, which places the revelation of the name at the burning bush (EXO.6.2–3). Before that point there are none; afterwards יוֹכֶבֶד *yôḵeḇeḏ*, Moses' mother (EXO.6.20; NUM.26.59), and יְהוֹשֻׁעַ *yəhôšuaʿ* (27), whom Moses renames from הוֹשֵׁעַ (NUM.13.16). יְהוּדָה *yəhûḏâ* is not a theophoric name in the usual analysis (the text explains it from הוֹדָה "praise", GEN.29.35).

> **NUM.13.16** · `wlc:NUM:486`
> וַיִּקְרָא מֹשֶׁה לְהוֹשֵׁעַ בִּן־נוּן יְהוֹשֻׁעַ
> *wayyiqrāʾ mōšeh ləhôšēaʿ bin-nûn yəhôšuaʿ* — and-called Moses to-Hoshea son-of Nun: Joshua — "And Moses called Hoshea the son of Nun Joshua." (JPS 1917)

### 8.3 Name explanations

The Torah explains names by sound, not by etymology in the modern sense: אַבְרָהָם "father of a multitude" from אַב־הֲמוֹן (GEN.17.5), יִשְׂרָאֵל from שָׂרִיתָ עִם־אֱלֹהִים "thou hast striven with God" (GEN.32.29 Heb.), מֹשֶׁה from מָשָׁה "draw out" (EXO.2.10), and the sons of Jacob in GEN 29–30 each from a phrase of their mother. These explanations are part of the text's meaning and are what the English translations keep in their notes; for a model they are a lesson in why "the same letters" is not "the same word".

> **GEN.17.5** · `wlc:GEN:402`
> וְלֹא־יִקָּרֵא עוֹד אֶת־שִׁמְךָ אַבְרָם וְהָיָה שִׁמְךָ אַבְרָהָם כִּי אַב־הֲמוֹן גּוֹיִם נְתַתִּיךָ
> *wəlōʾ-yiqqārēʾ ʿôḏ ʾeṯ-šimḵā ʾaḇrām wəhāyâ šimḵā ʾaḇrāhām kî ʾaḇ-hămôn gôyim nəṯattîḵā* — and-not shall-be-called still your-name Abram, and-shall-be your-name Abraham, for father-of multitude-of nations I-have-made-you — "Neither shall thy name any more be called Abram, but thy name shall be Abraham; for the father of a multitude of nations have I made thee." (JPS 1917)

### 8.4 Gentilics and place names

Peoples are named by the nisbe ־ִי (§3.4): הַכְּנַעֲנִי "the Canaanite", הָאֱמֹרִי "the Amorite", הַחִתִּי, הַחִוִּי, הַיְבוּסִי — the list of the seven nations (DEU.7.1). Places are often descriptive: בְּאֵר שֶׁבַע "well of seven / of the oath" (GEN.21.31), קָדֵשׁ בַּרְנֵעַ, מִצְרַיִם (a dual in form).

---

## 9. The lexicons

### 9.1 Brown–Driver–Briggs (1906)

*A Hebrew and English Lexicon of the Old Testament*, based on Gesenius, by F. Brown, S. R. Driver and C. A. Briggs (Oxford 1906), is the standard English lexicon of the older generation and is in the public domain. It is ordered **by root**: a noun is found under its verbal root, with cross-references from its alphabetical place. Each entry gives cognates (Phoenician, Aramaic, Arabic, Assyrian, Ethiopic), the senses in bold, the forms with statistics, and a full list of references for rare words. Our copy is Sefaria's digital text (`BDB_Dictionary.jsonl`, 11 015 entries, public domain) and the Internet Archive scan. Its weaknesses for a machine: homonyms are numbered by superscripts and Roman numerals (שֵׁשׁ¹–⁵, דבר I/II) that a key must disambiguate; abbreviations are dense ("Hiph. Pi. Niph." in the senses); and its etymological judgements are of 1906.

### 9.2 Gesenius–Tregelles (1857)

W. Gesenius' *Hebrew and Chaldee Lexicon to the Old Testament Scriptures*, translated by S. P. Tregelles (London 1857), is the predecessor of BDB, ordered alphabetically, with Latin-flavoured definitions and many comparative notes. The scan and OCR are held (public domain); the OCR of the Hebrew is poor, so it serves for reading, not for building data.

### 9.3 Strong, the OSHB lexicon and the Lexical Index

Strong's *Exhaustive Concordance* (1890) numbered the Hebrew vocabulary 1–8674, and its short definitions are public domain. The Open Scriptures Hebrew Lexicon (CC BY 4.0) digitises BDB and Strong's dictionary as XML and adds a **Lexical Index** that links each lexeme (with Strong's and BDB's ids) to a short English definition. Our dictionary uses the Lexical Index for its `oshb_def` column.

### 9.4 Jastrow (1903) and the later language

For Mishnaic and Talmudic Hebrew and Aramaic, M. Jastrow's *Dictionary of the Targumim, the Talmud Babli and Yerushalmi, and the Midrashic Literature* (1903) is the public-domain standard (Sefaria text, 32 512 entries; the scans also held). Its entries mark the biblical words reused in the later language with "b. h." and give the later senses with Talmudic references: מִצְוָה there is the religious duty and the good deed (Jastrow s.v. מִצְוָה), the endpoint of the change described by Vergari 2021. J. Levy's *Neuhebräisches und chaldäisches Wörterbuch* (1876–1889, German) is held as scans. See [11-mishnaic.md](11-mishnaic.md).

### 9.5 Modern lexicons (not held)

The modern standard dictionaries — *HALOT* (Koehler–Baumgartner), Clines' *Dictionary of Classical Hebrew*, the *Semantic Dictionary of Biblical Hebrew* (whose domains MACULA carries but we do not), Klein's etymological dictionary (only Sefaria's CC BY-NC text is held, for reference) — are under copyright or non-commercial licences; see [sources.md](sources.md), "wanted".

---

## 10. Mova's Torah dictionary

### 10.1 How it is built

`dictionary hebrew <trees-dir> <BDB_Dictionary.jsonl> <LexicalIndex.xml> <dict.tsv> [<en-pool.tsv>]` (the dictionary builder, a closed tool) reads every word of the trees and writes one row per lexeme to `$MOVA_DATA/corpora/judaism/dict/torah-en.tsv`. The key is the Strong's number; for MACULA's lettered numbers it is `lemma:<lemma>`. Of the BDB entries carrying the number, the one whose headword equals the lemma is taken first.

| column | content | source and licence |
|---|---|---|
| `key` | Strong's number, or `lemma:…` | MACULA (CC BY 4.0) |
| `lemma` | pointed lemma | MACULA |
| `count` | Torah tokens | our count |
| `glosses` | MACULA's three commonest English glosses with counts | MACULA (CC BY 4.0) |
| `oshb_def` | short definition of the Lexical Index | OSHB (CC BY 4.0) |
| `bdb_headword` | the chosen BDB headword | BDB 1906 (public domain) |
| `bdb_short` | BDB's bold senses, markup removed | BDB |
| `bdb_full` | BDB's whole entry as plain text | BDB |

With an English pool (`school-sents usfm`, JPS 1917), it also writes `torah-en.tsv.examples`: for each key, the word's **two shortest one-verse sentences**, in layers:

| column | layer |
|---|---|
| `key`, `verse` | the word and the verse (`GEN.19.36`) |
| `hebrew` | the verse in Hebrew, with accents |
| `literal` | MACULA's word-for-word English glosses |
| `literary` | the JPS 1917 verse |
| `idea` | empty: a commentary layer to come |

Measured on 2026-10-17: 3 486 keys; 3 333 with a BDB entry, 3 451 with an OSHB definition; 5 816 examples; two runs give the same files.

### 10.2 A sample entry (abridged)

The row for נֶפֶשׁ, cut to the first part of each column:

| key | lemma | count | glosses | oshb_def | bdb_short |
|---|---|---:|---|---|---|
| 5315 | נֶפֶשׁ | 205 | person (39); self (28); life (23) | soul | n.f. soul, living being, life, self, person, desire, appetite, emotion passion |

and its examples (shortest one-verse sentences):

| verse | literal (MACULA) | literary (JPS 1917) |
|---|---|---|
| NUM.31.46 | and person[s] of.humankind six- teen thousand | and sixteen thousand persons — |
| DEU.24.6 | not anyone.will.take.as.pledge a.hand-mill and an.upper.millstone (dm) a.life it [is].taking.as.pledge | No man shall take the mill or the upper millstone to pledge; for he taketh a man's life to pledge. |

(The BDB text is public domain and the OSHB and MACULA columns are CC BY 4.0; the selection and layout are ours. Credit: Open Scriptures Hebrew Bible Project; MACULA Hebrew Linguistic Datasets, Biblica / Clear Bible / the Groves Center.)

The layers reproduce the classical way of learning a Hebrew word: the form in its verse, a word-by-word crib, a literary translation, and (to come) a note on its idea.

### 10.3 Frequencies from the dictionary

The `count` column gives the profile of §4: 17 keys ≥ 1 000, 151 ≥ 100, 873 ≥ 10, 1 103 hapaxes, 125 558 tokens in all. The dictionary's count for יהוה (1 813) is lower than the PROPN count (1 820) because the seven tokens pointed יֱהֹוִה have their own Strong's number (3069).

### 10.4 Known gaps

1. **Homonyms under one Strong's number.** The OSHB column and sometimes the BDB column take the wrong homonym, because Strong's number does not separate them and the first entry wins. Observed on 2026-10-17:

   | key | lemma | Torah sense | `oshb_def` given | `bdb` given |
   |---|---|---|---|---|
   | 2617 | חֶסֶד | kindness | shame | (right entry) |
   | 1004 | בַּיִת | house | between | (right entry) |
   | 1254 | בָּרָא | create | be fat | create |
   | 4057 | מִדְבָּר | wilderness | mouth | the "mouth" entry (Song 4.3) |
   | 8336 | שֵׁשׁ | fine linen | alabaster | the numeral "six" |
   | 5869 | עַיִן | eye | spring | eye |
   | 6030 | עָנָה | answer | dwell | answer |
   | 2583 | חָנָה | encamp | decline | — |
   | 5387 | נָשִׂיא | chief | rising mist | — |
   | 5971 | עַם | people | kinsman | a cross-reference only |
   | 2416 | חַיִּים | life | kinsfolk | — |
   | 7218 | רֹאשׁ | head | Rôsh (the name) | — |

   A fix needs a sense key finer than Strong's: OSHB's lexical index ids, BDB's own ids (`rid`), or MACULA's lettered numbers, chosen by the gloss.
2. **Missing entries.** The article ה and the preposition בְּ have no BDB entry of their own (BDB treats them under ה and ב with the letters), and מִן is matched to the neighbouring entry מְנִי (tools/macula/README.md). פָּנֶה's `bdb_short` is a run of stem abbreviations ("Niph. adv.loc. … Hiph.") from a mis-parsed entry.
3. **Verse numbering.** The English pool follows the English numbering of the eBible JPS file, while the Hebrew follows the Masoretic one. Where they differ (GEN 31–32, EXO 7–8 and 21–22, LEV 5–6, NUM 16–17 and 29–30, DEU 12–13, 22–23, 28–29 among others) the `literary` column of an example can hold a neighbouring verse: NUM.17.8 (Heb.) is paired with the English 17.8, which is the Hebrew 17.23. The verse map needs a Hebrew–English versification table.
4. **Words with spaces.** 592 FORM fields in the trees contain a space: multi-word place and personal names that MACULA gives as one word (קָדֵשׁ בַּרְנֵעַ, צָפְנַת פַּעְנֵחַ), and words before a paseq. In GEN.41.45 the two-word name is even given twice, once per source word, with relation `dep` and no Strong's number. Names with spaces split the dictionary's keys and break CoNLL-U validators.
5. **Lemma spelling.** The same name can appear as two lemmas (בֵּית אֵל and בֵּיתאֵל, 13 tokens each), and pointed lemmas can differ in the Unicode order of their marks. Counting by Strong's number, or comparing lemmas by consonants as `macula compare` does, avoids both.
6. **Glosses are contextual.** MACULA's glosses are translations of each token in context ("he.said", "the.house", "[is].good"), not dictionary senses; the `glosses` column therefore mixes inflection and sense. The BDB columns give the senses.
7. **No root column.** The dictionary is keyed by lexeme; a root view (all words of ק־ד־שׁ together) would have to come from BDB's root order or from a morphological analysis.

---

## 11. In Mova

- **Trees.** Each token's lexical identity is in LEMMA (pointed) and MISC `Strong=`; MISC `Gloss=` gives MACULA's contextual English. The UPOS comes from MACULA's word class (`noun` with `type=proper` → PROPN). Word-formation classes that MACULA marks are in MISC `Type=` (`gentilic`, `ordinal_number`, `cardinal_number`, `adjective`). There is no root and no pattern field; MACULA's SDBH semantic domains are deliberately left out.
- **The dictionary** (the dictionary builder, §10) is the Torah lexicon of Mova: lexeme, count, glosses, OSHB and BDB definitions, layered examples. It feeds the Torah course of the school (`macula lessons`) and is the place to add a root column and a sense key.
- **The pragmatics** (`macula prag`) uses the lexicon only through a few lemmas (interrogatives, negatives, נָא); see [05-pronouns-particles.md](05-pronouns-particles.md).
- **Comparison and measurement.** No lexical measure exists yet. Natural ones: dictionary coverage of a new text (how many of its tokens have a key with a BDB sense), the homonym error rate of §10.4 against a hand-checked sample, and gloss agreement with an independent translation.

---

## 12. Sources

- **GKC** — W. Gesenius, E. Kautzsch (ed.), A. E. Cowley (tr.), *Gesenius' Hebrew Grammar*, 2nd English ed., Oxford 1910. Public domain. §§17, 19, 30, 55, 77–86, 93, 102, 122, 124, 132. `$RESEARCH_RAW/hebgram/open/gesenius-kautzsch-cowley-1910-wikisource/gesenius-kautzsch-cowley-1910-wikisource.txt`; https://en.wikisource.org/wiki/Gesenius%27_Hebrew_Grammar.
- **BDB** — F. Brown, S. R. Driver, C. A. Briggs, *A Hebrew and English Lexicon of the Old Testament*, Oxford 1906. Public domain. Entries cited by headword and Sefaria id (BDB00094 אַבְרֵךְ, BDB03457 חַרְטֹם, BDB03752 יְאֹר, BDB05902 מֹשֶׁה, BDB06886 סָרִיס, BDB08097 פַּרְעֹה, BDB10710 תֵּבָה; כֹּפֶר; שֵׁשׁ; נֶפֶשׁ; רוּחַ; דבר I/II; לֵב; יהוה). `$RESEARCH_RAW/lexres/dictionaries/sefaria/BDB_Dictionary.jsonl`; scan and OCR `$RESEARCH_RAW/lexres/dictionaries/internet-archive/bdb-1906/`; https://archive.org/details/hebrewenglishlex00browuoft.
- **Gesenius–Tregelles** — W. Gesenius, *Hebrew and Chaldee Lexicon to the Old Testament Scriptures*, tr. S. P. Tregelles, London 1857. Public domain. `$RESEARCH_RAW/lexres/dictionaries/internet-archive/gesenius-tregelles-1857/`; https://archive.org/details/hebrewchaldeelex00geseuoft.
- **Jastrow** — M. Jastrow, *A Dictionary of the Targumim, the Talmud Babli and Yerushalmi, and the Midrashic Literature*, London/New York 1903. Public domain. s.v. מִצְוָה. `$RESEARCH_RAW/lexres/dictionaries/sefaria/Jastrow_Dictionary.jsonl`; scans `$RESEARCH_RAW/lexres/dictionaries/internet-archive/jastrow-1903/`.
- **Levy** — J. Levy, *Neuhebräisches und chaldäisches Wörterbuch über die Talmudim und Midraschim*, Leipzig 1876–1889. Public domain. `$RESEARCH_RAW/lexres/dictionaries/internet-archive/levy-neuhebr-1876-1889/`.
- **OSHB Hebrew Lexicon** — Open Scriptures, HebrewLexicon (BDB XML, Strong's Hebrew XML, Lexical Index), commit 21c9add, CC BY 4.0 (BDB and Strong's text public domain). `$RESEARCH_RAW/lexres/dictionaries/oshb-hebrewlexicon/`; https://github.com/openscriptures/HebrewLexicon.
- **Davidson 1916** — A. B. Davidson, *An Introductory Hebrew Grammar*, 19th ed., Edinburgh 1916. Public domain. The formation of nouns. `$RESEARCH_RAW/hebgram/pd/davidson-introductory-hebrew-grammar-1916/`.
- **Segal 1927** — M. H. Segal, *A Grammar of Mishnaic Hebrew*, Oxford 1927. PD in the USA only; cited, not quoted. Noun formation in Mishnaic Hebrew. `$RESEARCH_RAW/hebgram/private/segal-grammar-mishnaic-1927/`.
- **Notarius 2025** — T. Notarius, "Loanwords in the Language of Archaic Biblical Hebrew Poetry: A Case for Akkadianisms", *Orientalia Suecana* 74 (2025) 136–148, CC BY 4.0, doi:10.33063/os.v74.782. `$RESEARCH_RAW/hbo-2025-notarius-loanwords-language-archaic-biblical-hebrew/`.
- **Vergari 2021** — R. Vergari, "From polysemy to semantic change: remarks on the lexeme *miṣwà* in Biblical and Mishnaic Hebrew", *Quaderni di Linguistica e Studi Orientali* 7 (2021) 233–262, CC BY 4.0, doi:10.13128/QULSO-2421-7220-12010. `$RESEARCH_RAW/hbo-2021-vergari-polysemy-semantic-change-remarks-lexeme/`.
- **Legnaioli 2016** — A. Legnaioli, "Time in Ancient Hebrew Lexicon: The Case of *yrḥ* and *ḥdš*", *Quaderni di Linguistica e Studi Orientali* 2 (2016) 235–263, CC BY 4.0, doi:10.13128/QULSO-2421-7220-18755. `$RESEARCH_RAW/hbo-2016-legnaioli-time-ancient-hebrew-lexicon-case/`.
- **Azar, Pahmer, Waxman 2020** — "A Thesaurus for Biblical Hebrew", *Proceedings of LT4HALA 2020*, 68–73. `$RESEARCH_RAW/aclbulk-2020-azar-thesaurus-biblical-hebrew/`.
- **Benbaji, Doron, Hénot-Mortier 2022** — "Word-Embeddings Distinguish Denominal and Root-Derived Verbs in Semitic", arXiv, CC BY 4.0. `$RESEARCH_RAW/hbo-2022-benbaji-word-embeddings-distinguish-denominal-root/`.
- **Hardy 2022** — H. H. Hardy II, *The Development of Biblical Hebrew Prepositions*, 2022 (open access). `$RESEARCH_RAW/hbobook-2022-hardy-development-biblical-hebrew-prepositions/book.pdf`.
- **Staps 2022** — C. Staps, "Lip̄nē 'in the face of'", *Journal of Hebrew Scriptures* 22, CC BY 4.0, doi:10.5508/jhs29582. `$RESEARCH_RAW/hbo-2022-staps-lipne-face-locative-preposition-threatening/`.
- **Rezetko and Young 2014** — R. Rezetko, I. Young, *Historical Linguistics and Biblical Hebrew: Steps Toward an Integrated Approach*, SBL Press 2014 (open access). `$RESEARCH_RAW/hbobook-2014-rezetko-historical-linguistics-biblical-hebrew/book.pdf`.
- **Miller-Naudé and Naudé 2016** — "Is the adjective distinct from the noun as a grammatical category in Biblical Hebrew?", *In die Skriflig* 50(4), CC BY 4.0, doi:10.4102/ids.v50i4.2005 (background to the adjective patterns of §3). `$RESEARCH_RAW/hbo-2016-miller-naude-adjective-distinct-noun-grammatical-category/`.
- **Data** — MACULA Hebrew Linguistic Datasets, CC BY 4.0, https://github.com/Clear-Bible/macula-hebrew/ (Biblica, Clear Bible, the Groves Center, OpenScriptures), converted by `tools/macula` into `$MOVA_DATA/corpora/judaism/text/torah-wlc-macula/`; the dictionary `$MOVA_DATA/corpora/judaism/dict/torah-en.tsv` and `.examples`; JPS 1917 (public domain, eBible.org). All counts dated 2026-10-17.
- **Wanted** (not held; under copyright or NC): L. Koehler, W. Baumgartner, *HALOT*; D. J. A. Clines (ed.), *The Dictionary of Classical Hebrew*; R. de Blois, *Semantic Dictionary of Biblical Hebrew* (domains used by MACULA, licence excludes us); T. O. Lambdin, "Egyptian Loan Words in the Old Testament", *JAOS* 73 (1953); Y. Muchiki, *Egyptian Proper Names and Loanwords in North-West Semitic* (1999); P. V. Mankowski, *Akkadian Loanwords in Biblical Hebrew* (2000); E. Klein, *A Comprehensive Etymological Dictionary of the Hebrew Language* (only Sefaria's CC BY-NC text is held, for reference).
