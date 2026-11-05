# The grammar of Zoharic Aramaic

The Zohar's Aramaic has a grammar of its own: it uses the forms of the classical dialects, but **mixes** them freely, gives some of them **new functions**, and lays them over a **Hebrew** skeleton. This file describes it by comparison with the varieties described in the other folders. The main descriptions are Scholem 1941 (lecture 5, "The Zohar I: The Book and Its Author", on the language), Tishby 1989 (introduction), Liebes 1976 and 1993, and the notes of Matt's translation (2004–2017); none is on disk, so we cite them from the literature and give our own examples from the public-domain texts.

**Data used here.** Counts are unpointed tokens in the two public-domain Zohar texts on disk (Tiqqunei ha-Zohar, vocalised; Zohar Ḥadash; 344 133 words; Sefaria), made on 2026-10-17. Examples from the main Zohar are quoted **consonantally** (the ancient text is public domain; the vocalised editions on disk have no clear licence) and transliterated by us.

## 1. Spelling

- **Plene spelling** like the Bavli's: אִיהוּ, אִינּוּן, אוֹרַיְיתָא, וּבְגִין.
- **Double ו / י** for consonants: גַּוְונָא, צְוָוחָא, אוֹרַיְיתָא.
- **Abbreviations** like the Bavli's, and its own: ת"ח (תָּא חֲזֵי), קב"ה (קוּדְשָׁא בְּרִיךְ הוּא), ר"ש (רַבִּי שִׁמְעוֹן), ר"א (רַבִּי אַבָּא / אֶלְעָזָר), ע"ד (עַל דָּא), כד"א (כְּמָה דְּאַתְּ אָמַר), and the divine names spelled with quotation marks to avoid writing them: הוי"ה, יאהדונה"י (counted 46× in the Tiqqunim), ידו"ד.
- **Editorial insertions** in the printed text: variant readings in parentheses marked נ"א ("another version") or ס"א, and cross-references to other pages. They must be removed before the text is analysed ([mova.md](mova.md) §3).

## 2. Nouns and the states

| point | Zohar | classical norm |
|---|---|---|
| determined state | ־ָא used freely, as in JBA: often not definite | definite (BA, targum) |
| construct | frequent, often modelled on Hebrew construct chains: רָזָא דְמִלָּה "the secret of the matter" | ד־ (JBA) |
| ד־ genitive | everywhere: הוּרְמְנוּתָא דְמַלְכָּא "the authority of the king" | |
| plural | targumic ־ַיָּא beside Babylonian ־ֵי, without system: עִלָּאִין "upper ones", סְתִימִין "hidden ones", עָלְמִין "worlds" | |
| Hebrew nouns with Aramaic endings | סְפִירָן "sefirot", נְקוּדָה (Hebrew, 261×), מַחֲשָׁבָה (Hebrew) | |
| abstract nouns in ־וּ / ־וּתָא | many, some new: טְהִירוּ "radiance", קַרְדִינוּתָא "hardness, darkness (?)", נְבִיעוּ "a spring, flowing" | |

**State agreement breaks down.** Attributive adjectives often disagree with their nouns in state (a determined noun with an absolute adjective and the reverse), as in בּוּצִינָא קַדִּישָׁא beside בּוּצִינָא קַדִּישׁ. This is one of the "errors" noticed by Scholem; for a model it means: do not use state agreement as a constraint when parsing `arc-x-zohar`.

## 3. Pronouns

| person | Zohar | note |
|---|---|---|
| 1 sg. | אֲנָא | |
| 2 m. sg. | אַנְתְּ (Tiqqunim: אַנְתְּ הוּא "you are he"), אַתְּ | the old form אַנְתְּ is Biblical Aramaic (Strack 1901 §6a) and archaic in JBA (Margolis 1910 §8b) and PTM (Stevenson 1924 §3); Onkelos has only אַתְּ (Dalman 1905 §16.1, p. 107; no אנת in the Sefaria Onkelos, checked 2026-10-18) |
| 3 m. sg. | **אִיהוּ** | the JBA form (Margolis 1910 §8b gives it as the ordinary independent pronoun, with הוּא archaic; Jastrow and modern descriptions call it emphatic), used for everything |
| 3 f. sg. | **אִיהִי** | |
| 1 pl. | אֲנַן, אֲנַחְנָא | |
| 3 m. pl. | **אִינּוּן** (1 595) | targumic and JPA form (Dalman 1905 §16.1; archaic in JBA, Margolis 1910 §8b); also "those" |
| 3 m. pl. (to them) | **לוֹן** (989) beside לְהוֹן (91), לְהוּ (93) | the JPA form (Stevenson 1924 §4: PTM ־וֹן beside ־הוֹן) dominates in the Tiqqunim; לְהוֹן is targumic, לְהוּ JBA (Margolis 1910 §25) |
| demonstratives | **דָּא** (4 225), הַאי (652), דֵּין (157), הָדֵין (75), אִלֵּין, הַהוּא | דָּא, דֵּין, הָדֵין, אִלֵּין: Onkelos and Galilean (Dalman 1905 §17); הַאי: JBA (Margolis 1910 §9) |

