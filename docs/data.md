# Data, libraries and licences

This page covers where Mova's data lives, the licence rules every source passes through, the open mathematics and science library and its fetch tool, the Wiktionary dumps and their tools, the paper cards and how a paper's ideas reach the code, the manual usage log, the evaluation lists in `eval/`, and the competitions Mova can enter.

## Where data lives

The repository holds code, rules, seeds and small data. Everything large lives outside it, under `$MOVA_DATA`, and so do run results:

| what | where |
|---|---|
| raw downloads (dumps, papers, books) | `$MOVA_DATA/raw/<source>/` |
| catalogs and databases | `$MOVA_DATA/db/` (for example `math.duckdb`) |
| models (`en.bin`, `gparse`) | `$MOVA_DATA/en/models/` |
| run outputs and logs | `$MOVA_DATA/runs/<run>/` |

Every download is recorded in a manifest in the `tools/sources` format, one JSON line per item: `size`, `md5`, `sha256`, `license`, `license_url`, `key`, `content_type`, `fetched`, and for compacted sources also `orig_size`, `orig_sha256`, `text_files`, `dropped_files`. `sources verify DIR MANIFEST` checks size, md5 and signature. Raw catalog pages and XML are kept next to the data, so catalogs can be rebuilt without the network.

## Licence rules

[Data and model licences](../licence/DATA.md) is the one inventory of source terms and release rules, and the saved-model training gate is `en::license::commercial_gate`. Only legal open sources are fetched: no shadow libraries, no lent books, no datasets taken down by DMCA.

Licence classes used by the fetch tools (`raw::License` for arXiv, `books::BookLicense` for textbooks):

| class | licences | training | redistribution |
|---|---|---|---|
| **open** | CC BY 3.0/4.0, CC0 1.0, public domain | yes | yes, with attribution |
| **open, copyleft** | CC BY-SA, GNU FDL | learned from and measured on; the trained model keeps the licence | not shipped |
| **nc** | CC BY-NC-SA, CC BY-NC-ND | no; reading and research only | no |
| **arxiv** | the standard arXiv licence, and pre-2008 papers with no licence field | no; reading and research only | no; only arXiv distributes |

Negative controls in the tests: NC, ND, standard arXiv and unknown licences do not pass, and neither does the build gate.

## Mathematics and science library

An open library of mathematics, statistics, physics and chemistry at university-student level, for the SLM's mathematical skills. Student-level textbooks come first, then papers that teach solving problems: olympiad and competition mathematics, `math.HO`, symbolic computation (`cs.SC`), student-level methods, number theory, combinatorics, algebra, inequalities, geometry and analysis. LaTeX and text are preferred; PDF is taken only where nothing else exists.

### Sources

| source | what it gives | access |
|---|---|---|
| arXiv OAI-PMH (`https://oaipmh.arxiv.org/oai`, `arXivRaw`) | metadata of the 32 `math.*` subsets and `cs.SC`: licence, size per version, dates, categories, MSC, DOI | metadata is CC0 and may be stored and shared |
| `export.arxiv.org/src/<id>vN` | LaTeX sources | targeted picks only; arXiv forbids bulk programmatic download there, and rate-limits it |
| Google Cloud mirror `gs://arxiv-dataset` | one PDF per paper, up to date | anonymous, free; no LaTeX |
| AWS S3 requester-pays `s3://arxiv` | monthly tars of PDF and sources, categories mixed | needs an AWS account; the only legal bulk route to LaTeX |
| AIM Open Textbook Initiative, Open Textbook Library, OpenStax | textbook catalogs | licence per book; the OpenStax API is authoritative over other catalogs |
| OAPEN REST API | open books as PDF and ready text, licence from MARC/ONIX | HTML pages are closed; use the API |
| Internet Archive, publisher sites | individual open textbooks and their LaTeX sources | per book |

Adjacent arXiv categories need no separate harvest: arXiv aliases already place them in mathematics (`math-ph` ≡ `math.MP`, `stat.TH` ≡ `math.ST`, `cs.IT` ≡ `math.IT`, `cs.NA` ≡ `math.NA`). `cs.DM` and `cs.LO` without a mathematics cross-listing are not harvested; `--set cs:cs:DM` adds one.

### Selection rules

