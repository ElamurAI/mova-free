# Findings of the Fifth Shared Task on Multilingual Coreference Resolution: Expanding Datasets for Long-Range Entities

**Authors:** Michal Novák, Miloslav Konopík, Anna Nedoluzhko, Martin Popel, Ondrej Prazak, Jakub Sido, Milan Straka, Zdeněk Žabokrtský, Daniel Zeman · **Year:** 2026 · **Venue:** Proceedings of the 2nd Joint Workshop on Computational Approaches to Discourse, Context and Document-Level Inferences and Computational Models of Reference, Anaphora and Coreference (CODI-CRAC 2026)
**Link:** https://aclanthology.org/2026.codi-1.21/ (ACL Anthology 2026.codi-1.21; DOI 10.18653/v1/2026.codi-1.21)
**License of the paper:** CC BY 4.0 (ACL Anthology) — https://creativecommons.org/licenses/by/4.0/

## Summary
The fifth edition of the multilingual coreference shared task, held with CODI-CRAC 2026, again asked for mention detection and identity coreference clustering. This year the focus was on long-range entities whose mentions are spread over long documents, which benchmarks dominated by short documents underrepresent. CorefUD 1.4 added five datasets and two languages (Dutch and Latin), including literary corpora with long chains and a spoken Czech corpus; 27 datasets in 19 languages were used. A dataset counts as long-range when the 95th percentile of first-to-last mention distance of non-singleton entities exceeds 1500 words. Zero anaphora is again represented by UD empty nodes. Ten systems participated in an unconstrained and an LLM track; encoder-based CorPipe systems kept first place, with the best LLM systems not far behind.

## How Mova uses it
- `ai/coref/src/score.rs` — together with the 2025 edition, the source for head-based mention matching as the primary scoring mode (CRAC 2023–2026 convention).
- the development notes of `ai/coref` — one row of the paper's results table (CorPipeEnsemble 76.1, Stanza 73.0, baselines 62.4) is quoted as an indicative GUM reference; the row label was lost in the extracted text, so its attribution to en_gum is marked as uncertain.

## Effectiveness in Mova
Not measured separately: the paper supplies the evaluation convention and reference numbers. Mova's untrained deterministic resolver reaches CoNLL F1 60.3 on GUM test with gold trees and 54.0 with its own trees (head match, without singletons), below the shared-task baselines; settings differ (CorefUD 1.4 there, full UD 2.18 test here), so the comparison is only approximate.

---

👨‍🔬💥
