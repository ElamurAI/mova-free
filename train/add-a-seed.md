# Add a seed

## Gist

A seed is a short markdown note holding one piece of knowledge, with its source. It contains fenced
blocks that a machine can read. Think of a seed as the thing a rule grows from. There are two kinds:

- **Global seeds** (`ai/global/seeds/**/*.md`): concepts, links between them, verb classes,
  relation properties and values. `ai/global/build.rs` compiles them into Rust tables. A wrong seed
  breaks the build.
- **Grammar seeds** (`ai/en/seeds/<section>/*.md`): ```` ```rule ```` blocks for English, run by the
  deterministic engine `en::expert`. A unit test checks that every rule parses.

## When to use

- A reader prints a **gap**: "no class for verb X", "where do they go when STATE", "state S: unknown
  whether trouble or relief". A gap means level 1 is missing knowledge. Add a global seed. Do not
  teach the reader to memorize the answer.
- You know a general fact that many tasks need, such as an inverse relation, a cause-effect link, or
  what a verb does to a quantity.
- You want the tagger or parser output checked against a grammar fact (grammar seed).

Domain-specific depth, such as legal terms or chemistry at expert level, belongs in a domain module,
not in the global level. See [your-own-domain.md](your-own-domain.md).

## Global seed file layout

Write prose for humans first, then the blocks. The build reads only the blocks. Files are found
recursively and sorted by path. The sections already in the repo are `self/`, `shortcuts/`,
`values/` and `wikidata/`.

```markdown
# Short title

**Gist.** One to three sentences: the knowledge.
**Conditions and exceptions.** Where it does not hold.
**Examples.** Two to four short English examples.
**Sources.** Book and page, paper id, dataset.

```relation
...
```
```

## Block syntax (exactly what `ai/global/build.rs` parses)

Each non-empty line inside a block is one entry. Leading and trailing spaces are trimmed.

| block | line format | compiled into | notes |
| --- | --- | --- | --- |
| ```` ```concept ```` | `name: word word word` | `CONCEPTS` | the words trigger the concept in text (`global::concepts_in`) |
| ```` ```category ```` | `name: word word word` | `CONCEPTS` with `category: true` | explains steps (time units, measures, countable things) but never starts a consequence chain |
| ```` ```link ```` | `from -> to : why` | `LINKS` | both ends must be declared concepts or verb classes; `why` is the explanation that gets printed |
| ```` ```verbs ```` | `class effect lemma lemma ...` | `fn verb_class(lemma) -> Option<(&str, char)>` | `effect` is one of `=` `+` `-` `±` (what the verb does to the holder's quantity); the class name also becomes a concept |
| ```` ```relation ```` | `name: key=value key=value` | `RELATIONS`, `global::relation(name, key)` | values contain no spaces |
| ```` ```principle ```` | `id:`, `name:`, `gist:`, `against:`, `towards:`, optional `rule:` (one per line) | `PRINCIPLES` | `against`/`towards` are space-separated words, `_` stands for a space; `rule` names a predicate over the story world (`world::values`) |

These relation keys are currently read by code (`ai/world/src/babi.rs`, `ai/math/src/qread.rs`):

| key | meaning | example |
| --- | --- | --- |
| `inverse` | the reverse relation | `north_of: inverse=south_of` |
| `same` | an alias | `fit_inside: same=smaller_than` |
| `vec` | a direction on the plane, `x,y` | `east_of: vec=1,0` |
| `step` | the step letter in path answers | `north_of: step=n` |
| `transitive` | chains compose | `bigger_than: transitive=yes` |
| `order` | position in time | `morning: order=1` |
| `go` | where an agent in this state goes | `tired: go=bedroom` |

Any other key compiles without error, but no code reads it until you write that code.

## Steps: global seed

1. Find the gap. Run the reader on a tiny story (see [story-worlds.md](story-worlds.md)):

   ```sh
   cargo run --release --manifest-path ai/world/Cargo.toml -- \
     babi-probe "Sumit is sleepy." "Where will sumit go?"
   # ? Where will sumit go? → ("?", "gap: where they go when sleepy")
   ```

   The `gap:` marker shows the missing knowledge.
2. Check whether the knowledge already exists: `grep -rn "sleepy" ai/global/seeds`.
3. Add it. Use an existing file when the topic matches, or create a new `.md` file. Here, the
   motivations seed (`ai/global/seeds/shortcuts/motivations.md`) already holds
   `hungry: go=kitchen` and similar lines:

   ```relation
   sleepy: go=bedroom
   ```

4. Rebuild and run the tests. `build.rs` reruns whenever anything under `seeds/` changes:

   ```sh
   cargo test --release --manifest-path ai/global/Cargo.toml
   ```

5. Rebuild the consumer and probe again. `world` and `math` depend on `global` by path:

   ```sh
   cargo run --release --manifest-path ai/world/Cargo.toml -- \
     babi-probe "Sumit is sleepy." "Where will sumit go?"
   # ? Where will sumit go? → ("bedroom", "sleepy → bedroom (first level)")
   ```

6. Measure the whole benchmark before and after, twice each (see "Measure" below).

## Example: a new verb class with a link

```markdown
# Heating

**Gist.** Heating makes a thing hot.
**Sources.** Any school physics textbook.

```verbs
heat + heat warm boil fry
```

```concept
hot: hot boiling
```

