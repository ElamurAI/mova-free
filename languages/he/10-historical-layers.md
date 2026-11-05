# 10. Historical layers: Biblical, Mishnaic, Medieval and Modern Hebrew

How Modern Hebrew differs from the Hebrew of the Bible and of the Mishna, and what it took from each: the verb system, relative clauses, possession, pronouns, negation, phonology, word order, and the influence of the European languages of the revivers. It also sketches the stages in between — Medieval Hebrew and the Hebrew of the *Haskala* — and says what the post-Rabbinic UD treebank (PostRab) shows about where a Modern Hebrew parser breaks on older texts. For Biblical Hebrew in depth see the Ancient Hebrew manual ([`../hbo/README.md`](../hbo/README.md), if present in this checkout).

[← README](README.md) · [← 09 Pragmatics and politeness](09-pragmatics-politeness.md) · [11 UD analysis →](11-ud-analysis.md)

Abbreviations: **BH** Biblical Hebrew (Classical, roughly 10th–2nd c. BCE in its texts), **LBH** Late Biblical Hebrew (Chronicles, Ezra-Nehemiah, Esther, Daniel), **QH** the Hebrew of the Dead Sea Scrolls, **MH** Mishnaic (Rabbinic) Hebrew (Tannaitic, c. 1st–3rd c. CE, spoken; Amoraic, later, literary), **MedH** Medieval Hebrew, **ModH** Modern Hebrew. Biblical forms are given in a simplified Tiberian transliteration in italics with macrons only where the vowel quality matters; Modern forms in the manual's transliteration ([02](02-script-and-spelling.md)).

---

## Contents

