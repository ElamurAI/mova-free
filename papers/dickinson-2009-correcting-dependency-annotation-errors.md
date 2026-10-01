# Correcting Dependency Annotation Errors

**Authors:** Markus Dickinson · **Year:** 2009 · **Venue:** Proceedings of the 12th Conference of the European Chapter of the ACL (EACL 2009)
**Link:** https://aclanthology.org/E09-1023/ (ACL Anthology E09-1023)
**License of the paper:** CC BY-NC-SA 3.0 (ACL Anthology, pre-2016) — https://creativecommons.org/licenses/by-nc-sa/3.0/

## Summary
The paper moves from detecting errors in dependency treebank annotation to correcting them automatically. It explains why this is hard: erroneous annotations often recur identically in similar contexts, so a model trained on the corpus easily learns the noise. The data is the written part of the Swedish Talbanken05 treebank, with correction candidates taken from variation n-grams found by earlier detection work. For each word pair the model predicts a dependency label together with which word is the head, or a special label when there is no relation; classification uses memory-based learning. Simple combinations of local features perform poorly, so the author takes ambiguity classes from strict lexical models and passes them to more flexible POS-based models, as output constraints, as extra features, or both. The best configuration labels about three quarters of the problematic positions correctly, clearly above the no-change baseline. The idea of one model refining another is presented as applicable to other annotation layers and to dependency parsing.

## How Mova uses it
- ai/en/seeds/errors/consistency-detection-methods.md: a seed card surveying automatic ways to find annotation errors; this paper is cited for method 1 ("variation nuclei" — the same word pair in the same context with different relations signals an error) and for the result that such positions can be automatically corrected (76.7% vs 70.1% unchanged, as reported in the paper).
- The card feeds the expert-system principle that a rule which fires often on a gold treebank is either wrong or is catching errors in the gold standard, and the noise-robust training in Mova's taggers that turns repeatedly-wrong labels into suspicions rather than facts.

## Effectiveness in Mova
Background reading; not measured separately. No Dickinson-style correction classifier is implemented in Mova.

---

👨‍🔬💥
