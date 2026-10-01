# Instance-Based Learning Algorithms

**Authors:** David W. Aha, Dennis Kibler, Marc K. Albert · **Year:** 1991 · **Venue:** Machine Learning 6(1)
**Link:** https://doi.org/10.1007/BF00153759 (DOI 10.1007/BF00153759)
**License of the paper:** unknown (publisher copyright)

## Summary
The paper frames nearest-neighbour classification as a family of incremental "instance-based" learners that keep stored examples instead of building an abstract model. The baseline, IB1, stores every training instance and classifies a new case by the label of its most similar stored instances under a simple distance. Follow-up variants reduce storage by keeping only instances that were misclassified (IB2) and add a significance test to discard noisy instances (IB3). The authors analyse the trade-offs between storage, noise tolerance and accuracy and compare the algorithms with decision-tree learners on standard benchmark datasets. The main message is that a lazy, memory-based learner can be competitive while staying very simple and making each decision traceable to concrete stored examples.

## How Mova uses it
- `ai/prag/src/model.rs` — the k-NN / IB1 classifier, one of three models behind a shared `Classifier` trait in the pragmatics crate (sentence form, speech act, indirectness, hedging, polarity, voice).
- Adaptation, following the TiMBL tradition: the whole training memory is kept, distance is feature overlap weighted by gain ratio, and the k nearest distance levels vote; k is chosen by leave-one-out on the training data (form 3, act 5, indirect 11, hedge 3, polarity 11, voice 7).
- Transparency: the explanation for a prediction is the list of most similar remembered sentences carrying the same label.

## Effectiveness in Mova
Measured in the development notes of `ai/prag` (accuracy / macro-F1, %). Held-out silver test (538 sentences), k-NN vs majority baseline vs averaged perceptron:
- form 91,6 / 63,5 (baseline 77,3 / 12,5; perceptron 94,1 / 70,9)
- act 67,5 / 29,4 (baseline 33,3 / 3,6; perceptron 68,8 / 30,0)
- indirect 90,1 / 52,5; hedge 98,0 / 77,5; polarity 53,5 / 37,5; voice 98,9 / 98,9

5-fold cross-validation on all silver data (2 986 sentences): form 92,5 / 68,4, act 68,1 / 26,0, indirect 92,5 / 53,9, hedge 97,8 / 73,4, polarity 54,3 / 38,9, voice 98,2 / 98,2. The negative control (same model on shuffled labels) stays at baseline level (e.g. form 77,0 / 12,4). k-NN is close to, but generally below, the perceptron; its value in Mova is example-based explanations.

---

👨‍🔬💥
