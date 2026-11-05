# 12. Biblical, Mishnaic and Modern Hebrew compared

Hebrew has been written for about three thousand years, and a reader who knows one stage of it can read the others, but not without being misled. This chapter lines the stages up point by point: Archaic Biblical Hebrew (the old poems), Classical Biblical Hebrew (the prose of the Torah and the Former Prophets), Late Biblical Hebrew (the books of the Persian period), the Hebrew of the Dead Sea Scrolls, Mishnaic Hebrew (chapter 11), Medieval Hebrew and Modern Israeli Hebrew. It ends with what this means for Mova: language codes, treebanks and licences, and why a parser of Modern Hebrew misreads the Bible. Mova's own manual of Modern Hebrew is in [`../he/`](../he/).

## Contents

1. The stages
2. Counting the change: markers book by book
3. Pronouns, relative and possession
4. The verb system
5. Word order and clause types
6. Negation and particles
7. Phonology and reading traditions
8. Script and spelling
9. Vocabulary shifts
10. The linguistic dating debate
11. For Mova: codes, treebanks, licences
12. Why a Modern Hebrew parser misreads the Bible
13. Sources

---

## 1. The stages

| stage | code used in Mova's data | approx. date | main texts |
|---|---|---|---|
| Archaic Biblical Hebrew (ABH) | hbo | before the 10th c. BCE in origin (debated) | Gen 49, Exod 15, Num 23–24, Deut 32–33, Judg 5, Ps 68 |
| Classical / Standard Biblical Hebrew (CBH, SBH) | hbo | monarchy to exile | Genesis–Kings (prose), much of the pre-exilic prophets; inscriptions (Siloam, Lachish, Arad) |
| Late Biblical Hebrew (LBH) | hbo | Persian period, 6th–4th c. BCE | Chronicles, Ezra, Nehemiah, Esther, Daniel (Hebrew parts), Qoheleth (with MH features) |
| Qumran Hebrew (QH) | hbo | 2nd c. BCE – 1st c. CE | the Dead Sea Scrolls' non-biblical texts |
| Mishnaic / Rabbinic Hebrew (MH1, MH2) | hbo-x-mishnaic | 1st – 5th c. CE | Mishnah, Tosefta, halakhic midrashim; Hebrew of the Talmuds |
| Medieval Hebrew | he-x-rabbinic (in the research corpora) | 6th – 18th c. | piyyut, commentaries (Rashi), philosophy, codes (Shulḥan ʿArukh), poetry |
| Modern Israeli Hebrew | he | revival from the late 19th c. | speech, press, literature, the web |

GKC places "New Hebrew" (the Mishnah) and the Rabbinic language in the same Canaanite branch as Biblical Hebrew (GKC §1), and sketches the history of the language with the Bible's own layers (GKC §2). Segal's four periods are Biblical, Mishnaic, Medieval and Modern (Segal 1927 §1). The finer division of the biblical corpus into ABH, CBH and LBH is the work of the 20th century, especially Hurvitz and his school; it is contested (section 10).

## 2. Counting the change: markers book by book

The OSHB morphology (CC BY 4.0) marks every word of the Masoretic text with lemma and form, so a few classical markers can be counted for every book. Counts below are Hebrew words only (Aramaic passages excluded), measured on 2026-10-17 over `$RESEARCH_RAW/scripture/1-originals/oshb-morphhb/wlc/*.xml` by Strong's lemma (אֲנִי 589, אָנֹכִי 595, אֲשֶׁר 834, the prefix שֶׁ־ coded `s/`, מַלְכוּת 4438, מַמְלָכָה 4467) and by the verb-form code (wayyiqtol `w`, weqatal `q`, qatal `p`, yiqtol `i`, infinitive absolute `a`, participle `r`).

**Pronouns and relatives.**

