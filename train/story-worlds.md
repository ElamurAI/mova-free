# Story worlds: bAbI, StepGame, SpartQA

## Gist

The `world` crate (`ai/world`) reads short stories into a temporary *story world* (who is where, who holds
what, which thing relates to which) and answers questions from that world. It does not train on the
questions, and its code has no trigger words. When it cannot answer, it prints `?` and the reason. A gap
like that tells you what knowledge to add next.

Three readers are covered here:

| Command | Dataset | Reader | Knowledge it takes from the global level |
|---|---|---|---|
| `world babi` | bAbI tasks 1-20 | `ai/world/src/babi.rs`: a UD tree from `en`, then verb class / relation lookups | `global::verb_class` (```verbs blocks) and `global::relation` (```relation blocks) |
| `world stepgame` | StepGame k=1..10 | `ai/world/src/stepgame.rs`: an averaged perceptron with integer weights reads each sentence into a vector, and the world adds vectors along a path | none. The nine direction vectors are a constant in the code (`LABELS`) |
| `world spartqa` | SpartQA-human | `ai/world/src/spartqa.rs`: object mentions plus relation words between mentions, with three-valued inference | `global::concept_words` (```category blocks in `scene.md`) |

Global knowledge lives in the seed files `ai/global/seeds/**/*.md`. At compile time, `ai/global/build.rs`
turns their fenced blocks into Rust tables (`RELATIONS`, `verb_class()`, `CONCEPTS`, ...). The seed file
is the source of the knowledge, and the compiled tables are what the readers query.

## When to use

- A probe or an eval prints a gap such as `verb "sprint" has no class at the global level` or
  `gap: where people go when sleepy`. Add a verb or a relation to a seed.
- You want to learn a fact table from *training* data and freeze it as a seed (`world babi-learn`).
- You changed the parser (`en`) or a seed, and you need to show that nothing regressed.

## Setup

- **No workspace.** Each crate has its own `Cargo.toml` and `Cargo.lock`, and crates depend on each other
  through relative paths (`ai/world` → `../global`, `../en`, `../prag`, `../math`). Always pass
  `--manifest-path`.
- **Parser model.** bAbI needs the `en` tagger/parser model. It ships in
  `ai/en/models/ud-ewt-eslspok.bin` and is used by default (`WORLD_EN_MODEL` overrides it). Without a model
  the reader fails with an error; it does not silently fall back to rules. To train your own from the UD
  English EWT and ESL-Spok treebanks (CC BY-SA 4.0):
  ```sh
  cargo run --release --manifest-path ai/en/Cargo.toml -- train <en_ewt-ud-train.conllu> <en_eslspok-ud-train.conllu> -- models/ud-ewt-eslspok.bin
  ```
  StepGame and SpartQA do not use the model.
- **Datasets** are not in the repo. Get them from their public sources:
  - **bAbI** tasks 1-20 v1.2 (Facebook Research, CC BY 3.0). The archive is `tasks_1-20_v1-2.tar.gz`; there
    is also a mirror on Hugging Face as `facebook/babi_qa`. Use the `en/` folder, which holds files
    `qaN_<name>_{train,test}.txt`.
  - **StepGame** (MIT): GitHub `ShiZhengyan/StepGame`. You need a folder with `qa1_train.json` and
    `qa1_test.json` ... `qa10_test.json`.
  - **SpartQA-human**: GitHub `HLR/SpartQA_generation`. The archive `SpartQA_Human.zip` contains
    `human_test.json` and `human_train_annotation.json`. No data license is stated, so treat it as
    non-commercial: use it for measurement only, and copy nothing from it into code or seeds.

The commands below use these placeholder paths: `data/babi/en`, `data/stepgame`,
`data/SpartQA_Human/human_test.json`.

## Commands (verified against `ai/world/src/main.rs`)

All arguments are positional. The code has a home-directory fallback when the folder argument is missing,
so always pass the folder explicitly.

```sh
W="cargo run --release --manifest-path ai/world/Cargo.toml --"

$W babi <en-dir> [test|train] [N]   # all files *_<split>.txt in the folder; N = errors shown per task (default 0)
$W babi-probe "<sentence>" ... "<question?>"   # any argument containing '?' is a question; the others are read into the story
$W babi-learn <qa20_..._train.txt>  # learn "state -> go=place" from training data; prints relation lines
$W babi-contrast <src-dir> <dst-dir>  # writes contrast copies: new names, places, things, synonym verbs
$W stepgame <dir> [epochs] [N]      # trains on qa1_train.json (default 10 epochs), tests qa1..qa10_test.json
$W spartqa <file.json> [N]          # accuracy per question type (YN / FB / CO / FR)
```

Environment switches (all read in the source):

| Variable | Effect |
|---|---|
| `BABI_DEBUG=1` | prints the UD tree of each sentence (form, lemma, UPOS, head, relation) to stderr |
| `STEPGAME_ERR=1` | prints reader errors on level-1 test sentences, grouped by template (agents shown as A1/A2) |
| `STEPGAME_EXCLUDED=<file.tsv>` | writes the list of questions excluded from the "clean" number, one per line, with the reason |
| `SPARTQA_SCENE=1` | on each error that is shown, prints the story, the objects read, the facts, and the block facts |
| `SPARTQA_DEBUG=<substring>` | dumps the scene of every story that contains the substring |
| `GLOBAL_EXCLUDE=a,b` | **build time**: seeds whose path contains any of these substrings are left out of the build (ablation) |

The key output words are `total`, `solved (≥95%)`, `without doubtful`, and `gap` (missing knowledge).

## Seed blocks the readers use

**Verb classes** (```verbs, in `ai/global/seeds/shortcuts/possession.md`). Each line has three parts:
`<class> <effect> <lemma>...`. The effect is one of `=`, `+`, `-`, `±`, which mean no change, more, less,
and transfer of quantity. Multi-word verbs are written with an underscore (`pick_up`), and the reader
forms them from a verb plus its particle (`compound:prt`).

