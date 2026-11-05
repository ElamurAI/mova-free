# Sources

The works cited in this folder (the overview and Biblical Aramaic), with local paths where we have them and open URLs where they exist. Works also used by the other folders are listed here once; each other folder lists its own. Paths under `$RESEARCH_RAW/` are the Scientist's raw store (outside the repository, not published). Checked 2026-10-17; grammars on disk added 2026-10-18.

## 1. Texts

| work | edition | licence | where |
|---|---|---|---|
| Hebrew Bible, Aramaic chapters (Gen 31:47; Jer 10:11; Dan 2:4b–7:28; Ezra 4:8–6:18; 7:12–26) | Westminster Leningrad Codex (WLC) as "Tanach with Nikkud" on Sefaria | public domain | `$RESEARCH_RAW/scripture/1-originals/sefaria/json/Tanakh/Writings/Daniel/Hebrew/Tanach with Nikkud.json` (and Ezra, Jeremiah, Genesis); https://www.sefaria.org |
| same, with accents | "Tanach with Ta'amei Hamikra" | public domain | same folder |
| MACULA Hebrew Linguistic Datasets (WLC + OSHB morphology + Westminster Hebrew Syntax + glosses) | commit 47db250 (2026-04-24) | CC BY 4.0 (© Biblica, Inc.) | `$RESEARCH_RAW/lexres/treebanks/macula-hebrew/`; https://github.com/Clear-Bible/macula-hebrew |
| Open Scriptures Hebrew Bible (morphhb) | WLC with lemma and morphology, OSIS | WLC PD; morphology CC BY 4.0 | `$RESEARCH_RAW/scripture/1-originals/oshb-morphhb/`; https://github.com/openscriptures/morphhb |
| unfoldingWord Hebrew Bible | USFM 3 with lemma and morphology | CC BY-SA 4.0 (test only) | `$RESEARCH_RAW/scripture/1-originals/uw-uhb/` |
| the corpus registry | `texts.tsv` (file, edition, licence, `lang`, `mixed`) | ours (Research) | `$RESEARCH_RAW/scripture/texts.tsv`; guide: Research repository `docs/papers/scripture.md` |

## 2. Dictionaries

