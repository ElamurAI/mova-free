# Ancient Hebrew (hbo) — a reference manual for Mova

This is an open, complete reference grammar of **Biblical Hebrew** (ISO 639-3 `hbo`, Ancient Hebrew), with a chapter on **Mishnaic Hebrew** (`hbo-x-mishnaic`) and one comparing both with Modern Hebrew. It is written for two readers at once: a person who wants to read the Torah in Hebrew, and Mova, whose tools read the Torah as dependency trees. Every grammatical point is illustrated from the Torah as Mova holds it, with the sentence id of our trees, and every chapter ends with a section saying how Mova's tools handle the point and where they stop.

The manual is written in our own words from public-domain grammars (Gesenius–Kautzsch–Cowley 1910, Davidson, Driver, Harper, Wickes, BDB 1906 and others), open-access research (Khan 2020, Heijmans 2020, Hornkohl & Khan 2021, all CC BY 4.0) and our own measurements on the data. Works under non-commercial or share-alike terms are cited and paraphrased, never copied. The full list, and the key works we lack, are in [sources.md](sources.md).

## Chapters

| # | file | what it covers |
|---|---|---|
| 1 | [01-script.md](01-script.md) | the alphabet, final letters, matres lectionis, the Tiberian vowel points, shewa, dagesh, maqqef, ketiv/qere, Unicode; the letter classes of Sefer Yetzirah |
| 2 | [02-cantillation.md](02-cantillation.md) | the accents (teʿamim): disjunctive and conjunctive, the prose and poetic systems, the dichotomy of the verse, pausal forms, accents as syntax |
| 3 | [03-phonology.md](03-phonology.md) | consonants, begadkefat, gutturals and resh, vowels, syllables, stress, sandhi, historical sound changes, reading traditions |
| 4 | [04-noun.md](04-noun.md) | gender, number, the article, state and the construct chain, suffixes, noun patterns and segolates, irregular nouns, adjectives, numerals |
| 5 | [05-pronouns-particles.md](05-pronouns-particles.md) | pronouns, suffixes on verbs and prepositions, the object marker אֵת, demonstratives, interrogatives, the relative, negatives, prepositions, conjunctions, adverbs, interjections |
| 6 | [06-verb.md](06-verb.md) | the root, the binyanim, qatal, yiqtol, wayyiqtol, weqatal, volitives, infinitives, participles, the strong verb's paradigms, the aspect–tense debate |
| 7 | [07-weak-verbs.md](07-weak-verbs.md) | the weak classes: guttural, I-nun, I-yod/waw, hollow, III-he, geminate, doubly weak and irregular verbs |
| 8 | [08-syntax.md](08-syntax.md) | word order, verbless clauses, agreement, the construct, relative clauses, objects, infinitives, coordination, subordination, questions, negation, dislocation, oaths |
| 9 | [09-discourse.md](09-discourse.md) | narrative and its mainline, background clauses, procedure, direct speech and לֵאמֹר, וַיְהִי, וְהִנֵּה, participant tracking, genres, politeness, and `macula prag` |
| 10 | [10-lexicon.md](10-lexicon.md) | roots and word formation, the Torah's frequency profile, semantic fields, polysemy, loanwords, names, and Mova's Torah dictionary |
| 11 | [11-mishnaic.md](11-mishnaic.md) | Mishnaic Hebrew: periods, manuscripts, phonology, pronouns, שֶׁ־ and שֶׁל, the noun, the verb and the participle-present, syntax, vocabulary, worked passages |
| 12 | [12-biblical-mishnaic-modern.md](12-biblical-mishnaic-modern.md) | the stages compared, with counts book by book; the dating debate; codes, treebanks and licences; why a Modern Hebrew parser misreads the Bible |
| 13 | [13-mova-tools.md](13-mova-tools.md) | Mova's tools for hbo in one place: `macula` trees, FEATS/MISC, the UD conventions, PTNK scores, `macula prag`, the dictionary builder, the course, the parse database, known defects, gaps |
| — | [sources.md](sources.md) | every source cited, with licence and local path or open URL; the wanted list |

## Conventions

**Transliteration** (close to the SBL academic style):

| letters | | | | | | | | | | | |
|---|---|---|---|---|---|---|---|---|---|---|---|
| א ʾ | בּ b, ב ḇ | גּ g, ג ḡ | דּ d, ד ḏ | ה h | ו w | ז z | ח ḥ | ט ṭ | י y | כּ k, כ ḵ | ל l |
| מ m | נ n | ס s | ע ʿ | פּ p, פ p̄ | צ ṣ | ק q | ר r | שׂ ś | שׁ š | תּ t, ת ṯ | |

