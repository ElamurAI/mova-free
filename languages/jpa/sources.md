# Sources

Works cited in this folder. General works on Aramaic, the dictionaries (Jastrow, BDB, Levy) and the code scheme: [../arc/sources.md](../arc/sources.md). Checked 2026-10-17; grammar paths added 2026-10-18.

## 1. Texts on disk

| text | edition | licence | path |
|---|---|---|---|
| Jerusalem Talmud | Guggenheimer, H. W., ed. and transl. 1999–2015. *The Jerusalem Talmud.* Berlin: De Gruyter. (Text with vocalisation, on Sefaria) | CC BY | `$RESEARCH_RAW/scripture/2-rabbinic/sefaria/json/Talmud/Yerushalmi/<Seder>/Jerusalem Talmud <Tractate>/Hebrew/The Jerusalem Talmud, edition by Heinrich W. Guggenheimer. …json` |
| Jerusalem Talmud | Venice 1523; Mechon-Mamre | unknown → private (not used) | `…/private/json/Talmud/Yerushalmi/…` |
| Pseudo-Jonathan | "Targum Jonathan on Genesis" … "Deuteronomy" | public domain | `$RESEARCH_RAW/scripture/1-originals/sefaria/json/Tanakh/Targum/Targum Jonathan/Torah/` |
| Fragment targum | "Targum Jerusalem on Torah" | public domain | `$RESEARCH_RAW/scripture/1-originals/sefaria/json/Tanakh/Targum/Targum Jerusalem/Targum Jerusalem/Hebrew/` |
| Targum Neofiti | "The Vatican Manuscript of the Targum Neofiti" | unknown → private (not used; 137 words) | `…/private/json/Tanakh/Targum/Targum Neofiti/` |

## 2. Dictionaries

- Jastrow 1903 — public domain; local JSONL; cites the Yerushalmi ("Y.") and the Palestinian targums ("Targ. Y.", "Targ. Y. II") throughout.
- Levy 1876–1889 — public domain; scans.
- Sokoloff, M. 2002b. *A Dictionary of Jewish Palestinian Aramaic of the Byzantine Period.* 2nd ed. Ramat-Gan: Bar-Ilan University Press; Baltimore: Johns Hopkins (3rd ed. 2017). In copyright; wanted.
- Krauss, S. 1898–1899. *Griechische und lateinische Lehnwörter im Talmud, Midrasch und Targum.* 2 vols. Berlin. Public domain. archive.org `griechischeundla01krau` (both volumes in one scan); on disk: `$RESEARCH_RAW/hebgram/pd/krauss-lehnwoerter-1898-1899/`. Vol. II (the dictionary) starts at PDF page 401; vol. II page = PDF page − 412. The loanword table of [lexicon.md](lexicon.md) §3 was checked against it on 2026-10-18.
- Sperber, D. 1984. *A Dictionary of Greek and Latin Legal Terms in Rabbinic Literature.* Ramat-Gan: Bar-Ilan University Press. In copyright.
- CAL, https://cal.huc.edu/ — has JPA texts lemmatised; consult only.

## 3. Grammars and studies

**Public domain:**

