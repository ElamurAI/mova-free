# Adapting Mova to your own domain

## Gist

To adapt Mova to a new domain you do not fine-tune weights. You add knowledge in three places:

1. **General knowledge** that every domain needs goes into **global seeds** (`ai/global/seeds`).
2. **The depth of your field** goes into a **domain module**: a crate that implements
   `layers::Domain`, with its own seeds and tables.
3. **Your text** is read by the existing context-level readers (`en`, `world`, `math`, `prag`). If
   their output is not good enough, retrain the parser on your annotated data.

Then measure honestly. Use a fixed split, two identical runs, a paired comparison, contrast sets,
and a published list of excluded items.

## When to use

- You have a body of text or questions in one field (inventory, recipes, contracts, lab protocols,
  customs codes) and want transparent, explainable answers.
- You can say what "correct" means for at least a few hundred items, and keep some of them unseen
  as a test set.

## Steps

### 1. Scope, data and licenses

- Write down the task in one sentence, the input format and the answer format.
- Split the data once into train, valid and test, and save the split as files. Never tune on test.
- Record the license of every source. Treebank data used for parser training must be registered in
  `ai/en/data/train-licenses.tsv` (see [retrain-the-parser.md](retrain-the-parser.md)). For
  everything else, keep a sources table in your module's README. Data and models keep the licenses
  of their sources.

### 2. Find the gaps first

Run what already exists on a few train items before you write code. Readers print **gaps**: a verb
with no class, a state with no known consequence, an unknown relation. Collect them into a list.
That list is your work queue.

```sh
cargo run --release --manifest-path ai/world/Cargo.toml -- \
  babi-probe "Mary restocked the pantry." "Where is Mary?"
# Mary restocked the pantry. → Some("verb «restock» has no class at level 1")
# ? Where is Mary? → ("?", "where mary is — unknown")
```

Grep the family of crates and seeds before you write anything new. Usually a verb class, a relation
or a reader for your case already exists.

### 3. Seeds: general facts up, expert facts down

