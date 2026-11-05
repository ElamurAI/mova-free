# wikt-page — raw wikitext of pages by exact title

Reads single pages from a Wikimedia multistream dump without unpacking it. Usage: [docs/data.md](../../docs/data.md).

## Run

```bash
cargo build --release -p wikt-page
target/release/wikt-page <dump-dir> <title>...
target/release/wikt-page $MOVA_DATA/raw/wiktionary/enwiktionary/<date> language 'Template:en-noun'
```

It looks each title up in `*-multistream-index*.bz2` (`offset:page_id:title`) and decompresses exactly one bz2 stream (about 100 pages) at that offset. Titles are exact, with the namespace prefix as the wiki spells it. Output: `=== <title> | ns=… id=… rev=… | <file>` followed by the wikitext. Titles not found go to stderr, exit code 1.
