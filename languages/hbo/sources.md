# Sources

Every work cited in the manual, grouped by kind, with its licence status and where it is held. Paths under `$RESEARCH_RAW/` are the research library (not in git); the catalogue of the grammars is `$RESEARCH_RAW/hebgram/grammars.tsv`, that of the lexical resources `$RESEARCH_RAW/lexres/resources.tsv`, that of the scriptural texts `$RESEARCH_RAW/scripture/texts.tsv`. Each chapter's own "Sources" section gives the sections and pages used there.

**How each kind is used.** Public domain and CC BY: quoted briefly, paraphrased, used for examples and counts. CC BY-SA (Wikibooks, Wiktionary, the UD documentation, some papers): read and paraphrased, never copied into the repository. CC BY-NC and other non-commercial terms (UD PTNK, ETCBC, several Open Book Publishers volumes): cited and described, nothing copied, used at most as a measure. "PD-USA only" (Segal 1909, 1927): cited by section, at most a short quote. See `licence/DATA.md`.

## 1. Data Mova reads

| work | licence | where |
|---|---|---|
| MACULA Hebrew Linguistic Datasets (Clear Bible / Biblica; Westminster Hebrew Syntax of the J. Alan Groves Center; OSHB text and morphology) | CC BY 4.0 | https://github.com/Clear-Bible/macula-hebrew/ ; converted trees `$MOVA_DATA/corpora/judaism/text/torah-wlc-macula/` |
| Open Scriptures Hebrew Bible (morphhb): Westminster Leningrad Codex with lemmas and morphology | CC BY 4.0 (WLC public domain) | https://github.com/openscriptures/morphhb ; `$RESEARCH_RAW/scripture/1-originals/oshb-morphhb/` |
| OpenScriptures HebrewLexicon (Lexical Index, BDB XML, Strong's XML) | CC BY 4.0 | `$RESEARCH_RAW/lexres/dictionaries/oshb-hebrewlexicon/` |
| The Holy Scriptures according to the Masoretic Text (JPS 1917), via eBible.org | public domain | `$MOVA_DATA/corpora/judaism/text/torah-jps1917/` |
| Mishnah: Torat Emet 357; ed. Romm, Vilna 1913 (Hebrew); Sefaria Community Translation (CC0); Kulp, Mishnah Yomit (CC BY) | PD / CC0 / CC BY | `$RESEARCH_RAW/scripture/2-rabbinic/sefaria/json/Mishnah/` |
| Clear Bible speaker/quotation data; Staps 2024 annotated כִּי set; ETCBC genre ranges (MIT) | CC BY 4.0 / MIT | as named in chapter 9 |
| UD Ancient Hebrew PTNK, UD 2.18 (Swanson & Tyers) | CC BY-NC 4.0 per README: **a measure only** | `$RESEARCH_RAW/lexres/treebanks/ud-2.18/nc/UD_Ancient_Hebrew-PTNK/` |
| UD Hebrew HTB | CC BY-NC-SA 4.0: a measure only | `$RESEARCH_RAW/lexres/treebanks/ud-2.18/nc/UD_Hebrew-HTB/` |
| UD Hebrew IAHLTwiki, IAHLTknesset, PostRab | CC BY-SA 4.0 | `$RESEARCH_RAW/lexres/treebanks/ud-2.18/` |
| Universal Dependencies documentation for Hebrew and Ancient Hebrew | CC BY-SA 4.0: paraphrased | `$RESEARCH_RAW/hebgram/sa/ud-docs-he-hbo/` |
| The Unicode Standard, ch. 9 (Hebrew), and the Unicode Character Database | Unicode licence | https://www.unicode.org/ |

## 2. Grammars (public domain)

- **GKC** — Gesenius, W.; Kautzsch, E. (ed.); Cowley, A. E. (tr.) 1910. *Gesenius' Hebrew Grammar*, 2nd English ed. Oxford: Clarendon. Wikisource transcription `$RESEARCH_RAW/hebgram/open/gesenius-kautzsch-cowley-1910-wikisource/` (text public domain; wiki markup CC BY-SA, not copied); scan https://archive.org/details/geseniushebrewgr00geseuoft , `$RESEARCH_RAW/hebgram/pd/gesenius-kautzsch-cowley-1910/`. German 28th ed. 1909 and Bergsträsser 1918 also held (`pd/gesenius-kautzsch-1909`, `pd/gesenius-bergstraesser-1918`).
- Davidson, A. B. 1901. *Hebrew Syntax*, 3rd ed. Edinburgh: T&T Clark. `$RESEARCH_RAW/hebgram/pd/davidson-hebrew-syntax-1901/`
- Davidson, A. B. 1916. *An Introductory Hebrew Grammar*, 19th ed. (rev. McFadyen). `$RESEARCH_RAW/hebgram/pd/davidson-introductory-hebrew-grammar-1916/`
- Driver, S. R. 1892. *A Treatise on the Use of the Tenses in Hebrew*, 3rd ed. Oxford. `$RESEARCH_RAW/hebgram/pd/driver-tenses-1892/`
- Harper, W. R. 1891. *Elements of Hebrew*; 1908. *Elements of Hebrew Syntax*; 1910. *Introductory Hebrew Method and Manual*. `$RESEARCH_RAW/hebgram/pd/harper-*`
- Joüon, P. 1923. *Grammaire de l'hébreu biblique*. Rome. `$RESEARCH_RAW/hebgram/pd/jouon-grammaire-1923/`
- Green, W. H. 1888. *A Grammar of the Hebrew Language*; Mitchell, H. G. 1902. *Hebrew Lessons*. `$RESEARCH_RAW/hebgram/pd/`
- Wickes, W. 1881. *A Treatise on the Accentuation of the Three So-called Poetical Books*; 1887. *A Treatise on the Accentuation of the Twenty-one So-called Prose Books*. `$RESEARCH_RAW/hebgram/pd/wickes-accentuation-*`
- Praetorius, F. 1901. *Über die Herkunft der hebräischen Accente*. `$RESEARCH_RAW/hebgram/pd/praetorius-herkunft-accente-1901/`
- Ginsburg, C. D. 1897. *Introduction to the Massoretico-Critical Edition of the Hebrew Bible*. `$RESEARCH_RAW/hebgram/pd/ginsburg-introduction-masoretic-1897/`
- Further PD grammars in the library, consulted for cross-checks: Ewald 1836/1870/1891, Olshausen 1861, Stade 1879, König 1881–1895, Strack 1890, Bauer–Leander 1922, Beer 1920, Luzzatto 1853, Wright 1890, Zimmern 1898, Nöldeke 1899, Brockelmann 1908–1913 (scope private), Lidzbarski 1898, Cooke 1903. `$RESEARCH_RAW/hebgram/pd/`

## 3. Grammars and studies of Mishnaic and later Hebrew

- Segal, M. H. 1909. *Mišnaic Hebrew and its Relation to Biblical Hebrew and to Aramaic* (from JQR 20, 1908). **PD-USA only**: cited. `$RESEARCH_RAW/hebgram/private/segal-misnaic-hebrew-1909/`
- Segal, M. H. 1927. *A Grammar of Mishnaic Hebrew*. Oxford. **PD-USA only**: cited by §. `$RESEARCH_RAW/hebgram/private/segal-grammar-mishnaic-1927/`
- Heijmans, S. (ed.) 2020. *Studies in Rabbinic Hebrew*. Cambridge: Open Book Publishers. CC BY 4.0. https://doi.org/10.11647/OBP.0164 ; `$RESEARCH_RAW/hebgram/open/heijmans-studies-rabbinic-hebrew-2020/` (chapters by Breuer, Henshke, Shemesh-Raiskin, Stadel, Avirbach, Wormser, Efrati, Heijmans).
- Hornkohl, A. D.; Khan, G. (eds.) 2021. *New Perspectives in Biblical and Rabbinic Hebrew*. OBP. CC BY 4.0. `$RESEARCH_RAW/hebgram/open/hornkohl-khan-new-perspectives-2021/`
- Cook, J. A. 2017. "Language contact and the genesis of Mishnaic Hebrew." CC BY-SA. `$RESEARCH_RAW/hbo-2017-cook-language-contact-genesis-mishnaic-hebrew/`
- Shmidman, A. et al. 2022. "Introducing BEREL: BERT Embeddings for Rabbinic-Encoded Language." `$RESEARCH_RAW/hbo-2022-shmidman-introducing-berel-*`
- Tal, R. et al. 2025. "A New Hebrew Universal Dependency Treebank: The First Treebank of Post-Rabbinic Historical Hebrew." TLT 2025.
- Tsarfaty, R. et al. 2019. "What's wrong with Hebrew NLP? And how to make it right." `$RESEARCH_RAW/aclbulk-2019-tsarfaty-what-s-wrong-hebrew-nlp/`

## 4. Pronunciation, Masorah and reading traditions

- Khan, G. 2020. *The Tiberian Pronunciation Tradition of Biblical Hebrew*, vols. 1–2. Cambridge: OBP. CC BY 4.0. https://doi.org/10.11647/OBP.0163 ; `$RESEARCH_RAW/hebgram/open/khan-tiberian-pronunciation-v1-2020/`, `-v2-2020/`
- Khan, G.; Hornkohl, A. D. (eds.) 2020. *Studies in Semitic Vocalisation and Reading Traditions*. OBP. CC BY 4.0. `$RESEARCH_RAW/hebgram/open/khan-hornkohl-semitic-vocalisation-2020/`
- Posegay, N. 2021. *Points of Contact*. OBP. CC BY 4.0. `$RESEARCH_RAW/hebgram/open/posegay-points-of-contact-2021/`
- Kantor, B. 2023. *A Linguistic Classification of the Reading Traditions of Biblical Hebrew*. OBP. CC BY-NC: cited only.
- Hornkohl, A. D. 2023. *The Historical Depth of the Tiberian Reading Tradition*; Khan, Crowther, Hornkohl (eds.) 2022, *Studies in the Masoretic Tradition*; *Interconnected Traditions* (Festschrift Khan, 2025). OBP, CC BY-NC: cited only. `$RESEARCH_RAW/hebgram/nc/`
- Suchard, B. D. 2019. "Sound changes in the (pre-)Masoretic reading tradition …" Studia Orientalia Electronica 7. Licence uncertain (footer CC BY 3.0, registry NC-SA): cited only. `$RESEARCH_RAW/hbo-2019-suchard-*`
- De Hoop, R.; Sanders, P. 2022. "The system of Masoretic accentuation: some introductory issues." JHS. CC BY 4.0. `$RESEARCH_RAW/hbo-2022-hoop-system-masoretic-accentuation-*`
- Pitcher, S. 2021. "Towards a prosodic model of Tiberian Hebrew." CC BY. `$RESEARCH_RAW/hbo-2021-pitcher-*`
- Naudé, J. A.; Miller-Naudé, C. L. 2017/2018. On the interface of syntax and prosody (left dislocation and the accents). `$RESEARCH_RAW/hbo-2018-naude-interface-syntax-prosody-*`
- Wu, L.; Lowery, K. 2006. "A Hebrew tree bank based on cantillation marks." LREC. CC BY-NC: cited only. `$RESEARCH_RAW/lingbulk-2006-wu-hebrew-tree-bank-based-cantillation/`

## 5. Lexicons and dictionaries

- **BDB** — Brown, F.; Driver, S. R.; Briggs, C. A. 1906. *A Hebrew and English Lexicon of the Old Testament*. Oxford. Public domain. Digital text (Sefaria) `$RESEARCH_RAW/lexres/dictionaries/sefaria/BDB_Dictionary.jsonl`; scan https://archive.org/details/hebrewenglishlex00browuoft
- Gesenius, W. 1857. *Hebrew and Chaldee Lexicon to the Old Testament Scriptures*, tr. S. P. Tregelles. Public domain. `$RESEARCH_RAW/lexres/dictionaries/internet-archive/gesenius-tregelles-1857/`
- Jastrow, M. 1903. *A Dictionary of the Targumim, the Talmud Babli and Yerushalmi, and the Midrashic Literature*. Public domain. `$RESEARCH_RAW/lexres/dictionaries/sefaria/Jastrow_Dictionary.jsonl`; scans https://archive.org/details/dictionaryoftarg01jastuoft
- Levy, J. 1876–1889. *Neuhebräisches und chaldäisches Wörterbuch*. Public domain. `$RESEARCH_RAW/lexres/dictionaries/internet-archive/levy-neuhebr-1876-1889/`
- Davidson, B. 1848. *The Analytical Hebrew and Chaldee Lexicon*; Fürst 1867; Siegfried–Stade 1893; Mandelkern 1896 (concordance); Wigram 1866 (concordance). Public domain. `$RESEARCH_RAW/hebgram/pd/`
- Klein, E. *A Comprehensive Etymological Dictionary of the Hebrew Language*: only Sefaria's CC BY-NC text is held, for reference.

## 6. Papers and monographs on Biblical Hebrew

Held under `$RESEARCH_RAW/hbo-<year>-<author>-…/` and `hbobook-…/`; licence per folder's `SOURCES.md` (open access unless noted; NC works cited only).

**Verb and tense–aspect:** Andrason 2010 (yiqtol), 2011 (wayyiqtol), 2012 (performative qatal), 2013, 2016, 2026; Baranowski 2016 (wayyiqtol and Amarna); Benton 2009 (Niphal/Hitpael aspect; NC-ND, cited); Collins 1995 (wayyiqtol pluperfect; CC BY-SA, cited); Cook 2008 (vav-prefixed forms); Doron 2020 (infinitive); Dyck 2015; Ehrensvärd 2022 (negating the infinitive); Endo 1993 (Joseph story); Hutton & Marzouk 2012 (the tG stem); Isaksson 2024 (*The Verb in Classical Hebrew*, OBP, NC: listed only); Joosten 2020 (participle; metadata only); Olivero 2023 (yiqtol as evidential; metadata only); Price 2014 (fem. sg. qal); Sato 2021 (statives; metadata only); Schiller 2020 (x-qatal).

**Syntax, word order, information structure:** Boulet 2019 (accusative); Bunis 2025 (the object marker); Floor 2004; Holmstedt 2006, 2011 (subject–verb order in Genesis), 2014; Joosten 2025 (word order); Kaajan 2019 (non-configurational); Miller-Naudé & Naudé 2016 (the adjective), 2019 (dislocations), 2026 (quotative frames); Naaijer 2017 (object), 2020 (clause structure); Bijl 2019; Snyman 2004 (negation); Van der Merwe 2018 (fronting); Westbury 2010, 2014, 2016 (left dislocation), 2018 (wayihi); Wilson 2014 (valency); Zulker 2025 (apodosis); Benigni 2022 (כִּי); Meister 1996 (interrogatives); Staps 2019, 2022 (prepositions, לִפְנֵי); Hardy 2022 (*The Development of Biblical Hebrew Prepositions*); Kroeze 1994 (gender); Stein 2008 (the noun).

**Discourse and pragmatics:** Li 2006 (va-yehi); Longacre 2006; Lenzi 2015 (sequences of forms); Yoo 2008 (paragraphs); Runge 2007 (participant reference); Levinsohn 2018; Winther-Nielsen 2016 (participant tracking); Savran 2009; Warren-Rothlin 2007 (politeness); Hare 2018 (gendered speech); Kim 2023 (forms of address; cited, not read); Patty 2026 (narrative conversation); Andrason, Hornea & Joubert 2020 (interjections); Dzikiewicz 2017 (Gen 1:1).

**Lexicon:** Legnaioli 2016 (time); Vergari 2021 (polysemy); Notarius 2025 (loanwords in archaic BH); Azar, Pahmer & Waxman 2020 (a thesaurus of Biblical Hebrew); Benbaji, Doron & Hénot-Mortier 2022 (embeddings and denominal roots); Hacohen 2021 (DOM in Modern Hebrew).

**History and dating:** Rezetko & Young 2014 (*Historical Linguistics and Biblical Hebrew*, open access); Rezetko 2016, 2019; Young 2008; Naudé 2016; Hornkohl 2020 (DSS and Tiberian); Hornkohl 2024 (*Diachronic Diversity in CBH*, NC: cited); Winther-Nielsen 2019; Hutton 2015; Stefaniak 2026 (Arad inscriptions).

**Resources and computation:** Swanson & Tyers 2022 (UD PTNK, LREC); Roorda 2014, 2015, 2019 (LAF-Fabric, Text-Fabric); Winther-Nielsen 2016, 2017 (tools for learners); Wu & Lowery 2006.

## 7. Open web

- Wikibooks, *Biblical Hebrew* (CC BY-SA; read, not copied): `$RESEARCH_RAW/hebgram/sa/wikibooks-en-hebrew/`
- unfoldingWord Hebrew Grammar (CC BY-SA; read, not copied): `$RESEARCH_RAW/hebgram/sa/unfoldingword-uhg/`
- Wikipedia, "Sefer Torah", "Paleo-Hebrew alphabet" (CC BY-SA; facts only).

## 8. Mova's own

`tools/macula/README.md` and `src/main.rs`; `tools/cycle/src/db.rs`; `docs/school.md`; `licence/DATA.md`; `$MOVA_DATA/corpora/judaism/hbo-prag.tsv`, `dict/torah-en.tsv(.examples)`, `parsedb-hbo/`; commits 57f01449a, 2dd207e22, 585ceae61, ed0701a37, 3dfca471a. Counts in this manual: 2026-10-17.

## Wanted

Key works not held, or held under terms that forbid use here. Most are in copyright; the list says what to look for.

**Reference grammars and syntax:** Waltke & O'Connor, *An Introduction to Biblical Hebrew Syntax* (1990); Joüon & Muraoka, *A Grammar of Biblical Hebrew* (2nd ed. 2006); Van der Merwe, Naudé & Kroeze, *A Biblical Hebrew Reference Grammar* (2nd ed. 2017); Andersen, *The Hebrew Verbless Clause in the Pentateuch* (1970); Blau, *Phonology and Morphology of Biblical Hebrew* (2010); Fox, *Semitic Noun Patterns* (2003); Emerton 1977 on *hištaḥawāh*.

**Verb and discourse:** Niccacci, *The Syntax of the Verb in Classical Hebrew Prose* (1990); Longacre, *Joseph* (1989/2003); Miller, *The Representation of Speech in Biblical Hebrew Narrative* (1996); Talstra, "Text Grammar and Hebrew Bible" I–II; Cook, *Time and the Biblical Hebrew Verb* (2012); Runge & Westbury, *Lexham Discourse Hebrew Bible* (data).

**Accents and Masorah:** Price, *The Syntax of Masoretic Accents in the Hebrew Bible* (1990); Jacobson, *Chanting the Hebrew Bible* (2002/2017); Dresher 1994 and DeCaen & Dresher 2020 on the prosodic basis of the accents; Breuer's accent manuals; Yeivin, *Introduction to the Tiberian Masorah* (1980); Revell and Dotan on pointing and pausal forms; facsimiles of the Aleppo and Leningrad codices; a machine-readable ketiv list of the WLC.

**Phonology and history:** Blau, *On Polyphony in Biblical Hebrew* (1982); Steiner on Septuagint transcriptions; Ben-Ḥayyim, *A Grammar of Samaritan Hebrew* (2000); Kutscher, *A History of the Hebrew Language* (1982); Sáenz-Badillos, *A History of the Hebrew Language* (1993); Qimron, *The Hebrew of the Dead Sea Scrolls* (1986); Hurvitz 1982 and *A Concise Lexicon of Late Biblical Hebrew* (2014); Young, Rezetko & Ehrensvärd, *Linguistic Dating of Biblical Texts* (2008).

**Lexicons:** Koehler & Baumgartner, *HALOT*; Clines, *The Dictionary of Classical Hebrew*; de Blois, *Semantic Dictionary of Biblical Hebrew* (its domains are in MACULA but their licence excludes us); Lambdin 1953 on Egyptian loanwords; Muchiki 1999; Mankowski 2000 on Akkadian loanwords.

**Mishnaic Hebrew:** Pérez Fernández, *An Introductory Grammar of Rabbinic Hebrew* (1997); Bar-Asher, *Studies in Mishnaic Hebrew* (2009); Haneman, *A Morphology of Mishnaic Hebrew* (1980); an open, morphologically tagged Mishnah (Kaufmann); an open edition of the Bar Kokhba letters; a UD treebank of Mishnaic Hebrew.

**Data:** an open-licence Ancient Hebrew UD treebank (PTNK and ETCBC are NC); an open noun-and-suffix paradigm resource.
