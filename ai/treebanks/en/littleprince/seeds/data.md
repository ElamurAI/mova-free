# LittlePrince — what these data are and which layers exist

**Gist.** 500 sentences (6852 words) from the English translation of "The Little Prince". The basis is silver Stanza parses from the English Little Prince SNACS corpus (v1.0). Students of the course 11-422 Grammar Formalisms (CMU, Lori Levin) corrected them in ArboratorGrew: two annotators per sentence, ≈ 25% checked by the course instructor. Test only. License CC BY-SA 4.0. In UD since 2.17.

**Conditions and exceptions.**
- `# sent_id` — *lpp_1943.NNN*, by the sentence number in the text.
- `# mwe` in 499 sentences — inherited from the SNACS corpus: the same text as `# text`.
- The README metadata say «XPOS: not available», but the data have XPOS on all words: PTB tags, 39 values. Stanza probably left it, and it is unknown whether it was corrected together with UPOS.
- `# text` — a tokenized string with spaces (*mistakes , also .*), there is no `SpaceAfter`. No MWT. So the original text cannot be reconstructed, as in ESLSpok.
- `NumForm` is in MISC (31: `NumForm=Word` 16, `NumForm=Digit` 15), not in FEATS.

**Examples.** `lpp_1943.265` *Bit by bit I came to understand the secrets of your sad little life ...*; `lpp_1943.284` *" One day , " you said to me , " I saw the sunset forty - four times ! "*

**In UD.**

| layer | state |
|---|---|
| XPOS | PTB from Stanza; no `ADD`, `AFX`, `FW`, `GW`, `LS`, `NFP`, `WP$`, `$`, `-LRB-`/`-RRB-` |
| FEATS | present; older EWT customs (`stanza-legacy.md`) |
| DEPS | absent |
| MISC | «_» in 6820 of 6852 |
| MWT | absent |

Our annotator on LittlePrince: UPOS 96.29, XPOS 95.81, UFeats 85.90, LAS 79.80, MLAS 59.02. UPOS and LAS are almost as on EWT, UFeats is 10 points lower — the customs from `stanza-legacy.md`.

**Sources.** README `UD_English-LittlePrince`; `data/raw/ud-docs/docs/treebanks/en_littleprince/index.md`; Qi et al. 2020 (Stanza), `2020.acl-demos.14` (papers/q/qi/qi-2020-stanza-python-natural-language-processing); digest, §1.11 (it says "no XPOS" — per the metadata, not per the data).
