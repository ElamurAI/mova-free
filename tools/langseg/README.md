# langseg

A language segmenter. Texts Mova reads mix languages: a paper has English prose with Biblical Hebrew, Aramaic and Modern Hebrew quotes; a Talmud page mixes Mishnaic Hebrew and Babylonian Aramaic. `langseg` cuts a text into segments and gives each a language code, so that each segment can go to its own parser and pragmatics.

Codes: `en`, `translit` (Hebrew or Aramaic in scholarly Latin transliteration; the variety is not decided), `other` (any other script), and the Hebrew-script varieties `hbo` (Biblical Hebrew), `hbo-x-mishnaic` (Mishnaic and other Rabbinic Hebrew), `he` (Modern Hebrew), `arc` (Biblical Aramaic), `arc-x-targum`, `tmr` (Babylonian Talmudic Aramaic), `jpa` (Jewish Palestinian Aramaic), `arc-x-zohar`.

## How it works

1. **Script runs.** The text is cut into tokens with byte offsets: Hebrew script (letters with their points, accents, geresh, and an inner `"` as in ת"ח), Latin (with combining marks and an inner hyphen or apostrophe), other scripts. Spaces, digits and punctuation are neutral: they join tokens of one script and split tokens of two.
2. **Transliteration.** Inside a Latin run a word with a sign of transliteration (ʾ ʿ ḥ ṭ ṣ ś š ḇ ḡ ḏ ḵ ṯ ā ē ī ō ū ă ĕ ŏ ə) is `translit`, and so is every Latin word in the same italic span (`*…*`, `_…_`, `<i>`, `<em>`). The rest of the run is `en`.
3. **Varieties.** Every Hebrew-script word fires features in five groups:
   - `P` pointing: vowel points, cantillation accents, a pointed wayyiqtol (וַ + dagesh), gershayim, geresh;
   - `C` manual cue words, from the comparison table in `languages/arc/family.md` §5, `languages/arc/mova.md` §5, `languages/hbo/11-mishnaic.md` and `languages/he/` (די / אשר / של, ית / את, כען / כדון / השתא / עתה / עכשיו, איכא / אית / יש, קא, מאי, האי / הדין / דנה / זה, איהו, אינון / אינהו / המו, the l-imperfect לימא, ארי, קדם, ייי, רזא, סטרא, Modern מאוד, האם, לגבי …), 132 forms in `src/cues.rs`;
   - `A` manual cue morphemes: the determined ending ־א / ־יא / ־ייא, plurals ־ין / ־ים / ־ות, suffixes ־הון / ־הם, ־כון / ־כם, prefixes ד־, ה־, ש־, כש־, את־ / הת־ / נת־, לי־, וי־;
   - `M` generic prefixes and suffixes (first one or two letters, last one to three);
   - `W` the consonantal word itself.

   Features are counted on the consonantal skeleton, so a cue fires on pointed and unpointed text alike. Each feature has a log-probability per variety (naive Bayes, one multinomial per feature group, add-½ smoothing, rare `M`/`W` features dropped), learned from the pool below. A Viterbi pass over the words of a run chooses one variety per word with a cost for every switch (16 nats, chosen on dev with `mix --split dev --sweep`; a quarter of that at a sentence end), so a run is cut where the language changes. Each segment reports the runner-up variety, the margin, and the features with the largest summed log-odds of the winner over the runner-up, with what the manual says about each cue.
4. **Cleaning the Talmud labels.** The corpus labels are by tractate: a Bavli or Yerushalmi gemara segment is labelled `tmr` / `jpa` even when it is a Hebrew baraita or a quoted Mishnah. Before the final count, two rounds relabel a training segment of those corpora as `hbo-x-mishnaic` when the model of the round before reads it so. Evaluation labels are never relabelled. The number of rounds was chosen on the dev audit (`audit/dev.tsv`).

Everything is deterministic (`BTreeMap`, seeded generators); two runs of `eval --split dev` give identical output.

## Run

```
cargo build --release -p langseg
langseg corpus                   # the pool: $MOVA_DATA/langseg/{train,dev,test}.tsv (raw data from $RESEARCH_RAW, default $MOVA_DATA/research/raw)
langseg train                    # $MOVA_DATA/langseg/model.tsv
langseg seg --explain 3 < text   # one text on stdin
langseg seg --pool < pool.tsv    # id<TAB>sentence lines
langseg eval --split dev [--strip] [--groups PCA] [--no-clean] [--shuffle] [--errors N]
langseg mix --split dev [--sweep] [--strip] [--shuffle]
langseg audit --split dev --gold tools/langseg/audit/dev.tsv
langseg cues                     # every manual cue with the variety it was learned to favour
langseg talmud <he.json> <en.json> <Tractate> <outdir> [--verses <pool.tsv.verses>]   # the Yerushalmi, below
cargo test --release -p langseg
```

`$MOVA_DATA` defaults to `~/mova-data`. Output of `seg`: `id<TAB>start<TAB>end<TAB>lang<TAB>text`, byte offsets into the input; with `--explain N` a sixth column, for example

```
-	161	176	tmr	מאי טעמא	vs arc-x-zohar by 12.3: C:q_taama(+2.2; tmr (מאי טעמא)), W:טעמא(+1.8), C:q_may(+1.7; tmr, arc-x-zohar (מאי))
-	271	278	translit	ʾămar
```

`--strip` removes points and accents before scoring (the unpointed condition). Every read of the test split is appended to `$MOVA_DATA/langseg/test-reads.log`, and the same command on the test split is refused a second time unless `--again` is given.

## The Yerushalmi by address

`langseg talmud` reads one tractate of the Jerusalem Talmud in Guggenheimer's edition (Sefaria JSON, CC BY; the
Hebrew/Aramaic text and the English translation) and writes, by address `<Tractate>.<chapter>.<halakhah>.<segment>`:

