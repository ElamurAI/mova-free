# :tmod and :npmod merged into :unmarked (UD 2.15)

**Gist.** Prepositionless nominal adverbials and modifiers: *We met last year*, *five days before the funeral*, *$5 a share*. Before UD 2.15 they were split into `:tmod` (time) and `:npmod` (the rest), and since 2.15 EWT and GUM have a single subtype `:unmarked` (`obl:unmarked`, `nmod:unmarked`). The old label is produced by an annotator trained on old releases. Or by data from treebanks that still keep the old scheme: ATIS, ESLSpok, CHILDES, LittlePrince.

**Conditions and exceptions.** For ATIS, ESLSpok, CHILDES and LittlePrince themselves this is not an error but their convention. Mova keeps EWT 2.18 style, so the old label is an error for it. LAS under the CoNLL 2018 metrics does not see subtypes, so this discrepancy does not affect LAS.

**Examples.**
- *We met last year* → obl:unmarked(met, year).
- *IBM earned $5 a share* → nmod:unmarked($, share).
- *five days before the funeral* → nmod:unmarked(funeral, days).

**Check against gold.** EWT 2.18 — 0; ESLSpok 2.18 — 151.

**Sources.** UD `_en/dep/obl-unmarked.md`, `_en/dep/nmod-unmarked.md`, `_en/dep/nmod.md`; english-banks §1.4 (2.15: issues #1028, #1094), §2 (discrepancies within UD English), §4 item 4 (version drift).

```rule
rule: en.errors.tmod-npmod
what: obsolete subtype :tmod/:npmod — since UD 2.15 (EWT, GUM) only :unmarked
match: t[rel=obl:tmod|obl:npmod|nmod:tmod|nmod:npmod]
require: t[rel=obl:unmarked|nmod:unmarked]
severity: error
source: UD _en/dep/obl-unmarked.md, _en/dep/nmod-unmarked.md; english-banks §1.4 (2.15, #1028, #1094)
```
