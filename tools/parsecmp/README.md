# parsecmp — many parsers against the reference trees, and parser agreement

How the parser collection is run: [docs/parsers.md](../../docs/parsers.md).

## Run

```bash
cargo build --release -p parsecmp
target/release/parsecmp <run dir>
target/release/parsecmp agree <dir> <a> <b> [<c>]
```

- `parsecmp <run dir>` compares every parser output under `<run dir>/out/` with the reference trees `in/haiku.conllu` (course per sentence in `in/course.txt`). Sentences are aligned by order; a sentence whose token count differs is skipped for that parser and counted. It writes `report.md`, `suspects.tsv` (tokens where a strong majority of parsers disagrees with the reference) and `per-course.tsv` into the run dir.
- `parsecmp agree` reads `<dir>/<a>/*.conllu`, `<dir>/<b>/*.conllu` (and `<c>`) for the same sentences and prints, per file, UAS and LAS (universal relation, punctuation left out) of each pair. Two witnesses that agree with each other but not with `a` point to `a` misreading the text; when the witnesses disagree too, the text is hard.

## Limits

The list of dependency systems is fixed in the code (the stronger ones only). Constituency outputs (`*.ptb`) are not compared with the reference; the report gives labelled bracket F1 between each pair of them. The reference trees are themselves teacher annotations, not gold. Agreement is evidence of difficulty or error, not a gold label.
