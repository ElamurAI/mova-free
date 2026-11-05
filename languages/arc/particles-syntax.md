# Particles and syntax of Biblical Aramaic

The small words (the relative די, the object markers, prepositions, conjunctions, negation, the existential) and the way clauses are built. References: Rosenthal 1961 (particles; syntax); Bauer and Leander 1927 §§ 68–110 (particles and syntax); Kautzsch 1884; Greenspahn 2003; Buth 1987, *Word Order in Aramaic from the Perspectives of Functional Grammar and Discourse Analysis* (PhD, UCLA); Folmer 1995; Cook 2019. Examples from WLC (public domain).

## 1. The particle דִּי

דִּי (*dī*) is the most frequent word of Aramaic after ו "and". In Biblical Aramaic it is written as a separate word (often joined by maqqef); in later Aramaic it shrinks to a prefix ד־ (*də-*), written together with the next word ([../tmr/particles-syntax.md](../tmr/particles-syntax.md)). MACULA tags 287 relative particles (`Tr`) in the Aramaic chapters, nearly all דִּי (2026-10-17).

It has five uses:

| use | function | example | source |
|---|---|---|---|
| relative | introduces a relative clause: "who, which, that" | צֶלֶם דַּהֲבָא דִּי הֲקֵים נְבוּכַדְנֶצַּר "the golden image that Nebuchadnezzar set up" | Dan 3:5 |
| genitive | "of", between two nouns | גֻבָּא דִּי אַרְיָוָתָא "the den of lions" | Dan 6:17 |
| complementiser | "that", after verbs of knowing, saying, seeing | יְדַע דִּי־רְשִׁים כְּתָבָא "he knew that the document was signed" | Dan 6:11 |
| purpose / result | "so that", with the imperfect | דִּי לָא־תִשְׁנֵא צְבוּ "so that nothing might be changed" | Dan 6:18 |
| causal | "because", often in כָּל־קֳבֵל דִּי | כָּל־קֳבֵל דִּי־הֲוָא עָבֵד "just as he had been doing" | Dan 6:11 |

It also starts **direct speech** in some passages (like Greek ὅτι recitativum) and forms compound conjunctions: עַד דִּי "until", מִן־דִּי "after, since", כְּדִי "when, as", כָּל־קֳבֵל דִּי "because, inasmuch as", בְּדִיל דִּי (later) "in order that".

**Resumptive pronouns.** When the relativised noun is not the subject, a pronoun in the relative clause points back to it:

```
Aramaic:   אֱלָהָךְ דִּי אַנְתְּ פָּלַח־לֵהּ בִּתְדִירָא הוּא יְשֵׁיזְבִנָּךְ
Translit.: ʾĕlāhāḵ dī ʾant pālaḥ-lēh bi-ṯḏīrā hū yəšēzəḇinnāḵ
Gloss:     god-2MS REL you serve.PEAL.PTCP.ACT.MS-to-3MS in-continuity.DET he deliver.SHAPH.IPF.3MS-ENERG-2MS
English:   Your God, whom you serve continually — he will deliver you!
Source:    Dan 6:17 (WLC, public domain; Ketiv אנתה, Qere אַנְתְּ)
```

Here לֵהּ "to him" resumes אֱלָהָךְ, and הוּא resumes the whole fronted noun phrase (a *casus pendens*, §8).

## 2. Marking the direct object

Biblical Aramaic marks a definite direct object in four ways:

| way | frequency | example | source |
|---|---|---|---|
| no marker | most objects | עֲבַד צְלֵם "he made an image" | Dan 3:1 |
| ל־ before a determined object | frequent, mainly with persons | הַיְתִיו לְדָנִיֵּאל "they brought Daniel" | Dan 6:17 |
| יָת + suffix | once | דִּי־מַנִּיתָ יָתְהוֹן "whom you appointed" | Dan 3:12 |
| the independent pronoun הִמּוֹ | 3 m. pl. object | וְהוֹתֵב הִמּוֹ "and he settled them" | Ezra 4:10 |
| pronoun suffix on the verb | | שַׁכְלְלֵהּ "he completed it" | Ezra 5:11 |

