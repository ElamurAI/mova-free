# ParTUT — what these data are and which layers exist

**Gist.** The English part of the parallel Italian-French-English treebank ParTUT (Turin): 2090 sentences and 49,478 words. Sources:
- Creative Commons licenses;
- DGT-TM, JRC-Acquis, the Universal Declaration of Human Rights — law;
- Europarl;
- Facebook pages;
- Project Syndicate;
- Wikipedia;
- WIT3 (talks).

The annotation was originally done in the TUT scheme — dependencies grown out of Italian grammar. From there it was converted to Universal Stanford Dependencies, then to UD, with manual corrections. All columns are marked "converted with corrections". No changes since 2.5.

**Conditions and exceptions.**
- Split: train 1781 / dev 156 / test 153 sentences. The English part is split the same way as the Italian one, to preserve parallelism.
- License CC BY-NC-SA 4.0: for `mova` test and research only.
- `# sent_id` and `# text` are present, `# newdoc id` is not.

**Examples.** `…Shakespeare's property…` — *Shakespeare's* is split as an MWT: *Shakespeare* (`nmod:poss`) + *'s* (PART `case`); *property* — `nmod` → the next noun (`nmod-compound.md`).

**In UD.**

| layer | state |
|---|---|
| XPOS | the Italian TUT/ISST set: `S`, `E`, `V`, `A`, `RD`, `FF`, `SP`, `AP`…; «_» — 73 (`xpos-tut.md`) |
| FEATS | present, without `Case`, `Voice`, `NumForm`, `Reflex` (`feats.md`) |
| lemmas | present |
| DEPS | absent |
| MISC | «_» in 43,590 of 49,478: only `SpaceAfter` |
| MWT | 445, of which 353 are possessive *X's* |

Our annotator on the ParTUT test: UPOS 92.12, UFeats 78.90, LAS 77.67, MLAS 44.45. XPOS is not measured: the tags are not PTB.

**Sources.** README `UD_English-ParTUT`; `data/raw/ud-docs/docs/treebanks/en_partut/index.md`; Sanguinetti & Bosco 2014 (PartTUT; conversion to USD, CLiC-it 2014; TLT-13); Bosco et al. 2013, `W13-2308`; digest, §1.11.
