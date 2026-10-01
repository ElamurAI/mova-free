# Minor changes make a difference: a case study on the consistency of UD-based dependency parsers

**Authors:** Dmytro Kalpakchi, Johan Boye · **Year:** 2021 · **Venue:** Proceedings of the Fifth Workshop on Universal Dependencies (UDW, SyntaxFest 2021)
**Link:** https://aclanthology.org/2021.udw-1.8/ (ACL Anthology 2021.udw-1.8)
**License of the paper:** CC BY 4.0 (ACL Anthology) — https://creativecommons.org/licenses/by/4.0/

## Summary
The paper asks whether off-the-shelf UD parsers stay consistent when a sentence is changed only trivially. The authors take treebank sentences containing a four-digit number (mostly a year) in English, Swedish, Russian and Ukrainian, substitute 50 random values for that number, and parse every variant with pretrained Stanza models. Whole-sentence parse quality is compared with a tree-kernel similarity, and groups of identical errors are found as maximal cliques. Standard metrics (UAS, LAS, BLEX) barely move, yet a large share of variant bundles (about 44% on average) show errors that differ from one variant to another. Two retraining fixes are tried: augmenting the training data with re-numbered sentences, and replacing numbers with a placeholder token; only augmentation helps noticeably. The authors also point to a likely temporal bias in treebanks and warn applications that rely on UD trees.

## How Mova uses it
- `ai/en/seeds/errors/consistency-detection-methods.md` — listed as method 5 ("robustness to small changes") among automatic ways to find annotation errors: places where the parse flips under a trivial substitution are candidates for errors.
- `ai/en/seeds/errors/numbered-entities-nummod.md` — cited as evidence that numbers are unstable for parsers, as background to the rules `en.nominal.nummod-before-noun` and `en.nominal.nummod-num` (numbers after a noun are identifiers, not counts).
- Used as background reading only; Mova has no perturbation-based checker built from this paper.

## Effectiveness in Mova
Background reading behind one seed note and one detection idea; no component implements the method. Not measured separately.

---

👨‍🔬💥