The **object marker יָת** (Hebrew אֵת) appears **only once** in Biblical Aramaic (Dan 3:12). Old Aramaic had אית; Imperial Aramaic used ל־ and יָת (Folmer 1995). In **Onkelos** יָת becomes the marker of almost every definite object, because the targum renders Hebrew אֵת word for word ([../arc-x-targum/translation-syntax.md](../arc-x-targum/translation-syntax.md)). In JBA ל־ is the rule and יָת is gone except in fixed phrases and with suffixes ([../tmr/particles-syntax.md](../tmr/particles-syntax.md)).

The ל־ object marker is the same preposition as "to", so for a parser לְדָנִיֵּאל after הַיְתִיו "they brought" is ambiguous between object and goal; MACULA's syntax decides by the verb's frame ([mova.md](mova.md) §4).

## 3. Prepositions

| preposition | meaning | example | source |
|---|---|---|---|
| בְּ־ | in, with, by, at (time) | בַּהּ־שַׁעֲתָא "at that moment" | Dan 3:6 |
| לְ־ | to, for; object marker | לְמַלְכָּא "to the king" | Dan 2:4 |
| כְּ־ | like, as, about | כְּבַר אֱנָשׁ "like a son of man" | Dan 7:13 |
| מִן | from, than; by (agent); because of | מִן־עָלְמָא וְעַד־עָלְמָא "from age to age" | Dan 2:20 |
| עַל | on, over, to, against, concerning | עַל־יְרוּשְׁלֶם "concerning Jerusalem" | Ezra 4:8 |
| עַד | until, up to | עַד־סוֹפָא "to the end" | Dan 6:27 |
| עִם | with | עִם־עֲנָנֵי שְׁמַיָּא "with the clouds of heaven" | Dan 7:13 |
| קֳדָם | before (in front of; in the presence of) | קֳדָם אֱלָהֵהּ "before his God" | Dan 6:11 |
| מִן־קֳדָם | from before; by; because of | זָיְעִין וְדָחֲלִין מִן־קֳדָם אֱלָהֵהּ "trembling and fearing before his God" | Dan 6:27 |
| לָקֳבֵל | opposite, in front of | לָקֳבֵל נֶבְרַשְׁתָּא "opposite the lampstand" | Dan 5:5 |
| כָּל־קֳבֵל | because of, according to | כָּל־קֳבֵל דְּנָה "because of this" | Dan 2:12 |
| תְּחוֹת | under | מִן־תְּחוֹת שְׁמַיָּא "from under the heavens" | Jer 10:11 |
| בֵּין | between | | Dan 7:5 |
| בָּתַר | after | בָּתְרָךְ "after you" | Dan 2:39 |
| בְּגוֹא / לְגוֹא | inside, into | לְגוֹא־אַתּוּן נוּרָא "into the furnace of fire" | Dan 3:6 |
| לְוָת | to, beside | מִן־לְוָתָךְ "from you" | Ezra 4:12 (Kautzsch 1884 §68, p. 128: only in this phrase) |
| נֶגֶד | toward | נֶגֶד יְרוּשְׁלֶם "toward Jerusalem" | Dan 6:11 |
| עַל־דְּנָה | therefore | | Dan 3:16 |
| מִן־קַדְמַת | before (time) | מִן־קַדְמַת דְּנָה "before this" | Dan 6:11 |

Checked 2026-10-18: Kautzsch 1884 §68, pp. 126–129 (each preposition with its attestations).

**קֳדָם "before"** deserves attention: it is the preposition the targums use to avoid direct contact between God and humans ("he prayed before the LORD" for "he prayed to the LORD"); Biblical Aramaic already uses it with kings and God (Dan 2:9; 6:11) ([../arc-x-targum/translation-syntax.md](../arc-x-targum/translation-syntax.md) §3).

## 4. Conjunctions and adverbs

