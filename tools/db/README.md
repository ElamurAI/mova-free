# db — the annotation database and its tool (DuckDB)

A DuckDB database for the annotation of Mova's sentences: treebank gold, silver, bronze, model runs, pragmatics and the tales, all over the same sentences. It lives in the data directory, and the whole Mova corpus is loaded into it.

**Where things are:**
- database — `$MOVA_DATA/db/mova.duckdb` (the dev workspace, outside the repo);
- in the repo:
  - schema — `schema.sql`;
  - internal SQL for loading and exporting — `sql/`;
  - example queries — `queries/`;
  - corpus manifest — `data/corpus.tsv`;
  - effort of calibration runs — `data/calib-runs.tsv`;
  - code — `src/` (the `db` command).

**How data flows.** We do not use the duckdb Rust crate: it is a long C++ build. The `duckdb` CLI does the work (variable `DUCKDB`):
- Rust writes TSV, the CLI loads it with `COPY … FROM` in one transaction;
- for export the CLI writes `COPY … TO` into TSV, Rust assembles CoNLL-U.

TSV has no quotes or escapes, NULL is `\N`. A value containing TAB, LF, CR or equal to `\N` cannot be written: this is an error, not silent corruption. CoNLL-U lines have none of these by definition, and JSON is written by serde_json with escapes.

## Commands

```bash
cargo build --release -p db
D=target/release/db
$D init    <db>                         # the whole schema file, migrations included
$D migrate <db>                         # apply schema migrations; idempotent
$D load    <db> <source> <dialect> [--kind K] [--docs newdoc|idprefix|file] [--match new|auto|existing] [--run name] [--layer ud] [--lang en] [--origin O] [--method text] [--license L] [--genre G] [--tool T] <files.conllu>…
$D corpus  <db> [corpus.tsv]            # the whole corpus per data/corpus.tsv
$D tales   <db> <manifest.jsonl> <raw> <run dir> <train.conllu>…
$D export  <db> <key=value>… > out.conllu   # keys: file run source tool layer dialect origin doc sent sid aid gate
$D verify  <db> [file]                  # lossless round trip: export = file
$D stats   <db>
$D sql     <db> <file.sql>              # markdown output
$D prag    <db> <labels.tsv> --run name --source silver|rust [--tool opus|rust] [--origin O] [--method M] [--params params.json] [--path P] [--note N]
$D qa      <db> <qa.tsv> --run name [--source opus] [--tool T] [--params params.json] [--path P] [--note N] [--replaces run]
```

The database is `$MOVA_DATA/db/mova.duckdb`. Rust writes TSV and the DuckDB CLI loads it with `COPY`; the CLI is `~/bin/duckdb` unless `DUCKDB` names another.

## Rebuild from scratch

```sh
cd tools/db && cargo build --release
D=../../target/release/db; B=$MOVA_DATA/db/mova.duckdb; U=$MOVA_DATA/en/ud/ud-treebanks-v2.18; R=$MOVA_DATA/runs/<tales-run>
$D init $B                     # the current schema (v4, the whole schema file with its migrations); does not overwrite an existing database
$D corpus $B                   # treebanks, Mova corpus, calibration — per data/corpus.tsv
$D tales $B $R/tales.jsonl $MOVA_DATA/raw $R $U/UD_English-EWT/en_ewt-ud-train.conllu $U/UD_English-ESLSpok/en_eslspok-ud-train.conllu
$D verify $B                   # lossless: every file byte for byte
$D stats $B                    # overview (sql/stats.sql)
$D sql $B queries/rust-vs-opus.sql
$D export $B doc=gutenberg:2591:hansel-and-gretel > hansel.conllu
```

Other commands:
- `db load <db> <source> <dialect> [options] <files.conllu>…` — one file or several;
- `db export <db> key=value…` — keys `file run source tool layer dialect origin doc sent sid aid gate`;
- `db migrate <db>` — bring an existing database to the current schema;
- `db prag <db> <tsv> --run <run> --source <source> [--tool --origin --method --params --path --note]` — load pragmatics labels into `pragmatics` ([prag/README.md](../../prag/README.md));
- `db qa <db> <qa.tsv> --run <run> …` — load question–answer pairs (see below).

Temporary TSVs go to `$TMPDIR`; `DB_KEEP=1` keeps them for inspection.

## Schema

Idea: different annotations from different sources and different layers lie on the same sentences. Text goes `docs` → `sentences`. Each annotation is a separate set in `annsets` with its own provenance.

