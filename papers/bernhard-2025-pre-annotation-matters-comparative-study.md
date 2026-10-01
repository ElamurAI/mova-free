# Pre-annotation Matters: A Comparative Study on POS and Dependency Annotation for an Alsatian Dialect

**Authors:** Delphine Bernhard, Nathanaël Beiner, Barbara Hoff · **Year:** 2025 · **Venue:** Proceedings of the 19th Linguistic Annotation Workshop (LAW-XIX-2025)
**Link:** https://aclanthology.org/2025.law-1.14/ (ACL Anthology 2025.law-1.14; DOI 10.18653/v1/2025.law-1.14)
**License of the paper:** CC BY 4.0 — https://creativecommons.org/licenses/by/4.0/

## Summary
The study asks whether automatic pre-annotation helps human annotators build a UD treebank (POS tags and dependencies) for an Alsatian Alemannic dialect that has no annotated data, no tools and no standard spelling. Three near-zero-shot pre-annotators are compared: German UDPipe 2 models applied after a light normalisation of the dialect towards German, the generative LLM Mistral Large driven by prompts, and an ArboratorGrew parser trained on a mix of closely related languages and dialects. Human correctors then fixed the output batch by batch. Good final quality was reachable with all set-ups, and annotators adjusted their correction effort to how good the pre-annotation looked rather than accepting it blindly. The parser trained on related varieties gave the best overall results on both tasks. The LLM tagged parts of speech better than one of the UDPipe baselines but built noticeably worse trees, and its output often needed repairs to be valid CoNLL-U. The paper is practical guidance for small teams building treebanks for low-resource varieties.

## How Mova uses it
- ai/en/seeds/errors/llm-annotator-failure-modes.md: a seed card listing typical failure modes of LLM-based UD annotators; this paper supplies the first two — "tags are better than trees" (LLM tagging accuracy high, attachment much lower) and "short arcs" (LLM output has a shorter mean dependency distance than the gold standard, i.e. a bias towards the nearest head).
- The short-arc finding motivates several deterministic checks in the expert-system rules (engine ai/en/src/expert.rs): `advcl` under a noun (ai/en/seeds/errors/advcl-on-noun.md), `obl` under a nominal (obl-on-nominal.md) and an auxiliary after its head in VP ellipsis.
- The observation that all tools confused the perfect with the copula/passive is cited in copula-be-only.md and passive-structure.md (rules: `cop` only on *be*; `nsubj:pass`/`aux:pass` only with a VBN head).
- "Dependencies worse than tags" backs function-words-leaves.md (auxiliaries and copulas must be leaves) and nominal-rels-on-verb.md (nominal relations must not hang on verbs).
- More generally it supports Mova's "draft" mode in the annotator: the Rust parser annotates first and the large model only edits, rather than letting an LLM produce trees from scratch.

## Effectiveness in Mova
Not measured separately as an idea. The rules it informs are validated on gold treebanks (UD English EWT 2.18), for example: `advcl` under a noun fired 201 times with 15 violations (fragments and headlines); `cop` not on *be* — 0 of 5,939; `nsubj:pass` with a non-VBN head — 0 of 1,445; auxiliaries/copulas with core dependents — 0 of 15,648. These checks show the rules are consistent with the gold standard; the paper's role is as a source of which errors to look for.

---

👨‍🔬💥
