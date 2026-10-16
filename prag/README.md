# prag — Mova pragmatics: LLM silver and a transparent SLM

`prag` is Mova's pragmatics: the frames reader (who did what to whom, why, what came next), speech acts and their labeller, implicatures, irony candidates, expressions read by their meaning, and the agreer that checks our labels against open witnesses. [`docs/pragmatics.md`](../docs/pragmatics.md) is the guide to the frames.

**The first brief:** train the small model on pragmatics, even though AST is not good enough yet; instead of a perceptron, try memory cells, something like a graph; if the perceptron shows better quality and even speed, that is a strong signal that the little graph is not pulling its weight yet.

A separate crate that depends on `en`, not the other way round. The first wave:
- **LLM silver** (Opus) — 2,986 sentences of open texts;
- three **SLM models** in Rust behind a shared interface: an averaged perceptron, IGTree (memory as a graph, a prefix tree) and k nearest neighbours (IB1);
- evaluation, predictions in the database and decision traces.

## Schema — first cut (`docs/context.md`)

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
- **`gutenberg-tales`** — fairy tales from the database, public domain. 1,999 sentences:
  - 209 windows — consecutive whole paragraphs of one tale, 10–18 sentences;
  - 150 tales, book shares proportional to size: Grimm 831, Andersen 543, Kipling 310, Aesop 304, Potter 11;
  - 719 sentences with quotation marks, i.e. with dialogue; the rest is narration;
  - context for the LLM — paragraphs and 1–2 sentences before the window.
- **`tatoeba-eng-cc0`** — 1,000 standalone sentences (CC0): one sentence, 1–25 words, different texts. The UD bronze of these sentences (`en`) is loaded into the database, run `prag-2026-09-26:tatoeba-ud`, document `tatoeba`. Otherwise `pragmatics` would have nothing to refer to.
- **GUM** (CC BY-NC-SA) — only for evaluating `form` against `s_type`. Negative control of the gate: GUM in training means a stop (test `license_gate_blocks_gum`).

## LLM silver and its cost

Run parameters:
- Opus `claude-opus-5-5`, effort high via `claude -p --settings '{"env":{"CLAUDE_CODE_EFFORT_LEVEL":"high"}}'`; the call works as in `annot/src/llm.rs`, shared module `src/opus.rs`;
- 57 batches of 23–60 sentences, 4 calls in parallel;
- output — 8-column TSV with numbers local to the batch.

| | |
|---|---|
| sentences sent | 2,999 |
| accepted by gates | **2,986** |
| rejects | **13** (0.43 %) |
| calls | 57 (pilot 2 + wave 55), no retries |
| time | ~10 min wall clock (pilot 47 s + wave 9 min 09 s); 2,228 s summed over calls — 0.74 s per sentence |
| tokens | output 204,999 (68 per sentence), cache: read 485,311, write 1,154,458; uncached input 114 |
| `total_cost_usd` estimate | $13.4 (subscription, so this is only an estimate) |

Cache writes are mostly the built-in system prompt of `claude -p`, ~20k tokens per call. Our prompt is ~4k.

**Gates** (`silver::gate`), all with negative controls in tests:
- values only from the closed sets;
- every sentence has a row;
- an extra or repeated number rejects the whole batch, because the alignment is in doubt;
- voice consistent with the source: tale — `narrator|character`; Tatoeba without quotation marks — `none`.

Rejects go to `rejects.tsv`, with no repeated calls. All 13 are about voice:
- 5 tale sentences with `none`: the moral of a fable, a narrator's digression;
- 8 Tatoeba sentences with `narrator`/`character` without quotation marks.

There were no format failures and no number shifts.

