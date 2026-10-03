# Word problems: exact core, step solver, world scripts, world reader, SVAMP / GSM8K

Crates: `ai/math` (exact core, step solver `mathsolve`, world reader `qread`) and `ai/world` (world scripts: format, checker, optional LLM writer). The crates do **not** form a Cargo workspace. Each has its own `Cargo.lock`, so pass `--manifest-path`. `ai/math` has three binaries (`math`, `mathsolve`, `vmmtests`) and no `default-run`, so `--bin` is required:

```sh
cargo run --release --manifest-path ai/math/Cargo.toml --bin mathsolve -- <cmd> ...
cargo run --release --manifest-path ai/math/Cargo.toml --bin math      -- <cmd> ...
cargo run --release --manifest-path ai/world/Cargo.toml                -- <cmd> ...
```

---

## 0. Setup (read once)

### Gist
Three things to know before running anything beyond `calc` and `controls`.

### Steps
1. **Parser model.** `mathsolve` (`steps`, `qread`, `qread-text`) loads the `en` UD model shipped in `ai/en/models/ud-ewt-eslspok.bin` (the path is fixed at build time). To use your own model, train it with `en train` (see retrain-the-parser.md) and replace that file, or use `MATH_EN_MODEL` with the `math` binary.
2. **Run directory.** Every `mathsolve` command creates its run directory at startup: `MATH2_RUN`, or `$MOVA_DATA/runs/math2-2026-09-26`. Set it explicitly:
   ```sh
   export MATH2_RUN=runs/math2
   ```
3. **Dataset root.** `mathsolve steps|qread` and `world quant-*` read datasets from `$MOVA_DATA/raw` (`MOVA_DATA` defaults to `data` in the current folder). Call that directory `<DATA_ROOT>` below. The expected files are listed in section 6. The world scripts come with the corpus: `train/get-corpus.sh` puts them in `corpus/math-scripts/`.

### Pitfalls
- If the model file is missing, `steps` and `qread` fail at load time. This is not a data problem.
- `mathsolve steps --set tiny` also reads a TinyGSM sample from `$MOVA_DATA/raw/mathsolve/tinygsm/`. It is optional; skip it unless you have that data.

---

## 1. Exact core

### Gist
All arithmetic is exact:
- **v1** (`ai/math/src/rat.rs`, i128) for units and temperatures. An overflow is an explicit `Overflow` error, never a silent f64.
- **v2** (`ai/math/src/big.rs`) for word problems: arbitrary-size integers and rationals via `dashu` (`Q = RBig`, always reduced).

`calc.rs` evaluates expressions on v2. Every result gets an **independent check**: the same expression computed with different arithmetic (f64, or a fingerprint modulo 2^61−1 for huge integers). A failed check blocks the answer.

### When to use
- Checking a number or formula.
- Debugging why a solver step produced a value.
- Running the gate's negative controls after touching the core.

### Steps
```sh
# expression -> exact value + independent check
cargo run --release --manifest-path ai/math/Cargo.toml --bin mathsolve -- calc "3^40 + 17" "(16 - 3 - 4) * 2"

# negative controls: each must be RED (caught); exits non-zero if any passes
cargo run --release --manifest-path ai/math/Cargo.toml --bin mathsolve -- controls

# English question -> expression tree -> steps, checks, answer (units, %, stats, probability)
cargo run --release --manifest-path ai/math/Cargo.toml --bin math -- eval "What is 15% of 80?"

# frozen suite: id <TAB> question <TAB> expected
cargo run --release --manifest-path ai/math/Cargo.toml --bin math -- test ai/math/data/suite-v1.tsv
```

### Example
```
expr:    3^40+17
  ✓ independent path: fingerprint modulo 2^61−1 fingerprint 628450412988459063
answer: 12157665459056928818
```

`controls` ends with `negative controls red: 7 of 7`.

In `math test`, the "expected" column takes one of:
- a value with an optional unit (`12`, `≈ 22.2222 °C`, `50/9 Δ°C`);
- `NU` (an honest "I don't understand");
- `ERR:dim`, `ERR:temp`, `ERR:overflow` or `ERR:div0`.

