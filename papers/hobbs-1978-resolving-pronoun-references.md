# Resolving Pronoun References

**Authors:** Jerry R. Hobbs · **Year:** 1978 · **Venue:** Lingua 44(4)
**Link:** https://doi.org/10.1016/0024-3841(78)90006-2 (DOI 10.1016/0024-3841(78)90006-2)
**License of the paper:** unknown (publisher copyright)

## Summary
The paper presents what became known as the Hobbs algorithm, a purely syntactic procedure for finding the antecedent of a pronoun. It walks the parse tree of the current sentence upward from the pronoun and searches candidate noun phrases breadth-first, left to right, while respecting constraints that prevent a pronoun from taking a co-argument as its antecedent; if nothing suitable is found it continues into previous sentences, searching their trees in the same order. Candidates must agree in gender and number. Hobbs evaluated the procedure by hand on several texts and found it surprisingly accurate given that it uses no semantics, and he contrasted it with a more knowledge-based semantic approach. The "naive" algorithm became a long-standing baseline for anaphora resolution.

## How Mova uses it
- `ai/coref/src/sieve.rs` — the candidate order for pronoun resolution follows the spirit of Hobbs: breadth-first, left-to-right tree traversal ordered by depth of the head and then position; first the mention's own clause, then the rest of the sentence, then previous sentences (left to right for pronouns so that subjects come first, right to left for nominal mentions).
- The original paper was not consulted directly; the order is implemented as described in Raghunathan et al. 2010 and Lee et al. 2013, with higher tree nodes first as in Haghighi & Klein 2009.
- Transparency: `coref explain` prints the candidate's rank in the Hobbs order for each resolved pronoun (e.g. "candidate #1 in Hobbs order, 1 sentence back").

## Effectiveness in Mova
Measured on GUM dev with gold trees (the development notes of `ai/coref`), Hobbs order versus pure recency: pronoun precision 64.2% vs 55.4%; CoNLL F1 73.43 / 63.34 vs 72.23 / 61.33 (with / without singletons).

---

👨‍🔬💥
