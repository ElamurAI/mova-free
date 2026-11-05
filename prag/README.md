# prag — Mova pragmatics: LLM silver and a transparent SLM

`prag` is Mova's pragmatics: the frames reader (who did what to whom, why, what came next), speech acts and their labeller, implicatures, irony candidates, expressions read by their meaning, and the agreer that checks our labels against open witnesses. [`docs/pragmatics.md`](../docs/pragmatics.md) is the guide to the frames; the implicature, irony and Rational Speech Acts pieces are described in [algorithms from the literature](../docs/pragmatics.md#algorithms-from-the-literature) and the lens in [expressions by their real meaning](../docs/pragmatics.md#expressions-by-their-real-meaning). This README covers the speech-act labeller and the crate's commands.

The SLM learns pragmatics with memory models (something like a graph) next to a perceptron, and the models are compared on the same features.

A separate crate that depends on `en`, not the other way round. It holds:
- **LLM silver** (the teacher) — speech-act labels of sentences of open texts;
- three **SLM models** in Rust behind a shared interface: an averaged perceptron, IGTree (memory as a graph, a prefix tree) and k nearest neighbours (IB1);
- evaluation, predictions in the database and decision traces.

## Schema (`docs/context.md`)

| field | values |
|---|---|
| `form` | `decl` · `q` · `wh` · `imp` · `excl` · `frag` · `intj` · `other` |
| `act` | `assert` · `narrate` · `ask` · `request` · `offer` · `suggest` · `promise` · `thank` · `apologize` · `greet` · `agree` · `disagree` · `evaluate` · `express` · `warn` · `other` |
| `indirect` | 0/1 — the form is not the usual one for the act ("Can you open the window?" — `q`, but `request`) |
| `hedge` | 0/1 — *maybe, I think, sort of, perhaps* |
| `polarity` | `pos` · `neg` · `none` — the speaker's evaluation; grammatical negation by itself is not `neg` |
| `voice` | `narrator` · `character` (direct speech) · `none` (a standalone sentence outside a story) |
| `means` | what was meant, up to ~15 words in English; "-" means nothing beyond the literal. Written only by the LLM |

A sentence with direct speech inside a narrator frame ("‘Pay me my wages,’ said Hans.") is labelled by the speech itself: `voice=character`, the frame is ignored. Full definitions are in the prompt (`src/silver.rs`, `RULES`). Closed sets are enums (`src/schema.rs`).

## Data

Only texts with a commercial licence. The gate is `en::license::commercial_gate`; the sources are added to `en/data/train-licenses.tsv`:
- **`gutenberg-tales`** — fairy tales from the database, public domain:
  - windows — consecutive whole paragraphs of one tale, 10–18 sentences;
  - book shares proportional to size;
  - context for the LLM — paragraphs and 1–2 sentences before the window.
- **`tatoeba-eng-cc0`** — standalone sentences (CC0): one sentence, 1–25 words, different texts. The UD bronze of these sentences (`en`) is loaded into the database as document `tatoeba`. Otherwise `pragmatics` would have nothing to refer to.
- **GUM** (CC BY-NC-SA) — only for evaluating `form` against `s_type`. Negative control of the gate: GUM in training means a stop (test `license_gate_blocks_gum`).

## LLM silver

The teacher is Codex CLI through the shared [`mova-teacher`](../teacher) crate (`src/opus.rs` calls it): model from `MOVA_TEACHER_MODEL` or `PRAG_MODEL` (default `gpt-6.1-terra`), `PRAG_EFFORT` medium by default, no fallback to the Claude API. Sentences go in batches; the output is an 8-column TSV with numbers local to the batch.

**Gates** (`silver::gate`), all with negative controls in tests:
- values only from the closed sets;
- every sentence has a row;
- an extra or repeated number rejects the whole batch, because the alignment is in doubt;
- voice consistent with the source: tale — `narrator|character`; Tatoeba without quotation marks — `none`.

Rejects go to `rejects.tsv`, with no repeated calls.

**Database** (`$MOVA_DATA/db/mova.duckdb`, schema v2 — migration in `tools/db/schema.sql`, command `db migrate`):
- silver: sets `pragmatics / silver / opus`;
- SLM predictions: sets `pragmatics / rust`, one run per model, with decision traces in `pragmatics.trace`;
- provenance is in `runs.params`: model, effort, accounting of calls and tokens;
- view `prag_labels` — labels with source, run and text;
- comparison in SQL — `sql/silver-vs-rust.sql`.

