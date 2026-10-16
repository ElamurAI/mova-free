# Treebanks

Sentences annotated in [Universal Dependencies](https://universaldependencies.org/) (CoNLL-U: words, lemmas, tags,
morphological features, dependency tree), UD 2.18, EWT conventions.

| File | Sentences | Texts | Annotation |
|---|---|---|---|
| `mova-school-agreed-en.conllu` (new in 0.4) | 42,759 | the lessons of Mova's primary school: 26 courses written for Mova (ours) | see below |
| `mova-silver-en.conllu` | 348 | Tatoeba sentences under CC0 (`# source` says which), example sentences from Mova's seeds, and 6 sentences from ACL Anthology papers under CC BY 4.0 | parsed by Mova, checked and corrected by a large language model, with UD validation gates |
| `mova-bronze-en.conllu` | 326 | example sentences from Mova's seeds | parsed by Mova only, not checked |

## The agreed school

The sentences of the school's lessons (`# course`, `# lesson`, `# sent_id = school:<course>:<lesson>:<n>`), in the
26 primary courses (the middle-school courses are not included).

**How made** (2026-10-15):
1. Mova's own parser (`en`) drafts each sentence; a large language model (Claude Haiku 5.5, medium effort) patches
   the draft under UD validation gates (`# annotation` says so).
2. Three open parsers parse the same tokens: Stanza and Trankit (UD) and spaCy trf.
3. Written rules (`tools/treeagree` in the repository) change a word only on the evidence of the parsers that agree
   with each other, and never past the UD gates: 82.6 k of 1.17 M words changed (7.1%), each with its rule in MISC
   (`Agree=R7-rel-agree`, the teacher's earlier choice in `Haiku=`). Spot checks: the strongest rule about 80%
   right, the others about 90%.

**How Mova uses it:** the training data of Mova's `en` model (`en.bin`, with UD English EWT and ESLSpok) and of
its GPU parser. A held-out part (every 10th lesson) is the school test of Mova's parsing numbers.