**Label distribution** (2,986):
- `form`: decl 78.9 % · imp 6.9 · wh 4.6 · q 4.2 · frag 3.3 · excl 1.0 · intj 0.9 · other 0.2
- `act`: narrate 32.5 % · assert 29.2 · evaluate 9.1 · request 7.1 · ask 5.9 · express 4.9 · disagree 2.6 · suggest 2.1 · warn 1.6 · offer 1.3 · promise 1.3 · agree 1.0 · greet 0.5 · other 0.5 · thank 0.1 · apologize 0.1
- `indirect` 7.3 % · `hedge` 2.7 % · `polarity`: none 52.6 · neg 28.5 · pos 19.0 · `voice`: narrator 34.3 · none 33.1 · character 32.6
- `means` is not "-" in 1,404 sentences (47 %). Indirect acts: request 55, evaluate 48, suggest 26, disagree 21, assert 19, express 15, offer 13…

**Database** (`$MOVA_DATA/db/mova.duckdb`, schema v2 — migration in `db/schema.sql`, command `db migrate`):
- run `prag-2026-09-26`, sets `pragmatics / silver / opus`: 2,986 sets and rows;
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
- **UD tree** (`en::annotate`, model `ud-ewt-eslspok.bin`):
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
  - the k nearest distances vote, k chosen by leave-one-out on training data: form 3, act 5, indirect 11, hedge 3, polarity 11, voice 7;
  - explanation — the most similar sentences in memory with the same label.

## Results

### Held-out silver (80/20: tales split by document, Tatoeba by sentence; test 538)

Accuracy / macro-F1, %. Baseline — majority class of the training set. Control — the same model on shuffled labels: it must land at baseline level.

| field | baseline | perceptron | IGTree | k-NN | control: perceptron | IGTree | k-NN |
|---|---|---|---|---|---|---|---|
| form | 77.3 / 12.5 | **94.1 / 70.9** | 88.7 / 55.8 | 91.6 / 63.5 | 76.4 / 12.4 | 74.3 / 14.0 | 77.0 / 12.4 |
| act | 33.3 / 3.6 | **68.8 / 30.0** | 65.6 / 25.1 | 67.5 / 29.4 | 24.5 / 5.4 | 29.2 / 5.7 | 28.1 / 5.2 |
| indirect | 90.5 / 47.5 | 89.8 / **57.4** | 88.7 / 54.0 | 90.1 / 52.5 | 90.3 / 47.5 | 89.0 / 48.7 | 90.5 / 47.5 |
| hedge | 96.8 / 49.2 | 97.4 / 74.3 | **98.5 / 84.2** | 98.0 / 77.5 | 96.8 / 49.2 | 96.3 / 49.1 | 96.8 / 49.2 |
| polarity | 51.5 / 22.7 | **56.1 / 48.2** | 48.9 / 31.1 | 53.5 / 37.5 | 46.8 / 32.2 | 46.5 / 27.9 | 47.2 / 26.8 |
| voice | 33.3 / 16.6 | **99.6 / 99.6** | 98.5 / 98.5 | 98.9 / 98.9 | 34.4 / 34.5 | 31.6 / 27.5 | 32.9 / 32.3 |

The negative control goes red as it should: with shuffled labels the accuracy of all three models is at baseline level (no more than 1.1 pp above) or below, far from the real models.

### Cross-validation on the whole silver (5 folds, 2,986 sentences)

A test of 538 sentences is small for rare acts, so cross-validation gives a more stable comparison. Each sentence is predicted by a model that has not seen it. These predictions are what is written to the database: sets `pragmatics / rust`, runs `prag-2026-09-26:perceptron|igtree|knn`, with decision traces in `pragmatics.trace`.

| field | baseline | perceptron | IGTree | k-NN |
|---|---|---|---|---|
| form | 78.9 / 11.0 | **92.7 / 70.4** | 89.6 / 64.5 | 92.5 / 68.4 |
| act | 28.9 / 3.9 | **70.3 / 38.0** | 64.1 / 23.9 | 68.1 / 26.0 |
| indirect | 92.7 / 48.1 | 92.3 / **59.9** | 92.2 / 58.2 | 92.5 / 53.9 |
| hedge | 97.3 / 49.3 | **98.1 / 76.3** | 98.0 / 75.5 | 97.8 / 73.4 |
| polarity | 52.6 / 23.0 | **55.4 / 45.7** | 49.0 / 31.8 | 54.3 / 38.9 |
| voice | 29.4 / 27.4 | **98.4 / 98.4** | 97.6 / 97.6 | 98.2 / 98.2 |

