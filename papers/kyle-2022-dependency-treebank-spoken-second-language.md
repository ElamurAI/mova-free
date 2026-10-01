# A Dependency Treebank of Spoken Second Language English

**Authors:** Kristopher Kyle, Masaki Eguchi, Aaron Miller, Theodore Sither · **Year:** 2022 · **Venue:** Proceedings of the 17th Workshop on Innovative Use of NLP for Building Educational Applications (BEA 2022)
**Link:** https://aclanthology.org/2022.bea-1.7/ (ACL Anthology 2022.bea-1.7; DOI 10.18653/v1/2022.bea-1.7)
**License of the paper:** CC BY 4.0 (ACL Anthology) — https://creativecommons.org/licenses/by/4.0/

## Summary
POS taggers and parsers lose accuracy on text unlike their training data, and second-language (L2) speech is a clear case; a written L2 treebank existed, but no freely available spoken one. The authors build SL2E from utterances of Japanese learners in oral proficiency interviews (NICT JLE corpus), with interviewer turns removed. Sentences are hand-annotated with Penn POS tags and UD dependencies, each by at least two annotators with a third adjudicating disagreements. They then train spaCy models (RoBERTa-base) on L1 data only, on L1 plus L2, and on L1 plus L2 at equal total size. Even a small amount of L2 data improves both tagging and parsing markedly, with the largest gains on spoken L2 (LAS on SL2E rising from 0.876 to about 0.935). The resource is aimed at language-development research and educational NLP.

## How Mova uses it
- `ai/en/seeds/errors/literal-annotation-learner.md` — cited as evidence that learner language needs in-domain data (a little L2 data lifts LAS from 0.876 to 0.935), in the note establishing that Mova annotates erroneous text literally (what is written, not what was meant).
- Background reading only; SL2E is not used as training or evaluation data in Mova.

## Effectiveness in Mova
Background reading for one seed note. Not measured separately.

---

👨‍🔬💥