| book | אֲנִי | אָנֹכִי | אֲשֶׁר | שֶׁ־ | מַלְכוּת | מַמְלָכָה |
|---|---:|---:|---:|---:|---:|---:|
| Genesis | 41 | 56 | 411 | 1 | 0 | 2 |
| Exodus | 39 | 22 | 309 | 0 | 0 | 1 |
| Leviticus | 71 | 0 | 309 | 0 | 0 | 0 |
| Numbers | 21 | 7 | 295 | 0 | 1 | 2 |
| Deuteronomy | 9 | 56 | 584 | 0 | 0 | 7 |
| Judges | 12 | 17 | 177 | 5 | 0 | 0 |
| 1 Samuel | 20 | 26 | 237 | 0 | 1 | 6 |
| 2 Samuel | 30 | 24 | 191 | 0 | 0 | 6 |
| 1 Kings | 30 | 7 | 377 | 0 | 1 | 12 |
| 2 Kings | 16 | 2 | 319 | 1 | 0 | 5 |
| Ezekiel | 169 | 1 | 343 | 0 | 0 | 4 |
| Psalms | 70 | 13 | 102 | 21 | 6 | 6 |
| Song of Songs | 12 | 0 | 1 | 32 | 0 | 0 |
| Qoheleth | 29 | 0 | 89 | 70 | 1 | 0 |
| Lamentations | 4 | 0 | 9 | 4 | 0 | 1 |
| Esther | 6 | 0 | 99 | 0 | 26 | 0 |
| Daniel (Hebr.) | 23 | 1 | 47 | 0 | 16 | 0 |
| Ezra (Hebr.) | 2 | 0 | 24 | 1 | 6 | 1 |
| Nehemiah | 15 | 1 | 96 | 0 | 2 | 1 |
| 1 Chronicles | 12 | 1 | 91 | 2 | 11 | 3 |
| 2 Chronicles | 18 | 0 | 254 | 0 | 17 | 19 |

Three things show at once. **אָנֹכִי** is common in Genesis, Deuteronomy, Judges and Samuel and almost gone from Ezekiel and the late books (MH has only אֲנִי). **שֶׁ־** is concentrated in the Song of Songs, Qoheleth, Lamentations and some Psalms, and in a handful of places in Judges (Judg 5:7; 6:17; 7:12; 8:26), the base of the view that it was a northern or colloquial form long before it became the relative of MH (GKC §36). **מַלְכוּת** "kingdom", the -ût abstract, belongs to Esther, Daniel, Chronicles and Ezra, where the earlier books say מַמְלָכָה; Leviticus's 71 אֲנִי against 0 אָנֹכִי is the formula אֲנִי יְהוָה "I am the LORD" of the Holiness Code, a reminder that genre and formula weigh as much as date.

A parallel passage shows the same substitution: where 2 Sam 7:12 has וַהֲכִינֹתִי אֶת־מַמְלַכְתּוֹ "and I will establish his kingdom", 1 Chr 17:11 has וַהֲכִינוֹתִי אֶת־מַלְכוּתוֹ, with the late noun and the fuller spelling of the verb (OSHB text, checked 2026-10-17).

**Verb forms (per 1 000 Hebrew words).**

| book | words | wayyiqtol | weqatal | qatal | yiqtol | inf. abs. | participle |
|---|---:|---:|---:|---:|---:|---:|---:|
| Genesis | 20 627 | 102 | 8 | 46 | 26 | 2.7 | 16 |
| Exodus | 16 726 | 53 | 31 | 33 | 45 | 3.5 | 17 |
| Leviticus | 11 955 | 16 | 59 | 17 | 74 | 3.2 | 17 |
| Numbers | 16 422 | 46 | 25 | 27 | 39 | 2.4 | 16 |
| Deuteronomy | 14 320 | 18 | 44 | 38 | 68 | 4.4 | 28 |
| Judges | 9 905 | 115 | 7 | 49 | 22 | 2.6 | 23 |
| 1 Samuel | 13 335 | 100 | 13 | 50 | 28 | 3.7 | 23 |
| 2 Kings | 12 354 | 99 | 6 | 56 | 19 | 1.9 | 19 |
| Esther | 3 057 | 52 | 3 | 52 | 17 | 4.9 | 30 |
| Nehemiah | 5 336 | 50 | 4 | 45 | 18 | 1.7 | 33 |
| 1 Chronicles | 10 790 | 44 | 3 | 37 | 8 | 0.9 | 16 |
| 2 Chronicles | 13 354 | 74 | 5 | 50 | 15 | 1.2 | 24 |
| Qoheleth | 2 999 | 1 | 6 | 63 | 65 | 6.3 | 37 |
| Song of Songs | 1 255 | 2 | 2 | 83 | 40 | 0.8 | 39 |