### Speed and size (all 6 fields together)

| model | training | prediction per sentence | size |
|---|---|---|---|
| perceptron | 0.12 s | 0.016 ms | 906 KB (dense f32 weights, ~4,700 features) |
| IGTree | 0.01 s | **0.008 ms** | **72 KB** (3,800 nodes) |
| k-NN | 3.9 s (choosing k) | 1.6 ms | 1,880 KB (the whole memory) |

Shared feature extraction takes **0.45 ms per sentence**: tokens, the `en` tagger and parser, cues. The `en` model loads in 0.11 s. The whole `prag eval` takes ~40 s: three models, controls, 5 folds and GUM.

### Agreement of perceptron and memory — the "ask the LLM" signal

Cross-validated, 2,986 sentences. "All three" means agreement of the three models; in that case the last two columns show the perceptron's accuracy outside agreement.

| field | pair | agreement, % | accuracy when agreeing, % | disagreeing | perceptron when disagreeing, % | memory when disagreeing, % |
|---|---|---|---|---|---|---|
| form | perceptron ~ k-NN | 94.7 | 95.3 | 159 | 45.3 | 41.5 |
| form | all three | 90.0 | 96.3 | 298 | 59.7 | — |
| act | perceptron ~ IGTree | 73.2 | 80.9 | 800 | 41.1 | 18.1 |
| act | perceptron ~ k-NN | 77.6 | 80.5 | 670 | 34.8 | 25.1 |
| act | all three | **67.2** | **84.5** | 979 | **41.2** | — |
| polarity | all three | 57.3 | 61.8 | 1,275 | 46.7 | — |
| indirect | all three | 95.9 | 93.9 | 121 | 54.5 | — |

Act:
- the three agree on two thirds of the sentences, and there accuracy is 84.5 %;
- on the remaining 979 sentences (33 %) the perceptron is right only 41 % of the time;
- these sentences hold 576 of the 887 act errors, i.e. 65 %.

Model disagreement is a natural "ask the LLM" trigger: a third of the sentences go to Opus and take two thirds of the errors with them. On disagreements the perceptron is mostly more accurate than memory (exception — `indirect`: 46.8 vs 53.2 for k-NN), so on disagreement it is the one to trust, or the LLM.

Full tables (all pairs, confusion of each model) — `$MOVA_DATA/runs/prag-2026-09-26/report.md`.

### Confusion matrix for act — perceptron, cross-validated

Rows — silver, columns — prediction; the 12 most frequent of the 16 acts are shown.

| silver \ prediction | assert | narrate | ask | request | offer | suggest | promise | agree | disagree | evaluate | express | warn |
|---|---|---|---|---|---|---|---|---|---|---|---|---|
| **assert** | 706 | 40 | 6 | 18 | 4 | 2 | 5 | 4 | 12 | 49 | 24 | 3 |
| **narrate** | 19 | 938 | · | 3 | · | · | · | · | 1 | 5 | 2 | · |
| **ask** | 3 | 1 | 153 | 4 | 2 | · | · | 1 | 1 | 9 | 2 | · |
| **request** | 32 | 8 | 6 | 112 | 5 | 11 | 6 | 2 | 5 | 12 | 9 | 2 |
| **offer** | 8 | · | 5 | 6 | 3 | 2 | 6 | · | 1 | 4 | 1 | 4 |
| **suggest** | 17 | 1 | · | 19 | · | 13 | 1 | 1 | 2 | 5 | 2 | 3 |
| **promise** | 13 | · | · | 5 | 3 | · | 10 | 3 | 1 | 1 | · | 3 |
| **agree** | 7 | · | · | 3 | 2 | 2 | 2 | 9 | · | 2 | 2 | · |
| **disagree** | 28 | · | 2 | 1 | 2 | 2 | · | 1 | 30 | 8 | 3 | 2 |
| **evaluate** | 139 | 8 | 18 | 11 | 1 | 5 | · | · | 7 | 63 | 18 | 2 |
| **express** | 50 | 3 | 9 | 5 | 1 | 1 | · | 2 | 4 | 18 | 51 | 2 |
| **warn** | 15 | · | 1 | 10 | 1 | 3 | 5 | · | · | 4 | 5 | 5 |

