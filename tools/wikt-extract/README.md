# wikt-extract — English Wiktionary entries as an add-on dictionary module

Reads the English entries of the English Wiktionary from the raw multistream dump and writes them as a Mova add-on
module (`docs/modules.md`): a dictionary layer in the columns of the school layers plus extra columns, its gap and
coverage reports, and `module.json`. The output is CC BY-SA 4.0 and GFDL, like Wiktionary: it is distributed apart
from Mova and never enters the repository, a release or a binary (`licence/DATA.md`).

## Run

```bash
cargo build --release -p wikt-extract
D=$MOVA_DATA/raw/wiktionary/enwiktionary/20260901
target/release/wikt-extract page  $D telly mate             # the rows of some pages
target/release/wikt-extract build $D $MOVA_DATA/modules/wiktionary-en/20260901-1 --threads 6
target/release/wikt-extract check $MOVA_DATA/modules/wiktionary-en/20260901-1
target/release/wikt-extract cover LAYER WORDS               # which words of a list have an entry
```

`build` refuses an output folder that already holds a `module.json` (a version is written once) or lies inside a Git
work tree. Measurements of the full pass, the coverage and the limits: [docs/data.md](../../docs/data.md),
«Wiktionary».

## How it works

- **Dump.** The index of each dump part (`offset:page_id:title`) names its bz2 streams of about 100 pages; each stream
  is decompressed on its own (as `tools/wikt-page` does for one page), so a fixed number of threads takes streams from
  one queue and memory stays bounded. Decompressed XML is never written to disk.
- **Section.** Only the `==English==` section of main-namespace pages. Each part-of-speech heading under it (under
  each `===Etymology N===`) is one entry: its head line (`{{en-noun}}`, `{{en-verb}}`, `{{en-adj}}`, `{{head|en|…}}`,
  head labels `{{tlb}}`), its senses (`#`, subsenses `##`), examples (`#:` lines, `{{ux}}`), in-sense nyms
  (`#: {{syn|en|…}}`, `{{ant}}`), quotations (`#*`, counted, their text not taken), and the `Synonyms` and `Antonyms`
  subsections (`{{l|en|…}}`, `{{col|en|…}}`, plain links).
- **Wikitext.** Template calls are parsed with their nesting and parameters; links, bold and italics, references,
  comments and HTML tags are reduced to text. Each known template has a handler that prints what the template prints
  (`{{l|en|word|alt}}` → alt or word, `{{w|A|b}}` → b, `{{taxlink|Felis catus|species}}` → Felis catus,
  `{{gloss|x}}` → (x), `{{plural of|en|cat}}` → Plural of cat, `{{place|en|town|p/Kent|c/England}}` → A town in Kent,
  England) and records what it means (labels, form-of lemma, nyms, examples). Categories, sense ids and maintenance
  templates are printed as nothing and counted as dropped. **A template with no handler is a gap**: its text is left
  out, its name is counted in the row's `gaps` column and in `gaps.tsv`; it is never dropped silently.
- **Forms.** From the head template's parameters, without the Lua modules: `en-noun` plurals (`+`, `s`, `es`, `~`,
  `-`, full forms), `en-verb` (positions 1–4, `+`, `++`), `en-adj`/`en-adv` (`er`, `more`, `-`, full forms). The
  default forms follow `Module:en-headword`'s main rules (-es after a sibilant, -ies after a consonant and y, final e
  dropped before -ing, the final consonant doubled in a one-syllable word ending in one vowel and one consonant); a
  parameter with an inline specification (`s<l:nonstandard>`) is kept raw (`raw=…`).
- **Order.** Rows are sorted by word in lower case (the lower-case title first), title and order in the page, so the
  output does not depend on the number of threads.

## Layer columns

| # | column | example |
|---|---|---|
| 1 | word (the page title) | `telly` |
| 2 | part of speech | `noun` |
| 3 | meaning: senses numbered, labels in brackets | `1) Television. 2) A television set.` |
| 4 | sense in the lessons (empty) | |
| 5 | examples, by sense number | `1: Not much on telly tonight, as usual!` |
| 6 | note: the etymology number | `etymology 1` |
| 7 | labels, by sense (`*` = the head line) | `*:UK,Commonwealth,Ireland,colloquial` |
| 8 | synonyms, by sense (`*` = the Synonyms section) | `*:tube,TV` |
| 9 | antonyms | |
| 10 | forms | `countable=both; plural=tellies; raw=s<l:nonstandard>` |
| 11 | form of, by sense | `1:diminutive of:breakfast` (brekkie) |
| 12 | gaps: templates with no handler | `antonym×1` |
| 13 | provenance: page id, revision id, dump file, stream offset | `74428 rev 91706010 enwiktionary-…-p1p1500000.bz2@80233783` |

The first six are the columns `world::dictstore` reads for every layer; it keeps 7–13 as `Entry::extra`
(`dictstore::EXTRA_COLUMNS`). The header's first line declares the licence (`# License: CC BY-SA 4.0 and GFDL …`),
so `dictstore::shippable` refuses the file for a shipped use.

## Limits

- English entries of the English edition only; translations, pronunciation, etymology text and quotations are not
  taken; `Thesaurus:` pages and translation subpages are not read.
- The templates are emulated, not executed: the Lua modules (inflection, `place`, labels' display forms) are not run,
  so forms and some printed texts are approximations; categories are not reproduced.
- The first sense of a page is often not the sense a reader needs (an initialism in another field, a place before a
  surname): an entry is a draft layer, knowledge only with `allow_unreviewed`.
