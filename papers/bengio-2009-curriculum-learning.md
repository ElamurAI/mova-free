# Curriculum Learning

**Authors:** Yoshua Bengio, Jérôme Louradour, Ronan Collobert, Jason Weston · **Year:** 2009 · **Venue:** ICML 2009
**Link:** https://ronan.collobert.com/pub/2009_curriculum_icml.pdf (author copy; DOI 10.1145/1553374.1553380)
**License of the paper:** author copy, read-only (no open license) — unknown

## Summary
The paper brings a pedagogical intuition into machine learning: present training examples in a meaningful order, from easy to hard, instead of at random. The authors formalise this as a sequence of training distributions that start concentrated on simple examples, gradually reweight towards harder ones and end at the target distribution. They interpret a curriculum as a kind of continuation method: the learner first optimises a smoothed, easier version of a non-convex objective and then moves towards the real one, which can steer it into better local minima. Experiments cover a toy convex setting, shape recognition and a neural language model whose vocabulary is grown step by step. Even simple two-stage curricula speed up convergence and improve generalisation, with the clearest gains on test data, so the curriculum also acts somewhat like a regulariser. The work coined the term and gave the theoretical framing for ordering training data in deep learning.

## How Mova uses it
- ai/math/src/bin/mathsolve.rs, flag `--curriculum` of the `steps` model (word-problem solver as a transition system trained by a structured perceptron, ai/math/src/steps.rs): before normal training the model gets 3 epochs on problems needing one operation, then 3 epochs on problems with at most two, then all problems in the usual order.
- The staging is by number of reduce operations in the gold action sequence, combined with the "worked examples from simple to complex" idea from Sweller.
- It is an optional experiment flag, not part of the default training.

## Effectiveness in Mova
Measured on SVAMP: 57.3% → 55.7% with the staged curriculum, i.e. no gain (recorded as "zero"). A naive variant that simply sorts all training problems by difficulty dropped to 50.0%, because it breaks perceptron averaging. The flag is therefore off by default.

---

👨‍🔬💥
