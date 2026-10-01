# Towards AI-Complete Question Answering: A Set of Prerequisite Toy Tasks

**Authors:** Jason Weston, Antoine Bordes, Sumit Chopra, Alexander M. Rush, Bart van Merriënboer, Armand Joulin, Tomas Mikolov · **Year:** 2015 · **Venue:** arXiv:1502.05698 (later ICLR 2016)
**Link:** https://arxiv.org/abs/1502.05698 (DOI 10.48550/arXiv.1502.05698)
**License of the paper:** arXiv non-exclusive distribution license — http://arxiv.org/licenses/nonexclusive-distrib/1.0/ (the bAbI data is released under CC BY 3.0)

## Summary
The paper proposes bAbI, a suite of 20 synthetic question-answering tasks meant to work like unit tests for text understanding and reasoning. Stories are produced by a simple world simulator, similar to a text adventure, in which characters move between locations and pick up, drop or hand over objects. Each task targets one skill: retrieving one, two or three supporting facts, argument relations, yes/no questions, counting, lists, negation, coreference, time reasoning, basic deduction and induction, positional and size reasoning, path finding and agents' motivations. Answers are single words or short lists, so scoring is unambiguous, and the data is noise-free so a human can answer everything. As an illustration, the authors extend Memory Networks with adaptive numbers of memory hops, multi-word answers and better sentence representations; the improved models solve many but not all tasks, and a structured-SVM pipeline with external NLP tools fails several. The value of the suite is that passing or failing a given task points to a specific missing ability.

## How Mova uses it
- `ai/world/src/babi.rs` — the `world babi` command solves all 20 tasks with a reader built on the `en` parse tree, verb classes, and a first-level `relation` block, without training on the questions.
- `ai/global/seeds/shortcuts/relations.md` — the relation seed (inverses such as north/south, plane vectors, transitivity of bigger/smaller, *fits inside* = smaller than, order of parts of the day) cites bAbI tasks 4, 14, 17, 18 and 19; facts come from the story, inferences come from this shared seed.
- `world babi-learn` — agents' motivations (hungry → kitchen, etc.) are learned automatically from the training split and written back as a seed.
- `world babi-contrast` — contrast variants in the spirit of Gardner et al. 2020 (new names, places, objects, synonym verbs) test that the reader does not depend on bAbI's fixed vocabulary.

## Effectiveness in Mova
From the "Ideas from papers through the bench" table in Mova's development notes: test **99.8%, 20/20 tasks ≥ 95%**; training split 99.7% (not tuned to). Motivations were learned from the training split (375/375). Majority voting in induction was worse than using the freshest example (97.6% vs 99.5%). On contrast variants accuracy first dropped to 74.6%, exposing parser defects (*attic* tagged ADJ, *Taras* lemmatised as "tara", *Following* as a verb, *is* as a main verb) and two lexicon gaps (*hurry*, *abandon*); after fixing them contrast accuracy reached **99.7%, 20/20**, with no drop on the original (99.8%).

---

👨‍🔬💥
