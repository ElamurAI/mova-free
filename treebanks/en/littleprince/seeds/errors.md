# LittlePrince — known annotation errors

**Gist.** LittlePrince trees and UPOS were corrected by two students per sentence, while features and lemmas were probably corrected selectively (`stanza-legacy.md`). Tree errors are isolated. Feature inconsistency is massive, but it is inherited, not errors on individual words. The converter does not fix errors.

**Examples and numbers.**
- **Mixed norms in one treebank** — the main flaw:
  - *day* as `obl:unmarked` (5) and as `obl:tmod` (2);
  - *his* → *he* (39) and *his* → *his* (5);
  - Number on past *say* (6), but not on most other verbs.
- **XPOS not aligned with the corrected UPOS:** NN on ADJ (1) and on VERB (1). NN on PRON — 26, but that is an EWT custom (*something* NN/PRON), not an error.
- **Infinitive root without a subject** — 1: `lpp_1943.627` *" To admire mean that you regard me…"* — *mean* here is a finite verb with an infinitive subject, but is annotated as `VerbForm=Inf` without `csubj`.
- **Passive subject as `nsubj`** (`tb.en.pass-subj`) — 1 of 28.

**In UD.** 500 sentences — test only. It is useful for evaluating `en` after the reverse converter, but the mixed norms will produce noise even after it (`../convert.md`, "Round trip").

**Sources.** README `UD_English-LittlePrince`; rules in `../../seeds-overview.md`.
