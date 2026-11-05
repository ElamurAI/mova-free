# gutenberg — Project Gutenberg books for the corpus

Selects English books from Project Gutenberg's `pg_catalog.csv` by topic group and downloads their plain texts slowly from the bulk-download mirror.

## Build and run

```bash
cargo build --release -p gutenberg
target/release/gutenberg select <pg_catalog.csv> <out.jsonl> <group>... [--max-died YEAR] [--skip <dir>]... [--not-pd]
target/release/gutenberg fetch <list.jsonl> <out-root> [--pause SECONDS]
```

## select

- Groups, matched case-insensitively on Subjects and Bookshelves: `time`, `titanic`, `shipwreck`, `disaster`, `detective`, `legal`, `tales`, `erotic`, `children`, `fiction`. A book goes to the first group it matches, in the order given on the command line. War books are left out of `disaster`.
- Only `Text` entries in English.
- Public-domain rule: every dated person of the book died in `--max-died` or earlier (default 1955: life + 70, public domain in the EU and Ukraine in 2026). A book with an undated person is not counted as public domain.
- `--not-pd` selects the opposite side: books that are not (or not provably) public domain in the EU/UA. They are for analysis logs only (gap journal, unknown words, absurdity), never corpus text, trees or answer texts, and go to a separate root.
- `--skip <dir>` skips ids already present (`pg<id>.txt` files or `…-pg-<id>` folders).
- Output: one JSON line per book with `id`, `group`, `title`, `authors`, `died`, `public_domain_eu_ua`, `subjects`; a count per group on stdout.

## fetch

Writes `<out-root>/<group>/pg<id>.txt` from the PGLAF mirror that Project Gutenberg asks bulk downloads to use. It skips existing files, pauses between requests (default 2 s), logs books without a plain text to `missing.txt`, and stops after 10 failures in a row.

## Limits

The public-domain test is by death years in the catalog only; it does not check translators or editions beyond the listed persons. Data licences: [licence/DATA.md](../../licence/DATA.md).