- Only the **open** class is downloaded (for textbooks also CC BY-SA and GFDL, as the copyleft class).
- A book whose sources disagree on its licence (one says closed terms, another an open licence) is not downloaded.
- Lab manuals, workbooks, answer keys and companions are filtered out of textbook PDFs; pre-university level (`pre`) is not downloaded. Solutions manuals of strong books may be added by hand (`manual-list.tsv`).
- Major current textbook editions often moved to CC BY-NC-SA; then only the earlier open editions qualify.
- The arXiv extension list (`sql/extend.sql`) takes open papers with sources up to 30 MB, in tiers: problem solving and competitions, `math.HO`, `cs.SC` and student-level methods, `math.GM`, surveys and elementary papers, then the remaining short papers.
- Problem sets with answers for evaluation (GSM8K, SVAMP, AQuA-RAT, MathQA, MMLU mathematics, TheoremQA, ProofNet, Lila, DeepMind Mathematics, open problem books) are checked for licence before use. Competition-derived sets whose compilers do not hold the problems' rights (Omni-MATH, OlympiadBench, AIME collections, NuminaMath) are at most for internal evaluation with a provenance mark, by explicit decision. `hendrycks/competition_math` (MATH) was taken down by DMCA; it and every copy or derivative (MATH-500, PRM800K, MetaMathQA and others) are forbidden. Lila must be checked for MATH problems before use.

### `tools/mathlib`

```sh
B=target/release/mathlib; M=$MOVA_DATA/raw/math; DB=$MOVA_DATA/db/math.duckdb
# 1. metadata, subset by subset (state in state.tsv; a rerun resumes; a STOP file pauses)
$B oai $M/arxiv-oai/math-AG --set math:math:AG        # likewise for every math.* subset and cs:cs:SC; --from DATE for updates
# 2. catalog: arXivRaw -> TSV -> DuckDB; textbooks
$B parse $M/arxiv.tsv $M/arxiv-oai/math-?? $M/arxiv-oai/cs-SC
$B books-fetch $M/textbooks/catalog                    # or otl:<subject> (physics, chemistry ...)
$B books $M/textbooks/catalog $M/books.tsv && $B books-pdf $M/textbooks/catalog $M/books.tsv
$B sql $DB tools/mathlib/sql/load.sql arxiv=$M/arxiv.tsv books=$M/books.tsv "files=$M/*/manifest*.tsv"
$B sql $DB tools/mathlib/sql/stats.sql --mode markdown
# 3. download: list -> polite fetch -> manifest -> verification
$B sql $DB tools/mathlib/sql/books-pick.sql out=$M/books-list.tsv
$B fetch $M/books-list.tsv $M/textbooks $M/textbooks/manifest.jsonl --delay-ms 2000 --max-bytes 2000000000 --expect-pdf --kind textbook
$B sql $DB tools/mathlib/sql/extend.sql out=$M/arxiv-src/list.tsv gcs_out=$M/arxiv-pdf/list.tsv
$B fetch $M/arxiv-src/list.tsv $M/arxiv-src $M/arxiv-src/manifest.jsonl --delay-ms 3500 --compact-src
target/release/sources verify $M/textbooks $M/textbooks/manifest.jsonl
```

How it behaves:

- **Licences.** `fetch` downloads only allowed licences. A list with any other licence is an error before the first request.
- **Politeness.** OAI-PMH: one connection, at least 3 s between requests (the tool pauses 4 s), `Retry-After` respected on 503; 403 or 429 stops. The first request to every host reads its `robots.txt` (RFC 9309: the `*` group or ours, longest match, `*`/`$`, `Crawl-delay` as the per-host pause; 4xx means no restrictions, 5xx closes the host). A 403 or 429 closes the host for the rest of the run. No blind retries.
- **No GitHub.** `robots::github_host` refuses any GitHub host by name or by address (GitHub Pages under custom domains), before any HTTP request; the textbook catalog marks such books `github=true`.
- **Safeguards.** Free disk space is checked before the start and every 10 files (`--min-free-gb`, default 150); `--max-bytes` caps the total and `--file-max` a single file; a `STOP` file next to the manifest pauses before the next file, and a rerun does not download existing files again.
- **Compact sources.** `--compact-src` keeps only text files (`.tex`, `.bbl`, `.bib`, `.sty` ...) from a source archive and drops figures.
- **Log.** Every row is appended to `manifest.log`: ok, miss, bad-signature, cap, stop-disk. The manifest is checkpointed every 100 files.
- `resumptionToken` expires at midnight UTC; on `badResumptionToken` restart that subset. `parse` merges cross-listed duplicates by id, keeping the later `datestamp`.
- `books` adds a `science` column (math, statistics, physics, chemistry, other); `BOOKS_ALL=1` also includes OpenStax science books.

Papers and books never go into git; only the code and this page do.

### Not collected

- LaTeX sources of open mathematics papers in bulk: needs the S3 route (ranged GETs over tar headers, or filtering inside the same AWS region), an explicit decision.
- More open books from OAPEN (Springer Open Access, OBP), catalogued by Thema classification and MARC licence.
- LaTeX sources of textbooks published outside GitHub.
- Physics and chemistry textbooks, statistics papers (JSS, R Journal), CODATA and the PubChem periodic table: their licences are checked and the lists are ready.