(Raw counts: Genesis wayyiqtol 2 107, weqatal 164; Exodus 889 / 524; Leviticus 189 / 707; Numbers 752 / 408; Deuteronomy 255 / 632; Qoheleth 3 / 19. The Torah totals agree with MACULA's `Type` counts in our trees within a few forms: wayyiqtol 4 192 in OSHB against 4 187 in MACULA, weqatal 2 435 against 2 434.)

The wayyiqtol rate measures genre first: narrative (Genesis, Judges, Samuel, Kings) runs at about 100 per 1 000 words, law (Leviticus, Deuteronomy) is weqatal and yiqtol. The late narrative books (Esther, Nehemiah, Chronicles) still narrate with wayyiqtol, but less densely, and Qoheleth, a late reflective text close to MH, has almost none, while its participle rate is the highest in the table: the MH system of chapter 11 (qatal past, participle present) is visible inside the Bible.

## 3. Pronouns, relative and possession

| | ABH / CBH | LBH | Qumran | MH | Modern |
|---|---|---|---|---|---|
| "I" | אָנֹכִי, אֲנִי | mostly אֲנִי | אֲנִי (אנוכי in biblicising texts) | אֲנִי | אֲנִי |
| "we" | אֲנַחְנוּ (נַחְנוּ) | אֲנַחְנוּ | אנחנו, אנו | אָנוּ | אֲנַחְנוּ (אָנוּ formal) |
| "she" | הִיא (Torah ketiv הִוא) | הִיא | היאה (long forms) | הִיא | הִיא |
| "they" (m) | הֵם, הֵמָּה | הֵם, הֵמָּה | המה, הם | הֵם | הֵם |
| "this" (f) | זֹאת | זֹאת | זואת | זוֹ | זֹאת (formal), זוֹ |
| "these" | אֵלֶּה | אֵלֶּה | אלה | אֵלּוּ | אֵלֶּה, אֵלּוּ |
| relative | אֲשֶׁר (ABH זֶה/זוּ in poetry) | אֲשֶׁר, some שֶׁ־ | אשר | שֶׁ־ | שֶׁ־ (אֲשֶׁר formal) |
| possession | construct chain, suffix | same, + ל־ phrases | same | שֶׁל, "his-X of Y" | שֶׁל, construct in compounds |
| object marker | אֵת / אֶת־ (definite objects) | same, also with ל־ (Aramaic) | את | אֶת | אֶת (obligatory with definite objects) |

The poetic relatives זֶה and זוּ ("the people that you redeemed", Exod 15:13 עַם־זוּ גָּאָלְתָּ; GKC §34 d on זוּ) are an archaic layer. The Torah's own spelling הִוא for "she", read הִיא, is a perpetual qere (chapter 1).

## 4. The verb system

| function | CBH (Torah prose) | LBH | MH | Modern |
|---|---|---|---|---|
| narrative past | wayyiqtol chain | wayyiqtol, thinner; more qatal | qatal | past (qatal-derived) |
| perfect / result | qatal | qatal | qatal | past |
| present, ongoing | participle; verbless clause | participle | participle (a present tense) | present (participle-derived) |
| future, prediction | yiqtol; weqatal | yiqtol | yiqtol; עָתִיד לְ־ | future (yiqtol-derived) |
| procedure, law | weqatal chains; 2nd p. yiqtol | yiqtol | participle; yiqtol | imperative / future / infinitive |
| habitual past | yiqtol; weqatal | yiqtol | הָיָה + participle | הָיָה + participle (היה כותב) |
| command | imperative; jussive; לֹא + yiqtol | same | imperative; אַל + yiqtol | imperative; future; infinitive (לא לגעת) |
| wish, 1st person | cohortative (אֵלְכָה) | rarer | yiqtol | future, נא rarely |
| emphasis | infinitive absolute (מוֹת יוּמָת) | rarer | almost gone | gone (lexicalised adverbs: הַרְבֵּה) |
| infinitive | with and without ל; בְּ/כְּ + inf. temporal | ל + inf. grows | ל only | ל only |
| reflexive of Piel | Hithpael | Hithpael | Nithpaʿal (perfect) | Hitpaʿel |

Cook argues that wayyiqtol is a past perfective in origin and that its loss belongs to the shift of the whole system from aspect to tense (Cook 2008 and chapter 6). The MH participle as a present tense and the yiqtol as future are the direct ancestors of the Modern three-tense system; the Modern Hebrew grammar of tenses is in fact the MH system with the Biblical paradigms (Segal 1927 §§306–343; [`../he/05-verbs.md`](../he/05-verbs.md)).

> **GEN.1.3** · `wlc:GEN:2` — CBH narrative
> וַיֹּאמֶר אֱלֹהִים יְהִי אוֹר וַיְהִי־אוֹר
> *wayyōʾmer ʾĕlōhîm yəhî ʾôr wayhî-ʾôr* — and-said God let-be light and-was light

The same event in MH style (an illustration, not a text): אָמַר הַקָּדוֹשׁ בָּרוּךְ הוּא יְהִי אוֹר, וְהָיָה אוֹר *ʾāmar haqqāḏôš bārûḵ hûʾ yəhî ʾôr, wəhāyâ ʾôr* — qatal for the past, and the וְ + qatal that in the Torah would be a weqatal ("and it will be") is in MH a plain past. In Modern Hebrew וְהָיָה אוֹר can only be read as a future in the biblical idiom, or a past in the MH one, which is exactly the trap of section 12.

## 5. Word order and clause types

| | CBH | LBH | MH | Modern |
|---|---|---|---|---|
| basic order, narrative | V–S–O (forced by wayyiqtol) | V–S–O, more S–V | S–V(participle)–O; V–S with qatal | S–V–O |
| verbless clause | S–P or P–S; pronoun as copula (הוּא) | same | same; הוּא/הִיא as copula grows | S–(pronoun)–P (הוּא copula) |
| "there is" | יֵשׁ / אֵין | same | יֵשׁ / אֵין; יֵשׁ לוֹ "he has" | יֵשׁ / אֵין; יֵשׁ לִי |
| casuistic law | כִּי / אִם + yiqtol, אִישׁ כִּי | same | participle with article (הַקּוֹרֵא) | אִם + future; מִי שֶׁ־ |
| purpose | לְמַעַן, ל + inf. | ל + inf. | כְּדֵי (לְ / שֶׁ־) | כְּדֵי לְ / כְּדֵי שֶׁ־ |

In our Torah trees the verb-first order is measurable directly: the root of a wayyiqtol clause precedes its `nsubj` by construction (chapter 8 gives the counts). Holmstedt argues that the basic order of CBH is in fact S–V and that verb-first order is triggered (by wayyiqtol, by fronted elements); the question is a live one (Holmstedt 2011; chapter 8).

## 6. Negation and particles

| | CBH | MH | Modern |
|---|---|---|---|
| verb negation | לֹא | לֹא | לֹא |
| prohibition | אַל + jussive; לֹא + yiqtol (permanent) | אַל + yiqtol | אַל + future; לֹא + infinitive |
| nominal / participle negation | אֵין, אַיִן | אֵין (+ participle) | אֵין (formal), לֹא (+ present) |
| "now" | עַתָּה | עַכְשָׁיו | עַכְשָׁיו, עַתָּה (formal) |
| "here" | פֹּה, הֵנָּה | כָּאן | פֹּה, כָּאן |
| "but rather" | כִּי אִם | אֶלָּא | אֶלָּא |
| "behold" | הִנֵּה, הֵן | הֲרֵי | הִנֵּה, הֲרֵי |
| "perhaps" | אוּלַי | שֶׁמָּא | אוּלַי, שֶׁמָּא (formal) |
| "how?" | אֵיךְ, אֵיכָה | כֵּיצַד | אֵיךְ, כֵּיצַד (formal) |
| "why?" | לָמָּה, מַדּוּעַ | לָמָּה, מִפְּנֵי מָה | לָמָּה, מַדּוּעַ (formal) |
| "very" | מְאֹד | הַרְבֵּה, בְּיוֹתֵר | מְאוֹד |

Modern Hebrew keeps doublets from both layers, and uses the Biblical member as the formal one (עַתָּה, מַדּוּעַ), the Mishnaic one as neutral (עַכְשָׁיו, כָּאן) — a stylistic resource of the revived language.

## 7. Phonology and reading traditions

The pointed text of the Bible records the **Tiberian** reading tradition of about the 9th–10th centuries CE; it is not the pronunciation of the biblical period itself, and not the pronunciation of any community today (Khan 2020, vol. 1, introduction; CC BY 4.0). Other traditions:

| feature | Tiberian (Masoretic) | Babylonian, Palestinian vocalisation | MH manuscripts (Kaufmann, Parma A) | Samaritan | Modern Israeli |
|---|---|---|---|---|---|
| vowel qualities | 7 (a ā e ē i o u; ā = [ɔ]) | 6 (Bab.: no distinct segol) / 5 | close to Palestinian; *qameṣ* and *pataḥ* interchange | own system | 5 (a e i o u) |
| begadkefat | six letters alternate | same | same, with local deviations | ? | only b/v, k/x, p/f alternate; g, d, t do not |
| gutturals ʾ h ḥ ʿ | all distinct | distinct | weakened, confused in writing | merged in part | ʾ and ʿ mostly silent or [ʔ]; ḥ = [χ], merged with kh |
| emphatics ṭ ṣ q | distinct | distinct | distinct | | ṭ = t, q = k; ṣ = [ts] |
| w | [w] (and [v] for spirant b) | | | | [v], merged with spirant b |
| gemination (dagesh forte) | real | real | real | | lost |
| stress | mostly final | | | often penultimate | mostly final, many penultimate (segolates, colloquial) |

(Khan 2020 vol. 1; Kantor 2023, cited only; Henshke 2020 for MS Cambridge; Heijmans 2020: 193–199 on vowel reduction in Greek loans; the Modern column is standard description, see [`../he/`](../he/) for the details.)

Consequences for reading: Modern Hebrew speakers merge ט/ת, ק/כּ, כ/ח, ו/ב (spirant), א/ע, and do not double consonants, so the many minimal pairs of the Bible collapse in Modern pronunciation (שָׂם / שָׁם differ in Tiberian and in some reconstructions; קֹל / כֹּל "voice / all" differ in Tiberian but not always in Israeli speech).

## 8. Script and spelling

| | Biblical (inscriptions) | Biblical (Masoretic) | Qumran | MH | Modern |
|---|---|---|---|---|---|
| script | Paleo-Hebrew | square (Aramaic-derived) | square (Paleo for the divine name in some scrolls) | square | square; cursive in handwriting |
| vowel letters | sparing (Siloam: few internal) | as in the consonantal text (mixed plene/defective) | very full (כוה, היאה, לוא) | full; וו/יי for consonants | "full spelling without points" (כתיב מלא) |
| vowel points | none | Tiberian points and accents | none | in some manuscripts (Kaufmann) | only in poetry, children's books, prayer books, the Bible |
| word division | dots | spaces, maqqef | spaces | spaces | spaces |

The Siloam inscription (late 8th c. BCE), whose facsimile is printed at the front of GKC, shows the spare spelling of the monarchy (GKC §2 d and plate). Qumran's full spelling (כול, לוא, הואה) and long pronoun forms are a distinct scribal practice (Hornkohl 2020 on the relation of DSS Hebrew to the Tiberian tradition).

## 9. Vocabulary shifts

Pairs where a later stage replaces an earlier word, or gives it a new sense (BDB, Jastrow, Segal 1927 §§9–12, §§93–102; Rezetko & Young 2014 discuss whether such pairs date texts):

| CBH (Torah) | LBH | MH | Modern | meaning |
|---|---|---|---|---|
| לָקַח | קִבֵּל (Esth, Chr, Ezra) | קִבֵּל | קִבֵּל | receive |
| מַמְלָכָה | מַלְכוּת | מַלְכוּת | מַלְכוּת, מַמְלָכָה | kingdom |
| עַתָּה | עַתָּה | עַכְשָׁיו | עַכְשָׁיו | now |
| עוֹלָם "eternity" | עוֹלָם | עוֹלָם "world" | עוֹלָם "world" | |
| אֶרֶץ | | אֶרֶץ (and "the Land") | אֶרֶץ, אֶרֶץ יִשְׂרָאֵל | land, earth |
| סֵפֶר "document" | סֵפֶר, אִגֶּרֶת (Persian-period) | סֵפֶר "book", אִגֶּרֶת "letter" | סֵפֶר, אִגֶּרֶת | |
| פַּרְדֵּס (none in the Torah) | פַּרְדֵּס (Neh, Song, Qoh; Persian) | פַּרְדֵּס | פַּרְדֵּס "orchard (citrus)" | |
| יָצָא "go out" | | יָצָא (+ "fulfil a duty") | יָצָא | |
| מְלָאכָה "work" | | מְלָאכָה (Sabbath category) | מְלָאכָה "craft" | |
| חַשְׁמַל (Ezek, "amber, a gleam") | | | חַשְׁמַל "electricity" | |
| — | | — | Modern coinages: מַחְשֵׁב "computer" (root חשׁב in the maqṭēl tool pattern) | |

Modern Hebrew's vocabulary draws on all layers and on European languages, and revives many biblical roots in new senses; the semantic distance between a biblical word and its Modern homograph is a constant source of error for a reader or a model trained on Modern text (chapter 10 lists the Torah's polysemous words).

