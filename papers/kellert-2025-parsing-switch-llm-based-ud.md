# Parsing the Switch: LLM-Based UD Annotation for Complex Code-Switched and Low-Resource Languages

**Authors:** Olga Kellert, Nemika Tyagi, Muhammad Imran, Nelvin Licona-Guevara, Carlos Gómez-Rodríguez · **Year:** 2025 · **Venue:** Findings of the Association for Computational Linguistics: EMNLP 2025
**Link:** https://aclanthology.org/2025.findings-emnlp.863/ (ACL Anthology 2025.findings-emnlp.863; DOI 10.18653/v1/2025.findings-emnlp.863)
**License of the paper:** CC BY 4.0 (ACL Anthology) — https://creativecommons.org/licenses/by/4.0/

## Summary
The paper tackles syntactic annotation of code-switched text, where monolingual parsers generalise poorly and annotated data hardly exists. It introduces the BiLingua Pipeline: an LLM (GPT-4.1) is prompted with few-shot examples and a set of linguistic rules to produce UD part-of-speech tags, heads and relations, after which experts and native speakers review and correct the output. The pipeline is applied to Spanish-English conversational data from the Miami Corpus and to Spanish-Guaraní social-media and news text, yielding two released datasets, including the first UD corpus for Spanish-Guaraní. After expert revision the annotation reaches up to 95.29% LAS, well above earlier baselines and multilingual parsers. The authors also propose a more lenient evaluation that does not penalise linguistically acceptable alternatives, and analyse at which syntactic positions speakers switch languages. The overall message is that a carefully guided LLM plus human review can bootstrap syntactic resources for low-resource, mixed-language settings.

## How Mova uses it
- `ai/en/seeds/errors/dep-last-resort.md` — cited as a source of the rule `en.errors.dep-last-resort`: LLM annotators fall back to `dep` on repetitions, ellipsis and contractions (the paper's prompt even allowed `dep` for ellipsis), so every `dep` is flagged for review in EWT style.
- `ai/en/seeds/errors/llm-annotator-failure-modes.md` — failure mode 5 ("inconsistency"): human correction raised LAS from about 76 to 95, which Mova reads as a warning that the yardstick must be an independent gold standard, not corrected LLM output.
- `ai/en/seeds/errors/vp-ellipsis-aux-head.md` — background for the rule `en.errors.aux-before-verb` (LLMs are inconsistent on ellipsis).

## Effectiveness in Mova
The paper informs one warning rule and the catalogue of LLM-annotator failure modes used when Mova's deterministic annotator is checked against LLM drafts. Not measured separately.

---

👨‍🔬💥
