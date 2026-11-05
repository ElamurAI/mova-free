# wikt-count — streaming statistics over a Wiktionary dump

A one-off counter over a Wikimedia XML dump read from stdin. It surveys Wiktionary dumps; see [docs/data.md](../../docs/data.md).

## Run

```bash
cargo build --release -p wikt-count
lbzip2 -dc $MOVA_DATA/raw/wiktionary/<db>/<date>/*-multistream*.xml*.bz2 | target/release/wikt-count <db> summary|en|uk <out-prefix>
```

- Every mode writes `<out-prefix>.summary.json`: per namespace pages, redirects, pages with a `[[link]]` (MediaWiki's article count), text bytes; for main-namespace articles also the year of the last edit and the last editors (top 100, distinct count, anonymous).
- `en` and `uk` also parse the language sections of the main namespace: `.langs.tsv` (counters per language) and `.templates.tsv` (templates in the focus languages); `uk` adds `.morph.tsv` and `.sumcite.tsv`.

## Limits

Line-based parsing of the export-0.11 format (one tag per line, escaped text), without an XML parser. Wiktionary is CC BY-SA: derived counts are for measurement; see [licence/DATA.md](../../licence/DATA.md).