| word | meaning | example | source |
|---|---|---|---|
| וְ־ | and | | |
| אוֹ | or | | Dan 6:8 |
| הֵן | if; whether | הֵן אִיתַי אֱלָהַנָא … "if our God … exists" | Dan 3:17 |
| וְהֵן לָא | and if not | וְהֵן לָא יְדִיעַ לֶהֱוֵא־לָךְ "but if not, be it known to you" | Dan 3:18 |
| לָהֵן | but, except; therefore | לָהֵן חֶלְמָא אֱמַרוּ לִי "therefore tell me the dream" | Dan 2:9 |
| בְּרַם | but, however | | Dan 2:28; 4:12 |
| אֱדַיִן, בֵּאדַיִן | then | בֵּאדַיִן מַלְכָּא אֲמַר "then the king commanded" | Dan 6:17 |
| כְּעַן, כְּעֶנֶת, כְּעֶת | now | כְּעַן הֵן אִיתֵיכוֹן עֲתִידִין "now if you are ready" | Dan 3:15 |
| כְּדִי | when, as soon as | וְדָנִיֵּאל כְּדִי יְדַע "when Daniel knew" | Dan 6:11 |
| עַד דִּי | until | עַד דִּי כָרְסָוָן רְמִיו "until thrones were placed" | Dan 7:9 |
| מִן־דִּי | as soon as, after (temporal) | מִן־דִּי הַרְגִּזוּ אֲבָהֳתַנָא "after our fathers had angered" (often translated "because") | Ezra 5:12; also Ezra 4:23; Dan 4:23 (Kautzsch 1884 §69.6, p. 130; Bauer and Leander 1927 §70u, p. 264) |
| אַף | also | | Ezra 5:14 |
| כֹּל קֳבֵל דִּי | because | | Dan 2:8 |
| תִּנְיָנוּת | a second time | עֲנוֹ תִנְיָנוּת "they answered a second time" | Dan 2:7 |
| אָסְפַּרְנָא | diligently (Pers.) | אָסְפַּרְנָא יִתְעֲבִד "let it be done diligently" | Ezra 6:12 |
| אַדְרַזְדָּא | diligently (Pers.) | | Ezra 7:23 |
| תַּמָּה | there | שַׁכִּן שְׁמֵהּ תַּמָּה "caused his name to dwell there" | Ezra 6:12 |
| כָּה | here | עַד־כָּה "so far" | Dan 7:28 (the only occurrence) |
| כִּדְנָה, כְּנֵמָא | thus | | Jer 10:11; Ezra 4:8 |

Checked 2026-10-18: Kautzsch 1884 §67, pp. 123–125 (negations, adverbs of place and time, כְּעַן with the short כְּעֶנֶת / כְּעֶת only in letter headings, the Persian adverbs) and §69, pp. 129–131 (conjunctions). Corrected: "here" is כָּה, attested once. Second pass 2026-10-18: Bauer and Leander 1927 §70, pp. 262–266. Corrected: מִן־דִּי is temporal in both grammars (Bauer and Leander §70u: "as soon as", "after", Dan 4:23; Ezra 4:23; 5:12); neither lists it among the causal conjunctions (§70g′–h′: דִּי "because" Dan 2:47; 6:24, and כָּל־קֳבֵל דִּי), so the gloss "because" is a translators' inference, not a grammatical meaning, and the earlier disagreement is resolved. The other rows agree: כְּדִי "as soon as" (§70t), עַד דִּי (§70v), בְּרַם (§70o), לָהֵן "but" (§70q) and "therefore" (§68v, p. 256), the Persian adverbs אָסְפַּרְנָא and אַדְרַזְדָּא (§68y, p. 256).

## 5. Negation

| particle | use | example | source |
|---|---|---|---|
| לָא (also לָה) | general negation of verbs, nouns, clauses | לָא יִפֵּל "does not fall" | Dan 3:6 |
| אַל | negative command, with the jussive | אַל־יְבַהֲלוּךְ "let them not alarm you" | Dan 5:10 |
| לָא אִיתַי | there is not | לָא אִיתַי דִּי יְחַוִּנַּהּ "there is no one who can tell it" | Dan 2:11 |
| דִּי־לָא | without, which not | אֶבֶן דִּי־לָא בִידַיִן "a stone (cut) without hands" | Dan 2:45 |

Checked 2026-10-18: Kautzsch 1884 §67.1, p. 123: לָא, written לָה once (Dan 4:32, כְּלָה); אַל only before the imperfect (Dan 2:24; 4:16; 5:10).

Later Aramaic adds the fused לֵית "there is not" (Onkelos, JPA) and the JBA לָאו "not (it is not that)" and לֵיכָּא ([family.md](family.md) §5).

