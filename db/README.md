# Mova annotation database (DuckDB)

A DuckDB database for the annotation of Mova's sentences: treebank gold, silver, bronze, model runs, pragmatics and the tales, all over the same sentences. It lives in the data directory, and the whole Mova corpus is loaded into it.

**Where things are:**
- database — `$MOVA_DATA/db/mova.duckdb` (the dev workspace, outside the repo; ~170 MB);
- in the repo:
  - schema — `db/schema.sql`;
  - internal SQL for loading and exporting — `db/sql/`;
  - example queries — `db/queries/`;
  - corpus manifest — `db/corpus.tsv`;
  - effort of calibration runs — `db/calib-runs.tsv`;
  - code — `tools/db` (the `db` command).

**How data flows.** We do not use the duckdb Rust crate: it is a long C++ build. The CLI `~/bin/duckdb` does the work (variable `DUCKDB`):
- Rust writes TSV, the CLI loads it with `COPY … FROM` in one transaction;
- for export the CLI writes `COPY … TO` into TSV, Rust assembles CoNLL-U.

TSV has no quotes or escapes, NULL is `\N`. A value containing TAB, LF, CR or equal to `\N` cannot be written: this is an error, not silent corruption. CoNLL-U lines have none of these by definition, and JSON is written by serde_json with escapes.

## Rebuild from scratch

```sh
cd ~/Mova/tools/db && cargo build --release
D=~/Mova/target/release/db; B=$MOVA_DATA/db/mova.duckdb; U=$MOVA_DATA/en/ud/ud-treebanks-v2.18; R=$MOVA_DATA/runs/tales-2026-09-26
$D init $B                     # schema v1; does not overwrite an existing database
$D corpus $B                   # treebanks, Mova corpus, calibration — per db/corpus.tsv (~40 s)
$D tales $B $R/tales.jsonl $MOVA_DATA/raw $R $U/UD_English-EWT/en_ewt-ud-train.conllu $U/UD_English-ESLSpok/en_eslspok-ud-train.conllu   # ~1 min
$D verify $B                   # lossless: every file byte for byte
$D stats $B                    # overview (db/sql/stats.sql)
$D sql $B ../../db/queries/rust-vs-opus.sql
$D export $B doc=gutenberg:2591:hansel-and-gretel > hansel.conllu
```

Other commands:
- `db load <db> <source> <dialect> [options] <files.conllu>…` — one file or several;
- `db export <db> key=value…` — keys `file run source tool layer dialect origin doc sent sid aid gate`.

Temporary TSVs go to `$TMPDIR`; `DB_KEEP=1` keeps them for inspection.

## Schema

Idea: different annotations from different sources and different layers lie on the same sentences. Text goes `docs` → `sentences`. Each annotation is a separate set in `annsets` with its own provenance.

| table | what it holds | why |
|---|---|---|
| `runs` | a run or load: name, kind (treebank, corpus, calib, tales), tool, directory, `params` JSON, when | who annotated, with what and when. Treebank card, run mode and effort, Opus calls, training files |
| `files` | a loaded file: path, sha1, size, leading blank lines, trailing LF, counters | so that `db export file=…` returns the same file byte for byte, and `verify` can check it |
| `docs` | a document: key, source, title, author, language, license, genre, `meta` JSON | where the text comes from and on what terms |
| `sentences` | document, paragraph, order, `sent_id`, text, comments JSON | one sentence — many annotations |
| `annsets` | an annotation set: layer, source, who, run or treebank (`origin`), dialect, when, how, gate, position in the file, block comments verbatim | provenance of every view of a sentence |
| `tokens` | CoNLL-U rows of a set: ten raw columns, including multiword tokens (`3-4`) and empty nodes (`8.1`) | lossless, any dialect |

**Document:**
- treebank — a block from `# newdoc` to the next one, or the whole file; key `UD_English-GUM:en_gum-ud-dev:GUM_academic_exposure`;
- fairy tale — `gutenberg:2591:hansel-and-gretel`;
- Mova corpus sentences — by `sent_id` prefix: `tatoeba`, `seeds:<seed>`, `papers:<dir>`.

