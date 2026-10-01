# Automatic Annotation and Evaluation of Error Types for Grammatical Error Correction

**Authors:** Christopher Bryant, Mariano Felice, Ted Briscoe · **Year:** 2017 · **Venue:** Proceedings of the 55th Annual Meeting of the ACL (Volume 1: Long Papers)
**Link:** https://aclanthology.org/P17-1074/ (ACL Anthology P17-1074; DOI 10.18653/v1/P17-1074)
**License of the paper:** CC BY 4.0 — https://creativecommons.org/licenses/by/4.0/

## Summary
Grammatical error correction (GEC) systems were usually judged by a single overall score, and because system output carries no error labels, per-type performance could only be estimated through recall. The paper introduces ERRANT, a toolkit that extracts edits from a pair of original and corrected sentences using a linguistically informed alignment and then assigns each edit an error type. Typing is done by a set of roughly fifty deterministic rules over automatically obtained features — part-of-speech tags, lemmas, a dependency parse and a word list — so it needs no training data and is not tied to any particular corpus. The scheme has 25 main categories combined with three operations (missing, replaced, unnecessary), allowing evaluation at several levels of detail. Expert raters judged the vast majority of assigned types as good or acceptable. Applying ERRANT to all CoNLL-2014 shared-task submissions gave the first detailed per-type comparison, showing that a system weaker overall can still lead in specific categories. The tool also helps standardise existing GEC corpora and reduce annotation effort.

## How Mova uses it
- ai/en/seeds/errors/errant-taxonomy.md: a seed card summarising the ERRANT taxonomy and its ordered verb rules (ORTH → SPELL → same lemma → FORM → TENSE → SVA), and mapping which error types leave a visible trace in a UD tree; each such type became its own error-rule card.
- Rules in the expert system (engine ai/en/src/expert.rs) cite ERRANT types as their category: `a`/`an` choice (DET; ai/en/seeds/errors/a-an-choice.md), adverb between verb and object (WO; adverb-placement.md), coordinated-subject agreement (VERB:SVA; coordinated-subject-agreement.md), determiner–noun number (DET, NOUN:NUM), double comparative (ADJ:FORM), missing article (M:DET), missing copula (M:VERB), passive auxiliary form (VERB:FORM).
- The design principle carried over is ERRANT's own: deterministic, explainable rules over POS, lemma and parse features, no learned classifier.

## Effectiveness in Mova
Not measured separately as an idea. The rules categorised with ERRANT types are checked on UD English EWT 2.18, e.g. "*a* before a vowel-initial noun" 363 hits / 6 flags (all six real text errors), "*an* before a consonant" 198 / 3 (abbreviations), coordinated subject with singular verb 394 / 5, adverb between verb and object 21 / 3 (all heavy objects). Its role is a taxonomy and design reference for the error-rule section.

---

👨‍🔬💥
