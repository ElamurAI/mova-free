# Aramaic in Mova: what it would need

**Status (2026-10-17): Mova has no Aramaic data parsed.** There is no Aramaic treebank in `treebanks/` except the tiny Neo-Aramaic `aii` (UD_Assyrian-AS, 57 sentences, 453 tokens, CC BY-SA 4.0), which is a modern language and not one of ours. The Hebrew side is further along: `tools/macula` converts the MACULA trees of the Torah into `hbo` dependency trees (125 807 words, 2026-10-17; [tools/macula/README.md](../../tools/macula/README.md)).

This file is the plan for all five varieties. Each variety's own `mova.md` adds what is specific to it.

## 1. The codes

`arc`, `arc-x-targum`, `tmr`, `jpa`, `arc-x-zohar` — fixed in the corpus registry `texts.tsv` (column `lang`, with other languages of a mixed file in `mixed`) and explained in [README.md](README.md). In CoNLL-U the code goes in the document's `# lang =` line, or per sentence when a document mixes languages (`# lang = tmr` / `# lang = hbo-x-mishnaic`), and per word in MISC (`Lang=hbo`) for code-switches inside a sentence.

## 2. What exists on disk

All under the Scientist's raw store, `$RESEARCH_RAW/` (outside the repository). Counts of whitespace-separated words were made on 2026-10-17 by flattening the Sefaria JSON text arrays with `jq` and stripping HTML tags; they include the Hebrew parts of mixed texts.

| variety | edition | licence (texts.tsv) | use | files | words |
|---|---|---|---|---:|---:|
| `arc` | Daniel, Ezra in WLC (Sefaria "Tanach with Nikkud") | public domain | build and test | 2 books | 7 549 Aramaic morph units (MACULA) |
| `arc` | MACULA Hebrew, WLC with OSHB morphology and syntax trees, `lang` = A | CC BY 4.0 | build and test (with attribution) | 1 TSV + lowfat XML | 7 549 units, 1 068 verbs |
| `arc-x-targum` | Onkelos (Sefaria, "Onkelos Genesis" etc.) | public domain | build and test | 5 | 82 914 |
| `arc-x-targum` | Onkelos, Yemenite *Taj* vocalisation | CC BY-SA | as little as possible: test, learn; not shipped | 5 | 80 946 |
| `arc-x-targum` | Onkelos, Metsudah *Sifsei Chachomim* | CC BY-NC | tests only | 5 | — |
| `arc-x-targum` | Targum Jonathan to the Prophets (Mikraot Gedolot) | public domain | build and test | 21 | 157 682 |
| `arc-x-targum` | targums to the Writings and Chronicles | public domain | build and test | 11 | 104 183 |
| `jpa` | Pseudo-Jonathan (5 books) and the Fragment ("Jerusalem") targum | public domain | build and test | 6 | 122 656 |
| `jpa` | Jerusalem Talmud, Guggenheimer edition (text) | CC BY | build and test (with attribution) | 39 | 811 526 |
| `jpa` | Jerusalem Talmud, Venice and Mechon-Mamre | unknown → private | not used | | |
| `jpa` | Targum Neofiti (Vatican MS) | unknown → private | not used (and the file holds only a fragment) | 1 | 137 |
| `tmr` | Babylonian Talmud, Wikisource edition | CC BY-SA | as little as possible: test, learn; not shipped | 37 | 1 858 059 |
| `tmr` | Babylonian Talmud, William Davidson (Steinsaltz) Aramaic | CC BY-NC | tests only | 37 | 1 854 708 |
| `arc-x-zohar` | Zohar Ḥadash; Tiqqunei ha-Zohar (vocalised) | public domain | build and test | 2 | 362 315 |
| `arc-x-zohar` | the main Zohar, vocalised (Israel 2013), Sulam edition | unknown → private | not used | 1+ | 903 606 (vocalised) |

**Dictionaries** (`$RESEARCH_RAW/lexres/dictionaries/`):