## Wiktionary

Wiktionary is Mova's first open dictionary source: all languages, English and Ukrainian first. Everything is built from the raw dumps; ready-made parses (Kaikki, wiktextract) are not used.

**Data.** `$MOVA_DATA/raw/wiktionary/<db>/<run>/` mirrors the `dumps.wikimedia.org` layout, one snapshot run for all editions; provenance, sha1 and reproduction commands are in `SOURCES.md` beside it. The format is `pages-articles-multistream` with an `offset:page_id:title` index. It holds articles, templates (`ns 10`) and Lua modules (`ns 828`): everything needed to expand templates ourselves. Streams hold 100 pages, so a single page can be fetched by title and the dump can be parsed in parallel. Large editions are split into parts, each with its own index. Processing is streaming only; decompressed XML is never written to disk. The licence is CC BY-SA 4.0 plus GFDL, so derived data falls under the CC BY-SA rule above.

**Tools.**

| tool | what it does |
|---|---|
| `tools/wikt-page DIR TITLE...` | prints the raw wikitext of exact titles (with namespace prefix) under a header `=== title \| ns id rev \| file`; unknown titles go to stderr with exit code 1. Output matches `index.php?oldid=<rev>&action=raw` byte for byte |
| `tools/wikt-count` | streaming statistics over a dump read from stdin |
| `tools/wikt-idioms` | extracts idioms, proverbs, phrases, phrasal and multiword verbs, and register- or region-labelled words from an English dump; feeds the expressions base ([pragmatics.md](pragmatics.md)) |
| `tools/wikt-extract` | the English entries of the English edition as the add-on dictionary module `wiktionary-en` (below; [README](../tools/wikt-extract/README.md)): `page DIR TITLE…` prints rows, `build DIR OUT` makes the module, `check OUT` verifies it, `cover LAYER WORDS` reports a word list's coverage |

**Article structure.** Grammar is recorded in template parameters, not in the text, so a parser must read template calls with their parameters and, for inflection, reproduce the logic of templates and Lua modules rather than parse the generated tables.

- English Wiktionary puts every language with a given spelling on one page (`==Ukrainian==`, `==Russian==` ...), then `===Etymology N===`, `====Noun====` and so on. Inflection templates (`{{uk-noun|...}}`, `{{uk-ndecl|...}}`) pass a specification in `<...>` to a Lua module that derives the paradigm from the stressed lemma. Pronunciation, etymology, examples and labels are templates too (`{{uk-IPA}}`, `{{inh}}`, `{{ux}}`, `{{lb}}`). Large articles move translations to a `<word>/translations` subpage (`{{t}}`, `{{t+}}`, `{{tt}}`, `{{tt+}}`, `{{t-check}}`).
- Ukrainian Wiktionary uses one `= {{-uk-}} =` block with `===...===` sections. Inflection follows Zaliznyak's scheme in a noun template (declension type, gender, animacy), with separate templates for morphemes, transcriptions with stress and audio, examples, labels, related words and translations.

The two editions encode the same grammar differently, so cross-checking them is a ready quality control.

**The English module.** `wikt-extract` reads the English entries of the English edition from the raw dump and writes
the add-on module `wiktionary-en` ([modules.md](modules.md)): a dictionary layer in the school layers' six columns
plus labels, synonyms, antonyms, forms, form-of links, template gaps and provenance (page id, revision id, dump file,
stream offset), the list of templates it had no handler for, a report and `module.json`. Each dump stream is decoded
on its own from the index's offsets, on a fixed number of threads; the rows are sorted, so the layer does not depend on
the thread count. Templates are emulated, not executed: a template with no handler leaves its text out and is counted
in the row and in `gaps.tsv`, never dropped silently. The module is CC BY-SA 4.0 and GFDL like its source; it lives in
`$MOVA_DATA/modules/wiktionary-en/<version>/`, is distributed apart from Mova, and the tool refuses to write it inside
a Git work tree.

