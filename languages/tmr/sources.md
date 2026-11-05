# Sources

Works cited in this folder. The general works on Aramaic, the dictionaries (Jastrow, BDB, Levy) and the code scheme are in [../arc/sources.md](../arc/sources.md). Checked 2026-10-17; grammar path added 2026-10-18.

## 1. Texts on disk

| text | edition | licence | path (under `$RESEARCH_RAW/scripture/2-rabbinic/sefaria/`) |
|---|---|---|---|
| Babylonian Talmud | Wikisource Talmud Bavli | CC BY-SA (measurement; short quotations with attribution) | `json/Talmud/Bavli/<Seder>/<Tractate>/Hebrew/Wikisource Talmud Bavli.json` |
| Babylonian Talmud | William Davidson Edition – Aramaic; – Vocalized Aramaic | CC BY-NC (tests only; not quoted) | `nc/json/Talmud/Bavli/...` |
| Ein Yaakov | Daat (Hebrew/Aramaic); Glick 1916–1921 (English) | public domain | `json/Midrash/Aggadah/Ein Yaakov/...` |
| Rodkinson's English Talmud | Wikisource transcription (incomplete) | public domain | see `SOURCES.md` in the scripture store |

## 2. Dictionaries

- Jastrow 1903 (public domain; JSONL with 32 512 entries, scans) — the main JBA dictionary available to us. See [../arc/sources.md](../arc/sources.md).
- Levy, J. 1876–1889. *Neuhebräisches und chaldäisches Wörterbuch über die Talmudim und Midraschim.* Public domain; scans in `$RESEARCH_RAW/lexres/dictionaries/internet-archive/levy-neuhebr-1876-1889/`.
- Kohut, A. 1878–1892. *Aruch Completum* (ʿArukh ha-shalem). 8 vols. Public domain. Wanted.
- Sokoloff, M. 2002a. *A Dictionary of Jewish Babylonian Aramaic of the Talmudic and Geonic Periods.* Ramat-Gan: Bar-Ilan University Press; Baltimore: Johns Hopkins. In copyright. Wanted.
- Frank, Y. 2001. *The Practical Talmud Dictionary.* Jerusalem: Ariel. In copyright.
- Melamed, E. Z. 1992. *Aramaic–Hebrew–English Dictionary of the Babylonian Talmud.* Jerusalem: Feldheim. In copyright.
- CAL, https://cal.huc.edu/ — JBA corpus and lexicon; consult only.

## 3. Grammars and studies

**Public domain (on archive.org or elsewhere; on disk where a path is given):**

- Margolis, M. L. 1910. *A Manual of the Aramaic Language of the Babylonian Talmud: Grammar, Chrestomathy and Glossaries.* Munich: Beck. archive.org `manualofaramaicl00marguoft`; on disk: `$RESEARCH_RAW/hebgram/pd/margolis-babylonian-aramaic-1910/` (PDF + OCR; the OCR layer has no Hebrew script, so forms were read from the page images; printed page = PDF page − 22). This folder was checked against its tables on 2026-10-18. Margolis prints JBA unvocalised and marks archaic forms with †; German edition *Lehrbuch der aramäischen Sprache des babylonischen Talmuds* (1910), PDF with Public Domain Mark at https://opendata.uni-halle.de/handle/1981185920/99668.
- Levias, C. 1900. *A Grammar of the Aramaic Idiom Contained in the Babylonian Talmud.* Cincinnati: Bloch. Public domain. archive.org `cu31924068095417`; on disk: `$RESEARCH_RAW/hebgram/pd/levias-babylonian-aramaic-1900/` (PDF + OCR; printed page = PDF page − 10). Levias points his forms; he says the pointing is an aid to exposition, not a restored original (preface, p. vi). Used for the vowels in the second check of 2026-10-18.
- Luzzatto, S. D. 1873 (German, transl. Krüger) / 1876 (English, transl. Goldammer). *Grammar of the Biblical Chaldaic Language and the Talmud Babli Idioms.* Public domain.
- Montgomery, J. A. 1913. *Aramaic Incantation Texts from Nippur.* Philadelphia: University Museum. Public domain. archive.org `aramaicincantati00montuoft`; on disk: `$RESEARCH_RAW/hebgram/pd/montgomery-aramaic-incantation-1913/` (PDF + OCR; printed page = PDF page − 4). The bowls are unpointed; the sketch of their language is on pp. 29–31 and the list of pronominal forms on p. 306 (end of Glossary C). Used in the second check of 2026-10-18.
- Krauss, S. 1898–1899. *Griechische und lateinische Lehnwörter im Talmud, Midrasch und Targum.* 2 vols. Berlin: Calvary. Public domain. On disk: `$RESEARCH_RAW/hebgram/pd/krauss-lehnwoerter-1898-1899/` (one scan with both volumes; vol. II, the dictionary, has printed page = PDF page − 412). Used for the Greek and Latin loanwords on 2026-10-18.
- Kohut, A. 1878–1892. *Aruch Completum* (ערוך השלם). 8 vols. Vienna. Public domain. On disk: `$RESEARCH_RAW/hebgram/pd/kohut-aruch-completum-1878-1892/` (one PDF per volume; its OCR keeps the Hebrew script; cited by volume and PDF page). Used for the Persian and Latin etymologies on 2026-10-18.