`meta::*` of the first sentence of a document (GUM, fairy tales) give the title, author, genre and license. All of them also go into `docs.meta`.

**Document-level set** (`annsets.sid` = NULL) — for layers that belong to a document: entities, passport, summaries.

**Views:**
- `words` — syntactic words with integer `i` and `h`, without multiword tokens and empty nodes;
- `sets` — sets with run, file, document and sentence names.

**Upper layers** (`docs/context.md`) — placeholder tables, empty for now. Anchor to sentences and words: `sid` + `base` (the ud set whose word IDs are meant) + `w_from..w_to`. A layer set is an `annsets` row with its own `layer`, `source`, `run`, `method`.

| table | layer | main fields |
|---|---|---|
| `entities` | 4, entities | `eid` ("e3", as in the md graph), type as in GUM, name, Number/Gender/Person, animacy, attention weight, `facts` with anchors |
| `mentions` | 4, coreference | mention: `eid`, word span, head, information status, bridging relation (`Bridge=`) |
| `events` | 10, narrative | event: predicate, frame, roles (who, what, when, where, why) JSON, event relations (cause, sequence) |
| `pragmatics` | 6, pragmatics | `s_type`, the act and its form, what was meant, presuppositions, stance, speaker and addressee |
| `passport` | 7, document passport | domain, genre, style, narrator, purpose, mode, addressee, time and place, anchors |
| `summaries` | 10, summaries by level | level (paragraph, chapter, document, book, series), bounds, text, anchors to sentences — a claim without an anchor is not accepted |

## Value dictionary

| field | values |
|---|---|
| `annsets.source` | `gold` (treebank) · `silver` (checked by Opus, gates) · `bronze` (Rust only, gates) · `opus` · `rust` · `udpipe` (raw run output) — CHECK in the schema |
| `annsets.tool` | who annotated: `human` · `opus` · `rust` · `udpipe`. Default — from the source: silver → opus, bronze → rust |
| `annsets.dialect` | `ud-2.18` (everything for now); later `mova` and other dialects from `docs/dialect.md` |
| `annsets.layer` | `ud` (everything for now); later `entities`, `events`, `pragmatics`, `passport`, `summaries` |
| `annsets.origin` | treebank (`UD_English-EWT`), corpus file (`mova-silver-en`), run and stage (`calib-ud-patch300-2026-09-26/final`), run and book (`tales-2026-09-26/en-tales-pg2591-grimm`) |
| `annsets.method` | the block's `# annotation`, if present. Otherwise — the run: for calibration, the mode from `calls.jsonl`, effort (`db/calib-runs.tsv`, otherwise from the name, otherwise "unknown") and the stage. Stages follow `annot` files: `opus-a` — Opus's first view (A), `final` — final, `rust-b` — Rust |
| `annsets.created` | run date from `# annotation` (`…/runs/silver-ud-2026-09-25`), otherwise the file modification time |
| `annsets.gate` | `en` gates: `ok` or the first reason |

**Gates** — the same for all sources:
- tree: `en::convert::tree_gate` — heads in range, one root, `root` only on it, no cycles;
- closed lists of UPOS, PTB, relations and FEATS pairs;
- the UD 2.18 registry for English (`en::udreg`).

"_" means the layer is absent and is not counted as a violation. Lemma "_" skips lemma checks.

Positive control on gold — 0 violations in EWT, GENTLE, PUD, ESLSpok, GUMReddit, LittlePrince, CTeTex, Pronouns and 3 of 14,353 in GUM (`Gender=Fem,Masc`). Red where a treebank has other conventions:
- XPOS is not PTB — LinES, ParTUT (all sentences), CHILDES (20%);
- the old feature `PronType=Int,Rel` — ATIS (33%).

## Loading (`db load`, `db corpus`)

