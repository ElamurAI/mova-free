# How Is LLM Reasoning Distracted by Irrelevant Context? An Analysis Using a Controlled Benchmark

**Authors:** Minglai Yang, Ethan Huang, Liang Zhang, Mihai Surdeanu, William Wang, Liangming Pan · **Year:** 2025 · **Venue:** arXiv preprint (arXiv:2505.18761)
**Link:** https://arxiv.org/abs/2505.18761 (DOI 10.48550/arXiv.2505.18761)
**License of the paper:** CC BY 4.0 (arXiv) — http://creativecommons.org/licenses/by/4.0/

## Summary
The paper measures how much large language models are thrown off in grade-school math reasoning when a problem contains irrelevant information. Earlier benchmarks such as GSM-IC added a single unrelated sentence without control over its structure. The authors build GSM-DC, where each problem is a directed acyclic graph of dependencies between quantities: the solution path is found by topological sorting, and nodes outside that path serve as controlled distractors before the graph is rendered into text with templates. This lets reasoning depth and amount of noise be varied independently and every reasoning step be checked automatically. Accuracy drops steadily as distractors increase, affecting both the choice of reasoning path and the arithmetic. Models trained on strong distractors were the most robust, and a stepwise tree search guided by a process reward model added further robustness out of distribution.

## How Mova uses it
- `ai/math/src/bin/mathsolve.rs`, flag `--augment`: following the idea of training on noisy problems (together with Anantheswaran et al. 2024), each training problem gets a copy with a sentence containing a number from another problem inserted before the question; the target equation stays the same.
- The adaptation is much cruder than the paper: distractors are random sentences from other problems at a fixed position, not graph-controlled nodes related to the problem.

## Effectiveness in Mova
Measured and harmful: SVAMP 57.3% → 52.7% (same result in two runs). The artificial distractor always sits in the same place, so the model learns the position rather than the principle; in the papers distractors are tailored to the problem (similar object, same topic). The idea is not used by default.

---

👨‍🔬💥
