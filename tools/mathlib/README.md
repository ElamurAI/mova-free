# mathlib — the maths library catalog and downloads

`mathlib` builds the catalog of open maths papers and textbooks and downloads a bounded, licence-checked selection. How the library is collected and its licence rules: [docs/data.md](../../docs/data.md#mathematics-and-science-library).

## Run

```bash
cargo build --release -p mathlib
```

| Command | What it does |
|---|---|
| `mathlib oai <out-dir> [--set S] [--prefix arXivRaw] [--from YYYY-MM-DD] [--base URL] [--delay-ms 4000] [--max-pages N]` | OAI-PMH ListRecords → `<out-dir>/pNNNNN.xml.gz` and a resumable `state.tsv` |
| `mathlib parse <out.tsv> <oai-dir>...` | arXivRaw records → catalog TSV (id, categories, licence, year, versions, sizes) |
| `mathlib sql <db> <file.sql> [key=value]... [--mode markdown]` | a SQL template (`{key}` → value) through the DuckDB CLI: load, statistics, pilot selection |
| `mathlib fetch <list.tsv> <raw-root> <manifest.jsonl> [--delay-ms 3500] [--max-bytes N] [--file-max N] [--min-free-gb 150] [--expect-pdf] [--compact-src] [--kind paper\|textbook]` | polite download of a list (url, licence) into `<raw-root>/<slug>/`; manifest in the [sources](../sources/README.md) format plus sha256 and licence per file |
| `mathlib gcs-list <bucket> <prefix> <out.tsv>` | names and sizes in a public GCS bucket (the arXiv mirror), no download |
| `mathlib books-fetch <catalog-dir> [all\|openstax\|aim\|otl]` | raw pages of open-textbook catalogs |
| `mathlib books-pdf <catalog-dir> <books.tsv>` | look for direct PDFs of books without one → `sites-pdf.tsv` |
| `mathlib books <catalog-dir> <out.tsv>` | offline catalog of open textbooks: licence, topic, level, direct PDF |

## Limits

Downloads stop below the free-space floor and keep to the delays above. Licences are taken from the catalog; the library is for learning and measurement under the rules of [licence/DATA.md](../../licence/DATA.md).