| option | values |
|---|---|
| `--kind` | `treebank` — the card `treebanks/<language>/<code>/README.md` gives license, genres, authors and part sizes; `calib` — an `annot` run directory (`calls.jsonl`, `report.txt`); `corpus`; `tales`; `file` (default) |
| `--docs` | `newdoc` (default) · `idprefix` · `file` |
| `--match` | `new` — every block is a new sentence (treebanks, fairy tales); `auto` — a block with the same `sent_id` and text lands on the existing sentence, otherwise a new one (silver onto bronze); `existing` — only onto existing ones, otherwise an error (calibration on EWT test) |
| `--run`, `--layer`, `--lang`, `--origin`, `--method`, `--license`, `--genre`, `--tool` | override what was inferred |

Rules:
- the same file a second time is skipped (path and sha1);
- a changed file is an error;
- after COMMIT the counters in the database are reconciled with the file.

`db corpus` reads `db/corpus.tsv`. Order matters: texts first (treebanks, corpus), then annotations on existing sentences.

## Lossless

- **Writing.** The file is read by `en::conllu::Doc`, and export assembles the same `Doc` back and writes it with the same `Doc::text()`. Block comments are stored verbatim in `annsets.comments`, rows in `tokens`. Blank lines after a block, at the start of the file, and the trailing LF — in `annsets.blank_after` and `files`.
- **`db verify`** checks for every file:
  - the export equals the file on disk and has the same sha1 as at load time;
  - counters in the database (blocks, rows, words, multiword tokens, empty nodes) match the file;
  - for treebanks, the part size (sentences, tokens, words, fused, empty) matches the card.

  If the file is no longer on disk but the sha1 matches — `NOTE`, not a failure: the database itself reproduces the same bytes.
- **Negative control:**
  - the test `tools/db/tests/roundtrip.rs` on a real database. A corrupted row in `tokens` and a changed file on disk give red, reloading a changed file gives an error, a sentence outside the database with `existing` gives an error;
  - on a copy of the full database (26.09): one changed `deprel` in EWT test gave exactly one `FAIL` and exit code 1.

## Fairy tales (`db tales`)

The five-year-old test (`docs/context.md`). Fairy tales in the public domain in UA, EU and the US, Project Gutenberg, plain text:

| # | book | translation |
|---|---|---|
| 2591 | Grimms' Fairy Tales | Edgar Taylor and Marian Edwardes |
| 1597 | Andersen's Fairy Tales | the text does not name the translator |
| 21 | Three Hundred Aesop's Fables | George Fyler Townsend |
| 2781 | Just So Stories (Kipling) | — |
| 14838 | The Tale of Peter Rabbit (Potter) | — |

**Where from:**
- manifest and working files — `$MOVA_DATA/runs/tales-2026-09-26/` (`tales.jsonl`);
- downloaded by the sources pipeline: `sources plan` → curl → `sources verify`;
- raw files — `$MOVA_DATA/raw/en-tales-*/`.

The license of each book is detailed in the manifest and in `docs.meta`.

**How we split.** The manifest field `tales` says how to split a book:
- `contents` — tales by table of contents: items with the smallest indent, the heading is a separate line between blank lines;
- `ignore` — table-of-contents lines that are not headings;
- `skip` — sections that are not tales: preface, biography, notes, Aesop's index;
- `start` / `end` / `title` — bounds and title for a book without a table of contents;
- `book`, `author`, `translator`, `genre`, `year`.

A heading or marker not found is an error.

**Pipeline:**
1. The PG header and footer and `[Illustration]` are cut off.
2. Paragraphs — from blank lines, lines joined with a space. The paragraph type goes into `# newpar_block`, as in GUM: `p`, `head` (subheading), `lg` (indented verse).
3. Tokens come from `en::tok`. Sentence boundaries — after `. ! ? …` and closing quotes or brackets, unless a lowercase letter follows. Tokens in each paragraph reproduce the text verbatim.
4. UD comes from the Rust annotator `en::annotate` — a shared `en` function, the same path as `rust_annotate` in `annot`. Training — EWT train + ESLSpok train.

   Cross-check: 496 random fairy-tale sentences through `annot bronze` and through `db tales` gave 12,786 words with identical form, lemma, UPOS, XPOS, FEATS, head and relation — 0 differences.
