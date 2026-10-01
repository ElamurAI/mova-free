# Learning To Use Formulas To Solve Simple Arithmetic Problems

**Authors:** Arindam Mitra, Chitta Baral · **Year:** 2016 · **Venue:** Proceedings of the 54th Annual Meeting of the Association for Computational Linguistics (Volume 1: Long Papers)
**Link:** https://aclanthology.org/P16-1202/ (ACL Anthology P16-1202; DOI 10.18653/v1/P16-1202)
**License of the paper:** CC BY 4.0 (ACL Anthology) — https://creativecommons.org/licenses/by/4.0/

## Summary
The paper splits solving addition/subtraction word problems into two steps: recognise which abstract formula describes the story, then turn the filled-in formula into an equation mechanically. Three formulas suffice for this class: part-whole, change (start, end, gains, losses) and comparison (more, less, difference). Each formula is a template with slots; the system enumerates all assignments of the problem's numbers and the unknown to the slots and scores them with a log-linear model trained on formula-plus-variable annotations. Features are hand-crafted logical cues about agreement of type, verb, subject and tense between quantities, plus hyponymy and antonymy from ConceptNet and WordNet. On the AddSub dataset it outperforms the earlier systems it compares against. Limitations include one formula application per problem, modal verbs, missing world knowledge and parser errors.

## How Mova uses it
- `ai/global/seeds/shortcuts/units.md` — the comparison formula is the source of a first-level link "compare → subtract": questions of the form "how many more / fewer" are explained as a difference, i.e. subtraction.
- The link sits next to other concept links (rate → multiply/divide, total → add, share → divide) used by the math solver in `ai/math` to explain its steps in words ("comparison → subtraction") rather than to force a search decision.

## Effectiveness in Mova
Not measured separately. The link was added as part of a batch of concepts from the solver's "teach me" reports (noun categories, comparison → subtraction, share equally → division, rate → division); together they raised step-explanation coverage on SVAMP from 38% to 62% (214 of 344) with accuracy unchanged at 57.3%. The individual contribution of the comparison link is not isolated.

---

👨‍🔬💥