The summary line `# TOTAL <n> <hit> <honest NU> <wrong>` gives the totals.

### Pitfalls
- Round only on output. A non-terminating decimal prints as `≈ … (exactly a/b)`.
- Temperature is affine: point − point = difference, and point + point is an error with an explanation.
- Don't edit `suite-v1.tsv` to make it pass. A new suite version goes in a new file.

---

## 2. Step solver: `mathsolve steps`

### Gist
A transition system reads the numbers in text order and, at each one, chooses SHIFT, SKIP or REDUCE (op + direction). The features come from the `en` UD tree (unit noun, verb class, subject, cue words, question words). The model is a structured perceptron with beam search and early update. The exact core computes every subtree.

With `--exact`, the gold trees come from SVAMP equations (`( 290.0 / 2.0 )`) or GSM8K calculator steps (`<<48/2=24>>`). Without it, the gold trees are guessed by weak supervision. Training happens in memory on every run; no model file is written.

### When to use
- Training and evaluating the word-problem solver on SVAMP or GSM8K.
- Trying a feature or idea as a flag and measuring it against the baseline.

### Steps
The flags come from the `"steps"` arm of `ai/math/src/bin/mathsolve.rs`. Defaults are in brackets.

| flag | meaning |
|---|---|
| `--set gsm8k\|svamp\|mix\|tiny` [gsm8k] | `svamp`: train on SVAMP train, test on SVAMP test. `gsm8k`: train on GSM8K socratic train, test on its test split. `mix`: train on SVAMP + GSM8K, test on SVAMP. `tiny`: train on GSM8K + a TinyGSM sample (`--tiny N` [20000]), test on GSM8K |
| `--exact` | exact gold trees from the annotation (recommended; needed by `--world`, `--princ`, `--qread`, `--arith-feat`) |
| `--train N` [all] | cap on training items |
| `--epochs E` [5] / `--beam B` [16] / `--maxq Q` [9] | perceptron epochs, beam width, max quantities per problem |
| `--rerank K` [0] | train a judge over the top-K candidates. Its training candidates come from models trained without their own fold (4 folds) |
| `--world <scripts.jsonl>` | train the world writer on verified world scripts (section 4) |
| `--world-feat` | use the learned world's answer as a judge feature (with `--world`) |
| `--qread` | use the world reader's answer as a judge feature. It only counts when every number was read (section 5) |
| `--princ <scripts.jsonl>` | sentence-principle classifier from scripts (5-fold cross-check printed) |
| `--dag` | v5 DAG model (trees with repeated numbers; text order not needed) |
| `--augment`, `--curriculum`, `--star N`, `--shapes`, `--arith-feat`, `--bag`, `--bag-judge`, `--rethink` | experimental variants (data augmentation, curriculum, self-training rounds, shape library, ensembles, second pass with a wider beam) |
| `--show N`, `--explain N` | print the first N test items with their steps / a full explanation |
| `--hints N` | **calls an LLM via a CLI** for up to N disagreement cases, with a small spend cap. Optional; leave it off for local runs |

Environment variables:
- `MATH_NOLEX=1|2`: non-lexical features (used by the benchmark command below).
- `MATH_IGNORE=0..4`: the "ignore extra numbers" trigger.
- `MATH_DUMP=<file>`: per-item dump, one `id<TAB>ok(0/1)<TAB>value<TAB>tree` line per item.
- `MATH_SHOW_MISS=N`: show N items where the right answer was ranked but not chosen.
- `MATH_HUMBLE=<th>` [0.7]: confidence threshold for the self-assessment.
- `MATH_UNITHARD`, `MATH_MAXVIOL`, `MATH_IMPLIED`: experiments.

### Example
The configuration used as the benchmark baseline:
```sh
export MATH2_RUN=runs/math2
MATH_NOLEX=1 cargo run --release --manifest-path ai/math/Cargo.toml --bin mathsolve -- \
  steps --set svamp --exact --epochs 20 --beam 32 --rerank 5 --world corpus/math-scripts/scripts.jsonl

# GSM8K, same settings without --world
MATH_NOLEX=1 cargo run --release --manifest-path ai/math/Cargo.toml --bin mathsolve -- \
  steps --set gsm8k --exact --epochs 20 --beam 32 --rerank 5
```
Add `--qread` to the SVAMP command to use the world reader as a judge feature.

