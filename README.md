# mova-free

**Version 0.2** · free for everyone, for any purpose · mascot: the snake 🐍

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

## What it can do (measured)

| Task | Result | Notes |
|---|---|---|
| [bAbI](https://arxiv.org/abs/1502.05698), 20 story-reasoning tasks | **99.8%**, 20 of 20 tasks ≥ 95% | no training on the questions; one fact (states → places) learned from the training split |
| bAbI contrast set (new names, places, things, synonym verbs) | **99.7%**, 20 of 20 | first run 74.6% exposed parser errors and two vocabulary gaps; fixed in the reader, not by tuning on test |
| [StepGame](https://github.com/ShiZhengyan/StepGame), relative positions over 1–10 steps | **96.3%** (k=1 98.2%, k=10 94.7%) | 98.0–99.6% on questions not touched by two expressions the dataset labels inconsistently; Mova finds those itself |
| [SpartQA-Human](https://github.com/HLR/SpartQA_generation), spatial questions on human-written scenes | yes/no **60.1%** (68.5% two-class), relation questions 50.6% | majority baseline 34.9%; no neural network, no training on this data |
| [SVAMP](https://github.com/arkilpatel/SVAMP), arithmetic word problems | **59.7%** (179 of 300) | step-by-step solver plus a world reader as a feature for its judge |

How to reproduce each number: [tests/README.md](tests/README.md).

## Repository

| Folder | What |
|---|---|
| [ai/](ai/) | the model: Rust crates (`en`, `global`, `layers`, `prag`, `coref`, `qagen`, `math`, `latex`, `world`, `coder`, `mlab`, `vfs`) and dialect rules |
| [docs/](docs/) | how it works: the seeds (grammar, errors, knowledge) explained in English |
| [papers/](papers/) | one page per scientific work Mova uses: summary, link, authors, license, how Mova uses it, how much it helped |
| [corpus/](corpus/) | open data Mova is developed and tested on: tales, questions, treebanks, math scripts |
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
