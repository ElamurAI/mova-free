# Targeted Syntactic Evaluation for Grammatical Error Correction

**Authors:** Aomi Koyama, Masato Mita, Su-Youn Yoon, Yasufumi Takama, Mamoru Komachi · **Year:** 2025 · **Venue:** Proceedings of the 63rd Annual Meeting of the Association for Computational Linguistics (Volume 1: Long Papers)
**Link:** https://aclanthology.org/2025.acl-long.1026/ (ACL Anthology 2025.acl-long.1026; DOI 10.18653/v1/2025.acl-long.1026)
**License of the paper:** CC BY 4.0 (ACL Anthology) — https://creativecommons.org/licenses/by/4.0/

## Summary
Standard grammatical error correction (GEC) benchmarks are built from learner essays, so some grammar items are rare or missing and several errors in one sentence interact. The authors bring the idea of targeted syntactic evaluation, i.e. minimal pairs of an ungrammatical and a grammatical sentence, to GEC. They build CTSEG, a dataset of 1,578 hand-written sentences organised by CEFR level (A1 to B2), created by experienced English teachers and checked by a reviewer, with a much finer error-label inventory than ERRANT (for example, 15 subtypes for verb tense). Sequence-to-sequence, sequence-tagging and prompt-based GEC systems are evaluated: all handle beginner-level items well but degrade clearly on intermediate and advanced items. Naming the target construction in the prompt improves GPT-4's use of it. The paradigm exposes weaknesses hidden by aggregate leaderboard metrics and can be carried over to other languages.

## How Mova uses it
- `ai/en/seeds/errors/errant-taxonomy.md` — cited alongside ERRANT as a finer-grained, CEFR-ordered view of error types (verb tense split into 15 subtypes).
- `ai/en/seeds/errors/modal-do-to-base.md` — background for the rules `en.errors.to-base`, `en.errors.modal-base`, `en.errors.do-base` (base form after a modal, do-support and infinitival to).
- `ai/en/seeds/errors/preposition-errors.md` — background for verb-preposition rules (`en.errors.depend-on`, `en.errors.discuss-no-about`, `en.errors.arrive-not-to`, `en.errors.listen-to`).
- The rules themselves are derived mainly from ERRANT and treebank checks; this paper provides the minimal-pair framing and taxonomy context.

## Effectiveness in Mova
Background reading for the learner-error seed notes; no rule depends on it alone and CTSEG is not used as a benchmark. Not measured separately.

---

👨‍🔬💥
