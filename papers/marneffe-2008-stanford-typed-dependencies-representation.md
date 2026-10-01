# The Stanford Typed Dependencies Representation

**Authors:** Marie-Catherine de Marneffe, Christopher D. Manning · **Year:** 2008 · **Venue:** Coling 2008: Proceedings of the workshop on Cross-Framework and Cross-Domain Parser Evaluation
**Link:** https://aclanthology.org/W08-1301/ (ACL Anthology W08-1301)
**License of the paper:** CC BY-NC-SA 3.0 (ACL Anthology, pre-2016) — https://creativecommons.org/licenses/by-nc-sa/3.0/

## Summary
The paper describes Stanford typed dependencies (SD), a dependency scheme designed to make parser output usable by people outside linguistics who need to extract relations from text. It states the design principles: every relation is a binary link between two words, relations should be meaningful and based on traditional grammar, content words should be linked rather than function words where possible, and labels form a hierarchy so that a more general label can be used when the specific one is unclear. SD is compared with the GR and PARC schemes on shared-task sentences; it distinguishes more relation types, makes content words heads (including in copular constructions), and in its collapsed form folds prepositions and coordination into direct links between content words. The authors argue that SD is a suitable gold standard for parser evaluation, closer to downstream needs than bracket-based scores. SD became a de facto standard and the basis of Universal Dependencies.

## How Mova uses it
- `ai/en/seeds/errors/xcomp-no-subject.md` — cited (via Mova's notes on English treebank history) for the observation that control was lost in older SD conversions, as background to the rule `en.errors.xcomp-no-subject` (an `xcomp` must not have its own subject; a clause with its own subject is `ccomp`).
- More generally, SD is the historical source of conventions that old converters and models still reproduce (e.g. `cc` on the first conjunct, labels like `dobj`, `nsubjpass`), which Mova's label registry rejects.

## Effectiveness in Mova
Background reading for one seed rule and for understanding legacy conventions. Not measured separately.

---

👨‍🔬💥
