# CTeTex — known annotation errors

**Gist.** CTeTex was annotated manually by one main annotator (≈ 180 hours). There are few systematic errors. The problem for merging is incomplete FEATS and non-standard customs, not faults. Below is what contradicts the UD guideline itself. The converter does not fix this.

**Examples and numbers.**
- **`dep` on *s* in *requirement(s)*** — 7 of 8 `dep`. This is a plural ending in parentheses. EWT annotates such *s* as X/AFX with `amod` (9 times, 1 more — `compound`), not `dep`.
- **`reparandum` in written text** — 2 (*on*, *for*): requirements have no speaker self-corrections, so these are most likely typos. `Typo` or `goeswith` would be more correct.
- **`Foreign=Yes` in MISC on formula symbols** — 21 (*n*, *2*, */*, *160*). `Foreign` is a FEATS feature for foreign words, not for mathematical notation.
- **Possessive *'* as PUNCT `punct`** — 4: *users'* etc. It should be PART `case`, like *'s*.
- **Passive subject as `nsubj`** (`tb.en.pass-subj`) — 3 of 107.
- **Numbers in variables and TBD.** The variables *t*, *n* have UPOS NUM, *TBD* — VERB or NUM. This is a deliberate choice by the authors (sec. 4.1–4.2), not an error, but EWT has no analogues.

**In UD.** CTeTex is test only. Evaluating `en` on it requires the reverse converter (`../convert.md`): otherwise UFeats and MLAS measure the difference in customs, not quality.

**Sources.** Hassert et al. 2021, `2021.udw-1.5`, sec. 4.1–4.2 (variables — NUM, TBD — by the head of the expression); https://universaldependencies.org/u/dep/goeswith.html, https://universaldependencies.org/u/dep/reparandum.html, https://universaldependencies.org/u/feat/Foreign.html.