```verbs
get + get grab pick_up get_on buy receive find collect ...
give ± give pass hand_over sell lend pay send ...
lose - lose eat spend use break drink throw ... drop discard abandon put_down ...
move = move hurry choose come start gnaw wake run walk drive travel journey ...
```

The bAbI reader acts on these classes:

- `move`: the subject goes to the first nominal oblique.
- `get` or `have` with an object: the subject now holds the thing.
- `lose`: the subject drops the thing.
- `give`: the thing moves from the subject to the `to`-oblique or the indirect object.

Bare `be` with `in` or `at` is also read as a location. Any other verb gives
`verb "<lemma>" has no class at the global level`.

**Relations** (```relation, one per line as `name: key=value ...`). From
`ai/global/seeds/shortcuts/relations.md`:

```relation
north_of: inverse=south_of vec=0,1 step=n
bigger_than: inverse=smaller_than transitive=yes
fit_inside: same=smaller_than
morning: order=1
```

From `ai/global/seeds/shortcuts/motivations.md`:

```relation
hungry: go=kitchen
tired: go=bedroom
```

Keys that `babi.rs` actually reads:

| Key | Meaning in the reader |
|---|---|
| `inverse` | The opposite relation. A word plus its preposition (`north`+`of`, `bigger`+`than`, `fit`+`inside`) is recognised as a relation **only** if the relation has `inverse` or `same` |
| `same` | An alias, rewritten to the canonical relation (`fit_inside` → `smaller_than`) |
| `vec` | A 2-D direction, used for left/right/above/below questions |
| `step` | A path step (`n`, `s`, `e`, `w`). Path finding needs `step` on **both** the relation and its inverse |
| `transitive` | `yes`: chain facts ("bigger than" through intermediate things) |
| `order` | A time word and its rank within one story (`yesterday`=0 ... `evening`=3) |
| `go` | Motivation: the state → where the person will go. It also answers "why did X go ..." |

`symmetric=yes` (in `scene.md`) is documentation only: no reader looks it up at the time of writing. The
SpartQA relation words (`left`, `above`, `near`, `touching`, ...) are a `match` in `spartqa.rs`. SpartQA
does read the `scene_color`, `scene_size`, `scene_shape` and `scene_any` categories from `scene.md`.

**Build gate.** `ai/global/build.rs` panics, so the build fails, and the message names the seed file, if
any of these hold:

- a relation line has no `:`, or a field has no `=`;
- a relation name is defined twice, in any seed;
- a ```verbs line has no effect, or the effect is not one of `= + - ±`;
- a verb class name collides with a concept;
- a ```link line points to an undeclared concept, or a link is duplicated.

When a word appears in two verb classes, the **first** one in build order wins, and nothing warns you.
Files are walked in sorted path order.

## Steps: add a verb or a relation

1. **Find the gap.**
   ```sh
   $W babi-probe "Mary sprinted to the attic." "Where is Mary?"
   ```
   Output (translated):
   ```
   Mary sprinted to the attic. → Some("verb «sprint» has no class at the global level")
   ? Where is Mary? → ("?", "where mary is — unknown")
   ```
   Use `BABI_DEBUG=1` to make sure the parse is sane (here `sprinted` is lemmatised to `sprint` and is the
   VERB root). If the tree is wrong, the fix belongs in `en`, not in a seed.
2. **Edit the seed.** Append the lemma to the right class line in
   `ai/global/seeds/shortcuts/possession.md`, for example `move = move hurry ... sprint`. For a new relation,
   add a line to a ```relation block. Give it `inverse=` (or `same=`) so the reader recognises it. Add
   `step=` on both directions if path questions need it.
3. **Rebuild and probe again.** `build.rs` re-runs when anything under `seeds/` changes, so the next
   `cargo run` recompiles `global` and `world`. Rerun the same probe. The sentence should now read as
   `None` (understood) and the question should be answered.
4. **Check for regressions.** Run `cargo test --release --manifest-path ai/global/Cargo.toml`. It includes
   `verb_class` assertions, such as `eat` → `lose`. Then run the full evals below, on both the original
   files and the contrast files. Verb classes are also used by `ai/math`, which reads quantity word
   problems, so run its checks too.
5. **Write down why** in the seed's prose (gist, conditions, examples, sources). The block is the data, and
   the prose is the explanation.

## Steps: learn knowledge from training data into a seed

`world babi-learn` reads a bAbI task-20 *training* file. For every `Where will X go?` question, it takes
X's last stated attribute ("Jason is thirsty") and counts each attribute → gold-place pair. Then it prints
the majority place for each attribute, with its support. It never opens the test file.

```sh
$W babi-learn data/babi/en/qa20_agents-motivations_train.txt
```

Output on the 1k training split:

```
bored: go=garden    # 98/98
hungry: go=kitchen    # 89/89
thirsty: go=kitchen    # 97/97
tired: go=bedroom    # 91/91
```

To turn it into a seed:

1. **Decide.** A person (or a gate) decides what to keep. Keep only rows with clean support (here
   375/375). A split count is a signal to look at the data, not a rule to freeze.
2. **Strip the `# n/all` comments.** Inside a block, `#` is a field without `=`, so the build fails with
   `relation "...": "#" without "="`. Move the counts into the prose.
3. **Check the names are new.** For example, `hungry` already exists in `motivations.md`. A second
   definition makes the build fail with `relation "hungry" is repeated`.
4. **Write the seed** at `ai/global/seeds/shortcuts/<topic>.md`:

   ````markdown
   # State → where people go

   **Gist.** A person's state suggests a goal: hungry or thirsty → kitchen, tired → bedroom, bored → garden.

   **Conditions and exceptions.** A house with typical rooms; another world (forest, city) has other goals.
   Learned with `world babi-learn` from the bAbI task-20 training split: 375 of 375 questions, no exceptions;
   the test split was not opened.

   **Examples.** "Jason is thirsty. Where will Jason go?" → kitchen.

   **Sources.** Weston et al. 2015, bAbI task 20 (CC BY 3.0), training split.

   ```relation
   hungry: go=kitchen
   thirsty: go=kitchen
   tired: go=bedroom
   bored: go=garden
   ```
   ````

5. **Confirm the seed is what answers.** Rebuild, then probe `"Jason is thirsty." "Where will Jason go?"`.
   The answer should be `("kitchen", "thirsty → kitchen (first level)")`. Then do an ablation: build with
   `GLOBAL_EXCLUDE=motivations` and qa20 should drop. This shows the score comes from the seed. The run
   only needs the env var; do not delete the seed file.

`babi-learn` handles only the task-20 question shape. To learn another table, copy its pattern from
`learn_motivations` in `babi.rs`: read the story, count (state, gold) pairs on training data only, print a
block line with its support, and let a person decide what to keep.

## Evaluate

```sh
# bAbI: test split; also run train (should be close: nothing was fitted to test)
$W babi data/babi/en test 0
$W babi data/babi/en train 0
# Contrast set: same stories with new names, places, things and synonym verbs (some deliberately not in the seeds)
$W babi-contrast data/babi/en data/babi-contrast
$W babi data/babi-contrast test 0

# StepGame: full test, plus the list of excluded items
STEPGAME_EXCLUDED=stepgame-excluded.tsv $W stepgame data/stepgame 10 0

# SpartQA-human test
$W spartqa data/SpartQA_Human/human_test.json 5
```

Output formats:

- **bAbI** prints one line per task, `qaN_name ok/n acc%`, and ends with
  `total ok/n (acc%), solved (≥95%) s/20`. With `N` > 0, each error is printed with the last 12
  context sentences, the answer, the gold answer, and the reason.
- **StepGame** prints the training accuracy for each epoch on stderr. On stdout it prints one line per k:
  the full accuracy and then `without doubtful`. After training, the reader flags any
  sentence template that has contradictory labels in `qa1_train.json` (at least 10 examples, with the top
  label under 60%). Test questions whose story uses a flagged template count as "doubtful".
  - **The headline number is the full test.** The clean number is secondary, and every excluded item is
    listed by name in the TSV, so the clean number cannot be read as quietly pruning the test.
  - TSV columns: `k`, `id` in `qa{k}_test.json`, the flagged template numbers, and whether the answer was
    right (`1` or `0`). The templates themselves are listed in `#` lines at the end of the file.
- **SpartQA** prints `TYPE: ok/n (acc%)` for each type, then a total. The test split here is the 2022
  version (YN 143, FR 77, with FB and CO also reduced). The total is therefore **not** comparable with
  results on the original 510-question test. Compare only per type, under matching rules.

Results at the time of writing, from the project's committed architecture notes. These runs reproduce them:

- **bAbI (1k):** test 99.8%, 20/20 tasks at ≥95%; train 99.7%. The contrast set first scored 74.6%. That
  exposed parser faults and two vocabulary gaps (hurry, abandon). After the fixes it scores 99.7%, and the
  original was unchanged.
- **StepGame:** 96.3% on the full test (k=1 98.2% → k=10 94.7%). Two contradictory templates were found;
  without them every k scores 98.0-99.6%, and 11,553 questions are excluded by name.
- **SpartQA-human:** 59.3% on the test (majority baseline 34.9%): YN 60.1%, FB 67.1%, CO 58.2%, FR 50.6%.

Runtime is seconds: about 14 s for the bAbI test, about 8 s for StepGame, and under 1 s for SpartQA.

## Pitfalls

- **Determinism before claims.** Run the same eval twice and get identical output before you claim a gain.
  The readers use `BTreeMap`/`BTreeSet`, and the StepGame perceptron uses a fixed visiting order and
  integer weights. Any new code on these paths must stay deterministic: no `HashMap` iteration order, no RNG.
- **Tune on train, measure on test.** Choose fixes by looking at training-split errors (`babi ... train N`,
  SpartQA training annotations). `babi-learn` must only ever see `_train` files.
- **A seed fix does not fix a parse.** If `BABI_DEBUG` shows a wrong tag or head (e.g. `attic` as ADJ, a
  name lemmatised to a common noun), adding words to seeds hides the problem. The contrast set exists to
  expose this.
- **Global seeds are shared.** A word added to a verb class changes `ai/math` and `world`'s quantity reader
  as well. A word that already sits in an earlier class is silently ignored.
- **A relation the reader cannot see.** It needs `inverse=` or `same=`. A path needs `step=` on both
  directions.
- **SpartQA data is non-commercial.** It is measurement only. Nothing from it may go into seeds or code.
- **Positional arguments are not checked.** `babi-learn`, `babi-contrast` and `spartqa` panic if the path
  is missing. `babi` and `stepgame` fall back to a built-in folder, so pass the folder explicitly.
