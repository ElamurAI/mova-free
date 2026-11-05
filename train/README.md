# Training and rebuilding Mova

**Get the data first:** `train/get-corpus.sh 0.4` downloads the open Mova corpus 0.4 (books, treebanks, dictionary, math scripts, contrast sets; see [corpus/open](../corpus/open/README.md)) from `research.elamur.ai/download/0/0.4/`, checks its SHA-256 and unpacks it into `corpus/open/`. The guides below assume that layout. The current corpus, 0.5, comes with `train/get-corpus.sh` (or `mova fetch 0.5 corpus`), see [releases/0.5-files.md](../releases/0.5-files.md).

Mova is a Hybrid Combinatorial–Operational System written in Rust, with a Combinatorial Small Language Model at its core. These guides focus
on readable Markdown seeds, compiled Rust tables and the CPU learning path.
The optional neural GPU parser has its own [training guide](../docs/training.md).
Retraining combines seed edits, learned parameters and measurements. Each guide
is written for a person and an AI coding agent working in this repository.

## Gist

- **Knowledge lives in files you can read.** Seeds (`global/seeds`, `en/seeds`) are markdown
  with small fenced blocks. Each crate's `build.rs` turns them into Rust tables. A bad seed stops the
  build.
- **Statistics are learned from data with a known license.** Taggers and parsers are trained from UD
  treebanks into a model file. Readers and solvers learn from training splits only.
- **Every judgment comes with a reason.** Every answer can be traced to a link, a rule or a table
  row, and to the file it came from.

## The big picture: three levels

Knowledge sits on three levels: global (seeds compiled into `global`), domain modules (`en`, `math`, `world`,
`latex`, ...) and the context of the current task (the readers inside `world`, `math`, `prag`, ...). The levels are
described in [docs/architecture.md](../docs/architecture.md#three-levels-of-logic).

The key rule: **the context level never invents knowledge.** Level 3 combines facts from the text
with links from level 1. When it cannot explain a step that way, it reports a **gap**, such as "verb
`gnaw` has no class" or "state `proud`: unknown whether this is trouble or relief". Do not fix a gap
by making level 3 memorize more. Fix it by adding a seed to level 1 (or a domain module) and
rebuilding. `layers` wires the levels together. It routes a query to the global level, then to a
domain module, and escalates when neither covers it. Every routing decision is logged with its
score and threshold.

## Seeds -> build.rs -> Rust tables

```
global/seeds/**/*.md  --(global/build.rs)-->  OUT_DIR/global.rs
   ```principle  ```concept  ```category              PRINCIPLES, CONCEPTS, LINKS, RELATIONS,
   ```link  ```verbs  ```relation                     fn verb_class(lemma) -> Option<(class, effect)>
```

- Nothing is read from disk at run time. The tables are compiled into the binary.
- Build gates: a link to an undeclared concept, a duplicate concept, link, principle or relation, a
  malformed line, or a verb class that has the same name as a concept all make `cargo build` panic.
  The panic message names the file. See [add-a-seed.md](add-a-seed.md).
- English grammar seeds (`en/seeds`, ```` ```rule ```` blocks) are run by the deterministic
  engine `en::expert`. A unit test checks that every rule parses.
- The English lexicon (`en/data/lemmas.tsv`, `forms.tsv`, `consts.txt`) is compiled by
  `en/build.rs` into hash tables and word constants. Rules compare integers, not strings.

## Determinism

Mova must produce the same output for the same input every time; a gain that the next run cannot reproduce does not
count. Use `BTreeMap`/`BTreeSet` or sorted vectors wherever order decides anything, run the baseline and the change
twice each (`cmp run1.txt run2.txt`) before comparing, and report the test score once. The rules are in
[docs/training.md](../docs/training.md#changing-the-source-code).

## Every judgment explained

- Global answers carry the link chain and the seed file, for example `lose -> decrease -> subtract`
  from `global/seeds/shortcuts/possession.md`.
- Grammar rules report the rule id, the bound words and the rule's source.
- Solvers and readers print their steps. They print a gap when no level-1 link explains a step.
- Explanations can be switched on (`layers::Mode::Explain`) and are off for plain answers. Measuring
  how many steps are covered by explanations tells you how complete level 1 is.

## When to use which guide

| you want to ... | read |
| --- | --- |
| teach Mova a new fact, verb class, relation or value | [add-a-seed.md](add-a-seed.md) |
| retrain the English tagger and parser on your treebank | [retrain-the-parser.md](retrain-the-parser.md) |
| read another UD flavour, or write a conversion rule | [dialects-and-converters.md](dialects-and-converters.md) |
| solve or evaluate arithmetic word problems (SVAMP, GSM8K) | [math-world.md](math-world.md) |
| work on bAbI, StepGame, SpartQA story worlds | [story-worlds.md](story-worlds.md) |
| run the book and fairy-tale reading experiments | [reading-tales.md](reading-tales.md) |
| adapt Mova to your own domain end to end | [your-own-domain.md](your-own-domain.md) |

## Steps (first session)

1. Install stable Rust (edition 2024). The crates form one Cargo workspace: `cargo build --release`
   at the root builds them all into `target/release/`. To work on one crate, pass `-p <crate>` or
   `--manifest-path`, or `cd` into it.
2. Build and test the global level. It takes seconds and has no dependencies:

   ```sh
   cargo test --release --manifest-path global/Cargo.toml
   ```

3. Build the English module. It is the base of `prag`, `world`, `math`, `coref` and `qagen`:

   ```sh
   cargo build --release --manifest-path en/Cargo.toml
   cargo test --release --manifest-path en/Cargo.toml --lib seeds_all_parse
   ```

4. Pick the guide for your task from the table above.

## Example: what "explained" looks like

The global crate's own tests show the idea. A falling apple leads to harm through four declared
links. An apple lying on a table produces nothing (a negative control):

```rust
let c = global::consequences("An apple fell on Newton's head.");
assert!(c.iter().any(|x| x.starts_with("fall → accelerate → impact → pain → harm")));
assert!(global::consequences("An apple lies on the table.").is_empty());
```

## Pitfalls

- A gate that can never fail is not a gate. Every check you add needs a negative control that shows
  red: a bad seed that breaks the build, or a sentence the rule must not fire on.
- `world`, `math`, `prag`, `qagen`, `coref` and `layers` depend on `en` and `global` through
  path dependencies. A change to a seed or to `en` changes their behaviour too. Re-measure the
  modules you care about.
- Data and trained models keep the licenses of their sources ([licence/DATA.md](../licence/DATA.md)). Register the
  license of any training data you add (see [retrain-the-parser.md](retrain-the-parser.md)).