## 10. The linguistic dating debate

**The typological model.** Avi Hurvitz and his school argued that LBH can be identified by features that (a) are concentrated in the books known to be late, (b) contrast with an earlier equivalent, (c) are attested in post-biblical Hebrew or Aramaic, and (d) occur together in a text; a text with an accumulation of them is late (Hurvitz 1972, 1982; wanted). The counts of section 2 are the kind of evidence this model uses.

**The challenge.** Young, Rezetko and Ehrensvärd (2008) argued that "early" and "late" BH are better described as two co-existing styles, one more conservative and one more open to variation, and that linguistic features cannot date individual texts; Rezetko and Young (2014) develop the argument with the methods of historical sociolinguistics and variationist linguistics, stressing that the Bible's texts were transmitted and edited for centuries and that their language reflects scribal history, not just the date of composition (`$RESEARCH_RAW/hbobook-2014-rezetko-historical-linguistics-biblical-hebrew/`; Rezetko 2016 on the lexicon; Rezetko 2019, "Currents in the historical linguistics and linguistic dating of the Hebrew Bible").

**Responses and middle positions.** Hornkohl (2024, *Diachronic Diversity in Classical Biblical Hebrew*, OBP; NC licence, cited only) defends a diachronic reading with attention to variation inside CBH. Naudé (2016) and Young (2008) consider what the Qumran texts show about the continuity of LBH features; Winther-Nielsen (2019) reviews the debate with computational methods (`$RESEARCH_RAW/hbo-2019-winther-nielsen-historical-linguistics-*`). Statistical work on syntactic variation (Bijl 2019; Naaijer 2017, 2020) tests features over the whole corpus rather than lists.

