# StepGame: A New Benchmark for Robust Multi-Hop Spatial Reasoning in Texts

**Authors:** Zhengxiang Shi, Qiang Zhang, Aldo Lipani · **Year:** 2022 · **Venue:** AAAI 2022; preprint arXiv:2204.08292
**Link:** https://arxiv.org/abs/2204.08292 (DOI 10.48550/arXiv.2204.08292)
**License of the paper:** CC BY 4.0 (arXiv) — http://creativecommons.org/licenses/by/4.0/ (the dataset is released under MIT)

## Summary
The paper argues that near-perfect scores on the spatial tasks of bAbI (tasks 17 and 19) overstate what models can do, because bAbI uses a handful of fixed phrasings, needs only one or two reasoning steps, and most test pairs also occur in training. StepGame is a question-answering benchmark in which agents are placed on a grid and the model must say where one agent is relative to another, choosing among eight directions plus "overlap". Descriptions of each relation come from crowdsourced templates, so the language is much more varied, and chains of reasoning range from 1 to 10 steps; test stories also contain distracting sentences. Models are trained on shorter chains and evaluated on longer ones to test generalisation. The authors also propose TP-MANN, a recurrent memory network based on tensor-product representations, which beats the neural baselines, although all models lose accuracy quickly as the number of steps grows.

## How Mova uses it
- `ai/world/src/stepgame.rs` — the `world stepgame` command evaluates Mova's world model on this benchmark.
- Sentence reader: a sentence with two agents is mapped to the vector of the first agent relative to the second (9 classes). It is an averaged perceptron on word n-grams with the agents replaced by A1/A2, with integer weights so training is deterministic, and it is trained only on level-1 data (one sentence = one relation).
- World ("third level"): each read relation becomes an edge vector in a graph of agents; the answer is the sum of vectors along the path from the queried agent to the reference agent, and the signs on the two axes give the class. Composition of directions is therefore exact arithmetic, not learned.
- A suspicion detector over the training data flags phrasings that receive contradictory labels; the excluded questions can be listed one by one with reasons: `STEPGAME_EXCLUDED=<file> world stepgame <data dir>`.

## Effectiveness in Mova
From the "Ideas from papers through the bench" table in Mova's development notes: **96.3%** on the full test set (k=1 98.2% → k=10 94.7%), deterministic. The suspicion detector found 2 phrasings that are contradictory in the data (one sentence carries 2–3 different labels, a generator defect); without them accuracy is **98.0–99.6%** on all k, so long chains barely degrade because composition is exact. The headline number remains 96.3% on the full test; 11,553 questions excluded from the "clean" figure are listed with reasons. For comparison, the paper reports far lower neural results (e.g. TP-MANN averaging about 53% for k=1–5), though setups differ (Mova trains its reader only on level 1 and composes symbolically).

---

👨‍🔬💥