- `<tractate>.tsv`: `address, lang, text`, the original cut into language segments (one row per segment, in order;
  a segment keeps the punctuation up to the next one). Tags, page marks and note anchors are taken off; the heading
  `משנה:` / `הלכה:` is dropped and becomes the part (`mishnah` / `halakhah`).
- `<tractate>.en.tsv`: `address, text`, the translation without Guggenheimer's footnotes (his commentary).
- with `--verses` (the English pool's map `id<TAB>address`, from `school-sents addressed`): `index-<tractate>.tsv`,
  `address, part, original, en`: the languages of the address's original segments in order, and its English
  sentence ids.

Both tractates of the pilot (Berakhot, Peah) are in this model's train split with their corpus labels, so the
segments of these two are not a held-out measurement.

## Data

Selected through the scripture registry `$RESEARCH_RAW/scripture/texts.tsv`: only rows with scope `open` and a licence that is neither NC nor unknown. Segments are sentences cut at `. : ׃ ? ! ;` and then at 25 words, at least 3 words. Split by document hash: 1 in 10 documents test, 1 in 10 dev; a document is a tractate (Mishnah, Tosefta, Bavli and Yerushalmi of one tractate together), a targum chapter, a Zohar section, a block of ten Bible verses (a single verse for Biblical Aramaic, which is small); UD Hebrew keeps its own split.

| code | source (licence) |
|---|---|
| `hbo` | MACULA Hebrew WLC lowfat, `lang="H"` (CC BY 4.0) |
| `arc` | MACULA Hebrew, `lang="A"`: Daniel, Ezra, Jer 10:11, Gen 31:47 |
| `arc-x-targum` | Onkelos, Jonathan, targums to the Writings (PD; the Taj, CC BY-SA) |
| `tmr` | Bavli, Wikisource (CC BY-SA), gemara |
| `jpa` | Yerushalmi, Guggenheimer (CC BY), halakha |
| `arc-x-zohar` | Zohar Chadash, Tikkunei Zohar vocalised (PD), without Midrash ha-Ne'elam |
| `hbo-x-mishnaic` | Mishnah (PD), Tosefta (CC BY), the Mishnah sections of both Talmuds |
| `he` | UD Hebrew IAHLTwiki and IAHLTknesset (CC BY-SA) |
| `en` | Sefaria CC0 English Mishnah translations (mixed documents only) |

Training uses at most 20 000 segments per variety, measurements at most 2 000, the first by id hash. The model and the pool live in `$MOVA_DATA`, not in the repo; the model is learned from CC BY-SA text among others, so it is a measurement tool and is not shipped. `audit/dev.tsv` and `audit/test.tsv` hold ids and hand labels only (A = Aramaic as the corpus says, H = Hebrew, M = mixed, names only or a Bible verse; M is left out).

## Measurement

`eval` reports accuracy per variety against the corpus labels and a confusion matrix; `--strip` gives the unpointed
condition, `--groups PCA` the manual cues only, `--no-clean` the model without the Talmud cleaning, and `--shuffle`
the shuffled-label negative control (it must fall to chance). Most `tmr` and `jpa` "errors" against corpus labels are
Hebrew sentences in the Talmud, so `audit` measures the model against hand labels of the language actually written.
`mix` builds mixed documents (2–4 pieces of different varieties or English joined by a space, gold per token) and
reports token accuracy and boundary precision, recall and F1.

## Limits

- The Talmud labels are by corpus. Scores for `tmr` and `jpa` against corpus labels mix model error with label noise; the hand audit is small.
- `jpa` is the weakest variety: the Guggenheimer Yerushalmi is mostly Hebrew at the sentence level, so even after cleaning its Aramaic is learned from few segments, and short Palestinian Aramaic sentences go to `arc-x-targum`.
- Biblical Aramaic is a small corpus, so its dev and test splits are small. A short Biblical Aramaic phrase without points can be read as targumic Aramaic.
- Modern Hebrew comes from two genres (Wikipedia, Knesset). Unpointed Biblical quotes in a modern text are decided by vocabulary only.
- The learned weights disagree with the manual on some cues (`langseg cues`): ארום and ברם favour the targums and the Yerushalmi rather than what the table says, האי favours the Zohar, and ת"ח (spelt out תא חזי in the open Zohar editions) carries no Zoharic weight.
- Transliteration is found only by its special letters and by italics; a plain-ASCII transliteration (*malka*) stays `en`. The Latin side tells no other language from English.
- Yiddish, Ladino, Judeo-Arabic and Syriac in Hebrew script are not modelled and get one of the eight varieties.

The Elamur team
