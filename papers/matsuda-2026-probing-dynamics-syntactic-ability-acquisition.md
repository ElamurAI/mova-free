# Probing the Dynamics of Syntactic Ability Acquisition Throughout LLM Pretraining

**Authors:** Hiroshi Matsuda, Masayuki Asahara · **Year:** 2026 · **Venue:** Proceedings of the Ninth Workshop on Universal Dependencies (UDW 2026)
**Link:** https://aclanthology.org/2026.udw-1.1/ (ACL Anthology 2026.udw-1.1; DOI 10.63317/2n8gkp49p3wg)
**License of the paper:** CC BY 4.0 (ACL Anthology) — https://creativecommons.org/licenses/by/4.0/

## Summary
The authors study when syntactic skills appear during LLM pretraining and which ones stay weak. Their method, LoRA probing, keeps the base model frozen and trains only low-rank adapters to produce step-by-step UD parses in a simple tabular format. They apply it to the public intermediate checkpoints of OLMo-2-7B, training adapters at 24 pretraining stages and testing on UD treebanks in 33 languages. To fit OLMo-2's short context window they design a compact two-step template that predicts tags first and then heads and labels together without repeating word forms; it roughly matches the three-step template in accuracy while halving the context and greatly increasing throughput. The output format is learned very early in pretraining. Although the model was pretrained on almost exclusively English text, it reaches LAS above 80 in most of the languages tested, but relations such as iobj and csubj are acquired late and remain unstable across many languages.

## How Mova uses it
- `ai/en/seeds/errors/clausal-subject-csubj.md` — the finding that csubj is late and unstable in LLMs supports a rule flagging `nsubj` on a VERB (a clausal subject should be `csubj`).
- `ai/en/seeds/errors/iobj-bare-nominal.md` and `ai/en/seeds/errors/iobj-verb-classes.md` — cited for iobj instability; the rules forbid `case` under `iobj` and `iobj` with verbs that only take a `to`-recipient (explain, describe, suggest...).
- `ai/en/seeds/lexicon/verb-iobj-licensors.md` — listed among the sources for the lexical rule that only double-object verbs (171 lemmas from VerbNet classes) may license `iobj`.
- `ai/en/seeds/errors/llm-annotator-failure-modes.md` — "unstable rare relations" is one entry in the catalogue of LLM annotation failures, pointing to the rules above.

## Effectiveness in Mova
Not measured separately. The paper explains why the rules target these relations; the rules themselves are checked against gold treebanks (for example, in EWT 2.18 `iobj` with a `case` dependent occurs 0 times out of 795, and `nsubj` on a VERB occurs 4 times), but those are rule-validation counts, not a measured contribution of the paper.

---

👨‍🔬💥
