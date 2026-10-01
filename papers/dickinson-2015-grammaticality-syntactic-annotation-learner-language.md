# On Grammaticality in the Syntactic Annotation of Learner Language

**Authors:** Markus Dickinson, Marwa Ragheb · **Year:** 2015 · **Venue:** Proceedings of the 9th Linguistic Annotation Workshop (LAW IX)
**Link:** https://aclanthology.org/W15-1619/ (ACL Anthology W15-1619; DOI 10.3115/v1/W15-1619)
**License of the paper:** CC BY-NC-SA 3.0 — https://creativecommons.org/licenses/by-nc-sa/3.0/

## Summary
The paper asks whether an annotator of dependency syntax in second-language (learner) texts can avoid judging grammaticality at all. It works within the SALLE scheme, which annotates English learner essays in several layers (for example, separate morphological and distributional evidence for the same word) and aims to record linguistic evidence rather than errors. The authors focus on "non-canonical" categories that license missing material: ellipsis with absent heads, and enumeration (coordination without a conjunction). Through worked examples they show that choosing between an ellipsis analysis, a coordination analysis, or some other tree quietly depends on whether the annotator considers the sentence well-formed, and so does any procedure that picks one tree out of several candidates. The paper is theoretical, with no experiments. It lays out three possible annotation strategies and discusses how each affects annotation practice and the "comparative fallacy" known from second-language acquisition research. The practical message is that the grammatical assumptions behind a learner-corpus annotation scheme must be written down explicitly.

## How Mova uses it
- `ai/en/seeds/errors/literal-annotation-learner.md`: background for the seed "text with errors is annotated literally". It states as a policy point that the choice between ellipsis, coordination and enumeration depends on the annotator's grammaticality judgement and that this assumption must be explicit.
- `ai/en/seeds/errors/gapping-orphan.md`: cited for the caveat that in learner text, whether a gap is analysed as ellipsis (`orphan`) or asyndetic coordination depends on that judgement. The seed's rule `en.errors.orphan-head` checks that an `orphan` hangs from a promoted remnant (conj/root/advcl/parataxis).
- `ai/en/seeds/errors/missing-copula.md`: cited for absent heads and ellipsis in learner text. It motivates the warning rule `en.errors.missing-copula`, which flags a nominal or adjectival predicate with a subject but no copula ("He very happy"), with an exception for telegraphic review style.

## Effectiveness in Mova
Background reading behind wording and caveats in three error-detection seeds. It is not the source of any rule logic and was not measured separately. For reference, the seeds report how often their rules fire on the gold treebanks (for example `en.errors.missing-copula`: EWT 2.18, 3727 checks / 44 hits, mostly telegraphic reviews and a dropped "is"), but those counts describe the rules, not this paper's contribution.

---

👨‍🔬💥
