# Universal Dependencies for Learner English

**Authors:** Yevgeni Berzak, Jessica Kenney, Carolyn Spadine, Jing Xian Wang, Lucia Lam, Keiko Sophie Mori, Sebastian Garza, Boris Katz · **Year:** 2016 · **Venue:** Proceedings of the 54th Annual Meeting of the ACL (Volume 1: Long Papers)
**Link:** https://aclanthology.org/P16-1070/ (ACL Anthology P16-1070; DOI 10.18653/v1/P16-1070)
**License of the paper:** CC BY 4.0 — https://creativecommons.org/licenses/by/4.0/

## Summary
Much of the English written today is produced by non-native speakers, yet before this work there was no open syntactic treebank of learner English. The authors built the Treebank of Learner English (TLE) from essays of the FCE corpus written by learners with ten different native languages. POS tags and UD trees were assigned manually, and every sentence was reviewed by further annotators. Since FCE already marks errors and corrections, each sentence is annotated twice: as written and in its corrected form. For ungrammatical constructions the authors propose a "literal reading" principle: the syntax describes what the learner actually wrote, not what they presumably meant. Inter-annotator agreement shows this guideline yields consistent annotation. Tagging and parsing experiments on the paired original/corrected sentences indicate that grammatical errors hurt parsers less than earlier estimates suggested. The resource enables both empirical study of learner syntax and automatic processing of ungrammatical text.

## How Mova uses it
- ai/en/seeds/errors/literal-annotation-learner.md: the literal-reading principle is adopted for Mova's annotation — a sentence with errors gets the tree of its actual words; spelling errors use UD's Typo=Yes/CorrectForm mechanism, while grammatical errors (agreement, verb form) are left visible in the tree. This is what lets agreement rules double as grammar-error detectors, and it is used to catch LLM annotators that silently "fix" the text.
- ai/en/seeds/errors/missing-copula.md: rule `en.errors.missing-copula` (nominal/adjectival predicate with a subject but no copula, e.g. "He very happy"), justified by the literal-reading analysis.
- ai/en/seeds/errors/missing-article.md: rule `en.errors.bare-count-noun` (singular count noun as subject/object without determiner); TLE statistics (ten L1s, average errors per sentence) are cited as background.

## Effectiveness in Mova
Not measured separately. The rules derived with its help are checked against gold treebanks: on UD English EWT 2.18 the missing-copula rule fired 3,727 times with 44 flags (mostly telegraphic review style and genuinely missing *is*), and the bare-count-noun rule 953 / 42 (mostly real missing articles and telegraphic style); on GUM 3,011 / 37 and 512 / 15 respectively. The paper's main role is the literal-annotation principle that the whole error-rule section relies on.

---

👨‍🔬💥