## 6. Existence and possession

"There is" is אִיתַי; "X has Y" is "there is to X Y" (ל־ + possessor):

```
Aramaic:   וַחֲבָל לָא־אִיתַי בְּהוֹן
Translit.: wa-ḥăḇāl lā-ʾīṯay bəhōn
Gloss:     and-harm NEG-EXIST in-3MPL
English:   and there is no harm in them
Source:    Dan 3:25 (WLC, public domain)
```

```
Aramaic:   דִּי חָכְמְתָא וּגְבוּרְתָא דִּי לֵהּ־הִיא
Translit.: dī ḥoḵmətā ū-ḡəḇūrtā dī lēh-hī
Gloss:     for wisdom.DET and-might.DET REL to-3MS-COP.3FS
English:   for wisdom and might are his
Source:    Dan 2:20 (WLC, public domain)
```

## 7. Nominal clauses and the copula

Aramaic, like Hebrew, has **nominal clauses** without a verb "to be" in the present: מַלְכוּתֵהּ מַלְכוּת עָלַם "his kingdom (is) an everlasting kingdom" (Dan 3:33). Three devices make the link explicit:

1. **A 3rd-person pronoun as copula** (§ in [pronouns.md](pronouns.md) §1): אֱלָהֲכוֹן הוּא אֱלָהּ אֱלָהִין "your God is the God of gods" (Dan 2:47); דִּי־הוּא אֱלָהָא חַיָּא "for he is the living God" (Dan 6:27).
2. **אִיתַי with a suffix** for "is (in fact), exists": לָא אִיתֵיכוֹן פָּלְחִין "you do not serve" (Dan 3:14).
3. **הֲוָה** in the past and future: הֲוָת בָּטְלָא "it was stopped" (Ezra 4:24); לֶהֱוֵא "let it be" (Dan 2:20).

The pronoun copula becomes enclitic in JBA (ניהו, נינהו) and a hallmark of the Zohar (איהו), [family.md](family.md) §5.

## 8. Word order

Biblical Aramaic word order is **freer than Hebrew's**. The basic orders found are verb-initial (VSO, VOS), subject-initial (SVO) and **object before verb (OV)**. Object-before-verb order is especially common in Ezra's official documents and in Daniel, and is often explained by contact with Akkadian and Old Persian (both verb-final) in the imperial chancery (Kutscher 1970; Folmer 1995; Buth 1987 argues for a functional account: the fronted element is a topic or a focus).

| order | example | source |
|---|---|---|
| S V O | נְבוּכַדְנֶצַּר מַלְכָּא עֲבַד צְלֵם "King Nebuchadnezzar made an image" | Dan 3:1 |
| O V S | פִּתְגָמָא שְׁלַח מַלְכָּא "an answer the king sent" | Ezra 4:17 |
| O V | עַיְנַי לִשְׁמַיָּא נִטְלֵת "my eyes to heaven I lifted" | Dan 4:31 |
| O V | חָכְמְתָא וּגְבוּרְתָא יְהַבְתְּ לִי "wisdom and might you have given me" | Dan 2:23 |
| S V O | אֲנָה דָרְיָוֶשׁ שָׂמֶת טְעֵם "I, Darius, have issued a decree" | Ezra 6:12 |
| V S | עָנֵה מַלְכָּא "the king answered" | Dan 2:5 |
| PP V | לֵהּ יִפְלְחוּן "him they will serve" | Dan 7:14 |

```
Aramaic:   וְלִקְצָת יוֹמַיָּה אֲנָה נְבוּכַדְנֶצַּר עַיְנַי לִשְׁמַיָּא נִטְלֵת
Translit.: wə-li-qṣāṯ yōmayyā ʾănā nəḇūḵaḏneṣṣar ʿaynay li-šmayyā niṭlēṯ
Gloss:     and-at-end.CST day.M.PL.DET I Nebuchadnezzar eye.DU-1SG to-heaven.DET lift.PEAL.PF.1SG
English:   At the end of the days I, Nebuchadnezzar, lifted my eyes to heaven
Source:    Dan 4:31 (WLC, public domain)
```

