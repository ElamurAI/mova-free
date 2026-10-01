# CorefUD 1.0: Coreference Meets Universal Dependencies

**Authors:** Anna Nedoluzhko, Michal Novák, Martin Popel, Zdeněk Žabokrtský, Amir Zeldes, Daniel Zeman · **Year:** 2022 · **Venue:** Proceedings of the Thirteenth Language Resources and Evaluation Conference (LREC 2022)
**Link:** https://aclanthology.org/2022.lrec-1.520/ (ACL Anthology 2022.lrec-1.520)
**License of the paper:** CC BY 4.0 (ACL Anthology) — https://creativecommons.org/licenses/by/4.0/

## Summary
Syntax has a shared multilingual annotation standard in Universal Dependencies, but coreference resolution has had no comparable standard: corpora define the phenomena differently and use different formats, so most systems are evaluated only on English OntoNotes. CorefUD is a collection of coreference corpora in several languages converted to one scheme that sits on top of UD morphosyntax (in the CoNLL-U MISC column) and also accommodates related layers such as named entities. Corpora were chosen for open licensing, size, language and annotation-scheme diversity and documentation, then converted, with an 8/1/1 train/dev/test split where none existed. The paper compares in detail which mention properties and link types each source corpus annotates. Zero mentions such as dropped pronouns are represented as UD empty nodes. The authors argue for tying coreference to syntax, since mentions usually coincide with constituents, and present the release as a step towards cross-lingual convergence in the spirit of Universal Anaphora.

## How Mova uses it
- `ai/coref` — `coref resolve` writes its output in the CorefUD format: `Entity=` annotations in MISC with a `# global.Entity = eid-etype-head-other` header, where `head` is the position of the head inside the span and `other` records the sieve that attached the mention.
- Mention spans follow CorefUD conventions (nominal dependents only, preposition outside the span, apposition as a separate mention), and the gold coreference of GUM in UD 2.18 is read in the same format for evaluation.

## Effectiveness in Mova
Not measured separately: CorefUD is the data and output format, not an algorithm. It makes evaluation against GUM possible, where the deterministic resolver reaches CoNLL F1 60.31 on the GUM test (gold trees, head match, without singletons) and 73.04 with singletons.

---

👨‍🔬💥
