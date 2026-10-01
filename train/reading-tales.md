# Reading tales: `world read*`, `reader-*`, and `qagen`

## Gist

Mova reads public-domain tales and answers questions about them without a neural network. Every step is a
deterministic Rust rule or a small linear model whose weights you can inspect:

- `world read`: splits a Project Gutenberg book into stories and paragraphs, then writes a skeleton summary
  of each paragraph from the UD parse (actor, action, object, recipient, place, negation; for speech, only
  "X says").
- `world read-qa`, `read-graph`, `read-events`, `read-feel`: answer FairytaleQA questions from the raw story
  text, then score them with ROUGE-L against a "whole retrieved sentence" baseline.
- `world read-qgen` → `read-short` → `read-train`: an LLM writes many short-answer questions per paragraph,
  once. Scoring them, and training the sentence **retriever** and answer **extractor**, needs no LLM after that.
- `world reader-*`: the question reader v2 over a story world. Its evaluation has negative controls.
- `qagen`: generates FairytaleQA-style training questions per paragraph through an LLM, with strict gates and
  a rejection log.

LLM steps are optional. They call an external LLM command-line client and are marked **[LLM]** below.
Everything else runs offline.

## When to use

- You want a reading-comprehension baseline that explains each answer ("what → object", "why → cause or
  purpose").
- You want cheap training and checking data: answers of at most 3 words, verbatim in the text, scored by
  exact match and F1.
- You are testing a new answering idea and need an honest comparison: train on one split, test on another,
  run negative controls, and run twice to check determinism.

## Setup

The crates are **not** a Cargo workspace. Each crate under `ai/` has its own `Cargo.toml` and `Cargo.lock`,
so run each one through `--manifest-path`:

```sh
cargo run --release --manifest-path ai/world/Cargo.toml -- <command> ...
cargo run --release --manifest-path ai/qagen/Cargo.toml -- <command> ...
```

The examples below shorten this to `world <command>` and `qagen <command>`. `world` depends on `ai/prag`,
`ai/en`, `ai/math` and `ai/global` by path. `qagen` depends on `ai/prag` and `ai/en`.

**`en` model (required for every parse).** The readers parse sentences with the `en` annotator. Train one
on UD English treebanks (we use UD English EWT + ESLSpok train, CC BY-SA 4.0):

```sh
cargo run --release --manifest-path ai/en/Cargo.toml -- \
  train <UD_English-EWT/en_ewt-ud-train.conllu> <UD_English-ESLSpok/...-train.conllu> -- models/en.bin
# optional: the shipped model ai/en/models/ud-ewt-eslspok.bin is used by default
export WORLD_EN_MODEL=models/en.bin     # world
export QAGEN_EN_MODEL=models/en.bin     # qagen gate (rule 2) and qagen fix
```

If you skip these variables, the code falls back to a default path that probably does not exist on your
machine. Always set them.

**Environment variables**

| variable | used by | meaning |
|---|---|---|
| `WORLD_EN_MODEL` | world | path to the `en` model |
| `QAGEN_EN_MODEL` | qagen | path to the `en` model, for lemmas |
| `FTQA` | world `read-qa`, `read-graph`, `read-events`, `read-feel`, `reader-*`, `ftqa-*` | FairytaleQA directory (test split) with `stories/` and `questions/` |
| `PRAG_MODEL`, `PRAG_EFFORT` | every [LLM] step | model name and effort passed to the LLM client (default effort `medium`) |
| `DUCKDB` | qagen `select` | path to the `duckdb` CLI binary |
| `READ_SPANS=1` | `read-train` | extractor candidates become all 1–3-word spans instead of syntactic groups |
| `READ_NB` (0.3), `READ_NEXT` | `read-qa` and friends | weight of neighbouring sentences; `READ_NEXT=0` disables the "next sentence" rule |

Metric names are the usual ones: EM (exact match), F1, ROUGE-L. On the result lines, "snake" is the model,
"sentence" (or "whole sentence") is the whole-sentence baseline, "rules" is the rule-based extractor,
"learned choice" is the learned extractor, and "learned search" is the learned retriever.

## Data

None of the data below ships in the repo. Every path is a placeholder for your own working directory.

- **Books.** Plain-text Project Gutenberg files (public domain). For example: Grimm's Fairy Tales (#2591),
  Andersen (#1597), Aesop's Fables (#21), Lang's Blue Fairy Book. Download the "Plain Text UTF-8" version.
- **FairytaleQA.** The Hugging Face dataset `WorkInTheDark/FairytaleQA` (Apache-2.0 per its card). Use it
  only for evaluation, never for training. Export the test split and the validation split as two separate
  CSV files.

## Data formats (exact, from the readers in code)

**Gutenberg book `.txt`** (`world::read::split_book`):
- Only the text between the `*** START` and `*** END` lines is read.
- A story title is a line after a blank line that is either:
  - all capitals, at least 5 letters and under 70 characters; or
  - indented by 3 or more spaces, capitalised, at most 10 words, with no trailing `.,;:`, and followed by
    a blank line.
- Paragraphs are separated by blank lines.
- Stories with fewer than 200 characters of text are dropped. This also drops a table of contents.

**`<book-dir>/summaries.jsonl`**: one JSON object per paragraph, written by `world read`:

```json
{"book":"grimm","story":0,"title":"THE GOLDEN BIRD","par":0,"text":"<paragraph>","summary":"<skeleton; skeleton>"}
```

`book` is the file stem, and `story` and `par` count from 0. Downstream commands group paragraphs by
`title`, so `title` is the story key.

**`<qa-dir>/qa.jsonl`**: short questions. `read-qgen` writes this file; `read-short` and `read-train`
read it:

```json
{"story":"THE GOLDEN BIRD","para":3,"multi":false,"question":"What did the king own?","answer":"a garden"}
```

- `story` must equal a `title` in `summaries.jsonl`.
- For a multi-paragraph question, `para` is `-1` and `multi` is `true`.
- The answer must be 1–3 words, copied verbatim from the text.

**`<qa-dir>/sets.json`** and **`queue.jsonl`**: written by `read-short` and updated by `read-classify`.
- `sets.json` maps `"<story>|<question>"` to `[[correct answers…],[wrong answers…]]`.
- `queue.jsonl` holds unknown model answers: `{"key","question","gold","answer"}`.

**`check.jsonl`** (`read-check`): `{"title","par","faithful":0-2,"coverage":0-2,"missing"}`.

**`calls.jsonl`**: one line per LLM call, written by every [LLM] step. It records tokens, seconds and
estimated cost.

**FairytaleQA CSV** (`world::ftqa::read_rows`):
- Comma-separated, RFC 4180 quoting.
- The header must be exactly
  `story_name,story_section,question,answer1,answer2,local_or_sum,attribute,ex_or_im,ex_or_im2`.
- `local_or_sum` must be `local` or `summary`.

**`$FTQA/stories/<name>.tsv`**: written by `ftqa-import`, read by `world::ftqa::read_story`:

```
# sec	sent	text
1	1	a hungry fox saw some grapes on a vine .
1	2	he could not reach them .
2	3	so he walked away and said they were sour .
```

- There are exactly 3 columns.
- `sent` must count 1, 2, 3, … with no gaps; any other value stops the reader with an error.
- `sec` is the FairytaleQA section number.
- Lines that start with `#` are skipped.

**`$FTQA/questions/<name>.tsv`** (`world::ftqa::read_qa`): exactly 9 tab-separated columns:

```
# n	attr	ex	ex2	local	secs	question	answer1	answer2
1	action	explicit		local	1	what did the fox see ?	some grapes
```

`secs` is a comma-separated list of section numbers.

**Directory layout for the FTQA commands.** Each command reads `stories/*.tsv` in sorted order and the
matching file in `questions/`. A story without a questions file is skipped.

## Steps

### 1. Read a book and summarise paragraphs

```sh
world read work/books/grimm.txt work/grimm [--limit <stories>]
# → work/grimm/summaries.jsonl
world read-check work/grimm [--limit <paragraphs>]        # [LLM] rates faithfulness and coverage 0–2, 15 per call
# → work/grimm/check.jsonl, calls.jsonl
```

### 2. Answer FairytaleQA from raw text (no LLM)

```sh
world ftqa-import work/ftqa-test.csv work/ftqa          # CSV → stories/, questions/; prints a summary table
world ftqa-import work/ftqa-valid.csv work/ftqa-valid
FTQA=work/ftqa world read-qa --show 5                   # sentence retrieval + role by question type; ROUGE-L vs whole sentence
FTQA=work/ftqa world read-graph                         # tunes context-graph weights on valid, measures on test
FTQA=work/ftqa world read-events                        # learns a hop policy per question type on valid, tests on test
FTQA=work/ftqa world read-feel                          # explicit or inferred feeling words, valid and test
```

`read-graph`, `read-events` and `read-feel` take no arguments:
- the **test** split comes from `FTQA`;
- the **validation** split comes from `$MOVA_DATA/raw/fairytaleqa-valid` (`MOVA_DATA` defaults to `data` in
  the current folder).

### 3. Short questions, retriever and extractor

```sh
world read-qgen work/grimm work/grimm-qa [--stories 20]   # [LLM] 8 paragraphs per call; resumable
world read-short work/grimm work/grimm-qa [--show 10]     # EM and F1 with no LLM; writes sets.json, queue.jsonl
world read-classify work/grimm-qa [--limit 200]           # [LLM] optional: judge unknown answers, 40 per call
world read-train work/grimm work/grimm-qa work/aesop work/aesop-qa [...]   # pairs: <book-dir> <qa-dir>
```

`read-qgen`:
- asks for 4–6 questions per paragraph, plus 3–5 questions that span paragraphs;
- keeps a question only if its answer is 1–3 words and appears verbatim in the paragraph (for a
  multi-paragraph question, in the batch of paragraphs);
- appends each batch to `qa.jsonl` as soon as it arrives, and skips stories that are already done when you
  rerun it.

`read-train`:
- **Inputs.** Pairs of directories: a book directory (with `summaries.jsonl`) and its QA directory (with
  `qa.jsonl`). A trailing unpaired argument is ignored.
- **Split.** By story: story IDs (`<book-dir>|<title>`) are sorted, and every 5th story is test. The test set
  contains only tales the model never saw.
- **Extractor.** An averaged perceptron, 15 epochs. It chooses among candidate answers in the chosen
  sentence: noun groups, heads, "X of Y", prepositional adverbials and verb groups. With `READ_SPANS=1`, the
  candidates are all spans of 1–3 words. Features combine question type with syntactic role, part of speech,
  overlap with the question and distance to the verb.
- **Retriever.** An averaged perceptron, 10 epochs, over the 15 sentences that share the most words with the
  question, plus the sentence of the best event. A sentence is labelled positive if it contains the gold
  answer. Features:
  - IDF-weighted word overlap, including with the previous and next sentence;
  - unmatched question words;
  - whether the question's verb or entities appear;
  - question type × whether the sentence has a number, a place or a quote;
  - sentence length.
- **Pair trigger.** For multi-paragraph questions, a "bridge" sentence is added. Its threshold is picked on
  train.
- **Outputs.** Metric lines on stdout only: rules vs learned extractor, retriever hit rate, retriever +
  extractor, pair trigger (useful vs harmful firings), and multi-paragraph results.
- **No model file is written.** The weights live in memory and are retrained on every run, which takes
  seconds to minutes.
- **Memory.** Memory grows with the number of questions. At the time of writing, 6 books with about 12k
  questions peaked at about 5.7 GB, so run large jobs under a memory limit.

### 4. Question reader v2 over a story world (optional, needs world files)

```sh
world reader-parse work/dev-questions.tsv              # lines: type<TAB>explicit|implicit<TAB>question → UD question frame
world reader-span-dev work/ftqa-valid [--show <type>]  # sentence spans on valid, without a world
world reader-ask <name> work/worlds <n> [--facts]      # debug one question
world reader-eval work/worlds work/out [--dev q.tsv] [--skip q.tsv] [--control shuffle|noindex] \
                  [--sample k --seed s --exclude seen.tsv ...]
```

`work/worlds/` holds:
- `tales.txt`, with one story name per line;
- `cmds-<name>.txt`, the world-change commands for each story. An LLM writes these through `world ftqa-vmm
  <name> <dir>` **[LLM]**.

Questions come from `$FTQA/questions/<name>.tsv`. Each run writes these files into `work/out/`:
- `answers<tag>.tsv`;
- `report<tag>.md`;
- `dev<tag>.txt`, only with `--dev`;
- `manual-sample.tsv`, on the first `--sample` run. Fill it in as `manual-verdicts.tsv` and rerun to get
  the manual table. Its columns are `story n v2 v1 base state comment`, with scores of 1, 0.5 or 0.

The question files for `--dev`, `--skip` and `--exclude` are TSV files of `story<TAB>n`.

### 5. `qagen`: LLM training questions with gates

```sh
qagen select <db.duckdb> work/qg [--paras 300] [--batch 20] [--seed 26] [--rule 2] [--exclude other/paras.jsonl]
qagen prompt work/qg 0                     # show the prompt for batch 0, no call
qagen run    work/qg [--batches 0,1] [--jobs 5] [--max-calls 20]    # [LLM] → raw/bNNN.txt, calls.jsonl
qagen gate   work/qg                       # → qa.tsv, rejects.tsv, params.json (JSON summary on stdout)
qagen sample work/qg [--n 20] [--seed 7] [--weak 0.5]   # → sample.md, or weak.md with --weak
```

**Database.** `select` needs a DuckDB database with tables `docs(doc, key, title, author, license)` and
`sentences(doc, para, ord, sent_id, text, comments)`. The exact query is `ai/qagen/sql/paras.sql`.
- Document keys must look like `gutenberg:<book>:<tale>`.
- Every `license` must start with `public domain`, or the run stops.
- `comments.newpar_block` gives the block type: `p`, `lg` or `head`. Only `p` (prose) is eligible.
- An eligible paragraph also has 2–16 sentences and 40–250 words.

**Other entry point.** You can also skip `select` and write `paras.jsonl` and `pick.json` yourself:

```json
{"doc":"gutenberg:21:the-fox","book":"gutenberg:21","title":"The Fox","author":"Aesop","para":1,"block":"p",
 "sents":[{"sent_id":"gutenberg:21:the-fox:1","text":"A hungry fox saw some grapes on a vine."},
          {"sent_id":"gutenberg:21:the-fox:2","text":"He could not reach them, so he walked away."}],
 "dialogue":false,"words":19,"batch":0}
```

Each `paras.jsonl` record must be on one line; the example above is wrapped for reading.

```json
{"paras":1,"batch":20,"seed":26,"min_words":40,"max_words":250,"min_sents":2,"max_sents":16,"rule":2}
```

**LLM output format.** The LLM's raw output (`raw/bNNN.txt`, or `bNNN.retry.txt` after the single retry
on a format failure) is TSV with 6 columns:

```
p	type	ex	anchors	question	answer
```

- `type` is one of the 7 FairytaleQA attributes.
- `ex` is `explicit` or `implicit`.
- `anchors` holds the numbers of sentences within the paragraph.

**Rule 2.** An explicit answer must be a verbatim span of an anchor sentence. An implicit answer that
mostly reuses one anchor's lemmas is flagged `doubt`.

**Output files.**
- `qa.tsv`: `doc_key para k qtype ex_or_im anchors question answer`. Anchors are `sent_id`s.
- `rejects.tsv`: `batch doc_key para level reason line`. Each line logs the first reason only.

**Run behaviour.**
- A call error stops the run (fail-fast).
- The only retry is one retry on a format failure.
- A rerun calls only the batches that are still missing.
- `--max-calls` caps the total calls per run directory.

**`qagen fix`.** These subcommands repair answers in an older rule-1 run. See `ai/qagen/src/main.rs`.

`qagen` writes `qa.tsv`, which is a different format from the `qa.jsonl` that `read-train` reads. There is
no converter between them at the time of writing.

## Example

A tiny offline round trip. All files are hand-made, and no LLM is involved.

```sh
mkdir -p work/qa
cat > work/book.txt <<'EOF'
*** START OF THE PROJECT GUTENBERG EBOOK TINY TALES ***

THE DOG AND THE BONE

A dog was carrying a bone in his mouth across a bridge. He looked down and saw his reflection in the water of the river.

He thought it was another dog with a bigger bone. He opened his mouth to bark, and his own bone fell into the river.

*** END OF THE PROJECT GUTENBERG EBOOK TINY TALES ***
EOF
world read work/book.txt work/book
cat > work/qa/qa.jsonl <<'EOF'
{"story":"THE DOG AND THE BONE","para":0,"multi":false,"question":"What was the dog carrying?","answer":"a bone"}
{"story":"THE DOG AND THE BONE","para":1,"multi":false,"question":"Where did the bone fall?","answer":"into the river"}
EOF
world read-short work/book work/qa --show 2
```

On a similar tiny input, the model answered "a bone" (F1 1.00) and "the river" (F1 0.67 against "into the
river"). It printed EM and F1 per question type, and queued "the river" in `queue.jsonl` as an unknown
alternative answer. For a fox fable, a `summaries.jsonl` summary reads like `hungry fox saw fine bunches;
vine trained along high trellis`.

**Gate demo.** Write a `qagen` raw line whose explicit answer is not in the anchor, for example `a hungry
wolf` for "Who was hungry?". `qagen gate` keeps that line out of `qa.tsv` and logs it in `rejects.tsv` at
level `pair`, with the reason "not a span of an anchor sentence". It also adds a `note` when a paragraph has
fewer than 3 accepted questions. That is the gate showing red.

## Evaluating honestly

- **Split before you tune.** FairytaleQA is test-only. Tune weights and policies on the validation split
  (`read-graph`, `read-events`), then measure on test once. `read-train` splits by story, so test tales are
  unseen.
- **Always report a baseline next to the model.** Every FTQA command prints the "whole retrieved sentence"
  ROUGE-L next to the model's score, and `read-train` prints rules next to learned. ROUGE-L rewards long
  answers, so the whole-sentence baseline is hard to beat.
- **Use negative controls.**
  - `reader-eval --control shuffle` answers from another tale's world, so its score should collapse.
  - `--control noindex` removes the fact index, so its score should fall to the baseline.
  - A gate that cannot reject anything is not a gate. Feed it a known-bad line, as in the demo above.
- **Freeze your dev questions.** Develop with `--dev q.tsv`, then report with `--skip q.tsv` on questions
  development never saw. The output files get a `-clean` suffix.
- **Check by hand on a fresh sample.** `--sample k --seed s --exclude <earlier samples>` draws a sample per
  question type, deterministically by hash. Fill in `manual-verdicts.tsv` and rerun. Automatic metrics and
  manual accuracy often disagree.
- **Keep the LLM out of scoring.** Short verbatim answers allow EM and F1 with no LLM in the loop.
  `read-classify` only grows the set of accepted alternatives; keep it separate from the headline number.
- **Determinism.** Run the same command twice and `diff` the output before you claim any gain. The
  `read-qa` and `read-train` paths produced byte-identical output on repeat runs in our checks. Any new
  code that iterates over a `HashMap` while deciding something can break this; use `BTreeMap`.

Results at the time of writing, from committed project notes. Treat them as orientation, not as
benchmarks.

| command | setup | result |
|---|---|---|
| `read-qa` | FairytaleQA, 1007 questions | ROUGE-L 0.181, below the whole-sentence baseline at 0.192 |
| `read-events` | FairytaleQA test | 0.199 vs 0.192; adding `read-feel` feelings gives 0.212 |
| `read-train` | 4 books, 750 test questions | rules EM 16.0% / F1 0.216; retriever + extractor EM 24.1% / F1 0.296 |
| reader v2 | 305 held-out questions | ROUGE-L 0.294 vs baseline 0.229 |

Reader v2 controls: with the shuffled world, the state-only answers scored about 0.003; with no index, the
score equalled the baseline.

## Pitfalls

- If the model file is missing (default `ai/en/models/ud-ewt-eslspok.bin`, or the path in `WORLD_EN_MODEL` /
  `QAGEN_EN_MODEL`), the run fails with "en model …".
- `read-short` and `read-train` match `qa.jsonl` to stories by **title**. If you edit a title or re-split
  the book, questions silently drop out.
- `read-qgen --stories N` takes the first N stories in book order, minus those already done. To regenerate a
  story, delete its lines from `qa.jsonl`.
- `qagen select` refuses to run on a directory that already has `raw/b000.txt`, because a new selection
  would shift batches. Use a new run directory.
- `read-graph`, `read-events` and `read-feel` read the validation split from `$MOVA_DATA/raw/fairytaleqa-valid`.
  See step 2.
- `read-train` writes no model. To compare variants (for example `READ_SPANS=1`), run both twice and compare
  the printed lines.
- Every [LLM] step costs tokens. Check `calls.jsonl` and use the caps (`--max-calls`, `--limit`,
  `--stories`) before a large run.
