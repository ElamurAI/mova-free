# Double comparison: more happier

**Gist.** Degrees of comparison in English are formed either with an ending (*happier, happiest*) or with the word *more/most* (*more beautiful*), but not both at once: ✗ *more happier*, ✗ *most easiest*. ERRANT puts such errors into a separate class ADJ:FORM (*goodest → best*, *more easy → easier*).

**Conditions and exceptions.**
- *more* as a quantity (*more people*) is not covered by the rule: only `advmod` on JJR/RBR is taken.
- Archaic *most unkindest* (Shakespeare) is a stylistic exception.

**Examples.**
- ✗ *The customers are more happier.* → ✓ *happier*.
- ✓ *more beautiful*

**In UD.** advmod(happier, more), where *happier* is JJR.

**Check against gold.** EWT 2.18: 2 — both errors in the text.

**Sources.** `P17-1074` (table 2: ADJ:FORM); `W19-4406` (ADJ:FORM — 0.24 % of W&I edits).

The rule is `en.nominal.double-comparison` in `nominal/double-comparison.md`.
