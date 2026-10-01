# «Are you Afraid of Ghosts?» A Proposal for Busting Predicate Ellipsis in Universal Dependencies

**Authors:** Claudia Corbetta, Federica Iurescia, Marco Carlo Passarotti · **Year:** 2025 · **Venue:** Proceedings of the 23rd International Workshop on Treebanks and Linguistic Theories (TLT, SyntaxFest 2025)
**Link:** https://aclanthology.org/2025.tlt-1.6/ (ACL Anthology 2025.tlt-1.6)
**License of the paper:** CC BY 4.0 — https://creativecommons.org/licenses/by/4.0/

## Summary
The paper deals with predicate ellipsis — a missing verb in a clause — and how to annotate it in Universal Dependencies treebanks. The basic UD layer has no empty nodes, so ellipsis is only worked around: one remaining element is promoted to the head position, or the `orphan` relation links the remnants, and the gap itself is never marked explicitly. The authors propose two complementary workflows: a theoretical one for recognising ellipsis (locating the gap, the remnants and the antecedent) and a practical one for restoring the missing predicate as an empty node in the Enhanced Dependencies layer. The approach is tested on the Italian-Old treebank, which contains Dante's Divine Comedy, and difficult cases are discussed, such as where to place the restored node and what to do when no antecedent is available in context. The authors stress the shortage of gold data with explicit ellipsis for both linguistic analysis and machine learning, and present the work as a first step towards language-independent UD guidelines for ellipsis.

## How Mova uses it
- ai/en/seeds/errors/gapping-orphan.md: rule `en.errors.orphan-head` — in gapping ("Marie went to Paris and Miriam to Prague") an `orphan` must hang on the promoted remnant (a conjunct, root, `advcl` or `parataxis`); otherwise the gapping was analysed wrongly.
- ai/en/seeds/errors/vp-ellipsis-aux-head.md: rule `en.errors.aux-before-verb` — in VP ellipsis ("Mary will too") the stranded auxiliary becomes the clause head; an `aux` placed after its verb signals that the elided clause was wrongly attached as an auxiliary of the first verb.
- The paper is cited for the basic-layer convention (promotion or `orphan`, empty nodes only in Enhanced UD), which these rules check in the expert system (engine ai/en/src/expert.rs).

## Effectiveness in Mova
Not measured separately. The rules it supports are validated on gold treebanks: on UD English EWT 2.18, 29 `orphan` relations with 0 violations (GUM: 128 / 6); `aux` after a verbal head — 2 cases (inversion), GUM — 1. Role: source for one pair of annotation-consistency rules.

---

👨‍🔬💥