`wikt-extract check` validates a built module and `cover` reports how much of a word list it covers; extraction
counts, resource use and template-gap reports stay with the run. The layer is sorted, so it does not depend on the
thread count. A dictionary entry or an acronym match does not establish that its sense fits the document at hand. What
the module changes in the pupil: [building the dictionary](training.md#building-the-dictionary).

**Other editions.** The stream reader, the template parser and the row format serve every edition; what differs is
the section and template vocabulary, so each edition needs its own entry reader and template handlers.

## Paper cards

Every paper, book or manual Mova reads gets a short card, so that what the literature says reaches the grammar (`languages/<code>/`) and the code. The cards live in the Research library (its `cards/` folder) and are published on [research.elamur.ai](https://research.elamur.ai/free/en/projects/mova/); this repo keeps only the lists of the works Mova uses: [papers/README.md](../papers/README.md) (the main list, each row linked to its card there) and `papers/measured.tsv` (what came of each measured work), with `papers/aliases.tsv` for merged duplicate cards.

**Where.** Cards Mova already uses in code are at the top of the cards folder. Other cards are in `<code>/<slug>.md`, one folder per language or group: `en` (English and Germanic), `hbo` (Ancient Hebrew), `he` (Modern Hebrew), `arc` (Aramaic, Syriac, Neo-Aramaic), `sem` (general Semitic and Arabic morphology), and the topic folder `mtrans` (machine translation). The slug is the library's: `<venue-or-lang>-<year>-<first author>-<title words>`.

**Format.**

```
# <Title>

**Authors:** ... · **Year:** ... · **Venue:** ...
**Link:** <open URL or library path> · **Licence:** <CC BY 4.0 / public domain / copyrighted — summary only>

## What it is about
Three to six sentences in our own words: the question, the data, the method, the result.

## Ideas
- I1. <one idea: what could be done, and where>
- I2. ...

## For Mova
Where the ideas would apply: a chapter of `languages/<code>/`, a crate or tool, a measure to run.
```

**Rules.**

- Own words only. Quote at most a short phrase, with its page. A copyrighted paper is summarised, never copied.
- An idea is something that can be tried, measured or written into the grammar, not a restatement of the abstract. Ideas are numbered I1, I2 ... within the card.
- A paper with nothing to take still gets a card, with "Ideas: none for Mova" and why.
- The card says what the paper says. Whether an idea worked is recorded where it is tried: in that commit and in the chapter or README it changes.

**How ideas reach the code.** Where code applies a paper's idea, a comment cites the card and idea, for example `(ning-2020 I3)`. The main list names each work's role for Mova (implements, measured, rules, background).

**Hubs.** Pages per concept, per author and per cited card are generated from the library's metadata, citations and cards, never stored in this repo: research.elamur.ai publishes them, and an internal tool can write them as Markdown into a build output. Two runs on the same inputs give the same bytes.

## The usage log

[usage.tsv](usage.tsv) is the manual log of where papers and books are applied. It holds only what the repository text does not show as a reference: a work mentioned only by title or by author and year, a method without a reference, reading that influenced a decision. References already in the text (ACL ids, `papers/...` and `books/...` paths, aliases from `tools/papers/aliases.tsv`, "author year" with a work in the library) are collected automatically and do not belong there.

Columns, tab-separated: `date` (YYYY-MM-DD), `work` (ACL id, library folder, or "outside the library: author year title"), `where` (repo file, optionally `#section`), `how` (rule source, knowledge source, bar or metric, method, data, format, precedent, review, mention), `who`, `note`. A bad row (unknown id, folder, file or section; bad date, `how` or `who`) stops the collector before any output is written. The log feeds back-citations, the bibliography and system papers.

## Evaluation lists

[eval/](eval/) holds small lists that qualify an evaluation, such as `stepgame-excluded.tsv`: StepGame test questions whose story contains a statement contradictory in the training data, so a clean score can leave them out. The benchmark files themselves stay outside the repository under their own licences.

## Competitions

Mova enters linguistic shared tasks where possible. Candidates, each with papers in the library:

| competition | what it measures | where Mova fits |
|---|---|---|
| **CRAC** (CorefUD) | multilingual coreference, with an LLM track | context and pronouns, the story layer; SLM plus a large model |
| **UniDive** | morphosyntactic parsing without the notion of "word" | the metadialect, converters ([dialect.md](dialect.md)) |
| **PARSEME 2.0** | multiword expressions over UD | expressions in seeds and domain snapshots |
| **DISRPT** | discourse units and relations | the story layer, pragmatics |
| **SemEval** | new tasks each year (sarcasm, abductive events ...) | pragmatics of what was meant |
| **BabyLM** | a small model on limited data, including Gricean pragmatics | the SLM learner |

Rules that concern Mova:

- The **closed** track accepts only the competition data; the **open** or LLM track accepts any resources. The SLM alone goes to the closed track; the SLM with a large model goes to the open one.
- Parsing is measured with the CoNLL 2018 script (LAS, MLAS, BLEX), so scores are comparable.
- Competition data is for research and sometimes has NC parts. The competition build is separate and never enters the production model; `commercial_gate` stops it.
- A system paper is usually required; it is written from the usage log.
- Calendars and deadlines are checked when the time comes.
