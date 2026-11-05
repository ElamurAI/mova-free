# Sources

All the sources used in the Modern Hebrew manual, grouped by kind, with their licences and where they are. Each chapter has its own reference list with the exact pages and sections used; this page is the union, plus the list of key works we do not hold («wanted»).

[← README](README.md)

Paths under `$RESEARCH_RAW/` are the layout of the raw folder of the project's research library (outside the repo; the variables are defined in the root `README.md`). Licences: **PD** public domain; **CC BY**, **CC BY-SA**, **CC BY-NC(-SA)** Creative Commons; **AGPL** GNU Affero GPL. Under the project's rules (`licence/DATA.md`) NC data serves only tests and measurements, and CC BY-SA data is quoted, tested on and learned from but not carried or modified into a release.

---

## 1. Treebanks and annotation guidelines

| Source | Licence | Where | Used in |
|---|---|---|---|
| UD 2.18 UD_Hebrew-HTB (Ha'aretz 1990–91; Sima'an et al. 2001; Sade et al. 2018) | CC BY-NC-SA 4.0 | https://github.com/UniversalDependencies/UD_Hebrew-HTB — `$RESEARCH_RAW/lexres/treebanks/ud-2.18/nc/UD_Hebrew-HTB/` | 02, 11 (counts only, two short fragments described) |
| UD 2.18 UD_Hebrew-IAHLTwiki (Zeldes et al. 2022) | CC BY-SA 4.0 | https://github.com/UniversalDependencies/UD_Hebrew-IAHLTwiki — `$RESEARCH_RAW/lexres/treebanks/ud-2.18/UD_Hebrew-IAHLTwiki/` | 02, 04, 05, 06, 07, 11 |
| UD 2.18 UD_Hebrew-IAHLTknesset (Zeldes et al. 2022; Goldin et al. 2024) | CC BY-SA 4.0 | https://github.com/UniversalDependencies/UD_Hebrew-IAHLTknesset — same folder | 02, 04, 05, 06, 07, 09, 11 |
| UD 2.18 UD_Hebrew-PostRab (Tal et al. 2025) | CC BY-SA 4.0 | https://github.com/UniversalDependencies/UD_Hebrew-PostRab — same folder | 10, 11 |
| IAHLT revised HTB in the new tokenization | see repository | https://github.com/IAHLT/UD_Hebrew | 11 (described) |
| UD Hebrew documentation (language page; `compound:smixut`, `case:acc`, `case:gen`, `nsubj:cop`, `mark:q`, `compound:affix`, `HebBinyan`, `HebExistential`, `Prefix`) | CC BY-SA (conservative reading; repo Apache-2.0) | https://universaldependencies.org/he/ — snapshot `$RESEARCH_RAW/hebgram/sa/ud-docs-he-hbo/` (commit 3ae7161, 2026-10-17) | 04, 06, 07, 11 |
| Mova treebank cards for Hebrew | Elamur Cosmic | `treebanks/cards.tsv` in the Mova repo | 11, 12 |

## 2. Lexical and morphological resources

