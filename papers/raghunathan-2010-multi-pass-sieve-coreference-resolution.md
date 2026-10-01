# A Multi-Pass Sieve for Coreference Resolution

**Authors:** Karthik Raghunathan, Heeyoung Lee, Sudarshan Rangarajan, Nathanael Chambers, Mihai Surdeanu, Dan Jurafsky, Christopher Manning · **Year:** 2010 · **Venue:** Proceedings of the 2010 Conference on Empirical Methods in Natural Language Processing (EMNLP)
**Link:** https://aclanthology.org/D10-1048/ (ACL Anthology D10-1048)
**License of the paper:** CC BY-NC-SA 3.0 (ACL Anthology, before 2016) — https://creativecommons.org/licenses/by-nc-sa/3.0/

## Summary
The paper proposes an unsupervised, deterministic coreference system built as a sequence of "sieves": independent rule-based passes applied from the most precise to the least precise. Each pass sees the clusters built by earlier passes, so later, weaker rules can use attributes shared across a whole entity cluster rather than a single mention. Early passes handle high-confidence cases such as exact string match and appositive or predicate-nominative constructions; later passes relax head matching and finally resolve pronouns under agreement constraints. Candidate antecedents are ordered by a syntactic tree traversal in the spirit of Hobbs. The authors report that this simple, modular design was competitive with or better than learned systems of the time on standard benchmarks; it became the basis of the Stanford deterministic coreference system.

## How Mova uses it
- `ai/coref/src/sieve.rs` — the overall architecture: an ordered list of sieves from most to least precise (speaker, exact, relaxed, appos, pred, relpron, acronym, reflexive, strict head-match variants, proper_head, pronoun), each attaching only the first mention of its cluster and skipping indefinite mentions except on exact match.
- Candidate ordering in the Hobbs style, taken from the description in this paper and its follow-up (Lee et al. 2013): breadth-first left-to-right over the tree, own clause first, then the rest of the sentence, then previous sentences (left-to-right for pronouns, right-to-left for nominals).
- Cluster-level features: a cluster's number, gender, animacy and person are the union of its mentions' features, so pronoun agreement is checked against the whole entity.
- Every link carries the id of the sieve that made it, which `coref explain` prints as the reason.

## Effectiveness in Mova
Measured on GUM test (UD 2.18), head match, CoNLL F1 with / without singletons, gold trees: the default sieve (12 passes) scores 73.04 / 60.31, versus exact string match only 53.07 / 27.88 and "pronoun → nearest agreeing mention" 57.73 / 32.67; on Mova's own predicted trees 67.90 / 54.03. On dev, Hobbs-style candidate ordering beats pure recency: pronoun precision 64.2% vs 55.4%, CoNLL 73.43 / 63.34 vs 72.23 / 61.33. Two relaxed sieves were disabled because their precision on dev was too low (strict_c 31%, relaxed_head 24%).

---

👨‍🔬💥
