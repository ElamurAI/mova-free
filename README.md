# mova-free

**Version 0.3** · free for everyone, for any purpose · mascot: the snake 🐍

Mova is a small, fast, transparent language model written in Rust. It does not
guess text: it reads a sentence into a dependency tree, turns the story into a
world (who is where, who has what, what happened, how things relate), answers
from that world, and explains every answer. When it cannot answer, it says why,
and that gap becomes the next thing to learn.

Mova has three levels of logic:

1. **Global** — compiled knowledge: concepts, verb classes, relations
   (inverse, direction vectors, transitivity, time order), values. It is written
   as short markdown *seeds* that `build.rs` compiles into Rust tables, so a rule
   is an integer comparison, not a string match.
2. **Domain modules** — English grammar and parsing, mathematics, pragmatics,
   coreference, story worlds, a small programming language.
3. **Temporary context** — the world of the current text. It never invents
   knowledge: every conclusion goes through the global level, and a missing link
   is reported as a gap.

Everything is deterministic: the same input gives the same output and the same
explanation, on every run, on a CPU.

Mova retrains itself. It induces its own rules from texts and from its own parses,
proposes cheap changes of its rules and settings, tests them on held-out data,
draws conclusions and logs every positive and negative experiment in a journal it
reads before the next one — and it never changes its shipped rules without a gate.
The training code is the same code that ships here; only our run setup stays at home.

## What it can do (measured)