| table | what it holds | why |
|---|---|---|
| `runs` | a run or load: name, kind (treebank, corpus, calib, tales), tool, directory, `params` JSON, when | who annotated, with what and when. Treebank card, run mode and effort, the teacher calls, training files |
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

**Upper layers** (`docs/context.md`). `pragmatics` (migration v2, loaded by `db prag`, with the `prag_labels` view) and `qa` (v3 and v4, loaded by `db qa`) are populated; `entities`, `mentions`, `events`, `passport` and `summaries` are placeholder tables, empty for now. Anchor to sentences and words: `sid` + `base` (the ud set whose word IDs are meant) + `w_from..w_to`. A layer set is an `annsets` row with its own `layer`, `source`, `run`, `method`.

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
| `annsets.source` | `gold` (treebank) · `silver` (checked by the teacher, gates) · `bronze` (Rust only, gates) · `opus` · `rust` · `udpipe` (raw run output) — CHECK in the schema |
| `annsets.tool` | who annotated: `human` · `opus` · `rust` · `udpipe`. Default — from the source: silver → opus, bronze → rust |
| `annsets.dialect` | `ud-2.18` (everything for now); later `mova` and other dialects from `docs/dialect.md` |
| `annsets.layer` | `ud` (everything for now); later `entities`, `events`, `pragmatics`, `passport`, `summaries` |
| `annsets.origin` | treebank (`UD_English-EWT`), corpus file (`mova-silver-en`), run and stage (`calib-ud-patch300-2026-09-26/final`), run and book (`tales-2026-09-26/en-tales-pg2591-grimm`) |
| `annsets.method` | the block's `# annotation`, if present. Otherwise — the run: for calibration, the mode from `calls.jsonl`, effort (`data/calib-runs.tsv`, otherwise from the name, otherwise "unknown") and the stage. Stages follow `annot` files: `opus-a` — the teacher's first view (A), `final` — final, `rust-b` — Rust |
| `annsets.created` | run date from `# annotation` (`…/runs/silver-ud-2026-09-25`), otherwise the file modification time |
| `annsets.gate` | `en` gates: `ok` or the first reason |

**Gates** — the same for all sources:
- tree: `en::convert::tree_gate` — heads in range, one root, `root` only on it, no cycles;
- closed lists of UPOS, PTB, relations and FEATS pairs;
- the UD 2.18 registry for English (`en::udreg`).

"_" means the layer is absent and is not counted as a violation. Lemma "_" skips lemma checks.

The gates also run on the gold treebanks as a positive control. They show red where a treebank follows other conventions: XPOS that is not PTB (LinES, ParTUT, CHILDES) or the old feature `PronType=Int,Rel` (ATIS).

## Loading (`db load`, `db corpus`)

| option | values |
|---|---|
| `--kind` | `treebank` — the treebank's row in `treebanks/cards.tsv` gives license, genres, authors and part sizes; `calib` — an `annot` run directory (`calls.jsonl`, `report.txt`); `corpus`; `tales`; `file` (default) |
| `--docs` | `newdoc` (default) · `idprefix` · `file` |
| `--match` | `new` — every block is a new sentence (treebanks, fairy tales); `auto` — a block with the same `sent_id` and text lands on the existing sentence, otherwise a new one (silver onto bronze); `existing` — only onto existing ones, otherwise an error (calibration on EWT test) |
| `--run`, `--layer`, `--lang`, `--origin`, `--method`, `--license`, `--genre`, `--tool` | override what was inferred |

Rules:
- the same file a second time is skipped (path and sha1);
- a changed file is an error;
- after COMMIT the counters in the database are reconciled with the file.

`db corpus` reads `data/corpus.tsv`. Order matters: texts first (treebanks, corpus), then annotations on existing sentences.

## Lossless

- **Writing.** The file is read by `en::conllu::Doc`, and export assembles the same `Doc` back and writes it with the same `Doc::text()`. Block comments are stored verbatim in `annsets.comments`, rows in `tokens`. Blank lines after a block, at the start of the file, and the trailing LF — in `annsets.blank_after` and `files`.
- **`db verify`** checks for every file:
  - the export equals the file on disk and has the same sha1 as at load time;
  - counters in the database (blocks, rows, words, multiword tokens, empty nodes) match the file;
  - for treebanks, the part size (sentences, tokens, words, fused, empty) matches the card.

  If the file is no longer on disk but the sha1 matches — `NOTE`, not a failure: the database itself reproduces the same bytes.
- **Negative control:**
  - the test `tools/db/tests/roundtrip.rs` on a real database. A corrupted row in `tokens` and a changed file on disk give red, reloading a changed file gives an error, a sentence outside the database with `existing` gives an error;
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
- manifest and working files — `$MOVA_DATA/runs/<tales-run>/` (`tales.jsonl`);
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

