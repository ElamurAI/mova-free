# Cutting Through the Noise: Boosting LLM Performance on Math Word Problems

**Authors:** Ujjwala Anantheswaran, Himanshu Gupta, Kevin Scaria, Shreyas Verma, Chitta Baral, Swaroop Mishra · **Year:** 2024 · **Venue:** arXiv preprint
**Link:** https://arxiv.org/abs/2406.15444 (arXiv 2406.15444; DOI 10.48550/arXiv.2406.15444)
**License of the paper:** CC0 1.0 (arXiv) — http://creativecommons.org/publicdomain/zero/1.0/

## Summary
The paper studies how large language models solving math word problems are thrown off by irrelevant numbers in the problem text. The authors use a two-step prompting chain: a model first invents an unrelated physical quantity (such as volume, temperature or speed) with start and end values, then weaves it into the problem so that the solution and answer stay unchanged; constraints ensure the new variable has a different unit and adds no information about the original quantities. This yields PROBLEMATHIC, a dataset of paired clean and noisy problems at two difficulty levels (simple additive problems and complex ones with nested operations), with hand-written noisy test items, plus GSM-8K-Adv, a noisy variant of GSM8K. Distractors cut accuracy by about a quarter on average across models. Fine-tuning open models on noisy examples with explanations that point out the irrelevant variables improves accuracy on noisy problems by about 8%. The main finding is that models tend to use every number in the problem mechanically.

## How Mova uses it
- Inspired the `--augment` option of the math solver training in ai/math/src/bin/mathsolve.rs: every training problem gets a copy with a sentence containing a number from another problem inserted just before the question, while the target equation stays the same.
- Adapted to Mova's small trained model: instead of LLM-generated, type-aware distractors, a cheap deterministic donor sentence is used; the idea was taken together with a related work (Yang 2025).
- Tested as one idea per run on the project's benchmark stand, comparing against the unchanged baseline.

## Effectiveness in Mova
Measured: SVAMP 57.3% → 52.7% (identical in two runs), i.e. a loss, so the idea was not adopted. Diagnosis: the artificial distractor always sits in the same position, so the model learns the position rather than the principle; in the paper, distractors are tailored to the problem type (similar object, same topic).

---

👨‍🔬💥
