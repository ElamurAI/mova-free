# A Corpus for Reasoning About Natural Language Grounded in Photographs (NLVR2)

**Authors:** Alane Suhr, Stephanie Zhou, Ally Zhang, Iris Zhang, Huajun Bai, Yoav Artzi · **Year:** 2019 · **Venue:** ACL 2019
**Link:** https://aclanthology.org/P19-1644/ (ACL Anthology P19-1644; arXiv:1811.00491)
**License of the paper:** CC BY 4.0 (ACL Anthology) — https://creativecommons.org/licenses/by/4.0/

## Summary
The paper introduces NLVR2, a dataset for visual reasoning in which a model receives a pair of real photographs and an English sentence and must decide whether the sentence is true of the pair. It follows the earlier NLVR dataset, which used synthetic images of coloured geometric shapes, and moves to natural photographs to get richer visual and linguistic variety. Sentences were written by crowdworkers who were shown contrasting image pairs, which encourages statements involving counting, comparison, quantifiers, spatial relations and set reasoning. The collection procedure is designed to reduce simple language-only biases. Baseline and then state-of-the-art vision-and-language models perform well below human accuracy, showing that compositional grounded reasoning was still hard.

## How Mova uses it
- `ai/global/seeds/shortcuts/scene.md` — the first-level "scene" seed (objects described by size, colour and shape; partial descriptions referring back to an earlier compatible object; symmetric, non-transitive *near*/*far*/*touching*) cites this work together with SpartQA (Mirzaee et al. 2021) as background.
- The vocabulary of colours, sizes and shapes in that seed is closer to the synthetic shape images of the original NLVR than to NLVR2's photographs; the NLVR line of work is used as a general reference for how scene descriptions are phrased, not as a dataset.
- The seed is consumed by `ai/world/src/spartqa.rs` (mentions, object resolution, relations) for the SpartQA-human evaluation.

## Effectiveness in Mova
Not measured separately, and Mova does not evaluate on NLVR or NLVR2. The scene seed it informs is part of the SpartQA-human reader, which scores 59.3% on test (majority baseline 34.9%) per Mova's development notes.

---

👨‍🔬💥