- Facts any educated reader knows ("using an item decreases the stock", "a box fits inside a
  bigger box") go into global seeds. See [add-a-seed.md](add-a-seed.md) for the exact block syntax
  and the build gates.
- Facts only an expert knows belong in your domain module. Give the module its own `seeds/` folder
  and a `build.rs`. `ai/global/build.rs` (about 180 lines) is a good template: walk `seeds/**/*.md`,
  parse fenced blocks, panic with the file name on any undeclared reference or duplicate, and write
  `OUT_DIR/<module>.rs`.
- Keep culture-specific or "trick" logic out of level 1. Level 1 holds common human knowledge.

### 4. A domain module

A domain declares what it covers, with a reason. It answers with a justification, or returns `None`
for "not mine / cannot". `layers::Context::route` tries the global level first, then the best domain
above the threshold (0.5 by default), then escalates. Every routing decision is logged with all the
scores.

Example: a small `pantry` crate next to `ai/layers` (`ai/pantry/Cargo.toml` depends on
`layers = { path = "../layers" }` and `global = { path = "../global" }`). The sign of each step comes
from the global verb classes, so the module explains itself through level 1:

```rust
use layers::{Access, Answer, Context, Domain, Query, Route};

pub struct Pantry;
const WORDS: &[&str] = &["pantry", "shelf", "stock", "jars", "cans"];

impl Domain for Pantry {
    fn id(&self) -> &str { "pantry" }
    fn version(&self) -> &str { "0.1" }
    fn access(&self) -> Access { Access::Public }

    fn covers(&self, q: &Query, _: &Context) -> (f64, String) {
        let t = q.text.to_lowercase();
        let hits: Vec<&str> = WORDS.iter().copied().filter(|w| t.contains(w)).collect();
        ((hits.len() as f64 / 2.0).min(1.0), format!("domain words: {hits:?}"))
    }

    fn solve(&self, q: &Query, _: &Context) -> Option<Answer> {
        let w: Vec<String> = q.text.to_lowercase().split(|c: char| !c.is_alphanumeric())
            .filter(|s| !s.is_empty()).map(String::from).collect();
        let nums: Vec<i64> = w.iter().filter_map(|x| x.parse().ok()).collect();
        let (verb, class, eff) = w.iter().find_map(|x| {
            // naive lemmas for the sketch; a real module takes lemmas from the `en` parser
            [x.as_str(), x.strip_suffix('d').unwrap_or(""), x.strip_suffix("ed").unwrap_or("")].into_iter()
                .find_map(|l| global::verb_class(l).filter(|(c, _)| *c != "have").map(|(c, e)| (l.to_string(), c, e)))
        })?;
        let [a, b] = nums[..] else { return None };
        let v = match eff { '+' => a + b, '-' | '±' => a - b, _ => return None };
        Some(Answer {
            value: v.to_string(),
            by: Route::Domain("pantry".into()),
            why: vec![format!("'{verb}' is class '{class}' with effect '{eff}' (global level)"),
                      format!("{a} {eff} {b} = {v}")],
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn routes_and_explains() {
        let mut ctx = Context::default();
        ctx.register(Box::new(Pantry));
        let a = ctx.route(&Query::new("The pantry shelf had 12 jars. We used 5 jars. How many are left?"));
        assert_eq!(a.by, Route::Domain("pantry".into()));
        assert_eq!(a.value, "7");
        assert!(a.why[0].contains("class 'lose'"));
        // negative control: not our domain -> escalation, and the log shows the score
        let b = ctx.route(&Query::new("Why did the fox praise the crow?"));
        assert_eq!(b.by, Route::Escalate);
        assert!(ctx.log.last().unwrap().contains("pantry 0.00"));
    }
}
```

```sh
cargo test --release --manifest-path ai/pantry/Cargo.toml
```

This sketch is deliberately naive. A real module reads the parsed tree from `en` and the quantity
world from `math` (see [math-world.md](math-world.md)), and reads events and states from `world`
(see [story-worlds.md](story-worlds.md)). Its contract stays the same: a coverage score with a
reason, an answer with steps, and `None` rather than a guess.

### 5. The parser on your text (only if needed)

Check parser quality on a few hundred annotated sentences from your domain. Retrain only if the
errors actually hurt the answers. See [retrain-the-parser.md](retrain-the-parser.md) for training,
the license gate, and the domain round-trip check `en rt-text`. If your annotations use another UD
flavour, convert them first (see [dialects-and-converters.md](dialects-and-converters.md)).

### 6. Tests that can fail

- Give every rule or table a unit test with a **negative control**: an input that must *not*
  trigger it, like "an apple lies on the table" for the falling-apple chain.
- Gate tests (seeds parse, no duplicate ids, references resolve) belong in `build.rs` or in
  `cargo test`. Break one on purpose once and watch it go red.

### 7. Measure honestly

1. **Determinism.** Run the baseline twice and the change twice. Each pair must be byte-identical
   (`cmp`). Use `BTreeMap`/`BTreeSet` or sorted vectors anywhere order can affect the result. An
   earlier "gain" in this project turned out to be `HashMap` order noise.
2. **Paired comparison on the same items.** Do not compare two accuracies from different runs on
   different subsets. Keep a per-item result file (`id<TAB>1|0`) for both systems on the same test
   set, then count the items the change fixed and the items it broke:

   ```sh
   join -t $'\t' base.tsv new.tsv | awk -F'\t' '$2==0&&$3==1{f++} $2==1&&$3==0{b++} END{print "fixed",f+0,"broken",b+0}'
   ```

   Test the difference with an exact sign test on the discordant items. `mlab` computes it:

   ```sh
   cargo run --release --manifest-path ai/mlab/Cargo.toml -- \
     -e 'fixed = 19; broken = 6; p = min(1, 2 * binocdf(min(fixed, broken), fixed + broken, 0.5))'
   # p = 0.014633
   ```

   For parsing, `en ctx-report <gold.conllu> base=<f1>,<f2> new=<f1>,<f2> [--boot N]` does this per
   sentence. It runs a paired bootstrap over sentences, averages across seeds, and shows which
   relations and sentence types the difference comes from.
3. **Contrast sets.** Change surface details that must not matter (names, places, animals, number
   values) and keep the logic. A model that understands keeps its score; one that memorized drops.
   For bAbI, `world babi-contrast <src-dir> <dst-dir>` writes such a copy. Rerun `world babi
   <dst-dir> test` and compare. Build the same kind of tool for your domain.
4. **Ablation.** Measure what each component is worth by removing it. Use a copy without the
   component, or `GLOBAL_EXCLUDE=<path-substring>` for seeds. Measure, report, delete the copy.
5. **Report excluded items.** If you report a "clean" number on a filtered subset, the full-test
   number comes first. Publish the list of excluded items, each with its reason, as a TSV next to
   the result (for example `docs/eval/<task>-excluded.tsv`: id, reason, whether the system got it
   right). StepGame shows the pattern: `STEPGAME_EXCLUDED=<file.tsv> world stepgame ...` writes it.
6. **Compare like with like.** Use the same split and the same metric as the published results you
   cite, and say which ones they are.

## Example: one iteration

1. `babi-probe` on a domain sentence prints "verb `restock` has no class at the global level".
2. Add `restock` to the `get` line of `ai/global/seeds/shortcuts/possession.md`, or put it in your
   own class in the domain seeds.
3. `cargo test --release --manifest-path ai/global/Cargo.toml`, then the domain tests.
4. Run the full valid set twice. Then compare per item with the previous run: fixed 14, broken 3,
   sign test p = 0.013. Keep the change. A verb class is a solver feature, so if "broken" had
   outweighed "fixed", you would add the word as a plain `concept` for explanations instead.
5. Report on test once, at a milestone, with the excluded-items list.

## Pitfalls

- **Memorizing in level 3.** If the context level needs a new fact to answer, the fact belongs in a
  seed. A reader that learns surface words from training data will score well on the test and fail
  on contrast sets.
- **Training labels are not truth.** Annotated data contains errors. Prefer rules from grammars,
  papers and dictionaries, and use the examples to *check* rules. Keep an independent gold set and
  watch rules that fire often on gold data.
- **A silent `None` is fine; a confident wrong answer is not.** Let the module escalate with a report
  of what it tried, and turn the report into seeds.
- **Shared dependencies.** Changing `ai/en` or `ai/global` changes every module that depends on
  them. Re-measure all the benchmarks you report, not only your own.
- **Licenses travel with data.** A model trained on non-commercial data is non-commercial. Register
  your sources before training (see [retrain-the-parser.md](retrain-the-parser.md)).
