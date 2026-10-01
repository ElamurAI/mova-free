# Findings of the Fourth Shared Task on Multilingual Coreference Resolution: Can LLMs Dethrone Traditional Approaches?

**Authors:** Michal Novák, Miloslav Konopik, Anna Nedoluzhko, Martin Popel, Ondrej Prazak, Jakub Sido, Milan Straka, Zdeněk Žabokrtský, Daniel Zeman · **Year:** 2025 · **Venue:** Proceedings of the Eighth Workshop on Computational Models of Reference, Anaphora and Coreference (CODI-CRAC 2025)
**Link:** https://aclanthology.org/2025.crac-1.9/ (ACL Anthology 2025.crac-1.9; DOI 10.18653/v1/2025.crac-1.9)
**License of the paper:** CC BY 4.0 (ACL Anthology) — https://creativecommons.org/licenses/by/4.0/

## Summary
This overview reports the fourth edition of the multilingual coreference shared task at CODI-CRAC 2025. Participants had to detect mentions, including zero mentions in pro-drop languages, and cluster them by identity coreference. The data came from CorefUD 1.3, with 22 datasets in 17 languages used in the task; new additions included spoken French, Hindi and Korean. The main novelty was a dedicated LLM track with a simplified plaintext encoding of coreference written inline in the text, easier for generative models than CoNLL-U, alongside an unconstrained track. Nine systems took part, four of them LLM-based. A traditional encoder-based system (CorPipe) again ranked first, but LLM approaches matched or beat it on some datasets. The official primary metric is CoNLL F1 computed with head-based mention matching.

## How Mova uses it
- `ai/coref/src/score.rs` — mention matching by head, as in the shared task's primary metric: gold and system mentions match when they share a head; several mentions with the same head are disambiguated by span (exact first, then largest overlap). Exact-span matching is kept as a reference.
- the development notes of `ai/coref` — the per-system CoNLL F1 on en_gum from the paper's results table is reproduced as an indicative reference point for Mova's own GUM scores.

## Effectiveness in Mova
This paper provides the evaluation protocol and reference numbers rather than a technique, so it is not measured separately. Indicative comparison on GUM (CoNLL F1, head match, without singletons): CRAC 2025 systems range from 76.1 (CorPipeEnsemble) to 61.7 (mBERT end-to-end baseline); Mova's deterministic resolver scores 60.3 on gold trees and 54.0 on its own predicted trees. The settings differ (CorefUD 1.3 and a reduced mini-test there, the full UD 2.18 test here; trained neural systems there, untrained rules here), so the comparison is only approximate.

---

👨‍🔬💥
