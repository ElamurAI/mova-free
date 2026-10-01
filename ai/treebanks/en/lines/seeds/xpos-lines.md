# LinES — XPOS: own morphological tags, not PTB

**Gist.** LinES XPOS describes not the part of speech but the morphological form: `SG-NOM`, `PL-NOM`, `PAST`, `INF`, `ING`, `PERS-P3SG-NOM`, `P3SG-GEN`, `DEF`, `IND-SG`, `NEG`. The part of speech is carried by UPOS. Function-word ADP, ADV, CCONJ, SCONJ, PART have «_». `mova` keeps XPOS = PTB with EWT web tags, so for `mova` LinES XPOS is a different layer, not a variant of the same one.

**Conditions and exceptions.**
- 159 values. The most frequent:
  - «_» 22,538;
  - `SG-NOM` 16,645;
  - `DEF` 5838;
  - `Comma` 5693, `Period` 5130 — punctuation names in words;
  - `PAST` 5387;
  - `PL-NOM` 4824;
  - `INF` 3296.
- UPOS → XPOS correspondence:
  - NOUN → `SG-NOM` 13,909 / `PL-NOM` 4744;
  - DET → `DEF` 5838 / `IND-SG` 2919;
  - VERB → `PAST` 3909 / `INF` 2929 / `ING` 1883 / `PRES` 1547 / `PASS` 1338 / `PERF` 755;
  - AUX → `PAST-AUX` 1381 / `PRES-AUX` 1321 / `PAST` 1476 / `PRES` 833.
- A PTB tag can be recovered from this only partly. `SG-NOM` on NOUN is NN, on PROPN NNP. `PAST` — VBD. `PERF` and `PASS` — VBN. `ING` — VBG. `INF` — VB. But `PRES` does not distinguish VBP from VBZ, and «_» on ADP does not say whether it is IN or TO.
- The CoNLL-U reader in `en` rejects such tags as unknown (`Tag::parse` → none), so LinES does not go into PTB tagger training (`Sentence::tagged()`).

**Examples.** `PRON → PERS-P3SG-NOM` (*he, she, it*) 1485, `PART → NEG` (*not*) 870, `PUNCT → Quote` 527.

**In UD.** The converter into `mova` can set a PTB tag where the correspondence is unambiguous: NN, NNS, NNP, VBD, VBN, VBG, VB, DT. VBP/VBZ need Number and Person, and IN/TO/RP/RB need the lemma and role. So `mova` XPOS for LinES will be incomplete. Back, `mova` → LinES, the map is unambiguous for content words, because LinES XPOS is derived from UPOS and FEATS (`../convert.md`).

**Sources.** `data/raw/ud-docs/docs/treebanks/en_lines/index.md` («XPOS: annotated manually»); Ahrenberg 2007, `W07-2441`; `en/src/gram.rs` (the list of PTB tags in `en`).