- Dalman, G. 1905. *Grammatik des jüdisch-palästinischen Aramäisch.* 2nd ed. Leipzig: Hinrichs. (Dalman d. 1941.) archive.org `grammatikdesj00dalmuoft`; on disk: `$RESEARCH_RAW/hebgram/pd/dalman-jpa-grammatik-1905/` (PDF + OCR). Printed page = PDF page − 22. Galilean forms are printed unpointed; the comparative tables of the *Anhang* (pp. 395–411) set them beside Onkelos.
- Frankel, Z. 1870. *Mevo ha-Yerushalmi* (Introduction to the Jerusalem Talmud, Hebrew). Breslau. Public domain.
- Stevenson, W. B. 1924. *Grammar of Palestinian Jewish Aramaic.* Oxford: Clarendon. Public domain in the EU and the USA (Stevenson d. 1954; published 1924), as recorded in `$RESEARCH_RAW/hebgram/grammars.tsv`; archive.org `grammarofpalesti0000wbst`; on disk: `$RESEARCH_RAW/hebgram/pd/stevenson-palestinian-aramaic-1924/`. Printed page = PDF page − 4. It sets PTM (Palestinian Talmud and Midrash) beside OJ (Onkelos and Jonathan) and Biblical Aramaic, with vocalised PTM forms. Both grammars were read from the page images (the OCR has no Hebrew script); this folder was checked against them on 2026-10-18. A second pass the same day read Dalman's phonology (§§ 10–15, pp. 56–105), adverbs and prepositions (§§ 43–47 and *Anhang* II–III, pp. 397–399), possessives (§18.4) and weak-first-root verbs (§§ 66–69, 73–74, pp. 291–311, 352–357).

**In copyright (cited from the literature):**

- Kutscher, E. Y. 1976. *Studies in Galilean Aramaic.* Ramat-Gan: Bar-Ilan University.
- Fassberg, S. E. 1991. *A Grammar of the Palestinian Targum Fragments from the Cairo Genizah.* Harvard Semitic Studies 38. Atlanta: Scholars Press.
- Golomb, D. M. 1985. *A Grammar of Targum Neofiti.* Harvard Semitic Monographs 34. Chico: Scholars Press.
- Cook, E. M. 1986. *Rewriting the Bible: The Text and Language of the Pseudo-Jonathan Targum.* PhD diss., UCLA.
- Díez Macho, A. 1968–1979. *Neophyti 1.* 6 vols. Madrid–Barcelona: CSIC. (The edition of Neofiti.)
- Klein, M. L. 1980. *The Fragment-Targums of the Pentateuch According to Their Extant Sources.* Rome: Biblical Institute Press; 1986. *Genizah Manuscripts of Palestinian Targum to the Pentateuch.* Cincinnati: HUC Press.
- Naveh, J. 1978. *On Stone and Mosaic: The Aramaic and Hebrew Inscriptions from Ancient Synagogues* (Hebrew). Jerusalem: Israel Exploration Society.
- Sussmann, Y. 1974. "A Halakhic Inscription from the Beth-Shean Valley" (Hebrew). *Tarbiz* 43: 88–158.
- Sokoloff, M. and J. Yahalom. 1999. *Jewish Palestinian Aramaic Poetry from Late Antiquity* (Hebrew). Jerusalem: Israel Academy.
- Odeberg, H. 1939. *The Aramaic Portions of Bereshit Rabba.* Lund: Gleerup. (Odeberg d. 1973.)

**Rabbinic sources cited:** y. Bava Metzia 2:5 (8c); y. Berakhot 2:8 (5c); y. Maaser Sheni 4:6; y. Kilayim 9:3; y. Sanhedrin 11:5; y. Yevamot 12:2; b. Eruvin 53b.

## 4. Wanted

| work | why | status |
|---|---|---|
| a public-domain text of Targum Neofiti | the only complete Palestinian targum | the manuscript (Vat. ebr. 440) is public; images may be open (Vatican Library); a PD transcription is to be looked for |
| Dalman 1905 (scan and OCR) | the PD grammar of JPA and the targums | on disk since 2026-10-17 |
| Stevenson 1924 | a short PD grammar in English | on disk since 2026-10-17 |
| Krauss 1898–1899 | Greek and Latin loanwords | on disk since 2026-10-18 |
| Frankel 1870 | the terms of the Yerushalmi | PD — **to request** |
| the Leiden manuscript of the Yerushalmi (Or. 4720) | the best complete witness | images published by Leiden University; check the terms |
| Sokoloff 2002b / 2017 | the dictionary | in copyright — catalogue |
| Fassberg 1991; Golomb 1985; Kutscher 1976 | grammars | in copyright — catalogue |
| Naveh 1978 | synagogue inscriptions | in copyright — catalogue |

The Elamur team
