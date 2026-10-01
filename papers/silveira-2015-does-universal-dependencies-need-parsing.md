# Does Universal Dependencies need a parsing representation? An investigation of English

**Authors:** Natalia Silveira, Christopher Manning · **Year:** 2015 · **Venue:** Proceedings of the Third International Conference on Dependency Linguistics (Depling 2015)
**Link:** https://aclanthology.org/W15-2134/ (ACL Anthology W15-2134)
**License of the paper:** CC BY-NC-SA 3.0 (ACL Anthology, before 2016) — https://creativecommons.org/licenses/by-nc-sa/3.0/

## Summary
UD makes content words the heads and attaches function words (auxiliaries, copulas, subordinators, adpositions) to them. The authors ask whether English parsing would benefit from an intermediate representation where function words are heads, with a conversion back to UD afterwards. They define reversible tree transformations for four constructions (prepositional phrases, verb groups, copular clauses and marked subordinate clauses) that are lossless on gold data, and train MaltParser on each transformed version of the UD English Web Treebank. Some variants are indeed easier to parse when scored in their own representation. After converting parser output back to UD, however, accuracy always drops: one error in a function-head tree can become two in UD, and moving dependents propagates wrong attachments. The paper is a useful negative result: parsing into a "friendlier" representation and converting does not improve English UD parsing.

## How Mova uses it
- `ai/en/seeds/errors/function-words-leaves.md`: rule `en.errors.aux-cop-leaf` in the grammar expert system (`ai/en/src/expert.rs`) — function words (`aux`, `cop`, `case`, `mark`, …) should be leaves with almost no dependents. The paper is cited as the rationale for keeping content heads even though function heads are easier for a parser.
- Indirectly, it supports the choice to train the English parser (`ai/en/src/parse.rs`) directly on UD trees rather than on a converted intermediate scheme.

## Effectiveness in Mova
Not measured separately. Background reading behind one annotation-checking rule and a design choice (parse UD directly); no experiment with function-head intermediate representations was run.

---

👨‍🔬💥
