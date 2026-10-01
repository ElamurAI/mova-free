# Mova's UD dialect and converters

## Gist

Treebanks disagree. UD versions rename labels: `obl:tmod` became `obl:unmarked` in 2.15. Single
treebanks keep their own habits. Older conversions (PTB and others) follow other conventions. Mova
fixes one internal dialect, **`mova`**. Every treebank is converted *into* it before training, and
every target is produced *from* it. Quality on any treebank is measured by converting back and
comparing with that treebank's own gold.

Converters are **plain markdown**: fenced ```` ```convert ```` blocks inside `convert.md` files. A
small deterministic engine in `ai/en/src/convert.rs` runs them. The engine reuses the matching
language of the grammar-check seeds (`ai/en/src/expert.rs`) and adds `from`, `to` and `set`.

## What `mova` is

- **Base:** UD 2.18 with the conventions of English EWT. The machine-readable part is the registry
  `ai/en/data/ud-registry-en.tsv` (UPOS x feature pairs, relations, auxiliaries).
- **Layers on top:**
  - `ud-next`: guideline changes after 2.18 that are expected in the next release
    (`ai/dialects/ud-next/`).
  - Borrowings from other dialects.
  - Own normalizations: `det:poss` -> `nmod:poss`; `:tmod`/`:npmod` -> `:unmarked`; verb Number and
    Person from the subject.
- **Choice rule:** take what is more consistent and more informative *and* can be converted back to
  the standard without loss. A lossy convention can be an export target, never the base.
- Each decision is a seed in `ai/dialects/mova/seeds/*.md`, summarized in
  [dialect-mova.md](../docs/dialect-mova.md).

## Layout

| path | holds |
| --- | --- |
| `ai/dialects/<name>/README.md` | what the dialect is and how it differs |
| `ai/dialects/<name>/convert.md` | rules between `mova` and a version dialect (`ud-2.14`, `ud-next`) |
| `ai/dialects/<name>/seeds/*.md` | one decision or difference per file, often with ```` ```rule ```` counters |
| `ai/treebanks/en/<treebank>/convert.md` | rules between `mova` and one treebank's habits (`gum`, `atis`, `childes`, `ewt`, ...) |
| `ai/en/src/convert.rs` | the engine: parser, application order, tree gate, unit tests |

A dialect name is any string without whitespace. The name is just what rules put in `from:` and
`to:`.

## When to use

- Before training `en` on a treebank that is not plain EWT 2.18 (older release, other habits).
- To export Mova's output in another treebank's conventions, or to score against that treebank's gold.
- When you find a systematic difference (a label, a lemma, a feature) and want it fixed everywhere
  by one rule.

## Rule syntax (exactly as parsed)

A rule is one fenced block tagged `convert`. The tag must be followed by a space or the end of the
line, so ```` ```convert-todo ```` blocks are drafts and are ignored. Inside a block every line is
`field: value`. Split happens at the first `:`, each field sits on one line, and fields cannot
repeat. An unknown field is an error. A line without `:` is ignored, so don't put prose inside the
block.

```convert
rule: ud-2.14.obl-unmarked
what: obl:tmod, obl:npmod -> obl:unmarked
from: ud-2.14
to: mova
match: t[rel=obl:tmod|obl:npmod]
set: t[rel=obl:unmarked]
source: UD _en/dep/obl-unmarked.md (History)
```

| field | required | meaning |
| --- | --- | --- |
| `rule` | yes | id, unique across all files loaded together |
| `from`, `to` | yes | dialect names; must differ; no spaces |
| `match` | yes | nodes `name[cond, cond, ...]` separated by `;`; each binds a different word |
| `require` | no | clauses separated by `;`, **all** must hold; alternatives inside a clause joined by ` or ` |
| `unless` | no | same form; the binding is skipped if **any** clause holds |
| `set` | yes | actions `name[action, action]; name[action]` on `match` nodes |
| `source` | yes | where the convention comes from (guideline page, treebank README, issue) |
| `what` | no | one-line human description |

**Conditions on a node** (comma-separated; `lemma`, `form` and affixes compare lowercase; `|` means
"one of"):

- `upos=NOUN|PROPN`, `xpos=VBD` (PTB tags), `lemma=who|whom`, `form=you`, `lemma=_` (no lemma)
- `rel=obl:tmod`: exact relation, which must be a known `Rel` in `ai/en/src/gram.rs`. `rel~nsubj`
  matches the base relation, any subtype.
- `feats.Number=Sing|Plur`: the feature has one of the values. `feats.Case`: the feature is present.
  `!feats.Case`: the feature is absent. `feats.Number=@v`: same value as on node `v`.
- `head=0` (root), `head=v`: the head is node `v`.
- `before=v`, `after=v` (anywhere), `next=v` (right after `v`), `prev=v` (right before `v`).
- `suffix=`, `prefix=`, `lemma.suffix=`, `lemma.prefix=`, `lemma=@form` (lemma equals form).
- Negate any condition with `!=` (`rel!=nmod:poss`). Negate `~` with `!~`.

**Clauses in `require` / `unless`:**

- `v[...]`: matched node `v` also satisfies these conditions.
- `not v[...]`: it does not.
- `exists c[...]`: some word *not bound in `match`* satisfies them.
- `none c[...]`: no such word exists.

Example: `require: none c[rel~nsubj, head=v]; none c[rel=expl, head=v]`.

**Actions in `set`:**

| action | effect |
| --- | --- |
| `rel=nmod` | set DEPREL (a known relation) |
| `upos=ADV` | set UPOS |
| `xpos=NN` / `xpos=_` | set XPOS (PTB tag or `_`) |
| `lemma=who` / `lemma=@form` | set the lemma / lemma = lowercased form |
| `feats+=Case=Acc` | set one feature (replaces its old value; keys stay sorted) |
| `feats-=Case` | remove a feature |
| `feats+=Number=@v` | copy a feature from node `v` (no-op if `v` lacks it) |
| `head=v` / `head=@v` / `head=0` | head becomes node `v` / `v`'s head / root |
| `misc+=Key=Val` / `misc-=Key` | set / remove a MISC key |

Separate actions with `,` or `;` inside the brackets. A value may contain commas after `feats+=`,
`lemma=` or `misc+=` (`feats+=PronType=Int,Rel`).

**How rules run:**

- Files load in alphabetical path order, and blocks in file order. Only rules of the requested
  `from -> to` pair run.
- Each rule finds all bindings on the current sentence state, computes every value from that same
  state, and then applies them together. The next rule sees the result, so order matters: put
  specific rules before the catch-all.
- If one rule sets two different values for the same field of the same word, the run fails with a
  "conflict" error. A FEATS key and a MISC key each count as their own field.
- After all rules, a **tree gate** checks that heads are in range, there is exactly one root, only
  the root is labeled `root`, and there are no cycles.
- Lines that actions don't touch are copied byte for byte: comments, multiword tokens, empty nodes,
  DEPS, MISC, and features the engine doesn't know.
- Any rule that fails to parse in the loaded directory stops the command (fail-fast).

## Steps: add a rule

1. Find the difference with a counter: count it in the data before you change anything.
2. Add a ```` ```convert ```` block to the right file:
   - `ai/treebanks/en/<treebank>/convert.md` for one treebank's habit;
   - `ai/dialects/<name>/convert.md` for a UD version.
   Write both directions when the change is reversible (`x -> mova` and `mova -> x`). Give the
   reverse rules `from: mova`.
3. Run the forward conversion and read the summary on stderr: rule count, sentences, words, changed
   words.
4. Run the round trip `x -> mova -> x` with `convert-check`. Every column should show 0 differences
   unless the reverse is knowingly lossy. Then explain the remainder in the `convert.md` prose.
5. Run the engine's tests. `repo_rules_all_parse` loads every `convert.md` under `ai/dialects` and
   `ai/treebanks` and fails on any parse error or duplicate id.
6. Retrain `en` on the converted data if the rule changes training input
   ([retrain-the-parser.md](retrain-the-parser.md)).

## Example

```sh
# forward: ud-2.14 -> mova, CoNLL-U on stdout, summary on stderr
cargo run --release --manifest-path ai/en/Cargo.toml -- \
  convert ai/dialects/ud-2.14 ud-2.14 mova data/ud/en_eslspok-ud-train.conllu > eslspok-train.conllu

# round trip report (markdown tables: forward changes, what did not come back, per-rule counts)
cargo run --release --manifest-path ai/en/Cargo.toml -- \
  convert-check ai/dialects/ud-2.14 ud-2.14 data/ud/en_eslspok-ud-train.conllu

# a treebank converter: point at its directory and use its dialect name
cargo run --release --manifest-path ai/en/Cargo.toml -- \
  convert ai/treebanks/en/gum gum mova data/ud/en_gum-ud-train.conllu > gum-train.conllu

# engine tests, including "every convert.md in the repo parses"
cd ai/en && cargo test --release convert
```

The first argument is a directory, searched recursively for files named `convert.md` (hidden
directories and `target` are skipped), or a single file. `convert-check` takes the non-`mova`
dialect and needs rules for at least one direction.

On a one-sentence file with `yesterday` as `obl:tmod`, the forward run rewrites it to `obl:unmarked`.
The round trip returns it exactly: 1 of 1 restored, 0 differences in every column.

Related checks:

- `en ud-check <file.conllu>...` reports violations of the English UD 2.18 registry, which is useful
  on converted output.
- `en expert-check <seeds-dir> [--gold <gold.conllu>] <file.conllu>...` counts ```` ```rule ````
  seeds (same matching language, with `severity` instead of `set`) and their violations.

## Pitfalls

- **Unknown subtypes.** A label outside `Rel` (e.g. `det:poss`) cannot appear in a `rel=` condition
  or action. When read, it falls back to its base. Match the base plus a feature instead:
  `t[rel=det]` with `require: t[feats.Poss=Yes]`.
- **Unknown features** are invisible to conditions. In `feats+=` they are a parse error.
- **DEPS is not seen or updated.** Changing DEPREL leaves the enhanced graph stale. Rebuild it
  separately if you need it.
- **Changing a head.** When you set `head=0`, also set `rel=root` in the same rule (and re-label the
  old root). Otherwise the tree gate fails.
- **Commas in `match` values** are not supported. Use `|` lists.
- **Duplicate ids across files.** Ids must be unique among *all* rules loaded from the directory you
  pass. Prefix ids with the dialect (`gum.`, `mova.ud-next.`).
- **License follows the data, not the dialect.** `gum-train.conllu` converted to `mova` is still GUM
  (NC). Keep the source name in the file name so the training gate catches it.