Checked 2026-10-18: the attributions in this table against the grammars named in it. The Zoharic uses themselves (אִיהוּ for every person, the copula) have no grammar on disk (unchecked: no grammar of Zoharic Aramaic on disk).

**אִיהוּ as a copula.** The defining syntactic feature of the Zohar is the use of אִיהוּ / אִיהִי (and דְּאִיהוּ "which is") to link subject and predicate, often with a demonstrative: דָּא אִיהוּ "this is", וְדָא אִיהוּ רָזָא "and this is the secret". In the Bavli אִיהוּ is a pronoun, "he (himself)" (emphatic in Jastrow; the plain form in Margolis 1910 §8b), not a copula (the copula there is נִיהוּ); the Zohar turns it into a general copula, much like Hebrew הוּא in medieval prose (Scholem 1941). Frequency: ~670 per 100 000 words in the Zohar texts against ~17 in the Bavli and 0 in the Yerushalmi and Onkelos ([README.md](README.md) §2).

```
Aramaic:   מָאן שׁוֹשַׁנָּה? דָּא כְּנֶסֶת יִשְׂרָאֵל
Translit.: mān šōšannā? dā kəneseṯ yiśrāʾēl
Gloss:     who rose? this assembly(Heb.) Israel
English:   What is "the rose"? This is the Assembly of Israel.
Source:    Zohar 1:1a (standard text, public domain)
```

Note the Hebrew noun phrase כְּנֶסֶת יִשְׂרָאֵל kept in Hebrew, the question word מָאן "who" used for "what", and דָּא as the predicate-identifier.

## 4. The verb

### 4.1 Forms

The Zohar uses all the classical stems — peal, pael, aphel, ithpeel, ithpaal — with forms from Onkelos and the Bavli side by side:

| category | Zohar forms | sources of the forms |
|---|---|---|
| imperfect 3 m. sg. | **יְהֵא** (152), יְהֵי (122); also לֵימָא (17), לֶיהֱוֵי (2) | mostly targumic י־; JBA ל־ in fixed phrases |
| 1 pl. "let us say" | נֵימָא (25) | JBA |
| participle + pronoun | אֲנָא רָחִיץ "I trust" (*Berikh shemeh*), and fused סָגִידְנָא "I bow", רָחִיצְנָא, סָמִיכְנָא | JBA-type enclitics |
| durative קָא | קָא (121), דְּקָא (83) | JBA, but less frequent |
| object marker | יָת (24), יָתִי "me" (*Berikh shemeh*) | targumic (Dalman 1905 §16c, p. 110: Onkelos יָת, with suffixes as object pronouns), rare |
| passive / reflexive | אִתְקְרֵי "is called" (205), אִקְרֵי / אִיקְרֵי (88 / 121), אִתְגַּלְיָיא "is revealed" (37), אִתְפַּשַּׁט "spread out" (45), אִסְתַּלַּק "ascended, departed" (75), אִתְּעַר "awoke" (37) | all dialects; frequent |
| participles as narrative and descriptive verbs | נָפִיק "goes out" (118), עָבִיד "makes" (134), גָּלִיף "engraves", בָּטַשׁ "strikes" | |

Checked 2026-10-18: the source attributions against Onkelos (Sefaria counts: יְהֵי 216, יְהֵא 24; Dalman 1905, *Anhang* VI) and JBA (Margolis 1910 §28d, Tab. II, Tab. XII, Tab. XXa: ל־ / נ־ imperfects, לֵימָא, לֶיהֱוֵי, the enclitic ־נָא). The Zoharic forms and counts stay as measured; their analysis has no grammar on disk (unchecked).

