# Assessing the Annotation Consistency of the Universal Dependencies Corpora

**Authors:** Marie-Catherine de Marneffe, Matias Grioni, Jenna Kanerva, Filip Ginter · **Year:** 2017 · **Venue:** Proceedings of the Fourth International Conference on Dependency Linguistics (Depling 2017)
**Link:** https://aclanthology.org/W17-6514/ (ACL Anthology W17-6514)
**License of the paper:** CC BY 4.0 (ACL Anthology) — https://creativecommons.org/licenses/by/4.0/

## Summary
UD treebanks are produced quickly and for many languages, so manual quality control is hard; the paper looks for a way to find inconsistent annotation automatically. It builds on the "variation nuclei" method of Boyd et al. (2008): pairs of words with the same left and right context that are linked by different dependency relations in different sentences signal that one of the occurrences is probably wrong. The method is reproduced on UD v2 for English, French and Finnish, and extended by comparing lemmas instead of word forms and by adding large automatically parsed corpora (parsebanks). The lemma variant works well for English and French (precision around 62–65%) but produces many false alarms for Finnish, where case changes syntactic role; using word forms helps there. Parsebanks surface new errors but add only moderate recall. The authors release a web tool for manual review of the flagged cases.

## How Mova uses it
- `ai/en/seeds/errors/consistency-detection-methods.md` — method 1 ("variation nuclei") among the automatic error-detection methods Mova catalogues, with the paper's figures (266 suspicious lemma pairs in English UD v2, 62% precision; French 65%, Finnish 19%).
- The same principle underlies Mova's expert-rule practice: a rule that fires often on gold data is either wrong or is catching gold-standard errors, and violations on EWT 2.18 have indeed exposed treebank errors.
- Mova does not run a variation-nuclei detector itself; the paper is a method reference.

## Effectiveness in Mova
Background reading for the error-detection seed note. Not measured separately.

---

👨‍🔬💥