What the matrix shows:
- the model recognises `narrate`, `assert`, `ask` and direct `request` by form;
- the main pit is `evaluate` and `express`, which become `assert` (139 and 50);
- `offer`, `suggest`, `warn` and `promise` get confused with `request` and `assert`: the difference is in intent, not syntax;
- k-NN barely sees `agree`/`disagree` (0 and 2 hits vs 9 and 30 for the perceptron): short "No!" and "Yes" drown among neighbours in memory.

### `form` vs GUM test `s_type` (evaluation only)

Label mapping:
- decl, q, wh, imp, intj, frag, other — one to one;
- ger, inf, sub → `frag` (no finite main clause);
- `multiple` (51) — excluded from the comparison: we have one form per utterance;
- GUM has no `excl`.

We compare 1,413 of 1,464 sentences.

| model | accuracy | macro-F1 | accuracy on decl/q/wh/imp |
|---|---|---|---|
| baseline (all decl) | 69.4 | 11.7 | 88.5 |
| perceptron | **76.1** | **46.7** | **95.1** |
| IGTree | 72.5 | 34.6 | 92.2 |
| k-NN | 75.2 | 45.3 | 95.1 |

On the main sentence types the transfer from tales and Tatoeba to GUM genres is good: 95 %. The main discrepancy is in the definition of a fragment:
- 208 of 235 GUM `frag` (together with `sub`, `ger`, `inf`) are called `decl` by the model. In GUM these are headings, captions and standalone subordinate clauses ("Because it was cold."), while in our silver fragments are mostly dialogue lines;
- GUM `intj` ("Yeah.", "Okay.") is mostly `frag` for us: that is how the LLM labelled it under our definition.

To compute GUM metrics, the `frag`/`intj` definitions need to be aligned with the GUM scheme, or labels mapped at the schema level.

## Decision examples (test and cross-validated predictions)

Numbers next to perceptron features are the contribution to the margin of the chosen class over the runner-up. In an IGTree path — the node's default label and count.

**Correct, all three.** *The Innkeeper said, “Why do you howl so fearfully?”* — silver: `wh`, `ask`, character.
- perceptron → `ask`: `end=?` +17.6 · `w3=you` +16.5 · `qm=yes` +13.7 · `modal=-` +8.7 · `neg=-` +6.2
- IGTree → `ask`, path: root → narrate 790/2448 · `qm=yes` → ask 143/215 · `inv=-` → ask 55/88 · `end=?` → ask 55/87 · … · `sayv=said` → ask 6/7
- k-NN → `ask`, because the nearest are: "‘Where are you going?’ said Rose-red…" (ask, 1.73) · "“And you,” they said, “how many sons have you at a birth?”" (ask, 1.83) · "Said the peasants: ‘Are there any more there?’" (ask, 1.87)

**Indirect request.** *The Hare said to him, “I wish you would act sincerely by me, and show yourself in your true colors.”* — silver: `decl`, `request`, indirect=1; means: "the Hound should stop acting two ways and show his true intent".
- perceptron → `express`: `subj=i` +11.1 · `w2=wish` +10.2 · `rlem=wish` +10.2 · `w1=i` +7.5 · `prev=narrate` +5.3 — it sees the literal "I wish";
- IGTree → `request`, but indirect=0, path: … `sayv=said` → request 14/43 · `you=yes` → request 6/21 · `quote=framed` → request 6/21 · `w1=i` → request 3/6. A hit based on 3–6 examples is luck rather than knowledge;
- k-NN → `express`, because the nearest are: "…he said, ‘I wish I had someone to bring the cart after me…’" (express, 1.05) · "I wish you had been made to run the soles off your shoes!’" (express).

