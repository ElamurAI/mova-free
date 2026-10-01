# Features from context, form wins; `Exponence` — proposal

**Proposal:**
- **accept** the principle of clarification No. 18: feature values come from context, and in a conflict the form wins;
- **wait** on `Exponence` in MISC.

**What exactly in `mova`.**
- keep the "FEATS from the subject" normalization (`en::ud::agreement`, `train/dialects-and-converters.md`) and give it a boundary: where the form is unambiguous, agreement does not override it. VBZ is always `Sing|3`. VBP is never `Sing|3`: *the team **are*** → `Plur|3`. *Was* is always `Sing`;
- put gates `udnext.vbz-3sg` and `udnext.vbp-not-3sg` on the `en` output and on silver data.

**Why accept.**
- **This is already EWT practice:**
  - VBD and VBP get `Number`/`Person` from the subject (VBD `Sing|3` 3499, VBP `Plur|3` 2515…);
  - *you* and *it* get `Case` by position;
  - VBZ `Sing|3` — 5655 of 5656 (the exception is a token with empty FEATS);
  - there is no VBP `Sing|3` in the basic tree.
- **Lossless:** export to 2.18 changes nothing.
- **What it guards against:** the subject normalization must not override the form under semantic agreement (*the team are*, *a number of people are*). Otherwise it would produce a feature that is not in the standard.

**Why wait on `Exponence`.**
- There is no value set. Clarification No. 18 says directly: "We are not prepared to standardize the set of values without further community experimentation". There is only a pilot in French (`Exponence[Number]=Inherent`).
- #1233 is still open: "lexical" features on *a/each/every* and on modals.
- For English one split would be useful: `Number`/`Person` on VBD/VBP — from context, on VBZ — from the form. But it is computable from XPOS, so there is no need to write it into MISC until there is a standard.

**Check.** `udnext.vbz-3sg`, `udnext.vbp-not-3sg` (`dialects/ud-next/seeds/features-context.md`). On EWT 2.18: 1 and 0 violations.

**Origin.** `dialects/ud-next/seeds/features-context.md`; UD docs/changes.md#morphosyntactic-features`; #1233.

Mova decides.