### 4.2 "Wrong" stems and new meanings

Scholem (1941, lecture 5) and later Liebes (1976) describe systematic departures from classical usage. The kinds of departure, with examples we can show in the texts:

| kind | example | classical usage |
|---|---|---|
| a Hebrew verb in Aramaic form | דְּקָשִׁיר לוֹן וּמְיַחֵד לוֹן "who binds them and unites them" (Tiqqunim, introduction) | יחד "unite" in the pael is a Hebrew (rabbinic and medieval) usage given an Aramaic participle |
| a Hebrew construction translated word for word | לָא בְחֻשְׁבָּן "not in number" (Tiqqunim, introduction) | Hebrew וְלֹא בְמִסְפָּר |
| Hebrew words with Aramaic grammar | אִתְחַשֵּׁיב לֵיהּ כְּאִלּוּ "it is counted for him as if" (Tiqqunim) — Hebrew כְּאִלּוּ | Aramaic כְּמָאן דְּ־ |
| an Aramaic word with a medieval Hebrew meaning | סִטְרָא "side" in the sense of a domain or aspect (סִטְרָא אַחֲרָא "the other side" = the realm of evil) | "side" (physical) |
| t-stems used as active or intransitive without a passive sense | אִתְּעַר "arouse, awaken" (from Hebrew הִתְעוֹרֵר, both senses) | |

The precise list of the Zohar's "errors" is a matter of scholarship (Scholem gives examples of confused stems, wrong prepositions, and verbs with invented meanings; Liebes collects the vocabulary). A model should not try to correct them; it should **recognise** them, as a feature of the variety.

## 5. Syntax

- **Hebrew word order** in Aramaic words: clause-initial verbs in narrative frames (פָּתַח וְאָמַר), but nominal sentences of the Hebrew type everywhere in the exposition: X דְּאִיהוּ Y, וְדָא אִיהוּ, בְּגִין דְּ־.
- **Homiletic structure**: a verse is quoted (כְּתִיב, כְּמָה דְּאַתְּ אָמַר), a word in it is asked about (מַאן …? מַאי …?), identified with a concept (דָּא …), and the identification is supported by another verse ([style-formulas.md](style-formulas.md)).
- **Comparisons** built on מַה … אוּף … "as … so also …" (Hebrew מַה … אַף …): מַה שׁוֹשַׁנָּה … אִית בַּהּ סוּמָק וְחִוָּר, אוּף כְּנֶסֶת יִשְׂרָאֵל אִית בַּהּ דִּין וְרַחֲמֵי "as the rose has red and white, so too the Assembly of Israel has judgment and mercy" (Zohar 1:1a).
- **Long chains of ד־** (relative and genitive): סָתִים גּוֹ סְתִימִין דְרָזָא דְּאֵין סוֹף "hidden within the hidden ones of the secret of the Infinite" (Zohar 1:15a).
- **Paired opposites** and lists: לָא חִוָּור וְלָא אוּכָם וְלָא סוּמָק וְלָא יָרוֹק "not white and not black and not red and not green" (Zohar 1:15a).

## 6. Strata

The parts of the Zohar differ in language (Scholem 1941; Tishby 1989; Huss 2016):

| stratum | language features |
|---|---|
| main Zohar, Idrot, Sifra di-Tseniʿuta | the "classical" Zoharic Aramaic: narrative frames with the companions; rich, varied vocabulary; Idrot and Sifra di-Tseniʿuta in a deliberately archaic, compressed style |
| Midrash ha-Neʿelam | mostly Hebrew, with Aramaic passages |
| Raʿaya Meheimna and Tiqqunei ha-Zohar | a later and more uniform Aramaic, more Hebrew vocabulary, heavy use of the names of the sefirot, numerology and letter symbolism; long lists; many repetitions of אִיהוּ / אִיהִי and לוֹן |
| Zohar Ḥadash | a mixture of all the above |

The public-domain texts on disk are the **Tiqqunim** and **Zohar Ḥadash**, so the counts in this folder lean towards the later stratum. When a public-domain main Zohar becomes available, the counts should be redone per stratum ([mova.md](mova.md)).

The Elamur team
