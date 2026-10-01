# Annotating Coordination in the Penn Treebank

**Authors:** Wolfgang Maier, Sandra Kübler, Erhard Hinrichs, Julia Krivanek · **Year:** 2012 · **Venue:** Proceedings of the Sixth Linguistic Annotation Workshop (LAW VI)
**Link:** https://aclanthology.org/W12-3624/ (ACL Anthology W12-3624)
**License of the paper:** CC BY-NC-SA 3.0 (ACL Anthology, pre-2016) — https://creativecommons.org/licenses/by-nc-sa/3.0/

## Summary
Coordination matters for many NLP tasks, but large treebanks such as the Penn Treebank do not mark it explicitly: coordinated phrases carry ordinary labels and a comma between conjuncts has the same tag as any other comma, so conjunction-less coordination is nearly invisible. The authors add an annotation layer to roughly half of the PTB in which every sentence-internal punctuation mark is labelled as coordinating or not. Decisions are made from the tree, essentially by checking whether the neighbours under the lowest common node are like constituents (with special handling for unlike coordination and prenominal coordination). Only about 14% of commas but most semicolons turn out to be coordinating. With the new layer far more coordinate structures, especially those with three or more conjuncts, can be found than by searching for the CC tag alone. The authors see this as a basis for learning coordination boundaries and improving parsing.

## How Mova uses it
- `ai/en/seeds/errors/coordination-structure.md` — cited as background in the note on UD v2 coordination (first conjunct is the head, `cc` attaches to the following conjunct), which carries the rules `en.errors.conj-rightward` and `en.errors.cc-before-conjunct`. The point taken from the paper: in PTB a coordinating comma is indistinguishable from an ordinary one, and only about 14% of commas are coordinating, so commas cannot be treated as conjunction markers.
- No rule is derived from it directly.

## Effectiveness in Mova
Background reading for one seed note. Not measured separately.

---

👨‍🔬💥