5. Comments:
   - document — `# newdoc id`, `# meta::title/author/translator/book/genre/year/license/source/sourceURL/sha1`;
   - paragraph — `# newpar id`, `# newpar_block`;
   - sentence — `# sent_id`, `# text`, `# source`, `# annotation`. MISC — `SpaceAfter=No`.
6. Files — `$MOVA_DATA/runs/tales-2026-09-26/conllu/<book>.conllu`. In the database: set `ud` / `bronze` / `rust`, dialect `ud-2.18`, run `tales-2026-09-26`.

Sentences that failed the gates stay in the database with the reason in `gate`: 122 of 10,255, 1.2%. Only `gate = 'ok'` counts as bronze for training. The most frequent reasons are Rust errors:
- "cannot" as one token with AUX;
- archaic "hast";
- `’ll` with a curly apostrophe as the lemma;
- a copula other than "be".

| book | tales | paragraphs | sentences | words |
|---|---:|---:|---:|---:|
| Grimms' Fairy Tales | 62 | 907 | 4 263 | 121 850 |
| Andersen's Fairy Tales | 18 | 976 | 2 786 | 67 774 |
| Just So Stories | 12 | 687 | 1 588 | 37 523 |
| Three Hundred Aesop's Fables | 313 | 427 | 1 560 | 39 927 |
| The Tale of Peter Rabbit | 1 | 36 | 58 | 1 148 |
| **total** | **406** | **3 033** | **10 255** | **268 222** |

## What is in the database (26.09.2026)

28 runs, 72 files, 2,680 documents, 109,589 sentences, 113,973 sets, 1,473,155 CoNLL-U rows.

| source | what | sets | sentences | words |
|---|---|---:|---:|---:|
| gold | 13 English UD 2.18 treebanks, 29 files; licenses from the cards (NC — GENTLE, GUM, LinES, ParTUT) | 98 986 | 98 986 | 1 126 448 |
| bronze | `corpus/mova-bronze-en.conllu` (326) + fairy tales (10 255) | 10 581 | 10 581 | 269 894 |
| silver | `corpus/mova-silver-en.conllu`: 326 of 348 landed on bronze sentences | 348 | 348 | 1 982 |
| opus | 12 calibration runs: `opus-a` and `final` | 2 698 | 573 | 33 956 |
| rust | the same runs: `rust-b` | 1 360 | 573 | 17 183 |

- Calibration landed on EWT test sentences: 573 distinct sentences, up to 34 sets per sentence.
- The runs' `udpipe-c.conllu` was not loaded: UDPipe models have an NC license, and the task did not mention it.
- The `corpus/` files are in place; removing them from the repo is a decision to make after checking the database.

## Example queries

`db sql <db> <file>` (markdown output) or `duckdb -readonly $MOVA_DATA/db/mova.duckdb < db/queries/<file>`.

### Where Rust and Opus disagree — `queries/rust-vs-opus.sql`

Pairs of ud sets on the same sentence: Rust (`rust-b`, bronze) versus Opus's final annotation from the same run, or silver. Agreement is the share of words with the same head and relation.

| Opus (final) | pairs | disagree | LAS agreement, % | tags and lemmas, % |
|---|---:|---:|---:|---:|
| mova-silver-en (versus bronze) | 326 | 138 | 86.4 | 95.7 |
| calib-ud-patch-all300-2026-09-26/final | 300 | 186 | 82.1 | 92.6 |
| calib-ud-patch-hints300-2026-09-26/final | 300 | 223 | 82.3 | 92.9 |
| calib-ud-patch300-2026-09-26/final | 300 | 222 | 82.4 | 93.0 |
| calib-ud-high300-2026-09-25/final | 292 | 229 | 77.8 | 91.4 |