| Source | Licence | Where | Used in |
|---|---|---|---|
| Hspell 1.4 — Hebrew spell checker and morphological analyser (Nadav Har'El, Dan Kenigsberg) | AGPL-3.0 | http://hspell.ivrix.org.il/ — `$RESEARCH_RAW/lexres/morphology/sa/hspell/`; HebMorph with Hspell data `$RESEARCH_RAW/hebgram/sa/hebmorph/` | 02, 05, 12 |
| UniMorph Hebrew (`heb`, `heb_voc`, `from_UD_unvoc`; annotator Omer Goldman) | CC BY-SA 3.0 | https://github.com/unimorph/heb — `$RESEARCH_RAW/hebgram/sa/unimorph-heb/` | 05, 05b (paradigm checks), 12 |
| kaikki.org extract of English Wiktionary, Hebrew | CC BY-SA | https://kaikki.org/dictionary/Hebrew/ — `$RESEARCH_RAW/hebgram/sa/kaikki-enwiktionary-hebrew-aramaic/` | 04, 05, 06, 08 (etymology checks) |
| Wiktionary Hebrew and Aramaic appendices | CC BY-SA | `$RESEARCH_RAW/hebgram/sa/wiktionary-hebrew-aramaic-appendices/` | background |
| Dicta, Hebrew suffixed verbal forms dataset | CC BY 4.0 | https://huggingface.co/datasets/dicta-il/hebrew_suffix_verbal_forms — `$RESEARCH_RAW/lexres/morphology/dicta/hebrew_suffix_verbal_forms/` | 05, 07 |
| Dicta, unvocalized Hebrew wordlist | CC BY-NC 4.0 | https://huggingface.co/datasets/dicta-il/wordlist — `$RESEARCH_RAW/lexres/morphology/dicta/nc/wordlist/` | 12 (licence table) |
| Project Ben-Yehuda: Ben-Yehuda's introduction to the Great Dictionary | PD | https://github.com/projectbenyehuda/public_domain_dump — `$RESEARCH_RAW/lexres/dictionaries/ben-yehuda-pby/` | 08 |
| Klein, *A Comprehensive Etymological Dictionary of the Hebrew Language* (Carta 1987), Sefaria text | CC BY-NC (per Sefaria) | `$RESEARCH_RAW/lexres/dictionaries/sefaria/nc/` | 08 (reference only) |
| GF Resource Grammar Library, Hebrew | LGPL/BSD (GF RGL) | `gf-rgl/src/hebrew` in the parsers folder | 12 |

## 3. Grammars and open textbooks

| Source | Licence | Where | Used in |
|---|---|---|---|
| Hebrew Wikibooks, *דקדוק העברית החדשה* (Grammar of Modern Hebrew) | CC BY-SA | https://he.wikibooks.org/wiki/דקדוק_העברית_החדשה — `$RESEARCH_RAW/hebgram/sa/wikibooks-he-dikduk-haivrit-hahadasha/` | 02, 03, 04, 05 (consulted, not copied) |
| Hebrew Wikibooks, *לשון* | CC BY-SA | `$RESEARCH_RAW/hebgram/sa/wikibooks-he-lashon/` | background |
| English Wikibooks, *Hebrew* | CC BY-SA | `$RESEARCH_RAW/hebgram/sa/wikibooks-en-hebrew/` | background |
| Gesenius, Kautzsch and Cowley, *Gesenius' Hebrew Grammar* (1910) | PD | `$RESEARCH_RAW/hebgram/pd/gesenius-kautzsch-cowley-1910/` | 10 |
| Khan, Geoffrey. 2020. *The Tiberian Pronunciation Tradition of Biblical Hebrew* (OBP 0163) | open access (OBP; licence as stated in the book) | https://doi.org/10.11647/OBP.0163 — `$RESEARCH_RAW/hebgram/open/khan-tiberian-pronunciation-v1-2020/` | 03, 10 |
| Heijmans, Shai. 2020. *Studies in Rabbinic Hebrew* (OBP 0164) | open (OBP) | `$RESEARCH_RAW/hebgram/open/heijmans-studies-rabbinic-hebrew-2020/` | 10 |
| Segal, M. H. 1927. *A Grammar of Mishnaic Hebrew* | not PD in all jurisdictions: logs and citation only | `$RESEARCH_RAW/hebgram/private/segal-grammar-mishnaic-1927/` | 10 (cited only) |
| Kantor, Benjamin Paul. 2023. *The Standard Language Ideology of the Hebrew and Arabic Grammarians of the ʿAbbasid Period* (OBP 0382) | CC BY-NC 4.0 | `$RESEARCH_RAW/hebgram/nc/kantor-standard-language-ideology-2023/` | 10 (summarised) |
| Kim, Young Bok. 2023. *Hebrew Forms of Address: A Sociolinguistic Analysis* (SBL, open access) | open | `$RESEARCH_RAW/hbobook-2023-kim-hebrew-forms-address/` | 09 |

## 4. Papers held locally

Computational and corpus work (ACL Anthology, CC BY, unless noted):

- Adler, Meni and Michael Elhadad. 2006. An Unsupervised Morpheme-Based HMM for Hebrew Morphological Disambiguation. ACL. https://aclanthology.org/P06-1084/ — `$RESEARCH_RAW/aclbulk-2006-adler-unsupervised-morpheme-based-hmm-hebrew/`
- Adler, Meni, Yael Netzer, Yoav Goldberg, David Gabay and Michael Elhadad. 2008. Tagging a Hebrew Corpus: The Case of Participles. LREC. — `$RESEARCH_RAW/lingbulk-2008-adler-tagging-hebrew-corpus-case-participles/`
- Cohen, Shay B. and Noah A. Smith. 2007. Joint Morphological and Syntactic Disambiguation. EMNLP-CoNLL. https://aclanthology.org/D07-1022/ — `$RESEARCH_RAW/aclbulk-2007-cohen-joint-morphological-syntactic-disambiguation/`
- Dahan Netzer, Yael and Michael Elhadad. 1998. Generating Determiners and Quantifiers in Hebrew. — `$RESEARCH_RAW/aclbulk-1998-netzer-generating-determiners-quantifiers-hebrew/`
- Dahan Netzer, Yael and Michael Elhadad. 1999. Bilingual Hebrew-English Generation of Possessives and Partitives. — `$RESEARCH_RAW/aclbulk-1999-netzer-bilingual-hebrew-english-generation-possessives/`
- Finkel, Raphael and Gregory Stump. 2002. Generating Hebrew Verb Morphology by Default Inheritance Hierarchies. ACL-02 Workshop on Computational Approaches to Semitic Languages. — `$RESEARCH_RAW/onto-2002-finkel-generating-hebrew-verb-morphology-default/`
- Gershuni, Elazar and Yuval Pinter. 2022. Restoring Hebrew Diacritics Without a Dictionary. Findings of NAACL. — `$RESEARCH_RAW/findings-2022-gershuni-restoring-hebrew-diacritics-without-dictionary/`
- Goldberg, Yoav and Michael Elhadad. 2009. Hebrew Dependency Parsing: Initial Results. IWPT. https://aclanthology.org/W09-3819/ — `$RESEARCH_RAW/lingbulk-2009-goldberg-hebrew-dependency-parsing-initial-results/`
- Goldberg, Yoav and Michael Elhadad. 2011. Joint Hebrew Segmentation and Parsing using a PCFGLA Lattice Parser. ACL. — `$RESEARCH_RAW/aclbulk-2011-goldberg-joint-hebrew-segmentation-parsing-pcfgla/`
- Goldberg, Yoav and Reut Tsarfaty. 2008. A Single Generative Model for Joint Morphological Segmentation and Syntactic Parsing. ACL. — `$RESEARCH_RAW/aclbulk-2008-goldberg-single-generative-model-joint-morphological/`
- Goldin, Gili, Nick Howell, Noam Ordan, Ella Rabinovich and Shuly Wintner. 2024. The Knesset Corpus: An Annotated Corpus of Hebrew Parliamentary Proceedings. arXiv:2405.18115. — `$RESEARCH_RAW/arxivrecent-2024-goldin-knesset-corpus-annotated-corpus-hebrew/`
- Itai, Alon, Shuly Wintner and Shlomo Yona. 2006. A Computational Lexicon of Contemporary Hebrew. LREC. — `$RESEARCH_RAW/lingbulk-2006-itai-computational-lexicon-contemporary-hebrew/`
- Liebeskind, Chaya and Yaakov HaCohen-Kerner. 2016. A Lexical Resource of Hebrew Verb-Noun Multi-Word Expressions. LREC. — `$RESEARCH_RAW/lingbulk-2016-liebeskind-lexical-resource-hebrew-verb-noun/`
- Litvak, Marina, Natalia Vanetik, Chaya Liebeskind, Omar Hmdia and Rizek Abu Madeghem. 2022. Offensive language detection in Hebrew: can other languages help? LREC (CC BY-NC). — `$RESEARCH_RAW/lingbulk-2022-litvak-offensive-language-detection-hebrew-can/`
- Mor-Lan, Guy, Naama Rivlin-Angert, Yael R. Kaplan, Tamir Sheafer and Shaul R. Shenhav. 2025. HebID: Detecting Social Identities in Hebrew-language Political Text. Findings of ACL. — `$RESEARCH_RAW/findings-2025-lan-hebid-detecting-social-identities-hebrew/`
- Ornan, Uzzi. 2014. Innovation of Verbs in Hebrew. ACL workshop. — `$RESEARCH_RAW/aclbulk-2014-ornan-innovation-verbs-hebrew/`
- Seker, Amit, Elron Bandel, Dan Bareket, Idan Brusilovsky, Refael Shaked Greenfeld and Reut Tsarfaty. 2022. AlephBERT. ACL. https://aclanthology.org/2022.acl-long.4/ — `$RESEARCH_RAW/acl-2022-seker-alephbert-language-model-pre-training/`
- Shmidman, Avi, Shaltiel Shmidman, Moshe Koppel and Yoav Goldberg. 2020. Nakdan: Professional Hebrew Diacritizer. ACL demos. — `$RESEARCH_RAW/aclbulk-2020-shmidman-nakdan-professional-hebrew-diacritizer/`
- Shmidman, Shaltiel, Avi Shmidman and Moshe Koppel. 2023. DictaBERT: A State-of-the-Art BERT Suite for Modern Hebrew. arXiv:2308.16687. — `$RESEARCH_RAW/arxivrecent-2023-shmidman-dictabert-state-art-bert-suite/`
- Shmidman, Shaltiel et al. 2024. Adapting LLMs to Hebrew: Unveiling DictaLM 2.0. arXiv:2407.07080. — `$RESEARCH_RAW/arxivrecent-2024-shmidman-adapting-llms-hebrew-unveiling-dictalm/`
- Shmidman, Avi, Elda Weizman and Avishay Gerczuk. 2025. Irony Detection in Hebrew Documents: A Novel Dataset and an Evaluation of Neural Classification Methods. — `$RESEARCH_RAW/prag-2025-shmidman-irony-detection-hebrew-documents-novel/`
- Tsarfaty, Reut. 2006. Integrated Morphological and Syntactic Disambiguation for Modern Hebrew. COLING-ACL SRW. — `$RESEARCH_RAW/aclbulk-2006-tsarfaty-integrated-morphological-syntactic-disambiguation-modern/`
- Tsarfaty, Reut, Djamé Seddah, Sandra Kübler and Joakim Nivre. 2013. Parsing Morphologically Rich Languages: Introduction to the Special Issue. *Computational Linguistics* 39(1). — `$RESEARCH_RAW/cl-2013-tsarfaty-parsing-morphologically-rich-languages-introduction/`
- Tsarfaty, Reut, Shoval Sadde, Stav Klein and Amit Seker. 2019. What's Wrong with Hebrew NLP? And How to Make it Right. EMNLP-IJCNLP demos. https://aclanthology.org/D19-3044/ — `$RESEARCH_RAW/aclbulk-2019-tsarfaty-what-s-wrong-hebrew-nlp/`
- Vanetik, Natalia, Lior Liberov, Marina Litvak and Chaya Liebeskind. 2025. Towards Safer Hebrew Communication: A Dataset for Offensive Language Detoxification. — `$RESEARCH_RAW/aclbulk-2025-vanetik-safer-hebrew-communication-dataset-offensive/`
- Wintner, Shuly. 1998. Linguistically Motivated Computational Grammar for Hebrew. — `$RESEARCH_RAW/aclbulk-1998-wintner-linguistically-motivated-computational-grammar-hebrew/`
- Wintner, Shuly. 2003/2004. Resources for Processing Israeli Hebrew. — `$RESEARCH_RAW/lingbulk-2003-wintner-resources-processing-israeli-hebrew/`
- Yannai, Yig'al. 1982/1983. Innovation and Neologism in Modern Hebrew. *Translating and the Computer 4*. — `$RESEARCH_RAW/lingbulk-1982-yannai-innovation-neologism-modern-hebrew/`
- Zeldes, Amir. 2018. A Characterwise Windowed Approach to Hebrew Morphological Segmentation. SIGMORPHON. https://aclanthology.org/W18-5811/ — `$RESEARCH_RAW/aclbulk-2018-zeldes-characterwise-windowed-approach-hebrew-morphological/`
- Zeldes, Amir, Nick Howell, Noam Ordan and Yifat Ben Moshe. 2022. A Second Wave of UD Hebrew Treebanking and Cross-Domain Parsing. EMNLP, 4331–4344. https://aclanthology.org/2022.emnlp-main.292/ (fetched 2026-10-17; open access)
- Tal, Rachel, Shlomit Fuchs, Orly Albeck, Elisheva Brauner, Avi Shmidman, Yitzchak Lindenbaum and Ephraim Meiri. 2025. A New Hebrew Universal Dependency Treebank: The First Treebank of Post-Rabbinic Historical Hebrew. TLT. https://aclanthology.org/2025.tlt-1.11/ (abstract read online)

Theoretical linguistics (*Glossa*, CC BY 4.0):

- Bruening, Benjamin. 2022. Hebrew nominals do not require functional structure above the NP. *Glossa* 7(1). https://doi.org/10.16995/glossa.5763 — `$RESEARCH_RAW/lingbulk-2022-bruening-hebrew-nominals-do-not-require/`
- Fadlon, Julie, Julia Horvath, Tal Siloni and Kenneth Wexler. 2018. The acquisition of Hebrew idioms: stages, internal composition, and implications for storage. *Glossa* 3(1): 99. — `$RESEARCH_RAW/lingbulk-2018-fadlon-acquisition-hebrew-idioms-stages-internal/`
- Hacohen, Aviya, Olga Kagan and Dana Plaut. 2021. Differential object marking in Modern Hebrew: definiteness and partitivity. *Glossa* 6(1): 148. https://doi.org/10.16995/glossa.5729 — `$RESEARCH_RAW/lingbulk-2021-hacohen-differential-object-marking-modern-hebrew/`
- Kastner, Itamar. 2017. Reflexive verbs in Hebrew: deep unaccusativity meets lexical semantics. *Glossa* 2(1): 75. https://doi.org/10.5334/gjgl.299 — `$RESEARCH_RAW/lingbulk-2017-kastner-reflexive-verbs-hebrew-deep-unaccusativity/`
- Landau, Idan. 2018. Missing objects in Hebrew: argument ellipsis, not VP ellipsis. *Glossa* 3(1): 76. https://doi.org/10.5334/gjgl.560 — `$RESEARCH_RAW/lingbulk-2018-landau-missing-objects-hebrew-argument-ellipsis/`

## 5. Open web pages (consulted 2026-10-17; paraphrased, not copied)

- Academy of the Hebrew Language: decisions on spelling without points (2017), punctuation (2002/2017), transliteration (2006). https://hebrew-academy.org.il/topic/hahlatot/ — automatic fetching was refused (HTTP 403) on 2026-10-17; the rules were checked through secondary summaries and should be checked against the Academy's PDFs before quoting.
- Unicode Standard, Hebrew block U+0590–U+05FF, and UAX #9 (bidirectional algorithm). https://www.unicode.org/charts/PDF/U0590.pdf, https://www.unicode.org/reports/tr9/
- SBL Handbook of Style, 2nd ed. (2014), § 5.1 Hebrew transliteration.
- Wikipedia (English): Modern Hebrew; Modern Hebrew grammar; Modern Hebrew phonology; Modern Hebrew verb conjugation; Hebrew alphabet; Romanization of Hebrew; Geresh; Gershayim; Hebrew numerals; Dugri; Firgun; Revival of the Hebrew language; Eliezer Ben-Yehuda; Academy of the Hebrew Language. Hebrew Wikipedia: כתיב חסר ניקוד. (CC BY-SA: used for orientation and checking only.)

## 6. Cited from the literature but not held

These works are cited in the chapters from their standard descriptions (often through the held works above). They are the first candidates for the library:

**Reference grammars**

- Coffin, Edna Amir and Shmuel Bolozky. 2005. *A Reference Grammar of Modern Hebrew*. CUP.
- Glinert, Lewis. 1989. *The Grammar of Modern Hebrew*. CUP.
- Glinert, Lewis. 2005/2017. *Modern Hebrew: An Essential Grammar*. 3rd/4th ed. Routledge.
- Berman, Ruth A. 1978. *Modern Hebrew Structure*. University Publishing Projects.
- Schwarzwald, Ora (Rodrigue). 2001. *Modern Hebrew*. Lincom; and 2011, "Modern Hebrew", in Weninger et al. (eds.), *The Semitic Languages*. De Gruyter.
- Bolozky, Shmuel. 1996. *501 Hebrew Verbs*. Barron's.

**Phonology and phonetics**

- Laufer, Asher. 1999. Hebrew. *Handbook of the IPA*. CUP.
- Bolozky, Shmuel. 1997. Israeli Hebrew phonology. In A. S. Kaye (ed.), *Phonologies of Asia and Africa*. Eisenbrauns.
- Asherov, Daniel and Evan-Gary Cohen. 2019. A phonetic description of Modern Hebrew consonants and vowels. *Brill's Annual of Afroasiatic Languages and Linguistics* 11.
- Gafter, Roey J. 2014 (thesis), 2016 (*Journal of Sociolinguistics*), 2021 (*Sociolinguistic Studies*): pharyngeals and trilled r as sociolinguistic variables.

**Morphology and word formation**

- Bolozky, Shmuel. 1999. *Measuring Productivity in Word Formation: The Case of Israeli Hebrew*. Brill.
- Ravid, Dorit. 1995. *Language Change in Child and Adult Hebrew*. OUP.
- Bat-El, Outi. 1994. Stem modification and cluster transfer in Modern Hebrew. *NLLT* 12.
- Doron, Edit. 2003. Agency and voice: the semantics of the Semitic templates. *Natural Language Semantics* 11.
- Wigderson, Shira. 2015. The sudden disappearance of nitpael and the rise of hitpael in Modern Hebrew. *Journal of Jewish Languages* 3.
- Schwarzwald, Ora. 2013. Infinitive: Modern Hebrew. *Encyclopedia of Hebrew Language and Linguistics*. Brill.

**Syntax and semantics**

- Borer, Hagit. 1984. Restrictive relatives in Modern Hebrew. *NLLT* 2. — 1989. Anaphoric AGR.
- Borer, Hagit and Yosef Grodzinsky. 1986. Syntactic cliticization and lexical cliticization: the case of Hebrew dative clitics.
- Danon, Gabi. 2001, 2006, 2008 (definiteness, caseless nominals, definiteness spreading in the construct).
- Doron, Edit. 1982 (resumptive pronouns), 1983 (*Verbless Predicates in Hebrew*), 2000 (VSO and left-conjunct agreement).
- Berman, Ruth A. and Alexander Grosu. 1976. Aspects of the copula in Modern Hebrew.
- Ritter, Elizabeth. 1991. Two functional categories in noun phrases: evidence from Modern Hebrew.
- Shlonsky, Ur. 1987, 1992, 1997 (*Clause Structure and Word Order in Hebrew and Arabic*), 2004, 2009 (partial null subjects).
- Landau, Idan. 1999. Possessor raising and the structure of VP. *Lingua* 107.
- Reinhart, Tanya and Tal Siloni. 2005. The lexicon-syntax parameter. *LI* 36.
- Goldberg, Lotus. 2005. *Verb-Stranding VP Ellipsis*. PhD, McGill.
- Ziv, Yael. 1976 (possessive constructions), 1994 (dislocations).
- Wintner, Shuly. 2000. Definiteness in the Hebrew noun phrase. *Journal of Linguistics* 36.
- Ariel, Mira. 1990. *Accessing Noun-Phrase Antecedents*. Routledge.

**Pragmatics and discourse**

- Katriel, Tamar. 1985 (griping), 1986 (*Talking Straight: Dugri Speech in Israeli Sabra Culture*, CUP), 1993 (*lefargen*), 2004 (*Dialogic Moments*).
- Blum-Kulka, Shoshana. 1982, 1987; Blum-Kulka and Olshtain 1984; Blum-Kulka, House and Kasper (eds.) 1989 (CCSARP); Blum-Kulka 1997 (*Dinner Talk*).
- Maschler, Yael. 2009. *Metalanguage in Interaction: Hebrew Discourse Markers*. Benjamins.
- Ariel, Mira. 1998. Discourse markers and form-function correlations (*harey*).
- Roniger, Luis and Michael Feige. 1992. From pioneer to freier.
- Dekel, Nurit. 2014. *Colloquial Israeli Hebrew: A Corpus-based Survey*. De Gruyter Mouton.

**NLP**

- Sima'an, Khalil, Alon Itai, Yoad Winter, Alon Altman and Noa Nativ. 2001. Building a Tree-Bank of Modern Hebrew Text. *TAL* 42(2).
- Sade, Shoval, Amit Seker and Reut Tsarfaty. 2018. The Hebrew Universal Dependency Treebank: Past, Present and Future. UDW. https://aclanthology.org/W18-6016/
- More, Amir, Amit Seker, Victoria Basmova and Reut Tsarfaty. 2019. Joint Transition-Based Models for Morpho-Syntactic Parsing. *TACL* 7. https://aclanthology.org/Q19-1003/
- Itai, Alon and Shuly Wintner. 2008. Language Resources for Hebrew. *LRE* 42.
- Tsarfaty, Reut. 2013. A Unified Morpho-Syntactic Scheme of Stanford Dependencies. ACL. https://aclanthology.org/P13-2103/

**History of the language and the revival** (chapters 01, 08, 10)

- Kutscher, Eduard Yechezkel. 1982. *A History of the Hebrew Language*. Magnes / Brill.
- Sáenz-Badillos, Angel. 1993. *A History of the Hebrew Language*. Trans. J. Elwolde. CUP.
- Fellman, Jack. 1973. *The Revival of a Classical Tongue: Eliezer Ben Yehuda and the Modern Hebrew Language*. Mouton.
- Harshav, Benjamin. 1993. *Language in Time of Revolution*. University of California Press.
- Bachi, Roberto. 1956. A statistical analysis of the revival of Hebrew in Israel. *Scripta Hierosolymitana* 3.
- Zuckermann, Ghil'ad. 2003. *Language Contact and Lexical Enrichment in Israeli Hebrew*. Palgrave; 2020. *Revivalistics*. OUP.
- Wexler, Paul. 1990. *The Schizoid Nature of Modern Hebrew*. Harrassowitz.
- Izre'el, Shlomo. 2003. The emergence of spoken Israeli Hebrew. In B. Hary (ed.), *Corpus Linguistics and Modern Hebrew*.
- Rosén, Haiim B. 1977. *Contemporary Hebrew*. Mouton.
- Blanc, Haim. 1964. Israeli Hebrew texts.
- Huehnergard, John and Aaron D. Rubin. 2011. Phyla and waves: models of classification of the Semitic languages; Rubin, Aaron D. 2010. *A Brief Introduction to the Semitic Languages*. Gorgias.
- Cook, John A. 2012. *Time and the Biblical Hebrew Verb*. Eisenbrauns.
- Rosenthal, Ruvik. 2005. *The Comprehensive Slang Dictionary*. Keter; Ben-Amotz, Dahn and Netiva Ben-Yehuda. 1972/1982. *World Dictionary of Hebrew Slang*.
- Henkin, Roni. Articles on Arabic loanwords in Hebrew slang; Naiditch, Larissa. 2000. On Hebrew in the speech of Russian immigrants.

**Held locally for chapters 01, 08, 10** (in addition to § 3–4)

- Ben-Yehuda, Eliezer. Introduction (*ha-Mavo ha-Gadol*) to the *Complete Dictionary of Ancient and Modern Hebrew*, 1908–1959. PD. https://benyehuda.org — `$RESEARCH_RAW/lexres/dictionaries/ben-yehuda-pby/txt/p22/m7327.txt`
- Driver, S. R. 1892. *A Treatise on the Use of the Tenses in Hebrew*. PD. — `$RESEARCH_RAW/hebgram/pd/driver-tenses-1892/`
- Andrason, Alexander. 2011. Biblical Hebrew wayyiqtol: a dynamic definition. *JHS* 11. — `$RESEARCH_RAW/hbo-2011-andrason-biblical-hebrew-wayyiqtol-dynamic-definition/`
- Holmstedt, Robert D. 2011. The typological classification of the Hebrew of Genesis. *JHS* 11. — `$RESEARCH_RAW/hbo-2011-holmstedt-typological-classification-hebrew-genesis-subject/`
- Kaajan, Marianne. 2019. Is Biblical Hebrew a non-configurational language? *HIPHIL Novum* 5(2). — `$RESEARCH_RAW/hbo-2019-kaajan-biblical-hebrew-non-configurational-language/`
- Rezetko, Robert and Ian Young. 2014. *Historical Linguistics and Biblical Hebrew*. SBL. — `$RESEARCH_RAW/hbobook-2014-rezetko-historical-linguistics-biblical-hebrew/`
- Hornkohl, Aaron D. 2024. *Diachronic Diversity in Classical Biblical Hebrew*. OBP. — `$RESEARCH_RAW/hbobook-obp0433-hornkohl-diachronic-diversity-classical-biblical-hebrew/`
- Isaksson, Bo. 2021. *The Verb in Classical Hebrew*. OBP. — `$RESEARCH_RAW/hbobook-obp0414-isaksson-verb-classical-hebrew/`
- Khan, Geoffrey and Aaron D. Hornkohl (eds.). 2020. *Studies in Semitic Vocalisation and Reading Traditions*. OBP. — `$RESEARCH_RAW/hebgram/open/khan-hornkohl-semitic-vocalisation-2020/`
- Martínez Delgado, José. 2023. *Introduction to Andalusi Hebrew Metrics*. OBP. — `$RESEARCH_RAW/hbobook-obp0351-delgado-introduction-andalusi-hebrew-metrics/`
- Tal, Rachel, Cheyn Shmuel Shmidman and Avi Shmidman. 2026. Modeling the Dalet Clitic in Historical Hebrew Texts. — `$RESEARCH_RAW/aclbulk-2026-tal-modeling-dalet-clitic-historical-hebrew/`
- Klein, Ernest. 1987. *A Comprehensive Etymological Dictionary of the Hebrew Language*. Carta; Sefaria text CC BY-NC — `$RESEARCH_RAW/lexres/dictionaries/sefaria/nc/` (reference only).


## 7. Wanted

Key works for Modern Hebrew that the library lacks, in order of use to this manual. Open-access items can be fetched; the rest are for a purchase list or a library visit (via the project's knowledge keeper, never from shadow libraries).

1. **A modern reference grammar**: Coffin and Bolozky 2005; Glinert 1989 and 2017. Nearly every chapter leans on facts they state.
2. **The Academy of the Hebrew Language's official texts** (PDFs): full-spelling rules 2017, transliteration rules 2006, punctuation rules, verb tables (*luḥot ha-pó'al*), decisions on gender in language. Public, but the site refused automatic fetching (HTTP 403, 2026-10-17): fetch by hand.
3. **Spoken Hebrew**: Dekel 2014; Maschler 2009; Ravid 1995 (spoken levelling of verb forms); a corpus of spoken Israeli Hebrew (e.g. CoSIH / the Haifa corpus of spoken Hebrew) with its documentation.
4. **Phonetics**: Laufer 1999; Asherov and Cohen 2019 (open?); Bolozky 1997; Gafter's papers; a corpus-based study of Hebrew intonation.
5. **Pragmatics**: Katriel 1986; Blum-Kulka et al. 1989; Blum-Kulka 1997.
6. **Syntax classics**: Doron 1983; Shlonsky 1997; Borer 1984; Danon 2001 and 2008; Berman 1978.
7. **Word formation and the lexicon**: Bolozky 1999; Bat-El 1994; Doron 2003; the Academy's word-lists database (public, online); Rosenthal's *Dictionary of Israeli Slang* (Keter 2005, not open).
8. **NLP**: Sade et al. 2018; More et al. 2019 (TACL, open); Sima'an et al. 2001; the IAHLT revised HTB; Stanza / Trankit / HebPipe Hebrew models for witnesses (licences to check).
9. **History of the language and the revival**: Kutscher 1982; Sáenz-Badillos 1993; Fellman 1973; Harshav 1993; Zuckermann 2003 and 2020; Ben-Yehuda's dictionary volumes themselves (only the introduction is held); Even-Shoshan's dictionary.
11. **Statistics of speakers**: the Israel Central Bureau of Statistics language survey (the speaker numbers in 01 come through Wikipedia only).
10. **Hebrew of other communities**: Henkin's work on Arabic speakers' Hebrew and on Negev Bedouin Hebrew; Haredi Hebrew; immigrant Hebrew (Russian, Amharic).