**What Mova takes from it.** Counts like those of section 2 are features of texts, not dates. Mova reports them with their date of measurement and their source, and does not turn them into a dating claim.

## 11. For Mova: codes, treebanks, licences

| layer | code | Mova's data | treebank (UD 2.18) | licence of the treebank | Mova may |
|---|---|---|---|---|---|
| Biblical (Torah) | hbo | our trees from MACULA (CC BY 4.0): 5 853 sentences, 125 807 tokens | UD_Ancient_Hebrew-PTNK (Genesis–Deuteronomy, Ruth) | CC BY-NC 4.0 (README) | measure against it only |
| Mishnaic | hbo-x-mishnaic | texts only (PD editions) | none | — | — |
| Medieval / post-rabbinic | he-x-rabbinic | texts only | UD_Hebrew-PostRab (≈300 sentences, test only; Rashi, Shulḥan ʿArukh, Sefer ha-Ḥinukh, Maharil …) | CC BY-SA 4.0 | quote, test, learn from; not ship |
| Modern | he | see ../he/ | UD_Hebrew-HTB (newspaper; 5 168 train sentences) | CC BY-NC-SA 4.0 | measure only |
| | | | UD_Hebrew-IAHLTwiki (Wikipedia; 4 298 train) | CC BY-SA 4.0 | quote, test, learn; not ship |
| | | | UD_Hebrew-IAHLTknesset (parliament; 2 362 train) | CC BY-SA 4.0 | quote, test, learn; not ship |

