# Fantastic Questions and Where to Find Them: FairytaleQA – An Authentic Dataset for Narrative Comprehension

**Authors:** Ying Xu, Dakuo Wang, Mo Yu, Daniel Ritchie, Bingsheng Yao, Tongshuang Wu, Zheng Zhang, Toby Jia-Jun Li, Nora Bradford, Branda Sun, Tran Bao Hoang, Yisi Sang, Yufang Hou, Xiaojuan Ma, Diyi Yang, Nanyun Peng, Zhou Yu, Mark Warschauer · **Year:** 2022 · **Venue:** Proceedings of the 60th Annual Meeting of the Association for Computational Linguistics (Volume 1: Long Papers)
**Link:** https://aclanthology.org/2022.acl-long.34/ (ACL Anthology 2022.acl-long.34; DOI 10.18653/v1/2022.acl-long.34)
**License of the paper:** CC BY 4.0 (ACL Anthology) — https://creativecommons.org/licenses/by/4.0/

## Summary
FairytaleQA is a question-answering dataset for narrative comprehension aimed at the reading level of kindergarten to eighth-grade children. Its questions were written by education experts following an evidence-based framework from reading research, so that each question targets a specific narrative element. The dataset contains 10,580 questions over 278 child-friendly stories, labelled with seven types (character, setting, action, feeling, causal relationship, outcome resolution, prediction) and as explicit (answer present in the text) or implicit (answer must be inferred). The authors show that existing QA models perform unevenly across these fine-grained types, which makes the labels useful for diagnosing reading skills. They also use the data for question generation in the education domain and report that a generator trained on it asks higher-quality and more diverse questions.

## How Mova uses it
- `ai/world/src/ftqa.rs`: loader for the FairytaleQA CSV (copy from Hugging Face, Apache-2.0 per its card); it reconstructs full stories from per-question sections and splits sentences. Used only for evaluation, never for training.
- `world read-qa`: answers questions by picking the sentence with the highest weighted overlap of content lemmas, then extracting a constituent by question type (who → agent, where → place, why → cause, what did X do → clause).
- `world read-events`: an event graph (verb with participants; next/previous, cause, shared participant links) with a transparent traversal policy per question type, learned on the validation split and measured on test.
- `world read-feel`: "feeling" questions answered from explicit emotion words near the event or by inference through first-level links (e.g. death → loss → sadness), later extended with ATOMIC-2020 reactions.
- The seven question types and the explicit/implicit split are adopted verbatim as the evaluation taxonomy.

## Effectiveness in Mova
FairytaleQA is the benchmark rather than a method, so the numbers below are Mova results measured on it (ROUGE-L):
- Sentence + constituent by question type (`read-qa`, 23 stories, 1007 questions): 0.181 vs 0.192 for returning the whole retrieved sentence; for "where" questions 0.277 vs 0.202.
- Event graph (`read-events`): test 0.199 — first result above the whole-sentence baseline (0.192); "what happened" 0.264 vs 0.244, "what" 0.210 vs 0.182.
- Feelings (`read-feel`): feel questions 0.037 → 0.141; with the event graph overall 0.212; with ATOMIC-2020 feelings overall 0.215.
- A context graph of entities (v1) gave no gain: valid 0.182 → 0.184, test 0.181 → 0.178.

---

👨‍🔬💥