What the output reports (in order):
1. `<set> (exact gold): training with gold N; no tree …; no exact tree X; tree not in text order Y`. This is the training data and the **excluded items** with their reasons: no exact tree, or a tree not buildable in text order.
2. `epoch k: correct on training a/N`: training accuracy per epoch.
3. `<set> test: SLM alone (v4) correct acc/n (p%)`: **the headline test accuracy**.
4. `top-K: correct among them …`: how often the right answer is among the top-K candidates.
5. Self-assessment:
   - accuracy when only answering above a confidence threshold (`threshold`);
   - accuracy when the first model and the judge agree (`opinions agree`);
   - accuracy for the most confident 10/20/30/50% (`most confident`);
   - trigger counts, e.g. `v2: answered — correct / wrong` and `world: …` (world agreement).

### Pitfalls
- **Determinism first.** Run the same command twice and diff the `MATH_DUMP` files before claiming a gain. An unordered map once produced a "gain" that turned out to be run-to-run noise.
- Report the exclusion counts from line 1 together with accuracy.
- For a small SVAMP gain, compare per-item dumps (e.g. a McNemar test on the items that changed), not just percentages.
- Without `--exact`, `--world`, `--princ` and `--qread` silently have no training items to attach to.

---

## 3. World scripts: format

### Gist
A world script turns a word problem into a **quantity world**:
- the state is `entity.thing → exact number`;
- every command is anchored to the sentence that justifies it (`@N`) and labelled with the principle it applies (`k=`).

The executor (`ai/world/src/quant.rs`: `parse_line`, `run`, `check`) runs scripts with the exact core. It accepts a script only if it passes every gate.

### When to use
- Reading or writing training data for the world writer.
- Checking a hand-written script against a problem.

### Steps
The line grammar comes from `parse_line`. Blank lines and lines starting with `#` are skipped.

```
@N set   E.x = EXPR          k=KIND
@N add   E.x = EXPR          k=KIND
@N sub   E.x = EXPR          k=KIND
@N mul   E.x = EXPR          k=KIND
@N div   E.x = EXPR          k=KIND
@N move  A.x -> B.x = EXPR   k=KIND      (from A to B; A must exist)
@N ask   EXPR                            (k= optional)
```
- `N`: a 1-based sentence number. Sentences are split at `.`, `?` or `!` when followed by a space or the end of text, so `$2.50` is not split.
- Cells are `entity.thing`: lowercased, with non-empty parts around the dot, e.g. `janet.egg` or `tom.$`.
- `EXPR`: numbers, cell references, `+ - * /` and parentheses.
- `KIND` is one of `given total gain loss transfer rate compare part convert combine unit`. Any other value is an error.

Gates (each reports an error):
- the sentence `@N` exists;
- every number in `EXPR` appears in sentence N or an earlier one. Digits and number words both count, a percentage counts both as `20` and as `0.2`, and the constants `0 1 2 3 4 7 10 12 24 30 52 60 100 365 1000` and their reciprocals are always allowed;
- `add/sub/mul/div` and `move` apply only to an existing cell (use `set` first);
- no division by zero;
- there is an `ask`;
- the answer equals the gold answer.

**Corpus file** (`corpus/math-scripts/`, JSONL, one problem per line; written by `quant-vmm`, read by `load_done`, `quant-stats` and `steps --world/--princ`):
```json
{"id":"svamp-train-chal-777","set":"svamp-train","question":"There are 87 oranges ... How big is each group of bananas?",
 "gold":"145","script":"@1 set philip.orange = 87 k=given\n...\n@3 ask philip.banana_per_group\n",
 "ok":true,"errors":[],"round":1}
```
- `ok` is true only if the gates found no errors.
- `round` is 1 for the first attempt and 2 if the single retry fixed the script.
- `steps` uses only `ok: true` records and matches them to training problems by the **exact `question` text**. For SVAMP that text is `Body` + space + `Question`, with `". "` inserted if `Body` lacks final punctuation.