An indirect "I wish you would…" as a request is a job for the LLM. So far the SLM has learned only the classic "Can you…?":

**Indirect request where there is none (teasing).** *‘Can your name be RUMPELSTILTSKIN?’ said the lady slyly.* — silver: `q`, `ask`, hedge=1; means: "she already knows his name and is teasing him".
- perceptron → `request`, indirect=1: `wh=-` +15.0 · `modal=can` +11.0 · `you=yes` +9.1; for indirect — `qm=yes` +11.5;
- that is, "can + you + ?", the learned cue of an indirect request, fired on "Can **your name** be…";
- IGTree → `assert`, k-NN → `assert`.

**A perceptron error that memory fixes.** *How magnificent was the sight from this height!* — silver: `excl`, `evaluate`, narrator.
- perceptron → `decl`, `narrate`: `quote=none` +21.0 · `genre=tale` +20.8 · `sayv=-` +14.2 · `tense=Past` +10.9. Strong narration features outweighed the rare `excl=yes`;
- k-NN → `excl`, `evaluate`, because the nearest is: "‘What a noble heart that good man has!’ thought he." (evaluate, 2.23).

**An error of all three.** *I will climb up yonder rock.”* — silver: `promise`.
- all say `assert`;
- perceptron: `you=-` +12.0 · `aux1=-` +8.4 · `quote=cont` +8.2 — the cue `cue=promise` ("I will") is weak, because "I will…" in the silver is more often an assertion;
- k-NN: nearest are "I can climb trees." · "I can crack shells." (assert).

What distinguishes a promise from an intention is context and addressee, not form.

**Irony.** *…but the horse let him sing on, and made his way quietly over the fields…* — silver: `narrate`, indirect=1; means: "the lion's furious roaring is ironically called singing".
- all three: `narrate`, indirect=0 (perceptron: `genre=tale` +23.9 · `quote=none` +22.8).

Irony is beyond the SLM with these features; it stays with the LLM.

More examples with full explanations from all three models — `$MOVA_DATA/runs/prag-2026-09-26/examples.md`.

## Conclusion by field

| field | quality | speed | who |
|---|---|---|---|
| `form` | perceptron 92.7 ≈ k-NN 92.5 > IGTree 89.6 (cross-validated) | IGTree 2× faster than the perceptron, k-NN 100× slower | **perceptron** |
| `act` | perceptron 70.3 / 38.0 > k-NN 68.1 / 26.0 > IGTree 64.1 / 23.9 | same | **perceptron**; k-NN as a second opinion |
| `indirect` | nobody beats the baseline on accuracy (92.2–92.5 vs 92.7); macro-F1 is best for the perceptron (59.9) | — | **LLM**; the SLM only catches "Can you…?" |
| `hedge` | tie: 98.0–98.1 (cross-validated); on the test set IGTree 98.5 / 84.2 | IGTree is the cheapest (86 nodes) | **IGTree or perceptron** — equal |
| `polarity` | weak: perceptron 55.4 / 45.7 vs baseline 52.6 / 23.0; IGTree below baseline | — | **LLM** for now; the SLM only as a hint |
| `voice` | ~98 % for all: genre + quotation marks | — | **anyone**, IGTree is cheapest |

**Honestly, by the stated criterion.** The perceptron wins on quality for `form`, `act`, `polarity`, `voice`. On `hedge` it is a tie; on `indirect` nobody wins.

The little IGTree graph is faster at bare prediction (0.008 vs 0.016 ms) and 12 times smaller. But prediction for every model except k-NN is pennies next to the shared feature extraction (0.45 ms), so the speed advantage gives nothing in practice. Therefore **the little graph is not pulling its weight yet**.