| dictionary | covers | licence | form |
|---|---|---|---|
| Jastrow 1903, *A Dictionary of the Targumim, the Talmud Babli and Yerushalmi, and the Midrashic Literature* | targum, JPA, JBA, Mishnaic Hebrew | public domain | Sefaria JSONL, 32 512 entries; scans and OCR on archive.org copy |
| BDB 1906, Aramaic part | Biblical Aramaic | public domain | Sefaria JSONL, 863 entries |
| Levy 1867–1868, *Chaldäisches Wörterbuch über die Targumim* | targums | public domain | scan + OCR (German) |
| Levy 1876–1889, *Neuhebräisches und chaldäisches Wörterbuch über die Talmudim und Midraschim* | JPA, JBA, Hebrew | public domain | scan + OCR (German), 4 vols. |
| Gesenius–Tregelles 1857, *Hebrew and Chaldee Lexicon* | Biblical Hebrew and Aramaic | public domain | scan + OCR |
| OSHB Hebrew lexicon (BDB XML, Strong's) | Biblical Hebrew and Aramaic | CC BY 4.0 | XML |
| CAL (Comprehensive Aramaic Lexicon), https://cal.huc.edu/ | all Aramaic | copyrighted; no download terms | link only, consult by hand |

## 3. Tokenisation

Aramaic words carry clitics on both sides, like Hebrew, and the texts come in three spellings (pointed, unpointed, plene). What the tokenizer must do:

1. **Proclitics.** ו "and", ד "of, which", ל "to; object", ב "in", כ "like", מ "from" (מִן reduced), and in JBA ק (קָא) and the demonstrative ה. They stack: וּדְלָא "and which not", דְּבֵי "of the house of", וְכִדְנָה. In MACULA each is a word of its own; Mova's `macula` already follows that ([../../tools/macula/README.md](../../tools/macula/README.md)). In BA the relative is still a separate word (דִּי); from the targums on it is the prefix ד־, and **every** word starting with ד is ambiguous (דְּהַב "gold" vs ד־ + הַב). A dictionary check against Jastrow headwords is the cheap first filter.
2. **Enclitics.** Pronoun suffixes on nouns, prepositions and verbs; in JBA also enclitic subject pronouns on participles (קָאָמֵינָא = קָא + אָמֵר + אֲנָא "I am saying") and the copula ניהו. These should be split into syntactic words, as UD does for Arabic and Hebrew.
3. **The determined ending ־א / ־ה** is **not** split: it is inflection (a feature, `Definite=Def` in UD terms), not a word. MACULA splits it as an article (`Td`); the converter should fold it back into the noun and set the feature. This is the one place where MACULA's segmentation should not be copied for Aramaic.
4. **Ketiv/Qere.** The Qere goes into the tree; the Ketiv into MISC (`Ketiv=…`).
5. **Abbreviations** (JBA and the Zohar): ת"ש = תָּא שְׁמַע, ת"ח = תָּא חֲזֵי, א"ר = אָמַר רַבִּי, ר"ש = רַבִּי שִׁמְעוֹן, קב"ה = קוּדְשָׁא בְּרִיךְ הוּא. A table of abbreviations per variety expands them before parsing; the abbreviated form stays in MISC.
6. **Pointing.** Strip points and accents for matching, keep them in FORM. The order of combining marks differs between sources; compare by consonants (as `macula` does for Hebrew lemmas).

## 4. Trees

### 4.1 The first Aramaic trees: Daniel and Ezra from MACULA

MACULA already has syntax trees for the Aramaic chapters, with `lang` = A on each word. Extending `tools/macula` with a language split would give **the first `arc` treebank**: about 7 500 morph units, a few hundred sentences, CC BY 4.0. What is needed:

- read `lang` per word and split each verse into its Hebrew and Aramaic parts (Dan 2:4 changes language mid-verse, [texts.md](texts.md) §4.1);
- write `arc` trees to their own file (`DAN.arc.conllu`, `EZR.arc.conllu`, `JER.arc.conllu`, `GEN.arc.conllu`);
- fold the determined ending into the noun (§3);
- map MACULA's Aramaic stems (peal, pael, haphel, aphel, shaphel, peil, hithpeel, hithpaal, hophal, ithpeel, ithpaal, hishtaphel) to a feature. The Hebrew converter uses `HebBinyan`; for Aramaic a neutral `Stem=` (or `AramStem=`) is clearer, with the same values in lower case as the manual (`Stem=Peal`);
- states: `Definite=Ind` (absolute), `Definite=Cons` (construct), `Definite=Def` (determined), as UD uses for Hebrew and Arabic;
- keep the same head rules as for Hebrew; add one for the pronominal copula (הוּא, הִמּוֹ as `cop`) and one for ל־ marking a direct object (`case` on an `obj`, not `obl`).

There is **no gold UD Aramaic treebank** to measure against. The check is therefore: (a) the tree validator (one head, one root, no cycle; a negative control must show red), (b) two runs give the same bytes, (c) a hand-checked sample of 50 sentences, annotated independently, before any number is claimed.

### 4.2 Targums by projection from Hebrew

The targums translate the Hebrew Bible **verse by verse**, and Onkelos often **word by word** ([../arc-x-targum/translation-syntax.md](../arc-x-targum/translation-syntax.md)). MACULA gives trees for every Hebrew verse. So:

1. align each Onkelos verse with its Hebrew verse (same reference: free);
2. align words inside the verse: by the regular correspondences (Hebrew אֶת ↔ Aramaic יָת, Hebrew ה־ ↔ Aramaic ־א, Hebrew בָּרָא ↔ Aramaic בְּרָא), by a Hebrew–Aramaic dictionary built from Jastrow's and BDB's cross-references, and by position;
3. project the Hebrew tree onto the aligned Aramaic words, and let a small rule set fix the known divergences (Aramaic additions such as מֵימְרָא דַּייָ, קֳדָם, determined state, the ית object).

This is the cheapest way to tens of thousands of Aramaic sentences with trees. Its accuracy must be measured on a hand-checked sample before it is trusted; projection errors are expected where the targum paraphrases (poetry, Genesis 49, Deuteronomy 32–33) and should be flagged by alignment confidence.

### 4.3 The Talmuds and the Zohar

No treebank exists for JBA, JPA or Zoharic Aramaic. The path: a tagger and parser trained on the `arc` and projected `arc-x-targum` trees, used as a first guess, with the vocabulary of Jastrow; then corrections and self-training. The Bavli's dialogue syntax (short questions, quotations, technical terms, [../tmr/discourse-terms.md](../tmr/discourse-terms.md)) is far from the Bible's narrative, so expect a large drop; measure it on a hand-annotated sample, never on the training material.

## 5. Language identification

Every corpus here mixes Hebrew and Aramaic: the Bavli (Mishnah and baraitot in Hebrew), the Yerushalmi, Daniel 2:4, the Zohar (Midrash ha-Ne'elam is mostly Hebrew), the commentaries (Hebrew with Aramaic quotations). The registry's `lang` and `mixed` are set by **Sefaria category**, not by sentence.

A per-sentence identifier is the first tool to build. Features that need no training (the comparison table in [family.md](family.md) §5):

| cue | points to |
|---|---|
| ־ָא / ־יָא at the end of many nouns; דְּ־ prefixes; יָת | Aramaic |
| הַ־ article, אֶת, אֲשֶׁר, שֶׁ־, ־ִים, ־וֹת | Hebrew |
| לֵימָא, אִיכָּא, לֵיכָּא, הַאי, הָנֵי, קָא, מַאי, אַמַּאי | `tmr` |
| חֲמֵי, כְּדוֹן, הָדֵין, הָדָא, אִית, לֵית, ־וּן perfects | `jpa` |
| יָת on most objects, אֲרֵי, קֳדָם יְיָ, מֵימְרָא | `arc-x-targum` |
| אִיהוּ / אִיהִי as copula, ת"ח, רָזָא, סִטְרָא, sefirot names | `arc-x-zohar` |
| דִּי as a separate word, הִמּוֹ, haphel in ה־, Persian chancery words | `arc` |

Score each sentence by counts of cue words (a deterministic, explainable classifier: it can say which cues decided), and measure on hand-labelled sentences from each corpus, including the hard cases (Hebrew baraitot quoted in Aramaic sugyot; Aramaic phrases inside Hebrew commentaries). A classifier that cannot be shown wrong on a planted Hebrew sentence in an Aramaic paragraph is not tested.

Built: [`tools/langseg`](../../tools/langseg) uses this table as named cues beside learned word and affix features, splits mixed runs with a Viterbi pass, and explains each segment; its README has the measurements (2026-10-18: macro 83.0% over the eight Hebrew-script varieties on held-out tractates, chapters and verses; 81.0% on a hand audit of Talmud sentences).

## 6. The lexicon

- **One Aramaic–English dictionary for all five varieties: Jastrow 1903** (public domain, 32 512 entries, already as JSONL). It covers the targums, both Talmuds and the midrash, and its entries cite sources (Targ., Y., B.), so each sense can be tagged by variety. BDB-Aram (863 entries) covers the Bible.
- **Lemmas.** Jastrow's headwords are pointed and sometimes numbered (קַמָּא, חֲמִי, כְּדוּן); key them by consonants, as the dictionary builder does for Hebrew.
- **Hebrew–Aramaic links.** The regular correspondences ([script-phonology.md](script-phonology.md) §4) and the cross-references in Jastrow and BDB ("BH", "h. text") give links between Hebrew and Aramaic lemmas. These are what the projection of §4.2 needs, and what a reader of Hebrew needs to learn Aramaic.
- **Variety tags on words.** The comparison words (חזי / חמי, השתא / כדון, איכא / אית) are few, but very frequent: they belong in a small hand-made table, not in a learned model.

## 7. Licences, in one place

Following `licence/DATA.md`:

- **Build and ship:** public-domain texts (WLC, Onkelos and Jonathan in Sefaria's PD versions, Pseudo-Jonathan, the Fragment targum, Zohar Ḥadash, Tiqqunei ha-Zohar vocalised) and dictionaries (Jastrow, BDB, Levy, Gesenius); CC BY data (MACULA, the Guggenheimer Yerushalmi) with attribution.
- **As little as possible:** CC BY-SA (the Wikisource Bavli, the Yemenite Taj Onkelos): test, measure, learn; not carried into a release.
- **Tests only:** CC BY-NC (William Davidson Talmud, Metsudah Onkelos).
- **Not used:** licence unknown (Venice and Mechon-Mamre Yerushalmi, Neofiti, the main Zohar's modern editions, the Sulam).

The ancient texts themselves are public domain; the restriction is on the **digital editions**. A PD source for the main text of the Zohar and the Bavli is therefore on the "wanted" list ([sources.md](sources.md)).

## 8. A plan, in order

| step | output | size (expected) | measure |
|---|---|---|---|
| 1 | `macula` with a language split → `arc` trees of Daniel, Ezra, Jer 10:11, Gen 31:47 | ~7 500 units | validator, determinism, 50-sentence hand check |
| 2 | Hebrew/Aramaic sentence identifier with the cue table | all corpora | hand-labelled sample per corpus; negative control |
| 3 | Jastrow + BDB-Aram as one keyed dictionary with variety tags | ~33 000 entries | coverage of each corpus's word types |
| 4 | Onkelos–Hebrew word alignment and tree projection → `arc-x-targum` trees | ~83 000 words (Torah); then Jonathan 158 000 | hand-checked sample; alignment confidence |
| 5 | tagger and parser for Aramaic (all varieties, variety as a feature) trained on 1 + 4 | | held-out targum verses; hand-annotated JBA, JPA, Zohar samples |
| 6 | the Bavli: Hebrew/Aramaic split, abbreviation expansion, parsing | ~1.9 M words (CC BY-SA edition: test only until a PD text is found) | hand-annotated sugyot |

Nothing in this plan is started (2026-10-17). Steps 4–6 are new directions and need the operator's word before they are run.

The Elamur team
