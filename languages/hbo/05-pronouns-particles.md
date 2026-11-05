# Pronouns, particles and prepositions

The small words of Biblical Hebrew carry most of its grammar. A handful of pronouns, one-letter prefixes, a few dozen prepositions, conjunctions, negatives and deictic particles make up more than half of the running words of the Torah: in our trees the conjunction ו alone is 13 768 of 125 807 words, the article 9 109, the suffixed and independent pronouns 13 572, and the object marker 4 147 (counts of 2026-10-17). This chapter describes them form by form, with paradigm tables, Torah counts and examples from our data, and then says how Mova's tools read them. The verb's own suffixes and the noun's suffixed forms are taken up again in [04-noun.md](04-noun.md) and [06-verb.md](06-verb.md); the syntax of clauses built with these particles is in [08-syntax.md](08-syntax.md) and their discourse use in [09-discourse.md](09-discourse.md).

## Contents

1. [Overview and counts](#1-overview-and-counts)
2. [Independent personal pronouns](#2-independent-personal-pronouns)
3. [Pronominal suffixes](#3-pronominal-suffixes)
4. [The object marker אֵת and the preposition אֵת "with"](#4-the-object-marker-אֵת-and-the-preposition-אֵת-with)
5. [The article](#5-the-article)
6. [Demonstratives](#6-demonstratives)
7. [Interrogatives](#7-interrogatives)
8. [The relative: אֲשֶׁר, שֶׁ, זוּ](#8-the-relative-אֲשֶׁר-שֶׁ-זוּ)
9. [Negatives](#9-negatives)
10. [Existence: יֵשׁ and אַיִן](#10-existence-יֵשׁ-and-אַיִן)
11. [Prepositions](#11-prepositions)
12. [Conjunctions](#12-conjunctions)
13. [Adverbs](#13-adverbs)
14. [Interjections and discourse particles](#14-interjections-and-discourse-particles)
15. [In Mova](#15-in-mova)
16. [Sources](#16-sources)

Conventions: transliteration as in [README.md](README.md) (SBL academic); examples give the verse, our sentence id (`wlc:BOOK:n`), the pointed text without accents, a transliteration, a word-for-word gloss and the JPS 1917 translation (public domain). Where the JPS file numbers a verse differently from the Hebrew, the English is our own and marked so.

---

## 1. Overview and counts

The Torah's function words, by lemma, as the MACULA-based trees give them (all five books, 2026-10-17; `UPOS` as in our CoNLL-U):

| lemma | translit. | meaning | UPOS | Torah |
|---|---|---|---|---:|
| וְ | wə- | and, then, but | CCONJ | 13 768 |
| הוּא (suffixes and 3ms) | hûʾ | he; lemma of every suffix pronoun | PRON | 12 586 |
| הַ | ha- | the | DET | 9 109 |
| לְ | lə- | to, for | ADP | 5 727 |
| אֵת (object marker) | ʾēṯ | — | ADP | 4 147 |
| בְּ | bə- | in, with, by | ADP | 3 881 |
| מִן | min | from, than | ADP | 2 005 |
| אֲשֶׁר | ʾăšer | who, which, that | SCONJ | 1 908 |
| אֶל | ʾel | to, toward | ADP | 1 682 |
| עַל | ʿal | on, over, about | ADP | 1 527 |
| לֹא | lōʾ | not | ADV | 1 340 |
| כִּי | kî | that, because, when, indeed | SCONJ | 1 040 |
| כְּ | kə- | like, as | ADP | 571 |
| אִם | ʾim | if, whether | SCONJ | 327 |
| עַד | ʿaḏ | until, as far as | ADP | 309 |
| אֵת ("with") | ʾēṯ | with | ADP | 302 |
| שָׁם | šām | there | ADV | 259 |
| אוֹ | ʾô | or | CCONJ | 256 |
| עִם | ʿim | with | ADP | 243 |
| הִנֵּה | hinnēh | look, here is | INTJ | 230 |
| אַחַר / אַחֲרֵי | ʾaḥar / ʾaḥărê | after, behind | ADP | 196 |
| גַּם | gam | also, even | CCONJ | 177 |
| כֵּן | kēn | so, thus | ADV | 173 |
| בֵּין | bên | between | ADP | 165 |
| תַּחַת | taḥaṯ | under, instead of | ADP | 133 |
| מָה | mâ | what? | PRON | 133 |
| אַיִן / אֵין | ʾayin / ʾên | there is not | ADV | 129 |
| נָא | nāʾ | please, now | INTJ | 110 |
| אַל | ʾal | do not | ADV | 105 |
| הֲ | hă- | interrogative | PART | 96 (+17 as הַ) |
| עוֹד | ʿôḏ | still, again | ADV | 95 |
| עַתָּה | ʿattâ | now | ADV | 84 |
| לְמַעַן | ləmaʿan | so that, for the sake of | SCONJ | 75 |
| מְאֹד | məʾōḏ | very | ADV | 74 |
| פֶּן | pen | lest | SCONJ | 62 |
| מִי | mî | who? | PRON | 55 |
| לָמָּה | lāmmâ | why? | ADP (sic) | 43 |
| אַךְ | ʾaḵ | only, surely | ADV | 43 |
| כֹּה | kōh | thus, here | ADV | 42 |
| רַק | raq | only | ADV | 40 |
| יֵשׁ | yēš | there is | ADV | 30 |
| כְּמוֹ | kəmô | like | ADP | 28 |
| אָז | ʾāz | then | ADV | 21 |
| אוּלַי | ʾûlay | perhaps | ADV | 18 |
| טֶרֶם | ṭerem | not yet, before | ADV | 17 |
| אָמֵן | ʾāmēn | amen, surely | ADV | 14 |

The grammars group these words under pronouns, adverbs, prepositions, conjunctions and interjections (GKC §§32–37, 100–105), and note that most particles are either worn-down nouns (אַחַר "back part" → "after", תַּחַת "the under part" → "under") or old deictic elements (GKC §99). Hardy 2022 traces the grammaticalisation of the prepositions in detail.

---

## 2. Independent personal pronouns

The independent ("separate") pronoun is the subject of a verbless clause or an emphatic subject beside a finite verb, whose own ending already marks the person (GKC §32b; §135a). It never stands as an object: the object is a suffix on the verb or on אֵת (§4).

### 2.1 Paradigm

| person | form | translit. | pausal / variant | Torah tokens |
|---|---|---|---|---:|
| 1 c. sg. | אָנֹכִי | ʾānōḵî | pause אָנֹכִי (ʾānōḵî, stress shifts) | 141 |
| 1 c. sg. | אֲנִי | ʾănî | pause אָנִי (ʾānî) | 181 |
| 2 m. sg. | אַתָּה | ʾattâ | pause אָתָּה (ʾāttâ); rarely written את | 175 |
| 2 f. sg. | אַתְּ | ʾatt | old ketiv אַתִּי (ʾattî) | 7 |
| 3 m. sg. | הוּא | hûʾ | — | 433 (as a word) |
| 3 f. sg. | הִיא | hîʾ | Torah ketiv הִוא (perpetual qere) | 211 |
| 1 c. pl. | אֲנַחְנוּ | ʾănaḥnû | pause אֲנָחְנוּ; short נַחְנוּ (naḥnû) | 28 + 4 |
| 2 m. pl. | אַתֶּם | ʾattem | — | 82 |
| 2 f. pl. | אַתֵּנָה / אַתֵּן | ʾattēnâ / ʾattēn | — | 1 |
| 3 m. pl. | הֵם / הֵמָּה | hēm / hēmmâ | — | 95 + 25 |
| 3 f. pl. | הֵנָּה / הֵן | hēnnâ / hēn | — | 17 |

(Paradigm after GKC §32; counts from the Torah trees, `Type=personal`, 2026-10-17. The 3 m. sg. count is of הוּא as a separate word; the same lemma also heads the 12 092 suffix pronouns, §3.)

### 2.2 אָנֹכִי and אֲנִי

The Torah uses both forms of "I" almost equally (141 : 181). They do not differ in meaning; the choice is partly stylistic and partly a matter of source and period, with אֲנִי gaining ground in later books (GKC §32c; BDB s.v. אָנֹכִי). Both occur in the same speeches:

> **GEN.4.9** · `wlc:GEN:88`
> וַיֹּאמֶר יְהוָה אֶל־קַיִן אֵי הֶבֶל אָחִיךָ וַיֹּאמֶר לֹא יָדַעְתִּי הֲשֹׁמֵר אָחִי אָנֹכִי
> *wayyōʾmer YHWH ʾel-qayin ʾê heḇel ʾāḥîḵā wayyōʾmer lōʾ yāḏaʿtî hăšōmēr ʾāḥî ʾānōḵî* — and-said YHWH to-Cain where Abel your-brother; and-he-said not I-know; ?-keeper-of my-brother I — "And the LORD said unto Cain: 'Where is Abel thy brother?' And he said: 'I know not; am I my brother's keeper?'" (JPS 1917)

> **GEN.27.32** · `wlc:GEN:759`
> וַיֹּאמֶר לוֹ יִצְחָק אָבִיו מִי־אָתָּה וַיֹּאמֶר אֲנִי בִּנְךָ בְכֹרְךָ עֵשָׂו
> *wayyōʾmer lô yiṣḥāq ʾāḇîw mî-ʾāttâ wayyōʾmer ʾănî binḵā ḇəḵōrəḵā ʿēśāw* — and-said to-him Isaac his-father who you; and-he-said I your-son your-firstborn Esau — "And Isaac his father said unto him: 'Who art thou?' And he said: 'I am thy son, thy first-born, Esau.'" (JPS 1917)

Note the pausal אָתָּה (*ʾāttâ*) at the major break of the verse.

The emphatic pronoun beside a finite verb is common in contrastive speech:

> **GEN.28.16** · `wlc:GEN:789`
> וַיִּיקַץ יַעֲקֹב מִשְּׁנָתוֹ וַיֹּאמֶר אָכֵן יֵשׁ יְהוָה בַּמָּקוֹם הַזֶּה וְאָנֹכִי לֹא יָדָעְתִּי
> *wayyîqaṣ yaʿăqōḇ miššənāṯô wayyōʾmer ʾāḵēn yēš YHWH bammāqôm hazzeh wəʾānōḵî lōʾ yāḏāʿtî* — and-awoke Jacob from-his-sleep and-said surely there-is YHWH in-the-place the-this, and-I not I-knew — "Surely the LORD is in this place; and I knew it not." (JPS 1917)

### 2.3 The 3rd feminine singular in the Torah: הִוא

In the consonantal text of the Pentateuch the feminine "she" is mostly written הוא, the same letters as "he"; the Masoretes point it הִוא and read *hîʾ* everywhere (a *qere perpetuum*). GKC §32l argues that this is an orthographic habit of one recension of the Pentateuch, not an old epicene pronoun, since every other Semitic language keeps the genders apart and the Torah itself has הִיא in a few places. Our trees (WLC via MACULA) agree: of 211 feminine tokens, **192** are written הוא and **19** היא (GEN.14.2, 19.20, 20.5, 26.7, 38.25, 40.10; EXO.1.16; LEV.5.11, 11.39, 13.6, 13.10, 13.21, 16.31, 20.17, 20.18, 21.9; NUM.5.13, 5.14; DEU.13.16). GKC counts eleven such places; the difference reflects the Leningrad Codex behind WLC versus the editions Gesenius used. GEN.38.25 has both spellings in one verse:

> **GEN.38.25** · `wlc:GEN:1144`
> הִוא מוּצֵאת וְהִיא שָׁלְחָה אֶל־חָמִיהָ לֵאמֹר לְאִישׁ אֲשֶׁר־אֵלֶּה לּוֹ אָנֹכִי הָרָה
> *hîʾ mûṣēʾṯ wəhîʾ šālḥâ ʾel-ḥāmîhā lēʾmōr ləʾîš ʾăšer-ʾēlleh lô ʾānōḵî hārâ* — she being-brought-out and-she sent to-her-father-in-law saying: by-man who these to-him I pregnant — "When she was brought forth, she sent to her father-in-law, saying: 'By the man, whose these are, am I with child.'" (JPS 1917)

MACULA's lemma is always הִיא and its FEATS `Gender=Fem`, so the spelling difference is visible only in FORM.

### 2.4 Rarer forms

- **נַחְנוּ** *naḥnû* "we": four times in the Torah (EXO.16.7, 16.8; NUM.32.32; GEN.42.11 in pause נָחְנוּ). GKC §32d. In GEN.42.11 the long and short forms stand side by side:

> **GEN.42.11** · `wlc:GEN:1263`
> כֻּלָּנוּ בְּנֵי אִישׁ־אֶחָד נָחְנוּ כֵּנִים אֲנַחְנוּ לֹא־הָיוּ עֲבָדֶיךָ מְרַגְּלִים
> *kullānû bənê ʾîš-ʾeḥāḏ nāḥnû kēnîm ʾănaḥnû lōʾ-hāyû ʿăḇāḏêḵā məraggəlîm* — all-of-us sons-of man-one we; honest we; not were your-servants spies — "We are all one man's sons; we are upright men, thy servants are no spies." (JPS 1917)

- **אַתְּ** as a masculine: Moses to God in NUM.11.15 and the people to Moses in DEU.5.24 (Hebrew numbering; English 5.27), written את and pointed אַתְּ (GKC §32g). MACULA lemmatises both under אַתָּה, so the short spelling shows only in FORM.

> **NUM.11.15** · `wlc:NUM:434`
> וְאִם־כָּכָה אַתְּ־עֹשֶׂה לִּי הָרְגֵנִי נָא הָרֹג
> *wəʾim-kāḵâ ʾatt-ʿōśeh llî horḡēnî nāʾ hārōḡ* — and-if thus you doing to-me, kill-me please killing — "And if Thou deal thus with me, kill me, I pray Thee, out of hand." (JPS 1917)

- **הֵמָּה / הֵנָּה**, the longer plural forms, are common in the Torah (25 and 17) beside הֵם (95). הֵן "they (f.)" as a pronoun is rare; most הֵן tokens are the particle "look" (§14).
- **Mishnaic Hebrew** replaces אֲנַחְנוּ by אָנוּ *ʾānû* (GKC §32d), and uses הֵן / הֵם; אָנֹכִי is gone. See [11-mishnaic.md](11-mishnaic.md).

### 2.5 The pronoun as copula and as demonstrative

The 3rd person pronoun links subject and predicate in an identifying clause ("the pronoun of the third person used as copula", GKC §141g–h):

> **DEU.4.35** · `wlc:DEU:146`
> כִּי יְהוָה הוּא הָאֱלֹהִים אֵין עוֹד מִלְבַדּוֹ
> *kî YHWH hûʾ hāʾĕlōhîm ʾên ʿôḏ millaḇaddô* — that YHWH he the-God; there-is-not more beside-him — "that the LORD, He is God; there is none else beside Him." (JPS 1917)

With the article, הַהוּא / הַהִוא / הָהֵם / הָהֵנָּה means "that, those" (GKC §136): בַּיָּמִים הָהֵם *bayyāmîm hāhēm* "in those days" (GEN.6.4). In our trees these 3rd-person pronouns after a definite noun hang on the noun as `det` (132 tokens with `Type=personal` and relation `det`).

---

## 3. Pronominal suffixes

Possessors of nouns, objects of verbs and complements of prepositions are expressed by suffixes. They come in two sets, depending on whether the base ends in a consonant or in the plural-noun ending *-ê* (GKC §§33, 58, 91, 103).

### 3.1 On nouns

Using סוּס *sûs* "horse" (sg.) and סוּסִים *sûsîm* (pl.):

| person | on a singular noun | translit. | on a plural noun | translit. |
|---|---|---|---|---|
| 1 c. sg. | סוּסִי | sûsî | סוּסַי | sûsay |
| 2 m. sg. | סוּסְךָ | sûsəḵā | סוּסֶיךָ | sûsêḵā |
| 2 f. sg. | סוּסֵךְ | sûsēḵ | סוּסַיִךְ | sûsayiḵ |
| 3 m. sg. | סוּסוֹ | sûsô | סוּסָיו | sûsāyw |
| 3 f. sg. | סוּסָהּ | sûsāh | סוּסֶיהָ | sûsêhā |
| 1 c. pl. | סוּסֵנוּ | sûsēnû | סוּסֵינוּ | sûsênû |
| 2 m. pl. | סוּסְכֶם | sûsəḵem | סוּסֵיכֶם | sûsêḵem |
| 2 f. pl. | סוּסְכֶן | sûsəḵen | סוּסֵיכֶן | sûsêḵen |
| 3 m. pl. | סוּסָם | sûsām | סוּסֵיהֶם | sûsêhem |
| 3 f. pl. | סוּסָן | sûsān | סוּסֵיהֶן | sûsêhen |

(Standard paradigm, GKC §91; Davidson 1916, Introductory Grammar, the noun with suffixes.) The 3 m. sg. ־וֹ is sometimes written ־ה in older spelling (e.g. אָהֳלֹה "his tent", GEN.9.21; GKC §91). Before the "heavy" suffixes ־כֶם ־כֶן ־הֶם ־הֶן the noun takes its construct form (GKC §91). Nouns with suffixes are definite and take no article.

In the Torah, the commonest suffixes on nouns are 3 m. sg. (2 276 tokens with relation `nmod:poss`) and 2 m. sg. (1 503), the second reflecting Deuteronomy's address to "you": אֱלֹהֶיךָ *ʾĕlōhêḵā* "your God" (DEU.6.5 and passim).

### 3.2 On verbs

Object suffixes attach directly to the finite verb, the imperative and the infinitive (GKC §§58–61):

| person | after a consonant-final form | after a vowel-final form | example from the Torah |
|---|---|---|---|
| 1 c. sg. | ־נִי / ־ֵנִי -nî / -ēnî | ־נִי -nî | הִשִּׁיאַנִי *hiššîʾanî* "it deceived me" (GEN.3.13); יַהַרְגֵנִי *yahargēnî* "will kill me" (GEN.4.14) |
| 2 m. sg. | ־ְךָ / ־ֶךָּ -əḵā / -ekkā | ־ךָ -ḵā | צִוִּיתִיךָ *ṣiwwîṯîḵā* "I commanded you" (GEN.3.11); אַרְאֶךָּ *ʾarʾekkā* "I will show you" (GEN.12.1) |
| 2 f. sg. | ־ֵךְ -ēḵ | ־ךְ -ḵ | — |
| 3 m. sg. | ־הוּ / ־וֹ / ־ֵהוּ -hû / -ô / -ēhû | ־הוּ / ־ו | וַיִּמְצָאֵהוּ *wayyimṣāʾēhû* "and he found him" (GEN.37.15) |
| 3 f. sg. | ־ָהּ / ־ֶהָ -āh / -ehā | ־הָ -hā | — |
| 1 c. pl. | ־נוּ / ־ֵנוּ -nû / -ēnû | ־נוּ | — |
| 2 m. pl. | ־כֶם -ḵem | ־כֶם | — |
| 3 m. pl. | ־ָם / ־ֵם -ām / -ēm | ־ם | מַשְׁחִיתָם *mašḥîṯām* "destroying them" (GEN.6.13) |
| 3 f. pl. | ־ָן / ־ֵן -ān / -ēn | ־ן | — |

The yiqtol (imperfect) can take the "energic" nun before the suffix, giving ־ֶנּוּ *-ennû* "him", ־ֶנָּה *-ennâ* "her", ־ֶךָּ *-ekkā* "you" (GKC §58i). Verb suffixes compete with אֵת + suffix (§4.3): the Priestly prose of the Torah prefers אֹתוֹ, older narrative the suffix (GKC §117e, citing GEN.6.22 against 7.5).

> **GEN.26.27** · `wlc:GEN:719`
> וַיֹּאמֶר אֲלֵהֶם יִצְחָק מַדּוּעַ בָּאתֶם אֵלָי וְאַתֶּם שְׂנֵאתֶם אֹתִי וַתְּשַׁלְּחוּנִי מֵאִתְּכֶם
> *wayyōʾmer ʾălēhem yiṣḥāq maddûaʿ bāʾṯem ʾēlāy wəʾattem śənēṯem ʾōṯî wattəšalləḥûnî mēʾittəḵem* — and-said to-them Isaac why you-came to-me, and-you you-hated me and-you-sent-away-me from-with-you — "Wherefore are ye come unto me, seeing ye hate me, and have sent me away from you?" (JPS 1917)

One verse shows four kinds of pronoun: the independent אַתֶּם, the suffix on a preposition (אֵלָי), the object on אֵת (אֹתִי) and on the verb (־נִי), and אֵת "with" + suffix (מֵאִתְּכֶם).

### 3.3 On prepositions: the paradigms in the Torah

Prepositions take suffixes either directly (לְ, בְּ, עִם, אֵת) or through the plural-noun base *-ê-* (אֶל, עַל, אַחֲרֵי, תַּחַת, עַד), because the latter go back to plural or construct nominal forms (GKC §103a, n–o). The table gives the forms found in our trees (the commonest pointed spelling, with Torah counts; a dash means not attested in the Torah; 2026-10-17).

**Direct type: לְ, בְּ, כְּמוֹ, עִם**

| person | לְ "to" | | בְּ "in" | | כְּמוֹ "like" | | עִם "with" | |
|---|---|---:|---|---:|---|---:|---|---:|
| 1 c. sg. | לִי lî | 136 | בִּי bî | 11 | כָּמֹנִי kāmōnî | 3 | עִמִּי ʿimmî (+ עִמָּדִי ʿimmāḏî 19) | 10 |
| 2 m. sg. | לְךָ ləḵā (pause לָךְ lāḵ) | 193 | בְּךָ bəḵā | 15 | כָּמוֹךָ kāmôḵā | 5 | עִמְּךָ ʿimməḵā | 12 |
| 2 f. sg. | לָךְ lāḵ | 86\* | בָּךְ bāḵ | 7\* | — | | עִמָּךְ ʿimmāḵ | 43\* |
| 3 m. sg. | לוֹ lô | 311 | בּוֹ bô | 91 | כָּמֹהוּ kāmōhû | 7 | עִמּוֹ ʿimmô | 44 |
| 3 f. sg. | לָהּ lāh | 44 | בָּהּ bāh | 46 | כָּמוֹהָ kāmôhā | 1 | עִמָּהּ ʿimmāh | 10 |
| 1 c. pl. | לָנוּ lānû | 50 | בָּנוּ bānû | 7 | כָּמֹנוּ kāmōnû | 1 | עִמָּנוּ ʿimmānû | 14 |
| 2 m. pl. | לָכֶם lāḵem | 234 | בָּכֶם bāḵem | 14 | — | | עִמָּכֶם ʿimmāḵem | 22 |
| 3 m. pl. | לָהֶם lāhem | 166 | בָּם / בָּהֶם bām / bāhem | 28 | — | | עִמָּם ʿimmām | 4 |
| 3 f. pl. | לָהֶן lāhen | 6 | בָּהֵן bāhēn | 9 | — | | — | |

\* The form לָךְ / בָּךְ / עִמָּךְ is both the 2 f. sg. and the pausal 2 m. sg. (GKC §103g). MACULA labels most of them `Gender=Fem`, but many are pausal masculines (e.g. GEN.15.1 אָנֹכִי מָגֵן לָךְ, addressed to Abram), so these counts mix both.

**The ʾēt-type: אֵת (object marker) and אֵת "with"** — the object marker takes the base אֹת- / אֶתְ-, the preposition the base אִתּ-:

| person | object marker | | | preposition "with" | |
|---|---|---|---:|---|---|
| 1 c. sg. | אֹתִי | ʾōṯî | 31 | אִתִּי | ʾittî |
| 2 m. sg. | אֹתְךָ | ʾōṯəḵā | | אִתְּךָ | ʾittəḵā (16) |
| 2 f. sg. | אֹתָךְ | ʾōṯāḵ | | אִתָּךְ | ʾittāḵ (18) |
| 3 m. sg. | אֹתוֹ | ʾōṯô | 254 | אִתּוֹ | ʾittô |
| 3 f. sg. | אֹתָהּ | ʾōṯāh | 94 | אִתָּהּ | ʾittāh |
| 1 c. pl. | אֹתָנוּ | ʾōṯānû | 16 | אִתָּנוּ | ʾittānû |
| 2 m. pl. | אֶתְכֶם | ʾeṯḵem | 120 | אִתְּכֶם | ʾittəḵem |
| 3 m. pl. | אֹתָם | ʾōṯām | 184 | אִתָּם | ʾittām |
| 3 f. pl. | אֶתְהֶן / אֹתָן | ʾeṯhen / ʾōṯān | 6 | אִתָּן | ʾittān |

(MACULA keeps both as lemma אֵת but distinguishes them by Strong's number: 853 object marker, 854 "with". The counts in the table's first numeric column are of the object marker as our trees give it; the 2 m./2 f. sg. counts under "with" are from the 854 lemma. The pointings follow GKC §103b and §117.)

**The plural-base type: אֶל, עַל, אַחֲרֵי, תַּחַת**

| person | אֶל "to" | | עַל "on" | | אַחֲרֵי "after" | | תַּחַת "under" |
|---|---|---:|---|---:|---|---:|---|
| 1 c. sg. | אֵלַי ʾēlay | 60 | עָלַי ʿālay | 17 | אַחֲרַי ʾaḥăray | 4 | תַּחְתַּי taḥtay |
| 2 m. sg. | אֵלֶיךָ ʾēlêḵā | 61 | עָלֶיךָ ʿālêḵā | 36 | אַחֲרֶיךָ ʾaḥărêḵā | 15 | תַּחְתֶּיךָ taḥtêḵā (1) |
| 2 f. sg. | אֵלַיִךְ ʾēlayiḵ | 1 | עָלַיִךְ ʿālayiḵ | | | | |
| 3 m. sg. | אֵלָיו ʾēlāyw | 123 | עָלָיו ʿālāyw | 154 | אַחֲרָיו ʾaḥărāyw | 14 | תַּחְתָּיו taḥtāyw (12) |
| 3 f. sg. | אֵלֶיהָ ʾēlêhā | 20 | עָלֶיהָ ʿālêhā | 54 | אַחֲרֶיהָ ʾaḥărêhā | 1 | תַּחְתֶּיהָ taḥtêhā (2) |
| 1 c. pl. | אֵלֵינוּ ʾēlênû | 12 | עָלֵינוּ ʿālênû | 14 | אַחֲרֵינוּ ʾaḥărênû | 2 | |
| 2 m. pl. | אֲלֵיכֶם ʾălêḵem | 21 | עֲלֵיכֶם ʿălêḵem | 20 | אַחֲרֵיכֶם ʾaḥărêḵem | 5 | |
| 3 m. pl. | אֲלֵהֶם / אֲלֵיהֶם ʾălēhem | 85 | עֲלֵיהֶם ʿălêhem | 33 | אַחֲרֵיהֶם ʾaḥărêhem | 16 | תַּחְתָּם taḥtām (3) |
| 3 f. pl. | אֲלֵיהֶן ʾălêhen | 2 | עֲלֵיהֶן ʿălêhen | 5 | אַחֲרֵיהֶן ʾaḥărêhen | 5 | |

**מִן "from" with suffixes** has a reduplicated base מִמֶּ- in the singular (GKC §103i, m):

| person | form | translit. | Torah |
|---|---|---|---:|
| 1 c. sg. | מִמֶּנִּי | mimmennî | 10 |
| 2 m. sg. | מִמְּךָ | mimməḵā | 15 |
| 2 f. sg. | מִמֵּךְ | mimmēḵ | 1 |
| 3 m. sg. | מִמֶּנּוּ | mimmennû | (ambiguous, below) |
| 3 f. sg. | מִמֶּנָּה | mimmennâ | 24 |
| 1 c. pl. | מִמֶּנּוּ | mimmennû | (ambiguous) |
| 2 m. pl. | מִכֶּם | mikkem | 12 |
| 3 m. pl. | מֵהֶם | mēhem | 25 |

מִמֶּנּוּ is both "from him" and "from us" (GKC §103m). MACULA resolves it in context: 68 tokens are labelled 1 c. pl. and 8 are labelled 3 m. sg. by the first-attested form, but the commoner reading in the Torah is "from it/him", as in GEN.2.17 and GEN.3.3 (לֹא תֹאכְלוּ מִמֶּנּוּ "ye shall not eat of it"), so the labels need checking against the context before they are used; GEN.3.22 כְּאַחַד מִמֶּנּוּ "as one of us" is the 1st plural.

**Compound and rarer prepositions with suffixes** found in the Torah: בֵּינִי / בֵּינֶךָ *bênî / bênəḵā* "between me / you" (GEN.13.8; with the plural base בֵּינוֹתֵינוּ, GEN.26.28), בַּעֲדוֹ *baʿăḏô* "on his behalf", נֶגְדִּי *neḡdî* "before me", לְמַעַנְכֶם *ləmaʿanḵem* "for your sake", בִּלְעָדַי *bilʿāḏay* "apart from me" (GEN.14.24), אֶצְלָהּ *ʾeṣlāh* "beside her" (GEN.39.10).

---

## 4. The object marker אֵת and the preposition אֵת "with"

### 4.1 Form and distribution

The object marker (*nota accusativi*) is written אֵת when it carries its own accent and אֶת־ when joined by maqqef to the next word (GKC §117a). It has no meaning of its own; it marks a **definite** direct object: a proper name, a noun with the article, a noun with a suffix, a construct chain whose last member is definite, or כֹּל "all" in such a chain (GKC §117a–d). An indefinite object takes no אֵת. The marker is a prose feature: it is rare in the old poems (it never occurs in Exodus 15 or Deuteronomy 32, GKC §117b, Rem. 1), and becomes almost obligatory in classical narrative.

In the Torah our trees have **4 147** object markers (Strong 853) against **302** prepositions "with" (Strong 854). MACULA's `Type=direct_object_marker` is on 4 153 tokens: Type and Strong disagree on a few, among them EXO.32.11 (וַיְחַל מֹשֶׁה אֶת־פְּנֵי יְהוָה "Moses besought the LORD") and the pilgrimage formula of EXO.34.23–24 and DEU.16.16 ("appear אֶת־פְּנֵי the Lord"); in the formula the Masoretic pointing of the verb as passive leaves אֵת to be read as "with, before" or as an old object marker.

> **GEN.1.1** · `wlc:GEN:0`
> בְּרֵאשִׁית בָּרָא אֱלֹהִים אֵת הַשָּׁמַיִם וְאֵת הָאָרֶץ
> *bərēʾšîṯ bārāʾ ʾĕlōhîm ʾēṯ haššāmayim wəʾēṯ hāʾāreṣ* — in-beginning created God OM the-heavens and-OM the-earth — "In the beginning God created the heaven and the earth." (JPS 1917)

Each coordinated object repeats its own marker. Compare an indefinite object without אֵת:

> **GEN.37.16** · `wlc:GEN:1099`
> וַיֹּאמֶר אֶת־אַחַי אָנֹכִי מְבַקֵּשׁ הַגִּידָה־נָּא לִי אֵיפֹה הֵם רֹעִים
> *wayyōʾmer ʾeṯ-ʾaḥay ʾānōḵî məḇaqqēš haggîḏâ-nnāʾ lî ʾêp̄ōh hēm rōʿîm* — and-he-said OM my-brothers I seeking; tell please to-me where they pasturing — "And he said: 'I seek my brethren. Tell me, I pray thee, where they are feeding the flock.'" (JPS 1917)

The fronted definite object אֶת־אַחַי keeps its marker; the pronominal "me" after the preposition is a suffix.

### 4.2 Why it matters to a parser

Because Hebrew word order is free enough to put either subject or object first, and because the subject has no marker, אֵת is often the only sign of which noun is the object. In GEN.1.1 the verb and the two nouns would be ambiguous in principle; the marker settles it. Boulet 2019 treats the "accusative" of the grammars in a generative frame and separates true arguments from secondary predicates and adjuncts; Bunis 2025 follows the later reduced forms of the marker (Bar-Kosiba's ת־), which point toward the cliticised Mishnaic and Modern usage.

### 4.3 אֵת with suffixes as the object

The marker carries a pronominal object when a verb suffix is not possible or not wanted: when the object is fronted, when the verb already has a suffix, after an infinitive absolute, and in some styles generally (GKC §117e). Torah counts: אֹתוֹ 254, אֹתָם 184, אֶתְכֶם 120, אֹתָהּ 94 (§3.3).

> **GEN.45.3** · `wlc:GEN:1361`
> וַיֹּאמֶר יוֹסֵף אֶל־אֶחָיו אֲנִי יוֹסֵף הַעוֹד אָבִי חָי וְלֹא־יָכְלוּ אֶחָיו לַעֲנוֹת אֹתוֹ כִּי נִבְהֲלוּ מִפָּנָיו
> *wayyōʾmer yôsēp̄ ʾel-ʾeḥāyw ʾănî yôsēp̄ haʿôḏ ʾāḇî ḥāy wəlōʾ-yāḵlû ʾeḥāyw laʿănôṯ ʾōṯô kî niḇhălû mippānāyw* — and-said Joseph to-his-brothers: I Joseph; ?-still my-father alive? and-not could his-brothers to-answer OM-him for they-were-terrified from-his-face — "I am Joseph; doth my father yet live?" (JPS 1917)

### 4.4 The preposition אֵת "with"

The homonym אֵת "with, beside" takes the suffix base אִתּ- (§3.3) and is often compounded: מֵאֵת *mēʾēṯ* "from (beside)", as מֵאִתְּכֶם in GEN.26.27 above. It is near in meaning to עִם, and BDB gives it a separate entry (BDB 85, אֵת II). In the trees it is `ADP` / `case` like the marker, but with Strong 854 and Gloss "with".

### 4.5 Later developments

In Late Biblical Hebrew and the Mishnah אֵת can mark an emphasised nominative or topic ("as for …"), and Mishnaic Hebrew uses אוֹתוֹ / אוֹתָהּ as a demonstrative "that same" (GKC §117i, m). Modern Hebrew keeps the marker, obligatorily before definite objects (Hacohen 2021 on differential object marking in Modern Hebrew).

---

## 5. The article

### 5.1 Form

The article is a prefix ה with *a* and doubling of the next consonant: הַ + dagesh forte, הַמֶּלֶךְ *hammeleḵ* "the king" (GKC §35). Before gutturals and ר, which do not double, the vowel changes (GKC §35b–k):

| before | article | example | translit. |
|---|---|---|---|
| ordinary consonant | הַ + doubling | הַשָּׁמַיִם | haššāmayim |
| א, ע, ר (no doubling, compensatory lengthening) | הָ | הָאָרֶץ, הָעֵץ, הָרֹאשׁ | hāʾāreṣ, hāʿēṣ, hārōʾš |
| ה, ח (virtual doubling) | הַ | הַחֹשֶׁךְ, הַהוּא | haḥōšeḵ, hahûʾ |
| unstressed חָ, and הָ / עָ | הֶ | הֶחָכָם, הֶהָרִים | heḥāḵām, hehārîm |
| יְ, מְ before a guttural-free shewa | often הַ without dagesh | הַיְאֹר | hayʾōr |

Torah forms of the article in our trees (Strong 1886a): הַ 5 076, הָ 2 232, הֶ 123, and **1 679 empty** forms. The empty ones are the article swallowed after the prepositions בְּ, כְּ, לְ, which take over its vowel: לַמֶּלֶךְ *lammeleḵ* "to the king" for \*לְהַמֶּלֶךְ (GKC §35n). MACULA keeps a word for the hidden article, with an empty FORM, so that the tree still shows `det`; the UD guidelines for Ancient Hebrew ask for the same (the article is a token even when no letter is written; UD hbo tokenization guidelines, paraphrased).

### 5.2 Use

The article marks a definite noun not otherwise determined; names, suffixed nouns and construct heads take none (GKC §§125–127). An attributive adjective or demonstrative agrees in definiteness: הַמָּקוֹם הַזֶּה *hammāqôm hazzeh* "this place", lit. "the-place the-this" (GEN.28.16). The article on a participle often works as a relative: הַנֹּגֵעַ *hannōḡēaʿ* "he who touches" (LEV.11.39; GKC §138i). In the vocative and in exclamations it marks address (GKC §126e).

### 5.3 The interrogative הֲ and the directional ה

Two other prefixed or suffixed ה must not be confused with the article:

- the **interrogative** הֲ (§7.2), pointed with hateph-pataḥ; MACULA has 96 under lemma הֲ and 17 more under lemma הַ (pointed with pataḥ before gutturals, e.g. הַעוֹד in GEN.45.3), all with `Type=interrogative`;
- the **directional** ־ָה "toward", unstressed, on nouns of place: אַרְצָה *ʾarṣâ* "to the ground", שָׁמָּה *šāmmâ* "thither" (GKC §90c). MACULA treats it as a word of its own, lemma ה, `UPOS=X`, `Type=directional_he` (392 tokens), and the paragogic ־ָה on verbs (`paragogic_he`, 73) likewise.

---

## 6. Demonstratives

### 6.1 Forms

| | near ("this, these") | | far ("that, those") | |
|---|---|---|---|---|
| m. sg. | זֶה | zeh | הַהוּא | hahûʾ |
| f. sg. | זֹאת (rare זֹה, זוֹ) | zōʾṯ | הַהִוא / הַהִיא | hahîʾ |
| c. pl. | אֵלֶּה (rare אֵל) | ʾēlleh | הָהֵם / הָהֵמָּה (m.), הָהֵנָּה (f.) | hāhēm, hāhēnnâ |

(GKC §34, §136.) Torah counts from the trees: זֶה 337, אֵלֶּה 264, זֹאת 179, the short plural הָאֵל 8 (all in GEN, LEV, DEU, e.g. GEN.19.8; GKC §34b note).

### 6.2 Uses

- **Attributive**, after a definite noun and itself with the article: הַדְּבָרִים הָאֵלֶּה "these things" (GEN.15.1; GEN.22.1).
- **Predicative / pronominal**, without article, as subject or predicate: זֹאת הַפַּעַם עֶצֶם מֵעֲצָמַי "this one at last is bone of my bones" (GEN.2.23); וְזֶה שַׁעַר הַשָּׁמָיִם "and this is the gate of heaven" (GEN.28.17).
- **Enclitic**, after an interrogative, adding urgency: לָמָּה זֶּה "why ever" (GEN.32.30 Heb.; EXO.2.20), מַה־זֹּאת עָשִׂית "what is this you have done?" (GEN.3.13); also temporal זֶה עֶשְׂרִים שָׁנָה "these twenty years now" (GEN.31.38) (GKC §136c–d).
- **Relative** in poetry: זוּ "which" (§8).

> **GEN.2.23** · `wlc:GEN:53`
> וַיֹּאמֶר הָאָדָם זֹאת הַפַּעַם עֶצֶם מֵעֲצָמַי וּבָשָׂר מִבְּשָׂרִי לְזֹאת יִקָּרֵא אִשָּׁה כִּי מֵאִישׁ לֻקֳחָה־זֹּאת
> *wayyōʾmer hāʾāḏām zōʾṯ happaʿam ʿeṣem mēʿăṣāmay ûḇāśār mibbəśārî ləzōʾṯ yiqqārēʾ ʾiššâ kî mēʾîš luqŏḥâ-zzōʾṯ* — and-said the-man: this the-time bone from-my-bones and-flesh from-my-flesh; to-this shall-be-called woman, for from-man was-taken this — "This is now bone of my bones, and flesh of my flesh; she shall be called Woman, because she was taken out of Man." (JPS 1917)

Three uses of זֹאת in one verse: subject of a verbless clause, object of a preposition, subject of a passive verb.

### 6.3 In the trees

MACULA classes זֶה / זֹאת / אֵלֶּה as **adjectives** (`UPOS=ADJ`, `Type=demonstrative`). After a noun they therefore hang as `amod` (זֶה 196, אֵלֶּה 99, זֹאת 90), while the pronoun-based הַהוּא etc. (class pronoun) are `det` (132). The UD conventions for Hebrew, and UD PTNK, take the attributive demonstrative as `det`; this is the "det → amod 263" difference listed in [tools/macula/README.md](../../tools/macula/README.md). Standing alone, the demonstrative is `nsubj`, `obl` or the root of a verbless clause (אֵלֶּה root 53 times, the headings "these are the generations / the words …").

---

## 7. Interrogatives

### 7.1 Pronouns and adverbs

| word | translit. | meaning | Torah | example |
|---|---|---|---:|---|
| מִי | mî | who? | 55 | מִי אָנֹכִי (EXO.3.11) |
| מָה, מַה־, מֶה | mâ, mah-, meh | what? how! | 133 | מַה־תְּבַקֵּשׁ (GEN.37.15) |
| לָמָּה, לָמָה | lāmmâ | why? (lit. "for what") | 43 | לָמָּה תַעֲמֹד בַּחוּץ (GEN.24.31) |
| מַדּוּעַ | maddûaʿ | why? (lit. "what known") | 10 | GEN.26.27 |
| אֵי, אַיֵּה | ʾê, ʾayyēh | where? | 4 + 4 | אֵי הֶבֶל (GEN.4.9); אַיֵּה הַשֶּׂה (GEN.22.7) |
| אַיּוֹ, אַיֶּכָּה | ʾayyô, ʾayyekkâ | where is he? where are you? | | EXO.2.20; GEN.3.9 |
| אֵיפֹה | ʾêp̄ōh | where? | 1 | GEN.37.16 |
| אָן, אָנָה | ʾān, ʾānâ | whither? | 7 | אָנָה תֵלֵכִי (GEN.16.8) |
| אֵיךְ, אֵיכָה | ʾêḵ, ʾêḵâ | how? | 6 + 5 | GEN.26.9; DEU.1.12 |
| מָתַי, לְמָתַי | māṯay | when? | 5 | EXO.8.5 Heb. |
| מָן | mān | what? (Aramaic-like, in the manna story) | 1 | EXO.16.15 |

(GKC §37 for the pronouns, §100 and §150 for interrogative adverbs and questions; counts from `Type=interrogative` and Strong numbers, 2026-10-17.) מָה takes the doubling of the next consonant: מַה־זֹּאת (GEN.3.13), and turns into an exclamation: מַה־נּוֹרָא הַמָּקוֹם הַזֶּה "how awesome is this place!" (GEN.28.17) (GKC §148).

> **GEN.3.9** · `wlc:GEN:64`
> וַיִּקְרָא יְהוָה אֱלֹהִים אֶל־הָאָדָם וַיֹּאמֶר לוֹ אַיֶּכָּה
> *wayyiqrāʾ YHWH ʾĕlōhîm ʾel-hāʾāḏām wayyōʾmer lô ʾayyekkâ* — and-called YHWH God to-the-man and-said to-him: where-you? — "And the LORD God called unto the man, and said unto him: 'Where art thou?'" (JPS 1917)

> **EXO.16.15** · `wlc:EXO:429`
> וַיֹּאמְרוּ אִישׁ אֶל־אָחִיו מָן הוּא כִּי לֹא יָדְעוּ מַה־הוּא
> *wayyōʾmərû ʾîš ʾel-ʾāḥîw mān hûʾ kî lōʾ yāḏəʿû mah-hûʾ* — and-they-said each to-his-brother: what it? for not they-knew what it — "they said one to another: 'What is it?' — for they knew not what it was." (JPS 1917)

The word מָן here is glossed "what?" by MACULA; the narrator explains it at once with the ordinary מַה־הוּא. BDB (s.v. מָן) relates it to Aramaic *mān* "who/what" and to the name of the manna.

### 7.2 The interrogative הֲ: yes/no questions

A polar question is marked by prefixed הֲ (before gutturals הַ, before a guttural with qameṣ הֶ), or by intonation alone (GKC §100k–n, §150). Disjunctive questions use הֲ … אִם "whether … or": הֲיֵשׁ יְהוָה בְּקִרְבֵּנוּ אִם־אָיִן "Is the LORD among us, or not?" (EXO.17.7). A question with הֲלֹא expects "yes" ("is it not …?"), and הֲ before a negated clause is a rhetorical assertion (GKC §150e).

> **GEN.18.14** · `wlc:GEN:438`
> הֲיִפָּלֵא מֵיְהוָה דָּבָר
> *hăyippālēʾ mêYHWH dāḇār* — ?-is-too-wonderful for-YHWH thing — "Is any thing too hard for the LORD?" (JPS 1917)

> **GEN.17.17** · `wlc:GEN:414`
> הַלְּבֶן מֵאָה־שָׁנָה יִוָּלֵד וְאִם־שָׂרָה הֲבַת־תִּשְׁעִים שָׁנָה תֵּלֵד
> *hallaḇen mēʾâ-šānâ yiwwālēḏ wəʾim-śārâ hăḇaṯ-tišʿîm šānâ tēlēḏ* — ?-to-son-of hundred year shall-be-born, and-if Sarah ?-daughter-of ninety year shall-bear — "Shall a child be born unto him that is a hundred years old? and shall Sarah, that is ninety years old, bear?" (JPS 1917)

Here the question particle is on a preposition (הַלְּ = הֲ + לְ, with doubling) and on a noun.

### 7.3 Indefinite and exclamatory uses

מִי and מָה also make indefinite and wish expressions: מִי יִתֵּן "who will give" = "would that …" (NUM.11.29: וּמִי יִתֵּן כָּל־עַם יְהוָה נְבִיאִים "would that all the LORD's people were prophets"; GKC §151a), and מִי כָמוֹךָ "who is like you?" (DEU.33.29) is praise rather than a question. Meister 1996 studies the range of BH interrogatives and their rhetorical uses.

---

## 8. The relative: אֲשֶׁר, שֶׁ, זוּ

### 8.1 אֲשֶׁר

אֲשֶׁר is an indeclinable relative particle, not a pronoun: it does not inflect and does not itself fill a role in the relative clause (GKC §36, §138). The role is shown, where needed, by a **resumptive** pronoun or suffix inside the clause (GKC §138b–c):

> **EXO.3.5** · `wlc:EXO:51`
> כִּי הַמָּקוֹם אֲשֶׁר אַתָּה עוֹמֵד עָלָיו אַדְמַת־קֹדֶשׁ הוּא
> *kî hammāqôm ʾăšer ʾattâ ʿômēḏ ʿālāyw ʾaḏmaṯ-qōḏeš hûʾ* — for the-place which you standing on-it ground-of holiness it — "for the place whereon thou standest is holy ground." (JPS 1917)

> **GEN.41.38** · `wlc:GEN:1233`
> הֲנִמְצָא כָזֶה אִישׁ אֲשֶׁר רוּחַ אֱלֹהִים בּוֹ
> *hănimṣāʾ ḵāzeh ʾîš ʾăšer rûaḥ ʾĕlōhîm bô* — ?-can-be-found like-this man which spirit-of God in-him — "Can we find such a one as this, a man in whom the spirit of God is?" (JPS 1917)

The resumptive עָלָיו "on it" and בּוֹ "in him" give אֲשֶׁר its role. Object relatives usually leave the resumptive out: הָאָרֶץ אֲשֶׁר אַרְאֶךָּ "the land that I will show you" (GEN.12.1).

Further uses (GKC §138e–f, §157, §158, §165, §166): **headless** "the one who, that which": כָּל־אֲשֶׁר עָשָׂה "all that he had made" (GEN.1.31); **complement** "that" after verbs of knowing and saying (rarer than כִּי); **causal** "because" (יַעַן אֲשֶׁר, אֲשֶׁר alone); **purpose / result** (לְמַעַן אֲשֶׁר); **comparative** כַּאֲשֶׁר "as" (MACULA glosses אֲשֶׁר "as" in such phrases). Torah: 1 908 tokens, every one a `mark` in our trees.

### 8.2 שֶׁ

The short relative שֶׁ (with doubling of the next letter) is northern, colloquial and late in the Bible, and becomes the only relative of Mishnaic Hebrew (GKC §36; Segal 1927, §§ on the relative). In the Torah it may occur once, in the obscure בְּשַׁגַּם of GEN.6.3, which MACULA parses as בְּ + שַׁ + גַּם "in that also" (Strong 7945 not used; MACULA gives `Type=relative`, Strong 7578a):

> **GEN.6.3** · `wlc:GEN:140`
> וַיֹּאמֶר יְהוָה לֹא־יָדוֹן רוּחִי בָאָדָם לְעֹלָם בְּשַׁגַּם הוּא בָשָׂר
> *wayyōʾmer YHWH lōʾ-yāḏôn rûḥî ḇāʾāḏām ləʿōlām bəšaggam hûʾ ḇāśār* — and-said YHWH: not shall-abide my-spirit in-man forever, in-that-also he flesh — "My spirit shall not abide in man for ever, for that he also is flesh." (JPS 1917)

GKC §67p discusses בְּשַׁגָּם as either "in their erring" (from שָׁגַג) or "because also"; the Masoretic pointing allows both. The word is a good test case of how a parse can fix a reading that the text leaves open.

### 8.3 זוּ and the article as relative

Archaic poetry uses the demonstrative זוּ / זֶה as a relative: עַם־זוּ גָּאָלְתָּ "the people which you redeemed" (EXO.15.13; also EXO.15.16) (GKC §138g). The article with a finite verb as a relative is late and rare (GKC §138i–k).

---

## 9. Negatives

| word | translit. | use | Torah |
|---|---|---|---:|
| לֹא | lōʾ | general negation of verbs (indicative), adjectives, nouns in verbless clauses; with yiqtol = prohibition | 1 340 |
| אַל | ʾal | negation of jussive / cohortative: "do not", "let not" | 105 |
| אַיִן / אֵין | ʾayin / ʾên | non-existence "there is not"; negates participles and verbless clauses | 129 |
| בִּלְתִּי | biltî | negates infinitives: לְבִלְתִּי "so as not to" | 30 (lemma בֵּלֶת) |
| טֶרֶם | ṭerem | "not yet", "before" | 17 |
| בְּלִי, בַּל | bəlî, bal | poetic "without", "not" | — / — |
| אֶפֶס | ʾep̄es | "end; nothing but, only" | 6 |

(GKC §152; Davidson 1901, Syntax, the negative sentence, pp. 171–175; counts by Strong number, 2026-10-17.)

**לֹא with yiqtol** is the permanent prohibition of the law; **אַל with the jussive** is the immediate "don't" of speech (GKC §107o, §109c, §152b).

> **EXO.20.13** · `wlc:EXO:531`
> לֹא תִּרְצָח
> *lōʾ tirṣāḥ* — not you-shall-murder — "Thou shalt not murder." (JPS 1917)

> **GEN.15.1** · `wlc:GEN:361`
> אַל־תִּירָא אַבְרָם אָנֹכִי מָגֵן לָךְ שְׂכָרְךָ הַרְבֵּה מְאֹד
> *ʾal-tîrāʾ ʾaḇrām ʾānōḵî māḡēn lāḵ śəḵārəḵā harbēh məʾōḏ* — do-not fear Abram; I shield to-you; your-reward much very — "Fear not, Abram, I am thy shield, thy reward shall be exceeding great." (JPS 1917)

**לְבִלְתִּי + infinitive** negates purpose:

> **GEN.3.11** · `wlc:GEN:66`
> הֲמִן־הָעֵץ אֲשֶׁר צִוִּיתִיךָ לְבִלְתִּי אֲכָל־מִמֶּנּוּ אָכָלְתָּ
> *hămin-hāʿēṣ ʾăšer ṣiwwîṯîḵā ləḇiltî ʾăḵol-mimmennû ʾāḵāltā* — ?-from the-tree which I-commanded-you so-as-not to-eat from-it you-ate — "Hast thou eaten of the tree, whereof I commanded thee that thou shouldest not eat?" (JPS 1917)

MACULA lemmatises בִּלְתִּי under בֵּלֶת (Strong 1115, BDB's base "destruction, nothing"), with `UPOS=ADP`; only one token carries `Type=negative`. A tool that wants negation must therefore look at the lemma and Strong number, not only at the Type.

**טֶרֶם** "not yet" takes yiqtol even of past events (GKC §107c, §152r):

> **GEN.2.5** · `wlc:GEN:35`
> וְכֹל שִׂיחַ הַשָּׂדֶה טֶרֶם יִהְיֶה בָאָרֶץ … וְאָדָם אַיִן לַעֲבֹד אֶת־הָאֲדָמָה
> *wəḵōl śîaḥ haśśāḏeh ṭerem yihyeh ḇāʾāreṣ … wəʾāḏām ʾayin laʿăḇōḏ ʾeṯ-hāʾăḏāmâ* — and-every shrub-of the-field not-yet was in-the-earth … and-man there-was-not to-till OM the-ground — "No shrub of the field was yet in the earth … and there was not a man to till the ground." (JPS 1917)

**Negative words in the Torah, by type** (MACULA `Type=negative`): לֹא 1 340, אַל 104, אַיִן 129 (97 + 32), בְּלִי / בֵּלֶת 3. In the rule-based pragmatics (§15.2), 427 of 5 853 sentences (7.3 %) are read as negative.

**Double and restrictive negation.** כִּי אִם after a negative means "but, except": לֹא יַעֲקֹב יֵאָמֵר עוֹד שִׁמְךָ כִּי אִם־יִשְׂרָאֵל "your name shall no more be called Jacob, but Israel" (GEN.32.29 Heb.); אֵין זֶה כִּי אִם־בֵּית אֱלֹהִים "this is none other than the house of God" (GEN.28.17) (GKC §163). Snyman 2004 analyses BH negation in a minimalist frame.

---

## 10. Existence: יֵשׁ and אַיִן

יֵשׁ "there is" and אַיִן / אֵין "there is not" are originally nouns ("existence", "nothing"), used as the predicate of existential clauses; they take suffixes for the subject (GKC §100o, §141k, §152i–o):

| | יֵשׁ | אַיִן |
|---|---|---|
| absolute | יֵשׁ yēš | אַיִן ʾayin (pause אָיִן) |
| construct / proclitic | יֶשׁ־ yeš- | אֵין ʾên |
| with suffix 3 m. sg. | יֶשְׁנוֹ yešnô | אֵינֶנּוּ ʾênennû "he is not" |
| with suffix 2 m. sg. | יֶשְׁךָ yešḵā | אֵינְךָ ʾênəḵā |
| with suffix 2 m. pl. | יֶשְׁכֶם yeškem | אֵינְכֶם ʾênəḵem |

> **GEN.24.23** · `wlc:GEN:614`
> בַּת־מִי אַתְּ הַגִּידִי נָא לִי הֲיֵשׁ בֵּית־אָבִיךְ מָקוֹם לָנוּ לָלִין
> *baṯ-mî ʾatt haggîḏî nāʾ lî hăyēš bêṯ-ʾāḇîḵ māqôm lānû lālîn* — daughter-of who you? tell please to-me: ?-is-there house-of your-father place for-us to-lodge — "Whose daughter art thou? tell me, I pray thee. Is there room in thy father's house for us to lodge in?" (JPS 1917)

אַיִן also negates participles (אֵינֶנּוּ מֵבִיא "he does not bring") and is the normal negation of the participle in Mishnaic Hebrew, where the participle has become a present tense (Segal 1927; see [11-mishnaic.md](11-mishnaic.md)). In the Torah יֵשׁ is rare (30), אַיִן common (129); MACULA tags several יֵשׁ tokens as `Type=demonstrative` (25), a reminder that its Type field reflects a source classification, not a UD feature.

---

## 11. Prepositions

### 11.1 The inseparable prepositions בְּ, כְּ, לְ

Three prepositions are one-letter prefixes with shewa (GKC §102):

| prefix | meaning | Torah |
|---|---|---:|
| בְּ bə- | in, at, with (instrument), by, against | 3 881 |
| כְּ kə- | like, as, according to, about (numbers) | 571 |
| לְ lə- | to, for, belonging to, toward; marks the infinitive | 5 727 |

Their vowel follows the shewa rules (GKC §102d–i):

- before a consonant with shewa: *i* — לִשְׁמִי *lišmî* "to my name" (GEN.32.30 Heb.), בִּמְקוֹם;
- before a hateph vowel: the matching short vowel — לַעֲבֹד *laʿăḇōḏ* (GEN.2.5), כַּאֲשֶׁר *kaʾăšer*;
- before the article: the article's vowel, the ה drops — בַּשָּׂדֶה *baśśāḏeh*, לַמּוֹעֵד *lammôʿēḏ* (GEN.18.14);
- before a stressed syllable, especially in fixed pairs and with infinitives: qameṣ — לָלִין *lālîn* (GEN.24.23), לָלֶכֶת *lāleḵeṯ* (GEN.24.5), לָנוּ;
- before יְ: contraction to *î* — לִיהוּדָה *lîhûḏâ*; before the divine name יְהוָה read *ʾăḏōnāy*: לַיהוָה *laYHWH* (EXO.15.1), בַּיהוָה (DEU.33.29).

לְ also marks possession in verbless clauses (לִי הוּא "it is mine", GEN.31.43), the beneficiary, the goal, the infinitive of purpose, and the "speech introducer" לֵאמֹר "saying" (with *ē* by contraction from \*ləʾmōr; GKC §23d, §102m).

### 11.2 מִן and its assimilation

מִן "from, out of, (more) than" is a separate word joined by maqqef (מִן־), or a prefix whose *n* assimilates to the next consonant, doubling it (GKC §102b, §19c):

| environment | form | example | Torah forms |
|---|---|---|---:|
| before most consonants | מִ + doubling | מִשְּׁנָתוֹ *miššənāṯô* (GEN.28.16) | מִ 1 092 |
| before gutturals and ר (no doubling, compensatory *ē*) | מֵ | מֵעֲצָמַי *mēʿăṣāmay* (GEN.2.23), מֵאִישׁ | מֵ 529 |
| before the article | usually מִן־ / מֵהָ | מִן־הָעֵץ (GEN.3.11) | מִן־ 248 |
| before suffixes | מִמֶּ- / מִ- / מֵ- | §3.3 | מִמּ 132 |

Uses: separation ("from"), source and material, partitive ("some of": מִפְּרִי הָעֵץ GEN.3.3), cause (מִפְּנֵיהֶם "because of them", GEN.6.13), and **comparison**: עָרוּם מִכֹּל "more subtle than all" (GEN.3.1), since Hebrew has no comparative form (GKC §133a). Compound מֵעַל "from upon", מִתַּחַת "from under", מֵאֵת "from (beside)", מִפְּנֵי "from before, because of".

### 11.3 Separate prepositions

The commonest independent prepositions in the Torah: אֶל "to" 1 682, עַל "on, over, concerning, against" 1 527, עַד "up to, until" 309, אֵת "with" 302, עִם "with" 243, אַחַר / אַחֲרֵי "after, behind" 196, בֵּין "between" 165, תַּחַת "under, instead of" 133, בַּעַד "behind, on behalf of" 20, מוּל "in front of" 19, נֶגֶד "opposite" 17, אֵצֶל "beside", לִפְנֵי (§11.4).

- **אֶל vs עַל.** אֶל is motion "toward", the addressee of speech, and frequent with verbs of saying; עַל is position "on, over", the topic "concerning", hostility "against". The two often interchange in later texts (BDB s.v. אֶל, note 2).
- **בֵּין … וּבֵין** "between … and": בֵּינִי וּבֵינֶיךָ "between me and thee" (GEN.13.8); בֵּין הָאוֹר וּבֵין הַחֹשֶׁךְ (GEN.1.4).
- **תַּחַת** also "instead of": תַּחַת הֶבֶל "instead of Abel" (GEN.4.25).

### 11.4 Compound prepositions built on nouns

Many prepositions are a simple preposition plus the construct of a noun (GKC §101; Hardy 2022):

| compound | literally | meaning | Torah (head noun) |
|---|---|---|---|
| לִפְנֵי | to the face of | before, in front of, in the presence of | פָּנִים 627 tokens in all uses |
| מִפְּנֵי | from the face of | from before; because of | |
| עַל־פְּנֵי | on the face of | on the surface of; in front of | עַל־פְּנֵי תְהוֹם (GEN.1.2) |
| מֵעַל פְּנֵי | from upon the face of | off the face of | GEN.4.14 |
| בְּתוֹךְ | in the middle of | among, within | בְּתוֹךְ־הַגָּן (GEN.3.3) |
| מִתּוֹךְ | from the midst of | out of | |
| בְּקֶרֶב | in the inward part of | among | בְּקִרְבֵּנוּ (EXO.17.7) |
| לְמַעַן | for the purpose of | for the sake of; so that | 75 + 3 |
| לְבַד, מִלְּבַד | alone, apart from | besides | מִלְבַדּוֹ (DEU.4.35) |
| בְּעֵינֵי | in the eyes of | in the opinion of | בְּעֵינֶיךָ (NUM.11.15) |
| בְּיַד | in the hand of | by means of | |
| עַל־דְּבַר | on the matter of | because of | עַל־דְּבַר אִשְׁתִּי (GEN.20.11) |
| בַּעֲבוּר | for the passing of | for the sake of | בַּעֲבוּרֵךְ (GEN.12.13) |
| בִּגְלַל | in the rolling of | on account of | בִּגְלָלֵךְ (GEN.12.13) |

לִפְנֵי can carry a threat, "in the face of", when it marks the side that inflicts defeat (Staps 2022). In our trees these compounds are not single words: the head noun (פָּנֶה, תָּוֶךְ, קֶרֶב …) is `NOUN` with its preposition as `case`, and the object of the compound is the noun's `compound:smixut` (construct) dependent. MACULA's lemma for "face" is the singular פָּנֶה (Strong 6440); its glosses show the grammaticalisation: 335 of 627 tokens have an empty gloss (absorbed into the preposition's gloss), 83 "before", only 68 "face".

---

## 12. Conjunctions

### 12.1 וְ "and"

The conjunction ו is the single commonest word of the Torah (13 768; 10.9 % of all words). It joins words, phrases and clauses, and — as part of the consecutive verb forms — carries the narrative forward (see [06-verb.md](06-verb.md), [09-discourse.md](09-discourse.md)). Its vocalisation (GKC §104d–g) in our trees:

| form | translit. | environment | Torah |
|---|---|---|---:|
| וְ | wə- | default | 6 738 |
| וַ | wa- + doubling | wayyiqtol (consecutive imperfect); before hateph-pataḥ | 4 511 |
| וּ | û- | before a labial (ב מ פ) or a consonant with shewa | 2 111 |
| וָ | wā- | before a stressed syllable, esp. in pairs: תֹהוּ וָבֹהוּ, טוֹב וָרָע | 290 |
| וִ | wî- | before יְ (contracted) | 80 |
| וֶ | we- | before hateph-səḡôl | 21 |
| וֵ | wē- | before א with hateph | 13 |

(Three tokens carry the extraordinary dots of the Masora.) The pair form וָ appears in fixed couplets: נָע וָנָד "a fugitive and a wanderer" (GEN.4.12), טוֹב וָרָע "good and evil" (GEN.2.17), and the wayyiqtol of the 1st singular וָאֹכֵל "and I ate" (GEN.3.13), where א cannot double.

Besides "and", ו means "but", "while", "so that", "then" (introducing an apodosis), "namely", and, with a pronoun or noun, begins a circumstantial clause: וְאָנֹכִי לֹא יָדָעְתִּי "and I — I did not know" (GEN.28.16) (GKC §154; §142d). Zulker 2025 studies *wa-/pa-* apodoses across Semitic.

### 12.2 Subordinating conjunctions

| word | translit. | meanings | Torah |
|---|---|---|---:|
| כִּי | kî | that (complement); because; when, if (in laws); indeed (asseverative); but (after negation) | 1 040 |
| אִם | ʾim | if; whether; or (in questions); oath "surely not" | 327 |
| פֶּן | pen | lest | 62 |
| לְמַעַן | ləmaʿan | in order that; for the sake of | 78 |
| כַּאֲשֶׁר | kaʾăšer | as, when | (in אֲשֶׁר 1 908) |
| יַעַן, יַעַן אֲשֶׁר | yaʿan | because | |
| עֵקֶב | ʿēqeḇ | because (as a reward for) | |
| טֶרֶם, בְּטֶרֶם | ṭerem | before | 17 |
| עַד, עַד־אֲשֶׁר, עַד כִּי | ʿaḏ | until | 309 (incl. preposition) |
| לוּ, לוּלֵא | lû, lûlēʾ | if (unreal); if not | |

כִּי is the most versatile particle of the language; its asseverative "indeed" and its many clause-linking uses are discussed by Benigni 2022 (BH כִּי from a discourse-analysis view) and BDB s.v. כִּי. MACULA glosses most tokens "(dm)" (discourse marker, 865), "(dm).that" (152) and "(cmp)" (complementiser, 20), leaving the choice of meaning open. In legal style כִּי opens the main case and אִם the sub-cases (GKC §159bb):

> **GEN.4.12** · `wlc:GEN:91`
> כִּי תַעֲבֹד אֶת־הָאֲדָמָה לֹא־תֹסֵף תֵּת־כֹּחָהּ לָךְ
> *kî ṯaʿăḇōḏ ʾeṯ-hāʾăḏāmâ lōʾ-ṯōsēp̄ tēṯ-kōḥāh lāḵ* — when you-till OM the-ground, not it-will-continue to-give its-strength to-you — "When thou tillest the ground, it shall not henceforth yield unto thee her strength." (JPS 1917)

**פֶּן "lest"** takes yiqtol:

> **GEN.3.3** · `wlc:GEN:58`
> לֹא תֹאכְלוּ מִמֶּנּוּ וְלֹא תִגְּעוּ בּוֹ פֶּן־תְּמֻתוּן
> *lōʾ ṯōʾḵəlû mimmennû wəlōʾ ṯiggəʿû bô pen-təmuṯûn* — not you-shall-eat from-it and-not you-shall-touch on-it lest you-die — "Ye shall not eat of it, neither shall ye touch it, lest ye die." (JPS 1917)

The paragogic nun of תְּמֻתוּן (MACULA `Type=paragogic_nun`, 105 in the Torah) is a feature of older, emphatic prose (GKC §47m).

**לְמַעַן "so that"**:

> **GEN.12.13** · `wlc:GEN:311`
> אִמְרִי־נָא אֲחֹתִי אָתְּ לְמַעַן יִיטַב־לִי בַעֲבוּרֵךְ וְחָיְתָה נַפְשִׁי בִּגְלָלֵךְ
> *ʾimrî-nāʾ ʾăḥōṯî ʾātt ləmaʿan yîṭaḇ-lî ḇaʿăḇûrēḵ wəḥāyəṯâ nap̄šî biḡlālēḵ* — say please: my-sister you, so-that it-may-go-well to-me for-your-sake and-shall-live my-soul because-of-you — "Say, I pray thee, thou art my sister; that it may be well with me for thy sake, and that my soul may live because of thee." (JPS 1917)

### 12.3 Coordinating and focusing particles

- **אוֹ** "or" (256), common in the laws of Leviticus and Numbers.
- **גַּם** "also, even; both … and" (177): גַּם מִתְּמוֹל גַּם מִשִּׁלְשֹׁם "neither yesterday nor the day before" (EXO.4.10).
- **אַף** "also, even, how much more" (22): אַף כִּי־אָמַר אֱלֹהִים "Has God really said …?" (GEN.3.1), אַף־אֲנִי "I too" (GEN.40.16).
- **אַךְ** "only; surely" (43), **רַק** "only" (40): רַק אֵין־יִרְאַת אֱלֹהִים "surely the fear of God is not …" (GEN.20.11).
- **כִּי אִם** "but, except" (§9).

MACULA classes גַּם, אַף, אַךְ, רַק as `Type=affirmation` (269 tokens).

---

## 13. Adverbs

| word | translit. | meaning | Torah |
|---|---|---|---:|
| שָׁם, שָׁמָּה | šām, šāmmâ | there, thither | 259 |
| כֵּן | kēn | so, thus; right, honest | 173 |
| עוֹד | ʿôḏ | still, yet, again, more | 95 |
| עַתָּה, וְעַתָּה | ʿattâ | now; and now (= therefore) | 84 |
| מְאֹד | məʾōḏ | very, much (orig. noun "might") | 74 |
| סָבִיב | sāḇîḇ | around | 73 |
| כֹּה | kōh | thus; here | 42 |
| תָּמִיד | tāmîḏ | continually | 35 |
| אָז | ʾāz | then | 21 |
| אוּלַי | ʾûlay | perhaps | 18 |
| מָחָר | māḥār | tomorrow | 17 |
| הֲלֹם | hălōm | hither | EXO.3.5 |
| יַחְדָּו | yaḥdāw | together | 16 |

(GKC §100.) Several are frozen nouns (מְאֹד "might", סָבִיב "circuit", תָּמִיד "continuance") or nouns with the old adverbial ending ־ָם (יוֹמָם "by day", חִנָּם "for nothing", GKC §100g). The directional ־ָה makes adverbs of place: שָׁמָּה, הֵנָּה "hither" (GKC §90).

- **עַל־כֵּן** "therefore", introducing an aetiology:

> **GEN.2.24** · `wlc:GEN:54`
> עַל־כֵּן יַעֲזָב־אִישׁ אֶת־אָבִיו וְאֶת־אִמּוֹ וְדָבַק בְּאִשְׁתּוֹ וְהָיוּ לְבָשָׂר אֶחָד
> *ʿal-kēn yaʿăzoḇ-ʾîš ʾeṯ-ʾāḇîw wəʾeṯ-ʾimmô wəḏāḇaq bəʾištô wəhāyû ləḇāśār ʾeḥāḏ* — on-so leaves man OM his-father and-OM his-mother and-cleaves to-his-wife, and-they-become to-flesh one — "Therefore shall a man leave his father and his mother, and shall cleave unto his wife, and they shall be one flesh." (JPS 1917)

- **וַיְהִי־כֵן** "and it was so" closes each act of creation (GEN.1.7, 1.9, 1.11 …).
- **וְעַתָּה** "and now" turns from the grounds to the consequence or the request (GEN.3.22; GEN.11.6) and is a discourse marker as much as an adverb.
- **אָז + yiqtol** for a past event, an archaic use (GKC §107c):

> **EXO.15.1** · `wlc:EXO:388`
> אָז יָשִׁיר־מֹשֶׁה וּבְנֵי יִשְׂרָאֵל אֶת־הַשִּׁירָה הַזֹּאת לַיהוָה
> *ʾāz yāšîr-mōšeh ûḇənê yiśrāʾēl ʾeṯ-haššîrâ hazzōʾṯ laYHWH* — then sang Moses and-sons-of Israel OM the-song the-this to-YHWH — "Then sang Moses and the children of Israel this song unto the LORD." (JPS 1917)

- **אוּלַי** "perhaps", often with a fear or a hope:

> **GEN.18.24** · `wlc:GEN:448`
> אוּלַי יֵשׁ חֲמִשִּׁים צַדִּיקִם בְּתוֹךְ הָעִיר הַאַף תִּסְפֶּה
> *ʾûlay yēš ḥămiššîm ṣaddîqim bəṯôḵ hāʿîr haʾap̄ tispeh* — perhaps there-are fifty righteous in-midst-of the-city; ?-even you-will-sweep-away — "Peradventure there are fifty righteous within the city; wilt Thou indeed sweep away …?" (JPS 1917)

MACULA classes the negatives and some deictics as adverbs; in our trees adverbs are `advmod` unless MACULA's group makes them a clause's predicate.

---

## 14. Interjections and discourse particles

### 14.1 הִנֵּה and הֵן

הִנֵּה "look! here is …" (230 in the Torah) and its short form הֵן (28) point the hearer to something present or newly perceived (GKC §100o, §105b; §147b). With suffixes הִנֵּה is the answer of readiness: הִנֵּנִי *hinnēnî* "here I am" (GEN.22.1; pausal הִנֶּנִּי GEN.22.7). In narrative, וְהִנֵּה shifts to a character's point of view ("and behold — it was very good", GEN.1.31; "and behold, he was wandering in the field", GEN.37.15); its discourse function is treated in [09-discourse.md](09-discourse.md).

| person | הִנֵּה + suffix | translit. |
|---|---|---|
| 1 c. sg. | הִנְנִי, הִנֵּנִי | hinnənî, hinnēnî |
| 2 m. sg. | הִנְּךָ | hinnəḵā |
| 3 m. sg. | הִנּוֹ | hinnô |
| 1 c. pl. | הִנְנוּ, הִנֶּנּוּ | hinnənû, hinnennû |
| 2 m. pl. | הִנְּכֶם | hinnəḵem |
| 3 m. pl. | הִנָּם | hinnām |

(GKC §100o, §147b.)

> **GEN.6.13** · `wlc:GEN:150`
> קֵץ כָּל־בָּשָׂר בָּא לְפָנַי כִּי־מָלְאָה הָאָרֶץ חָמָס מִפְּנֵיהֶם וְהִנְנִי מַשְׁחִיתָם אֶת־הָאָרֶץ
> *qēṣ kol-bāśār bāʾ ləp̄ānay kî-māləʾâ hāʾāreṣ ḥāmās mippənêhem wəhinənî mašḥîṯām ʾeṯ-hāʾāreṣ* — end-of all-flesh has-come before-me, for is-full the-earth violence because-of-them; and-behold-I destroying-them with the-earth — "The end of all flesh is come before Me; for the earth is filled with violence through them; and, behold, I will destroy them with the earth." (JPS 1917)

הִנֵּה + participle announces an imminent act ("I am about to destroy"). Note also אֶת־הָאָרֶץ here: with the earth, or the object marker (GKC §117 hesitates); MACULA reads the marker.

MACULA labels הִנֵּה variously (`Type=demonstrative` 226, `interjection` 4), and our trees attach it as `conj` (137), `ccomp` (44), `root` (26), `nsubj` (19): the generic head rules have no special place for a presentative. PTNK and the UD guidelines use `discourse` or `advmod` for such particles; a dedicated rule is an open task (§15.3).

### 14.2 נָא

נָא (110 in the Torah) follows imperatives, jussives, cohortatives and some particles (אַל־נָא, הִנֵּה־נָא, אִם־נָא). It is usually rendered "please, I pray" and taken as a mark of politeness or of a logically consequent request (GKC §105b; Warren-Rothlin 2007 on politeness strategies in BH). Some of the force lies in the social relation, not in the particle, so a "please" in English sometimes over-translates it.

> **GEN.13.8** · `wlc:GEN:326`
> אַל־נָא תְהִי מְרִיבָה בֵּינִי וּבֵינֶיךָ וּבֵין רֹעַי וּבֵין רֹעֶיךָ כִּי־אֲנָשִׁים אַחִים אֲנָחְנוּ
> *ʾal-nāʾ ṯəhî mərîḇâ bênî ûḇênêḵā ûḇên rōʿay ûḇên rōʿêḵā kî-ʾănāšîm ʾaḥîm ʾănāḥnû* — not-please let-be strife between-me and-between-you and-between my-herdsmen and-between your-herdsmen, for men brothers we — "Let there be no strife, I pray thee, between me and thee … for we are brethren." (JPS 1917)

MACULA: `Type=exhortation` 88 + `interjection` 23 for נָא; our trees make it `advmod` (110 of 110). The `macula prag` rules read it as a **hedge** (softening), §15.2.

### 14.3 Other interjections

| word | translit. | meaning | Torah examples |
|---|---|---|---|
| בִּי | bî | "pardon, my lord" (before addressing a superior) | בִּי אֲדֹנִי (GEN.44.18), בִּי אֲדֹנָי (EXO.4.10) — 5 |
| אָנָּא | ʾānnāʾ | "ah, please!" | EXO.32.31, 2 |
| אוֹי | ʾôy | "woe!" | NUM.21.29, 24.23 |
| הוֹי | hôy | "ah! woe!" | not in the Torah (prophets) |
| הֵא | hēʾ | "here (take)!" (Aramaising) | GEN.47.23 |
| חָלִילָה | ḥālîlâ | "far be it!" (lit. "to profanation") | GEN.18.25 twice, 4 |
| אָמֵן | ʾāmēn | "so be it, surely" | NUM.5.22; DEU.27.15–26, 14 |
| אַשְׁרֵי | ʾašrê | "happy (is)!" (construct pl. noun) | אַשְׁרֶיךָ (DEU.33.29) |

(GKC §105; BDB s.vv.; Andrason, Hornea and Joubert 2020 describe the phonetics, morphology and syntax of BH interjections and how far each departs from the canonical interjection.) בִּי is usually taken as a contracted noun "a request" rather than the preposition with a suffix (BDB 106, בִּי); MACULA lemmatises it בִּי and types it `exhortation`.

> **GEN.44.18** · `wlc:GEN:1342`
> וַיִּגַּשׁ אֵלָיו יְהוּדָה וַיֹּאמֶר בִּי אֲדֹנִי יְדַבֶּר־נָא עַבְדְּךָ דָבָר בְּאָזְנֵי אֲדֹנִי
> *wayyiggaš ʾēlāyw yəhûḏâ wayyōʾmer bî ʾăḏōnî yəḏabber-nāʾ ʿaḇdəḵā ḏāḇār bəʾozənê ʾăḏōnî* — and-drew-near to-him Judah and-said: pardon my-lord, let-speak please your-servant a-word in-ears-of my-lord — "Oh my lord, let thy servant, I pray thee, speak a word in my lord's ears." (JPS 1917)

The verse is a compact lesson in deference: בִּי, the third-person self-reference עַבְדְּךָ "your servant", the jussive with נָא, and the address אֲדֹנִי.

> **NUM.5.22** · `wlc:NUM:209`
> וְאָמְרָה הָאִשָּׁה אָמֵן אָמֵן
> *wəʾāmərâ hāʾiššâ ʾāmēn ʾāmēn* — and-shall-say the-woman: amen amen — "and the woman shall say: 'Amen, Amen.'" (JPS 1917)

---

## 15. In Mova

### 15.1 How the trees hold these words

Our trees ([tools/macula](../../tools/macula/README.md); `$MOVA_DATA/corpora/judaism/text/torah-wlc-macula/*.conllu`) take MACULA's word division: **every prefix and every suffix pronoun is a word of its own**. בְּרֵאשִׁית is two tokens (בְּ `ADP` + רֵאשִׁית `NOUN`), אֵלָיו is two (אֵלָי `ADP` + ו `PRON`), the hidden article after לְ is a token with an empty FORM. This matches the UD tokenisation guidelines for Ancient Hebrew (prepositions, suffixes, ו and ה are split; UD hbo docs) and lets every particle carry its own relation.

What each class becomes:

| class (MACULA) | UPOS | relation (head rules) | notes |
|---|---|---|---|
| preposition `prep`, object marker `om` | ADP | `case` on the noun | 4 147 object markers all `case`; PTNK agrees (case for the marker) |
| article `art` | DET | `det` | empty FORM after בְּ כְּ לְ |
| conjunction `cj` | CCONJ, or SCONJ if in the subordinator list | `cc` on the next conjunct; `mark` for subordinators | ו at a clause's head hangs on that clause's head |
| relative `rel` (אֲשֶׁר, שֶׁ, זוּ) | SCONJ | `mark` | 1 908 / 1 908 |
| suffix pronoun | PRON, lemma הוּא, FEATS Person/Gender/Number | `nmod:poss` 6 571, `obl` 2 806, `obj` 1 975, `nsubj` 385 | the person is in FEATS, not in the lemma |
| independent pronoun | PRON | `nsubj`; `det` after a definite noun (הַהוּא) | |
| demonstrative זֶה / זֹאת / אֵלֶּה | ADJ | `amod` after a noun | UD/PTNK: `det` (open difference) |
| negatives, deictic adverbs | ADV | `advmod` | בִּלְתִּי is ADP under lemma בֵּלֶת |
| interjections `ij` | INTJ | `discourse` where the group allows; otherwise generic | הִנֵּה: conj / ccomp / root |
| particles `ptcl` (הֲ …) | PART | `advmod` | |

The subordinators are the consonantal lemmas listed in `SUBORDINATORS` in `tools/macula/src/main.rs` (כי, אם, למען, פן and others): a `cj` whose lemma is in the list becomes `SCONJ` and `mark`. כִּי is `mark` 1 015 times and `nmod` 25 times (inside groups where MACULA treats it as part of a noun phrase).

**Lemmas are pointed, and their points may come in either Unicode order.** Two tokens of the same word can have lemmas that differ only in the order of combining marks (e.g. dagesh before or after the vowel). `macula compare` therefore compares lemmas by consonants (`consonants()` in the tool). Anyone counting by lemma should do the same, or count by Strong number, as this chapter does.

**MISC carries the source's own classification.** `Type=` holds MACULA's subtype: `direct_object_marker`, `definite_article`, `relative`, `interrogative`, `negative`, `demonstrative`, `personal`, `pronominal`, `exhortation`, `interjection`, `affirmation`, `directional_he`, `paragogic_he`, `paragogic_nun`. `Strong=` separates homonyms (853 object marker vs 854 "with"; MACULA's lettered numbers like 2050b for ו, 1886a for the article, 2050c and others for the suffix pronouns). `Gloss=` gives MACULA's English, with "(et)" for the object marker, "(dm)" for discourse-marker כִּי, "(cmp)" for complementisers.

The UD features of PTNK that our FEATS do not yet carry: `PronType=Prs/Dem/Int/Rel`, `Polarity=Neg`, `Definite` (MACULA has no state or definiteness feature), and `ExtPos` for multiword particles. PTNK also treats לֵאמֹר as a `mark` with `fixed` infinitive; our trees keep לְ as `case` and the infinitive as `advcl` (301 times). These differences are measured in `macula compare` (test UAS 88.38 / LAS 82.23 on 2026-10-17; see [13-mova-tools.md](13-mova-tools.md)).

### 15.2 The rule-based pragmatics and these particles

`macula prag <trees-dir> <out.tsv>` reads every Hebrew sentence into the same six fields as Mova's English pragmatics (form, act, indirect, hedge, polarity, voice), by rules over the particles of the **root's own clause** (`hebrew_prag` in `tools/macula/src/main.rs`):

- **form `wh`** if the root's clause has an interrogative from the list מִי, מָה, מַדּוּעַ, אֵיךְ, מָתַי, לָמָּה, אַיֵּה; **`q`** if it has any other interrogative (הֲ, אֵי, אֵיכָה …); **`imp`** for an imperative, jussive or cohortative root;
- **hedge 1** when נָא is in the root's clause;
- **polarity `neg`** when לֹא, אַל or אַיִן is in the root's clause;
- **voice `narrator`** when the root is a 3rd-person wayyiqtol or qatal: then the whole sentence is read as narration, whatever the speech inside it.

Results on the Torah (`$MOVA_DATA/corpora/judaism/hbo-prag.tsv`, 5 853 sentences, 2026-10-17): form decl 5 591, imp 211, wh 34, q 17; act assert 2 869, narrate 2 010, request 927, ask 47; hedge 17; polarity neg 427; voice narrator 4 149, character 1 704.

Examples of what the rules see:

| sentence | verse | reading | why |
|---|---|---|---|
| `wlc:EXO:531` | EXO.20.13 לֹא תִרְצָח | decl, request, indirect 1, neg, character | 2nd-person yiqtol at the root = a command of the law; לֹא |
| `wlc:GEN:311` | GEN.12.13 אִמְרִי־נָא … | imp, request, hedge 1 | imperative + נָא |
| `wlc:DEU:197` | DEU.6.4 שְׁמַע יִשְׂרָאֵל | imp, request | imperative root |
| `wlc:DEU:11` | DEU.1.12 אֵיכָה אֶשָּׂא לְבַדִּי | q, ask | interrogative, but אֵיכָה is not in the wh list |
| `wlc:GEN:88` | GEN.4.9 "Where is Abel …?" | decl, narrate, narrator | the root is וַיֹּאמֶר; the questions inside are not read |

Known gaps, all visible in the examples:

1. **Speech inside a narrative frame is not read.** Most of the Torah's questions, commands and negations stand inside "and he said …", so they count for the frame, not for themselves. This is why act `ask` is only 47. Splitting the quoted speech (the `ccomp` of a verb of saying, which the UD conventions now give) into its own pragmatic unit is the next step.
2. **The wh list compares pointed lemmas.** לָמָּה in the trees has the lemma לָמָה (no dagesh) and `UPOS=ADP`, so it never matches; אֵיכָה, אֵי, אָן, אָנָה, אֵיפֹה are not in the list and fall to `q`. Comparing by consonants, as `macula compare` does, and extending the list would fix both.
3. **Negation by other words** (בִּלְתִּי, טֶרֶם, בְּלִי, the negative oath with אִם) is not counted.
4. **נָא is the only hedge.** בִּי, אָנָּא and the self-abasing "your servant" are not read as politeness.

Agreement with the English reading of the same verses (`macula prag-compare`, JPS 1917 through the English pipeline; 2026-10-17): form 89 %, voice 66–74 %, polarity 75–82 %, act about 50 %. These are a first baseline, not a measure of correctness: both sides are rule outputs.

### 15.3 Open tasks for these words

- Give הִנֵּה, נָא, בִּי a `discourse` relation by rule, as PTNK does.
- Map MACULA's `Type` into UD features: `PronType` (personal, demonstrative, interrogative, relative), `Polarity=Neg`, `Definite=Def` on nouns with the article or a suffix.
- Treat attributive זֶה / זֹאת / אֵלֶּה as `det` to match the UD Hebrew conventions.
- Resolve מִמֶּנּוּ ("from him" / "from us") and לָךְ (2 f. sg. / pausal 2 m. sg.) from the context, not from the form.
- The Torah dictionary lacks entries of its own for the article and בְּ (see [10-lexicon.md](10-lexicon.md)).

---

## 16. Sources

- **GKC** — W. Gesenius, E. Kautzsch (ed.), A. E. Cowley (tr.), *Gesenius' Hebrew Grammar*, 2nd English ed., Oxford 1910. Public domain. §§7, 19, 23, 32–37, 47, 58–61, 67, 90, 91, 99–105, 107, 109, 117, 125–127, 133, 135–138, 141, 142, 147, 148, 150–152, 154, 157–159, 163, 165, 166. Wikisource text: `$RESEARCH_RAW/hebgram/open/gesenius-kautzsch-cowley-1910-wikisource/gesenius-kautzsch-cowley-1910-wikisource.txt`; https://en.wikisource.org/wiki/Gesenius%27_Hebrew_Grammar.
- **BDB** — F. Brown, S. R. Driver, C. A. Briggs, *A Hebrew and English Lexicon of the Old Testament*, Oxford 1906. Public domain. Entries אֵת I and II (p. 84–86), אֶל, בִּי (p. 106), כִּי, מָן, אָנֹכִי. `$RESEARCH_RAW/lexres/dictionaries/sefaria/BDB_Dictionary.jsonl`; scan `$RESEARCH_RAW/lexres/dictionaries/internet-archive/bdb-1906/`.
- **Davidson 1901** — A. B. Davidson, *Hebrew Syntax*, 3rd ed., Edinburgh 1901. Public domain. Pronouns pp. 1–12; interrogative sentence pp. 164–171; negative sentence pp. 171–175; relative sentence pp. 190–193. `$RESEARCH_RAW/hebgram/pd/davidson-hebrew-syntax-1901/`.
- **Davidson 1916** — A. B. Davidson, *An Introductory Hebrew Grammar*, 19th ed., Edinburgh 1916. Public domain. Noun with suffixes; prepositions with suffixes. `$RESEARCH_RAW/hebgram/pd/davidson-introductory-hebrew-grammar-1916/`.
- **Segal 1927** — M. H. Segal, *A Grammar of Mishnaic Hebrew*, Oxford 1927. PD in the USA only; cited, not quoted. Relative שֶׁ; negation of the participle with אֵין. `$RESEARCH_RAW/hebgram/private/segal-grammar-mishnaic-1927/`.
- **Hardy 2022** — H. H. Hardy II, *The Development of Biblical Hebrew Prepositions*, 2022 (open access). `$RESEARCH_RAW/hbobook-2022-hardy-development-biblical-hebrew-prepositions/book.pdf`.
- **Staps 2022** — C. Staps, "Lip̄nē 'in the face of': A Locative Preposition with a Threatening Connotation", *Journal of Hebrew Scriptures* 22 (2022), CC BY 4.0, doi:10.5508/jhs29582. `$RESEARCH_RAW/hbo-2022-staps-lipne-face-locative-preposition-threatening/`.
- **Staps 2019** — C. Staps, on the grammaticalisation of bipartite prepositions in Ancient Hebrew, *Journal of Semitic Studies* (2019), doi:10.1093/jss/fgz050 (CC BY-NC; cited only). `$RESEARCH_RAW/hbo-2019-staps-prepositions-grammaticalization-ancient-hebrew-bipartite/`.
- **Boulet 2019** — J. E. J. Boulet, *A Linguistic Reappraisal of the Biblical Hebrew Accusative*, PhD thesis, University of Toronto 2019 (read only). `$RESEARCH_RAW/hbo-2019-boulet-linguistic-reappraisal-biblical-hebrew-accusative/`.
- **Bunis 2025** — "The Direct Object Marker /t-/ in Bar-Kosiba Hebrew", *Journal of Semitic Studies* (2025), CC BY, doi:10.1093/jss/fgaf008. `$RESEARCH_RAW/hbo-2025-bunis-direct-object-marker-t-bar/` (metadata only; PDF not yet fetched).
- **Hacohen 2021** — on differential object marking in Modern Hebrew. `$RESEARCH_RAW/lingbulk-2021-hacohen-differential-object-marking-modern-hebrew/`.
- **Benigni 2022** — A. Benigni, "The Biblical Hebrew Particle כִּי from a Discourse Analysis Perspective", *Zeitschrift für Althebraistik* (read only). `$RESEARCH_RAW/hbo-2022-benigni-biblical-hebrew-particle-ky-discourse/`.
- **Meister 1996** — on interrogatives in Biblical Hebrew, Stellenbosch (read only), https://scholar.sun.ac.za/handle/10019.1/55146. `$RESEARCH_RAW/hbo-1996-meister-interrogatives-biblical-hebrew-case-study/`.
- **Snyman 2004** — on negation in BH, minimalist assumptions, *Acta Academica* (read only), doi:10.38140/aa.v0i3.865. `$RESEARCH_RAW/hbo-2004-snyman-chapter-3-minimalist-assumptions-negation/`.
- **Andrason, Hornea, Joubert 2020** — "The Structure of Interjections in Biblical Hebrew: Phonetics, Morphology, and Syntax", *Journal of Hebrew Scriptures* 20, CC BY 2.5, doi:10.5508/jhs29555. `$RESEARCH_RAW/hbo-2020-andrason-structure-interjections-biblical-hebrew-phonetics/`.
- **Warren-Rothlin 2007** — on politeness strategies in Biblical Hebrew and West Semitic. `$RESEARCH_RAW/hbo-2007-warren-rothlin-politeness-strategies-biblical-hebrew-west/`.
- **Zulker 2025** — on Semitic *wa-/pa-* in the apodosis. `$RESEARCH_RAW/hbo-2025-zulker-semitic-wa-pa-apodosis-pragmatics/`.
- **UD Ancient Hebrew guidelines** (CC BY-SA; paraphrased, not copied): tokenisation and relation inventory. `$RESEARCH_RAW/hebgram/sa/ud-docs-he-hbo/_hbo/`; https://universaldependencies.org/hbo/.
- **UD Ancient Hebrew PTNK** (UD 2.18, CC BY-NC; measured against, never copied): `$RESEARCH_RAW/lexres/treebanks/ud-2.18/nc/UD_Ancient_Hebrew-PTNK/`.
- **Data**: MACULA Hebrew Linguistic Datasets (Biblica, Clear Bible, the Groves Center, OpenScriptures), CC BY 4.0, https://github.com/Clear-Bible/macula-hebrew/, as converted by `tools/macula` into `$MOVA_DATA/corpora/judaism/text/torah-wlc-macula/`; JPS 1917 translation (public domain) from eBible.org, `$MOVA_DATA/corpora/judaism/text/torah-jps1917/`; `$MOVA_DATA/corpora/judaism/hbo-prag.tsv`. All counts dated 2026-10-17.
