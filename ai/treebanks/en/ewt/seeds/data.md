# EWT — what these data are and which layers exist

**Gist.** The English Web Treebank on top of LDC2012T13: 16,622 sentences and 254,820 words from five web genres — blogs, newsgroups, e-mail, reviews, Yahoo! Answers. The LDC PTB trees were automatically converted to Stanford Dependencies, then manually corrected to UD. Dependencies were mostly annotated by one person, part by two, with agreement ≈ 96%. XPOS are the manual LDC tags. UPOS was derived from XPOS and the graph, lemmas and FEATS from CoreNLP; all of this has been selectively corrected over the years. EWT is the base of `mova` (`train/dialects-and-converters.md`): UD 2.18 with EWT conventions.

**Conditions and exceptions.**
- Split: train 12,544 / dev 2001 / test 2077 sentences.
- License CC BY-SA 4.0.
- Enhanced UD (DEPS) — automatic, with the Schuster & Manning 2016 converter, without manual checking. Reduced relatives were added in 2.14.
- The treebank changes its customs every year: `:outer`, `advcl:relcl`, `obl:agent`, `ExtPos`, `:unmarked`, `nmod:desc`, `flat` for place names. This is the chronology along which other treebanks got "stuck" at different versions (`../../seeds-overview.md`).

**Examples.** `email-enronsent30_02-0005` *You have now decided to set your sights on a position or situation that could give you greater prestige…* — an e-mail from a mailing list, a typical EWT text.

**In UD.** Layers — all of them:

| layer | state |
|---|---|
| lemmas | «_» only in 177 words |
| XPOS | PTB + LDC web tags: `ADD` 475, `NFP` 499, `AFX` 75, `GW` 345. `XX` is in the tag list but does not occur in 2.18 (`../../gum/seeds/xpos-web.md`) |
| FEATS | «_» in 79,476 words: punctuation and some function words |
| DEPS | enhanced; 43 empty nodes |
| MISC | `SpaceAfter` 30,961; STREUSLE in reviews (`Supersense`, `MWE*`, `PRel`); `Cxn`/`CxnElt` (UCxn, relative types); EWT's own attributes: `TemporalNPAdjunct` 1077, `FlatType` 425, `Promoted` 328, `Superlocation` 170; `CorrectForm` 1429 with `Typo=Yes` 1431 |
| MWT | 3327 |

Our annotator on the EWT test: UPOS 94.74, XPOS 94.40, UFeats 95.66, Lemmas 96.78, LAS 80.24, MLAS 70.75.

**Sources.** README `UD_English-EWT` (Introduction, Structure, MISC Annotations, Known Issues, Changelog); `data/raw/ud-docs/docs/treebanks/en_ewt/index.md`; Silveira et al. 2014, `L14-1067`; Zeldes & Schneider 2023, `2023.udw-1.7`; digest, §1.4.
