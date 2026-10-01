# Are NLP Models really able to Solve Simple Math Word Problems?

**Authors:** Arkil Patel, Satwik Bhattamishra, Navin Goyal · **Year:** 2021 · **Venue:** Proceedings of the 2021 Conference of the North American Chapter of the Association for Computational Linguistics: Human Language Technologies (NAACL-HLT)
**Link:** https://aclanthology.org/2021.naacl-main.168/ (ACL Anthology 2021.naacl-main.168; DOI 10.18653/v1/2021.naacl-main.168)
**License of the paper:** CC BY 4.0 (ACL Anthology) — https://creativecommons.org/licenses/by/4.0/

## Summary
The paper tests whether neural math word problem solvers really understand simple one-unknown arithmetic problems. On the standard MAWPS and ASDiv-A benchmarks, models still solve a large share of problems after the question is deleted, and a model that ignores word order reaches surprisingly high accuracy by keying on single cue words. The authors then build SVAMP: 1,000 problems created by hand as variations of 100 ASDiv-A seed problems, covering question sensitivity, reasoning changes (adding or altering information, inverting the operation) and structural invariance (reordering, irrelevant additions). The strongest model they test, Graph2Tree with RoBERTa, solves only 43.8% of SVAMP. The conclusion is that solvers rely on shallow heuristics, and that each number must be tied to its context and to the question.

## How Mova uses it
- SVAMP is the main benchmark of the math solver in `ai/math` (the "steps" model and its judge); nearly every idea tested in the solver is reported as an SVAMP delta.
- `ai/math/src/steps.rs` — question-binding features for the judge, motivated by the paper's finding that models ignore the question: whether content words of the question appear near each number, counts of used/unused numbers with and without such binding, and whether the operand order of a root subtraction or division agrees with the order in the question ("more X than Y" → X − Y).
- The paper's lesson about irrelevant numbers ("distractors") also drives later solver work on ignoring numbers not tied to the question.

## Effectiveness in Mova
Question binding was introduced together with other non-lexical features (verb classes, rates) and measured as a group: SVAMP 44.7% → 48.0%. The question-binding features alone are not measured separately. The current best solver configuration reaches 59.7% on SVAMP (179 of 300).

---

👨‍🔬💥
