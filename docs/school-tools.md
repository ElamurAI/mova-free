# School tools

The checks and safe edits of the school data — the lessons (`world/data/school/<course>/*.lesson`) and the pupils'
dictionary (`world/data/dictionary-improved.tsv`) — are `world` subcommands (`world/src/schooltools.rs`).

**The rule: agents use these commands, not scripts.** No python, awk, `split` or copy-back one-offs for school data.
A brief says «use `world knowledge …`»; a need the commands do not cover is a new subcommand in Rust, with a test.

Every length is counted in Unicode characters, never bytes. With no course named, a command takes every primary
course (`knowledge::course_names`: no philosophy, no programming, no maths `lessons` folder, no `dictionary`).
A course is a folder of `data/school`; a folder or a `.lesson` file path works too. `--help` prints the usage.

An option line: `exercise:|test:|exam: question | right | wrong | wrong | wrong`.

## `world knowledge balance [course…] [--band L_lo,L_hi,S_lo,S_hi] [--list out.tsv] [--chunk N --prefix p]`

How often the right answer is strictly the longest option (L) and strictly the shortest (S), per course and in all,
and the lessons outside the band, each with what is off: «right never shortest», «right-shortest too rare/too
often», «right-longest too rare/too often». Default band: L in [1/6, 1/3], S in [1/10, 1/3], S > 0 (fractions or
decimals). `--list` writes every lesson (`file L S n off`); `--chunk N --prefix p` splits the lessons outside the band
into `p001.tsv`, `p002.tsv`, … of N lessons each — the work queue for fixing agents.

```
world knowledge balance                                   # all primary courses
world knowledge balance english french --band 1/5,1/3,1/8,1/3
world knowledge balance --list /tmp/bal/all.tsv --chunk 40 --prefix /tmp/bal/chunk-
```

On 10.10: 2165 lessons, 70979 questions, right strictly longest 16959 (23.9%), strictly shortest 11846 (16.7%);
382 lessons outside the default band.

## `world knowledge lint [course…] [--fix-spaces]`

Errors: option lines with ≠5 fields, empty fields, not exactly one space on each side of `|`, duplicate options, a
wrong option equal to the right one, stray tabs. Reported apart: case-only duplicates («spelling question?» — often on
purpose, as in english 026/035, french 010/011/027/028) and wrong options glued from the other options («X and Y»,
«X; Y» — a heuristic, report only; «One hour and thirty minutes» beside «One hour» and «Thirty minutes» is often an
honest distractor). `--fix-spaces` normalises the spaces around `|` and at the field ends, nothing else.

```
world knowledge lint
world knowledge lint english --fix-spaces
```

## `world knowledge edit <edits.tsv> [--dry-run]`

Applies option edits; each row `file<TAB>line<TAB>old option text<TAB>new option text` (`#` lines skipped; the file as
a path, as `course/file.lesson`, or a bare file name found in one course). A file with any bad row is left untouched:
the line is not an option line, the old text is not exactly one option there, the new text is empty or holds `|`, a
tab or a newline, the question would change, the right answer would stop being first, the result would have duplicate
options (case-insensitive) or not three wrong ones. Prints L/S/n before and after for every touched file; exits with
an error if any file was refused.

```
printf 'english/012-x.lesson\t31\tA cat\tA small grey cat\n' > /tmp/edits.tsv
world knowledge edit /tmp/edits.tsv --dry-run
world knowledge edit /tmp/edits.tsv
```

## `world knowledge snapshot <course|file…> <dir>` and `world knowledge restore-lines <dir> <file> <line…>`

`snapshot` copies lesson files into `<dir>/<course>/<file>`. `restore-lines` puts back only the given lines (`12`,
`10-14`) of one file from its copy, and refuses if the file and the copy differ in their number of lines. Never copy
whole folders back: that wipes the neighbours' work (10.10, 08:33).

```
world knowledge snapshot english /tmp/snap-english
world knowledge restore-lines /tmp/snap-english english/012-x.lesson 31 40-42
```

## `world dictionary merge <records.tsv…> [--check]`

Validates the records — UTF-8, 6 TAB fields (no tab or carriage return inside a field), no empty field, the word
lower-case with no spaces at its ends, no dates — skips the words already in `data/dictionary-improved.tsv` and
repeats inside the input (the first kept), and appends the new ones. Any bad record: nothing is merged. `--check` only
validates. The binary embeds the dictionary at build time: rebuild after a merge.

```
world dictionary merge --check /tmp/records-*.tsv
world dictionary merge /tmp/records-*.tsv
```

## `world dictionary gaps [--split N --prefix p]`

The words of the primary lessons without an improved record (the list `dictionary schools` writes to gaps.txt),
counted from the records on disk, so a stale binary does not matter; `--split` writes `p001.txt`, … of N words each.
On 10.10: 0 gaps.

```
world dictionary gaps
world dictionary gaps --split 150 --prefix /tmp/gaps/batch-
```