## SLM

**Features** (`src/feats.rs`) — 32 slots with symbolic values, the same for all three models:
- **words**:
  - `w1 w2 w3 wl` — the first three words and the last one;
  - `u1` — UPOS of the first word;
  - `end` — final punctuation;
  - `qm` — "?" in the sentence;
  - `excl` — the construction *what a…! / how…!*;
  - `len` — length.
- **UD tree** (`en::annotate` with the given `en` model):
  - root: `rupos`, `rform` (`Mood=Imp`, `Fin`, `Inf`…), `rlem`, `tense`;
  - `aux1` — auxiliary at the start; `inv` — inversion of auxiliary and subject;
  - `wh` — wh-word at the start; `modal` — modal;
  - `neg` — negation on the root or elsewhere;
  - subject: `subj` (pronoun or part of speech), `sperson` (person), `you`, `p1`;
  - `voc` — vocative.
- **cues** (`data/cues.tsv`) — our own small list after Lakoff 1973, Hyland 1998, Searle 1969/1975 and Brown & Levinson 1987:
  - `please`;
  - `hedge` — a hedge word;
  - `eval` — evaluative words: pos, neg, mix;
  - `cue` — act marker: thank, apol, greet, promise, warn, suggest, offer, yes, no, intj.
- **context**:
  - `quote` — `none`, `whole`, `framed` (with a narrator frame) or `cont` (speech continues from the previous sentence);
  - `sayv` — speech verb in the frame;
  - `ppos` — position in the paragraph;
  - `prev` — act of the previous sentence: in training from the silver, in prediction the model's own prediction;
  - `genre` — tale or Tatoeba.

If a sentence contains direct speech, the words and the tree are taken from the quoted text. Quotes are tracked across the sentences of a paragraph. "’" between letters is an apostrophe. Titles in quotes ("the title of “Gentlemen Weavers.”") are not treated as direct speech.

**Models** (`src/model.rs`) — a shared trait `Classifier` (`predict` → class, confidence, explanation; `bytes`; `info`). The models run side by side on the same features:
- **perceptron** — averaged (Collins 2002), "slot=value" indicators, 12 epochs. Explanation — the 3–5 features contributing most to the margin of the chosen class over the runner-up.
- **IGTree** (Daelemans, van den Bosch & Weijters 1997; TiMBL):
  - slots ordered by gain ratio;
  - each node holds a default label and counts;
  - leaves with the parent's label are pruned;
  - prediction — walking the graph while values match. Explanation — the path walked.
- **k-NN / IB1** (Aha et al. 1991; TiMBL):
  - the whole memory; distance — overlap weighted by gain ratio;
  - the k nearest distances vote, k chosen per field by leave-one-out on training data;
  - explanation — the most similar sentences in memory with the same label.

## Evaluation

`prag eval` splits the silver 80/20 (tales by document, Tatoeba by sentence) and also runs 5-fold cross-validation on the whole silver; the cross-validated predictions are what is written to the database. Each field is compared with the majority-class baseline and with a negative control: the same model trained on shuffled labels must land at baseline level. The report also gives the agreement of the three models per field, the confusion matrix of each model, and `form` against GUM test `s_type` (evaluation only; GUM `ger`, `inf`, `sub` map to `frag`, `multiple` is excluded, GUM has no `excl`).

The perceptron is the main model; memory (k-NN or IGTree) runs alongside, and disagreement of the models marks a sentence for the LLM.

## Decision explanations

Numbers next to perceptron features are the contribution to the margin of the chosen class over the runner-up. In an IGTree path — the node's default label and count.

**Example.** *The Innkeeper said, “Why do you howl so fearfully?”* — silver: `wh`, `ask`, character.
- perceptron → `ask`: `end=?` +17.6 · `w3=you` +16.5 · `qm=yes` +13.7 · `modal=-` +8.7 · `neg=-` +6.2
- IGTree → `ask`, path: root → narrate 790/2448 · `qm=yes` → ask 143/215 · `inv=-` → ask 55/88 · `end=?` → ask 55/87 · … · `sayv=said` → ask 6/7
- k-NN → `ask`, because the nearest are: "‘Where are you going?’ said Rose-red…" (ask, 1.73) · "“And you,” they said, “how many sons have you at a birth?”" (ask, 1.83) · "Said the peasants: ‘Are there any more there?’" (ask, 1.87)