The reason is visible in the IGTree paths. Gain ratio puts binary slots at the top (`qm`, `inv`, `end`, `genre`…), while lexical cues (`w1`, `rlem`) end up deep down, where only a few examples reach.

k-NN is almost level with the perceptron on `form` and useful as a second opinion. But it is 100 times slower and loses rare short acts.

Practical summary:
- the main model is the perceptron;
- memory (k-NN or IGTree) runs alongside for agreement;
- disagreement goes to the LLM.

## How to run

```sh
export PATH=$HOME/.cargo/bin:$PATH
cd ~/Mova/prag && cargo build --release
P=~/Mova/target/release/prag; R=$MOVA_DATA/runs/prag-2026-09-26; M=$MOVA_DATA/en/models/en.bin
$P select $MOVA_DATA/db/mova.duckdb $MOVA_DATA/annot/src/tatoeba-eng-cc0.tsv $M $R   # items.jsonl, tatoeba-ud.conllu, select.md
$P prompt $R 5                          # batch prompt without a call
$P silver $R --batches 5,37 --jobs 2    # pilot; then without --batches — the rest (finished batches are skipped)
$P gate $R                              # gates only: silver.tsv, rejects.tsv, silver-params.json
$P eval $R $M --gum $MOVA_DATA/en/ud/ud-treebanks-v2.18/UD_English-GUM/en_gum-ud-test.conllu   # report.md, results.json, examples.md, rust-*.tsv
D=~/Mova/target/release/db; B=$MOVA_DATA/db/mova.duckdb
$D migrate $B
$D load $B bronze ud-2.18 --kind prag --docs idprefix --match auto --run prag-2026-09-26:tatoeba-ud --origin prag-2026-09-26/tatoeba-ud $R/tatoeba-ud.conllu
$D prag $B $R/silver.tsv --run prag-2026-09-26 --source silver --tool opus --origin prag-2026-09-26/silver --params $R/silver-params.json --path $R
$D prag $B $R/rust-perceptron.tsv --run prag-2026-09-26:perceptron --source rust --origin prag-2026-09-26/perceptron --params $R/results.json   # likewise igtree, knn
$D sql $B sql/silver-vs-rust.sql
```

Variables: `PRAG_MODEL` (claude-opus-5-5), `PRAG_EFFORT` (high), `DUCKDB`. Tests: `cargo test --release`. They check:
- gates and closed sets;
- quotation marks;
- selection by whole paragraphs;
- the three models on a rule and the negative control with shuffled labels;
- the licence gate.

## What next

1. **Routing to the LLM:** model disagreement plus a low margin → an Opus request. Measure the trigger's precision and recall, and its token cost per thousand sentences.
   - **Flywheel.** At some point the third will start to shrink. LLM answers on disagreements become SLM training data, so disagreement decreases. We measure the share of LLM calls per thousand sentences across versions — this is also the "data-centre offloading" curve. It will not reach zero: new domains and rare phenomena.
   - **Agreement audit.** Agreement hides errors: on the act field 15.5% of agreed sentences are wrong. So a small random sample of agreed sentences also goes to the LLM. Otherwise the SLM will learn errors on which all three models agree: examples are not truth (`docs/context.md`, "Small model and Opus: division of labour").
2. **Memory that pulls its weight:** TRIBL2 (IGTree on top, IB1 below) or a different slot order in IGTree. For the perceptron — feature pairs (`aux1×sperson`, `rform×you`).
3. **A second silver wave for rare acts** (thank, apologize, greet, promise, offer, warn): selection by cues, ~2–3k sentences. Only on assignment: this is an expensive run.
4. **`polarity`:** evaluative words from the silver itself (pos/neg shares per lemma), negation scope over the evaluative word (`neg × eval`).
5. **Align `frag`/`intj` with GUM** and maybe add `multiple`.
6. **A shared LLM-call crate** for `annot` and `prag` (`src/opus.rs` is already generic).

## Licence

Elamur Cosmic License ([LICENSE.md](../LICENSE.md)). Third-party data keeps its own terms, as noted where it is used.