Inside the noun phrase the order is fixed: noun – adjective – demonstrative; possessed – (דִּי) – possessor; numeral before or after the noun (אַמִּין שִׁתִּין "sixty cubits", but תְּלָתָה גֻּבְרִין is also possible).

### 8.1 Fronting and casus pendens

A noun phrase is often put first and picked up again by a pronoun (*casus pendens*, left dislocation): אֱלָהָךְ דִּי אַנְתְּ פָּלַח־לֵהּ … הוּא יְשֵׁיזְבִנָּךְ (Dan 6:17, §1). The same device marks the shift of topic in the visions: קַדְמָיְתָא כְאַרְיֵה "the first (beast) — like a lion" (Dan 7:4).

## 9. Complex sentences

| type | marker | example | source |
|---|---|---|---|
| conditional | הֵן … (imperfect / participle) | הֵן לָא תְהוֹדְעוּנַּנִי חֶלְמָא … הַדָּמִין תִּתְעַבְדוּן "if you do not tell me the dream … you will be torn limb from limb" | Dan 2:5 |
| conditional, two-sided | הֵן … וְהֵן לָא … | Dan 3:17–18 | |
| temporal | כְּדִי, בְּעִדָּנָא דִּי, עַד דִּי | בְּעִדָּנָא דִּי־תִשְׁמְעוּן "at the time that you hear" | Dan 3:5 |
| purpose | דִּי, לְ־ + infinitive | לְהַשְׁנָיָה לְחַבָּלָה "to alter, to destroy" | Ezra 6:12 |
| complement of a verb | דִּי | יְדַע דִּי־רְשִׁים כְּתָבָא | Dan 6:11 |
| complement of "able" | לְ־ + infinitive | יָכִל לְשֵׁיזָבוּתַנָא "able to deliver us" | Dan 3:17 |
| generalising relative | מַן־דִּי, מָה דִּי | וּמַן־דִּי־לָא יִפֵּל "and whoever does not fall" | Dan 3:6 |
| reported speech | verb of saying + direct speech, or דִּי | | |

```
Aramaic:   הֵן אִיתַי אֱלָהַנָא דִּי־אֲנַחְנָא פָלְחִין יָכִל לְשֵׁיזָבוּתַנָא
Translit.: hēn ʾīṯay ʾĕlāhanā dī-ʾănaḥnā p̄ālḥīn yāḵil lə-šēzāḇūṯanā
Gloss:     if EXIST god-1PL REL-we serve.PEAL.PTCP.ACT.MPL able.PEAL.PTCP.ACT.MS to-deliver.SHAPH.INF-1PL
English:   If our God whom we serve exists (= is able), he can deliver us
Source:    Dan 3:17 (WLC, public domain)
```

## 10. Agreement

- The verb agrees with its subject in person, gender and number; a verb before a plural subject may be singular (as in Hebrew).
- **Collective nouns** (עַם "people", חַיִל "army") take singular or plural verbs.
- **Feminine plural** subjects often take masculine plural verbs in later texts; Biblical Aramaic still keeps the feminine plural: אֶצְבְּעָן … וְכָתְבָן "fingers … and they wrote (f. pl.)" (Dan 5:5).
- The Ketiv/Qere system sometimes corrects agreement: Dan 5:5 Ketiv נפקו (m. pl.) → Qere נְפַקָה (f. pl.) to agree with אֶצְבְּעָן "fingers" (f.).

## 11. What the later varieties change (pointers)

| point | later | where |
|---|---|---|
| דִּי as a word | ד־ prefix | all |
| object | יָת on every definite object | [../arc-x-targum/translation-syntax.md](../arc-x-targum/translation-syntax.md) |
| object | ל־ + "anticipatory" suffix on the verb: אַשְׁכְּחֵיהּ לְרַב "he found Rav" | [../tmr/particles-syntax.md](../tmr/particles-syntax.md) |
| word order | follows the Hebrew verse | [../arc-x-targum/translation-syntax.md](../arc-x-targum/translation-syntax.md) |
| word order | free, topic-first; the dialogue of the sugya | [../tmr/particles-syntax.md](../tmr/particles-syntax.md) |
| copula | איהו everywhere | [../arc-x-zohar/grammar.md](../arc-x-zohar/grammar.md) |

The Elamur team