| work | licence | where |
|---|---|---|
| Brown, F., S. R. Driver and C. A. Briggs. 1906. *A Hebrew and English Lexicon of the Old Testament, with an Appendix Containing the Biblical Aramaic.* Oxford: Clarendon. Aramaic part as **BDB-Aram** | public domain | `$RESEARCH_RAW/lexres/dictionaries/sefaria/BDB_Aramaic_Dictionary.jsonl` (863 entries); scan: `$RESEARCH_RAW/lexres/dictionaries/internet-archive/bdb-1906/`; https://archive.org/details/hebrewenglishlex00browuoft |
| Jastrow, M. 1903. *A Dictionary of the Targumim, the Talmud Babli and Yerushalmi, and the Midrashic Literature.* London: Luzac; New York: Putnam | public domain | `$RESEARCH_RAW/lexres/dictionaries/sefaria/Jastrow_Dictionary.jsonl` (32 512 entries); scans: `$RESEARCH_RAW/lexres/dictionaries/internet-archive/jastrow-1903/`; https://archive.org/details/dictionaryoftarg01jastuoft |
| Gesenius, W. 1857. *Hebrew and Chaldee Lexicon to the Old Testament Scriptures*, transl. S. P. Tregelles | public domain | `$RESEARCH_RAW/lexres/dictionaries/internet-archive/gesenius-tregelles-1857/`; https://archive.org/details/hebrewchaldeelex00geseuoft |
| Levy, J. 1867–1868. *Chaldäisches Wörterbuch über die Targumim und einen grossen Theil des rabbinischen Schriftthums.* Leipzig | public domain | `$RESEARCH_RAW/lexres/dictionaries/internet-archive/levy-chald-1867/`; https://archive.org/details/chaldaischeswort00levy |
| Levy, J. 1876–1889. *Neuhebräisches und chaldäisches Wörterbuch über die Talmudim und Midraschim.* 4 vols. Leipzig | public domain | `$RESEARCH_RAW/lexres/dictionaries/internet-archive/levy-neuhebr-1876-1889/`; https://archive.org/details/b30093739_0001 |
| OpenScriptures Hebrew Lexicon (BDB XML, Strong's, lexical index) | CC BY 4.0 | `$RESEARCH_RAW/lexres/dictionaries/oshb-hebrewlexicon/` |
| Comprehensive Aramaic Lexicon (CAL), Hebrew Union College | copyrighted; consulted online only | https://cal.huc.edu/ |
| Vogt, E. 2011. *A Lexicon of Biblical Aramaic, Clarified by Ancient Documents.* Transl. J. A. Fitzmyer. Rome: Gregorian & Biblical Press | in copyright | wanted (§5) |
| Koehler, L., W. Baumgartner et al. 2000. *The Hebrew and Aramaic Lexicon of the Old Testament (HALOT)*, vol. 5: Aramaic. Leiden: Brill | in copyright | wanted |

## 3. Grammars and studies

**Public domain (or open access):**

- Kautzsch, E. 1884. *Grammatik des Biblisch-Aramäischen, mit einer kritischen Erörterung der aramäischen Wörter im Neuen Testament.* Leipzig: Vogel. Public domain (Kautzsch d. 1910); archive.org `grammatikdesbib00kaut`; on disk: `$RESEARCH_RAW/hebgram/pd/kautzsch-biblisch-aramaeisch-1884/` (PDF + OCR). Printed page = PDF page − 12.
- Marti, K. 1896 (2nd ed. 1911). *Kurzgefasste Grammatik der biblisch-aramäischen Sprache: Litteratur, Paradigmen, Texte und Glossar.* Berlin: Reuther. Public domain (Marti d. 1925); archive.org `kurzgefasstegra00martgoog`; on disk: `$RESEARCH_RAW/hebgram/pd/marti-biblisch-aramaeisch-1896/` (1st ed. 1896). Printed page = PDF page − 20 for the grammar; the appendix (paradigms, glossary) is paged with a star, n* = PDF page − 154. The paradigm tables are printed sideways. The glossary carries Iranian etymologies by F. C. Andreas. Used in the second check of 2026-10-18.
- Kautzsch, E. 1902. *Die Aramaismen im Alten Testament.* Halle. Public domain; archive.org `diearamaismenima01kautuoft`; on disk: `$RESEARCH_RAW/hebgram/pd/kautzsch-aramaismen-1902/`. Printed page = PDF page − 10.
- Luzzatto, S. D. 1865. *Elementi grammaticali del caldeo biblico e del dialetto talmudico babilonese*; German transl. M. S. Krüger 1873, *Grammatik der biblisch-chaldäischen Sprache und des Idioms des Thalmud Babli* (Breslau); English transl. J. S. Goldammer 1876, *Grammar of the Biblical Chaldaic Language and the Talmud Babli Idioms* (New York). Public domain.
- Strack, H. L. 1896 (6th ed. 1921). *Grammatik des Biblisch-Aramäischen.* Leipzig: Hinrichs. Public domain (Strack d. 1922). The 1901 edition (3rd) is on disk: `$RESEARCH_RAW/hebgram/pd/strack-biblisch-aramaeisch-1901/` (archive.org `grammatikdesbibl00stra`); its paradigm marks unattested forms with a degree sign, which makes it the main check for the tables here. Printed page = PDF page − 4.
- Margolis, M. L. 1910. *A Manual of the Aramaic Language of the Babylonian Talmud.* Munich: Beck. Public domain; archive.org `manualofaramaicl00marguoft`; on disk: `$RESEARCH_RAW/hebgram/pd/margolis-babylonian-aramaic-1910/` (the OCR text layer has no Hebrew script, so forms are read from the page images); German edition PDF open at https://opendata.uni-halle.de/handle/1981185920/99668 (Public Domain Mark).
- Dalman, G. 1905 (2nd ed.). *Grammatik des jüdisch-palästinischen Aramäisch.* Leipzig: Hinrichs. Public domain (Dalman d. 1941); on disk: `$RESEARCH_RAW/hebgram/pd/dalman-jpa-grammatik-1905/`.
- Bauer, H. and P. Leander. 1927. *Grammatik des Biblisch-Aramäischen.* Halle: Niemeyer. Public domain (Bauer d. 1937, Leander d. 1935); on disk: `$RESEARCH_RAW/hebgram/pd/bauer-leander-biblisch-aramaeisch-1927/` (archive.org `grammatikdesbibl0000hans`). Printed page = PDF page − 20. Its paradigm of the strong verb (§37, pp. 118–121) stars every form attested in no verb class, and §50 (pp. 170–177) lists every verb form of the Bible by person and stem; both were the main check of the second pass (2026-10-18).
- Krauss, S. 1898–1899. *Griechische und lateinische Lehnwörter im Talmud, Midrasch und Targum.* 2 vols. Berlin: Calvary. Public domain (Krauss d. 1948); archive.org `griechischeundla01krau` (both volumes in one scan); on disk: `$RESEARCH_RAW/hebgram/pd/krauss-lehnwoerter-1898-1899/`. Printed page = PDF page − 50 in vol. I (grammar) and − 412 in vol. II (dictionary). The OCR keeps part of the Hebrew and Greek; forms were read from the page images. Used for the Greek words of Daniel (2026-10-18).
- Kohut, A. 1878–1892. *Aruch Completum.* 8 vols. Vienna. Public domain; on disk: `$RESEARCH_RAW/hebgram/pd/kohut-aruch-completum-1878-1892/`. Not used for this folder: the Biblical Aramaic loanwords are all settled in the grammars above or stay open.
- Stevenson, W. B. 1924. *Grammar of Palestinian Jewish Aramaic.* Oxford: Clarendon. Public domain (Stevenson d. 1954); on disk: `$RESEARCH_RAW/hebgram/pd/stevenson-palestinian-aramaic-1924/`.

All the grammars above were scanned by the Internet Archive; their OCR layers render Hebrew script as Latin garbage, so every form cited from them in this folder was read from the page images (2026-10-18). The metadata and licences are in `$RESEARCH_RAW/hebgram/grammars.tsv`.
- Khan, G. 2020. *The Tiberian Pronunciation Tradition of Biblical Hebrew.* 2 vols. Cambridge: Open Book Publishers. Open access (licence as stated on the OBP page; read and cite, do not ship). Research library: `hbobook-obp0163-khan-tiberian-pronunciation-tradition-biblical-hebrew`.
- Khan, G. and P. M. Noorlander, eds. 2021. *Studies in the Grammar and Lexicon of Neo-Aramaic.* Cambridge: OBP. Open access. Research library: `hbobook-obp0209-khan-studies-grammar-lexicon-neo-aramaic`.
- Comrie, B., M. Haspelmath and B. Bickel. 2008. *The Leipzig Glossing Rules.* https://www.eva.mpg.de/lingua/resources/glossing-rules.php

**In copyright (cited from the literature and our own reading; not on disk):**

- Rosenthal, F. 1961 (7th ed. 2006). *A Grammar of Biblical Aramaic.* Wiesbaden: Harrassowitz.
- Johns, A. F. 1972. *A Short Grammar of Biblical Aramaic.* Berrien Springs: Andrews University Press.
- Greenspahn, F. E. 2003 (2nd ed.). *An Introduction to Aramaic.* Atlanta: SBL.
- Li, T. 2009. *The Verbal System of the Aramaic of Daniel: An Explanation in the Context of Grammaticalization.* Leiden: Brill.
- Cook, E. M. 2019. *Aramaic Ezra and Daniel: A Handbook on the Aramaic Text.* Waco: Baylor University Press.
- Folmer, M. L. 1995. *The Aramaic Language in the Achaemenid Period: A Study in Linguistic Variation.* Leuven: Peeters.
- Buth, R. 1987. *Word Order in Aramaic from the Perspectives of Functional Grammar and Discourse Analysis.* PhD diss., UCLA.
- Kaufman, S. A. 1974. *The Akkadian Influences on Aramaic.* Chicago: University of Chicago Press. (Assyriological Studies 19; the Oriental Institute publishes its older series as free PDFs: check https://isac.uchicago.edu/research/publications for this volume.)
- Kaufman, S. A. 1992. "Languages (Aramaic)." *Anchor Bible Dictionary* 4: 173–178.
- Hinz, W. 1975. *Altiranisches Sprachgut der Nebenüberlieferungen.* Wiesbaden: Harrassowitz.
- Fitzmyer, J. A. 1979. "The Phases of the Aramaic Language." In *A Wandering Aramean: Collected Aramaic Essays*, 57–84. Missoula: Scholars Press.
- Gzella, H. 2015. *A Cultural History of Aramaic: From the Beginnings to the Advent of Islam.* Leiden: Brill.
- Creason, S. 2004. "Aramaic." In R. D. Woodard, ed., *The Cambridge Encyclopedia of the World's Ancient Languages*, 391–426. Cambridge University Press.
- Huehnergard, J. 1995. "What is Aramaic?" *ARAM* 7: 261–282.
- Moscati, S., A. Spitaler, E. Ullendorff and W. von Soden. 1964. *An Introduction to the Comparative Grammar of the Semitic Languages.* Wiesbaden: Harrassowitz.
- Kutscher, E. Y. 1958. "The Language of the Genesis Apocryphon: A Preliminary Study." *Scripta Hierosolymitana* 4: 1–35.
- Kutscher, E. Y. 1970. "Aramaic." In T. A. Sebeok, ed., *Current Trends in Linguistics* 6: 347–412. The Hague: Mouton.
- Tal, A. 1975. *The Language of the Targum of the Former Prophets and its Position within the Aramaic Dialects* (Hebrew). Tel Aviv University.
- Wagner, M. 1966. *Die lexikalischen und grammatikalischen Aramaismen im alttestamentlichen Hebräisch.* Berlin: Töpelmann.
- Naveh, J. 1982. *Early History of the Alphabet.* Jerusalem: Magnes.
- Collins, J. J. 1993. *Daniel.* Hermeneia. Minneapolis: Fortress.
- Alexander, P. S. 1992. "Targum, Targumim." *Anchor Bible Dictionary* 6: 320–331.
- Flesher, P. V. M. and B. Chilton. 2011. *The Targums: A Critical Introduction.* Waco: Baylor.
- Morag, S. 1988. *Babylonian Aramaic: The Yemenite Tradition* (Hebrew). Jerusalem: Ben-Zvi Institute.
- Sokoloff, M. 2002. *A Dictionary of Jewish Babylonian Aramaic of the Talmudic and Geonic Periods.* Ramat-Gan: Bar-Ilan; and Sokoloff, M. 2002 (2nd ed.). *A Dictionary of Jewish Palestinian Aramaic of the Byzantine Period.* (3rd ed. 2017.)
- Bar-Asher Siegal, E. A. 2013. *Introduction to the Grammar of Jewish Babylonian Aramaic.* Münster: Ugarit-Verlag.
- Epstein, J. N. 1960. *A Grammar of Babylonian Aramaic* (Hebrew). Jerusalem: Magnes.
- Scholem, G. 1941. *Major Trends in Jewish Mysticism.* Jerusalem: Schocken (lecture 5 on the Zohar).
- Liebes, Y. 1976. *Peraqim be-millon sefer ha-Zohar* (Sections of the Zohar Lexicon). PhD diss., Hebrew University. Liebes, Y. 1993. *Studies in the Zohar.* Albany: SUNY Press.

## 4. Web pages consulted (2026-10-17)

- Wikipedia, "Jewish Babylonian Aramaic", https://en.wikipedia.org/wiki/Jewish_Babylonian_Aramaic (pronoun and verb tables; used as a cross-check only).
- SIL, ISO 639-3 code tables, https://iso639-3.sil.org/code_tables/639/data (codes `arc`, `tmr`, `jpa`, `oar`, `syc`, `myz`, `sam`).

## 5. Wanted

Key works not on disk, in order of use for Mova. Public-domain items can be fetched by the Scientist (they are his to download, on request); the others are for the purchase catalogue only.

| work | why | status |
|---|---|---|
| Kautzsch 1884, *Grammatik des Biblisch-Aramäischen* | the classic full grammar of BA, PD | PD, on archive.org — **to request** |
| Marti 1896/1911, *Kurzgefasste Grammatik der biblisch-aramäischen Sprache* (with paradigms and glossary) | paradigms and glossary, PD | PD — **to request** |
| Strack, *Grammatik des Biblisch-Aramäischen* (6th ed. 1921) | compact grammar with texts; the 1901 edition is on disk | PD — the later edition still **to request** |
| Luzzatto/Goldammer 1876 | BA and JBA in one book, in English, PD | PD — **to request** |
| Margolis 1910 (German edition PDF from Halle, better scan) | JBA paradigms; the English edition is on disk, forms read from page images | PD — optional |
| Kaufman 1974, *The Akkadian Influences on Aramaic* | the reference for Akkadian loans | check whether the Oriental Institute has made it a free PDF |
| Rosenthal 1961 / 2006 | the standard modern grammar of BA | in copyright — catalogue |
| Vogt 2011; HALOT vol. 5 | modern BA lexicons with Iranian and Akkadian etymologies | in copyright — catalogue |
| Cook 2019; Li 2009 | modern syntax and verb system of BA | in copyright — catalogue |
| Folmer 1995; Gzella 2015 | Imperial Aramaic background | in copyright — catalogue |
| a public-domain digital text of the Bavli (Vilna) and of the main Zohar (Mantua/Cremona or Vilna) with a clear licence | the CC BY-SA and unknown-licence editions on disk cannot be shipped | to look for (Wikisource he, Hebrewbooks scans, Al-HaTorah) |
| the Aramaic portions of the Dead Sea Scrolls (Genesis Apocryphon, 11QtgJob) | the link between BA and the targumic language | editions in copyright; images open (Leon Levy library) |

The Elamur team
