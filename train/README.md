# Training and rebuilding Mova

**Get the data first:** `train/get-corpus.sh` downloads the open Mova corpus (tales, summaries, questions, treebanks, math scripts) from [research.elamur.ai](https://research.elamur.ai/corpus/), checks its SHA-256 and unpacks it into `corpus/`. The guides below assume it is there.


Mova is a small language model written in Rust. You can see how it works. Most of what it knows is
written as plain markdown "seeds" and compiled into Rust tables. The rest is learned from open data
by deterministic table and Markov models. No neural network is involved. Retraining Mova means
editing seeds, retraining tables, and measuring honestly. These guides cover all three. Each guide
is written for two readers: a person, and an AI coding agent working in this repository.

## Gist

- **Knowledge lives in files you can read.** Seeds (`ai/global/seeds`, `ai/en/seeds`) are markdown
  with small fenced blocks. Each crate's `build.rs` turns them into Rust tables. A bad seed stops the
  build.
- **Statistics are learned from data with a known license.** Taggers and parsers are trained from UD
  treebanks into a model file. Readers and solvers learn from training splits only.
- **Every judgment comes with a reason.** Every answer can be traced to a link, a rule or a table
  row, and to the file it came from.

## The big picture: three levels

| level | what it holds | when it changes | where |
| --- | --- | --- | --- |
| 1. Global | principles and values, concepts, shortcuts between concepts ("fall -> accelerate -> impact -> pain -> harm"), verb classes, relation properties, the exact arithmetic core | only on a new build that passes the gates | `ai/global` (seeds -> `build.rs` -> `OUT_DIR/global.rs`) |
| 2. Domain modules | the full depth of one field: grammar of English, word problems, story worlds, LaTeX, code | only on a new build | `ai/en`, `ai/math`, `ai/world`, `ai/latex`, `ai/coder`, ... |
| 3. Context | the state of the current document, task or dialogue: entities, quantities, events, who holds what | lives only while the task runs | the readers inside `ai/world`, `ai/math`, `ai/prag`, ... |

The key rule: **the context level never invents knowledge.** Level 3 combines facts from the text
with links from level 1. When it cannot explain a step that way, it reports a **gap**, such as "verb
`gnaw` has no class" or "state `proud`: unknown whether this is trouble or relief". Do not fix a gap
by making level 3 memorize more. Fix it by adding a seed to level 1 (or a domain module) and
rebuilding. `ai/layers` wires the levels together. It routes a query to the global level, then to a
domain module, and escalates when neither covers it. Every routing decision is logged with its
score and threshold.

## Seeds -> build.rs -> Rust tables

```
ai/global/seeds/**/*.md  --(ai/global/build.rs)-->  OUT_DIR/global.rs
   ```principle  ```concept  ```category              PRINCIPLES, CONCEPTS, LINKS, RELATIONS,
   ```link  ```verbs  ```relation                     fn verb_class(lemma) -> Option<(class, effect)>
```

- Nothing is read from disk at run time. The tables are compiled into the binary.
- Build gates: a link to an undeclared concept, a duplicate concept, link, principle or relation, a
  malformed line, or a verb class that has the same name as a concept all make `cargo build` panic.
  The panic message names the file. See [add-a-seed.md](add-a-seed.md).
- English grammar seeds (`ai/en/seeds`, ```` ```rule ```` blocks) are run by the deterministic
  engine `en::expert`. A unit test checks that every rule parses.
- The English lexicon (`ai/en/data/lemmas.tsv`, `forms.tsv`, `consts.txt`) is compiled by
  `ai/en/build.rs` into hash tables and word constants. Rules compare integers, not strings.

## Determinism

Mova must produce the same output for the same input every time. A gain that the next run cannot
reproduce does not count.

- In anything that searches, ranks or breaks ties, iterate over `BTreeMap`/`BTreeSet` or sorted
  vectors. The standard `HashMap` iterates in random order. Once, an apparent accuracy jump turned
  out to be nothing but `HashMap` order noise.
- **Run twice before you claim a gain.** Run the baseline twice and get the same number. Run the
  change twice and get the same number. Only then compare. Byte-identical outputs are the best check
  (`cmp run1.txt run2.txt`).
- Keep the test split untouched. Tune on train/valid. Report the test score once.

## Every judgment explained

- Global answers carry the link chain and the seed file, for example `lose -> decrease -> subtract`
  from `ai/global/seeds/shortcuts/possession.md`.
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

1. Install stable Rust (edition 2024). The crates do not form a Cargo workspace. Each crate has its
   own `Cargo.toml` and lock file, so pass `--manifest-path`, or `cd` into the crate.
2. Build and test the global level. It takes seconds and has no dependencies:

   ```sh
   cargo test --release --manifest-path ai/global/Cargo.toml
   ```

3. Build the English module. It is the base of `prag`, `world`, `math`, `coref` and `qagen`:

   ```sh
   cargo build --release --manifest-path ai/en/Cargo.toml
   cargo test --release --manifest-path ai/en/Cargo.toml --lib seeds_all_parse
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
- `world`, `math`, `prag`, `qagen`, `coref` and `layers` depend on `ai/en` and `ai/global` through
  path dependencies. A change to a seed or to `en` changes their behaviour too. Re-measure the
  modules you care about.
- Data and trained models keep the licenses of their sources (`LICENSE-DATA.md`). Register the
  license of any training data you add (see [retrain-the-parser.md](retrain-the-parser.md)).