Next, the query gives the sentences with the largest tree disagreement: "Police in the Indian capital Delhi say…" — 13 of 26 words, `in 6:case→5:case; the 6:det→5:det; …`.

### All sets of one sentence — `queries/sentence-sets.sql`

The variable `sent` is the sentence's `sent_id`. Example — EWT `weblog-blogspot.com_tacitusproject_…-0007`:

| aid | source | who | run or treebank | gate | when | how annotated |
|---:|---|---|---|---|---|---|
| 58278 | gold | human | UD_English-EWT | ok | 2026-05-06 | UD 2.18: annotation of treebank UD_English-EWT |
| 100060 | opus | opus | calib-ud-patch-all300-2026-09-26/opus-a | ok | 2026-09-26 | patch mode · effort high · Opus's first view |
| 101409 | opus | opus | calib-ud-patch-all300-2026-09-26/final | ok | 2026-09-26 | … · Opus final |
| 102769 | rust | rust | calib-ud-patch-all300-2026-09-26/rust-b | ok | 2026-09-26 | … · Rust annotator en |

| i | form | EWT | final | opus-a | rust-b |
|--:|---|---|---|---|---|
| 1 | Two | 2:nummod NUM | 2:nummod NUM | 2:nummod NUM | 2:nummod NUM |
| 3 | later | 0:root ADV | 0:root ADV | 0:root ADV | **8:advmod ADV** |
| 8 | continues | 3:conj VERB | 3:conj VERB | 3:conj VERB | **0:root VERB** |
| 9 | . | 3:punct PUNCT | 3:punct PUNCT | 3:punct PUNCT | **8:punct PUNCT** |

(rows 2, 4–7 are identical in all)

### Layer statistics by source — `queries/layers-by-source.sql`

| layer | source | who | sets | sentences | documents | rows | words | gates ok, % |
|---|---|---|---:|---:|---:|---:|---:|---:|
| ud | gold | human | 98 986 | 98 986 | 2 117 | 1 150 140 | 1 126 448 | 80.6 |
| ud | bronze | rust | 10 581 | 10 581 | 556 | 269 894 | 269 894 | 98.8 |
| ud | opus | opus | 2 698 | 573 | 221 | 33 956 | 33 956 | 100.0 |
| ud | rust | rust | 1 360 | 573 | 221 | 17 183 | 17 183 | 99.6 |
| ud | silver | opus | 348 | 348 | 157 | 1 982 | 1 982 | 100.0 |

`db stats` also gives tables, runs, document licenses and fairy tales.

## Next

- Upper layers into their own tables: entities and coreference of the fairy tales (GUM `Entity=` is already in the gold `tokens.misc`), pragmatics, passport, summaries with anchors.
- Switch `annot` to `en::annotate` instead of its own `rust_annotate`; at the same time write bronze and silver directly into the database, `db load`.
- Removing a run from the database (`db drop`) — not yet: a rebuild from scratch takes ~2 min.
- `udpipe-c` and intermediate files of silver runs (`silver-*/opus-a`, `rust-b`) — to be decided later.

## Questions and answers (`qa`, migration v3, 26.09.2026)

Training "question — answer" pairs from the LLM per fairy-tale paragraph (`~/Mova/qagen`, command `db qa`). Fields:
- type per FairytaleQA: character, setting, action, feeling, causal relationship, outcome resolution, prediction;
- explicit or inferred answer;
- anchors to the paragraph's sentences;
- source and run.

Pilot `qagen-2026-09-26`: 1,626 pairs over 300 paragraphs. View — `qa_pairs`.

**v2 (migration v4, 26.09.2026):**
- an explicit answer is a verbatim span of the anchor sentence; the gate checks this;
- columns `replaces` and `replaced_by` — corrected pilot pairs stay, with a link to their replacement;
- view `qa_current` — only current pairs.

Current pairs — 7,843 over all 1,440 eligible paragraphs: 5,208 explicit, 2,635 inferred.

## Licence

Elamur Cosmic License ([LICENSE.md](../LICENSE.md)). Third-party data keeps its own terms, as noted where it is used.