- [10.1 The stages](#101-the-stages)
- [10.2 What Modern Hebrew is built from](#102-what-modern-hebrew-is-built-from)
- [10.3 The verb system](#103-the-verb-system)
- [10.4 Relative clauses: אשר and ש](#104-relative-clauses-אשר-and-ש)
- [10.5 Possession: construct, של and suffixes](#105-possession-construct-של-and-suffixes)
- [10.6 Pronouns and demonstratives](#106-pronouns-and-demonstratives)
- [10.7 Negation](#107-negation)
- [10.8 Phonology: from Tiberian to Israeli](#108-phonology-from-tiberian-to-israeli)
- [10.9 Word order](#109-word-order)
- [10.10 The lexicon across the layers](#1010-the-lexicon-across-the-layers)
- [10.11 European influence on Modern Hebrew](#1011-european-influence-on-modern-hebrew)
- [10.12 Medieval Hebrew and the Haskala](#1012-medieval-hebrew-and-the-haskala)
- [10.13 Older Hebrew in the treebanks: PostRab and PTNK](#1013-older-hebrew-in-the-treebanks-postrab-and-ptnk)
- [10.14 One sentence through the ages](#1014-one-sentence-through-the-ages)
- [References](#references)

---

## 10.1 The stages

| Stage | Period | Main texts | Spoken? |
|---|---|---|---|
| Archaic BH | before c. 1000 BCE (in its texts) | poems: Song of Deborah (Judg 5), Song of the Sea (Ex 15) | yes |
| Classical BH | monarchy period | prose of Genesis–Kings | yes |
| LBH | after the Babylonian exile | Chronicles, Ezra, Nehemiah, Esther, Daniel | yes, alongside Aramaic |
| QH | 3rd c. BCE – 1st c. CE | Dead Sea Scrolls | partly |
| MH (Tannaitic) | 1st–3rd c. CE | Mishna, Tosefta, halakhic midrash | yes, until about 200 CE in Judea |
| Amoraic Hebrew | 3rd–6th c. | Hebrew parts of the Talmuds and midrash | literary; Aramaic spoken |
| Medieval | 6th–18th c. | liturgical poetry (*piyyut*), Spanish golden-age poetry, philosophy and science translated from Arabic, commentary (Rashi), law (Maimonides, *Shulhan Arukh*), letters | written, read, and occasionally spoken as a lingua franca |
| Haskala | late 18th–19th c. | Enlightenment prose, journalism, novels, in a mostly biblical style | written |
| Revival / ModH | from the 1880s | speech, schools, press, literature, the State | yes, native |

The periodisation is standard (Kutscher 1982; Sáenz-Badillos 1993 — *not held*); the date by which Hebrew stopped being spoken in daily life is debated (Kutscher placed it around 200 CE; others argue for later pockets). Whether LBH and its relation to Classical BH reflects date or style is also debated (Rezetko and Young 2014, local `$RESEARCH_RAW/hbobook-2014-rezetko-historical-linguistics-biblical-hebrew/`; Hornkohl 2024, local `$RESEARCH_RAW/hbobook-obp0433-hornkohl-diachronic-diversity-classical-biblical-hebrew/`).

## 10.2 What Modern Hebrew is built from

A rough map of what each layer contributed (Kutscher 1982; Sáenz-Badillos 1993; Glinert 1989 — *not held*; details below):

| Area | Mainly from | Note |
|---|---|---|
| morphology of nouns and verbs | BH, with MH forms where they differ | binyanim, mishkalim, construct, suffixes |
| tense system | MH | past–present (participle)–future, not the BH aspect system |
| relative ש | MH (already in late and northern BH) | אשר survives in formal style |
| analytic possessive של | MH | |
| negation of the participle | MH אין (formal), plus the new colloquial לא | |
| core vocabulary | BH | body, nature, family, basic verbs |
| everyday and abstract vocabulary | MH, Aramaic, MedH | *ilan*, *khama* literary; *ába, íma*; *metsi'ut*, *eykhut* |
| style of high literature | BH (and MH) | the Haskala wrote biblical Hebrew; modern prose mixes |
| phonetics | the revivers' European languages, on a Sephardi model | [§ 10.8](#108-phonology-from-tiberian-to-israeli) |
| idiom and much syntax | contact with Yiddish, Russian, German, English, Arabic | [§ 10.11](#1011-european-influence-on-modern-hebrew) |

The revival's policy is often summarised as "Mishnaic grammar for speech, Biblical vocabulary and style for elevation" — a simplification, but it captures why Modern Hebrew uses MH structures (participle as present, *she-*, *shel*) with a lexicon drawn from every layer.

## 10.3 The verb system

### Biblical Hebrew

BH has two finite conjugations — the suffix conjugation (*qatal*, "perfect") and the prefix conjugation (*yiqtol*, "imperfect") — and two "consecutive" forms with a prefixed *w-*: *wayyiqtol* (narrative past: "and he did") and *weqatal* (future, habitual, procedural sequence: "and he will do"). The forms are traditionally described as aspectual (perfective vs imperfective) rather than tense-based, and their analysis remains one of the most debated questions in Semitic linguistics (Driver 1892, local pd `$RESEARCH_RAW/hebgram/pd/driver-tenses-1892/`; GKC 1910 §§ 106–112, local pd `$RESEARCH_RAW/hebgram/pd/gesenius-kautzsch-cowley-1910/`; Cook 2012 — *not held*; Isaksson 2021, local `$RESEARCH_RAW/hbobook-obp0414-isaksson-verb-classical-hebrew/`; Andrason 2011, local `$RESEARCH_RAW/hbo-2011-andrason-biblical-hebrew-wayyiqtol-dynamic-definition/`).

```
ויאמר אלהים יהי אור ויהי אור          (Gen 1:3)
wayyōmer ʾĕlōhîm yǝhî ʾôr wayyǝhî ʾôr
and.say.WAYYIQTOL.3MS God be.JUSS.3MS light and.be.WAYYIQTOL.3MS light
'And God said, "Let there be light," and there was light.'
```

The participle in BH is a verbal adjective, used for ongoing or imminent action (*hinnê ʾānōkî bāʾ* "here I am coming"), not yet a tense.

### Mishnaic Hebrew

MH reorganises the system into **tenses** (Segal 1927, *A Grammar of Mishnaic Hebrew*, local but cited only; Kutscher 1982; Heijmans (ed.) 2020, local CC BY):

- *qatal* = past;
- the **participle** = present (and immediate future);
- *yiqtol* = future and modal (wishes, commands, purpose);
- the consecutive forms *wayyiqtol*/*weqatal* disappear;
- **היה + participle** expresses the past habitual and the counterfactual: *hāyâ ʾômēr* "he used to say" (the Mishna's frame for sayings of the sages, *Avot*).

### Modern Hebrew

ModH takes the MH system almost unchanged ([05](05-verbs.md)):

| Function | BH | MH | ModH |
|---|---|---|---|
| narrative past | *wayyiqtol* (*wayyōmer*) | *qatal* (*ʾāmar*) | past *amar* |
| present | participle (verbal adjective) / *yiqtol* | participle (*ʾômēr*) | participle *omer* |
| future | *yiqtol* / *weqatal* | *yiqtol* | future *yomar* |
| past habit | *yiqtol*, *weqatal* | *hāyâ* + participle | *haya* + participle (*haya omer*) |
| counterfactual | *lû* + *qatal* | *ʾillû* + *qatal*; *hāyâ* + participle | *im/ilu* + *haya* + participle ([07](07-syntax.md)) |
| infinitive | construct and absolute | with *l-* | *li-ktov*; the absolute only in fixed formal phrases |
| object pronoun | suffix on the verb | suffix or *ʾôtô* | *oto* (suffix literary) |
| 2/3 FPL future | *tiqṭōlnâ* | rare, merged | normative *tikhtóvna*, spoken *tikhtevu* |

Two further MH → ModH changes: the **nitpa'el** (MH) as the past of the reflexive-passive beside hitpa'el, which survives only in literary style (UD IAHLT tags it `NITPAEL`, 21 tokens; [11](11-ud-analysis.md)); and the **passive of pa'al** (BH *qal passive*), which MH lost in favour of nif'al — ModH keeps only its participle (*katuv* "written").

## 10.4 Relative clauses: אשר and ש

BH relativises mainly with the free word אשר *ʾăšer*; the prefix *še-* (*ša-*) appears in a few, mostly late or northern, texts (Song of Songs, Ecclesiastes, Judges 5) (GKC § 36, § 138). MH uses only *še-*. ModH uses *she-* everywhere and *asher* in formal writing ([07](07-syntax.md)).

The **article as relative** (*ha-* + participle or, rarely, + finite verb) is BH (*hā-ʿōśê* "who does"; even *ha-hôlîd* "who begot" in Gen 21:3) and continues in formal ModH with participles.

Counted in UD 2.18 (2026-10-17), per 1 000 words: *asher* 0.9 in IAHLTwiki, 0.16 in IAHLTknesset, 3.5 in PostRab; *she-* (SCONJ) 18.6, 33.1 and 41.8. Formal written ModH keeps a little *asher*; speech almost none; the post-Rabbinic texts use both, with *she-* already dominant (MH influence).

## 10.5 Possession: construct, של and suffixes

| | BH | MH | ModH |
|---|---|---|---|
| construct state | the main genitive: *bêt hammélek* "the king's house" | still common | common in writing and fixed compounds; less in speech ([04](04-nouns-adjectives-pronouns.md)) |
| analytic genitive | *ʾăšer lǝ-* "which is to" (rare) | **של** *šel* (from *še-* + *lǝ-*) | *shel*, the default in speech |
| pronoun suffix on nouns | universal: *bêtô* "his house" | common | formal (*beyt-o*); colloquially with kinship terms and fixed phrases |
| double genitive | — | *bêtô šel X* | *beyt-o shel dáni* (formal) |

של is a Mishnaic grammaticalisation of the relative *še-* + the preposition *lǝ-* "to, of" (Kutscher 1982; Segal 1927). In ModH it is a free word, inflecting with suffixes (*sheli*, *shelkha*, *shelo*…; [06](06-prepositions-particles.md)). The construct remains productive for compounds (*beyt séfer* "school"), which is why UD makes it a `compound:smixut` rather than an ordinary genitive ([11](11-ud-analysis.md)).

## 10.6 Pronouns and demonstratives

| | BH | MH | ModH |
|---|---|---|---|
| "I" | *ʾănî*, *ʾānōkî* | *ʾănî* | *ani* (*anokhi* poetic) |
| "we" | *ʾănaḥnû* (rare *naḥnû*) | **ʾānû** | *anákhnu*; *ánu* formal |
| "you" F.PL / "they" F.PL | *ʾattēn*, *hēnnâ* | merged with masculine *-n* forms | *aten*, *hen* normative; *atem*, *hem* for all in speech |
| "this" M / F | *zê*, *zōʾt* | *zê*, **zô** | *ze*, *zot*; *zo* formal |
| "these" | *ʾēlleh* | **ʾēllû** | *éle*; *élu* formal |
| "that" (distal) | *hahûʾ* | *ʾôtô* (identity), *hallāz* | *ha-hu*, *ha-hi* (colloquial distal) |
| possessive "my" | suffix *-î* | suffix, *šellî* | *sheli*, suffix formal |

MH forms (*ánu*, *zo*, *élu*) survive in ModH as formal doublets beside the BH forms used in speech — one of the layered features a tagger must know ([04](04-nouns-adjectives-pronouns.md)).

## 10.7 Negation

| | BH | MH | ModH |
|---|---|---|---|
| verbs | לא *lōʾ* | *lōʾ* | *lo* |
| prohibition | אל *ʾal* + jussive | *ʾal* + *yiqtol* | *al* + future ([07](07-syntax.md)) |
| existential | אין *ʾên* | *ʾên* | *eyn* |
| participle (present) | *ʾên* + participle (some) | **ʾên** + participle, inflected *ʾênô* | *lo* in speech; *eyno* in formal writing |
| "not" before nouns, adjectives | *lōʾ*, *bǝlî* | *lōʾ*, *ʾên* | *lo*, *bilti*, *i-* |
| "nothing / nobody" | *lōʾ … kol* | *ʾên … kǝlûm* | *klum*, *af ekhad … lo* (negative concord) |

The MH participial negation with inflected *ʾên* became the formal ModH *eyno yodéa*, while speech generalised *lo* to the participle. Counted in UD 2.18 (2026-10-17), the lemma אין against לא: PostRab 81 : 93, IAHLTwiki 78 : 459, IAHLTknesset 152 : 980 — the older texts negate with *eyn* almost as often as with *lo*, Modern Hebrew six times less. (Knesset *eyn* is mostly the existential "there isn't".)

## 10.8 Phonology: from Tiberian to Israeli

The pointed Bible preserves the **Tiberian** reading tradition of the Masoretes (8th–10th c. CE). Khan (2020) reconstructs it in detail from medieval sources: a system of seven vowel qualities with distinctions of length tied to stress and syllable structure, the pharyngeals ḥ and ʿ, the emphatics ṭ and ṣ and q, the spirantization of *bgdkpt* after vowels, gemination (dagesh forte), and a reduced vowel (*shewa*) with rules for when it was pronounced (Khan 2020, vol. 1, local CC BY `$RESEARCH_RAW/hebgram/open/khan-tiberian-pronunciation-v1-2020/`). Israeli pronunciation differs at almost every point ([03](03-phonology.md)):

| Feature | Tiberian (Khan 2020) | Israeli standard |
|---|---|---|
| vowels | 7 qualities (incl. *qameṣ* [ɔ] and *ṣere* [e] vs *segol* [ɛ]) + reduced vowels | 5: a e i o u |
| length | conditioned by stress and syllable | none |
| ח / כ | [ħ] / [χ] | both [χ] |
| ע / א | [ʕ] / [ʔ] | both [ʔ] or nothing |
| ט / ת | emphatic [tˤ] / [t] | both [t] |
| ק / כּ | [q] / [k] | both [k] |
| צ | emphatic [sˤ] | [ts] |
| ר | (variants, with an alveolar realisation) | uvular [ʁ] |
| ו | [w] or [v] (debated) | [v] |
| spirantization of *g d t* | yes (6 letters) | only *b k p* alternate |
| gemination | phonemic | lost |
| stress | marked by accents; mostly final | mostly final; penultimate in segolates and many loans |

The revivers chose a **Sephardi**-type pronunciation as the model — five vowels, *qameṣ* as [a], final stress — over the Ashkenazi tradition of their own prayers ([o] for *qameṣ*, [s] for undotted ת, penultimate stress; Zuckermann 2003, *not held*). In practice their European native phonetics shaped the result: uvular *r*, loss of pharyngeals and emphatics. The **Mizrahi** pronunciation keeps the pharyngeals and is closer to the Tiberian consonantism ([01](01-overview.md), [03](03-phonology.md)). Other living reading traditions — Yemenite, Samaritan, the medieval Babylonian and Palestinian pointings — are studied in Khan and Hornkohl (eds.) 2020 (local CC BY `$RESEARCH_RAW/hebgram/open/khan-hornkohl-semitic-vocalisation-2020/`).

## 10.9 Word order

| | BH prose | MH | ModH |
|---|---|---|---|
| basic order of a verbal clause | **VSO** in narrative (*wayyōmer ʾĕlōhîm* "and-said God") | SVO common; participle clauses S–Part | **SVO** ([07](07-syntax.md)) |
| fronting | X–*qatal* for background, contrast | | topicalisation; VS after fronted adverbs ("triggered inversion") — a BH-like residue |
| agreement with a following coordinated subject | first-conjunct agreement normal | | first-conjunct agreement in formal style (Doron 2000, *not held*) |
| verbless clauses | S–P and P–S, with pronominal *hûʾ* | | S (COP) P |

Whether BH is basically VSO or SVO with verb-first narrative forms is itself debated (Holmstedt 2011, local `$RESEARCH_RAW/hbo-2011-holmstedt-typological-classification-hebrew-genesis-subject/`; Kaajan 2019, local `$RESEARCH_RAW/hbo-2019-kaajan-biblical-hebrew-non-configurational-language/`). The shift to SVO in MH and ModH is usually attributed partly to Aramaic contact (MH) and partly to European contact (ModH).

## 10.10 The lexicon across the layers

See [08 § 8.6](08-lexicon-register.md#86-the-inherited-layers-biblical-mishnaic-aramaic-medieval). In brief:

- **Doublets** with a register split: BH *ets* / MH *ilan* "tree"; BH *shémesh* / MH *khama* "sun"; BH *lakákh* / MH *natal* "take"; BH *ánu* / MH… the pronoun pairs above. ModH usually keeps the BH word as neutral and the MH one as literary — or the reverse (*ilan* is literary, but *lehatkhil* "to begin", hif'il of MH origin, is neutral).
- **Aramaic** words (*ába*, *íma*, *agav*, *dávka*, *bérez*, *mada*) enter through MH and the Talmud.
- **Greek and Latin** loans of MH (*sanhedrin*, *pruzbul*, *kategor*, *sandal*) — Heijmans (2020) studies vowel reduction in Greek loanwords in the Mishna, showing how they were adapted to Hebrew patterns.
- **Medieval** abstract vocabulary from the translators of Arabic ([08](08-lexicon-register.md)).

## 10.11 European influence on Modern Hebrew

The revivers spoke Yiddish, Russian, Polish, German, French, Ladino, Arabic and other languages; their children spoke Hebrew with the phonetics and much of the idiom of their parents' languages. Influences commonly cited (Zuckermann 2003; Glinert 1989; Wexler 1990 — *not held*; see [01 § 1.2](01-overview.md#12-continuity-or-a-new-language-the-israeli-debate)):

| Area | Example | Probable model |
|---|---|---|
| phonetics | uvular [ʁ], loss of pharyngeals and gemination | Yiddish, German, Russian, French |
| vowel reduction and stress in loans | *télefon*, *universíta* | European source words |
| SVO order, adverb placement | | European languages generally |
| *ze* as a copula | *shokolad ze ta'im* | Slavic *to / èto*, Yiddish *dos iz* (contested; [07](07-syntax.md)) |
| *yesh li et* | possessed as object | European "have" (the verb takes an object) |
| analytic tense with *haya* + participle for conditional | *hayíti ba* "I would have come" | MH, reinforced by European conditionals |
| lexical calques | *ma nishma* (Yiddish *vos hert zikh*), *lesakhek tafkid* "play a role" | [08 § 8.9](08-lexicon-register.md#89-calques-and-loan-meanings) |
| suffixes | *-nik*, *-chik*, *-le*, *-izm* | Yiddish, Slavic |
| semantic extension | *khashmal* "electricity" | Greek *ēlektron* via European languages |
| politeness and address | no T/V; first names | ideological egalitarianism of the revival; contrast with BH deference forms ([09](09-pragmatics-politeness.md)) |

Arabic influence is mainly lexical and in slang, plus the Mizrahi phonetics ([08](08-lexicon-register.md)); English influence is lexical, phraseological and orthographic (loan spellings with geresh, acronyms).

## 10.12 Medieval Hebrew and the Haskala

**Medieval Hebrew** was not one language but several written registers:

- **Liturgical poetry** (*piyyut*, from the 5th–6th c.: Yannai, Qillir) — daring new forms, many from MH, invented derivations.
- **The Spanish school** (10th–12th c.: Dunash, Shmuel ha-Nagid, Ibn Gabirol, Yehuda ha-Levi) — poetry in pure biblical Hebrew with Arabic metres; Martínez Delgado (2023) introduces Andalusi Hebrew metrics (local CC BY `$RESEARCH_RAW/hbobook-obp0351-delgado-introduction-andalusi-hebrew-metrics/`).
- **Philosophy and science** translated from Arabic (the Ibn Tibbon family, 12th–13th c.) — the source of much abstract vocabulary ([08](08-lexicon-register.md)).
- **Rabbinic Hebrew** of commentary, responsa and codes (Rashi, Maimonides' *Mishne Tora* in MH style, the *Shulhan Arukh*) — MH grammar with Aramaic admixture.
- **Grammatical science**: the medieval Hebrew grammarians (Saadia, Hayyuj, Ibn Janah, Ibn Ezra, Radak) worked in dialogue with Arabic grammar. Kantor (2023) studies the *standard language ideology* of the Hebrew and Arabic grammarians of the ʿAbbasid period — how both traditions came to regard one reading (Tiberian for Hebrew, the ʿArabiyya for Arabic) as the pure and correct language (*not quoted*: the book is CC BY-NC; local `$RESEARCH_RAW/hebgram/nc/kantor-standard-language-ideology-2023/`). The ideology of a correct, classical standard against everyday usage reappears in the Modern debate between the Academy's norms and speech ([08 § 8.12](08-lexicon-register.md#812-norm-and-use)).

The **Aramaic dalet** ד־ "that, of" (from Aramaic *dī/də-*) is used in many rabbinic and later Hebrew texts beside Hebrew *she-* and *shel*. Tal, Shmidman and Shmidman (2026) built a BERT model for historical Hebrew with all prefixes segmented, used it to tell the dalet's two functions (subordinator vs possessive) apart with F1 ≈ 0.89, and charted the choice between dalet and Hebrew alternatives across regions and periods in more than 300 million words (local: `$RESEARCH_RAW/aclbulk-2026-tal-modeling-dalet-clitic-historical-hebrew/`). Modern Hebrew keeps the dalet only in fixed Aramaic phrases (*de-fákto* is Latin; genuine ones: *kol de-hu* "anything at all", *b-de'avad* "after the fact").

**The Haskala** (Berlin from the 1780s, then Galicia and Russia) used Hebrew for secular prose, journalism, science popularisation and the first Hebrew novels (Abraham Mapu's *Ahavat Tsiyon*, 1853). Its style, *melitsa*, wove biblical phrases into new contexts and avoided MH; it was criticised by the next generation (Mendele Mokher Sforim, from the 1880s) who created the *nusakh*, a synthesis of all historical layers — the immediate literary ancestor of Modern written Hebrew (Sáenz-Badillos 1993; Harshav 1993 — *not held*).

## 10.13 Older Hebrew in the treebanks: PostRab and PTNK

Two UD treebanks hold pre-modern Hebrew:

- **UD Ancient Hebrew PTNK** (`hbo`; Biblical Hebrew; CC BY-NC-SA) — a separate language in UD; Mova measures its MACULA-based Biblical trees against it (`tools/macula`, test LAS 82.23 on 2026-10-17; [12](12-mova.md)).
- **UD Hebrew PostRab** (`he`; CC BY-SA; UD 2.18 has 296 test sentences, 8 029 words) — post-Rabbinic texts: Rashi's Torah commentary, the *Shulhan Arukh*, *Sefer ha-Hinukh*, *Sefer Maharil*, the cosmographical *Igeret Orhot Olam*, and a miscellany (treebank README; Tal et al. 2025).

PostRab is filed under `he` but its language is MH-based rabbinic Hebrew with Aramaic. The UD 2.18 counts (2026-10-17; [11](11-ud-analysis.md)) show the difference from Modern Hebrew:

| Measure (per 1 000 words) | PostRab | IAHLTwiki | IAHLTknesset |
|---|---:|---:|---:|
| multiword tokens | 304 | 246 | 229 |
| אין (lemma) | 10.1 | 0.6 | 2.3 |
| של (lemma) | 6.6 | 17.6 | 12.6 |
| אשר (lemma) | 3.5 | 0.9 | 0.2 |
| PRON (UPOS) | 117 | 40 | 95 |
| PROPN (UPOS) | 18 | 74 | 35 |
| punctuation | 0 | 118 | 120 |

The texts have **no punctuation** (sentence boundaries are an editorial decision), more clitics per token, fewer analytic possessives (*shel*), three times the pronouns of Wikipedia, more *eyn*, and Aramaic forms (the dalet in דבאוכלין "that in food", sentence 2 of the test file). Relations specific to PostRab — `compound:svc` (serial verbs) and `compound:redup` (reduplication) — do not occur in the Modern treebanks. A parser trained on Modern Hebrew will therefore stumble on: missing punctuation, Aramaic words and clitics, the inflected *eyn*, verbless and participial clauses of legal style, and abbreviations of rabbinic references.

## 10.14 One sentence through the ages

"The man said to his son: Go to the field."

```
BH:    ויאמר האיש אל בנו לך השדה
       wayyōmer hāʾîš ʾel bǝnô lēk haśśādeh
       and.say.WAYYIQTOL.3MS DEF-man to son-3MS.POSS go.IMP.2MS DEF-field
MH:    אמר האיש לבנו: לך לשדה
       ʾāmar hāʾîš libnô lēk laśśādeh
       say.PST.3MS DEF-man to-son-3MS.POSS go.IMP.2MS to.DEF-field
ModH:  האיש אמר לבן שלו: לך לשדה / תלך לשדה
       ha-ish amar la-ben shelo: lekh la-sade / telekh la-sade
       DEF-man say.PST.3MS to.DEF-son of.him go.IMP.2MS / go.FUT.2MS to.DEF-field
```

The BH line opens with the narrative *wayyiqtol* (VSO) and uses *ʾel* "to"; the goal of motion can be a bare noun (an accusative of direction), as here. MH has the plain past, *l-* "to". ModH has SVO, the analytic possessive *shelo* (or formal *li-vno*), and a choice of imperative or future-as-imperative ([05](05-verbs.md), [09](09-pragmatics-politeness.md)).

---

## References

Works marked *not held* are not in the local library; see «wanted» in [sources.md](sources.md).

- Andrason, Alexander. 2011. Biblical Hebrew wayyiqtol: a dynamic definition. *Journal of Hebrew Scriptures* 11. Local: `$RESEARCH_RAW/hbo-2011-andrason-biblical-hebrew-wayyiqtol-dynamic-definition/`.
- Cook, John A. 2012. *Time and the Biblical Hebrew Verb*. Winona Lake: Eisenbrauns. *Not held.*
- Martínez Delgado, José. 2023. *Introduction to Andalusi Hebrew Metrics*. Cambridge: Open Book Publishers. Local: `$RESEARCH_RAW/hbobook-obp0351-delgado-introduction-andalusi-hebrew-metrics/`.
- Doron, Edit. 2000. VSO and left-conjunct agreement: Biblical Hebrew vs. Modern Hebrew. In A. Carnie and E. Guilfoyle (eds.), *The Syntax of Verb Initial Languages*. OUP. *Not held.*
- Driver, S. R. 1892. *A Treatise on the Use of the Tenses in Hebrew*. 3rd ed. Oxford. Public domain; local `$RESEARCH_RAW/hebgram/pd/driver-tenses-1892/`.
- Gesenius, W., E. Kautzsch and A. E. Cowley. 1910. *Gesenius' Hebrew Grammar* (GKC). 2nd English ed. Oxford. Public domain; local `$RESEARCH_RAW/hebgram/pd/gesenius-kautzsch-cowley-1910/`.
- Glinert, Lewis. 1989. *The Grammar of Modern Hebrew*. CUP. *Not held.*
- Harshav, Benjamin. 1993. *Language in Time of Revolution*. University of California Press. *Not held.*
- Heijmans, Shai (ed.). 2020. *Studies in Rabbinic Hebrew*. Cambridge: Open Book Publishers. CC BY 4.0. https://doi.org/10.11647/OBP.0164 — local `$RESEARCH_RAW/hebgram/open/heijmans-studies-rabbinic-hebrew-2020/`.
- Holmstedt, Robert D. 2011. The typological classification of the Hebrew of Genesis: subject-verb or verb-subject? *Journal of Hebrew Scriptures* 11. Local: `$RESEARCH_RAW/hbo-2011-holmstedt-typological-classification-hebrew-genesis-subject/`.
- Hornkohl, Aaron D. 2024. *Diachronic Diversity in Classical Biblical Hebrew*. Cambridge: Open Book Publishers. Local: `$RESEARCH_RAW/hbobook-obp0433-hornkohl-diachronic-diversity-classical-biblical-hebrew/`.
- Isaksson, Bo. 2021. *The Verb in Classical Hebrew*. Open Book Publishers. Local: `$RESEARCH_RAW/hbobook-obp0414-isaksson-verb-classical-hebrew/`.
- Kaajan, Marianne. 2019. Is Biblical Hebrew a non-configurational language? Reconsidering the evidence from discontinuous phrases. *HIPHIL Novum* 5(2). Local (read only): `$RESEARCH_RAW/hbo-2019-kaajan-biblical-hebrew-non-configurational-language/`.
- Kantor, Benjamin Paul. 2023. *The Standard Language Ideology of the Hebrew and Arabic Grammarians of the ʿAbbasid Period*. Cambridge: Open Book Publishers. CC BY-NC 4.0. https://doi.org/10.11647/OBP.0382 — local `$RESEARCH_RAW/hebgram/nc/kantor-standard-language-ideology-2023/` (summarised, not quoted).
- Khan, Geoffrey. 2020. *The Tiberian Pronunciation Tradition of Biblical Hebrew*. 2 vols. Cambridge: Open Book Publishers. CC BY. https://doi.org/10.11647/OBP.0163 — local `$RESEARCH_RAW/hebgram/open/khan-tiberian-pronunciation-v1-2020/`, `…-v2-2020/`.
- Khan, Geoffrey and Aaron D. Hornkohl (eds.). 2020. *Studies in Semitic Vocalisation and Reading Traditions*. Open Book Publishers. Local: `$RESEARCH_RAW/hebgram/open/khan-hornkohl-semitic-vocalisation-2020/`.
- Kutscher, Eduard Yechezkel. 1982. *A History of the Hebrew Language*. Jerusalem: Magnes; Leiden: Brill. *Not held.*
- Rezetko, Robert and Ian Young. 2014. *Historical Linguistics and Biblical Hebrew*. Atlanta: SBL. Local: `$RESEARCH_RAW/hbobook-2014-rezetko-historical-linguistics-biblical-hebrew/`.
- Sáenz-Badillos, Angel. 1993. *A History of the Hebrew Language*. Trans. J. Elwolde. CUP. *Not held.*
- Segal, M. H. 1927. *A Grammar of Mishnaic Hebrew*. Oxford: Clarendon. Local (logs and citation only): `$RESEARCH_RAW/hebgram/private/segal-grammar-mishnaic-1927/`. Also Segal 1909, *Mišnaic Hebrew and its Relation to Biblical Hebrew and to Aramaic* (JQR 20), local `$RESEARCH_RAW/hebgram/private/segal-misnaic-hebrew-1909/`.
- Tal, Rachel, Cheyn Shmuel Shmidman and Avi Shmidman. 2026. Modeling the Dalet Clitic in Historical Hebrew Texts: A New Prefix-Segmented BERT Model and Stylistic Analysis. Local: `$RESEARCH_RAW/aclbulk-2026-tal-modeling-dalet-clitic-historical-hebrew/`.
- Tal, Rachel et al. 2025. A New Hebrew Universal Dependency Treebank: The First Treebank of Post-Rabbinic Historical Hebrew. TLT 2025. https://aclanthology.org/2025.tlt-1.11/
- UD 2.18 treebanks UD_Hebrew-PostRab, -IAHLTwiki, -IAHLTknesset (CC BY-SA 4.0). Local `$RESEARCH_RAW/lexres/treebanks/ud-2.18/` (counts 2026-10-17).
- Wexler, Paul. 1990. *The Schizoid Nature of Modern Hebrew*. Harrassowitz. *Not held.*
- Zuckermann, Ghil'ad. 2003. *Language Contact and Lexical Enrichment in Israeli Hebrew*. Palgrave Macmillan. *Not held.*
