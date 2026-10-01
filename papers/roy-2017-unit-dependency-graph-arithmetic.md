# Unit Dependency Graph and its Application to Arithmetic Word Problem Solving

**Authors:** Subhro Roy, Dan Roth · **Year:** 2017 · **Venue:** AAAI 2017
**Link:** https://arxiv.org/abs/1612.00969 (arXiv 1612.00969)
**License of the paper:** CC BY 4.0 (arXiv) — https://creativecommons.org/licenses/by/4.0/

## Summary
The paper adds explicit reasoning about units of measure to an arithmetic word-problem solver. It introduces the Unit Dependency Graph (UDG): one node per quantity plus one for the question, where a node can be marked as a rate ("40 miles per hour", or implicitly "each student has 3 books") with a numerator and a denominator unit. Edges state that two nodes share a unit, or that a rate's numerator or denominator matches another node's unit. Two classifiers (for nodes and for edges) predict the graph under joint constrained inference; edge labels are derived automatically from gold solution trees, so only the rate/non-rate decision needed manual annotation for a small set of problems. The graph and the expression tree are then searched jointly with a beam, so that a candidate tree must agree with the units: same units combine by addition or subtraction, a rate links to the question through multiplication or division. The resulting solver (UnitDep) outperformed the systems of its time and was more robust when lexical and template overlap between training and test was reduced.

## How Mova uses it
- `ai/global/seeds/shortcuts/units.md`: a first-level knowledge "seed" that states the core rule (a "X per Y" quantity is a rate; total = rate × number of units; equal units combine only by addition/subtraction) and links concepts such as `rate -> multiply`, `total -> add`.
- `ai/math/src/steps.rs`: the word-problem solver extracts a unit and a "per" (rate) marker for each number from the dependency tree and uses them as non-lexical features of the transition model (`rate:…`, unit agreement between operands, binding to the question).
- A hard version of the constraint was also tried (`MATH_UNITHARD=1`): search forbids ± between different units and × between equal units without a rate.
- Unlike the paper, there are no trained unit/rate classifiers; units come from simple tree heuristics.

## Effectiveness in Mova
- Non-lexical features (verb classes, rates, binding to the question; together with Roy & Roth 2018): SVAMP 44.7% → 48.0% (improvement).
- Hard unit constraint (`MATH_UNITHARD=1`): SVAMP 57.3% → 46.7% — harmful. Cases like "more cups of flour than sugar" subtract different things measured in the same unit, and the unit extraction is too coarse for a hard ban (the paper uses a trained classifier of units and rates). The idea works only as soft features.

---

👨‍🔬💥
