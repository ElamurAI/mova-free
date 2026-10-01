# STaR: Bootstrapping Reasoning With Reasoning

**Authors:** Eric Zelikman, Yuhuai Wu, Jesse Mu, Noah D. Goodman · **Year:** 2022 · **Venue:** Advances in Neural Information Processing Systems (NeurIPS 2022)
**Link:** https://arxiv.org/abs/2203.14465 (arXiv 2203.14465)
**License of the paper:** arXiv non-exclusive license to distribute — http://arxiv.org/licenses/nonexclusive-distrib/1.0/

## Summary
STaR (Self-Taught Reasoner) is a self-training loop for teaching a language model to produce step-by-step rationales. Starting from a few examples of rationales, the model generates a rationale and answer for each training question; those that lead to the correct answer are kept and the model is fine-tuned on them, and the process repeats. For questions it fails, the model is given the correct answer as a hint and asked to produce a rationale for it ("rationalization"), which adds harder examples to the training set. The approach lets a model learn from its own reasoning without human-written rationales for the whole dataset. A known weakness is that a correct final answer does not guarantee a correct rationale, so wrong reasoning that happens to hit the answer can be reinforced.

## How Mova uses it
- `ai/math/src/bin/mathsolve.rs`, flag `--star N`: GSM8K training problems without an exact gold tree go into a pool; in each round the current (averaged perceptron) solver proposes its top-5 expression trees, and a tree whose value equals the gold answer is turned into an oracle action sequence and added to training data; the model is then retrained.
- The adaptation uses a small transparent transition model and expression trees in place of a neural LM and text rationales; no rationalization step.

## Effectiveness in Mova
Measured, no gain: +440 solutions added over 2 rounds, GSM8K 10.6% → 10.4%. The cause is the known STaR weakness — the answer matches but is computed from the wrong numbers. Not used by default.

---

👨‍🔬💥
