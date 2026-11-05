# Nouns and pronouns in Targumic Aramaic

The noun system is that of Biblical Aramaic ([../arc/nouns.md](../arc/nouns.md)): two genders, singular, plural and a few duals, three states. The determined state is still **definite**: יָת שְׁמַיָּא וְיָת אַרְעָא "the heavens and the earth" (Gen 1:1). The pronoun system has lost the old forms of Ezra (הִמּוֹ, ־הֹם) and gained the demonstratives with ה־ (הָדֵין). Counts are word tokens in the public-domain Onkelos text (Sefaria, unpointed for counting), made on 2026-10-17; they are spelling counts, not lemma counts.

References: Dalman 1905 (pronouns §§ 16–20, pp. 106–123; nominal inflection §§ 38–41, pp. 188–208); Berliner 1884 vol. 2; Fassberg 2012; Golomb 1985 (for the Palestinian comparison).

Checked 2026-10-18 against Dalman 1905, read from the page images (the OCR has no Hebrew script), and against the Sefaria Onkelos text. Dalman cites Onkelos from the Sabbioneta edition with Babylonian (supralinear) vowels; where its spelling differs from the Sefaria text, both are given.

## 1. States

| state | form | Onkelos example | Hebrew of the verse |
|---|---|---|---|
| determined (definite) | ־ָא | נְהוֹרָא "the light" (Gen 1:3) | הָאוֹר |
| absolute (indefinite, predicative) | — | טָב "good" (Gen 1:4: אֲרֵי טָב) | טוֹב |
| construct | — | אַפֵּי תְהוֹמָא "the face of the deep" (Gen 1:2) | פְּנֵי תְהוֹם |

Dalman 1905 §38.1, p. 188, notes that even Onkelos has determined forms with indefinite meaning where the Hebrew is indefinite: מַלְכָּא חַדְתָּא "a new king" (Exod 1:8).

**Definiteness follows the Hebrew.** Where the Hebrew noun has the article, Onkelos uses the determined state; where the Hebrew is indefinite, mostly the absolute: יוֹמָא חָד "day one" (Gen 1:5) renders יוֹם אֶחָד with a determined noun, a mild departure. Proper names stay in the absolute: אָדָם (Gen 2:7), but הָאָדָם "the man" is often rendered as a name (יָת אָדָם), or as אֱנָשָׁא "mankind" (Gen 1:26 נַעֲבִיד אֶינָשָׁא for נַעֲשֶׂה אָדָם).

**The construct versus ד־.** Onkelos keeps the Hebrew construct chain where it can (בֵּית עַבְדוּתָא "house of bondage", Exod 20:2), and uses ד־ where an Aramaic construct would be awkward or where the Hebrew has a periphrasis: בְּעוֹפָא דִּשְׁמַיָּא "the birds of the sky" (Gen 1:26, Hebrew בְּעוֹף הַשָּׁמַיִם, a construct!); אַרְעָא דְמִצְרַיִם "the land of Egypt" (Exod 20:2, Hebrew אֶרֶץ מִצְרַיִם). Both constructions translate the same Hebrew; a parser must give them the same relation ([mova.md](mova.md) §4).

## 2. Endings

| | m. sg. | f. sg. | m. pl. | f. pl. |
|---|---|---|---|---|
| absolute | — (מְלַךְ) | ־ָא, ־ָה (מִלָּא) | ־ִין (מַלְכִין) | ־ָן (מַלְכָן) |
| construct | — | ־ַת | ־ֵי (מַלְכֵי) | ־ָת |
| determined | ־ָא (מַלְכָּא) | ־ְתָא (מַלְכְּתָא) | ־ַיָּא (מַלְכַיָּא) | ־ָתָא (מַלְכָתָא) |

