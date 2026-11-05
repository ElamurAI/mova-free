# treebanks — UD treebank facts and their papers

The crate writes the facts table and index of the [treebanks/](../../treebanks/README.md) folder: one row per Universal Dependencies treebank, with the papers about it.

## Run

```bash
cargo build --release -p treebanks
target/release/treebanks build <ud-treebanks-vX.tgz> <ud-docs/docs/treebanks> <anthology.bib[.gz]> <papers-dir> <out-dir> <run-dir>
target/release/treebanks index <out-dir>
```

- The release archive is read as a stream: README, `stats.xml` and the size of each `.conllu` per split.
- Gates (nothing is written if one fails): every split's size equals the release `stats.xml`; every README has its metadata (License, Genre, the five layers, Contributors, Data available since).
- Output: `<out>/cards.tsv` and `<out>/INDEX.md`. `index` re-renders `INDEX.md` from `cards.tsv` alone, for example after a treebank folder gets seeds or a converter.
- Run files: `missing-ids.txt` (ACL ids not yet in the library, the input of [`sources bib-ids`](../sources/README.md)), `refs.jsonl` (every reference with its result), `title-matches.tsv` (title matches to review by eye), `unmatched.tsv` (references without a match, with hints).

## `cards.tsv`

A header row, then one row per treebank, sorted by language code and treebank name. Values are facts from the README's machine-readable metadata, the counted sizes and the reference matching; no README prose is carried.

| Column | Value |
|---|---|
| `ud_name` | release folder, `UD_English-EWT` |
| `lang`, `language`, `family` | ISO code from the file names, language name, family from the UD docs page |
| `treebank`, `ud_code` | `ewt`, `en_ewt` |
| `release` | UD release, `2.18` |
| `license`, `genres`, `includes_text`, `parallel`, `since`, `authors` | metadata (`authors` is `Contributors` as «First Last; …») |
| `lemmas`, `upos`, `xpos`, `features`, `relations` | how each layer was annotated, on the UD scale (manual native, converted from manual, converted with corrections, automatic with corrections, automatic, not available) |
| `train`, `dev`, `test`, `total` | five numbers: sentences, tokens, words, multiword tokens, empty nodes; an absent split is empty |
| `ud_page` | the treebank page in the UD docs, empty when the docs snapshot has none |
| `readme` | the README in the release |
| `papers` | matched papers, space-separated `<ACL id>\|<how>` or `lib:<library folder>\|<how>`; `how` is `url`, `doi`, `bibtex` or `title`, `~y` marks a README year off by one, `\|=<id>` follows for another record of the same paper (ACL id or library folder) |
| `unmatched_refs` | references with no match in the library or the ACL Anthology (listed in the run's `unmatched.tsv`) |

Readers: `tools/db` (`load --kind treebank` and `verify`, the size check) and the index.

Seeds (`seeds.md`) and converter notes (`convert.md`) inside the treebank folders are written by hand; `build` does not touch them.
