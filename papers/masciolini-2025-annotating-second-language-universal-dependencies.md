# Annotating Second Language in Universal Dependencies: a Review of Current Practices and Directions for Harmonized Guidelines

**Authors:** Arianna Masciolini, Aleksandrs Berdicevskis, Maria Irena Szawerna, Elena Volodina · **Year:** 2025 · **Venue:** Proceedings of the Eighth Workshop on Universal Dependencies (UDW, SyntaxFest 2025)
**Link:** https://aclanthology.org/2025.udw-1.17/ (ACL Anthology 2025.udw-1.17)
**License of the paper:** CC BY 4.0 (ACL Anthology) — https://creativecommons.org/licenses/by/4.0/

## Summary
UD is increasingly used for learner (L2) texts, but grammatical errors, calques, code-switching and other interlanguage phenomena fit poorly into guidelines written for standard language. Preparing a UD treebank from the Swedish SweLL-gold corpus, the authors review UD's universal rules for errors and foreign material and compare the choices actually made in existing L2 treebanks for several target languages. They pay special attention to parallel "L1-L2" formats in which each learner sentence comes with a correction hypothesis. The central dilemma is how far analysis should follow the actual forms versus the writer's probable intent. They recommend literal token-level annotation combined with syntactic analysis informed by the correction, propose a new dependency subtype marking deliberate violations of validation rules caused by L2 phenomena, and suggest treating syntactic calques similarly to code-switching. The proposals are meant as a basis for harmonised guidelines, also useful for spoken, user-generated and dialectal data.

## How Mova uses it
- `ai/en/seeds/errors/overregularized-verbs.md` — source of the rule `en.errors.overregularized-typo`: over-regularised forms (*goed*, *buyed*) are kept as written, with Typo=Yes, the lemma and tag of the intended word and the correct form in MISC; the rule flags such forms without Typo=Yes.
- `ai/en/seeds/errors/real-word-confusions.md` — source for five rules on homophone confusions (`en.errors.there-as-their`, `en.errors.their-as-there`, `en.errors.to-as-too`, `en.errors.then-as-than`, `en.errors.where-as-were`), all relying on the UD convention for errors summarised in the paper.
- `ai/en/seeds/errors/literal-annotation-learner.md` — the overview of L2 treebanks and UD error rules underpins Mova's principle that erroneous text is annotated literally, so agreement and form rules double as grammatical-error detectors.

## Effectiveness in Mova
The paper is a direct source for six warning rules. Their gold-data checks are recorded in the seed notes: on EWT 2.18 the over-regularisation rule finds 0 such forms (no false alarms) and the spoken L2 treebank ESLSpok has 2; the homophone rules show 0 violations on EWT 2.18 (e.g. there/PRP$ 15/0, to/ADV 12/0). These are rule-consistency checks, not a gain over a baseline; the contribution of the paper is not measured separately.

---

👨‍🔬💥
