# The Noun, the Adjective and the Numeral

The Biblical Hebrew noun is inflected for **gender** (masculine, feminine), **number** (singular, plural, dual) and **state** (absolute, construct), and it takes **pronominal suffixes** for possession. It has no case endings (only relics), and definiteness is marked by the **article** ה- or by a definite genitive. Most nouns are built on a consonantal root and a vowel pattern (*mišqal*), and the shape of the pattern decides how the vowels change under inflection. This chapter gives the endings, the paradigms of the main patterns (the segolates above all), the irregular nouns, the directional ending, the adjective and the numerals, each with Torah examples from our trees and with notes on how MACULA and Mova's tools represent them.

## Contents

1. [Overview](#1-overview)
2. [Gender](#2-gender)
3. [Number: singular, plural, dual](#3-number-singular-plural-dual)
4. [The article](#4-the-article)
5. [State: absolute and construct](#5-state-absolute-and-construct)
6. [The construct chain](#6-the-construct-chain)
7. [Pronominal suffixes](#7-pronominal-suffixes)
8. [Noun patterns and vowel change](#8-noun-patterns-and-vowel-change)
9. [Segolate nouns](#9-segolate-nouns)
10. [Geminate nouns and nouns from weak roots](#10-geminate-nouns-and-nouns-from-weak-roots)
11. [Irregular nouns](#11-irregular-nouns)
12. [The directional he and other case relics](#12-the-directional-he-and-other-case-relics)
13. [Adjectives](#13-adjectives)
14. [Numerals](#14-numerals)
15. [Gentilics](#15-gentilics)
16. [In Mova](#16-in-mova)
17. [Sources](#17-sources)

Conventions as in the README and chapter 03: SBL academic transliteration; examples with verse and sentence id (`wlc:BOOK:n`), the Hebrew without cantillation marks, the English of JPS 1917. Counts are from our Torah trees (`$MOVA_DATA/corpora/judaism/text/torah-wlc-macula/`, 125 807 tokens) on 2026-10-17. "Paradigm" tables give standard forms from the grammars (GKC, Davidson); the forms that actually occur in the Torah are marked or quoted with their verse.

---

## 1. Overview

| category | values | marked by | in our trees |
|---|---|---|---|
| gender | masculine, feminine (common for a few) | lexical; fem. endings ־ָה, ־ֶת, ־ִית, ־וּת | FEATS `Gender=Masc/Fem/Com` |
| number | singular, plural, dual | endings ־ִים, ־וֹת, ־ַיִם | FEATS `Number=Sing/Plur/Dual` |
| state | absolute, construct | vowel changes and endings (־ֵי, ־ַת) | **no feature**; only the relation `compound:smixut` on the following noun |
| definiteness | definite, indefinite | article ה-, a definite genitive, a suffix, a proper name | article as its own token (`DET`, `Type=definite_article`) |
| possession | 10 person/gender/number suffixes | suffixes ־ִי, ־ְךָ, ־וֹ … | suffix as its own token (`PRON`, `Type=pronominal`, relation `nmod:poss`) |

NOUN is the commonest part of speech in our Torah trees (29 610 tokens; PROPN 8 004, ADJ 2 273, NUM 2 422).

---

## 2. Gender

Every noun is masculine or feminine; there is no neuter (GKC §80, §122). Gender is mostly lexical, but the feminine has endings:

| ending | from | examples | notes |
|---|---|---|---|
| ־ָה *-â* | *-at > *-ā | תּוֹרָה "law", אִשָּׁה "woman", אֲדָמָה "ground", שָׁנָה "year" | the commonest; construct ־ַת *-aṯ* (תּוֹרַת) |
| ־ֶת *-eṯ* (also ־ַת after a guttural) | *-t* added to the stem, segolized | דֶּלֶת "door", מִשְׁמֶרֶת "charge", פָּרֹכֶת "curtain", כֻּתֹּנֶת "tunic", דַּעַת "knowledge" | segolate stress (chapter 03 §8) |
| ־ִית *-îṯ* | *-iy-t | רֵאשִׁית "beginning", שְׁאֵרִית "remnant", and the fem. gentilics עִבְרִית | |
| ־וּת *-ûṯ* | *-uw-t | עֵדוּת "testimony" (EXO.26.33), מַלְכוּת "kingdom" (mostly late books) | abstracts |
| ־ֹת *-ōṯ* | | חָכְמוֹת (Proverbs); rare | |

Many feminine nouns have **no ending**: natural feminines (אֵם "mother", אָתוֹן "she-ass"), paired body parts (יָד "hand", רֶגֶל "foot", עַיִן "eye", אֹזֶן "ear"), and some others (אֶרֶץ "land", עִיר "city", חֶרֶב "sword", נֶפֶשׁ "soul", אֶבֶן "stone") (GKC §122h–l). A few nouns are used in both genders (דֶּרֶךְ "way", רוּחַ "spirit", שֶׁמֶשׁ "sun", מַחֲנֶה "camp") — they get `Gender=Com` or no gender in MACULA.

**In our trees.** Of the 29 610 NOUN tokens, 14 327 + 5 331 + 71 are `Gender=Masc` (sg/pl/dual), 4 645 + 1 218 + 107 are `Gender=Fem`, and 3 911 have no gender (2026-10-17). The feminine lemmas end in ה (3 501 tokens) or ת (1 493) much more often than in anything else; the ungendered ones include the words that are listed in the lexicon as of both genders, e.g. אֶרֶץ (FEATS `Number=Sing` only in GEN.1.1) and אוֹר (GEN.1.3).

> **GEN.1.1** · `wlc:GEN:0`
> בְּרֵאשִׁית בָּרָא אֱלֹהִים אֵת הַשָּׁמַיִם וְאֵת הָאָרֶץ
> *bərēʾšîṯ bārāʾ ʾĕlōhîm ʾēṯ haššāmayim wəʾēṯ hāʾāreṣ* — in-beginning created God OBJ the-heavens and-OBJ the-earth — "In the beginning God created the heaven and the earth." (JPS 1917)

רֵאשִׁית has the feminine ending ־ִית (FEATS `Gender=Fem|Number=Sing`); אֱלֹהִים is grammatically plural (`Gender=Masc|Number=Plur`) but takes a singular verb (בָּרָא, 3ms) when it means the God of Israel — the "plural of majesty" (GKC §124g, §145h).

**Agreement.** Verbs, adjectives and pronouns agree with the noun's grammatical gender, not with its ending: עִיר גְּדוֹלָה "a great city" (fem. adjective with an unmarked fem. noun), אָבוֹת "fathers" with a masc. verb (GKC §145). But collective nouns may take plural agreement and abstract feminines may take masculine (GKC §145b–u).

---

## 3. Number: singular, plural, dual

### 3.1 Endings

| | masculine | feminine |
|---|---|---|
| singular absolute | סוּס *sûs* "horse" | סוּסָה *sûsâ* "mare" |
| plural absolute | סוּסִים *sûsîm* | סוּסוֹת *sûsôṯ* |
| dual absolute | יוֹמַיִם *yômayim* "two days" | שְׂפָתַיִם *śəp̄āṯayim* "lips" |
| singular construct | סוּס | סוּסַת |
| plural construct | סוּסֵי *sûsê* | סוּסוֹת |
| dual construct | יְמֵי־ (rare); עֵינֵי *ʿênê* | שִׂפְתֵי |

(GKC §87, §88, §89; Davidson 1916, the "noun" lessons.)

The plural endings are **not tied to gender**: masculine nouns may take ־וֹת (אָבוֹת "fathers", מְקוֹמוֹת "places", שֵׁמוֹת "names") and feminine nouns ־ִים (נָשִׁים "women", שָׁנִים "years", עָרִים "cities") (GKC §87m–p). The noun keeps its gender: אָבוֹת is masculine.

Counted on the FORMs in our trees (2026-10-17): plural masculine nouns end in ־ִים 1 958 times and in ־וֹת/־ֹת 265 times; plural feminine nouns end in ־וֹת/־ֹת 682 times and in ־ִים 305 times. The other plural tokens (about 4 000) are construct forms (־ֵי) or forms whose suffix is split off as a separate token (e.g. FORM אֲבֹתֶי + suffix ־ךָ).

> **GEN.26.5** · `wlc:GEN:697`
> עֵקֶב אֲשֶׁר־שָׁמַע אַבְרָהָם בְּקֹלִי וַיִּשְׁמֹר מִשְׁמַרְתִּי מִצְוֹתַי חֻקּוֹתַי וְתוֹרֹתָי
> *ʿēqeḇ ʾăšer-šāmaʿ ʾaḇrāhām bəqōlî wayyišmōr mišmartî miṣwōṯay ḥuqqôṯay wəṯôrōṯāy* — because that-heard Abraham to-my-voice and-kept my-charge my-commandments my-statutes and-my-laws — "because that Abraham hearkened to My voice, and kept My charge, My commandments, My statutes, and My laws." (JPS 1917)

Four feminine nouns with the 1cs suffix: singular מִשְׁמַרְתִּי (construct stem מִשְׁמֶרֶת → מִשְׁמַרְתּ-), plural מִצְוֹתַי, חֻקּוֹתַי, תוֹרֹתָי (the last in pause, ־ָי for ־ַי). In MACULA each suffix is a separate token: `מִשְׁמַרְתּ` + `ִ֔י` (`PRON`, `Type=pronominal`, `nmod:poss`).

### 3.2 Uses of the plural

Beyond counting, the plural expresses (GKC §124):

- **abstracts and states**: חַיִּים "life" (GEN.2.9 עֵץ הַחַיִּים), נְעוּרִים "youth", זְקֻנִים "old age";
- **extension**: פָּנִים "face", מַיִם "water", שָׁמַיִם "heaven" — plural or dual in form, always plural in agreement;
- **majesty or intensity**: אֱלֹהִים "God", אֲדֹנִים "lord" (of one master), בְּעָלִים "owner" (EXO.21.29) (GKC §124g–i).

**Collectives.** Some singulars denote a class: עוֹף "birds", צֹאן "flock", בָּקָר "cattle", אָדָם "mankind", and the gentilics in the singular (הַכְּנַעֲנִי "the Canaanites"). They often take plural agreement (GKC §123a–b; Davidson 1901 §17).

### 3.3 The dual

The dual ending ־ַיִם (stressed on the first a, *-áyim*) is productive only for: natural pairs (עֵינַיִם "eyes", אָזְנַיִם "ears", יָדַיִם "hands", רַגְלַיִם "feet", שְׂפָתַיִם "lips", כְּנָפַיִם "wings", מָתְנַיִם "loins"); some measures of time and amount (יוֹמַיִם "two days", שְׁנָתַיִם "two years", מָאתַיִם "two hundred", אַלְפַּיִם "two thousand", פַּעֲמַיִם "twice"); and the number two itself (שְׁנַיִם, fem. שְׁתַּיִם) (GKC §88; Davidson 1901 §15). Adjectives and verbs have no dual: they use the plural with a dual noun (Davidson 1901 §31).

The words מַיִם and שָׁמַיִם look dual but are generally taken as plurals of a lost singular (GKC §88d). MACULA tags them mostly `Number=Plur`.

In our trees, 779 tokens have `Number=Dual` (2026-10-17). The commonest lemmas: שְׁנַיִם "two" 329, עַיִן "eye" 184, יָד "hand" 52, רֶגֶל "foot" 26, שָׁנָה "year" 23, אֹזֶן "ear" 22, מֵאָה "hundred" (= מָאתַיִם) 19, כַּף "palm" 13, עֶרֶב "evening" 11, אַף "nose" 10, אַמָּה "cubit" 9.

> **GEN.3.7** · `wlc:GEN:62`
> וַתִּפָּקַחְנָה עֵינֵי שְׁנֵיהֶם וַיֵּדְעוּ כִּי עֵירֻמִּם הֵם
> *wattippāqaḥnâ ʿênê šənêhem wayyēḏəʿû kî ʿêrummim hēm* — and-were-opened eyes-of the-two-of-them and-they-knew that naked they — "And the eyes of them both were opened, and they knew that they were naked" (JPS 1917)

Two dual constructs: עֵינֵי "the eyes of" and שְׁנֵי־ "the two of" + suffix ־הֶם. The verb וַתִּפָּקַחְנָה is 3fp, agreeing with the feminine עַיִן.

> **EXO.12.6** · `wlc:EXO:289`
> … וְשָׁחֲטוּ אֹתוֹ כֹּל קְהַל עֲדַת־יִשְׂרָאֵל בֵּין הָעַרְבָּיִם
> *wəšāḥăṭû ʾōṯô kōl qəhal ʿăḏaṯ-yiśrāʾēl bên hāʿarbāyim* — and-they-shall-slaughter it all assembly-of congregation-of-Israel between the-two-evenings — "and the whole assembly of the congregation of Israel shall kill it at dusk." (JPS 1917)

בֵּין הָעַרְבַּיִם "between the two evenings", i.e. at dusk, is an idiomatic dual (pausal ־ָיִם here). MACULA glosses it "the two evenings" and tags `Number=Dual`.

---

## 4. The article

Hebrew has a definite article and no indefinite one (Davidson 1901 §19). The article is the prefix *ha-* followed by doubling of the first consonant of the word (GKC §35):

| before | form | example | Torah |
|---|---|---|---|
| most consonants | הַ + dagesh | הַשָּׁמַיִם, הַגָּן, הַמְּאֹרֹת | GEN.1.1, GEN.2.16, GEN.1.16 |
| א ע ר (no doubling) | הָ (compensatory lengthening) | הָאָרֶץ, הָאָדָם, הָעָם, הָרָעָב | GEN.1.1, GEN.2.16, GEN.12.10 |
| ה ח (virtual doubling) | הַ | הַחֹשֶׁךְ, הַהוּא | GEN.1.4 |
| unstressed הָ, עָ, and חָ | הֶ | הֶהָרִים, הֶעָוֹן, הֶחָכָם | |
| stressed הָ, עָ | הָ | הָהָר, הָעָם | |
| after בְּ, כְּ, לְ | the ה drops, its vowel goes to the preposition | בָּאָרֶץ, לָאוֹר, כַּיּוֹם | GEN.12.10, GEN.1.5 |
| a few words | always with the "pausal" long vowel | הָאָרֶץ, הָהָר, הֶחָג, הָעָם, הַפָּר | GKC §35o |

(GKC §35b–o; chapter 03 §4 and §10.)

**Counts (2026-10-17).** The article is tokenized separately in MACULA: 9 114 tokens with `Type=definite_article` (UPOS `DET`, relation `det`). Their FORMs, without accents: הַ 4 793+, הָ 1 827+, הֶ 69+, and 1 679 with an **empty FORM** — the article absorbed by a preceding preposition, which MACULA still counts as a word (see §16).

> **GEN.12.10** · `wlc:GEN:308`
> וַיְהִי רָעָב בָּאָרֶץ וַיֵּרֶד אַבְרָם מִצְרַיְמָה לָגוּר שָׁם כִּי־כָבֵד הָרָעָב בָּאָרֶץ
> *wayhî rāʿāḇ bāʾāreṣ wayyēreḏ ʾaḇrām miṣraymâ lāḡûr šām kî-ḵāḇēḏ hārāʿāḇ bāʾāreṣ* — and-was famine in-the-land and-went-down Abram to-Egypt to-sojourn there for-heavy the-famine in-the-land — "And there was a famine in the land; and Abram went down into Egypt to sojourn there; for the famine was sore in the land." (JPS 1917)

The verse has the indefinite רָעָב "a famine" and then the anaphoric definite הָרָעָב "the famine" — the article marks a referent already introduced (GKC §126d). בָּאָרֶץ is three tokens in the tree: `בָּ` (ADP) + `` (DET, empty) + `אָרֶץ`.

**Uses.** The article marks (GKC §126; Davidson 1901 §§19–22):

- a referent known from the text (anaphoric) or from the situation: הָרָעָב above;
- a unique referent: הַשֶּׁמֶשׁ "the sun", הָאָרֶץ "the earth";
- a class (generic): הַכְּנַעֲנִי "the Canaanite(s)";
- a vocative: הַמֶּלֶךְ "O king";
- "the present": הַיּוֹם "today" (lit. "the day"), הַפַּעַם "this time";
- with an adjective or participle, a relative: הַמְדַבֵּר "the one who speaks" (GEN.45.12).

**Proper names** and nouns with a suffix or a definite genitive are definite without the article (§6; GKC §125).

---

## 5. State: absolute and construct

A noun that is followed by a genitive dependent ("X of Y") stands in the **construct state** (*nismāḵ*); a noun not so bound is in the **absolute state** (GKC §89). The construct forms a single stress unit with the following word, loses its own main stress, and its vowels reduce; in the feminine singular the old ending *-at* resurfaces; in the masculine plural and dual the ending is ־ֵי.

| pattern | absolute sg | construct sg | absolute pl | construct pl |
|---|---|---|---|---|
| qātāl | דָּבָר "word" | דְּבַר | דְּבָרִים | דִּבְרֵי |
| qātēl | זָקֵן "elder" | זְקַן | זְקֵנִים | זִקְנֵי |
| qātāl, guttural | חָכָם "wise" | חֲכַם | חֲכָמִים | חַכְמֵי |
| segolate | מֶלֶךְ "king" | מֶלֶךְ | מְלָכִים | מַלְכֵי |
| unchangeable vowel | סוּס "horse" | סוּס | סוּסִים | סוּסֵי |
| fem. -â | תּוֹרָה "law" | תּוֹרַת | תּוֹרוֹת | תּוֹרוֹת |
| fem. -â, reducing | שָׁנָה "year" | שְׁנַת | שָׁנִים | שְׁנֵי |
| fem. qətālâ | צְדָקָה "righteousness" | צִדְקַת | צְדָקוֹת | צִדְקוֹת |
| fem. -eṯ | מִשְׁמֶרֶת "charge" | מִשְׁמֶרֶת | מִשְׁמָרוֹת | מִשְׁמְרוֹת |
| III-he | שָׂדֶה "field" | שְׂדֵה | שָׂדוֹת | שְׂדוֹת |
| diphthong | בַּיִת "house" | בֵּית | בָּתִּים | בָּתֵּי |
| diphthong | מָוֶת "death" | מוֹת | — | — |
| dual | עֵינַיִם "eyes" | — | — | עֵינֵי |

(GKC §89, §92–95; Davidson 1916.)

The rules behind the table are those of chapter 03 §9: the construct has no stress of its own, so its pretonic long vowel reduces (dāḇā́r → dəḇar), and when two shewas would result the rule of shewa gives *i* (*dəḇərê → dibrê).

> **GEN.2.4** · `wlc:GEN:34`
> אֵלֶּה תוֹלְדוֹת הַשָּׁמַיִם וְהָאָרֶץ … בְּיוֹם עֲשׂוֹת יְהוָה אֱלֹהִים אֶרֶץ וְשָׁמָיִם
> *ʾēlleh ṯôləḏôṯ haššāmayim wəhāʾāreṣ … bəyôm ʿăśôṯ YHWH ʾĕlōhîm ʾereṣ wəšāmāyim* — these generations-of the-heavens and-the-earth … in-day-of making YHWH God earth and-heavens — "These are the generations of the heaven and of the earth when they were created, in the day that the LORD God made earth and heaven." (JPS 1917)

תּוֹלְדוֹת "generations of" is a feminine plural construct; בְּיוֹם עֲשׂוֹת "in the day of making" shows a noun in construct before an infinitive (GKC §130a). In MACULA's tree, the second construct is analysed with עֲשׂוֹת as `acl` of יוֹם, not as `compound:smixut` — the construct before a clause is read as a clause.

---

## 6. The construct chain

### 6.1 Structure

A **construct chain** (*sᵉmîḵûṯ*) is one or more construct nouns followed by an absolute noun: X-cs (Y-cs …) Z-abs "X of Y of Z" (GKC §127–130; Davidson 1901 §§23–28). Its rules:

1. **Nothing intervenes** between the members, except (rarely) the article or a suffix that belongs to the last; an adjective that modifies the first member comes **after** the whole chain (GKC §128a, §132).
2. **Definiteness is a property of the whole chain and is set by the last member**: if the last member is definite (article, proper name, suffix), the whole chain is definite and no earlier member takes the article (GKC §127a). פְּנֵי תְהוֹם "the face of the deep" is definite because תְּהוֹם is treated as a name; עֵץ הַגָּן "the trees of the garden" is definite through הַגָּן.
3. A noun in construct cannot normally take the article or a suffix itself.
4. "A son of ninety years" etc.: the construct also expresses age, material, quality, a genitive of the object and of the subject (GKC §128).

| relation | example | gloss |
|---|---|---|
| possession | בֵּית אָבִיךָ (GEN.12.1) | your father's house |
| part / whole | פְּנֵי תְהוֹם (GEN.1.2) | the face of the deep |
| material | אֲרוֹן עֲצֵי שִׁטִּים (EXO.25.10) | an ark of acacia wood |
| quality ("attributive genitive") | הַר קָדְשִׁי | my holy mountain (lit. mountain of my holiness) |
| superlative | עֶבֶד עֲבָדִים (GEN.9.25), קֹדֶשׁ הַקֳּדָשִׁים (EXO.26.33) | the lowest servant; the holiest place |
| objective genitive | עֵץ הַדַּעַת (GEN.2.9) | the tree of the knowledge (of good and evil) |
| age | בֶּן־תִּשְׁעִים שָׁנָה (GEN.17.1) | ninety years old (lit. a son of ninety years) |
| explicative (name) | אֶרֶץ כְּנַעַן | the land of Canaan |

> **GEN.1.2** · `wlc:GEN:1`
> וְחֹשֶׁךְ עַל־פְּנֵי תְהוֹם וְרוּחַ אֱלֹהִים מְרַחֶפֶת עַל־פְּנֵי הַמָּיִם
> *wəḥōšeḵ ʿal-pənê ṯəhôm wərûaḥ ʾĕlōhîm məraḥep̄eṯ ʿal-pənê hammāyim* — and-darkness on-face-of deep and-spirit-of God hovering on-face-of the-waters — "and darkness was upon the face of the deep; and the spirit of God hovered over the face of the waters." (JPS 1917)

Three chains: פְּנֵי תְהוֹם, רוּחַ אֱלֹהִים, פְּנֵי הַמָּיִם. In our tree each second member hangs on the first with `compound:smixut` (תְהוֹם → פְּנֵי; אֱלֹהִים → רוּחַ; מָּיִם → פְּנֵי). Note also the construct form פְּנֵי of the plural פָּנִים "face"; MACULA's lemma is פָּנֶה.

### 6.2 Long chains and coordination

Chains of three, four and five members occur. In our trees (2026-10-17) 7 421 tokens carry `compound:smixut`, headed by 7 407 construct nouns. Counted per dependent, 6 750 links are in chains of two words, 619 in chains of three, 50 of four and 2 of five. The longest is in EXO.40.6:

> **EXO.40.6** · `wlc:EXO:1180`
> וְנָתַתָּה אֵת מִזְבַּח הָעֹלָה לִפְנֵי פֶּתַח מִשְׁכַּן אֹהֶל־מוֹעֵד
> *wənāṯattâ ʾēṯ mizbaḥ hāʿōlâ lip̄nê peṯaḥ miškan ʾōhel-môʿēḏ* — and-you-shall-put OBJ altar-of the-burnt-offering before door-of tabernacle-of tent-of meeting — "And thou shalt set the altar of burnt-offering before the door of the tabernacle of the tent of meeting." (JPS 1917)

לִפְנֵי פֶּתַח מִשְׁכַּן אֹהֶל מוֹעֵד: five nouns, each in construct with the next. In the tree, פֶּתַח, מִשְׁכַּן, אֹהֶל, מוֹעֵד are each a `compound:smixut` of the one before: the chain is a right-branching staircase, as in the UD Hebrew guidelines (and PTNK).

**Coordination inside a chain.** Two genitives may follow one construct (תּוֹלְדוֹת הַשָּׁמַיִם וְהָאָרֶץ, GEN.2.4); two constructs with one genitive are usually avoided — the second is repeated with a suffix ("the sons of David and his daughters") (GKC §128a). The construct is repeated in אֱלֹהֵי הַשָּׁמַיִם וֵאלֹהֵי הָאָרֶץ (GEN.24.3).

**Alternatives to the construct.** Possession can also be expressed with the preposition לְ, especially when the possessed is indefinite: מִזְמוֹר לְדָוִד "a psalm of David", or "a son to Jesse"; GKC §129. In Mishnaic Hebrew the particle שֶׁל takes over (chapter 11).

### 6.3 What the construct looks like in MACULA

MACULA has no state attribute in the data that `tools/macula` reads, and our FEATS carry no `State` (UD Hebrew uses `Definite=Cons` on the nomen regens; the UD hbo guidelines list only Gender and Number for NOUN). The construct is recoverable from the tree: a noun with a `compound:smixut` dependent is in construct. The only exceptions are numerals and quantifiers (כָּל־), whose dependents MACULA attaches with `nmod` (see §14; GEN.2.16 מִכֹּל עֵץ־הַגָּן: עֵץ is `nmod` of כֹּל, הַגָּן is `compound:smixut` of עֵץ).

---

## 7. Pronominal suffixes

### 7.1 The two series

Possessive pronouns are suffixes. They come in two series, one for singular nouns (and feminine plurals in ־וֹת, which add the plural series on top) and one for masculine plurals and duals, where a *y* of the plural construct ending ־ֵי appears (GKC §91; Davidson 1916). Paradigm on סוּס "horse" (masc.):

| person | on singular סוּס | on plural סוּסִים |
|---|---|---|
| 1cs "my" | סוּסִי *sûsî* | סוּסַי *sûsay* |
| 2ms "your" | סוּסְךָ *sûsəḵā* | סוּסֶיךָ *sûseyḵā* |
| 2fs | סוּסֵךְ *sûsēḵ* | סוּסַיִךְ *sûsayiḵ* |
| 3ms "his" | סוּסוֹ *sûsô* (סוּסֵהוּ) | סוּסָיו *sûsāyw* |
| 3fs "her" | סוּסָהּ *sûsāh* | סוּסֶיהָ *sûseyhā* |
| 1cp "our" | סוּסֵנוּ *sûsēnû* | סוּסֵינוּ *sûsênû* |
| 2mp | סוּסְכֶם *sûsəḵem* | סוּסֵיכֶם *sûsêḵem* |
| 2fp | סוּסְכֶן *sûsəḵen* | סוּסֵיכֶן *sûsêḵen* |
| 3mp "their" | סוּסָם *sûsām* | סוּסֵיהֶם *sûsêhem* |
| 3fp | סוּסָן *sûsān* | סוּסֵיהֶן *sûsêhen* |

On a feminine noun the suffixes attach to the construct stem (סוּסָה → סוּסָת־), and a feminine plural takes the plural series after ־וֹת:

| person | סוּסָה "mare" (sg) | סוּסוֹת (pl) |
|---|---|---|
| 1cs | סוּסָתִי | סוּסוֹתַי |
| 2ms | סוּסָתְךָ | סוּסוֹתֶיךָ |
| 2fs | סוּסָתֵךְ | סוּסוֹתַיִךְ |
| 3ms | סוּסָתוֹ | סוּסוֹתָיו |
| 3fs | סוּסָתָהּ | סוּסוֹתֶיהָ |
| 1cp | סוּסָתֵנוּ | סוּסוֹתֵינוּ |
| 2mp | סוּסַתְכֶם | סוּסוֹתֵיכֶם |
| 2fp | סוּסַתְכֶן | סוּסוֹתֵיכֶן |
| 3mp | סוּסָתָם | סוּסוֹתֵיהֶם / סוּסוֹתָם |
| 3fp | סוּסָתָן | סוּסוֹתֵיהֶן / סוּסוֹתָן |

The 2mp, 2fp, 3mp, 3fp suffixes ־כֶם ־כֶן ־הֶם ־הֶן are "heavy": they take the stress and so reduce the stem like a construct (דְּבַרְכֶם, not *dəḇārḵem), see chapter 03 §9. The others are "light" and keep the absolute stem (דְּבָרִי) (GKC §91, §92).

The noun סוּס itself occurs 12 times in the Torah (torah-en.tsv key 5483), e.g. EXO.15.1 סוּס וְרֹכְבוֹ "the horse and his rider" — the second word has the 3ms suffix: `רֹכְב` + `ֽוֹ` (nmod:poss).

> **GEN.12.1** · `wlc:GEN:299`
> וַיֹּאמֶר יְהוָה אֶל־אַבְרָם לֶךְ־לְךָ מֵאַרְצְךָ וּמִמּוֹלַדְתְּךָ וּמִבֵּית אָבִיךָ אֶל־הָאָרֶץ אֲשֶׁר אַרְאֶךָּ
> *wayyōʾmer YHWH ʾel-ʾaḇrām leḵ-ləḵā mēʾarṣəḵā ûmimmôlaḏtəḵā ûmibbêṯ ʾāḇîḵā ʾel-hāʾāreṣ ʾăšer ʾarʾekkā* — and-said YHWH to-Abram go-for-you from-your-land and-from-your-kindred and-from-house-of your-father to-the-land which I-will-show-you — "Now the LORD said unto Abram: 'Get thee out of thy country, and from thy kindred, and from thy father's house, unto the land that I will show thee.'" (JPS 1917)

The 2ms suffix ־ְךָ appears four times: on a segolate (אֶרֶץ → אַרְצְךָ, §9), on a fem. noun in ־ֶת (מוֹלֶדֶת → מוֹלַדְתְּךָ), on the irregular אָב (אָבִיךָ, §11), on a preposition (לְךָ), and on a verb as object (אַרְאֶךָּ, with energic nun assimilated). In the tree each is a separate PRON token; on nouns it is `nmod:poss`, on לְ `obl`, on the verb `obj`.

### 7.2 Counts in our trees

MACULA splits every suffix pronoun off as a token: 12 172 tokens with `Type=pronominal` (UPOS PRON) in the Torah, 2026-10-17. Of them 6 596 are `nmod:poss` (on nouns), the rest are objects of verbs and prepositions. By person, gender and number:

| FEATS | count |
|---|---|
| 3ms | 3 975 |
| 2ms | 2 268 |
| 3mp | 1 830 |
| 1cs | 1 234 |
| 2mp | 1 090 |
| 3fs | 1 011 |
| 1cp | 455 |
| 2fs | 223 |
| 3fp | 79 |
| 2fp | 4 |
| 2cp | 3 |

The high 2ms and 2mp figures reflect Deuteronomy, a speech addressed to "you". MACULA gives the suffix pronouns the lemma הוּא in almost all cases (12 153 of 12 172), and puts the person, gender and number in the features; the UD Ancient Hebrew guidelines instead want the matching independent pronoun as the lemma (אֲנִי for ־ִי, אַתָּה for ־ְךָ …). Converting is mechanical from FEATS but is not done yet (§16).

### 7.3 The suffix and definiteness

A noun with a possessive suffix is definite (GKC §127a): an adjective after it takes the article — בִּנְךָ הַגָּדוֹל "your elder son" — and the noun does not.

---

## 8. Noun patterns and vowel change

Hebrew nouns are formed from roots by **patterns** (*mišqālîm*): fixed vowels and sometimes prefixes, infixes or doubling around the three root consonants (GKC §§81–86; root and lexicon in chapter 10). The pattern decides how the noun inflects, because it decides which vowels are changeable (chapter 03 §5.2). The main classes, with the abstract root q-ṭ-l:

| pattern | example | meaning type | vowel behaviour |
|---|---|---|---|
| qaṭl, qiṭl, quṭl (segolates) | מֶלֶךְ, סֵפֶר, קֹדֶשׁ | concrete and abstract nouns | §9 |
| qāṭāl | דָּבָר "word", זָהָב "gold" | many concrete nouns, adjectives (חָכָם) | both vowels changeable |
| qāṭēl | זָקֵן "old", כָּבֵד "heavy" | stative adjectives | both changeable |
| qāṭôl, qāṭûl | גָּדוֹל "great", בָּרוּךְ "blessed" | adjectives, passive participles | second unchangeable |
| qōṭēl | שֹׁפֵט "judge", אֹיֵב "enemy" | agent (active participle) | ō unchangeable, ē changeable |
| qəṭîl, qāṭîl | פָּקִיד "officer", נָבִיא "prophet" | agent / passive | î unchangeable |
| qaṭṭāl | גַּנָּב "thief", דַּיָּן "judge" | profession | doubling and ā |
| qiṭṭēl | עִוֵּר "blind", אִלֵּם "dumb" | physical defects | doubling |
| maqṭāl, miqṭāl | מִקְדָּשׁ "sanctuary", מִזְבֵּחַ "altar", מִדְבָּר "wilderness" | place, instrument | prefix m- |
| taqṭîl, tiqṭōl | תַּלְמִיד "pupil", תִּפְאֶרֶת | verbal nouns | prefix t- |
| -ān, -ôn | קָרְבָּן "offering", זִכָּרוֹן "memorial" | abstracts, results | |
| fem. qəṭālâ, qəṭullâ | צְדָקָה, גְּאֻלָּה | abstracts | |

(GKC §84a–b, §85; Davidson 1916.)

Two nouns of the same pattern inflect the same way: מִזְבֵּחַ "altar" (cs מִזְבַּח, EXO.40.6) behaves like מִשְׁכָּן "dwelling" (cs מִשְׁכַּן, EXO.40.6) and מִקְדָּשׁ. This is why grammars group the paradigms by pattern (GKC §§92–95) rather than by root.

---

## 9. Segolate nouns

The **segolates** are nouns of the old patterns *qaṭl*, *qiṭl*, *quṭl*: one stem vowel and a final two-consonant cluster. When the old case ending was lost (chapter 03 §12.2), the final cluster was broken by a helping vowel, usually segol, and the stress stayed on the first syllable: *malk > מֶ֫לֶךְ *méleḵ*. The original short vowel comes back wherever a suffix makes the cluster medial (GKC §84a, §93; Suchard 2019, §2.3 on "segolization" as a change shared with Biblical Aramaic).

### 9.1 The three types

| | a-type *qaṭl* | i-type *qiṭl* | u-type *quṭl* |
|---|---|---|---|
| example | מֶלֶךְ "king" | סֵפֶר "book" | קֹדֶשׁ "holiness" |
| sg. absolute | מֶלֶךְ *meleḵ* | סֵפֶר *sēp̄er* | קֹדֶשׁ *qōḏeš* |
| sg. construct | מֶלֶךְ | סֵפֶר | קֹדֶשׁ |
| sg. + 1cs | מַלְכִּי *malkî* | סִפְרִי *sip̄rî* | קָדְשִׁי *qoḏšî* |
| sg. + 2mp | מַלְכְּכֶם | סִפְרְכֶם | קָדְשְׁכֶם |
| pl. absolute | מְלָכִים *məlāḵîm* | סְפָרִים *səp̄ārîm* | קֳדָשִׁים *qŏḏāšîm* |
| pl. construct | מַלְכֵי *malḵê* | סִפְרֵי *sip̄rê* | קָדְשֵׁי *qoḏšê* |
| pl. + 1cs | מְלָכַי | סְפָרַי | קֳדָשַׁי |
| pl. + 2mp | מַלְכֵיכֶם | סִפְרֵיכֶם | קָדְשֵׁיכֶם |
| fem. counterpart | מַלְכָּה "queen" | | |

(GKC §93, Paradigm I a–c.)

The **plural** is formed as if the noun were *qəṭālîm*: a pretonic ā and a reduced first vowel (מְלָכִים), so the segolates look like qāṭāl nouns in the plural absolute. In the plural construct and with heavy suffixes the original vowel returns (מַלְכֵי, קָדְשֵׁי). The u-type has qamets hatuf (*o*) in the closed syllable of קָדְשִׁי *qoḏšî* and hatef qamets in קֳדָשִׁים.

**In the Torah (2026-10-17).** The lemma מֶלֶךְ has 101 tokens (torah-en.tsv key 4428): FORM מֶלֶךְ 85, מַלְכֵי 7, מְלָכִים 7, מַלְכּ- (before a suffix) 2. The lemma קֹדֶשׁ has 223 tokens (key 6944): קֹדֶשׁ 174, קֳדָשִׁים / קָדָשִׁים 33 (the qamets form mostly in pause), קָדְשֵׁי 9, קָדְשׁ- 5. סֵפֶר has 18 tokens (key 5612): סֵפֶר 16 and סִפְר- 2.

> **GEN.36.31** · `wlc:GEN:1071`
> וְאֵלֶּה הַמְּלָכִים אֲשֶׁר מָלְכוּ בְּאֶרֶץ אֱדוֹם לִפְנֵי מְלָךְ־מֶלֶךְ לִבְנֵי יִשְׂרָאֵל
> *wəʾēlleh hamməlāḵîm ʾăšer mālḵû bəʾereṣ ʾĕḏôm lip̄nê məlāḵ-meleḵ liḇnê yiśrāʾēl* — and-these the-kings who reigned in-land-of Edom before reigning-of king over-the-sons-of Israel — "And these are the kings that reigned in the land of Edom, before there reigned any king over the children of Israel." (JPS 1917)

The verse has the plural הַמְּלָכִים, the verb מָלְכוּ "they reigned" and the infinitive construct מְלָךְ "the reigning of" of the same root מלך, and the segolate singular מֶלֶךְ.

> **GEN.17.16** · `wlc:GEN:413`
> … וְהָיְתָה לְגוֹיִם מַלְכֵי עַמִּים מִמֶּנָּה יִהְיוּ
> *wəhāyəṯâ ləḡôyim malḵê ʿammîm mimmennâ yihyû* — and-she-shall-become to-nations kings-of peoples from-her shall-be — "and she shall be a mother of nations; kings of peoples shall be of her." (JPS 1917)

מַלְכֵי עַמִּים: the plural construct of the segolate (original *a* restored) and the plural of the geminate עַם (§10).

> **EXO.24.7** · `wlc:EXO:651`
> וַיִּקַּח סֵפֶר הַבְּרִית וַיִּקְרָא בְּאָזְנֵי הָעָם וַיֹּאמְרוּ כֹּל אֲשֶׁר־דִּבֶּר יְהוָה נַעֲשֶׂה וְנִשְׁמָע
> *wayyiqqaḥ sēp̄er habbərîṯ wayyiqrāʾ bəʾoznê hāʿām wayyōʾmərû kōl ʾăšer-dibber YHWH naʿăśeh wənišmāʿ* — and-he-took book-of the-covenant and-read in-ears-of the-people and-they-said all that-spoke YHWH we-will-do and-we-will-obey — "And he took the book of the covenant, and read in the hearing of the people; and they said: 'All that the LORD hath spoken will we do, and obey.'" (JPS 1917)

סֵפֶר in construct (= absolute form, as with all segolates) and the dual construct of the segolate אֹזֶן "ear": אָזְנֵי *ʾoznê*, with the u-vowel restored as qamets hatuf.

> **EXO.26.33** · `wlc:EXO:735`
> … וְהִבְדִּילָה הַפָּרֹכֶת לָכֶם בֵּין הַקֹּדֶשׁ וּבֵין קֹדֶשׁ הַקֳּדָשִׁים
> *wəhiḇdîlâ happārōḵeṯ lāḵem bên haqqōḏeš ûḇên qōḏeš haqqŏḏāšîm* — and-shall-divide the-veil for-you between the-holy and-between holy-of the-holies — "and the veil shall divide unto you between the holy place and the most holy." (JPS 1917)

### 9.2 Subtypes

- **Guttural segolates** take patah instead of segol next to a guttural: נַעַר "lad", זֶרַע "seed", פֹּעַל "work", לֶחֶם "bread" (the helping vowel is patah when the 2nd or 3rd radical is a guttural; GKC §93q–u).
- **Diphthong segolates** (*qayl, *qawl): בַּיִת "house" (*bayt), עַיִן "eye", זַיִת "olive"; מָוֶת "death" (*mawt), תָּוֶךְ "midst" (*tawk). Construct and suffix forms contract: בֵּית, עֵין, מוֹת, תּוֹךְ (GKC §93u–x; chapter 03 §11).
- **III-he segolates** (*qaṣy): פְּרִי "fruit", חֳלִי "sickness", בְּכִי "weeping" (GKC §84a, §93x).
- **Feminine segolates** in ־ֶת: מִשְׁמֶרֶת, דֶּלֶת, קְטֹרֶת; with suffix מִשְׁמַרְתִּי (GEN.26.5).

The segolate pattern is the single largest class of Hebrew nouns; in the Torah dictionary (torah-en.tsv) most two-syllable lemmas with segol in the second syllable are segolates (אֶרֶץ, דֶּרֶךְ, עֶבֶד, נֶפֶשׁ, זֶרַע, לֶחֶם, בֹּקֶר, עֶרֶב, חֹדֶשׁ, אֹהֶל).

---

## 10. Geminate nouns and nouns from weak roots

**Geminate nouns** come from roots whose 2nd and 3rd radicals are the same (ע״ע). In the singular absolute the doubling is lost at the end of the word; it reappears before any ending (GKC §93ee–hh):

| absolute | + suffix | plural | plural construct | gloss |
|---|---|---|---|---|
| עַם | עַמִּי | עַמִּים | עַמֵּי | people |
| חֹק | חֻקִּי | חֻקִּים | חֻקֵּי | statute |
| לֵב | לִבִּי | (לִבּוֹת) | | heart |
| הַר | הַרְרִי (poetic) | הָרִים | הָרֵי | mountain (doubling of r impossible: ā) |
| כַּף | כַּפִּי | כַּפּוֹת | כַּפּוֹת | palm |
| fem. חֻקָּה | חֻקָּתִי | חֻקּוֹת | חֻקּוֹת | statute |

GEN.26.5 has the fem. חֻקּוֹתַי "my statutes" (lemma חֻקָּה). The doubling in these nouns is visible in the FORM (dagesh forte) and nowhere else in the tree.

**Nouns from hollow roots** (ע״ו/ע״י) keep the long vowel: קוֹל "voice", שֵׂם — and the plurals קֹלוֹת, שֵׁמוֹת. **Nouns from III-he roots** end in ־ֶה in the absolute and ־ֵה in the construct: שָׂדֶה / שְׂדֵה "field", מַרְאֶה / מַרְאֵה "appearance" (GEN.29.17), מַעֲשֶׂה / מַעֲשֵׂה "work" (GKC §93ll–uu).

> **GEN.29.17** · `wlc:GEN:812`
> וְעֵינֵי לֵאָה רַכּוֹת וְרָחֵל הָיְתָה יְפַת־תֹּאַר וִיפַת מַרְאֶה
> *wəʿênê lēʾâ rakkôṯ wərāḥēl hāyəṯâ yəp̄aṯ-tōʾar wîp̄aṯ marʾeh* — and-eyes-of Leah weak and-Rachel was beautiful-of form and-beautiful-of appearance — "And Leah's eyes were weak; but Rachel was of beautiful form and fair to look upon." (JPS 1917)

Here the adjective יָפָה "beautiful" stands in construct (יְפַת) before a noun that specifies the respect — "beautiful of form" (GKC §128x). רַכּוֹת is the fem. pl. of the geminate adjective רַךְ "soft, weak", agreeing with the feminine dual עֵינַיִם.

---

## 11. Irregular nouns

A handful of very common nouns have stems that change unpredictably; they must be learned (GKC §96; Davidson 1916). The table gives the main forms and the Torah counts by lemma from torah-en.tsv (2026-10-17).

| noun | gloss | construct sg | + 3ms suffix | plural abs | plural cs | Torah tokens |
|---|---|---|---|---|---|---|
| אָב | father | אֲבִי | אָבִיו | אָבוֹת (masc!) | אֲבוֹת | 413 |
| אָח | brother | אֲחִי | אָחִיו | אַחִים | אֲחֵי (+ suff. אֶחָיו) | 285 |
| אִישׁ | man | אִישׁ | אִישָׁהּ ("her husband") | אֲנָשִׁים | אַנְשֵׁי | 570 |
| אִשָּׁה | woman, wife | אֵשֶׁת | אִשְׁתּוֹ | נָשִׁים | נְשֵׁי | 307 |
| בֵּן | son | בֶּן־ | בְּנוֹ | בָּנִים | בְּנֵי | 1 496 |
| בַּת | daughter | בַּת | בִּתּוֹ | בָּנוֹת | בְּנוֹת | 204 |
| בַּיִת | house | בֵּית | בֵּיתוֹ | בָּתִּים | בָּתֵּי | (many) |
| יוֹם | day | יוֹם | — | יָמִים | יְמֵי | 668 |
| עִיר | city | עִיר | עִירוֹ | עָרִים | עָרֵי | 172 |
| פֶּה | mouth | פִּי | פִּיו / פִּיהוּ | פִּיּוֹת | | 124 |
| מַיִם | water | מֵי, מֵימֵי | מֵימָיו | — | | 207 |
| שָׁמַיִם | heaven | שְׁמֵי | | — | | |
| רֹאשׁ | head | רֹאשׁ | רֹאשׁוֹ | רָאשִׁים | רָאשֵׁי | |
| כְּלִי | vessel | כְּלִי | | כֵּלִים | כְּלֵי | |

Notes:

- אָב and אָח take a connecting *-î-* before suffixes and in the construct, from an old genitive ending (*abī-) (GKC §96, §90k): אֲבִי, אָבִיךָ, אָבִיו. In our trees the lemma אָב has the FORMs אָבִי 168, אֲבִי 59, אָב 49, אֲבֹת 48, אֲבֹתֶי- 34, אֲבֹתֵי 24, אָבוֹת 8, אֲבוֹת 7 (2026-10-17) — the defective spelling אֲבֹת is far commoner than the plene one in the Torah.
- אִישׁ and אִשָּׁה come from different roots in the plural (אֲנָשִׁים from אנשׁ; נָשִׁים); the suffixed forms of אִשָּׁה use the stem אִשְׁתּ-.
- בֵּן and בַּת share the plural stem בָּנ-: בָּנִים / בָּנוֹת.
- בָּתִּים has a dagesh in ת after a long vowel, an anomaly discussed at length by Khan (2020, vol. 1, §I.3.1.12).
- יוֹם has the dual יוֹמַיִם "two days" and the plural יָמִים (with stem change); the adverb יוֹמָם "by day" has an old adverbial ending (GKC §100g).
- פֶּה has the construct פִּי, identical with "my mouth" פִּי (GEN.45.12 כִּי־פִי הַמְדַבֵּר "that it is my mouth that speaks" — MACULA splits it into `פ` + `ִ֖י`).

> **GEN.2.24** · `wlc:GEN:54`
> עַל־כֵּן יַעֲזָב־אִישׁ אֶת־אָבִיו וְאֶת־אִמּוֹ וְדָבַק בְּאִשְׁתּוֹ וְהָיוּ לְבָשָׂר אֶחָד
> *ʿal-kēn yaʿăzoḇ-ʾîš ʾeṯ-ʾāḇîw wəʾeṯ-ʾimmô wəḏāḇaq bəʾištô wəhāyû ləḇāśār ʾeḥāḏ* — therefore leaves-man OBJ-his-father and-OBJ-his-mother and-cleaves to-his-wife and-they-become to-flesh one — "Therefore shall a man leave his father and his mother, and shall cleave unto his wife, and they shall be one flesh." (JPS 1917)

Three irregular nouns with the 3ms suffix: אָבִיו (connecting *-î-*), אִמּוֹ (geminate אֵם → אִמּ-), אִשְׁתּוֹ (stem אִשְׁתּ-). In MACULA the FORM of the first is split `אָבִ֖י` + `ו`, with the *-î-* staying on the noun.

> **GEN.18.2** · `wlc:GEN:426`
> וַיִּשָּׂא עֵינָיו וַיַּרְא וְהִנֵּה שְׁלֹשָׁה אֲנָשִׁים נִצָּבִים עָלָיו … וַיִּשְׁתַּחוּ אָרְצָה
> *wayyiśśāʾ ʿênāyw wayyarʾ wəhinnēh šəlōšâ ʾănāšîm niṣṣāḇîm ʿālāyw … wayyištaḥû ʾārṣâ* — and-he-lifted his-eyes and-saw and-behold three men standing over-him … and-bowed-down to-the-ground — "and he lifted up his eyes and looked, and, lo, three men stood over against him; … and bowed down to the earth" (JPS 1917)

אֲנָשִׁים, the suppletive plural of אִישׁ; עֵינָיו, the dual with the 3ms plural-series suffix; and אָרְצָה with the directional ending (§12).

---

## 12. The directional he and other case relics

### 12.1 The directional ending ־ָה

An unstressed ־ָה *-â* on a noun or adverb marks **direction towards** (*he locale*): אַרְצָה "to the ground", מִצְרַיְמָה "to Egypt", שָׁמָּה "thither", הָהָרָה "to the mountain", צָפֹנָה "northward", יָמָּה "westward (seaward)", הַבַּיְתָה "into the house" (GKC §90c–i). It is distinct from the feminine ־ָה by its lack of stress (אַ֫רְצָה vs. feminine ־ָ֫ה) and by its function; and from the 3fs suffix ־ָהּ by the absence of mappiq (chapter 03 §11). It can stand on a construct noun (אַרְצָה כְּנַעַן "to the land of Canaan", GEN.11.31) and with the article (הָאֹהֱלָה "into the tent", GEN.24.67). With a preposition it sometimes keeps a weakened sense or none (לְמַעְלָה "upward", מִלְמַעְלָה "from above") (GKC §90e).

GKC (§90a, c) took it for a remnant of the old accusative ending *-a*; the Ugaritic evidence, where the directional *-h* is written with a consonant while the accusative *-a* is not, has led many since to separate the two (see "wanted" in sources.md for the later literature).

**In our trees** the directional he is a separate token: 392 tokens, lemma ה, `Type=directional_he`, attached to its host with `nmod` (2026-10-17). The commonest hosts (by lemma): שָׁם "there" 75, מַעַל "above" 53, אֶרֶץ 30, מִזְבֵּחַ 28, מִצְרַיִם 27, קֵדֶם "east" 15, בַּיִת 14, הַר 11, יָם 11, חוּץ 11, מִזְרָח 10, תֵּימָן 9, צָפוֹן 8, מִדְבָּר 7.

> **GEN.24.67** · `wlc:GEN:658`
> וַיְבִאֶהָ יִצְחָק הָאֹהֱלָה שָׂרָה אִמּוֹ וַיִּקַּח אֶת־רִבְקָה וַתְּהִי־לוֹ לְאִשָּׁה וַיֶּאֱהָבֶהָ וַיִּנָּחֵם יִצְחָק אַחֲרֵי אִמּוֹ
> *wayḇiʾehā yiṣḥāq hāʾōhĕlâ śārâ ʾimmô wayyiqqaḥ ʾeṯ-riḇqâ wattəhî-lô ləʾiššâ wayyeʾĕhāḇehā wayyinnāḥēm yiṣḥāq ʾaḥărê ʾimmô* — and-brought-her Isaac to-the-tent-of Sarah his-mother and-took OBJ-Rebekah and-she-became to-him for-wife and-he-loved-her and-was-comforted Isaac after his-mother — "And Isaac brought her into his mother Sarah's tent, and took Rebekah, and she became his wife; and he loved her. And Isaac was comforted for his mother." (JPS 1917)

הָאֹהֱלָה "into the tent (of Sarah)" combines the article and the directional ending on a noun that is also the head of a construct with שָׂרָה. In the tree: `הָ` (det) + `אֹ֙הֱל` (obl) + `ָה֙` (nmod, directional) — and MACULA's English glosses are shifted by one token here (`הָ`/"to", `אֹהֱל`/"the", `ָה`/"tent"), because the gloss of the English phrase is distributed over the Hebrew tokens left to right.

> **GEN.12.10** (above, §4): מִצְרַיְמָה "to Egypt" — `מִצְרַ֙יְמ` + `ָה֙`.

### 12.2 Other relics

- **Connecting vowels in the construct**: the *-î* of אֲבִי, אֲחִי and in poetic forms like בְּנוֹ בְעֹר "son of Beor" (NUM.24.3, *-ô*) and חַיְתוֹ־אֶרֶץ "beast of the earth" (GEN.1.24) — remnants of nominative *-u* and genitive *-i* (GKC §90k–o).
- **Adverbial *-ām***: יוֹמָם "by day", אָמְנָם "truly", חִנָּם "for nothing", רֵיקָם "empty-handed" (GKC §100g).
- **The "accusative" of time and place**: nouns used adverbially without a preposition (הַיּוֹם "today", בֹּקֶר "in the morning") (GKC §118). MACULA tags these as `adv` role → `obl` or `advmod` (chapter 08).

---

## 13. Adjectives

### 13.1 Forms

Adjectives inflect like nouns, for gender and number (no dual), and agree with their noun (GKC §132; Davidson 1901 §§30–31):

| | masc. | fem. |
|---|---|---|
| sg | טוֹב, גָּדוֹל, קָטֹן, יָפֶה | טוֹבָה, גְּדוֹלָה, קְטַנָּה, יָפָה |
| pl | טוֹבִים, גְּדוֹלִים, קְטַנִּים, יָפִים | טוֹבוֹת, גְּדוֹלוֹת, קְטַנּוֹת, יָפוֹת |

The patterns are those of §8: qāṭôl (גָּדוֹל), qāṭōn (קָטֹן, with doubled nun before endings: קְטַנָּה), qāṭēl (זָקֵן, כָּבֵד), qāṭāl (חָכָם), III-he (יָפֶה). There is no separate comparative or superlative form (Davidson 1901 §33).

MACULA tags 2 273 tokens ADJ in the Torah; 1 503 have `Type=adjective` (the remainder are gentilics and other adjectival classes), 2026-10-17.

### 13.2 Attributive and predicative

- **Attributive**: the adjective follows the noun and agrees with it in gender, number **and definiteness** — if the noun is definite, the adjective takes the article (GKC §126u, §132a):
  - indefinite: גּוֹי גָּדוֹל "a great nation" (GEN.12.2);
  - definite: הַמָּאוֹר הַגָּדֹל "the great light" (GEN.1.16), הַתַּנִּינִם הַגְּדֹלִים "the great sea-monsters" (GEN.1.21).
- **Predicative**: the adjective usually precedes the subject or follows it without the article; the clause is verbless (chapter 08): כִּי־טוֹב "that (it was) good" (GEN.1.4), כָבֵד הָרָעָב "the famine (was) heavy" (GEN.12.10) (GKC §141).

> **GEN.1.16** · `wlc:GEN:15`
> וַיַּעַשׂ אֱלֹהִים אֶת־שְׁנֵי הַמְּאֹרֹת הַגְּדֹלִים אֶת־הַמָּאוֹר הַגָּדֹל לְמֶמְשֶׁלֶת הַיּוֹם וְאֶת־הַמָּאוֹר הַקָּטֹן לְמֶמְשֶׁלֶת הַלַּיְלָה וְאֵת הַכּוֹכָבִים
> *wayyaʿaś ʾĕlōhîm ʾeṯ-šənê hamməʾōrōṯ haggəḏōlîm ʾeṯ-hammāʾôr haggāḏōl ləmemšeleṯ hayyôm wəʾeṯ-hammāʾôr haqqāṭōn ləmemšeleṯ hallaylâ wəʾēṯ hakkôḵāḇîm* — and-made God OBJ-two-of the-lights the-great OBJ-the-light the-great for-rule-of the-day and-OBJ-the-light the-small for-rule-of the-night and-OBJ the-stars — "And God made the two great lights: the greater light to rule the day, and the lesser light to rule the night; and the stars." (JPS 1917)

הַמְּאֹרֹת הַגְּדֹלִים: a noun with the feminine-looking plural ־וֹת takes a masculine adjective, because מָאוֹר is masculine. "The greater light", "the lesser light": the positive adjective with the article has comparative force when two are contrasted (GKC §133f). In the tree each adjective is `amod` of its noun and its article is a `det` of the adjective.

> **GEN.29.16** · `wlc:GEN:811`
> וּלְלָבָן שְׁתֵּי בָנוֹת שֵׁם הַגְּדֹלָה לֵאָה וְשֵׁם הַקְּטַנָּה רָחֵל
> *ûləlāḇān šəttê ḇānôṯ šēm haggəḏōlâ lēʾâ wəšēm haqqəṭannâ rāḥēl* — and-to-Laban two daughters name-of the-great Leah and-name-of the-small Rachel — "Now Laban had two daughters: the name of the elder was Leah, and the name of the younger was Rachel." (JPS 1917)

The adjectives הַגְּדֹלָה, הַקְּטַנָּה stand alone as nouns ("the elder", "the younger") and are the genitives of שֵׁם in construct. MACULA tags them as `compound:smixut` of שֵׁם, as for nouns.

### 13.3 Comparison

Hebrew compares with the preposition מִן "from, than" after the positive adjective or a stative verb: X עָרוּם מִכֹּל "more cunning than all" (GKC §133a; Davidson 1901 §33). The superlative is expressed by the article ("the great one" among several), by a construct chain with a plural of the same noun (קֹדֶשׁ הַקֳּדָשִׁים, עֶבֶד עֲבָדִים), or by a construct with כֹּל, or by מְאֹד "very" (GKC §133g–l; Davidson 1901 §34).

> **GEN.3.1** · `wlc:GEN:56`
> וְהַנָּחָשׁ הָיָה עָרוּם מִכֹּל חַיַּת הַשָּׂדֶה אֲשֶׁר עָשָׂה יְהוָה אֱלֹהִים
> *wəhannāḥāš hāyâ ʿārûm mikkōl ḥayyaṯ haśśāḏeh ʾăšer ʿāśâ YHWH ʾĕlōhîm* — and-the-serpent was cunning from-all beast-of the-field which made YHWH God — "Now the serpent was more subtle than any beast of the field which the LORD God had made." (JPS 1917)

> **GEN.1.31** · `wlc:GEN:30`
> וַיַּרְא אֱלֹהִים אֶת־כָּל־אֲשֶׁר עָשָׂה וְהִנֵּה־טוֹב מְאֹד
> *wayyarʾ ʾĕlōhîm ʾeṯ-kol-ʾăšer ʿāśâ wəhinnēh-ṭôḇ məʾōḏ* — and-saw God OBJ-all-that he-made and-behold-good very — "And God saw every thing that He had made, and, behold, it was very good." (JPS 1917)

> **GEN.9.25** · `wlc:GEN:230`
> וַיֹּאמֶר אָרוּר כְּנָעַן עֶבֶד עֲבָדִים יִהְיֶה לְאֶחָיו
> *wayyōʾmer ʾārûr kənāʿan ʿeḇeḏ ʿăḇāḏîm yihyeh ləʾeḥāyw* — and-he-said cursed Canaan servant-of servants he-shall-be to-his-brothers — "And he said: Cursed be Canaan; a servant of servants shall he be unto his brethren." (JPS 1917)

**In our trees.** The predicate adjective after הָיָה is attached as `obj` of the verb in MACULA's reading (GEN.3.1: עָרוּם is `obj` of הָיָה; also GEN.1.2 תֹהוּ), because MACULA's clause role for a predicate complement of הָיָה is mapped by `tools/macula` to `obj` there. UD PTNK treats הָיָה as a copula in such places (tools/macula/README.md, "PTNK also reads הָיָה as a copula in places where we keep it the head"). The comparative מִן is `case` on כֹּל, with כֹּל as `nmod` of the adjective.

### 13.4 Adjectives before the noun and in construct

A few adjectives precede the noun: רַב "many" (רַבִּים עַמִּים), and quantifiers; and an adjective may head a construct with a noun of respect (יְפַת־תֹּאַר "beautiful of form", GEN.29.17; GKC §128x, §132c). The demonstratives זֶה/זֹאת/אֵלֶּה follow the noun and its adjectives and take the article in the attributive position: הַמָּקוֹם הַזֶּה "this place" (GEN.28.17); the pronoun chapter treats them in full.

---

## 14. Numerals

### 14.1 Cardinals 1–10

The numerals 1 and 2 are adjectives (1) and dual nouns (2); 3–10 are nouns with a striking **gender polarity**: with a masculine noun they take the form with the feminine ending, with a feminine noun the form without it (GKC §97a; Davidson 1901 §36).

| | with masc. noun: absolute | construct | with fem. noun: absolute | construct |
|---|---|---|---|---|
| 1 | אֶחָד | אַחַד | אַחַת | אַחַת |
| 2 | שְׁנַיִם | שְׁנֵי | שְׁתַּיִם | שְׁתֵּי |
| 3 | שְׁלֹשָׁה | שְׁלֹשֶׁת | שָׁלֹשׁ | שְׁלֹשׁ |
| 4 | אַרְבָּעָה | אַרְבַּעַת | אַרְבַּע | אַרְבַּע |
| 5 | חֲמִשָּׁה | חֲמֵשֶׁת | חָמֵשׁ | חֲמֵשׁ |
| 6 | שִׁשָּׁה | שֵׁשֶׁת | שֵׁשׁ | שֵׁשׁ |
| 7 | שִׁבְעָה | שִׁבְעַת | שֶׁבַע | שְׁבַע |
| 8 | שְׁמֹנָה | שְׁמֹנַת | שְׁמֹנֶה | שְׁמֹנֶה |
| 9 | תִּשְׁעָה | תִּשְׁעַת | תֵּשַׁע | תְּשַׁע |
| 10 | עֲשָׂרָה | עֲשֶׂרֶת | עֶשֶׂר | עֶשֶׂר |

(GKC §97; Davidson 1916.)

The numeral usually precedes the noun, in apposition (שְׁלֹשָׁה בָנִים "three sons", GEN.6.10) or in construct (שְׁלֹשֶׁת יָמִים "three days"); it may also follow (בָּנִים שְׁלֹשָׁה) (GKC §134a–c).

> **GEN.6.10** · `wlc:GEN:147`
> וַיּוֹלֶד נֹחַ שְׁלֹשָׁה בָנִים אֶת־שֵׁם אֶת־חָם וְאֶת־יָפֶת
> *wayyôleḏ nōaḥ šəlōšâ ḇānîm ʾeṯ-šēm ʾeṯ-ḥām wəʾeṯ-yāp̄eṯ* — and-begot Noah three sons OBJ-Shem OBJ-Ham and-OBJ-Japheth — "And Noah begot three sons, Shem, Ham, and Japheth." (JPS 1917)

שְׁלֹשָׁה with the *-â* ending + masculine בָּנִים — polarity.

> **GEN.41.2** · `wlc:GEN:1197`
> וְהִנֵּה מִן־הַיְאֹר עֹלֹת שֶׁבַע פָּרוֹת יְפוֹת מַרְאֶה וּבְרִיאֹת בָּשָׂר וַתִּרְעֶינָה בָּאָחוּ
> *wəhinnēh min-hayʾōr ʿōlōṯ šeḇaʿ pārôṯ yəp̄ôṯ marʾeh ûḇərîʾōṯ bāśār wattirʿênâ bāʾāḥû* — and-behold from-the-Nile coming-up seven cows fair-of appearance and-fat-of flesh and-they-grazed in-the-reed-grass — "And, behold, there came up out of the river seven kine, well-favoured and fat-fleshed; and they fed in the reed-grass." (JPS 1917)

שֶׁבַע without the ending + feminine פָּרוֹת — polarity the other way.

### 14.2 Tens, hundreds, thousands; compound numbers

- 11–19: units + עָשָׂר (masc.) / עֶשְׂרֵה (fem.): אַחַד עָשָׂר "eleven" (GEN.32.23), שִׁבְעָה־עָשָׂר "seventeen" (GEN.7.11), אַרְבָּעָה עָשָׂר "fourteen" (EXO.12.6); also עַשְׁתֵּי עָשָׂר for 11 (3 tokens in the Torah).
- tens are plurals in ־ִים of the units: עֶשְׂרִים 20, שְׁלֹשִׁים 30, אַרְבָּעִים 40 … תִּשְׁעִים 90.
- מֵאָה 100, dual מָאתַיִם 200, plural מֵאוֹת for 300+ (שְׁלֹשׁ מֵאוֹת); אֶלֶף 1000, dual אַלְפַּיִם, plural אֲלָפִים; רְבָבָה 10 000.
- In compound numbers the order is free: larger first or smaller first, linked by וְ; the counted noun may be repeated with each part, in the singular after large numbers ("year") and plural after units (GKC §134h–i; Davidson 1901 §37).

> **GEN.5.5** · `wlc:GEN:110`
> וַיִּהְיוּ כָּל־יְמֵי אָדָם אֲשֶׁר־חַי תְּשַׁע מֵאוֹת שָׁנָה וּשְׁלֹשִׁים שָׁנָה וַיָּמֹת
> *wayyihyû kol-yəmê ʾāḏām ʾăšer-ḥay təšaʿ mēʾôṯ šānâ ûšəlōšîm šānâ wayyāmōṯ* — and-were all-days-of Adam that-he-lived nine hundreds year and-thirty year and-he-died — "And all the days that Adam lived were nine hundred and thirty years; and he died." (JPS 1917)

> **GEN.17.1** · `wlc:GEN:398`
> וַיְהִי אַבְרָם בֶּן־תִּשְׁעִים שָׁנָה וְתֵשַׁע שָׁנִים
> *wayhî ʾaḇrām ben-tišʿîm šānâ wəṯēšaʿ šānîm* — and-was Abram son-of-ninety year and-nine years — "And when Abram was ninety years old and nine" (JPS 1917)

שָׁנָה (singular) after ninety, שָׁנִים (plural) after nine; age by "son of" + number.

> **NUM.1.46** · `wlc:NUM:45`
> וַיִּהְיוּ כָּל־הַפְּקֻדִים שֵׁשׁ־מֵאוֹת אֶלֶף וּשְׁלֹשֶׁת אֲלָפִים וַחֲמֵשׁ מֵאוֹת וַחֲמִשִּׁים
> *wayyihyû kol-happəquḏîm šēš-mēʾôṯ ʾelep̄ ûšəlōšeṯ ʾălāp̄îm waḥămēš mēʾôṯ waḥămiššîm* — and-were all-the-numbered six-hundreds thousand and-three-of thousands and-five hundreds and-fifty — "even all those that were numbered were six hundred thousand and three thousand and five hundred and fifty." (JPS 1917)

603 550: 600 × 1000 + 3 × 1000 + 500 + 50. שְׁלֹשֶׁת אֲלָפִים uses the construct of 3 with the masculine אֶלֶף.

> **GEN.32.23** · `wlc:GEN:950`
> וַיָּקָם בַּלַּיְלָה הוּא וַיִּקַּח אֶת־שְׁתֵּי נָשָׁיו וְאֶת־שְׁתֵּי שִׁפְחֹתָיו וְאֶת־אַחַד עָשָׂר יְלָדָיו
> *wayyāqom ballaylâ hûʾ wayyiqqaḥ ʾeṯ-šəttê nāšāyw wəʾeṯ-šəttê šip̄ḥōṯāyw wəʾeṯ-ʾaḥaḏ ʿāśār yəlāḏāyw* — and-rose in-the-night that and-took OBJ-two-of his-wives and-OBJ-two-of his-maids and-OBJ-eleven his-children — "And he rose up that night, and took his two wives, and his two handmaids, and his eleven children" (JPS 1917; the eBible file numbers this verse 32:22 with the note "(32-23)", following the English versification)

### 14.3 Ordinals

Ordinals 1–10 are adjectives in ־ִי (fem. ־ִית): רִאשׁוֹן "first" (a different root), שֵׁנִי, שְׁלִישִׁי, רְבִיעִי, חֲמִישִׁי, שִׁשִּׁי, שְׁבִיעִי, שְׁמִינִי, תְּשִׁיעִי, עֲשִׂירִי. Above ten the cardinals are used (GKC §98; Davidson 1901 §38). The days of creation use the cardinal for the first (יוֹם אֶחָד, GEN.1.5) and ordinals for the rest; the sixth has the article (יוֹם הַשִּׁשִּׁי, GEN.1.31), the others not.

> **GEN.7.11** · `wlc:GEN:170`
> בִּשְׁנַת שֵׁשׁ־מֵאוֹת שָׁנָה לְחַיֵּי־נֹחַ בַּחֹדֶשׁ הַשֵּׁנִי בְּשִׁבְעָה־עָשָׂר יוֹם לַחֹדֶשׁ
> *bišnaṯ šēš-mēʾôṯ šānâ ləḥayyê-nōaḥ baḥōḏeš haššēnî bəšiḇʿâ-ʿāśār yôm laḥōḏeš* — in-year-of six-hundreds year of-life-of-Noah in-the-month the-second on-seventeen day of-the-month — "In the six hundredth year of Noah's life, in the second month, on the seventeenth day of the month" (JPS 1917)

"In the 600th year" uses the cardinal in a construct with שְׁנַת; "the second month" the ordinal adjective with the article; "the 17th day" the cardinal.

**Counts (2026-10-17).** `Type=cardinal_number` 2 253 tokens, `Type=ordinal_number` 235: שֵׁנִי 58, שְׁבִיעִי 57, שְׁלִישִׁי 28, רִאשׁוֹן 26, רְבִיעִי 15, שְׁמִינִי 14, עֲשִׂירִי 11, שִׁשִּׁי 9, חֲמִישִׁי 6, fem. חֲמִישִׁית and שְׁלִישִׁית 3 each, עַשְׁתֵּי 3, תְּשִׁיעִי 2. The seventh is so common because of the Sabbath and the seventh month and year.

### 14.4 Numerals in MACULA and UD

UD would make the counted noun the head and the numeral its `nummod`, with the parts of a compound number linked by `flat` or `compound` (UD hbo dependency index lists `nummod`, `flat`; the Hebrew guidelines are CC BY-SA, paraphrased). MACULA's phrase rule instead puts the numeral first: in `שְׁלֹשָׁה בָנִים` (GEN.6.10) and `שְׁלֹשָׁה אֲנָשִׁים` (GEN.18.2) the numeral is the head (`obj`, `nsubj`) and the noun is its `nmod`. `tools/macula` keeps that and only turns the internal links of a compound number into `flat` (שֵׁשׁ־מֵאוֹת, אַחַד עָשָׂר). As a result the Torah has only 572 `nummod` relations, almost all of them אֶחָד and the ordinals, which do follow their noun (יוֹם אֶחָד, הַחֹדֶשׁ הַשֵּׁנִי), and 450 `flat`. Of the deviations from PTNK listed in tools/macula/README.md, this is one source of `nmod`/`nummod` disagreement still to fix.

---

## 15. Gentilics

Nouns of nationality and descent (*gentilics*) are formed with ־ִי (fem. ־ִית, pl. ־ִים, fem. pl. ־ִיּוֹת): עִבְרִי "Hebrew", מִצְרִי "Egyptian", כְּנַעֲנִי "Canaanite", לֵוִי "Levite", יִשְׂרְאֵלִי "Israelite" (GKC §86h). In the singular with the article they often denote the whole people (הַכְּנַעֲנִי "the Canaanites", GEN.12.6, GEN.24.3). The feminine יְהוּדִית "in Judean (language)" is the name of the language in 2 Kings 18:26 (not in the Torah).

> **GEN.14.13** · `wlc:GEN:349`
> וַיָּבֹא הַפָּלִיט וַיַּגֵּד לְאַבְרָם הָעִבְרִי וְהוּא שֹׁכֵן בְּאֵלֹנֵי מַמְרֵא הָאֱמֹרִי …
> *wayyāḇōʾ happālîṭ wayyaggēḏ ləʾaḇrām hāʿiḇrî wəhûʾ šōḵēn bəʾēlōnê mamrēʾ hāʾĕmōrî* — and-came the-fugitive and-told to-Abram the-Hebrew and-he dwelling in-oaks-of Mamre the-Amorite — "And there came one that had escaped, and told Abram the Hebrew — now he dwelt by the terebinths of Mamre the Amorite" (JPS 1917)

Two gentilics in apposition with the article: הָעִבְרִי, הָאֱמֹרִי (`appos` in the tree).

**Counts (2026-10-17).** `Type=gentilic` 436 tokens. By lemma: לֵוִי 87 (the tribe name and "Levite" share it), אֱמֹרִי 40, כְּנַעֲנִי 31, עִבְרִי 22, מִצְרִי 19, חִתִּי 19, חִוִּי 12, יְבוּסִי 11, פְּרִזִּי 10, פְּלִשְׁתִּי 10, then clan names (גֵּרְשֻׁנִּי, קְהָתִי, רְאוּבֵנִי …). MACULA tags gentilics NOUN or ADJ depending on use; the plurals of the Table of Nations (GEN.10.13 לוּדִים, עֲנָמִים, לְהָבִים, נַפְתֻּחִים) are tagged as gentilics with the plural as lemma.

---

## 16. In Mova

**Tokenization.** MACULA (and so our trees) cuts every noun word into its morphemes: conjunction ו, prepositions ב כ ל מ, article ה, the noun stem, the directional ending, the possessive suffix. וּמִבֵּית אָבִיךָ (GEN.12.1) is six tokens: `וּ` `מִ` `בֵּית` `אָבִי` `ךָ` — the PTNK and UD Ancient Hebrew convention as well (UD hbo tokenization page). Each token has its own MISC `Ref` (word reference in the verse, the same for all pieces of one written word) and `MId`.

**Features.** FEATS on nouns: `Gender` (Masc/Fem/Com, missing for some), `Number` (Sing/Plur/Dual). There is **no `State` and no `Definite` feature**; definiteness is the presence of a `det` child (article), a `nmod:poss` child (suffix), a `compound:smixut` child that is definite, or PROPN. The construct is a noun with a `compound:smixut` dependent (§6.3). MISC `Type` marks `definite_article` (9 114), `pronominal` (12 172), `directional_he` (392), `adjective` (1 503), `cardinal_number` (2 253), `ordinal_number` (235), `gentilic` (436).

**Lemmas and the dictionary.** The LEMMA is MACULA's pointed lemma: the context form of the absolute singular (אֶרֶץ, not pausal אָרֶץ; שָׁמַיִם; פָּנֶה for פָּנִים). The Torah dictionary `$MOVA_DATA/corpora/judaism/dict/torah-en.tsv` keys each word by Strong's number (3 486 keys, 2026-10-17) and gives the count, MACULA's commonest glosses, the OSHB definition and the Brown–Driver–Briggs entry; e.g. key 4428 מֶלֶךְ, 101 tokens, glosses "the.king (75); kings (9); a.king (5)". The `.examples` file gives each word its two shortest one-verse examples with the Hebrew, the word-for-word glosses and JPS 1917. Inflected forms are not listed; the dictionary is by lemma.

**UD conventions applied by tools/macula** (from its README): construct = `compound:smixut`; demonstrative after a noun = `det` — but in the Torah the attributive demonstrative with its article (הַמָּקוֹם הַזֶּה GEN.28.17, בַּיּוֹם הַזֶּה GEN.7.11) still comes out as `amod` (זֶה 196, זֹאת 90 times, 2026-10-17): the `det` rule of tools/macula fires on a bare demonstrative word, while MACULA groups the article and the demonstrative into an adjective phrase, which the phrase rule makes `amod`; compound numbers = `flat`; adjectives `amod`; possessive suffixes `nmod:poss`; a second noun in a phrase that is not a genitive `nmod`; apposition `appos`.

**Pragmatics.** `macula prag` does not look at the noun morphology except through person features: a 1st or 2nd person **suffix** on a noun in the root's own clause (`Person=1/2` on a `nmod:poss`) makes the sentence count as spoken by a character ("voice: character"), since a narrator does not say "my" or "your" (tools/macula/src/main.rs, `hebrew_prag`). So DEU.6.5 (בְּכָל־לְבָבְךָ) is read as a character's speech.

**Gaps (2026-10-17).**

- no `State`/`Definite` features (could be derived: construct from `compound:smixut`, definite from article/suffix/name/definite genitive);
- suffix lemmas are הוּא for all persons; UD wants the matching independent pronoun;
- empty-FORM article tokens (1 679) after prepositions, with no multiword-token line;
- numerals head their nouns (MACULA's phrase structure), so `nummod` is rare;
- predicate adjectives and nouns after הָיָה are `obj`;
- MACULA's English glosses can be shifted across the pieces of a split word (GEN.24.67 הָאֹהֱלָה; GEN.12.10 מִצְרַיְמָה) — the gloss is a gloss of the written word, not of the token;
- no paradigm generator: Mova cannot yet produce or analyse an inflected noun form by rule; it relies on MACULA's analysis of the attested forms.

---

## 17. Sources

- **GKC** — W. Gesenius, E. Kautzsch, A. E. Cowley, *Gesenius' Hebrew Grammar*, 2nd English ed., Oxford 1910, public domain. §35 (article), §§79–98 (the noun, its gender, number, construct, case relics, suffixes, vowel changes, paradigms, irregular nouns, numerals), §100 (adverbs), §§122–134 (syntax of gender, number, determination, construct, apposition, adjective, comparison, numerals), §141, §145. Text: $RESEARCH_RAW/hebgram/open/gesenius-kautzsch-cowley-1910-wikisource/gesenius-kautzsch-cowley-1910-wikisource.txt; https://en.wikisource.org/wiki/Gesenius%27_Hebrew_Grammar
- **Davidson 1901** — A. B. Davidson, *Hebrew Syntax*, 3rd ed., Edinburgh 1901, public domain. §§15–17 (number), §§19–22 (article), §§23–28 (construct), §§30–34 (adjective, comparison), §§35–38 (numerals). $RESEARCH_RAW/hebgram/pd/davidson-hebrew-syntax-1901/cu31924060183385/
- **Davidson 1916** — A. B. Davidson, *An Introductory Hebrew Grammar*, 19th ed., Edinburgh 1916, public domain (noun paradigms). $RESEARCH_RAW/hebgram/pd/davidson-introductory-hebrew-grammar-1916/
- **Khan 2020** — Geoffrey Khan, *The Tiberian Pronunciation Tradition of Biblical Hebrew*, OBP 2020, CC BY 4.0, https://doi.org/10.11647/OBP.0163 (§I.3.1.12 on בָּתִּים). $RESEARCH_RAW/hebgram/open/khan-tiberian-pronunciation-v1-2020/
- **Suchard 2019** — Benjamin D. Suchard, "Sound Changes in the (Pre-)Masoretic Reading Tradition and the Original Pronunciation of Biblical Aramaic", *Studia Orientalia Electronica* 7 (2019), §2.3 on segolization. $RESEARCH_RAW/hbo-2019-suchard-sound-changes-pre-masoretic-reading/paper.pdf
- **BDB** — F. Brown, S. R. Driver, C. A. Briggs, *A Hebrew and English Lexicon of the Old Testament*, Oxford 1906, public domain; via Sefaria $RESEARCH_RAW/lexres/dictionaries/sefaria/BDB_Dictionary.jsonl and in torah-en.tsv.
- **UD Ancient Hebrew guidelines** — Universal Dependencies documentation for hbo (tokenization, features, relations), CC BY-SA 4.0: described, not copied. $RESEARCH_RAW/hebgram/sa/ud-docs-he-hbo/; https://universaldependencies.org/hbo/
- **UD Ancient Hebrew PTNK** — treebank README (UD 2.18), CC BY-NC-SA: only described. $RESEARCH_RAW/lexres/treebanks/ud-2.18/nc/UD_Ancient_Hebrew-PTNK/README.md
- **Miller-Naudé & Naudé 2016** — on the adjective as a category distinct from the noun in Biblical Hebrew. $RESEARCH_RAW/hbo-2016-miller-naude-adjective-distinct-noun-grammatical-category/
- **Stein 2008** — on the noun in Biblical Hebrew and affiliation terms. $RESEARCH_RAW/hbo-2008-stein-noun-biblical-hebrew-term-affiliation/
- **Kroeze 1994** — on gender and sex in Biblical Hebrew. $RESEARCH_RAW/hbo-1994-kroeze-three-dimensional-approach-gender-sex/
- **MACULA Hebrew** — https://github.com/Clear-Bible/macula-hebrew/, CC BY 4.0 (Biblica, Clear Bible, the Groves Center; text and morphology from OSHB, CC BY 4.0). Our trees: $MOVA_DATA/corpora/judaism/text/torah-wlc-macula/; converter tools/macula (README, src/main.rs).
- **Torah dictionary** — $MOVA_DATA/corpora/judaism/dict/torah-en.tsv(.examples), built by the dictionary builder (BDB 1906 public domain, OSHB lexical index CC BY 4.0, MACULA glosses CC BY 4.0).
- **JPS 1917** — public domain, via eBible.org: $MOVA_DATA/corpora/judaism/text/torah-jps1917/
- Wanted (not held): J. Fox, *Semitic Noun Patterns* (2003); on the directional he after Ugaritic, e.g. the treatment in Waltke–O'Connor (1990) §10.5 and in Joüon–Muraoka (2006) §93c (both under copyright); a full paradigm source for the noun with suffixes in machine-readable form (e.g. ETCBC/BHSA morphology, NC in our registry: $RESEARCH_RAW/hebgram/nc/etcbc-bhsa-docs).
