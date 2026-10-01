# PUD — what these data are and which layers exist

**Gist.** The English part of Parallel Universal Dependencies for CoNLL 2017: 1000 sentences (21,180 words) from news (`n…`) and Wikipedia (`w…`). 750 sentences are originally English (`…01…`). 250 were translated from German, French, Italian and Spanish (`02`–`05`); from English they were further translated into other languages. Each sentence has counterparts in the PUD of other languages (`# parallel_id`). Annotation history:
1. Google annotated it following "universal" guidelines based on Stanford Dependencies.
2. Martin Popel automatically converted it to UD v2 (Udapi, `ud.Google2ud`).
3. Schuster, Reddy and Manning manually corrected UPOS and syntax.
4. Lemmas and FEATS were assigned by CoreNLP, without checking.

Test only. License CC BY-SA 3.0.

**Conditions and exceptions.**
- The README changelog ends at 2.6 ("No changes"). Yet the 2.18 data have subtypes and features of later versions: `nmod:unmarked` 111, `obl:unmarked` 37, `nmod:desc` 41, `advcl:relcl` 1, `ExtPos` 76, `PronType=Dem` on *there* (35 of 35). So the treebank received mass edits from the UD maintainers that are not recorded in the README.
- DEPS — enhanced UD, automatic since 2.2, unchecked. 7 empty nodes.
- 5 sentences still contain annotator notes `# Checktree: …` (*Not sure about this one*, *Ellipsis*), and one more `# notes`.

**Examples.** `n01002042` *The new spending is fueled by Clinton’s large bank account.*; `n01085008` *Fast forward to 2016 and this is increasingly worthy of attention.*

**In UD.**

| layer | state |
|---|---|
| XPOS | PTB, manual; no `ADD`, `LS`, `NFP` |
| FEATS | CoreNLP: agreement by form, no `Voice`, no `Case=Gen` (`feats-corenlp.md`) |
| lemmas | CoreNLP, unchecked |
| DEPS | present |
| MWT | 129 |

Our annotator on PUD: UPOS 94.42, XPOS 94.07, UFeats 87.55, LAS 77.02, MLAS 58.04. UPOS and XPOS are almost as on EWT, UFeats is 8 points lower — CoreNLP customs.

**Sources.** README `UD_English-PUD` (from Google's README); `data/raw/ud-docs/docs/treebanks/en_pud/index.md`; Zeman et al. 2017, `K17-3001` (papers/z/ze/zeman-2017-conll-2017-shared-task-multilingual); McDonald et al. 2013 (Google universal treebanks, ACL 2013).
