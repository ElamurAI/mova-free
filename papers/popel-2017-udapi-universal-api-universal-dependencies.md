# Udapi: Universal API for Universal Dependencies

**Authors:** Martin Popel, Zdeněk Žabokrtský, Martin Vojtek · **Year:** 2017 · **Venue:** Proceedings of the NoDaLiDa 2017 Workshop on Universal Dependencies (UDW 2017)
**Link:** https://aclanthology.org/W17-0412/ (ACL Anthology W17-0412)
**License of the paper:** CC BY 4.0 (ACL Anthology) — https://creativecommons.org/licenses/by/4.0/

## Summary
As UD and the CoNLL-U format spread, ad hoc scripts for processing treebanks became a common source of subtle bugs, such as forgetting to renumber nodes after a deletion. Udapi is an open-source framework with an object-oriented API for UD data, implemented in Python (the main version), Perl and Java. Processing is organised as a scenario of sequential blocks over a document–bundle–tree–node hierarchy, usable from code or from the `udapy` command line. It supports visualisation (terminal, HTML, LaTeX), format conversion, querying with arbitrary Python conditions, editing, content validation that goes beyond the official online checks, parsing through UDPipe, and UAS/LAS evaluation. In the authors' benchmark all three implementations were many times faster and far less memory-hungry than the older Treex framework. Blocks from Udapi were used to convert several treebanks to UD v2.

## How Mova uses it
- `ai/coref/src/score.rs` and `ai/coref/src/corefud.rs` — the head of a mention span is computed as in Udapi's `corefud.MoveHead` block: the word whose own head lies outside the span; among several candidates, the highest in the tree, then non-punctuation, then the first. This head is the key for head-based mention matching in the coreference scorer.
- The rule is reimplemented in Rust rather than calling Udapi, so scoring stays self-contained and deterministic.

## Effectiveness in Mova
Not measured separately. The head definition is a small but load-bearing convention: it determines which mentions count as matching in every reported coreference score (for example CoNLL F1 60.31 on GUM test with gold trees, without singletons). Following the shared-task tooling keeps Mova's numbers comparable in method to the CRAC evaluations.

---

👨‍🔬💥