5. Comments:
   - document — `# newdoc id`, `# meta::title/author/translator/book/genre/year/license/source/sourceURL/sha1`;
   - paragraph — `# newpar id`, `# newpar_block`;
   - sentence — `# sent_id`, `# text`, `# source`, `# annotation`. MISC — `SpaceAfter=No`.
6. Files — `$MOVA_DATA/runs/<tales-run>/conllu/<book>.conllu`. In the database: set `ud` / `bronze` / `rust`, dialect `ud-2.18`, run named after the run directory.

Sentences that failed the gates stay in the database with the reason in `gate`. Only `gate = 'ok'` counts as bronze for training. Typical reasons are Rust errors:
- "cannot" as one token with AUX;
- archaic "hast";
- `’ll` with a curly apostrophe as the lemma;
- a copula other than "be".

## Example queries

`db sql <db> <file>` (markdown output) or `duckdb -readonly $MOVA_DATA/db/mova.duckdb < tools/db/queries/<file>`.

### Where Rust and the teacher disagree — `queries/rust-vs-opus.sql`

Pairs of ud sets on the same sentence: Rust (`rust-b`, bronze) versus the teacher's final annotation from the same run, or silver. Agreement is the share of words with the same head and relation, reported per teacher run with the count of disagreeing pairs and the agreement of tags and lemmas.

Next, the query gives the sentences with the largest tree disagreement, with the differing words (`in 6:case→5:case; the 6:det→5:det; …`).

### All sets of one sentence — `queries/sentence-sets.sql`

The variable `sent` is the sentence's `sent_id`. Example — EWT `weblog-blogspot.com_tacitusproject_…-0007`:

| aid | source | who | run or treebank | gate | when | how annotated |
|---:|---|---|---|---|---|---|
| 58278 | gold | human | UD_English-EWT | ok | 2026-05-06 | UD 2.18: annotation of treebank UD_English-EWT |
| 100060 | opus | opus | calib-ud-patch-all300-2026-09-26/opus-a | ok | 2026-09-26 | patch mode · the teacher's first view |
| 101409 | opus | opus | calib-ud-patch-all300-2026-09-26/final | ok | 2026-09-26 | … · the teacher final |
| 102769 | rust | rust | calib-ud-patch-all300-2026-09-26/rust-b | ok | 2026-09-26 | … · Rust annotator en |

| i | form | EWT | final | opus-a | rust-b |
|--:|---|---|---|---|---|
| 1 | Two | 2:nummod NUM | 2:nummod NUM | 2:nummod NUM | 2:nummod NUM |
| 3 | later | 0:root ADV | 0:root ADV | 0:root ADV | **8:advmod ADV** |
| 8 | continues | 3:conj VERB | 3:conj VERB | 3:conj VERB | **0:root VERB** |
| 9 | . | 3:punct PUNCT | 3:punct PUNCT | 3:punct PUNCT | **8:punct PUNCT** |

(rows 2, 4–7 are identical in all)

### Layer statistics by source — `queries/layers-by-source.sql`

Counts of sets, sentences, documents, rows and words per layer and source, with the share of sets that pass the gates.

`db stats` also gives tables, runs, document licenses and fairy tales.

## Limits

- Most upper layers are not in their tables yet: entities and coreference of the fairy tales (GUM `Entity=` is in the gold `tokens.misc`), events, passport, summaries with anchors. Pragmatics and qa are loaded.
- `annot` does not write bronze and silver into the database directly (`db load` does).
- There is no `db drop` to remove a run: rebuild from scratch.
- `udpipe-c` (UDPipe models have an NC licence) and the intermediate files of silver runs (`silver-*/opus-a`, `rust-b`) are not loaded.

## Questions and answers (`qa`, migrations v3 and v4)

Training "question — answer" pairs from the LLM per fairy-tale paragraph (`qagen/`, command `db qa`). Fields:
- type per FairytaleQA: character, setting, action, feeling, causal relationship, outcome resolution, prediction;
- explicit or inferred answer;
- anchors to the paragraph's sentences;
- source and run.

Rules:
- an explicit answer is a verbatim span of the anchor sentence; the gate checks this;
- columns `replaces` and `replaced_by` keep corrected pairs with a link to their replacement;
- view `qa_pairs` shows every pair, view `qa_current` only the current ones.

## Licence

Elamur Cosmic License ([LICENSE.md](../../LICENSE.md)). Third-party data keeps its own terms, as noted where it is used.