Vowels: pataḥ *a*, qameṣ *ā*, qameṣ ḥaṭuf *o*, segol *e*, ṣere *ē* (with yod *ê*), ḥireq *i* (with yod *î*), ḥolem *ō* (with waw *ô*), qibbuṣ *u*, shureq *û*; vocal shewa *ə*; ḥaṭef vowels *ă ĕ ŏ*; final qameṣ + he *â*. Dagesh forte doubles the letter (*wayyōʾmer*). The divine name יהוה is given as written and transliterated *YHWH*; the text reads it *ʾăḏōnāy* (chapter 1, perpetual qere).

**Examples** are given as:

> **GEN.1.3** · `wlc:GEN:2`
> וַיֹּאמֶר אֱלֹהִים יְהִי אוֹר
> *wayyōʾmer ʾĕlōhîm yəhî ʾôr* — and-said God let-be light — "And God said: Let there be light." (JPS 1917)

- **GEN.1.3** is the verse (book, chapter, verse) as in our trees' `# verse` line;
- `wlc:GEN:2` is the sentence id (`# sent_id`) in `$MOVA_DATA/corpora/judaism/text/torah-wlc-macula/GEN.conllu`, so any example can be opened as a tree;
- the Hebrew is the Westminster Leningrad Codex as given by MACULA; in running examples the accents are usually left out for legibility, while quoted tree lines keep them as in the data;
- the word-for-word gloss follows MACULA's English glosses where they help; the translation is the Jewish Publication Society's 1917 version (public domain), which Mova holds aligned verse by verse.

Mishnah passages are cited by tractate, chapter and mishnah (m. Berakhot 1:1) from public-domain editions; their translations are ours.

**Citations.** GKC = Gesenius' Hebrew Grammar, ed. Kautzsch, tr. Cowley, 2nd English ed., Oxford 1910, cited by section and paragraph letter (GKC §49 b). BDB = Brown–Driver–Briggs 1906. Davidson = A. B. Davidson, *Hebrew Syntax*, 3rd ed. 1901 (cited by §) or *Introductory Hebrew Grammar* (1916). Driver = S. R. Driver, *A Treatise on the Use of the Tenses in Hebrew*, 3rd ed. 1892. Khan 2020 = *The Tiberian Pronunciation Tradition of Biblical Hebrew*. Other works by author and year; full entries in [sources.md](sources.md).

**Numbers.** Every count is measured on our data and given with its date (2026-10-17 unless said otherwise). The Torah as converted: 5 853 sentences, 125 807 tokens in MACULA's segmentation (prefixes and suffixes are tokens of their own).

## The data behind the examples

| what | where | licence |
|---|---|---|
| Torah trees (CoNLL-U, hbo) | `$MOVA_DATA/corpora/judaism/text/torah-wlc-macula/{GEN,EXO,LEV,NUM,DEU}.conllu`, `verses.tsv` | converted from MACULA Hebrew, CC BY 4.0 |
| Hebrew pragmatics by rules | `$MOVA_DATA/corpora/judaism/hbo-prag.tsv` | ours |
| Torah dictionary | `$MOVA_DATA/corpora/judaism/dict/torah-en.tsv`, `.examples` | ours, from BDB 1906 (public domain) and OSHB (CC BY 4.0) |
| JPS 1917 English | `$MOVA_DATA/corpora/judaism/text/torah-jps1917/` | public domain |
| UD Ancient Hebrew PTNK | `$RESEARCH_RAW/lexres/treebanks/ud-2.18/nc/UD_Ancient_Hebrew-PTNK/` | CC BY-NC: a measure only, never copied |

The tools are described in [13-mova-tools.md](13-mova-tools.md); the commands are:

```
cargo run --release -p macula -- <lowfat-dir> <out-dir> GEN,EXO,LEV,NUM,DEU   # trees
cargo run --release -p macula -- compare <out-dir> <ptnk.conllu>…            # against PTNK
cargo run --release -p macula -- prag <trees-dir> <hbo-prag.tsv>               # pragmatics
cargo run --release -p macula -- dict <trees-dir> <BDB.jsonl> <LexicalIndex.xml> <dict.tsv> [<en-pool.tsv>]
```

## Scope and limits

- **Biblical Hebrew means, in practice, the Torah.** That is the text Mova has as trees. The grammar describes the whole of Classical Biblical Hebrew and cites the Prophets and Writings where the Torah lacks an example, saying so.
- **The morphology in the examples is MACULA's** (via OSHB); Mova does not yet analyse Hebrew words itself. The relations are Mova's own, by rule.
- **Aramaic** (the Aramaic of Daniel and Ezra, the Targums) is outside this manual; it is the subject of the Aramaic manual (`languages/arc/`, in preparation).
- **Modern Hebrew** has its own manual (`languages/he/`); chapter 12 links the two.

Licence: this manual is ours, under the Elamur Cosmic License ([LICENSE.md](../../LICENSE.md)). The texts it quotes keep their own terms, listed in [sources.md](sources.md) and [licence/DATA.md](../../licence/DATA.md).
