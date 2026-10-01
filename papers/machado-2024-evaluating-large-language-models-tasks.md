# Evaluating large language models for the tasks of PoS tagging within the Universal Dependency framework

**Authors:** Mateus Machado, Evandro Ruiz · **Year:** 2024 · **Venue:** Proceedings of the 16th International Conference on Computational Processing of Portuguese (PROPOR 2024), Vol. 1
**Link:** https://aclanthology.org/2024.propor-1.46/ (ACL Anthology 2024.propor-1.46)
**License of the paper:** CC BY 4.0 (ACL Anthology) — https://creativecommons.org/licenses/by/4.0/

## Summary
The paper tests how well large language models, without fine-tuning, assign the 17 UD universal POS tags to Brazilian Portuguese text. Evaluation uses the journalistic part of the Porttinari-base treebank, released after the models were trained, which lowers the risk of test contamination. Three systems are compared: GPT-3 via API, a locally run LLaMA-7B, and the Portuguese commercial service Maritaca. Each model sees ten tagged example sentences in a Portuguese prompt and must tag the next sentence; sampling temperature is tuned per model on a small sample. GPT-3 is clearly best at about 90% accuracy, LLaMA trails by roughly 20 points, and Maritaca is weakest, partly because it invents labels outside the UD tagset. The authors conclude that LLMs can provide a first-pass UPOS annotation for under-resourced settings if a human checks the result.

## How Mova uses it
- `ai/en/seeds/errors/llm-annotator-failure-modes.md` — failure mode 3 ("labels outside the tagset"): Maritaca produced 93 distinct labels instead of 17 UPOS tags. This supports Mova's practice of passing every LLM-drafted annotation through a registry gate that rejects unknown or obsolete labels.
- Background reading only; no code is derived from it directly.

## Effectiveness in Mova
Background evidence for one item of the LLM-annotator failure catalogue. Not measured separately.

---

👨‍🔬💥
