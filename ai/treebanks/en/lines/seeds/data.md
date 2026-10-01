# LinES — what these data are and which layers exist

**Gist.** The English half of the parallel English-Swedish treebank LinES: 5696 sentences and 106,307 words. Three parts:
- fiction (the majority);
- online software help;
- Europarl.

Each sentence has a Swedish counterpart with the same index (`# parallel_id`) in UD_Swedish-LinES. The treebank was first annotated in a different, non-UD dependency scheme. It was then automatically converted to UD 1 and partly reviewed (Ahrenberg 2015), and in 2017 to UD 2 with another review. Lemmas exist since 2.1. Development happens outside the UD repository: errors must be fixed in the source or the converter.

**Conditions and exceptions.**
- Split: train 3457 / dev 1118 / test 1121 sentences.
- License CC BY-NC-SA 4.0. The 2.18 release has no README, only LICENSE.txt; the description is on the treebank page.
- The page metadata say «Features: not available», but the data do have FEATS. They are derived from LinES's own XPOS (`xpos-lines.md`), and the customs here are its own (`verb-feats.md`, `nominal-feats.md`).

**Examples.** The fiction part, dialogues: *you* — 823, `vocative` 104, `dislocated` 131 (EWT — 7).

**In UD.**

| layer | state |
|---|---|
| XPOS | LinES's own morphological set, 159 values; «_» in 22,538 words — function-word ADP, ADV, CCONJ, SCONJ, PART (`xpos-lines.md`) |
| FEATS | present; «_» in 35,574 words |
| lemmas | present; «_» — 1 |
| DEPS | absent |
| MISC | almost empty: «_» in 93,567 of 106,307 |
| MWT | 1170 |

Our annotator on the LinES test: UPOS 93.13, UFeats 83.44, LAS 76.05, MLAS 52.12. XPOS is not measured: the tags are not PTB. UFeats is 12 points lower than on EWT — these are the customs from `verb-feats.md` and `nominal-feats.md`.

**Sources.** `data/raw/ud-docs/docs/treebanks/en_lines/index.md` (description, table of annotation sources); Ahrenberg 2007, `W07-2441`; Ahrenberg 2015, `W15-2103` (papers/a/ah/ahrenberg-2015-converting-english-swedish-parallel-treebank, abstract only); digest, §1.11.