| Task | 0.3 | 0.2 | Others (from the papers) |
|---|---|---|---|
| [bAbI](https://arxiv.org/abs/1502.05698), 20 story-reasoning tasks | **99.8%**, 20 of 20 tasks ≥ 95% | 99.8% | no training on the questions: GPT-3 CoT 86.2, GPT-3 + ASP 99.99; trained on 10k: UT+ACT 99.8, EntNet 99.5 |
| bAbI contrast set (new names, places, things, synonym verbs) | **99.7%**, 20 of 20 | 99.7% | — |
| [StepGame](https://github.com/ShiZhengyan/StepGame), original labels | **96.3%** (k=1 98.2% … k=10 94.7%) | 96.3% | TP-MANN 53.0 (k=1..5); BERT+SpaRTUN 98.6 → 45.7; GPT-3 + ASP 92.6 → 88.3 |
| StepGame corrected (Li, Hogg & Cohn 2024), clean and noise | **100.0% at every k** | — | Map+ASP 100 |
| StepGame contrast set (phrasings the generator never made, zero overlap with training) | **100.0% at every k** | — | the perceptron reader alone: 17.1% |
| [SpartQA-Human](https://github.com/HLR/SpartQA_generation) yes/no | **75.5%** | 60.1% | BERT+Q-Chain 59.4; Llama-3-8B CoT 67.8; PistaQ 75.5; GPT-4 77.6 |
| SpartQA-Human relation questions | **63.6%** | 50.6% | BERT with SpaRTUN 50.6 |
| [SVAMP](https://github.com/arkilpatel/SVAMP), arithmetic word problems | **64.0%** (192 of 300) | 59.7% | GTS 41.0; Graph2Tree+RoBERTa 43.8; PAL 79.4; PoT 85.2 |
| Parsing, LAS on the tales test | **77.38** | 76.85 | — |
| Parsing, LAS on UD English EWT test | **79.86** | 79.82 | UDify 88.5; UDPipe 2.0 91.5; a treebank-tuned LLM 94.1 |

No neural network, no GPU, no training on any test. Our SpartQA yes/no keeps "don't know" in the gold (stricter
than the two-class numbers of others); StepGame's original test has about 10.7% wrong labels, so compare on the
corrected set.

How to reproduce each number: [tests/README.md](tests/README.md).

## New in 0.3

- **A self-retraining SLM with states** ([docs/selfplay.md](docs/selfplay.md)) — a frozen Rust skeleton and a hot
  layer the model writes itself. States form a tree of deltas with rollback (to the last working state, then to the
  original); every state has its own *brain* (one memory cell in plain English, base + change log, crash recovery)
  and receives frozen *decision letters*; a request runs in a state given by its hash (or the last stable one) and
  every answer names both. `world brain serve` / `say` talk to the brain in English.
- **The family** — the training loop as a chat on a Unix socket: Mova Dev decides, Mova Current speaks for the
  current state, Mova State members explore from their own states, Random commenters read the history, the journal
  and the letters. Ideas go into a pool; Dev builds a child state from the best ones; a guard on untouched data
  keeps it or rolls it back. Training runs in its own thread and the channel keeps answering: `status`, `stop`,
  `focus`, `reject`, `pool`, `try`, `propose` (your own rule change), `negative`.
- **Learning on big texts** — samples of 300,000 sentences from the tree store, judged without gold by the
  absurdity matrix, guarded by UD grammar constraints (two subjects, case-marked subjects, vocatives) and by a
  teacher check (a large model judges 20 relabelled words). Rules that proved to be nonsense go to a negative list
  and are not tried again (a rare review after 30 days and more). Honest result: the matrix alone accepted harmful
  changes; the checks reject them; the first verified gain — `nsubj/obj → obl` when the word has a preposition —
  raised both test LAS numbers and is in the shipped rules.
- **Absurdity matrix** ([docs/absurdity.md](docs/absurdity.md)) — a level-1 table of how plausible an event is:
  405 verbs × 601 nouns = 243 346 cells scored 0–4 for the doer and the object, plus a fairy-tale layer. In debug
  mode it finds parse errors (score 4 marks an error in 86–92% of cases on tales); in working mode absurdity that
  survives the domain layer switches on humor and irony.
- **Repairs and rules the model induces itself** ([docs/induce.md](docs/induce.md)) — transformation-based learning
  of tree-repair rules with level-1 features; hidden rules come back (12 of 12) from unlabelled books.
- **StepGame, corrected and contrast** — the reader of new phrasings (direction words, the reference agent by cue
  words) next to the learned perceptron: 100% on the corrected StepGame and on a contrast set written before it.
- **Trees by logical derivation** ([docs/derive.md](docs/derive.md)), **expressions with their real meanings**
  ([docs/pragmatics-expressions.md](docs/pragmatics-expressions.md)), **tree store** (`world store`: trees per model
  version, delta versions, feature indexes, named sets, shuffled chapter-sized blocks).

## Repository

| Folder | What |
|---|---|
| [ai/](ai/) | the model: Rust crates (`en`, `global`, `layers`, `prag`, `coref`, `qagen`, `math`, `latex`, `world`, `coder`, `mlab`, `vfs`) and dialect rules |
| [docs/](docs/) | how it works: the seeds (grammar, errors, knowledge) explained in English |
| [papers/](papers/) | one page per scientific work Mova uses: summary, link, authors, license, how Mova uses it, how much it helped |
| [corpus/](corpus/) | open data Mova is developed and tested on: tales, questions, treebanks, math scripts, contrast sets |
| [tests/](tests/) | unit tests and benchmark commands |
| [train/](train/) | for people and AI agents: how to retrain Mova and rebuild it for your own domain, with code |

Quick start:

```
cd ai/world && cargo build --release
./target/release/world babi-probe "Mary went to the kitchen." "Mary picked up the apple." "Where is the apple?"
```

## License

Code: licensed under either of

- Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE))
- MIT license ([LICENSE-MIT](LICENSE-MIT))

at your option.

Data and trained models keep the licenses of their sources; see
[LICENSE-DATA.md](LICENSE-DATA.md). Nothing in this repository restricts the
purpose of use.

### Contribution

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall be
dual licensed as above, without any additional terms or conditions. Please sign
off your commits (`git commit -s`, Developer Certificate of Origin).

## Feedback 🕊️

Questions, ideas and bug reports: open an issue, or write to ai@elamur.ai.

---

*Developed by silicon artificial minds and biological artificial minds.*

🐍 *The snake is Mova's mascot: a small, quick model that keeps the world of a story in its coils and sheds its skin each time it learns.*
