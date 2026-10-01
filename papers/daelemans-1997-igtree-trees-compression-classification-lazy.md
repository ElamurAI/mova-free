# IGTree: Using Trees for Compression and Classification in Lazy Learning Algorithms

**Authors:** Walter Daelemans, Antal van den Bosch, Ton Weijters · **Year:** 1997 · **Venue:** Artificial Intelligence Review 11
**Link:** unknown
**License of the paper:** unknown (publisher copyright)

## Summary
The paper introduces IGTree, a compressed approximation of memory-based (nearest-neighbour) learning. Features are ordered once by information gain, and the training instances are stored as a decision tree in which every level tests the next most informative feature. Each node keeps its default (most frequent) class, and branches whose class equals their parent's default are pruned, which shrinks memory considerably. Classification walks down the tree as long as feature values match and returns the default class of the last matched node. The authors show that this gives large speed and memory savings over full instance-based search while losing little accuracy on language-processing tasks. IGTree is one of the algorithms of the TiMBL memory-based learning toolkit.

## How Mova uses it
- `ai/prag/src/model.rs` — IGTree is one of three models behind a shared `Classifier` trait in the pragmatics crate, trained on the same features as the perceptron and k-NN.
- Adaptation: feature slots ordered by gain ratio; each node stores a default label and counts; leaves with the parent's label are pruned; prediction walks the tree while values match.
- Transparency: the explanation of a prediction is the path walked through the tree.

## Effectiveness in Mova
Measured in the development notes of `ai/prag` (accuracy / macro-F1, %). Held-out silver test (538 sentences), IGTree vs majority baseline vs averaged perceptron:
- form 88,7 / 55,8 (baseline 77,3 / 12,5; perceptron 94,1 / 70,9)
- act 65,6 / 25,1 (baseline 33,3 / 3,6; perceptron 68,8 / 30,0)
- hedge **98,5 / 84,2** — best of the three models on this field (perceptron 97,4 / 74,3)
- indirect 88,7 / 54,0; polarity 48,9 / 31,1; voice 98,5 / 98,5

5-fold cross-validation (2 986 sentences): form 89,6 / 64,5, act 64,1 / 23,9, indirect 92,2 / 58,2, hedge 98,0 / 75,5, polarity 49,0 / 31,8, voice 97,6 / 97,6. Shuffled-label negative control stays at baseline level. Overall IGTree trails the perceptron except on hedging in the held-out split.

---

👨‍🔬💥
