# ParTUT — XPOS from the Italian TUT/ISST set

**Gist.** ParTUT XPOS are Italian-grammar tags shared by all three languages of the treebank: `S` — noun, `E` — preposition, `V` — verb, `A` — adjective, `RD` — definite article, `RI` — indefinite, `SP` — proper noun, `FF`/`FS`/`FB` — punctuation, `VA`/`VM` — auxiliary/modal, `AP` — possessive adjective, `PE`/`PR`/`PD` — pronouns, `CS`/`CC` — conjunctions. A PTB tag cannot be recovered from it: `V` does not distinguish VB, VBD, VBZ, VBP; `S` — NN and NNS. In `mova` XPOS = PTB, so ParTUT XPOS is a separate layer.

**Conditions and exceptions.**
- The most frequent UPOS → XPOS pairs:
  - NOUN → `S` 10,571, ADP → `E` 5980, VERB → `V` 4722, ADJ → `A` 3740;
  - DET → `RD` 2958 / `RI` 973 / `AP` 634 / `DD` 444 / `DI` 232;
  - PUNCT → `FF` 2836 / `FS` 1959 / `FB` 797;
  - PROPN → `SP` 2230;
  - AUX → `VA` 973 / `VM` 758 / `V` 707: the copula *be* has had `V` rather than `VA` since 2.1.
- 73 words with XPOS «_».
- The CoNLL-U reader in `en` does not know these tags, so ParTUT does not train the PTB tagger (`Sentence::tagged()`).

**Examples.** *Shakespeare* — PROPN/`SP`; *'s* — PART/`PART`; *property* — NOUN/`S`.

**In UD.** Neither `ParTUT → mova` nor `mova → ParTUT` can be translated by rules:
- a PTB tag can still be derived from `V` + FEATS: `Tense=Past|VerbForm=Fin` → VBD etc.;
- back, `S`, `V`, `A` follow unambiguously from UPOS;
- `RD`/`RI`/`AP`/`DD`/`DI` — from `PronType` and `Definite`.

But the engine will not accept a tag outside PTB either in a condition or in an action: string XPOS is needed (`../convert.md`).

**Sources.** README `UD_English-ParTUT`, changelog v2.1 («changed xpos of copulas (from VA to V)»); Bosco et al. 2000, `L00-1168` (TUT); digest.
