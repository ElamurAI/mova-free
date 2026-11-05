# 1. Overview: Modern Hebrew

What Modern Hebrew is: its names and the debate behind them, who speaks it and how many, where it sits in the Semitic family, a typological profile, its varieties and sociolects, and a short timeline of the revival that made a written and liturgical language into the everyday mother tongue of millions. Later chapters go into each area; this one is the map.

[← README](README.md) · [02 Script and spelling →](02-script-and-spelling.md)

Related: lexicon and the revivers' methods in [08](08-lexicon-register.md); the older layers of the language in [10](10-historical-layers.md); how Mova would treat Hebrew in [12](12-mova.md).

---

## Contents

- [1.1 Names](#11-names)
- [1.2 Continuity or a new language? The "Israeli" debate](#12-continuity-or-a-new-language-the-israeli-debate)
- [1.3 Speakers and status](#13-speakers-and-status)
- [1.4 Family](#14-family)
- [1.5 Typological profile](#15-typological-profile)
- [1.6 Varieties and sociolects](#16-varieties-and-sociolects)
- [1.7 A timeline of the revival](#17-a-timeline-of-the-revival)
- [1.8 Hebrew in a sentence: a first sample](#18-hebrew-in-a-sentence-a-first-sample)
- [1.9 What makes Hebrew hard for a machine](#19-what-makes-hebrew-hard-for-a-machine)
- [References](#references)

---

## 1.1 Names

| Name | Who uses it | Note |
|---|---|---|
| עִבְרִית *ivrit* | speakers themselves | "Hebrew", the ordinary name; adjective *ivri* "Hebrew" |
| Modern Hebrew | linguistics, language teaching, UD (`he`) | contrasts with Biblical, Mishnaic, Medieval Hebrew |
| Israeli Hebrew | many linguists (e.g. Berman, Bolozky, Izre'el) | stresses the speech community of Israel |
| Contemporary Hebrew | e.g. Rosén 1977, Itai et al. 2006 | the present-day standard |
| "Israeli" (*Yisraelit*) | Zuckermann 1999 onwards | stresses hybrid origin; see § 1.2 |
| לשון הקודש *leshon ha-kódesh* "the holy tongue" | traditional Jewish usage | the language of Scripture and prayer, Hebrew and Aramaic together; Haredi speakers may contrast it with *ivrit* |
| ISO 639 | `he` (639-1), `heb` (639-2/3) | Ancient Hebrew is `hbo`, a separate code |

Historical stages ([10](10-historical-layers.md)) have their own names: Biblical (Classical) Hebrew, Mishnaic or Rabbinic Hebrew, Medieval Hebrew, the Hebrew of the *Haskala* (Enlightenment), and Modern Hebrew from the revival in the 1880s.

## 1.2 Continuity or a new language? The "Israeli" debate

Two views of what Modern Hebrew *is* coexist in the literature; this manual uses the neutral "Modern Hebrew" and presents both.

**The continuity view** (the mainstream in Israeli linguistics and in the Academy; e.g. Rosén 1977; Kutscher 1982; Sáenz-Badillos 1993 — *not held*): Hebrew was never dead as a written language. It was read, written, prayed and composed in without a break for two thousand years — in liturgy, law, commentary, poetry, letters, science and business. What the revival created was a community of **native speakers**, not a new language. Modern Hebrew's grammar is Semitic at its core: root-and-pattern morphology, binyanim, the construct state, gender agreement, verbless clauses. Its vocabulary and its morphology are mostly continuous with the classical layers ([10](10-historical-layers.md)). On this view, European influence on syntax and semantics is contact influence of the kind every language undergoes.

**The hybrid view.** Zuckermann (1999 onwards; *Language Contact and Lexical Enrichment in Israeli Hebrew* 2003; *Revivalistics* 2020 — *not held*) proposes the name "Israeli" for a language that he describes as Semito-European: genetically both Hebrew and Yiddish (and other European languages of the revivers), with Hebrew providing most of the morphology and lexicon and European languages much of the phonology, syntax and semantics. His evidence includes the European-like vowel system and uvular *r*, SVO order, the calqued semantics of many idioms, the loss of the pharyngeals in the dominant pronunciation, and the *ze* copula ([07](07-syntax.md)). A more radical claim — Wexler (1990, *The Schizoid Nature of Modern Hebrew: A Slavic Language in Search of a Semitic Past*, *not held*) — is that Modern Hebrew is relexified Yiddish, i.e. a Slavic language at its base; it has few followers. Izre'el (2003, *not held*) argued that spoken Israeli Hebrew should be studied as a language in its own right, without committing to a genetic hybrid.

**Assessment for this manual.** The facts both sides point to are real: the morphology is overwhelmingly Semitic and continuous with earlier Hebrew; the phonetics, much of the idiom, and parts of the syntax show European contact. The label one chooses depends on what one weighs as "genetic". For an NLP system the debate has a practical edge: Modern Hebrew text cannot be parsed with a Biblical Hebrew grammar (the PostRab and PTNK treebanks show how far the older varieties differ; [10](10-historical-layers.md), [11](11-ud-analysis.md)), and UD treats Ancient Hebrew (`hbo`) as a separate language.

## 1.3 Speakers and status

**Numbers.** Wikipedia's English article on Modern Hebrew (accessed 2026-10-17) gives about **10.6 million** speakers in its infobox, of whom **6.6 million** native and **4 million** second-language, for 2014, citing the UCLA Language Materials Project, Dekel (2014) and Ethnologue; its text speaks of over 10 million speakers of all levels, citing a 2023 Israeli Central Bureau of Statistics (CBS) survey, with over 6.5 million native speakers — mostly Israeli-born Jews — plus about 2 million immigrants, about 1.5 million Arab citizens of Israel (mostly Arabic-dominant) and about half a million Israelis and Jews abroad. These figures are approximations; we have not checked the CBS survey itself (listed under «wanted» in [sources.md](sources.md)).

**Status.**
- Hebrew was one of the three official languages of Mandatory Palestine (with English and Arabic) from 1922.
- It is the main language of the State of Israel since 1948: of government, schooling, the army, the media, science and culture.
- The Basic Law: Israel as the Nation-State of the Jewish People (2018) declares Hebrew "the language of the State" and gives Arabic a "special status" (it had been treated as an official language before). The change was controversial.
- The **Academy of the Hebrew Language** (founded by law in 1953, successor of the Language Committee of 1890/1904) is the official authority on vocabulary, spelling, transliteration and grammar ([08](08-lexicon-register.md)).

**Outside Israel**, Hebrew is a heritage and second language in Jewish communities, taught in Jewish day schools, and the liturgical language of Jews everywhere (in traditional pronunciations: Ashkenazi, Sephardi, Yemenite and others; [03](03-phonology.md)).

## 1.4 Family

```
Afro-Asiatic
└── Semitic
    ├── East Semitic: Akkadian (†), Eblaite (†)
    └── West Semitic
        ├── Ethiopian Semitic: Ge'ez, Amharic, Tigrinya …
        ├── Modern South Arabian
        └── Central Semitic
            ├── Arabic
            └── Northwest Semitic
                ├── Aramaic (Old, Imperial, Jewish Babylonian, Syriac, Neo-Aramaic …)
                ├── Ugaritic (†)
                └── Canaanite
                    ├── Phoenician–Punic (†)
                    ├── Moabite, Ammonite, Edomite (†)
                    └── Hebrew: Biblical → Mishnaic → Medieval → Modern
```

The tree follows the usual classification (Huehnergard and Rubin 2011; Rubin 2010 — *not held*); the position of Arabic and of Ugaritic is debated. Hebrew's closest living relatives are Aramaic (in Neo-Aramaic varieties) and, more distantly, Arabic — with which it shares cognate roots (Hebrew *shalom* / Arabic *salām* "peace"; Hebrew *kelev* / Arabic *kalb* "dog") and, in Israel, much contact ([08](08-lexicon-register.md)). UD's language family label for Hebrew is Afro-Asiatic, Semitic.

## 1.5 Typological profile

| Feature | Modern Hebrew | Where |
|---|---|---|
| Script | abjad, right to left; vowels optional (niqqud) | [02](02-script-and-spelling.md) |
| Phonology | 5 vowels; ~23 consonants; uvular /ʁ/; pharyngeals merged in the standard; stress mostly final | [03](03-phonology.md) |
| Morphology | root-and-pattern (non-concatenative) stems + concatenative affixes and clitics; fusional inflection | [04](04-nouns-adjectives-pronouns.md), [05](05-verbs.md) |
| Clitic prefixes | ו "and", ה "the", ש "that", ב ל כ מ prepositions — written attached | [06](06-prepositions-particles.md), [11](11-ud-analysis.md) |
| Gender | masculine / feminine, in nouns, adjectives, verbs (incl. 2nd person), pronouns, numerals | [04](04-nouns-adjectives-pronouns.md) |
| Number | singular / plural; dual in a few nouns | [04](04-nouns-adjectives-pronouns.md) |
| Definiteness | prefixed article ה; construct state; no indefinite article | [04](04-nouns-adjectives-pronouns.md) |
| Case | none on nouns; object marker את for definite objects (DOM) | [07](07-syntax.md) |
| Verb categories | 7 binyanim (voice/valency); past (suffixing), future (prefixing), present = participle; imperative; infinitive | [05](05-verbs.md) |
| Basic word order | SVO; VS after fronted elements and with unaccusatives | [07](07-syntax.md) |
| Adpositions | prepositions, many inflected for person | [06](06-prepositions-particles.md) |
| Noun phrase | N–Adj, N–Dem, N–Genitive; numerals before N (except "one") | [04](04-nouns-adjectives-pronouns.md), [07](07-syntax.md) |
| Copula | none in the present (or pronominal הוא/זה); היה in past/future | [07](07-syntax.md) |
| "Have" | none: existential יש/אין + ל | [07](07-syntax.md) |
| Pro-drop | partial: 1st/2nd person past and future | [07](07-syntax.md) |
| Relatives | prefix ש / word אשר; resumptive pronouns | [07](07-syntax.md) |
| Negation | לא; אין for existence; negative concord | [07](07-syntax.md) |
| Questions | intonation (formal האם); wh-fronting; no preposition stranding | [07](07-syntax.md) |
| Address | no T/V distinction; gendered "you" | [09](09-pragmatics-politeness.md) |

In WALS-type terms Hebrew is a head-initial, prepositional, SVO language with rich agreement and non-concatenative stem morphology — the last being its most distinctive feature for computation.

## 1.6 Varieties and sociolects

Modern Hebrew has no regional dialects in the classical sense: the speech community is young and mobile, and the standard spread through schools, the army and the media within two generations. Variation is **social and ethnic**, and it is shrinking among the native-born (Blanc 1964; Bolozky 1997; Gafter 2014 — *not held*):

| Variety | Main features | Note |
|---|---|---|
| General Israeli (the standard) | uvular [ʁ]; ח and כ both [χ]; א and ע both [ʔ] or silent; 5 vowels | originally the speech of Ashkenazi-origin native speakers; now the norm of media and most native speakers |
| Mizrahi (Oriental) pronunciation | pharyngeal [ħ] for ח and [ʕ] for ע; often an alveolar trilled [r]; gemination in careful speech | traditional among speakers of Middle Eastern and North African origin, especially older ones; prestigious in some contexts (liturgy, some singers), stigmatised in others; receding among the young (Gafter 2014, 2016) |
| Hebrew of Arabic-speaking citizens | pharyngeals and emphatic consonants carried over from Arabic; fluent second-language Hebrew, much code-switching | about 1.5 million speakers (§ 1.3); research on their Hebrew: Amara, Henkin — *not held* |
| Russian-influenced Hebrew | among the ~1 million immigrants from the former USSR since 1989 and their children: palatalised consonants, Russian intonation, loans | the largest immigrant group of recent decades |
| Amharic- and Tigrinya-influenced Hebrew | Ethiopian Jews (Beta Israel) | small literature |
| Haredi (ultra-Orthodox) Hebrew | many Yiddish words and calques; Ashkenazi forms in religious terms; Aramaic and rabbinic vocabulary; some communities avoid secular neologisms | Yiddish remains the home language in some Hasidic groups, Hebrew in most |
| Anglo-Hebrew | English speakers' Hebrew; loans and calques | small but visible |
| Youth and army slang | loans from Arabic, English; clipping, acronyms | [08](08-lexicon-register.md), [09](09-pragmatics-politeness.md) |

**Register** matters more than region: there is a large gap between the formal written language (close to the classical norms of morphology) and casual speech (with levelled verb forms, *yesh li et*, frozen *haya*, the *ze* copula). Chapters [05](05-verbs.md), [07](07-syntax.md) and [08](08-lexicon-register.md) mark "normative" vs "spoken" throughout.

## 1.7 A timeline of the revival

Dates as commonly given in the histories of the revival (Fellman 1973; Sáenz-Badillos 1993; Harshav 1993 — *not held*; Wikipedia, "Revival of the Hebrew language", accessed 2026-10-17). The narrative of a single heroic reviver is a simplification the specialists themselves criticise: the revival was the work of schools, teachers, writers and a generation of children (Fellman 1973; Harshav 1993).

| Year | Event |
|---|---|
| 18th–19th c. | the *Haskala* (Jewish Enlightenment) writes secular prose, journalism and science in Hebrew, mostly in a biblical style ([10](10-historical-layers.md)) |
| 1856 | the first Hebrew weekly, *Ha-Maggid*, appears (Lyck, East Prussia) |
| 1878 | Eliezer Ben-Yehuda (born Perlman, 1858) conceives the idea of national revival "in the land of the fathers" during the Russo-Turkish war, by his own account (introduction to his dictionary; [08](08-lexicon-register.md)) |
| 1879 | his article *She'ela nikhbada* "A weighty question" in *Ha-Shachar* (Vienna) argues for a Jewish national home |
| 1881 | Ben-Yehuda settles in Jerusalem and resolves to speak only Hebrew at home |
| 1882 | his son Ben-Zion (later Itamar Ben-Avi) is born, raised as the first child with Hebrew as home language (as the story goes) |
| 1880s–1890s | Hebrew is used as the language of instruction ("Hebrew in Hebrew") in schools of the new agricultural settlements; Hebrew kindergartens from the 1890s |
| 1890 | the Language Committee (*Va'ad ha-Lashon*) is founded; refounded 1904 |
| 1906 | the Herzliya Hebrew Gymnasium opens in Jaffa: a full secondary school in Hebrew |
| 1908 | the first volume of Ben-Yehuda's *Complete Dictionary of Ancient and Modern Hebrew* appears; it is completed in 17 volumes in 1959, long after his death (1922) |
| 1913–14 | the "War of the Languages": teachers and students strike against the plan to teach in German at the new Technion in Haifa; Hebrew wins |
| 1918/1925 | the Hebrew University of Jerusalem is founded / opened |
| 1922 | Hebrew becomes an official language of Mandatory Palestine |
| 1948 | the State of Israel; mass immigration (1948–1951) makes Hebrew the common language of immigrants from dozens of countries; the *ulpan* intensive Hebrew courses begin |
| 1953 | the Academy of the Hebrew Language is established by law |
| 1989–2000 | about a million immigrants from the former USSR |
| 2018 | the Nation-State Basic Law names Hebrew the language of the state |

By the time of the first Israeli censuses, Hebrew was the main language of most Jewish native-born Israelis; the shift took about two generations (Bachi 1956 — *not held*, on the 1948–1954 statistics).

## 1.8 Hebrew in a sentence: a first sample

One sentence showing several of the features the following chapters describe:

```
כשהגענו לבית שלהם, אמרו לנו שאין להם זמן.
kshe-higánu la-báyit shelahem, ámru lánu she-eyn lahem zman
when-arrive.PST.1PL to.DEF-house of.them say.PST.3PL to.us COMP-NEG.EXIST to.them time
'When we got to their house, they told us they had no time.'
```

- *kshe-*, *la-*, *she-* are clitics written on the next word ([06](06-prepositions-particles.md), [11](11-ud-analysis.md)).
- *la-báyit*: the article is hidden in the vowel of the preposition ([02](02-script-and-spelling.md)).
- *higánu* has no subject pronoun (1PL past, pro-drop); *ámru* has none either: it is impersonal-ish "they" with an antecedent in context ([07](07-syntax.md)).
- *eyn lahem zman*: possession by a negative existential ([07](07-syntax.md)).
- *shelahem* "their" is the analytic possessive of speech, beside formal *beytam* ([04](04-nouns-adjectives-pronouns.md)).
- The embedded clause keeps the present: "that there is not to them time" — no backshift ([07](07-syntax.md)).

## 1.9 What makes Hebrew hard for a machine

1. **Unpointed script**: one written word, many readings ([02](02-script-and-spelling.md) counts the ambiguity in the UD corpora).
2. **Clitics written together** with the next or previous word: a third of tokens are multiword ([11](11-ud-analysis.md)).
3. **Non-concatenative morphology**: the root is interleaved with the pattern ([05](05-verbs.md)).
4. **Spelling variation** between full and defective spelling and between the norm and practice ([02](02-script-and-spelling.md)).
5. **Register gap** between written and spoken grammar ([08](08-lexicon-register.md)).
6. **Right-to-left text mixed with numbers and Latin**.
7. **Small open treebanks**: about 14 000 sentences of UD in all, one of them non-commercial ([11](11-ud-analysis.md)).

Mova's plan for these is in [12](12-mova.md).

---

## References

Works marked *not held* are not in the local library; they are listed under «wanted» in [sources.md](sources.md).

- Bachi, Roberto. 1956. A statistical analysis of the revival of Hebrew in Israel. *Scripta Hierosolymitana* 3. *Not held.*
- Ben-Yehuda, Eliezer. (1908–1959). Introduction to the *Complete Dictionary* (*ha-Mavo ha-Gadol*). Public domain, Project Ben-Yehuda: https://benyehuda.org — local: `$RESEARCH_RAW/lexres/dictionaries/ben-yehuda-pby/txt/p22/m7327.txt`.
- Blanc, Haim. 1964. Israeli Hebrew texts. *Studies in Egyptology and Linguistics in Honour of H. J. Polotsky*. *Not held.*
- Bolozky, Shmuel. 1997. Israeli Hebrew phonology. In A. Kaye (ed.), *Phonologies of Asia and Africa*. *Not held.*
- Dekel, Nurit. 2014. *Colloquial Israeli Hebrew: A Corpus-based Survey*. Berlin: De Gruyter. *Not held.*
- Fellman, Jack. 1973. *The Revival of a Classical Tongue: Eliezer Ben Yehuda and the Modern Hebrew Language*. The Hague: Mouton. *Not held.*
- Gafter, Roey J. 2014. *"The most beautiful and correct Hebrew": Authenticity, ethnic identity and linguistic variation in the Greater Tel Aviv area*. PhD thesis, Stanford. *Not held.*
- Harshav, Benjamin. 1993. *Language in Time of Revolution*. Berkeley: University of California Press. *Not held.*
- Huehnergard, John and Aaron D. Rubin. 2011. Phyla and waves: models of classification of the Semitic languages. In S. Weninger (ed.), *The Semitic Languages: An International Handbook*. *Not held.*
- Itai, Alon, Shuly Wintner and Shlomo Yona. 2006. A Computational Lexicon of Contemporary Hebrew. LREC 2006. Local: `$RESEARCH_RAW/lingbulk-2006-itai-computational-lexicon-contemporary-hebrew/`.
- Izre'el, Shlomo. 2003. The emergence of spoken Israeli Hebrew. In B. Hary (ed.), *Corpus Linguistics and Modern Hebrew*. Tel Aviv. *Not held.*
- Kutscher, Eduard Yechezkel. 1982. *A History of the Hebrew Language*. Jerusalem: Magnes / Leiden: Brill. *Not held* (listed in `$RESEARCH_RAW/hebgram/grammars.tsv`).
- Rosén, Haiim B. 1977. *Contemporary Hebrew*. The Hague: Mouton. *Not held.*
- Rubin, Aaron D. 2010. *A Brief Introduction to the Semitic Languages*. Piscataway: Gorgias. *Not held.*
- Sáenz-Badillos, Angel. 1993. *A History of the Hebrew Language*. Cambridge: CUP. *Not held* (listed in `grammars.tsv`).
- Wexler, Paul. 1990. *The Schizoid Nature of Modern Hebrew: A Slavic Language in Search of a Semitic Past*. Wiesbaden: Harrassowitz. *Not held.*
- Wikipedia (English), "Modern Hebrew" and "Revival of the Hebrew language", https://en.wikipedia.org/wiki/Modern_Hebrew, https://en.wikipedia.org/wiki/Revival_of_the_Hebrew_language (accessed 2026-10-17; CC BY-SA, consulted, not copied).
- Zuckermann, Ghil'ad. 2003. *Language Contact and Lexical Enrichment in Israeli Hebrew*. Houndmills: Palgrave Macmillan. *Not held.*
- Zuckermann, Ghil'ad. 2020. *Revivalistics: From the Genesis of Israeli to Language Reclamation in Australia and Beyond*. New York: OUP. *Not held.*
- UD treebank cards for Hebrew, Mova repo `treebanks/cards.tsv`.