(Sentence counts from the files under `$RESEARCH_RAW/lexres/treebanks/ud-2.18/`, 2026-10-17; licences from each README; the rules from `licence/DATA.md`.)

The codes `hbo-x-mishnaic` and `he-x-rabbinic` are private-use extensions used by the research corpora (`$RESEARCH_RAW/scripture/texts.tsv`, `$RESEARCH_RAW/lexres/resources.tsv`) to keep the layers apart; ISO 639-3 has only `hbo` (Ancient Hebrew) and `heb` / `he` (Hebrew).

## 12. Why a Modern Hebrew parser misreads the Bible

A model trained on Modern Hebrew text (HTB, IAHLT) brings Modern expectations to every word. The Torah breaks them in predictable places; each line below can be checked against our trees.

1. **wayyiqtol read as future.** וַיֹּאמֶר (GEN.1.3, `wlc:GEN:2`, MACULA `Type=wayyiqtol`) is morphologically a prefix form; in Modern Hebrew a prefix form is the future (יֹאמַר "he will say"), and ו + future is "and he will say". The Bible's commonest verb form (4 187 in the Torah) would be read as the wrong tense every time. A Modern tagger would also not separate the doubled first letter (יּ) that marks the consecutive.
2. **weqatal read as past.** וְהָיָה, וְאָכַלְתָּ in law and prophecy (2 434 weqatal in the Torah) are futures or commands; a Modern reader takes ו + past as "and it was".
3. **The construct state without שֶׁל.** פְּנֵי תְהוֹם "the face of the deep" (GEN.1.2, `wlc:GEN:1`) is a construct chain; Modern Hebrew still has the construct, but most possessive phrases in Modern text use שֶׁל, so a Modern parser sees fewer chains and may attach the second noun as an apposition or an object. Our trees mark 7 421 `compound:smixut` relations in the Torah.
4. **The object marker before every definite object.** אֵת הַשָּׁמַיִם וְאֵת הָאָרֶץ (GEN.1.1) is handled, but the Biblical אֵת is also used before a definite subject in some passages and before suffixes as אֹתוֹ / אוֹתוֹ, and the preposition אֵת "with" is a homograph (chapters 5 and 8).
5. **Verbless clauses with a pronoun copula** are common in both, but Biblical Hebrew omits the pronoun far more often, and puts the predicate first (טוֹב "good", GEN.1.4).
6. **Spelling.** Modern models are trained on unpointed full spelling; the Bible's defective spelling (קֹל for קוֹל, יֹשֵׁב for יוֹשֵׁב) and its points and accents are noise for them. Stripping the points recovers a consonantal text that is still defective.
7. **Archaic suffixes and paragogic letters**: ־ֵהוּ, ־ֶנְהוּ, the paragogic nun (105 in the Torah) and he (73), the directional he (392), the 3 m.sg suffix ־ֹה — none exists in Modern text.
8. **Vocabulary**: words whose Modern sense differs (עוֹלָם "world" vs "eternity"; חַשְׁמַל; מְלָאכָה), and words the Modern language does not have.

