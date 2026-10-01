# COMET-ATOMIC 2020: On Symbolic and Neural Commonsense Knowledge Graphs

**Authors:** Jena D. Hwang, Chandra Bhagavatula, Ronan Le Bras, Jeff Da, Keisuke Sakaguchi, Antoine Bosselut, Yejin Choi · **Year:** 2020 (arXiv) / 2021 (AAAI) · **Venue:** Proceedings of the AAAI Conference on Artificial Intelligence 35(7):6384–6392 (2021)
**Link:** https://arxiv.org/abs/2010.05953 (arXiv 2010.05953; DOI 10.48550/arXiv.2010.05953)
**License of the paper:** CC BY 4.0 — https://creativecommons.org/licenses/by/4.0/

## Summary
The paper argues that hand-built commonsense knowledge graphs can never cover everything an NLP system will meet. It therefore proposes judging a knowledge graph by how well neural "knowledge models" trained on it can generate new commonsense facts. With this goal it introduces ATOMIC 2020, a general-purpose commonsense graph of if-then knowledge about social interactions, physical objects and events, built to contain knowledge that pretrained language models do not readily have. The authors compare it with other major commonsense graphs in a large pairwise study of coverage and accuracy. They show that knowledge models (COMET) trained on ATOMIC 2020 generate more accurate knowledge for unseen entities and events than models trained on other resources. In human evaluation, a BART-based COMET model trained on ATOMIC 2020 beats few-shot GPT-3 by about 12 absolute points while being over 430 times smaller.

## How Mova uses it
- Mova uses the ATOMIC 2020 data (CC BY 4.0), not the neural COMET model. Small SQL extractions (`ai/global/data/atomic-feelings.sql`, `ai/global/data/atomic-uses.sql`) produce compiled first-level seeds. No knowledge model is trained.
- `ai/global/seeds/shortcuts/atomic-feelings.md`: from the `xReact` relation, Mova selects "event verb → feeling of the doer" by lift over the base rate. The base-rate correction is needed because ATOMIC is skewed positive. Examples: hit/beat/fight → anger; lose/miss/cry/bury → sadness; break/drop/steal/forget → shame; hide/run → fear; receive/thank/marry/hug → gratitude.
- `ai/global/seeds/shortcuts/atomic-feelings-o.md`: the `oReact` relation gives the feelings of others (help/save/protect → gratitude, leave/rob/hurt → sadness, chase/grab/scare → fear). It applies when the character in the question is the object of the verb.
- `ai/global/seeds/shortcuts/atomic-uses.md`: the `ObjectUse` and `CapableOf` relations give what objects are for and what creatures can do (knife/axe/saw → cut, bird → fly) for 417 objects and 47 creatures in Mova's corpus. They are queried via `global::uses()` and `global::capable()`, and the test includes negative controls (a table does not cut, an apple does not fly).

## Effectiveness in Mova
Measured (architecture status) on FairytaleQA feeling questions, comparing the story reader before and after adding the ATOMIC `xReact` table: test 0.130 → 0.158, validation 0.105 → 0.118 (inferred answers 28 → 71). Overall reader score 0.212 → 0.215, identical on a repeat run (deterministic), with no change on fables. Adding `oReact`: validation 0.118 → 0.127, test unchanged at 0.158. The `ObjectUse`/`CapableOf` tables pass their negative-control test but have no downstream measurement yet.

---

👨‍🔬💥
