# Structured Perceptron with Inexact Search

**Authors:** Liang Huang, Suphan Fayong, Yang Guo · **Year:** 2012 · **Venue:** Proceedings of NAACL-HLT 2012
**Link:** https://aclanthology.org/N12-1015/ (ACL Anthology N12-1015)
**License of the paper:** CC BY-NC-SA 3.0 (ACL Anthology, before 2016) — https://creativecommons.org/licenses/by-nc-sa/3.0/

## Summary
The convergence guarantee of the structured perceptron assumes exact search for the best structure, yet practical systems such as incremental parsers and translation decoders use beam search. The authors show that convergence still holds with inexact search as long as every weight update fixes a "violation", i.e. a case where the model scores a wrong partial structure at least as high as the correct one. They formalise this as the violation-fixing perceptron and prove the same mistake bound as for the classic algorithm. The framework explains why standard full-sequence updates with beam search can fail, and shows that Collins and Roark's early update and LaSO are special cases, giving early update its first theoretical justification. They also propose new update strategies; the best, max-violation, updates at the prefix where the gap between the best wrong hypothesis and the gold prefix is largest, and on POS tagging and incremental parsing it matches or beats early update while training about three times faster.

## How Mova uses it
- `ai/math/src/steps.rs` (`Model::run`) — the arithmetic word-problem solver builds expression trees with a beam-search perceptron trained with early update by default.
- Max-violation is implemented as an option (`MATH_MAXVIOL=1`): the gold action prefix is scored alongside the beam, and the update happens at the step where "best beam state minus gold prefix" is largest, instead of the first step where gold falls out of the beam.

## Effectiveness in Mova
Measured on SVAMP (architecture notes, table of ideas from papers): max-violation lowered accuracy from 57,3% to 56,0%, so it stays off by default and early update remains the training regime.

---

👨‍🔬💥
