# The weak verbs

A root is "weak" when one of its radicals is a letter that the sound rules of Hebrew do not let through unchanged: a guttural (א ה ח ע, and in part ר), which refuses doubling and vocal shewa; *nun*, which assimilates to the next consonant; *yod* and *waw*, which turn into vowels; a final *he*, which is not a consonant at all but a vowel letter; or a doubled radical. Twenty of the thirty commonest verbs of the Torah are weak in at least one way (see [06-verb.md §15.3](06-verb.md#153-the-thirty-commonest-verbs)), so reading the Torah means reading weak verbs. This chapter takes the classes one by one: how to recognise each, its forms in the main stems, its commonest Torah members with counts, and how the trees let Mova recover the root from a form whose letters have changed.

Paradigms are given for the stems the class actually uses; the strong-verb tables and the names of the forms are in [06-verb.md](06-verb.md). All counts are from the MACULA-based Torah trees (`$MOVA_DATA/corpora/judaism/text/torah-wlc-macula/*.conllu`) on **2026-10-17**; "attested" forms are quoted without accents, with the first verse where the trees have them.

## Contents

1. [Naming the classes](#1-naming-the-classes)
2. [Verbs with gutturals: I-guttural](#2-verbs-with-gutturals-i-guttural)
3. [II-guttural and II-resh](#3-ii-guttural-and-ii-resh)
4. [III-guttural (ḥet, ʿayin) and III-aleph](#4-iii-guttural-ḥet-ʿayin-and-iii-aleph)
5. [I-nun](#5-i-nun)
6. [I-yod and I-waw](#6-i-yod-and-i-waw)
7. [Hollow verbs: II-waw and II-yod](#7-hollow-verbs-ii-waw-and-ii-yod)
8. [III-he](#8-iii-he)
9. [Geminate verbs (II = III)](#9-geminate-verbs-ii--iii)
10. [Doubly weak and irregular verbs](#10-doubly-weak-and-irregular-verbs)
11. [The hishtaphel of חוה](#11-the-hishtaphel-of-חוה)
12. [How to find the root of a weak form](#12-how-to-find-the-root-of-a-weak-form)
13. [The weak classes in the Torah: counts](#13-the-weak-classes-in-the-torah-counts)
14. [In Mova](#14-in-mova)
15. [Sources](#15-sources)

---

## 1. Naming the classes

The traditional names use the letters of the model verb פעל for the three radical positions (GKC §41; JM §§68–82):

| class | traditional name | position | example | GKC |
|---|---|---|---|---|
| I-guttural | פ״גרונית | 1st radical guttural | עָמַד "stand" | §63 |
| I-aleph (quiescent) | פ״א | 1st radical א that goes silent | אָכַל "eat", אָמַר "say" | §68 |
| II-guttural | ע״גרונית | 2nd radical guttural or ר | בָּרַךְ "bless", שָׁחַט "slaughter" | §64 |
| III-guttural | ל״גרונית | 3rd radical ח or ע | שָׁמַע "hear", שָׁלַח "send" | §65 |
| III-aleph | ל״א | 3rd radical א | מָצָא "find" | §74 |
| I-nun | פ״נ | 1st radical נ | נָפַל "fall", נָתַן "give" | §66 |
| I-yod / I-waw | פ״י / פ״ו | 1st radical י (mostly from *w*) | יָשַׁב "sit", יָטַב "be good" | §§69–71 |
| hollow | ע״ו / ע״י | 2nd radical a vowel | קוּם "rise", שִׂים "put" | §§72–73 |
| III-he | ל״ה (originally ל״י/ל״ו) | 3rd radical a vowel, written ה | בָּנָה "build" | §75 |
| geminate | ע״ע | 2nd = 3rd radical | סָבַב "surround" | §67 |
| doubly weak | — | two of the above | נָשָׂא, נָטָה, יָצָא, בּוֹא | §76 |

A root can belong to several classes at once: עָשָׂה is I-guttural and III-he; יָצָא is I-yod and III-aleph; נָשָׂא is I-nun and III-aleph. GKC §77 discusses how the classes borrow forms from one another (a hollow verb taking geminate forms, a III-aleph verb taking III-he forms).

## 2. Verbs with gutturals: I-guttural

### 2.1 The rules

The gutturals א ה ח ע share four habits (GKC §22):

1. they take no dagesh forte, so a doubling is given up and the vowel before may lengthen (compensatory lengthening) or stay short (virtual doubling);
2. they prefer *a*-vowels before and after them;
3. under a guttural, a vocal shewa becomes a hateph (ă, ĕ, ŏ);
4. a silent shewa after a vowel is often replaced by a hateph echoing that vowel (יַעֲמֹד, not *yaʿmōd*).

### 2.2 Forms

For a I-guttural root the effects show where the first radical closes a syllable after a prefix, or would carry vocal shewa (GKC §63):

| | Qal | Niphal | Hiphil | Hophal |
|---|---|---|---|---|
| qatal 3ms | עָמַד *ʿāmaḏ* | נֶעֱמַד *neʿĕmaḏ* | הֶעֱמִיד *heʿĕmîḏ* | הָעֳמַד *hoʿŏmaḏ* |
| yiqtol 3ms | יַעֲמֹד *yaʿămōḏ* (also יֶחְזַק, יֶאֱסֹף) | יֵעָמֵד *yēʿāmēḏ* | יַעֲמִיד *yaʿămîḏ* | יָעֳמַד *yoʿŏmaḏ* |
| wayyiqtol 3ms | וַיַּעֲמֹד | — | וַיַּעֲמֵד | — |
| imperative 2ms | עֲמֹד *ʿămōḏ* | הֵעָמֵד | הַעֲמֵד | — |
| infinitive construct | עֲמֹד | הֵעָמֵד | הַעֲמִיד | — |
| participle | עֹמֵד | נֶעֱמָד | מַעֲמִיד | מָעֳמָד |
| qatal 2mp | עֲמַדְתֶּם (hateph for *ə*) | | | |

Notes: in the Qal yiqtol the prefix vowel is *a* before ע and ח (יַעֲמֹד, יַחְשֹׁב) but *e* before stative or ח-initial verbs of *a*-type (יֶחְזַק "be strong", יֶחְדַּל "cease"); in the Niphal yiqtol the doubling of the first radical is impossible and the prefix vowel lengthens to *ē* (יֵעָמֵד, not *yiʿʿāmēḏ*) (GKC §63).

**Attested in the Torah (2026-10-17).** Qal עָמַד 3ms (GEN.19.27); וַיַּעֲמֹד 9× (DEU.31.15); יַעֲמֹד (EXO.21.21); imperative עֲמֹד (DEU.5.31); Hiphil וְהֶעֱמִיד 6× (LEV.14.11), וַיַּעֲמִד (GEN.47.7); Hophal יָעֳמַד (LEV.16.10).

> **DEU.31.15** · `wlc:DEU:850`
> וַיֵּרָא יְהוָה בָּאֹהֶל בְּעַמּוּד עָנָן וַיַּעֲמֹד עַמּוּד הֶעָנָן עַל־פֶּתַח הָאֹהֶל
> *wayyērāʾ YHWH bāʾōhel bəʿammûḏ ʿānān wayyaʿămōḏ ʿammûḏ heʿānān ʿal-peṯaḥ hāʾōhel* — and-appeared YHWH in-the-tent in-a-pillar-of cloud and-stood the-pillar-of the-cloud over-the-door-of the-tent — "And the LORD appeared in the Tent in a pillar of cloud; and the pillar of cloud stood over the door of the Tent." (JPS 1917) — a III-aleph Niphal (וַיֵּרָא) beside a I-guttural Qal (וַיַּעֲמֹד).

### 2.3 I-aleph verbs with a silent aleph (פ״א)

Five Torah verbs — אָמַר "say" (1 373), אָכַל "eat" (328), אָבַד "perish" (35), אָבָה "be willing" (11), אָפָה "bake" (9) — let the א go silent in the Qal yiqtol: the prefix takes a long *ō* and the א is only written (יֹאמַר *yōʾmar*, not *yeʾĕmōr*); in the 1cs the prefix א and the radical א merge in writing (אֹמַר for *ʾeʾmar*) (GKC §68). In the other stems they behave like ordinary I-guttural verbs (Niphal נֶאֱכַל, Hiphil הֶאֱכִיל).

| | אָמַר | אָכַל |
|---|---|---|
| yiqtol 3ms | יֹאמַר *yōʾmar* | יֹאכַל *yōʾḵal* |
| yiqtol 1cs | אֹמַר *ʾōmar* | אֹכַל *ʾōḵal* |
| wayyiqtol 3ms | וַיֹּאמֶר *wayyōʾmer* (pause וַיֹּאמַר) | וַיֹּאכַל *wayyōʾḵal* |
| wayyiqtol 1cs | וָאֹמַר *wāʾōmar* | וָאֹכֵל *wāʾōḵēl* |
| imperative | אֱמֹר *ʾĕmōr* | אֱכֹל *ʾĕḵōl* |
| infinitive construct | אֱמֹר; with לְ: לֵאמֹר *lēʾmōr* | אֱכֹל; לֶאֱכֹל |

The wayyiqtol וַיֹּאמֶר is the single commonest finite form of the Torah: 592 × 3ms with *e* and 40 × in pause with *a* (וַיֹּאמַר) in the trees. Before a speech the infinitive לֵאמֹר "saying" (305 tokens, the form split by MACULA into לֵ + אמֹר) has the vowel *ē* on the preposition because the א is silent (GKC §68).

> **GEN.3.13** · `wlc:GEN:68`
> וַתֹּאמֶר הָאִשָּׁה הַנָּחָשׁ הִשִּׁיאַנִי וָאֹכֵל
> *wattōʾmer hāʾiššâ hannāḥāš hiššîʾanî wāʾōḵēl* — and-said the-woman the-serpent deceived-me and-I-ate — "And the woman said: 'The serpent beguiled me, and I did eat.'" (JPS 1917) — three weak verbs: I-aleph אמר, I-nun + III-aleph נשׁא (Hiphil הִשִּׁיא "deceive"), I-aleph אכל.

## 3. II-guttural and II-resh

The second radical is doubled in the D stems (Piel, Pual, Hithpael). When it is a guttural or ר the doubling is lost: before א and ר the preceding vowel is lengthened (בֵּרַךְ, בֹּרַךְ), before ה, ח, ע it usually stays short and the doubling is "virtual" (נִחַם, טִהַר) (GKC §64, §22). A vocal shewa under the second radical becomes *ă* (שָׁחֲטוּ, not *šāḥəṭû*), and the Qal yiqtol and imperative take *a* instead of *ō* (יִשְׁחַט, שְׁאַל) (GKC §64).

| | Qal | Piel | Pual | Hithpael |
|---|---|---|---|---|
| qatal 3ms | שָׁחַט | בֵּרַךְ *bēraḵ* | בֹּרַךְ *bōraḵ* | הִתְבָּרֵךְ |
| qatal 3cp | שָׁחֲטוּ *šāḥăṭû* | בֵּרְכוּ | בֹּרְכוּ | הִתְבָּרְכוּ |
| yiqtol 3ms | יִשְׁחַט *yišḥaṭ* | יְבָרֵךְ *yəḇārēḵ* | יְבֹרַךְ | יִתְבָּרֵךְ |
| wayyiqtol 3ms | וַיִּשְׁחַט | וַיְבָרֶךְ *wayḇāreḵ* | | |
| imperative | שְׁחַט | בָּרֵךְ *bārēḵ* | — | |
| participle | שֹׁחֵט | מְבָרֵךְ | מְבֹרָךְ | |

**Attested in the Torah.** Piel בֵּרַךְ / בֵּרַכְתִּי / בֵּרַכְתָּ (DEU.33.1, GEN.17.20, GEN.32.27); וַיְבָרֶךְ 18× (EXO.39.43) and וַיְבָרֲכֵם with suffix (GEN.26.12); יְבָרֶכְךָ (NUM.6.24, DEU.15.10); imperative בָּרֵךְ (DEU.26.15); Hithpael הִתְבָּרֵךְ (DEU.29.18). Qal שָׁחַט 16× weqatal in Leviticus (LEV.1.5), וַיִּשְׁחַט / וַיִּשְׁחָט (LEV.9.8, 8.15), יִשְׁחַט (LEV.4.24); Qal of גָּאַל: יִגְאַל (LEV.25.33), Niphal יִגָּאֵל (LEV.25.30).

> **EXO.39.43** · `wlc:EXO:1174`
> וַיַּרְא מֹשֶׁה אֶת־כָּל־הַמְּלָאכָה … וַיְבָרֶךְ אֹתָם מֹשֶׁה
> *wayyarʾ mōše ʾeṯ-kol-hamməlāʾḵâ … wayḇāreḵ ʾōṯām mōše* — and-saw Moses OM-all-the-work … and-blessed them Moses — "And Moses saw all the work … And Moses blessed them." (JPS 1917)

> **GEN.12.2** · `wlc:GEN:300`
> וְאֶעֶשְׂךָ לְגוֹי גָּדוֹל וַאֲבָרֶכְךָ וַאֲגַדְּלָה שְׁמֶךָ וֶהְיֵה בְּרָכָה
> *wəʾeʿeśḵā ləḡôy gāḏôl waʾăḇāreḵḵā waʾăḡaddəlâ šəmeḵā wehyē bərāḵâ* — and-I-will-make-you into-a-nation great and-I-will-bless-you and-I-will-make-great your-name and-be a-blessing — "And I will make of thee a great nation, and I will bless thee, and make thy name great; and be thou a blessing." (JPS 1917) — the II-*resh* Piel (אֲבָרֶכְךָ, no doubling), the strong Piel (אֲגַדְּלָה, doubled ד), and the III-he forms אֶעֶשְׂךָ and הֱיֵה.

The commonest II-guttural/II-*resh* Torah verbs (by the trees; the list overlaps with other classes): רָאָה 396, קָרָא 200, קָרַב 194, בָּרַךְ 137, יָרַשׁ 101, יָרַד 88, יָרֵא 80, כָּרַת 69, טָהֵר 54, רָחַץ 47, שָׁחַט 47, הָרַג 42, אָרַר 41, אָהֵב 40, בָּחַר 39 (2 845 tokens over 193 lemmas, 2026-10-17).

## 4. III-guttural (ḥet, ʿayin) and III-aleph

### 4.1 III-ḥet and III-ʿayin

A final ח or ע wants an *a*-vowel before it (GKC §65):

- where the strong verb has *ō* or *ē* before the last radical, the III-guttural verb has *a*: Qal yiqtol יִשְׁמַע, יִשְׁלַח (not *yišmōʿ*); Piel yiqtol יְשַׁלַּח beside יְשַׁלֵּחַ;
- where a long vowel (*ē, î, ô, û*) must stay, a short "furtive" *pataḥ* is slipped in before the guttural and read first: שָׁמוֹעַ *šāmôaʿ*, הִשְׁמִיעַ *hišmîaʿ*, שֹׁלֵחַ *šōlēaḥ* (GKC §22, §65);
- the 2fs qatal gets a helping vowel: שָׁלַחַתְּ *šālaḥat*.

| | Qal | Niphal | Piel | Hiphil |
|---|---|---|---|---|
| qatal 3ms | שָׁמַע *šāmaʿ* | נִשְׁמַע | שִׁלַּח *šillaḥ* | הִשְׁמִיעַ *hišmîaʿ* |
| yiqtol 3ms | יִשְׁמַע *yišmaʿ* | יִשָּׁמַע | יְשַׁלַּח | יַשְׁמִיעַ |
| wayyiqtol 3ms | וַיִּשְׁמַע | | וַיְשַׁלַּח | וַיַּשְׁמַע |
| imperative | שְׁמַע *šəmaʿ* | | שַׁלַּח | הַשְׁמַע |
| infinitive absolute | שָׁמוֹעַ *šāmôaʿ* | | שַׁלֵּחַ | הַשְׁמֵעַ |
| infinitive construct | שְׁמֹעַ | | שַׁלַּח | הַשְׁמִיעַ |
| participle | שֹׁמֵעַ *šōmēaʿ* | נִשְׁמָע | מְשַׁלֵּחַ | מַשְׁמִיעַ |

**Attested in the Torah.** שָׁמַע 18× 3ms (DEU.1.45); וַיִּשְׁמַע 33× (DEU.1.34); יִשְׁמַע (DEU.18.19); imperative שְׁמַע 13× (DEU.4.1); infinitive absolute שָׁמֹעַ / שָׁמוֹעַ 4× each (DEU.1.16, 15.5); infinitive construct שְׁמֹעַ (DEU.5.25); Niphal נִשְׁמַע (DEU.4.32), יִשָּׁמַע (EXO.23.13); Hiphil הִשְׁמִיעֲךָ (DEU.4.36). Commonest III-ḥet/ʿayin verbs: לָקַח 393, שָׁמַע 241, שָׁלַח 201, יָדַע 174, נָסַע 120, שָׁבַע 70, נָגַע 54, זָבַח 41 (1 778 tokens, 81 lemmas).

> **DEU.6.4** · `wlc:DEU:197`
> שְׁמַע יִשְׂרָאֵל יְהוָה אֱלֹהֵינוּ יְהוָה אֶחָד
> *šəmaʿ yiśrāʾēl YHWH ʾĕlōhênû YHWH ʾeḥāḏ* — hear Israel YHWH our-God YHWH one — "Hear, O Israel: the LORD our God, the LORD is one." (JPS 1917) — the imperative of a III-ʿayin verb, with *a* where the strong verb has *ō* (קְטֹל).

> **DEU.1.34** · `wlc:DEU:33`
> וַיִּשְׁמַע יְהוָה אֶת־קוֹל דִּבְרֵיכֶם וַיִּקְצֹף וַיִּשָּׁבַע לֵאמֹר
> *wayyišmaʿ YHWH ʾeṯ-qôl diḇrêḵem wayyiqṣōp̄ wayyiššāḇaʿ lēʾmōr* — and-heard YHWH OM-the-voice-of your-words and-was-angry and-swore saying — "And the LORD heard the voice of your words, and was wroth, and swore, saying:" (JPS 1917) — Qal and Niphal III-ʿayin wayyiqtols around a strong verb.

### 4.2 III-aleph

A final א closes no syllable: it goes silent at the end of a word and before consonantal endings, and the vowel before it lengthens (GKC §74):

- qatal 3ms מָצָא *māṣāʾ* (the *a* is long because the syllable is open);
- before *-tî, -tā* etc. the qatal has *ā* in the Qal (מָצָאתִי *māṣāʾṯî*) but *ē* in the other stems (נִמְצֵאתִי, הוֹצֵאתִי);
- the Qal yiqtol has *ā*: יִמְצָא *yimṣāʾ*;
- the feminine plural yiqtol ends in -ֶאנָה (תִּמְצֶאנָה);
- the infinitive construct of a few verbs has a feminine ending: קְרֹאת, מְלֹאת (GKC §74).

| | Qal | Niphal | Piel | Hiphil |
|---|---|---|---|---|
| qatal 3ms | מָצָא | נִמְצָא | מִלֵּא | הִמְצִיא |
| qatal 1cs | מָצָאתִי *māṣāʾṯî* | נִמְצֵאתִי | מִלֵּאתִי | הִמְצֵאתִי |
| yiqtol 3ms | יִמְצָא | יִמָּצֵא | יְמַלֵּא | יַמְצִיא |
| wayyiqtol 3ms | וַיִּמְצָא | וַיִּמָּצֵא | וַיְמַלֵּא | וַיַּמְצֵא |
| imperative | מְצָא | הִמָּצֵא | מַלֵּא | הַמְצֵא |
| infinitive construct | מְצֹא | הִמָּצֵא | מַלֵּא | הַמְצִיא |
| participle | מֹצֵא | נִמְצָא | מְמַלֵּא | מַמְצִיא |

**Attested in the Torah.** מָצָא 13× (DEU.24.1); מָצָאתִי 11× (DEU.22.14); וַיִּמְצָא 6× (GEN.16.7, with suffix וַיִּמְצָאָהּ); יִמְצָא (DEU.22.25); infinitive construct מְצֹא (GEN.19.11); Niphal נִמְצָא (EXO.35.23), יִמָּצֵא 13× (DEU.17.2). Commonest III-aleph verbs: בּוֹא 617, יָצָא 347, קָרָא 200, נָשָׂא 168, מָצָא 122, טָמֵא 114, יָרֵא 80, חָטָא 71, מָלֵא 58 (1 938 tokens, 31 lemmas).

> **GEN.16.7** · `wlc:GEN:388`
> וַיִּמְצָאָהּ מַלְאַךְ יְהוָה עַל־עֵין הַמַּיִם בַּמִּדְבָּר
> *wayyimṣāʾāh malʾaḵ YHWH ʿal-ʿên hammayim bammiḏbār* — and-found-her the-angel-of YHWH by-the-spring-of the-water in-the-wilderness — "And the angel of the LORD found her by a fountain of water in the wilderness." (JPS 1917)

## 5. I-nun

### 5.1 Recognition

A *nun* that closes a syllable is assimilated to the following consonant, which is then doubled: *yinpōl* → יִפֹּל *yippōl* (GKC §66; §19). The *nun* therefore disappears wherever a prefix stands before it — in the Qal yiqtol and wayyiqtol, the whole Niphal except the qatal and participle where it is the stem's own נ, and the whole Hiphil and Hophal — and the sign of the root is a **dagesh in the second radical right after the prefix**. In the Qal imperative and infinitive construct of verbs with *a* in the yiqtol the *nun* falls away entirely (aphaeresis): גַּשׁ "approach!", infinitive גֶּשֶׁת (GKC §66). Verbs with a guttural second radical keep the *nun* (יִנְחַל "inherit").

| | Qal (yiqtol *ō*) נָפַל | Qal (yiqtol *a*) נָגַשׁ | Niphal | Hiphil | Hophal |
|---|---|---|---|---|---|
| qatal 3ms | נָפַל | נָגַשׁ (not in Qal; Niphal נִגַּשׁ) | נִגַּשׁ *niggaš* | הִפִּיל *hippîl*, הִגִּישׁ | הֻגַּשׁ *huggaš* |
| yiqtol 3ms | יִפֹּל *yippōl* | יִגַּשׁ *yiggaš* | יִנָּגֵשׁ | יַפִּיל *yappîl* | יֻגַּשׁ |
| wayyiqtol 3ms | וַיִּפֹּל | וַיִּגַּשׁ | | וַיַּפֵּל | |
| imperative | נְפֹל | גַּשׁ, גְּשָׁה | הִנָּגֵשׁ | הַגֵּשׁ | — |
| infinitive construct | נְפֹל | גֶּשֶׁת | הִנָּגֵשׁ | הַגִּישׁ | |
| participle | נֹפֵל | נֹגֵשׁ | נִגָּשׁ | מַפִּיל, מַגִּישׁ | מֻגָּשׁ |

**Attested in the Torah.** נָפַל (EXO.21.33); וַיִּפֹּל 10× (EXO.32.28); יִפֹּל 5× (DEU.22.8); Hiphil וַיַּפֵּל (GEN.2.21 "caused a deep sleep to fall"), הִפִּיל (DEU.25.2); Hithpael הִתְנַפַּלְתִּי (DEU.9.25). נגשׁ: וַיִּגַּשׁ 5× (GEN.18.23), imperative גְּשָׁה / גֶּשׁ (GEN.27.21, 19.9), infinitive גִּשְׁתָּם (EXO.28.43), Niphal נִגַּשׁ (EXO.20.21), Hiphil וַיַּגֵּשׁ (GEN.48.10), imperative הַגִּשָׁה (GEN.27.25).

> **EXO.32.28** · `wlc:EXO:933`
> וַיִּפֹּל מִן־הָעָם בַּיּוֹם הַהוּא כִּשְׁלֹשֶׁת אַלְפֵי אִישׁ
> *wayyippōl min-hāʿām bayyôm hahûʾ kišlōšeṯ ʾalp̄ê ʾîš* — and-fell from-the-people on-the-day the-that about-three thousand man — "and there fell of the people that day about three thousand men." (JPS 1917)

### 5.2 נָתַן "give"

The commonest I-nun verb (645 Torah tokens) assimilates its *first* nun and also its *last*: the third radical נ assimilates to the *t* of the qatal endings (נָתַתִּי *nāṯattî* < *nāṯantî*). It is the only I-nun verb with *ē* in the yiqtol (יִתֵּן), and its imperative is תֵּן / תְּנָה, its infinitive construct תֵּת (< *tint*) (GKC §66).

| form | נָתַן | Torah (2026-10-17) |
|---|---|---|
| qatal 3ms | נָתַן | 47× (DEU.1.21) |
| qatal 1cs | נָתַתִּי *nāṯattî* | 44× (DEU.1.8) |
| qatal 2ms | נָתַתָּה / נָתַתָּ | נָתַתָּה (DEU.26.10) |
| yiqtol 3ms | יִתֵּן *yittēn* | 36× (DEU.5.29) |
| yiqtol 1cs | אֶתֵּן | 19× (DEU.1.36) |
| cohortative | אֶתְּנָה | 6× (EXO.24.12) |
| wayyiqtol 3ms | וַיִּתֵּן / וַיִּתֶּן | 62× / 13× (DEU.3.3, DEU.26.9) |
| imperative | תֵּן, תֶּן־, תְּנָה | תֶּן־לִי (GEN.14.21) |
| infinitive construct | תֵּת, with לְ: לָתֵת | תֵּת 31×, תֶת 11× (DEU.1.8, DEU.4.38) |
| infinitive absolute | נָתוֹן / נָתֹן | נָתוֹן תִּתֵּן (DEU.15.10) |
| Niphal | נִתַּן, יִנָּתֵן | (LEV.19.20; EXO.5.18) |
| Qal passive / Hophal | יֻתַּן | 3× (LEV.11.38) — read by GKC as a Qal passive (§53 u) |

> **DEU.1.8** · `wlc:DEU:7`
> רְאֵה נָתַתִּי לִפְנֵיכֶם אֶת־הָאָרֶץ … לָתֵת לָהֶם וּלְזַרְעָם אַחֲרֵיהֶם
> *rəʾē nāṯattî lip̄nêḵem ʾeṯ-hāʾāreṣ … lāṯēṯ lāhem ûləzarʿām ʾaḥărêhem* — see I-have-set before-you OM-the-land … to-give to-them and-to-their-seed after-them — "Behold, I have set the land before you … to give unto them and to their seed after them." (JPS 1917)

> **GEN.14.21** · `wlc:GEN:357`
> תֶּן־לִי הַנֶּפֶשׁ וְהָרְכֻשׁ קַח־לָךְ
> *ten-lî hannep̄eš wəhārəḵuš qaḥ-lāḵ* — give-to-me the-persons and-the-goods take-for-yourself — "Give me the persons, and take the goods to thyself." (JPS 1917) — the imperatives of נָתַן and לָקַח side by side, both without their first radical.

### 5.3 לָקַח "take"

לָקַח (393) treats its ל like the *nun* of a I-nun verb: yiqtol יִקַּח, imperative קַח, infinitive construct קַחַת (GKC §66 g). The Qal passive יֻקַּח (GEN.18.4) and תֻּקַּח (GEN.12.15) also belong here.

| form | Torah (2026-10-17) |
|---|---|
| qatal לָקַח | 12× 3ms (DEU.3.14); weqatal וְלָקַחְתָּ 26× (DEU.7.25) |
| wayyiqtol וַיִּקַּח | 86× (+ pausal וַיִּקָּח 5×) (EXO.2.1) |
| yiqtol יִקַּח | 18× (DEU.10.17) |
| imperative קַח | 31× (EXO.7.9) |
| infinitive קַחַת | 9× (DEU.4.34) |
| Pual לֻקַּח | GEN.3.23 "from which he was taken" |

> **EXO.2.1** · `wlc:EXO:22`
> וַיֵּלֶךְ אִישׁ מִבֵּית לֵוִי וַיִּקַּח אֶת־בַּת־לֵוִי
> *wayyēleḵ ʾîš mibbêṯ lēwî wayyiqqaḥ ʾeṯ-baṯ-lēwî* — and-went a-man from-the-house-of Levi and-took OM-a-daughter-of-Levi — "And there went a man of the house of Levi, and took to wife a daughter of Levi." (JPS 1917) — both verbs have lost their first radical: הלך behaves as I-yod, לקח as I-nun.

Commonest I-nun verbs in the Torah: נָתַן 645, נָשָׂא 168, נָסַע 120, נָכָה 99, נָפַל 58, נָגַד 55 (Hiphil הִגִּיד "tell"), נָגַע 54, נָטָה 50, נָגַשׁ 45 (1 790 tokens, 78 lemmas; לָקַח, not I-nun by its letters, is counted with III-guttural).

## 6. I-yod and I-waw

### 6.1 Three groups

Most verbs that begin with י were originally I-waw (the *w* is still seen in the Niphal and Hiphil: נוֹלַד, הוֹשִׁיב); a few are true I-yod (יָטַב, יָנַק, יָקַץ, יָשַׁר); and a few assimilate the *y* like a *nun* (יָצַק "pour", יָצַר "form", יָצַת) (GKC §§69–71).

**Group 1 (original I-waw), like יָשַׁב.** In the Qal yiqtol the first radical drops and the prefix has *ē*: יֵשֵׁב *yēšēḇ*; imperative שֵׁב, infinitive construct שֶׁבֶת (a segolate with feminine *t*). In the Niphal and Hiphil the old *w* comes back as a vowel *ô*: נוֹשַׁב, הוֹשִׁיב; in the Hophal as *û*: הוּשַׁב (GKC §69). The verbs that drop the first radical this way are יָלַד, יָצָא, יָשַׁב, יָרַד, and also הָלַךְ (GKC §69, x).

**Group 2 (original I-yod), like יָטַב.** The *y* stays as a vowel letter: Qal yiqtol יִיטַב *yîṭaḇ*, Hiphil הֵיטִיב *hêṭîḇ*, yiqtol יֵיטִיב (GKC §70).

**Group 3, יָרַשׁ, יָרֵא, יָדַע, יָכֹל** have mixed behaviour: יָרַשׁ has yiqtol יִירַשׁ (group 2-like) but imperative רֵשׁ and infinitive רֶשֶׁת (group 1-like); יָדַע has yiqtol יֵדַע and infinitive דַּעַת; יָכֹל has yiqtol יוּכַל (GKC §69, r; §70).

### 6.2 Paradigm

| | Qal (I-waw) יָשַׁב | Qal (I-yod) יָטַב | Niphal | Hiphil (I-waw) | Hiphil (I-yod) | Hophal |
|---|---|---|---|---|---|---|
| qatal 3ms | יָשַׁב | יָטַב | נוֹשַׁב *nôšaḇ* | הוֹשִׁיב *hôšîḇ* | הֵיטִיב *hêṭîḇ* | הוּשַׁב *hûšaḇ* |
| yiqtol 3ms | יֵשֵׁב *yēšēḇ* | יִיטַב *yîṭaḇ* | יִוָּשֵׁב *yiwwāšēḇ* | יוֹשִׁיב | יֵיטִיב | יוּשַׁב |
| wayyiqtol 3ms | וַיֵּשֶׁב *wayyēšeḇ* | וַיִּיטַב | וַיִּוָּשֵׁב | וַיּוֹשֶׁב | וַיֵּיטֶב | |
| imperative | שֵׁב, שְׁבָה | — | הִוָּשֵׁב | הוֹשֵׁב | הֵיטֵב | — |
| infinitive construct | שֶׁבֶת, with suffix שִׁבְתּ־ | — | הִוָּשֵׁב | הוֹשִׁיב | הֵיטִיב | |
| infinitive absolute | יָשׁוֹב | | | הוֹשֵׁב | הֵיטֵב | |
| participle | יֹשֵׁב | | נוֹשָׁב | מוֹשִׁיב | מֵיטִיב | מוּשָׁב |

In the Niphal yiqtol and imperative the *w* reappears as a doubled consonant (יִוָּלֵד, הִוָּשֵׁב) (GKC §69).

**Attested in the Torah (2026-10-17).**

- יָשַׁב (199): יָשַׁב (GEN.13.12), וַיֵּשֶׁב 27× (EXO.2.15), יֵשֵׁב 5× (DEU.23.17), imperative שְׁבָה / שֵׁב / שֵׁב (GEN.27.19, GEN.20.15, GEN.35.1), infinitive שֶׁבֶת 12× (DEU.1.6) and שִׁבְתְּךָ 7× (DEU.6.7), Hiphil הוֹשֵׁב (GEN.47.6), וַיּוֹשֵׁב (GEN.47.11).
- יָלַד (205): Qal יָלַד 11×, יָלְדָה; infinitive לֶדֶת / לִדְתָּהּ (GEN.25.24, 38.27); Hiphil וַיּוֹלֶד 37× (GEN.5.3) and infinitive הוֹלִיד 17× (GEN.5.4) — "begot", the backbone of the genealogies; Niphal נוֹלַד (GEN.21.3), וַיִּוָּלֵד (GEN.4.18); Pual יֻלַּד 7× (GEN.4.26).
- יָצָא (347, also III-aleph): וַיֵּצֵא 40× (DEU.1.44), יֵצֵא 13×, imperative צֵא (EXO.11.8), infinitive צֵאת 30× (DEU.4.45); Hiphil הוֹצִיא 16× (DEU.1.27), הוֹצֵאתִי 12× (DEU.5.6), וַיּוֹצֵא / וַיּוֹצִא (EXO.19.17, DEU.4.20), imperative הוֹצֵא (EXO.3.10) beside the rare הַיְצֵא (GEN.8.17, Ketiv) (GKC §69).
- יָדַע (174): יָדַעְתִּי 20× (DEU.3.19), וַיֵּדַע 6× (EXO.2.25), yiqtol תֵּדַע 8×, imperative דַּע (GEN.20.7), infinitive דַּעַת 8× (DEU.4.35); Niphal נוֹדַע (DEU.21.1), יִוָּדַע (EXO.33.16); Hiphil הוֹדִיעַ (GEN.41.39), imperative הוֹדִעֵנִי (EXO.33.13); Hithpael הִתְוַדַּע (GEN.45.1).
- יָטַב (38): Qal יִיטַב 10× (DEU.4.40), וַיִּיטַב 4× (DEU.1.23); Hiphil הֵיטִיב (GEN.12.16), וַיֵּיטֶב (EXO.1.20), cohortative אֵיטִיבָה (GEN.32.10), infinitive absolute הֵיטֵב 6× (DEU.9.21).
- יָרַשׁ (101): Qal יִירַשׁ (GEN.21.10), imperative רֵשׁ / רָשׁ (DEU.1.21, 2.24), infinitive רִשְׁתָּהּ 27× (DEU.3.18) and רֶשֶׁת 9×; Hiphil הוֹרִישׁ (DEU.4.38), וַיּוֹרֶשׁ (NUM.21.32).
- יָרֵא (80, stative and III-aleph): יָרֵא (DEU.25.18), יִרְאָה (infinitive, DEU.4.10), אַל־תִּירָא 6× (DEU.1.21), וַיִּירָא (EXO.2.14), וָאִירָא (GEN.3.10).

> **GEN.27.19** · `wlc:GEN:746`
> קוּם־נָא שְׁבָה וְאָכְלָה מִצֵּידִי
> *qûm-nāʾ šəḇâ wəʾoḵlâ miṣṣêḏî* — rise-please sit and-eat from-my-game — "Arise, I pray thee, sit and eat of my venison." (JPS 1917) — the I-waw imperative שְׁבָה with paragogic *-â*, after a hollow imperative.

> **DEU.4.40** · `wlc:DEU:151`
> אֲשֶׁר יִיטַב לְךָ וּלְבָנֶיךָ אַחֲרֶיךָ
> *ʾăšer yîṭaḇ ləḵā ûləḇānêḵā ʾaḥărêḵā* — that it-may-be-good for-you and-for-your-children after-you — "that it may go well with thee, and with thy children after thee." (JPS 1917) — the true I-yod verb keeps its *y* as *î*.

> **GEN.12.16** · `wlc:GEN:314`
> וּלְאַבְרָם הֵיטִיב בַּעֲבוּרָהּ
> *ûləʾaḇrām hêṭîḇ baʿăḇûrāh* — and-to-Abram he-did-good for-her-sake — "And he dealt well with Abram for her sake." (JPS 1917)

> **DEU.5.6** · `wlc:DEU:166`
> אָנֹכִי יְהוָה אֱלֹהֶיךָ אֲשֶׁר הוֹצֵאתִיךָ מֵאֶרֶץ מִצְרַיִם
> *ʾānōḵî YHWH ʾĕlōheḵā ʾăšer hôṣēʾṯîḵā mēʾereṣ miṣrayim* — I YHWH your-God who I-brought-you-out from-the-land-of Egypt — "I am the LORD thy God, who brought thee out of the land of Egypt." (JPS 1917) — Hiphil of a I-waw + III-aleph root: *ô* from the old *w*, *ē* before the ending from the silent א.

Commonest I-yod/waw verbs in the Torah: יָצָא 347, יָלַד 205, יָשַׁב 199, יָדַע 174, יָרַשׁ 101, יָרַד 88, יָרֵא 80, יָכֹל 64, יָסַף 58, יָטַב 38 (1 560 tokens, 48 lemmas).

## 7. Hollow verbs: II-waw and II-yod

### 7.1 Recognition

In hollow roots the middle radical is a long vowel, not a consonant. The citation form is the infinitive construct (קוּם, שִׂים, בּוֹא) because the qatal 3ms (קָם) shows only two letters (GKC §72). Signs: a two-letter stem with a long vowel (קָם, יָקוּם, הֵקִים), a prefix with *ā* in the Qal yiqtol (יָקוּם) and *ē* in the Hiphil qatal (הֵקִים), and a Niphal with *ô* (נָכוֹן). The D stems are replaced by Polel / Polal / Hithpolel (קוֹמֵם, רוֹמֵם; [06-verb.md §2.9](06-verb.md#29-the-minor-stems)).

| | Qal (*û*) קוּם | Qal (*î*) שִׂים | Qal (*ô*) בּוֹא | Qal stative מוּת | Niphal כּוּן | Hiphil | Hophal |
|---|---|---|---|---|---|---|---|
| qatal 3ms | קָם *qām* | שָׂם *śām* | בָּא *bāʾ* | מֵת *mēṯ* | נָכוֹן *nāḵôn* | הֵקִים *hēqîm* | הוּקַם *hûqam* |
| qatal 1cs | קַמְתִּי | שַׂמְתִּי | בָּאתִי | מַתִּי | נְכוּנוֹתִי | הֲקִימוֹתִי *hăqîmôṯî* | הוּקַמְתִּי |
| yiqtol 3ms | יָקוּם *yāqûm* | יָשִׂים | יָבוֹא | יָמוּת | יִכּוֹן | יָקִים *yāqîm* | יוּקַם |
| jussive 3ms | יָקֹם *yāqōm* | יָשֵׂם | יָבֹא | יָמֹת | | יָקֵם *yāqēm* | |
| wayyiqtol 3ms | וַיָּקָם *wayyāqom* | וַיָּשֶׂם | וַיָּבֹא | וַיָּמָת | | וַיָּקֶם *wayyāqem* | |
| imperative | קוּם, קוּמָה | שִׂים | בּוֹא | מוּת | הִכּוֹן | הָקֵם *hāqēm* | — |
| infinitive construct | קוּם | שׂוּם / שִׂים | בּוֹא | מוּת | הִכּוֹן | הָקִים | |
| infinitive absolute | קוֹם | שׂוֹם | בּוֹא | מוֹת | | הָקֵם | |
| participle | קָם *qām* | שָׂם | בָּא | מֵת | נָכוֹן | מֵקִים | מוּקָם |

Notes: the Qal participle has the same form as the qatal 3ms (קָם "he rose" / "rising"); the context and the accent decide (GKC §72). In the Hiphil qatal 1st and 2nd persons, and in the Niphal, a connecting vowel *-ô-* is inserted before the ending (הֲקִימֹתִי, הֲשִׁבֹתִי) (GKC §72). The wayyiqtol pulls the stress back and shortens the vowel: וַיָּ֫קָם (with qamets hatuf, *wayyāqom*), Hiphil וַיָּ֫קֶם (GKC §72, aa).

### 7.2 Attested in the Torah (2026-10-17)

- **בּוֹא "come"** (617; Qal 427, Hiphil 179, Hophal 11). Qal בָּא, וַיָּבֹא, יָבוֹא; Hiphil "bring": הֵבִיא 3× (GEN.4.4), וְהֵבִיא 19× (LEV.4.4), וַיָּבֵא 18× (EXO.4.6), יָבִיא / יְבִיא (LEV.4.32, DEU.6.10), הֵבֵאתִי (DEU.26.10), imperative הָבִיאוּ (EXO.32.2), infinitive הָבִיא 9× (DEU.6.23), participle מֵבִיא (EXO.10.4).
- **מוּת "die"** (303; Qal 239, Hophal 43, Hiphil 21). מֵת 13× (DEU.10.6) and weqatal 18×; וַיָּמָת 24× and וַיָּמֹת 16× (DEU.34.5, GEN.5.5); יָמוּת 20× (DEU.20.5); infinitive absolute מוֹת 31× (EXO.19.12); infinitive construct מוּת 12× (DEU.2.16); cohortative אָמוּתָה (GEN.46.30); Hiphil אָמִית (DEU.32.39), infinitive הָמִית / הֲמִית (EXO.16.3, DEU.13.10); Hophal יוּמַת / יוּמָת 33× (DEU.13.6), the death penalty of the laws.
- **שׁוּב "return"** (180; Qal 113, Hiphil 63, Hophal 4). וַיָּשָׁב 18× (EXO.4.18); יָשׁוּב 9× (DEU.13.18); imperative שׁוּב (EXO.32.12); cohortative אָשׁוּבָה (EXO.4.18); Hiphil הֵשִׁיב (EXO.34.35), וַיָּשֶׁב (EXO.4.7), imperative הָשֵׁב (EXO.4.7), infinitive absolute הָשֵׁב 5× (DEU.22.1), וַהֲשֵׁבֹתָ (DEU.4.39).
- **שִׂים "put"** (152). שָׂם 10× (DEU.4.44), וַיָּשֶׂם 33× (EXO.9.5), יָשִׂים (GEN.30.42), imperative שִׂים 8× (EXO.17.14), jussive וְיָשֵׂם (NUM.6.26).
- **קוּם "rise"** (145). וַיָּקָם 24× (EXO.1.8), יָקוּם 15× (DEU.13.2), imperative קוּם 13× (DEU.9.12), jussive יָקֻם (GEN.27.31); Hiphil הֵקִים (NUM.30.15), וַיָּקֶם (EXO.40.18), imperative הָקֵם (GEN.38.8), infinitive הָקִים (DEU.8.18), וַהֲקִמֹתִי (GEN.9.11); Hophal הוּקַם (EXO.40.17).

> **EXO.1.8** · `wlc:EXO:7`
> וַיָּקָם מֶלֶךְ־חָדָשׁ עַל־מִצְרָיִם אֲשֶׁר לֹא־יָדַע אֶת־יוֹסֵף
> *wayyāqom meleḵ-ḥāḏāš ʿal-miṣrāyim ʾăšer lōʾ-yāḏaʿ ʾeṯ-yôsēp̄* — and-arose a-king new over-Egypt who not-knew OM-Joseph — "Now there arose a new king over Egypt, who knew not Joseph." (JPS 1917)

> **DEU.34.5** · `wlc:DEU:951`
> וַיָּמָת שָׁם מֹשֶׁה עֶבֶד־יְהוָה בְּאֶרֶץ מוֹאָב עַל־פִּי יְהוָה
> *wayyāmoṯ šām mōše ʿeḇeḏ-YHWH bəʾereṣ môʾāḇ ʿal-pî YHWH* — and-died there Moses servant-of-YHWH in-the-land-of Moab by-the-mouth-of YHWH — "So Moses the servant of the LORD died there in the land of Moab, according to the word of the LORD." (JPS 1917)

> **GEN.38.8** · `wlc:GEN:1127`
> בֹּא אֶל־אֵשֶׁת אָחִיךָ וְיַבֵּם אֹתָהּ וְהָקֵם זֶרַע לְאָחִיךָ
> *bōʾ ʾel-ʾēšeṯ ʾāḥîḵā wəyabbēm ʾōṯāh wəhāqēm zeraʿ ləʾāḥîḵā* — come to-the-wife-of your-brother and-do-levirate-duty her and-raise-up seed for-your-brother — "Go in unto thy brother's wife, and perform the duty of a husband's brother unto her, and raise up seed to thy brother." (JPS 1917) — hollow Qal and Hiphil imperatives.

> **DEU.32.39** · `wlc:DEU:904`
> אֲנִי אָמִית וַאֲחַיֶּה מָחַצְתִּי וַאֲנִי אֶרְפָּא
> *ʾănî ʾāmîṯ waʾăḥayye māḥaṣtî waʾănî ʾerpāʾ* — I I-kill and-I-make-alive I-wounded and-I I-heal — "I kill, and I make alive; I have wounded, and I heal." (JPS 1917) — a hollow Hiphil (אָמִית), a III-he Piel (אֲחַיֶּה), a III-aleph Qal (אֶרְפָּא).

Commonest hollow verbs in the Torah: בּוֹא 617, מוּת 303, שׁוּב 180, שִׂים 152, קוּם 145, סוּר 55, רוּם 45, גּוּר 37, נוּס 32, זוּב 30 (1 972 tokens, 89 lemmas by our count, which uses the shape of the lemma; 2026-10-17).

## 8. III-he

### 8.1 Recognition

The final ה of these verbs is not a consonant but a vowel letter; the original third radical was *y* (or *w*), which comes back as a vowel before endings (GKC §75). Every form ends in a vowel or loses its last syllable. The endings are the same in all stems:

| form | ending | Qal of בָּנָה |
|---|---|---|
| qatal 3ms | ־ָה *-â* | בָּנָה *bānâ* |
| qatal 3fs | ־ְתָה *-əṯâ* | בָּנְתָה *bānəṯâ* |
| qatal 1st/2nd person | ־ִי־ *-î-* (Qal), ־ֵי־ *-ê-* (passive stems, often others) | בָּנִיתִי, בָּנִיתָ, בְּנִיתֶם |
| qatal 3cp | ־וּ *-û* | בָּנוּ |
| yiqtol (no ending) | ־ֶה *-e* | יִבְנֶה *yiḇne* |
| yiqtol fp | ־ֶינָה *-ênâ* | תִּבְנֶינָה |
| jussive / wayyiqtol | apocopated: last vowel dropped | יִבֶן, וַיִּבֶן *wayyiḇen* |
| imperative 2ms | ־ֵה *-ē* | בְּנֵה *bənē* |
| infinitive construct | ־וֹת *-ôṯ* | בְּנוֹת *bənôṯ* |
| infinitive absolute | ־ֹה *-ō* | בָּנֹה |
| participle active ms / fs | ־ֶה / ־ָה | בֹּנֶה, בֹּנָה |
| participle passive | ־וּי | בָּנוּי |

### 8.2 The apocopated forms

The short forms (jussive and wayyiqtol) drop the final *-e*; the remaining two consonants then often take a helping vowel, as a segolate does (GKC §75):

| long yiqtol | short form | example |
|---|---|---|
| יִבְנֶה | וַיִּבֶן *wayyiḇen* | EXO.1.11 (15× in the Torah) |
| יַעֲשֶׂה | וַיַּעַשׂ *wayyaʿaś*, jussive תַּעַשׂ | וַיַּעַשׂ 82× (EXO.1.21); אַל־תַּעַשׂ (GEN.22.12) |
| יִרְאֶה | וַיַּרְא *wayyarʾ* | 74× (DEU.26.7) |
| יִהְיֶה | וַיְהִי *wayhî*, jussive יְהִי *yəhî* | וַיְהִי 187×, יְהִי (GEN.1.3) |
| יִחְיֶה | וַיְחִי *wayḥî* | 33× (GEN.5.3) |
| יַעֲלֶה | וַיַּעַל *wayyaʿal* | 19× (DEU.34.1) |
| Hiphil יַכֶּה | וַיַּךְ *wayyaḵ* | 8× (EXO.2.12) |
| יִטֶּה | וַיֵּט *wayyēṭ* | 14× (EXO.8.2) |
| Piel יְצַוֶּה | וַיְצַו *waysaw* | 22× (DEU.27.1) |

So the commonest verbs of narrative — וַיְהִי, וַיַּרְא, וַיַּעַשׂ, וַיַּעַל, וַיְצַו — are all short III-he forms.

### 8.3 Paradigm

| | Qal | Niphal | Piel | Hiphil | Hophal |
|---|---|---|---|---|---|
| qatal 3ms | גָּלָה *gālâ* | נִגְלָה | גִּלָּה | הִגְלָה | הָגְלָה |
| qatal 3fs | גָּלְתָה | נִגְלְתָה | גִּלְּתָה | הִגְלְתָה | הָגְלְתָה |
| qatal 1cs | גָּלִיתִי | נִגְלֵיתִי | גִּלִּיתִי | הִגְלֵיתִי | הָגְלֵיתִי |
| qatal 3cp | גָּלוּ | נִגְלוּ | גִּלּוּ | הִגְלוּ | הָגְלוּ |
| yiqtol 3ms | יִגְלֶה | יִגָּלֶה | יְגַלֶּה | יַגְלֶה | יָגְלֶה |
| short / wayyiqtol | וַיִּגֶל | וַיִּגָּל | וַיְגַל | וַיֶּגֶל | |
| imperative | גְּלֵה | הִגָּלֵה | גַּלֵּה | הַגְלֵה | — |
| infinitive construct | גְּלוֹת | הִגָּלוֹת | גַּלּוֹת | הַגְלוֹת | הָגְלוֹת |
| participle | גֹּלֶה; pass. גָּלוּי | נִגְלֶה | מְגַלֶּה | מַגְלֶה | מָגְלֶה |

(GKC §75, Paradigm P.)

### 8.4 Attested in the Torah (2026-10-17)

- **עָשָׂה "do, make"** (860; also I-guttural): עָשָׂה 85× (DEU.1.30), weqatal וְעָשִׂיתָ 70× (DEU.3.2), עָשִׂיתִי (DEU.10.5); וַיַּעַשׂ 82× (EXO.1.21); yiqtol תַּעֲשֶׂה 72× (DEU.12.14), יַעֲשֶׂה 31×, אֶעֱשֶׂה 26×; imperative עֲשֵׂה 16× (EXO.25.19); infinitive עֲשׂוֹת 74× / עֲשֹׂת 20× (DEU.1.14); Niphal יֵעָשֶׂה 17× (DEU.25.9).
- **הָיָה "be"** (1 049; also I-guttural): הָיָה 64× qatal, 139× weqatal; יִהְיֶה 206× (DEU.1.39); וַיְהִי 187×; jussive יְהִי (GEN.1.3); infinitive הֱיוֹת / לִהְיוֹת 20× (DEU.4.20); imperative הֱיֵה (EXO.24.12, GEN.12.2 וֶהְיֵה).
- **רָאָה "see"** (396; also II-resh, III-he; I-guttural ר is not a guttural for doubling but behaves like one): וַיַּרְא 74×; רָאָה 7× qatal, 26× weqatal; רָאִיתִי 10×; imperative רְאֵה 22× (DEU.1.8); infinitive רְאוֹת 21× (DEU.29.3); Niphal "appear" נִרְאָה 9× (EXO.3.16), וַיֵּרָא 13× (DEU.31.15), יֵרָאֶה 8× (DEU.16.4); Hiphil "show" הֶרְאָה (EXO.27.8), אַרְאֶךָּ (GEN.12.1), Hophal מָרְאֶה (EXO.25.40).
- **צָוָה "command"** (252, Piel): צִוָּה 110× + with suffix צִוָּהוּ 15× (DEU.1.3, GEN.7.5); וַיְצַו 22× (DEU.27.1); imperative צַו 8× (DEU.2.4); Pual צֻוָּה (NUM.3.16), צֻוֵּיתִי (LEV.8.35).
- **עָלָה "go up"** (201): Qal עָלָה, וַיַּעַל 19×, imperative עֲלֵה 10× (DEU.1.21); Hiphil "offer up, bring up" הֶעֱלָה, וַיַּעַל 6× (EXO.40.25), הַעֲלֹת (EXO.3.8), imperative הַעֲלֵהוּ (GEN.22.2).
- **בָּנָה "build"**: בָּנָה, וַיִּבֶן 15× (EXO.1.11), imperative בְּנֵה (NUM.23.1), Niphal אִבָּנֶה "I shall be built up" (GEN.16.2).

> **EXO.1.11** · `wlc:EXO:10`
> וַיִּבֶן עָרֵי מִסְכְּנוֹת לְפַרְעֹה אֶת־פִּתֹם וְאֶת־רַעַמְסֵס
> *wayyiḇen ʿārê misk(ə)nôṯ ləp̄arʿō ʾeṯ-piṯōm wəʾeṯ-raʿamsēs* — and-he-built cities-of storage for-Pharaoh OM-Pithom and-OM-Raamses — "And they built for Pharaoh store-cities, Pithom and Raamses." (JPS 1917; the Hebrew verb is singular, "the people" being the subject)

> **EXO.2.12** · `wlc:EXO:33`
> וַיִּפֶן כֹּה וָכֹה וַיַּרְא כִּי אֵין אִישׁ וַיַּךְ אֶת־הַמִּצְרִי
> *wayyip̄en kō wāḵō wayyarʾ kî ʾên ʾîš wayyaḵ ʾeṯ-hammiṣrî* — and-he-turned thus and-thus and-saw that there-was-no man and-struck OM-the-Egyptian — "And he looked this way and that way, and when he saw that there was no man, he smote the Egyptian." (JPS 1917) — three apocopated III-he wayyiqtols in one verse: וַיִּפֶן (פנה), וַיַּרְא (ראה), וַיַּךְ (נכה, Hiphil, also I-nun).

Commonest III-he verbs: הָיָה 1 049, עָשָׂה 860, רָאָה 396, צָוָה 252, עָלָה 201, נָכָה 99, חָנָה 86, חָיָה 83, עָנָה 75, רָבָה 63, נָטָה 50, שָׁתָה 50, כָּסָה 47, כָּלָה 46, חוה 45 (4 136 tokens, 110 lemmas). III-he is, by tokens, the largest weak class of the Torah.

## 9. Geminate verbs (II = III)

When the second and third radicals are the same letter (סבב, ארר, תמם, קלל), the two often merge into one, doubled where a vowel follows (סַבּוּ) and simple at the end of a word (סַב) (GKC §67). Signs:

- Qal qatal 3ms either full (סָבַב, statives תַּם, קַל) or short;
- Qal yiqtol with *ā* in the prefix and *ō* (יָסֹב) or, the "Aramaic" type, with doubled first radical (יִסֹּב, יִתֹּם) (GKC §67);
- Hiphil qatal הֵסֵב, yiqtol יָסֵב; Niphal נָסַב;
- D stems replaced by Poel / Pilpel (סוֹבֵב, יְסֹבְבֶנְהוּ, גִּלְגֵּל).

| | Qal | Niphal | Hiphil | Poel |
|---|---|---|---|---|
| qatal 3ms | סָבַב / סַב | נָסַב | הֵסֵב | סוֹבֵב |
| qatal 3cp | סָבְבוּ / סַבּוּ | נָסַבּוּ | הֵסֵבּוּ | |
| yiqtol 3ms | יָסֹב / יִסֹּב | יִסַּב | יָסֵב | יְסוֹבֵב |
| wayyiqtol 3ms | וַיָּסָב / וַיִּסֹּב | | וַיַּסֵּב | |
| imperative | סֹב | | הָסֵב | |
| infinitive construct | סֹב | הִסֵּב | הָסֵב | |
| participle | סֹבֵב | נָסָב | מֵסֵב | מְסוֹבֵב |

**Attested in the Torah (2026-10-17).** סָבַב (15 tokens): וַיִּסֹּב (GEN.42.24), וַנָּסָב (DEU.2.1), infinitive סְבֹב / סֹב (NUM.21.4, DEU.2.3), Hiphil וַיַּסֵּב (EXO.13.18), Niphal וְנָסַב (NUM.34.4), Poel יְסֹבְבֶנְהוּ (DEU.32.10). אָרַר "curse" (41): passive participle אָרוּר (GEN.3.14, DEU.28.16–19), yiqtol אָאֹר (GEN.12.3), imperative אָרָה (NUM.22.6), Hophal/Qal passive יוּאָר (NUM.22.6). תָּמַם "be finished": תַּם (GEN.47.18), תַּמּוּ (DEU.2.16), וַיִּתֹּם (GEN.47.15), infinitive תֹּם (DEU.2.14). קָלַל "be light": קַלּוּ (GEN.8.8), וָאֵקַל (GEN.16.5); Piel "curse" קִלֵּל (LEV.20.9), יְקַלֵּל, וַיְקַלֵּל (LEV.24.11).

> **GEN.42.24** · `wlc:GEN:1276`
> וַיִּסֹּב מֵעֲלֵיהֶם וַיֵּבְךְּ וַיָּשָׁב אֲלֵהֶם
> *wayyissōḇ mēʿălêhem wayyēḇk wayyāšoḇ ʾălēhem* — and-he-turned from-them and-wept and-returned to-them — "And he turned himself about from them, and wept; and he returned to them." (JPS 1917) — geminate (סבב), III-he (בכה, apocopated וַיֵּבְךְּ) and hollow (שׁוב).

> **GEN.3.14** · `wlc:GEN:69`
> כִּי עָשִׂיתָ זֹּאת אָרוּר אַתָּה מִכָּל־הַבְּהֵמָה
> *kî ʿāśîṯā zōʾṯ ʾārûr ʾattâ mikkol-habbəhēmâ* — because you-did this cursed you from-all-the-cattle — "Because thou hast done this, cursed art thou from among all cattle." (JPS 1917)

Commonest geminate verbs: אָרַר 41, חָלַל 41, בָּלַל 40, רָעַע 20, קָלַל 17, סָבַב 15, תָּמַם 14, פָּרַר 13, חָנַן 10 (392 tokens, 78 lemmas). The class is small in tokens but large in lemmas; many of its members are hard to tell from hollow verbs, with which they share forms (GKC §77).

## 10. Doubly weak and irregular verbs

Ninety-four Torah lemmas (4 728 tokens) are weak in two radicals by our count (2026-10-17). The rules of each class apply together; where they conflict, usage decides (GKC §76).

| verb | classes | key forms (Torah) |
|---|---|---|
| נָשָׂא "lift, carry" | I-nun + III-aleph | imperative שָׂא 10× (DEU.3.27); infinitive שְׂאֵת / שֵׂאת (DEU.1.9, 10.8); yiqtol יִשָּׂא 16× (DEU.1.31); וַיִּשָּׂא 25× (EXO.10.19); jussive יִשָּׂא (NUM.6.26) (GKC §76) |
| נָטָה "stretch out" | I-nun + III-he | וַיֵּט 14× (EXO.8.2); imperative נְטֵה 8× (EXO.7.19); Hiphil תַטֶּה (DEU.16.19), infinitive הַטֹּת (EXO.23.2) |
| נָכָה "strike" (Hiphil) | I-nun + III-he | הִכָּה 10× (DEU.4.46); וַיַּךְ 8× (EXO.2.12); יַכֶּה / יַכֶּנּוּ (DEU.19.4, 25.3); infinitive הַכּוֹת / הַכֹּת 6× each (DEU.25.2, DEU.1.4); imperative הַךְ (EXO.8.12) |
| יָצָא "go out" | I-yod + III-aleph | see §6.2 |
| יָרֵא "fear" | I-yod + III-aleph, stative | see §6.2 |
| בּוֹא "come" | hollow + III-aleph | see §7.2 |
| הָיָה "be" | I-guttural + III-he | see §8.4 |
| חָיָה "live" | I-guttural + III-he (and partly geminate: qatal חַי) | qatal חַי / חָי (GEN.5.5, 25.7); וַיְחִי 33× (GEN.5.3); imperative וֶחְיֵה (GEN.20.7); yiqtol יִחְיֶה (DEU.8.3); Piel אֲחַיֶּה (DEU.32.39); Hiphil לְהַחֲיוֹת (GEN.6.19, 19.19) (GKC §76) |
| עָשָׂה, עָלָה, עָנָה, חָנָה | I-guttural + III-he | see §8.4 |
| נָסַע, נָגַע | I-nun + III-ʿayin | וַיִּסַּע (GEN.12.9), וַנִּסַּע (DEU.2.1) |
| יָדַע | I-yod + III-ʿayin | see §6.2 |

> **GEN.20.7** · `wlc:GEN:502`
> וְיִתְפַּלֵּל בַּעַדְךָ וֶחְיֵה וְאִם־אֵינְךָ מֵשִׁיב דַּע כִּי־מוֹת תָּמוּת
> *wəyiṯpallēl baʿaḏḵā weḥyē wəʾim-ʾênḵā mēšîḇ daʿ kî-môṯ tāmûṯ* — and-he-will-pray for-you and-live and-if-you-are-not returning know that-dying you-will-die — "and he shall pray for thee, and thou shalt live; and if thou restore her not, know thou that thou shalt surely die." (JPS 1917) — חיה (doubly weak imperative), hollow שׁוב (Hiphil participle מֵשִׁיב), I-yod ידע (imperative דַּע), hollow מות (infinitive absolute + yiqtol).

### 10.1 הלך "go"

הָלַךְ is a I-guttural verb in the qatal, participle, infinitive absolute and Hithpael (הָלַךְ, הֹלֵךְ, הָלוֹךְ, הִתְהַלֵּךְ), but in the Qal yiqtol, wayyiqtol, imperative and infinitive construct and in the Hiphil it behaves as if its root were *ylk* (I-waw): יֵלֵךְ, וַיֵּלֶךְ, לֵךְ, לֶכֶת, הוֹלִיךְ (GKC §69 x). MACULA gives the two sets **two lemmas with one Strong number (1980)**: the pointed הָלַךְ (85 tokens: qatal, participle, infinitive absolute, Hithpael) and the bare הלך (227 tokens: the I-waw-like forms). Together they make 312 tokens, which would put "go" ninth among Torah verbs.

| form | Torah (2026-10-17) | MACULA lemma |
|---|---|---|
| qatal הָלַךְ, הָלַכְתָּ | 8× 3ms (DEU.4.3); weqatal וְהָלַכְתָּ 5× (DEU.8.19) | הָלַךְ |
| participle הֹלֵךְ / הוֹלֵךְ | GEN.32.21 … | הָלַךְ |
| infinitive absolute הָלוֹךְ | 4× (GEN.8.3, GEN.12.9) | הָלַךְ |
| Hithpael הִתְהַלֵּךְ | וַיִּתְהַלֵּךְ (GEN.5.22), imperative (GEN.13.17) | הָלַךְ |
| wayyiqtol וַיֵּלֶךְ | 38× + pausal וַיֵּלַךְ 3× (DEU.17.3) | הלך |
| yiqtol יֵלֵךְ, אֵלֵךְ | 9×, 8× (DEU.20.5, DEU.2.27) | הלך |
| imperative לֵךְ, לְכָה | 19×, 13× (DEU.5.30, EXO.3.10) | הלך |
| infinitive construct לֶכֶת | 17× (DEU.8.6) | הלך |
| cohortative אֵלְכָה | 4× (GEN.24.56) | הלך |
| Hiphil וַיּוֹלֶךְ, יוֹלֵךְ | (EXO.14.21, DEU.28.36) | הלך |

> **GEN.12.1** · `wlc:GEN:299`
> וַיֹּאמֶר יְהוָה אֶל־אַבְרָם לֶךְ־לְךָ מֵאַרְצְךָ … אֶל־הָאָרֶץ אֲשֶׁר אַרְאֶךָּ
> *wayyōʾmer YHWH ʾel-ʾaḇrām leḵ-ləḵā mēʾarṣəḵā … ʾel-hāʾāreṣ ʾăšer ʾarʾekkā* — and-said YHWH to-Abram go-for-yourself from-your-land … to-the-land which I-will-show-you — "Now the LORD said unto Abram: 'Get thee out of thy country … unto the land that I will show thee.'" (JPS 1917) — the imperative of הלך, and a III-he Hiphil (ראה) with energic suffix.

### 10.2 Other irregulars

- **נָתַן** and **לָקַח**: §5.2–5.3.
- **יָכֹל** "be able": yiqtol יוּכַל, as if a Hophal (GKC §69).
- **שָׁתָה** "drink": Hiphil הִשְׁקָה from a different root (שׁקה) supplies the causative "give to drink" (GKC §78 — suppletion; the verba defectiva).
- **בּוֹשׁ** "be ashamed": Hithpolel יִתְבֹּשָׁשׁוּ (GEN.2.25).

## 11. The hishtaphel of חוה

The verb "bow down, prostrate oneself" is common (45 Torah tokens) and odd in shape: הִשְׁתַּחֲוָה, yiqtol יִשְׁתַּחֲוֶה, wayyiqtol וַיִּשְׁתַּחוּ (with the short form ending in *-û*, for *wayyištaḥw*), plural וַיִּשְׁתַּחֲווּ, 3fp וַתִּשְׁתַּחֲוֶיןָ.

- **The older analysis** (GKC §75 kk; BDB 1005): a reflexive (Hithpalel) of a Palel of the III-he root שׁחה "bow", with the *t* metathesized after the sibilant (as in any Hithpael of a שׁ-initial root, [06-verb.md §2.6](06-verb.md#26-hithpael)) and the third radical *w* kept as a consonant.
- **The analysis now usual** in grammars and lexicons since comparison with Ugaritic, where the verb appears as a Š-stem with *t* of the root *ḥwy*: a *hishtaphel* (causative-reflexive with *š*) of a root חוה. MACULA (following the OSHB morphology) adopts it: the lemma is חוה, Strong 7812a.

In the trees the stem label is inconsistent: 21 tokens are `HebBinyan=HITHPAEL` (וַיִּשְׁתַּחוּ, GEN.24.26; DEU.17.3) and 24 are `HebBinyan=UNKNOWN: V` (וַתִּשְׁתַּחֲוֶיןָ, GEN.33.6; לֹא תִשְׁתַּחֲוֶה, EXO.34.14; וְהִשְׁתַּחֲוִיתָ, DEU.30.17). The `V` is the OSHB stem code for this form.

> **GEN.33.6** · `wlc:GEN:966`
> וַתִּגַּשְׁןָ הַשְּׁפָחוֹת הֵנָּה וְיַלְדֵיהֶן וַתִּשְׁתַּחֲוֶיןָ
> *wattiggašnā haššəp̄āḥôṯ hēnnâ wəyaldêhen wattištaḥăwênā* — and-came-near the-maids they and-their-children and-they-bowed-down — "Then the handmaids came near, they and their children, and they bowed down." (JPS 1917) — a I-nun Qal wayyiqtol 3fp (וַתִּגַּשְׁןָ) and the hishtaphel 3fp.

> **GEN.24.26** · `wlc:GEN:617`
> וַיִּקֹּד הָאִישׁ וַיִּשְׁתַּחוּ לַיהוָה
> *wayyiqqōḏ hāʾîš wayyištaḥû layhwh* — and-bowed the-man and-prostrated-himself to-YHWH — "And the man bowed his head, and prostrated himself before the LORD." (JPS 1917) — וַיִּקֹּד is a geminate verb (קדד) with the doubled-first-radical yiqtol.

## 12. How to find the root of a weak form

When a form has fewer than three root letters, the traditional procedure is: strip the prefixes and suffixes of the conjugation, look at what is left, and restore the missing letter from the signs of each class (GKC §77; Davidson 1916 has drill tables of this kind). The signs, in the order they usually decide:

| sign in the form | likely missing or changed radical | example |
|---|---|---|
| dagesh in the 1st letter after a yiqtol/wayyiqtol prefix with *i* | I-nun (or לקח, or I-yod group 3) | יִפֹּל → נפל; וַיִּקַּח → לקח; וַיִּצֶק → יצק |
| prefix with *ē* and two letters (Qal) | I-waw | וַיֵּשֶׁב → ישׁב; וַיֵּלֶךְ → הלך |
| *ô* after Niphal/Hiphil prefix, *û* after Hophal | I-waw | הוֹצִיא → יצא; הוּשַׁב → ישׁב |
| *ê* after prefix | I-yod (true) | הֵיטִיב → יטב |
| prefix with *ā* and long vowel in two-letter stem (Qal) | hollow | יָקוּם → קום; וַיָּשֶׂם → שׂים |
| Hiphil with *ē*/*ă* in the prefix, two letters + *î* | hollow (or geminate) | הֵבִיא → בוא; הֵקִים → קום |
| final *-e*, *-ê-*, *-î-*, *-ôṯ*; or a short form with two letters after *wa-* + doubled prefix | III-he | וַיַּרְא → ראה; לַעֲשׂוֹת → עשׂה; וַיְהִי → היה |
| two letters with *a*/*ā* and doubling before endings | geminate | סַבּוּ → סבב; תַּם → תמם |
| *ō* in the Qal prefix with silent א | I-aleph | וַיֹּאמֶר → אמר |
| infinitive in *-eṯ* / *-aṯ* with two root letters | I-waw or I-nun (or לקח) | שֶׁבֶת, לֶדֶת, דַּעַת, גֶּשֶׁת, תֵּת, קַחַת |

Several readings can stay open: וַיָּשֶׁב is the Hiphil wayyiqtol of שׁוב "he brought back" (EXO.4.7) but a form like וַיֵּשֶׁב is the Qal of ישׁב; the infinitive שֶׁבֶת is ישׁב, but שְׁבֹת is שׁבת "cease". The pointing, which the Masoretes fixed, is what separates them, and the pointing is in the trees' FORM.

## 13. The weak classes in the Torah: counts

Counted on the LEMMA of `VERB` tokens in the trees, 2026-10-17. A lemma belongs to every class its letters put it in, so the rows overlap. "I-guttural" includes the I-aleph verbs and הָיָה / חָיָה (their ה and ח are gutturals); "doubly weak" counts lemmas in at least two of the classes I-guttural, III-guttural, III-aleph, I-nun, I-yod, III-he, hollow, geminate. Our classifier works on the lemma's letters and points, so a few lemmas MACULA gives without points (הלך, קרא "meet", נוח, חוה) are classed by letters only.

| class | tokens | lemmas | commonest members (tokens) |
|---|---:|---:|---|
| I-guttural (incl. I-aleph) | 5 799 | 167 | אָמַר 1 373, הָיָה 1 049, עָשָׂה 860, אָכַל 328, הלך 227, עָלָה 201, עָבַר 130, עָבַד 113 |
| — of which I-aleph quiescent | 1 756 | 5 | אָמַר 1 373, אָכַל 328, אָבַד 35, אָבָה 11, אָפָה 9 |
| doubly weak | 4 728 | 94 | הָיָה, עָשָׂה, בּוֹא, יָצָא, עָלָה, יָדַע, נָשָׂא, נָסַע |
| III-he | 4 136 | 110 | הָיָה, עָשָׂה, רָאָה 396, צָוָה 252, עָלָה, נָכָה 99, חָנָה 86, חָיָה 83 |
| II-guttural / II-resh | 2 845 | 193 | רָאָה, קָרָא 200, קָרַב 194, בָּרַךְ 137, יָרַשׁ 101, יָרַד 88 |
| hollow | 1 972 | 89 | בּוֹא 617, מוּת 303, שׁוּב 180, שִׂים 152, קוּם 145 |
| III-aleph | 1 938 | 31 | בּוֹא, יָצָא 347, קָרָא, נָשָׂא 168, מָצָא 122, טָמֵא 114 |
| I-nun | 1 790 | 78 | נָתַן 645, נָשָׂא, נָסַע 120, נָכָה, נָפַל 58 |
| III-ḥet / III-ʿayin | 1 778 | 81 | לָקַח 393, שָׁמַע 241, שָׁלַח 201, יָדַע 174, נָסַע |
| I-yod / I-waw | 1 560 | 48 | יָצָא, יָלַד 205, יָשַׁב 199, יָדַע, יָרַשׁ |
| geminate | 392 | 78 | אָרַר 41, חָלַל 41, בָּלַל 40, רָעַע 20 |

For comparison, the whole Torah has 17 922 `VERB` tokens over 837 lemmas. Weak verbs by any of these classes cover well over half of the verb tokens.

## 14. In Mova

**The root is recoverable, the analysis is not ours.** In the trees every verb carries MACULA's LEMMA (the Qal 3ms citation form of the root, pointed), its Strong's number in MISC, the stem in `HebBinyan` and the form in `Type`. A form whose letters have changed is therefore always tied to its root:

| FORM (in the trees) | LEMMA | Strong | HebBinyan | Type | verse |
|---|---|---|---|---|---|
| יֵּשֶׁב (after וַ) | יָשַׁב | 3427 | QAL | wayyiqtol | EXO.2.15 |
| שְׁבָה (+ separate ־ָה) | יָשַׁב | 3427 | QAL | imperative | GEN.27.19 |
| יִּקַּח | לָקַח | 3947 | QAL | wayyiqtol | EXO.2.1 |
| תֵת (after לָ) | נָתַן | 5414 | QAL | infinitive_construct | DEU.1.8 |
| יָּקָם | קוּם | 6965 | QAL | wayyiqtol | EXO.1.8 |
| הוֹצֵאתִי (+ suffix ךָ) | יָצָא | 3318 | HIPHIL | qatal | DEU.5.6 |
| יַּךְ | נָכָה | 5221 | HIPHIL | wayyiqtol | EXO.2.12 |
| יְהִי | הָיָה | 1961 | QAL | jussive | GEN.1.3 |
| יֵּלֶךְ | הלך | 1980 | QAL | wayyiqtol | EXO.2.1 |
| תִּשְׁתַּחֲוֶיןָ | חוה | 7812a | `UNKNOWN: V` | wayyiqtol | GEN.33.6 |

Because the waw, the article, prepositions and pronoun suffixes are split off as their own tokens, the FORM of a weak verb is often only part of the written word (תֵת in לָתֵת, אמֹר in לֵאמֹר). The dictionary `torah-en.tsv` is keyed on the Strong's number, so every weak form of a root reaches the same entry (for יָשַׁב, 3427: 199 tokens, glosses "he.dwelt (18); you.will.dwell (16); dwell (14)", BDB "sit").

**What Mova cannot do yet.** There is no morphological analyser of our own for Hebrew: all of the above comes from MACULA, which takes its morphology from the OSHB. For text outside the MACULA Bible (the Mishnah, Qumran, inscriptions) Mova has no lemma, stem or form. A rule-based analyser in Rust would need:

1. **A segmenter** for the proclitics (ו, ה, ב, כ, ל, מ, שׁ in Mishnaic Hebrew) and the pronoun suffixes, with the pointing as evidence (the vowel of ה and of בְּ/בַּ, dagesh after the article);
2. **The stem templates** of [06-verb.md §4](06-verb.md#4-paradigm-tables-of-the-strong-verb) as patterns over consonant slots and vowels, with the alternations of each weak class of this chapter as rules (assimilation of נ, *w* → *ô/û*, apocope of III-he, compensatory lengthening before gutturals, furtive pataḥ, hollow *-ô-* connecting vowel);
3. **A candidate generator** that restores a missing radical in every way the signs of §12 allow and keeps the readings that are attested roots in a lexicon (`torah-en.tsv` and BDB give the list of roots);
4. **Disambiguation by context**, at least by the tree (a wayyiqtol cannot follow a subject in its clause), and a measure: MACULA's own lemma and stem on the Torah are the gold standard to score it against, word by word, with the held-out books kept apart.

Such an analyser is a separate project and is not started; until then Mova's Hebrew stays on the MACULA Bible.

**Known faults in the data (2026-10-17).**

- `HebBinyan=UNKNOWN: V` (24, the hishtaphel), `UNKNOWN: U` (6, the hothpaal of פקד) and `QAL PASSIVE` (6) carry a space or colon in FEATS, which is invalid CoNLL-U; the 21 `HITHPAEL` tokens of חוה should get the same label as the other 24.
- הלך / הָלַךְ are two lemmas for one root; any count by lemma (our dictionary, the class counts above) splits the verb unless it merges by Strong's number.
- A few lemmas are unpointed (הלך, קרא, נוח, חוה, שׂים), which a classifier by vowel letters reads less reliably.
- Our class counts use the letters of the lemma, and they strip the dots of שׁ/שׂ, so roots like שׂחט/שׁחט cannot be told apart in such counts unless the dot is kept.

## 15. Sources

- Gesenius, W.; Kautzsch, E.; Cowley, A. E., 1910. *Gesenius' Hebrew Grammar*. Oxford. Public domain. §§19, 22, 41, 62–78, 102 m, 53 u; Paradigms. Wikisource text `$RESEARCH_RAW/hebgram/open/gesenius-kautzsch-cowley-1910-wikisource/gesenius-kautzsch-cowley-1910-wikisource.txt`; https://en.wikisource.org/wiki/Gesenius%27_Hebrew_Grammar.
- Joüon, P., 1923. *Grammaire de l'hébreu biblique*. Rome. Public domain. §§68–82 (weak verbs). `$RESEARCH_RAW/hebgram/pd/jouon-grammaire-1923/`.
- Davidson, A. B., 1916. *An Introductory Hebrew Grammar*, 20th ed. (rev. J. E. McFadyen). Edinburgh. Public domain. Weak-verb lessons and paradigms. `$RESEARCH_RAW/hebgram/pd/davidson-introductory-hebrew-grammar-1916/`; https://archive.org/details/introductoryhebr00davirich.
- Harper, W. R., 1891. *Elements of Hebrew*. Public domain. `$RESEARCH_RAW/hebgram/pd/harper-elements-of-hebrew-1891/`.
- Brown, F.; Driver, S. R.; Briggs, C. A., 1906. *A Hebrew and English Lexicon of the Old Testament*. Oxford. Public domain. p. 1005 (שׁחה, הִשְׁתַּחֲוָה). `$RESEARCH_RAW/lexres/dictionaries/sefaria/BDB_Dictionary.jsonl`; https://archive.org/details/hebrewenglishlex00browuoft.
- Open Scriptures Hebrew Bible, morphology (stem codes, incl. `v` for this form and `u` hothpaal), CC BY 4.0. https://github.com/openscriptures/morphhb; local copy `$RESEARCH_RAW/scripture/1-originals/oshb-morphhb/`.
- MACULA Hebrew Linguistic Datasets, CC BY 4.0. https://github.com/Clear-Bible/macula-hebrew/. Converted by `tools/macula` (tools/macula/README.md).
- The Torah in JPS 1917 (public domain), from eBible.org: `$MOVA_DATA/corpora/judaism/text/torah-jps1917/`. Its English versification differs from the Hebrew in a few chapters (e.g. GEN.32, EXO.8, DEU.13); the English quoted here is that of the matching Hebrew verse.

**Wanted** (not held): J. A. Emerton, "The Etymology of *hištaḥawāh*", *Oudtestamentische Studiën* 20 (1977) — the standard argument for the hishtaphel; a modern open paradigm book of the weak verb with frequency data.