Checked 2026-10-18: Dalman 1905 §38.1–2, pp. 189–190 (the paradigms of טָב and מְלַךְ / מַלְכָּא, valid for Galilean and targumic Aramaic alike): m. מַלְכִין, מַלְכַיָּא, מַלְכֵי; f. sg. absolute מַלְכָּה (the table's מִלָּא shows the spelling with א, which Dalman uses in טָבָא), determined מַלְכְּתָא, construct מַלְכַּת; f. pl. מַלְכָן, מַלְכָתָא, construct מַלְכַת. The construct vowels (ָ against ַ) cannot be compared exactly, because Dalman's targum forms carry supralinear vowels.

Examples from Onkelos: פִּתְגָּמַיָּא הָאִלֵּין "these words" (Gen 22:1, m. pl. det.); טוּרַיָּא "the mountains" (Gen 22:2); דָּרַיָּא "the generations" (Gen 22:14); בְּנוּנֵי יַמָּא "the fish of the sea" (Gen 1:26, m. pl. cst.); עַמְמַיָּא "the peoples" (Gen 49:10).

**Infinitive nouns.** Onkelos uses many infinitive forms of the peal (*miqṭal*) as nouns with suffixes, to render Hebrew infinitives construct: בְּמִתְּבָךְ "in your sitting", בִּמְהָכָךְ "in your going", בְּמִשְׁכְּבָךְ "in your lying down", בִּמְקִימָךְ "in your rising" (Deut 6:7); בְּמֶהֱוֵיהוֹן "when they were" (Gen 4:8, for Hebrew בִּהְיוֹתָם).

## 3. Independent pronouns

| person | Onkelos | count (unpointed, Onkelos) | example | Biblical Aramaic |
|---|---|---:|---|---|
| 1 sg. | אֲנָא | 308 | הָא אֲנָא "here I am" (Gen 22:1) | אֲנָה |
| 2 m. sg. | אַתְּ | 163 (את, includes the 2 f. sg.) | אָן אָתְּ "where are you?" (Gen 3:9) | אַנְתְּ |
| 2 f. sg. | אַתְּ | (in the above) | | — |
| 3 m. sg. | הוּא | 312 | | הוּא |
| 3 f. sg. | הִיא | 153 | דְדִילֵהּ הִיא מַלְכוּתָא "whose is the kingdom" (Gen 49:10) | הִיא |
| 1 pl. | אֲנַחְנָא | 36 | | אֲנַחְנָא |
| 2 m. pl. | אַתּוּן | 72 | | אַנְתּוּן |
| 2 f. pl. | אַתִּין | 2 | | — |
| 3 m. pl. | אִנּוּן | 100 | | הִמּוֹ, אִנּוּן |
| 3 f. pl. | אִנִּין | 0 in the unpointed count (spelled אנון too) | Gen 6:2 in Dalman's Sabbioneta text; the Sefaria text has אִנוּן there | אִנִּין |

Checked 2026-10-18: Dalman 1905 §16.1, p. 107 (Onkelos: אֲנָא, אַתְּ, הוּא, הִיא, אֲנַחְנָא and rarer נַחְנָא Gen 42:11, אַתּוּן, f. אַתִּין Gen 31:6, אִנּוּן, f. אִנִּין Gen 6:2). The 2 f. pl. אַתִּין is also in the Sefaria text (וְאַתִּין, Gen 31:6, with the verb יְדַעְתִּין and the suffix ־כֶן in אֲבוּכֶן).

The 3rd-person pronoun as **copula** is less frequent than in Biblical Aramaic, because Onkelos follows the Hebrew nominal clause: אֲנָא יְיָ אֱלָהָךְ "I am the LORD your God" (Exod 20:2).

## 4. Suffixes

| person | on a sg. noun | Onkelos example | on a m. pl. noun | Onkelos example |
|---|---|---|---|---|
| 1 sg. | ־ִי | בְּרִי "my son" (Gen 22:8) | ־ַי | |
| 2 m. sg. | ־ָךְ | אֱלָהָךְ "your God" (Exod 20:2); בְּרָךְ, יְחִידָךְ (Gen 22:2); לִבָּךְ, נַפְשָׁךְ, נִכְסָךְ (Deut 6:5) | ־ָיךְ / ־ָךְ | לִבְנָיךְ "to your sons" (Deut 6:7) |
| 3 m. sg. | ־ֵהּ | אִתְּתֵהּ "his wife" (Gen 3:8); שְׁמֵהּ "his name" (Exod 15:3) | ־וֹהִי | אַחוֹהִי "his brother" (Gen 4:8); בְּאַפּוֹהִי "in his nostrils" (Gen 2:7); בְּנוֹהִי (Gen 49:10) |
| 3 f. sg. | ־ַהּ | יְדַהּ | ־ַהָא | |
| 1 pl. | ־ַנָא | אֱלָהָנָא "our God" (Deut 6:4) | ־ָנָא | אֲבוּנָא "our father" (10× in Onkelos) |
| 2 m. pl. | ־כוֹן | לְוָתְכוֹן "to you" (Exod 3:14); אֱלָהֲכוֹן (87×) | ־ֵיכוֹן | |
| 3 m. pl. | ־הוֹן | אֲבוּהוֹן "their father" (20×) | ־ֵיהוֹן | תַרְוֵיהוֹן "the two of them" (Gen 22:8); בְּמֶהֱוֵיהוֹן (Gen 4:8) |

Checked 2026-10-18: Dalman 1905 §16b.1, p. 109 (the Onkelos suffixes on singular and masculine plural nouns, with the variants after vowels: ־הִי for ־ֵהּ, ־הָא for ־ַהּ). The consonants agree with the table; Dalman's Onkelos spells the 3 m. sg. ־יה where the Sefaria text mostly has ־ֵהּ (שְׁמֵהּ 44×, שְׁמֵיהּ 1×; יְדֵהּ 50×, יְדֵיהּ 5×; counted 2026-10-18). He also lists a 2 f. sg. ־ִיךְ and, as rare, 2 f. pl. ־כֵין, 3 f. pl. ־הֵין.

**Possessive pronouns** (Dalman 1905 §18, p. 119: Onkelos has only דִּיל־) are built on דִּיל־ "of": דִּילִי "mine" (13× in Onkelos), דִּילָךְ, דִּילֵהּ, דִּילַהּ (21×), דִּילָנָא, דִּילְכוֹן, דִּילְהוֹן. They render Hebrew לִי "mine" and the like: דְדִילֵהּ הִיא מַלְכוּתָא "to whom the kingdom belongs" (Gen 49:10). In Biblical Aramaic the same is expressed by דִּי לֵהּ (Dan 2:20, [../arc/pronouns.md](../arc/pronouns.md) §8).

**Reflexive**: נַפְשֵׁהּ "himself" (rare in Onkelos, 1× נפשיה in the unpointed count), mostly the t-stems.

## 5. Demonstratives

| | near | far |
|---|---|---|
| m. sg. | דֵּין (188×); הָדֵין (206×) | הַהוּא (76×) |
| f. sg. | דָּא (67×); הָדָא (90×) | הַהִיא (45×) |
| pl. | אִלֵּין (135×); הָאִלֵּין (87×) | הָאִנּוּן (12×) |

Checked 2026-10-18: Dalman 1905 §17.7, p. 113 (Onkelos: דֵּין, rarely דְּנַן, mostly adverbial; דָּא; אִלֵּין; הָדֵין, הָדָא, הָאִלֵּין; הַהוּא, הַהִיא, הָאִנּוּן; also דִּיכִי "that"). In the Sefaria text דְּנַן / דְּנָן occurs 25× (counted 2026-10-18), as an adverb or after a noun.

The forms with **ה־** (הָדֵין, הָדָא, הָאִלֵּין, and the distal הַהוּא, הַהִיא, הָאִנּוּן) are new against Biblical Aramaic and are the normal attributive demonstratives in Onkelos: בְּאָרְחָא הָדֵין "on this way" (Gen 28:20), בְּיוֹמָא הָדֵין בְּטוּרָא הָדֵין "on this day, on this mountain" (Gen 22:14), בְּאַתְרָא הַהוּא "in that place" (Gen 22:14), פִּתְגָּמַיָּא הָאִלֵּין (Gen 22:1). They mirror the Hebrew article on the demonstrative (הַדְּבָרִים הָאֵלֶּה, הַמָּקוֹם הַהוּא). The bare דֵּין "this" is used as a pronoun ("this one"): דֵּין מִן דֵּין "one from another" (Tg Isa 6:3).

## 6. Interrogatives and indefinites

| word | meaning | Onkelos count | example |
|---|---|---:|---|
| מָן, מַן | who? | (מן is also "from": 512×) | |
| מָא, מָה | what? | 25 + 69 | |
| לְמָא | why? | 24 | |
| אָן | where? | 6 | אָן אָתְּ (Gen 3:9, Hebrew אַיֶּכָּה) |
| אֵימָתַי | when? | 6 | |
| דִּלְמָא | lest, perhaps | 14 | (mostly Hebrew פֶּן; Hebrew אוּלַי is מָאִים, 17×) |
| כָּל, כֹּל | all, every | 937 + וכל 198 | |
| מַן דִּי, מָא דִּי | whoever, whatever | | |

Checked 2026-10-18: Dalman 1905 §19, p. 120 (Onkelos מַן Gen 3:11, מָא Exod 32:21 beside מָה, לְמָא, כְּמָה "how many", מְדֵין "what then?"). אָן, אֵימָתַי and דִּלְמָא are adverbs in Dalman. Second pass 2026-10-18: Dalman 1905 §44.3, p. 218 (Onkelos אָן Gen 18:9, לְאָן Gen 16:8, מְנָן "whence?" Gen 29:4, from מִן אָן); §43.2, p. 212 (Onkelos always אֵימָתַי, against Galilean אימת; עַד אֵימָתַי Exod 10:3, לְאֵימָתַי Exod 8:5; footnote: the Sabbioneta pointing doubles the ת); §46.2, p. 223 (דִּלְמָא "perhaps", from דִּי לְמָא; Onkelos uses it mostly for Hebrew פֶּן and renders Hebrew אוּלַי with מָאִים, §52). Corrected: the דִּלְמָא row named אוּלַי as its Hebrew source; the Sefaria Onkelos has מָאִים 17× (counted 2026-10-18), among them every אוּלַי of Gen 18:24–32. Disagreement: Dalman cites דִּלְמָא for Gen 43:12 O, where the Sefaria text has מָאִים. Berliner 1884, vol. 2, p. 147: the Sabbioneta edition spells מָא, כְּמָא with א, cod. de Rossi 7 with ה; this matches the two spellings מָא / מָה of the table.

## 7. Existential

| form | count | meaning | example |
|---|---:|---|---|
| אִית | 18 | there is | (Hebrew יֵשׁ) |
| לֵית | 97 | there is not | (Hebrew אֵין) |
| לֵיתוֹהִי | 15 | he is not | (Hebrew אֵינֶנּוּ: Gen 37:30; 42:13 type) |
| אִיתוֹהִי | 1 | he is | |

Checked 2026-10-18: Dalman 1905 §16.6, p. 108: in Onkelos אִית and לֵית take suffixes like a verb or noun (אִיתָךְ Gen 43:4, אִיתוֹהִי Deut 29:14, לֵיתוֹהִי Gen 37:30, לֵיתֵיכוֹן Deut 1:32).

לֵית "there is not" (from *lā ʾīṯ*) is new against Biblical Aramaic (לָא אִיתַי) and is shared with JPA; JBA has לֵיכָּא ([../arc/family.md](../arc/family.md) §5).

## 8. Numerals

As in Biblical Aramaic ([../arc/nouns.md](../arc/nouns.md) §7), with the spelling ס for שׂ in "ten" (עֲסַר) in many prints, and the ordinals קַדְמָאָה "first", תִּנְיָנָא "second", תְּלִיתָאָה "third" (in Tg Ruth 1:1 the list of famines runs קַדְמָאֵי … עֲשִׂירָאֵי). "One" is חָד / חֲדָא: יוֹמָא חָד (Gen 1:5); עַל חַד מִן טוּרַיָּא "on one of the mountains" (Gen 22:2); כַּחֲדָא "together" (Gen 22:8, Hebrew יַחְדָּו).

The Elamur team