**In copyright (cited from the literature):**

- Epstein, J. N. 1960. *A Grammar of Babylonian Aramaic* (Hebrew), ed. E. Z. Melamed. Jerusalem: Magnes.
- Bar-Asher Siegal, E. A. 2013. *Introduction to the Grammar of Jewish Babylonian Aramaic.* Lehrbücher orientalischer Sprachen III/3. Münster: Ugarit-Verlag. (Whether it is available open access should be checked.)
- Morgenstern, M. 2011. *Studies in Jewish Babylonian Aramaic Based upon Early Eastern Manuscripts.* Harvard Semitic Studies 62. Winona Lake: Eisenbrauns.
- Morag, S. 1988. *Babylonian Aramaic: The Yemenite Tradition* (Hebrew). Jerusalem: Ben-Zvi Institute.
- Kutscher, E. Y. 1971. "Aramaic." *Encyclopaedia Judaica* 3: 259–287.
- Kutscher, E. Y. 1982. *A History of the Hebrew Language.* Jerusalem: Magnes; Leiden: Brill.
- Juusola, H. 1999. *Linguistic Peculiarities in the Aramaic Magic Bowl Texts.* Studia Orientalia 86. Helsinki.
- Naveh, J. and S. Shaked. 1985. *Amulets and Magic Bowls.* Jerusalem: Magnes; 1993. *Magic Spells and Formulae.* Jerusalem: Magnes.
- Shaked, S., J. N. Ford and S. Bhayro. 2013. *Aramaic Bowl Spells: Jewish Babylonian Aramaic Bowls*, vol. 1. Leiden: Brill.
- Shaked, S. Articles on Iranian loanwords in Jewish Babylonian Aramaic (see the bibliography of Sokoloff 2002a); cited here as "Shaked" without a year.
- Kaufman, S. A. 1974. *The Akkadian Influences on Aramaic.* Chicago: University of Chicago Press.
- Steinsaltz, A. 1989. *The Talmud: The Steinsaltz Edition. A Reference Guide.* New York: Random House.

**Web pages consulted (2026-10-17):** Wikipedia, "Jewish Babylonian Aramaic", https://en.wikipedia.org/wiki/Jewish_Babylonian_Aramaic (pronoun and verb tables, used as a cross-check).

**Rabbinic sources cited:** b. Berakhot 2a; b. Shabbat 31a; b. Eruvin 53b; b. Taanit 23a; b. Kiddushin 49a; b. Bava Metzia 2a, 59b; m. Avot 4:11.

## 4. Wanted

| work | why | status |
|---|---|---|
| a public-domain digital text of the Bavli (Vilna) | the open editions on disk are CC BY-SA and CC BY-NC; nothing can be built and shipped from them | to look for (he.wikisource's own licence is CC BY-SA too; old PD digitisations; scans of the Vilna print + our own OCR) |
| Margolis 1910 (Halle PDF of the German edition, PD) | a better scan of the same paradigms | optional — the English edition is on disk |
| Levias 1900 | the first full grammar of the Bavli's Aramaic in English | PD — **to request** |
| Luzzatto/Goldammer 1876 | | PD — **to request** |
| Montgomery 1913 | incantation bowls, PD | PD — **to request** |
| Krauss 1898–1899 | Greek and Latin loanwords | PD — **to request** |
| Kohut, *Aruch Completum* | medieval lexicon with etymologies | PD — **to request** |
| Sokoloff 2002a | the modern dictionary of JBA | in copyright — catalogue |
| Bar-Asher Siegal 2013 | the modern grammar of JBA in English | check for open access; else catalogue |
| Epstein 1960; Morgenstern 2011; Morag 1988 | grammar and manuscripts | in copyright — catalogue |

The Elamur team
