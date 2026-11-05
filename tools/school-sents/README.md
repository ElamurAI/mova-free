# school-sents — sentence pools for the annotator and the cycle

`school-sents` cuts texts into sentences and writes them as a pool (`id<TAB>sentence`, or for lessons a four-column TSV) for the SLM, the annotator and the [teacher cycle](../cycle/README.md).

## Build

```bash
cargo build --release -p school-sents
```

## Forms

| Command | Input → output |
|---|---|
| `school-sents <school-dir> <out.tsv>` | every `<course>/*.lesson`: the `explain:` and `fact:` values, cut into sentences; fragments under three words dropped. Columns: course, lesson file, index in the lesson, sentence |
| `school-sents corpus <corpus-dir> <pool.tsv>` | the open corpus (release 0.4 layout, `corpus/open`): books (`book:<file>:<n>`), the pupils' dictionary (`dict:<word>:<n>`) and the train-split questions of the maths scripts (`math:<id>:<n>`); the contrast sets are tests and are not read |
| `school-sents books <books-dir> <pool.tsv> [--skip <file.txt>]… [--exclude <test.conllu>]…` | books only; `--exclude` drops every sentence that is a sentence of a held-out test set |
| `school-sents usfm <dir> <pool.tsv> <prefix> [BOOK,…\|-] [--hebrew-verses]` | scripture in USFM (`*.usfm` or `*.usfm.zst`), one sentence per id `<prefix>:<BOOK>:<n>`; `<pool.tsv>.verses` maps ids to verses; `--hebrew-verses` maps each sentence to the Hebrew verse numbering |
| `school-sents notes <dir> <pool.tsv> <prefix>` | Markdown notes (`*.md` except README.md) with markup taken off; `<prefix>:<file stem>:<n>` |
| `school-sents addressed <pool.tsv> <prefix>=<in.tsv>…` | texts by address (`address<TAB>text`); `<prefix>:<n>`; `<pool.tsv>.verses` maps ids to addresses |
| `school-sents scans <dir> <pool.tsv> <prefix>` | English prose of scanned books (`*.txt`): running heads, page numbers and line-end hyphens repaired; OCR noise (under 70% known English forms) left out; `<pool.tsv>.counts` per book |

Book texts lose the Project Gutenberg header and footer; a «sentence» over 200 words (a table of contents, a run-together list) is left out. Sentences with Hebrew or Aramaic script are counted in `notes` and `scans`.

## Limits

The cut is rule-based (`.`, `!`, `?` before a space and a capital or an opening quote, with extra title and numbering rules for prose). Lessons keep the older cut on purpose: the parse database's sentence ids count them.
