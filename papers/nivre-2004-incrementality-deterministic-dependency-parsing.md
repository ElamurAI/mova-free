# Incrementality in Deterministic Dependency Parsing

**Authors:** Joakim Nivre · **Year:** 2004 · **Venue:** Proceedings of the ACL Workshop on Incremental Parsing: Bringing Engineering and Cognition Together
**Link:** https://aclanthology.org/W04-0308/ (ACL Anthology W04-0308)
**License of the paper:** unknown

## Summary
The paper studies how incremental a deterministic, left-to-right dependency parser can be, in the sense of keeping the partially built structure connected as words arrive. It formalises parsing as a transition system over a stack and an input buffer, and analyses the arc-standard style of transitions (shift, left-arc, right-arc) and its relation to an arc-eager variant. The author shows that strict incrementality cannot be achieved for all structures in a purely bottom-up arc-standard system, while the arc-eager strategy allows a higher degree of incrementality. The analysis connects parsing algorithms to questions of human sentence processing and provides the formal basis for many later transition-based parsers trained with classifiers and static oracles.

## How Mova uses it
- `ai/en/src/parse.rs` — the dependency parser is a table-driven arc-standard transition automaton: state = stack + buffer, actions SHIFT / LEFT(label) / RIGHT(label).
- Decisions come from tables "feature → action weight", trained with an averaged perceptron against a static oracle derived from UD trees (the transition framework from this paper; feature templates simplified from Zhang & Nivre 2011).
- Adapted into two stages, action type (3 classes) and then the UD label only for arcs, to keep tables small; features are integers (template number + word/tag codes).
- Extended with beam search (8 hypotheses) and early update (Zhang & Clark 2008).

## Effectiveness in Mova
The transition system itself is not compared against alternatives (e.g. arc-eager), so its share is not measured separately. Parser results from the `en` README and `ai/en/src/parse.rs`, UD English EWT test UAS/LAS: 86.9 / 83.8 with gold tags; 84.2 / 79.8 with Mova's own tags; greedy 83.4 / 79.0; greedy with TnT tags 82.3 / 77.3; PUD 81.5 / 76.3, GUM 81.5 / 76.2; about 58k tokens/s. Adding the beam raised EWT test LAS from 79.00 to 79.77 (dev 77.33 → 79.17, PUD 74.48 → 76.33).

---

👨‍🔬💥