```link
heat -> hot : heating makes a thing hot
```
```

A unit test pins the behaviour, including a negative control:

```rust
#[test]
fn heat_class() {
    assert_eq!(global::verb_class("boil"), Some(("heat", '+')));
    let p = global::path(global::concept("heat").unwrap(), global::concept("hot").unwrap()).unwrap();
    assert_eq!(p[0].why, "heating makes a thing hot");
    assert_eq!(global::verb_class("freeze"), None); // must stay unknown
}
```

## Build gates (what makes the build fail)

`cargo build` panics, naming the seed file, when:

- a link points to an undeclared concept (`hot -> burn_risk` with no `burn_risk` concept);
- a concept or category name is declared twice;
- the same `from -> to` link appears twice;
- a relation name repeats, a relation line has no `:`, or a `key=value` item has no `=`;
- a `link` line has no ` : why` or no `->`, or a `concept` line has no `:`;
- a verb line has no effect, or its effect is not one of `= + - ±`;
- a verb class has the same name as a concept;
- a principle lacks `id:`, `name:`, `gist:`, `against:` or `towards:`, or its `id` repeats.

The gates are tested to really fail. Adding `hot -> burn_risk : ...` without declaring `burn_risk`
stops the build with "concept «burn_risk» not declared".

## Steps: grammar seed (`ai/en/seeds`)

1. Pick a section: `morph`, `nominal`, `verbal`, `lexicon` or `errors`. Grep first, because more
   than 400 rules already exist (`grep -rn "much" ai/en/seeds`).
2. Write the prose (Gist, Conditions, Examples, How it looks in UD, Sources) and a rule block:

   ```rule
   rule: en.verbal.double-modal
   what: two modal auxiliaries on one verb (might could go) - standard English allows one
   match: v[upos=VERB|AUX]; m1[xpos=MD, rel=aux, head=v]; m2[xpos=MD, rel=aux, head=v, after=m1]
   require: not m2[]
   severity: warn
   source: CGEL ch. 3 (modals have no non-finite forms)
   ```

   Fields: `rule` (unique id), `match` (nodes `name[cond, cond]` separated by `;`), `require`
   (clauses separated by `;`, alternatives within a clause separated by ` or `; each item is
   `v[...]`, `not v[...]`, `exists c[...]` or `none c[...]`), optional `unless`, `what`,
   `severity` (default `warn`), `source`. Conditions: `upos=`, `xpos=`, `lemma=`, `form=`, `rel=`,
   `rel~` (base relation), `feats.X=`, `feats.X`, `!feats.X`, `head=v|0`, `before=`, `after=`,
   `next=`, `prev=`, `suffix=`, `prefix=`, `lemma.suffix=`, `lemma.prefix=`, `feats.X=@node`,
   `lemma=@form`. Use `!=` and `!~` to negate. Separate alternative values with `|`.
3. Check that it fires where it should and stays silent elsewhere. Use a tiny CoNLL-U file with one
   bad sentence and one good sentence:

   ```sh
   EN_EXPERT_LIST=1 cargo run --release --manifest-path ai/en/Cargo.toml -- \
     expert-check ai/en/seeds/verbal tests/double-modal.conllu
   # en.verbal.double-modal | warn | 1 | 1 | ... | dm1: go / might / could
   ```

   On a parser's output with a gold file, `expert-check <dir> --gold gold.conllu pred.conllu` adds
   a precision column: how many violations sit on tokens where the prediction is really wrong.
4. Run the gate test. It requires that every rule in `ai/en/seeds` parses and that rule ids are
   unique:

   ```sh
   cargo test --release --manifest-path ai/en/Cargo.toml --lib seeds_all_parse
   ```

5. Run the rule over a gold treebank such as UD EWT. A rule that fires often on gold data is either
   wrong or is catching errors in the gold data. Read the hits before you trust the rule.

## Measure

- Verb classes are features of the word-problem solver and the story readers. Widening a class can
  lower accuracy. One extension of the verb classes did exactly that. Treat every level-1 change as
  an experiment. Measure the benchmark before and after, two identical runs each. If a class
  extension hurts, add the words as a plain `concept` for explanations instead.
- Ablation without editing files: `GLOBAL_EXCLUDE=substr,substr` drops every seed whose path
  contains one of the substrings at build time. Compare runs with and without a seed to see what it
  is worth:

  ```sh
  GLOBAL_EXCLUDE=motivations cargo run --release --manifest-path ai/world/Cargo.toml -- \
    babi-probe "Sumit is tired." "Where will sumit go?"
  ```

## Pitfalls

- **Unknown block names are silently ignored.** ```` ```relations ```` (with an extra `s`) or
  ```` ```verb ```` compiles to nothing. After adding a seed, check that a test or a probe sees it.
- **The first verb class wins.** When a lemma appears in two `verbs` lines, the line read first
  wins, in path order of the files. No error is raised.
- Verb lemmas match exactly. Multi-word verbs are written with `_` (`pick_up`). Principle words also
  use `_` for spaces.
- An unclosed fence at the end of a file drops that block.
- `expert-check` prints rules that fail to parse (as `NOT PARSED`) but does not fail. Use the
  `seeds_all_parse` test as the gate.
- Readers that parse text (`world`, `math`) use the English model shipped in
  `ai/en/models/ud-ewt-eslspok.bin`; to use your own, see [retrain-the-parser.md](retrain-the-parser.md).
- Do not put culture-specific or trick logic into level 1. Level 1 holds common human knowledge.
  Alternatives belong in a domain or an auxiliary module.