`prag eval` writes more explanations from all three models to `examples.md`.

## How to run

```sh
export PATH=$HOME/.cargo/bin:$PATH
cargo build --release -p prag
P=target/release/prag; R=$MOVA_DATA/runs/<run>; N=<run>; M=$MOVA_DATA/en/models/en.bin
$P select $MOVA_DATA/db/mova.duckdb $MOVA_DATA/annot/src/tatoeba-eng-cc0.tsv $M $R   # items.jsonl, tatoeba-ud.conllu, select.md
$P prompt $R 5                          # batch prompt without a call
$P silver $R --batches 5,37 --jobs 2    # a pilot; then without --batches — the rest (finished batches are skipped)
$P gate $R                              # gates only: silver.tsv, rejects.tsv, silver-params.json
$P eval $R $M --gum $MOVA_DATA/en/ud/ud-treebanks-v2.18/UD_English-GUM/en_gum-ud-test.conllu   # report.md, results.json, examples.md, rust-*.tsv
D=target/release/db; B=$MOVA_DATA/db/mova.duckdb
$D migrate $B
$D load $B bronze ud-2.18 --kind prag --docs idprefix --match auto --run $N:tatoeba-ud --origin $N/tatoeba-ud $R/tatoeba-ud.conllu
$D prag $B $R/silver.tsv --run $N --source silver --tool opus --origin $N/silver --params $R/silver-params.json --path $R
$D prag $B $R/rust-perceptron.tsv --run $N:perceptron --source rust --origin $N/perceptron --params $R/results.json   # likewise igtree, knn
$D sql $B prag/sql/silver-vs-rust.sql
```

Other commands (usage at the top of `src/main.rs`):

```
prag roles <in.conllu> <out.tsv>        the reader's frames as (sent_id, dependent, predicate, role); the input of gparse GP_PRAG
prag label <silver run dir> <en model> <in.conllu> <out.tsv>   the pragmatics SLM labels sentences read as documents
prag agree <labels.tsv> <witness.tsv> <trees.conllu> <out.tsv> <report.md>   the pragmatics agreer
prag world <in.conllu> <out.tsv>        the temporary world before each sentence, for the teacher's prompt
prag lens <en model> <text>             expressions with their real meanings and the domains to switch on
prag infer <en model> <text>            implicatures, presuppositions and irony candidates, each with its reason
prag rsa scalar|reference               Rational Speech Acts on a built-in game
prag lexicon convert <verbnet dir> <propbank dir> <nombank dir> <out dir> [<test-only dir>]
prag lexicon nominals <wordnet dict dir> <out dir>
```

Variables: `PRAG_MODEL`, `PRAG_EFFORT`, `DUCKDB`. Tests: `cargo test --release`. They check:
- gates and closed sets;
- quotation marks;
- selection by whole paragraphs;
- the three models on a rule and the negative control with shuffled labels;
- the licence gate.

## Limits

- **No routing to the LLM.** Model disagreement plus a low margin could trigger a teacher request; the trigger is not built.
  - LLM answers on disagreements would become SLM training data, so the share of LLM calls should fall across versions, though not to zero: new domains and rare phenomena remain.
  - Agreement hides errors: sentences on which all three models agree can still be wrong. A small random sample of agreed sentences would have to go to the LLM too, or the SLM learns errors on which all three models agree: examples are not truth (`docs/context.md`, "Small model and the teacher: division of labour").
- **Memory models are basic:** no TRIBL2 (IGTree on top, IB1 below) or other slot order in IGTree; no feature pairs for the perceptron (`aux1×sperson`, `rform×you`). Gain ratio puts binary slots at the top of IGTree, so lexical cues sit deep where few examples reach.
- **Indirect acts, irony and promises** depend on intent, context and addressee rather than form; the SLM catches the classic "Can you…?" and leaves the rest to the LLM.
- **Rare acts are thin** (thank, apologize, greet, promise, offer, warn): a second silver wave selected by cues is an expensive run and needs the operator's word.
- **`polarity`** has no evaluative words from the silver (pos/neg shares per lemma) and no negation scope over the evaluative word (`neg × eval`).
- **`frag`/`intj`** are not aligned with GUM, where fragments are headings, captions and standalone subordinate clauses; there is no `multiple`.

## Licence

Elamur Cosmic License ([LICENSE.md](../LICENSE.md)). Third-party data keeps its own terms, as noted where it is used.