This is why Mova keeps the layers apart: the Torah has its own trees from MACULA, measured against PTNK; the English JPS 1917 translation is aligned verse by verse; and the Modern Hebrew manual and tools in `../he/` are a separate track. A future Biblical analyser (chapter 13, gaps) is to be built from the Biblical grammar of chapters 3–7, not adapted from a Modern one.

## 13. Sources

- Gesenius, W., Kautzsch, E., Cowley, A. E. 1910. *Gesenius' Hebrew Grammar.* §§1–3 (Semitic languages, history), §34 (demonstratives, זוּ), §36 (relative; שֶׁ־), §87 (plural endings). Public domain. `$RESEARCH_RAW/hebgram/open/gesenius-kautzsch-cowley-1910-wikisource/`
- Segal, M. H. 1927. *A Grammar of Mishnaic Hebrew.* §1 (periods), §§9–12, §§93–102 (vocabulary), §§306–343 (tenses, participle). PD-USA only; cited. `$RESEARCH_RAW/hebgram/private/segal-grammar-mishnaic-1927/`
- Khan, G. 2020. *The Tiberian Pronunciation Tradition of Biblical Hebrew*, vols. 1–2. Cambridge: OBP. CC BY 4.0. https://doi.org/10.11647/OBP.0163 ; `$RESEARCH_RAW/hebgram/open/khan-tiberian-pronunciation-v1-2020/`, `-v2-2020/`
- Heijmans, S. (ed.) 2020. *Studies in Rabbinic Hebrew.* OBP. CC BY 4.0. (Henshke on MS Cambridge; Heijmans on Greek loanwords.)
- Hornkohl, A. D., Khan, G. (eds.) 2021. *New Perspectives in Biblical and Rabbinic Hebrew.* OBP. CC BY 4.0.
- Hornkohl, A. D. 2020. "The Hebrew of the Dead Sea Scrolls and the Tiberian tradition." `$RESEARCH_RAW/hbo-2020-hornkohl-hebrew-dead-sea-scrolls-tiberian/`
- Hornkohl, A. D. 2024. *Diachronic Diversity in Classical Biblical Hebrew.* OBP, CC BY-NC; cited only. `$RESEARCH_RAW/hebgram/nc/hornkohl-diachronic-diversity-cbh-2024/`
- Kantor, B. 2023. *A Linguistic Classification of the Reading Traditions of Biblical Hebrew.* OBP, CC BY-NC; cited only.
- Rezetko, R., Young, I. 2014. *Historical Linguistics and Biblical Hebrew: Steps Toward an Integrated Approach.* SBL Press (open access). `$RESEARCH_RAW/hbobook-2014-rezetko-historical-linguistics-biblical-hebrew/`
- Rezetko, R. 2016. "An alternative approach to the lexicon of Late Biblical Hebrew." `$RESEARCH_RAW/hbo-2016-rezetko-alternative-approach-lexicon-late-*`
- Rezetko, R. 2019. "Currents in the historical linguistics and linguistic dating of the Hebrew Bible." `$RESEARCH_RAW/hbo-2019-rezetko-currents-*`
- Young, I. 2008. "Late Biblical Hebrew and the Qumran pesher Habakkuk." `$RESEARCH_RAW/hbo-2008-young-late-biblical-hebrew-qumran-pesher/`
- Naudé, J. A. 2016. "The contribution of Qumran to historical Hebrew linguistics." `$RESEARCH_RAW/hbo-2016-naude-contribution-qumran-*`
- Winther-Nielsen, N. 2019. "Historical linguistics and linguistic dating of Hebrew …" `$RESEARCH_RAW/hbo-2019-winther-nielsen-historical-linguistics-linguistic-dating-hebrew/`
- Bijl, E. 2019; Naaijer, M. 2017, 2020 — statistical studies of syntactic variation. `$RESEARCH_RAW/hbo-2019-bijl-*`, `hbo-2017-naaijer-*`, `hbo-2020-naaijer-*`
- Holmstedt, R. D. 2011. "The typological classification of the Hebrew of Genesis: subject–verb or verb–subject?" `$RESEARCH_RAW/hbo-2011-holmstedt-typological-classification-hebrew-genesis-subject/`
- Cook, J. A. 2008. "The vav-prefixed verb forms in Elementary Hebrew grammar." `$RESEARCH_RAW/hbo-2008-cook-vav-prefixed-*`
- Open Scriptures Hebrew Bible (morphhb), WLC with lemmas and morphology. CC BY 4.0. `$RESEARCH_RAW/scripture/1-originals/oshb-morphhb/` — counts of section 2.
- Swanson, D., Tyers, F. 2022. UD Ancient Hebrew PTNK (LREC). Tal, R. et al. 2025. "A New Hebrew Universal Dependency Treebank: The First Treebank of Post-Rabbinic Historical Hebrew" (TLT 2025). UD Hebrew HTB and IAHLT READMEs. `$RESEARCH_RAW/lexres/treebanks/ud-2.18/`
- Tsarfaty, R. et al. 2019. "What's wrong with Hebrew NLP? And how to make it right." `$RESEARCH_RAW/aclbulk-2019-tsarfaty-what-s-wrong-hebrew-nlp/`
- Mova: `licence/DATA.md`; `tools/macula/README.md`; the Modern Hebrew manual `languages/he/`.

**Wanted:** Hurvitz, *A Linguistic Study of the Relationship between the Priestly Source and the Book of Ezekiel* (1982) and *A Concise Lexicon of Late Biblical Hebrew* (2014); Young, Rezetko & Ehrensvärd, *Linguistic Dating of Biblical Texts* (2008, 2 vols.); Kutscher, *A History of the Hebrew Language* (1982); Sáenz-Badillos, *A History of the Hebrew Language* (1993); Qimron, *The Hebrew of the Dead Sea Scrolls* (1986); Zuckermann on the revival.