### Example
Script file `s.txt` for SVAMP train item `chal-777`:
```
@1 set philip.orange = 87 k=given
@1 set philip.banana = 290 k=given
@2 set philip.banana_group = 2 k=given
@3 set philip.banana_per_group = philip.banana / philip.banana_group k=unit
@3 ask philip.banana_per_group
```
```sh
cargo run --release --manifest-path ai/world/Cargo.toml -- quant-check svamp-train svamp-train-chal-777 s.txt
# answer Some("145"); errors 0
cargo run --release --manifest-path ai/world/Cargo.toml -- quant-stats corpus/math-scripts
# green counts per set, plus principle and verb histograms (reads <dir>/scripts.jsonl)
```
A script that uses `300` instead of `290` fails with two errors: `number 300 not from the text up to sentence @1` and `answer 300 ≠ reference 145`.

### Pitfalls
- `k=` must be preceded by a space; it is split off with `rsplit_once(" k=")`.
- Set names for `quant-check`/`quant-vmm` are `svamp-train`, `svamp-test`, `gsm8k-train` and `gsm8k-test`. SVAMP ids are `<set>-<ID>`; GSM8K ids are `<set>-<index>`.
- Negative controls live in `ai/world/src/quant.rs` tests and must stay red: `cargo test --release --manifest-path ai/world/Cargo.toml --lib quant`.

---

## 4. Training the world writer

### Gist
The small model learns to write the quantity world itself (`ai/math/src/qworld.rs`):
- `WorldModel` labels each number with an action (`=`, `+`, `-`, rate) and a cell;
- `AskModel` picks a query template (`State`, `Diff`, `Sum`, `Ratio`, `Product`).

Labels come from verified world scripts (section 3). Without trained models, a rule-based writer (`qworld::write`) is used.

### When to use
You have verified scripts (shipped in `corpus/math-scripts/` or generated yourself) and want the world as a third opinion or as a judge feature.

### Steps
1. Optional: generate more scripts with an LLM. `quant-vmm` sends batches to an LLM through a CLI (`claude -p`), checks each script, retries red ones once with the error list (the gold answer is never shown), and appends to `<dir>/scripts.jsonl`. LLM costs are logged in `<dir>/calls.jsonl`. It resumes by skipping ids already present. This step is paid and optional:
   ```sh
   MOVA_SUBSCRIPTION=1 cargo run --release --manifest-path ai/world/Cargo.toml -- \
     quant-vmm svamp-train runs/quant --limit 20 --batch 10 --par 2 --cap 2
   ```
   `MOVA_SUBSCRIPTION=1` makes the CLI use its own login instead of looking for a locally stored API key. `--cap` is the spend ceiling in USD [10]. The model and effort come from `PRAG_MODEL` / `PRAG_EFFORT` [medium].
2. Train and use the writer inside `steps` (it is not saved; it is retrained on every run):
   ```sh
   cargo run --release --manifest-path ai/math/Cargo.toml --bin mathsolve -- \
     steps --set svamp --exact --rerank 5 --world corpus/math-scripts/scripts.jsonl --world-feat
   ```
3. Read the report lines:
   - `world: training problems with scripts N, number labels M`: training problems matched to scripts, and labelled numbers;
   - `query: templates from scripts …`: template histogram;
   - at test time, `world: correct, agrees with judge` and similar lines count how often the world was right and whether it agreed with the judge.

### Pitfalls
- Scripts match only if the `question` text is byte-identical to the problem text built by the loader.
- Few scripts give little signal. Treat more scripts as more training and test data, not as a guaranteed accuracy gain.

---

## 5. World reader: `qread` (world v2 for numbers)

### Gist
`ai/math/src/qread.rs` reads a problem as **events in time**:
- **who**: the subject; a pronoun maps to the previous subject;
- **what**: the noun with its modifiers;
- **how much**: a number, or an unknown variable for "some" / "several";
- **verb class**: have → set, get/make → add, lose → subtract, give → transfer.

"He had 492 left" or "if he has 116 now" are not events but **conditions on the state**. The world solves for the unknowns from them (linear equations). The question becomes a query: state now / initially, sum of a class of events, difference, total.

