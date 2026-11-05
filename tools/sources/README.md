# sources — manifests of open sources, download queues and verification

`sources` turns JSONL manifests of open resources into a layout under the raw-data root, curl queues per host, and a verification pass.

## Run

```bash
cargo build --release -p sources
target/release/sources plan    <raw-root> <queues-dir> <manifest.jsonl>...
target/release/sources verify  <raw-root> <manifest.jsonl>...
target/release/sources merge   <out.jsonl> <manifest.jsonl>...
target/release/sources bib     <anthology.bib> <rules.tsv> <out-dir>
target/release/sources bib-ids <anthology.bib> <ids.txt> <out.jsonl>
```

- Manifest row: `{"slug","name","langs","kind","year","status","license","license_url","source_page","format","files":[{"url","size","md5","sha1"}],"priority","notes"}`.
- `plan` writes `<raw-root>/<slug>/SOURCES.md` (provenance) and `<queues-dir>/curl-<host>.cfg` for files not yet present or of a different size: sequential within a host, parallel across hosts. Wiktionary-derived and non-open entries are not planned. A duplicate slug, URL or file name within a slug is an error.
- `verify` checks that every file exists, its size and md5/sha1 match, and its signature matches the extension (pdf, djvu, zip, gz, bz2, xz, zst). Any failure gives exit code 1.
- `merge` joins manifests: a resource whose URLs are a subset of an earlier one's is dropped, shared URLs stay with the first owner; every decision is printed.
- `bib` selects ACL Anthology papers by topic rules into manifests; `bib-ids` does it from an id list (`id [\t topic [\t priority]]`).

Used by [tools/mathlib](../mathlib/README.md) (manifest format) and [tools/treebanks](../treebanks/README.md) (missing paper ids).
