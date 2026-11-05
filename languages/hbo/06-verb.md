# The verb

The Biblical Hebrew verb is built on a consonantal root, usually of three letters, poured into one of about seven derived stems (*binyanim*), each of which has its own set of conjugations: two finite inflected forms (the suffix form *qatal* and the prefix form *yiqtol*), the forms with "consecutive" *waw* (*wayyiqtol*, *weqatal*), three volitives (imperative, jussive, cohortative), two infinitives and the participles. This chapter gives the stems with their meanings and markers, full paradigm tables of the strong verb, the uses of each form, the debate over tense and aspect, the paragogic and energic endings, the object suffixes on the verb, and the counts of all of these in the Torah as Mova's trees give them. The weak verbs, whose root letters disappear or change, have a chapter of their own ([07-weak-verbs.md](07-weak-verbs.md)).

All counts are from the MACULA-based Torah trees in `$MOVA_DATA/corpora/judaism/text/torah-wlc-macula/*.conllu` (GEN–DEU, 5 853 sentences, 125 807 words, 17 922 words with UPOS `VERB`), counted on **2026-10-17**.

## Contents

1. [The root](#1-the-root)
2. [The stems (binyanim)](#2-the-stems-binyanim)
3. [The conjugations: an overview](#3-the-conjugations-an-overview)
4. [Paradigm tables of the strong verb](#4-paradigm-tables-of-the-strong-verb)
5. [Qatal (the perfect)](#5-qatal-the-perfect)
6. [Yiqtol (the imperfect)](#6-yiqtol-the-imperfect)
7. [Wayyiqtol and weqatal: the forms with consecutive waw](#7-wayyiqtol-and-weqatal-the-forms-with-consecutive-waw)
8. [The volitives: imperative, jussive, cohortative](#8-the-volitives-imperative-jussive-cohortative)
9. [The infinitives](#9-the-infinitives)
10. [The participles](#10-the-participles)
11. [Stative verbs](#11-stative-verbs)
12. [Paragogic and energic endings](#12-paragogic-and-energic-endings)
13. [Object suffixes on the verb](#13-object-suffixes-on-the-verb)
14. [Tense, aspect or mood: the debate](#14-tense-aspect-or-mood-the-debate)
15. [The verb in the Torah: counts](#15-the-verb-in-the-torah-counts)
16. [In Mova](#16-in-mova)
17. [Sources](#17-sources)

Abbreviations: GKC = Gesenius–Kautzsch–Cowley 1910; JM = Joüon 1923 (*Grammaire de l'hébreu biblique*; the paragraph numbers are kept in the later Joüon–Muraoka editions); Driver = Driver 1892, *A Treatise on the Use of the Tenses in Hebrew*. Person–gender–number is written 3ms, 2fs, 1cp and so on.

---

## 1. The root

A Hebrew verb is named by its root (*šoreš*), conventionally cited in the Qal qatal 3ms, which is the shortest form with all three radicals: פָּקַד *pāqaḏ* "he visited, attended to, mustered". The radicals are called by the letters of the old model verb פעל: the first radical is the *pe* (פ״), the second the *ʿayin* (ע״), the third the *lamed* (ל״). So יָשַׁב is a פ״י (I-yod) verb, קוּם an ע״ו (II-waw, "hollow") verb, בָּנָה a ל״ה (III-he) verb (GKC §30, §41; JM §40).

Almost all verbs are triliteral. A few quadriliterals exist (GKC §56), and some apparent quadriliterals are reduplicated stems of the Pilpel type (כִּלְכֵּל from כול, §2.8 below; GKC §30, §55).

The root carries a lexical core; the stem, the vowel pattern and the affixes carry the grammar. The same root פקד gives the Qal "attend to, number", the Niphal "be missed, be numbered", the Hiphil "appoint, entrust", the Hophal "be entrusted", the Pual "be mustered", and the nouns פְּקֻדָּה "visitation, charge", פִּקָּדוֹן "deposit", מִפְקָד "appointment" (BDB 823–824).

> **GEN.21.1** · `wlc:GEN:514`
> וַיהוָה פָּקַד אֶת־שָׂרָה כַּאֲשֶׁר אָמָר
> *wayhwh pāqaḏ ʾeṯ-śārâ kaʾăšer ʾāmār* — and-YHWH attended-to OM-Sarah as he-had-said — "And the LORD remembered Sarah as He had said." (JPS 1917)

> **GEN.39.4** · `wlc:GEN:1153`
> וַיַּפְקִדֵהוּ עַל־בֵּיתוֹ
> *wayyap̄qiḏēhû ʿal-bêṯô* — and-he-appointed-him over-his-house — "And he appointed him overseer over his house." (JPS 1917) — the same root in the Hiphil, with an object suffix.

> **NUM.31.49** · `wlc:NUM:1109`
> וְלֹא־נִפְקַד מִמֶּנּוּ אִישׁ
> *wəlōʾ-nip̄qaḏ mimmennû ʾîš* — and-not was-missing from-us a-man — "and there lacketh not one man of us." — the Niphal.

In Mova's trees the root is not a field of its own: it is reached through the LEMMA (the Qal qatal 3ms citation form, pointed, e.g. פָּקַד) and the MISC `Strong=` number (6485 for פָּקַד), which are the same for every stem of the root. A bare root (consonants only) can be read off the lemma by dropping the points, which is exactly what `tools/macula` does when it compares lemmas with UD PTNK (`consonants()` in `tools/macula/src/main.rs`).

## 2. The stems (binyanim)

Each root may appear in one or more stems. The Hebrew names are the 3ms qatal of the old model verb פעל in that stem: Qal (קַל "light", the plain stem), Niphal (נִפְעַל), Piel (פִּעֵל), Pual (פֻּעַל), Hithpael (הִתְפַּעֵל), Hiphil (הִפְעִיל), Hophal (הָפְעַל). They form three families (GKC §39):

- the **simple** stems: Qal (active) and Niphal (reflexive, middle, passive);
- the **D stems** (doubled middle radical): Piel (active), Pual (passive), Hithpael (reflexive, reciprocal);
- the **causative** stems: Hiphil (active), Hophal (passive).

### 2.1 The seven main stems: markers and meanings

| stem | qatal 3ms | yiqtol 3ms | marker | typical meaning | Torah tokens |
|---|---|---|---|---|---:|
| Qal | קָטַל *qāṭal* | יִקְטֹל *yiqṭōl* | none | plain action or state | 12 751 |
| Niphal | נִקְטַל *niqṭal* | יִקָּטֵל *yiqqāṭēl* | prefix *n-* (assimilated in yiqtol, so the first radical is doubled) | reflexive, middle, passive of Qal | 883 |
| Piel | קִטֵּל *qiṭṭēl* | יְקַטֵּל *yəqaṭṭēl* | doubled 2nd radical; *ə* in the yiqtol prefix | factitive, resultative, denominative, plural action | 1 859 |
| Pual | קֻטַּל *quṭṭal* | יְקֻטַּל *yəquṭṭal* | doubled 2nd radical; *u–a* vowels | passive of Piel | 92 |
| Hithpael | הִתְקַטֵּל *hiṯqaṭṭēl* | יִתְקַטֵּל *yiṯqaṭṭēl* | prefix *hiṯ-*, doubled 2nd radical | reflexive, reciprocal, iterative of Piel | 179 |
| Hiphil | הִקְטִיל *hiqṭîl* | יַקְטִיל *yaqṭîl* | prefix *h-*; *a* in the yiqtol prefix; *î* in the stem | causative of Qal; also declarative and inner-causative | 2 069 |
| Hophal | הָקְטַל *hoqṭal* | יָקְטַל *yoqṭal* | prefix *ho-/hu-*; *o/u–a* vowels | passive of Hiphil | 149 |

The counts are tokens in the Torah whose FEATS carry `HebBinyan=` with that value (2026-10-17). The minor stems and the oddities of the feature values are in §2.8 and §16.

The meanings are tendencies, not rules: many verbs exist only in one stem with no "base" meaning to derive from (דִּבֶּר "speak" is Piel, 408 of its 415 Torah tokens; צִוָּה "command" is Piel, 246 of 252), and the meaning of a stem must be learned with each verb (GKC §39; JM §40; on the semantics of the D stem see GKC §52).

### 2.2 Qal

The Qal is the unmarked stem. It is both transitive (פָּקַד "number", נָתַן "give") and intransitive (יָשַׁב "sit, dwell", מֵת "die"). It is also the home of the **statives** (כָּבֵד "be heavy", קָטֹן "be small"; §11). Some Qal verbs have an old internal passive, the **Qal passive**, which the Masoretes mostly pointed as Pual or Hophal (GKC §52 e, §53 u); MACULA marks six Torah tokens as `QAL PASSIVE`:

> **GEN.12.15** · `wlc:GEN:313`
> וַתֻּקַּח הָאִשָּׁה בֵּית פַּרְעֹה
> *wattuqqaḥ hāʾiššâ bêṯ parʿō* — and-was-taken the-woman house-of Pharaoh — "and the woman was taken into Pharaoh's house." (JPS 1917)

The others are יֻקַּח (GEN.18.4), טֹרָף (GEN.44.28), אֻכָּל (EXO.3.2) and זֹרַק twice (NUM.19.13, 19.20).

### 2.3 Niphal

Marked by a prefixed *n*: in the qatal and participle it stands as נִ־ (נִקְטַל, נִקְטָל), in the yiqtol, imperative and infinitive construct it is assimilated to the first radical, which is then doubled (יִקָּטֵל < *yinqāṭēl*), and the imperative and infinitive take a prefixed הִ־ (הִקָּטֵל) (GKC §51). Its meanings run from reflexive ("guard oneself") through middle and reciprocal ("fight one another") to plain passive of the Qal ("be circumcised, be found") (GKC §51; on its aspectual profile compared with the Hithpael see Benton 2009).

> **GEN.17.26** · `wlc:GEN:423`
> בְּעֶצֶם הַיּוֹם הַזֶּה נִמּוֹל אַבְרָהָם וְיִשְׁמָעֵאל בְּנוֹ
> *bəʿeṣem hayyôm hazze nimmôl ʾaḇrāhām wəyišmāʿēʾl bənô* — in-the-bone-of the-day the-this was-circumcised Abraham and-Ishmael his-son — "In the selfsame day was Abraham circumcised, and Ishmael his son." (JPS 1917)

> **EXO.14.14** · `wlc:EXO:370`
> יְהוָה יִלָּחֵם לָכֶם וְאַתֶּם תַּחֲרִישׁוּן
> *YHWH yillāḥēm lāḵem wəʾattem taḥărîšûn* — YHWH will-fight for-you and-you will-keep-silent — "The LORD will fight for you, and ye shall hold your peace." (JPS 1917) — a Niphal with middle sense (לחם is used only in the Niphal in the Torah) and a Hiphil yiqtol with paragogic *nun* (§12).

> **GEN.21.24** · `wlc:GEN:537`
> וַיֹּאמֶר אַבְרָהָם אָנֹכִי אִשָּׁבֵעַ
> *wayyōʾmer ʾaḇrāhām ʾānōḵî ʾiššāḇēaʿ* — and-said Abraham I I-will-swear — "And Abraham said: 'I will swear.'" (JPS 1917) — the Niphal 1cs yiqtol; "swear" is Niphal, "make swear" is Hiphil (GEN.50.25 below).

### 2.4 Piel

Marked by the doubled second radical (dagesh forte) and, in the yiqtol and participle, a vocal shewa under the prefix (יְקַטֵּל, מְקַטֵּל). The traditional label "intensive" is too narrow. With intransitive Qal verbs the Piel is often **factitive** (Qal "be great" → Piel "make great": גִּדֵּל), with transitive ones **resultative** or plural ("break" → "smash to pieces"); many Piels are **denominative** (from nouns: דִּבֶּר "speak" beside דָּבָר "word", כִּהֵן "act as priest" from כֹּהֵן) (GKC §52; JM §52).

> **GEN.49.18** · `wlc:GEN:1491`
> לִישׁוּעָתְךָ קִוִּיתִי יְהוָה
> *lîšûʿāṯəḵā qiwwîṯî YHWH* — for-your-salvation I-wait YHWH — "I wait for Thy salvation, O Lord." (JPS 1917)

The Piel of ברך "bless" (108 of its 137 Torah tokens) is the commonest Piel after דבר and צוה; with a II-*resh* root the doubling is impossible and the preceding vowel is lengthened instead (בֵּרַךְ, not *birrak*; see the II-guttural verbs in [07-weak-verbs.md](07-weak-verbs.md)).

### 2.5 Pual

The passive of the Piel: doubled second radical, *u* (qibbuts) in the first syllable, *a* in the second (קֻטַּל, יְקֻטַּל, מְקֻטָּל). It has no imperative and almost no infinitive (GKC §52, q–s). In the Torah it is rare (92 tokens), and most of them are participles of the craft vocabulary of the Tabernacle ("dyed red", "joined", "twisted").

> **EXO.9.32** · `wlc:EXO:241`
> וְהַחִטָּה וְהַכֻּסֶּמֶת לֹא נֻכּוּ כִּי אֲפִילֹת הֵנָּה
> *wəhaḥiṭṭâ wəhakkussemeṯ lōʾ nukkû kî ʾăp̄îlōṯ hēnnâ* — and-the-wheat and-the-spelt not were-struck for late they — "But the wheat and the spelt were not smitten; for they ripen late." (JPS 1917) — MACULA codes נֻכּוּ as Pual of נָכָה; the Hiphil הִכָּה "strike" is the usual stem of this root.

> **DEU.28.34** · `wlc:DEU:752`
> וְהָיִיתָ מְשֻׁגָּע מִמַּרְאֵה עֵינֶיךָ
> *wəhāyîṯā məšuggāʿ mimmarʾē ʿênêḵā* — and-you-shall-be driven-mad from-the-sight-of your-eyes — "so that thou shalt be mad for the sight of thine eyes." (JPS 1917) — Pual participle with prefix *mə-*.

> **EXO.38.21** · `wlc:EXO:1121`
> אֲשֶׁר פֻּקַּד עַל־פִּי מֹשֶׁה
> *ʾăšer puqqaḏ ʿal-pî mōše* — which was-counted by-the-mouth-of Moses — "as they were rendered according to the commandment of Moses."

### 2.6 Hithpael

The prefix *hiṯ-* (yiqtol *yiṯ-*, participle *miṯ-*) plus the doubled second radical. The *t* of the prefix changes place or quality next to sibilants and dentals: before ס, שׂ, שׁ it **metathesizes** (הִשְׁתַּמֵּר, not *hitšammēr*), before צ it metathesizes and becomes ט (הִצְטַדֵּק), and before ד, ט, ת it assimilates (הִטַּהֵר "purify oneself") (GKC §54; on the tG stem and its history see Hutton & Marzouk 2012). Meanings: reflexive ("sanctify oneself"), reciprocal ("look at one another"), iterative ("walk about": הִתְהַלֵּךְ), estimative or simulative ("pretend to be": הִתְנַכֵּר "make oneself strange") (GKC §54).

> **GEN.5.22** · `wlc:GEN:127`
> וַיִּתְהַלֵּךְ חֲנוֹךְ אֶת־הָאֱלֹהִים
> *wayyiṯhallēḵ ḥănôḵ ʾeṯ-hāʾĕlōhîm* — and-walked-about Enoch with-God — "And Enoch walked with God." (JPS 1917)

> **DEU.3.23** · `wlc:DEU:105`
> וָאֶתְחַנַּן אֶל־יְהוָה בָּעֵת הַהִוא לֵאמֹר
> *wāʾeṯḥannan ʾel-YHWH bāʿēṯ hahiwʾ lēʾmōr* — and-I-sought-favour to-YHWH at-the-time the-that saying — "And I besought the LORD at that time, saying:" (JPS 1917)

**The form הִשְׁתַּחֲוָה "bow down".** This frequent verb (45 Torah tokens) looks like a Hithpael of שׁחה with the *t* metathesized. GKC (§75 kk) and BDB (1005) analyse it so, as the reflexive of a Palel of שׁחה (שַׁחֲוָה, with the third radical *w* kept), and give the 3ms wayyiqtol וַיִּשְׁתַּחוּ as a contraction of *wayyištaḥw*. Later comparative work, which draws on Ugaritic, reads it instead as a rare *hishtaphel* (Š-stem with infixed *t*) of a root חוה. MACULA follows that analysis. It gives the lemma חוה (Strong 7812a) but is not consistent about the stem: 21 tokens are labelled `HITHPAEL` (e.g. וַיִּשְׁתַּחוּ, GEN.24.26) and 24 are labelled `UNKNOWN: V` (e.g. וַתִּשְׁתַּחֲוֶיןָ, GEN.33.6). See [07-weak-verbs.md §11](07-weak-verbs.md#11-the-hishtaphel-of-חוה).

### 2.7 Hiphil

The causative stem: prefix *h-* in the qatal, imperative and infinitive (הִקְטִיל, הַקְטֵל), a prefix vowel *a* in the yiqtol and participle (יַקְטִיל, מַקְטִיל), and a long *î* in the stem in most forms. Meanings (GKC §53; JM §54):

- **causative** of the Qal: יָצָא "go out" → הוֹצִיא "bring out"; בּוֹא "come" → הֵבִיא "bring"; קָרַב "draw near" → הִקְרִיב "bring near, offer" (149 of the 194 Torah tokens of קָרַב are Hiphil: the offering of sacrifices in Leviticus);
- **declarative**: הִצְדִּיק "declare righteous";
- **inner or intransitive**: הֶאֱמִין "believe, put trust", הִשְׁכִּים "rise early";
- **denominative**: הֶאֱזִין "give ear" (from אֹזֶן).

> **GEN.15.6** · `wlc:GEN:366`
> וְהֶאֱמִן בַּיהוָה וַיַּחְשְׁבֶהָ לּוֹ צְדָקָה
> *wəheʾĕmin bayhwh wayyaḥšəḇehā lô ṣəḏāqâ* — and-he-trusted in-YHWH and-he-reckoned-it to-him righteousness — "And he believed in the LORD; and He counted it to him for righteousness." (JPS 1917)

> **GEN.50.25** · `wlc:GEN:1531`
> וַיַּשְׁבַּע יוֹסֵף אֶת־בְּנֵי יִשְׂרָאֵל לֵאמֹר פָּקֹד יִפְקֹד אֱלֹהִים אֶתְכֶם
> *wayyašbaʿ yôsēp̄ ʾeṯ-bənê yiśrāʾēl lēʾmōr pāqōḏ yip̄qōḏ ʾĕlōhîm ʾeṯḵem* — and-made-swear Joseph OM-sons-of Israel saying visiting will-visit God you — "And Joseph took an oath of the children of Israel, saying: 'God will surely remember you.'" (JPS 1917)

In MACULA's argument frames the Hiphil is the only stem with a **causer** role: 1 243 Torah Hiphil tokens carry `Frame=AA:…;A0:…` (causer and agent), another 91 `A0:…;AA:…`, and 49 `AA, A0, A1`, against no AA at all in the Qal (§15.4).

### 2.8 Hophal

The passive of the Hiphil: prefix *ho-* (with qamets hatuf) or *hu-*, vowels *o/u–a* (הָקְטַל, יָקְטַל, מָקְטָל). In I-yod and hollow roots the prefix vowel becomes *û* (הוּמַת, יוּמַת) (GKC §53).

> **EXO.21.12** · `wlc:EXO:556`
> מַכֵּה אִישׁ וָמֵת מוֹת יוּמָת
> *makkē ʾîš wāmēṯ môṯ yûmāṯ* — striker-of a-man and-he-died dying he-shall-be-put-to-death — "He that smiteth a man, so that he dieth, shall surely be put to death." (JPS 1917)

> **EXO.25.40** · `wlc:EXO:702`
> וּרְאֵה וַעֲשֵׂה בְּתַבְנִיתָם אֲשֶׁר־אַתָּה מָרְאֶה בָּהָר
> *ûrəʾē waʿăśē bəṯaḇnîṯām ʾăšer-ʾattâ morʾe bāhār* — and-see and-make by-their-pattern which-you being-shown on-the-mountain — "And see that thou make them after their pattern, which is being shown thee in the mount." (JPS 1917) — Hophal participle.

> **LEV.5.23** · `wlc:LEV:107`
> אֶת־הַפִּקָּדוֹן אֲשֶׁר הָפְקַד אִתּוֹ
> *ʾeṯ-happiqqāḏôn ʾăšer hop̄qaḏ ʾittô* — OM-the-deposit which was-entrusted with-him — "or the deposit which was deposited with him."

### 2.9 The minor stems

Hollow, geminate and some III-he roots replace the D stems with forms that repeat a radical or lengthen a vowel (GKC §55; JM §59). The Torah has them all, but rarely:

| stem | built like | Torah tokens (2026-10-17) | example |
|---|---|---:|---|
| Polel | Piel of hollow/geminate roots, *ō* after the first radical, 3rd radical repeated | 7 | וַאֲרֹמְמֶנְהוּ "and I will exalt him" (EXO.15.2); מְחֹלְלֶךָ "who brought you forth" (DEU.32.18) |
| Poel | the same pattern in strong or geminate roots | 13 | מְקֹשֵׁשׁ עֵצִים "gathering sticks" (NUM.15.32); לֹא תְעוֹנֵנוּ "you shall not practise soothsaying" (LEV.19.26) |
| Hithpolel | Hithpael of the same | 7 | וְלֹא יִתְבֹּשָׁשׁוּ "and were not ashamed" (GEN.2.25); מִסְתּוֹלֵל "exalting yourself" (EXO.9.17) |
| Pilpel | the two strong radicals of a hollow/geminate root repeated | 5 | וַיְכַלְכֵּל יוֹסֵף "and Joseph sustained" (GEN.47.12); כִּמְתַעְתֵּעַ "as a mocker" (GEN.27.12) |
| Hithpalpel | Hithpael of the Pilpel | 3 | הִתְמַהְמָהְנוּ "we had lingered" (GEN.43.10) |
| Palel | III-he root with its *w* kept as a consonant (GKC §55 d, §75 kk) | 1 | כִּמְטַחֲוֵי קֶשֶׁת "about a bowshot", from טחה (GEN.21.16) |
| Nithpael | Niphal-like prefix *n-* on a Hithpael (a late form) | 1 | וְנִכַּפֵּר לָהֶם הַדָּם "and the blood shall be forgiven them" (DEU.21.8) |
| Hothpaal | passive of the Hithpael | 6, labelled `UNKNOWN: U` | לֹא הָתְפָּקְדוּ "were not numbered" (NUM.1.47, 2.33 …) |
| Hishtaphel | Š-stem with *t* | 24 labelled `UNKNOWN: V`, 21 labelled `HITHPAEL` | וַתִּשְׁתַּחֲוֶיןָ "and they bowed down" (GEN.33.6) |

> **GEN.47.12** · `wlc:GEN:1432`
> וַיְכַלְכֵּל יוֹסֵף אֶת־אָבִיו וְאֶת־אֶחָיו
> *waykalkēl yôsēp̄ ʾeṯ-ʾāḇîw wəʾeṯ-ʾeḥāyw* — and-sustained Joseph OM-his-father and-OM-his-brothers — "And Joseph sustained his father, and his brethren." (JPS 1917)

> **NUM.1.47** · `wlc:NUM:46`
> וְהַלְוִיִּם לְמַטֵּה אֲבֹתָם לֹא הָתְפָּקְדוּ בְּתוֹכָם
> *wəhalwiyyim ləmaṭṭē ʾăḇōṯām lōʾ hoṯpāqəḏû bəṯôḵām* — and-the-Levites by-tribe-of their-fathers not were-numbered in-their-midst — "But the Levites after the tribe of their fathers were not numbered among them." (JPS 1917) — the Hothpaal, a passive of the Hithpael found only with this root (GKC §54 l).

## 3. The conjugations: an overview

| form | other names | shape | basic use in prose | Torah tokens |
|---|---|---|---|---:|
| qatal | perfect, suffix conjugation | stem + suffixes | completed / past situation; states; performatives | 2 694 |
| yiqtol | imperfect, prefix conjugation | prefixes (+ suffixes) + stem | future, habitual, modal; commands of the law | 3 776 |
| wayyiqtol | imperfect with waw consecutive, preterite | וַ + doubling + short prefix form | the main line of past narrative | 4 187 |
| weqatal | perfect with waw consecutive | וְ + qatal (stress often shifted to the end) | sequence in future, habitual and instruction | 2 434 |
| imperative | — | yiqtol without prefix, 2nd person only | direct command (positive) | 782 |
| jussive | — | short yiqtol, 3rd (and 2nd) person | wish, indirect command; with אַל prohibition | 192 |
| cohortative | — | yiqtol 1st person + ־ָה | self-exhortation "let me / let us" | 129 |
| infinitive construct | — | קְטֹל | verbal noun after prepositions, ל־ purpose | 1 739 |
| infinitive absolute | — | קָטוֹל | emphasis, manner, command, report | 255 |
| participle (active) | — | קֹטֵל, מְקַטֵּל … | ongoing, imminent, agent noun | 1 472 |
| participle (passive) | — | קָטוּל, מְקֻטָּל … | resultant state | 395 |

Counts are MACULA's `Type=` values in the Torah (2026-10-17). The wayyiqtol is the commonest single form: a third of all finite verbs in the five books.

## 4. Paradigm tables of the strong verb

The tables use the traditional model root קטל "kill" (GKC Paradigm B; it is rare in the Bible and is used for its regular shape). Forms that the Torah actually has for פקד are given after each table with their verses, taken from the trees (2026-10-17).

### 4.1 Qatal (perfect)

| | Qal | Niphal | Piel | Pual | Hithpael | Hiphil | Hophal |
|---|---|---|---|---|---|---|---|
| 3ms | קָטַל *qāṭal* | נִקְטַל *niqṭal* | קִטֵּל *qiṭṭēl* | קֻטַּל *quṭṭal* | הִתְקַטֵּל *hiṯqaṭṭēl* | הִקְטִיל *hiqṭîl* | הָקְטַל *hoqṭal* |
| 3fs | קָטְלָה *qāṭəlâ* | נִקְטְלָה *niqṭəlâ* | קִטְּלָה *qiṭṭəlâ* | קֻטְּלָה *quṭṭəlâ* | הִתְקַטְּלָה *hiṯqaṭṭəlâ* | הִקְטִילָה *hiqṭîlâ* | הָקְטְלָה *hoqṭəlâ* |
| 2ms | קָטַלְתָּ *qāṭaltā* | נִקְטַלְתָּ *niqṭaltā* | קִטַּלְתָּ *qiṭṭaltā* | קֻטַּלְתָּ *quṭṭaltā* | הִתְקַטַּלְתָּ *hiṯqaṭṭaltā* | הִקְטַלְתָּ *hiqṭaltā* | הָקְטַלְתָּ *hoqṭaltā* |
| 2fs | קָטַלְתְּ *qāṭalt* | נִקְטַלְתְּ *niqṭalt* | קִטַּלְתְּ *qiṭṭalt* | קֻטַּלְתְּ *quṭṭalt* | הִתְקַטַּלְתְּ *hiṯqaṭṭalt* | הִקְטַלְתְּ *hiqṭalt* | הָקְטַלְתְּ *hoqṭalt* |
| 1cs | קָטַלְתִּי *qāṭaltî* | נִקְטַלְתִּי *niqṭaltî* | קִטַּלְתִּי *qiṭṭaltî* | קֻטַּלְתִּי *quṭṭaltî* | הִתְקַטַּלְתִּי *hiṯqaṭṭaltî* | הִקְטַלְתִּי *hiqṭaltî* | הָקְטַלְתִּי *hoqṭaltî* |
| 3cp | קָטְלוּ *qāṭəlû* | נִקְטְלוּ *niqṭəlû* | קִטְּלוּ *qiṭṭəlû* | קֻטְּלוּ *quṭṭəlû* | הִתְקַטְּלוּ *hiṯqaṭṭəlû* | הִקְטִילוּ *hiqṭîlû* | הָקְטְלוּ *hoqṭəlû* |
| 2mp | קְטַלְתֶּם *qəṭaltem* | נִקְטַלְתֶּם *niqṭaltem* | קִטַּלְתֶּם *qiṭṭaltem* | קֻטַּלְתֶּם *quṭṭaltem* | הִתְקַטַּלְתֶּם *hiṯqaṭṭaltem* | הִקְטַלְתֶּם *hiqṭaltem* | הָקְטַלְתֶּם *hoqṭaltem* |
| 2fp | קְטַלְתֶּן *qəṭalten* | נִקְטַלְתֶּן *niqṭalten* | קִטַּלְתֶּן *qiṭṭalten* | קֻטַּלְתֶּן *quṭṭalten* | הִתְקַטַּלְתֶּן *hiṯqaṭṭalten* | הִקְטַלְתֶּן *hiqṭalten* | הָקְטַלְתֶּן *hoqṭalten* |
| 1cp | קָטַלְנוּ *qāṭalnû* | נִקְטַלְנוּ *niqṭalnû* | קִטַּלְנוּ *qiṭṭalnû* | קֻטַּלְנוּ *quṭṭalnû* | הִתְקַטַּלְנוּ *hiṯqaṭṭalnû* | הִקְטַלְנוּ *hiqṭalnû* | הָקְטַלְנוּ *hoqṭalnû* |

Notes (GKC §44, §51–54):

- The endings are the same in every stem: *-â, -tā, -t, -tî, -û, -tem, -ten, -nû*; they go back to suffixed pronouns (GKC §44).
- Before a vowel ending (3fs, 3cp) the second-syllable vowel reduces to shewa (קָטְלָה); in the Hiphil the *î* stays (הִקְטִילָה).
- In the Piel 3ms the second vowel is *ē* (קִטֵּל), but *a* before consonantal endings (קִטַּלְתָּ); some Piels have *a* also in 3ms (דִּבֶּר with *e*, בֵּרַךְ).
- In pause the reduced vowel comes back (קָטָ֫לָה, קָטָ֫לוּ) (GKC §29).
- The 2ms ending is sometimes written plene with *he*: נָתַתָּה (DEU.26.10), נָשָׂאתָה (NUM.14.19).

**פקד in the Torah (qatal and weqatal).** Qal פָּקַד 3ms 9× (GEN.21.1, EXO.4.31 …); פָּקַדְתִּי 1cs (EXO.3.16); פָּקְדוּ 3cp (NUM.26.63); weqatal וּפְקַדְתֶּם 2mp (NUM.4.27), וּפָקַדְתִּי 1cs (EXO.32.34); Niphal נִפְקַד (NUM.31.49); Pual פֻּקַּד (EXO.38.21); Hiphil הִפְקִיד (GEN.39.5), וְהִפְקַדְתִּי (LEV.26.16); Hophal הָפְקַד (LEV.5.23); Hothpaal הָתְפָּקְדוּ (NUM.1.47, 2.33, 26.62).

### 4.2 Yiqtol (imperfect)

| | Qal | Niphal | Piel | Pual | Hithpael | Hiphil | Hophal |
|---|---|---|---|---|---|---|---|
| 3ms | יִקְטֹל *yiqṭōl* | יִקָּטֵל *yiqqāṭēl* | יְקַטֵּל *yəqaṭṭēl* | יְקֻטַּל *yəquṭṭal* | יִתְקַטֵּל *yiṯqaṭṭēl* | יַקְטִיל *yaqṭîl* | יָקְטַל *yoqṭal* |
| 3fs | תִּקְטֹל *tiqṭōl* | תִּקָּטֵל | תְּקַטֵּל | תְּקֻטַּל | תִּתְקַטֵּל | תַּקְטִיל | תָּקְטַל |
| 2ms | תִּקְטֹל *tiqṭōl* | תִּקָּטֵל | תְּקַטֵּל | תְּקֻטַּל | תִּתְקַטֵּל | תַּקְטִיל | תָּקְטַל |
| 2fs | תִּקְטְלִי *tiqṭəlî* | תִּקָּטְלִי | תְּקַטְּלִי | תְּקֻטְּלִי | תִּתְקַטְּלִי | תַּקְטִילִי | תָּקְטְלִי |
| 1cs | אֶקְטֹל *ʾeqṭōl* | אֶקָּטֵל | אֲקַטֵּל | אֲקֻטַּל | אֶתְקַטֵּל | אַקְטִיל | אָקְטַל |
| 3mp | יִקְטְלוּ *yiqṭəlû* | יִקָּטְלוּ | יְקַטְּלוּ | יְקֻטְּלוּ | יִתְקַטְּלוּ | יַקְטִילוּ | יָקְטְלוּ |
| 3fp | תִּקְטֹלְנָה *tiqṭōlnâ* | תִּקָּטַלְנָה | תְּקַטֵּלְנָה | תְּקֻטַּלְנָה | תִּתְקַטֵּלְנָה | תַּקְטֵלְנָה | תָּקְטַלְנָה |
| 2mp | תִּקְטְלוּ *tiqṭəlû* | תִּקָּטְלוּ | תְּקַטְּלוּ | תְּקֻטְּלוּ | תִּתְקַטְּלוּ | תַּקְטִילוּ | תָּקְטְלוּ |
| 2fp | תִּקְטֹלְנָה *tiqṭōlnâ* | תִּקָּטַלְנָה | תְּקַטֵּלְנָה | תְּקֻטַּלְנָה | תִּתְקַטֵּלְנָה | תַּקְטֵלְנָה | תָּקְטַלְנָה |
| 1cp | נִקְטֹל *niqṭōl* | נִקָּטֵל | נְקַטֵּל | נְקֻטַּל | נִתְקַטֵּל | נַקְטִיל | נָקְטַל |

Notes (GKC §47, §51–54):

- The prefixes are *ʾ-, t-, y-, n-*; the suffixes *-î* (2fs), *-û* (m. pl.), *-nâ* (f. pl.). The same form serves 3fs and 2ms, and 3fp and 2fp; the context decides.
- The prefix vowel tells the stem: *i* (Qal, Niphal, Hithpael), *ə* (Piel, Pual), *a* (Hiphil), *o/u* (Hophal); in the 1cs it is *e*, *ă* or *a* because א takes no shewa.
- The Qal theme vowel is *ō* for most active verbs (יִקְטֹל), *a* for statives and for verbs with a guttural second or third radical (יִכְבַּד, יִשְׁמַע) (GKC §47, §43).
- The m. pl. forms often take paragogic *nun*: יִקְטְלוּן (§12).

**פקד in the Torah.** Qal יִפְקֹד (EXO.13.19 …), תִּפְקֹד (NUM.3.10), תִּפְקְדוּ (NUM.1.3); Niphal יִפָּקֵד (NUM.16.29); Hiphil jussive יַפְקֵד (GEN.41.34).

> **NUM.3.15** · `wlc:NUM:102`
> פְּקֹד אֶת־בְּנֵי לֵוִי … כָּל־זָכָר מִבֶּן־חֹדֶשׁ וָמַעְלָה תִּפְקְדֵם
> *pəqōḏ ʾeṯ-bənê lēwî … kol-zāḵār mibben-ḥōḏeš wāmaʿlâ tip̄qəḏēm* — number OM-sons-of Levi … every-male from-a-son-of-a-month and-upward you-shall-number-them — "Number the children of Levi … every male from a month old and upward shalt thou number them." (JPS 1917) — imperative at the start, yiqtol with suffix at the end.

### 4.3 Wayyiqtol

The wayyiqtol is וַ *wa-* plus doubling of the prefix consonant (וַיִּ, וַתִּ, וַנִּ; before א, which cannot double, וָאֶ with compensatory lengthening), and the **short** prefix form, which differs from the yiqtol where the jussive does: in the Hiphil (וַיַּקְטֵל against יַקְטִיל) and in the weak verbs (וַיָּקָם, וַיְהִי, וַיִּבֶן; see [07-weak-verbs.md](07-weak-verbs.md)). The stress also tends to move back one syllable when the last syllable is closed and the one before it open (וַיָּ֫קָם) (GKC §49).

| | Qal | Niphal | Piel | Hithpael | Hiphil |
|---|---|---|---|---|---|
| 3ms | וַיִּקְטֹל | וַיִּקָּטֵל | וַיְקַטֵּל | וַיִּתְקַטֵּל | וַיַּקְטֵל |
| 3fs | וַתִּקְטֹל | וַתִּקָּטֵל | וַתְּקַטֵּל | וַתִּתְקַטֵּל | וַתַּקְטֵל |
| 1cs | וָאֶקְטֹל | וָאֶקָּטֵל | וָאֲקַטֵּל | וָאֶתְקַטֵּל | וָאַקְטִל |
| 3mp | וַיִּקְטְלוּ | וַיִּקָּטְלוּ | וַיְקַטְּלוּ | וַיִּתְקַטְּלוּ | וַיַּקְטִילוּ |
| 1cp | וַנִּקְטֹל | וַנִּקָּטֵל | וַנְקַטֵּל | וַנִּתְקַטֵּל | וַנַּקְטֵל |

The 1cs sometimes has a cohortative *-â* (וָאֶשְׁלְחָה), especially in later books (GKC §49; Driver §69). Note the dropped dagesh of the prefix before shewa (וַיְקַטֵּל, not *wayyəqaṭṭēl*; GKC §20 m).

**פקד in the Torah.** וַיִּפְקֹד 3ms Qal 5× (GEN.40.4, NUM.3.16 …), וָאֶפְקֹד 1cs (LEV.18.25), Hiphil וַיַּפְקִדֵהוּ (GEN.39.4).

### 4.4 Imperative, infinitives, participles

| | Qal | Niphal | Piel | Pual | Hithpael | Hiphil | Hophal |
|---|---|---|---|---|---|---|---|
| imperative 2ms | קְטֹל *qəṭōl* | הִקָּטֵל *hiqqāṭēl* | קַטֵּל *qaṭṭēl* | — | הִתְקַטֵּל *hiṯqaṭṭēl* | הַקְטֵל *haqṭēl* | — |
| imperative 2fs | קִטְלִי *qiṭlî* | הִקָּטְלִי | קַטְּלִי | — | הִתְקַטְּלִי | הַקְטִילִי | — |
| imperative 2mp | קִטְלוּ *qiṭlû* | הִקָּטְלוּ | קַטְּלוּ | — | הִתְקַטְּלוּ | הַקְטִילוּ | — |
| imperative 2fp | קְטֹלְנָה *qəṭōlnâ* | הִקָּטַלְנָה | קַטֵּלְנָה | — | הִתְקַטֵּלְנָה | הַקְטֵלְנָה | — |
| infinitive construct | קְטֹל *qəṭōl* | הִקָּטֵל *hiqqāṭēl* | קַטֵּל *qaṭṭēl* | (קֻטַּל) | הִתְקַטֵּל | הַקְטִיל *haqṭîl* | (הָקְטַל) |
| infinitive absolute | קָטוֹל *qāṭôl* | נִקְטֹל, הִקָּטֹל | קַטֹּל, קַטֵּל | קֻטֹּל | הִתְקַטֵּל | הַקְטֵל *haqṭēl* | הָקְטֵל |
| participle m. sg. | קֹטֵל *qōṭēl* (act.), קָטוּל *qāṭûl* (pass.) | נִקְטָל *niqṭāl* | מְקַטֵּל *məqaṭṭēl* | מְקֻטָּל *məquṭṭāl* | מִתְקַטֵּל *miṯqaṭṭēl* | מַקְטִיל *maqṭîl* | מָקְטָל *moqṭāl* |
| participle f. sg. | קֹטֶלֶת, קֹטְלָה; קְטוּלָה | נִקְטֶלֶת, נִקְטָלָה | מְקַטֶּלֶת | מְקֻטֶּלֶת | מִתְקַטֶּלֶת | מַקְטֶלֶת, מַקְטִילָה | מָקְטֶלֶת |
| participle m. pl. | קֹטְלִים; קְטוּלִים | נִקְטָלִים | מְקַטְּלִים | מְקֻטָּלִים | מִתְקַטְּלִים | מַקְטִילִים | מָקְטָלִים |
| participle f. pl. | קֹטְלוֹת; קְטוּלוֹת | נִקְטָלוֹת | מְקַטְּלוֹת | מְקֻטָּלוֹת | מִתְקַטְּלוֹת | מַקְטִילוֹת | מָקְטָלוֹת |

The passive stems Pual and Hophal have no imperative (GKC §46, §52, §53). The Qal f. sg. participle has two shapes, the segolate קֹטֶלֶת and the *-â* form קֹטְלָה; their history is studied by Price (2014), who reconstructs the first as the older one.

**פקד in the Torah.** imperative פְּקֹד (NUM.3.15, 3.40); Hiphil imperative הַפְקֵד (NUM.1.50); infinitive construct פְּקֹד (EXO.30.12); infinitive absolute פָּקֹד 4× (GEN.50.25, EXO.3.16, EXO.13.19, EXO.32.34); active participle פֹּקֵד 4× (DEU.5.9 "visiting the iniquity"); passive participle פְּקֻדִים / פְּקֻדֵי 77× — the census lists of Numbers ("the numbered of …"), which make פָּקַד's Torah profile a passive-participle profile.

## 5. Qatal (the perfect)

The qatal presents a situation as a whole, typically one that is done or holds (Driver §§7–20; GKC §106; JM §112). In prose it is used:

1. **For past events off the main narrative line**, especially after a subject or other word put first (X-qatal), giving background, a pluperfect or a contrast (Driver §§16–17, §76; on the information structure of X-qatal see Schiller 2020):

> **GEN.13.12** · `wlc:GEN:330`
> אַבְרָם יָשַׁב בְּאֶרֶץ־כְּנָעַן וְלוֹט יָשַׁב בְּעָרֵי הַכִּכָּר
> *ʾaḇrām yāšaḇ bəʾereṣ-kənāʿan wəlôṭ yāšaḇ bəʿārê hakkikkār* — Abram dwelt in-land-of-Canaan and-Lot dwelt in-cities-of the-plain — "Abram dwelt in the land of Canaan, and Lot dwelt in the cities of the Plain." (JPS 1917) — two subject-first qatal clauses in contrast.

2. **In speech, for the past and the present perfect**: יָדַעְתִּי "I know (have come to know)" (GEN.22.12).

3. **For states**, especially of stative verbs, with present reference: כָּבְדָה "is heavy" (GEN.18.20; §11).

4. **As a performative** — the act is done by saying it: "I hereby give" (GKC §106 i, m; Andrason 2012 calls this the performative qatal and derives it from the perfect's present-perfect value):

> **GEN.23.11** · `wlc:GEN:582`
> הַשָּׂדֶה נָתַתִּי לָךְ וְהַמְּעָרָה אֲשֶׁר־בּוֹ לְךָ נְתַתִּיהָ
> *haśśāḏe nāṯattî lāḵ wəhammǝʿārâ ʾăšer-bô ləḵā nəṯattîhā* — the-field I-give to-you and-the-cave which-in-it to-you I-give-it — "the field give I thee, and the cave that is therein, I give it thee." (JPS 1917)

5. **For certainty about the future** ("prophetic perfect"), mostly in poetry and prophecy (GKC §106 n).

6. **In conditional and wish clauses** after לוּ, לוּלֵא (GKC §106):

> **GEN.43.10** · `wlc:GEN:1300`
> כִּי לוּלֵא הִתְמַהְמָהְנוּ כִּי־עַתָּה שַׁבְנוּ זֶה פַעֲמָיִם
> *kî lûlēʾ hiṯmahmāhnû kî-ʿattâ šaḇnû ze p̄aʿămāyim* — for unless we-had-lingered surely-now we-had-returned this twice — "For except we had lingered, surely we had now returned a second time." (JPS 1917)

## 6. Yiqtol (the imperfect)

The yiqtol presents a situation as not (yet) complete or as not factual: future, repeated, customary, possible, permitted, obligatory (Driver §§21–43; GKC §107; JM §113). Its main uses:

1. **Future** in speech: יְהוָה יִלָּחֵם לָכֶם "the LORD will fight for you" (EXO.14.14).
2. **Habitual past or present**, "used to" (GKC §107).
3. **Modal**: "may, can, must, should" — the commands and prohibitions of the law are yiqtol, mostly in the 2nd person with לֹא:

> **NUM.3.15** (end) · `wlc:NUM:102`
> תִּפְקְדֵם *tip̄qəḏēm* — "you shall number them".

> **DEU.1.29** · `wlc:DEU:28`
> וָאֹמַר אֲלֵכֶם לֹא־תַעַרְצוּן וְלֹא־תִירְאוּן מֵהֶם
> *wāʾōmar ʾălēḵem lōʾ-ṯaʿarṣûn wəlōʾ-ṯîrəʾûn mēhem* — and-I-said to-you not-you-shall-dread and-not-you-shall-fear from-them — "Then I said unto you: 'Dread not, neither be afraid of them.'" (JPS 1917)

4. **In subordinate clauses** of purpose and result after לְמַעַן, פֶּן, אֲשֶׁר (GKC §107).
5. **Past with אָז, טֶרֶם, בְּטֶרֶם** — "then he sang", "before …" (GKC §107).

Some recent work treats yiqtol as one "panchronic" gram whose meanings (imperfective, future, modal) are stages of a single grammaticalization path from a present (Andrason 2010), or reads some of its past uses as an evidential strategy (Olivero 2023; known to us from its abstract only).

**Negation.** Yiqtol takes לֹא; the jussive takes אַל (§8.2). לֹא + yiqtol is the permanent prohibition of the law (לֹא תִרְצָח), אַל + jussive the immediate one ("do not do this now") (GKC §107, §109; JM §114).

## 7. Wayyiqtol and weqatal: the forms with consecutive waw

### 7.1 Wayyiqtol

Past narrative in Hebrew prose is a chain of wayyiqtol clauses, each opening with its verb (GKC §111; Driver §§66–85; JM §118). Each link usually moves the story on in time or logic; a clause that does not (background, simultaneous, contrastive) is switched off the chain by putting something else first and using a qatal (§5) or a participle or a verbless clause.

> **GEN.1.3** · `wlc:GEN:2`
> וַיֹּאמֶר אֱלֹהִים יְהִי אוֹר וַיְהִי־אוֹר
> *wayyōʾmer ʾĕlōhîm yəhî ʾôr wayhî-ʾôr* — and-said God let-be light and-was light — "And God said: 'Let there be light.' And there was light." (JPS 1917) — wayyiqtol, jussive, and the short wayyiqtol וַיְהִי of הָיָה, all three in one verse.

> **EXO.2.15** · `wlc:EXO:36`
> וַיִּשְׁמַע פַּרְעֹה … וַיְבַקֵּשׁ לַהֲרֹג אֶת־מֹשֶׁה וַיִּבְרַח מֹשֶׁה … וַיֵּשֶׁב בְּאֶרֶץ־מִדְיָן וַיֵּשֶׁב עַל־הַבְּאֵר
> *wayyišmaʿ parʿō … waybaqqēš lahărōḡ ʾeṯ-mōše wayyiḇraḥ mōše … wayyēšeḇ bəʾereṣ-miḏyān wayyēšeḇ ʿal-habbəʾēr* — "Now when Pharaoh heard this thing, he sought to slay Moses. But Moses fled … and dwelt in the land of Midian; and he sat down by a well." (JPS 1917) — five wayyiqtols in a row (Qal, Piel, Qal, Qal, Qal).

**Origin.** The wayyiqtol continues the old Semitic short prefix form *yaqtul*, a past perfective, attested also in the Amarna letters from Canaan (14th c. BCE), and not the imperfect *yaqtulu* (Baranowski 2016; Cook 2008; GKC §49 note). This is why it agrees in shape with the jussive (also from *yaqtul*) rather than with the yiqtol.

**Beyond sequence.** The wayyiqtol can sum up, explain, or (rarely) refer back to a time before the preceding event. Collins (1995) argues that an unmarked pluperfect use exists and can be recognised under stated conditions (his example GEN.2.19: the animals had been formed before they were brought); Andrason (2011) gives the form a "dynamic" definition with a spread of taxis, aspect and tense values rather than one core meaning.

> **GEN.2.19** · `wlc:GEN:49`
> וַיִּצֶר יְהוָה אֱלֹהִים מִן־הָאֲדָמָה כָּל־חַיַּת הַשָּׂדֶה … וַיָּבֵא אֶל־הָאָדָם
> *wayyiṣer YHWH ʾĕlōhîm min-hāʾăḏāmâ kol-ḥayyaṯ haśśāḏe … wayyāḇēʾ ʾel-hāʾāḏām* — and-formed YHWH God from-the-ground every-beast-of the-field … and-brought to-the-man — "And out of the ground the LORD God formed every beast of the field … and brought them unto the man." (JPS 1917)

**וַיְהִי.** The wayyiqtol of הָיָה is the commonest single verb form of the Torah (187× 3ms in the trees; DEU.1.3, GEN.38.27 …). Before a time phrase it opens a new scene, "and it came to pass" (GKC §111); its discourse use is treated in the chapter on discourse.

### 7.2 Weqatal

Weqatal is וְ + qatal, often with the stress moved to the last syllable in the 1cs and 2ms (וְקָטַלְתָּ֫; GKC §49). It continues a yiqtol, an imperative, a participle or an infinitive of future, habitual or instructive time: it is the chain form of the non-past, as the wayyiqtol is of the past (GKC §112; Driver §§105–129; JM §119).

> **EXO.6.7** · `wlc:EXO:129`
> וְלָקַחְתִּי אֶתְכֶם לִי לְעָם וְהָיִיתִי לָכֶם לֵאלֹהִים וִידַעְתֶּם כִּי אֲנִי יְהוָה
> *wəlāqaḥtî ʾeṯḵem lî ləʿām wəhāyîṯî lāḵem lēʾlōhîm wîḏaʿtem kî ʾănî YHWH* — and-I-will-take you to-me as-a-people and-I-will-be to-you as-God and-you-shall-know that I YHWH — "and I will take you to Me for a people, and I will be to you a God; and ye shall know that I am the LORD." (JPS 1917)

> **EXO.3.16** · `wlc:EXO:62`
> לֵךְ וְאָסַפְתָּ אֶת־זִקְנֵי יִשְׂרָאֵל וְאָמַרְתָּ אֲלֵהֶם
> *lēḵ wəʾāsap̄tā ʾeṯ-ziqnê yiśrāʾēl wəʾāmartā ʾălēhem* — go and-gather OM-elders-of Israel and-say to-them — "Go, and gather the elders of Israel together, and say unto them." (JPS 1917) — weqatal continuing an imperative.

In the laws of Leviticus and the instructions for the Tabernacle long weqatal chains carry the procedure ("and he shall take … and he shall slaughter … and the priest shall sprinkle"). Cook (2008) argues that weqatal, like the volitives, belongs to the modal (irreal) side of the system and that teaching it as an "inverted" perfect misleads beginners; Joosten likewise places weqatal and yiqtol together on the modal side (Dyck 2015 compares the two theories on Psalms 1–41).

**Weqatal or qatal with simple *waw*?** Not every וְ + qatal is consecutive. When the qatal is past and coordinate (וְהֶאֱמִן "and he believed", GEN.15.6, if read as past), grammarians speak of a simple waw; MACULA settles the reading form by form, and GEN.15.6 is coded `Type=weqatal` in the trees. Driver's chapter IX treats these cases (§§130–134).

## 8. The volitives: imperative, jussive, cohortative

Hebrew has a full volitive paradigm across the persons: cohortative for the 1st person, imperative for the 2nd (positive), jussive for the 3rd and for the negative 2nd (GKC §48, §§108–110; Driver §§44–58; JM §114).

### 8.1 Imperative

Formed from the 2nd person yiqtol without its prefix (תִּקְטֹל → קְטֹל). It is used only for positive commands; a negative command uses אַל + jussive or לֹא + yiqtol (GKC §46, §110). The imperative is often softened with נָא "please" and lengthened with ־ָה (§12.2).

> **DEU.3.27** · `wlc:DEU:109`
> עֲלֵה רֹאשׁ הַפִּסְגָּה וְשָׂא עֵינֶיךָ … וּרְאֵה בְעֵינֶיךָ
> *ʿălē rōʾš happisgâ wəśāʾ ʿênêḵā … ûrəʾē ḇəʿênêḵā* — go-up top-of the-Pisgah and-lift your-eyes … and-see with-your-eyes — "Get thee up into the top of Pisgah, and lift up thine eyes … and behold with thine eyes." (JPS 1917) — three imperatives of weak roots (III-he, I-nun + III-aleph, III-he).

> **GEN.27.19** · `wlc:GEN:746`
> קוּם־נָא שְׁבָה וְאָכְלָה מִצֵּידִי
> *qûm-nāʾ šəḇâ wəʾoḵlâ miṣṣêḏî* — rise-please sit and-eat from-my-game — "Arise, I pray thee, sit and eat of my venison." (JPS 1917) — imperatives with נָא and with paragogic *-â*.

A sequence imperative + imperative or imperative + weqatal/cohortative expresses purpose or consequence ("do this, that you may live") (GKC §110).

### 8.2 Jussive

The jussive is the short form of the 3rd (and 2nd) person yiqtol. In the strong Qal verb it looks like the yiqtol; it is distinct in the Hiphil (יַקְטֵל vs. יַקְטִיל) and in weak verbs (יְהִי vs. יִהְיֶה; יָקֹם vs. יָקוּם) (GKC §48, §109). It expresses wish, blessing, permission, indirect command, and with אַל a prohibition.

> **NUM.6.24–25** · `wlc:NUM:242`, `wlc:NUM:243`
> יְבָרֶכְךָ יְהוָה וְיִשְׁמְרֶךָ׃ יָאֵר יְהוָה פָּנָיו אֵלֶיךָ וִיחֻנֶּךָּ׃
> *yəḇāreḵḵā YHWH wəyišmərekā; yāʾēr YHWH pānāyw ʾēlêḵā wîḥunnekkā* — may-he-bless-you YHWH and-may-he-keep-you; may-make-shine YHWH his-face toward-you and-be-gracious-to-you — "The LORD bless thee, and keep thee; The LORD make His face to shine upon thee, and be gracious unto thee." (JPS 1917)

> **GEN.22.12** · `wlc:GEN:559`
> אַל־תִּשְׁלַח יָדְךָ אֶל־הַנַּעַר וְאַל־תַּעַשׂ לוֹ מְאוּמָּה
> *ʾal-tišlaḥ yāḏəḵā ʾel-hannaʿar wəʾal-taʿaś lô məʾûmmâ* — not-send your-hand to-the-lad and-not-do to-him anything — "Lay not thy hand upon the lad, neither do thou any thing unto him." (JPS 1917) — אַל + 2nd person jussive; תַּעַשׂ is the short (apocopated) form of תַּעֲשֶׂה.

> **GEN.41.34** · `wlc:GEN:1229`
> יַעֲשֶׂה פַרְעֹה וְיַפְקֵד פְּקִדִים עַל־הָאָרֶץ
> *yaʿăśe p̄arʿō wəyap̄qēḏ pəqiḏîm ʿal-hāʾāreṣ* — let-do Pharaoh and-let-him-appoint overseers over-the-land — "Let Pharaoh do this, and let him appoint overseers over the land." (JPS 1917) — the Hiphil jussive יַפְקֵד with *ē*, against the yiqtol יַפְקִיד.

Driver (Appendix II) lists the places where a jussive form occurs without jussive meaning, mostly in poetry and for metrical reasons.

### 8.3 Cohortative

The 1st person yiqtol with ־ָה *-â*: "let me / let us", a resolve or a request for permission (GKC §48, §108; JM §114).

> **GEN.33.12** · `wlc:GEN:972`
> וַיֹּאמֶר נִסְעָה וְנֵלֵכָה וְאֵלְכָה לְנֶגְדֶּךָ
> *wayyōʾmer nisʿâ wənēlēḵâ wəʾēlḵâ ləneḡdeḵā* — and-he-said let-us-set-out and-let-us-go and-I-will-go before-you — "And he said: 'Let us take our journey, and let us go, and I will go before thee.'" (JPS 1917)

> **GEN.46.30** · `wlc:GEN:1416`
> אָמוּתָה הַפָּעַם אַחֲרֵי רְאוֹתִי אֶת־פָּנֶיךָ
> *ʾāmûṯâ happāʿam ʾaḥărê rəʾôṯî ʾeṯ-pānêḵā* — let-me-die this-time after my-seeing OM-your-face — "Now let me die, since I have seen thy face." (JPS 1917)

## 9. The infinitives

### 9.1 Infinitive construct

The infinitive construct (Qal קְטֹל) is a verbal noun: it takes prepositions (ל־, בְּ־, כְּ־, מִן), a subject or object suffix, and an object with אֵת (GKC §§114–115; JM §124; Doron 2020 argues that the two infinitives are one verb with two allomorphs, and that the infinitive is verbal rather than nominal).

- with **לְ־** it gives purpose, intention or complement: לָלֶדֶת "to give birth" (GEN.25.24), לִלְקֹט "to gather" (EXO.16.27); the commonest of all is לֵאמֹר "saying", which introduces direct speech (GKC §114);
- with **בְּ־ / כְּ־** it gives a temporal clause: בְּצֵאתְכֶם "when you came out" (DEU.25.17);
- a pronoun suffix on it is its **subject**: לִדְתָּהּ "her giving birth" (GEN.38.27), or object.

> **GEN.38.27** · `wlc:GEN:1146`
> וַיְהִי בְּעֵת לִדְתָּהּ וְהִנֵּה תְאוֹמִים בְּבִטְנָהּ
> *wayhî bəʿēṯ liḏtāh wəhinnē ṯəʾômîm bəḇiṭnāh* — and-it-was at-time-of her-giving-birth and-behold twins in-her-womb — "And it came to pass in the time of her travail, that, behold, twins were in her womb." (JPS 1917)

The infinitive construct is negated with לְבִלְתִּי (לְבִלְתִּי שַׁלְּחָם "not letting them go", EXO.9.17) (GKC §114; Ehrensvärd 2022 studies the negation of the infinitive across the corpus).

### 9.2 Infinitive absolute

The infinitive absolute (Qal קָטוֹל) has no suffixes and no prepositions (GKC §113; JM §123). Its uses:

1. **Emphasis (paronomasia)**: before a finite verb of the same root it strengthens or makes certain — "surely, indeed". This is the "infinitive absolute + finite" construction of the laws (מוֹת יוּמָת, 31 Qal *môṯ* in the Torah) and of promises (פָּקֹד יִפְקֹד) (GKC §113):

> **EXO.21.15** · `wlc:EXO:559`
> וּמַכֵּה אָבִיו וְאִמּוֹ מוֹת יוּמָת
> *ûmakkē ʾāḇîw wəʾimmô môṯ yûmāṯ* — and-striker-of his-father and-his-mother dying he-shall-be-put-to-death — "And he that smiteth his father, or his mother, shall be surely put to death." (JPS 1917) — infinitive absolute of the Qal before a Hophal yiqtol.

> **GEN.31.30** · `wlc:GEN:903`
> וְעַתָּה הָלֹךְ הָלַכְתָּ כִּי־נִכְסֹף נִכְסַפְתָּה לְבֵית אָבִיךָ
> *wəʿattâ hālōḵ hālaḵtā kî-niḵsōp̄ niḵsap̄tâ ləḇêṯ ʾāḇîḵā* — and-now going you-went for-longing you-longed for-house-of your-father — "And now that thou art surely gone, because thou sore longest after thy father's house …" (JPS 1917) — two paronomastic pairs, Qal and Niphal.

2. **Manner or continuation**, after the finite verb: "going on and on" (GKC §113):

> **GEN.12.9** · `wlc:GEN:307`
> וַיִּסַּע אַבְרָם הָלוֹךְ וְנָסוֹעַ הַנֶּגְבָּה
> *wayyissaʿ ʾaḇrām hālôḵ wənāsôaʿ hanneḡbâ* — and-journeyed Abram going and-journeying toward-the-Negev — "And Abram journeyed, going on still toward the South." (JPS 1917)

3. **Command**, in place of an imperative (GKC §113):

> **DEU.25.17** · `wlc:DEU:671`
> זָכוֹר אֵת אֲשֶׁר־עָשָׂה לְךָ עֲמָלֵק בַּדֶּרֶךְ בְּצֵאתְכֶם מִמִּצְרָיִם
> *zāḵôr ʾēṯ ʾăšer-ʿāśâ ləḵā ʿămālēq badderek bəṣēʾṯəḵem mimmiṣrāyim* — remembering OM what did to-you Amalek on-the-way in-your-going-out from-Egypt — "Remember what Amalek did unto thee by the way as ye came forth out of Egypt." (JPS 1917)

The Decalogue's זָכוֹר (EXO.20.8) and שָׁמוֹר (DEU.5.12) are of this kind.

4. **In place of a finite verb** continuing a chain (GKC §113), more often in later books.

## 10. The participles

The participle is in form a noun or adjective (it takes gender, number, the article and the construct state) and in use a verb (it takes objects and adverbs) (Driver §135; GKC §116; JM §121; Joosten 2020 argues that it is a full component of the verbal system, the form of the actual present). Its uses:

1. **Ongoing or imminent action**, as the predicate of a verbless clause — present, past or future by context ("he was sitting", "they are about to go out"):

> **EXO.13.4** · `wlc:EXO:338`
> הַיּוֹם אַתֶּם יֹצְאִים בְּחֹדֶשׁ הָאָבִיב
> *hayyôm ʾattem yōṣəʾîm bəḥōḏeš hāʾāḇîḇ* — the-day you going-out in-month-of the-Abib — "This day ye go forth in the month Abib." (JPS 1917)

> **GEN.1.2** · `wlc:GEN:1`
> וְרוּחַ אֱלֹהִים מְרַחֶפֶת עַל־פְּנֵי הַמָּיִם
> *wərûaḥ ʾĕlōhîm məraḥep̄eṯ ʿal-pənê hammāyim* — and-spirit-of God hovering over-face-of the-waters — "and the spirit of God hovered over the face of the waters." (JPS 1917) — a Piel participle (f. sg.) as the predicate of a background clause.

2. **After הִנֵּה**, for what is seen happening: וְהִנֵּה … (GKC §116).

3. **As an agent noun**, "one who does": שֹׁכֵב "one who lies" (EXO.22.18), מַכֵּה "one who strikes" (EXO.21.12, 21.15); with the article it is equivalent to a relative clause (הַמּוֹצִיא "who brought out", EXO.6.7) (GKC §116, q).

4. **Passive participle** of the Qal (קָטוּל) and of the passive stems: a resultant state, "blessed, cursed, written".

> **DEU.28.5** · `wlc:DEU:723`
> בָּרוּךְ טַנְאֲךָ וּמִשְׁאַרְתֶּךָ
> *bārûḵ ṭanʾăḵā ûmišʾarteḵā* — blessed your-basket and-your-kneading-trough — "Blessed shall be thy basket and thy kneading-trough." (JPS 1917)

The participle grows in importance after the biblical period: in Mishnaic Hebrew it is the regular present, and in Modern Hebrew the present tense (Price 2014; see the Mishnaic Hebrew chapter).

## 11. Stative verbs

Verbs of quality and state ("be heavy, be small, be old, be full, fear, love") form a Qal subclass with a different vowel pattern (GKC §43; JM §41):

| type | qatal 3ms | yiqtol 3ms | example |
|---|---|---|---|
| *qāṭēl* (e-stative) | כָּבֵד *kāḇēḏ* "be heavy" | יִכְבַּד *yiḵbaḏ* | זָקֵן "be old", מָלֵא "be full", יָרֵא "fear", טָמֵא "be unclean" |
| *qāṭōl* (o-stative) | קָטֹן *qāṭōn* "be small" | יִקְטַן *yiqṭan* | יָכֹל "be able" (yiqtol יוּכַל) |
| active | קָטַל *qāṭal* | יִקְטֹל *yiqṭōl* | — |

The statives take *a* in the yiqtol and imperative (יִכְבַּד, not *yiḵbōḏ*), and their qatal often has present meaning: "it is heavy", "I am small". Many also have an adjective of the same shape (כָּבֵד "heavy", זָקֵן "old"). The active/stative split is not clean: Sato (2021) shows that many so-called stative verbs are ambiguous between a state and an entry into the state (inchoative).

> **GEN.18.20** · `wlc:GEN:444`
> וְחַטָּאתָם כָבְדָה מְאֹד
> *wəḥaṭṭāʾṯām kāḇəḏâ məʾōḏ* — and-their-sin is-heavy very — "and, verily, their sin is exceeding grievous." (JPS 1917)

> **GEN.32.11** · `wlc:GEN:938`
> קָטֹנְתִּי מִכֹּל הַחֲסָדִים וּמִכָּל־הָאֱמֶת
> *qāṭōntî mikkōl haḥăsāḏîm ûmikkol-hāʾĕmeṯ* — I-am-small from-all the-kindnesses and-from-all-the-faithfulness — "I am not worthy of all the mercies, and of all the truth." (JPS 1917, numbered 32:10 in its English versification)

> **EXO.5.9** · `wlc:EXO:108`
> תִּכְבַּד הָעֲבֹדָה עַל־הָאֲנָשִׁים
> *tiḵbaḏ hāʿăḇōḏâ ʿal-hāʾănāšîm* — let-be-heavy the-work on-the-men — "Let heavier work be laid upon the men." (JPS 1917) — the stative yiqtol with *a*.

MACULA does not mark the stative class; in the trees כָּבֵד is a plain `HebBinyan=QAL` verb, and קְטַנָּה "small" (DEU.25.13) is an `ADJ`.

## 12. Paragogic and energic endings

### 12.1 Paragogic *nun*

The m. pl. (and 2fs) yiqtol may end in *-ûn* (*-în*) instead of *-û* (*-î*): יִשְׁמְעוּן, תַּחֲרִישׁוּן. Over 300 such forms occur in the Bible, mostly in older texts and at the end of a clause in pause; GKC (§47 m) takes it as the fuller original ending, preserved for emphasis. MACULA splits the *nun* off as a word of its own (`Type=paragogic_nun`, 105 Torah tokens; §16).

> **EXO.22.21** · `wlc:EXO:602`
> כָּל־אַלְמָנָה וְיָתוֹם לֹא תְעַנּוּן
> *kol-ʾalmānâ wəyāṯôm lōʾ ṯəʿannûn* — every-widow and-orphan not you-shall-afflict — "Ye shall not afflict any widow, or fatherless child." (JPS 1917)

### 12.2 Paragogic *he*

The ending ־ָה *-â* on the imperative (שִׁמְעָה, שְׁבָה, מִכְרָה) and on the 1st person yiqtol, where it makes the cohortative (§8.3) (GKC §48 i). MACULA marks 73 Torah tokens as `Type=paragogic_he`, split off as separate words.

> **GEN.25.31** · `wlc:GEN:689`
> וַיֹּאמֶר יַעֲקֹב מִכְרָה כַיּוֹם אֶת־בְּכֹרָתְךָ לִי
> *wayyōʾmer yaʿăqōḇ miḵrâ ḵayyôm ʾeṯ-bəḵōrāṯəḵā lî* — and-said Jacob sell today OM-your-birthright to-me — "And Jacob said: 'Sell me first thy birthright.'" (JPS 1917)

The *-â* is different from the directional *he* of nouns (הַנֶּגְבָּה "toward the Negev", GEN.12.9), which MACULA marks `Type=directional_he` (392 tokens).

### 12.3 Energic *nun*

Before an object suffix the yiqtol may insert a stressed syllable *-en(n)-*: יִשְׁמְרֶנְהוּ "he will guard him", contracted יִשְׁמְרֶנּוּ (GKC §58 i–l calls it the *Nûn energicum*). It occurs mainly in pause and gives the form more weight. The Song of Moses has three in one verse:

> **DEU.32.10** · `wlc:DEU:875`
> יִמְצָאֵהוּ בְּאֶרֶץ מִדְבָּר … יְסֹבְבֶנְהוּ יְבוֹנְנֵהוּ יִצְּרֶנְהוּ כְּאִישׁוֹן עֵינוֹ
> *yimṣāʾēhû bəʾereṣ miḏbār … yəsōḇəḇenhû yəḇônənēhû yiṣṣərenhû kəʾîšôn ʿênô* — he-found-him in-a-land-of desert … he-encircled-him he-cared-for-him he-guarded-him as-the-pupil-of his-eye — "He found him in a desert land … He compassed him about, He cared for him, He kept him as the apple of His eye." (JPS 1917) — Poel and Polel forms with energic and plain suffixes.

> **GEN.13.17** · `wlc:GEN:335`
> כִּי לְךָ אֶתְּנֶנָּה
> *kî ləḵā ʾettənennâ* — for to-you I-will-give-it — "for unto thee will I give it." (JPS 1917) — energic *-ennâ* (< *-enhā*).

## 13. Object suffixes on the verb

A pronoun object may be attached to the verb instead of standing with אֵת (GKC §§57–61; JM §61). The suffixes are those of the noun, with a connecting vowel: *a* after qatal, *ē/e* after yiqtol.

| object | on qatal 3ms קְטָלַ־ | on yiqtol 3ms יִקְטְלֵ־ | energic yiqtol |
|---|---|---|---|
| me | קְטָלַנִי *qəṭālanî* | יִקְטְלֵנִי *yiqṭəlēnî* | יִקְטְלֶנִּי |
| you (ms) | קְטָלְךָ *qəṭālḵā* | יִקְטָלְךָ *yiqṭolḵā* | יִקְטְלֶךָּ |
| you (fs) | קְטָלֵךְ *qəṭālēḵ* | יִקְטְלֵךְ *yiqṭəlēḵ* | — |
| him | קְטָלוֹ / קְטָלָהוּ *qəṭālô* | יִקְטְלֵהוּ *yiqṭəlēhû* | יִקְטְלֶנּוּ *yiqṭəlennû* |
| her | קְטָלָהּ *qəṭālāh* | יִקְטְלֶהָ *yiqṭəlehā* | יִקְטְלֶנָּה *yiqṭəlennâ* |
| us | קְטָלָנוּ *qəṭālānû* | יִקְטְלֵנוּ *yiqṭəlēnû* | — |
| you (mp) | קְטַלְכֶם *qəṭalḵem* | יִקְטָלְכֶם | — |
| them (m) | קְטָלָם *qəṭālām* | יִקְטְלֵם *yiqṭəlēm* | — |
| them (f) | קְטָלָן | יִקְטְלֵן | — |

Before a suffix the qatal's first vowel reduces (קָטַל → קְטָלַ־) and the 2ms *-tā* becomes *-t-* (קְטַלְתַּנִי), the 3fs *-â* becomes *-aṯ-* (קְטָלַתְנִי), the 2mp *-tem* becomes *-tû-* (קְטַלְתּוּנִי) (GKC §59).

> **GEN.42.8** · `wlc:GEN:1260`
> וַיַּכֵּר יוֹסֵף אֶת־אֶחָיו וְהֵם לֹא הִכִּרֻהוּ
> *wayyakkēr yôsēp̄ ʾeṯ-ʾeḥāyw wəhēm lōʾ hikkiruhû* — and-recognised Joseph OM-his-brothers and-they not they-recognised-him — "And Joseph knew his brethren, but they knew him not." (JPS 1917) — the object is a full noun with אֶת in the first clause and a suffix in the second.

> **GEN.3.13** · `wlc:GEN:68`
> הַנָּחָשׁ הִשִּׁיאַנִי וָאֹכֵל
> *hannāḥāš hiššîʾanî wāʾōḵēl* — the-serpent deceived-me and-I-ate — "The serpent beguiled me, and I did eat." (JPS 1917)

In MACULA the suffix is a word of its own (`PRON`, `Type=pronominal`) hanging on the verb as `obj` (§16).

## 14. Tense, aspect or mood: the debate

What qatal, yiqtol, wayyiqtol and weqatal mean has been argued for two centuries. The main positions, in order of appearance:

- **Tense**: the medieval Jewish grammarians and early modern Christian ones read qatal as past and yiqtol as future, and the *waw* as "converting" one into the other (the *waw conversivum*). The "conversion" label is still common in teaching grammars; Cook (2008) shows how much confusion it causes in elementary teaching.
- **Aspect (Ewald, Driver)**: the forms do not mark time but the speaker's view of the action — qatal as "completed", yiqtol as "nascent, incomplete, in progress" (Driver 1892, §§1–6: the perfect presents an action "as completed", the imperfect "as nascent"). Driver explains the consecutive forms as a perfect or imperfect seen as following from what precedes, and his *Treatise* remains the fullest catalogue of the uses (GKC §§106–112 follow a similar line).
- **Historical (comparative Semitic)**: the wayyiqtol continues an old preterite *yaqtul*, the yiqtol an old imperfect *yaqtulu*, the qatal a stative/resultative *qatil/qatal* that became a perfect (GKC §47 note; Baranowski 2016 on the Amarna evidence). On this view the system is a mixture of layers, and the "waw consecutive" is not a converter but the survival of an old past form in one syntactic slot.
- **Aspect-prominent (Cook)**: Cook argues that the opposition is primarily aspectual (perfective qatal and wayyiqtol vs. imperfective yiqtol and participle), with a separate modal axis that groups weqatal with the volitives (Cook 2008; see Dyck 2015 for an application).
- **Modal / relative tense (Joosten)**: Joosten groups the forms by realis and irrealis: qatal and wayyiqtol for the past, participle for the present, yiqtol and weqatal for the modal and future (Dyck 2015 contrasts Cook's and Joosten's models; Joosten 2020 on the participle).
- **Grammaticalization paths (Andrason)**: Andrason treats each form as a polysemous gram spread over a path of meanings (resultative → perfect → perfective → past for qatal; a present-imperfective path for yiqtol), so that its uses are explained by where on the path each one lies, not by one abstract meaning (Andrason 2010, 2011, 2012; and his recent *Folia Orientalia* papers, 2026, which argue against binary, static models).
- **Discourse (text-linguistic)**: the forms mark the main line and the background of a text type (narrative, instruction, prediction) more than time or aspect (Endo 1993 on the Joseph story; Longacre's school, cited in the discourse chapter).
- Isaksson's monograph *The Verb in Classical Hebrew* (Open Book Publishers, 2024) gives a comparative-historical account of the "clause-type" system; it is held under a non-commercial licence and is listed here, not summarised.

These views are not entirely exclusive: most current descriptions keep the historical layers, an aspectual core for qatal/yiqtol, and the discourse role of wayyiqtol/weqatal. For Mova the practical consequence is that the four forms must be kept apart in the data (they are: `Type=qatal/yiqtol/wayyiqtol/weqatal`) and not collapsed into a tense feature.

## 15. The verb in the Torah: counts

All counts from the trees on 2026-10-17. MISC `Type=` gives the form, FEATS `HebBinyan=` the stem.

### 15.1 Stems

| stem | tokens | share of verbs |
|---|---:|---:|
| QAL | 12 751 | 71.1% |
| HIPHIL | 2 069 | 11.5% |
| PIEL | 1 859 | 10.4% |
| NIPHAL | 883 | 4.9% |
| HITHPAEL | 179 | 1.0% |
| HOPHAL | 149 | 0.8% |
| PUAL | 92 | 0.5% |
| `UNKNOWN: V` (hishtaphel) | 24 | |
| POEL | 13 | |
| POLEL | 7 | |
| HITHPOLEL | 7 | |
| `UNKNOWN: U` (hothpaal) | 6 | |
| `QAL PASSIVE` | 6 | |
| PILPEL | 5 | |
| HITHPALPEL | 3 | |
| PALEL | 1 | |
| NITHPAEL | 1 | |

### 15.2 Form by stem

| Type | QAL | HIPHIL | PIEL | NIPHAL | HITHPAEL | HOPHAL | PUAL | other | total |
|---|---:|---:|---:|---:|---:|---:|---:|---:|---:|
| wayyiqtol | 3 241 | 388 | 383 | 97 | 55 | 9 | 3 | 11 | 4 187 |
| yiqtol | 2 578 | 466 | 323 | 282 | 35 | 71 | 5 | 16 | 3 776 |
| qatal | 1 763 | 291 | 399 | 163 | 12 | 18 | 39 | 9 | 2 694 |
| weqatal | 1 641 | 383 | 243 | 119 | 21 | 9 | 6 | 12 | 2 434 |
| infinitive construct | 1 251 | 201 | 216 | 49 | 12 | 4 | — | 6 | 1 739 |
| participle active | 933 | 139 | 139 | 110 | 21 | — | — | 130* | 1 472 |
| imperative | 533 | 103 | 93 | 37 | 16 | — | — | — | 782 |
| participle passive | 305 | — | — | 5 | — | 37 | 32 | 16* | 395 |
| infinitive absolute | 153 | 44 | 42 | 10 | 1 | 1 | 1 | 3 | 255 |
| jussive | 139 | 30 | 9 | 7 | 4 | — | — | 3 | 192 |
| cohortative | 97 | 14 | 12 | 4 | 2 | — | — | — | 129 |

\* The "other" column of the two participle rows includes participles of minor stems and words that MACULA classes as nouns or adjectives with a participle type (the `Type=` totals count all words, the stem columns only `VERB` words with a stem). The other cells are `VERB` tokens; some row totals therefore exceed the sum of the shown stems by a few words. MACULA's `participle_active` / `participle_passive` follow the OSHB morphology codes (participle vs. passive participle), so a Niphal participle is mostly coded "active" (110) although its meaning is passive.

Things to notice: the wayyiqtol is three quarters Qal; the Piel is the only stem where qatal outnumbers yiqtol (the narrative "he commanded, he spoke": צִוָּה, דִּבֶּר); the Niphal is strongest in the yiqtol (the passive of the laws: "it shall be eaten, it shall be done"); the Hophal lives almost only in the yiqtol יוּמַת "he shall be put to death" and the participles.

### 15.3 The thirty commonest verbs

| # | lemma | gloss | tokens | by stem |
|---:|---|---|---:|---|
| 1 | אָמַר | say | 1 373 | Qal 1 366, Niph 5, Hiph 2 |
| 2 | הָיָה | be, become | 1 049 | Qal 1 046, Niph 3 |
| 3 | עָשָׂה | do, make | 860 | Qal 824, Niph 36 |
| 4 | נָתַן | give | 645 | Qal 632, Niph 10, Hoph 3 |
| 5 | בּוֹא | come; Hiph bring | 617 | Qal 427, Hiph 179, Hoph 11 |
| 6 | דָבַר | speak | 415 | Piel 408, Qal 6, Hith 1 |
| 7 | רָאָה | see | 396 | Qal 326, Niph 49, Hiph 16, Hoph 4, Hith 1 |
| 8 | לָקַח | take | 393 | Qal 389, Pual 3, Hith 1 (+2 Qal passive) |
| 9 | יָצָא | go out; Hiph bring out | 347 | Qal 235, Hiph 111, Hoph 1 |
| 10 | אָכַל | eat | 328 | Qal 289, Niph 34, Hiph 5 |
| 11 | מוּת | die; Hiph kill | 303 | Qal 239, Hoph 43, Hiph 21 |
| 12 | צָוָה | command | 252 | Piel 246, Pual 6 |
| 13 | שָׁמַע | hear | 241 | Qal 233, Hiph 4, Niph 4 |
| 14 | הלך / הָלַךְ | go, walk | 227 + 85 | Qal 219 + 74, Hiph 8, Hith 11 |
| 15 | יָלַד | bear; Hiph beget | 205 | Qal 107, Hiph 64, Niph 11, Pual 11, Piel 10, Hith 1, Hoph 1 |
| 16 | עָלָה | go up; Hiph offer up | 201 | Qal 131, Hiph 60, Niph 10 |
| 17 | שָׁלַח | send | 201 | Piel 106, Qal 92, Hiph 2, Pual 1 |
| 18 | קָרָא | call | 200 | Qal 189, Niph 11 |
| 19 | יָשַׁב | sit, dwell | 199 | Qal 195, Hiph 3, Niph 1 |
| 20 | קָרַב | draw near; Hiph offer | 194 | Hiph 149, Qal 44, Niph 1 |
| 21 | שׁוּב | return | 180 | Qal 113, Hiph 63, Hoph 4 |
| 22 | יָדַע | know | 174 | Qal 154, Hiph 8, Niph 8, Hith 2, Hoph 2 |
| 23 | נָשָׂא | lift, carry | 168 | Qal 163, Hith 3, Hiph 1, Niph 1 |
| 24 | שׂים | put, set | 152 | Qal 151, Hoph 1 |
| 25 | שָׁמַר | keep, guard | 148 | Qal 127, Niph 21 |
| 26 | קוּם | rise | 145 | Qal 103, Hiph 41, Hoph 1 |
| 27 | בָּרַךְ | bless | 137 | Piel 108, Qal 20, Hith 3, Niph 3, Pual 2, Hiph 1 |
| 28 | פָּקַד | attend to, number | 134 | Qal 122, Hiph 5, Hothpaal 3, Niph 2, Hoph 1, Pual 1 |
| 29 | עָבַר | pass over | 130 | Qal 111, Hiph 18, Hith 1 |
| 30 | מָצָא | find | 122 | Qal 89, Niph 30, Hiph 3 |

837 distinct verb lemmas occur in the Torah. Of the top thirty, twenty are weak in at least one radical — the reason the weak verbs need their own chapter. The split lemma of "go" is MACULA's: forms that keep the ה (הָלַךְ, הֹלֵךְ, הָלוֹךְ) have the pointed lemma הָלַךְ, forms that behave like a I-yod verb (וַיֵּלֶךְ, לֵךְ, לֶכֶת, אֵלְכָה) have the bare lemma הלך; both have Strong 1980 (see [07-weak-verbs.md §10.1](07-weak-verbs.md#101-הלך-go)).

### 15.4 Argument frames by stem

MACULA's `Frame=` lists the argument slots of a verb with the ids of their heads: A0 (agent / first argument), A1 (patient / second), A2 (third), AA (the added causer of a causative). Grouped by stem (2026-10-17, `VERB` tokens):

| stem | commonest frames |
|---|---|
| Qal | A0 6 221; A0+A1 3 960; none 638; A1 265 |
| Hiphil | **AA+A0 1 243**; A0 301; AA+A0 (reversed order) 91; AA+A0+A1 49 |
| Piel | A0+A1 875; A0 606; A0+A2+A1 55 |
| Niphal | **A1 486**; A0 244 |
| Hophal | A1 122 |
| Pual | A1 69 |
| Hithpael | A0 130 |

The table shows the stems' grammar in the frames themselves: the Hiphil adds a causer (AA) to the Qal's agent, the passive stems (Niphal, Pual, Hophal) promote the patient (A1) to the only argument, and the Hithpael keeps one agent-like argument.

## 16. In Mova

**Where the verb lives in the trees.** Each verb is a token with UPOS `VERB`, its pointed form in FORM (with accents), the Qal 3ms citation form in LEMMA, and

- FEATS: `Gender`, `Number`, `Person` (Person is absent on infinitives and participles) and `HebBinyan` (the stem in capitals);
- MISC: `Type=` (qatal, yiqtol, wayyiqtol, weqatal, imperative, jussive, cohortative, infinitive_construct, infinitive_absolute, participle_active, participle_passive), `Gloss=` (MACULA's English, e.g. `he.created`, `let.it.be`), `Strong=`, `Frame=`.

There is no UD `VerbForm`, `Aspect`, `Mood`, `Tense` or `Voice` feature: MACULA's form type stays in MISC as `Type=` (tools/macula/README.md). The UD guidelines for Ancient Hebrew list these features for verbs (`hbo/feat/index` in the UD docs), so mapping `Type` to `VerbForm`/`Mood` (and `wayyiqtol`/`weqatal` to a feature of their own) is a gap.

**Segmentation.** MACULA splits prefixes and suffixes as words: the *waw* of wayyiqtol is a `CCONJ` token וַ (hanging on the verb as `cc`), the article and prepositions on infinitives are their own tokens, and **object suffixes are separate `PRON` tokens** (`Type=pronominal`, lemma הוּא) attached as `obj`; a suffix on an infinitive construct is its `nsubj` (לִדְתָּ + ־ָהּ, GEN.38.27). The verb's FORM is therefore often truncated (יְבָרֶכ + ְךָ in NUM.6.24) and its `Gloss` is sometimes moved onto the suffix (`Gloss=he.bless.you` on the suffix, `Gloss=may` on the verb). Paragogic *nun* and *he* are also split off as words (UPOS `X`, `Type=paragogic_nun` / `paragogic_he`); the paragogic *nun* hangs on its verb as `nmod` with lemma נ, which is a relation of no UD meaning (a known wart, §16 gaps).

**Relations.** The converter makes the verb the head of its clause (`root`, `conj` for later clauses of the chain, `ccomp` for a speech clause after a verb of saying, `advcl` for an infinitive after a preposition). An infinitive absolute before its finite verb is `advmod` (פָּקֹד in GEN.50.25). A chain of wayyiqtols becomes `root` + `conj` + `conj` …, which UD PTNK often reads as `parataxis` (788 train cases of PTNK `parataxis` against our `conj`; tools/macula/README.md).

**Pragmatics.** `macula prag` (`hebrew_prag()` in tools/macula/src/main.rs) reads the root's `Type` and `Person`: a 3rd-person wayyiqtol or qatal at the root is a narrator's frame (`narrate` / `assert`), an imperative, jussive or cohortative is `imp`/`request`, a 2nd-person yiqtol or weqatal at the root is a command of the law (`request`, `indirect=1`), and לֹא / אַל / אַיִן in the root's own clause make `polarity=neg`. Over the Torah (2026-10-17): 5 591 `decl`, 211 `imp`, 34 `wh`, 17 `q`; acts 2 869 assert, 2 010 narrate, 927 request, 47 ask. Root types: wayyiqtol 2 074, yiqtol 1 062, weqatal 748, qatal 436, imperative 173, jussive 30. Examples: GEN.1.3 → decl/narrate/narrator (the speech inside is not read: the frame wins); NUM.6.24 (jussive) → imp/request/character; NUM.3.15 (imperative) → imp/request; GEN.33.12 → narrate, because the root is the wayyiqtol וַיֹּאמֶר and the three cohortatives are inside its `ccomp`.

**The dictionary.** `torah-en.tsv` keys verbs by Strong's number (פָּקַד = 6485, נָתַן = 5414); each entry has MACULA's commonest glosses with counts (for 6485: "their.enrolled (41); he.enrolled (9)"), the OSHB short definition and the BDB entry. `torah-en.tsv.examples` gives two short verses per word with Hebrew, MACULA's word-by-word gloss and JPS 1917.

**Gaps and known faults (2026-10-17).**

- **FEATS values with spaces.** Three stem values are written as MACULA has them, with a space or a colon: `HebBinyan=QAL PASSIVE` (6 tokens), `HebBinyan=UNKNOWN: V` (24, the hishtaphel of חוה) and `HebBinyan=UNKNOWN: U` (6, the hothpaal of פקד). A space in FEATS is invalid CoNLL-U. They should be mapped to `QAL_PASSIVE` (or `QALPASS`), `HISHTAPHEL` and `HOTHPAAL`; the 21 tokens of חוה labelled `HITHPAEL` should then be relabelled too.
- **No stative class** and no mark of the Qal passive beyond those six tokens.
- **No `VerbForm`/`Mood`** in FEATS (see above).
- **Paragogic nun as `nmod`.** Better as part of the verb (no split) or as `dep`/`goeswith`-like attachment with a fixed relation.
- **prag misses two command forms.** The infinitive absolute used as a command (זָכוֹר, DEU.25.17 → read `decl/assert/narrator`) and the 3rd-person yiqtol of casuistic law (מוֹת יוּמָת, EXO.21.15 → `assert`) are not read as commands. Rules for both would be small: an `infinitive_absolute` at the root with no finite verb of the same lemma in the clause is `imp/request`; a Hophal or Niphal yiqtol at the root inside a law clause is a directive.
- **The Decalogue has no spaces** in `# text` for EXO.20 and DEU.5 (the double cantillation of the Ten Commandments joins words; e.g. `wlc:EXO:526`). The tokens are right; only the text line is affected.
- **Dictionary for verbs.** The BDB short field of a verb often collapses to its stem labels ("vb. Niph." for פָּקַד); the OSHB short definition of קוּם (6965) is wrong ("Leb Qamay", a name); הָיָה gets "fall out" from the neighbouring entry הָוָה. The extended-Strong verb חוה has neither.

## 17. Sources

- Gesenius, W.; Kautzsch, E. (ed.); Cowley, A. E. (tr.), 1910. *Gesenius' Hebrew Grammar*, 2nd English ed. Oxford. Public domain. §§30, 39–61, 75 kk, 106–116. Wikisource text: `$RESEARCH_RAW/hebgram/open/gesenius-kautzsch-cowley-1910-wikisource/gesenius-kautzsch-cowley-1910-wikisource.txt`; https://en.wikisource.org/wiki/Gesenius%27_Hebrew_Grammar; scan https://archive.org/details/geseniushebrewgr00geseuoft.
- Driver, S. R., 1892. *A Treatise on the Use of the Tenses in Hebrew and Some Other Syntactical Questions*, 3rd ed. Oxford. Public domain. Ch. I (§§1–6), II (§§7–20), III (§§21–43), IV (§§44–58), VI (§§66–85), VIII (§§105–129), IX (§§130–134), X (§135), App. II. `$RESEARCH_RAW/hebgram/pd/driver-tenses-1892/`; https://archive.org/details/treatiseonuseof00driv.
- Joüon, P., 1923. *Grammaire de l'hébreu biblique*. Rome. Public domain. §§40–41, 52, 54, 59, 61, 111–124. `$RESEARCH_RAW/hebgram/pd/jouon-grammaire-1923/`.
- Brown, F.; Driver, S. R.; Briggs, C. A., 1906. *A Hebrew and English Lexicon of the Old Testament*. Oxford. Public domain. pp. 823–824 (פקד), 1005 (שׁחה/חוה). Sefaria text `$RESEARCH_RAW/lexres/dictionaries/sefaria/BDB_Dictionary.jsonl`; https://archive.org/details/hebrewenglishlex00browuoft.
- Andrason, A., 2010. "The Panchronic Yiqtol: Functionally Consistent and Cognitively Plausible." *Journal of Hebrew Scriptures* 10. CC BY 2.5. `$RESEARCH_RAW/hbo-2010-andrason-panchronic-yiqtol-functionally-consistent-cognitively/`; https://jhsonline.org/index.php/jhs/article/view/11272.
- Andrason, A., 2011. "Biblical Hebrew Wayyiqtol: A Dynamic Definition." *JHS* 11, art. 8. CC BY 2.5. `$RESEARCH_RAW/hbo-2011-andrason-biblical-hebrew-wayyiqtol-dynamic-definition/`; https://jhsonline.org/index.php/jhs/article/view/11523.
- Andrason, A., 2012. "Making It Sound – The Performative Qatal and Its Explanation." *JHS* 12. CC BY 2.5. `$RESEARCH_RAW/hbo-2012-andrason-making-sound-performative-qatal-explanation/`; https://jhsonline.org/index.php/jhs/article/view/18390.
- Andrason, A., 2026. "Toward the Ocean of the Biblical Hebrew Verbal System" and "Against Binarism, Simplicity, Neatness and Stasis". *Folia Orientalia*. CC BY. `$RESEARCH_RAW/hbo-2026-andrason-toward-ocean-biblical-hebrew-verbal/`, `$RESEARCH_RAW/hbo-2026-andrason-against-binarism-simplicity-neatness-stasis/`; https://wydawnictwo.pan.pl/index.php/fo/article/view/7021.
- Baranowski, K. J., 2016. "The Biblical Hebrew wayyiqtol and the Evidence of the Amarna Letters from Canaan." *JHS* 16. `$RESEARCH_RAW/hbo-2016-baranowski-biblical-hebrew-wayyiqtol-evidence-amarna/`; https://jhsonline.org/index.php/jhs/article/view/29353.
- Benton, R. C., 2009. *Aspect and the Biblical Hebrew Niphal and Hitpael*. PhD thesis, University of Wisconsin. CC BY-NC-ND (cited, not quoted). `$RESEARCH_RAW/hbo-2009-benton-aspect-biblical-hebrew-niphal-hitpael/`; http://digital.library.wisc.edu/1793/36168.
- Collins, C. J., 1995. "The Wayyiqtol as 'Pluperfect': When and Why." *Tyndale Bulletin* 46. CC BY-SA (cited, not quoted). `$RESEARCH_RAW/hbo-1995-collins-wayyiqtol-pluperfect-why/`.
- Cook, J. A., 2008. "The Vav-Prefixed Verb Forms in Elementary Hebrew Grammar." *JHS* 8, art. 3. CC BY 2.5. `$RESEARCH_RAW/hbo-2008-cook-vav-prefixed-verb-forms-elementary/`; https://jhsonline.org/index.php/jhs/article/view/6201.
- Doron, E., 2020. "The Biblical Hebrew Infinitive." `$RESEARCH_RAW/hbo-2020-doron-biblical-hebrew-infinitive/`; https://zenodo.org/records/4749002.
- Dyck, J., 2015. *A Critical Engagement with the Biblical Hebrew Verbal System Theories of John Cook and Jan Joosten …* MA thesis, McMaster. `$RESEARCH_RAW/hbo-2015-dyck-critical-engagement-biblical-hebrew-verbal/`.
- Ehrensvärd, M., 2022. "Negating the Infinitive in Biblical Hebrew." *ZAH*. `$RESEARCH_RAW/hbo-2022-ehrensvard-negating-infinitive-biblical-hebrew/`.
- Endo, Y., 1993. "The Verbal System of Classical Hebrew in the Joseph Story: An Approach from Discourse Analysis." *Tyndale Bulletin* (summary). `$RESEARCH_RAW/hbo-1993-endo-verbal-system-classical-hebrew-joseph/`.
- Hutton, J. M.; Marzouk, S., 2012. "The Morphology of the tG-Stem in Hebrew and Tirgaltî in Hos 11:3." *JHS* 12. CC BY 2.5. `$RESEARCH_RAW/hbo-2012-hutton-morphology-tg-stem-hebrew-tirgalti/`.
- Isaksson, B., 2024. *The Verb in Classical Hebrew*. Open Book Publishers. Non-commercial licence: listed only. `$RESEARCH_RAW/hbobook-obp0414-isaksson-verb-classical-hebrew/`.
- Joosten, J., 2020. "The Participle as a Component of the Verbal System in Biblical Hebrew." *Brill's Journal of Afroasiatic Languages and Linguistics*. CC BY (catalogue entry; PDF not held). https://doi.org/10.1163/18776930-20201001.
- Olivero, V., 2023. "Yiqtol as Evidential Strategy in Biblical Hebrew." *Journal of Semitic Studies*. CC BY (catalogue entry; PDF not held). http://dx.doi.org/10.1093/jss/fgad004.
- Price, J. H., 2014. "The Biblical Hebrew Feminine Singular Qal Participle: A Historical Reconstruction." *JHS* 14. `$RESEARCH_RAW/hbo-2014-price-biblical-hebrew-feminine-singular-qal/`.
- Sato, H., 2021. *Semantic Ambiguity of So-Called "Stative Verbs" in Biblical Hebrew …* University of Toronto thesis (catalogue entry; PDF not held). http://hdl.handle.net/1807/107415.
- Schiller, A., 2020. "X-qatal: Information Structure …" *Protokolle zur Bibel*. `$RESEARCH_RAW/hbo-2020-schiller-x-qatal-information-structure-back/`.
- MACULA Hebrew Linguistic Datasets (Biblica, Clear Bible, Groves Center; text and morphology from OSHB), CC BY 4.0. https://github.com/Clear-Bible/macula-hebrew/. Converted by `tools/macula` (tools/macula/README.md).
- UD guidelines for Ancient Hebrew (features and relations index), CC BY-SA 4.0 — consulted, not copied. `$RESEARCH_RAW/hebgram/sa/ud-docs-he-hbo/_hbo/`.
- The Torah in JPS 1917 (public domain), from eBible.org: `$MOVA_DATA/corpora/judaism/text/torah-jps1917/`.