If the world doesn't understand something, it returns `None` with a reason in the log and lists the numbers it could not read (`unread`). It does not guess. Rates and shares ("each", "per", "half", …) are left to `steps`.

### When to use
- Additive and transfer problems with unknown starting amounts.
- As a judge feature (`steps --qread`).

### Steps
```sh
# probe one problem (the last sentence is the question)
cargo run --release --manifest-path ai/math/Cargo.toml --bin mathsolve -- qread-text \
  "Paul had some crayons. He gave 52 crayons to his friends and lost 535 crayons. He had 492 crayons left. How many crayons did he have at first?"

# coverage and accuracy on SVAMP: qread [svamp|svamp-train] [N = wrong answers to print]
cargo run --release --manifest-path ai/math/Cargo.toml --bin mathsolve -- qread svamp 5
```
- `QREAD_STRICT=1`: only answer when every number was read.
- `QREAD_DUMP=<file>`: write `index<TAB>ok<TAB>value` for every answered item.

### Example
```
Some(1079.0) unread []
t0: paul have(have) crayon Set None
t1: paul give(give) crayon Sub Some(52.0)
t1: paul lose(lose) crayon Sub Some(535.0)
t2: paul have(have) crayon Is Some(492.0)
state initially Some("paul") Some(("crayon", "crayon"))
```
`unread` lists the numbers the world could not read; `state initially` is the state at the start.

`qread svamp` prints `svamp: the world answered A/300, of them correct K (p%)`: the world answered A of 300, and K of those were right. **Report both coverage (A/300) and accuracy (K/A).**

### Pitfalls
- High accuracy on low coverage is not a solver. Quote both numbers.
- As a judge feature (`--qread`), the reader's answer is used only when `unread` is empty.

---

## 6. Evaluating on SVAMP / GSM8K

### Gist
The datasets are not in the repo. Download them (both MIT) and convert them to the JSON arrays the loaders expect under `<DATA_ROOT>` (section 0).

### Steps
1. **SVAMP**: take the 700/300 split from Hugging Face `ChilleD/SVAMP` (`train.json`, `test.json`). The original 1000 problems are on GitHub at `arkilpatel/SVAMP` (same fields, no split). Each item needs `ID`, `Body`, `Question`, `Equation`, `Answer`.
   ```
   <DATA_ROOT>/mwpdata-svamp/svamp-train.json
   <DATA_ROOT>/mwpdata-svamp/svamp-test.json
   ```
2. **GSM8K**: take the socratic files from GitHub `openai/grade_school_math` (`grade_school_math/data/train_socratic.jsonl`, `test_socratic.jsonl`). The loaders want a **JSON array**, not JSONL. Fields: `question`, and `answer` with `<<…>>` steps and a final `#### N`.
   ```sh
   jq -s . train_socratic.jsonl > <DATA_ROOT>/mwpdata-gsm8k-socratic/gsm8k-socratic-train.json
   jq -s . test_socratic.jsonl  > <DATA_ROOT>/mwpdata-gsm8k-socratic/gsm8k-socratic-test.json
   ```
3. Run the commands from section 2 (`steps --set svamp|gsm8k …`) and section 5 (`qread svamp`). Each command prints its own summary; see the output lists above.
4. Before claiming any change:
   - run twice and confirm identical results (diff `MATH_DUMP`);
   - report the excluded training items (line 1 of `steps`) and the reader's coverage;
   - compare per item against the baseline.

### Example
At the time of writing, the project notes report the following on SVAMP test (300 problems) with the benchmark configuration plus `--qread`:
- 179/300 (59.7%) correct;
- against a baseline with the same lexicon: 19 items gained, 6 lost (McNemar p = 0.015);
- against an older, stronger baseline the gain was **not yet significant** (p = 0.27).

GSM8K test (1319) was around 10–11% with the same solver. Treat these figures as a snapshot, not a guarantee.

### Pitfalls
- SVAMP "test" in this guide means the 300-item split. Don't compare it directly with numbers computed on all 1000 SVAMP problems.
- GSM8K training problems without an exact tree (implicit numbers) or with trees that reuse numbers are excluded unless you use `--dag`. Say so whenever you quote a number.
- Never tune on the test split. Make fixes based on errors in the train split only.
