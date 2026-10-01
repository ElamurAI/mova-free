# SPARTQA: A Textual Question Answering Benchmark for Spatial Reasoning

**Authors:** Roshanak Mirzaee, Hossein Rajaby Faghihi, Qiang Ning, Parisa Kordjamshidi · **Year:** 2021 · **Venue:** Proceedings of the 2021 Conference of the North American Chapter of the Association for Computational Linguistics: Human Language Technologies (NAACL-HLT)
**Link:** https://aclanthology.org/2021.naacl-main.364/ (ACL Anthology 2021.naacl-main.364; DOI 10.18653/v1/2021.naacl-main.364)
**License of the paper:** CC BY 4.0 (ACL Anthology) — https://creativecommons.org/licenses/by/4.0/

## Summary
The paper introduces a question-answering benchmark for spatial reasoning over text that is richer than the earlier synthetic bAbI spatial task. Scene descriptions are based on NLVR images: a small human-written part (SpartQA-Human, about 1.1k questions) and a large automatically generated part (SpartQA-Auto) built with context-free grammars and spatial inference rules. Descriptions are deliberately incomplete, so answers require transitivity, symmetry, inverse relations, inclusion and exclusion. There are four question types: which relation holds, which block an object is in, which of two objects fits, and yes/no with a "don't know" option under an open-world assumption. Because language models do poorly when trained only on the small human set, the authors propose distant supervision: further pretraining on the auto-generated data. This improves results on SpartQA-Human and also transfers to bAbI and BoolQ.

## How Mova uses it
- `ai/global/seeds/shortcuts/scene.md` — a first-level knowledge seed for scene descriptions: categories of colours, sizes and shapes, generic nouns ("thing", "object", "one"), synonyms of above/below, and the relations near, far and touching declared symmetric (but not transitive).
- SpartQA-Human is used as an evaluation set for the `world spartqa` reader in the `ai/world` crate: mentions are built from the seed categories, scoped to a block, relations are composed, and the output is three-valued (follows / opposite / unknown).
- The dataset is under a non-commercial license, so it is used only for measurement, not shipped or trained on beyond rule fixes guided by training-split errors.

## Effectiveness in Mova
Measured on SpartQA-Human (test): 59.3% overall (majority baseline 34.9%; first version 50.5%), by type YN 60.1%, FB 67.1%, CO 58.2%, FR 50.6%; training split 61.3%. Fixes were made only from training-split errors. Mova's test split is the 2022 version (YN 143, FR 77 questions), so the overall figure is not comparable with the 2021 BERT transfer result on the 510-question test. Comparable figures: YN 60.1% with DK in the gold labels, or 68.5% two-class (DK mapped to No), versus BERT+Q-Chain 59.44, Llama-3-8B CoT 67.83, PistaQ 75.52 and GPT-4 77.62; FR 50.6% (exact set match) versus BERT with SpaRTUN 50.64. This is achieved with no neural network and no training on this data. The ceiling is limited: questions were written from images, so some answers are not derivable from (or contradict) the text.

---

👨‍🔬💥
