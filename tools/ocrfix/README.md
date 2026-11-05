# ocrfix

OCR of public-domain printed editions of Ancient Hebrew and Aramaic (square script), deterministic correction, and measurement against hand transcriptions.

The scans of the research collection (`raw/hbo-corpus`, `raw/arc-corpus`, `raw/hebgram/pd`) come from the Internet Archive; their text layers carry almost no Hebrew letters. This tool reads them again.

## Tools (no root needed)

- **Tesseract 5.5.3** from conda-forge, installed with a static `micromamba` into `$MOVA_DATA/ocr/env`, with the `tessdata_best` models (`heb`, `eng`, `deu`, `script/Hebrew`) in `$MOVA_DATA/ocr/tessdata_best` (Apache 2.0).
- **Kraken 7.1.1** in a venv, `$MOVA_DATA/ocr/venv`, with the BiblIA model (medieval Hebrew manuscripts, Zenodo 10.5281/zenodo.5468286, CC BY 4.0). No Kraken model for *printed* Hebrew is published; the manuscript model still reads some printed editions well.
- `pdftoppm` / `pdfimages` / `pdfinfo` (poppler, already on the system).

```
MOVA_DATA=<your data directory>
curl -sL https://micro.mamba.pm/api/micromamba/linux-64/latest | tar -xj -C $MOVA_DATA/ocr bin/micromamba
MAMBA_ROOT_PREFIX=$MOVA_DATA/ocr/mamba $MOVA_DATA/ocr/bin/micromamba create -y -p $MOVA_DATA/ocr/env -c conda-forge tesseract python=3.11
for f in heb eng deu script/Hebrew; do curl -sLo $MOVA_DATA/ocr/tessdata_best/$(basename $f).traineddata https://github.com/tesseract-ocr/tessdata_best/raw/main/$f.traineddata; done
$MOVA_DATA/ocr/env/bin/python -m venv $MOVA_DATA/ocr/venv && $MOVA_DATA/ocr/venv/bin/pip install kraken
$MOVA_DATA/ocr/venv/bin/kraken get 10.5281/zenodo.5468286
```

The tool finds them through `MOVA_TESSERACT`, `MOVA_TESSDATA`, `MOVA_KRAKEN`, `MOVA_KRAKEN_MODEL` (and `MOVA_TESS_PSM` for Tesseract's page segmentation mode).

## How it works

1. **Two Tesseract readings per page** (`ocrfix ocr`), at the scan's native resolution: one with the Hebrew model alone (it reads Hebrew best, but turns English into Hebrew-looking debris), one with Hebrew and a Latin model together (the *mask*: it tells the scripts apart, but loses Hebrew words in mixed lines). Words are matched by their boxes. A line the mask reads as mostly Hebrew script is kept whole from the Hebrew reading; in a Latin line only the words the mask also reads as Hebrew are kept, marked `~` (cited words). Numbers and letters mixed with digits are dropped. Words split at an editor's bracket (`מ[ לך`) are joined again.
2. **A second witness** (`ocrfix ocr --lang kraken`, then `ocrfix vote`): Kraken reads the same pages; its visual-order lines are turned to logical order; verse-number debris (one or two letters with a geresh) is dropped. The two word sequences are aligned (edit distance on words). Tesseract's word stays unless Kraken's aligned word differs by at most two letters (and at most a third of the word) and is clearly better attested: more than twice as frequent plus one, and, when Tesseract's form is itself attested five times or more, twenty times as frequent.
3. **Noisy channel** (`ocrfix correct`): each word against attested forms. Prior: counts from MACULA/WLC (Hebrew Bible) and from the Biblical Aramaic, Onkelos, Jonathan and the Writings targumim, plus the edition's own words (`--self`). Channel: the confusable pairs of the square script in `data/confusions.tsv` (ב/כ, ד/ר, ה/ח/ת, ו/ז/י, ם/ס, ע/צ, ש/ט, כ/נ …, vav and yod dropped or doubled), each with a per-edit base cost added (`--edit-cost`, 4 nats by default; the Run commands below use 5). Words under three letters are left alone; short words take only substitutions. Every change is logged with its counts (`--explain`).
4. **Edition text** (`ocrfix lines`): `ID:pNNNN:lNN<TAB>text|cited<TAB>words`, page and physical line ids.
5. **Measurement** (`ocrfix cer`): character error rate over the Hebrew-script letters; any run of other signs is one word gap; editors' brackets count as part of the word; `--points` keeps vowel points.

Everything is deterministic: Tesseract runs with one thread per page, maps are `BTreeMap`, so two readings of the same page are byte-identical.

## Measurement

`ocrfix cer` measures against the hand transcriptions in `data/gt/` (their headers say what is in and out): pilot
pages, used to design the pipeline, and hold-out pages, transcribed before their OCR was looked at. Hold-out pages set
nothing.

Why the pipeline has these steps: any Latin model in the same Tesseract pass as Hebrew (`heb+eng`) loses Hebrew words,
hence the two passes; Tesseract `tessdata` (fast) and `script/Hebrew` read worse than `tessdata_best` `heb`; Kraken
alone inserts letters from verse numbers. The noisy channel changes little on its own: the errors left are mostly real
words (זאת/ואת), forms missing from the biblical and targumic lexicons (Imperial Aramaic זי, names), or systematic
errors the edition repeats. A second witness helps where it is independent (Kraken on Ben Sira).

## Run

```
ocrfix lexicon --out lex/hbo-wlc.tsv --conllu $MOVA_DATA/corpora/judaism/text/tanakh-wlc-macula
ocrfix lexicon --out lex/arc.tsv --conllu .../tanakh-arc-macula --verses .../targum-onkelos --verses .../targum-jonathan --verses .../targum-writings
ocrfix ocr --pdf BOOK.pdf --out runs/BOOK/tess [--mask heb+deu]
ocrfix ocr --lang kraken --pdf BOOK.pdf --out runs/BOOK/kraken --only-heb runs/BOOK/tess --min-heb 200
ocrfix vote --a runs/BOOK/tess --b runs/BOOK/kraken --out runs/BOOK/vote --lex lex/hbo-wlc.tsv --lex lex/arc.tsv --self 1
ocrfix correct --lex lex/hbo-wlc.tsv --lex lex/arc.tsv --self 0.5 --edit-cost 5 --out runs/BOOK/fixed runs/BOOK/vote/p*.txt
ocrfix lines --dir runs/BOOK/fixed --edition BOOK > text.tsv
ocrfix cer --gt data/gt/PAGE.txt --hyp runs/BOOK/fixed/pNNNN.txt --confusions 10
```

## Limits

- Grammars that cite pointed words inside German or English lines (Kautzsch 1884) are not served: the Latin model reads the Hebrew words as Latin debris, and the Hebrew model reads the German as Hebrew debris.
- Typefaces unlike modern Hebrew type (Ungnad 1911: nun like kaf, shin like a cup) give many errors, and the lexicon does not repair them: the edition's systematic errors enter its own word counts.
- Points: Tesseract emits some, Kraken's manuscript model few; no hand transcription with points was made, so their accuracy is not measured.
- Two-column pages (Ben Sira MS B) need `MOVA_TESS_PSM=6`; Kraken still reads them column by column, which weakens the vote there.
